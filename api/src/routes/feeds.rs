use crate::models::feed::*;
use crate::models::user::User;
use crate::models::post::*;
use crate::pool;
use crate::schema::{feeds, users, user_feeds, posts};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use feed_rs::model;
use feed_rs::parser;
use poem::{
    error::{Error, Result},
    handler,
    http::StatusCode,
    web::{Data, Json, Path},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use futures::{stream, StreamExt};
use reqwest::Client;

const CONCURRENT_REQUESTS: usize = 2;

fn bad_request() -> Error {
    Error::from_status(StatusCode::BAD_REQUEST)
}

#[handler]
pub async fn refresh_feeds(
    Data(user): Data<&User>,
    pool: Data<&pool::Pool>,
) -> Result<Json<serde_json::Value>> {
    let conn = pool.get().map_err(|_e| bad_request())?;

    let feeds: Vec<Feed> = UserFeed::belonging_to(user)
        .inner_join(feeds::table)
        .select(feeds::all_columns)
        .load::<Feed>(&conn)
        .map_err(|_e| bad_request())?;

    let client = Client::new();

    let urls: Vec<String> = feeds.iter().map(|x| x.rss_url.clone()).collect::<Vec<String>>();

    let mut bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = reqwest::get(url).await;
                resp?.text().await
            }
        })
    .buffer_unordered(CONCURRENT_REQUESTS);

    let mut posts: Vec<Post> = vec![];
    while let Some(item) = bodies.next().await {
        let text = item.unwrap();
        println!("text {:?}", text);
        let feed: model::Feed = match parser::parse(text.as_bytes()) {
            Ok(f) => f,
            Err(e) => {
                println!("Parsing failed 😿 {:?}", e);
                continue
            }
        };

        // Convert the entry from the feed_rs library into our own type.
        let new_posts: Vec<NewPost> = feed
            .entries
            .iter()
            .cloned()
            .map(|x| NewPost::try_from(x))
            .flatten()
            .collect();

        println!("Got something? {:?}", new_posts);

        for p in new_posts {
            let post = p.insert_into(posts::table).get_result::<Post>(&conn).unwrap();
            posts.push(post);
        }
    }

    Ok(Json(json!({ "feeds": posts })))
}

#[handler]
pub async fn list_feeds(
    Data(user): Data<&User>,
    pool: Data<&pool::Pool>,
) -> Result<Json<serde_json::Value>> {
    let conn = pool.get().map_err(|_e| bad_request())?;

    let feeds: Vec<Feed> = UserFeed::belonging_to(user)
            .inner_join(feeds::table)
            .select(feeds::all_columns)
            .load::<Feed>(&conn)
            .map_err(|_e| bad_request())?;

    Ok(Json(json!({ "feeds": feeds })))
}

#[handler]
pub async fn list_posts(
    user_id: Path<String>,
    pool: Data<&pool::Pool>,
) -> Result<Json<serde_json::Value>> {
    let conn = pool.get().map_err(|_e| bad_request())?;

    let id = user_id.parse::<i32>().map_err(|_e| bad_request())?;

    let posts: Vec<Post> = users::table.inner_join(
            user_feeds::table.inner_join(
                feeds::table.inner_join(
                    posts::table
                )
            )
        )
        .filter(users::id.eq(id))
        .select(posts::all_columns)
        .load(&conn)
        .map_err(|_e| bad_request())?;

    Ok(Json(json!({ "posts": posts })))
}

#[derive(Debug, Deserialize)]
pub struct CreateFeed {
    uri: String,
}

#[handler]
pub async fn create_feed(
    Data(user): Data<&User>,
    Json(body): Json<CreateFeed>,
    pool: Data<&pool::Pool>,
) -> Result<Json<serde_json::Value>> {
    let conn = pool.get().map_err(|_e| bad_request())?;

    let rss_url = body.uri.to_owned();
    let text = reqwest::get(&rss_url)
        .await
        .map_err(|_e| bad_request())?
        .text()
        .await
        .map_err(|_e| bad_request())?;

    let feed: model::Feed = parser::parse(text.as_bytes()).map_err(|_e| bad_request())?;

    // Convert to our feed type
    let mut our_feed: NewFeed = NewFeed::try_from(feed.clone()).map_err(|_e| bad_request())?;
    our_feed.rss_url(Some(rss_url));

    let inserted_feed = our_feed.insert_into(feeds::table)
            .get_result::<Feed>(&conn)
            .map_err(|_e| bad_request())?;

    let new_user_feed: UserFeed = UserFeed {
        user_id: user.id,
        feed_id: inserted_feed.id
    };

    // Insert the join-table record.
    new_user_feed.insert_into(user_feeds::table)
        .execute(&conn)
        .map_err(|_e| bad_request())?;

    // Convert the entry from the feed_rs library into our own type.
    let new_posts: Vec<NewPost> = feed
        .entries
        .iter()
        .cloned()
        .map(|x| NewPost::try_from(x))
        .flatten()
        .collect();

    // TODO: Do this with a map?
    let mut posts: Vec<Post> = vec![];
    for mut p in new_posts {
        p.feed_id(Some(inserted_feed.id));
        let post = p.insert_into(posts::table).get_result::<Post>(&conn).unwrap();
        posts.push(post);
    }

    Ok(Json(json!(Posts { posts })))
}

// TODO: Use this. Figure out how to best store. Doesn't feel like it should have its own table.
#[derive(Debug, Deserialize, Serialize)]
struct Author {
    name: String,
    url: Option<String>,
    email: Option<String>,
}

impl From<model::Person> for Author {
    fn from(p: model::Person) -> Self {
        Self {
            name: p.name,
            url: p.uri,
            email: p.email,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Entry {
    title: Option<String>,
    summary: Option<String>,
    published_at: Option<DateTime<Utc>>,
    // authors: Vec<Author>,
    url: Option<String>,
}

impl From<model::Entry> for Entry {
    fn from(e: model::Entry) -> Self {
        // let authors = e.authors.iter().cloned().map(|x| Author::from(x)).collect();
        let link = e.links.first().map(|x| x.href.clone());

        Self {
            title: e.title.map(|x| x.content),
            summary: e.summary.map(|x| x.content),
            published_at: e.published,
            url: link,
            // authors,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Posts {
    posts: Vec<Post>,
}

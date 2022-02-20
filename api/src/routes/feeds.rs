use crate::models::*;
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

fn bad_request() -> Error {
    Error::from_status(StatusCode::BAD_REQUEST)
}

fn get_user_from_user_id(user_id: Path<String>, conn: &diesel::PgConnection) -> Result<User> {
    let id = user_id.parse::<i32>().map_err(|_e| bad_request())?;

    users::table.find(id).first::<User>(conn).map_err(|_e| bad_request())
}

// TODO: loop through all the feeds for a user and fetch them
// pub async fn refresh_feeds() {}

#[handler]
pub async fn list_feeds(
    user_id: Path<String>,
    pool: Data<&pool::Pool>,
) -> Result<Json<serde_json::Value>> {
    let conn = pool.get().map_err(|_e| bad_request())?;

    let user = get_user_from_user_id(user_id, &conn)?;

    let feeds: Vec<Feed> = UserFeed::belonging_to(&user)
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
struct CreateFeed {
    uri: String,
}

#[handler]
pub async fn create_feed(
    user_id: Path<String>,
    Json(body): Json<CreateFeed>,
    pool: Data<&pool::Pool>,
) -> Result<Json<serde_json::Value>> {
    let conn = pool.get().map_err(|_e| bad_request())?;

    let user = get_user_from_user_id(user_id, &conn)?;

    let text = reqwest::get(body.uri)
        .await
        .map_err(|_e| bad_request())?
        .text()
        .await
        .map_err(|_e| bad_request())?;

    let feed: model::Feed = parser::parse(text.as_bytes()).map_err(|_e| bad_request())?;

    // Convert to our feed type
    let our_feed: NewFeed = NewFeed::try_from(feed.clone()).map_err(|_e| bad_request())?;

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
    for p in new_posts {
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

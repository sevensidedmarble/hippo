use crate::models::*;
use crate::pool;
use crate::schema::feeds;
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

// pub async fn refresh_feeds() {}
// pub async fn list_feeds() {}

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

    let text = reqwest::get(body.uri)
        .await
        .map_err(|_e| bad_request())?
        .text()
        .await
        .map_err(|_e| bad_request())?;

    let feed: model::Feed = parser::parse(text.as_bytes()).map_err(|_e| bad_request())?;

    // Convert to our feed type
    let our_feed: NewFeed = NewFeed::try_from(feed.clone()).map_err(|_e| bad_request())?;

    match our_feed.insert_into(feeds::table).execute(&conn) {
        Ok(rows) => println!("inserted {:?} rows", rows),
        Err(_) => println!("something went wrong"),
    };

    // insert_into(users::table)
    //     .values(our_feed)
    //     .execute(&conn)
    //     .map_err(|_e| bad_request())?;

    // Convert the entry from the feed_rs library into our own type.
    let entries: Vec<_> = feed
        .entries
        .iter()
        .cloned()
        .map(|x| Entry::from(x))
        .collect();

    Ok(Json(json!(Entries { entries })))
}

// #[derive(Debug, Deserialize, Serialize)]
// struct Feed {
//     url: String,
//     title: Option<String>,
//     changed_at: Option<DateTime<Utc>>,
//     description: Option<String>,
// }

// impl TryFrom<model::Feed> for Feed {
//     type Error = &'static str;

//     fn try_from(f: model::Feed) -> Result<Self, Self::Error> {
//         match f.links.first().map(|x| x.href.clone()) {
//             Some(url) => Ok(Self {
//                 url,
//                 title: f.title.map(|x| x.content),
//                 changed_at: f.updated,
//                 description: f.description.map(|x| x.content),
//             }),
//             None => Err("No URL"),
//         }
//     }
// }

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
struct Entries {
    entries: Vec<Entry>,
}

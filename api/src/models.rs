use super::schema::{feeds, posts, users};
use chrono::Utc;
use diesel::{Insertable, Queryable};
use feed_rs::model;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Queryable)]
pub struct Feed {
    pub id: i32,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub changed_at: Option<chrono::NaiveDateTime>, // When the feed says it was last changed
    pub fetched_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize)]
#[table_name = "feeds"]
pub struct NewFeed {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub changed_at: Option<chrono::NaiveDateTime>,
    pub fetched_at: Option<chrono::NaiveDateTime>,
}

impl TryFrom<model::Feed> for NewFeed {
    type Error = &'static str;

    fn try_from(f: model::Feed) -> Result<Self, Self::Error> {
        match f.links.first().map(|x| x.href.clone()) {
            Some(url) => Ok(Self {
                url,
                title: f.title.map(|x| x.content),
                changed_at: f.updated.map(|x| x.naive_utc()),
                description: f.description.map(|x| x.content),
                fetched_at: Some(Utc::now().naive_utc()),
            }),
            None => Err("No URL"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Queryable)]
pub struct Post {
    pub id: i32,
    pub feed_id: i32,
    pub url: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub feed_id: i32,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Queryable)]
pub struct UserFeed {
    pub user_id: i32,
    pub feed_id: i32,
}

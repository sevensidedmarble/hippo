use super::feed::Feed;
use crate::schema::posts;
use diesel::{Associations, Insertable, Queryable};
use feed_rs::model;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Associations, Deserialize, Serialize, Debug, Clone, Queryable)]
#[belongs_to(Feed)]
pub struct Post {
    pub id: i32,
    pub feed_id: Option<i32>,
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
    pub feed_id: Option<i32>,
    pub url: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub published_at: Option<chrono::NaiveDateTime>,
}

impl NewPost {
    pub fn feed_id(&mut self, feed_id: Option<i32>) {
        self.feed_id = feed_id;
    }
}

impl TryFrom<model::Entry> for NewPost {
    type Error = &'static str;

    fn try_from(e: model::Entry) -> Result<Self, Self::Error> {
        match e.links.first().map(|x| x.href.clone()) {
            Some(url) => Ok(Self {
                feed_id: None,
                url,
                title: e.title.map(|x| x.content),
                published_at: e.published.map(|x| x.naive_utc()),
                summary: e.summary.map(|x| x.content),
            }),
            None => Err("No URL"),
        }
    }
}

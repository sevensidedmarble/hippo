joinable!(user_feeds -> users (user_id));
joinable!(user_feeds -> feeds (feed_id));
joinable!(posts -> feeds (feed_id));

table! {
    feeds (id) {
        id -> Int4,
        rss_url -> Text,
        url -> Nullable<Text>,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        changed_at -> Nullable<Timestamp>,
        fetched_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Int4,
        feed_id -> Nullable<Int4>,
        url -> Text,
        title -> Nullable<Text>,
        summary -> Nullable<Text>,
        published_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_feeds (user_id, feed_id) {
        user_id -> Int4,
        feed_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    feeds,
    posts,
    user_feeds,
    users,
);

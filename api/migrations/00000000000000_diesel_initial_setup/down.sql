DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS feeds;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS user_feeds;

DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();

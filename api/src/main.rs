#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use poem::{
    get, handler,
    listener::TcpListener,
    middleware::{AddData, Cors},
    post,
    session::{CookieConfig, RedisStorage, ServerSession, Session},
    EndpointExt, Route, Server,
};
use redis::{aio::ConnectionManager, Client as Redis};
use tracing::info;
use std::process;

mod crypto;
mod models;
mod middleware;
mod pool;
mod routes;
mod schema;

fn redis_url() -> String {
    std::env::var("REDIS_URL").expect("REDIS_URL must be set")
}

#[handler]
async fn count(session: &Session) -> String {
    let count = session.get::<i32>("count").unwrap_or(0) + 1;
    session.set("count", count);
    format!("Hello!\nHow many times have seen you: {}", count)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Load any environment variables.
    dotenv().ok();

    // Setup logging stuff.
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    // Open connection to redis.
    let client = Redis::open(redis_url()).unwrap();
    let redis_conn = ConnectionManager::new(client).await.unwrap_or_else(|err| {
        println!("There was a problem connecting to Redis: {}", err);
        process::exit(1)
    });

    // Initialize postgres connection pool.
    let pool = pool::init_pool();

    let cors_middleware = Cors::new().allow_origins_fn(|_| true).allow_credentials(true);

    // Setup the endpoint handlers.
    let app = Route::new()
        .at("/auth/login", post(routes::auth::login))
        .at("/auth/register", post(routes::auth::register))
        .at("/users/:user_id/feeds", post(routes::feeds::create_feed).get(routes::feeds::list_feeds).around(middleware::auth::auth_middleware))
        .at("/users/:user_id/posts", get(routes::feeds::list_posts).around(middleware::auth::auth_middleware))
        .at("/users/:user_id/refresh_feeds", post(routes::feeds::refresh_feeds).around(middleware::auth::auth_middleware))
        .at("/", get(count))
        .at("/:user_id", get(count).around(middleware::auth::auth_middleware))
        .with(cors_middleware)
        .with(AddData::new(pool))
        .with(ServerSession::new(
            CookieConfig::default().secure(false),
            RedisStorage::new(redis_conn),
        ));

    // Go!
    info!("Booting up...");
    Server::new(TcpListener::bind("0.0.0.0:8888"))
        .run(app)
        .await
}

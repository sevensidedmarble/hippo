#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use poem::{
    get, handler,
    listener::TcpListener,
    middleware::AddData,
    post,
    session::{CookieConfig, RedisStorage, ServerSession, Session},
    EndpointExt, Route, Server,
};
use redis::{aio::ConnectionManager, Client};
use tracing::info;

mod crypto;
mod models;
mod pool;
mod routes;
mod schema;

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
    let client = Client::open("redis://127.0.0.1/").unwrap();

    // Initialize postgres connection pool.
    let pool = pool::init_pool();

    // Setup the endpoint handlers.
    let app = Route::new()
        .at("/auth/login", post(routes::auth::login))
        .at("/auth/register", post(routes::auth::register))
        .at("/users/:user_id/feeds", post(routes::feeds::create_feed))
        .at("/", get(count))
        .with(AddData::new(pool))
        .with(ServerSession::new(
            CookieConfig::default().secure(false),
            RedisStorage::new(ConnectionManager::new(client).await.unwrap()),
        ));

    // Go!
    info!("Booting up...");
    Server::new(TcpListener::bind("127.0.0.1:8888"))
        .run(app)
        .await
}

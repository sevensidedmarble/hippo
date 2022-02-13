use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use std::process;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).unwrap_or_else(|_| {
        println!("There was a problem initializing the db connection pool. Is your DB running?");
        process::exit(1)
    })
}

fn database_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

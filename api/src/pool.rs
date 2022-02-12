use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

fn database_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

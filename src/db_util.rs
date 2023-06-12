use std::env;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;

use dotenvy::dotenv;
use r2d2;
use r2d2::Pool;

const DATABASE_URL: &'static str = "DATABASE_URL";

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var(DATABASE_URL).expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Could not build connection pool")
}
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;
use r2d2::Pool;

pub fn establish_connection(db_url: &String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager).expect("Could not build connection pool")
}

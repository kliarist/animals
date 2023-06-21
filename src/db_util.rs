use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use r2d2::{Pool, PooledConnection};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn establish_connection(db_url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn run_pending_migrations(mut connection: PooledConnection<ConnectionManager<PgConnection>>) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    Connection, SqliteConnection,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env::var;

pub mod models;
pub mod query;
mod schema;

pub fn run_migration(connection: &mut SqliteConnection) {
    const MIGRATIONS: EmbeddedMigrations = diesel_migrations::embed_migrations!();

    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

fn get_connection() -> SqliteConnection {
    dotenv::dotenv().ok();

    let database_url =
        var("DATABASE_URL").expect("Environnement variable DATABASE_URL must be set.");

    SqliteConnection::establish(&database_url).expect("Failed to connect to the database")
}

pub fn get_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv::dotenv().ok();

    let database_url =
        var("DATABASE_URL").expect("Environnement variable DATABASE_URL must be set.");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .max_size(10)
        .build(manager)
        .expect("Failed to build the connection pool")
}

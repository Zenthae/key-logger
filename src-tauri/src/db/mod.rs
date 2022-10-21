use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    Connection, SqliteConnection,
};
use diesel_migrations::EmbeddedMigrations;
use dotenv::dotenv;
use std::env::var;

pub mod models;
pub mod query;
mod schema;

pub struct Database {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Database {
    /// Specify database_url or None
    pub fn new(database_url: Option<String>, pool_size: Option<u32>) -> Database {
        let url = if let Some(url) = database_url {
            url
        } else {
            dotenv().ok();
            var("DATABASE_URL").expect("Environnement variable DATABASE_URL must be set.")
        };

        let size = if let Some(s) = pool_size { s } else { 10 };

        let manager = ConnectionManager::<SqliteConnection>::new(url);

        let pool = Pool::builder()
            .test_on_check_out(true)
            .max_size(size)
            .build(manager)
            .expect("Failed to build the connection pool.");

        Database { pool }
    }

    /// Execute SQLite database migrations
    pub fn run_migration(&self) {
        const MIGRATIONS: EmbeddedMigrations = diesel_migrations::embed_migrations!();

        // self.get_connexion()
    }

    /// Force close all connection to the database
    pub fn close() {
        todo!()
    }

    pub fn get_connexion(&self) -> PooledConnection<ConnectionManager<SqliteConnection>> {
        self.pool.get().unwrap()
    }
}

pub fn run_migration(connexion: &mut SqliteConnection) {}

pub fn get_connexion() -> SqliteConnection {
    dotenv::dotenv().ok();

    let database_url =
        var("DATABASE_URL").expect("Environnement variable DATABASE_URL must be set.");

    SqliteConnection::establish(&database_url).expect("Failed to connect to the database")
}

pub fn get_pool() {
    Pool::builder()
        .test_on_check_out(true)
        .max_size(10)
        .build(manager)
        .expect("Failed to build the connexion pool")
}

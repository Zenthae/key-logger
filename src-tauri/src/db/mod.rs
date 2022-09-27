use std::env;

use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use dotenv::dotenv;

pub mod models;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(connection: &mut SqliteConnection) {
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn register_event() {}

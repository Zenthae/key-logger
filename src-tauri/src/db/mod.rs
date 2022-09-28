pub mod models;
pub mod schema;

use crate::db::models::NewEvent;

use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(conn: &mut SqliteConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn register_event(conn: &mut SqliteConnection, event_time: &i32, key_name: &str) -> usize {
    use crate::db::schema::key_event;

    let new_event = NewEvent {
        event_time,
        key_name,
    };

    diesel::insert_into(key_event::table)
        .values(&new_event)
        .execute(conn)
        .expect("Error saving new Event")
}

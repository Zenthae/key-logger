use std::{env, time::Duration};

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub mod query;

/// Execute database migrations
pub async fn run_migrations(db: &DatabaseConnection) {
    Migrator::up(db, None)
        .await
        .expect("Failed to run database migrations");
}

/// Get a connection pool to the database with custom options
pub async fn get_connection() -> DatabaseConnection {
    let mut options = ConnectOptions::new(get_database_url());
    options
        .min_connections(1)
        .max_connections(100)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true);

    Database::connect(options)
        .await
        .expect("Failed to connect to the database")
}

/// Calculate the database url depending on the context the app is run on
fn get_database_url() -> String {
    dotenv::dotenv().ok();

    env::var("DATABASE_URL").expect("Environnement variable DATABASE_URL must be set.")
}

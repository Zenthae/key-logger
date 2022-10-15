use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use dotenv::dotenv;
use std::env::var;

pub struct Database {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Database {
    /// Specify database_url or None
    pub fn new(database_url: Option<String>) -> Database {
        let url = match database_url {
            Some(url) => url,
            None => {
                dotenv().ok();
                var("DATABASE_URL").expect("Environnement variable DATABASE_URL must be set.")
            }
        };

        let manager = ConnectionManager::new(url);

        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Failed to build the connection pool.");

        Database { pool }
    }

    /// Execute SQLite database migrations
    pub fn run_migration() {
        todo!()
    }

    /// Force close all connection to the database
    pub fn close() {
        todo!()
    }

    pub fn get_connexion() {
        todo!()
    }
}

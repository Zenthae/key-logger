use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use std::env::var;

use self::query::Query;

pub mod models;
pub mod query;
pub(crate) mod schema;

pub type DBPool = Pool<ConnectionManager<SqliteConnection>>;
pub type PooledConn = PooledConnection<ConnectionManager<SqliteConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Database {
        dotenv::dotenv().ok();

        let database_url =
            var("DATABASE_URL").expect("Environnement variable DATABASE_URL must be set.");

        let manager = ConnectionManager::<SqliteConnection>::new(database_url);

        let pool = Pool::builder()
            .test_on_check_out(true)
            .max_size(10)
            .build(manager)
            .expect("Failed to build the connection pool");

        Database { pool }
    }

    /// Must be run when the database schema get updated
    pub fn run_migration(&self) {
        const MIGRATIONS: EmbeddedMigrations = diesel_migrations::embed_migrations!();

        self.pool
            .clone()
            .get()
            .unwrap()
            .run_pending_migrations(MIGRATIONS)
            .unwrap();
    }

    pub fn connection(&self) -> PooledConn {
        self.pool.clone().get().unwrap()
    }

    /// Allow access to the different queries.
    /// Return a query object with a different connection
    pub fn query(&self) -> Query {
        let conn = self.pool.clone().get().unwrap();
        Query::new(conn)
    }
}

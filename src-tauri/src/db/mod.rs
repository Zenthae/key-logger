use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub mod models;
pub mod query;
pub mod schema;

pub fn get_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("Environnement variable DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to build the connection pool")
}

pub fn run_migrations(conn: &mut SqliteConnection) {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

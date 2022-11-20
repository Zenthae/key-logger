use sea_orm::prelude::*;

pub struct Database {
    pool: DatabaseConnection,
}

impl Database {
    pub fn new() -> Self {
        Self { pool: () }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

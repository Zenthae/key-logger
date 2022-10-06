use diesel::{prelude::*, SqliteConnection};

use crate::db::models::NewEvent;

pub fn create_event(conn: &mut SqliteConnection, event: NewEvent) -> String {
    use crate::db::schema::key_event::dsl::*;

    diesel::insert_into(key_event)
        .values(&event)
        .execute(conn)
        .expect("Failed to save event");

    serde_json::to_string(&event).unwrap()
}

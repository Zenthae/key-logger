use super::models::NewEvent;
use diesel::{insert_into, prelude::*};

pub fn insert_event(connection: &mut SqliteConnection, event: NewEvent) {
    use super::schema::key_event::dsl::*;

    insert_into(key_event).values(&event).execute(connection);
}

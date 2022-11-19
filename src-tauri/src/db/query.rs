use super::{models::NewEvent, PooledConn};
use chrono::{DateTime, Utc};
use diesel::{insert_into, prelude::*};

pub struct Query {
    connection: PooledConn,
}

impl Query {
    pub fn new(connection: PooledConn) -> Query {
        Query { connection }
    }
    pub fn insert_event(&mut self, event: NewEvent) {
        use super::schema::key_event::dsl::*;
        // Deref PooledConnection
        // https://github.com/sfackler/r2d2/issues/37
        let conn = &mut *self.connection;

        insert_into(key_event).values(&event).execute(conn);
    }

    /// Return the events recorded between the left and right DateTime.
    pub fn events_between(left: DateTime<Utc>, right: DateTime<Utc>) {}
}

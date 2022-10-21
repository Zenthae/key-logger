use super::schema::key_event;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Event {
    pub id: usize,
    pub event_time: DateTime<Utc>,
    pub key_name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = key_event)]
pub struct NewEvent<'a> {
    pub event_time: &'a DateTime<Utc>,
    pub key_name: &'a str,
}

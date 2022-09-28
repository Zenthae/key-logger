use crate::db::schema::key_event;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub event_time: i32,
    pub key_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = key_event)]
pub struct NewEvent<'a> {
    pub event_time: &'a i32,
    pub key_name: &'a str,
}

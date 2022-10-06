#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use db::run_migrations;

use crate::db::{models::NewEvent, query::create_event};

// #[macro_use]
// extern crate diesel;
// extern crate diesel_migrations;

mod db;
mod listener;

fn main() {
    let pool = db::get_connection_pool();
    let conn = &mut pool.clone().get().unwrap();

    run_migrations(conn);

    let event = NewEvent {
        event_time: &0,
        key_name: "A",
    };

    create_event(conn, event);
}

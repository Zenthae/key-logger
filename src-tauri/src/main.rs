#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use diesel::prelude::*;
use diesel_migrations::embed_migrations;
use std::sync::Mutex;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
embed_migrations!("./migrations/");

pub mod db;
pub mod schema;

struct AppState {
    database_connection: Mutex<SqliteConnection>,
}

fn main() {
    let c = db::establish_connection();

    let state = AppState {
        database_connection: Mutex::new(db::establish_connection()),
    };

    diesel_migrations::run_pending_migrations(&c).expect("Error migrating database");

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use db::{establish_connection, run_migrations};
// use diesel::prelude::*;
// use std::sync::Mutex;

pub mod db;

struct AppState {
    // database_connection: Mutex<SqliteConnection>,
}

fn main() {
    run_migrations(&mut establish_connection());

    let state = AppState {
        // database_connection: Mutex::new(db::establish_connection()),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

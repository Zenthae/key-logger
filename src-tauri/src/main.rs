#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod db;

use db::{establish_connection, run_migrations};

struct AppState {}

fn main() {
    run_migrations(&mut establish_connection());

    let state = AppState {};

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

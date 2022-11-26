#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use recorder::Recorder;
use sea_orm::DatabaseConnection;

mod command;
mod database;
mod error;
mod logger;
mod pipeline;
mod recorder;

pub struct AppState {
    database: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    let db = database::get_connection().await;
    database::run_migrations(&db).await;

    let mut recorder = Recorder::new();

    recorder.init();

    recorder.run();

    let state = AppState { database: db };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![command::get_event_by_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

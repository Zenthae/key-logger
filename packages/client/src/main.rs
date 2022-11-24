#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;

use shared::{database, recorder::Recorder, DatabaseConnection};

struct AppState {
    database: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    let mut recorder = Recorder::default();
    recorder.init();

    let database = database::get_connection().await;

    let state = AppState { database };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![command::hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(unused)]

use db::Database;
use key_logger::KeyLogger;
use pipeline::Pipeline;

mod command;
mod db;
mod key_logger;
mod pipeline;

pub struct AppState {
    database: Database,
}

fn main() {
    let database = Database::new();
    database.run_migration();

    let mut pipeline = Pipeline::new();
    let tx = pipeline.open(database.connection());

    let mut logger = KeyLogger::new(tx);
    logger.start();

    let state = AppState { database };

    tauri::Builder::default()
        .manage(state)
        .run(tauri::generate_context!())
        .expect("Failed to start tauri app");
}

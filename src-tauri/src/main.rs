#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(unused)]

use std::sync::Mutex;

use db::Database;
use diesel::connection::SimpleConnection;
use key_logger::KeyLogger;
use tauri::{Manager, WindowEvent};

mod db;
mod key_logger;

struct AppState {}

/// App Start :
/// - Initialize app state
/// - Initialize DB connexion pool
/// - Initialize Logger
/// - Pass 1 DB connexion from the pool to the logger
/// - Start logging
/// - Start GUI
///
/// App Stop :
/// - Stop GUI
/// - Stop logging
/// - Close all DB connexions
/// - Drop logger
/// - Drop DB connexion pool
/// - Save app state ?
/// - Drop app state
/// - End of process
fn main() {
    let state = AppState {};
    // let db_connexion_pool = Database::new(None);
    let mut logger = Mutex::new(KeyLogger::new());
    logger.lock().unwrap().start();

    tauri::Builder::default()
        .on_window_event(move |event| {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                logger.lock().unwrap().stop();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("Failed to start tauri app");
    println!("closing");
}

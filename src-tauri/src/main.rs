#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(unused)]

use std::sync::{mpsc, Mutex};

use db::{get_connection_pool, run_migration};
use diesel::connection::SimpleConnection;
use key_logger::KeyLogger;
use pipeline::Pipeline;
use rdev::EventType;
use tauri::{Manager, WindowEvent};

mod db;
mod key_logger;
mod pipeline;

struct AppState {}

/// App Start :
/// - Initialize app state
/// - Initialize DB connection pool
/// - Initialize Logger
/// - Pass 1 DB connection from the pool to the logger
/// - Start logging
/// - Start GUI
///
/// App Stop :
/// - Stop GUI
/// - Stop logging
/// - Close all DB connections
/// - Drop logger
/// - Drop DB connection pool
/// - Save app state ?
/// - Drop app state
/// - End of process
fn main() {
    let state = AppState {};

    let pool = get_connection_pool();

    run_migration(&mut pool.get().unwrap());

    let pool = pool.clone();

    let mut pipeline = Pipeline::new();
    let tx = pipeline.open(&mut pool.get().unwrap());

    let mut logger = Mutex::new(KeyLogger::new(tx));

    logger.lock().unwrap().start();

    tauri::Builder::default()
        .on_window_event(move |event| {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                logger.lock().unwrap().stop();
            }
        })
        .run(tauri::generate_context!())
        .expect("Failed to start tauri app");
}

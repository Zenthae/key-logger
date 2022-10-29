#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(unused)]

use std::sync::{mpsc, Arc, Mutex};

use db::{get_connection_pool, run_migration};
use diesel::{
    connection::SimpleConnection,
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use key_logger::KeyLogger;
use pipeline::Pipeline;
use rdev::EventType;
use tauri::{Manager, WindowEvent};

mod db;
mod key_logger;
mod pipeline;

type DBPool = Pool<ConnectionManager<SqliteConnection>>;

struct AppState {
    database_connection_pool: Arc<Mutex<DBPool>>,
}

impl AppState {
    pub fn new(pool: DBPool) -> AppState {
        AppState {
            database_connection_pool: Arc::new(Mutex::new(pool)),
        }
    }

    pub fn get_db_connection(&self) -> &SqliteConnection {
        let conn = *self
            .database_connection_pool
            .clone()
            .lock()
            .unwrap()
            .get()
            .unwrap();

        &conn
    }
}

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
    let state = AppState::new(get_connection_pool());

    run_migration(&mut state.get_db_connection());

    let mut pipeline = Pipeline::new();
    let tx = pipeline.open(&mut state.get_db_connection());

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

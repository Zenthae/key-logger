#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(unused)]

use db::Database;
use diesel::connection::SimpleConnection;
use key_logger::KeyLogger;

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
    let mut logger = KeyLogger::new();
    logger.start();
    std::thread::sleep(std::time::Duration::from_secs(2));
    logger.stop();
    logger.handle.unwrap().join();
}
// Move code in new() to start()

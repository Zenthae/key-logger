#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use key_logger::listener;

struct AppState {}

fn main() {
    let state = AppState {};

    let database_connexion_pool = DatabaseConnexionPool::new();

    let listener = Listener::new(database_connexion_pool.get());

    // Start front end and call Listener::close(); on shutdown event
}

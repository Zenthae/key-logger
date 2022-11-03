#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(unused)]

use chrono::prelude::*;
use db::{models::NewEvent, Database};

mod command;
mod db;
mod key_logger;
mod pipeline;

pub struct AppState {}

fn main() {
    let state = AppState {};

    let database = Database::new();
    database.query().insert_event(NewEvent {
        key_name: "A",
        event_time: &DateTime::default(),
    });

    tauri::Builder::default()
        .manage(state)
        .run(tauri::generate_context!())
        .expect("Failed to start tauri app");
}

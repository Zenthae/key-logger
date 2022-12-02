#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use recorder::Recorder;
use sea_orm::DatabaseConnection;
use tauri::WindowEvent;

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

    let recorder = Arc::new(Mutex::new(Recorder::new()));

    recorder.lock().unwrap().init();

    recorder.lock().unwrap().run();

    // let db1 = db.clone();

    // async move {
    //     database::query::event::get_by_id(&db1, 1).await;
    // }
    // .await;

    let state = AppState { database: db };

    tauri::Builder::default()
        .manage(state)
        .on_window_event(move |event| {
            if let WindowEvent::Focused(is_focused) = event.event() {
                if *is_focused {
                    recorder.lock().unwrap().stop();
                } else {
                    recorder.lock().unwrap().run();
                }
            };

            if let WindowEvent::CloseRequested { .. } = event.event() {
                recorder.lock().unwrap().stop();
            }
        })
        .invoke_handler(tauri::generate_handler![command::get_event_by_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

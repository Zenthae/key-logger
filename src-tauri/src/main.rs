#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use error::Result;
use pipeline::Pipeline;
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
async fn main() -> Result<()> {
    let db = database::get_connection().await;
    database::run_migrations(&db).await;

    let pipeline = Arc::new(Mutex::new(Pipeline::new()));

    let tx = pipeline.lock().unwrap().init(db.clone());

    let recorder = Arc::new(Mutex::new(Recorder::new()));
    recorder.lock().unwrap().init(tx);

    pipeline.lock().unwrap().run();
    recorder.lock().unwrap().run();

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
                pipeline.lock().unwrap().stop();
            }
        })
        .invoke_handler(tauri::generate_handler![command::get_event_by_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

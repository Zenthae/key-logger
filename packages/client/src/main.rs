#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use shared::recorder::Recorder;

fn main() {
    let mut recorder = Recorder::default();
    recorder.init();

    recorder.run();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

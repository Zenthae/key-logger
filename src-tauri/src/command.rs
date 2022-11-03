use tauri::State;

use crate::AppState;

#[tauri::command]
pub fn test(state: State<AppState>) -> String {
    "done !".to_string()
}

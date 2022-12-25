use crate::AppState;
use tauri::State;

use crate::database::query;

#[tauri::command]
pub async fn get_event_by_id(state: State<'_, AppState>) -> Result<entity::event::Model, ()> {
    Ok(query::event::get_by_id(&state.database, 1).await)
}

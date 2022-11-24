#[tauri::command]
pub async fn hello() -> String {
    println!("Working :");
    "Hello !".to_string()
}

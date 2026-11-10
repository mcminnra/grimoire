/// Responds to a frontend ping to verify the IPC bridge.
#[tauri::command]
fn ping() -> String {
    "pong".to_string()
}

/// Builds and runs the Tauri application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

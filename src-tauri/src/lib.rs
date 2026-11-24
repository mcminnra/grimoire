mod parse {
    pub mod error;
    pub mod log;
    pub mod parse;
    pub mod raw_log;
}

use parse::parse::LogLoadStatus;

#[tauri::command]
fn ping() -> String {
    "pong".to_string()
}

#[tauri::command]
fn load_game_logs() -> Result<LogLoadStatus, String> {
    parse::parse::load_game_logs().map_err(|e| e.to_string())
}

/// Builds and runs the Tauri application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping, load_game_logs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

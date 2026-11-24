mod parse {
    pub mod error;
    pub mod log;
    pub mod parse;
}
mod store;

use parse::log::LogView;
use store::AppState;
use tauri::{AppHandle, Emitter, State};

/// Event emitted with a log's `file_name` as payload whenever that log changes in state.
const LOG_CHANGED: &str = "log-changed";

/// Every log, sorted by file name. The first call loads the log directory.
#[tauri::command(async)]
fn get_logs(state: State<'_, AppState>) -> Result<Vec<LogView>, String> {
    let mut store = store::lock(&state).map_err(|e| e.to_string())?;
    let logs = store.logs().map_err(|e| e.to_string())?;
    Ok(logs.values().map(LogView::from).collect())
}

/// The log keyed by `file_name`, or `None` if there isn't one.
#[tauri::command(async)]
fn get_log(state: State<'_, AppState>, file_name: String) -> Result<Option<LogView>, String> {
    let mut store = store::lock(&state).map_err(|e| e.to_string())?;
    let log = store.get(&file_name).map_err(|e| e.to_string())?;
    Ok(log.map(LogView::from))
}

/// Writes `source` to the log's file, re-parses it, and emits [`LOG_CHANGED`].
#[tauri::command(async)]
fn save_log(
    app: AppHandle,
    state: State<'_, AppState>,
    file_name: String,
    source: String,
) -> Result<LogView, String> {
    let view = {
        let mut store = store::lock(&state).map_err(|e| e.to_string())?;
        let log = store.save(&file_name, source).map_err(|e| e.to_string())?;
        LogView::from(log)
    };
    app.emit(LOG_CHANGED, &file_name)
        .map_err(|e| e.to_string())?;
    Ok(view)
}

/// Builds and runs the Tauri application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![get_logs, get_log, save_log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

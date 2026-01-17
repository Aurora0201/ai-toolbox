use tauri::State;
use crate::AppState;
use crate::db::{self, TokenStat};

/// Command to retrieve token usage statistics from the database.
#[tauri::command]
pub fn get_token_stats(state: State<'_, AppState>) -> Result<Vec<TokenStat>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::get_aggregated_stats(&conn).map_err(|e| e.to_string())
}

/// Command to record token usage in the database.
#[tauri::command]
pub fn record_tokens(
    state: State<'_, AppState>,
    date: String,
    prompt: i64,
    completion: i64,
    model: String,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::record_tokens(&conn, &date, prompt, completion, &model).map_err(|e| e.to_string())
}

/// Command to clear all application data (currently just token statistics).
#[tauri::command]
pub fn clear_all_data(state: State<'_, AppState>) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::clear_database(&conn).map_err(|e| e.to_string())
}

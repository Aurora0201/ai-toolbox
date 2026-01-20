use tauri::State;
use crate::AppState;
use crate::db::{self, TokenStat};
use crate::error::{AppResult, AppError};
use log::{debug, warn};

/// Command to retrieve token usage statistics from the database.
#[tauri::command]
pub fn get_token_stats(state: State<'_, AppState>) -> AppResult<Vec<TokenStat>> {
    debug!("Fetching token statistics");
    let conn = state.db.lock().map_err(|e| AppError::Unknown(e.to_string()))?;
    db::get_aggregated_stats(&conn)
}

/// Command to record token usage in the database.
#[tauri::command]
pub fn record_tokens(
    state: State<'_, AppState>,
    date: String,
    prompt: i64,
    completion: i64,
    model: String,
) -> AppResult<()> {
    debug!("Recording token usage - Model: {}, Date: {}, Prompt: {}, Completion: {}", model, date, prompt, completion);
    let conn = state.db.lock().map_err(|e| AppError::Unknown(e.to_string()))?;
    db::record_tokens(&conn, &date, prompt, completion, &model)
}

/// Command to clear all application data (currently just token statistics).
#[tauri::command]
pub fn clear_all_data(state: State<'_, AppState>) -> AppResult<()> {
    warn!("Clearing all token statistics database");
    let conn = state.db.lock().map_err(|e| AppError::Unknown(e.to_string()))?;
    db::clear_database(&conn)
}

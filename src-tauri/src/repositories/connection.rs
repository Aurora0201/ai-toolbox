use rusqlite::Connection;
use tauri::Manager;
use crate::domain::error::{AppError, AppResult};
use log::info;

/// Initialize the SQLite database.
/// Creates the app data directory and the token_stats table if they don't exist.
pub fn init_db(app_handle: &tauri::AppHandle) -> AppResult<Connection> {
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| AppError::Unknown(format!("Failed to get app data directory: {}", e)))?;
    
    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir)
            .map_err(|e| AppError::Io(format!("Failed to create app data directory: {}", e)))?;
    }

    let db_path = app_dir.join("stats.db");
    let conn = Connection::open(&db_path)?; // Auto-converts to AppError::Database via From impl
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS token_stats (
            id INTEGER PRIMARY KEY,
            date TEXT NOT NULL,
            prompt_tokens INTEGER NOT NULL,
            completion_tokens INTEGER NOT NULL,
            model_name TEXT NOT NULL
        )",
        [],
    )?; // Auto-converts
    
    info!("Database initialized at {:?}", db_path);
    
    Ok(conn)
}

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use crate::error::{AppError, AppResult};
use log::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStat {
    pub date: String,
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
}

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

/// Record token usage for a specific model on a specific date.
pub fn record_tokens(conn: &Connection, date: &str, prompt: i64, completion: i64, model: &str) -> AppResult<()> {
    conn.execute(
        "INSERT INTO token_stats (date, prompt_tokens, completion_tokens, model_name) VALUES (?1, ?2, ?3, ?4)",
        params![date, prompt, completion, model],
    )?;
    Ok(())
}

/// Retrieve aggregated token statistics grouped by date.
pub fn get_aggregated_stats(conn: &Connection) -> AppResult<Vec<TokenStat>> {
    let mut stmt = conn.prepare(
        "SELECT date, SUM(prompt_tokens), SUM(completion_tokens) 
         FROM token_stats 
         GROUP BY date 
         ORDER BY date ASC",
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(TokenStat {
            date: row.get(0)?,
            prompt_tokens: row.get(1)?,
            completion_tokens: row.get(2)?,
        })
    })?;

    let mut stats = Vec::new();
    for row in rows {
        stats.push(row?);
    }
    Ok(stats)
}

/// Delete all records from the token_stats table.
pub fn clear_database(conn: &Connection) -> AppResult<()> {
    conn.execute("DELETE FROM token_stats", [])?;
    Ok(())
}

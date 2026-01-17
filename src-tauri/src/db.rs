use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStat {
    pub date: String,
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
}

/// Initialize the SQLite database.
/// Creates the app data directory and the token_stats table if they don't exist.
pub fn init_db(app_handle: &tauri::AppHandle) -> Result<Connection> {
    let app_dir = app_handle.path().app_data_dir()
        .context("Failed to get app data directory")?;
    
    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir).context("Failed to create app data directory")?;
    }

    let db_path = app_dir.join("stats.db");
    let conn = Connection::open(db_path).context("Failed to open database")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS token_stats (
            id INTEGER PRIMARY KEY,
            date TEXT NOT NULL,
            prompt_tokens INTEGER NOT NULL,
            completion_tokens INTEGER NOT NULL,
            model_name TEXT NOT NULL
        )",
        [],
    ).context("Failed to create table")?;
    
    Ok(conn)
}

/// Record token usage for a specific model on a specific date.
pub fn record_tokens(conn: &Connection, date: &str, prompt: i64, completion: i64, model: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO token_stats (date, prompt_tokens, completion_tokens, model_name) VALUES (?1, ?2, ?3, ?4)",
        params![date, prompt, completion, model],
    ).context("Failed to insert token stats")?;
    Ok(())
}

/// Retrieve aggregated token statistics grouped by date.
pub fn get_aggregated_stats(conn: &Connection) -> Result<Vec<TokenStat>> {
    let mut stmt = conn.prepare(
        "SELECT date, SUM(prompt_tokens), SUM(completion_tokens) 
         FROM token_stats 
         GROUP BY date 
         ORDER BY date ASC",
    ).context("Failed to prepare query")?;
    
    let rows = stmt.query_map([], |row| {
        Ok(TokenStat {
            date: row.get(0)?,
            prompt_tokens: row.get(1)?,
            completion_tokens: row.get(2)?,
        })
    }).context("Failed to execute query")?;

    let mut stats = Vec::new();
    for row in rows {
        stats.push(row.context("Failed to read row")?);
    }
    Ok(stats)
}
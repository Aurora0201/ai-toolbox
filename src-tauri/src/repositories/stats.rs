use rusqlite::{params, Connection};
use crate::domain::error::AppResult;
use crate::domain::models::TokenStat;

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

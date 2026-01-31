use crate::domain::error::{AppError, AppResult};
use chrono::{Duration, Local, NaiveDate};
use log::{info, warn};
use std::fs;
use tauri::Manager;

/// Cleans up log files older than 7 days in the application log directory.
pub fn cleanup_old_logs(app: &tauri::AppHandle) -> AppResult<()> {
    let log_dir = app
        .path()
        .app_log_dir()
        .map_err(|e| AppError::Io(format!("Failed to get log dir: {}", e)))?;

    if !log_dir.exists() {
        return Ok(());
    }

    let now = Local::now().date_naive();
    let retention_period = Duration::days(7);

    info!("Starting log cleanup in: {:?}", log_dir);

    match fs::read_dir(&log_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        // Target format: YYYY-MM-DD.log
                        if file_name.ends_with(".log") && file_name.len() == 14 {
                            let date_str = &file_name[..10];
                            if let Ok(file_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                                if now - file_date >= retention_period {
                                    info!("Deleting expired log file: {:?}", file_name);
                                    if let Err(e) = fs::remove_file(&path) {
                                        warn!("Failed to delete log file {:?}: {}", path, e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            warn!("Failed to read log directory: {}", e);
        }
    }

    Ok(())
}

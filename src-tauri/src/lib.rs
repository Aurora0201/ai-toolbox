mod db;
mod ollama;
mod commands;
mod error;

use ollama::OllamaClient;
use std::sync::Mutex;
use std::fs;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};
use log::{LevelFilter, info, warn};
use chrono::{Local, Duration, NaiveDate};

pub use error::{AppError, AppResult};

/// Application state shared across Tauri commands.
pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
    pub ollama: OllamaClient,
}

/// Cleans up log files older than 7 days in the application log directory.
fn cleanup_old_logs(app: &tauri::AppHandle) -> AppResult<()> {
    let log_dir = app.path().app_log_dir()
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

/// The main entry point for the Tauri application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_filename = Local::now().format("%Y-%m-%d.log").to_string();

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir { file_name: Some(log_filename) }),
                Target::new(TargetKind::Webview),
            ])
            // 1. Global Default Level Strategy
            .level(if cfg!(debug_assertions) { 
                LevelFilter::Debug 
            } else { 
                LevelFilter::Info 
            })
            // 2. Crate-specific Noise Filtering (White-listing / Black-listing approach)
            // Silence noisy libraries by forcing them to higher severity levels
            .level_for("tauri", LevelFilter::Info)
            .level_for("hyper", LevelFilter::Warn)
            .level_for("tao", LevelFilter::Warn)
            .level_for("wry", LevelFilter::Warn)
            .level_for("reqwest", LevelFilter::Warn)
            .level_for("html5gum", LevelFilter::Warn)
            // Ensure our own crate logs are visible (matches global default)
            .level_for("ai_toolbox_lib", if cfg!(debug_assertions) { 
                LevelFilter::Debug 
            } else { 
                LevelFilter::Info 
            })
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
            .build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            info!("Application starting up");

            // Cleanup old logs
            if let Err(e) = cleanup_old_logs(app.handle()) {
                eprintln!("Failed to cleanup old logs: {}", e);
            }

            // Initialize database and setup state
            let conn = db::init_db(app.handle())?;
            
            app.manage(AppState {
                db: Mutex::new(conn),
                ollama: OllamaClient::new("http://localhost:11434".to_string()),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::ollama::get_models,
            commands::ollama::get_running_models,
            commands::ollama::delete_model,
            commands::ollama::pull_model,
            commands::ollama::start_model,
            commands::ollama::unload_model,
            commands::ollama::generate_completion,
            commands::ollama::update_ollama_config,
            commands::ollama::check_connection,
            commands::db::get_token_stats,
            commands::db::record_tokens,
            commands::db::clear_all_data,
            commands::system::get_gpu_info,
            commands::system::set_log_level,
            commands::system::open_log_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
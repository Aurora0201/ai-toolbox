mod db;
mod ollama;
mod commands;
mod error;

use ollama::OllamaClient;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};
use log::LevelFilter;

pub use error::{AppError, AppResult};

/// Application state shared across Tauri commands.
pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
    pub ollama: OllamaClient,
}

/// The main entry point for the Tauri application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir { file_name: None }),
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
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepOne)
            .build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
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
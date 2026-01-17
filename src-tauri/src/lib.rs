mod db;
mod ollama;
mod commands;

use ollama::OllamaClient;
use std::sync::Mutex;
use tauri::Manager;

/// Application state shared across Tauri commands.
pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
    pub ollama: OllamaClient,
}

/// The main entry point for the Tauri application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize database and setup state
            let conn = db::init_db(app.handle())
                .map_err(|e| e.to_string())?;
            
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
            commands::system::get_gpu_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

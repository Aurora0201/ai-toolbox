use std::sync::Mutex;
use crate::clients::ollama::OllamaClient;

/// Application state shared across Tauri commands.
pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
    pub ollama: OllamaClient,
}

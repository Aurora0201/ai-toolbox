use crate::clients::ollama::OllamaClient;
use std::sync::Mutex;

/// Application state shared across Tauri commands.
pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
    pub ollama: OllamaClient,
}

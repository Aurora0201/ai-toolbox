use log::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Ollama error: {0}")]
    Ollama(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Custom From implementation to inject automatic logging
impl From<AppError> for tauri::ipc::InvokeError {
    fn from(err: AppError) -> Self {
        error!("Command Error: {}", err);
        tauri::ipc::InvokeError::from(err.to_string())
    }
}

// Implement From for common errors to simplify conversion
impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Network(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Unknown(format!("Serialization error: {}", err))
    }
}

pub type AppResult<T> = Result<T, AppError>;

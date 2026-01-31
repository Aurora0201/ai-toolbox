use crate::domain::error::{AppError, AppResult};
use crate::services::system::{self as system_service, GpuInfo};
use log::debug;
use tauri::Manager;

/// Command to set the global log level at runtime.
#[tauri::command]
pub async fn set_log_level(level: String) -> AppResult<()> {
    let level_filter = match level.to_lowercase().as_str() {
        "off" => log::LevelFilter::Off,
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };

    debug!("Setting log level to: {:?}", level_filter);
    log::set_max_level(level_filter);

    Ok(())
}

/// Command to open the application log directory in the file explorer.
#[tauri::command]
pub async fn open_log_dir(app: tauri::AppHandle) -> AppResult<()> {
    let log_path = app
        .path()
        .app_log_dir()
        .map_err(|e| AppError::Io(format!("Failed to get log dir: {}", e)))?;

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(log_path)
            .spawn()
            .map_err(|e| AppError::Io(format!("Failed to open log dir: {}", e)))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(log_path)
            .spawn()
            .map_err(|e| AppError::Io(format!("Failed to open log dir: {}", e)))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(log_path)
            .spawn()
            .map_err(|e| AppError::Io(format!("Failed to open log dir: {}", e)))?;
    }

    Ok(())
}

/// Command to get GPU info (Name, Total VRAM, Used VRAM).
/// Uses PowerShell on Windows for broad support (AMD/Intel/NVIDIA).
#[tauri::command]
pub async fn get_gpu_info() -> AppResult<GpuInfo> {
    system_service::get_gpu_info().await
}

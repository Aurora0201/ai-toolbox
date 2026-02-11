use std::time::Duration;
use tauri::{AppHandle, Manager, Emitter};
use tokio::time::sleep;
use crate::app::state::AppState;
use crate::services::system::{get_gpu_info, GpuInfo};
use crate::domain::models::RunningModel;

/// Starts a background monitoring task that polls Ollama and system resources.
/// It emits events to the frontend only when relevant changes are detected.
pub fn start_monitoring(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut last_running_models: Vec<RunningModel> = Vec::new();
        let mut last_gpu_info = GpuInfo::default();
        
        // Initial delay to let the app initialize
        sleep(Duration::from_secs(1)).await;

        loop {
            // 1. Monitor Running Models
            if let Some(state) = app_handle.try_state::<AppState>() {
                if let Ok(current_running) = state.ollama.get_running_models().await {
                    if current_running != last_running_models {
                        log::debug!("Running models changed, emitting event");
                        let _ = app_handle.emit("running-models-update", &current_running);
                        last_running_models = current_running;
                    }
                }
            }

            // 2. Monitor GPU Info
            if let Ok(current_gpu) = get_gpu_info().await {
                // We emit GPU info every poll because VRAM usage changes frequently
                // and it's used for real-time dashboard. 
                // However, we can check if it changed significantly if needed.
                if current_gpu != last_gpu_info {
                    let _ = app_handle.emit("gpu-info-update", &current_gpu);
                    last_gpu_info = current_gpu;
                }
            }

            // Poll every 5 seconds (same as previous frontend polling)
            sleep(Duration::from_secs(5)).await;
        }
    });
}

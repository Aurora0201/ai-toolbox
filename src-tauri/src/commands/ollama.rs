use tauri::{State, Window, Emitter};
use crate::AppState;
use crate::ollama::{Model, RunningModel};
use crate::error::AppResult;

/// Command to fetch all available models from Ollama.
#[tauri::command]
pub async fn get_models(state: State<'_, AppState>) -> AppResult<Vec<Model>> {
    state.ollama.get_tags().await
}

/// Command to fetch currently running models and their resource usage.
#[tauri::command]
pub async fn get_running_models(state: State<'_, AppState>) -> AppResult<Vec<RunningModel>> {
    state.ollama.get_running_models().await
}

/// Command to delete a model from Ollama.
#[tauri::command]
pub async fn delete_model(state: State<'_, AppState>, name: String) -> AppResult<()> {
    state.ollama.delete_model(name).await
}

/// Command to pull a new model from Ollama.
#[tauri::command]
pub async fn pull_model(
    state: State<'_, AppState>, 
    window: Window,
    name: String
) -> AppResult<()> {
    state.ollama.pull_model(name, |progress| async {
        let _ = window.emit("pull-progress", progress);
    }).await
}

/// Command to start (load) a model into memory.
#[tauri::command]
pub async fn start_model(state: State<'_, AppState>, name: String) -> AppResult<()> {
    state.ollama.start_model(name).await
}

/// Command to unload a model from memory to free VRAM.
#[tauri::command]
pub async fn unload_model(state: State<'_, AppState>, name: String) -> AppResult<()> {
    state.ollama.unload_model(name).await
}

/// Command to update the Ollama API endpoint.
#[tauri::command]
pub async fn update_ollama_config(state: State<'_, AppState>, endpoint: String) -> AppResult<()> {
    state.ollama.set_base_url(endpoint);
    Ok(())
}

/// Command to test the connection to an Ollama endpoint.
#[tauri::command]
pub async fn check_connection(state: State<'_, AppState>) -> AppResult<()> {
    state.ollama.get_tags().await.map(|_| ())
}

use tauri::{State, Window, Emitter};
use crate::AppState;
use crate::ollama::{Model, RunningModel};

/// Command to fetch all available models from Ollama.
#[tauri::command]
pub async fn get_models(state: State<'_, AppState>) -> Result<Vec<Model>, String> {
    state.ollama.get_tags().await.map_err(|e| e.to_string())
}

/// Command to fetch currently running models and their resource usage.
#[tauri::command]
pub async fn get_running_models(state: State<'_, AppState>) -> Result<Vec<RunningModel>, String> {
    state.ollama.get_running_models().await.map_err(|e| e.to_string())
}

/// Command to delete a model from Ollama.
#[tauri::command]
pub async fn delete_model(state: State<'_, AppState>, name: String) -> Result<(), String> {
    state.ollama.delete_model(name).await.map_err(|e| e.to_string())
}

/// Command to pull a new model from Ollama.
#[tauri::command]
pub async fn pull_model(
    state: State<'_, AppState>, 
    window: Window,
    name: String
) -> Result<(), String> {
    state.ollama.pull_model(name, |progress| async {
        let _ = window.emit("pull-progress", progress);
    }).await.map_err(|e| e.to_string())
}

/// Command to start (load) a model into memory.
#[tauri::command]
pub async fn start_model(state: State<'_, AppState>, name: String) -> Result<(), String> {
    state.ollama.start_model(name).await.map_err(|e| e.to_string())
}

/// Command to unload a model from memory to free VRAM.
#[tauri::command]
pub async fn unload_model(state: State<'_, AppState>, name: String) -> Result<(), String> {
    state.ollama.unload_model(name).await.map_err(|e| e.to_string())
}

/// Command to update the Ollama API endpoint.
#[tauri::command]
pub async fn update_ollama_config(state: State<'_, AppState>, endpoint: String) -> Result<(), String> {
    state.ollama.set_base_url(endpoint);
    Ok(())
}

/// Command to test the connection to an Ollama endpoint.
#[tauri::command]
pub async fn check_connection(state: State<'_, AppState>) -> Result<(), String> {
    state.ollama.get_tags().await.map(|_| ()).map_err(|e| e.to_string())
}
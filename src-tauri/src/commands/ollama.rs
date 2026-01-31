use crate::app::state::AppState;
use crate::domain::error::AppResult;
use crate::domain::models::{GenerateRequest, Model, RunningModel};
use log::{debug, info};
use tauri::{Emitter, State, Window};

/// Command to fetch all available models from Ollama.
#[tauri::command]
pub async fn get_models(state: State<'_, AppState>) -> AppResult<Vec<Model>> {
    debug!("Fetching available models from Ollama");
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
    info!("Deleting model: {}", name);
    state.ollama.delete_model(name).await
}

/// Command to pull a new model from Ollama.
#[tauri::command]
pub async fn pull_model(state: State<'_, AppState>, window: Window, name: String) -> AppResult<()> {
    info!("Starting pull for model: {}", name);
    state
        .ollama
        .pull_model(name.clone(), |progress| {
            let window_clone = window.clone();
            async move {
                let _ = window_clone.emit("pull-progress", progress);
            }
        })
        .await?;

    info!("Successfully pulled model: {}", name);
    Ok(())
}

/// Command to start (load) a model into memory.
#[tauri::command]
pub async fn start_model(state: State<'_, AppState>, name: String) -> AppResult<()> {
    info!("Starting model: {}", name);
    state.ollama.start_model(name).await
}

/// Command to unload a model from memory to free VRAM.
#[tauri::command]
pub async fn unload_model(state: State<'_, AppState>, name: String) -> AppResult<()> {
    info!("Unloading model: {}", name);
    state.ollama.unload_model(name).await
}

/// Command to generate a response from a model (Chat).
#[tauri::command]
pub async fn generate_completion(
    state: State<'_, AppState>,
    window: Window,
    request: GenerateRequest,
) -> AppResult<()> {
    info!("Generating completion for model: {}", request.model);
    state
        .ollama
        .generate_completion(request, |response| {
            let window_clone = window.clone();
            async move {
                let _ = window_clone.emit("chat-response", response);
            }
        })
        .await?;
    Ok(())
}

/// Command to update the Ollama API endpoint.
#[tauri::command]
pub async fn update_ollama_config(state: State<'_, AppState>, endpoint: String) -> AppResult<()> {
    info!("Updating Ollama config endpoint to: {}", endpoint);
    state.ollama.set_base_url(endpoint);
    Ok(())
}

/// Command to test the connection to an Ollama endpoint.
#[tauri::command]
pub async fn check_connection(state: State<'_, AppState>) -> AppResult<()> {
    debug!("Checking Ollama connection");
    state.ollama.get_tags().await.map(|_| ())
}

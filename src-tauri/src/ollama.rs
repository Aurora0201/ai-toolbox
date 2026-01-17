use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::{Result, Context};

/// Represents a model installed in Ollama.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub name: String,
    pub size: i64,
    pub modified_at: String,
}

/// Response structure for the Ollama tags API.
#[derive(Debug, Serialize, Deserialize)]
pub struct TagsResponse {
    pub models: Vec<Model>,
}

/// Represents a model currently running in memory.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunningModel {
    pub name: String,
    pub size: i64,
    pub size_vram: i64,
}

/// Response structure for the Ollama ps API.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResponse {
    pub models: Vec<RunningModel>,
}

/// Client for interacting with the Ollama API.
pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    /// Create a new OllamaClient with the specified base URL.
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Fetch the list of installed models.
    pub async fn get_tags(&self) -> Result<Vec<Model>> {
        let url = format!("{}/api/tags", self.base_url);
        let resp = self.client.get(url).send().await
            .context("Failed to send get_tags request")?;
        let tags: TagsResponse = resp.json().await
            .context("Failed to parse tags response")?;
        Ok(tags.models)
    }

    /// Fetch the list of currently running models.
    pub async fn get_running_models(&self) -> Result<Vec<RunningModel>> {
        let url = format!("{}/api/ps", self.base_url);
        let resp = self.client.get(url).send().await
            .context("Failed to send get_running_models request")?;
        let ps: ProcessResponse = resp.json().await
            .context("Failed to parse ps response")?;
        Ok(ps.models)
    }

    /// Delete an installed model.
    pub async fn delete_model(&self, name: String) -> Result<()> {
        let url = format!("{}/api/delete", self.base_url);
        self.client.delete(url)
            .json(&serde_json::json!({ "name": name }))
            .send().await
            .context("Failed to send delete_model request")?;
        Ok(())
    }

    /// Unload a model from memory (VRAM) by setting its keep_alive to 0.
    pub async fn unload_model(&self, name: String) -> Result<()> {
        let url = format!("{}/api/generate", self.base_url);
        self.client.post(url)
            .json(&serde_json::json!({
                "model": name,
                "keep_alive": 0
            }))
            .send().await
            .context("Failed to send unload_model request")?;
        Ok(())
    }
    
    /// Pull (download) a new model from the Ollama library.
    pub async fn pull_model(&self, name: String) -> Result<()> {
        let url = format!("{}/api/pull", self.base_url);
        self.client.post(url)
            .json(&serde_json::json!({ "name": name, "stream": false }))
            .send().await
            .context("Failed to send pull_model request")?;
        Ok(())
    }

    /// Start (preload) a model by sending an empty generate request.
    pub async fn start_model(&self, name: String) -> Result<()> {
        let url = format!("{}/api/generate", self.base_url);
        self.client.post(url)
            .json(&serde_json::json!({
                "model": name,
                "keep_alive": -1 // Keep loaded indefinitely
            }))
            .send().await
            .context("Failed to send start_model request")?;
        Ok(())
    }
}
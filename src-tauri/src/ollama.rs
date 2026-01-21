use serde::{Deserialize, Serialize};
use reqwest::Client;
use futures_util::StreamExt;
use std::sync::Mutex;
use crate::error::AppResult;

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

/// Advanced model parameters (options).
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GenerationOptions {
    pub num_keep: Option<u64>,
    pub seed: Option<i64>,
    pub num_predict: Option<i64>,
    pub top_k: Option<i64>,
    pub top_p: Option<f64>,
    pub min_p: Option<f64>,
    pub typical_p: Option<f64>,
    pub repeat_last_n: Option<i64>,
    pub temperature: Option<f64>,
    pub repeat_penalty: Option<f64>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub penalize_newline: Option<bool>,
    pub stop: Option<Vec<String>>,
    pub numa: Option<bool>,
    pub num_ctx: Option<u64>,
    pub num_batch: Option<u64>,
    pub num_gpu: Option<u64>,
    pub main_gpu: Option<u64>,
    pub low_vram: Option<bool>,
    pub f16_kv: Option<bool>,
    pub vocab_only: Option<bool>,
    pub use_mmap: Option<bool>,
    pub use_mlock: Option<bool>,
    pub num_thread: Option<u64>,
}

/// Request structure for generating a completion.
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub images: Option<Vec<String>>,
    pub format: Option<String>,
    pub options: Option<GenerationOptions>,
    pub system: Option<String>,
    pub template: Option<String>,
    pub stream: Option<bool>,
    pub raw: Option<bool>,
    pub keep_alive: Option<String>,
    pub context: Option<Vec<i64>>,
}

/// Response structure for a generation chunk.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub thinking: Option<String>, 
    pub done: bool,
    pub done_reason: Option<String>,
    pub context: Option<Vec<i64>>,
    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<u64>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<u64>,
    pub eval_duration: Option<u64>,
}

/// Payload sent to the frontend during chat generation.
#[derive(Debug, Serialize, Clone)]
pub struct ChatResponse {
    pub content: String,
    pub thinking: String,
    pub is_thinking: bool,
    pub done: bool,
    pub prompt_eval_count: u64,
    pub eval_count: u64,
}

/// Represents the progress of a model pull operation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PullProgress {
    pub status: Option<String>,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
    pub error: Option<String>,
}

/// Client for interacting with the Ollama API.
pub struct OllamaClient {
    client: Client,
    base_url: Mutex<String>,
}

impl OllamaClient {
    /// Create a new OllamaClient with the specified base URL.
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url: Mutex::new(base_url),
        }
    }

    /// Update the base URL for the Ollama API.
    pub fn set_base_url(&self, new_url: String) {
        if let Ok(mut url) = self.base_url.lock() {
            *url = new_url;
        }
    }

    /// Get the current base URL.
    fn get_base_url(&self) -> String {
        self.base_url.lock()
            .map(|url| url.clone())
            .unwrap_or_else(|_| "http://localhost:11434".to_string())
    }

    /// Fetch the list of installed models.
    pub async fn get_tags(&self) -> AppResult<Vec<Model>> {
        let url = format!("{}/api/tags", self.get_base_url());
        let resp = self.client.get(url).send().await?;
        let tags: TagsResponse = resp.json().await?;
        log::debug!("Fetched tags from Ollama: {:?}", tags);
        Ok(tags.models)
    }

    /// Fetch the list of currently running models.
    pub async fn get_running_models(&self) -> AppResult<Vec<RunningModel>> {
        let url = format!("{}/api/ps", self.get_base_url());
        let resp = self.client.get(url).send().await?;
        let ps: ProcessResponse = resp.json().await?;
        log::debug!("Fetched running models (ps) from Ollama: {:?}", ps);
        Ok(ps.models)
    }

    /// Delete an installed model.
    pub async fn delete_model(&self, name: String) -> AppResult<()> {
        let url = format!("{}/api/delete", self.get_base_url());
        self.client.delete(url)
            .json(&serde_json::json!({ "name": name }))
            .send().await?;
        Ok(())
    }

    /// Unload a model from memory (VRAM) by setting its keep_alive to 0.
    pub async fn unload_model(&self, name: String) -> AppResult<()> {
        let url = format!("{}/api/generate", self.get_base_url());
        self.client.post(url)
            .json(&serde_json::json!({
                "model": name,
                "keep_alive": 0
            }))
            .send().await?;
        Ok(())
    }
    
    /// Pull (download) a new model from the Ollama library with progress reporting.
    pub async fn pull_model<F, Fut>(&self, name: String, on_progress: F) -> AppResult<()> 
    where 
        F: Fn(PullProgress) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let url = format!("{}/api/pull", self.get_base_url());
        let resp = self.client.post(url)
            .json(&serde_json::json!({ "name": name, "stream": true }))
            .send().await?;

        if !resp.status().is_success() {
             return Err(anyhow::anyhow!("Ollama API Error: {}", resp.status()).into());
        }

        let mut stream = resp.bytes_stream();
        while let Some(item) = stream.next().await {
            let chunk = item?;
            // A chunk might contain multiple JSON objects separated by newlines
            let cursor = std::io::Cursor::new(chunk);
            let deserializer = serde_json::Deserializer::from_reader(cursor);
            let iter = deserializer.into_iter::<PullProgress>();
            
            for res in iter {
                match res {
                    Ok(progress) => {
                         if let Some(err_msg) = &progress.error {
                             return Err(anyhow::anyhow!("Pull failed: {}", err_msg).into());
                         }
                         on_progress(progress).await;
                    }
                    Err(e) => {
                        log::warn!("Failed to parse pull progress chunk: {}", e);
                    }
                }
            }
        }
        Ok(())
    }

    /// Start (preload) a model by sending an empty generate request.
    pub async fn start_model(&self, name: String) -> AppResult<()> {
        let url = format!("{}/api/generate", self.get_base_url());
        self.client.post(url)
            .json(&serde_json::json!({
                "model": name,
                "keep_alive": -1 // Keep loaded indefinitely
            }))
            .send().await?;
        Ok(())
    }

    /// Generate a completion for a prompt with streaming support and tag parsing.
    pub async fn generate_completion<F, Fut>(&self, request: GenerateRequest, on_event: F) -> AppResult<()>
    where
        F: Fn(ChatResponse) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let url = format!("{}/api/generate", self.get_base_url());
        let resp = self.client.post(url)
            .json(&request)
            .send().await?;

        let mut stream = resp.bytes_stream();
        
        // State for the tag parser
        let mut buffer = String::new();
        let mut in_think_block = false;

        while let Some(item) = stream.next().await {
            let chunk = item?;
            let cursor = std::io::Cursor::new(chunk);
            let deserializer = serde_json::Deserializer::from_reader(cursor);
            let iter = deserializer.into_iter::<GenerateResponse>();

            for response_res in iter {
                if let Ok(response) = response_res {
                     if response.done {
                        // Flush remaining buffer
                        let mut final_content = String::new();
                        let mut final_thinking = String::new();

                        if in_think_block {
                            final_thinking = buffer.clone();
                        } else {
                            final_content = buffer.clone();
                        }

                        on_event(ChatResponse {
                            content: final_content,
                            thinking: final_thinking,
                            is_thinking: false,
                            done: true,
                            prompt_eval_count: response.prompt_eval_count.unwrap_or(0),
                            eval_count: response.eval_count.unwrap_or(0),
                        }).await;
                        return Ok(());
                    }

                    // Handle direct 'thinking' field from Ollama (if present)
                    if let Some(think_str) = &response.thinking {
                        if !think_str.is_empty() {
                                on_event(ChatResponse {
                                content: "".to_string(),
                                thinking: think_str.clone(),
                                is_thinking: true,
                                done: false,
                                prompt_eval_count: 0,
                                eval_count: 0,
                            }).await;
                        }
                    }

                    // Append new content to buffer
                    buffer.push_str(&response.response);

                    // Process buffer for tags
                    let mut processed_content = String::new();
                    let mut processed_thinking = String::new();
                    let mut buffer_cleared_until = 0;

                    loop {
                        let current_slice = &buffer[buffer_cleared_until..];
                        
                        if in_think_block {
                            // Look for closing tag </think>
                            if let Some(idx) = current_slice.find("</think>") {
                                // Content before tag is thinking
                                processed_thinking.push_str(&current_slice[..idx]);
                                // Move past tag
                                buffer_cleared_until += idx + 8; // 8 is len of </think>
                                in_think_block = false;
                            } else {
                                // No closing tag found.
                                // We can safely emit everything EXCEPT the last few chars 
                                // which might be a partial </think> tag.
                                let mut safe_len = current_slice.len().saturating_sub(7); // </think is 7 chars (partial)
                                while !current_slice.is_char_boundary(safe_len) {
                                    safe_len -= 1;
                                }
                                
                                if safe_len > 0 {
                                    processed_thinking.push_str(&current_slice[..safe_len]);
                                    buffer_cleared_until += safe_len;
                                }
                                break;
                            }
                        } else {
                            // Look for opening tag <think>
                            if let Some(idx) = current_slice.find("<think>") {
                                // Content before tag is normal content
                                processed_content.push_str(&current_slice[..idx]);
                                // Move past tag
                                buffer_cleared_until += idx + 7; // 7 is len of <think>
                                in_think_block = true;
                            } else {
                                // No opening tag found.
                                // Emit everything except partial <think
                                let mut safe_len = current_slice.len().saturating_sub(6); // <think is 6 chars
                                while !current_slice.is_char_boundary(safe_len) {
                                    safe_len -= 1;
                                }

                                if safe_len > 0 {
                                    processed_content.push_str(&current_slice[..safe_len]);
                                    buffer_cleared_until += safe_len;
                                }
                                break;
                            }
                        }
                    }

                    // Remove processed part from buffer
                    if buffer_cleared_until > 0 {
                        buffer = buffer[buffer_cleared_until..].to_string();
                    }

                    // Emit if we have something
                    if !processed_content.is_empty() || !processed_thinking.is_empty() {
                            on_event(ChatResponse {
                            content: processed_content,
                            thinking: processed_thinking,
                            is_thinking: in_think_block || response.thinking.is_some(),
                            done: false,
                            prompt_eval_count: 0,
                            eval_count: 0,
                        }).await;
                    }
                }
            }
        }
        Ok(())
    }
}
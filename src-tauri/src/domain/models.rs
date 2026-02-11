use serde::{Deserialize, Serialize};

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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

/// DB Stat struct
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStat {
    pub date: String,
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
}

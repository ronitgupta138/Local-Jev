//! Domain models, request/response DTOs, and error types for local-jev.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Request payload for `/v1/systemone/choice`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceRequest {
    /// Contextual environment state, prompt history, or code diff.
    pub state: String,
    /// Fast decision query (e.g. "Which tool should be invoked next?").
    pub question: String,
    /// Candidate options to select from.
    pub options: Vec<String>,
    /// Softmax temperature scaling (default: 0.1 for sharp decisions).
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

fn default_temperature() -> f32 {
    0.1
}

/// Individual option candidate probability distribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionProbability {
    pub option: String,
    pub probability: f32,
    pub logit: f32,
}

/// Response payload for `/v1/systemone/choice`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceResponse {
    /// Selected optimal option.
    pub choice: String,
    /// Calibrated confidence score (0.0 - 1.0).
    pub confidence: f32,
    /// Full normalized probability distribution over candidate options.
    pub probabilities: Vec<OptionProbability>,
    /// Wall-clock inference latency in milliseconds.
    pub latency_ms: f64,
    /// Whether response was served from cache.
    pub cached: bool,
}

/// Request payload for `/v1/systemone/score`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRequest {
    pub text: String,
    pub criteria: String,
}

/// Response payload for `/v1/systemone/score`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreResponse {
    pub score: f32,
    pub latency_ms: f64,
}

/// Request payload for `/v1/systemone/noul`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoulRequest {
    pub state: String,
    pub candidates: Vec<String>,
}

/// Response payload for `/v1/systemone/noul`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoulResponse {
    pub selected: String,
    pub category: String,
    pub confidence: f32,
    pub latency_ms: f64,
}

/// Request payload for fast agent route selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRequest {
    pub prompt: String,
    pub routes: Vec<String>,
}

/// Response payload for fast agent route selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponse {
    pub route: String,
    pub confidence: f32,
    pub latency_ms: f64,
}

/// Model metadata response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub engine: String,
    pub latency_target_ms: f64,
}

/// Errors raised across local-jev engine and server.
#[derive(Debug, Error)]
pub enum JevError {
    #[error("Options list cannot be empty")]
    EmptyOptions,

    #[error("Inference engine error: {0}")]
    Inference(String),

    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

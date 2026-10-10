//! High-Concurrency Axum REST Server Compatible with TypeSafe Jev API.

use crate::domain::{
    ChoiceRequest, ChoiceResponse, ModelInfo, NoulRequest, NoulResponse, RouteRequest,
    RouteResponse, ScoreRequest, ScoreResponse,
};
use crate::engine::JevDecisionEngine;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub struct AppState {
    pub engine: JevDecisionEngine,
}

pub fn create_router(engine: JevDecisionEngine) -> Router {
    let state = Arc::new(AppState { engine });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health_handler))
        .route("/v1/models", get(models_handler))
        .route("/v1/systemone/choice", post(choice_handler))
        .route("/v1/systemone/score", post(score_handler))
        .route("/v1/systemone/noul", post(noul_handler))
        .route("/v1/systemone/route", post(route_handler))
        .layer(cors)
        .with_state(state)
}

async fn health_handler() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "local-jev",
        "version": "1.0.0",
        "offline": true
    }))
}

async fn models_handler() -> impl IntoResponse {
    Json(json!({
        "data": [
            ModelInfo {
                id: "typesafe-jev-systemone".to_string(),
                name: "TypeSafe Jev System One (Local CPU Drop-In)".to_string(),
                engine: "Single-Forward-Pass-Logit-Masking".to_string(),
                latency_target_ms: 18.0,
            }
        ]
    }))
}

async fn choice_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChoiceRequest>,
) -> Result<Json<ChoiceResponse>, (StatusCode, String)> {
    state
        .engine
        .decide_choice(&payload)
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
}

async fn score_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ScoreRequest>,
) -> Result<Json<ScoreResponse>, (StatusCode, String)> {
    state
        .engine
        .score_text(&payload)
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
}

async fn noul_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NoulRequest>,
) -> Result<Json<NoulResponse>, (StatusCode, String)> {
    state
        .engine
        .classify_noul(&payload)
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
}

async fn route_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RouteRequest>,
) -> Result<Json<RouteResponse>, (StatusCode, String)> {
    state
        .engine
        .route_prompt(&payload)
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
}

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use local_jev::domain::{ChoiceRequest, NoulRequest, RouteRequest, ScoreRequest};
use local_jev::engine::JevDecisionEngine;
use local_jev::server::create_router;
use tower::ServiceExt;

#[test]
fn test_choice_selection_accuracy() {
    let engine = JevDecisionEngine::new();
    let req = ChoiceRequest {
        state: "The compiler returned error: unresolved import `crate::auth::User`".into(),
        question: "Which tool should be invoked to search the codebase?".into(),
        options: vec![
            "grep".into(),
            "reboot".into(),
            "curl".into(),
            "npm install".into(),
        ],
        temperature: 0.1,
    };

    let res = engine.decide_choice(&req).unwrap();
    assert_eq!(res.choice, "grep");
    assert!(res.confidence > 0.6);
    assert!(res.latency_ms < 20.0, "Latency exceeded 20ms: {}ms", res.latency_ms);
}

#[test]
fn test_semantic_score_relevance() {
    let engine = JevDecisionEngine::new();
    let req = ScoreRequest {
        text: "Resolve memory corruption bug in Greybus staging audio subsystem".into(),
        criteria: "Kernel audio bug fix".into(),
    };

    let res = engine.score_text(&req).unwrap();
    assert!(res.score > 0.6, "Expected high relevance score: {}", res.score);
    assert!(res.latency_ms < 20.0);
}

#[test]
fn test_agent_noul_classification() {
    let engine = JevDecisionEngine::new();
    let req = NoulRequest {
        state: "Checking repository structure".into(),
        candidates: vec![
            "grep pattern in files".into(),
            "write new feature file".into(),
            "think about architecture".into(),
        ],
    };

    let res = engine.classify_noul(&req).unwrap();
    assert_eq!(res.category, "OBSERVATION");
}

#[test]
fn test_fast_route_classification() {
    let engine = JevDecisionEngine::new();
    let req = RouteRequest {
        prompt: "Please trigger zero-downtime deployment to production AWS cluster".into(),
        routes: vec![
            "deploy_pipeline".into(),
            "bug_triage".into(),
            "code_review".into(),
        ],
    };

    let res = engine.route_prompt(&req).unwrap();
    assert_eq!(res.route, "deploy_pipeline");
    assert!(res.confidence > 0.5);
}

#[tokio::test]
async fn test_typesafe_http_server_endpoints() {
    let engine = JevDecisionEngine::new();
    let app = create_router(engine);

    // 1. Health endpoint
    let response = app
        .clone()
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 2. Models endpoint
    let response = app
        .clone()
        .oneshot(Request::builder().uri("/v1/models").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. /v1/systemone/choice endpoint
    let choice_payload = serde_json::json!({
        "state": "Need to find all instances of function login()",
        "question": "Which search command should be executed?",
        "options": ["grep -rn login", "git commit", "docker stop"],
        "temperature": 0.1
    });

    let req = Request::builder()
        .method("POST")
        .uri("/v1/systemone/choice")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&choice_payload).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let choice_json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(choice_json["choice"], "grep -rn login");
    assert_eq!(choice_json["cached"], false);

    // 4. /v1/systemone/batch endpoint
    let batch_payload = serde_json::json!({
        "requests": [
            {
                "state": "Compiler unresolved import",
                "question": "Tool?",
                "options": ["cargo check", "reboot"],
                "temperature": 0.1
            }
        ]
    });

    let req = Request::builder()
        .method("POST")
        .uri("/v1/systemone/batch")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&batch_payload).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 5. /v1/chat/completions endpoint
    let chat_payload = serde_json::json!({
        "model": "claude-3-5-sonnet",
        "messages": [
            { "role": "user", "content": "Please inspect README.md file" }
        ],
        "tools": [
            {
                "type": "function",
                "function": { "name": "read_file" }
            }
        ]
    });

    let req = Request::builder()
        .method("POST")
        .uri("/v1/chat/completions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&chat_payload).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let chat_res: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(chat_res["speculative"], true);

    // 6. /metrics endpoint
    let req = Request::builder().uri("/metrics").body(Body::empty()).unwrap();
    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

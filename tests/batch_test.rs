use local_jev::domain::{BatchChoiceRequest, ChoiceRequest};
use local_jev::engine::JevDecisionEngine;

#[test]
fn test_batch_choice_engine() {
    let engine = JevDecisionEngine::new();

    let reqs = vec![
        ChoiceRequest {
            state: "User asks to list files in directory".to_string(),
            question: "Which tool to use?".to_string(),
            options: vec!["ls".to_string(), "rm".to_string(), "reboot".to_string()],
            temperature: 0.1,
        },
        ChoiceRequest {
            state: "Found bug in login handler".to_string(),
            question: "Which file should we inspect?".to_string(),
            options: vec!["src/auth.rs".to_string(), "Cargo.lock".to_string()],
            temperature: 0.1,
        },
        ChoiceRequest {
            state: "Deploy release to production".to_string(),
            question: "Which agent to route to?".to_string(),
            options: vec!["deploy_agent".to_string(), "triage_agent".to_string()],
            temperature: 0.1,
        },
    ];

    let batch_req = BatchChoiceRequest { requests: reqs };
    let batch_res = engine.decide_batch(&batch_req).expect("Batch processing should succeed");

    assert_eq!(batch_res.count, 3);
    assert_eq!(batch_res.responses.len(), 3);
    assert_eq!(batch_res.responses[0].choice, "ls");
    assert_eq!(batch_res.responses[1].choice, "src/auth.rs");
    assert_eq!(batch_res.responses[2].choice, "deploy_agent");
    assert!(batch_res.total_latency_ms > 0.0);
    assert!(batch_res.throughput_qps > 0.0);
}

#[test]
fn test_empty_batch_returns_bad_request() {
    let engine = JevDecisionEngine::new();
    let batch_req = BatchChoiceRequest { requests: vec![] };
    let res = engine.decide_batch(&batch_req);
    assert!(res.is_err());
}

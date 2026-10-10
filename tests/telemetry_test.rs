use local_jev::domain::ChoiceRequest;
use local_jev::engine::JevDecisionEngine;
use local_jev::telemetry::MicroBatchScheduler;
use std::sync::Arc;

#[test]
fn test_microbatch_scheduler_concurrent_queries() {
    let engine = Arc::new(JevDecisionEngine::new());
    let scheduler = MicroBatchScheduler::new(engine);

    let requests = vec![
        ChoiceRequest {
            state: "Looking for database configuration file".into(),
            question: "Select target file".into(),
            options: vec!["db.rs".into(), "routes.rs".into()],
            temperature: 0.1,
        },
        ChoiceRequest {
            state: "Looking for route definitions".into(),
            question: "Select target file".into(),
            options: vec!["db.rs".into(), "routes.rs".into()],
            temperature: 0.1,
        },
    ];

    let responses = scheduler.execute_batch(&requests).unwrap();
    assert_eq!(responses.len(), 2);
    assert_eq!(responses[0].choice, "db.rs");
    assert_eq!(responses[1].choice, "routes.rs");

    let snap = scheduler.metrics.snapshot();
    assert_eq!(snap.total_decisions, 2);
}

#[test]
fn test_prometheus_metrics_export() {
    let engine = Arc::new(JevDecisionEngine::new());
    let scheduler = MicroBatchScheduler::new(engine);

    scheduler.metrics.record_decision(1.2, true);
    scheduler.metrics.record_decision(1.5, false);

    let prometheus = scheduler.metrics.format_prometheus();
    assert!(prometheus.contains("jev_decisions_total 2"));
    assert!(prometheus.contains("jev_cache_hits_total 1"));
    assert!(prometheus.contains("jev_cache_hit_ratio 0.5000"));
}

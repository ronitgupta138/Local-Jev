use local_jev::cascade_evaluator::{CascadeConfidenceEvaluator, CascadeVerdict};

#[test]
fn test_cascade_confidence_boundaries() {
    let evaluator = CascadeConfidenceEvaluator::new(0.85, 0.40);

    assert_eq!(evaluator.evaluate(0.95), CascadeVerdict::EarlyExitFastModel);
    assert_eq!(evaluator.evaluate(0.85), CascadeVerdict::EarlyExitFastModel);
    assert_eq!(evaluator.evaluate(0.25), CascadeVerdict::EscalateToFrontier);
    assert_eq!(evaluator.evaluate(0.40), CascadeVerdict::EscalateToFrontier);
    assert_eq!(evaluator.evaluate(0.60), CascadeVerdict::AmbiguousDefer);
}

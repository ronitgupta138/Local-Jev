use local_jev::cascade::CascadeEngine;
use local_jev::domain::ChoiceRequest;

#[test]
fn test_cascade_high_confidence_short_circuit() {
    let cascade = CascadeEngine::new(0.60);
    let req = ChoiceRequest {
        state: "Unresolved import crate::auth::User".into(),
        question: "Which search tool should be invoked?".into(),
        options: vec!["grep".into(), "reboot".into(), "curl".into()],
        temperature: 0.1,
    };

    let decision = cascade.decide(&req).unwrap();
    assert!(decision.is_speculative);
    assert!(!decision.requires_fallback);
    assert_eq!(decision.response.choice, "grep");
}

#[test]
fn test_cascade_ambiguous_fallback_flagging() {
    // Set a very high threshold (0.999) to force fallback flagging
    let cascade = CascadeEngine::new(0.999);
    let req = ChoiceRequest {
        state: "Ambiguous code condition".into(),
        question: "Select action".into(),
        options: vec!["option_1".into(), "option_2".into()],
        temperature: 1.0,
    };

    let decision = cascade.decide(&req).unwrap();
    assert!(!decision.is_speculative);
    assert!(decision.requires_fallback);
}

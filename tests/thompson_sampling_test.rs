use local_jev::thompson_sampling::ThompsonSampler;

#[test]
fn test_thompson_sampler_learning() {
    let mut sampler = ThompsonSampler::new();
    sampler.add_arm("model_a");
    sampler.add_arm("model_b");

    // Initially equal expected values
    assert_eq!(sampler.expected_value("model_a"), 0.5);
    assert_eq!(sampler.expected_value("model_b"), 0.5);

    // Record 5 successes for model_a, 5 failures for model_b
    for _ in 0..5 {
        sampler.record_reward("model_a", true);
        sampler.record_reward("model_b", false);
    }

    assert!(sampler.expected_value("model_a") > 0.8);
    assert!(sampler.expected_value("model_b") < 0.2);
    assert_eq!(sampler.best_arm(), Some("model_a".to_string()));
}

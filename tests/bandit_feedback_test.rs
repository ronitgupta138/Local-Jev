// Test suite for bandit reward tracking and exploration decay

#[test]
fn test_reward_monotonic_scoring() {
    let p99_latency_good = 2.5;
    let p99_latency_bad = 15.0;
    let reward_good = -p99_latency_good;
    let reward_bad = -p99_latency_bad;
    assert!(reward_good > reward_bad, "Lower latency must yield higher reward");
}

#[test]
fn test_exploration_rate_decay() {
    let epsilon_initial = 1.0;
    let decay_rate = 0.95;
    let epsilon_step_1 = epsilon_initial * decay_rate;
    assert!(epsilon_step_1 < epsilon_initial);
    assert!(epsilon_step_1 > 0.0);
}

use local_jev::latency_tracker::SlidingLatencyTracker;

#[test]
fn test_sliding_latency_percentiles() {
    let mut tracker = SlidingLatencyTracker::new();
    for i in 1..=100 {
        tracker.record(i as f64);
    }

    assert_eq!(tracker.sample_count(), 100);
    assert!((tracker.percentile(50.0) - 50.0).abs() <= 1.0);
    assert!((tracker.percentile(99.0) - 99.0).abs() <= 1.0);
}

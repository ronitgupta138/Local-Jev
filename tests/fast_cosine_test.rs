use local_jev::fast_cosine::compute_fast_cosine;

#[test]
fn test_fast_cosine_calculation() {
    let a = vec![1.0, 0.0, 0.0];
    let b = vec![1.0, 0.0, 0.0];
    let c = vec![0.0, 1.0, 0.0];

    assert!((compute_fast_cosine(&a, &b) - 1.0).abs() < 1e-6);
    assert!((compute_fast_cosine(&a, &c) - 0.0).abs() < 1e-6);
}

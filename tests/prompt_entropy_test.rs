use local_jev::prompt_entropy::calculate_text_entropy;

#[test]
fn test_calculate_text_entropy() {
    assert_eq!(calculate_text_entropy(""), 0.0);
    assert_eq!(calculate_text_entropy("aaaaa"), 0.0);
    let ent = calculate_text_entropy("abcdefgh");
    assert!((ent - 3.0).abs() < 1e-5); // 8 distinct chars = log2(8) = 3 bits
}

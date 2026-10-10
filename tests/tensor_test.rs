use local_jev::tensor::QuantizedLinear;

#[test]
fn test_quantized_linear_forward() {
    let layer = QuantizedLinear::new(8, 4);
    let input = vec![1.0, 0.5, -0.5, 2.0, 1.5, -1.0, 0.0, 3.0];
    let output = layer.forward(&input);

    assert_eq!(output.len(), 4);
    // Deterministic non-zero values
    assert!(output.iter().any(|&v| v != 0.0));
}

#[test]
fn test_quantized_linear_unaligned_dimensions() {
    let layer = QuantizedLinear::new(7, 3); // 7 is not divisible by 4 (tests remainder loop)
    let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let output = layer.forward(&input);

    assert_eq!(output.len(), 3);
}

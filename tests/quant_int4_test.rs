use local_jev::quant_int4::Int4Vector;

#[test]
fn test_int4_roundtrip_quantization() {
    let floats = vec![0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0];
    let int4 = Int4Vector::quantize(&floats);
    assert_eq!(int4.packed_bytes.len(), 3); // 6 values fit in 3 bytes

    let recovered = int4.dequantize();
    assert_eq!(recovered.len(), 6);
    for (orig, rec) in floats.iter().zip(recovered.iter()) {
        assert!((orig - rec).abs() <= 0.35, "Error must be bounded by quantization scale");
    }
}

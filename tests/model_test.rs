use local_jev::model::{ModelBackbone, ModelBackboneMode};
use tempfile::NamedTempFile;

#[test]
fn test_model_backbone_default_mode() {
    let backbone = ModelBackbone::new();
    assert_eq!(*backbone.mode(), ModelBackboneMode::ZeroWeightFast);
    assert!(!backbone.is_neural());
}

#[test]
fn test_model_backbone_file_loader() {
    let tmp = NamedTempFile::new().unwrap();
    let backbone = ModelBackbone::from_file(tmp.path()).unwrap();
    assert!(backbone.is_neural());
    assert!(matches!(backbone.mode(), ModelBackboneMode::NeuralWeights(_)));
}

#[test]
fn test_model_backbone_missing_file_error() {
    let res = ModelBackbone::from_file("/tmp/nonexistent_model_file.onnx");
    assert!(res.is_err());
}

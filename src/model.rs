//! Adaptive model weights backbone loader for quantized micro-models (ONNX / GGUF).

use crate::domain::JevError;
use std::path::{Path, PathBuf};

/// Model backbone inference mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelBackboneMode {
    /// Zero-weight lexical-semantic heuristic engine (<1ms CPU latency).
    ZeroWeightFast,
    /// Neural tensor weights loaded from disk (ONNX / GGUF).
    NeuralWeights(PathBuf),
}

/// Model backbone configuration manager.
pub struct ModelBackbone {
    mode: ModelBackboneMode,
}

impl ModelBackbone {
    pub fn new() -> Self {
        Self {
            mode: ModelBackboneMode::ZeroWeightFast,
        }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, JevError> {
        let p = path.as_ref().to_path_buf();
        if !p.exists() {
            return Err(JevError::Inference(format!(
                "Model weight file not found at: {}",
                p.display()
            )));
        }
        Ok(Self {
            mode: ModelBackboneMode::NeuralWeights(p),
        })
    }

    pub fn mode(&self) -> &ModelBackboneMode {
        &self.mode
    }

    pub fn is_neural(&self) -> bool {
        matches!(self.mode, ModelBackboneMode::NeuralWeights(_))
    }
}

impl Default for ModelBackbone {
    fn default() -> Self {
        Self::new()
    }
}

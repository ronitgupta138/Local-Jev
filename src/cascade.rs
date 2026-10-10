//! Speculative cascade router: short-circuits high-confidence queries and flags ambiguous prompts.

use crate::domain::{ChoiceRequest, ChoiceResponse, JevError};
use crate::engine::JevDecisionEngine;

/// Outcome of a speculative cascade evaluation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CascadeDecision {
    pub response: ChoiceResponse,
    pub is_speculative: bool,
    pub requires_fallback: bool,
}

pub struct CascadeEngine {
    engine: JevDecisionEngine,
    confidence_threshold: f32,
}

impl CascadeEngine {
    pub fn new(confidence_threshold: f32) -> Self {
        Self {
            engine: JevDecisionEngine::new(),
            confidence_threshold,
        }
    }

    /// Evaluate query: short-circuit if confidence >= threshold, flag fallback if ambiguous.
    pub fn decide(&self, req: &ChoiceRequest) -> Result<CascadeDecision, JevError> {
        let res = self.engine.decide_choice(req)?;
        let is_high_confidence = res.confidence >= self.confidence_threshold;

        Ok(CascadeDecision {
            response: res,
            is_speculative: is_high_confidence,
            requires_fallback: !is_high_confidence,
        })
    }
}

impl Default for CascadeEngine {
    fn default() -> Self {
        Self::new(0.65)
    }
}

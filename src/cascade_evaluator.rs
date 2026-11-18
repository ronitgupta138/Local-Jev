#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CascadeVerdict {
    EarlyExitFastModel,
    EscalateToFrontier,
    AmbiguousDefer,
}

pub struct CascadeConfidenceEvaluator {
    pub high_confidence_threshold: f32,
    pub low_confidence_threshold: f32,
}

impl CascadeConfidenceEvaluator {
    pub fn new(high: f32, low: f32) -> Self {
        Self {
            high_confidence_threshold: high,
            low_confidence_threshold: low,
        }
    }

    pub fn evaluate(&self, confidence_score: f32) -> CascadeVerdict {
        if confidence_score >= self.high_confidence_threshold {
            CascadeVerdict::EarlyExitFastModel
        } else if confidence_score <= self.low_confidence_threshold {
            CascadeVerdict::EscalateToFrontier
        } else {
            CascadeVerdict::AmbiguousDefer
        }
    }
}

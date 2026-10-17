//! Online preference learning and contextual bandit feedback engine.

use std::collections::HashMap;
use std::sync::RwLock;

/// Outcome feedback recorded by an agent after decision execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackOutcome {
    Success,
    Failure,
}

/// Bandit prior tracking success/failure counts for option affinities.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct OptionPrior {
    pub successes: u32,
    pub failures: u32,
}

impl OptionPrior {
    pub fn update(&mut self, outcome: FeedbackOutcome) {
        match outcome {
            FeedbackOutcome::Success => self.successes += 1,
            FeedbackOutcome::Failure => self.failures += 1,
        }
    }

    /// Upper Confidence Bound (UCB) bonus score.
    pub fn score_bonus(&self, total_trials: u32) -> f32 {
        let total = self.successes + self.failures;
        if total == 0 {
            return 1.5; // Optimism in the face of uncertainty
        }
        let win_rate = self.successes as f32 / total as f32;
        let exploration = (2.0 * (total_trials as f32 + 1.0).ln() / total as f32).sqrt();
        win_rate * 2.0 + exploration
    }
}

/// In-memory contextual bandit learner adjusting option logits based on real execution.
#[derive(Debug, Default)]
pub struct BanditLearner {
    priors: RwLock<HashMap<String, OptionPrior>>,
}

impl BanditLearner {
    pub fn new() -> Self {
        Self {
            priors: RwLock::new(HashMap::new()),
        }
    }

    /// Record execution feedback for a given context and choice.
    pub fn record_feedback(&self, context: &str, choice: &str, outcome: FeedbackOutcome) {
        let key = format!("{}:{}", context.trim().to_lowercase(), choice.trim().to_lowercase());
        if let Ok(mut lock) = self.priors.write() {
            lock.entry(key).or_default().update(outcome);
        }
    }

    /// Compute preference bonus for candidate option.
    pub fn get_preference_bonus(&self, context: &str, choice: &str) -> f32 {
        let key = format!("{}:{}", context.trim().to_lowercase(), choice.trim().to_lowercase());
        if let Ok(lock) = self.priors.read()
            && let Some(prior) = lock.get(&key) {
                return prior.score_bonus(lock.len() as u32);
        }
        0.0
    }

    /// Atomically save bandit state to a JSON checkpoint on disk.
    pub fn save_checkpoint(&self, path: &std::path::Path) -> Result<(), crate::domain::JevError> {
        let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let mut tmp = tempfile::NamedTempFile::new_in(parent)
            .map_err(|e| crate::domain::JevError::Io(e))?;

        let map = self.priors.read()
            .map_err(|_| crate::domain::JevError::Inference("Failed to acquire read lock on bandit priors".to_string()))?
            .clone();

        serde_json::to_writer_pretty(&mut tmp, &map)?;
        tmp.persist(path).map_err(|e| crate::domain::JevError::Io(e.error))?;
        Ok(())
    }

    /// Load bandit priors from a persistent checkpoint file.
    pub fn load_checkpoint(&self, path: &std::path::Path) -> Result<usize, crate::domain::JevError> {
        if !path.exists() {
            return Ok(0);
        }
        let file = std::fs::File::open(path)?;
        let map: HashMap<String, OptionPrior> = serde_json::from_reader(file)?;
        let count = map.len();
        if let Ok(mut lock) = self.priors.write() {
            *lock = map;
        }
        Ok(count)
    }
}

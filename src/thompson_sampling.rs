use std::collections::HashMap;

/// Thompson Sampling multi-armed bandit state.
#[derive(Debug, Clone, Default)]
pub struct ThompsonSampler {
    // Arm name -> (alpha: successes, beta: failures)
    arms: HashMap<String, (u32, u32)>,
}

impl ThompsonSampler {
    pub fn new() -> Self {
        Self { arms: HashMap::new() }
    }

    pub fn add_arm(&mut self, name: impl Into<String>) {
        self.arms.entry(name.into()).or_insert((1, 1)); // Uniform Beta(1,1) prior
    }

    pub fn record_reward(&mut self, name: &str, success: bool) {
        let entry = self.arms.entry(name.to_string()).or_insert((1, 1));
        if success {
            entry.0 += 1;
        } else {
            entry.1 += 1;
        }
    }

    pub fn expected_value(&self, name: &str) -> f64 {
        if let Some(&(alpha, beta)) = self.arms.get(name) {
            alpha as f64 / (alpha + beta) as f64
        } else {
            0.5
        }
    }

    pub fn best_arm(&self) -> Option<String> {
        self.arms.iter()
            .max_by(|a, b| {
                let mean_a = a.1 .0 as f64 / (a.1 .0 + a.1 .1) as f64;
                let mean_b = b.1 .0 as f64 / (b.1 .0 + b.1 .1) as f64;
                mean_a.partial_cmp(&mean_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(name, _)| name.clone())
    }
}

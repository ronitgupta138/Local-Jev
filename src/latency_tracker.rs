/// Sliding window latency recorder for P50 and P99 monitoring.
#[derive(Debug, Clone, Default)]
pub struct SlidingLatencyTracker {
    latencies: Vec<f64>,
}

impl SlidingLatencyTracker {
    pub fn new() -> Self {
        Self { latencies: Vec::new() }
    }

    pub fn record(&mut self, latency_ms: f64) {
        self.latencies.push(latency_ms);
    }

    pub fn sample_count(&self) -> usize {
        self.latencies.len()
    }

    pub fn percentile(&self, p: f64) -> f64 {
        if self.latencies.is_empty() {
            return 0.0;
        }
        let mut sorted = self.latencies.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let idx = ((p / 100.0) * (sorted.len() as f64 - 1.0)).round() as usize;
        sorted[idx]
    }
}

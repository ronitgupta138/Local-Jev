//! High-concurrency query queue, micro-batching scheduler, and performance telemetry for local-jev.

use crate::domain::{ChoiceRequest, ChoiceResponse, JevError};
use crate::engine::JevDecisionEngine;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Performance telemetry metrics tracked by local-jev.
#[derive(Debug, Default)]
pub struct EngineMetrics {
    pub total_decisions: AtomicU64,
    pub cache_hits: AtomicU64,
    pub cumulative_latency_us: AtomicU64,
    pub speculative_hits: AtomicU64,
    pub batch_decisions: AtomicU64,
}

impl EngineMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_decision(&self, latency_ms: f64, cached: bool) {
        self.total_decisions.fetch_add(1, Ordering::Relaxed);
        if cached {
            self.cache_hits.fetch_add(1, Ordering::Relaxed);
        }
        self.cumulative_latency_us
            .fetch_add((latency_ms * 1000.0) as u64, Ordering::Relaxed);
    }

    pub fn record_speculative_hit(&self) {
        self.speculative_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_batch(&self, count: usize) {
        self.batch_decisions.fetch_add(count as u64, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        let total = self.total_decisions.load(Ordering::Relaxed);
        let hits = self.cache_hits.load(Ordering::Relaxed);
        let total_us = self.cumulative_latency_us.load(Ordering::Relaxed);
        let spec_hits = self.speculative_hits.load(Ordering::Relaxed);
        let batch_count = self.batch_decisions.load(Ordering::Relaxed);

        let avg_latency_ms = if total > 0 {
            (total_us as f64 / total as f64) / 1000.0
        } else {
            0.0
        };

        let hit_rate = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };

        MetricsSnapshot {
            total_decisions: total,
            cache_hits: hits,
            cache_hit_rate: hit_rate,
            avg_latency_ms,
            speculative_hits: spec_hits,
            batch_decisions: batch_count,
        }
    }

    pub fn format_prometheus(&self) -> String {
        let snap = self.snapshot();
        format!(
            "# HELP jev_decisions_total Total number of decisions evaluated\n\
             # TYPE jev_decisions_total counter\n\
             jev_decisions_total {}\n\
             # HELP jev_cache_hits_total Total number of decision cache hits\n\
             # TYPE jev_cache_hits_total counter\n\
             jev_cache_hits_total {}\n\
             # HELP jev_cache_hit_ratio Ratio of decisions served from cache\n\
             # TYPE jev_cache_hit_ratio gauge\n\
             jev_cache_hit_ratio {:.4}\n\
             # HELP jev_latency_avg_ms Average inference latency in milliseconds\n\
             # TYPE jev_latency_avg_ms gauge\n\
             jev_latency_avg_ms {:.4}\n\
             # HELP jev_speculative_hits_total Total speculative tool calls intercepted\n\
             # TYPE jev_speculative_hits_total counter\n\
             jev_speculative_hits_total {}\n\
             # HELP jev_batch_decisions_total Total decisions processed in batch mode\n\
             # TYPE jev_batch_decisions_total counter\n\
             jev_batch_decisions_total {}\n",
            snap.total_decisions,
            snap.cache_hits,
            snap.cache_hit_rate,
            snap.avg_latency_ms,
            snap.speculative_hits,
            snap.batch_decisions
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricsSnapshot {
    pub total_decisions: u64,
    pub cache_hits: u64,
    pub cache_hit_rate: f64,
    pub avg_latency_ms: f64,
    pub speculative_hits: u64,
    pub batch_decisions: u64,
}

/// Batched decision runner processing multiple queries with micro-batching.
pub struct MicroBatchScheduler {
    engine: Arc<JevDecisionEngine>,
    pub metrics: Arc<EngineMetrics>,
}

impl MicroBatchScheduler {
    pub fn new(engine: Arc<JevDecisionEngine>) -> Self {
        Self {
            engine,
            metrics: Arc::new(EngineMetrics::new()),
        }
    }

    /// Process a batch of candidate decision queries concurrently.
    pub fn execute_batch(&self, requests: &[ChoiceRequest]) -> Result<Vec<ChoiceResponse>, JevError> {
        let mut responses = Vec::with_capacity(requests.len());
        for req in requests {
            let start = Instant::now();
            let mut res = self.engine.decide_choice(req)?;
            res.latency_ms = start.elapsed().as_secs_f64() * 1000.0;
            self.metrics.record_decision(res.latency_ms, res.cached);
            responses.push(res);
        }
        Ok(responses)
    }
}

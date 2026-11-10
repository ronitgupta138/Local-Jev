# Contextual Multi-Armed Bandit Feedback Loop for JVM Optimization

## 1. Formulation
Local-Jev dynamically selects optimal JVM execution profiles (heap boundaries, garbage collector selection, tier compilation thresholds) using a LinUCB contextual bandit:
- **Context Vector $x_t$:** Hardware topology (cores, NUMA nodes), available RAM, active container cgroup limits, and historical workload burstiness.
- **Action Space $A$:** Curated discrete JVM parameter bundles (e.g., ZGC low-latency vs G1 throughput vs Shenandoah pause-less).
- **Reward Function $r_t$:**
  $$r_t = -(\alpha \cdot \text{P99_latency} + \beta \cdot \text{RSS_memory_mb} + \gamma \cdot \text{GC_pause_ms})$$

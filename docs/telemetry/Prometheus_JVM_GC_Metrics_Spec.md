# Prometheus JVM Garbage Collection Telemetry Specification

## 1. Metric Definitions
- jvm_gc_pause_seconds (Histogram with exponential buckets).
- jvm_memory_pool_bytes_used (Labels: pool, generation).

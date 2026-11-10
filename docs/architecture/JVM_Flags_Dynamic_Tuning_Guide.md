# Autonomous JVM Flag Selection & GC Ergonomics Guide

## 1. Flag Selection Decision Rules
- If container memory limit $< 2\text{GB}$: Select G1GC with `-XX:MaxGCPauseMillis=20` and aggressive young generation resizing.
- If physical memory $> 16\text{GB}$ and latency SLA $< 5\text{ms}$: Select Generational ZGC (`-XX:+UseZGC -XX:+ZGenerational`).
- On AMD Zen architectures: Enable `-XX:+UseLargePages` and optimize L3 cache boundary alignment.

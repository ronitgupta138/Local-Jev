# HotSpot JIT Deoptimization Loop Detection

## 1. Mechanism
- Monitors HotSpot compilation events via JVM TI.
- Detects methods toggling between C2 compilation and interpreter due to unstable branch profiling.

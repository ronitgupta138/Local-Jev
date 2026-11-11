# Linux Cgroups V2 Memory Pressure Stall Information (PSI) Monitoring

## 1. Metric Sampling
- Reads /sys/fs/cgroup/memory.pressure for some and full stall percentages across 10-second averages.
- Triggers dynamic JVM heap shrinking before memory saturation occurs.

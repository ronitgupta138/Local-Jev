# Linux JVM Discovery Heuristics

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Scanning /usr/lib/jvm, /opt/java, and ~/.local/share/jvm for standard JDK directory layouts.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

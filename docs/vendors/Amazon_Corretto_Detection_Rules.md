# Amazon Corretto Detection Rules

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Recognizing Corretto LTS builds and Amazon Linux optimized JVM profiles.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

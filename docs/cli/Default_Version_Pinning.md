# Default Global Version Pinning

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Configuring global fallback JDK version in ~/.config/jev/config.toml.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

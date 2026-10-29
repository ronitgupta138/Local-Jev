# Fish Shell Prompt Integration

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Fast fish prompt segment displaying active JDK vendor and major version.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

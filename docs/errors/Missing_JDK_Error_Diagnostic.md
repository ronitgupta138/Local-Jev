# Missing JDK Error Diagnostic Guidance

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Actionable terminal error messages with one-click installation commands when JDK is missing.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

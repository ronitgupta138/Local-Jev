# Oracle JDK Detection and Metadata

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Identifying Oracle commercial vs OTN licensed distributions via vendor strings.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

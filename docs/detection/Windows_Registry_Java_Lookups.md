# Windows Registry Java Lookups

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Querying HKLM and HKCU JavaSoft registry keys for installed JDK software.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

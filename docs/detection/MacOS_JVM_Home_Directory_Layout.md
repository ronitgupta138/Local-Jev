# macOS JVM Home Directory Layout

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Parsing /Library/Java/JavaVirtualMachines and querying `/usr/libexec/java_home`.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

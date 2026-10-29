# Dot Java Version File Parsing

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Walking parent directories to discover .java-version or pom.xml java.version targets.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

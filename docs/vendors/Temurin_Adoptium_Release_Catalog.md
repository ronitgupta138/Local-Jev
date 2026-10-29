# Eclipse Temurin Adoptium Release Catalog

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Parsing Eclipse Adoptium API metadata and verifying SHA256 checksums.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

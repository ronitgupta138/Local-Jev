# Listing Installed Java Versions CLI

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Formatting installed JDKs in clean Unicode tables with active version indicator.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

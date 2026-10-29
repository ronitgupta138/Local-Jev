# Symlink Shimming Architecture

## Overview
Technical specification for **Local-Jev** systems engine.

## Specification Details
Atomic directory symlinking of ~/.local/share/jev/current to target JDK home.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

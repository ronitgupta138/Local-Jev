# Contributing to `local-jev`

Thank you for your interest in contributing to `local-jev`! We welcome contributions to our sub-millisecond logit calculations, SIMD quantized linear operators, cascade routing algorithms, and SDK clients.

---

## 🛠️ Development Setup

### Prerequisites
- **Rust Toolchain:** 1.80+ (edition 2024 / 2021)
- **Cargo:** Included with Rust

### Build & Test
```bash
# Clone the repository
git clone https://github.com/ronitgupta138/local-jev.git
cd local-jev

# Run the full test suite
cargo test

# Check code formatting and lints
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

---

## 📐 Architecture Guidelines

1. **Sub-18ms Latency Invariant:** All routing, logit evaluation, and classification passes must execute in under 18ms on a consumer CPU. Avoid unnecessary memory allocations or blocking system calls on hot paths.
2. **100% Offline Guarantee:** `local-jev` must never make external network calls or require remote telemetry credentials. All token projections, bandit updates, and vector comparisons remain strictly local.
3. **TypeSafe Jev Parity:** API endpoints under `/v1/systemone/*` must maintain exact wire compatibility with TypeSafe Jev schemas.

---

## 🚀 Pull Request Checklist

- [ ] All unit and integration tests pass (`cargo test`).
- [ ] No compiler or clippy warnings (`cargo clippy --all-targets -- -D warnings`).
- [ ] Added test coverage for new endpoints or mathematical layers.
- [ ] Clean commit messages following Conventional Commits (`feat:`, `fix:`, `docs:`, `test:`).
- [ ] Zero leaked credentials or machine-specific paths.

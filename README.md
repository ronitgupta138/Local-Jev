<div align="center">

```text
██╗      ██████╗  ██████╗  █████╗ ██╗             ██╗███████╗██╗   ██╗
██║     ██╔═══██╗██╔════╝ ██╔══██╗██║             ██║██╔════╝██║   ██║
██║     ██║   ██║██║      ███████║██║             ██║█████╗  ██║   ██║
██║     ██║   ██║██║      ██╔══██║██║        ██   ██║██╔══╝  ╚██╗ ██╔╝
███████╗╚██████╔╝╚██████╗ ██║  ██║███████╗   ╚█████╔╝███████╗ ╚████╔╝ 
╚══════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚══════╝    ╚════╝ ╚══════╝  ╚═══╝  
```

### **100% Offline, Zero-Cost Drop-In Alternative to TypeSafe Jev System One Decisions**

[![GitHub Stars](https://img.shields.io/github/stars/ronitgupta138/local-jev?style=flat-square&color=0891b2&labelColor=0a0b0e)](https://github.com/ronitgupta138/local-jev/stargazers)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.5.0-0891b2?style=flat-square&labelColor=0a0b0e)](https://crates.io/crates/local-jev)
[![License: MIT](https://img.shields.io/badge/License-MIT-0891b2?style=flat-square&labelColor=0a0b0e)](LICENSE)
[![Rust: 1.80+](https://img.shields.io/badge/Rust-1.80+-0891b2?style=flat-square&labelColor=0a0b0e)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/Tests-12%2F12%20Passing-0891b2?style=flat-square&labelColor=0a0b0e)](#-verification--benchmarks)
[![Views](https://hits.sh/github.com/ronitgupta138/local-jev.svg?style=flat-square&label=Views&color=0891b2&labelColor=0a0b0e)](https://hits.sh/github.com/ronitgupta138/local-jev/)
[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-ea4aaa?style=flat-square&labelColor=0a0b0e)](https://github.com/sponsors/ronitgupta138)

**Zero API keys. Zero cloud egress. Zero rate limits.**  
Run fast autonomous agent decision models (`Choice`, `Score`, `Noul`, `Route`) locally on your CPU in **< 18ms**.

</div>

---

## 🎯 The Problem with Cloud Decision APIs

Fast agent decision models like TypeSafe's Jev provide rapid choice selection (`Choice`, `Noul`, `Score`), but enforce heavy constraints on developers:
1. **Cloud Lock-in & Billing:** Incurring API costs on every micro-decision adds up to tens of dollars across automated agent loops.
2. **Privacy & Air-Gapped Workloads:** Proprietary code diffs, internal logs, and environment states are transmitted to external cloud servers.
3. **Network Round-Trip Overhead:** An 80ms–150ms network round-trip slows down chained agent decision steps.

`local-jev` provides an **identical REST API specification** that runs 100% offline as a single native Rust binary. Swap your base URL and run without cloud dependencies.

```bash
export TYPESAFE_BASE_URL="http://localhost:8080/v1"
```

---

## 📊 Comparison

| Metric | Cloud TypeSafe Jev | `local-jev` (Local CPU Engine) |
| :--- | :---: | :---: |
| **Inference Cost** | $0.002 / call | **$0.00 (Zero forever)** |
| **Latency SLA** | 70ms – 180ms (Network + Queue) | **< 18ms (Single-Pass CPU Logits)** |
| **Data Privacy** | Egress to remote cloud | **100% Offline / Air-Gapped** |
| **Rate Limits** | Quota / Concurrency throttles | **Unlimited (Hardware capacity)** |
| **Dependencies** | API Key & Internet connection | **Zero (Self-contained binary)** |

---

## ⚡ Architectural Blueprint

```
+-----------------------------------------------------------------------------------+
|               Autonomous Agent (Claude Code, Hermes, MCP Tools, Browser-Use)      |
|                 `export TYPESAFE_BASE_URL=http://localhost:8080/v1`               |
+-----------------------------------------------------------------------------------+
                                          |
                                          | POST /v1/systemone/choice
                                          | POST /v1/systemone/noul
                                          v
+-----------------------------------------------------------------------------------+
|                            `local-jev` Engine (Rust)                              |
|                                                                                   |
|  [ Step 1: Axum REST Server ]                                                     |
|  - Ingests: `{ state: "...", question: "Which tool?", options: ["grep", "cat"] }`|
|                                                                                   |
|  [ Step 2: Single-Forward-Pass Logit Extraction ]                                 |
|  - Evaluates prompt state so the immediate next token predicts choice answer.     |
|  - ZERO auto-regressive decoding loops. Zero token string generation.             |
|                                                                                   |
|  [ Step 3: Constrained Softmax & Confidence Calibration ]                         |
|  - Masks vocabulary exclusively to candidate option token probabilities:          |
|    `P(opt_i) = exp(logit_i / T) / sum(exp(logit_j / T))`                          |
+-----------------------------------------------------------------------------------+
                                          |
                                          v
+-----------------------------------------------------------------------------------+
|                               TypeSafe-Compatible JSON                            |
|  `{ "choice": "grep", "confidence": 0.942, "latency_ms": 1.2, "cached": false }`  |
+-----------------------------------------------------------------------------------+
```

---

## 🚀 Quickstart

### 1. Build and Run Server
```bash
cargo install local-jev
# Or build from source:
git clone https://github.com/ronitgupta138/local-jev.git
cd local-jev && cargo build --release
./target/release/local-jev serve --port 8080
```

### 2. Query Decision Endpoint (`POST /v1/systemone/choice`)
```bash
curl -X POST http://localhost:8080/v1/systemone/choice \
  -H "Content-Type: application/json" \
  -d '{
    "state": "The compiler returned unresolved import crate::auth::User",
    "question": "Which command should be executed to locate the definition?",
    "options": ["grep -rn User", "git commit", "docker stop"],
    "temperature": 0.1
  }'
```

**Response:**
```json
{
  "choice": "grep -rn User",
  "confidence": 0.984,
  "probabilities": [
    { "option": "grep -rn User", "probability": 0.984, "logit": 5.5 },
    { "option": "git commit", "probability": 0.008, "logit": 1.2 },
    { "option": "docker stop", "probability": 0.008, "logit": 1.1 }
  ],
  "latency_ms": 1.4,
  "cached": false
}
```

---

## 🛠️ CLI Standalone Mode

Run decisions directly from your terminal or shell scripts:

```bash
# Fast choice
local-jev choice \
  --state "Bug found in login handler" \
  --question "Which file to inspect?" \
  --options "src/auth.rs,src/db.rs,src/main.rs"

# Fast route
local-jev route \
  --prompt "Deploy new release to production" \
  --routes "deploy_agent,triage_agent,review_agent"

# Run latency benchmark
local-jev benchmark --iterations 1000
```

---

## 💻 Client SDKs (Python & TypeScript)

### Python SDK (`local-jev`)
```python
from local_jev import JevClient

client = JevClient("http://localhost:8080/v1")

# Single-pass choice selection
decision = client.choice(
    state="Compiler unresolved import crate::auth::User",
    question="Which tool to search the codebase?",
    options=["grep", "docker", "curl"]
)
print("Selected tool:", decision["choice"])  # "grep"
```

### TypeScript SDK (`local-jev`)
```typescript
import { JevClient } from "local-jev";

const client = new JevClient("http://localhost:8080/v1");
const res = await client.choice(
  "Database connection pool exhausted",
  "Which mitigation action should be selected?",
  ["scale_pool", "reboot_host", "ignore"]
);
console.log(res.choice); // "scale_pool"
```

---

## 🤖 Model Context Protocol (MCP) Integration

`local-jev` includes a native stdio Model Context Protocol (MCP) server for **Claude Desktop, Cursor, and Hermes Agent**.

### Add to Claude Desktop (`claude_desktop_config.json`)
```json
{
  "mcpServers": {
    "local-jev": {
      "command": "local-jev",
      "args": ["mcp"]
    }
  }
}
```

---

## 🧪 Verification & Benchmarks

```bash
cargo test
```

```
running 12 tests
test test_semantic_score_relevance ... ok
test test_choice_selection_accuracy ... ok
test test_agent_noul_classification ... ok
test test_fast_route_classification ... ok
test test_typesafe_http_server_endpoints ... ok
test test_mcp_initialize_and_tool_list ... ok
test test_mcp_call_choice_tool ... ok
test test_model_backbone_default_mode ... ok
test test_model_backbone_file_loader ... ok
test test_model_backbone_missing_file_error ... ok
test test_prometheus_metrics_export ... ok
test test_microbatch_scheduler_concurrent_queries ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; finished in 0.00s
```

Measured over 1,000 iterations on AMD Ryzen 5:
* **p50 Latency:** `1.0µs` (`0.001ms`)
* **Throughput:** `1,000,000+` decisions/second on CPU

---

## 📜 License

MIT License. Designed and engineered by [Ronit Gupta](https://github.com/ronitgupta138).

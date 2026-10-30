# ⚡ Zeno MCP

**High-Performance Pure Native Rust Model Context Protocol (MCP) Harness & Autonomous Agent Engine**  
*Zero Python · Zero Node.js · Zero-Token Local System 1 Routing · Sub-Millisecond Latency*

---

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![MCP](https://img.shields.io/badge/MCP-Claude%20Code%20%26%20Hermes%20Standard-blue.svg)](https://modelcontextprotocol.io/)
[![Latency](https://img.shields.io/badge/Router%20Latency-p50%20%3D%200.019ms-brightgreen.svg)](#-statistical-benchmarks)

---

## 🌟 Overview

**Zeno MCP** is a production-grade, memory-safe Model Context Protocol (MCP) harness and autonomous agent framework written entirely in pure native Rust. 

Engineered to resolve the two biggest challenges in LLM agent engineering—**tool execution latency** and **context window bloat**—Zeno MCP implements the security and tool-dispatch architectures defined by Claude Code, the Hermes Agent standard, and NSA Model Context Protocol Guidelines.

```
                    ┌────────────────────────────────────────────────────────┐
                    │               Incoming User Prompt                     │
                    └──────────────────────────┬─────────────────────────────┘
                                               │
                                 ┌─────────────▼─────────────┐
                                 │   Jev System 1 Router     │  ← 0.02ms Latency
                                 │   (Zero-Token Decision)   │  ← ECE = 0.0280
                                 └─────────────┬─────────────┘
                                               │
                   ┌───────────────────────────┴───────────────────────────┐
                   ▼                                                       ▼
     ┌───────────────────────────┐                           ┌───────────────────────────┐
     │  HNSW Injection Guard     │                           │   MCP Harness Sandbox     │
     │  (64-dim Cosine Embed)    │                           │   (Hermes Whitelist + OSV)│
     └─────────────┬─────────────┘                           └─────────────┬─────────────┘
                   │                                                       │
                   └───────────────────────────┬───────────────────────────┘
                                               │
                                 ┌─────────────▼─────────────┐
                                 │   Context Optimization   │  ← Structured JSON Truncation
                                 │   (ToolSearch + Cache-Pin)│  ← 75% Gemini Cost Discount
                                 └───────────────────────────┘
```

---

## 🚀 Key Innovations & Architecture

### 1. 🧠 Jev System 1 Routing Engine
- **Zero Generation Paradigm**: Eliminates expensive LLM calls for intent routing. Directly outputs typed, probabilistic decisions (`Choice`, `Score`, `Noul`) without generating a single completion token.
- **Calibrated Decision Logits**: Calibrated via Platt temperature scaling ($T=0.95$) with an Expected Calibration Error (ECE) $< 0.03$.
- **Latency**: Sub-millisecond evaluation (**p50: 0.019 ms**) on native CPU.

### 2. 🔬 HNSW Semantic Injection Classifier
- Replaces brittle keyword-matching guards with a 64-dimensional dense vector space indexed over 108 attack variations.
- Defends against indirect prompt injection, credential exfiltration, role-play jailbreaks, SSRF, and command tampering.

### 3. ✂️ Structured Context Truncation (PostToolUse)
- **JSON-Aware**: Preserves root-level syntax while collapsing intermediate array elements (`[... ✂️ OMITTED N intermediate array items ...]`), ensuring outputs remain 100% parseable by downstream models.
- **Terminal Line-Tail Awareness**: Retains compiler header diagnostics and tail panic traces, discarding verbose intermediate compilation logs.
- **Automated Advisory**: Scans tool responses for server banner disclosures (`Apache/2.4.6`, `CentOS`, `nginx/1.18`) and attaches defensive advisories.

### 4. 🔍 Dynamic ToolSearch & Lazy Promotion
- Implements Section 7 of Claude Code: prevents initial prompt context from being overwhelmed with dozens of full JSON tool schemas.
- Exposes only core tools initially. Models discover deferred tools on-demand using hybrid semantic cosine similarity + keyword overlap.

### 5. 📌 Gemini Context Cache Pinning (`cachedContent`)
- Pre-compiles static system harness instructions and tool schemas (~1,960 tokens).
- Leverages the Google Gemini `cachedContent` API to unlock an immediate **75% input token discount**.

### 6. 🌐 Native Chrome CDP Headless Browser
- Built-in Chrome DevTools Protocol engine in pure Rust with zero Node.js / Puppeteer dependencies.
- Executes real-time DOM extraction, form discovery, and automated web audits.

---

## 📊 Statistical Benchmarks

Benchmarked across 200 consecutive runs on on-device native CPU:

| Subsystem | Min | Mean | p50 | p95 | p99 | Max |
|:---|:---:|:---:|:---:|:---:|:---:|:---:|
| **🧠 Jev System 1 Router** | `0.014 ms` | `0.023 ms` | **`0.019 ms`** | `0.049 ms` | `0.058 ms` | `0.058 ms` |
| **🛡️ MCP Tool Pool & Env Sanitizer**| `0.601 ms` | `0.714 ms` | **`0.632 ms`** | `1.070 ms` | `1.088 ms` | `1.088 ms` |
| **🔬 HNSW Semantic Injection Guard** | `0.005 ms` | `0.011 ms` | **`0.008 ms`** | `0.025 ms` | `0.032 ms` | `0.032 ms` |

*All core decisions complete in under 1 millisecond with zero cloud API billing.*

---

## 🛠️ Installation & Building

### Prerequisites
- [Rust](https://rustup.rs/) (edition 2024 or 1.80+)
- Windows, macOS, or Linux

### Build from Source
```bash
git clone https://github.com/your-username/zeno-mcp.git
cd zeno-mcp

# Build optimized release binary
cargo build --release

# Run unit & security test suite
cargo test
```

The compiled binary will be located at `target/release/zeno`.

---

## 💻 CLI Commands & Usage

```bash
# 1. View Zeno MCP help & overview
zeno --help

# 2. View MCP harness security status and pre-trust gating
zeno mcp

# 3. Perform dynamic HNSW Semantic ToolSearch across deferred tools
zeno mcp --action search --query "compile rust binary"

# 4. Run real-time OSV supply-chain vulnerability screening
zeno mcp --action osv --package axios --ecosystem npm

# 5. Run zero-token Jev System 1 prompt routing
zeno jev --prompt "Audit live target https://pwn.college for security headers"

# 6. Execute live headless browser DOM audit
zeno browse --url "https://pwn.college"

# 7. Run sub-millisecond statistical latency benchmarks
zeno bench --runs 100
```

---

## ⚙️ Configuration & Environment Variables

Zeno MCP runs 100% offline with zero required configuration. To customize directories or enable optional hybrid Gemini cloud fallback, copy `.env.example` to `.env`:

```bash
# (Optional) Cloud fallback for generative reasoning
export GEMINI_API_KEY="your_api_key_here"

# (Optional) Custom workspace directories
export ZENO_SKILLS_PATH="./skills"
export ZENO_VAULT_PATH="./vault"
export ZENO_WORKSPACE_PATH="./workspace"
```

---

## 🔒 Security Architecture (CVE & NSA Compliance)

1. **CVE-2025-59536 Defense**: Enforces Pre-Trust boundaries. Untrusted MCP servers are halted before process initialization.
2. **Environment Variable Scrubbing**: Strips sensitive environment variables (API keys, authorization tokens, private keys) before spawning stdio subprocesses.
3. **Glob + Regex Execution Deny Rules**: Detects and neutralizes malicious patterns like `rm -rf`, path traversal (`../..`), and exfiltration pipes.
4. **Real-Time Supply Chain Screening**: Integrates directly with the `https://api.osv.dev/v1/query` registry to audit stdio tools against known vulnerabilities.

---

## 🤝 Contributing

Contributions to Zeno MCP are warmly welcome!
1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/zeno-upgrade`).
3. Commit your changes (`git commit -m 'feat: add feature'`).
4. Ensure all unit tests pass (`cargo test`).
5. Push to the branch (`git push origin feature/zeno-upgrade`).
6. Open a Pull Request.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

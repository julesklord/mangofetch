# 🗺️ MangoFetch Engineering Roadmap

> **Authoritative Context:** We prioritize modularity, memory safety, and minimal dependency bloat.

## Executive Summary

MangoFetch has successfully transitioned from a monolithic GUI/CLI hybrid into a highly decoupled, CLI-first media engine (`v0.2.x` -> `v0.3.x`). The next 12-18 months will focus on extending the core via plugins, re-introducing a highly optimized GUI through a meta-crate architecture, and venturing into decentralized distribution.

---

## 🟢 Phase 1: UX Polish & TUI Integration (v0.3.x)

Our immediate focus is refining the terminal experience without adding core bloat. The CLI must remain the fastest way to interact with the engine.

- **[x] v0.3.1 - Rebranding & Core Cleanup:** Finalize the "OmniGet" to "MangoFetch" transition, clean up environment variables, and stabilize test suites.
- **[ ] Alias System (`mango`):** Implement the secondary `mango` binary and short subcommands (`d`, `ls`, `cfg`) for power users.
- **[ ] Interactive TUI (`ratatui`):** Introduce an optional full-screen terminal interface for monitoring massive batch downloads and queue state.

---

## 🟡 Phase 2: Extensibility & Ecosystem (v0.4.x)

MangoFetch cannot natively support every platform on earth. We must decentralize platform extraction logic to the community.

- **[ ] SDK Stabilization:** Finalize the ABI for `mangofetch-plugin-sdk`.
- **[ ] Dynamic Loading:** Robust error handling for loading `.so`/`.dll` plugins at runtime.
- **[ ] Plugin Manager:** Add CLI commands to discover, install, and update community plugins (`mango plugin install <name>`).

---

## 🟠 Phase 3: GUI Reintegration & Meta-Crate (v0.5.x)

We will provide a premium desktop experience without compromising the CLI user base.

- **[ ] The Umbrella Crate:** Restructure the workspace to release a `mangofetch` crate that bundles both the CLI and GUI, while `mangofetch-cli` remains standalone.
- **[ ] Tauri v2 GUI (`mangofetch-gui`):** Develop a lightweight, heavily optimized graphical interface that acts strictly as a client to `mangofetch-core`.
- **[ ] IPC Optimization:** Ensure communication between the Rust core and the WebView frontend is non-blocking and memory-efficient.

---

## 🔴 Phase 4: Decentralization & Scaling (v0.6.x+)

Moving beyond single-client downloads into distributed network topologies.  

- **[ ] P2P Relay (`MANGOFETCH_RELAY`):** fully realize the P2P swarm logic for downloading and sharing large media archives.
- **[ ] Distributed Queueing:** Allow multiple MangoFetch instances (e.g., a server and a laptop) to share a single download queue.

---

## 🛡️ Continuous Auditing & Technical Debt

As we move fast, we must continuously pay down technical debt:

1. **Dependency Minimization:** Regularly run `cargo tree` and `cargo audit` to prune unused or vulnerable crates.
2. **Test Coverage:** Maintain integration tests for the queue engine. Any new platform extractor must include mocked network tests.
3. **Documentation:** Keep `README.md` and `AGENTS.md` strictly aligned with the current architectural state.

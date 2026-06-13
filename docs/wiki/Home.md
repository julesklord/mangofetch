# 🥭 MangoFetch TUI / Core Wiki

Welcome to the official documentation for **mangofetch**, the core library SDK and master Terminal User Interface (TUI) frontend of the mangoSuite.

This repository serves as the central hub of the ecosystem, containing:
1.  **`mangofetch` (TUI)**: Master terminal dashboard client using `ratatui` (runs by default).
2.  **`mangofetch-core`**: Asynchronous download engine used by all suite clients.
3.  **`mangofetch-plugin-sdk`**: Extension library to build custom extraction plugins.

---

## 🧭 Navigation

### 🚀 Getting Started
*   **[Installation Guide](Installation.md)**: How to install the TUI client or import the core download manager in Rust.
*   **[Quick Start Guide](Quick-Start.md)**: First download with the TUI in under 60 seconds.

### 🛠️ User Guides
*   **[TUI Experience](TUI-Experience.md)**: Master the interactive Terminal User Interface, themes, and mouse support.
*   **[CLI Reference](CLI-Guide.md)**: Standard CLI commands supported by the main binary.
*   **[Configuration](Configuration.md)**: Fine-tuning download directories and settings.

### 🏗️ Technical Architecture
*   **[The Core Engine](Architecture.md)**: Deep dive into the asynchronous queue, workers, and recovery logic.
*   **[Platform Registry](Platforms.md)**: How the engine matches and downloads URLs.
*   **[Plugin Development](Plugin-Development.md)**: How to build custom platform extractors using the SDK.

---

## 🍊 The mangoSuite
For non-TUI environments, you can use the separate client frontends:
*   **[mangofetch-cli](https://github.com/julesklord/mangofetch-cli)**: Scriptable, pure CLI frontend for headless/server use.
*   **[mangofetch-gui](https://github.com/julesklord/mangofetch-gui)**: Hardware-accelerated desktop client (egui).

---

## 🛡️ License
MangoFetch is licensed under the **GPL-3.0-or-later**.

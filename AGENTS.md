# MangoFetch Project Agent Instructions

High-signal guidance for working in the MangoFetch codebase.

## 🏗 Architecture & Core Concepts

- **Crate Split:**
  - `mangofetch-core`: UI-agnostic engine (queue, logic, platform downloaders). **Primary target for business logic.**
  - `mangofetch-cli`: `clap`-based binary using the core.
  - `mangofetch` (GUI): Tauri application. Frontend in SvelteKit (root folder), Backend in `src-tauri/src`.
  - `mangofetch-plugin-sdk`: SDK for extending download capabilities.
- **Shared Logic:** Never duplicate logic between CLI and GUI. Always move shared behaviors to `mangofetch-core`.
- **Portable Mode:** The app looks for `portable.txt` or `.portable` next to the executable to redirect data to a local `data` folder.

## 🛠 Developer Workflow

### Environment Setup

- **Rust 1.70+** is required.
- **Node.js** with `pnpm` for the Tauri frontend.
- **Dependencies:** `yt-dlp` and `FFmpeg` are auto-downloaded to the app data dir on first run. Use `mangofetch-cli check` to verify.

### Key Commands

- **CLI Development:** `cargo run` from root (defaults to `mangofetch-cli`).
- **GUI Development:** `pnpm tauri dev`.
- **Core Tests:** `cargo test -p mangofetch-core`.
- **CLI Tests:** `cargo test -p mangofetch-cli`.

- **Linter:** `cargo clippy`.
- **Formatter:** `cargo fmt`.
- **Svelte Check:** `pnpm check`.

## ⚠️ Gotchas & Constraints

- **Env Vars:** The app removes `PYTHONHOME` and `PYTHONPATH` during setup to avoid conflicts with `yt-dlp`.
- **CWD Sensitivity:** `Cargo.toml` at root manages the workspace. Use `workdir` accordingly in tool calls.
- **Automated Downloads:** Be aware that running the app/tests may trigger network requests to download binaries (`yt-dlp`, `FFmpeg`).
- **Pathing:** GUI data is in `%APPDATA%/mangofetch` (Windows) or `~/.local/share/mangofetch` (Linux) unless in portable mode.

## 🧪 Testing

- **Integration Tests:** Located in `src-tauri/mangofetch-core/tests/queue_tests.rs`.
- **Running Single Test:** `cargo test -p mangofetch-core --test queue_tests <test_name>`.
- **Mocking:** Be careful with tests that require actual media binaries; prefer unit tests for logic in `mangofetch-core`.

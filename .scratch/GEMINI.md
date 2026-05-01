# OmniGet Project Instructions

This document provides essential context and instructions for AI agents working on the OmniGet project. OmniGet is a multi-platform download manager featuring both a CLI and a GUI.

## Project Overview

OmniGet is a monorepo structured around a shared Rust core that provides a platform-agnostic download engine. It supports downloads from 1000+ sites via `yt-dlp`, as well as torrents/magnets and direct file downloads.

### Core Technologies
- **Backend:** Rust (Tokio, Reqwest, Clap, Indicatif)
- **Frontend:** SvelteKit + TypeScript + Vite
- **Desktop Bridge:** Tauri v2
- **Download Engine:** yt-dlp, FFmpeg, aria2c (managed as runtime dependencies)

### Architecture
- `src-tauri/omniget-core`: The "Engine". UI-agnostic library containing download queue management, dependency handling, and platform-specific downloaders (YouTube, Instagram, etc.).
- `src-tauri/omniget-cli`: Standalone CLI binary using `omniget-core`.
- `src-tauri/omniget-plugin-sdk`: SDK for extending functionality via plugins.
- `src-tauri/`: Main Tauri application crate (GUI backend logic, commands, tray icon).
- `src/`: SvelteKit frontend for the GUI.
- `browser-extension/`: Chrome and Firefox extensions for OmniGet.

## Building and Running

### CLI (Rust)
- **Dev Run:** `cargo run -p omniget-cli -- [args]` (from project root)
- **Build Dev:** `cargo build -p omniget-cli`
- **Build Release:** `cargo build -p omniget-cli --release`
- **Output:** Binaries are located in `src-tauri/target/[debug|release]/omniget-cli`

### GUI (Tauri + SvelteKit)
- **Install Dependencies:** `pnpm install`
- **Dev Mode:** `pnpm tauri dev`
- **Build:** `pnpm tauri build`
- **Frontend Only:** `pnpm dev`

### Project-wide commands (via package.json)
- `pnpm test`: Run Vitest tests for the frontend.
- `pnpm check`: Run Svelte-check and sync.
- `pnpm plugins:deploy`: Deploy plugins locally for development.

## Development Conventions

### Rust (Backend)
- **Shared Logic:** Always prioritize placing business logic, downloaders, and models in `omniget-core` so they are available to both CLI and GUI.
- **Linting:** Run `cargo clippy --workspace --all-targets` and `cargo fmt --all` before committing.
- **Testing:** Run `cargo test --workspace` to ensure core and CLI stability.
- **Conventions:** Follow idiomatic Rust patterns and use `anyhow` for error handling in the application layer.

### Frontend (SvelteKit)
- **Style:** Use Vanilla CSS where possible (as per project default preference).
- **TypeScript:** Strict typing is preferred.
- **I18n:** Localization keys are generated and managed via `sveltekit-i18n`. Use `pnpm generate:i18n-keys` after adding new translations.

### Workflow
- **Commits:** Follow [Conventional Commits](https://www.conventionalcommits.org/): `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`.
- **New Platforms:** Implement new downloaders in `src-tauri/omniget-core/src/platforms/` by implementing the `PlatformDownloader` trait.
- **Security:** Do NOT log or commit secrets. Follow `SECURITY.md`.

## Key Files
- `Cargo.toml`: Workspace configuration.
- `package.json`: Frontend and project scripts.
- `src-tauri/tauri.conf.json`: Tauri configuration (capabilities, bundle info).
- `src-tauri/omniget-core/src/core/manager/`: Download queue and engine logic.
- `src-tauri/omniget-cli/src/main.rs`: CLI command definitions.

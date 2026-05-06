# MangoFetch Project Instructions

This document provides essential context and instructions for AI agents working on the MangoFetch project. MangoFetch is a multi-platform download manager featuring both a CLI and a GUI.

## Project Overview

MangoFetch is a monorepo structured around a shared Rust core that provides a platform-agnostic download engine. It supports downloads from 1000+ sites via `yt-dlp`, as well as torrents/magnets and direct file downloads.

### Core Technologies

- **Backend:** Rust (Tokio, Reqwest, Clap, Indicatif)
- **Download Engine:** yt-dlp, FFmpeg, aria2c (managed as runtime dependencies)

### Architecture

- `mangofetch-core`: The "Engine". UI-agnostic library containing download queue management, dependency handling, and platform-specific downloaders (YouTube, Instagram, etc.).
- `mangofetch-cli`: Standalone CLI binary using `mangofetch-core`.
- `mangofetch-plugin-sdk`: SDK for extending functionality via plugins.

## Building and Running

### CLI (Rust)

- **Dev Run:** `cargo run -p mangofetch-cli -- [args]` (from project root)
- **Build Dev:** `cargo build -p mangofetch-cli`
- **Build Release:** `cargo build -p mangofetch-cli --release`
- **Output:** Binaries are located in `target/[debug|release]/mangofetch-cli`

### Project-wide commands

- `cargo test --workspace`: Run all tests.
- `cargo clippy`: Run linter.

## Development Conventions

### Rust (Backend)

- **Shared Logic:** Always prioritize placing business logic, downloaders, and models in `mangofetch-core` so they are available to the CLI.
- **Linting:** Run `cargo clippy --workspace --all-targets` and `cargo fmt --all` before committing.
- **Testing:** Run `cargo test --workspace` to ensure core and CLI stability.
- **Conventions:** Follow idiomatic Rust patterns and use `anyhow` for error handling in the application layer.

## Workflow

- **Commits:** Follow [Conventional Commits](https://www.conventionalcommits.org/): `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`.
- **New Platforms:** Implement new downloaders in `mangofetch-core/src/platforms/` by implementing the `PlatformDownloader` trait.
- **Security:** Do NOT log or commit secrets. Follow `SECURITY.md`.

## Key Files

- `Cargo.toml`: Workspace configuration.
- `package.json`: Frontend and project scripts.
- `src-tauri/tauri.conf.json`: Tauri configuration (capabilities, bundle info).
- `mangofetch-core/src/core/manager/`: Download queue and engine logic.
- `mangofetch-cli/src/main.rs`: CLI command definitions.
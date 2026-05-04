# MangoFetch CLI Overview

## Purpose

This document explains the scope and role of `mangofetch` in this repository.

## What is MangoFetch?

MangoFetch is a specialized, terminal-centric download manager. It serves as a unified interface for media extraction and file acquisition, leveraging a shared core engine that manages concurrent queues, dependency resolution, and multi-platform protocol handling.

## Repository scope

This repository is focused on delivering:

- `mangofetch-cli`: a standalone CLI binary for downloads
- `mangofetch-core`: a shared Rust core library for download logic, dependency discovery, yt-dlp integration, and plugin support
- `mangofetch-plugin-sdk`: plugin interfaces used by the CLI

## History

MangoFetch is an independent evolution of the OmniGet project, refactored for standalone CLI performance. While it shares its lineage with OmniGet, MangoFetch is optimized for low-latency execution and high-volume batch processing.

## Key benefits of MangoFetch

- Terminal-first downloads for video, audio, and torrents
- Scriptable interface for batch workflows and automation
- High-performance Rust-based engine
- Expandable plugin ecosystem through the shared SDK

## How to use this repo

- Build the CLI from `mangofetch-cli`
- Keep shared download logic within `mangofetch-core`
- Reference the history as technical lineage but maintain MangoFetch branding for all user-facing components

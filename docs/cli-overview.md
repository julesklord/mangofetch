# OmniGet CLI Overview

## Purpose

This document explains the scope and role of `omniget-cli` in this repository.

## What is `omniget-cli`?

`omniget-cli` is the command-line companion to the original OmniGet desktop product. It reuses the same shared download engine while exposing terminal-first workflows for automation and scripting.

## Repository scope

This repository is focused on delivering:

- `omniget-cli`: a standalone CLI binary for downloads
- `src-tauri/omniget-core`: a shared Rust core library for download logic, dependency discovery, yt-dlp integration, and plugin support
- `src-tauri/omniget-plugin-sdk`: plugin interfaces used by the CLI and compatible projects

## Original upstream

The original OmniGet desktop app is available at https://github.com/tonhowtf/omniget. That project is the upstream source for brand, mascot, and user-facing design, while this repo is dedicated to the CLI experience.

## Key benefits of `omniget-cli`

- Terminal-first downloads for video, audio, courses, and torrents
- Scriptable interface for batch workflows and automation
- Shared core with the original OmniGet engine for consistency
- Expandable plugin ecosystem through the shared SDK

## How to use this repo

- Build the CLI from `src-tauri/omniget-cli`
- Keep shared download logic within `src-tauri/omniget-core`
- Reference the original OmniGet project as upstream inspiration and compatibility
- Make documentation and naming CLI-first

## Recommended messaging

- "omniget-cli is the command-line companion to OmniGet."
- "Use `omniget-cli` for scripted downloads, automation, and terminal workflows."
- "This repo contains the CLI port and shared core library, with the original desktop app available upstream."

# omniget-cli

`omniget-cli` is the command-line companion to the original OmniGet desktop project. This repository is focused on the CLI port and shared download core, while the upstream desktop application remains available at https://github.com/tonhowtf/omniget.

## What is `omniget-cli`?

`omniget-cli` brings the same intelligent download engine to your terminal. It is designed for automation, scripting, and advanced workflows that need a lightweight command-line interface.

## Why this repo exists

- Primary focus: `omniget-cli`
- Shared core: `src-tauri/omniget-core`
- Original upstream desktop app: https://github.com/tonhowtf/omniget

## Features

- Download videos, audio, and course content from 1000+ sites supported by yt-dlp.
- Support course platforms through the shared plugin architecture.
- Reuse the same engine for FFmpeg, dependency discovery, and yt-dlp integration.
- Designed for automation, batch processing, and server environments.

## Getting started

Display CLI help:

```bash
cargo run -p omniget-cli -- --help
```

Example download command:

```bash
cargo run -p omniget-cli -- download https://www.youtube.com/watch?v=... 
```

## Supported content

`omniget-cli` is intended to support:

- Videos from YouTube, Instagram, TikTok, Twitter/X, Reddit, Twitch, Pinterest, Vimeo, Bluesky, and Bilibili
- Course platforms such as Hotmart, Udemy, Kiwify, Teachable, and more
- Torrents and magnet links
- Any website supported by yt-dlp

## Building from source

**Prerequisites:** [Rust](https://rustup.rs/) 1.70+, [Node.js](https://nodejs.org/) 18+, [pnpm](https://pnpm.io/)

```bash
git clone https://github.com/tonhowtf/omniget.git
cd omniget
cargo build -p omniget-cli --release
```

The CLI binary is built from `src-tauri/omniget-cli` and reuses the shared core library in `src-tauri/omniget-core`.

## Original project

This CLI port references the original OmniGet desktop project by tonhowtf. The original desktop app is available at https://github.com/tonhowtf/omniget.

## Learn more

- CLI overview: [docs/cli-overview.md](docs/cli-overview.md)
- Brand guidelines: [docs/brand-guidelines.md](docs/brand-guidelines.md)

## Contributing

Found a bug or want a feature? Open an issue or submit a pull request.

For brand and messaging guidance, see [docs/brand-guidelines.md](docs/brand-guidelines.md).

## License

[GPL-3.0](LICENSE). The OmniGet name, logo, and Loop mascot are project trademarks not covered by the code license.

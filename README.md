# 🥭 MangoFetch

**Brutally fast. Extensible. Pure Rust.**  
*The download engine the terminal deserves.*

---

<p align="center">
  <img src="demo.gif" alt="MangoFetch in action" width="850" />
</p>

<p align="center">
  <a href="https://github.com/julesklord/mangofetch-cli/releases/latest"><img src="https://img.shields.io/github/v/release/julesklord/mangofetch-cli?style=for-the-badge&color=orange&label=vibe" alt="Latest Release" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-black?style=for-the-badge" alt="License GPL-3.0" /></a>
  <img src="https://img.shields.io/badge/Built%20With-Rust-red?style=for-the-badge&logo=rust" alt="Built with Rust" />
  <img src="https://img.shields.io/badge/Vibe-Brutalist-white?style=for-the-badge" alt="Vibe: Brutalist" />
</p>

## Why MangoFetch?

Most downloaders are either too bloated or too fragile. **MangoFetch** is neither. It's a high-performance download engine wrapped in a minimalist, brutalist terminal interface. 

Built with **Tokio** for high-concurrency and **Reqwest** for rock-solid networking, it handles everything from a single YouTube short to massive batch archival tasks without breaking a sweat.

### The Good Stuff:
- **🚀 Zero Bloat:** Instant startup, minimal memory footprint.
- **⚡ Async Everything:** Powered by Rust's async runtime.
- **🛠️ Self-Healing:** Automatically manages `yt-dlp` and `ffmpeg` dependencies.
- **🎨 Brutalist UI:** Information-dense, distraction-free progress tracking.
- **📦 Platform Agnostic:** 1000+ sites supported via the core engine + native extractors for the big players.
- **🔌 SDK Ready:** Extend it with your own Rust plugins.

## 🛠️ Installation

### The Fast Way (Cargo)
```bash
cargo install mangofetch-cli
```

### The "I want to touch the code" Way
```bash
git clone https://github.com/julesklord/mangofetch-cli.git
cd mangofetch-cli
cargo build --release
# Binary at: target/release/mangofetch
```

## 🕹️ Quick Start

```bash
# Just get the video
mangofetch download "https://www.youtube.com/watch?v=dQw4w9WgXcQ"

# Peek at the metadata first
mangofetch info "https://www.instagram.com/p/..."

# Archival mode: Download everything from a file
mangofetch download-multiple links.txt --concurrent 5

# Check if you're ready to go
mangofetch check
```

## 🗺️ Roadmap & Vibe

| Status | Feature |
| :--- | :--- |
| ✅ | **Core Engine** (Concurrent Queues, Persistence) |
| ✅ | **Native Extractors** (YT, IG, TikTok, X, etc.) |
| ✅ | **Dependency Manager** (Auto-fetch yt-dlp/ffmpeg) |
| 🚧 | **TUI Dashboard** (Full screen monitoring) |
| 🚧 | **Plugin Store** (Community driven extractors) |

## 🤝 Support & Contribution

If you find a bug, open an issue. If you want to add a feature, open a PR. If you like the project, give it a star. 

**Let's build the best downloader for the terminal.**

---
| Version | Milestone |
|---------|-----------|
| **v0.3.1** | Rebranding cleanup and test fixes ✅ |
| **v0.3.0** | Interactive TUI mode (`ratatui`) |
| **v0.2.0** | Standalone rewrite — GUI removed, core refactored ✅ |
| **v0.4.0** | Plugin system |
| **v0.5.0** | P2P file sharing |

## Related projects

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) — Media extraction engine
- [omniget (tonhowft)](https://github.com/tonhowtf/omniget) — omniget - the original project who provides the core. 


## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

<p align="center">
  Built with 🦀 and 🥭 by <a href="https://github.com/julesklord">Jules Martins</a>
</p>

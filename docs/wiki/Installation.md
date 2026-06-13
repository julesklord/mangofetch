# 🚀 Installation Guide

This guide covers installing the interactive Terminal User Interface (TUI) client and integrating the core engine in Rust.

## 📋 System Requirements

*   **Operating Systems**: Windows 10/11, Linux, macOS.
*   **Rust (optional)**: Required if you are building from source or using the SDK.
*   **Python 3**: Required for `yt-dlp` functionality.

---

## 🛠️ Mandatory Dependencies

MangoFetch relies on a few high-quality external tools to handle specific media formats. Ensure these are installed and in your system `PATH`:

1.  **[FFmpeg](https://ffmpeg.org/)**: Essential for merging audio/video streams.
2.  **[yt-dlp](https://github.com/yt-dlp/yt-dlp)**: Engine behind supporting 1000+ video platforms.

> [!TIP]
> The TUI automatically checks for these dependencies on startup and will prompt you to download stand-alone binaries dynamically if missing.

---

## 📦 Installing the TUI

### 1. Via Cargo (Recommended)
Install the TUI to your system path:
```bash
cargo install mangofetch
```

### 2. Building from Source
If you prefer to build the latest version from the `main` branch:

```bash
# Clone the repository
git clone https://github.com/julesklord/mangofetch.git
cd mangofetch

# Build the TUI
cargo build --release -p mangofetch

# The binary will be located at:
# ./target/release/mangofetch
```

### 3. Running the TUI
Simply run `mangofetch` without arguments to launch the TUI directly:
```bash
mangofetch
```

---

## 📚 Using mangofetch-core as a Rust Library

Integrate `mangofetch-core` directly into bots, servers, or custom software:

Add this to `Cargo.toml`:
```toml
[dependencies]
mangofetch-core = "0.7.4"
```
Or use the development Git version:
```toml
[dependencies]
mangofetch-core = { git = "https://github.com/julesklord/mangofetch", branch = "main" }
```

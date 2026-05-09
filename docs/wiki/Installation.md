# 🚀 Installation Guide

MangoFetch is designed to be lightweight and portable, but it relies on a few high-quality external tools to handle specific media formats.

## 📋 System Requirements

*   **Operating Systems**: Windows 10/11, Linux, macOS.
*   **Rust (optional)**: Required if you are building from source.
*   **Python 3**: Required for `yt-dlp` functionality.

---

## 🛠️ Mandatory Dependencies

MangoFetch uses the best-in-class engines for media processing. Ensure these are installed and in your system `PATH`:

1.  **[FFmpeg](https://ffmpeg.org/)**: Essential for merging audio/video streams and transcoding.
2.  **[yt-dlp](https://github.com/yt-dlp/yt-dlp)**: The engine behind supporting 1000+ video platforms.

> [!TIP]
> MangoFetch can automatically check for these dependencies using the `mangofetch check` command.

---

## 📦 Installing MangoFetch

### 1. Pre-built Binaries
Download the latest release for your platform from the [Releases](https://github.com/julesklord/mangofetch-cli/releases) page.
1.  Extract the ZIP/Tarball.
2.  Move the `mangofetch` executable to a folder in your `PATH` (e.g., `/usr/local/bin` on Linux or a dedicated `C:\Tools` folder on Windows).

### 2. Building from Source
If you prefer to build the latest version from the `main` branch:

```bash
# Clone the repository
git clone https://github.com/julesklord/mangofetch.git
cd mangofetch

# Build the CLI
cargo build --release -p mangofetch-cli

# The binary will be located at:
# ./target/release/mangofetch
```

---

## ✅ Verifying Installation

Run the following command to ensure everything is set up correctly:

```bash
mangofetch check
```

> [!SCREENSHOT_PLACEHOLDER: Dependency Check Output]
> *Captura de la salida del comando 'mangofetch check' mostrando los estados en verde de FFmpeg y yt-dlp.*

---

## 🐚 Setup Alias (Optional but Recommended)

For power users, we recommend adding an alias to your shell configuration (`.bashrc`, `.zshrc`, or PowerShell Profile):

**PowerShell**:
```powershell
Set-Alias -Name mango -Value mangofetch
```

**Bash/Zsh**:
```bash
alias mango='mangofetch'
```

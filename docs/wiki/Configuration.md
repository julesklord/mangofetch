# ⚙️ Configuration Guide

MangoFetch is highly configurable, allowing you to fine-tune both the engine's performance and the interface's appearance. Settings are stored in a `settings.json` file in your application data directory.

## 📂 Locating Settings

You can find your configuration file at:
*   **Windows**: `%APPDATA%\mangofetch\settings.json`
*   **Linux/macOS**: `~/.config/mangofetch/settings.json` (or following XDG specs)

---

## 🎨 Appearance Settings

| Option | Description | Values |
| :--- | :--- | :--- |
| **TUI Theme** | The color palette used in interactive mode. | `mango`, `pitaya`, `coconut`, `guava`, `papaya`, `passionfruit`, `lychee` |
| **Nerd Fonts** | Enables high-quality icons in the terminal. | `ON` / `OFF` |
| **Language** | System language (Core engine). | `en`, `es`, etc. |

---

## ⬇️ Download Settings

| Option | Description | Default |
| :--- | :--- | :--- |
| **Default Quality** | Preferred video resolution. | `best`, `1080p`, `720p`, `480p`, `360p` |
| **Organize Platforms** | Save files in subfolders named after the platform. | `OFF` |
| **Skip Existing** | Skip downloading if a file with the same name exists. | `ON` |
| **Embed Metadata** | Add title, author, and tags to the media file. | `ON` |
| **Embed Thumbnail** | Add the video thumbnail as album art/cover. | `ON` |
| **Download Subtitles** | Attempt to fetch and embed subtitles. | `OFF` |
| **YouTube SponsorBlock**| Skip sponsored segments in YouTube videos. | `OFF` |

---

## ⚡ Advanced Engine Settings

These settings affect the speed and stability of the download engine.

| Option | Description | Typical Value |
| :--- | :--- | :--- |
| **Max Downloads** | Number of files to download simultaneously. | `2` - `5` |
| **Max Segments** | Parallel segments for direct (HTTP) downloads. | `8` - `32` |
| **Concurrent Fragments**| Fragments for HLS/DASH streams (yt-dlp). | `4` - `16` |
| **Stagger Delay** | Milliseconds to wait between starting connections. | `150ms` |
| **Retry Count** | Number of attempts if a download fails. | `3` |

---

## 🌐 Proxy Configuration

MangoFetch can route all traffic through a proxy for privacy or access.
*   **Enabled**: `ON` / `OFF`
*   **Type**: `http`, `socks5`, `socks5h`
*   **Host/Port**: Your proxy server details.

---

## 🛠️ Managing Config via CLI

You can update any setting without opening the JSON file using the `config` command:

```bash
# Get current value
mangofetch config get download.video_quality

# Set a new value
mangofetch config set advanced.max_concurrent_downloads 4
```

> [!SCREENSHOT_PLACEHOLDER: TUI Settings Tab]
> *Captura de la pestaña de Settings en la TUI mostrando la lista de opciones dinámica.*

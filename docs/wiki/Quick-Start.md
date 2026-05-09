# ⚡ Quick Start Guide

Start downloading your favorite media with MangoFetch in less than a minute.

## 1. Verify Dependencies
First, ensure you have **FFmpeg** and **yt-dlp** installed. Run:
```bash
mangofetch check
```

## 2. Your First Download
Download a video from YouTube (or any of the 1000+ supported sites):
```bash
mangofetch download "https://www.youtube.com/watch?v=aqz-KE-bpKQ"
```
*By default, this will download the best quality available to your user's download directory.*

## 3. Advanced CLI Download
Want audio only in a specific folder?
```bash
mangofetch download "URL" --audio-only --output ./Music
```

## 4. Enter Interactive Mode (TUI)
For a more visual experience, launch the dashboard:
```bash
mangofetch tui
```
> [!SCREENSHOT_PLACEHOLDER: TUI Queue Tab with an Active Download]
> *Captura de la pestaña Queue mostrando una descarga activa con la nueva barra de progreso.*

### Basic TUI Navigation:
*   **`Tab`**: Switch between Queue, Settings, and Logs.
*   **`/`**: Enter command mode (type `:q` to exit).
*   **`p`**: Pause a download.
*   **`r`**: Resume a download.

## 5. Change the Theme
Don't like orange? Switch to **Passionfruit** (Yellow/Purple):
```bash
mangofetch config set appearance.tui_theme passionfruit
```
*Or change it directly in the **Settings** tab of the TUI.*

---

**Ready for more?**
*   Check the [CLI Reference](CLI-Guide.md) for power-user flags.
*   Explore the [TUI Experience](TUI-Experience.md) for all shortcuts.

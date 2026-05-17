# 🖥️ TUI Experience Guide

MangoFetch features a state-of-the-art Terminal User Interface (TUI) built with `ratatui`. It provides a full-screen, interactive dashboard to manage your downloads with ease and style.

## 🚀 Launching the TUI

To enter interactive mode, run:
```bash
mangofetch tui
```

> [!SCREENSHOT_PLACEHOLDER: TUI Splash Screen]
> *Captura de la pantalla de bienvenida (Splash Screen) con el logo estilizado de Mango.*

---

## 🧭 Navigation

### Tabs
The TUI is organized into several functional tabs. You can navigate them using:
*   **`Tab`**: Move to the next tab.
*   **`Shift + Tab`**: Move to the previous tab.
*   **`1-6`**: Directly jump to a specific tab.

| Tab | Icon | Description |
| :--- | :---: | :--- |
| **Home** | 🏠 | Quick actions and project summary. |
| **Queue** | ⬇ | Active downloads and real-time progress. |
| **History** | 📜 | Completed and failed downloads. |
| **Settings** | ⚙ | Interactive configuration manager. |
| **About** | ℹ | Detailed project info and debug data. |
| **Logs** | 📋 | Real-time internal engine logs. |

---

## 🖱️ Mouse Support

As of **v0.5.0**, MangoFetch supports intuitive mouse interactions:
*   **Scrolling**: Use the scroll wheel to navigate tables or log views.
*   **Clicking**: Click on the tab headers at the top to switch views instantly.

---

## ⌨️ Shortcuts & Controls

### General Navigation
*   **`Tab`**: Move to the next tab (or next category in Queue/History).
*   **`Shift + Tab`**: Move to the previous tab.
*   **`1-6`**: Directly jump to a specific tab (1=Home, 2=Queue, 3=History, 4=Settings, 5=About, 6=Logs).
*   **`g`**: Go to first item (or top of logs).
*   **`G`**: Go to last item (or bottom of logs).

### Queue/History Actions
*   **`j` / `Down`**: Select the next item.
*   **`k` / `Up`**: Select the previous item.
*   **`p`**: Pause the selected download.
*   **`r`**: Resume the selected download.
*   **`x`**: Remove the selected item (requires confirmation).
*   **`a` / `n`**: Open the **Add Download** modal (Paste URL).

### Special Actions
*   **`q` (press twice quickly)**: Quit the application (double-q quit).
*   **`?`**: Toggle the **Help Modal**.
*   **`l`**: Toggle layout (change view mode).
*   **`:` / `/`**: Enter command mode (Vim-style).

### Settings Tab
*   **`[` / `]`**: Reorder statusbar modules.
*   **`Enter` / `Left` / `Right`**: Toggle a setting.

> [!SCREENSHOT_PLACEHOLDER: TUI Help Modal Overlay]
> *Captura del modal de ayuda desplegado sobre la pestaña de Queue.*

---

## 🍓 Tropical Themes

MangoFetch allows you to customize the color palette using our collection of tropical fruit themes. Change these in the **Settings** tab.

*   **🥭 Mango**: Warm oranges and golds (Default).
*   **🍇 Pitaya**: Vibrant pinks and deep purples.
*   **🥥 Coconut**: Natural beiges and browns.
*   **🍐 Guayaba**: Fresh pink flesh and green skin.
*   **🟠 Papaya**: Vibrant orange and dark seed accents.
*   **🌺 Passionfruit**: Electric yellow and deep passion purple.
*   **🍒 Lychee**: Translucent whites and vibrant red skin.
*   **⭐ Starfruit**: Bright yellow and green edges.
*   **🍇 Mangosteen**: Dark purple skin and pure white flesh.
*   **🥝 Kiwi**: Fuzzy brown and bright green.

> [!SCREENSHOT_PLACEHOLDER: TUI Gallery - All Themes Comparison]
> *Un collage mostrando la misma pantalla con los diferentes temas de frutas tropicales.*

---

## 🎬 Animations Toggle

As of **v0.5.5**, MangoFetch allows you to enable or disable TUI animations. This is useful for:
*   Users on slower terminals
*   Users who prefer a static interface
*   Reducing visual noise during intensive operations

Toggle animations in the **Settings** tab or via:
```bash
mangofetch config set appearance.animations true
```

---

## ⌨️ Command Mode (Vim-style)

For power users who want CLI speed within the TUI:
1.  Press **`/`** to enter Command Mode.
2.  Type your command (e.g., `:download https://...`).
3.  Press **`Enter`** to execute.

**Available Commands**:
*   `:d <URL> [quality]`: Queue a new download.
*   `:q`: Quit MangoFetch.
*   `:help`: Open the help modal.
*   `:clear`: Clear the logs (while in Logs tab).

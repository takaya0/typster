# Inverse Search (PDF → Source) Setup Guide

[日本語版](../ja/inverse-search.md)

## Overview

**Inverse Search** is a feature where clicking in a PDF viewer causes Zed to jump to the corresponding line in the Typst source file.

Typster supports two preview modes:

- **Browser preview** (recommended, no configuration needed): If no external viewer is detected, tinymist's background preview server is automatically enabled. Open `http://127.0.0.1:23635` in a browser (Chrome, etc.) to get SVG-based preview. Forward and inverse search work automatically over WebSocket.
- **External viewer** (manual configuration required): If you install a PDF viewer such as Skim, SumatraPDF, or Zathura, it is detected automatically. Forward search is configured automatically, but inverse search requires you to register the editor command in the viewer.

> **Browser preview requirement**: tinymist version 0.13.6 or later is required. If an older tinymist is on your PATH, update it or set `lsp.tinymist.binary.path` to the path of a newer binary.

### Specifying the Previewer

You can explicitly select which previewer to use by setting `typsterPreviewer` in Zed's `settings.json`:

```json
{
  "lsp": {
    "tinymist": {
      "settings": {
        "typsterPreviewer": "browser"
      }
    }
  }
}
```

Available values:

| Value | Description |
|-------|-------------|
| `"auto"` | Auto-detect (default). Falls back to built-in if no external viewer is found. |
| `"browser"` | tinymist browser preview (works with any browser such as Chrome) |
| `"skim"` | Skim (macOS) |
| `"sumatrapdf"` | SumatraPDF (Windows) |
| `"zathura"` | Zathura (Linux) |
| `"sioyek"` | Sioyek (cross-platform) |
| `"okular"` | Okular (Linux / KDE) |
| `"evince"` | Evince (Linux / GNOME) |

> **Note**: If you specify an external viewer that is not installed, Typster automatically falls back to browser preview.

### Using Browser Preview

1. Set `typsterPreviewer: "browser"` or open a Typst file without any external PDF viewer installed.
2. Open `http://127.0.0.1:23635` in a browser (Chrome, etc.).
3. Real-time preview without exporting a PDF.
4. Click anywhere in the browser to jump to the corresponding line in Zed (Inverse Search).
5. Moving the cursor in Zed scrolls the browser (Forward Search).

> **Note**: If you partially override the `preview` key in user settings, the entire auto-configured `preview` object is replaced. This is the same behaviour as `forwardSearch`. If you want to change other keys while keeping `background.enabled`, configure the entire `preview` object explicitly.

---

### Why External Viewers Require Manual Setup

Inverse search works by having the viewer call the editor. Therefore, **you must register the editor command in each viewer's application settings**. Typster cannot automate this step.

## Prerequisites

The `zed` CLI must be on your PATH. Run **Zed > Install CLI** from the Zed app menu. Verify with:

```sh
which zed
# → /usr/local/bin/zed or similar
```

---

## Per-Viewer Setup Instructions

### Skim (macOS)

1. Open Skim and go to **Skim > Preferences**
2. Select the **Sync** tab
3. Set the Preset in the **PDF-TeX Sync Support** section to **Custom**
4. Enter the following:
   - **Command**: `zed`
   - **Arguments**: `%file:%line`
5. Close Preferences

`⌘` + click on the PDF to open the corresponding line in Zed.

---

### SumatraPDF (Windows)

1. Open SumatraPDF and go to **Settings > Options**
2. In the **Set inverse search command line** field, enter:

   ```
   zed "%f:%l"
   ```

3. Click **OK**

Double-click on the PDF to open the corresponding line in Zed.

---

### Zathura (Linux)

Add the following to `~/.config/zathura/zathurarc`:

```
set synctex-editor-command "zed \"%{input}:%{line}\""
```

Restart Zathura after saving. `Ctrl` + click on the PDF to open the corresponding line in Zed.

---

### Sioyek (cross-platform)

Add the following to Sioyek's user preferences file (`prefs_user.config`):

```
inverse_search_command  zed "%1:%2"
```

File locations:
- **Windows**: `%APPDATA%\sioyek\prefs_user.config`
- **macOS**: `~/Library/Application Support/sioyek/prefs_user.config`
- **Linux**: `~/.config/sioyek/prefs_user.config`

Restart Sioyek after saving.

---

### Okular (Linux / KDE)

1. Open Okular and go to **Settings > Configure Okular**
2. Select the **Editor** tab
3. In the **Editor** dropdown, select **Custom Text Editor**
4. In the **Command** field, enter:

   ```
   zed %f:%l
   ```

5. Click **OK**

Clicking on the PDF opens the corresponding line in Zed.

---

### Evince (Linux / GNOME)

Evince implements inverse search via D-Bus and does not provide a direct editor command setting. Inverse search with Evince is not supported. We recommend using Zathura or Okular instead.

---

## Troubleshooting

### Clicking does not open Zed

- Check that the `zed` CLI is on PATH: `which zed`
- Double-check that the viewer settings have been saved

### Multiple Zed windows are open

`zed path:line` opens the file in an existing window. Which window comes to the foreground depends on your window manager behaviour.

### File path contains spaces

Some viewers require quoting commands (e.g., `"%f:%l"` in SumatraPDF). The examples above include quotes where necessary — copy them as-is.

### Line numbers are offset in Skim

Typst SyncTeX information is updated when the file is saved and the PDF is regenerated. Save and rebuild, then try again.

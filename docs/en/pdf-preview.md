# PDF Preview & Forward Search

[日本語版](../ja/pdf-preview.md)

Typster provides two PDF preview modes, selected automatically based on what is installed on your system.

---

## Browser Preview (Default)

When no supported external viewer is found, Typster enables tinymist's built-in browser preview server.

**Requirements**: tinymist 0.13.6 or later (automatically downloaded if not already cached).

**How to use**:
1. Open a `.typ` file in Zed.
2. Open your browser and navigate to `http://127.0.0.1:23635`.
3. The preview updates in real time as you type.
4. **Forward search**: move your cursor in Zed — the browser scrolls to the corresponding location automatically.
5. **Inverse search**: click anywhere in the browser — Zed jumps to the corresponding source line.

> No additional configuration is needed for browser preview. Forward and inverse search work out of the box via WebSocket.

---

## External Viewer Preview

If a supported PDF viewer is installed, Typster automatically:
- detects it during startup
- configures `forwardSearch.command` and `forwardSearch.args` for that viewer
- sets `exportPdf: "onSave"` so a PDF is available for the viewer

### Viewer Detection Priority

Viewers are detected in this order. The first one found is used.

| Priority | Viewer | Platform | Detection method |
|----------|--------|----------|-----------------|
| 1 | Skim | macOS | `/Applications/Skim.app/Contents/SharedSupport/displayline` exists, or `skimapp` on PATH |
| 2 | SumatraPDF | Windows | `SumatraPDF` on PATH |
| 3 | Zathura | Linux / cross-platform | `zathura` on PATH |
| 4 | Sioyek | cross-platform | `sioyek` on PATH |
| 5 | Okular | Linux / KDE | `okular` on PATH |
| 6 | Evince | Linux / GNOME | `evince` on PATH |

### Auto-Configured Forward Search Arguments

Typster sets these `forwardSearch` arguments automatically:

| Viewer | Command example |
|--------|----------------|
| Skim | `displayline %l %p %i` |
| SumatraPDF | `SumatraPDF -forward-search %i %l %p` |
| Zathura | `zathura --synctex-forward %l:1:%i %p` |
| Sioyek | `sioyek --reuse-window --execute-command toggle_synctex --forward-search-file %i --forward-search-line %l --open %p` |
| Okular | `okular --unique file:%p#src:%l%i` |
| Evince | `evince --forward-search %i %l %p` |

**Placeholder meanings**:
- `%l` — current line number
- `%p` — path to the compiled PDF file
- `%i` — path to the Typst source file

---

## Choosing a Previewer

Override automatic detection with `typsterPreviewer` in your settings:

```jsonc
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

| Value | Behaviour |
|-------|-----------|
| `"auto"` | Detect installed viewers in priority order; fall back to browser if none found (default) |
| `"browser"` | Always use browser preview, even if an external viewer is installed |
| `"skim"` | Use Skim; fall back to browser if not installed |
| `"sumatrapdf"` | Use SumatraPDF; fall back to browser if not installed |
| `"zathura"` | Use Zathura; fall back to browser if not installed |
| `"sioyek"` | Use Sioyek; fall back to browser if not installed |
| `"okular"` | Use Okular; fall back to browser if not installed |
| `"evince"` | Use Evince; fall back to browser if not installed |

---

## Inverse Search (PDF → Source)

For browser preview, inverse search works automatically.

For external viewers, you must configure the viewer to call `zed` when you click in the PDF. See [inverse-search.md](inverse-search.md) for step-by-step instructions for each viewer.

---

## Overriding Preview Settings

Because Typster deep-merges user settings with auto-detected values, you can override specific keys without losing the rest. For example, to change only the refresh mode while keeping browser preview enabled:

```jsonc
{
  "lsp": {
    "tinymist": {
      "settings": {
        "preview": {
          "refresh": "onSave"
        }
      }
    }
  }
}
```

See [configuration.md](configuration.md) for the full deep-merge explanation.

# Configuration Reference

[日本語版](../ja/configuration.md)

All Typster settings live in Zed's `settings.json`. Open it with `zed: open settings` from the command palette.

Settings can be placed at the top level (global) or inside a `.zed/settings.json` file in your project root (project-scoped).

---

## tinymist Binary

Control which `tinymist` binary is used. Typster resolves the binary in this priority order:

1. `lsp.tinymist.binary.path` (if set)
2. `tinymist` found on system PATH
3. Previously cached downloaded binary
4. Latest GitHub release from `Myriad-Dreamin/tinymist` (downloaded automatically)

```jsonc
{
  "lsp": {
    "tinymist": {
      "binary": {
        // Absolute path to a specific tinymist binary.
        // Use this to pin a version or point to a custom build.
        "path": "/usr/local/bin/tinymist",

        // Extra arguments passed to tinymist on startup.
        "arguments": []
      }
    }
  }
}
```

> **macOS note**: Typster supports macOS on Apple Silicon (arm64) only. Intel Mac users must install tinymist manually and set `binary.path`.

---

## Typster-Specific Settings

`typsterPreviewer` is the only key Typster owns. It is **consumed by Typster and never forwarded to tinymist**.

```jsonc
{
  "lsp": {
    "tinymist": {
      "settings": {
        "typsterPreviewer": "auto"
      }
    }
  }
}
```

| Value | Description |
|-------|-------------|
| `"auto"` | Auto-detect an installed viewer (default). Falls back to browser preview if none found. |
| `"browser"` | Force tinymist's built-in browser preview regardless of installed viewers. |
| `"skim"` | Skim (macOS) |
| `"sumatrapdf"` | SumatraPDF (Windows) |
| `"zathura"` | Zathura (Linux) |
| `"sioyek"` | Sioyek (cross-platform) |
| `"okular"` | Okular (Linux / KDE) |
| `"evince"` | Evince (Linux / GNOME) |

> If you specify an external viewer that is not installed, Typster automatically falls back to browser preview.

---

## Auto-Detected tinymist Settings

Typster automatically sets these tinymist keys based on your environment. You can override any of them with explicit values in `settings`.

| Key | Auto-detected value | Condition |
|-----|---------------------|-----------|
| `exportPdf` | `"onSave"` | External viewer detected |
| `exportPdf` | `"never"` | Browser preview mode |
| `formatterMode` | `"typstyle"` | `typstyle` found on PATH |
| `formatterMode` | `"typstfmt"` | `typstfmt` found on PATH (typstyle not found) |
| `semanticTokens` | `"enable"` | Always (default) |
| `preview.background.enabled` | `true` | Browser preview mode |
| `preview.refresh` | `"onType"` | Browser preview mode |
| `forwardSearch.command` | viewer executable | External viewer detected |
| `forwardSearch.args` | viewer-specific args | External viewer detected |

For the full list of tinymist settings, see the [tinymist configuration reference](https://github.com/Myriad-Dreamin/tinymist).

---

## Deep Merge Behavior

User settings are **deep-merged** on top of auto-detected settings — they do not replace the entire auto-detected configuration.

**Example**: Typster auto-detects browser preview and sets:

```json
{
  "preview": {
    "background": { "enabled": true },
    "refresh": "onType"
  }
}
```

If you only want to change `refresh` to `"onSave"`, you can write just that key:

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

The result is a merge — `preview.background.enabled` remains `true` and only `refresh` is overridden. This is the expected behavior as of the fix for [issue #15](https://github.com/yataka/typster/issues/15).

For non-object values (strings, numbers, arrays), user settings always win outright.

---

## Initialization Options

`initialization_options` are sent to tinymist at startup (separate from runtime `settings`).

```jsonc
{
  "lsp": {
    "tinymist": {
      "initialization_options": {
        // tinymist initialization options here
      }
    }
  }
}
```

See [tinymist's documentation](https://github.com/Myriad-Dreamin/tinymist) for available initialization options.

---

## Complete Example

A typical `settings.json` showing common customizations:

```jsonc
{
  "lsp": {
    "tinymist": {
      "binary": {
        // Optional: pin to a specific binary
        "path": "/usr/local/bin/tinymist"
      },
      "settings": {
        // Force browser preview (skip external viewer detection)
        "typsterPreviewer": "browser",

        // Override formatter (auto-detected by default)
        "formatterMode": "typstyle",

        // Change preview refresh mode
        "preview": {
          "refresh": "onSave"
        },

        // Change PDF export trigger
        "exportPdf": "onType"
      }
    }
  }
}
```

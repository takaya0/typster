# Quickstart

[日本語版](../ja/quickstart.md)

Get Typster running in 3 steps.

## Prerequisites

- [Zed](https://zed.dev) editor installed
- [Typst](https://typst.app) 0.14.2 or later (for the `typst` CLI and build tasks)
- tinymist is **downloaded automatically** — no manual installation needed

## Step 1: Install Typster

1. Open Zed
2. Press **⇧⌘X** or run `zed: extensions` from the command palette
3. Search for **Typster**
4. Click **Install**

## Step 2: Open a Typst File

Create or open any file with the `.typ` extension. Zed will activate Typster automatically.

You should see **"Typst language support"** appear in the Zed status bar, indicating that tinymist has started. Completions, hover documentation, and diagnostics are now active.

> **First run**: On the first open, Typster downloads the latest `tinymist` binary from GitHub. This takes a few seconds. Subsequent opens use the cached binary.

## Step 3: Preview Your Document

Typster configures PDF preview automatically based on what is installed on your system.

**No external PDF viewer installed (default)**

Typster enables tinymist's built-in browser preview. Open your browser and navigate to:

```
http://127.0.0.1:23635
```

The preview updates in real time as you type. Clicking in the browser jumps to the corresponding line in Zed (inverse search), and moving the cursor in Zed scrolls the browser (forward search).

**External PDF viewer installed**

If Skim (macOS), SumatraPDF (Windows), Zathura, Sioyek, Okular, or Evince is installed, Typster detects it automatically and:
- exports a PDF on every save (`exportPdf: "onSave"`)
- configures forward search to open the PDF at the current cursor position

See [pdf-preview.md](pdf-preview.md) for the full detection priority order and configuration options.

## What Happens Behind the Scenes

**tinymist binary resolution** (in priority order):
1. `lsp.tinymist.binary.path` in your Zed settings
2. `tinymist` found on your system PATH
3. Previously cached downloaded binary
4. Download the latest release from GitHub (`Myriad-Dreamin/tinymist`)

**Formatter auto-detection**: If `typstyle` or `typstfmt` is found on PATH, Typster sets `formatterMode` automatically (`typstyle` takes priority). Install either tool to enable formatting with **Format Document** (⌥⇧F).

## Next Steps

- [features.md](features.md) — full list of what Typster provides
- [configuration.md](configuration.md) — customize LSP, preview, and formatter settings
- [slash-commands.md](slash-commands.md) — use `/typst-docs` and `/typst-symbols` in the AI assistant
- [build-tasks.md](build-tasks.md) — compile and watch tasks

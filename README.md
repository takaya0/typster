# Typster

Typst language support for [Zed](https://zed.dev) — powered by [tinymist](https://github.com/Myriad-Dreamin/tinymist) LSP with PDF preview, forward/inverse search, and build tasks.

> **Requirements**: Typst 0.14.2 or later. tinymist is downloaded automatically if not found on PATH.

## Key Features

- **LSP via tinymist** — completions, hover docs, diagnostics, go-to-definition, rename, code actions
- **Syntax highlighting** — Tree-sitter grammar with semantic tokens enabled by default
- **Code labels** — rich completion and symbol labels with syntax-highlighted signatures
- **PDF preview** — browser preview (zero config) or external viewers (Skim, SumatraPDF, Zathura, Sioyek, Okular, Evince)
- **Forward search** — jump from Zed cursor position to the corresponding PDF location
- **Inverse search** — click in PDF to jump back to the source line in Zed
- **Auto-formatting** — detects `typstyle` or `typstfmt` on PATH automatically
- **Build tasks** — compile to PDF/SVG/PNG, watch mode, font listing via Zed's task runner
- **Slash commands** — `/typst-docs` and `/typst-symbols` inject Typst reference into the AI assistant

## Quick Install

1. Open Zed → **Extensions** (⇧⌘X or `zed: extensions` in the command palette)
2. Search for **Typster** and click **Install**
3. Open any `.typ` file — the language server starts automatically

## Documentation

| Topic | English | 日本語 |
|-------|---------|--------|
| Quickstart | [docs/en/quickstart.md](docs/en/quickstart.md) | [docs/ja/quickstart.md](docs/ja/quickstart.md) |
| Features | [docs/en/features.md](docs/en/features.md) | [docs/ja/features.md](docs/ja/features.md) |
| Configuration | [docs/en/configuration.md](docs/en/configuration.md) | [docs/ja/configuration.md](docs/ja/configuration.md) |
| PDF Preview & Forward Search | [docs/en/pdf-preview.md](docs/en/pdf-preview.md) | [docs/ja/pdf-preview.md](docs/ja/pdf-preview.md) |
| Inverse Search | [docs/en/inverse-search.md](docs/en/inverse-search.md) | [docs/ja/inverse-search.md](docs/ja/inverse-search.md) |
| Slash Commands | [docs/en/slash-commands.md](docs/en/slash-commands.md) | [docs/ja/slash-commands.md](docs/ja/slash-commands.md) |
| Build Tasks | [docs/en/build-tasks.md](docs/en/build-tasks.md) | [docs/ja/build-tasks.md](docs/ja/build-tasks.md) |
| Troubleshooting | [docs/en/troubleshooting.md](docs/en/troubleshooting.md) | [docs/ja/troubleshooting.md](docs/ja/troubleshooting.md) |

## Acknowledgments

Typster builds on the work of several excellent open-source projects, and we are grateful to their authors and contributors.

**Foundation**

- [zed-extensions/typst](https://github.com/zed-extensions/typst) — served as an early foundation and Zed-specific reference for this extension.

**Core Language Server**

- [Myriad-Dreamin/tinymist](https://github.com/Myriad-Dreamin/tinymist) — powers Typster's language features via the integrated Typst LSP server.

**Syntax & Highlighting**

- [uben0/tree-sitter-typst](https://github.com/uben0/tree-sitter-typst) — provides the Tree-sitter grammar used by the extension; highlight queries were adapted from its editor integrations.
- [WeetHet/typst.zed](https://github.com/WeetHet/typst.zed) — the syntax highlight queries are based on this project.

**PDF Preview & Forward Search**

- [niclas-timm/zed-latex](https://github.com/niclas-timm/zed-latex) — the PDF previewer priority order was adapted from this project.
- [James-Yu/LaTeX-Workshop](https://github.com/James-Yu/LaTeX-Workshop) — inspiration for the PDF preview and forward-search workflow design.

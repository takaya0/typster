# Features

[日本語版](../ja/features.md)

## Language Server (tinymist)

Typster integrates [tinymist](https://github.com/Myriad-Dreamin/tinymist), a full-featured Typst LSP server, providing:

- **Completions** — function names, parameters, symbols, and package imports
- **Hover documentation** — inline docs for functions and variables
- **Diagnostics** — compile errors and warnings shown inline
- **Go to definition** — jump to the definition of any function or variable
- **Find references** — list all usages of a symbol across your project
- **Rename symbol** — safely rename functions, variables, and labels
- **Code actions** — quick fixes and refactoring suggestions
- **Signature help** — parameter hints as you type function calls

For the complete list of tinymist capabilities, see [tinymist's documentation](https://github.com/Myriad-Dreamin/tinymist).

## Syntax Highlighting

Tree-sitter based grammar ([uben0/tree-sitter-typst](https://github.com/uben0/tree-sitter-typst)) provides:

- Accurate highlighting for markup, math, and code modes
- Code folding at headings, functions, and blocks
- Smart indentation
- Text objects for paragraph and function selection
- Document outline (headings shown in Zed's outline panel)
- Runnables (build task buttons on `.typ` files)

**Semantic tokens** are enabled by default, adding LSP-driven highlighting on top of the Tree-sitter layer for more accurate type and variable colouring. Override with `semanticTokens: "disable"` in your settings if needed.

## Code Labels

Typster provides rich completion and symbol labels. Completion items in the autocomplete popup show syntax-highlighted signatures, making it easier to distinguish between functions, variables, and symbols without opening their hover documentation.

## PDF Preview

Two preview modes are available, selected automatically based on installed software:

| Mode | When active | URL / trigger |
|------|-------------|---------------|
| Browser preview | No external viewer found (default) | `http://127.0.0.1:23635` |
| External viewer | Skim / SumatraPDF / Zathura / Sioyek / Okular / Evince detected | PDF opened automatically |

See [pdf-preview.md](pdf-preview.md) for setup details and the viewer detection priority order.

## Forward Search

Jump from your Zed cursor position to the corresponding location in the PDF:

- **Browser preview**: happens automatically via WebSocket — moving the cursor scrolls the browser.
- **External viewer**: triggered by the viewer-specific shortcut (configured automatically by Typster).

## Inverse Search (PDF → Source)

Click in the PDF to jump back to the matching source line in Zed:

- **Browser preview**: click anywhere in the browser preview.
- **External viewers**: requires a one-time manual configuration of the viewer. See [inverse-search.md](inverse-search.md).

## Formatting

Typster detects code formatters on PATH and configures tinymist automatically:

| Formatter | Detection | Priority |
|-----------|-----------|----------|
| [typstyle](https://github.com/Enter-tainer/typstyle) | `typstyle` binary on PATH | Higher |
| [typstfmt](https://github.com/astrale-sharp/typstfmt) | `typstfmt` binary on PATH | Lower |

Format the current document with **⌥⇧F** or `editor: format`. You can override the detected formatter with `formatterMode` in your settings.

## Build Tasks

Six tasks are provided via Zed's task runner:

| Task | Description |
|------|-------------|
| `typst compile` | Compile to default output |
| `typst watch` | Continuous compilation on save |
| `typst compile (to PDF)` | Explicit PDF output |
| `typst compile (to SVG)` | SVG output |
| `typst compile (to PNG)` | PNG output |
| `typst fonts` | List available fonts and variants |

See [build-tasks.md](build-tasks.md) for usage details.

## Slash Commands

Two slash commands are available in Zed's AI assistant panel:

| Command | Description |
|---------|-------------|
| `/typst-docs <function>` | Show the signature, parameters, and examples for a Typst standard library function |
| `/typst-symbols <category>` | Show math symbols in a category (arrow, greek, operator, relation, set, logic, accent, misc) |

See [slash-commands.md](slash-commands.md) for usage examples.

# Build Tasks

[日本語版](../ja/build-tasks.md)

Typster provides six Zed tasks for compiling and working with Typst files.

## Available Tasks

| Task | Command | Description |
|------|---------|-------------|
| `typst compile` | `typst compile <file>` | Compile to the default output format |
| `typst watch` | `typst watch <file>` | Watch mode — recompile automatically on every save |
| `typst compile (to PDF)` | `typst compile --format pdf <file>` | Compile explicitly to PDF |
| `typst compile (to SVG)` | `typst compile --format svg <file>` | Compile to SVG |
| `typst compile (to PNG)` | `typst compile --format png <file>` | Compile to PNG |
| `typst fonts` | `typst fonts --variants` | List all available fonts and their variants |

The `<file>` placeholder is replaced by the path of the currently open file when the task runs.

## How to Run a Task

**From the command palette**:
1. Press **⌘⇧P** to open the command palette
2. Type `task: spawn`
3. Select the task from the list

**From the Runnables button**:
Any `.typ` file shows a ▶ (play) button in the editor gutter. Clicking it runs the default `typst compile` task for that file.

## Requirements

The `typst` CLI must be installed and available on your PATH. Install it from [typst.app](https://typst.app) or via a package manager:

```sh
# Homebrew (macOS)
brew install typst

# Cargo
cargo install --git https://github.com/typst/typst --locked typst-cli
```

Verify the installation:
```sh
typst --version
```

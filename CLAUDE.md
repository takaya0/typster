# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Typster is a Zed editor extension providing Typst language support. It integrates the tinymist LSP server, PDF preview with forward search, and build tasks. Written in Rust, compiled to `wasm32-wasip1` for the Zed extension runtime.

## Build & Test Commands

```bash
# Build for Zed extension (WASM target)
cargo build --target wasm32-wasip1

# Build for native (used for testing)
cargo build

# Run all tests (native target only, not WASM)
cargo test

# Run a single test
cargo test <test_name>
```

Rust edition 2024, MSRV 1.94.0. No custom linter or formatter configs exist.

## Architecture

The extension uses `#[cfg(target_arch = "wasm32")]` to separate Zed API code (WASM-only) from pure logic (testable on native).

### Key Modules

- **`src/lib.rs`** — Entry point. Registers `TypsterExtension` implementing `zed::Extension` trait with three callbacks: `language_server_command`, `language_server_initialization_options`, `language_server_workspace_configuration`.

- **`src/platform.rs`** — `Environment` trait abstraction (`which()`, `path_exists()`). `WorktreeEnv` wraps Zed's Worktree for production; `FakeEnv` is the test double for native tests.

- **`src/tinymist_invocation.rs`** — Resolves the tinymist LSP binary with 4-step fallback: user config → system PATH → cached download → GitHub release download.

- **`src/tinymist_config/mod.rs`** — Builds workspace config. Auto-detects formatter (`typstyle`/`typstfmt`), merges user settings. Defaults: `exportPdf: "onSave"`, `semanticTokens: "enable"`.

- **`src/tinymist_config/preview_presets.rs`** — Detects installed PDF previewers by platform priority (Skim → SumatraPDF → Zathura → Sioyek → Okular → Evince) and configures forward search arguments.

- **`languages/typst/`** — Tree-sitter queries (highlights, folds, indents, brackets, injections, outline, runnables, textobjects) and language config.

### Testability Pattern

All business logic is testable on native target via the `Environment` trait. `FakeEnv` simulates binary availability and path existence without WASM. Tests live alongside the modules they test.

# Slash Commands

[日本語版](../ja/slash-commands.md)

Typster provides two slash commands for Zed's AI assistant panel. They inject Typst reference documentation as context, helping the assistant answer questions about Typst syntax and functions accurately.

## Usage

Open the AI assistant panel in Zed and type `/` to see available slash commands. Typster adds:

- `/typst-docs`
- `/typst-symbols`

Both commands support tab completion for their arguments.

---

## `/typst-docs <function>`

Shows the signature, parameters, and usage examples for a Typst standard library function.

**Example**:
```
/typst-docs heading
```

This injects the documentation for `heading` — its signature, all parameters with their types and defaults, and example usage — into the assistant context.

**Tab completion**: Type a partial function name and press Tab to see matching completions. For example, typing `tex` and pressing Tab completes to `text`.

**Common functions**: `heading`, `text`, `image`, `table`, `figure`, `cite`, `bibliography`, `math.equation`, `grid`, `box`, `block`, `list`, `enum`, `terms`, `link`, `ref`, `label`

---

## `/typst-symbols <category>`

Shows all math symbols in a category, with their Typst names and Unicode characters.

**Example**:
```
/typst-symbols arrow
```

**Available categories**:

| Category | Description |
|----------|-------------|
| `arrow` | Arrow symbols (→, ←, ↑, ↓, ⇒, ⟹, …) |
| `greek` | Greek letters (α, β, γ, δ, π, σ, …) |
| `operator` | Mathematical operators (∑, ∏, ∫, ∂, …) |
| `relation` | Relation symbols (=, ≠, <, >, ≤, ≥, ≈, …) |
| `set` | Set theory symbols (∈, ∉, ⊂, ⊃, ∪, ∩, …) |
| `logic` | Logical symbols (∧, ∨, ¬, ⊕, ∀, ∃, …) |
| `accent` | Accent marks (â, ë, î, ô, …) |
| `misc` | Miscellaneous symbols (∞, ∅, ℝ, ℕ, …) |

**Tab completion**: Type a partial category name and press Tab. For example, `log` completes to `logic`.

---

## Why Use These Commands?

Without these commands, an AI assistant may guess Typst function names or symbol codes incorrectly. Injecting the actual documentation ensures the assistant has accurate information about:

- Function signatures and required vs optional parameters
- Correct symbol names to use in math mode
- Valid values for enum parameters

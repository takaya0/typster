use std::ops::Range;

/// Intermediate representation for a code-based label, testable on native target.
///
/// The `code` field is valid Typst source that tree-sitter will parse for highlight
/// information. `display_range` is the byte range within `code` to actually show in the
/// UI; `filter_range` is the byte range within the *displayed* text used for fuzzy
/// matching.
pub(crate) struct LabelParts {
    pub code: String,
    pub display_range: Range<usize>,
    pub filter_range: Range<usize>,
}

/// Returns true if every character in `s` is a valid Typst identifier character
/// (ASCII letter, digit, underscore, or hyphen).  We use this to guard against
/// injecting arbitrary text into a tree-sitter code string.
fn is_safe_ident(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

// ─── Completion helpers ────────────────────────────────────────────────────

/// Build `LabelParts` for a Function completion.
///
/// Produces `#name(params)` so that tree-sitter recognises `name` as a call
/// target via `(call item: (ident) @function)`.
///
/// `params` comes exclusively from `label_details.detail` (the signature
/// fragment).  `completion.detail` is a prose description and must not be
/// used as a parameter source.
pub(crate) fn function_completion_label(name: &str, params: Option<&str>) -> Option<LabelParts> {
    if !is_safe_ident(name) {
        return None;
    }
    let params_str = params.unwrap_or("");
    // code: "#name(params)"
    //        ^     ^
    //        0     1+len(name)
    let code = format!("#{name}({params_str})");
    let display_start = 1; // skip leading '#'
    let display_end = code.len();
    let filter_end = display_start + name.len();
    Some(LabelParts {
        code,
        display_range: display_start..display_end,
        filter_range: 0..filter_end - display_start, // relative to displayed text
    })
}

/// Build `LabelParts` for a Variable/Constant/Field completion.
///
/// Produces `#let name` so that tree-sitter gives `name` the `(ident) @variable`
/// highlight.
pub(crate) fn variable_completion_label(name: &str) -> Option<LabelParts> {
    if !is_safe_ident(name) {
        return None;
    }
    // code: "#let name"
    //        01234^
    //              5   = start of `name`
    let code = format!("#let {name}");
    let display_start = 5; // "#let " is 5 bytes
    let display_end = code.len();
    Some(LabelParts {
        code,
        display_range: display_start..display_end,
        filter_range: 0..name.len(),
    })
}

// ─── Symbol helpers ────────────────────────────────────────────────────────

/// Build `LabelParts` for a Function/Method symbol.
///
/// Produces `#let name()` (without parameter guessing — only `name` is known).
pub(crate) fn function_symbol_label(name: &str) -> Option<LabelParts> {
    // Allow dotted names like "module.func"; split on '.' and validate each part.
    if name.is_empty() {
        return None;
    }
    let last = name.split('.').last().unwrap_or(name);
    if !is_safe_ident(last) {
        return None;
    }
    // Display just the last segment with `()`.
    let code = format!("#let {last}()");
    let display_start = 5;
    let display_end = code.len();
    Some(LabelParts {
        code,
        display_range: display_start..display_end,
        filter_range: 0..last.len(),
    })
}

/// Build `LabelParts` for a heading symbol.
///
/// Tree-sitter recognises `(heading) @title`, so we pass the name as-is when
/// it starts with `=` (Typst heading syntax).
pub(crate) fn heading_symbol_label(name: &str) -> Option<LabelParts> {
    if !name.starts_with('=') {
        return None;
    }
    let code = name.to_string();
    // Count leading `= ` prefix bytes so filter starts at the heading text.
    let prefix_len = name
        .chars()
        .take_while(|&c| c == '=' || c == ' ')
        .map(|c| c.len_utf8())
        .sum::<usize>();
    Some(LabelParts {
        display_range: 0..code.len(),
        filter_range: prefix_len..code.len(),
        code,
    })
}

/// Build `LabelParts` for a Variable/Constant symbol (reuses the same pattern).
pub(crate) fn variable_symbol_label(name: &str) -> Option<LabelParts> {
    variable_completion_label(name)
}

// ─── WASM layer ────────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
use zed_extension_api::{self as zed, CodeLabel, CodeLabelSpan};

#[cfg(target_arch = "wasm32")]
fn parts_to_code_label(parts: LabelParts) -> CodeLabel {
    CodeLabel {
        code: parts.code,
        spans: vec![CodeLabelSpan::code_range(parts.display_range)],
        filter_range: parts.filter_range.into(),
    }
}

#[cfg(target_arch = "wasm32")]
fn literal_label(text: &str, highlight_name: &str) -> CodeLabel {
    let len = text.len();
    CodeLabel {
        code: String::new(),
        spans: vec![CodeLabelSpan::literal(
            text,
            Some(highlight_name.to_string()),
        )],
        filter_range: zed::Range { start: 0, end: len as u32 },
    }
}

/// Returns a `CodeLabel` for the given LSP completion, or `None` to let Zed
/// use its default rendering.
#[cfg(target_arch = "wasm32")]
pub fn label_for_completion(completion: &zed::lsp::Completion) -> Option<CodeLabel> {
    use zed::lsp::CompletionKind;

    let label = &completion.label;
    if label.is_empty() {
        return None;
    }

    match completion.kind.as_ref()? {
        CompletionKind::Function | CompletionKind::Constructor => {
            // Params come only from label_details.detail (signature fragment).
            let params = completion
                .label_details
                .as_ref()
                .and_then(|d| d.detail.as_deref());
            // Strip surrounding parens if present, e.g. "(font, size)" → "font, size"
            let params_inner = params.map(|p| {
                let p = p.trim();
                if p.starts_with('(') && p.ends_with(')') {
                    &p[1..p.len() - 1]
                } else {
                    p
                }
            });
            function_completion_label(label, params_inner).map(parts_to_code_label)
        }
        CompletionKind::Keyword => {
            // Keywords may or may not include the leading `#`.
            Some(literal_label(label, "keyword"))
        }
        CompletionKind::Module => {
            Some(literal_label(label, "string"))
        }
        CompletionKind::Variable
        | CompletionKind::Constant
        | CompletionKind::Field
        | CompletionKind::Property => {
            variable_completion_label(label).map(parts_to_code_label)
        }
        _ => None,
    }
}

/// Returns a `CodeLabel` for the given LSP symbol, or `None` to let Zed use
/// its default rendering.
#[cfg(target_arch = "wasm32")]
pub fn label_for_symbol(symbol: &zed::lsp::Symbol) -> Option<CodeLabel> {
    use zed::lsp::SymbolKind;

    let name = &symbol.name;
    if name.is_empty() {
        return None;
    }

    match &symbol.kind {
        SymbolKind::Function | SymbolKind::Method | SymbolKind::Constructor => {
            function_symbol_label(name).map(parts_to_code_label)
        }
        SymbolKind::String => {
            // tinymist represents headings as SymbolKind::String with names like
            // "= Introduction" or "== Methods".
            heading_symbol_label(name).map(parts_to_code_label)
        }
        SymbolKind::Variable | SymbolKind::Constant => {
            variable_symbol_label(name).map(parts_to_code_label)
        }
        SymbolKind::Module | SymbolKind::Namespace | SymbolKind::Package => {
            Some(literal_label(name, "string"))
        }
        _ => None,
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // --- is_safe_ident ---

    #[test]
    fn is_safe_ident_returns_true_when_ascii_alphanumeric() {
        assert!(is_safe_ident("text"));
        assert!(is_safe_ident("my_func"));
        assert!(is_safe_ident("camelCase"));
        assert!(is_safe_ident("func-name"));
    }

    #[test]
    fn is_safe_ident_returns_false_when_empty() {
        assert!(!is_safe_ident(""));
    }

    #[test]
    fn is_safe_ident_returns_false_when_contains_special_chars() {
        assert!(!is_safe_ident("a b"));
        assert!(!is_safe_ident("a(b)"));
        assert!(!is_safe_ident("a.b"));
        assert!(!is_safe_ident("#set"));
    }

    // --- function_completion_label ---

    #[test]
    fn function_completion_label_returns_parts_when_valid_name_with_params() {
        let parts = function_completion_label("text", Some("font, size")).unwrap();
        assert_eq!(parts.code, "#text(font, size)");
        assert_eq!(&parts.code[parts.display_range.clone()], "text(font, size)");
        assert_eq!(parts.filter_range, 0..4);
    }

    #[test]
    fn function_completion_label_returns_parts_when_no_params() {
        let parts = function_completion_label("image", None).unwrap();
        assert_eq!(parts.code, "#image()");
        assert_eq!(&parts.code[parts.display_range.clone()], "image()");
        assert_eq!(parts.filter_range, 0..5);
    }

    #[test]
    fn function_completion_label_returns_none_when_name_has_special_chars() {
        assert!(function_completion_label("#text", None).is_none());
        assert!(function_completion_label("", None).is_none());
        assert!(function_completion_label("a b", None).is_none());
    }

    // --- variable_completion_label ---

    #[test]
    fn variable_completion_label_returns_parts_when_valid_name() {
        let parts = variable_completion_label("myVar").unwrap();
        assert_eq!(parts.code, "#let myVar");
        assert_eq!(&parts.code[parts.display_range.clone()], "myVar");
        assert_eq!(parts.filter_range, 0..5);
    }

    #[test]
    fn variable_completion_label_returns_none_when_empty() {
        assert!(variable_completion_label("").is_none());
    }

    // --- function_symbol_label ---

    #[test]
    fn function_symbol_label_returns_parts_when_simple_name() {
        let parts = function_symbol_label("render").unwrap();
        assert_eq!(parts.code, "#let render()");
        assert_eq!(&parts.code[parts.display_range.clone()], "render()");
        assert_eq!(parts.filter_range, 0..6);
    }

    #[test]
    fn function_symbol_label_returns_parts_when_dotted_name_uses_last_segment() {
        let parts = function_symbol_label("module.render").unwrap();
        assert_eq!(parts.code, "#let render()");
        assert_eq!(parts.filter_range, 0..6);
    }

    #[test]
    fn function_symbol_label_returns_none_when_empty() {
        assert!(function_symbol_label("").is_none());
    }

    // --- heading_symbol_label ---

    #[test]
    fn heading_symbol_label_returns_parts_when_h1() {
        let parts = heading_symbol_label("= Introduction").unwrap();
        assert_eq!(parts.code, "= Introduction");
        // filter_range skips "= " (2 bytes)
        assert_eq!(parts.filter_range, 2..14);
    }

    #[test]
    fn heading_symbol_label_returns_parts_when_h2() {
        let parts = heading_symbol_label("== Methods").unwrap();
        // "== " is 3 bytes
        assert_eq!(parts.filter_range, 3..10);
    }

    #[test]
    fn heading_symbol_label_returns_none_when_not_heading() {
        assert!(heading_symbol_label("Introduction").is_none());
        assert!(heading_symbol_label("").is_none());
    }
}

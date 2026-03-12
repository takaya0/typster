#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use super::{ArgumentCompletion, CommandOutput, OutputSection};

pub(crate) struct SymbolEntry {
    pub name: &'static str,
    pub codepoint: &'static str,
    pub typst_syntax: &'static str,
}

pub(crate) struct SymbolCategory {
    pub name: &'static str,
    pub description: &'static str,
    pub symbols: &'static [SymbolEntry],
}

static CATEGORIES: &[SymbolCategory] = &[
    SymbolCategory {
        name: "arrow",
        description: "Arrow symbols for use in math mode.",
        symbols: &[
            SymbolEntry { name: "arrow.r", codepoint: "→", typst_syntax: "$arrow.r$" },
            SymbolEntry { name: "arrow.l", codepoint: "←", typst_syntax: "$arrow.l$" },
            SymbolEntry { name: "arrow.t", codepoint: "↑", typst_syntax: "$arrow.t$" },
            SymbolEntry { name: "arrow.b", codepoint: "↓", typst_syntax: "$arrow.b$" },
            SymbolEntry { name: "arrow.l.r", codepoint: "↔", typst_syntax: "$arrow.l.r$" },
            SymbolEntry { name: "arrow.t.b", codepoint: "↕", typst_syntax: "$arrow.t.b$" },
            SymbolEntry { name: "arrow.r.long", codepoint: "⟶", typst_syntax: "$arrow.r.long$" },
            SymbolEntry { name: "arrow.l.long", codepoint: "⟵", typst_syntax: "$arrow.l.long$" },
            SymbolEntry { name: "arrow.r.double", codepoint: "⇒", typst_syntax: "$arrow.r.double$" },
            SymbolEntry { name: "arrow.l.double", codepoint: "⇐", typst_syntax: "$arrow.l.double$" },
            SymbolEntry { name: "arrow.l.r.double", codepoint: "⇔", typst_syntax: "$arrow.l.r.double$" },
            SymbolEntry { name: "arrow.r.long.double", codepoint: "⟹", typst_syntax: "$arrow.r.long.double$" },
            SymbolEntry { name: "arrow.l.long.double", codepoint: "⟸", typst_syntax: "$arrow.l.long.double$" },
            SymbolEntry { name: "arrow.r.bar", codepoint: "↦", typst_syntax: "$arrow.r.bar$" },
            SymbolEntry { name: "arrow.r.hook", codepoint: "↪", typst_syntax: "$arrow.r.hook$" },
            SymbolEntry { name: "arrow.l.hook", codepoint: "↩", typst_syntax: "$arrow.l.hook$" },
            SymbolEntry { name: "arrow.r.twohead", codepoint: "↠", typst_syntax: "$arrow.r.twohead$" },
            SymbolEntry { name: "arrow.r.dashed", codepoint: "⇢", typst_syntax: "$arrow.r.dashed$" },
            SymbolEntry { name: "arrow.l.dashed", codepoint: "⇠", typst_syntax: "$arrow.l.dashed$" },
            SymbolEntry { name: "arrow.curve.r", codepoint: "↷", typst_syntax: "$arrow.curve.r$" },
            SymbolEntry { name: "arrow.curve.l", codepoint: "↶", typst_syntax: "$arrow.curve.l$" },
        ],
    },
    SymbolCategory {
        name: "greek",
        description: "Greek letters (lowercase and uppercase) for math mode.",
        symbols: &[
            SymbolEntry { name: "alpha", codepoint: "α", typst_syntax: "$alpha$" },
            SymbolEntry { name: "beta", codepoint: "β", typst_syntax: "$beta$" },
            SymbolEntry { name: "gamma", codepoint: "γ", typst_syntax: "$gamma$" },
            SymbolEntry { name: "delta", codepoint: "δ", typst_syntax: "$delta$" },
            SymbolEntry { name: "epsilon", codepoint: "ε", typst_syntax: "$epsilon$" },
            SymbolEntry { name: "zeta", codepoint: "ζ", typst_syntax: "$zeta$" },
            SymbolEntry { name: "eta", codepoint: "η", typst_syntax: "$eta$" },
            SymbolEntry { name: "theta", codepoint: "θ", typst_syntax: "$theta$" },
            SymbolEntry { name: "iota", codepoint: "ι", typst_syntax: "$iota$" },
            SymbolEntry { name: "kappa", codepoint: "κ", typst_syntax: "$kappa$" },
            SymbolEntry { name: "lambda", codepoint: "λ", typst_syntax: "$lambda$" },
            SymbolEntry { name: "mu", codepoint: "μ", typst_syntax: "$mu$" },
            SymbolEntry { name: "nu", codepoint: "ν", typst_syntax: "$nu$" },
            SymbolEntry { name: "xi", codepoint: "ξ", typst_syntax: "$xi$" },
            SymbolEntry { name: "pi", codepoint: "π", typst_syntax: "$pi$" },
            SymbolEntry { name: "rho", codepoint: "ρ", typst_syntax: "$rho$" },
            SymbolEntry { name: "sigma", codepoint: "σ", typst_syntax: "$sigma$" },
            SymbolEntry { name: "tau", codepoint: "τ", typst_syntax: "$tau$" },
            SymbolEntry { name: "upsilon", codepoint: "υ", typst_syntax: "$upsilon$" },
            SymbolEntry { name: "phi", codepoint: "φ", typst_syntax: "$phi$" },
            SymbolEntry { name: "chi", codepoint: "χ", typst_syntax: "$chi$" },
            SymbolEntry { name: "psi", codepoint: "ψ", typst_syntax: "$psi$" },
            SymbolEntry { name: "omega", codepoint: "ω", typst_syntax: "$omega$" },
            SymbolEntry { name: "Alpha", codepoint: "Α", typst_syntax: "$Alpha$" },
            SymbolEntry { name: "Beta", codepoint: "Β", typst_syntax: "$Beta$" },
            SymbolEntry { name: "Gamma", codepoint: "Γ", typst_syntax: "$Gamma$" },
            SymbolEntry { name: "Delta", codepoint: "Δ", typst_syntax: "$Delta$" },
            SymbolEntry { name: "Epsilon", codepoint: "Ε", typst_syntax: "$Epsilon$" },
            SymbolEntry { name: "Zeta", codepoint: "Ζ", typst_syntax: "$Zeta$" },
            SymbolEntry { name: "Eta", codepoint: "Η", typst_syntax: "$Eta$" },
            SymbolEntry { name: "Theta", codepoint: "Θ", typst_syntax: "$Theta$" },
            SymbolEntry { name: "Iota", codepoint: "Ι", typst_syntax: "$Iota$" },
            SymbolEntry { name: "Kappa", codepoint: "Κ", typst_syntax: "$Kappa$" },
            SymbolEntry { name: "Lambda", codepoint: "Λ", typst_syntax: "$Lambda$" },
            SymbolEntry { name: "Mu", codepoint: "Μ", typst_syntax: "$Mu$" },
            SymbolEntry { name: "Nu", codepoint: "Ν", typst_syntax: "$Nu$" },
            SymbolEntry { name: "Xi", codepoint: "Ξ", typst_syntax: "$Xi$" },
            SymbolEntry { name: "Pi", codepoint: "Π", typst_syntax: "$Pi$" },
            SymbolEntry { name: "Rho", codepoint: "Ρ", typst_syntax: "$Rho$" },
            SymbolEntry { name: "Sigma", codepoint: "Σ", typst_syntax: "$Sigma$" },
            SymbolEntry { name: "Tau", codepoint: "Τ", typst_syntax: "$Tau$" },
            SymbolEntry { name: "Upsilon", codepoint: "Υ", typst_syntax: "$Upsilon$" },
            SymbolEntry { name: "Phi", codepoint: "Φ", typst_syntax: "$Phi$" },
            SymbolEntry { name: "Chi", codepoint: "Χ", typst_syntax: "$Chi$" },
            SymbolEntry { name: "Psi", codepoint: "Ψ", typst_syntax: "$Psi$" },
            SymbolEntry { name: "Omega", codepoint: "Ω", typst_syntax: "$Omega$" },
        ],
    },
    SymbolCategory {
        name: "operator",
        description: "Mathematical operators.",
        symbols: &[
            SymbolEntry { name: "plus", codepoint: "+", typst_syntax: "$+$" },
            SymbolEntry { name: "minus", codepoint: "−", typst_syntax: "$-$" },
            SymbolEntry { name: "times", codepoint: "×", typst_syntax: "$times$" },
            SymbolEntry { name: "div", codepoint: "÷", typst_syntax: "$div$" },
            SymbolEntry { name: "plus.minus", codepoint: "±", typst_syntax: "$plus.minus$" },
            SymbolEntry { name: "minus.plus", codepoint: "∓", typst_syntax: "$minus.plus$" },
            SymbolEntry { name: "dot.op", codepoint: "⋅", typst_syntax: "$dot.op$" },
            SymbolEntry { name: "star.op", codepoint: "⋆", typst_syntax: "$star.op$" },
            SymbolEntry { name: "circle.times", codepoint: "⊗", typst_syntax: "$circle.times$" },
            SymbolEntry { name: "circle.plus", codepoint: "⊕", typst_syntax: "$circle.plus$" },
            SymbolEntry { name: "circle.dot", codepoint: "⊙", typst_syntax: "$circle.dot$" },
            SymbolEntry { name: "circle.minus", codepoint: "⊖", typst_syntax: "$circle.minus$" },
            SymbolEntry { name: "slash", codepoint: "∕", typst_syntax: "$slash$" },
            SymbolEntry { name: "backslash", codepoint: "∖", typst_syntax: "$backslash$" },
            SymbolEntry { name: "sqrt", codepoint: "√", typst_syntax: "$sqrt(x)$" },
            SymbolEntry { name: "integral", codepoint: "∫", typst_syntax: "$integral$" },
            SymbolEntry { name: "integral.double", codepoint: "∬", typst_syntax: "$integral.double$" },
            SymbolEntry { name: "integral.triple", codepoint: "∭", typst_syntax: "$integral.triple$" },
            SymbolEntry { name: "integral.cont", codepoint: "∮", typst_syntax: "$integral.cont$" },
            SymbolEntry { name: "sum", codepoint: "∑", typst_syntax: "$sum$" },
            SymbolEntry { name: "product", codepoint: "∏", typst_syntax: "$product$" },
            SymbolEntry { name: "product.co", codepoint: "∐", typst_syntax: "$product.co$" },
            SymbolEntry { name: "nabla", codepoint: "∇", typst_syntax: "$nabla$" },
            SymbolEntry { name: "partial", codepoint: "∂", typst_syntax: "$partial$" },
        ],
    },
    SymbolCategory {
        name: "relation",
        description: "Comparison and relation symbols.",
        symbols: &[
            SymbolEntry { name: "eq", codepoint: "=", typst_syntax: "$=$" },
            SymbolEntry { name: "eq.not", codepoint: "≠", typst_syntax: "$eq.not$" },
            SymbolEntry { name: "eq.triple", codepoint: "≡", typst_syntax: "$eq.triple$" },
            SymbolEntry { name: "lt", codepoint: "<", typst_syntax: "$<$" },
            SymbolEntry { name: "gt", codepoint: ">", typst_syntax: "$>$" },
            SymbolEntry { name: "lt.eq", codepoint: "≤", typst_syntax: "$lt.eq$" },
            SymbolEntry { name: "gt.eq", codepoint: "≥", typst_syntax: "$gt.eq$" },
            SymbolEntry { name: "lt.eq.slant", codepoint: "⩽", typst_syntax: "$lt.eq.slant$" },
            SymbolEntry { name: "gt.eq.slant", codepoint: "⩾", typst_syntax: "$gt.eq.slant$" },
            SymbolEntry { name: "approx", codepoint: "≈", typst_syntax: "$approx$" },
            SymbolEntry { name: "approx.not", codepoint: "≉", typst_syntax: "$approx.not$" },
            SymbolEntry { name: "tilde.eq", codepoint: "≅", typst_syntax: "$tilde.eq$" },
            SymbolEntry { name: "tilde.op", codepoint: "∼", typst_syntax: "$tilde.op$" },
            SymbolEntry { name: "prec", codepoint: "≺", typst_syntax: "$prec$" },
            SymbolEntry { name: "succ", codepoint: "≻", typst_syntax: "$succ$" },
            SymbolEntry { name: "prec.eq", codepoint: "≼", typst_syntax: "$prec.eq$" },
            SymbolEntry { name: "succ.eq", codepoint: "≽", typst_syntax: "$succ.eq$" },
            SymbolEntry { name: "ll", codepoint: "≪", typst_syntax: "$ll$" },
            SymbolEntry { name: "gg", codepoint: "≫", typst_syntax: "$gg$" },
            SymbolEntry { name: "prop", codepoint: "∝", typst_syntax: "$prop$" },
            SymbolEntry { name: "perp", codepoint: "⊥", typst_syntax: "$perp$" },
            SymbolEntry { name: "parallel", codepoint: "∥", typst_syntax: "$parallel$" },
        ],
    },
    SymbolCategory {
        name: "set",
        description: "Set theory symbols.",
        symbols: &[
            SymbolEntry { name: "union", codepoint: "∪", typst_syntax: "$union$" },
            SymbolEntry { name: "sect", codepoint: "∩", typst_syntax: "$sect$" },
            SymbolEntry { name: "in", codepoint: "∈", typst_syntax: "$in$" },
            SymbolEntry { name: "in.not", codepoint: "∉", typst_syntax: "$in.not$" },
            SymbolEntry { name: "in.rev", codepoint: "∋", typst_syntax: "$in.rev$" },
            SymbolEntry { name: "subset", codepoint: "⊂", typst_syntax: "$subset$" },
            SymbolEntry { name: "supset", codepoint: "⊃", typst_syntax: "$supset$" },
            SymbolEntry { name: "subset.eq", codepoint: "⊆", typst_syntax: "$subset.eq$" },
            SymbolEntry { name: "supset.eq", codepoint: "⊇", typst_syntax: "$supset.eq$" },
            SymbolEntry { name: "subset.not", codepoint: "⊄", typst_syntax: "$subset.not$" },
            SymbolEntry { name: "supset.not", codepoint: "⊅", typst_syntax: "$supset.not$" },
            SymbolEntry { name: "subset.eq.not", codepoint: "⊈", typst_syntax: "$subset.eq.not$" },
            SymbolEntry { name: "union.sq", codepoint: "⊔", typst_syntax: "$union.sq$" },
            SymbolEntry { name: "sect.sq", codepoint: "⊓", typst_syntax: "$sect.sq$" },
            SymbolEntry { name: "nothing", codepoint: "∅", typst_syntax: "$nothing$" },
            SymbolEntry { name: "complement", codepoint: "∁", typst_syntax: "$complement$" },
        ],
    },
    SymbolCategory {
        name: "logic",
        description: "Logical symbols.",
        symbols: &[
            SymbolEntry { name: "and", codepoint: "∧", typst_syntax: "$and$" },
            SymbolEntry { name: "or", codepoint: "∨", typst_syntax: "$or$" },
            SymbolEntry { name: "not", codepoint: "¬", typst_syntax: "$not$" },
            SymbolEntry { name: "xor", codepoint: "⊕", typst_syntax: "$xor$" },
            SymbolEntry { name: "forall", codepoint: "∀", typst_syntax: "$forall$" },
            SymbolEntry { name: "exists", codepoint: "∃", typst_syntax: "$exists$" },
            SymbolEntry { name: "exists.not", codepoint: "∄", typst_syntax: "$exists.not$" },
            SymbolEntry { name: "tack.r", codepoint: "⊢", typst_syntax: "$tack.r$" },
            SymbolEntry { name: "tack.l", codepoint: "⊣", typst_syntax: "$tack.l$" },
            SymbolEntry { name: "tack.r.double", codepoint: "⊨", typst_syntax: "$tack.r.double$" },
            SymbolEntry { name: "top", codepoint: "⊤", typst_syntax: "$top$" },
            SymbolEntry { name: "bot", codepoint: "⊥", typst_syntax: "$bot$" },
        ],
    },
    SymbolCategory {
        name: "accent",
        description: "Accent marks for math mode. Apply with: #math.accent(x, mark).",
        symbols: &[
            SymbolEntry { name: "hat", codepoint: "̂", typst_syntax: "$hat(x)$" },
            SymbolEntry { name: "tilde", codepoint: "̃", typst_syntax: "$tilde(x)$" },
            SymbolEntry { name: "macron", codepoint: "̄", typst_syntax: "$macron(x)$" },
            SymbolEntry { name: "breve", codepoint: "̆", typst_syntax: "$breve(x)$" },
            SymbolEntry { name: "dot", codepoint: "̇", typst_syntax: "$dot(x)$" },
            SymbolEntry { name: "dot.double", codepoint: "̈", typst_syntax: "$dot.double(x)$" },
            SymbolEntry { name: "acute", codepoint: "́", typst_syntax: "$acute(x)$" },
            SymbolEntry { name: "grave", codepoint: "̀", typst_syntax: "$grave(x)$" },
            SymbolEntry { name: "arrow", codepoint: "⃗", typst_syntax: "$arrow(x)$" },
        ],
    },
    SymbolCategory {
        name: "misc",
        description: "Miscellaneous mathematical symbols.",
        symbols: &[
            SymbolEntry { name: "infinity", codepoint: "∞", typst_syntax: "$infinity$" },
            SymbolEntry { name: "dots.h", codepoint: "…", typst_syntax: "$dots.h$" },
            SymbolEntry { name: "dots.v", codepoint: "⋮", typst_syntax: "$dots.v$" },
            SymbolEntry { name: "dots.c", codepoint: "⋯", typst_syntax: "$dots.c$" },
            SymbolEntry { name: "dots.down", codepoint: "⋱", typst_syntax: "$dots.down$" },
            SymbolEntry { name: "star", codepoint: "★", typst_syntax: "$star$" },
            SymbolEntry { name: "dagger", codepoint: "†", typst_syntax: "$dagger$" },
            SymbolEntry { name: "dagger.double", codepoint: "‡", typst_syntax: "$dagger.double$" },
            SymbolEntry { name: "degree", codepoint: "°", typst_syntax: "$degree$" },
            SymbolEntry { name: "prime", codepoint: "′", typst_syntax: "$prime$" },
            SymbolEntry { name: "prime.double", codepoint: "″", typst_syntax: "$prime.double$" },
            SymbolEntry { name: "hash", codepoint: "#", typst_syntax: "$hash$" },
            SymbolEntry { name: "percent", codepoint: "%", typst_syntax: "$percent$" },
            SymbolEntry { name: "angle", codepoint: "∠", typst_syntax: "$angle$" },
            SymbolEntry { name: "angle.right", codepoint: "∟", typst_syntax: "$angle.right$" },
            SymbolEntry { name: "diameter", codepoint: "⌀", typst_syntax: "$diameter$" },
            SymbolEntry { name: "bullet", codepoint: "•", typst_syntax: "$bullet$" },
            SymbolEntry { name: "compose", codepoint: "∘", typst_syntax: "$compose$" },
        ],
    },
];

pub(crate) fn find_category(name: &str) -> Option<&'static SymbolCategory> {
    CATEGORIES
        .iter()
        .find(|c| c.name.eq_ignore_ascii_case(name))
}

pub(crate) fn search_categories(query: &str) -> Vec<&'static SymbolCategory> {
    let q = query.to_ascii_lowercase();
    CATEGORIES
        .iter()
        .filter(|c| c.name.starts_with(q.as_str()))
        .collect()
}

pub(crate) fn list_categories() -> &'static [SymbolCategory] {
    CATEGORIES
}

pub(crate) fn complete_symbols(args: &[String]) -> Vec<ArgumentCompletion> {
    let query = args.first().map(String::as_str).unwrap_or("");
    let cats: Vec<&SymbolCategory> = if query.is_empty() {
        CATEGORIES.iter().collect()
    } else {
        search_categories(query)
    };
    cats.into_iter()
        .map(|c| ArgumentCompletion {
            label: format!("{} — {}", c.name, c.description),
            new_text: c.name.to_string(),
            run_command: true,
        })
        .collect()
}

pub(crate) fn run_symbols(args: &[String]) -> Result<CommandOutput, String> {
    let name = args
        .first()
        .map(String::as_str)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            format!(
                "Usage: /typst-symbols <category>\nAvailable categories: {}",
                CATEGORIES
                    .iter()
                    .map(|c| c.name)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;
    let cat = find_category(name)
        .ok_or_else(|| format!("Category '{}' not found. Available: {}", name,
            CATEGORIES.iter().map(|c| c.name).collect::<Vec<_>>().join(", ")))?;
    Ok(format_symbol_category(cat))
}

fn format_symbol_category(cat: &SymbolCategory) -> CommandOutput {
    let mut text = String::new();

    text.push_str(&format!("# {} Symbols\n\n", capitalize(cat.name)));
    text.push_str(&format!("{}\n\n", cat.description));
    text.push_str("| Name | Symbol | Typst Syntax |\n");
    text.push_str("|------|--------|--------------|\n");
    for s in cat.symbols {
        text.push_str(&format!("| `{}` | {} | `{}` |\n", s.name, s.codepoint, s.typst_syntax));
    }

    let len = text.len();
    CommandOutput {
        sections: vec![OutputSection {
            range: 0..len,
            label: format!("typst-symbols: {}", cat.name),
        }],
        text,
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- find_category ---

    #[test]
    fn find_category_returns_category_when_name_matches() {
        let cat = find_category("greek").unwrap();
        assert_eq!(cat.name, "greek");
    }

    #[test]
    fn find_category_returns_category_when_name_differs_in_case() {
        let cat = find_category("GREEK").unwrap();
        assert_eq!(cat.name, "greek");
    }

    #[test]
    fn find_category_returns_none_when_name_not_found() {
        assert!(find_category("nonexistent_category").is_none());
    }

    // --- search_categories ---

    #[test]
    fn search_categories_returns_matches_when_prefix_matches() {
        let results = search_categories("ar");
        assert!(results.iter().any(|c| c.name == "arrow"));
    }

    #[test]
    fn search_categories_returns_empty_when_no_match() {
        let results = search_categories("zzz");
        assert!(results.is_empty());
    }

    // --- complete_symbols ---

    #[test]
    fn complete_symbols_returns_all_categories_when_no_args() {
        let completions = complete_symbols(&[]);
        assert_eq!(completions.len(), list_categories().len());
    }

    #[test]
    fn complete_symbols_returns_filtered_results_when_partial_arg() {
        let completions = complete_symbols(&["gr".to_string()]);
        assert!(completions.iter().any(|c| c.new_text == "greek"));
    }

    #[test]
    fn complete_symbols_sets_run_command_true() {
        let completions = complete_symbols(&[]);
        assert!(completions.iter().all(|c| c.run_command));
    }

    // --- run_symbols ---

    #[test]
    fn run_symbols_returns_output_when_valid_category() {
        let output = run_symbols(&["greek".to_string()]).unwrap();
        assert!(output.text.contains("# Greek Symbols"));
        assert!(output.text.contains("| Name |"));
    }

    #[test]
    fn run_symbols_returns_error_when_no_args() {
        assert!(run_symbols(&[]).is_err());
    }

    #[test]
    fn run_symbols_returns_error_when_category_not_found() {
        assert!(run_symbols(&["nonexistent".to_string()]).is_err());
    }

    // --- format_symbol_category ---

    #[test]
    fn format_symbol_category_includes_table_header_when_category_found() {
        let cat = find_category("arrow").unwrap();
        let output = format_symbol_category(cat);
        assert!(output.text.contains("| Name | Symbol | Typst Syntax |"));
    }

    #[test]
    fn format_symbol_category_section_range_spans_entire_text() {
        let cat = find_category("misc").unwrap();
        let output = format_symbol_category(cat);
        assert_eq!(output.sections.len(), 1);
        assert_eq!(output.sections[0].range.end, output.text.len());
    }
}

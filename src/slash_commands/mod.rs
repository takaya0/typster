#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

mod typst_docs;
mod typst_symbols;

pub(crate) struct ArgumentCompletion {
    pub label: String,
    pub new_text: String,
    pub run_command: bool,
}

pub(crate) struct CommandOutput {
    pub text: String,
    pub sections: Vec<OutputSection>,
}

pub(crate) struct OutputSection {
    pub range: std::ops::Range<usize>,
    pub label: String,
}

pub(crate) fn complete_typst_docs(args: &[String]) -> Vec<ArgumentCompletion> {
    typst_docs::complete_docs(args)
}

pub(crate) fn run_typst_docs(args: &[String]) -> Result<CommandOutput, String> {
    typst_docs::run_docs(args)
}

pub(crate) fn complete_typst_symbols(args: &[String]) -> Vec<ArgumentCompletion> {
    typst_symbols::complete_symbols(args)
}

pub(crate) fn run_typst_symbols(args: &[String]) -> Result<CommandOutput, String> {
    typst_symbols::run_symbols(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- complete_typst_docs ---

    #[test]
    fn complete_typst_docs_returns_all_functions_when_no_args() {
        let completions = complete_typst_docs(&[]);
        assert!(!completions.is_empty());
    }

    #[test]
    fn complete_typst_docs_returns_filtered_results_when_partial_arg() {
        let completions = complete_typst_docs(&["tex".to_string()]);
        assert!(completions.iter().any(|c| c.label == "text"));
    }

    // --- run_typst_docs ---

    #[test]
    fn run_typst_docs_returns_output_when_valid_function() {
        let output = run_typst_docs(&["heading".to_string()]).unwrap();
        assert!(output.text.contains("# heading"));
        assert!(!output.sections.is_empty());
    }

    #[test]
    fn run_typst_docs_returns_error_when_no_args() {
        assert!(run_typst_docs(&[]).is_err());
    }

    #[test]
    fn run_typst_docs_returns_error_when_function_not_found() {
        assert!(run_typst_docs(&["does_not_exist".to_string()]).is_err());
    }

    // --- complete_typst_symbols ---

    #[test]
    fn complete_typst_symbols_returns_all_categories_when_no_args() {
        let completions = complete_typst_symbols(&[]);
        assert!(!completions.is_empty());
    }

    #[test]
    fn complete_typst_symbols_returns_filtered_results_when_partial_arg() {
        let completions = complete_typst_symbols(&["log".to_string()]);
        assert!(completions.iter().any(|c| c.new_text == "logic"));
    }

    // --- run_typst_symbols ---

    #[test]
    fn run_typst_symbols_returns_output_when_valid_category() {
        let output = run_typst_symbols(&["arrow".to_string()]).unwrap();
        assert!(output.text.contains("# Arrow Symbols"));
        assert!(!output.sections.is_empty());
    }

    #[test]
    fn run_typst_symbols_returns_error_when_no_args() {
        assert!(run_typst_symbols(&[]).is_err());
    }

    #[test]
    fn run_typst_symbols_returns_error_when_category_not_found() {
        assert!(run_typst_symbols(&["xyz_invalid".to_string()]).is_err());
    }
}

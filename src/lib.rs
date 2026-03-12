mod code_label;
pub mod inverse_search;
pub mod platform;
mod slash_commands;
mod tinymist_config;
mod tinymist_invocation;

#[cfg(target_arch = "wasm32")]
use zed_extension_api::{self as zed, settings::LspSettings, Result};

#[cfg(target_arch = "wasm32")]
struct TypsterExtension {
    cached_binary_path: Option<String>,
}

#[cfg(target_arch = "wasm32")]
impl zed::Extension for TypsterExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = tinymist_invocation::resolve_binary(
            language_server_id,
            worktree,
            &mut self.cached_binary_path,
        )?;

        Ok(zed::Command {
            command: binary.path,
            args: binary.args.unwrap_or_else(|| vec!["lsp".to_string()]),
            env: binary.environment.unwrap_or_default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|s| s.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let config = tinymist_config::build_workspace_config(
            language_server_id.as_ref(),
            worktree,
        );
        Ok(config)
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        code_label::label_for_completion(&completion)
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &zed::LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<zed::CodeLabel> {
        code_label::label_for_symbol(&symbol)
    }

    fn complete_slash_command_argument(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
    ) -> Result<Vec<zed::SlashCommandArgumentCompletion>> {
        let completions = match command.name.as_str() {
            "typst-docs" => slash_commands::complete_typst_docs(&args),
            "typst-symbols" => slash_commands::complete_typst_symbols(&args),
            _ => return Ok(vec![]),
        };
        Ok(completions
            .into_iter()
            .map(|c| zed::SlashCommandArgumentCompletion {
                label: c.label,
                new_text: c.new_text,
                run_command: c.run_command,
            })
            .collect())
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput> {
        let output = match command.name.as_str() {
            "typst-docs" => slash_commands::run_typst_docs(&args),
            "typst-symbols" => slash_commands::run_typst_symbols(&args),
            _ => Err(format!("Unknown command: {}", command.name)),
        }?;
        Ok(zed::SlashCommandOutput {
            text: output.text,
            sections: output
                .sections
                .into_iter()
                .map(|s| zed::SlashCommandOutputSection {
                    range: zed::Range {
                        start: s.range.start as u32,
                        end: s.range.end as u32,
                    },
                    label: s.label,
                })
                .collect(),
        })
    }
}

#[cfg(target_arch = "wasm32")]
zed::register_extension!(TypsterExtension);

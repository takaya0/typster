pub mod inverse_search;
pub mod platform;
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
}

#[cfg(target_arch = "wasm32")]
zed::register_extension!(TypsterExtension);

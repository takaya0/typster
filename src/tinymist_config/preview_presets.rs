#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use crate::inverse_search::ViewerKind;
use crate::platform::Environment;
use serde_json::{Map, Value};

/// Detected PDF previewer with its forward search configuration.
#[derive(Debug)]
pub struct PreviewerConfig {
    /// Identifies which viewer was detected; usable for inverse search lookup.
    #[allow(dead_code)]
    pub viewer: ViewerKind,
    pub forward_search_executable: String,
    pub forward_search_args: Vec<String>,
}

/// Return the forward search configuration for a specific viewer kind,
/// if that viewer's binary or application path is available on the system.
/// Returns `None` if the viewer is not installed or if the kind is `BuiltinPreview`.
pub fn config_for_viewer(viewer: ViewerKind, env: &dyn Environment) -> Option<PreviewerConfig> {
    match viewer {
        ViewerKind::Skim => {
            let skim_path = "/Applications/Skim.app/Contents/SharedSupport/displayline";
            env.which("skimapp")
                .or_else(|| {
                    if env.path_exists(skim_path) {
                        Some(skim_path.to_string())
                    } else {
                        None
                    }
                })
                .map(|exe| PreviewerConfig {
                    viewer: ViewerKind::Skim,
                    forward_search_executable: exe,
                    forward_search_args: vec![
                        "%l".to_string(),
                        "%p".to_string(),
                        "%i".to_string(),
                    ],
                })
        }
        ViewerKind::SumatraPdf => env.which("SumatraPDF").map(|exe| PreviewerConfig {
            viewer: ViewerKind::SumatraPdf,
            forward_search_executable: exe,
            forward_search_args: vec![
                "-forward-search".to_string(),
                "%i".to_string(),
                "%l".to_string(),
                "%p".to_string(),
            ],
        }),
        ViewerKind::Zathura => env.which("zathura").map(|exe| PreviewerConfig {
            viewer: ViewerKind::Zathura,
            forward_search_executable: exe,
            forward_search_args: vec![
                "--synctex-forward".to_string(),
                "%l:1:%i".to_string(),
                "%p".to_string(),
            ],
        }),
        ViewerKind::Sioyek => env.which("sioyek").map(|exe| PreviewerConfig {
            viewer: ViewerKind::Sioyek,
            forward_search_executable: exe,
            forward_search_args: vec![
                "--reuse-window".to_string(),
                "--execute-command".to_string(),
                "toggle_synctex".to_string(),
                "--forward-search-file".to_string(),
                "%i".to_string(),
                "--forward-search-line".to_string(),
                "%l".to_string(),
                "--open".to_string(),
                "%p".to_string(),
            ],
        }),
        ViewerKind::Okular => env.which("okular").map(|exe| PreviewerConfig {
            viewer: ViewerKind::Okular,
            forward_search_executable: exe,
            forward_search_args: vec![
                "--unique".to_string(),
                "file:%p#src:%l%i".to_string(),
            ],
        }),
        ViewerKind::Evince => env.which("evince").map(|exe| PreviewerConfig {
            viewer: ViewerKind::Evince,
            forward_search_executable: exe,
            forward_search_args: vec![
                "--forward-search".to_string(),
                "%i".to_string(),
                "%l".to_string(),
                "%p".to_string(),
            ],
        }),
        ViewerKind::BuiltinPreview => None,
    }
}

/// Detect an available PDF previewer and return its forward search config.
/// Priority order matches zed-latex: Skim (macOS), SumatraPDF (Windows),
/// then Zathura, Sioyek, Okular, Evince (Linux/cross-platform).
pub fn detect_previewer(env: &dyn Environment) -> Option<PreviewerConfig> {
    const PRIORITY: &[ViewerKind] = &[
        ViewerKind::Skim,
        ViewerKind::SumatraPdf,
        ViewerKind::Zathura,
        ViewerKind::Sioyek,
        ViewerKind::Okular,
        ViewerKind::Evince,
    ];
    PRIORITY
        .iter()
        .find_map(|&kind| config_for_viewer(kind, env))
}

/// Return the minimum configuration to enable tinymist's built-in preview server.
/// Used as fallback when no external PDF viewer is detected.
///
/// Only `background.enabled` is set here; all other `preview.*` keys use tinymist's
/// own defaults (requires tinymist ≥ 0.13.6).
/// The returned map is intended to be inserted as the value of the `"preview"` key
/// in the workspace configuration sent to tinymist.
pub fn builtin_preview_defaults() -> Map<String, Value> {
    let mut preview = Map::new();
    preview.insert(
        "background".to_string(),
        serde_json::json!({ "enabled": true }),
    );
    preview
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::FakeEnv;

    #[test]
    fn detect_previewer_returns_none_when_no_binaries_found() {
        let env = FakeEnv::new();
        assert!(detect_previewer(&env).is_none());
    }

    #[test]
    fn detect_previewer_returns_skim_when_skim_app_path_exists() {
        let env = FakeEnv::new()
            .with_path("/Applications/Skim.app/Contents/SharedSupport/displayline");
        let config = detect_previewer(&env).unwrap();
        assert_eq!(
            config.forward_search_executable,
            "/Applications/Skim.app/Contents/SharedSupport/displayline"
        );
        assert_eq!(config.forward_search_args, vec!["%l", "%p", "%i"]);
        assert_eq!(config.viewer, ViewerKind::Skim);
    }

    #[test]
    fn detect_previewer_returns_zathura_when_zathura_on_path() {
        let env = FakeEnv::new().with_binary("zathura");
        let config = detect_previewer(&env).unwrap();
        assert_eq!(config.forward_search_executable, "/usr/bin/zathura");
        assert!(config
            .forward_search_args
            .contains(&"--synctex-forward".to_string()));
        assert_eq!(config.viewer, ViewerKind::Zathura);
    }

    #[test]
    fn detect_previewer_prefers_skim_over_zathura_when_both_available() {
        let env = FakeEnv::new()
            .with_path("/Applications/Skim.app/Contents/SharedSupport/displayline")
            .with_binary("zathura");
        let config = detect_previewer(&env).unwrap();
        assert!(config.forward_search_executable.contains("displayline"));
        assert_eq!(config.viewer, ViewerKind::Skim);
    }

    #[test]
    fn detect_previewer_returns_sioyek_when_sioyek_on_path() {
        let env = FakeEnv::new().with_binary("sioyek");
        let config = detect_previewer(&env).unwrap();
        assert_eq!(config.forward_search_args[0], "--reuse-window");
        assert_eq!(config.viewer, ViewerKind::Sioyek);
    }

    #[test]
    fn detect_previewer_returns_okular_when_okular_on_path() {
        let env = FakeEnv::new().with_binary("okular");
        let config = detect_previewer(&env).unwrap();
        assert!(config.forward_search_args[1].contains("src:"));
        assert_eq!(config.viewer, ViewerKind::Okular);
    }

    #[test]
    fn detect_previewer_returns_evince_when_evince_on_path() {
        let env = FakeEnv::new().with_binary("evince");
        let config = detect_previewer(&env).unwrap();
        assert_eq!(config.forward_search_args[0], "--forward-search");
        assert_eq!(config.viewer, ViewerKind::Evince);
    }

    #[test]
    fn config_for_viewer_returns_skim_config_when_path_exists() {
        let env = FakeEnv::new()
            .with_path("/Applications/Skim.app/Contents/SharedSupport/displayline");
        let config = config_for_viewer(ViewerKind::Skim, &env).unwrap();
        assert_eq!(config.viewer, ViewerKind::Skim);
    }

    #[test]
    fn config_for_viewer_returns_none_when_skim_not_installed() {
        let env = FakeEnv::new();
        assert!(config_for_viewer(ViewerKind::Skim, &env).is_none());
    }

    #[test]
    fn config_for_viewer_returns_none_for_builtin_preview() {
        let env = FakeEnv::new();
        assert!(config_for_viewer(ViewerKind::BuiltinPreview, &env).is_none());
    }

    #[test]
    fn builtin_preview_defaults_enables_background_server() {
        let defaults = builtin_preview_defaults();
        assert_eq!(defaults["background"]["enabled"], true);
    }
}

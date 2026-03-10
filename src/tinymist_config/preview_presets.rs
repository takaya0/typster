#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use crate::inverse_search::ViewerKind;
use crate::platform::Environment;

/// Detected PDF previewer with its forward search configuration.
#[derive(Debug)]
pub struct PreviewerConfig {
    /// Identifies which viewer was detected; usable for inverse search lookup.
    #[allow(dead_code)]
    pub viewer: ViewerKind,
    pub forward_search_executable: String,
    pub forward_search_args: Vec<String>,
}

/// Detect an available PDF previewer and return its forward search config.
/// Priority order matches zed-latex: Skim (macOS), SumatraPDF (Windows),
/// then Zathura, Sioyek, Okular, Evince (Linux/cross-platform).
pub fn detect_previewer(env: &dyn Environment) -> Option<PreviewerConfig> {
    // macOS: Skim
    let skim_path = "/Applications/Skim.app/Contents/SharedSupport/displayline";
    if let Some(skim) = env.which("skimapp").or_else(|| {
        if env.path_exists(skim_path) {
            Some(skim_path.to_string())
        } else {
            None
        }
    }) {
        return Some(PreviewerConfig {
            viewer: ViewerKind::Skim,
            forward_search_executable: skim,
            forward_search_args: vec![
                "%l".to_string(),
                "%p".to_string(),
                "%i".to_string(),
            ],
        });
    }

    // Windows: SumatraPDF
    if let Some(sumatra) = env.which("SumatraPDF") {
        return Some(PreviewerConfig {
            viewer: ViewerKind::SumatraPdf,
            forward_search_executable: sumatra,
            forward_search_args: vec![
                "-forward-search".to_string(),
                "%i".to_string(),
                "%l".to_string(),
                "%p".to_string(),
            ],
        });
    }

    // Zathura
    if let Some(zathura) = env.which("zathura") {
        return Some(PreviewerConfig {
            viewer: ViewerKind::Zathura,
            forward_search_executable: zathura,
            forward_search_args: vec![
                "--synctex-forward".to_string(),
                "%l:1:%i".to_string(),
                "%p".to_string(),
            ],
        });
    }

    // Sioyek (cross-platform)
    if let Some(sioyek) = env.which("sioyek") {
        return Some(PreviewerConfig {
            viewer: ViewerKind::Sioyek,
            forward_search_executable: sioyek,
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
        });
    }

    // Okular
    if let Some(okular) = env.which("okular") {
        return Some(PreviewerConfig {
            viewer: ViewerKind::Okular,
            forward_search_executable: okular,
            forward_search_args: vec![
                "--unique".to_string(),
                "file:%p#src:%l%i".to_string(),
            ],
        });
    }

    // Evince
    if let Some(evince) = env.which("evince") {
        return Some(PreviewerConfig {
            viewer: ViewerKind::Evince,
            forward_search_executable: evince,
            forward_search_args: vec![
                "--forward-search".to_string(),
                "%i".to_string(),
                "%l".to_string(),
                "%p".to_string(),
            ],
        });
    }

    None
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
        assert!(config
            .forward_search_executable
            .contains("displayline"));
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
}

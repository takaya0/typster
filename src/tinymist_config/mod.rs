#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

pub mod preview_presets;

use serde_json::{Map, Value};

use crate::inverse_search::ViewerKind;
use crate::platform::Environment;
use preview_presets::{builtin_preview_defaults, config_for_viewer, detect_previewer};

/// User-specified previewer choice, read from `lsp.tinymist.settings.typsterPreviewer`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewerChoice {
    /// Auto-detect an external viewer; fall back to browser preview if none found (default).
    Auto,
    /// Force tinymist's browser preview server regardless of installed viewers.
    Browser,
    Skim,
    SumatraPdf,
    Zathura,
    Sioyek,
    Okular,
    Evince,
}

impl PreviewerChoice {
    /// Parse from the string value of `typsterPreviewer`.
    /// Unknown values fall back to `Auto`.
    pub fn from_str(s: &str) -> Self {
        match s {
            "browser" => PreviewerChoice::Browser,
            "skim" => PreviewerChoice::Skim,
            "sumatrapdf" => PreviewerChoice::SumatraPdf,
            "zathura" => PreviewerChoice::Zathura,
            "sioyek" => PreviewerChoice::Sioyek,
            "okular" => PreviewerChoice::Okular,
            "evince" => PreviewerChoice::Evince,
            _ => PreviewerChoice::Auto,
        }
    }
}

/// Extract and remove the `typsterPreviewer` key from user settings.
/// If the key is present and its value is a string, parse it as a `PreviewerChoice`.
/// The key is removed from `user_settings` so it is not forwarded to tinymist.
pub fn extract_previewer_choice(user_settings: &mut Option<Value>) -> PreviewerChoice {
    let Some(Value::Object(map)) = user_settings else {
        return PreviewerChoice::Auto;
    };
    match map.remove("typsterPreviewer") {
        Some(Value::String(s)) => PreviewerChoice::from_str(&s),
        _ => PreviewerChoice::Auto,
    }
}

/// Detect which Typst formatter is available on the system.
/// Returns "typstyle" or "typstfmt" if found, None otherwise.
fn detect_formatter(env: &dyn Environment) -> Option<&'static str> {
    if env.which("typstyle").is_some() {
        Some("typstyle")
    } else if env.which("typstfmt").is_some() {
        Some("typstfmt")
    } else {
        None
    }
}

/// Apply previewer configuration to `map` based on `choice`.
/// External-viewer choices set `forwardSearch` + `exportPdf: "onSave"`.
/// Built-in preview sets `preview.background.enabled` + `exportPdf: "never"`.
/// For specific external viewers, falls back to built-in if the viewer is not installed.
fn apply_previewer_config(
    map: &mut Map<String, Value>,
    choice: PreviewerChoice,
    env: &dyn Environment,
) {
    let viewer_kind = match choice {
        PreviewerChoice::Browser => {
            use_browser_preview(map);
            return;
        }
        PreviewerChoice::Auto => {
            // Auto-detect: try each viewer in priority order.
            if let Some(previewer) = detect_previewer(env) {
                use_external_viewer(map, previewer);
            } else {
                use_browser_preview(map);
            }
            return;
        }
        PreviewerChoice::Skim => ViewerKind::Skim,
        PreviewerChoice::SumatraPdf => ViewerKind::SumatraPdf,
        PreviewerChoice::Zathura => ViewerKind::Zathura,
        PreviewerChoice::Sioyek => ViewerKind::Sioyek,
        PreviewerChoice::Okular => ViewerKind::Okular,
        PreviewerChoice::Evince => ViewerKind::Evince,
    };

    // Specific viewer: use it if installed, otherwise fall back to built-in.
    if let Some(previewer) = config_for_viewer(viewer_kind, env) {
        use_external_viewer(map, previewer);
    } else {
        use_browser_preview(map);
    }
}

fn use_external_viewer(map: &mut Map<String, Value>, previewer: preview_presets::PreviewerConfig) {
    map.insert(
        "forwardSearch".to_string(),
        serde_json::json!({
            "command": previewer.forward_search_executable,
            "args": previewer.forward_search_args,
        }),
    );
    // External viewer needs the exported PDF file to display.
    map.insert(
        "exportPdf".to_string(),
        Value::String("onSave".to_string()),
    );
}

fn use_browser_preview(map: &mut Map<String, Value>) {
    // Built-in preview renders directly from source; PDF export is not needed.
    map.insert(
        "preview".to_string(),
        Value::Object(builtin_preview_defaults()),
    );
    map.insert("exportPdf".to_string(), Value::String("never".to_string()));
}

/// Build the auto-detected portion of tinymist workspace configuration.
/// `choice` controls which previewer is selected (defaults to `Auto`).
pub fn build_auto_detected_config(
    env: &dyn Environment,
    choice: PreviewerChoice,
) -> Map<String, Value> {
    let mut map = Map::new();

    apply_previewer_config(&mut map, choice, env);

    if let Some(formatter) = detect_formatter(env) {
        map.insert(
            "formatterMode".to_string(),
            Value::String(formatter.to_string()),
        );
    }

    map.entry("semanticTokens".to_string())
        .or_insert_with(|| Value::String("enable".to_string()));

    map
}

/// Deep-merge auto-detected config with user settings.
/// User settings override auto-detected values recursively.
/// For non-object values (strings, numbers, arrays, etc.), user settings always win.
pub fn merge_configs(auto_detected: Value, user_settings: Value) -> Value {
    match (auto_detected, user_settings) {
        (Value::Object(mut base), Value::Object(overrides)) => {
            for (key, value) in overrides {
                let merged = if let Some(existing) = base.remove(&key) {
                    merge_configs(existing, value)
                } else {
                    value
                };
                base.insert(key, merged);
            }
            Value::Object(base)
        }
        (_, user) => user,
    }
}

/// Build the workspace configuration for tinymist by merging auto-detected
/// settings with user-provided settings. User settings override auto-detected
/// values key-by-key.
#[cfg(target_arch = "wasm32")]
pub fn build_workspace_config(
    server_id_str: &str,
    worktree: &zed_extension_api::Worktree,
) -> Option<Value> {
    use crate::platform::WorktreeEnv;
    use zed_extension_api::settings::LspSettings;

    let mut user_settings = LspSettings::for_worktree(server_id_str, worktree)
        .ok()
        .and_then(|s| s.settings.clone());

    let choice = extract_previewer_choice(&mut user_settings);

    let env = WorktreeEnv(worktree);
    let auto_config = build_auto_detected_config(&env, choice);

    if auto_config.is_empty() {
        return user_settings;
    }

    let auto_value = Value::Object(auto_config);

    match user_settings {
        Some(user) => Some(merge_configs(auto_value, user)),
        None => Some(auto_value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::FakeEnv;
    use serde_json::json;

    #[test]
    fn merge_configs_user_setting_overrides_auto_detected_value() {
        let auto = json!({"formatterMode": "typstyle", "exportPdf": "onSave"});
        let user = json!({"formatterMode": "typstfmt"});
        let merged = merge_configs(auto, user);
        assert_eq!(merged["formatterMode"], "typstfmt");
        assert_eq!(merged["exportPdf"], "onSave");
    }

    #[test]
    fn merge_configs_preserves_auto_detected_when_user_has_no_overlap() {
        let auto = json!({"formatterMode": "typstyle", "exportPdf": "onSave"});
        let user = json!({"customKey": true});
        let merged = merge_configs(auto, user);
        assert_eq!(merged["formatterMode"], "typstyle");
        assert_eq!(merged["customKey"], true);
    }

    #[test]
    fn merge_configs_deep_merges_nested_objects_when_both_sides_have_object() {
        // Issue #15 core scenario: user sets preview.refresh without losing preview.background.
        let auto = json!({"preview": {"background": {"enabled": true}, "refresh": "onType"}});
        let user = json!({"preview": {"refresh": "onSave"}});
        let merged = merge_configs(auto, user);
        assert_eq!(merged["preview"]["background"]["enabled"], true);
        assert_eq!(merged["preview"]["refresh"], "onSave");
    }

    #[test]
    fn merge_configs_replaces_leaf_value_when_user_overrides_object_with_scalar() {
        let auto = json!({"preview": {"background": {"enabled": true}}});
        let user = json!({"preview": "disabled"});
        let merged = merge_configs(auto, user);
        assert_eq!(merged["preview"], "disabled");
    }

    #[test]
    fn merge_configs_replaces_array_when_user_provides_array() {
        let auto = json!({"forwardSearch": {"args": ["--a", "--b"]}});
        let user = json!({"forwardSearch": {"args": ["--c"]}});
        let merged = merge_configs(auto, user);
        assert_eq!(merged["forwardSearch"]["args"], json!(["--c"]));
    }

    #[test]
    fn merge_configs_deep_merges_three_levels_when_both_sides_have_nested_objects() {
        let auto = json!({"a": {"b": {"c": 1, "d": 2}}});
        let user = json!({"a": {"b": {"c": 99}}});
        let merged = merge_configs(auto, user);
        assert_eq!(merged["a"]["b"]["c"], 99);
        assert_eq!(merged["a"]["b"]["d"], 2);
    }

    #[test]
    fn merge_configs_preserves_browser_preview_background_when_user_sets_only_refresh() {
        // E2E scenario: auto-detected config has preview.background.enabled, user sets preview.refresh.
        let env = FakeEnv::new();
        let auto_config = build_auto_detected_config(&env, PreviewerChoice::Browser);
        let auto_value = Value::Object(auto_config);
        let user = json!({"preview": {"refresh": "onSave"}});
        let merged = merge_configs(auto_value, user);
        assert_eq!(merged["preview"]["background"]["enabled"], true);
        assert_eq!(merged["preview"]["refresh"], "onSave");
    }

    #[test]
    fn build_auto_detected_config_sets_typstyle_when_available() {
        let env = FakeEnv::new().with_binary("typstyle");
        let config = build_auto_detected_config(&env, PreviewerChoice::Auto);
        assert_eq!(config["formatterMode"], "typstyle");
    }

    #[test]
    fn build_auto_detected_config_sets_typstfmt_when_typstyle_not_available() {
        let env = FakeEnv::new().with_binary("typstfmt");
        let config = build_auto_detected_config(&env, PreviewerChoice::Auto);
        assert_eq!(config["formatterMode"], "typstfmt");
    }

    #[test]
    fn build_auto_detected_config_prefers_typstyle_over_typstfmt() {
        let env = FakeEnv::new()
            .with_binary("typstyle")
            .with_binary("typstfmt");
        let config = build_auto_detected_config(&env, PreviewerChoice::Auto);
        assert_eq!(config["formatterMode"], "typstyle");
    }

    #[test]
    fn build_auto_detected_config_includes_export_pdf_default() {
        // Without an external viewer, built-in preview is used and PDF export is disabled.
        let env = FakeEnv::new();
        let config = build_auto_detected_config(&env, PreviewerChoice::Auto);
        assert_eq!(config["exportPdf"], "never");
    }

    #[test]
    fn build_auto_detected_config_includes_semantic_tokens_default() {
        let env = FakeEnv::new();
        let config = build_auto_detected_config(&env, PreviewerChoice::Auto);
        assert_eq!(config["semanticTokens"], "enable");
    }

    #[test]
    fn detect_formatter_returns_none_when_neither_available() {
        let env = FakeEnv::new();
        assert!(detect_formatter(&env).is_none());
    }

    #[test]
    fn detect_formatter_returns_typstyle_when_available() {
        let env = FakeEnv::new().with_binary("typstyle");
        assert_eq!(detect_formatter(&env), Some("typstyle"));
    }

    #[test]
    fn build_auto_detected_config_enables_browser_preview_when_no_viewer_found() {
        let env = FakeEnv::new();
        let config = build_auto_detected_config(&env, PreviewerChoice::Auto);
        assert_eq!(config["preview"]["background"]["enabled"], true);
        assert_eq!(config["preview"]["refresh"], "onType");
    }

    #[test]
    fn build_auto_detected_config_sets_export_pdf_never_when_browser_preview() {
        let env = FakeEnv::new();
        let config = build_auto_detected_config(&env, PreviewerChoice::Auto);
        assert_eq!(config["exportPdf"], "never");
    }

    #[test]
    fn build_auto_detected_config_does_not_set_forward_search_when_no_viewer_found() {
        let env = FakeEnv::new();
        let config = build_auto_detected_config(&env, PreviewerChoice::Auto);
        assert!(!config.contains_key("forwardSearch"));
    }

    #[test]
    fn build_auto_detected_config_sets_export_pdf_on_save_when_external_viewer_found() {
        let env = FakeEnv::new().with_binary("zathura");
        let config = build_auto_detected_config(&env, PreviewerChoice::Auto);
        assert_eq!(config["exportPdf"], "onSave");
    }

    #[test]
    fn build_auto_detected_config_does_not_set_preview_when_external_viewer_found() {
        let env = FakeEnv::new().with_binary("zathura");
        let config = build_auto_detected_config(&env, PreviewerChoice::Auto);
        assert!(!config.contains_key("preview"));
    }

    // --- PreviewerChoice::Browser ---

    #[test]
    fn build_auto_detected_config_forces_browser_when_choice_is_browser() {
        // Even if an external viewer is installed, Browser choice ignores it.
        let env = FakeEnv::new().with_binary("zathura");
        let config = build_auto_detected_config(&env, PreviewerChoice::Browser);
        assert_eq!(config["preview"]["background"]["enabled"], true);
        assert_eq!(config["preview"]["refresh"], "onType");
        assert_eq!(config["exportPdf"], "never");
        assert!(!config.contains_key("forwardSearch"));
    }

    // --- Specific viewer choices ---

    #[test]
    fn build_auto_detected_config_uses_zathura_when_choice_is_zathura_and_installed() {
        let env = FakeEnv::new().with_binary("zathura");
        let config = build_auto_detected_config(&env, PreviewerChoice::Zathura);
        assert_eq!(config["exportPdf"], "onSave");
        assert!(config["forwardSearch"]["command"]
            .as_str()
            .unwrap()
            .contains("zathura"));
    }

    #[test]
    fn build_auto_detected_config_falls_back_to_browser_when_specified_viewer_not_installed() {
        // User specifies Skim but it is not installed; falls back to built-in.
        let env = FakeEnv::new();
        let config = build_auto_detected_config(&env, PreviewerChoice::Skim);
        assert_eq!(config["preview"]["background"]["enabled"], true);
        assert_eq!(config["preview"]["refresh"], "onType");
        assert_eq!(config["exportPdf"], "never");
        assert!(!config.contains_key("forwardSearch"));
    }

    // --- extract_previewer_choice ---

    #[test]
    fn extract_previewer_choice_returns_auto_when_key_absent() {
        let mut settings = Some(json!({"otherKey": true}));
        let choice = extract_previewer_choice(&mut settings);
        assert_eq!(choice, PreviewerChoice::Auto);
        // Other keys are preserved.
        assert_eq!(settings.unwrap()["otherKey"], true);
    }

    #[test]
    fn extract_previewer_choice_parses_browser_string() {
        let mut settings = Some(json!({"typsterPreviewer": "browser"}));
        let choice = extract_previewer_choice(&mut settings);
        assert_eq!(choice, PreviewerChoice::Browser);
    }

    #[test]
    fn extract_previewer_choice_parses_skim_string() {
        let mut settings = Some(json!({"typsterPreviewer": "skim"}));
        let choice = extract_previewer_choice(&mut settings);
        assert_eq!(choice, PreviewerChoice::Skim);
    }

    #[test]
    fn extract_previewer_choice_removes_key_from_settings() {
        let mut settings = Some(json!({"typsterPreviewer": "zathura", "exportPdf": "onSave"}));
        extract_previewer_choice(&mut settings);
        // typsterPreviewer must not be forwarded to tinymist.
        let map = settings.unwrap();
        assert!(!map.as_object().unwrap().contains_key("typsterPreviewer"));
        assert_eq!(map["exportPdf"], "onSave");
    }

    #[test]
    fn extract_previewer_choice_returns_auto_for_unknown_value() {
        let mut settings = Some(json!({"typsterPreviewer": "chrome"}));
        let choice = extract_previewer_choice(&mut settings);
        assert_eq!(choice, PreviewerChoice::Auto);
    }

    #[test]
    fn extract_previewer_choice_returns_auto_when_settings_is_none() {
        let mut settings = None;
        let choice = extract_previewer_choice(&mut settings);
        assert_eq!(choice, PreviewerChoice::Auto);
    }
}

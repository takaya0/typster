#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

pub mod preview_presets;

use serde_json::{Map, Value};

use crate::platform::Environment;
use preview_presets::detect_previewer;

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

/// Build the auto-detected portion of tinymist workspace configuration.
/// This includes forward search, formatter mode, and sensible defaults.
pub fn build_auto_detected_config(env: &dyn Environment) -> Map<String, Value> {
    let mut map = Map::new();

    if let Some(previewer) = detect_previewer(env) {
        map.insert(
            "forwardSearch".to_string(),
            serde_json::json!({
                "command": previewer.forward_search_executable,
                "args": previewer.forward_search_args,
            }),
        );
    }

    if let Some(formatter) = detect_formatter(env) {
        map.insert(
            "formatterMode".to_string(),
            Value::String(formatter.to_string()),
        );
    }

    map.entry("exportPdf".to_string())
        .or_insert_with(|| Value::String("onSave".to_string()));

    map.entry("semanticTokens".to_string())
        .or_insert_with(|| Value::String("enable".to_string()));

    map
}

/// Shallow-merge auto-detected config with user settings.
/// User settings override auto-detected values key-by-key at the top level.
pub fn merge_configs(auto_detected: Value, user_settings: Value) -> Value {
    match (auto_detected, user_settings) {
        (Value::Object(mut base), Value::Object(overrides)) => {
            for (key, value) in overrides {
                base.insert(key, value);
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

    let user_settings = LspSettings::for_worktree(server_id_str, worktree)
        .ok()
        .and_then(|s| s.settings.clone());

    let env = WorktreeEnv(worktree);
    let auto_config = build_auto_detected_config(&env);

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
    fn build_auto_detected_config_sets_typstyle_when_available() {
        let env = FakeEnv::new().with_binary("typstyle");
        let config = build_auto_detected_config(&env);
        assert_eq!(config["formatterMode"], "typstyle");
    }

    #[test]
    fn build_auto_detected_config_sets_typstfmt_when_typstyle_not_available() {
        let env = FakeEnv::new().with_binary("typstfmt");
        let config = build_auto_detected_config(&env);
        assert_eq!(config["formatterMode"], "typstfmt");
    }

    #[test]
    fn build_auto_detected_config_prefers_typstyle_over_typstfmt() {
        let env = FakeEnv::new()
            .with_binary("typstyle")
            .with_binary("typstfmt");
        let config = build_auto_detected_config(&env);
        assert_eq!(config["formatterMode"], "typstyle");
    }

    #[test]
    fn build_auto_detected_config_includes_export_pdf_default() {
        let env = FakeEnv::new();
        let config = build_auto_detected_config(&env);
        assert_eq!(config["exportPdf"], "onSave");
    }

    #[test]
    fn build_auto_detected_config_includes_semantic_tokens_default() {
        let env = FakeEnv::new();
        let config = build_auto_detected_config(&env);
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
}

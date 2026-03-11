/// Supported PDF viewer kinds for both forward and inverse search.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerKind {
    Skim,
    SumatraPdf,
    Zathura,
    Sioyek,
    Okular,
    Evince,
    /// tinymist's built-in browser preview server (no external viewer needed).
    BuiltinPreview,
}

/// Setup information for configuring inverse search (PDF → source) in a viewer.
///
/// Inverse search is configured on the viewer side: the viewer calls `zed <file>:<line>`
/// when the user clicks in the PDF. Typster provides this information for documentation
/// purposes only — it cannot configure external viewers programmatically.
#[derive(Debug)]
pub struct InverseSearchSetup {
    pub viewer: ViewerKind,
    /// The editor command the viewer should invoke (typically `"zed"`).
    /// `None` means the viewer does not support a configurable editor command.
    pub editor_command: Option<&'static str>,
    /// The argument string with viewer-specific placeholders for file and line.
    /// `None` means not applicable.
    pub editor_args: Option<&'static str>,
    /// Human-readable description of where to configure inverse search in the viewer.
    pub setup_location: &'static str,
}

/// Return the inverse search setup information for the given viewer.
pub fn inverse_search_setup(viewer: ViewerKind) -> InverseSearchSetup {
    match viewer {
        ViewerKind::Skim => InverseSearchSetup {
            viewer,
            editor_command: Some("zed"),
            editor_args: Some("%file:%line"),
            setup_location: "Preferences > Sync > PDF-TeX Sync Support > Preset: Custom",
        },
        ViewerKind::SumatraPdf => InverseSearchSetup {
            viewer,
            editor_command: Some("zed"),
            editor_args: Some("\"%f:%l\""),
            setup_location: "Settings > Options > Set inverse search command line",
        },
        ViewerKind::Zathura => InverseSearchSetup {
            viewer,
            editor_command: Some("zed"),
            editor_args: Some("\"%{input}:%{line}\""),
            setup_location: "zathurarc: set synctex-editor-command",
        },
        ViewerKind::Sioyek => InverseSearchSetup {
            viewer,
            editor_command: Some("zed"),
            editor_args: Some("\"%1:%2\""),
            setup_location: "prefs_user.config: inverse_search_command",
        },
        ViewerKind::Okular => InverseSearchSetup {
            viewer,
            editor_command: Some("zed"),
            editor_args: Some("%f:%l"),
            setup_location:
                "Settings > Configure Okular > Editor > Custom Text Editor (command field)",
        },
        ViewerKind::Evince => InverseSearchSetup {
            viewer,
            editor_command: None,
            editor_args: None,
            setup_location:
                "Evince uses D-Bus for inverse search; direct editor configuration is not supported",
        },
        ViewerKind::BuiltinPreview => InverseSearchSetup {
            viewer,
            editor_command: None,
            editor_args: None,
            setup_location:
                "Built-in preview handles inverse search via WebSocket; no manual setup required",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_search_setup_returns_zed_command_for_skim() {
        let setup = inverse_search_setup(ViewerKind::Skim);
        assert_eq!(setup.editor_command, Some("zed"));
    }

    #[test]
    fn inverse_search_setup_returns_zed_command_for_sumatra_pdf() {
        let setup = inverse_search_setup(ViewerKind::SumatraPdf);
        assert_eq!(setup.editor_command, Some("zed"));
    }

    #[test]
    fn inverse_search_setup_returns_zed_command_for_zathura() {
        let setup = inverse_search_setup(ViewerKind::Zathura);
        assert_eq!(setup.editor_command, Some("zed"));
    }

    #[test]
    fn inverse_search_setup_returns_zed_command_for_sioyek() {
        let setup = inverse_search_setup(ViewerKind::Sioyek);
        assert_eq!(setup.editor_command, Some("zed"));
    }

    #[test]
    fn inverse_search_setup_returns_zed_command_for_okular() {
        let setup = inverse_search_setup(ViewerKind::Okular);
        assert_eq!(setup.editor_command, Some("zed"));
    }

    #[test]
    fn inverse_search_setup_returns_no_command_for_evince() {
        let setup = inverse_search_setup(ViewerKind::Evince);
        assert_eq!(setup.editor_command, None);
        assert_eq!(setup.editor_args, None);
    }

    #[test]
    fn inverse_search_setup_skim_args_contain_file_and_line_placeholders() {
        let setup = inverse_search_setup(ViewerKind::Skim);
        let args = setup.editor_args.unwrap();
        assert!(args.contains("%file"), "args should contain %file placeholder");
        assert!(args.contains("%line"), "args should contain %line placeholder");
    }

    #[test]
    fn inverse_search_setup_zathura_args_contain_viewer_specific_placeholders() {
        let setup = inverse_search_setup(ViewerKind::Zathura);
        let args = setup.editor_args.unwrap();
        assert!(args.contains("%{input}"), "args should contain %{{input}} placeholder");
        assert!(args.contains("%{line}"), "args should contain %{{line}} placeholder");
    }

    #[test]
    fn inverse_search_setup_sioyek_args_contain_positional_placeholders() {
        let setup = inverse_search_setup(ViewerKind::Sioyek);
        let args = setup.editor_args.unwrap();
        assert!(args.contains("%1"), "args should contain %1 placeholder");
        assert!(args.contains("%2"), "args should contain %2 placeholder");
    }

    #[test]
    fn inverse_search_setup_returns_no_command_for_builtin_preview() {
        let setup = inverse_search_setup(ViewerKind::BuiltinPreview);
        assert_eq!(setup.editor_command, None);
        assert_eq!(setup.editor_args, None);
    }

    #[test]
    fn inverse_search_setup_builtin_preview_indicates_no_manual_setup() {
        let setup = inverse_search_setup(ViewerKind::BuiltinPreview);
        assert!(
            setup.setup_location.contains("WebSocket"),
            "setup_location should mention WebSocket"
        );
    }
}

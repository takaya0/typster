#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

/// OS identifier for asset name construction (mirrors zed_extension_api::Os).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsKind {
    Mac,
    Linux,
    Windows,
}

/// Architecture identifier for asset name construction (mirrors zed_extension_api::Architecture).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchKind {
    Aarch64,
    X86,
    X8664,
}

/// Construct the GitHub release asset name for a given platform and architecture.
pub fn asset_name(os: OsKind, arch: ArchKind) -> String {
    let os_str = match os {
        OsKind::Mac => "darwin",
        OsKind::Linux => "linux",
        OsKind::Windows => "win32",
    };
    let arch_str = match arch {
        ArchKind::Aarch64 => "arm64",
        ArchKind::X86 => "x86",
        ArchKind::X8664 => "x64",
    };
    let ext = if os == OsKind::Windows { ".exe" } else { "" };
    format!("tinymist-{os_str}-{arch_str}{ext}")
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::{ArchKind, OsKind, asset_name};
    use std::fs;
    use zed_extension_api::{self as zed, settings::LspSettings, Result};

    #[derive(Clone)]
    pub struct TinymistBinary {
        pub path: String,
        pub args: Option<Vec<String>>,
        pub environment: Option<Vec<(String, String)>>,
    }

    /// Resolve the tinymist binary to use, downloading it from GitHub if necessary.
    ///
    /// Resolution order:
    /// 1. User-configured binary path in `lsp.tinymist.binary.path`
    /// 2. `tinymist` found on the system PATH via worktree
    /// 3. Previously cached downloaded binary
    /// 4. Download latest release from `Myriad-Dreamin/tinymist` on GitHub
    pub fn resolve_binary(
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
        cached_binary_path: &mut Option<String>,
    ) -> Result<TinymistBinary> {
        let binary_settings = LspSettings::for_worktree("tinymist", worktree)
            .ok()
            .and_then(|s| s.binary);

        let binary_args = binary_settings
            .as_ref()
            .and_then(|s| s.arguments.clone());

        // 1. User-configured path
        if let Some(path) = binary_settings.as_ref().and_then(|s| s.path.clone()) {
            return Ok(TinymistBinary {
                path,
                args: binary_args,
                environment: None,
            });
        }

        // 2. System PATH
        if let Some(path) = worktree.which("tinymist") {
            let env = worktree.shell_env();
            return Ok(TinymistBinary {
                path,
                args: binary_args,
                environment: Some(env),
            });
        }

        // 3. Cached binary
        if let Some(path) = cached_binary_path.as_deref()
            && fs::metadata(path).is_ok_and(|m| m.is_file())
        {
            return Ok(TinymistBinary {
                path: path.to_string(),
                args: binary_args,
                environment: None,
            });
        }

        // 4. Download from GitHub
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "Myriad-Dreamin/tinymist",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let os_kind = match platform {
            zed::Os::Mac => OsKind::Mac,
            zed::Os::Linux => OsKind::Linux,
            zed::Os::Windows => OsKind::Windows,
        };
        let arch_kind = match arch {
            zed::Architecture::Aarch64 => ArchKind::Aarch64,
            zed::Architecture::X86 => ArchKind::X86,
            zed::Architecture::X8664 => ArchKind::X8664,
        };
        if os_kind == OsKind::Mac && arch_kind != ArchKind::Aarch64 {
            return Err("typster supports macOS on Apple Silicon (arm64) only".into());
        }
        let asset_name_str = asset_name(os_kind, arch_kind);

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name_str)
            .ok_or_else(|| format!("no tinymist asset found matching {:?}", asset_name_str))?;

        let version_dir = format!("tinymist-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|e| format!("failed to create directory: {e}"))?;

        let binary_path = format!("{version_dir}/tinymist");

        if !fs::metadata(&binary_path).is_ok_and(|m| m.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download tinymist: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            // Remove old versions
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    if entry.file_name().to_str() != Some(&version_dir) {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }
        }

        *cached_binary_path = Some(binary_path.clone());
        Ok(TinymistBinary {
            path: binary_path,
            args: binary_args,
            environment: None,
        })
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::resolve_binary;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_name_returns_darwin_arm64_for_mac_aarch64() {
        assert_eq!(asset_name(OsKind::Mac, ArchKind::Aarch64), "tinymist-darwin-arm64");
    }

    #[test]
    fn asset_name_returns_linux_x64_for_linux_x8664() {
        assert_eq!(asset_name(OsKind::Linux, ArchKind::X8664), "tinymist-linux-x64");
    }

    #[test]
    fn asset_name_appends_exe_extension_for_windows() {
        assert_eq!(
            asset_name(OsKind::Windows, ArchKind::X8664),
            "tinymist-win32-x64.exe"
        );
    }

    #[test]
    fn asset_name_returns_no_extension_for_mac() {
        let name = asset_name(OsKind::Mac, ArchKind::Aarch64);
        assert!(!name.ends_with(".exe"));
    }

    #[test]
    fn asset_name_returns_no_extension_for_linux() {
        let name = asset_name(OsKind::Linux, ArchKind::Aarch64);
        assert!(!name.ends_with(".exe"));
    }

    #[test]
    fn asset_name_returns_x86_for_x86_arch() {
        assert_eq!(asset_name(OsKind::Linux, ArchKind::X86), "tinymist-linux-x86");
    }
}

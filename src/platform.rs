/// Abstracts host-environment queries so that pure logic can be unit tested.
pub trait Environment {
    /// Check if a binary is on PATH, returning its resolved path.
    fn which(&self, binary_name: &str) -> Option<String>;

    /// Check whether a filesystem path exists.
    fn path_exists(&self, path: &str) -> bool;
}

#[cfg(target_arch = "wasm32")]
pub struct WorktreeEnv<'a>(pub &'a zed_extension_api::Worktree);

#[cfg(target_arch = "wasm32")]
impl Environment for WorktreeEnv<'_> {
    fn which(&self, binary_name: &str) -> Option<String> {
        self.0.which(binary_name)
    }

    fn path_exists(&self, path: &str) -> bool {
        std::fs::metadata(path).is_ok()
    }
}

#[cfg(test)]
pub(crate) struct FakeEnv {
    binaries: std::collections::HashSet<String>,
    existing_paths: std::collections::HashSet<String>,
}

#[cfg(test)]
impl FakeEnv {
    pub(crate) fn new() -> Self {
        Self {
            binaries: std::collections::HashSet::new(),
            existing_paths: std::collections::HashSet::new(),
        }
    }

    pub(crate) fn with_binary(mut self, name: &str) -> Self {
        self.binaries.insert(name.to_string());
        self
    }

    pub(crate) fn with_path(mut self, path: &str) -> Self {
        self.existing_paths.insert(path.to_string());
        self
    }
}

#[cfg(test)]
impl Environment for FakeEnv {
    fn which(&self, name: &str) -> Option<String> {
        if self.binaries.contains(name) {
            Some(format!("/usr/bin/{name}"))
        } else {
            None
        }
    }

    fn path_exists(&self, path: &str) -> bool {
        self.existing_paths.contains(path)
    }
}

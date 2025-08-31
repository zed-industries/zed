//! Provides support for language toolchains.
//!
//! A language can have associated toolchains,
//! which is a set of tools used to interact with the projects written in said language.
//! For example, a Python project can have an associated virtual environment; a Rust project can have a toolchain override.

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use async_trait::async_trait;
use collections::HashMap;
use fs::Fs;
use gpui::{AsyncApp, SharedString};
use settings::WorktreeId;
use task::ShellKind;

use crate::{LanguageName, ManifestName};

/// Represents a single toolchain.
#[derive(Clone, Eq, Debug)]
pub struct Toolchain {
    /// User-facing label
    pub name: SharedString,
    pub path: SharedString,
    pub language_name: LanguageName,
    /// Full toolchain data (including language-specific details)
    pub as_json: serde_json::Value,
}

impl std::hash::Hash for Toolchain {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let Self {
            name,
            path,
            language_name,
            as_json: _,
        } = self;
        name.hash(state);
        path.hash(state);
        language_name.hash(state);
    }
}

impl PartialEq for Toolchain {
    fn eq(&self, other: &Self) -> bool {
        let Self {
            name,
            path,
            language_name,
            as_json: _,
        } = self;
        // Do not use as_json for comparisons; it shouldn't impact equality, as it's not user-surfaced.
        // Thus, there could be multiple entries that look the same in the UI.
        (name, path, language_name).eq(&(&other.name, &other.path, &other.language_name))
    }
}

#[async_trait]
pub trait ToolchainLister: Send + Sync {
    async fn list(
        &self,
        worktree_root: PathBuf,
        subroot_relative_path: Arc<Path>,
        project_env: Option<HashMap<String, String>>,
    ) -> ToolchainList;
    // Returns a term which we should use in UI to refer to a toolchain.
    fn term(&self) -> SharedString;
    /// Returns the name of the manifest file for this toolchain.
    fn manifest_name(&self) -> ManifestName;
    async fn activation_script(
        &self,
        toolchain: &Toolchain,
        shell: ShellKind,
        fs: &dyn Fs,
    ) -> Vec<String>;
}

#[async_trait(?Send)]
pub trait LanguageToolchainStore: Send + Sync + 'static {
    async fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        relative_path: Arc<Path>,
        language_name: LanguageName,
        cx: &mut AsyncApp,
    ) -> Option<Toolchain>;
}

pub trait LocalLanguageToolchainStore: Send + Sync + 'static {
    fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        relative_path: &Arc<Path>,
        language_name: LanguageName,
        cx: &mut AsyncApp,
    ) -> Option<Toolchain>;
}

#[async_trait(?Send)]
impl<T: LocalLanguageToolchainStore> LanguageToolchainStore for T {
    async fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        relative_path: Arc<Path>,
        language_name: LanguageName,
        cx: &mut AsyncApp,
    ) -> Option<Toolchain> {
        self.active_toolchain(worktree_id, &relative_path, language_name, cx)
    }
}

type DefaultIndex = usize;
#[derive(Default, Clone, Debug)]
pub struct ToolchainList {
    pub toolchains: Vec<Toolchain>,
    pub default: Option<DefaultIndex>,
    pub groups: Box<[(usize, SharedString)]>,
}

impl ToolchainList {
    pub fn toolchains(&self) -> &[Toolchain] {
        &self.toolchains
    }
    pub fn default_toolchain(&self) -> Option<Toolchain> {
        self.default.and_then(|ix| self.toolchains.get(ix)).cloned()
    }
    pub fn group_for_index(&self, index: usize) -> Option<(usize, SharedString)> {
        if index >= self.toolchains.len() {
            return None;
        }
        let first_equal_or_greater = self
            .groups
            .partition_point(|(group_lower_bound, _)| group_lower_bound <= &index);
        self.groups
            .get(first_equal_or_greater.checked_sub(1)?)
            .cloned()
    }
}

//! Provides support for language toolchains.
//!
//! A language can have associated toolchains,
//! which is a set of tools used to interact with the projects written in said language.
//! For example, a Python project can have an associated virtual environment; a Rust project can have a toolchain override.

use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use collections::HashMap;
use fs::Fs;
use gpui::{App, AsyncApp, SharedString};
use settings::WorktreeId;
use task::ShellKind;
use util::rel_path::RelPath;

use crate::{LanguageName, ManifestName};

/// Represents a single toolchain.
#[derive(Clone, Eq, Debug)]
pub struct Toolchain {
    /// User-facing label
    pub name: SharedString,
    /// Absolute path
    pub path: SharedString,
    pub language_name: LanguageName,
    /// Full toolchain data (including language-specific details)
    pub as_json: serde_json::Value,
}

/// Declares a scope of a toolchain added by user.
///
/// When the user adds a toolchain, we give them an option to see that toolchain in:
/// - All of their projects
/// - A project they're currently in.
/// - Only in the subproject they're currently in.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ToolchainScope {
    Subproject(WorktreeId, Arc<RelPath>),
    Project,
    /// Available in all projects on this box. It wouldn't make sense to show suggestions across machines.
    Global,
}

impl ToolchainScope {
    pub fn label(&self) -> &'static str {
        match self {
            ToolchainScope::Subproject(_, _) => "Subproject",
            ToolchainScope::Project => "Project",
            ToolchainScope::Global => "Global",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ToolchainScope::Subproject(_, _) => {
                "Available only in the subproject you're currently in."
            }
            ToolchainScope::Project => "Available in all locations in your current project.",
            ToolchainScope::Global => "Available in all of your projects on this machine.",
        }
    }
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
pub trait ToolchainLister: Send + Sync + 'static {
    /// List all available toolchains for a given path.
    async fn list(
        &self,
        worktree_root: PathBuf,
        subroot_relative_path: Arc<RelPath>,
        project_env: Option<HashMap<String, String>>,
        fs: &dyn Fs,
    ) -> ToolchainList;

    /// Given a user-created toolchain, resolve lister-specific details.
    /// Put another way: fill in the details of the toolchain so the user does not have to.
    async fn resolve(
        &self,
        path: PathBuf,
        project_env: Option<HashMap<String, String>>,
        fs: &dyn Fs,
    ) -> anyhow::Result<Toolchain>;

    fn activation_script(&self, toolchain: &Toolchain, shell: ShellKind, cx: &App) -> Vec<String>;

    /// Returns various "static" bits of information about this toolchain lister. This function should be pure.
    fn meta(&self) -> ToolchainMetadata;
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ToolchainMetadata {
    /// Returns a term which we should use in UI to refer to toolchains produced by a given `[ToolchainLister]`.
    pub term: SharedString,
    /// A user-facing placeholder describing the semantic meaning of a path to a new toolchain.
    pub new_toolchain_placeholder: SharedString,
    /// The name of the manifest file for this toolchain.
    pub manifest_name: ManifestName,
}

#[async_trait(?Send)]
pub trait LanguageToolchainStore: Send + Sync + 'static {
    async fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        relative_path: Arc<RelPath>,
        language_name: LanguageName,
        cx: &mut AsyncApp,
    ) -> Option<Toolchain>;
}

pub trait LocalLanguageToolchainStore: Send + Sync + 'static {
    fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        relative_path: &Arc<RelPath>,
        language_name: LanguageName,
        cx: &mut AsyncApp,
    ) -> Option<Toolchain>;
}

#[async_trait(?Send)]
impl<T: LocalLanguageToolchainStore> LanguageToolchainStore for T {
    async fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        relative_path: Arc<RelPath>,
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

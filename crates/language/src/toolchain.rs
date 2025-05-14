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
use gpui::{AsyncApp, SharedString};
use settings::WorktreeId;

use crate::LanguageName;

/// Represents a single toolchain.
#[derive(Clone, Debug)]
pub struct Toolchain {
    /// User-facing label
    pub name: SharedString,
    pub path: SharedString,
    pub language_name: LanguageName,
    /// Full toolchain data (including language-specific details)
    pub as_json: serde_json::Value,
}

impl PartialEq for Toolchain {
    fn eq(&self, other: &Self) -> bool {
        // Do not use as_json for comparisons; it shouldn't impact equality, as it's not user-surfaced.
        // Thus, there could be multiple entries that look the same in the UI.
        (&self.name, &self.path, &self.language_name).eq(&(
            &other.name,
            &other.path,
            &other.language_name,
        ))
    }
}

#[async_trait]
pub trait ToolchainLister: Send + Sync {
    async fn list(
        &self,
        worktree_root: PathBuf,
        project_env: Option<HashMap<String, String>>,
    ) -> ToolchainList;
    // Returns a term which we should use in UI to refer to a toolchain.
    fn term(&self) -> SharedString;
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

type DefaultIndex = usize;
#[derive(Default, Clone)]
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

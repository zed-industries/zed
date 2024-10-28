//! Provides support for language toolchains.
//!
//! A language can have associated toolchains,
//! which is a set of tools used to interact with the projects written in said language.
//! For example, a Python project can have an associated virtual environment; a Rust project can have a toolchain override.

use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use gpui::{AsyncAppContext, SharedString};
use settings::WorktreeId;

use crate::LanguageName;

/// Represents a single toolchain.
#[derive(Clone, Debug, PartialEq)]
pub struct Toolchain {
    /// User-facing label
    pub name: SharedString,
    pub path: SharedString,
    pub language_name: LanguageName,
}

#[async_trait(?Send)]
pub trait ToolchainLister: Send + Sync {
    async fn list(&self, _: PathBuf) -> ToolchainList;
}

#[async_trait(?Send)]
pub trait LanguageToolchainStore {
    async fn active_toolchain(
        self: Arc<Self>,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: &mut AsyncAppContext,
    ) -> Option<Toolchain>;
}

type DefaultIndex = usize;
#[derive(Default, Clone)]
pub struct ToolchainList {
    pub toolchains: Vec<Toolchain>,
    pub default: Option<DefaultIndex>,
}

impl ToolchainList {
    pub fn toolchains(&self) -> &[Toolchain] {
        &self.toolchains
    }
    pub fn default_toolchain(&self) -> Option<Toolchain> {
        self.default.and_then(|ix| self.toolchains.get(ix)).cloned()
    }
}

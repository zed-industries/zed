//! Provides support for language toolchains.
//!
//! A language can have associated toolchains,
//! which is a set of tools used to interact with the projects written in said language.
//! For example, a Python project can have an associated virtual environment; a Rust project can have a toolchain override.

use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use collections::HashMap;

use futures::future::BoxFuture;
use gpui::{App, AsyncApp};
use settings::WorktreeId;
use task::ShellKind;
use util::rel_path::RelPath;

use crate::LanguageName;

// Re-export core data types from language_core.
pub use language_core::{Toolchain, ToolchainList, ToolchainMetadata, ToolchainScope};

#[async_trait]
pub trait ToolchainLister: Send + Sync + 'static {
    /// List all available toolchains for a given path.
    async fn list(
        &self,
        worktree_root: PathBuf,
        subroot_relative_path: Arc<RelPath>,
        project_env: Option<HashMap<String, String>>,
    ) -> ToolchainList;

    /// Given a user-created toolchain, resolve lister-specific details.
    /// Put another way: fill in the details of the toolchain so the user does not have to.
    async fn resolve(
        &self,
        path: PathBuf,
        project_env: Option<HashMap<String, String>>,
    ) -> anyhow::Result<Toolchain>;

    fn activation_script(
        &self,
        toolchain: &Toolchain,
        shell: ShellKind,
        cx: &App,
    ) -> BoxFuture<'static, Vec<String>>;

    /// Returns various "static" bits of information about this toolchain lister. This function should be pure.
    fn meta(&self) -> ToolchainMetadata;
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

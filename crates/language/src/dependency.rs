//! Provides support for listing a project's external dependencies.
//!
//! A language can have an associated [`DependencyLister`], which enumerates the
//! external libraries (e.g. Rust crates, npm packages) that a project depends on.
//! The discovered dependencies can then be surfaced in the UI (e.g. the project
//! panel's "External Libraries" section) so users can browse their source code.

use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use gpui::SharedString;

use crate::LanguageName;

/// A single external dependency discovered for a project.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    /// Human-readable name of the dependency (e.g. `serde`).
    pub name: SharedString,
    /// Version of the dependency, if known (e.g. `1.0.193`).
    pub version: Option<SharedString>,
    /// Where the dependency's source was fetched from.
    pub source: DependencySource,
    /// Absolute path to the dependency's source root on disk
    /// (e.g. `~/.cargo/registry/src/.../serde-1.0.193`).
    pub source_path: PathBuf,
}

impl Dependency {
    /// A stable identifier for this dependency (name + version), suitable for
    /// deduplication and as a UI label.
    pub fn label(&self) -> String {
        match &self.version {
            Some(version) => format!("{} {}", self.name, version),
            None => self.name.to_string(),
        }
    }
}

/// Describes where a dependency's source lives.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DependencySource {
    /// Fetched from a package registry (e.g. crates.io, npm).
    Registry,
    /// Checked out from a git repository.
    Git {
        /// The repository URL.
        repo: String,
        /// The revision (commit/branch/tag) that was resolved.
        rev: String,
    },
    /// A local path dependency.
    Path,
}

/// A per-language provider that enumerates the external dependencies of a project.
///
/// Implementations are registered globally (see `DependencyProvidersStore` in the
/// `project` crate) and queried for each worktree root. A lister should return an
/// empty vector when it does not apply to a given project root (e.g. a
/// `RustDependencyLister` given a directory without a `Cargo.toml`).
#[async_trait]
pub trait DependencyLister: Send + Sync + 'static {
    /// The language this lister provides dependencies for.
    fn language_name(&self) -> LanguageName;

    /// Enumerate the external dependencies for the given project root.
    ///
    /// Workspace members that live inside the project root itself should be
    /// excluded; only external dependencies should be returned.
    async fn list(&self, project_root: PathBuf) -> Result<Vec<Dependency>>;
}

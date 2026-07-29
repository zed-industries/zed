use std::sync::Arc;

use settings::WorktreeId;
use util::rel_path::RelPath;

// Re-export ManifestName from language_core.
pub use language_core::ManifestName;

/// Represents a manifest query; given a path to a file, the manifest provider is tasked with finding a path to the directory containing the manifest for that file.
///
/// Since parts of the path might have already been explored, there's an additional `depth` parameter that indicates to what ancestry level a given path should be explored.
/// For example, given a path like `foo/bar/baz`, a depth of 2 would explore `foo/bar/baz` and `foo/bar`, but not `foo`.
pub struct ManifestQuery {
    /// Path to the file, relative to worktree root.
    pub path: Arc<RelPath>,
    pub depth: usize,
    pub delegate: Arc<dyn ManifestDelegate>,
}

pub trait ManifestProvider {
    fn name(&self) -> ManifestName;
    fn search(&self, query: ManifestQuery) -> Option<Arc<RelPath>>;
}

pub trait ManifestDelegate: Send + Sync {
    fn worktree_id(&self) -> WorktreeId;
    fn exists(&self, path: &RelPath, is_dir: Option<bool>) -> bool;
}

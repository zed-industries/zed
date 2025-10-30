use std::{borrow::Borrow, sync::Arc};

use gpui::SharedString;
use settings::WorktreeId;
use util::rel_path::RelPath;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManifestName(SharedString);

impl Borrow<SharedString> for ManifestName {
    fn borrow(&self) -> &SharedString {
        &self.0
    }
}

impl Borrow<str> for ManifestName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl From<SharedString> for ManifestName {
    fn from(value: SharedString) -> Self {
        Self(value)
    }
}

impl From<ManifestName> for SharedString {
    fn from(value: ManifestName) -> Self {
        value.0
    }
}

impl AsRef<SharedString> for ManifestName {
    fn as_ref(&self) -> &SharedString {
        &self.0
    }
}

/// Represents a manifest query; given a path to a file, [ManifestSearcher] is tasked with finding a path to the directory containing the manifest for that file.
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

use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    ffi::OsStr,
    path::Path,
    sync::Arc,
};

/// [RootPathTrie] is a workhorse of [super::ProjectTree]. It is responsible for determining the closest known project root for a given path.
/// It also determines how much of a given path is unexplored, thus letting callers fill in that gap if needed.
/// A path is unexplored when the closest ancestor of a path is not the path itself; that means that we have not yet ran the scan on that path.
/// For example, if there's a project root at path `python/project` and we query for a path `python/project/subdir/another_subdir/file.py`, there is
/// a known root at `python/project` and the unexplored part is `subdir/another_subdir` - we need to run a scan on these 2 directories
pub(super) struct RootPathTrie<Label> {
    path_component: Arc<OsStr>,
    labels: BTreeSet<Label>,
    children: BTreeMap<Arc<OsStr>, RootPathTrie<Label>>,
}

impl<Label: Ord> RootPathTrie<Label> {
    pub(crate) fn new() -> Self {
        Self::new_with_key(Arc::from(OsStr::new("")))
    }
    fn new_with_key(path_component: Arc<OsStr>) -> Self {
        RootPathTrie {
            path_component,
            labels: Default::default(),
            children: Default::default(),
        }
    }
    pub(crate) fn insert(&mut self, path: &TriePath, value: Label) {
        let mut current = self;

        for key in path.0.iter() {
            current = match current.children.entry(key.clone()) {
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(RootPathTrie::new_with_key(key.clone()))
                }
                Entry::Occupied(occupied_entry) => occupied_entry.into_mut(),
            };
        }
        current.labels.insert(value);
    }
    pub(crate) fn walk<'a>(
        &'a self,
        path: &TriePath,
        callback: &mut dyn FnMut(&'a Path, &BTreeSet<Label>),
    ) {
        let mut current = self;

        for key in path.0.iter() {
            if !current.labels.is_empty() {
                (callback)(Path::new(""), &current.labels);
            }
            current = match current.children.get(key) {
                Some(child) => child,
                None => return,
            };
        }
        if !current.labels.is_empty() {
            (callback)(Path::new(""), &current.labels);
        }
    }
}

/// [TriePath] is a [Path] preprocessed for amortizing the cost of doing multiple lookups in distinct [RootPathTrie]s.
#[derive(Clone)]
pub(crate) struct TriePath(Arc<[Arc<OsStr>]>);

impl From<&Path> for TriePath {
    fn from(value: &Path) -> Self {
        TriePath(value.components().map(|c| c.as_os_str().into()).collect())
    }
}

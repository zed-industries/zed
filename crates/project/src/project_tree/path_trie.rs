use std::{
    collections::{btree_map::Entry, BTreeMap},
    ffi::OsStr,
    path::Path,
    sync::Arc,
};

/// [RootPathTrie] is a workhorse of [super::ProjectTree]. It is responsible for determining the closest known project root for a given path.
/// We can not
/// It also determines how much of a given path is unexplored, thus letting callers fill in that gap if needed.
/// A path is unexplored when the closest ancestor of a path is not the path itself; that means that we have not yet ran the scan on that path.
/// For example, if there's a project root at path `python/project` and we query for a path `python/project/subdir/another_subdir/file.py`, there is
/// a known root at `python/project` and the unexplored part is `subdir/another_subdir` - we need to run a scan on these 2 directories
pub(super) struct RootPathTrie {
    path_component: Arc<OsStr>,
    value: Option<()>,
    children: BTreeMap<Arc<OsStr>, RootPathTrie>,
}

impl RootPathTrie {
    pub(crate) fn new() -> Self {
        Self::new_with_key(None)
    }
    fn new_with_key(value: Option<Arc<OsStr>>) -> Self {
        RootPathTrie {
            path_component: value.unwrap_or_else(|| Arc::from(OsStr::new(""))),
            value: None,
            children: BTreeMap::new(),
        }
    }
    fn insert(&mut self, path: &TriePath, value: ()) {
        let mut current = self;

        for key in path.0.iter() {
            current = match current.children.entry(key.clone()) {
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(RootPathTrie::new_with_key(Some(key.clone())))
                }
                Entry::Occupied(occupied_entry) => occupied_entry.into_mut(),
            };
        }
        current.value = Some(value);
    }
    fn closest_ancestor(&self, path: &TriePath) -> Option<()> {
        let mut current = self;
        let mut last_value = None;
        for key in path.0.iter() {
            if current.value.is_some() {
                last_value = current.value;
            }
            current = match current.children.get(key) {
                Some(child) => child,
                None => return last_value,
            };
        }
        if current.value.is_some() {
            last_value = current.value;
        }
        last_value
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

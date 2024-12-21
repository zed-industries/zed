use std::{
    collections::{btree_map::Entry, BTreeMap},
    ffi::OsStr,
    path::Path,
    sync::Arc,
};

/// `PathTrie` is a Trie composed of worktree path components.
pub(super) struct PathTrie {
    path_component: Arc<OsStr>,
    value: Option<()>,
    children: BTreeMap<Arc<OsStr>, PathTrie>,
}

impl PathTrie {
    pub(crate) fn new() -> Self {
        Self::new_with_key(None)
    }
    fn new_with_key(value: Option<Arc<OsStr>>) -> Self {
        PathTrie {
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
                    vacant_entry.insert(PathTrie::new_with_key(Some(key.clone())))
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

/// [TriePath] is a [Path] preprocessed for amortizing the cost of doing multiple lookups in distinct `PathTrie`s.
#[derive(Clone)]
pub(crate) struct TriePath(Arc<[Arc<OsStr>]>);

impl From<&Path> for TriePath {
    fn from(value: &Path) -> Self {
        TriePath(value.components().map(|c| c.as_os_str().into()).collect())
    }
}

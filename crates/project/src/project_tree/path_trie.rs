use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    ffi::OsStr,
    ops::ControlFlow,
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
    labels: BTreeMap<Label, LabelPresence>,
    children: BTreeMap<Arc<OsStr>, RootPathTrie<Label>>,
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub(super) enum LabelPresence {
    Present,
    KnownAbsent,
}
impl<Label: Ord> RootPathTrie<Label> {
    pub(super) fn new() -> Self {
        Self::new_with_key(Arc::from(OsStr::new("")))
    }
    fn new_with_key(path_component: Arc<OsStr>) -> Self {
        RootPathTrie {
            path_component,
            labels: Default::default(),
            children: Default::default(),
        }
    }
    pub(super) fn insert(&mut self, path: &TriePath, value: Label, presence: LabelPresence) {
        let mut current = self;

        for key in path.0.iter() {
            current = match current.children.entry(key.clone()) {
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(RootPathTrie::new_with_key(key.clone()))
                }
                Entry::Occupied(occupied_entry) => occupied_entry.into_mut(),
            };
        }
        let _previous_value = current.labels.insert(value, presence);
        debug_assert_eq!(_previous_value, None);
    }
    pub(super) fn walk<'a>(
        &'a self,
        path: &TriePath,
        callback: &mut dyn for<'b> FnMut(
            &'b Arc<Path>,
            &'a BTreeMap<Label, LabelPresence>,
        ) -> ControlFlow<()>,
    ) {
        let mut current = self;
        let tmp_path = Arc::from(Path::new(""));
        for key in path.0.iter() {
            if !current.labels.is_empty() {
                if (callback)(&tmp_path, &current.labels).is_break() {
                    break;
                };
            }
            current = match current.children.get(key) {
                Some(child) => child,
                None => return,
            };
        }
        if !current.labels.is_empty() {
            (callback)(&tmp_path, &current.labels);
        }
    }

    pub(super) fn remove(&mut self, path: &TriePath) {
        let mut current = &mut *self;
        // Tracks how many nodes (starting from the leaf, going upwards) can be removed
        let mut consecutive_node_chain = 0;
        for path in path.0.iter() {
            if current.children.len() > 1 {
                consecutive_node_chain = 0;
            } else if current.children.len() == 1 {
                consecutive_node_chain += 1;
            }
            current = match current.children.get_mut(path) {
                Some(child) => child,
                None => return,
            };
        }
        // Now walk the tree again, this time iterating only up to the root of the consecutive node chain.
        let consecutive_chain_start = path.0.len() - consecutive_node_chain;
        let mut current = self;
        for path in path.0[..consecutive_chain_start].iter() {
            current = match current.children.get_mut(path) {
                Some(child) => child,
                None => unreachable!(),
            };
        }
        current
            .children
            .remove(&path.0[consecutive_chain_start])
            .expect("The removal to succeed");
    }
}

/// [TriePath] is a [Path] preprocessed for amortizing the cost of doing multiple lookups in distinct [RootPathTrie]s.
#[derive(Clone)]
pub(super) struct TriePath(Arc<[Arc<OsStr>]>);

impl From<&Path> for TriePath {
    fn from(value: &Path) -> Self {
        TriePath(value.components().map(|c| c.as_os_str().into()).collect())
    }
}

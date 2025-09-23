use std::{
    collections::{BTreeMap, btree_map::Entry},
    ffi::OsStr,
    ops::ControlFlow,
    path::{Path, PathBuf},
    sync::Arc,
};

/// [RootPathTrie] is a workhorse of [super::ManifestTree]. It is responsible for determining the closest known entry for a given path.
/// It also determines how much of a given path is unexplored, thus letting callers fill in that gap if needed.
/// Conceptually, it allows one to annotate Worktree entries with arbitrary extra metadata and run closest-ancestor searches.
///
/// A path is unexplored when the closest ancestor of a path is not the path itself; that means that we have not yet ran the scan on that path.
/// For example, if there's a project root at path `python/project` and we query for a path `python/project/subdir/another_subdir/file.py`, there is
/// a known root at `python/project` and the unexplored part is `subdir/another_subdir` - we need to run a scan on these 2 directories.
pub(super) struct RootPathTrie<Label> {
    worktree_relative_path: Arc<Path>,
    labels: BTreeMap<Label, LabelPresence>,
    children: BTreeMap<Arc<OsStr>, RootPathTrie<Label>>,
}

/// Label presence is a marker that allows to optimize searches within [RootPathTrie]; node label can be:
/// - Present; we know there's definitely a project root at this node.
/// - Known Absent - we know there's definitely no project root at this node and none of it's ancestors are Present (descendants can be present though!).
///   The distinction is there to optimize searching; when we encounter a node with unknown status, we don't need to look at it's full path
///   to the root of the worktree; it's sufficient to explore only the path between last node with a KnownAbsent state and the directory of a path, since we run searches
///   from the leaf up to the root of the worktree.
///
/// In practical terms, it means that by storing label presence we don't need to do a project discovery on a given folder more than once
/// (unless the node is invalidated, which can happen when FS entries are renamed/removed).
///
/// Storing absent nodes allows us to recognize which paths have already been scanned for a project root unsuccessfully. This way we don't need to run
/// such scan more than once.
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub(super) enum LabelPresence {
    KnownAbsent,
    Present,
}

impl<Label: Ord + Clone> RootPathTrie<Label> {
    pub(super) fn new() -> Self {
        Self::new_with_key(Arc::from(Path::new("")))
    }
    fn new_with_key(worktree_relative_path: Arc<Path>) -> Self {
        RootPathTrie {
            worktree_relative_path,
            labels: Default::default(),
            children: Default::default(),
        }
    }
    // Internal implementation of inner that allows one to visit descendants of insertion point for a node.
    fn insert_inner(
        &mut self,
        path: &TriePath,
        value: Label,
        presence: LabelPresence,
    ) -> &mut Self {
        let mut current = self;

        let mut path_so_far = PathBuf::new();
        for key in path.0.iter() {
            path_so_far.push(Path::new(key));
            current = match current.children.entry(key.clone()) {
                Entry::Vacant(vacant_entry) => vacant_entry
                    .insert(RootPathTrie::new_with_key(Arc::from(path_so_far.as_path()))),
                Entry::Occupied(occupied_entry) => occupied_entry.into_mut(),
            };
        }
        let _previous_value = current.labels.insert(value, presence);
        debug_assert_eq!(_previous_value, None);
        current
    }
    pub(super) fn insert(&mut self, path: &TriePath, value: Label, presence: LabelPresence) {
        self.insert_inner(path, value, presence);
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
        for key in path.0.iter() {
            if !current.labels.is_empty()
                && (callback)(&current.worktree_relative_path, &current.labels).is_break()
            {
                return;
            };
            current = match current.children.get(key) {
                Some(child) => child,
                None => return,
            };
        }
        if !current.labels.is_empty() {
            let _ = (callback)(&current.worktree_relative_path, &current.labels);
        }
    }

    pub(super) fn remove(&mut self, path: &TriePath) {
        let mut current = self;
        for path in path.0.iter().take(path.0.len().saturating_sub(1)) {
            current = match current.children.get_mut(path) {
                Some(child) => child,
                None => return,
            };
        }
        if let Some(final_entry_name) = path.0.last() {
            current.children.remove(final_entry_name);
        }
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn test_insert_and_lookup() {
        let mut trie = RootPathTrie::<()>::new();
        trie.insert(
            &TriePath::from(Path::new("a/b/c")),
            (),
            LabelPresence::Present,
        );

        trie.walk(&TriePath::from(Path::new("a/b/c")), &mut |path, nodes| {
            assert_eq!(nodes.get(&()), Some(&LabelPresence::Present));
            assert_eq!(path.as_ref(), Path::new("a/b/c"));
            ControlFlow::Continue(())
        });
        // Now let's annotate a parent with "Known missing" node.
        trie.insert(
            &TriePath::from(Path::new("a")),
            (),
            LabelPresence::KnownAbsent,
        );

        // Ensure that we walk from the root to the leaf.
        let mut visited_paths = BTreeSet::new();
        trie.walk(&TriePath::from(Path::new("a/b/c")), &mut |path, nodes| {
            if path.as_ref() == Path::new("a/b/c") {
                assert_eq!(
                    visited_paths,
                    BTreeSet::from_iter([Arc::from(Path::new("a/"))])
                );
                assert_eq!(nodes.get(&()), Some(&LabelPresence::Present));
            } else if path.as_ref() == Path::new("a/") {
                assert!(visited_paths.is_empty());
                assert_eq!(nodes.get(&()), Some(&LabelPresence::KnownAbsent));
            } else {
                panic!("Unknown path");
            }
            // Assert that we only ever visit a path once.
            assert!(visited_paths.insert(path.clone()));
            ControlFlow::Continue(())
        });

        // One can also pass a path whose prefix is in the tree, but not that path itself.
        let mut visited_paths = BTreeSet::new();
        trie.walk(
            &TriePath::from(Path::new("a/b/c/d/e/f/g")),
            &mut |path, nodes| {
                if path.as_ref() == Path::new("a/b/c") {
                    assert_eq!(
                        visited_paths,
                        BTreeSet::from_iter([Arc::from(Path::new("a/"))])
                    );
                    assert_eq!(nodes.get(&()), Some(&LabelPresence::Present));
                } else if path.as_ref() == Path::new("a/") {
                    assert!(visited_paths.is_empty());
                    assert_eq!(nodes.get(&()), Some(&LabelPresence::KnownAbsent));
                } else {
                    panic!("Unknown path");
                }
                // Assert that we only ever visit a path once.
                assert!(visited_paths.insert(path.clone()));
                ControlFlow::Continue(())
            },
        );

        // Test breaking from the tree-walk.
        let mut visited_paths = BTreeSet::new();
        trie.walk(&TriePath::from(Path::new("a/b/c")), &mut |path, nodes| {
            if path.as_ref() == Path::new("a/") {
                assert!(visited_paths.is_empty());
                assert_eq!(nodes.get(&()), Some(&LabelPresence::KnownAbsent));
            } else {
                panic!("Unknown path");
            }
            // Assert that we only ever visit a path once.
            assert!(visited_paths.insert(path.clone()));
            ControlFlow::Break(())
        });
        assert_eq!(visited_paths.len(), 1);

        // Entry removal.
        trie.insert(
            &TriePath::from(Path::new("a/b")),
            (),
            LabelPresence::KnownAbsent,
        );
        let mut visited_paths = BTreeSet::new();
        trie.walk(&TriePath::from(Path::new("a/b/c")), &mut |path, _nodes| {
            // Assert that we only ever visit a path once.
            assert!(visited_paths.insert(path.clone()));
            ControlFlow::Continue(())
        });
        assert_eq!(visited_paths.len(), 3);
        trie.remove(&TriePath::from(Path::new("a/b/")));
        let mut visited_paths = BTreeSet::new();
        trie.walk(&TriePath::from(Path::new("a/b/c")), &mut |path, _nodes| {
            // Assert that we only ever visit a path once.
            assert!(visited_paths.insert(path.clone()));
            ControlFlow::Continue(())
        });
        assert_eq!(visited_paths.len(), 1);
        assert_eq!(
            visited_paths.into_iter().next().unwrap().as_ref(),
            Path::new("a/")
        );
    }

    #[test]
    fn path_to_a_root_can_contain_multiple_known_nodes() {
        let mut trie = RootPathTrie::<()>::new();
        trie.insert(
            &TriePath::from(Path::new("a/b")),
            (),
            LabelPresence::Present,
        );
        trie.insert(&TriePath::from(Path::new("a")), (), LabelPresence::Present);
        let mut visited_paths = BTreeSet::new();
        trie.walk(&TriePath::from(Path::new("a/b/c")), &mut |path, nodes| {
            assert_eq!(nodes.get(&()), Some(&LabelPresence::Present));
            if path.as_ref() != Path::new("a") && path.as_ref() != Path::new("a/b") {
                panic!("Unexpected path: {}", path.as_ref().display());
            }
            assert!(visited_paths.insert(path.clone()));
            ControlFlow::Continue(())
        });
        assert_eq!(visited_paths.len(), 2);
    }
}

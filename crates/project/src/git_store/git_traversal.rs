use git::status::GitSummary;
use std::{ops::Deref, path::Path};
use sum_tree::Cursor;
use text::Bias;
use worktree::{Entry, PathProgress, PathTarget, RepositoryEntry, StatusEntry, Traversal};

/// Walks the worktree entries and their associated git statuses.
pub struct GitTraversal<'a> {
    traversal: Traversal<'a>,
    current_entry_summary: Option<GitSummary>,
    repo_location: Option<(
        &'a RepositoryEntry,
        Cursor<'a, StatusEntry, PathProgress<'a>>,
    )>,
}

impl<'a> GitTraversal<'a> {
    pub fn new(traversal: Traversal<'a>) -> GitTraversal<'a> {
        let mut this = GitTraversal {
            traversal,
            current_entry_summary: None,
            repo_location: None,
        };
        this.synchronize_statuses(true);
        this
    }

    fn synchronize_statuses(&mut self, reset: bool) {
        self.current_entry_summary = None;

        let Some(entry) = self.entry() else {
            return;
        };

        let Some(repo) = self.traversal.snapshot().repository_for_path(&entry.path) else {
            self.repo_location = None;
            return;
        };

        // Update our state if we changed repositories.
        if reset
            || self
                .repo_location
                .as_ref()
                .map(|(prev_repo, _)| &prev_repo.work_directory)
                != Some(&repo.work_directory)
        {
            self.repo_location = Some((repo, repo.statuses_by_path.cursor::<PathProgress>(&())));
        }

        let Some((repo, statuses)) = &mut self.repo_location else {
            return;
        };

        let repo_path = repo.relativize(&entry.path).unwrap();

        if entry.is_dir() {
            let mut statuses = statuses.clone();
            statuses.seek_forward(&PathTarget::Path(repo_path.as_ref()), Bias::Left, &());
            let summary =
                statuses.summary(&PathTarget::Successor(repo_path.as_ref()), Bias::Left, &());

            self.current_entry_summary = Some(summary);
        } else if entry.is_file() {
            // For a file entry, park the cursor on the corresponding status
            if statuses.seek_forward(&PathTarget::Path(repo_path.as_ref()), Bias::Left, &()) {
                // TODO: Investigate statuses.item() being None here.
                self.current_entry_summary = statuses.item().map(|item| item.status.into());
            } else {
                self.current_entry_summary = Some(GitSummary::UNCHANGED);
            }
        }
    }

    pub fn advance(&mut self) -> bool {
        self.advance_by(1)
    }

    pub fn advance_by(&mut self, count: usize) -> bool {
        let found = self.traversal.advance_by(count);
        self.synchronize_statuses(false);
        found
    }

    pub fn advance_to_sibling(&mut self) -> bool {
        let found = self.traversal.advance_to_sibling();
        self.synchronize_statuses(false);
        found
    }

    pub fn back_to_parent(&mut self) -> bool {
        let found = self.traversal.back_to_parent();
        self.synchronize_statuses(true);
        found
    }

    pub fn start_offset(&self) -> usize {
        self.traversal.start_offset()
    }

    pub fn end_offset(&self) -> usize {
        self.traversal.end_offset()
    }

    pub fn entry(&self) -> Option<GitEntryRef<'a>> {
        let entry = self.traversal.entry()?;
        let git_summary = self.current_entry_summary.unwrap_or(GitSummary::UNCHANGED);
        Some(GitEntryRef { entry, git_summary })
    }
}

impl<'a> Iterator for GitTraversal<'a> {
    type Item = GitEntryRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.entry() {
            self.advance();
            Some(item)
        } else {
            None
        }
    }
}

pub struct ChildEntriesGitIter<'a> {
    parent_path: &'a Path,
    traversal: GitTraversal<'a>,
}

impl<'a> ChildEntriesGitIter<'a> {
    pub fn new(snapshot: &'a worktree::Snapshot, parent_path: &'a Path) -> Self {
        let mut traversal =
            GitTraversal::new(snapshot.traverse_from_path(true, true, true, parent_path));
        traversal.advance();
        ChildEntriesGitIter {
            parent_path,
            traversal,
        }
    }
}

impl<'a> Iterator for ChildEntriesGitIter<'a> {
    type Item = GitEntryRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.traversal.entry() {
            if item.path.starts_with(self.parent_path) {
                self.traversal.advance_to_sibling();
                return Some(item);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GitEntryRef<'a> {
    pub entry: &'a Entry,
    pub git_summary: GitSummary,
}

impl GitEntryRef<'_> {
    pub fn to_owned(&self) -> GitEntry {
        GitEntry {
            entry: self.entry.clone(),
            git_summary: self.git_summary,
        }
    }
}

impl Deref for GitEntryRef<'_> {
    type Target = Entry;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl AsRef<Entry> for GitEntryRef<'_> {
    fn as_ref(&self) -> &Entry {
        self.entry
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitEntry {
    pub entry: Entry,
    pub git_summary: GitSummary,
}

impl GitEntry {
    pub fn to_ref(&self) -> GitEntryRef {
        GitEntryRef {
            entry: &self.entry,
            git_summary: self.git_summary,
        }
    }
}

impl Deref for GitEntry {
    type Target = Entry;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl AsRef<Entry> for GitEntry {
    fn as_ref(&self) -> &Entry {
        &self.entry
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use fs::FakeFs;
    use git::status::{FileStatus, StatusCode, TrackedSummary, UnmergedStatus, UnmergedStatusCode};
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::{Settings as _, SettingsStore};
    use util::path;
    use worktree::{Worktree, WorktreeSettings};

    const CONFLICT: FileStatus = FileStatus::Unmerged(UnmergedStatus {
        first_head: UnmergedStatusCode::Updated,
        second_head: UnmergedStatusCode::Updated,
    });
    const ADDED: GitSummary = GitSummary {
        index: TrackedSummary::ADDED,
        count: 1,
        ..GitSummary::UNCHANGED
    };
    const MODIFIED: GitSummary = GitSummary {
        index: TrackedSummary::MODIFIED,
        count: 1,
        ..GitSummary::UNCHANGED
    };

    #[gpui::test]
    async fn test_git_traversal_with_one_repo(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "x": {
                    ".git": {},
                    "x1.txt": "foo",
                    "x2.txt": "bar",
                    "y": {
                        ".git": {},
                        "y1.txt": "baz",
                        "y2.txt": "qux"
                    },
                    "z.txt": "sneaky..."
                },
                "z": {
                    ".git": {},
                    "z1.txt": "quux",
                    "z2.txt": "quuux"
                }
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/x/.git")),
            &[
                (Path::new("x2.txt"), StatusCode::Modified.index()),
                (Path::new("z.txt"), StatusCode::Added.index()),
            ],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/x/y/.git")),
            &[(Path::new("y1.txt"), CONFLICT)],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/z/.git")),
            &[(Path::new("z2.txt"), StatusCode::Added.index())],
        );

        let tree = Worktree::local(
            Path::new(path!("/root")),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        cx.executor().run_until_parked();

        let snapshot = tree.read_with(cx, |tree, _| tree.snapshot());

        let mut traversal =
            GitTraversal::new(snapshot.traverse_from_path(true, false, true, Path::new("x")));

        let entry = traversal.next().unwrap();
        assert_eq!(entry.path.as_ref(), Path::new("x/x1.txt"));
        assert_eq!(entry.git_summary, GitSummary::UNCHANGED);
        let entry = traversal.next().unwrap();
        assert_eq!(entry.path.as_ref(), Path::new("x/x2.txt"));
        assert_eq!(entry.git_summary, MODIFIED);
        let entry = traversal.next().unwrap();
        assert_eq!(entry.path.as_ref(), Path::new("x/y/y1.txt"));
        assert_eq!(entry.git_summary, GitSummary::CONFLICT);
        let entry = traversal.next().unwrap();
        assert_eq!(entry.path.as_ref(), Path::new("x/y/y2.txt"));
        assert_eq!(entry.git_summary, GitSummary::UNCHANGED);
        let entry = traversal.next().unwrap();
        assert_eq!(entry.path.as_ref(), Path::new("x/z.txt"));
        assert_eq!(entry.git_summary, ADDED);
        let entry = traversal.next().unwrap();
        assert_eq!(entry.path.as_ref(), Path::new("z/z1.txt"));
        assert_eq!(entry.git_summary, GitSummary::UNCHANGED);
        let entry = traversal.next().unwrap();
        assert_eq!(entry.path.as_ref(), Path::new("z/z2.txt"));
        assert_eq!(entry.git_summary, ADDED);
    }

    #[gpui::test]
    async fn test_git_traversal_with_nested_repos(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "x": {
                    ".git": {},
                    "x1.txt": "foo",
                    "x2.txt": "bar",
                    "y": {
                        ".git": {},
                        "y1.txt": "baz",
                        "y2.txt": "qux"
                    },
                    "z.txt": "sneaky..."
                },
                "z": {
                    ".git": {},
                    "z1.txt": "quux",
                    "z2.txt": "quuux"
                }
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/x/.git")),
            &[
                (Path::new("x2.txt"), StatusCode::Modified.index()),
                (Path::new("z.txt"), StatusCode::Added.index()),
            ],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/x/y/.git")),
            &[(Path::new("y1.txt"), CONFLICT)],
        );

        fs.set_status_for_repo(
            Path::new(path!("/root/z/.git")),
            &[(Path::new("z2.txt"), StatusCode::Added.index())],
        );

        let tree = Worktree::local(
            Path::new(path!("/root")),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();

        cx.executor().run_until_parked();

        let snapshot = tree.read_with(cx, |tree, _| tree.snapshot());

        // Sanity check the propagation for x/y and z
        check_git_statuses(
            &snapshot,
            &[
                (Path::new("x/y"), GitSummary::CONFLICT),
                (Path::new("x/y/y1.txt"), GitSummary::CONFLICT),
                (Path::new("x/y/y2.txt"), GitSummary::UNCHANGED),
            ],
        );
        check_git_statuses(
            &snapshot,
            &[
                (Path::new("z"), ADDED),
                (Path::new("z/z1.txt"), GitSummary::UNCHANGED),
                (Path::new("z/z2.txt"), ADDED),
            ],
        );

        // Test one of the fundamental cases of propagation blocking, the transition from one git repository to another
        check_git_statuses(
            &snapshot,
            &[
                (Path::new("x"), MODIFIED + ADDED),
                (Path::new("x/y"), GitSummary::CONFLICT),
                (Path::new("x/y/y1.txt"), GitSummary::CONFLICT),
            ],
        );

        // Sanity check everything around it
        check_git_statuses(
            &snapshot,
            &[
                (Path::new("x"), MODIFIED + ADDED),
                (Path::new("x/x1.txt"), GitSummary::UNCHANGED),
                (Path::new("x/x2.txt"), MODIFIED),
                (Path::new("x/y"), GitSummary::CONFLICT),
                (Path::new("x/y/y1.txt"), GitSummary::CONFLICT),
                (Path::new("x/y/y2.txt"), GitSummary::UNCHANGED),
                (Path::new("x/z.txt"), ADDED),
            ],
        );

        // Test the other fundamental case, transitioning from git repository to non-git repository
        check_git_statuses(
            &snapshot,
            &[
                (Path::new(""), GitSummary::UNCHANGED),
                (Path::new("x"), MODIFIED + ADDED),
                (Path::new("x/x1.txt"), GitSummary::UNCHANGED),
            ],
        );

        // And all together now
        check_git_statuses(
            &snapshot,
            &[
                (Path::new(""), GitSummary::UNCHANGED),
                (Path::new("x"), MODIFIED + ADDED),
                (Path::new("x/x1.txt"), GitSummary::UNCHANGED),
                (Path::new("x/x2.txt"), MODIFIED),
                (Path::new("x/y"), GitSummary::CONFLICT),
                (Path::new("x/y/y1.txt"), GitSummary::CONFLICT),
                (Path::new("x/y/y2.txt"), GitSummary::UNCHANGED),
                (Path::new("x/z.txt"), ADDED),
                (Path::new("z"), ADDED),
                (Path::new("z/z1.txt"), GitSummary::UNCHANGED),
                (Path::new("z/z2.txt"), ADDED),
            ],
        );
    }

    #[gpui::test]
    async fn test_git_traversal_simple(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".git": {},
                "a": {
                    "b": {
                        "c1.txt": "",
                        "c2.txt": "",
                    },
                    "d": {
                        "e1.txt": "",
                        "e2.txt": "",
                        "e3.txt": "",
                    }
                },
                "f": {
                    "no-status.txt": ""
                },
                "g": {
                    "h1.txt": "",
                    "h2.txt": ""
                },
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/.git")),
            &[
                (Path::new("a/b/c1.txt"), StatusCode::Added.index()),
                (Path::new("a/d/e2.txt"), StatusCode::Modified.index()),
                (Path::new("g/h2.txt"), CONFLICT),
            ],
        );

        let tree = Worktree::local(
            Path::new(path!("/root")),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.executor().run_until_parked();

        let snapshot = tree.read_with(cx, |tree, _| tree.snapshot());

        check_git_statuses(
            &snapshot,
            &[
                (Path::new(""), GitSummary::CONFLICT + MODIFIED + ADDED),
                (Path::new("g"), GitSummary::CONFLICT),
                (Path::new("g/h2.txt"), GitSummary::CONFLICT),
            ],
        );

        check_git_statuses(
            &snapshot,
            &[
                (Path::new(""), GitSummary::CONFLICT + ADDED + MODIFIED),
                (Path::new("a"), ADDED + MODIFIED),
                (Path::new("a/b"), ADDED),
                (Path::new("a/b/c1.txt"), ADDED),
                (Path::new("a/b/c2.txt"), GitSummary::UNCHANGED),
                (Path::new("a/d"), MODIFIED),
                (Path::new("a/d/e2.txt"), MODIFIED),
                (Path::new("f"), GitSummary::UNCHANGED),
                (Path::new("f/no-status.txt"), GitSummary::UNCHANGED),
                (Path::new("g"), GitSummary::CONFLICT),
                (Path::new("g/h2.txt"), GitSummary::CONFLICT),
            ],
        );

        check_git_statuses(
            &snapshot,
            &[
                (Path::new("a/b"), ADDED),
                (Path::new("a/b/c1.txt"), ADDED),
                (Path::new("a/b/c2.txt"), GitSummary::UNCHANGED),
                (Path::new("a/d"), MODIFIED),
                (Path::new("a/d/e1.txt"), GitSummary::UNCHANGED),
                (Path::new("a/d/e2.txt"), MODIFIED),
                (Path::new("f"), GitSummary::UNCHANGED),
                (Path::new("f/no-status.txt"), GitSummary::UNCHANGED),
                (Path::new("g"), GitSummary::CONFLICT),
            ],
        );

        check_git_statuses(
            &snapshot,
            &[
                (Path::new("a/b/c1.txt"), ADDED),
                (Path::new("a/b/c2.txt"), GitSummary::UNCHANGED),
                (Path::new("a/d/e1.txt"), GitSummary::UNCHANGED),
                (Path::new("a/d/e2.txt"), MODIFIED),
                (Path::new("f/no-status.txt"), GitSummary::UNCHANGED),
            ],
        );
    }

    #[gpui::test]
    async fn test_git_traversal_with_repos_under_project(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                "x": {
                    ".git": {},
                    "x1.txt": "foo",
                    "x2.txt": "bar"
                },
                "y": {
                    ".git": {},
                    "y1.txt": "baz",
                    "y2.txt": "qux"
                },
                "z": {
                    ".git": {},
                    "z1.txt": "quux",
                    "z2.txt": "quuux"
                }
            }),
        )
        .await;

        fs.set_status_for_repo(
            Path::new(path!("/root/x/.git")),
            &[(Path::new("x1.txt"), StatusCode::Added.index())],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/y/.git")),
            &[
                (Path::new("y1.txt"), CONFLICT),
                (Path::new("y2.txt"), StatusCode::Modified.index()),
            ],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/z/.git")),
            &[(Path::new("z2.txt"), StatusCode::Modified.index())],
        );

        let tree = Worktree::local(
            Path::new(path!("/root")),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.executor().run_until_parked();

        let snapshot = tree.read_with(cx, |tree, _| tree.snapshot());

        check_git_statuses(
            &snapshot,
            &[(Path::new("x"), ADDED), (Path::new("x/x1.txt"), ADDED)],
        );

        check_git_statuses(
            &snapshot,
            &[
                (Path::new("y"), GitSummary::CONFLICT + MODIFIED),
                (Path::new("y/y1.txt"), GitSummary::CONFLICT),
                (Path::new("y/y2.txt"), MODIFIED),
            ],
        );

        check_git_statuses(
            &snapshot,
            &[
                (Path::new("z"), MODIFIED),
                (Path::new("z/z2.txt"), MODIFIED),
            ],
        );

        check_git_statuses(
            &snapshot,
            &[(Path::new("x"), ADDED), (Path::new("x/x1.txt"), ADDED)],
        );

        check_git_statuses(
            &snapshot,
            &[
                (Path::new("x"), ADDED),
                (Path::new("x/x1.txt"), ADDED),
                (Path::new("x/x2.txt"), GitSummary::UNCHANGED),
                (Path::new("y"), GitSummary::CONFLICT + MODIFIED),
                (Path::new("y/y1.txt"), GitSummary::CONFLICT),
                (Path::new("y/y2.txt"), MODIFIED),
                (Path::new("z"), MODIFIED),
                (Path::new("z/z1.txt"), GitSummary::UNCHANGED),
                (Path::new("z/z2.txt"), MODIFIED),
            ],
        );
    }

    fn init_test(cx: &mut gpui::TestAppContext) {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::try_init().ok();
        }

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            WorktreeSettings::register(cx);
        });
    }

    #[gpui::test]
    async fn test_bump_mtime_of_git_repo_workdir(cx: &mut TestAppContext) {
        init_test(cx);

        // Create a worktree with a git directory.
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".git": {},
                "a.txt": "",
                "b": {
                    "c.txt": "",
                },
            }),
        )
        .await;
        fs.set_head_and_index_for_repo(
            path!("/root/.git").as_ref(),
            &[("a.txt".into(), "".into()), ("b/c.txt".into(), "".into())],
        );
        cx.run_until_parked();

        let tree = Worktree::local(
            path!("/root").as_ref(),
            true,
            fs.clone(),
            Default::default(),
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        cx.executor().run_until_parked();

        let (old_entry_ids, old_mtimes) = tree.read_with(cx, |tree, _| {
            (
                tree.entries(true, 0).map(|e| e.id).collect::<Vec<_>>(),
                tree.entries(true, 0).map(|e| e.mtime).collect::<Vec<_>>(),
            )
        });

        // Regression test: after the directory is scanned, touch the git repo's
        // working directory, bumping its mtime. That directory keeps its project
        // entry id after the directories are re-scanned.
        fs.touch_path(path!("/root")).await;
        cx.executor().run_until_parked();

        let (new_entry_ids, new_mtimes) = tree.read_with(cx, |tree, _| {
            (
                tree.entries(true, 0).map(|e| e.id).collect::<Vec<_>>(),
                tree.entries(true, 0).map(|e| e.mtime).collect::<Vec<_>>(),
            )
        });
        assert_eq!(new_entry_ids, old_entry_ids);
        assert_ne!(new_mtimes, old_mtimes);

        // Regression test: changes to the git repository should still be
        // detected.
        fs.set_head_for_repo(
            path!("/root/.git").as_ref(),
            &[
                ("a.txt".into(), "".into()),
                ("b/c.txt".into(), "something-else".into()),
            ],
        );
        cx.executor().run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));

        let snapshot = tree.read_with(cx, |tree, _| tree.snapshot());

        check_git_statuses(
            &snapshot,
            &[
                (Path::new(""), MODIFIED),
                (Path::new("a.txt"), GitSummary::UNCHANGED),
                (Path::new("b/c.txt"), MODIFIED),
            ],
        );
    }

    #[track_caller]
    fn check_git_statuses(
        snapshot: &worktree::Snapshot,
        expected_statuses: &[(&Path, GitSummary)],
    ) {
        let mut traversal =
            GitTraversal::new(snapshot.traverse_from_path(true, true, false, "".as_ref()));
        let found_statuses = expected_statuses
            .iter()
            .map(|&(path, _)| {
                let git_entry = traversal
                    .find(|git_entry| &*git_entry.path == path)
                    .unwrap_or_else(|| panic!("Traversal has no entry for {path:?}"));
                (path, git_entry.git_summary)
            })
            .collect::<Vec<_>>();
        assert_eq!(found_statuses, expected_statuses);
    }
}

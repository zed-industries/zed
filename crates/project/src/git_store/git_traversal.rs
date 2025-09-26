use collections::HashMap;
use git::{repository::RepoPath, status::GitSummary};
use std::{collections::BTreeMap, ops::Deref, path::Path};
use sum_tree::Cursor;
use text::Bias;
use util::rel_path::RelPath;
use worktree::{Entry, PathProgress, PathTarget, Traversal};

use super::{RepositoryId, RepositorySnapshot, StatusEntry};

/// Walks the worktree entries and their associated git statuses.
pub struct GitTraversal<'a> {
    traversal: Traversal<'a>,
    current_entry_summary: Option<GitSummary>,
    repo_root_to_snapshot: BTreeMap<&'a Path, &'a RepositorySnapshot>,
    repo_location: Option<(
        RepositoryId,
        Cursor<'a, 'static, StatusEntry, PathProgress<'a>>,
    )>,
}

impl<'a> GitTraversal<'a> {
    pub fn new(
        repo_snapshots: &'a HashMap<RepositoryId, RepositorySnapshot>,
        traversal: Traversal<'a>,
    ) -> GitTraversal<'a> {
        let repo_root_to_snapshot = repo_snapshots
            .values()
            .map(|snapshot| (&*snapshot.work_directory_abs_path, snapshot))
            .collect();
        let mut this = GitTraversal {
            traversal,
            current_entry_summary: None,
            repo_location: None,
            repo_root_to_snapshot,
        };
        this.synchronize_statuses(true);
        this
    }

    fn repo_root_for_path(&self, path: &Path) -> Option<(&'a RepositorySnapshot, RepoPath)> {
        // We might need to perform a range search multiple times, as there may be a nested repository inbetween
        // the target and our path. E.g:
        // /our_root_repo/
        //   .git/
        //   other_repo/
        //     .git/
        //   our_query.txt
        let query = path.ancestors();
        for query in query {
            let (_, snapshot) = self
                .repo_root_to_snapshot
                .range(Path::new("")..=query)
                .last()?;

            let stripped = snapshot
                .abs_path_to_repo_path(path)
                .map(|repo_path| (*snapshot, repo_path));
            if stripped.is_some() {
                return stripped;
            }
        }

        None
    }

    fn synchronize_statuses(&mut self, reset: bool) {
        self.current_entry_summary = None;

        let Some(entry) = self.entry() else {
            return;
        };

        let abs_path = self.traversal.snapshot().absolutize(&entry.path);

        let Some((repo, repo_path)) = self.repo_root_for_path(&abs_path) else {
            self.repo_location = None;
            return;
        };

        // Update our state if we changed repositories.
        if reset
            || self
                .repo_location
                .as_ref()
                .map(|(prev_repo_id, _)| *prev_repo_id)
                != Some(repo.id)
        {
            self.repo_location = Some((repo.id, repo.statuses_by_path.cursor::<PathProgress>(())));
        }

        let Some((_, statuses)) = &mut self.repo_location else {
            return;
        };

        if entry.is_dir() {
            let mut statuses = statuses.clone();
            statuses.seek_forward(&PathTarget::Path(&repo_path), Bias::Left);
            let summary = statuses.summary(&PathTarget::Successor(&repo_path), Bias::Left);

            self.current_entry_summary = Some(summary);
        } else if entry.is_file() {
            // For a file entry, park the cursor on the corresponding status
            if statuses.seek_forward(&PathTarget::Path(&repo_path), Bias::Left) {
                // TODO: Investigate statuses.item() being None here.
                self.current_entry_summary = statuses.item().map(|item| item.status.into());
            } else {
                self.current_entry_summary = Some(GitSummary::UNCHANGED);
            }
        }
    }

    pub fn advance(&mut self) -> bool {
        let found = self.traversal.advance_by(1);
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
    parent_path: &'a RelPath,
    traversal: GitTraversal<'a>,
}

impl<'a> ChildEntriesGitIter<'a> {
    pub fn new(
        repo_snapshots: &'a HashMap<RepositoryId, RepositorySnapshot>,
        worktree_snapshot: &'a worktree::Snapshot,
        parent_path: &'a RelPath,
    ) -> Self {
        let mut traversal = GitTraversal::new(
            repo_snapshots,
            worktree_snapshot.traverse_from_path(true, true, true, parent_path),
        );
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
        if let Some(item) = self.traversal.entry()
            && item.path.starts_with(self.parent_path)
        {
            self.traversal.advance_to_sibling();
            return Some(item);
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
    pub fn to_owned(self) -> GitEntry {
        GitEntry {
            entry: self.entry.clone(),
            git_summary: self.git_summary,
        }
    }
}

impl Deref for GitEntryRef<'_> {
    type Target = Entry;

    fn deref(&self) -> &Self::Target {
        self.entry
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
    pub fn to_ref(&self) -> GitEntryRef<'_> {
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

    use crate::Project;

    use super::*;
    use fs::FakeFs;
    use git::status::{FileStatus, StatusCode, TrackedSummary, UnmergedStatus, UnmergedStatusCode};
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::SettingsStore;
    use util::{path, rel_path::rel_path};

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
                ("x2.txt", StatusCode::Modified.index()),
                ("z.txt", StatusCode::Added.index()),
            ],
        );
        fs.set_status_for_repo(Path::new(path!("/root/x/y/.git")), &[("y1.txt", CONFLICT)]);
        fs.set_status_for_repo(
            Path::new(path!("/root/z/.git")),
            &[("z2.txt", StatusCode::Added.index())],
        );

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let (repo_snapshots, worktree_snapshot) = project.read_with(cx, |project, cx| {
            (
                project.git_store().read(cx).repo_snapshots(cx),
                project.worktrees(cx).next().unwrap().read(cx).snapshot(),
            )
        });

        let traversal = GitTraversal::new(
            &repo_snapshots,
            worktree_snapshot.traverse_from_path(true, false, true, RelPath::unix("x").unwrap()),
        );
        let entries = traversal
            .map(|entry| (entry.path.clone(), entry.git_summary))
            .collect::<Vec<_>>();
        pretty_assertions::assert_eq!(
            entries,
            [
                (rel_path("x/x1.txt").into(), GitSummary::UNCHANGED),
                (rel_path("x/x2.txt").into(), MODIFIED),
                (rel_path("x/y/y1.txt").into(), GitSummary::CONFLICT),
                (rel_path("x/y/y2.txt").into(), GitSummary::UNCHANGED),
                (rel_path("x/z.txt").into(), ADDED),
                (rel_path("z/z1.txt").into(), GitSummary::UNCHANGED),
                (rel_path("z/z2.txt").into(), ADDED),
            ]
        )
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
                ("x2.txt", StatusCode::Modified.index()),
                ("z.txt", StatusCode::Added.index()),
            ],
        );
        fs.set_status_for_repo(Path::new(path!("/root/x/y/.git")), &[("y1.txt", CONFLICT)]);

        fs.set_status_for_repo(
            Path::new(path!("/root/z/.git")),
            &[("z2.txt", StatusCode::Added.index())],
        );

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let (repo_snapshots, worktree_snapshot) = project.read_with(cx, |project, cx| {
            (
                project.git_store().read(cx).repo_snapshots(cx),
                project.worktrees(cx).next().unwrap().read(cx).snapshot(),
            )
        });

        // Sanity check the propagation for x/y and z
        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("x/y", GitSummary::CONFLICT),
                ("x/y/y1.txt", GitSummary::CONFLICT),
                ("x/y/y2.txt", GitSummary::UNCHANGED),
            ],
        );
        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("z", ADDED),
                ("z/z1.txt", GitSummary::UNCHANGED),
                ("z/z2.txt", ADDED),
            ],
        );

        // Test one of the fundamental cases of propagation blocking, the transition from one git repository to another
        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("x", MODIFIED + ADDED),
                ("x/y", GitSummary::CONFLICT),
                ("x/y/y1.txt", GitSummary::CONFLICT),
            ],
        );

        // Sanity check everything around it
        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("x", MODIFIED + ADDED),
                ("x/x1.txt", GitSummary::UNCHANGED),
                ("x/x2.txt", MODIFIED),
                ("x/y", GitSummary::CONFLICT),
                ("x/y/y1.txt", GitSummary::CONFLICT),
                ("x/y/y2.txt", GitSummary::UNCHANGED),
                ("x/z.txt", ADDED),
            ],
        );

        // Test the other fundamental case, transitioning from git repository to non-git repository
        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("", GitSummary::UNCHANGED),
                ("x", MODIFIED + ADDED),
                ("x/x1.txt", GitSummary::UNCHANGED),
            ],
        );

        // And all together now
        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("", GitSummary::UNCHANGED),
                ("x", MODIFIED + ADDED),
                ("x/x1.txt", GitSummary::UNCHANGED),
                ("x/x2.txt", MODIFIED),
                ("x/y", GitSummary::CONFLICT),
                ("x/y/y1.txt", GitSummary::CONFLICT),
                ("x/y/y2.txt", GitSummary::UNCHANGED),
                ("x/z.txt", ADDED),
                ("z", ADDED),
                ("z/z1.txt", GitSummary::UNCHANGED),
                ("z/z2.txt", ADDED),
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
                ("a/b/c1.txt", StatusCode::Added.index()),
                ("a/d/e2.txt", StatusCode::Modified.index()),
                ("g/h2.txt", CONFLICT),
            ],
        );

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let (repo_snapshots, worktree_snapshot) = project.read_with(cx, |project, cx| {
            (
                project.git_store().read(cx).repo_snapshots(cx),
                project.worktrees(cx).next().unwrap().read(cx).snapshot(),
            )
        });

        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("", GitSummary::CONFLICT + MODIFIED + ADDED),
                ("g", GitSummary::CONFLICT),
                ("g/h2.txt", GitSummary::CONFLICT),
            ],
        );

        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("", GitSummary::CONFLICT + ADDED + MODIFIED),
                ("a", ADDED + MODIFIED),
                ("a/b", ADDED),
                ("a/b/c1.txt", ADDED),
                ("a/b/c2.txt", GitSummary::UNCHANGED),
                ("a/d", MODIFIED),
                ("a/d/e2.txt", MODIFIED),
                ("f", GitSummary::UNCHANGED),
                ("f/no-status.txt", GitSummary::UNCHANGED),
                ("g", GitSummary::CONFLICT),
                ("g/h2.txt", GitSummary::CONFLICT),
            ],
        );

        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("a/b", ADDED),
                ("a/b/c1.txt", ADDED),
                ("a/b/c2.txt", GitSummary::UNCHANGED),
                ("a/d", MODIFIED),
                ("a/d/e1.txt", GitSummary::UNCHANGED),
                ("a/d/e2.txt", MODIFIED),
                ("f", GitSummary::UNCHANGED),
                ("f/no-status.txt", GitSummary::UNCHANGED),
                ("g", GitSummary::CONFLICT),
            ],
        );

        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("a/b/c1.txt", ADDED),
                ("a/b/c2.txt", GitSummary::UNCHANGED),
                ("a/d/e1.txt", GitSummary::UNCHANGED),
                ("a/d/e2.txt", MODIFIED),
                ("f/no-status.txt", GitSummary::UNCHANGED),
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
            &[("x1.txt", StatusCode::Added.index())],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/y/.git")),
            &[
                ("y1.txt", CONFLICT),
                ("y2.txt", StatusCode::Modified.index()),
            ],
        );
        fs.set_status_for_repo(
            Path::new(path!("/root/z/.git")),
            &[("z2.txt", StatusCode::Modified.index())],
        );

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let (repo_snapshots, worktree_snapshot) = project.read_with(cx, |project, cx| {
            (
                project.git_store().read(cx).repo_snapshots(cx),
                project.worktrees(cx).next().unwrap().read(cx).snapshot(),
            )
        });

        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[("x", ADDED), ("x/x1.txt", ADDED)],
        );

        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("y", GitSummary::CONFLICT + MODIFIED),
                ("y/y1.txt", GitSummary::CONFLICT),
                ("y/y2.txt", MODIFIED),
            ],
        );

        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[("z", MODIFIED), ("z/z2.txt", MODIFIED)],
        );

        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[("x", ADDED), ("x/x1.txt", ADDED)],
        );

        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("x", ADDED),
                ("x/x1.txt", ADDED),
                ("x/x2.txt", GitSummary::UNCHANGED),
                ("y", GitSummary::CONFLICT + MODIFIED),
                ("y/y1.txt", GitSummary::CONFLICT),
                ("y/y2.txt", MODIFIED),
                ("z", MODIFIED),
                ("z/z1.txt", GitSummary::UNCHANGED),
                ("z/z2.txt", MODIFIED),
            ],
        );
    }

    fn init_test(cx: &mut gpui::TestAppContext) {
        zlog::init_test();

        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
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
            &[("a.txt", "".into()), ("b/c.txt", "".into())],
        );
        cx.run_until_parked();

        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let (old_entry_ids, old_mtimes) = project.read_with(cx, |project, cx| {
            let tree = project.worktrees(cx).next().unwrap().read(cx);
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

        let (new_entry_ids, new_mtimes) = project.read_with(cx, |project, cx| {
            let tree = project.worktrees(cx).next().unwrap().read(cx);
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
            &[("a.txt", "".into()), ("b/c.txt", "something-else".into())],
            "deadbeef",
        );
        cx.executor().run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(1));

        let (repo_snapshots, worktree_snapshot) = project.read_with(cx, |project, cx| {
            (
                project.git_store().read(cx).repo_snapshots(cx),
                project.worktrees(cx).next().unwrap().read(cx).snapshot(),
            )
        });

        check_git_statuses(
            &repo_snapshots,
            &worktree_snapshot,
            &[
                ("", MODIFIED),
                ("a.txt", GitSummary::UNCHANGED),
                ("b/c.txt", MODIFIED),
            ],
        );
    }

    #[track_caller]
    fn check_git_statuses(
        repo_snapshots: &HashMap<RepositoryId, RepositorySnapshot>,
        worktree_snapshot: &worktree::Snapshot,
        expected_statuses: &[(&str, GitSummary)],
    ) {
        let mut traversal = GitTraversal::new(
            repo_snapshots,
            worktree_snapshot.traverse_from_path(true, true, false, RelPath::empty()),
        );
        let found_statuses = expected_statuses
            .iter()
            .map(|&(path, _)| {
                let git_entry = traversal
                    .find(|git_entry| git_entry.path.as_ref() == rel_path(path))
                    .unwrap_or_else(|| panic!("Traversal has no entry for {path:?}"));
                (path, git_entry.git_summary)
            })
            .collect::<Vec<_>>();
        pretty_assertions::assert_eq!(found_statuses, expected_statuses);
    }
}

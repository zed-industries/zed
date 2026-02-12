mod conflict_set_tests {
    use std::sync::mpsc;

    use crate::Project;

    use fs::FakeFs;
    use git::{
        repository::{RepoPath, repo_path},
        status::{UnmergedStatus, UnmergedStatusCode},
    };
    use gpui::{BackgroundExecutor, TestAppContext};
    use project::git_store::*;
    use serde_json::json;
    use text::{Buffer, BufferId, OffsetRangeExt, Point, ReplicaId, ToOffset as _};
    use unindent::Unindent as _;
    use util::{path, rel_path::rel_path};

    #[test]
    fn test_parse_conflicts_in_buffer() {
        // Create a buffer with conflict markers
        let test_content = r#"
            This is some text before the conflict.
            <<<<<<< HEAD
            This is our version
            =======
            This is their version
            >>>>>>> branch-name

            Another conflict:
            <<<<<<< HEAD
            Our second change
            ||||||| merged common ancestors
            Original content
            =======
            Their second change
            >>>>>>> branch-name
        "#
        .unindent();

        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(ReplicaId::LOCAL, buffer_id, test_content);
        let snapshot = buffer.snapshot();

        let conflict_snapshot = ConflictSet::parse(&snapshot);
        assert_eq!(conflict_snapshot.conflicts.len(), 2);

        let first = &conflict_snapshot.conflicts[0];
        assert!(first.base.is_none());
        assert_eq!(first.ours_branch_name.as_ref(), "HEAD");
        assert_eq!(first.theirs_branch_name.as_ref(), "branch-name");
        let our_text = snapshot
            .text_for_range(first.ours.clone())
            .collect::<String>();
        let their_text = snapshot
            .text_for_range(first.theirs.clone())
            .collect::<String>();
        assert_eq!(our_text, "This is our version\n");
        assert_eq!(their_text, "This is their version\n");

        let second = &conflict_snapshot.conflicts[1];
        assert!(second.base.is_some());
        assert_eq!(second.ours_branch_name.as_ref(), "HEAD");
        assert_eq!(second.theirs_branch_name.as_ref(), "branch-name");
        let our_text = snapshot
            .text_for_range(second.ours.clone())
            .collect::<String>();
        let their_text = snapshot
            .text_for_range(second.theirs.clone())
            .collect::<String>();
        let base_text = snapshot
            .text_for_range(second.base.as_ref().unwrap().clone())
            .collect::<String>();
        assert_eq!(our_text, "Our second change\n");
        assert_eq!(their_text, "Their second change\n");
        assert_eq!(base_text, "Original content\n");

        // Test conflicts_in_range
        let range = snapshot.anchor_before(0)..snapshot.anchor_before(snapshot.len());
        let conflicts_in_range = conflict_snapshot.conflicts_in_range(range, &snapshot);
        assert_eq!(conflicts_in_range.len(), 2);

        // Test with a range that includes only the first conflict
        let first_conflict_end = conflict_snapshot.conflicts[0].range.end;
        let range = snapshot.anchor_before(0)..first_conflict_end;
        let conflicts_in_range = conflict_snapshot.conflicts_in_range(range, &snapshot);
        assert_eq!(conflicts_in_range.len(), 1);

        // Test with a range that includes only the second conflict
        let second_conflict_start = conflict_snapshot.conflicts[1].range.start;
        let range = second_conflict_start..snapshot.anchor_before(snapshot.len());
        let conflicts_in_range = conflict_snapshot.conflicts_in_range(range, &snapshot);
        assert_eq!(conflicts_in_range.len(), 1);

        // Test with a range that doesn't include any conflicts
        let range = buffer.anchor_after(first_conflict_end.to_next_offset(&buffer))
            ..buffer.anchor_before(second_conflict_start.to_previous_offset(&buffer));
        let conflicts_in_range = conflict_snapshot.conflicts_in_range(range, &snapshot);
        assert_eq!(conflicts_in_range.len(), 0);
    }

    #[test]
    fn test_nested_conflict_markers() {
        // Create a buffer with nested conflict markers
        let test_content = r#"
            This is some text before the conflict.
            <<<<<<< HEAD
            This is our version
            <<<<<<< HEAD
            This is a nested conflict marker
            =======
            This is their version in a nested conflict
            >>>>>>> branch-nested
            =======
            This is their version
            >>>>>>> branch-name
        "#
        .unindent();

        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(ReplicaId::LOCAL, buffer_id, test_content);
        let snapshot = buffer.snapshot();

        let conflict_snapshot = ConflictSet::parse(&snapshot);

        assert_eq!(conflict_snapshot.conflicts.len(), 1);

        // The conflict should have our version, their version, but no base
        let conflict = &conflict_snapshot.conflicts[0];
        assert!(conflict.base.is_none());
        assert_eq!(conflict.ours_branch_name.as_ref(), "HEAD");
        assert_eq!(conflict.theirs_branch_name.as_ref(), "branch-nested");

        // Check that the nested conflict was detected correctly
        let our_text = snapshot
            .text_for_range(conflict.ours.clone())
            .collect::<String>();
        assert_eq!(our_text, "This is a nested conflict marker\n");
        let their_text = snapshot
            .text_for_range(conflict.theirs.clone())
            .collect::<String>();
        assert_eq!(their_text, "This is their version in a nested conflict\n");
    }

    #[test]
    fn test_conflict_markers_at_eof() {
        let test_content = r#"
            <<<<<<< ours
            =======
            This is their version
            >>>>>>> "#
            .unindent();
        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(ReplicaId::LOCAL, buffer_id, test_content);
        let snapshot = buffer.snapshot();

        let conflict_snapshot = ConflictSet::parse(&snapshot);
        assert_eq!(conflict_snapshot.conflicts.len(), 1);
        assert_eq!(
            conflict_snapshot.conflicts[0].ours_branch_name.as_ref(),
            "ours"
        );
        assert_eq!(
            conflict_snapshot.conflicts[0].theirs_branch_name.as_ref(),
            "Origin" // default branch name if there is none
        );
    }

    #[test]
    fn test_conflicts_in_range() {
        // Create a buffer with conflict markers
        let test_content = r#"
            one
            <<<<<<< HEAD1
            two
            =======
            three
            >>>>>>> branch1
            four
            five
            <<<<<<< HEAD2
            six
            =======
            seven
            >>>>>>> branch2
            eight
            nine
            <<<<<<< HEAD3
            ten
            =======
            eleven
            >>>>>>> branch3
            twelve
            <<<<<<< HEAD4
            thirteen
            =======
            fourteen
            >>>>>>> branch4
            fifteen
        "#
        .unindent();

        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(ReplicaId::LOCAL, buffer_id, test_content.clone());
        let snapshot = buffer.snapshot();

        let conflict_snapshot = ConflictSet::parse(&snapshot);
        assert_eq!(conflict_snapshot.conflicts.len(), 4);
        assert_eq!(
            conflict_snapshot.conflicts[0].ours_branch_name.as_ref(),
            "HEAD1"
        );
        assert_eq!(
            conflict_snapshot.conflicts[0].theirs_branch_name.as_ref(),
            "branch1"
        );
        assert_eq!(
            conflict_snapshot.conflicts[1].ours_branch_name.as_ref(),
            "HEAD2"
        );
        assert_eq!(
            conflict_snapshot.conflicts[1].theirs_branch_name.as_ref(),
            "branch2"
        );
        assert_eq!(
            conflict_snapshot.conflicts[2].ours_branch_name.as_ref(),
            "HEAD3"
        );
        assert_eq!(
            conflict_snapshot.conflicts[2].theirs_branch_name.as_ref(),
            "branch3"
        );
        assert_eq!(
            conflict_snapshot.conflicts[3].ours_branch_name.as_ref(),
            "HEAD4"
        );
        assert_eq!(
            conflict_snapshot.conflicts[3].theirs_branch_name.as_ref(),
            "branch4"
        );

        let range = test_content.find("seven").unwrap()..test_content.find("eleven").unwrap();
        let range = buffer.anchor_before(range.start)..buffer.anchor_after(range.end);
        assert_eq!(
            conflict_snapshot.conflicts_in_range(range, &snapshot),
            &conflict_snapshot.conflicts[1..=2]
        );

        let range = test_content.find("one").unwrap()..test_content.find("<<<<<<< HEAD2").unwrap();
        let range = buffer.anchor_before(range.start)..buffer.anchor_after(range.end);
        assert_eq!(
            conflict_snapshot.conflicts_in_range(range, &snapshot),
            &conflict_snapshot.conflicts[0..=1]
        );

        let range =
            test_content.find("eight").unwrap() - 1..test_content.find(">>>>>>> branch3").unwrap();
        let range = buffer.anchor_before(range.start)..buffer.anchor_after(range.end);
        assert_eq!(
            conflict_snapshot.conflicts_in_range(range, &snapshot),
            &conflict_snapshot.conflicts[1..=2]
        );

        let range = test_content.find("thirteen").unwrap() - 1..test_content.len();
        let range = buffer.anchor_before(range.start)..buffer.anchor_after(range.end);
        assert_eq!(
            conflict_snapshot.conflicts_in_range(range, &snapshot),
            &conflict_snapshot.conflicts[3..=3]
        );
    }

    #[gpui::test]
    async fn test_conflict_updates(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        zlog::init_test();
        cx.update(|cx| {
            settings::init(cx);
        });
        let initial_text = "
            one
            two
            three
            four
            five
        "
        .unindent();
        let fs = FakeFs::new(executor);
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "a.txt": initial_text,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (git_store, buffer) = project.update(cx, |project, cx| {
            (
                project.git_store().clone(),
                project.open_local_buffer(path!("/project/a.txt"), cx),
            )
        });
        let buffer = buffer.await.unwrap();
        let conflict_set = git_store.update(cx, |git_store, cx| {
            git_store.open_conflict_set(buffer.clone(), cx)
        });
        let (events_tx, events_rx) = mpsc::channel::<ConflictSetUpdate>();
        let _conflict_set_subscription = cx.update(|cx| {
            cx.subscribe(&conflict_set, move |_, event, _| {
                events_tx.send(event.clone()).ok();
            })
        });
        let conflicts_snapshot =
            conflict_set.read_with(cx, |conflict_set, _| conflict_set.snapshot());
        assert!(conflicts_snapshot.conflicts.is_empty());

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [
                    (4..4, "<<<<<<< HEAD\n"),
                    (14..14, "=======\nTWO\n>>>>>>> branch\n"),
                ],
                None,
                cx,
            );
        });

        cx.run_until_parked();
        events_rx.try_recv().expect_err(
            "no conflicts should be registered as long as the file's status is unchanged",
        );

        fs.with_git_state(path!("/project/.git").as_ref(), true, |state| {
            state.unmerged_paths.insert(
                repo_path("a.txt"),
                UnmergedStatus {
                    first_head: UnmergedStatusCode::Updated,
                    second_head: UnmergedStatusCode::Updated,
                },
            );
            // Cause the repository to emit MergeHeadsChanged.
            state.refs.insert("MERGE_HEAD".into(), "123".into())
        })
        .unwrap();

        cx.run_until_parked();
        let update = events_rx
            .try_recv()
            .expect("status change should trigger conflict parsing");
        assert_eq!(update.old_range, 0..0);
        assert_eq!(update.new_range, 0..1);

        let conflict = conflict_set.read_with(cx, |conflict_set, _| {
            conflict_set.snapshot().conflicts[0].clone()
        });
        cx.update(|cx| {
            conflict.resolve(buffer.clone(), std::slice::from_ref(&conflict.theirs), cx);
        });

        cx.run_until_parked();
        let update = events_rx
            .try_recv()
            .expect("conflicts should be removed after resolution");
        assert_eq!(update.old_range, 0..1);
        assert_eq!(update.new_range, 0..0);
    }

    #[gpui::test]
    async fn test_conflict_updates_without_merge_head(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        zlog::init_test();
        cx.update(|cx| {
            settings::init(cx);
        });

        let initial_text = "
            zero
            <<<<<<< HEAD
            one
            =======
            two
            >>>>>>> Stashed Changes
            three
        "
        .unindent();

        let fs = FakeFs::new(executor);
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "a.txt": initial_text,
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (git_store, buffer) = project.update(cx, |project, cx| {
            (
                project.git_store().clone(),
                project.open_local_buffer(path!("/project/a.txt"), cx),
            )
        });

        cx.run_until_parked();
        fs.with_git_state(path!("/project/.git").as_ref(), true, |state| {
            state.unmerged_paths.insert(
                RepoPath::from_rel_path(rel_path("a.txt")),
                UnmergedStatus {
                    first_head: UnmergedStatusCode::Updated,
                    second_head: UnmergedStatusCode::Updated,
                },
            )
        })
        .unwrap();

        let buffer = buffer.await.unwrap();

        // Open the conflict set for a file that currently has conflicts.
        let conflict_set = git_store.update(cx, |git_store, cx| {
            git_store.open_conflict_set(buffer.clone(), cx)
        });

        cx.run_until_parked();
        conflict_set.update(cx, |conflict_set, cx| {
            let conflict_range = conflict_set.snapshot().conflicts[0]
                .range
                .to_point(buffer.read(cx));
            assert_eq!(conflict_range, Point::new(1, 0)..Point::new(6, 0));
        });

        // Simulate the conflict being removed by e.g. staging the file.
        fs.with_git_state(path!("/project/.git").as_ref(), true, |state| {
            state.unmerged_paths.remove(&repo_path("a.txt"))
        })
        .unwrap();

        cx.run_until_parked();
        conflict_set.update(cx, |conflict_set, _| {
            assert!(!conflict_set.has_conflict);
            assert_eq!(conflict_set.snapshot.conflicts.len(), 0);
        });

        // Simulate the conflict being re-added.
        fs.with_git_state(path!("/project/.git").as_ref(), true, |state| {
            state.unmerged_paths.insert(
                repo_path("a.txt"),
                UnmergedStatus {
                    first_head: UnmergedStatusCode::Updated,
                    second_head: UnmergedStatusCode::Updated,
                },
            )
        })
        .unwrap();

        cx.run_until_parked();
        conflict_set.update(cx, |conflict_set, cx| {
            let conflict_range = conflict_set.snapshot().conflicts[0]
                .range
                .to_point(buffer.read(cx));
            assert_eq!(conflict_range, Point::new(1, 0)..Point::new(6, 0));
        });
    }
}

mod git_traversal {
    use std::{path::Path, time::Duration};

    use collections::HashMap;
    use project::{
        Project,
        git_store::{RepositoryId, RepositorySnapshot},
    };

    use fs::FakeFs;
    use git::status::{
        FileStatus, GitSummary, StatusCode, TrackedSummary, UnmergedStatus, UnmergedStatusCode,
    };
    use gpui::TestAppContext;
    use project::GitTraversal;

    use serde_json::json;
    use settings::SettingsStore;
    use util::{
        path,
        rel_path::{RelPath, rel_path},
    };

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

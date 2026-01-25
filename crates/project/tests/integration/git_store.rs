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

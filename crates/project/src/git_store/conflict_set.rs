use gpui::{App, Context, Entity, EventEmitter};
use std::{cmp::Ordering, ops::Range, sync::Arc};
use text::{Anchor, BufferId, OffsetRangeExt as _};

pub struct ConflictSet {
    pub has_conflict: bool,
    pub snapshot: ConflictSetSnapshot,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConflictSetUpdate {
    pub buffer_range: Option<Range<Anchor>>,
    pub old_range: Range<usize>,
    pub new_range: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct ConflictSetSnapshot {
    pub buffer_id: BufferId,
    pub conflicts: Arc<[ConflictRegion]>,
}

impl ConflictSetSnapshot {
    pub fn conflicts_in_range(
        &self,
        range: Range<Anchor>,
        buffer: &text::BufferSnapshot,
    ) -> &[ConflictRegion] {
        let start_ix = self
            .conflicts
            .binary_search_by(|conflict| {
                conflict
                    .range
                    .end
                    .cmp(&range.start, buffer)
                    .then(Ordering::Greater)
            })
            .unwrap_err();
        let end_ix = start_ix
            + self.conflicts[start_ix..]
                .binary_search_by(|conflict| {
                    conflict
                        .range
                        .start
                        .cmp(&range.end, buffer)
                        .then(Ordering::Less)
                })
                .unwrap_err();
        &self.conflicts[start_ix..end_ix]
    }

    pub fn compare(&self, other: &Self, buffer: &text::BufferSnapshot) -> ConflictSetUpdate {
        let common_prefix_len = self
            .conflicts
            .iter()
            .zip(other.conflicts.iter())
            .take_while(|(old, new)| old == new)
            .count();
        let common_suffix_len = self.conflicts[common_prefix_len..]
            .iter()
            .rev()
            .zip(other.conflicts[common_prefix_len..].iter().rev())
            .take_while(|(old, new)| old == new)
            .count();
        let old_conflicts =
            &self.conflicts[common_prefix_len..(self.conflicts.len() - common_suffix_len)];
        let new_conflicts =
            &other.conflicts[common_prefix_len..(other.conflicts.len() - common_suffix_len)];
        let old_range = common_prefix_len..(common_prefix_len + old_conflicts.len());
        let new_range = common_prefix_len..(common_prefix_len + new_conflicts.len());
        let start = match (old_conflicts.first(), new_conflicts.first()) {
            (None, None) => None,
            (None, Some(conflict)) => Some(conflict.range.start),
            (Some(conflict), None) => Some(conflict.range.start),
            (Some(first), Some(second)) => Some(first.range.start.min(&second.range.start, buffer)),
        };
        let end = match (old_conflicts.last(), new_conflicts.last()) {
            (None, None) => None,
            (None, Some(conflict)) => Some(conflict.range.end),
            (Some(first), None) => Some(first.range.end),
            (Some(first), Some(second)) => Some(first.range.end.max(&second.range.end, buffer)),
        };
        ConflictSetUpdate {
            buffer_range: start.zip(end).map(|(start, end)| start..end),
            old_range,
            new_range,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictRegion {
    pub range: Range<Anchor>,
    pub ours: Range<Anchor>,
    pub theirs: Range<Anchor>,
    pub base: Option<Range<Anchor>>,
}

impl ConflictRegion {
    pub fn resolve(
        &self,
        buffer: Entity<language::Buffer>,
        ranges: &[Range<Anchor>],
        cx: &mut App,
    ) {
        let buffer_snapshot = buffer.read(cx).snapshot();
        let mut deletions = Vec::new();
        let empty = "";
        let outer_range = self.range.to_offset(&buffer_snapshot);
        let mut offset = outer_range.start;
        for kept_range in ranges {
            let kept_range = kept_range.to_offset(&buffer_snapshot);
            if kept_range.start > offset {
                deletions.push((offset..kept_range.start, empty));
            }
            offset = kept_range.end;
        }
        if outer_range.end > offset {
            deletions.push((offset..outer_range.end, empty));
        }

        buffer.update(cx, |buffer, cx| {
            buffer.edit(deletions, None, cx);
        });
    }
}

impl ConflictSet {
    pub fn new(buffer_id: BufferId, has_conflict: bool, _: &mut Context<Self>) -> Self {
        Self {
            has_conflict,
            snapshot: ConflictSetSnapshot {
                buffer_id,
                conflicts: Default::default(),
            },
        }
    }

    pub fn set_has_conflict(&mut self, has_conflict: bool, cx: &mut Context<Self>) -> bool {
        if has_conflict != self.has_conflict {
            self.has_conflict = has_conflict;
            if !self.has_conflict {
                cx.emit(ConflictSetUpdate {
                    buffer_range: None,
                    old_range: 0..self.snapshot.conflicts.len(),
                    new_range: 0..0,
                });
                self.snapshot.conflicts = Default::default();
            }
            true
        } else {
            false
        }
    }

    pub fn snapshot(&self) -> ConflictSetSnapshot {
        self.snapshot.clone()
    }

    pub fn set_snapshot(
        &mut self,
        snapshot: ConflictSetSnapshot,
        update: ConflictSetUpdate,
        cx: &mut Context<Self>,
    ) {
        self.snapshot = snapshot;
        cx.emit(update);
    }

    pub fn parse(buffer: &text::BufferSnapshot) -> ConflictSetSnapshot {
        let mut conflicts = Vec::new();

        let mut line_pos = 0;
        let buffer_len = buffer.len();
        let mut lines = buffer.text_for_range(0..buffer_len).lines();

        let mut conflict_start: Option<usize> = None;
        let mut ours_start: Option<usize> = None;
        let mut ours_end: Option<usize> = None;
        let mut base_start: Option<usize> = None;
        let mut base_end: Option<usize> = None;
        let mut theirs_start: Option<usize> = None;

        while let Some(line) = lines.next() {
            let line_end = line_pos + line.len();

            if line.starts_with("<<<<<<< ") {
                // If we see a new conflict marker while already parsing one,
                // abandon the previous one and start a new one
                conflict_start = Some(line_pos);
                ours_start = Some(line_end + 1);
            } else if line.starts_with("||||||| ")
                && conflict_start.is_some()
                && ours_start.is_some()
            {
                ours_end = Some(line_pos);
                base_start = Some(line_end + 1);
            } else if line.starts_with("=======")
                && conflict_start.is_some()
                && ours_start.is_some()
            {
                // Set ours_end if not already set (would be set if we have base markers)
                if ours_end.is_none() {
                    ours_end = Some(line_pos);
                } else if base_start.is_some() {
                    base_end = Some(line_pos);
                }
                theirs_start = Some(line_end + 1);
            } else if line.starts_with(">>>>>>> ")
                && conflict_start.is_some()
                && ours_start.is_some()
                && ours_end.is_some()
                && theirs_start.is_some()
            {
                let theirs_end = line_pos;
                let conflict_end = (line_end + 1).min(buffer_len);

                let range = buffer.anchor_after(conflict_start.unwrap())
                    ..buffer.anchor_before(conflict_end);
                let ours = buffer.anchor_after(ours_start.unwrap())
                    ..buffer.anchor_before(ours_end.unwrap());
                let theirs =
                    buffer.anchor_after(theirs_start.unwrap())..buffer.anchor_before(theirs_end);

                let base = base_start
                    .zip(base_end)
                    .map(|(start, end)| buffer.anchor_after(start)..buffer.anchor_before(end));

                conflicts.push(ConflictRegion {
                    range,
                    ours,
                    theirs,
                    base,
                });

                conflict_start = None;
                ours_start = None;
                ours_end = None;
                base_start = None;
                base_end = None;
                theirs_start = None;
            }

            line_pos = line_end + 1;
        }

        ConflictSetSnapshot {
            conflicts: conflicts.into(),
            buffer_id: buffer.remote_id(),
        }
    }
}

impl EventEmitter<ConflictSetUpdate> for ConflictSet {}

#[cfg(test)]
mod tests {
    use std::{path::Path, sync::mpsc};

    use crate::Project;

    use super::*;
    use fs::FakeFs;
    use git::status::{UnmergedStatus, UnmergedStatusCode};
    use gpui::{BackgroundExecutor, TestAppContext};
    use language::language_settings::AllLanguageSettings;
    use serde_json::json;
    use settings::Settings as _;
    use text::{Buffer, BufferId, Point, ToOffset as _};
    use unindent::Unindent as _;
    use util::path;
    use worktree::WorktreeSettings;

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
        let buffer = Buffer::new(0, buffer_id, test_content);
        let snapshot = buffer.snapshot();

        let conflict_snapshot = ConflictSet::parse(&snapshot);
        assert_eq!(conflict_snapshot.conflicts.len(), 2);

        let first = &conflict_snapshot.conflicts[0];
        assert!(first.base.is_none());
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
        let range = buffer.anchor_after(first_conflict_end.to_offset(&buffer) + 1)
            ..buffer.anchor_before(second_conflict_start.to_offset(&buffer) - 1);
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
        let buffer = Buffer::new(0, buffer_id, test_content);
        let snapshot = buffer.snapshot();

        let conflict_snapshot = ConflictSet::parse(&snapshot);

        assert_eq!(conflict_snapshot.conflicts.len(), 1);

        // The conflict should have our version, their version, but no base
        let conflict = &conflict_snapshot.conflicts[0];
        assert!(conflict.base.is_none());

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
        let buffer = Buffer::new(0, buffer_id, test_content);
        let snapshot = buffer.snapshot();

        let conflict_snapshot = ConflictSet::parse(&snapshot);
        assert_eq!(conflict_snapshot.conflicts.len(), 1);
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
        let buffer = Buffer::new(0, buffer_id, test_content.clone());
        let snapshot = buffer.snapshot();

        let conflict_snapshot = ConflictSet::parse(&snapshot);
        assert_eq!(conflict_snapshot.conflicts.len(), 4);

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
            WorktreeSettings::register(cx);
            Project::init_settings(cx);
            AllLanguageSettings::register(cx);
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
                "a.txt".into(),
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
            WorktreeSettings::register(cx);
            Project::init_settings(cx);
            AllLanguageSettings::register(cx);
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
                "a.txt".into(),
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
            state.unmerged_paths.remove(Path::new("a.txt"))
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
                "a.txt".into(),
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

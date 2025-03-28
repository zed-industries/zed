use anyhow::{Context as _, Result};
use buffer_diff::BufferDiff;
use collections::{BTreeMap, HashMap, HashSet};
use gpui::{App, AppContext, AsyncApp, Context, Entity, Subscription, Task, WeakEntity};
use language::{Buffer, BufferEvent, OffsetRangeExt, Operation, TextBufferSnapshot, ToOffset};
use std::{ops::Range, sync::Arc};

/// Tracks actions performed by tools in a thread
pub struct ActionLog {
    /// Buffers that user manually added to the context, and whose content has
    /// changed since the model last saw them.
    stale_buffers_in_context: HashSet<Entity<Buffer>>,
    /// Buffers that we want to notify the model about when they change.
    tracked_buffers: BTreeMap<Entity<Buffer>, TrackedBuffer>,
    /// Has the model edited a file since it last checked diagnostics?
    edited_since_project_diagnostics_check: bool,
}

impl ActionLog {
    /// Creates a new, empty action log.
    pub fn new() -> Self {
        Self {
            stale_buffers_in_context: HashSet::default(),
            tracked_buffers: BTreeMap::default(),
            edited_since_project_diagnostics_check: false,
        }
    }

    pub fn clear_reviewed_changes(&mut self, cx: &mut Context<Self>) {
        self.tracked_buffers
            .retain(|_buffer, tracked_buffer| match &mut tracked_buffer.change {
                Change::Edited {
                    accepted_edit_ids, ..
                } => {
                    accepted_edit_ids.clear();
                    true
                }
                Change::Deleted { reviewed, .. } => !*reviewed,
            });
        cx.notify();
    }

    /// Notifies a diagnostics check
    pub fn checked_project_diagnostics(&mut self) {
        self.edited_since_project_diagnostics_check = false;
    }

    /// Returns true if any files have been edited since the last project diagnostics check
    pub fn has_edited_files_since_project_diagnostics_check(&self) -> bool {
        self.edited_since_project_diagnostics_check
    }

    fn track_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        created: bool,
        cx: &mut Context<Self>,
    ) -> &mut TrackedBuffer {
        let tracked_buffer = self
            .tracked_buffers
            .entry(buffer.clone())
            .or_insert_with(|| {
                let text_snapshot = buffer.read(cx).text_snapshot();
                let unreviewed_diff = cx.new(|cx| BufferDiff::new(&text_snapshot, cx));
                let diff = cx.new(|cx| {
                    let mut diff = BufferDiff::new(&text_snapshot, cx);
                    diff.set_secondary_diff(unreviewed_diff.clone());
                    diff
                });
                let (diff_update_tx, diff_update_rx) = async_watch::channel(());
                TrackedBuffer {
                    buffer: buffer.clone(),
                    change: Change::Edited {
                        unreviewed_edit_ids: HashSet::default(),
                        accepted_edit_ids: HashSet::default(),
                        initial_content: if created {
                            None
                        } else {
                            Some(text_snapshot.clone())
                        },
                    },
                    version: buffer.read(cx).version(),
                    diff,
                    secondary_diff: unreviewed_diff,
                    diff_update: diff_update_tx,
                    _maintain_diff: cx.spawn({
                        let buffer = buffer.clone();
                        async move |this, cx| {
                            Self::maintain_diff(this, buffer, diff_update_rx, cx)
                                .await
                                .ok();
                        }
                    }),
                    _subscription: cx.subscribe(&buffer, Self::handle_buffer_event),
                }
            });
        tracked_buffer.version = buffer.read(cx).version();
        tracked_buffer
    }

    fn handle_buffer_event(
        &mut self,
        buffer: Entity<Buffer>,
        event: &BufferEvent,
        cx: &mut Context<Self>,
    ) {
        let Some(tracked_buffer) = self.tracked_buffers.get_mut(&buffer) else {
            return;
        };
        let BufferEvent::Operation { operation, .. } = event else {
            return;
        };
        let Operation::Buffer(text::Operation::Edit(operation)) = operation else {
            return;
        };

        if let Change::Edited {
            unreviewed_edit_ids,
            accepted_edit_ids,
            ..
        } = &mut tracked_buffer.change
        {
            if unreviewed_edit_ids.contains(&operation.timestamp)
                || accepted_edit_ids.contains(&operation.timestamp)
            {
                return;
            }

            let buffer = buffer.read(cx);
            let operation_edit_ranges = buffer
                .edited_ranges_for_edit_ids::<usize>([&operation.timestamp])
                .peekable();
            let tracked_edit_ranges = buffer
                .edited_ranges_for_edit_ids::<usize>(
                    unreviewed_edit_ids.iter().chain(accepted_edit_ids.iter()),
                )
                .peekable();
            let mut operation_edit_ranges_iter = operation_edit_ranges.peekable();
            let mut tracked_edit_ranges_iter = tracked_edit_ranges.peekable();

            let mut overlaps = false;
            while let (Some(op_range), Some(tracked_range)) = (
                operation_edit_ranges_iter.peek(),
                tracked_edit_ranges_iter.peek(),
            ) {
                if op_range.end < tracked_range.start {
                    operation_edit_ranges_iter.next();
                } else if tracked_range.end < op_range.start {
                    tracked_edit_ranges_iter.next();
                } else {
                    overlaps = true;
                    break;
                }
            }
            drop(operation_edit_ranges_iter);
            drop(tracked_edit_ranges_iter);

            if overlaps {
                unreviewed_edit_ids.insert(operation.timestamp);
                tracked_buffer.schedule_diff_update();
            }
        }
    }

    async fn maintain_diff(
        this: WeakEntity<Self>,
        buffer: Entity<Buffer>,
        mut diff_update: async_watch::Receiver<()>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        while let Some(_) = diff_update.recv().await.ok() {
            let update = this.update(cx, |this, cx| {
                let tracked_buffer = this
                    .tracked_buffers
                    .get_mut(&buffer)
                    .context("buffer not tracked")?;
                anyhow::Ok(tracked_buffer.update_diff(cx))
            })??;
            update.await;
            this.update(cx, |_this, cx| cx.notify())?;
        }

        Ok(())
    }

    /// Track a buffer as read, so we can notify the model about user edits.
    pub fn buffer_read(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.track_buffer(buffer, false, cx);
    }

    /// Track a buffer as read, so we can notify the model about user edits.
    pub fn buffer_created(
        &mut self,
        buffer: Entity<Buffer>,
        edit_id: Option<clock::Lamport>,
        cx: &mut Context<Self>,
    ) {
        self.track_buffer(buffer.clone(), true, cx);
        self.buffer_edited(buffer, edit_id.into_iter().collect(), cx)
    }

    /// Mark a buffer as edited, so we can refresh it in the context
    pub fn buffer_edited(
        &mut self,
        buffer: Entity<Buffer>,
        mut edit_ids: Vec<clock::Lamport>,
        cx: &mut Context<Self>,
    ) {
        self.edited_since_project_diagnostics_check = true;
        self.stale_buffers_in_context.insert(buffer.clone());

        let tracked_buffer = self.track_buffer(buffer.clone(), false, cx);

        match &mut tracked_buffer.change {
            Change::Edited {
                unreviewed_edit_ids,
                ..
            } => {
                unreviewed_edit_ids.extend(edit_ids.iter().copied());
            }
            Change::Deleted {
                deleted_content,
                deletion_id,
                ..
            } => {
                edit_ids.extend(*deletion_id);
                tracked_buffer.change = Change::Edited {
                    unreviewed_edit_ids: edit_ids.into_iter().collect(),
                    accepted_edit_ids: HashSet::default(),
                    initial_content: Some(deleted_content.clone()),
                };
            }
        }

        tracked_buffer.schedule_diff_update();
    }

    pub fn buffer_deleted(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        let tracked_buffer = self.track_buffer(buffer.clone(), false, cx);
        if let Change::Edited {
            initial_content, ..
        } = &tracked_buffer.change
        {
            if let Some(initial_content) = initial_content {
                let deletion_id = buffer.update(cx, |buffer, cx| buffer.set_text("", cx));
                tracked_buffer.change = Change::Deleted {
                    reviewed: false,
                    deleted_content: initial_content.clone(),
                    deletion_id,
                };
                tracked_buffer.schedule_diff_update();
            } else {
                self.tracked_buffers.remove(&buffer);
                cx.notify();
            }
        }
    }

    /// Accepts edits in a given range within a buffer.
    pub fn review_edits_in_range<T: ToOffset>(
        &mut self,
        buffer: Entity<Buffer>,
        buffer_range: Range<T>,
        accept: bool,
        cx: &mut Context<Self>,
    ) {
        let Some(tracked_buffer) = self.tracked_buffers.get_mut(&buffer) else {
            return;
        };

        let buffer = buffer.read(cx);
        let buffer_range = buffer_range.to_offset(buffer);

        match &mut tracked_buffer.change {
            Change::Deleted { reviewed, .. } => {
                *reviewed = accept;
            }
            Change::Edited {
                unreviewed_edit_ids,
                accepted_edit_ids,
                ..
            } => {
                let (source, destination) = if accept {
                    (unreviewed_edit_ids, accepted_edit_ids)
                } else {
                    (accepted_edit_ids, unreviewed_edit_ids)
                };
                source.retain(|edit_id| {
                    for range in buffer.edited_ranges_for_edit_ids::<usize>([edit_id]) {
                        if buffer_range.end >= range.start && buffer_range.start <= range.end {
                            destination.insert(*edit_id);
                            return false;
                        }
                    }
                    true
                });
            }
        }

        tracked_buffer.schedule_diff_update();
    }

    /// Returns the set of buffers that contain changes that haven't been reviewed by the user.
    pub fn changed_buffers(&self) -> BTreeMap<Entity<Buffer>, ChangedBuffer> {
        self.tracked_buffers
            .iter()
            .filter(|(_, tracked)| tracked.has_changes())
            .map(|(buffer, tracked)| {
                (
                    buffer.clone(),
                    ChangedBuffer {
                        diff: tracked.diff.clone(),
                        needs_review: match &tracked.change {
                            Change::Edited {
                                unreviewed_edit_ids,
                                ..
                            } => !unreviewed_edit_ids.is_empty(),
                            Change::Deleted { reviewed, .. } => !reviewed,
                        },
                    },
                )
            })
            .collect()
    }

    /// Iterate over buffers changed since last read or edited by the model
    pub fn stale_buffers<'a>(&'a self, cx: &'a App) -> impl Iterator<Item = &'a Entity<Buffer>> {
        self.tracked_buffers
            .iter()
            .filter(|(buffer, tracked)| tracked.version != buffer.read(cx).version)
            .map(|(buffer, _)| buffer)
    }

    /// Takes and returns the set of buffers pending refresh, clearing internal state.
    pub fn take_stale_buffers_in_context(&mut self) -> HashSet<Entity<Buffer>> {
        std::mem::take(&mut self.stale_buffers_in_context)
    }
}

struct TrackedBuffer {
    buffer: Entity<Buffer>,
    change: Change,
    version: clock::Global,
    diff: Entity<BufferDiff>,
    secondary_diff: Entity<BufferDiff>,
    diff_update: async_watch::Sender<()>,
    _maintain_diff: Task<()>,
    _subscription: Subscription,
}

enum Change {
    Edited {
        unreviewed_edit_ids: HashSet<clock::Lamport>,
        accepted_edit_ids: HashSet<clock::Lamport>,
        initial_content: Option<TextBufferSnapshot>,
    },
    Deleted {
        reviewed: bool,
        deleted_content: TextBufferSnapshot,
        deletion_id: Option<clock::Lamport>,
    },
}

impl TrackedBuffer {
    fn has_changes(&self) -> bool {
        match &self.change {
            Change::Edited {
                unreviewed_edit_ids,
                accepted_edit_ids,
                ..
            } => !unreviewed_edit_ids.is_empty() || !accepted_edit_ids.is_empty(),
            Change::Deleted { .. } => true,
        }
    }

    fn schedule_diff_update(&self) {
        self.diff_update.send(()).ok();
    }

    fn update_diff(&mut self, cx: &mut App) -> Task<()> {
        match &self.change {
            Change::Edited {
                unreviewed_edit_ids,
                accepted_edit_ids,
                ..
            } => {
                let edits_to_undo = unreviewed_edit_ids
                    .iter()
                    .chain(accepted_edit_ids)
                    .map(|edit_id| (*edit_id, u32::MAX))
                    .collect::<HashMap<_, _>>();
                let buffer_without_edits = self.buffer.update(cx, |buffer, cx| buffer.branch(cx));
                buffer_without_edits
                    .update(cx, |buffer, cx| buffer.undo_operations(edits_to_undo, cx));
                let primary_diff_update = self.diff.update(cx, |diff, cx| {
                    diff.set_base_text(
                        buffer_without_edits,
                        self.buffer.read(cx).text_snapshot(),
                        cx,
                    )
                });

                let unreviewed_edits_to_undo = unreviewed_edit_ids
                    .iter()
                    .map(|edit_id| (*edit_id, u32::MAX))
                    .collect::<HashMap<_, _>>();
                let buffer_without_unreviewed_edits =
                    self.buffer.update(cx, |buffer, cx| buffer.branch(cx));
                buffer_without_unreviewed_edits.update(cx, |buffer, cx| {
                    buffer.undo_operations(unreviewed_edits_to_undo, cx)
                });
                let secondary_diff_update = self.secondary_diff.update(cx, |diff, cx| {
                    diff.set_base_text(
                        buffer_without_unreviewed_edits.clone(),
                        self.buffer.read(cx).text_snapshot(),
                        cx,
                    )
                });

                cx.background_spawn(async move {
                    _ = primary_diff_update.await;
                    _ = secondary_diff_update.await;
                })
            }
            Change::Deleted {
                reviewed,
                deleted_content,
                ..
            } => {
                let reviewed = *reviewed;
                let deleted_content = deleted_content.clone();

                let primary_diff = self.diff.clone();
                let secondary_diff = self.secondary_diff.clone();
                let buffer_snapshot = self.buffer.read(cx).text_snapshot();
                let language = self.buffer.read(cx).language().cloned();
                let language_registry = self.buffer.read(cx).language_registry().clone();

                cx.spawn(async move |cx| {
                    let base_text = Arc::new(deleted_content.text());

                    let primary_diff_snapshot = BufferDiff::update_diff(
                        primary_diff.clone(),
                        buffer_snapshot.clone(),
                        Some(base_text.clone()),
                        true,
                        false,
                        language.clone(),
                        language_registry.clone(),
                        cx,
                    )
                    .await;
                    let secondary_diff_snapshot = BufferDiff::update_diff(
                        secondary_diff.clone(),
                        buffer_snapshot.clone(),
                        if reviewed {
                            None
                        } else {
                            Some(base_text.clone())
                        },
                        true,
                        false,
                        language.clone(),
                        language_registry.clone(),
                        cx,
                    )
                    .await;

                    if let Ok(primary_diff_snapshot) = primary_diff_snapshot {
                        primary_diff
                            .update(cx, |diff, cx| {
                                diff.set_snapshot(
                                    &buffer_snapshot,
                                    primary_diff_snapshot,
                                    false,
                                    None,
                                    cx,
                                )
                            })
                            .ok();
                    }

                    if let Ok(secondary_diff_snapshot) = secondary_diff_snapshot {
                        secondary_diff
                            .update(cx, |diff, cx| {
                                diff.set_snapshot(
                                    &buffer_snapshot,
                                    secondary_diff_snapshot,
                                    false,
                                    None,
                                    cx,
                                )
                            })
                            .ok();
                    }
                })
            }
        }
    }
}

pub struct ChangedBuffer {
    pub diff: Entity<BufferDiff>,
    pub needs_review: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use buffer_diff::DiffHunkStatusKind;
    use gpui::TestAppContext;
    use language::Point;

    #[gpui::test]
    async fn test_edit_review(cx: &mut TestAppContext) {
        let action_log = cx.new(|_| ActionLog::new());
        let buffer = cx.new(|cx| Buffer::local("abc\ndef\nghi\njkl\nmno", cx));

        let edit1 = buffer.update(cx, |buffer, cx| {
            buffer
                .edit([(Point::new(1, 1)..Point::new(1, 2), "E")], None, cx)
                .unwrap()
        });
        let edit2 = buffer.update(cx, |buffer, cx| {
            buffer
                .edit([(Point::new(4, 2)..Point::new(4, 3), "O")], None, cx)
                .unwrap()
        });
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abc\ndEf\nghi\njkl\nmnO"
        );

        action_log.update(cx, |log, cx| {
            log.buffer_edited(buffer.clone(), vec![edit1, edit2], cx)
        });
        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(2, 0),
                        review_status: ReviewStatus::Unreviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "def\n".into(),
                    },
                    HunkStatus {
                        range: Point::new(4, 0)..Point::new(4, 3),
                        review_status: ReviewStatus::Unreviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "mno".into(),
                    }
                ],
            )]
        );

        action_log.update(cx, |log, cx| {
            log.review_edits_in_range(buffer.clone(), Point::new(3, 0)..Point::new(4, 3), true, cx)
        });
        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(2, 0),
                        review_status: ReviewStatus::Unreviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "def\n".into(),
                    },
                    HunkStatus {
                        range: Point::new(4, 0)..Point::new(4, 3),
                        review_status: ReviewStatus::Reviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "mno".into(),
                    }
                ],
            )]
        );

        action_log.update(cx, |log, cx| {
            log.review_edits_in_range(
                buffer.clone(),
                Point::new(3, 0)..Point::new(4, 3),
                false,
                cx,
            )
        });
        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(2, 0),
                        review_status: ReviewStatus::Unreviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "def\n".into(),
                    },
                    HunkStatus {
                        range: Point::new(4, 0)..Point::new(4, 3),
                        review_status: ReviewStatus::Unreviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "mno".into(),
                    }
                ],
            )]
        );

        action_log.update(cx, |log, cx| {
            log.review_edits_in_range(buffer.clone(), Point::new(0, 0)..Point::new(4, 3), true, cx)
        });
        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(2, 0),
                        review_status: ReviewStatus::Reviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "def\n".into(),
                    },
                    HunkStatus {
                        range: Point::new(4, 0)..Point::new(4, 3),
                        review_status: ReviewStatus::Reviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "mno".into(),
                    }
                ],
            )]
        );
    }

    #[gpui::test]
    async fn test_overlapping_user_edits(cx: &mut TestAppContext) {
        let action_log = cx.new(|_| ActionLog::new());
        let buffer = cx.new(|cx| Buffer::local("abc\ndef\nghi\njkl\nmno", cx));

        let tool_edit = buffer.update(cx, |buffer, cx| {
            buffer
                .edit(
                    [(Point::new(0, 2)..Point::new(2, 3), "C\nDEF\nGHI")],
                    None,
                    cx,
                )
                .unwrap()
        });
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abC\nDEF\nGHI\njkl\nmno"
        );

        action_log.update(cx, |log, cx| {
            log.buffer_edited(buffer.clone(), vec![tool_edit], cx)
        });
        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(0, 0)..Point::new(3, 0),
                    review_status: ReviewStatus::Unreviewed,
                    diff_status: DiffHunkStatusKind::Modified,
                    old_text: "abc\ndef\nghi\n".into(),
                }],
            )]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(0, 2)..Point::new(0, 2), "X")], None, cx)
        });
        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(0, 0)..Point::new(3, 0),
                    review_status: ReviewStatus::Unreviewed,
                    diff_status: DiffHunkStatusKind::Modified,
                    old_text: "abc\ndef\nghi\n".into(),
                }],
            )]
        );
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct HunkStatus {
        range: Range<Point>,
        review_status: ReviewStatus,
        diff_status: DiffHunkStatusKind,
        old_text: String,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum ReviewStatus {
        Unreviewed,
        Reviewed,
    }

    fn unreviewed_hunks(
        action_log: &Entity<ActionLog>,
        cx: &TestAppContext,
    ) -> Vec<(Entity<Buffer>, Vec<HunkStatus>)> {
        cx.read(|cx| {
            action_log
                .read(cx)
                .changed_buffers()
                .into_iter()
                .map(|(buffer, tracked_buffer)| {
                    let snapshot = buffer.read(cx).snapshot();
                    (
                        buffer,
                        tracked_buffer
                            .diff
                            .read(cx)
                            .hunks(&snapshot, cx)
                            .map(|hunk| HunkStatus {
                                review_status: if hunk.status().has_secondary_hunk() {
                                    ReviewStatus::Unreviewed
                                } else {
                                    ReviewStatus::Reviewed
                                },
                                diff_status: hunk.status().kind,
                                range: hunk.range,
                                old_text: tracked_buffer
                                    .diff
                                    .read(cx)
                                    .base_text()
                                    .text_for_range(hunk.diff_base_byte_range)
                                    .collect(),
                            })
                            .collect(),
                    )
                })
                .collect()
        })
    }
}

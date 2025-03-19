use anyhow::{anyhow, Result};
use buffer_diff::BufferDiff;
use collections::{BTreeMap, HashMap, HashSet};
use gpui::{App, AppContext, Context, Entity, Task};
use language::{Buffer, OffsetRangeExt, ToOffset};
use std::{future::Future, ops::Range};

/// Tracks actions performed by tools in a thread
#[derive(Debug)]
pub struct ActionLog {
    /// Buffers that user manually added to the context, and whose content has
    /// changed since the model last saw them.
    stale_buffers_in_context: HashSet<Entity<Buffer>>,
    /// Buffers that we want to notify the model about when they change.
    tracked_buffers: BTreeMap<Entity<Buffer>, TrackedBuffer>,
}

#[derive(Debug, Clone)]
pub struct TrackedBuffer {
    buffer: Entity<Buffer>,
    unreviewed_edit_ids: Vec<clock::Lamport>,
    accepted_edit_ids: Vec<clock::Lamport>,
    version: clock::Global,
    diff: Entity<BufferDiff>,
    secondary_diff: Entity<BufferDiff>,
}

impl TrackedBuffer {
    pub fn needs_review(&self) -> bool {
        !self.unreviewed_edit_ids.is_empty()
    }

    pub fn diff(&self) -> &Entity<BufferDiff> {
        &self.diff
    }

    fn update_diff(&mut self, cx: &mut App) -> impl 'static + Future<Output = ()> {
        let edits_to_undo = self
            .unreviewed_edit_ids
            .iter()
            .chain(&self.accepted_edit_ids)
            .map(|edit_id| (*edit_id, u32::MAX))
            .collect::<HashMap<_, _>>();
        let buffer_without_edits = self.buffer.update(cx, |buffer, cx| buffer.branch(cx));
        buffer_without_edits.update(cx, |buffer, cx| {
            buffer.undo_operations(edits_to_undo, cx);
        });
        let primary_diff_update = self.diff.update(cx, |diff, cx| {
            diff.set_base_text(
                buffer_without_edits,
                self.buffer.read(cx).text_snapshot(),
                cx,
            )
        });

        let unreviewed_edits_to_undo = self
            .unreviewed_edit_ids
            .iter()
            .map(|edit_id| (*edit_id, u32::MAX))
            .collect::<HashMap<_, _>>();
        let buffer_without_unreviewed_edits =
            self.buffer.update(cx, |buffer, cx| buffer.branch(cx));
        buffer_without_unreviewed_edits.update(cx, |buffer, cx| {
            buffer.undo_operations(unreviewed_edits_to_undo, cx);
        });
        let secondary_diff_update = self.secondary_diff.update(cx, |diff, cx| {
            diff.set_base_text(
                buffer_without_unreviewed_edits.clone(),
                self.buffer.read(cx).text_snapshot(),
                cx,
            )
        });

        async move {
            _ = primary_diff_update.await;
            _ = secondary_diff_update.await;
        }
    }
}

impl ActionLog {
    /// Creates a new, empty action log.
    pub fn new() -> Self {
        Self {
            stale_buffers_in_context: HashSet::default(),
            tracked_buffers: BTreeMap::default(),
        }
    }

    fn track_buffer(
        &mut self,
        buffer: Entity<Buffer>,
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
                TrackedBuffer {
                    buffer: buffer.clone(),
                    unreviewed_edit_ids: Vec::new(),
                    accepted_edit_ids: Vec::new(),
                    version: buffer.read(cx).version(),
                    diff,
                    secondary_diff: unreviewed_diff,
                }
            });
        tracked_buffer.version = buffer.read(cx).version();
        tracked_buffer
    }

    /// Track a buffer as read, so we can notify the model about user edits.
    pub fn buffer_read(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.track_buffer(buffer, cx);
    }

    /// Mark a buffer as edited, so we can refresh it in the context
    pub fn buffer_edited(
        &mut self,
        buffer: Entity<Buffer>,
        edit_ids: Vec<clock::Lamport>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.stale_buffers_in_context.insert(buffer.clone());

        let tracked_buffer = self.track_buffer(buffer.clone(), cx);
        tracked_buffer
            .unreviewed_edit_ids
            .extend(edit_ids.iter().copied());
        let update = tracked_buffer.update_diff(cx);
        cx.spawn(async move |this, cx| {
            update.await;
            this.update(cx, |_this, cx| cx.notify())?;
            Ok(())
        })
    }

    /// Accepts edits in a given range within a buffer.
    pub fn review_edits_in_range<T: ToOffset>(
        &mut self,
        buffer: Entity<Buffer>,
        buffer_range: Range<T>,
        accept: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(tracked_buffer) = self.tracked_buffers.get_mut(&buffer) else {
            return Task::ready(Err(anyhow!("buffer not found")));
        };

        let buffer = buffer.read(cx);
        let buffer_range = buffer_range.to_offset(buffer);

        let source;
        let destination;
        if accept {
            source = &mut tracked_buffer.unreviewed_edit_ids;
            destination = &mut tracked_buffer.accepted_edit_ids;
        } else {
            source = &mut tracked_buffer.accepted_edit_ids;
            destination = &mut tracked_buffer.unreviewed_edit_ids;
        }

        source.retain(|edit_id| {
            for range in buffer.edited_ranges_for_edit_ids::<usize>([edit_id]) {
                if buffer_range.end >= range.start && buffer_range.start <= range.end {
                    destination.push(*edit_id);
                    return false;
                }
            }
            true
        });

        let update = tracked_buffer.update_diff(cx);
        cx.spawn(async move |this, cx| {
            update.await;
            this.update(cx, |_this, cx| cx.notify())?;
            Ok(())
        })
    }

    /// Returns the set of buffers that contain changes that haven't been reviewed by the user.
    pub fn unreviewed_buffers(&self) -> BTreeMap<Entity<Buffer>, TrackedBuffer> {
        self.tracked_buffers
            .iter()
            .map(|(buffer, tracked)| (buffer.clone(), tracked.clone()))
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

        action_log
            .update(cx, |log, cx| {
                log.buffer_edited(buffer.clone(), vec![edit1, edit2], cx)
            })
            .await
            .unwrap();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(2, 0),
                        review_status: ReviewStatus::Unreviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                    },
                    HunkStatus {
                        range: Point::new(4, 0)..Point::new(4, 3),
                        review_status: ReviewStatus::Unreviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                    }
                ],
            )]
        );

        action_log
            .update(cx, |log, cx| {
                log.review_edits_in_range(
                    buffer.clone(),
                    Point::new(3, 0)..Point::new(4, 3),
                    true,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(2, 0),
                        review_status: ReviewStatus::Unreviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                    },
                    HunkStatus {
                        range: Point::new(4, 0)..Point::new(4, 3),
                        review_status: ReviewStatus::Reviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                    }
                ],
            )]
        );

        action_log
            .update(cx, |log, cx| {
                log.review_edits_in_range(
                    buffer.clone(),
                    Point::new(3, 0)..Point::new(4, 3),
                    false,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(2, 0),
                        review_status: ReviewStatus::Unreviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                    },
                    HunkStatus {
                        range: Point::new(4, 0)..Point::new(4, 3),
                        review_status: ReviewStatus::Unreviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                    }
                ],
            )]
        );

        action_log
            .update(cx, |log, cx| {
                log.review_edits_in_range(
                    buffer.clone(),
                    Point::new(0, 0)..Point::new(4, 3),
                    true,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(2, 0),
                        review_status: ReviewStatus::Reviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                    },
                    HunkStatus {
                        range: Point::new(4, 0)..Point::new(4, 3),
                        review_status: ReviewStatus::Reviewed,
                        diff_status: DiffHunkStatusKind::Modified,
                    }
                ],
            )]
        );
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct HunkStatus {
        range: Range<Point>,
        review_status: ReviewStatus,
        diff_status: DiffHunkStatusKind,
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
                .unreviewed_buffers()
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
                            })
                            .collect(),
                    )
                })
                .collect()
        })
    }
}

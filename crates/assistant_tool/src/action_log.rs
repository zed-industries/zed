use anyhow::{Context as _, Result};
use buffer_diff::BufferDiff;
use collections::{BTreeMap, HashSet};
use futures::{StreamExt, channel::mpsc};
use gpui::{App, AppContext, AsyncApp, Context, Entity, Subscription, Task, WeakEntity};
use language::{Buffer, BufferEvent, DiskState, Point};
use std::{cmp, ops::Range, sync::Arc};
use text::{Edit, Patch, Rope};
use util::RangeExt;

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
                let diff = cx.new(|cx| BufferDiff::new(&text_snapshot, cx));
                let (diff_update_tx, diff_update_rx) = mpsc::unbounded();
                let base_text;
                let status;
                let unreviewed_changes;
                if created {
                    base_text = Rope::default();
                    status = TrackedBufferStatus::Created;
                    unreviewed_changes = Patch::new(vec![Edit {
                        old: 0..1,
                        new: 0..text_snapshot.max_point().row + 1,
                    }])
                } else {
                    base_text = buffer.read(cx).as_rope().clone();
                    status = TrackedBufferStatus::Modified;
                    unreviewed_changes = Patch::default();
                }
                TrackedBuffer {
                    buffer: buffer.clone(),
                    base_text,
                    unreviewed_changes,
                    snapshot: text_snapshot.clone(),
                    status,
                    version: buffer.read(cx).version(),
                    diff,
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
        match event {
            BufferEvent::Edited { .. } => self.handle_buffer_edited(buffer, cx),
            BufferEvent::FileHandleChanged => {
                self.handle_buffer_file_changed(buffer, cx);
            }
            _ => {}
        };
    }

    fn handle_buffer_edited(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        let Some(tracked_buffer) = self.tracked_buffers.get_mut(&buffer) else {
            return;
        };
        tracked_buffer.schedule_diff_update(ChangeAuthor::User, cx);
    }

    fn handle_buffer_file_changed(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        let Some(tracked_buffer) = self.tracked_buffers.get_mut(&buffer) else {
            return;
        };

        match tracked_buffer.status {
            TrackedBufferStatus::Created | TrackedBufferStatus::Modified => {
                if buffer
                    .read(cx)
                    .file()
                    .map_or(false, |file| file.disk_state() == DiskState::Deleted)
                {
                    // If the buffer had been edited by a tool, but it got
                    // deleted externally, we want to stop tracking it.
                    self.tracked_buffers.remove(&buffer);
                }
                cx.notify();
            }
            TrackedBufferStatus::Deleted => {
                if buffer
                    .read(cx)
                    .file()
                    .map_or(false, |file| file.disk_state() != DiskState::Deleted)
                {
                    // If the buffer had been deleted by a tool, but it got
                    // resurrected externally, we want to clear the changes we
                    // were tracking and reset the buffer's state.
                    self.tracked_buffers.remove(&buffer);
                    self.track_buffer(buffer, false, cx);
                }
                cx.notify();
            }
        }
    }

    async fn maintain_diff(
        this: WeakEntity<Self>,
        buffer: Entity<Buffer>,
        mut diff_update: mpsc::UnboundedReceiver<(ChangeAuthor, text::BufferSnapshot)>,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        while let Some((author, buffer_snapshot)) = diff_update.next().await {
            let (rebase, diff, language, language_registry) =
                this.read_with(cx, |this, cx| {
                    let tracked_buffer = this
                        .tracked_buffers
                        .get(&buffer)
                        .context("buffer not tracked")?;

                    let rebase = cx.background_spawn({
                        let mut base_text = tracked_buffer.base_text.clone();
                        let old_snapshot = tracked_buffer.snapshot.clone();
                        let new_snapshot = buffer_snapshot.clone();
                        let unreviewed_changes = tracked_buffer.unreviewed_changes.clone();
                        async move {
                            let edits = diff_snapshots(&old_snapshot, &new_snapshot);
                            let unreviewed_changes = match author {
                                ChangeAuthor::User => rebase_patch(
                                    &unreviewed_changes,
                                    edits,
                                    &mut base_text,
                                    new_snapshot.as_rope(),
                                ),
                                ChangeAuthor::Agent => unreviewed_changes.compose(edits),
                            };
                            (
                                Arc::new(base_text.to_string()),
                                base_text,
                                unreviewed_changes,
                            )
                        }
                    });

                    anyhow::Ok((
                        rebase,
                        tracked_buffer.diff.clone(),
                        tracked_buffer.buffer.read(cx).language().cloned(),
                        tracked_buffer.buffer.read(cx).language_registry(),
                    ))
                })??;

            let (new_base_text, new_base_text_rope, unreviewed_changes) = rebase.await;
            let diff_snapshot = BufferDiff::update_diff(
                diff.clone(),
                buffer_snapshot.clone(),
                Some(new_base_text),
                true,
                false,
                language,
                language_registry,
                cx,
            )
            .await;
            if let Ok(diff_snapshot) = diff_snapshot {
                diff.update(cx, |diff, cx| {
                    diff.set_snapshot(diff_snapshot, &buffer_snapshot, None, cx)
                })?;
            }
            this.update(cx, |this, cx| {
                let tracked_buffer = this
                    .tracked_buffers
                    .get_mut(&buffer)
                    .context("buffer not tracked")?;
                tracked_buffer.base_text = new_base_text_rope;
                tracked_buffer.snapshot = buffer_snapshot;
                tracked_buffer.unreviewed_changes = unreviewed_changes;
                cx.notify();
                anyhow::Ok(())
            })??;
        }

        Ok(())
    }

    /// Track a buffer as read, so we can notify the model about user edits.
    pub fn buffer_read(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.track_buffer(buffer, false, cx);
    }

    /// Track a buffer as read, so we can notify the model about user edits.
    pub fn will_create_buffer(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.track_buffer(buffer.clone(), true, cx);
        self.buffer_edited(buffer, cx)
    }

    /// Mark a buffer as edited, so we can refresh it in the context
    pub fn buffer_edited(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.edited_since_project_diagnostics_check = true;
        self.stale_buffers_in_context.insert(buffer.clone());

        let tracked_buffer = self.track_buffer(buffer.clone(), false, cx);
        if let TrackedBufferStatus::Deleted = tracked_buffer.status {
            tracked_buffer.status = TrackedBufferStatus::Modified;
        }
        tracked_buffer.schedule_diff_update(ChangeAuthor::Agent, cx);
    }

    pub fn will_delete_buffer(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        let tracked_buffer = self.track_buffer(buffer.clone(), false, cx);
        match tracked_buffer.status {
            TrackedBufferStatus::Created => {
                self.tracked_buffers.remove(&buffer);
                cx.notify();
            }
            TrackedBufferStatus::Modified => {
                buffer.update(cx, |buffer, cx| buffer.set_text("", cx));
                tracked_buffer.status = TrackedBufferStatus::Deleted;
                tracked_buffer.schedule_diff_update(ChangeAuthor::Agent, cx);
            }
            TrackedBufferStatus::Deleted => {}
        }
        cx.notify();
    }

    pub fn keep_edits_in_range<T>(
        &mut self,
        buffer: Entity<Buffer>,
        buffer_range: Range<T>,
        cx: &mut Context<Self>,
    ) where
        T: 'static + language::ToPoint, // + Clone
                                        // + Copy
                                        // + Ord
                                        // + Sub<T, Output = T>
                                        // + Add<T, Output = T>
                                        // + AddAssign
                                        // + Default
                                        // + PartialEq,
    {
        let Some(tracked_buffer) = self.tracked_buffers.get_mut(&buffer) else {
            return;
        };

        match tracked_buffer.status {
            TrackedBufferStatus::Deleted => {
                self.tracked_buffers.remove(&buffer);
                cx.notify();
            }
            _ => {
                let buffer = buffer.read(cx);
                let buffer_range =
                    buffer_range.start.to_point(buffer)..buffer_range.end.to_point(buffer);
                let buffer_row_range = buffer_range.start.row..buffer_range.end.row + 1;
                let mut delta = 0i32;
                tracked_buffer.unreviewed_changes.retain_mut(|edit| {
                    edit.old.start = (edit.old.start as i32 + delta) as u32;
                    edit.old.end = (edit.old.end as i32 + delta) as u32;
                    if edit.new.overlaps(&buffer_row_range) {
                        let old_bytes = tracked_buffer
                            .base_text
                            .point_to_offset(Point::new(edit.old.start, 0))
                            ..tracked_buffer.base_text.point_to_offset(cmp::min(
                                Point::new(edit.old.end, 0),
                                tracked_buffer.base_text.max_point(),
                            ));
                        let new_bytes = tracked_buffer
                            .snapshot
                            .point_to_offset(Point::new(edit.new.start, 0))
                            ..tracked_buffer.snapshot.point_to_offset(cmp::min(
                                Point::new(edit.new.end, 0),
                                tracked_buffer.snapshot.max_point(),
                            ));
                        tracked_buffer.base_text.replace(
                            old_bytes,
                            &tracked_buffer
                                .snapshot
                                .text_for_range(new_bytes)
                                .collect::<String>(),
                        );
                        delta += edit.new_len() as i32 - edit.old_len() as i32;
                        false
                    } else {
                        true
                    }
                });
                tracked_buffer.schedule_diff_update(ChangeAuthor::User, cx);
            }
        }
    }

    pub fn keep_all_edits(&mut self, cx: &mut Context<Self>) {
        self.tracked_buffers
            .retain(|_buffer, tracked_buffer| match tracked_buffer.status {
                TrackedBufferStatus::Deleted => false,
                _ => {
                    tracked_buffer.unreviewed_changes.clear();
                    tracked_buffer.base_text = tracked_buffer.snapshot.as_rope().clone();
                    tracked_buffer.schedule_diff_update(ChangeAuthor::User, cx);
                    true
                }
            });
        cx.notify();
    }

    /// Returns the set of buffers that contain changes that haven't been reviewed by the user.
    pub fn changed_buffers(&self, cx: &App) -> BTreeMap<Entity<Buffer>, Entity<BufferDiff>> {
        self.tracked_buffers
            .iter()
            .filter(|(_, tracked)| tracked.has_changes(cx))
            .map(|(buffer, tracked)| (buffer.clone(), tracked.diff.clone()))
            .collect()
    }

    /// Iterate over buffers changed since last read or edited by the model
    pub fn stale_buffers<'a>(&'a self, cx: &'a App) -> impl Iterator<Item = &'a Entity<Buffer>> {
        self.tracked_buffers
            .iter()
            .filter(|(buffer, tracked)| {
                let buffer = buffer.read(cx);

                tracked.version != buffer.version
                    && buffer
                        .file()
                        .map_or(false, |file| file.disk_state() != DiskState::Deleted)
            })
            .map(|(buffer, _)| buffer)
    }

    /// Takes and returns the set of buffers pending refresh, clearing internal state.
    pub fn take_stale_buffers_in_context(&mut self) -> HashSet<Entity<Buffer>> {
        std::mem::take(&mut self.stale_buffers_in_context)
    }
}

fn rebase_patch(
    patch: &Patch<u32>,
    edits: Vec<Edit<u32>>,
    old_text: &mut Rope,
    new_text: &Rope,
) -> Patch<u32> {
    let mut translated_unreviewed_edits = Patch::default();
    let mut conflicting_edits = Vec::new();

    let mut old_edits = patch.edits().iter().cloned().peekable();
    let mut new_edits = edits.into_iter().peekable();
    let mut applied_delta = 0i32;
    let mut rebased_delta = 0i32;

    while let Some(mut new_edit) = new_edits.next() {
        let mut conflict = false;

        // Push all the old edits that are before this new edit or that intersect with it.
        while let Some(old_edit) = old_edits.peek() {
            if new_edit.old.end <= old_edit.new.start {
                break;
            } else if new_edit.old.start >= old_edit.new.end {
                let mut old_edit = old_edits.next().unwrap();
                old_edit.old.start = (old_edit.old.start as i32 + applied_delta) as u32;
                old_edit.old.end = (old_edit.old.end as i32 + applied_delta) as u32;
                old_edit.new.start = (old_edit.new.start as i32 + applied_delta) as u32;
                old_edit.new.end = (old_edit.new.end as i32 + applied_delta) as u32;
                rebased_delta += old_edit.new_len() as i32 - old_edit.old_len() as i32;
                translated_unreviewed_edits.push(old_edit);
            } else {
                conflict = true;
                if new_edits
                    .peek()
                    .map_or(false, |next_edit| next_edit.old.overlaps(&old_edit.new))
                {
                    new_edit.old.start = (new_edit.old.start as i32 + applied_delta) as u32;
                    new_edit.old.end = (new_edit.old.end as i32 + applied_delta) as u32;
                    conflicting_edits.push(new_edit);
                    new_edit = new_edits.next().unwrap();
                } else {
                    let mut old_edit = old_edits.next().unwrap();
                    old_edit.old.start = (old_edit.old.start as i32 + applied_delta) as u32;
                    old_edit.old.end = (old_edit.old.end as i32 + applied_delta) as u32;
                    old_edit.new.start = (old_edit.new.start as i32 + applied_delta) as u32;
                    old_edit.new.end = (old_edit.new.end as i32 + applied_delta) as u32;
                    rebased_delta += old_edit.new_len() as i32 - old_edit.old_len() as i32;
                    translated_unreviewed_edits.push(old_edit);
                }
            }
        }

        if conflict {
            new_edit.old.start = (new_edit.old.start as i32 + applied_delta) as u32;
            new_edit.old.end = (new_edit.old.end as i32 + applied_delta) as u32;
            conflicting_edits.push(new_edit);
        } else {
            // This edit doesn't intersect with any old edit, so we can apply it to the old text.
            new_edit.old.start = (new_edit.old.start as i32 + applied_delta - rebased_delta) as u32;
            new_edit.old.end = (new_edit.old.end as i32 + applied_delta - rebased_delta) as u32;
            let old_bytes = old_text.point_to_offset(Point::new(new_edit.old.start, 0))
                ..old_text.point_to_offset(cmp::min(
                    Point::new(new_edit.old.end, 0),
                    old_text.max_point(),
                ));
            let new_bytes = new_text.point_to_offset(Point::new(new_edit.new.start, 0))
                ..new_text.point_to_offset(cmp::min(
                    Point::new(new_edit.new.end, 0),
                    new_text.max_point(),
                ));

            old_text.replace(
                old_bytes,
                &new_text.chunks_in_range(new_bytes).collect::<String>(),
            );
            applied_delta += new_edit.new_len() as i32 - new_edit.old_len() as i32;
        }
    }

    // Push all the outstanding old edits.
    for mut old_edit in old_edits {
        old_edit.old.start = (old_edit.old.start as i32 + applied_delta) as u32;
        old_edit.old.end = (old_edit.old.end as i32 + applied_delta) as u32;
        old_edit.new.start = (old_edit.new.start as i32 + applied_delta) as u32;
        old_edit.new.end = (old_edit.new.end as i32 + applied_delta) as u32;
        translated_unreviewed_edits.push(old_edit);
    }

    translated_unreviewed_edits.compose(conflicting_edits)
}

fn diff_snapshots(
    old_snapshot: &text::BufferSnapshot,
    new_snapshot: &text::BufferSnapshot,
) -> Vec<Edit<u32>> {
    let mut edits = new_snapshot
        .edits_since::<Point>(&old_snapshot.version)
        .map(|edit| {
            if edit.old.start.column == old_snapshot.line_len(edit.old.start.row)
                && new_snapshot.chars_at(edit.new.start).next() == Some('\n')
                && edit.old.start != old_snapshot.max_point()
            {
                Edit {
                    old: edit.old.start.row + 1..edit.old.end.row + 1,
                    new: edit.new.start.row + 1..edit.new.end.row + 1,
                }
            } else if edit.old.start.column == 0
                && edit.old.end.column == 0
                && edit.new.end.column == 0
                && edit.old.end != old_snapshot.max_point()
            {
                Edit {
                    old: edit.old.start.row..edit.old.end.row,
                    new: edit.new.start.row..edit.new.end.row,
                }
            } else {
                Edit {
                    old: edit.old.start.row..edit.old.end.row + 1,
                    new: edit.new.start.row..edit.new.end.row + 1,
                }
            }
        })
        .peekable();
    let mut row_edits = Vec::new();
    while let Some(mut edit) = edits.next() {
        while let Some(next_edit) = edits.peek() {
            if edit.old.end >= next_edit.old.start {
                edit.old.end = next_edit.old.end;
                edit.new.end = next_edit.new.end;
                edits.next();
            } else {
                break;
            }
        }
        row_edits.push(edit);
    }
    row_edits
}

enum ChangeAuthor {
    User,
    Agent,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum TrackedBufferStatus {
    Created,
    Modified,
    Deleted,
}

struct TrackedBuffer {
    buffer: Entity<Buffer>,
    base_text: Rope,
    unreviewed_changes: Patch<u32>,
    status: TrackedBufferStatus,
    version: clock::Global,
    diff: Entity<BufferDiff>,
    snapshot: text::BufferSnapshot,
    diff_update: mpsc::UnboundedSender<(ChangeAuthor, text::BufferSnapshot)>,
    _maintain_diff: Task<()>,
    _subscription: Subscription,
}

impl TrackedBuffer {
    fn has_changes(&self, cx: &App) -> bool {
        self.diff
            .read(cx)
            .hunks(&self.buffer.read(cx), cx)
            .next()
            .is_some()
    }

    fn schedule_diff_update(&self, author: ChangeAuthor, cx: &App) {
        self.diff_update
            .unbounded_send((author, self.buffer.read(cx).text_snapshot()))
            .ok();
    }
}

pub struct ChangedBuffer {
    pub diff: Entity<BufferDiff>,
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use buffer_diff::DiffHunkStatusKind;
    use gpui::TestAppContext;
    use language::Point;
    use project::{FakeFs, Fs, Project, RemoveOptions};
    use rand::prelude::*;
    use serde_json::json;
    use settings::SettingsStore;
    use util::{RandomCharIter, path, post_inc};

    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }

    #[gpui::test(iterations = 10)]
    async fn test_edit_review(cx: &mut TestAppContext) {
        let action_log = cx.new(|_| ActionLog::new());
        let buffer = cx.new(|cx| Buffer::local("abc\ndef\nghi\njkl\nmno", cx));

        cx.update(|cx| {
            action_log.update(cx, |log, cx| log.buffer_read(buffer.clone(), cx));
            buffer.update(cx, |buffer, cx| {
                buffer
                    .edit([(Point::new(1, 1)..Point::new(1, 2), "E")], None, cx)
                    .unwrap()
            });
            buffer.update(cx, |buffer, cx| {
                buffer
                    .edit([(Point::new(4, 2)..Point::new(4, 3), "O")], None, cx)
                    .unwrap()
            });
            action_log.update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
        });
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abc\ndEf\nghi\njkl\nmnO"
        );
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(2, 0),
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "def\n".into(),
                    },
                    HunkStatus {
                        range: Point::new(4, 0)..Point::new(4, 3),
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "mno".into(),
                    }
                ],
            )]
        );

        action_log.update(cx, |log, cx| {
            log.keep_edits_in_range(buffer.clone(), Point::new(3, 0)..Point::new(4, 3), cx)
        });
        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(1, 0)..Point::new(2, 0),
                    diff_status: DiffHunkStatusKind::Modified,
                    old_text: "def\n".into(),
                }],
            )]
        );

        action_log.update(cx, |log, cx| {
            log.keep_edits_in_range(buffer.clone(), Point::new(0, 0)..Point::new(4, 3), cx)
        });
        cx.run_until_parked();
        assert_eq!(unreviewed_hunks(&action_log, cx), vec![]);
    }

    #[gpui::test(iterations = 10)]
    async fn test_overlapping_user_edits(cx: &mut TestAppContext) {
        let action_log = cx.new(|_| ActionLog::new());
        let buffer = cx.new(|cx| Buffer::local("abc\ndef\nghi\njkl\nmno", cx));

        cx.update(|cx| {
            action_log.update(cx, |log, cx| log.buffer_read(buffer.clone(), cx));
            buffer.update(cx, |buffer, cx| {
                buffer
                    .edit([(Point::new(1, 2)..Point::new(2, 3), "F\nGHI")], None, cx)
                    .unwrap()
            });
            action_log.update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
        });
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abc\ndeF\nGHI\njkl\nmno"
        );
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(1, 0)..Point::new(3, 0),
                    diff_status: DiffHunkStatusKind::Modified,
                    old_text: "def\nghi\n".into(),
                }],
            )]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [
                    (Point::new(0, 2)..Point::new(0, 2), "X"),
                    (Point::new(3, 0)..Point::new(3, 0), "Y"),
                ],
                None,
                cx,
            )
        });
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abXc\ndeF\nGHI\nYjkl\nmno"
        );
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(1, 0)..Point::new(3, 0),
                    diff_status: DiffHunkStatusKind::Modified,
                    old_text: "def\nghi\n".into(),
                }],
            )]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(1, 1)..Point::new(1, 1), "Z")], None, cx)
        });
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abXc\ndZeF\nGHI\nYjkl\nmno"
        );
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(1, 0)..Point::new(3, 0),
                    diff_status: DiffHunkStatusKind::Modified,
                    old_text: "def\nghi\n".into(),
                }],
            )]
        );

        action_log.update(cx, |log, cx| {
            log.keep_edits_in_range(buffer.clone(), Point::new(0, 0)..Point::new(1, 0), cx)
        });
        cx.run_until_parked();
        assert_eq!(unreviewed_hunks(&action_log, cx), vec![]);
    }

    #[gpui::test(iterations = 10)]
    async fn test_creation(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });

        let action_log = cx.new(|_| ActionLog::new());

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({})).await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let file_path = project
            .read_with(cx, |project, cx| project.find_project_path("dir/file1", cx))
            .unwrap();

        // Simulate file2 being recreated by a tool.
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(file_path, cx))
            .await
            .unwrap();
        cx.update(|cx| {
            buffer.update(cx, |buffer, cx| buffer.set_text("lorem", cx));
            action_log.update(cx, |log, cx| log.will_create_buffer(buffer.clone(), cx));
        });
        project
            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
            .await
            .unwrap();
        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(0, 0)..Point::new(0, 5),
                    diff_status: DiffHunkStatusKind::Added,
                    old_text: "".into(),
                }],
            )]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "X")], None, cx));
        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(0, 0)..Point::new(0, 6),
                    diff_status: DiffHunkStatusKind::Added,
                    old_text: "".into(),
                }],
            )]
        );

        action_log.update(cx, |log, cx| {
            log.keep_edits_in_range(buffer.clone(), 0..5, cx)
        });
        cx.run_until_parked();
        assert_eq!(unreviewed_hunks(&action_log, cx), vec![]);
    }

    #[gpui::test(iterations = 10)]
    async fn test_deletion(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({"file1": "lorem\n", "file2": "ipsum\n"}),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let file1_path = project
            .read_with(cx, |project, cx| project.find_project_path("dir/file1", cx))
            .unwrap();
        let file2_path = project
            .read_with(cx, |project, cx| project.find_project_path("dir/file2", cx))
            .unwrap();

        let action_log = cx.new(|_| ActionLog::new());
        let buffer1 = project
            .update(cx, |project, cx| {
                project.open_buffer(file1_path.clone(), cx)
            })
            .await
            .unwrap();
        let buffer2 = project
            .update(cx, |project, cx| {
                project.open_buffer(file2_path.clone(), cx)
            })
            .await
            .unwrap();

        action_log.update(cx, |log, cx| log.will_delete_buffer(buffer1.clone(), cx));
        action_log.update(cx, |log, cx| log.will_delete_buffer(buffer2.clone(), cx));
        project
            .update(cx, |project, cx| {
                project.delete_file(file1_path.clone(), false, cx)
            })
            .unwrap()
            .await
            .unwrap();
        project
            .update(cx, |project, cx| {
                project.delete_file(file2_path.clone(), false, cx)
            })
            .unwrap()
            .await
            .unwrap();
        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![
                (
                    buffer1.clone(),
                    vec![HunkStatus {
                        range: Point::new(0, 0)..Point::new(0, 0),
                        diff_status: DiffHunkStatusKind::Deleted,
                        old_text: "lorem\n".into(),
                    }]
                ),
                (
                    buffer2.clone(),
                    vec![HunkStatus {
                        range: Point::new(0, 0)..Point::new(0, 0),
                        diff_status: DiffHunkStatusKind::Deleted,
                        old_text: "ipsum\n".into(),
                    }],
                )
            ]
        );

        // Simulate file1 being recreated externally.
        fs.insert_file(path!("/dir/file1"), "LOREM".as_bytes().to_vec())
            .await;

        // Simulate file2 being recreated by a tool.
        let buffer2 = project
            .update(cx, |project, cx| project.open_buffer(file2_path, cx))
            .await
            .unwrap();
        buffer2.update(cx, |buffer, cx| buffer.set_text("IPSUM", cx));
        action_log.update(cx, |log, cx| log.will_create_buffer(buffer2.clone(), cx));
        project
            .update(cx, |project, cx| project.save_buffer(buffer2.clone(), cx))
            .await
            .unwrap();

        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer2.clone(),
                vec![HunkStatus {
                    range: Point::new(0, 0)..Point::new(0, 5),
                    diff_status: DiffHunkStatusKind::Modified,
                    old_text: "ipsum\n".into(),
                }],
            )]
        );

        // Simulate file2 being deleted externally.
        fs.remove_file(path!("/dir/file2").as_ref(), RemoveOptions::default())
            .await
            .unwrap();
        cx.run_until_parked();
        assert_eq!(unreviewed_hunks(&action_log, cx), vec![]);
    }

    #[gpui::test(iterations = 100)]
    async fn test_random_diffs(mut rng: StdRng, cx: &mut TestAppContext) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(20);

        let action_log = cx.new(|_| ActionLog::new());
        let text = RandomCharIter::new(&mut rng).take(50).collect::<String>();
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        action_log.update(cx, |log, cx| log.buffer_read(buffer.clone(), cx));

        for _ in 0..operations {
            match rng.gen_range(0..100) {
                0..25 => {
                    action_log.update(cx, |log, cx| {
                        let range = buffer.read(cx).random_byte_range(0, &mut rng);
                        log::info!("keeping all edits in range {:?}", range);
                        log.keep_edits_in_range(buffer.clone(), range, cx)
                    });
                }
                _ => {
                    let is_agent_change = rng.gen_bool(0.5);
                    if is_agent_change {
                        log::info!("agent edit");
                    } else {
                        log::info!("user edit");
                    }
                    cx.update(|cx| {
                        buffer.update(cx, |buffer, cx| buffer.randomly_edit(&mut rng, 1, cx));
                        if is_agent_change {
                            action_log.update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
                        }
                    });
                }
            }

            if rng.gen_bool(0.2) {
                quiesce(&action_log, &buffer, cx);
            }
        }

        quiesce(&action_log, &buffer, cx);

        fn quiesce(
            action_log: &Entity<ActionLog>,
            buffer: &Entity<Buffer>,
            cx: &mut TestAppContext,
        ) {
            log::info!("quiescing...");
            cx.run_until_parked();
            action_log.update(cx, |log, cx| {
                let tracked_buffer = log.track_buffer(buffer.clone(), false, cx);
                let mut old_text = tracked_buffer.base_text.clone();
                let new_text = buffer.read(cx).as_rope();
                for edit in tracked_buffer.unreviewed_changes.edits() {
                    let old_start = old_text.point_to_offset(Point::new(edit.new.start, 0));
                    let old_end = old_text.point_to_offset(cmp::min(
                        Point::new(edit.new.start + edit.old_len(), 0),
                        old_text.max_point(),
                    ));
                    old_text.replace(
                        old_start..old_end,
                        &new_text.slice_rows(edit.new.clone()).to_string(),
                    );
                }
                pretty_assertions::assert_eq!(old_text.to_string(), new_text.to_string());
            })
        }
    }

    #[gpui::test(iterations = 100)]
    fn test_rebase_random(mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(20);

        let mut next_line_id = 0;
        let base_lines = (0..rng.gen_range(1..=20))
            .map(|_| post_inc(&mut next_line_id).to_string())
            .collect::<Vec<_>>();
        log::info!("base lines: {:?}", base_lines);

        let (new_lines, patch_1) =
            build_edits(&base_lines, operations, &mut rng, &mut next_line_id);
        log::info!("agent edits: {:#?}", patch_1);
        let (new_lines, patch_2) = build_edits(&new_lines, operations, &mut rng, &mut next_line_id);
        log::info!("user edits: {:#?}", patch_2);

        let mut old_text = Rope::from(base_lines.join("\n"));
        let new_text = Rope::from(new_lines.join("\n"));
        let patch = rebase_patch(&patch_1, patch_2.into_inner(), &mut old_text, &new_text);
        log::info!("rebased edits: {:#?}", patch.edits());

        for edit in patch.edits() {
            let old_start = old_text.point_to_offset(Point::new(edit.new.start, 0));
            let old_end = old_text.point_to_offset(cmp::min(
                Point::new(edit.new.start + edit.old_len(), 0),
                old_text.max_point(),
            ));
            old_text.replace(
                old_start..old_end,
                &new_text.slice_rows(edit.new.clone()).to_string(),
            );
        }
        pretty_assertions::assert_eq!(old_text.to_string(), new_text.to_string());
    }

    fn build_edits(
        lines: &Vec<String>,
        count: usize,
        rng: &mut StdRng,
        next_line_id: &mut usize,
    ) -> (Vec<String>, Patch<u32>) {
        let mut delta = 0i32;
        let mut last_edit_end = 0;
        let mut edits = Patch::default();
        let mut edited_lines = lines.clone();
        for _ in 0..count {
            if last_edit_end >= lines.len() {
                break;
            }

            let end = rng.gen_range(last_edit_end..lines.len());
            let start = rng.gen_range(last_edit_end..=end);
            let old_len = end - start;

            let mut new_len: usize = rng.gen_range(0..=3);
            if start == end && new_len == 0 {
                new_len += 1;
            }

            last_edit_end = end + 1;

            let new_lines = (0..new_len)
                .map(|_| post_inc(next_line_id).to_string())
                .collect::<Vec<_>>();
            log::info!("  editing {:?}: {:?}", start..end, new_lines);
            let old = start as u32..end as u32;
            let new = (start as i32 + delta) as u32..(start as i32 + delta + new_len as i32) as u32;
            edited_lines.splice(
                new.start as usize..new.start as usize + old.len(),
                new_lines,
            );
            edits.push(Edit { old, new });
            delta += new_len as i32 - old_len as i32;
        }
        (edited_lines, edits)
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct HunkStatus {
        range: Range<Point>,
        diff_status: DiffHunkStatusKind,
        old_text: String,
    }

    fn unreviewed_hunks(
        action_log: &Entity<ActionLog>,
        cx: &TestAppContext,
    ) -> Vec<(Entity<Buffer>, Vec<HunkStatus>)> {
        cx.read(|cx| {
            action_log
                .read(cx)
                .changed_buffers(cx)
                .into_iter()
                .map(|(buffer, diff)| {
                    let snapshot = buffer.read(cx).snapshot();
                    (
                        buffer,
                        diff.read(cx)
                            .hunks(&snapshot, cx)
                            .map(|hunk| HunkStatus {
                                diff_status: hunk.status().kind,
                                range: hunk.range,
                                old_text: diff
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

use anyhow::{Context as _, Result};
use buffer_diff::BufferDiff;
use collections::BTreeMap;
use futures::{StreamExt, channel::mpsc};
use gpui::{App, AppContext, AsyncApp, Context, Entity, Subscription, Task, WeakEntity};
use language::{Anchor, Buffer, BufferEvent, DiskState, Point, ToPoint};
use project::{Project, ProjectItem, lsp_store::OpenLspBufferHandle};
use std::{cmp, ops::Range, sync::Arc};
use text::{Edit, Patch, Rope};
use util::RangeExt;

/// Tracks actions performed by tools in a thread
pub struct ActionLog {
    /// Buffers that we want to notify the model about when they change.
    tracked_buffers: BTreeMap<Entity<Buffer>, TrackedBuffer>,
    /// Has the model edited a file since it last checked diagnostics?
    edited_since_project_diagnostics_check: bool,
    /// The project this action log is associated with
    project: Entity<Project>,
}

impl ActionLog {
    /// Creates a new, empty action log associated with the given project.
    pub fn new(project: Entity<Project>) -> Self {
        Self {
            tracked_buffers: BTreeMap::default(),
            edited_since_project_diagnostics_check: false,
            project,
        }
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    /// Notifies a diagnostics check
    pub fn checked_project_diagnostics(&mut self) {
        self.edited_since_project_diagnostics_check = false;
    }

    /// Returns true if any files have been edited since the last project diagnostics check
    pub fn has_edited_files_since_project_diagnostics_check(&self) -> bool {
        self.edited_since_project_diagnostics_check
    }

    fn track_buffer_internal(
        &mut self,
        buffer: Entity<Buffer>,
        is_created: bool,
        cx: &mut Context<Self>,
    ) -> &mut TrackedBuffer {
        let tracked_buffer = self
            .tracked_buffers
            .entry(buffer.clone())
            .or_insert_with(|| {
                let open_lsp_handle = self.project.update(cx, |project, cx| {
                    project.register_buffer_with_language_servers(&buffer, cx)
                });

                let text_snapshot = buffer.read(cx).text_snapshot();
                let diff = cx.new(|cx| BufferDiff::new(&text_snapshot, cx));
                let (diff_update_tx, diff_update_rx) = mpsc::unbounded();
                let base_text;
                let status;
                let unreviewed_changes;
                if is_created {
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
                    _open_lsp_handle: open_lsp_handle,
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
                    self.track_buffer_internal(buffer, false, cx);
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
                            if let ChangeAuthor::User = author {
                                apply_non_conflicting_edits(
                                    &unreviewed_changes,
                                    edits,
                                    &mut base_text,
                                    new_snapshot.as_rope(),
                                );
                            }
                            (Arc::new(base_text.to_string()), base_text)
                        }
                    });

                    anyhow::Ok((
                        rebase,
                        tracked_buffer.diff.clone(),
                        tracked_buffer.buffer.read(cx).language().cloned(),
                        tracked_buffer.buffer.read(cx).language_registry(),
                    ))
                })??;

            let (new_base_text, new_base_text_rope) = rebase.await;
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

            let mut unreviewed_changes = Patch::default();
            if let Ok(diff_snapshot) = diff_snapshot {
                unreviewed_changes = cx
                    .background_spawn({
                        let diff_snapshot = diff_snapshot.clone();
                        let buffer_snapshot = buffer_snapshot.clone();
                        let new_base_text_rope = new_base_text_rope.clone();
                        async move {
                            let mut unreviewed_changes = Patch::default();
                            for hunk in diff_snapshot.hunks_intersecting_range(
                                Anchor::MIN..Anchor::MAX,
                                &buffer_snapshot,
                            ) {
                                let old_range = new_base_text_rope
                                    .offset_to_point(hunk.diff_base_byte_range.start)
                                    ..new_base_text_rope
                                        .offset_to_point(hunk.diff_base_byte_range.end);
                                let new_range = hunk.range.start..hunk.range.end;
                                unreviewed_changes.push(point_to_row_edit(
                                    Edit {
                                        old: old_range,
                                        new: new_range,
                                    },
                                    &new_base_text_rope,
                                    &buffer_snapshot.as_rope(),
                                ));
                            }
                            unreviewed_changes
                        }
                    })
                    .await;

                diff.update(cx, |diff, cx| {
                    diff.set_snapshot(diff_snapshot, &buffer_snapshot, cx)
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
        self.track_buffer_internal(buffer, false, cx);
    }

    /// Mark a buffer as edited, so we can refresh it in the context
    pub fn buffer_created(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.edited_since_project_diagnostics_check = true;
        self.tracked_buffers.remove(&buffer);
        self.track_buffer_internal(buffer.clone(), true, cx);
    }

    /// Mark a buffer as edited, so we can refresh it in the context
    pub fn buffer_edited(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        self.edited_since_project_diagnostics_check = true;

        let tracked_buffer = self.track_buffer_internal(buffer.clone(), false, cx);
        if let TrackedBufferStatus::Deleted = tracked_buffer.status {
            tracked_buffer.status = TrackedBufferStatus::Modified;
        }
        tracked_buffer.schedule_diff_update(ChangeAuthor::Agent, cx);
    }

    pub fn will_delete_buffer(&mut self, buffer: Entity<Buffer>, cx: &mut Context<Self>) {
        let tracked_buffer = self.track_buffer_internal(buffer.clone(), false, cx);
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

    pub fn keep_edits_in_range(
        &mut self,
        buffer: Entity<Buffer>,
        buffer_range: Range<impl language::ToPoint>,
        cx: &mut Context<Self>,
    ) {
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
                let mut delta = 0i32;

                tracked_buffer.unreviewed_changes.retain_mut(|edit| {
                    edit.old.start = (edit.old.start as i32 + delta) as u32;
                    edit.old.end = (edit.old.end as i32 + delta) as u32;

                    if buffer_range.end.row < edit.new.start
                        || buffer_range.start.row > edit.new.end
                    {
                        true
                    } else {
                        let old_range = tracked_buffer
                            .base_text
                            .point_to_offset(Point::new(edit.old.start, 0))
                            ..tracked_buffer.base_text.point_to_offset(cmp::min(
                                Point::new(edit.old.end, 0),
                                tracked_buffer.base_text.max_point(),
                            ));
                        let new_range = tracked_buffer
                            .snapshot
                            .point_to_offset(Point::new(edit.new.start, 0))
                            ..tracked_buffer.snapshot.point_to_offset(cmp::min(
                                Point::new(edit.new.end, 0),
                                tracked_buffer.snapshot.max_point(),
                            ));
                        tracked_buffer.base_text.replace(
                            old_range,
                            &tracked_buffer
                                .snapshot
                                .text_for_range(new_range)
                                .collect::<String>(),
                        );
                        delta += edit.new_len() as i32 - edit.old_len() as i32;
                        false
                    }
                });
                tracked_buffer.schedule_diff_update(ChangeAuthor::User, cx);
            }
        }
    }

    pub fn reject_edits_in_ranges(
        &mut self,
        buffer: Entity<Buffer>,
        buffer_ranges: Vec<Range<impl language::ToPoint>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(tracked_buffer) = self.tracked_buffers.get_mut(&buffer) else {
            return Task::ready(Ok(()));
        };

        match tracked_buffer.status {
            TrackedBufferStatus::Created => {
                let delete = buffer
                    .read(cx)
                    .entry_id(cx)
                    .and_then(|entry_id| {
                        self.project
                            .update(cx, |project, cx| project.delete_entry(entry_id, false, cx))
                    })
                    .unwrap_or(Task::ready(Ok(())));
                self.tracked_buffers.remove(&buffer);
                cx.notify();
                delete
            }
            TrackedBufferStatus::Deleted => {
                buffer.update(cx, |buffer, cx| {
                    buffer.set_text(tracked_buffer.base_text.to_string(), cx)
                });
                let save = self
                    .project
                    .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx));

                // Clear all tracked changes for this buffer and start over as if we just read it.
                self.tracked_buffers.remove(&buffer);
                self.buffer_read(buffer.clone(), cx);
                cx.notify();
                save
            }
            TrackedBufferStatus::Modified => {
                buffer.update(cx, |buffer, cx| {
                    let mut buffer_row_ranges = buffer_ranges
                        .into_iter()
                        .map(|range| {
                            range.start.to_point(buffer).row..range.end.to_point(buffer).row
                        })
                        .peekable();

                    let mut edits_to_revert = Vec::new();
                    for edit in tracked_buffer.unreviewed_changes.edits() {
                        let new_range = tracked_buffer
                            .snapshot
                            .anchor_before(Point::new(edit.new.start, 0))
                            ..tracked_buffer.snapshot.anchor_after(cmp::min(
                                Point::new(edit.new.end, 0),
                                tracked_buffer.snapshot.max_point(),
                            ));
                        let new_row_range = new_range.start.to_point(buffer).row
                            ..new_range.end.to_point(buffer).row;

                        let mut revert = false;
                        while let Some(buffer_row_range) = buffer_row_ranges.peek() {
                            if buffer_row_range.end < new_row_range.start {
                                buffer_row_ranges.next();
                            } else if buffer_row_range.start > new_row_range.end {
                                break;
                            } else {
                                revert = true;
                                break;
                            }
                        }

                        if revert {
                            let old_range = tracked_buffer
                                .base_text
                                .point_to_offset(Point::new(edit.old.start, 0))
                                ..tracked_buffer.base_text.point_to_offset(cmp::min(
                                    Point::new(edit.old.end, 0),
                                    tracked_buffer.base_text.max_point(),
                                ));
                            let old_text = tracked_buffer
                                .base_text
                                .chunks_in_range(old_range)
                                .collect::<String>();
                            edits_to_revert.push((new_range, old_text));
                        }
                    }

                    buffer.edit(edits_to_revert, None, cx);
                });
                self.project
                    .update(cx, |project, cx| project.save_buffer(buffer, cx))
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
}

fn apply_non_conflicting_edits(
    patch: &Patch<u32>,
    edits: Vec<Edit<u32>>,
    old_text: &mut Rope,
    new_text: &Rope,
) {
    let mut old_edits = patch.edits().iter().cloned().peekable();
    let mut new_edits = edits.into_iter().peekable();
    let mut applied_delta = 0i32;
    let mut rebased_delta = 0i32;

    while let Some(mut new_edit) = new_edits.next() {
        let mut conflict = false;

        // Push all the old edits that are before this new edit or that intersect with it.
        while let Some(old_edit) = old_edits.peek() {
            if new_edit.old.end < old_edit.new.start
                || (!old_edit.new.is_empty() && new_edit.old.end == old_edit.new.start)
            {
                break;
            } else if new_edit.old.start > old_edit.new.end
                || (!old_edit.new.is_empty() && new_edit.old.start == old_edit.new.end)
            {
                let old_edit = old_edits.next().unwrap();
                rebased_delta += old_edit.new_len() as i32 - old_edit.old_len() as i32;
            } else {
                conflict = true;
                if new_edits
                    .peek()
                    .map_or(false, |next_edit| next_edit.old.overlaps(&old_edit.new))
                {
                    new_edit = new_edits.next().unwrap();
                } else {
                    let old_edit = old_edits.next().unwrap();
                    rebased_delta += old_edit.new_len() as i32 - old_edit.old_len() as i32;
                }
            }
        }

        if !conflict {
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
}

fn diff_snapshots(
    old_snapshot: &text::BufferSnapshot,
    new_snapshot: &text::BufferSnapshot,
) -> Vec<Edit<u32>> {
    let mut edits = new_snapshot
        .edits_since::<Point>(&old_snapshot.version)
        .map(|edit| point_to_row_edit(edit, old_snapshot.as_rope(), new_snapshot.as_rope()))
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

fn point_to_row_edit(edit: Edit<Point>, old_text: &Rope, new_text: &Rope) -> Edit<u32> {
    if edit.old.start.column == old_text.line_len(edit.old.start.row)
        && new_text
            .chars_at(new_text.point_to_offset(edit.new.start))
            .next()
            == Some('\n')
        && edit.old.start != old_text.max_point()
    {
        Edit {
            old: edit.old.start.row + 1..edit.old.end.row + 1,
            new: edit.new.start.row + 1..edit.new.end.row + 1,
        }
    } else if edit.old.start.column == 0
        && edit.old.end.column == 0
        && edit.new.end.column == 0
        && edit.old.end != old_text.max_point()
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
}

#[derive(Copy, Clone, Debug)]
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
    _open_lsp_handle: OpenLspBufferHandle,
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
    use util::{RandomCharIter, path};

    #[ctor::ctor]
    fn init_logger() {
        zlog::init_test();
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_keep_edits(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({"file": "abc\ndef\nghi\njkl\nmno"}))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let file_path = project
            .read_with(cx, |project, cx| project.find_project_path("dir/file", cx))
            .unwrap();
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(file_path, cx))
            .await
            .unwrap();

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
    async fn test_deletions(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({"file": "abc\ndef\nghi\njkl\nmno\npqr"}),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let file_path = project
            .read_with(cx, |project, cx| project.find_project_path("dir/file", cx))
            .unwrap();
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(file_path, cx))
            .await
            .unwrap();

        cx.update(|cx| {
            action_log.update(cx, |log, cx| log.buffer_read(buffer.clone(), cx));
            buffer.update(cx, |buffer, cx| {
                buffer
                    .edit([(Point::new(1, 0)..Point::new(2, 0), "")], None, cx)
                    .unwrap();
                buffer.finalize_last_transaction();
            });
            buffer.update(cx, |buffer, cx| {
                buffer
                    .edit([(Point::new(3, 0)..Point::new(4, 0), "")], None, cx)
                    .unwrap();
                buffer.finalize_last_transaction();
            });
            action_log.update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
        });
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abc\nghi\njkl\npqr"
        );
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(1, 0),
                        diff_status: DiffHunkStatusKind::Deleted,
                        old_text: "def\n".into(),
                    },
                    HunkStatus {
                        range: Point::new(3, 0)..Point::new(3, 0),
                        diff_status: DiffHunkStatusKind::Deleted,
                        old_text: "mno\n".into(),
                    }
                ],
            )]
        );

        buffer.update(cx, |buffer, cx| buffer.undo(cx));
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abc\nghi\njkl\nmno\npqr"
        );
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(1, 0)..Point::new(1, 0),
                    diff_status: DiffHunkStatusKind::Deleted,
                    old_text: "def\n".into(),
                }],
            )]
        );

        action_log.update(cx, |log, cx| {
            log.keep_edits_in_range(buffer.clone(), Point::new(1, 0)..Point::new(1, 0), cx)
        });
        cx.run_until_parked();
        assert_eq!(unreviewed_hunks(&action_log, cx), vec![]);
    }

    #[gpui::test(iterations = 10)]
    async fn test_overlapping_user_edits(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({"file": "abc\ndef\nghi\njkl\nmno"}))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let file_path = project
            .read_with(cx, |project, cx| project.find_project_path("dir/file", cx))
            .unwrap();
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(file_path, cx))
            .await
            .unwrap();

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
    async fn test_creating_files(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({})).await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let file_path = project
            .read_with(cx, |project, cx| project.find_project_path("dir/file1", cx))
            .unwrap();

        let buffer = project
            .update(cx, |project, cx| project.open_buffer(file_path, cx))
            .await
            .unwrap();
        cx.update(|cx| {
            action_log.update(cx, |log, cx| log.buffer_created(buffer.clone(), cx));
            buffer.update(cx, |buffer, cx| buffer.set_text("lorem", cx));
            action_log.update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
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
    async fn test_deleting_files(cx: &mut TestAppContext) {
        init_test(cx);

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

        let action_log = cx.new(|_| ActionLog::new(project.clone()));
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
        action_log.update(cx, |log, cx| log.buffer_read(buffer2.clone(), cx));
        buffer2.update(cx, |buffer, cx| buffer.set_text("IPSUM", cx));
        action_log.update(cx, |log, cx| log.buffer_edited(buffer2.clone(), cx));
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

    #[gpui::test(iterations = 10)]
    async fn test_reject_edits(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({"file": "abc\ndef\nghi\njkl\nmno"}))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let file_path = project
            .read_with(cx, |project, cx| project.find_project_path("dir/file", cx))
            .unwrap();
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(file_path, cx))
            .await
            .unwrap();

        cx.update(|cx| {
            action_log.update(cx, |log, cx| log.buffer_read(buffer.clone(), cx));
            buffer.update(cx, |buffer, cx| {
                buffer
                    .edit([(Point::new(1, 1)..Point::new(1, 2), "E\nXYZ")], None, cx)
                    .unwrap()
            });
            buffer.update(cx, |buffer, cx| {
                buffer
                    .edit([(Point::new(5, 2)..Point::new(5, 3), "O")], None, cx)
                    .unwrap()
            });
            action_log.update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
        });
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abc\ndE\nXYZf\nghi\njkl\nmnO"
        );
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(3, 0),
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "def\n".into(),
                    },
                    HunkStatus {
                        range: Point::new(5, 0)..Point::new(5, 3),
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "mno".into(),
                    }
                ],
            )]
        );

        // If the rejected range doesn't overlap with any hunk, we ignore it.
        action_log
            .update(cx, |log, cx| {
                log.reject_edits_in_ranges(
                    buffer.clone(),
                    vec![Point::new(4, 0)..Point::new(4, 0)],
                    cx,
                )
            })
            .await
            .unwrap();
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abc\ndE\nXYZf\nghi\njkl\nmnO"
        );
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(3, 0),
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "def\n".into(),
                    },
                    HunkStatus {
                        range: Point::new(5, 0)..Point::new(5, 3),
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "mno".into(),
                    }
                ],
            )]
        );

        action_log
            .update(cx, |log, cx| {
                log.reject_edits_in_ranges(
                    buffer.clone(),
                    vec![Point::new(0, 0)..Point::new(1, 0)],
                    cx,
                )
            })
            .await
            .unwrap();
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abc\ndef\nghi\njkl\nmnO"
        );
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(4, 0)..Point::new(4, 3),
                    diff_status: DiffHunkStatusKind::Modified,
                    old_text: "mno".into(),
                }],
            )]
        );

        action_log
            .update(cx, |log, cx| {
                log.reject_edits_in_ranges(
                    buffer.clone(),
                    vec![Point::new(4, 0)..Point::new(4, 0)],
                    cx,
                )
            })
            .await
            .unwrap();
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abc\ndef\nghi\njkl\nmno"
        );
        assert_eq!(unreviewed_hunks(&action_log, cx), vec![]);
    }

    #[gpui::test(iterations = 10)]
    async fn test_reject_multiple_edits(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({"file": "abc\ndef\nghi\njkl\nmno"}))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let file_path = project
            .read_with(cx, |project, cx| project.find_project_path("dir/file", cx))
            .unwrap();
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(file_path, cx))
            .await
            .unwrap();

        cx.update(|cx| {
            action_log.update(cx, |log, cx| log.buffer_read(buffer.clone(), cx));
            buffer.update(cx, |buffer, cx| {
                buffer
                    .edit([(Point::new(1, 1)..Point::new(1, 2), "E\nXYZ")], None, cx)
                    .unwrap()
            });
            buffer.update(cx, |buffer, cx| {
                buffer
                    .edit([(Point::new(5, 2)..Point::new(5, 3), "O")], None, cx)
                    .unwrap()
            });
            action_log.update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
        });
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abc\ndE\nXYZf\nghi\njkl\nmnO"
        );
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![
                    HunkStatus {
                        range: Point::new(1, 0)..Point::new(3, 0),
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "def\n".into(),
                    },
                    HunkStatus {
                        range: Point::new(5, 0)..Point::new(5, 3),
                        diff_status: DiffHunkStatusKind::Modified,
                        old_text: "mno".into(),
                    }
                ],
            )]
        );

        action_log.update(cx, |log, cx| {
            let range_1 = buffer.read(cx).anchor_before(Point::new(0, 0))
                ..buffer.read(cx).anchor_before(Point::new(1, 0));
            let range_2 = buffer.read(cx).anchor_before(Point::new(5, 0))
                ..buffer.read(cx).anchor_before(Point::new(5, 3));

            log.reject_edits_in_ranges(buffer.clone(), vec![range_1, range_2], cx)
                .detach();
            assert_eq!(
                buffer.read_with(cx, |buffer, _| buffer.text()),
                "abc\ndef\nghi\njkl\nmno"
            );
        });
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "abc\ndef\nghi\njkl\nmno"
        );
        assert_eq!(unreviewed_hunks(&action_log, cx), vec![]);
    }

    #[gpui::test(iterations = 10)]
    async fn test_reject_deleted_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({"file": "content"}))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let file_path = project
            .read_with(cx, |project, cx| project.find_project_path("dir/file", cx))
            .unwrap();
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(file_path.clone(), cx))
            .await
            .unwrap();

        cx.update(|cx| {
            action_log.update(cx, |log, cx| log.will_delete_buffer(buffer.clone(), cx));
        });
        project
            .update(cx, |project, cx| {
                project.delete_file(file_path.clone(), false, cx)
            })
            .unwrap()
            .await
            .unwrap();
        cx.run_until_parked();
        assert!(!fs.is_file(path!("/dir/file").as_ref()).await);
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(0, 0)..Point::new(0, 0),
                    diff_status: DiffHunkStatusKind::Deleted,
                    old_text: "content".into(),
                }]
            )]
        );

        action_log
            .update(cx, |log, cx| {
                log.reject_edits_in_ranges(
                    buffer.clone(),
                    vec![Point::new(0, 0)..Point::new(0, 0)],
                    cx,
                )
            })
            .await
            .unwrap();
        cx.run_until_parked();
        assert_eq!(buffer.read_with(cx, |buffer, _| buffer.text()), "content");
        assert!(fs.is_file(path!("/dir/file").as_ref()).await);
        assert_eq!(unreviewed_hunks(&action_log, cx), vec![]);
    }

    #[gpui::test(iterations = 10)]
    async fn test_reject_created_file(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let file_path = project
            .read_with(cx, |project, cx| {
                project.find_project_path("dir/new_file", cx)
            })
            .unwrap();

        let buffer = project
            .update(cx, |project, cx| project.open_buffer(file_path, cx))
            .await
            .unwrap();
        cx.update(|cx| {
            action_log.update(cx, |log, cx| log.buffer_created(buffer.clone(), cx));
            buffer.update(cx, |buffer, cx| buffer.set_text("content", cx));
            action_log.update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
        });
        project
            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
            .await
            .unwrap();
        assert!(fs.is_file(path!("/dir/new_file").as_ref()).await);
        cx.run_until_parked();
        assert_eq!(
            unreviewed_hunks(&action_log, cx),
            vec![(
                buffer.clone(),
                vec![HunkStatus {
                    range: Point::new(0, 0)..Point::new(0, 7),
                    diff_status: DiffHunkStatusKind::Added,
                    old_text: "".into(),
                }],
            )]
        );

        action_log
            .update(cx, |log, cx| {
                log.reject_edits_in_ranges(
                    buffer.clone(),
                    vec![Point::new(0, 0)..Point::new(0, 11)],
                    cx,
                )
            })
            .await
            .unwrap();
        cx.run_until_parked();
        assert!(!fs.is_file(path!("/dir/new_file").as_ref()).await);
        assert_eq!(unreviewed_hunks(&action_log, cx), vec![]);
    }

    #[gpui::test(iterations = 100)]
    async fn test_random_diffs(mut rng: StdRng, cx: &mut TestAppContext) {
        init_test(cx);

        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(20);

        let text = RandomCharIter::new(&mut rng).take(50).collect::<String>();
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({"file": text})).await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let file_path = project
            .read_with(cx, |project, cx| project.find_project_path("dir/file", cx))
            .unwrap();
        let buffer = project
            .update(cx, |project, cx| project.open_buffer(file_path, cx))
            .await
            .unwrap();

        action_log.update(cx, |log, cx| log.buffer_read(buffer.clone(), cx));

        for _ in 0..operations {
            match rng.gen_range(0..100) {
                0..25 => {
                    action_log.update(cx, |log, cx| {
                        let range = buffer.read(cx).random_byte_range(0, &mut rng);
                        log::info!("keeping edits in range {:?}", range);
                        log.keep_edits_in_range(buffer.clone(), range, cx)
                    });
                }
                25..50 => {
                    action_log
                        .update(cx, |log, cx| {
                            let range = buffer.read(cx).random_byte_range(0, &mut rng);
                            log::info!("rejecting edits in range {:?}", range);
                            log.reject_edits_in_ranges(buffer.clone(), vec![range], cx)
                        })
                        .await
                        .unwrap();
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
                let tracked_buffer = log.tracked_buffers.get(&buffer).unwrap();
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

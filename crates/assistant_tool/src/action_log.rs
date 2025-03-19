use anyhow::Result;
use buffer_diff::BufferDiff;
use collections::{BTreeMap, HashMap, HashSet};
use gpui::{App, AppContext, Context, Entity, Task};
use language::Buffer;

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
    unreviewed_edit_ids: Vec<clock::Lamport>,
    reviewed_edit_ids: Vec<clock::Lamport>,
    version: clock::Global,
    pub diff: Entity<BufferDiff>,
    secondary_diff: Entity<BufferDiff>,
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
                    unreviewed_edit_ids: Vec::new(),
                    reviewed_edit_ids: Vec::new(),
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

        let unreviewed_edits_to_undo = tracked_buffer
            .unreviewed_edit_ids
            .iter()
            .map(|edit_id| (*edit_id, u32::MAX))
            .collect::<HashMap<_, _>>();
        let buffer_without_unreviewed_edits = buffer.update(cx, |buffer, cx| buffer.branch(cx));
        buffer_without_unreviewed_edits.update(cx, |buffer, cx| {
            buffer.undo_operations(unreviewed_edits_to_undo, cx);
        });
        let primary_diff_update = tracked_buffer.diff.update(cx, |diff, cx| {
            diff.set_base_text(
                buffer_without_unreviewed_edits,
                buffer.read(cx).text_snapshot(),
                cx,
            )
        });

        let edits_to_undo = tracked_buffer
            .unreviewed_edit_ids
            .iter()
            .chain(&tracked_buffer.reviewed_edit_ids)
            .map(|edit_id| (*edit_id, u32::MAX))
            .collect::<HashMap<_, _>>();
        let buffer_without_edits = buffer.update(cx, |buffer, cx| buffer.branch(cx));
        buffer_without_edits.update(cx, |buffer, cx| {
            buffer.undo_operations(edits_to_undo, cx);
        });
        let secondary_diff_update = tracked_buffer.secondary_diff.update(cx, |diff, cx| {
            diff.set_base_text(buffer_without_edits, buffer.read(cx).text_snapshot(), cx)
        });

        cx.spawn(async move |this, cx| {
            _ = primary_diff_update.await;
            _ = secondary_diff_update.await;
            this.update(cx, |_this, cx| cx.notify())?;
            Ok(())
        })
    }

    /// Returns the set of buffers that contain changes that haven't been reviewed by the user.
    pub fn unreviewed_buffers(&self) -> BTreeMap<Entity<Buffer>, TrackedBuffer> {
        self.tracked_buffers
            .iter()
            .filter(|(_, tracked)| !tracked.unreviewed_edit_ids.is_empty())
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

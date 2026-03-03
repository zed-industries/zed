use anyhow::Result;
use buffer_diff::{BufferDiff, InternalDiffHunk};
use gpui::{App, AppContext, AsyncApp, Context, Entity, Task};
use itertools::Itertools;
use language::{
    Anchor, Buffer, Capability, LanguageRegistry, OffsetRangeExt as _, Point, TextBuffer,
};
use multi_buffer::{MultiBuffer, PathKey, excerpt_context_lines};
use std::{cmp::Reverse, ops::Range, path::Path, sync::Arc};
use streaming_diff::LineOperation;
use sum_tree::SumTree;
use util::ResultExt;

pub enum Diff {
    Pending(PendingDiff),
    Finalized(FinalizedDiff),
}

impl Diff {
    pub fn finalized(
        path: String,
        old_text: Option<String>,
        new_text: String,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut Context<Self>,
    ) -> Self {
        let multibuffer = cx.new(|_cx| MultiBuffer::without_headers(Capability::ReadOnly));
        let new_buffer = cx.new(|cx| Buffer::local(new_text, cx));
        let base_text = old_text.clone().unwrap_or(String::new()).into();
        let task = cx.spawn({
            let multibuffer = multibuffer.clone();
            let path = path.clone();
            let buffer = new_buffer.clone();
            async move |_, cx| {
                let language = language_registry
                    .load_language_for_file_path(Path::new(&path))
                    .await
                    .log_err();

                buffer.update(cx, |buffer, cx| buffer.set_language(language.clone(), cx));
                buffer.update(cx, |buffer, _| buffer.parsing_idle()).await;

                let diff = build_buffer_diff(
                    old_text.unwrap_or("".into()).into(),
                    &buffer,
                    Some(language_registry.clone()),
                    cx,
                )
                .await?;

                multibuffer.update(cx, |multibuffer, cx| {
                    let hunk_ranges = {
                        let buffer = buffer.read(cx);
                        diff.read(cx)
                            .snapshot(cx)
                            .hunks_intersecting_range(
                                Anchor::min_for_buffer(buffer.remote_id())
                                    ..Anchor::max_for_buffer(buffer.remote_id()),
                                buffer,
                            )
                            .map(|diff_hunk| diff_hunk.buffer_range.to_point(buffer))
                            .collect::<Vec<_>>()
                    };

                    multibuffer.set_excerpts_for_path(
                        PathKey::for_buffer(&buffer, cx),
                        buffer.clone(),
                        hunk_ranges,
                        excerpt_context_lines(cx),
                        cx,
                    );
                    multibuffer.add_diff(diff, cx);
                });

                anyhow::Ok(())
            }
        });

        Self::Finalized(FinalizedDiff {
            multibuffer,
            path,
            base_text,
            new_buffer,
            _update_diff: task,
        })
    }

    pub fn new(buffer: Entity<Buffer>, cx: &mut Context<Self>) -> Self {
        let buffer_text_snapshot = buffer.read(cx).text_snapshot();
        let language = buffer.read(cx).language().cloned();
        let language_registry = buffer.read(cx).language_registry();
        let buffer_diff = cx.new(|cx| {
            let mut diff = BufferDiff::new_unchanged(&buffer_text_snapshot, cx);
            diff.language_changed(language.clone(), language_registry.clone(), cx);
            let secondary_diff = cx.new(|cx| {
                // For the secondary diff buffer we skip assigning the language as we do not really need to perform any syntax highlighting on
                // it. As a result, by skipping it we are potentially shaving off a lot of RSS plus we get a snappier feel for large diff
                // view multibuffers.
                BufferDiff::new_unchanged(&buffer_text_snapshot, cx)
            });
            diff.set_secondary_diff(secondary_diff);
            diff
        });

        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::without_headers(Capability::ReadOnly);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer.add_diff(buffer_diff.clone(), cx);
            multibuffer
        });

        Self::Pending(PendingDiff {
            multibuffer,
            base_text: Arc::from(buffer_text_snapshot.text().as_str()),
            // _subscription: cx.observe(&buffer, |this, _, cx| {
            //     if let Diff::Pending(diff) = this {
            //         diff.update(cx);
            //     }
            // }),
            new_buffer: buffer,
            diff: buffer_diff,
            revealed_ranges: Vec::new(),
            update_diff: Task::ready(Ok(())),
            pending_update: None,
            is_updating: false,
        })
    }

    pub fn reveal_range(&mut self, range: Range<Anchor>, cx: &mut Context<Self>) {
        if let Self::Pending(diff) = self {
            diff.reveal_range(range, cx);
        }
    }

    pub fn finalize(&mut self, cx: &mut Context<Self>) {
        if let Self::Pending(diff) = self {
            *self = Self::Finalized(diff.finalize(cx));
        }
    }

    /// Returns the original text before any edits were applied.
    pub fn base_text(&self) -> &Arc<str> {
        match self {
            Self::Pending(PendingDiff { base_text, .. }) => base_text,
            Self::Finalized(FinalizedDiff { base_text, .. }) => base_text,
        }
    }

    /// Returns the buffer being edited (for pending diffs) or the snapshot buffer (for finalized diffs).
    pub fn buffer(&self) -> &Entity<Buffer> {
        match self {
            Self::Pending(PendingDiff { new_buffer, .. }) => new_buffer,
            Self::Finalized(FinalizedDiff { new_buffer, .. }) => new_buffer,
        }
    }

    pub fn file_path(&self, cx: &App) -> Option<String> {
        match self {
            Self::Pending(PendingDiff { new_buffer, .. }) => new_buffer
                .read(cx)
                .file()
                .map(|file| file.full_path(cx).to_string_lossy().into_owned()),
            Self::Finalized(FinalizedDiff { path, .. }) => Some(path.clone()),
        }
    }

    pub fn multibuffer(&self) -> &Entity<MultiBuffer> {
        match self {
            Self::Pending(PendingDiff { multibuffer, .. }) => multibuffer,
            Self::Finalized(FinalizedDiff { multibuffer, .. }) => multibuffer,
        }
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        let buffer_text = self
            .multibuffer()
            .read(cx)
            .all_buffers()
            .iter()
            .map(|buffer| buffer.read(cx).text())
            .join("\n");
        let path = match self {
            Diff::Pending(PendingDiff {
                new_buffer: buffer, ..
            }) => buffer
                .read(cx)
                .file()
                .map(|file| file.path().display(file.path_style(cx))),
            Diff::Finalized(FinalizedDiff { path, .. }) => Some(path.as_str().into()),
        };
        format!(
            "Diff: {}\n```\n{}\n```\n",
            path.unwrap_or("untitled".into()),
            buffer_text
        )
    }

    pub fn has_revealed_range(&self, cx: &App) -> bool {
        self.multibuffer().read(cx).paths().next().is_some()
    }

    pub fn needs_update(&self, old_text: &str, new_text: &str, cx: &App) -> bool {
        match self {
            Diff::Pending(PendingDiff {
                base_text,
                new_buffer,
                ..
            }) => {
                base_text.as_ref() != old_text
                    || !new_buffer.read(cx).as_rope().chunks().equals_str(new_text)
            }
            Diff::Finalized(FinalizedDiff {
                base_text,
                new_buffer,
                ..
            }) => {
                base_text.as_ref() != old_text
                    || !new_buffer.read(cx).as_rope().chunks().equals_str(new_text)
            }
        }
    }

    pub fn update_pending(
        &mut self,
        operations: Vec<LineOperation>,
        snapshot: text::BufferSnapshot,
        cx: &mut Context<Diff>,
    ) {
        match self {
            Diff::Pending(diff) => diff.update(operations, snapshot, cx),
            Diff::Finalized(_) => {}
        }
    }
}

pub struct PendingDiff {
    multibuffer: Entity<MultiBuffer>,
    base_text: Arc<str>,
    new_buffer: Entity<Buffer>,
    diff: Entity<BufferDiff>,
    revealed_ranges: Vec<Range<Anchor>>,
    // _subscription: Subscription,
    update_diff: Task<Result<()>>,
    // The latest update waiting to be processed. Storing only the latest means
    // intermediate chunks are coalesced when the worker task can't keep up.
    pending_update: Option<PendingUpdate>,
    is_updating: bool,
}

struct PendingUpdate {
    operations: Vec<LineOperation>,
    base_snapshot: text::BufferSnapshot,
    text_snapshot: text::BufferSnapshot,
}

fn compute_hunks(
    diff_base: &text::BufferSnapshot,
    buffer: &text::BufferSnapshot,
    line_operations: Vec<LineOperation>,
) -> SumTree<buffer_diff::InternalDiffHunk> {
    let mut tree = SumTree::new(buffer);

    let mut old_row = 0u32;
    let mut new_row = 0u32;

    // Merge adjacent Delete+Insert into a single Modified hunk
    let mut pending_delete_lines: Option<u32> = None;

    let flush_delete = |pending_delete_lines: &mut Option<u32>,
                        old_row: &mut u32,
                        new_row: u32,
                        tree: &mut SumTree<InternalDiffHunk>,
                        diff_base: &text::BufferSnapshot,
                        buffer: &text::BufferSnapshot| {
        if let Some(del_lines) = pending_delete_lines.take() {
            let old_start =
                diff_base.point_to_offset(Point::new(*old_row, 0).min(diff_base.max_point()));
            let old_end = diff_base
                .point_to_offset(Point::new(*old_row + del_lines, 0).min(diff_base.max_point()));
            let new_pos = buffer.anchor_before(Point::new(new_row, 0).min(buffer.max_point()));
            tree.push(
                InternalDiffHunk {
                    buffer_range: new_pos..new_pos,
                    diff_base_byte_range: old_start..old_end,
                    base_word_diffs: Vec::new(),
                    buffer_word_diffs: Vec::new(),
                },
                buffer,
            );
            *old_row += del_lines;
        }
    };

    for operation in line_operations {
        match operation {
            LineOperation::Delete { lines } => {
                // Accumulate deletions — they might be followed by an Insert (= modification)
                *pending_delete_lines.get_or_insert(0) += lines;
            }
            LineOperation::Insert { lines } => {
                let old_start =
                    diff_base.point_to_offset(Point::new(old_row, 0).min(diff_base.max_point()));
                let (old_end, del_lines) = if let Some(del_lines) = pending_delete_lines.take() {
                    // Delete followed by Insert = Modified hunk
                    let old_end = diff_base.point_to_offset(
                        Point::new(old_row + del_lines, 0).min(diff_base.max_point()),
                    );
                    (old_end, del_lines)
                } else {
                    // Pure insertion
                    (old_start, 0)
                };
                let new_start =
                    buffer.anchor_before(Point::new(new_row, 0).min(buffer.max_point()));
                let new_end =
                    buffer.anchor_before(Point::new(new_row + lines, 0).min(buffer.max_point()));
                tree.push(
                    InternalDiffHunk {
                        buffer_range: new_start..new_end,
                        diff_base_byte_range: old_start..old_end,
                        base_word_diffs: Vec::new(),
                        buffer_word_diffs: Vec::new(),
                    },
                    buffer,
                );
                old_row += del_lines;
                new_row += lines;
            }
            LineOperation::Keep { lines } => {
                // Flush any pending deletion before a Keep
                flush_delete(
                    &mut pending_delete_lines,
                    &mut old_row,
                    new_row,
                    &mut tree,
                    diff_base,
                    buffer,
                );
                // Keep = unchanged, no hunk to push
                old_row += lines;
                new_row += lines;
            }
        }
    }

    // Flush any trailing deletion
    flush_delete(
        &mut pending_delete_lines,
        &mut old_row,
        new_row,
        &mut tree,
        diff_base,
        buffer,
    );

    tree
}

impl PendingDiff {
    pub fn update(
        &mut self,
        operations: Vec<LineOperation>,
        base_snapshot: text::BufferSnapshot,
        cx: &mut Context<Diff>,
    ) {
        // Capture the buffer snapshot now, synchronously, so it matches the
        // line operations. Capturing it inside the spawned task would race with
        // subsequent chunks arriving before the task starts.
        let text_snapshot = self.new_buffer.read(cx).text_snapshot();
        self.pending_update = Some(PendingUpdate {
            operations,
            base_snapshot,
            text_snapshot,
        });
        if !self.is_updating {
            self.flush_pending_update(cx);
        }
    }

    fn flush_pending_update(&mut self, cx: &mut Context<Diff>) {
        let Some(PendingUpdate {
            operations,
            base_snapshot,
            text_snapshot,
        }) = self.pending_update.take()
        else {
            self.is_updating = false;
            return;
        };
        self.is_updating = true;

        let buffer_diff = self.diff.clone();
        let base_text = self.base_text.clone();
        let language = self.new_buffer.read(cx).language().cloned();
        self.update_diff = cx.spawn(async move |diff, cx| {
            let snapshot = text_snapshot.clone();
            let update = buffer_diff
                .update(cx, |diff, cx| {
                    diff.update_diff_impl(
                        text_snapshot.clone(),
                        Some(base_text.clone()),
                        None,
                        language,
                        move |_d, _b, _o| compute_hunks(&base_snapshot, &text_snapshot, operations),
                        cx,
                    )
                })
                .await;
            let (task1, task2) = buffer_diff.update(cx, |diff, cx| {
                let task1 = diff.set_snapshot(update.clone(), &snapshot, cx);
                let task2 = diff
                    .secondary_diff()
                    .unwrap()
                    .update(cx, |diff, cx| diff.set_snapshot(update, &snapshot, cx));
                (task1, task2)
            });
            task1.await;
            task2.await;
            diff.update(cx, |diff, cx| {
                if let Diff::Pending(diff) = diff {
                    diff.update_visible_ranges(cx);
                    // Pick up any update that arrived while this task was running.
                    diff.flush_pending_update(cx);
                }
            })
        });
    }

    pub fn reveal_range(&mut self, range: Range<Anchor>, cx: &mut Context<Diff>) {
        self.revealed_ranges.push(range);
        self.update_visible_ranges(cx);
    }

    fn finalize(&self, cx: &mut Context<Diff>) -> FinalizedDiff {
        let ranges = self.excerpt_ranges(cx);
        let base_text = self.base_text.clone();
        let new_buffer = self.new_buffer.read(cx);
        let language_registry = new_buffer.language_registry();

        let path = new_buffer
            .file()
            .map(|file| file.path().display(file.path_style(cx)))
            .unwrap_or("untitled".into())
            .into();
        let replica_id = new_buffer.replica_id();

        // Replace the buffer in the multibuffer with the snapshot
        let buffer = cx.new(|cx| {
            let language = self.new_buffer.read(cx).language().cloned();
            let buffer = TextBuffer::new_normalized(
                replica_id,
                cx.entity_id().as_non_zero_u64().into(),
                self.new_buffer.read(cx).line_ending(),
                self.new_buffer.read(cx).as_rope().clone(),
            );
            let mut buffer = Buffer::build(buffer, None, Capability::ReadWrite);
            buffer.set_language(language, cx);
            buffer
        });

        let buffer_diff = cx.spawn({
            let buffer = buffer.clone();
            async move |_this, cx| {
                buffer.update(cx, |buffer, _| buffer.parsing_idle()).await;
                build_buffer_diff(base_text, &buffer, language_registry, cx).await
            }
        });

        let update_diff = cx.spawn(async move |this, cx| {
            let buffer_diff = buffer_diff.await?;
            this.update(cx, |this, cx| {
                this.multibuffer().update(cx, |multibuffer, cx| {
                    let path_key = PathKey::for_buffer(&buffer, cx);
                    multibuffer.clear(cx);
                    multibuffer.set_excerpts_for_path(
                        path_key,
                        buffer,
                        ranges,
                        excerpt_context_lines(cx),
                        cx,
                    );
                    multibuffer.add_diff(buffer_diff.clone(), cx);
                });

                cx.notify();
            })
        });

        FinalizedDiff {
            path,
            base_text: self.base_text.clone(),
            multibuffer: self.multibuffer.clone(),
            new_buffer: self.new_buffer.clone(),
            _update_diff: update_diff,
        }
    }

    fn update_visible_ranges(&mut self, cx: &mut Context<Diff>) {
        let ranges = self.excerpt_ranges(cx);
        self.multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.set_excerpts_for_path(
                PathKey::for_buffer(&self.new_buffer, cx),
                self.new_buffer.clone(),
                ranges,
                excerpt_context_lines(cx),
                cx,
            );
            let end = multibuffer.len(cx);
            Some(multibuffer.snapshot(cx).offset_to_point(end).row + 1)
        });
        cx.notify();
    }

    fn excerpt_ranges(&self, cx: &App) -> Vec<Range<Point>> {
        let buffer = self.new_buffer.read(cx);
        let mut ranges = self
            .diff
            .read(cx)
            .snapshot(cx)
            .hunks_intersecting_range(
                Anchor::min_for_buffer(buffer.remote_id())
                    ..Anchor::max_for_buffer(buffer.remote_id()),
                buffer,
            )
            .map(|diff_hunk| diff_hunk.buffer_range.to_point(buffer))
            .collect::<Vec<_>>();
        ranges.extend(
            self.revealed_ranges
                .iter()
                .map(|range| range.to_point(buffer)),
        );
        ranges.sort_unstable_by_key(|range| (range.start, Reverse(range.end)));

        // Merge adjacent ranges
        let mut ranges = ranges.into_iter().peekable();
        let mut merged_ranges = Vec::new();
        while let Some(mut range) = ranges.next() {
            while let Some(next_range) = ranges.peek() {
                if range.end >= next_range.start {
                    range.end = range.end.max(next_range.end);
                    ranges.next();
                } else {
                    break;
                }
            }

            merged_ranges.push(range);
        }
        merged_ranges
    }
}

pub struct FinalizedDiff {
    path: String,
    base_text: Arc<str>,
    new_buffer: Entity<Buffer>,
    multibuffer: Entity<MultiBuffer>,
    _update_diff: Task<Result<()>>,
}

async fn build_buffer_diff(
    old_text: Arc<str>,
    buffer: &Entity<Buffer>,
    language_registry: Option<Arc<LanguageRegistry>>,
    cx: &mut AsyncApp,
) -> Result<Entity<BufferDiff>> {
    let language = cx.update(|cx| buffer.read(cx).language().cloned());
    let text_snapshot = cx.update(|cx| buffer.read(cx).text_snapshot());
    let buffer = cx.update(|cx| buffer.read(cx).snapshot());

    let secondary_diff = cx.new(|cx| BufferDiff::new(&buffer, cx));

    let update = secondary_diff
        .update(cx, |secondary_diff, cx| {
            secondary_diff.update_diff(
                text_snapshot.clone(),
                Some(old_text),
                Some(false),
                language.clone(),
                cx,
            )
        })
        .await;

    secondary_diff
        .update(cx, |secondary_diff, cx| {
            secondary_diff.set_snapshot(update.clone(), &buffer, cx)
        })
        .await;

    let diff = cx.new(|cx| BufferDiff::new(&buffer, cx));
    diff.update(cx, |diff, cx| {
        diff.language_changed(language, language_registry, cx);
        diff.set_secondary_diff(secondary_diff);
        diff.set_snapshot(update.clone(), &buffer, cx)
    })
    .await;
    Ok(diff)
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, TestAppContext};
    use language::Buffer;

    use crate::Diff;

    #[gpui::test]
    async fn test_pending_diff(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| Buffer::local("hello!", cx));
        let _diff = cx.new(|cx| Diff::new(buffer.clone(), cx));
        buffer.update(cx, |buffer, cx| {
            buffer.set_text("HELLO!", cx);
        });
        cx.run_until_parked();
    }
}

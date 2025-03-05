use futures::channel::oneshot;
use git2::{DiffLineType as GitDiffLineType, DiffOptions as GitOptions, Patch as GitPatch};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, Task};
use language::{Language, LanguageRegistry};
use rope::Rope;
use std::cmp::Ordering;
use std::mem;
use std::{future::Future, iter, ops::Range, sync::Arc};
use sum_tree::{SumTree, TreeMap};
use text::ToOffset as _;
use text::{Anchor, Bias, BufferId, OffsetRangeExt, Point};
use util::ResultExt;

pub struct BufferDiff {
    pub buffer_id: BufferId,
    inner: BufferDiffInner,
    secondary_diff: Option<Entity<BufferDiff>>,
}

#[derive(Clone, Debug)]
pub struct BufferDiffSnapshot {
    inner: BufferDiffInner,
    secondary_diff: Option<Box<BufferDiffSnapshot>>,
}

#[derive(Clone)]
struct BufferDiffInner {
    hunks: SumTree<InternalDiffHunk>,
    pending_hunks: TreeMap<usize, PendingHunk>,
    base_text: language::BufferSnapshot,
    base_text_exists: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiffHunkStatus {
    pub kind: DiffHunkStatusKind,
    pub secondary: DiffHunkSecondaryStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiffHunkStatusKind {
    Added,
    Modified,
    Deleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiffHunkSecondaryStatus {
    HasSecondaryHunk,
    OverlapsWithSecondaryHunk,
    None,
    SecondaryHunkAdditionPending,
    SecondaryHunkRemovalPending,
}

/// A diff hunk resolved to rows in the buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffHunk {
    /// The buffer range as points.
    pub range: Range<Point>,
    /// The range in the buffer to which this hunk corresponds.
    pub buffer_range: Range<Anchor>,
    /// The range in the buffer's diff base text to which this hunk corresponds.
    pub diff_base_byte_range: Range<usize>,
    pub secondary_status: DiffHunkSecondaryStatus,
}

/// We store [`InternalDiffHunk`]s internally so we don't need to store the additional row range.
#[derive(Debug, Clone, PartialEq, Eq)]
struct InternalDiffHunk {
    buffer_range: Range<Anchor>,
    diff_base_byte_range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PendingHunk {
    buffer_version: clock::Global,
    new_status: DiffHunkSecondaryStatus,
}

#[derive(Debug, Default, Clone)]
pub struct DiffHunkSummary {
    buffer_range: Range<Anchor>,
}

impl sum_tree::Item for InternalDiffHunk {
    type Summary = DiffHunkSummary;

    fn summary(&self, _cx: &text::BufferSnapshot) -> Self::Summary {
        DiffHunkSummary {
            buffer_range: self.buffer_range.clone(),
        }
    }
}

impl sum_tree::Summary for DiffHunkSummary {
    type Context = text::BufferSnapshot;

    fn zero(_cx: &Self::Context) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, other: &Self, buffer: &Self::Context) {
        self.buffer_range.start = self
            .buffer_range
            .start
            .min(&other.buffer_range.start, buffer);
        self.buffer_range.end = self.buffer_range.end.max(&other.buffer_range.end, buffer);
    }
}

impl sum_tree::SeekTarget<'_, DiffHunkSummary, DiffHunkSummary> for Anchor {
    fn cmp(&self, cursor_location: &DiffHunkSummary, buffer: &text::BufferSnapshot) -> Ordering {
        if self
            .cmp(&cursor_location.buffer_range.start, buffer)
            .is_lt()
        {
            Ordering::Less
        } else if self.cmp(&cursor_location.buffer_range.end, buffer).is_gt() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl std::fmt::Debug for BufferDiffInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferDiffSnapshot")
            .field("hunks", &self.hunks)
            .finish()
    }
}

impl BufferDiffSnapshot {
    pub fn is_empty(&self) -> bool {
        self.inner.hunks.is_empty()
    }

    pub fn secondary_diff(&self) -> Option<&BufferDiffSnapshot> {
        self.secondary_diff.as_deref()
    }

    pub fn hunks_intersecting_range<'a>(
        &'a self,
        range: Range<Anchor>,
        buffer: &'a text::BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiffHunk> {
        let unstaged_counterpart = self.secondary_diff.as_ref().map(|diff| &diff.inner);
        self.inner
            .hunks_intersecting_range(range, buffer, unstaged_counterpart)
    }

    pub fn hunks_intersecting_range_rev<'a>(
        &'a self,
        range: Range<Anchor>,
        buffer: &'a text::BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiffHunk> {
        self.inner.hunks_intersecting_range_rev(range, buffer)
    }

    pub fn base_text(&self) -> &language::BufferSnapshot {
        &self.inner.base_text
    }

    pub fn base_texts_eq(&self, other: &Self) -> bool {
        if self.inner.base_text_exists != other.inner.base_text_exists {
            return false;
        }
        let left = &self.inner.base_text;
        let right = &other.inner.base_text;
        let (old_id, old_empty) = (left.remote_id(), left.is_empty());
        let (new_id, new_empty) = (right.remote_id(), right.is_empty());
        new_id == old_id || (new_empty && old_empty)
    }
}

impl BufferDiffInner {
    fn stage_or_unstage_hunks(
        &mut self,
        unstaged_diff: &Self,
        stage: bool,
        hunks: &[DiffHunk],
        buffer: &text::BufferSnapshot,
        file_exists: bool,
    ) -> (Option<Rope>, Vec<(usize, PendingHunk)>) {
        let head_text = self
            .base_text_exists
            .then(|| self.base_text.as_rope().clone());
        let index_text = unstaged_diff
            .base_text_exists
            .then(|| unstaged_diff.base_text.as_rope().clone());

        // If the file doesn't exist in either HEAD or the index, then the
        // entire file must be either created or deleted in the index.
        let (index_text, head_text) = match (index_text, head_text) {
            (Some(index_text), Some(head_text)) if file_exists || !stage => (index_text, head_text),
            (_, head_text @ _) => {
                if stage {
                    log::debug!("stage all");
                    return (
                        file_exists.then(|| buffer.as_rope().clone()),
                        vec![(
                            0,
                            PendingHunk {
                                buffer_version: buffer.version().clone(),
                                new_status: DiffHunkSecondaryStatus::SecondaryHunkRemovalPending,
                            },
                        )],
                    );
                } else {
                    log::debug!("unstage all");
                    return (
                        head_text,
                        vec![(
                            0,
                            PendingHunk {
                                buffer_version: buffer.version().clone(),
                                new_status: DiffHunkSecondaryStatus::SecondaryHunkAdditionPending,
                            },
                        )],
                    );
                }
            }
        };

        let mut unstaged_hunk_cursor = unstaged_diff.hunks.cursor::<DiffHunkSummary>(buffer);
        unstaged_hunk_cursor.next(buffer);
        let mut edits = Vec::new();
        let mut pending_hunks = Vec::new();
        let mut prev_unstaged_hunk_buffer_offset = 0;
        let mut prev_unstaged_hunk_base_text_offset = 0;
        for DiffHunk {
            buffer_range,
            diff_base_byte_range,
            secondary_status,
            ..
        } in hunks.iter().cloned()
        {
            if (stage && secondary_status == DiffHunkSecondaryStatus::None)
                || (!stage && secondary_status == DiffHunkSecondaryStatus::HasSecondaryHunk)
            {
                continue;
            }

            let skipped_hunks = unstaged_hunk_cursor.slice(&buffer_range.start, Bias::Left, buffer);

            if let Some(secondary_hunk) = skipped_hunks.last() {
                prev_unstaged_hunk_base_text_offset = secondary_hunk.diff_base_byte_range.end;
                prev_unstaged_hunk_buffer_offset =
                    secondary_hunk.buffer_range.end.to_offset(buffer);
            }

            let mut buffer_offset_range = buffer_range.to_offset(buffer);
            let start_overshoot = buffer_offset_range.start - prev_unstaged_hunk_buffer_offset;
            let mut index_start = prev_unstaged_hunk_base_text_offset + start_overshoot;

            while let Some(unstaged_hunk) = unstaged_hunk_cursor.item().filter(|item| {
                item.buffer_range
                    .start
                    .cmp(&buffer_range.end, buffer)
                    .is_le()
            }) {
                let unstaged_hunk_offset_range = unstaged_hunk.buffer_range.to_offset(buffer);
                prev_unstaged_hunk_base_text_offset = unstaged_hunk.diff_base_byte_range.end;
                prev_unstaged_hunk_buffer_offset = unstaged_hunk_offset_range.end;

                index_start = index_start.min(unstaged_hunk.diff_base_byte_range.start);
                buffer_offset_range.start = buffer_offset_range
                    .start
                    .min(unstaged_hunk_offset_range.start);

                unstaged_hunk_cursor.next(buffer);
            }

            let end_overshoot = buffer_offset_range
                .end
                .saturating_sub(prev_unstaged_hunk_buffer_offset);
            let index_end = prev_unstaged_hunk_base_text_offset + end_overshoot;

            let index_range = index_start..index_end;
            buffer_offset_range.end = buffer_offset_range
                .end
                .max(prev_unstaged_hunk_buffer_offset);

            let replacement_text = if stage {
                log::debug!("stage hunk {:?}", buffer_offset_range);
                buffer
                    .text_for_range(buffer_offset_range)
                    .collect::<String>()
            } else {
                log::debug!("unstage hunk {:?}", buffer_offset_range);
                head_text
                    .chunks_in_range(diff_base_byte_range.clone())
                    .collect::<String>()
            };
            pending_hunks.push((
                diff_base_byte_range.start,
                PendingHunk {
                    buffer_version: buffer.version().clone(),
                    new_status: if stage {
                        DiffHunkSecondaryStatus::SecondaryHunkRemovalPending
                    } else {
                        DiffHunkSecondaryStatus::SecondaryHunkAdditionPending
                    },
                },
            ));
            edits.push((index_range, replacement_text));
        }

        let mut new_index_text = Rope::new();
        let mut index_cursor = index_text.cursor(0);
        for (old_range, replacement_text) in edits {
            new_index_text.append(index_cursor.slice(old_range.start));
            index_cursor.seek_forward(old_range.end);
            new_index_text.push(&replacement_text);
        }
        new_index_text.append(index_cursor.suffix());
        (Some(new_index_text), pending_hunks)
    }

    fn hunks_intersecting_range<'a>(
        &'a self,
        range: Range<Anchor>,
        buffer: &'a text::BufferSnapshot,
        secondary: Option<&'a Self>,
    ) -> impl 'a + Iterator<Item = DiffHunk> {
        let range = range.to_offset(buffer);

        let mut cursor = self
            .hunks
            .filter::<_, DiffHunkSummary>(buffer, move |summary| {
                let summary_range = summary.buffer_range.to_offset(buffer);
                let before_start = summary_range.end < range.start;
                let after_end = summary_range.start > range.end;
                !before_start && !after_end
            });

        let anchor_iter = iter::from_fn(move || {
            cursor.next(buffer);
            cursor.item()
        })
        .flat_map(move |hunk| {
            [
                (
                    &hunk.buffer_range.start,
                    (hunk.buffer_range.start, hunk.diff_base_byte_range.start),
                ),
                (
                    &hunk.buffer_range.end,
                    (hunk.buffer_range.end, hunk.diff_base_byte_range.end),
                ),
            ]
        });

        let mut secondary_cursor = None;
        let mut pending_hunks = TreeMap::default();
        if let Some(secondary) = secondary.as_ref() {
            let mut cursor = secondary.hunks.cursor::<DiffHunkSummary>(buffer);
            cursor.next(buffer);
            secondary_cursor = Some(cursor);
            pending_hunks = secondary.pending_hunks.clone();
        }

        let max_point = buffer.max_point();
        let mut summaries = buffer.summaries_for_anchors_with_payload::<Point, _, _>(anchor_iter);
        iter::from_fn(move || loop {
            let (start_point, (start_anchor, start_base)) = summaries.next()?;
            let (mut end_point, (mut end_anchor, end_base)) = summaries.next()?;

            if !start_anchor.is_valid(buffer) {
                continue;
            }

            if end_point.column > 0 && end_point < max_point {
                end_point.row += 1;
                end_point.column = 0;
                end_anchor = buffer.anchor_before(end_point);
            }

            let mut secondary_status = DiffHunkSecondaryStatus::None;

            let mut has_pending = false;
            if let Some(pending_hunk) = pending_hunks.get(&start_base) {
                if !buffer.has_edits_since_in_range(
                    &pending_hunk.buffer_version,
                    start_anchor..end_anchor,
                ) {
                    has_pending = true;
                    secondary_status = pending_hunk.new_status;
                }
            }

            if let (Some(secondary_cursor), false) = (secondary_cursor.as_mut(), has_pending) {
                if start_anchor
                    .cmp(&secondary_cursor.start().buffer_range.start, buffer)
                    .is_gt()
                {
                    secondary_cursor.seek_forward(&start_anchor, Bias::Left, buffer);
                }

                if let Some(secondary_hunk) = secondary_cursor.item() {
                    let mut secondary_range = secondary_hunk.buffer_range.to_point(buffer);
                    if secondary_range.end.column > 0 {
                        secondary_range.end.row += 1;
                        secondary_range.end.column = 0;
                    }
                    if secondary_range.is_empty() && secondary_hunk.diff_base_byte_range.is_empty()
                    {
                        // ignore
                    } else if secondary_range == (start_point..end_point) {
                        secondary_status = DiffHunkSecondaryStatus::HasSecondaryHunk;
                    } else if secondary_range.start <= end_point {
                        secondary_status = DiffHunkSecondaryStatus::OverlapsWithSecondaryHunk;
                    }
                }
            }

            return Some(DiffHunk {
                range: start_point..end_point,
                diff_base_byte_range: start_base..end_base,
                buffer_range: start_anchor..end_anchor,
                secondary_status,
            });
        })
    }

    fn hunks_intersecting_range_rev<'a>(
        &'a self,
        range: Range<Anchor>,
        buffer: &'a text::BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiffHunk> {
        let mut cursor = self
            .hunks
            .filter::<_, DiffHunkSummary>(buffer, move |summary| {
                let before_start = summary.buffer_range.end.cmp(&range.start, buffer).is_lt();
                let after_end = summary.buffer_range.start.cmp(&range.end, buffer).is_gt();
                !before_start && !after_end
            });

        iter::from_fn(move || {
            cursor.prev(buffer);

            let hunk = cursor.item()?;
            let range = hunk.buffer_range.to_point(buffer);

            Some(DiffHunk {
                range,
                diff_base_byte_range: hunk.diff_base_byte_range.clone(),
                buffer_range: hunk.buffer_range.clone(),
                // The secondary status is not used by callers of this method.
                secondary_status: DiffHunkSecondaryStatus::None,
            })
        })
    }

    fn compare(&self, old: &Self, new_snapshot: &text::BufferSnapshot) -> Option<Range<Anchor>> {
        let mut new_cursor = self.hunks.cursor::<()>(new_snapshot);
        let mut old_cursor = old.hunks.cursor::<()>(new_snapshot);
        old_cursor.next(new_snapshot);
        new_cursor.next(new_snapshot);
        let mut start = None;
        let mut end = None;

        loop {
            match (new_cursor.item(), old_cursor.item()) {
                (Some(new_hunk), Some(old_hunk)) => {
                    match new_hunk
                        .buffer_range
                        .start
                        .cmp(&old_hunk.buffer_range.start, new_snapshot)
                    {
                        Ordering::Less => {
                            start.get_or_insert(new_hunk.buffer_range.start);
                            end.replace(new_hunk.buffer_range.end);
                            new_cursor.next(new_snapshot);
                        }
                        Ordering::Equal => {
                            if new_hunk != old_hunk {
                                start.get_or_insert(new_hunk.buffer_range.start);
                                if old_hunk
                                    .buffer_range
                                    .end
                                    .cmp(&new_hunk.buffer_range.end, new_snapshot)
                                    .is_ge()
                                {
                                    end.replace(old_hunk.buffer_range.end);
                                } else {
                                    end.replace(new_hunk.buffer_range.end);
                                }
                            }

                            new_cursor.next(new_snapshot);
                            old_cursor.next(new_snapshot);
                        }
                        Ordering::Greater => {
                            start.get_or_insert(old_hunk.buffer_range.start);
                            end.replace(old_hunk.buffer_range.end);
                            old_cursor.next(new_snapshot);
                        }
                    }
                }
                (Some(new_hunk), None) => {
                    start.get_or_insert(new_hunk.buffer_range.start);
                    end.replace(new_hunk.buffer_range.end);
                    new_cursor.next(new_snapshot);
                }
                (None, Some(old_hunk)) => {
                    start.get_or_insert(old_hunk.buffer_range.start);
                    end.replace(old_hunk.buffer_range.end);
                    old_cursor.next(new_snapshot);
                }
                (None, None) => break,
            }
        }

        start.zip(end).map(|(start, end)| start..end)
    }
}

fn compute_hunks(
    diff_base: Option<(Arc<String>, Rope)>,
    buffer: text::BufferSnapshot,
) -> SumTree<InternalDiffHunk> {
    let mut tree = SumTree::new(&buffer);

    if let Some((diff_base, diff_base_rope)) = diff_base {
        let buffer_text = buffer.as_rope().to_string();

        let mut options = GitOptions::default();
        options.context_lines(0);
        let patch = GitPatch::from_buffers(
            diff_base.as_bytes(),
            None,
            buffer_text.as_bytes(),
            None,
            Some(&mut options),
        )
        .log_err();

        // A common case in Zed is that the empty buffer is represented as just a newline,
        // but if we just compute a naive diff you get a "preserved" line in the middle,
        // which is a bit odd.
        if buffer_text == "\n" && diff_base.ends_with("\n") && diff_base.len() > 1 {
            tree.push(
                InternalDiffHunk {
                    buffer_range: buffer.anchor_before(0)..buffer.anchor_before(0),
                    diff_base_byte_range: 0..diff_base.len() - 1,
                },
                &buffer,
            );
            return tree;
        }

        if let Some(patch) = patch {
            let mut divergence = 0;
            for hunk_index in 0..patch.num_hunks() {
                let hunk = process_patch_hunk(
                    &patch,
                    hunk_index,
                    &diff_base_rope,
                    &buffer,
                    &mut divergence,
                );
                tree.push(hunk, &buffer);
            }
        }
    } else {
        tree.push(
            InternalDiffHunk {
                buffer_range: Anchor::MIN..Anchor::MAX,
                diff_base_byte_range: 0..0,
            },
            &buffer,
        );
    }

    tree
}

fn process_patch_hunk(
    patch: &GitPatch<'_>,
    hunk_index: usize,
    diff_base: &Rope,
    buffer: &text::BufferSnapshot,
    buffer_row_divergence: &mut i64,
) -> InternalDiffHunk {
    let line_item_count = patch.num_lines_in_hunk(hunk_index).unwrap();
    assert!(line_item_count > 0);

    let mut first_deletion_buffer_row: Option<u32> = None;
    let mut buffer_row_range: Option<Range<u32>> = None;
    let mut diff_base_byte_range: Option<Range<usize>> = None;
    let mut first_addition_old_row: Option<u32> = None;

    for line_index in 0..line_item_count {
        let line = patch.line_in_hunk(hunk_index, line_index).unwrap();
        let kind = line.origin_value();
        let content_offset = line.content_offset() as isize;
        let content_len = line.content().len() as isize;
        match kind {
            GitDiffLineType::Addition => {
                if first_addition_old_row.is_none() {
                    first_addition_old_row = Some(
                        (line.new_lineno().unwrap() as i64 - *buffer_row_divergence - 1) as u32,
                    );
                }
                *buffer_row_divergence += 1;
                let row = line.new_lineno().unwrap().saturating_sub(1);

                match &mut buffer_row_range {
                    Some(Range { end, .. }) => *end = row + 1,
                    None => buffer_row_range = Some(row..row + 1),
                }
            }
            GitDiffLineType::Deletion => {
                let end = content_offset + content_len;

                match &mut diff_base_byte_range {
                    Some(head_byte_range) => head_byte_range.end = end as usize,
                    None => diff_base_byte_range = Some(content_offset as usize..end as usize),
                }

                if first_deletion_buffer_row.is_none() {
                    let old_row = line.old_lineno().unwrap().saturating_sub(1);
                    let row = old_row as i64 + *buffer_row_divergence;
                    first_deletion_buffer_row = Some(row as u32);
                }

                *buffer_row_divergence -= 1;
            }
            _ => {}
        }
    }

    let buffer_row_range = buffer_row_range.unwrap_or_else(|| {
        // Pure deletion hunk without addition.
        let row = first_deletion_buffer_row.unwrap();
        row..row
    });
    let diff_base_byte_range = diff_base_byte_range.unwrap_or_else(|| {
        // Pure addition hunk without deletion.
        let row = first_addition_old_row.unwrap();
        let offset = diff_base.point_to_offset(Point::new(row, 0));
        offset..offset
    });

    let start = Point::new(buffer_row_range.start, 0);
    let end = Point::new(buffer_row_range.end, 0);
    let buffer_range = buffer.anchor_before(start)..buffer.anchor_before(end);
    InternalDiffHunk {
        buffer_range,
        diff_base_byte_range,
    }
}

impl std::fmt::Debug for BufferDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferChangeSet")
            .field("buffer_id", &self.buffer_id)
            .field("snapshot", &self.inner)
            .finish()
    }
}

pub enum BufferDiffEvent {
    DiffChanged {
        changed_range: Option<Range<text::Anchor>>,
    },
    LanguageChanged,
}

impl EventEmitter<BufferDiffEvent> for BufferDiff {}

impl BufferDiff {
    #[cfg(test)]
    fn build_sync(
        buffer: text::BufferSnapshot,
        diff_base: String,
        cx: &mut gpui::TestAppContext,
    ) -> BufferDiffInner {
        let snapshot =
            cx.update(|cx| Self::build(buffer, Some(Arc::new(diff_base)), None, None, cx));
        cx.executor().block(snapshot)
    }

    fn build(
        buffer: text::BufferSnapshot,
        base_text: Option<Arc<String>>,
        language: Option<Arc<Language>>,
        language_registry: Option<Arc<LanguageRegistry>>,
        cx: &mut App,
    ) -> impl Future<Output = BufferDiffInner> {
        let base_text_pair;
        let base_text_exists;
        let base_text_snapshot;
        if let Some(text) = &base_text {
            let base_text_rope = Rope::from(text.as_str());
            base_text_pair = Some((text.clone(), base_text_rope.clone()));
            let snapshot = language::Buffer::build_snapshot(
                base_text_rope,
                language.clone(),
                language_registry.clone(),
                cx,
            );
            base_text_snapshot = cx.background_spawn(snapshot);
            base_text_exists = true;
        } else {
            base_text_pair = None;
            base_text_snapshot = Task::ready(language::Buffer::build_empty_snapshot(cx));
            base_text_exists = false;
        };

        let hunks = cx.background_spawn({
            let buffer = buffer.clone();
            async move { compute_hunks(base_text_pair, buffer) }
        });

        async move {
            let (base_text, hunks) = futures::join!(base_text_snapshot, hunks);
            BufferDiffInner {
                base_text,
                hunks,
                base_text_exists,
                pending_hunks: TreeMap::default(),
            }
        }
    }

    fn build_with_base_buffer(
        buffer: text::BufferSnapshot,
        base_text: Option<Arc<String>>,
        base_text_snapshot: language::BufferSnapshot,
        cx: &App,
    ) -> impl Future<Output = BufferDiffInner> {
        let base_text_exists = base_text.is_some();
        let base_text_pair = base_text.map(|text| (text, base_text_snapshot.as_rope().clone()));
        cx.background_spawn(async move {
            BufferDiffInner {
                base_text: base_text_snapshot,
                hunks: compute_hunks(base_text_pair, buffer),
                pending_hunks: TreeMap::default(),
                base_text_exists,
            }
        })
    }

    fn build_empty(buffer: &text::BufferSnapshot, cx: &mut App) -> BufferDiffInner {
        BufferDiffInner {
            base_text: language::Buffer::build_empty_snapshot(cx),
            hunks: SumTree::new(buffer),
            pending_hunks: TreeMap::default(),
            base_text_exists: false,
        }
    }

    pub fn set_secondary_diff(&mut self, diff: Entity<BufferDiff>) {
        self.secondary_diff = Some(diff);
    }

    pub fn secondary_diff(&self) -> Option<Entity<BufferDiff>> {
        self.secondary_diff.clone()
    }

    pub fn stage_or_unstage_hunks(
        &mut self,
        stage: bool,
        hunks: &[DiffHunk],
        buffer: &text::BufferSnapshot,
        file_exists: bool,
        cx: &mut Context<Self>,
    ) -> Option<Rope> {
        let (new_index_text, pending_hunks) = self.inner.stage_or_unstage_hunks(
            &self.secondary_diff.as_ref()?.read(cx).inner,
            stage,
            &hunks,
            buffer,
            file_exists,
        );
        if let Some(unstaged_diff) = &self.secondary_diff {
            unstaged_diff.update(cx, |diff, _| {
                for (offset, pending_hunk) in pending_hunks {
                    diff.inner.pending_hunks.insert(offset, pending_hunk);
                }
            });
        }
        if let Some((first, last)) = hunks.first().zip(hunks.last()) {
            let changed_range = first.buffer_range.start..last.buffer_range.end;
            cx.emit(BufferDiffEvent::DiffChanged {
                changed_range: Some(changed_range),
            });
        }
        new_index_text
    }

    pub fn range_to_hunk_range(
        &self,
        range: Range<Anchor>,
        buffer: &text::BufferSnapshot,
        cx: &App,
    ) -> Option<Range<Anchor>> {
        let start = self
            .hunks_intersecting_range(range.clone(), &buffer, cx)
            .next()?
            .buffer_range
            .start;
        let end = self
            .hunks_intersecting_range_rev(range.clone(), &buffer)
            .next()?
            .buffer_range
            .end;
        Some(start..end)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_diff(
        this: Entity<BufferDiff>,
        buffer: text::BufferSnapshot,
        base_text: Option<Arc<String>>,
        base_text_changed: bool,
        language_changed: bool,
        language: Option<Arc<Language>>,
        language_registry: Option<Arc<LanguageRegistry>>,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<Option<Range<Anchor>>> {
        let snapshot = if base_text_changed || language_changed {
            cx.update(|cx| {
                Self::build(
                    buffer.clone(),
                    base_text,
                    language.clone(),
                    language_registry.clone(),
                    cx,
                )
            })?
            .await
        } else {
            this.read_with(cx, |this, cx| {
                Self::build_with_base_buffer(
                    buffer.clone(),
                    base_text,
                    this.base_text().clone(),
                    cx,
                )
            })?
            .await
        };

        this.update(cx, |this, _| this.set_state(snapshot, &buffer))
    }

    pub fn update_diff_from(
        &mut self,
        buffer: &text::BufferSnapshot,
        other: &Entity<Self>,
        cx: &mut Context<Self>,
    ) -> Option<Range<Anchor>> {
        let other = other.read(cx).inner.clone();
        self.set_state(other, buffer)
    }

    fn set_state(
        &mut self,
        new_state: BufferDiffInner,
        buffer: &text::BufferSnapshot,
    ) -> Option<Range<Anchor>> {
        let (base_text_changed, changed_range) =
            match (self.inner.base_text_exists, new_state.base_text_exists) {
                (false, false) => (true, None),
                (true, true)
                    if self.inner.base_text.remote_id() == new_state.base_text.remote_id() =>
                {
                    (false, new_state.compare(&self.inner, buffer))
                }
                _ => (true, Some(text::Anchor::MIN..text::Anchor::MAX)),
            };
        let pending_hunks = mem::take(&mut self.inner.pending_hunks);
        self.inner = new_state;
        if !base_text_changed {
            self.inner.pending_hunks = pending_hunks;
        }
        changed_range
    }

    pub fn base_text(&self) -> &language::BufferSnapshot {
        &self.inner.base_text
    }

    pub fn base_text_exists(&self) -> bool {
        self.inner.base_text_exists
    }

    pub fn snapshot(&self, cx: &App) -> BufferDiffSnapshot {
        BufferDiffSnapshot {
            inner: self.inner.clone(),
            secondary_diff: self
                .secondary_diff
                .as_ref()
                .map(|diff| Box::new(diff.read(cx).snapshot(cx))),
        }
    }

    pub fn hunks_intersecting_range<'a>(
        &'a self,
        range: Range<text::Anchor>,
        buffer_snapshot: &'a text::BufferSnapshot,
        cx: &'a App,
    ) -> impl 'a + Iterator<Item = DiffHunk> {
        let unstaged_counterpart = self
            .secondary_diff
            .as_ref()
            .map(|diff| &diff.read(cx).inner);
        self.inner
            .hunks_intersecting_range(range, buffer_snapshot, unstaged_counterpart)
    }

    pub fn hunks_intersecting_range_rev<'a>(
        &'a self,
        range: Range<text::Anchor>,
        buffer_snapshot: &'a text::BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiffHunk> {
        self.inner
            .hunks_intersecting_range_rev(range, buffer_snapshot)
    }

    pub fn hunks_in_row_range<'a>(
        &'a self,
        range: Range<u32>,
        buffer: &'a text::BufferSnapshot,
        cx: &'a App,
    ) -> impl 'a + Iterator<Item = DiffHunk> {
        let start = buffer.anchor_before(Point::new(range.start, 0));
        let end = buffer.anchor_after(Point::new(range.end, 0));
        self.hunks_intersecting_range(start..end, buffer, cx)
    }

    /// Used in cases where the change set isn't derived from git.
    pub fn set_base_text(
        &mut self,
        base_buffer: Entity<language::Buffer>,
        buffer: text::BufferSnapshot,
        cx: &mut Context<Self>,
    ) -> oneshot::Receiver<()> {
        let (tx, rx) = oneshot::channel();
        let this = cx.weak_entity();
        let base_buffer = base_buffer.read(cx);
        let language_registry = base_buffer.language_registry();
        let base_buffer = base_buffer.snapshot();
        let base_text = Arc::new(base_buffer.text());

        let snapshot = BufferDiff::build(
            buffer.clone(),
            Some(base_text),
            base_buffer.language().cloned(),
            language_registry,
            cx,
        );
        let complete_on_drop = util::defer(|| {
            tx.send(()).ok();
        });
        cx.spawn(|_, mut cx| async move {
            let snapshot = snapshot.await;
            let Some(this) = this.upgrade() else {
                return;
            };
            this.update(&mut cx, |this, _| {
                this.set_state(snapshot, &buffer);
            })
            .log_err();
            drop(complete_on_drop)
        })
        .detach();
        rx
    }

    pub fn base_text_string(&self) -> Option<String> {
        self.inner
            .base_text_exists
            .then(|| self.inner.base_text.text())
    }

    pub fn new(buffer: &text::BufferSnapshot, cx: &mut App) -> Self {
        BufferDiff {
            buffer_id: buffer.remote_id(),
            inner: BufferDiff::build_empty(buffer, cx),
            secondary_diff: None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn new_with_base_text(
        base_text: &str,
        buffer: &Entity<language::Buffer>,
        cx: &mut App,
    ) -> Self {
        let mut base_text = base_text.to_owned();
        text::LineEnding::normalize(&mut base_text);
        let snapshot = BufferDiff::build(
            buffer.read(cx).text_snapshot(),
            Some(base_text.into()),
            None,
            None,
            cx,
        );
        let snapshot = cx.background_executor().block(snapshot);
        BufferDiff {
            buffer_id: buffer.read(cx).remote_id(),
            inner: snapshot,
            secondary_diff: None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn recalculate_diff_sync(&mut self, buffer: text::BufferSnapshot, cx: &mut Context<Self>) {
        let base_text = self.base_text_string().map(Arc::new);
        let snapshot = BufferDiff::build_with_base_buffer(
            buffer.clone(),
            base_text,
            self.inner.base_text.clone(),
            cx,
        );
        let snapshot = cx.background_executor().block(snapshot);
        let changed_range = self.set_state(snapshot, &buffer);
        cx.emit(BufferDiffEvent::DiffChanged { changed_range });
    }
}

impl DiffHunk {
    pub fn is_created_file(&self) -> bool {
        self.diff_base_byte_range == (0..0) && self.buffer_range == (Anchor::MIN..Anchor::MAX)
    }

    pub fn status(&self) -> DiffHunkStatus {
        let kind = if self.buffer_range.start == self.buffer_range.end {
            DiffHunkStatusKind::Deleted
        } else if self.diff_base_byte_range.is_empty() {
            DiffHunkStatusKind::Added
        } else {
            DiffHunkStatusKind::Modified
        };
        DiffHunkStatus {
            kind,
            secondary: self.secondary_status,
        }
    }
}

impl DiffHunkStatus {
    pub fn has_secondary_hunk(&self) -> bool {
        matches!(
            self.secondary,
            DiffHunkSecondaryStatus::HasSecondaryHunk
                | DiffHunkSecondaryStatus::SecondaryHunkAdditionPending
                | DiffHunkSecondaryStatus::OverlapsWithSecondaryHunk
        )
    }

    pub fn is_pending(&self) -> bool {
        matches!(
            self.secondary,
            DiffHunkSecondaryStatus::SecondaryHunkAdditionPending
                | DiffHunkSecondaryStatus::SecondaryHunkRemovalPending
        )
    }

    pub fn is_deleted(&self) -> bool {
        self.kind == DiffHunkStatusKind::Deleted
    }

    pub fn is_added(&self) -> bool {
        self.kind == DiffHunkStatusKind::Added
    }

    pub fn is_modified(&self) -> bool {
        self.kind == DiffHunkStatusKind::Modified
    }

    pub fn added(secondary: DiffHunkSecondaryStatus) -> Self {
        Self {
            kind: DiffHunkStatusKind::Added,
            secondary,
        }
    }

    pub fn modified(secondary: DiffHunkSecondaryStatus) -> Self {
        Self {
            kind: DiffHunkStatusKind::Modified,
            secondary,
        }
    }

    pub fn deleted(secondary: DiffHunkSecondaryStatus) -> Self {
        Self {
            kind: DiffHunkStatusKind::Deleted,
            secondary,
        }
    }

    pub fn deleted_none() -> Self {
        Self {
            kind: DiffHunkStatusKind::Deleted,
            secondary: DiffHunkSecondaryStatus::None,
        }
    }

    pub fn added_none() -> Self {
        Self {
            kind: DiffHunkStatusKind::Added,
            secondary: DiffHunkSecondaryStatus::None,
        }
    }

    pub fn modified_none() -> Self {
        Self {
            kind: DiffHunkStatusKind::Modified,
            secondary: DiffHunkSecondaryStatus::None,
        }
    }
}

/// Range (crossing new lines), old, new
#[cfg(any(test, feature = "test-support"))]
#[track_caller]
pub fn assert_hunks<Iter>(
    diff_hunks: Iter,
    buffer: &text::BufferSnapshot,
    diff_base: &str,
    expected_hunks: &[(Range<u32>, &str, &str, DiffHunkStatus)],
) where
    Iter: Iterator<Item = DiffHunk>,
{
    let actual_hunks = diff_hunks
        .map(|hunk| {
            (
                hunk.range.clone(),
                &diff_base[hunk.diff_base_byte_range.clone()],
                buffer
                    .text_for_range(hunk.range.clone())
                    .collect::<String>(),
                hunk.status(),
            )
        })
        .collect::<Vec<_>>();

    let expected_hunks: Vec<_> = expected_hunks
        .iter()
        .map(|(r, old_text, new_text, status)| {
            (
                Point::new(r.start, 0)..Point::new(r.end, 0),
                *old_text,
                new_text.to_string(),
                *status,
            )
        })
        .collect();

    assert_eq!(actual_hunks, expected_hunks);
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;

    use super::*;
    use gpui::TestAppContext;
    use rand::{rngs::StdRng, Rng as _};
    use text::{Buffer, BufferId, Rope};
    use unindent::Unindent as _;
    use util::test::marked_text_ranges;

    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }

    #[gpui::test]
    async fn test_buffer_diff_simple(cx: &mut gpui::TestAppContext) {
        let diff_base = "
            one
            two
            three
        "
        .unindent();

        let buffer_text = "
            one
            HELLO
            three
        "
        .unindent();

        let mut buffer = Buffer::new(0, BufferId::new(1).unwrap(), buffer_text);
        let mut diff = BufferDiff::build_sync(buffer.clone(), diff_base.clone(), cx);
        assert_hunks(
            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &buffer, None),
            &buffer,
            &diff_base,
            &[(1..2, "two\n", "HELLO\n", DiffHunkStatus::modified_none())],
        );

        buffer.edit([(0..0, "point five\n")]);
        diff = BufferDiff::build_sync(buffer.clone(), diff_base.clone(), cx);
        assert_hunks(
            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &buffer, None),
            &buffer,
            &diff_base,
            &[
                (0..1, "", "point five\n", DiffHunkStatus::added_none()),
                (2..3, "two\n", "HELLO\n", DiffHunkStatus::modified_none()),
            ],
        );

        diff = cx.update(|cx| BufferDiff::build_empty(&buffer, cx));
        assert_hunks(
            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &buffer, None),
            &buffer,
            &diff_base,
            &[],
        );
    }

    #[gpui::test]
    async fn test_buffer_diff_with_secondary(cx: &mut gpui::TestAppContext) {
        let head_text = "
            zero
            one
            two
            three
            four
            five
            six
            seven
            eight
            nine
        "
        .unindent();

        let index_text = "
            zero
            one
            TWO
            three
            FOUR
            five
            six
            seven
            eight
            NINE
        "
        .unindent();

        let buffer_text = "
            zero
            one
            TWO
            three
            FOUR
            FIVE
            six
            SEVEN
            eight
            nine
        "
        .unindent();

        let buffer = Buffer::new(0, BufferId::new(1).unwrap(), buffer_text);
        let unstaged_diff = BufferDiff::build_sync(buffer.clone(), index_text.clone(), cx);

        let uncommitted_diff = BufferDiff::build_sync(buffer.clone(), head_text.clone(), cx);

        let expected_hunks = vec![
            (2..3, "two\n", "TWO\n", DiffHunkStatus::modified_none()),
            (
                4..6,
                "four\nfive\n",
                "FOUR\nFIVE\n",
                DiffHunkStatus::modified(DiffHunkSecondaryStatus::OverlapsWithSecondaryHunk),
            ),
            (
                7..8,
                "seven\n",
                "SEVEN\n",
                DiffHunkStatus::modified(DiffHunkSecondaryStatus::HasSecondaryHunk),
            ),
        ];

        assert_hunks(
            uncommitted_diff.hunks_intersecting_range(
                Anchor::MIN..Anchor::MAX,
                &buffer,
                Some(&unstaged_diff),
            ),
            &buffer,
            &head_text,
            &expected_hunks,
        );
    }

    #[gpui::test]
    async fn test_buffer_diff_range(cx: &mut TestAppContext) {
        let diff_base = Arc::new(
            "
            one
            two
            three
            four
            five
            six
            seven
            eight
            nine
            ten
        "
            .unindent(),
        );

        let buffer_text = "
            A
            one
            B
            two
            C
            three
            HELLO
            four
            five
            SIXTEEN
            seven
            eight
            WORLD
            nine

            ten

        "
        .unindent();

        let buffer = Buffer::new(0, BufferId::new(1).unwrap(), buffer_text);
        let diff = cx
            .update(|cx| {
                BufferDiff::build(buffer.snapshot(), Some(diff_base.clone()), None, None, cx)
            })
            .await;
        assert_eq!(
            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &buffer, None)
                .count(),
            8
        );

        assert_hunks(
            diff.hunks_intersecting_range(
                buffer.anchor_before(Point::new(7, 0))..buffer.anchor_before(Point::new(12, 0)),
                &buffer,
                None,
            ),
            &buffer,
            &diff_base,
            &[
                (6..7, "", "HELLO\n", DiffHunkStatus::added_none()),
                (9..10, "six\n", "SIXTEEN\n", DiffHunkStatus::modified_none()),
                (12..13, "", "WORLD\n", DiffHunkStatus::added_none()),
            ],
        );
    }

    #[gpui::test]
    async fn test_stage_hunk(cx: &mut TestAppContext) {
        struct Example {
            name: &'static str,
            head_text: String,
            index_text: String,
            buffer_marked_text: String,
            final_index_text: String,
        }

        let table = [
            Example {
                name: "uncommitted hunk straddles end of unstaged hunk",
                head_text: "
                    one
                    two
                    three
                    four
                    five
                "
                .unindent(),
                index_text: "
                    one
                    TWO_HUNDRED
                    three
                    FOUR_HUNDRED
                    five
                "
                .unindent(),
                buffer_marked_text: "
                    ZERO
                    one
                    two
                    «THREE_HUNDRED
                    FOUR_HUNDRED»
                    five
                    SIX
                "
                .unindent(),
                final_index_text: "
                    one
                    two
                    THREE_HUNDRED
                    FOUR_HUNDRED
                    five
                "
                .unindent(),
            },
            Example {
                name: "uncommitted hunk straddles start of unstaged hunk",
                head_text: "
                    one
                    two
                    three
                    four
                    five
                "
                .unindent(),
                index_text: "
                    one
                    TWO_HUNDRED
                    three
                    FOUR_HUNDRED
                    five
                "
                .unindent(),
                buffer_marked_text: "
                    ZERO
                    one
                    «TWO_HUNDRED
                    THREE_HUNDRED»
                    four
                    five
                    SIX
                "
                .unindent(),
                final_index_text: "
                    one
                    TWO_HUNDRED
                    THREE_HUNDRED
                    four
                    five
                "
                .unindent(),
            },
            Example {
                name: "uncommitted hunk strictly contains unstaged hunks",
                head_text: "
                    one
                    two
                    three
                    four
                    five
                    six
                    seven
                "
                .unindent(),
                index_text: "
                    one
                    TWO
                    THREE
                    FOUR
                    FIVE
                    SIX
                    seven
                "
                .unindent(),
                buffer_marked_text: "
                    one
                    TWO
                    «THREE_HUNDRED
                    FOUR
                    FIVE_HUNDRED»
                    SIX
                    seven
                "
                .unindent(),
                final_index_text: "
                    one
                    TWO
                    THREE_HUNDRED
                    FOUR
                    FIVE_HUNDRED
                    SIX
                    seven
                "
                .unindent(),
            },
            Example {
                name: "uncommitted deletion hunk",
                head_text: "
                    one
                    two
                    three
                    four
                    five
                "
                .unindent(),
                index_text: "
                    one
                    two
                    three
                    four
                    five
                "
                .unindent(),
                buffer_marked_text: "
                    one
                    ˇfive
                "
                .unindent(),
                final_index_text: "
                    one
                    five
                "
                .unindent(),
            },
        ];

        for example in table {
            let (buffer_text, ranges) = marked_text_ranges(&example.buffer_marked_text, false);
            let buffer = Buffer::new(0, BufferId::new(1).unwrap(), buffer_text);
            let hunk_range =
                buffer.anchor_before(ranges[0].start)..buffer.anchor_before(ranges[0].end);

            let unstaged = BufferDiff::build_sync(buffer.clone(), example.index_text.clone(), cx);
            let uncommitted = BufferDiff::build_sync(buffer.clone(), example.head_text.clone(), cx);

            let unstaged_diff = cx.new(|cx| {
                let mut diff = BufferDiff::new(&buffer, cx);
                diff.set_state(unstaged, &buffer);
                diff
            });

            let uncommitted_diff = cx.new(|cx| {
                let mut diff = BufferDiff::new(&buffer, cx);
                diff.set_state(uncommitted, &buffer);
                diff.set_secondary_diff(unstaged_diff);
                diff
            });

            uncommitted_diff.update(cx, |diff, cx| {
                let hunks = diff
                    .hunks_intersecting_range(hunk_range.clone(), &buffer, &cx)
                    .collect::<Vec<_>>();
                for hunk in &hunks {
                    assert_ne!(hunk.secondary_status, DiffHunkSecondaryStatus::None)
                }

                let new_index_text = diff
                    .stage_or_unstage_hunks(true, &hunks, &buffer, true, cx)
                    .unwrap()
                    .to_string();

                let hunks = diff
                    .hunks_intersecting_range(hunk_range.clone(), &buffer, &cx)
                    .collect::<Vec<_>>();
                for hunk in &hunks {
                    assert_eq!(
                        hunk.secondary_status,
                        DiffHunkSecondaryStatus::SecondaryHunkRemovalPending
                    )
                }

                pretty_assertions::assert_eq!(
                    new_index_text,
                    example.final_index_text,
                    "example: {}",
                    example.name
                );
            });
        }
    }

    #[gpui::test]
    async fn test_buffer_diff_compare(cx: &mut TestAppContext) {
        let base_text = "
            zero
            one
            two
            three
            four
            five
            six
            seven
            eight
            nine
        "
        .unindent();

        let buffer_text_1 = "
            one
            three
            four
            five
            SIX
            seven
            eight
            NINE
        "
        .unindent();

        let mut buffer = Buffer::new(0, BufferId::new(1).unwrap(), buffer_text_1);

        let empty_diff = cx.update(|cx| BufferDiff::build_empty(&buffer, cx));
        let diff_1 = BufferDiff::build_sync(buffer.clone(), base_text.clone(), cx);
        let range = diff_1.compare(&empty_diff, &buffer).unwrap();
        assert_eq!(range.to_point(&buffer), Point::new(0, 0)..Point::new(8, 0));

        // Edit does not affect the diff.
        buffer.edit_via_marked_text(
            &"
                one
                three
                four
                five
                «SIX.5»
                seven
                eight
                NINE
            "
            .unindent(),
        );
        let diff_2 = BufferDiff::build_sync(buffer.clone(), base_text.clone(), cx);
        assert_eq!(None, diff_2.compare(&diff_1, &buffer));

        // Edit turns a deletion hunk into a modification.
        buffer.edit_via_marked_text(
            &"
                one
                «THREE»
                four
                five
                SIX.5
                seven
                eight
                NINE
            "
            .unindent(),
        );
        let diff_3 = BufferDiff::build_sync(buffer.clone(), base_text.clone(), cx);
        let range = diff_3.compare(&diff_2, &buffer).unwrap();
        assert_eq!(range.to_point(&buffer), Point::new(1, 0)..Point::new(2, 0));

        // Edit turns a modification hunk into a deletion.
        buffer.edit_via_marked_text(
            &"
                one
                THREE
                four
                five«»
                seven
                eight
                NINE
            "
            .unindent(),
        );
        let diff_4 = BufferDiff::build_sync(buffer.clone(), base_text.clone(), cx);
        let range = diff_4.compare(&diff_3, &buffer).unwrap();
        assert_eq!(range.to_point(&buffer), Point::new(3, 4)..Point::new(4, 0));

        // Edit introduces a new insertion hunk.
        buffer.edit_via_marked_text(
            &"
                one
                THREE
                four«
                FOUR.5
                »five
                seven
                eight
                NINE
            "
            .unindent(),
        );
        let diff_5 = BufferDiff::build_sync(buffer.snapshot(), base_text.clone(), cx);
        let range = diff_5.compare(&diff_4, &buffer).unwrap();
        assert_eq!(range.to_point(&buffer), Point::new(3, 0)..Point::new(4, 0));

        // Edit removes a hunk.
        buffer.edit_via_marked_text(
            &"
                one
                THREE
                four
                FOUR.5
                five
                seven
                eight
                «nine»
            "
            .unindent(),
        );
        let diff_6 = BufferDiff::build_sync(buffer.snapshot(), base_text, cx);
        let range = diff_6.compare(&diff_5, &buffer).unwrap();
        assert_eq!(range.to_point(&buffer), Point::new(7, 0)..Point::new(8, 0));
    }

    #[gpui::test(iterations = 100)]
    async fn test_staging_and_unstaging_hunks(cx: &mut TestAppContext, mut rng: StdRng) {
        fn gen_line(rng: &mut StdRng) -> String {
            if rng.gen_bool(0.2) {
                "\n".to_owned()
            } else {
                let c = rng.gen_range('A'..='Z');
                format!("{c}{c}{c}\n")
            }
        }

        fn gen_working_copy(rng: &mut StdRng, head: &str) -> String {
            let mut old_lines = {
                let mut old_lines = Vec::new();
                let mut old_lines_iter = head.lines();
                while let Some(line) = old_lines_iter.next() {
                    assert!(!line.ends_with("\n"));
                    old_lines.push(line.to_owned());
                }
                if old_lines.last().is_some_and(|line| line.is_empty()) {
                    old_lines.pop();
                }
                old_lines.into_iter()
            };
            let mut result = String::new();
            let unchanged_count = rng.gen_range(0..=old_lines.len());
            result +=
                &old_lines
                    .by_ref()
                    .take(unchanged_count)
                    .fold(String::new(), |mut s, line| {
                        writeln!(&mut s, "{line}").unwrap();
                        s
                    });
            while old_lines.len() > 0 {
                let deleted_count = rng.gen_range(0..=old_lines.len());
                let _advance = old_lines
                    .by_ref()
                    .take(deleted_count)
                    .map(|line| line.len() + 1)
                    .sum::<usize>();
                let minimum_added = if deleted_count == 0 { 1 } else { 0 };
                let added_count = rng.gen_range(minimum_added..=5);
                let addition = (0..added_count).map(|_| gen_line(rng)).collect::<String>();
                result += &addition;

                if old_lines.len() > 0 {
                    let blank_lines = old_lines.clone().take_while(|line| line.is_empty()).count();
                    if blank_lines == old_lines.len() {
                        break;
                    };
                    let unchanged_count = rng.gen_range((blank_lines + 1).max(1)..=old_lines.len());
                    result += &old_lines.by_ref().take(unchanged_count).fold(
                        String::new(),
                        |mut s, line| {
                            writeln!(&mut s, "{line}").unwrap();
                            s
                        },
                    );
                }
            }
            result
        }

        fn uncommitted_diff(
            working_copy: &language::BufferSnapshot,
            index_text: &Rope,
            head_text: String,
            cx: &mut TestAppContext,
        ) -> Entity<BufferDiff> {
            let inner = BufferDiff::build_sync(working_copy.text.clone(), head_text, cx);
            let secondary = BufferDiff {
                buffer_id: working_copy.remote_id(),
                inner: BufferDiff::build_sync(
                    working_copy.text.clone(),
                    index_text.to_string(),
                    cx,
                ),
                secondary_diff: None,
            };
            let secondary = cx.new(|_| secondary);
            cx.new(|_| BufferDiff {
                buffer_id: working_copy.remote_id(),
                inner,
                secondary_diff: Some(secondary),
            })
        }

        let operations = std::env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let rng = &mut rng;
        let head_text = ('a'..='z').fold(String::new(), |mut s, c| {
            writeln!(&mut s, "{c}{c}{c}").unwrap();
            s
        });
        let working_copy = gen_working_copy(rng, &head_text);
        let working_copy = cx.new(|cx| {
            language::Buffer::local_normalized(
                Rope::from(working_copy.as_str()),
                text::LineEnding::default(),
                cx,
            )
        });
        let working_copy = working_copy.read_with(cx, |working_copy, _| working_copy.snapshot());
        let mut index_text = if rng.gen() {
            Rope::from(head_text.as_str())
        } else {
            working_copy.as_rope().clone()
        };

        let mut diff = uncommitted_diff(&working_copy, &index_text, head_text.clone(), cx);
        let mut hunks = diff.update(cx, |diff, cx| {
            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &working_copy, cx)
                .collect::<Vec<_>>()
        });
        if hunks.len() == 0 {
            return;
        }

        for _ in 0..operations {
            let i = rng.gen_range(0..hunks.len());
            let hunk = &mut hunks[i];
            let hunk_to_change = hunk.clone();
            let stage = match hunk.secondary_status {
                DiffHunkSecondaryStatus::HasSecondaryHunk => {
                    hunk.secondary_status = DiffHunkSecondaryStatus::None;
                    true
                }
                DiffHunkSecondaryStatus::None => {
                    hunk.secondary_status = DiffHunkSecondaryStatus::HasSecondaryHunk;
                    false
                }
                _ => unreachable!(),
            };

            index_text = diff.update(cx, |diff, cx| {
                diff.stage_or_unstage_hunks(stage, &[hunk_to_change], &working_copy, true, cx)
                    .unwrap()
            });

            diff = uncommitted_diff(&working_copy, &index_text, head_text.clone(), cx);
            let found_hunks = diff.update(cx, |diff, cx| {
                diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &working_copy, cx)
                    .collect::<Vec<_>>()
            });
            assert_eq!(hunks.len(), found_hunks.len());

            for (expected_hunk, found_hunk) in hunks.iter().zip(&found_hunks) {
                assert_eq!(
                    expected_hunk.buffer_range.to_point(&working_copy),
                    found_hunk.buffer_range.to_point(&working_copy)
                );
                assert_eq!(
                    expected_hunk.diff_base_byte_range,
                    found_hunk.diff_base_byte_range
                );
                assert_eq!(expected_hunk.secondary_status, found_hunk.secondary_status);
            }
            hunks = found_hunks;
        }
    }
}

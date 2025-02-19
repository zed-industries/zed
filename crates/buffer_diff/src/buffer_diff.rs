use futures::{channel::oneshot, future::OptionFuture};
use git2::{DiffLineType as GitDiffLineType, DiffOptions as GitOptions, Patch as GitPatch};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, EventEmitter};
use language::{Language, LanguageRegistry};
use rope::Rope;
use std::{cmp, future::Future, iter, ops::Range, sync::Arc};
use sum_tree::SumTree;
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
    pub is_single_insertion: bool,
}

#[derive(Clone)]
struct BufferDiffInner {
    hunks: SumTree<InternalDiffHunk>,
    base_text: Option<language::BufferSnapshot>,
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
}

impl DiffHunkSecondaryStatus {
    pub fn is_secondary(&self) -> bool {
        match self {
            DiffHunkSecondaryStatus::HasSecondaryHunk => true,
            DiffHunkSecondaryStatus::OverlapsWithSecondaryHunk => true,
            DiffHunkSecondaryStatus::None => false,
        }
    }
}

/// A diff hunk resolved to rows in the buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffHunk {
    /// The buffer range, expressed in terms of rows.
    pub row_range: Range<u32>,
    /// The range in the buffer to which this hunk corresponds.
    pub buffer_range: Range<Anchor>,
    /// The range in the buffer's diff base text to which this hunk corresponds.
    pub diff_base_byte_range: Range<usize>,
    pub secondary_status: DiffHunkSecondaryStatus,
    pub secondary_diff_base_byte_range: Option<Range<usize>>,
}

/// We store [`InternalDiffHunk`]s internally so we don't need to store the additional row range.
#[derive(Debug, Clone, PartialEq, Eq)]
struct InternalDiffHunk {
    buffer_range: Range<Anchor>,
    diff_base_byte_range: Range<usize>,
}

impl sum_tree::Item for InternalDiffHunk {
    type Summary = DiffHunkSummary;

    fn summary(&self, _cx: &text::BufferSnapshot) -> Self::Summary {
        DiffHunkSummary {
            buffer_range: self.buffer_range.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DiffHunkSummary {
    buffer_range: Range<Anchor>,
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

impl<'a> sum_tree::SeekTarget<'a, DiffHunkSummary, DiffHunkSummary> for Anchor {
    fn cmp(
        &self,
        cursor_location: &DiffHunkSummary,
        buffer: &text::BufferSnapshot,
    ) -> cmp::Ordering {
        self.cmp(&cursor_location.buffer_range.end, buffer)
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

    pub fn base_text(&self) -> Option<&language::BufferSnapshot> {
        self.inner.base_text.as_ref()
    }

    pub fn base_texts_eq(&self, other: &Self) -> bool {
        match (other.base_text(), self.base_text()) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(old), Some(new)) => {
                let (old_id, old_empty) = (old.remote_id(), old.is_empty());
                let (new_id, new_empty) = (new.remote_id(), new.is_empty());
                new_id == old_id || (new_empty && old_empty)
            }
        }
    }

    fn buffer_range_to_unchanged_diff_base_range(
        &self,
        buffer_range: Range<Anchor>,
        buffer: &text::BufferSnapshot,
    ) -> Option<Range<usize>> {
        let mut hunks = self.inner.hunks.iter();
        let mut start = 0;
        let mut pos = buffer.anchor_before(0);
        while let Some(hunk) = hunks.next() {
            assert!(buffer_range.start.cmp(&pos, buffer).is_ge());
            assert!(hunk.buffer_range.start.cmp(&pos, buffer).is_ge());
            if hunk
                .buffer_range
                .start
                .cmp(&buffer_range.end, buffer)
                .is_ge()
            {
                // target buffer range is contained in the unchanged stretch leading up to this next hunk,
                // so do a final adjustment based on that
                break;
            }

            // if the target buffer range intersects this hunk at all, no dice
            if buffer_range
                .start
                .cmp(&hunk.buffer_range.end, buffer)
                .is_lt()
            {
                return None;
            }

            start += hunk.buffer_range.start.to_offset(buffer) - pos.to_offset(buffer);
            start += hunk.diff_base_byte_range.end - hunk.diff_base_byte_range.start;
            pos = hunk.buffer_range.end;
        }
        start += buffer_range.start.to_offset(buffer) - pos.to_offset(buffer);
        let end = start + buffer_range.end.to_offset(buffer) - buffer_range.start.to_offset(buffer);
        Some(start..end)
    }

    pub fn secondary_edits_for_stage_or_unstage(
        &self,
        stage: bool,
        hunks: impl Iterator<Item = (Range<usize>, Option<Range<usize>>, Range<Anchor>)>,
        buffer: &text::BufferSnapshot,
    ) -> Vec<(Range<usize>, String)> {
        let Some(secondary_diff) = self.secondary_diff() else {
            log::debug!("no secondary diff");
            return Vec::new();
        };
        let index_base = secondary_diff.base_text().map_or_else(
            || Rope::from(""),
            |snapshot| snapshot.text.as_rope().clone(),
        );
        let head_base = self.base_text().map_or_else(
            || Rope::from(""),
            |snapshot| snapshot.text.as_rope().clone(),
        );
        log::debug!("original: {:?}", index_base.to_string());
        let mut edits = Vec::new();
        for (diff_base_byte_range, secondary_diff_base_byte_range, buffer_range) in hunks {
            let (index_byte_range, replacement_text) = if stage {
                log::debug!("staging");
                let mut replacement_text = String::new();
                let Some(index_byte_range) = secondary_diff_base_byte_range.clone() else {
                    log::debug!("not a stageable hunk");
                    continue;
                };
                log::debug!("using {:?}", index_byte_range);
                for chunk in buffer.text_for_range(buffer_range.clone()) {
                    replacement_text.push_str(chunk);
                }
                (index_byte_range, replacement_text)
            } else {
                log::debug!("unstaging");
                let mut replacement_text = String::new();
                let Some(index_byte_range) = secondary_diff
                    .buffer_range_to_unchanged_diff_base_range(buffer_range.clone(), &buffer)
                else {
                    log::debug!("not an unstageable hunk");
                    continue;
                };
                for chunk in head_base.chunks_in_range(diff_base_byte_range.clone()) {
                    replacement_text.push_str(chunk);
                }
                (index_byte_range, replacement_text)
            };
            edits.push((index_byte_range, replacement_text));
        }
        log::debug!("edits: {edits:?}");
        edits
    }
}

impl BufferDiffInner {
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

        let mut secondary_cursor = secondary.as_ref().map(|diff| {
            let mut cursor = diff.hunks.cursor::<DiffHunkSummary>(buffer);
            cursor.next(buffer);
            cursor
        });

        let mut summaries = buffer.summaries_for_anchors_with_payload::<Point, _, _>(anchor_iter);
        iter::from_fn(move || loop {
            let (start_point, (start_anchor, start_base)) = summaries.next()?;
            let (mut end_point, (mut end_anchor, end_base)) = summaries.next()?;

            if !start_anchor.is_valid(buffer) {
                continue;
            }

            if end_point.column > 0 {
                end_point.row += 1;
                end_point.column = 0;
                end_anchor = buffer.anchor_before(end_point);
            }

            let mut secondary_status = DiffHunkSecondaryStatus::None;
            let mut secondary_diff_base_byte_range = None;
            if let Some(secondary_cursor) = secondary_cursor.as_mut() {
                if start_anchor
                    .cmp(&secondary_cursor.start().buffer_range.start, buffer)
                    .is_gt()
                {
                    secondary_cursor.seek_forward(&end_anchor, Bias::Left, buffer);
                }

                if let Some(secondary_hunk) = secondary_cursor.item() {
                    let mut secondary_range = secondary_hunk.buffer_range.to_point(buffer);
                    if secondary_range.end.column > 0 {
                        secondary_range.end.row += 1;
                        secondary_range.end.column = 0;
                    }
                    if secondary_range == (start_point..end_point) {
                        secondary_status = DiffHunkSecondaryStatus::HasSecondaryHunk;
                        secondary_diff_base_byte_range =
                            Some(secondary_hunk.diff_base_byte_range.clone());
                    } else if secondary_range.start <= end_point {
                        secondary_status = DiffHunkSecondaryStatus::OverlapsWithSecondaryHunk;
                    }
                }
            }

            return Some(DiffHunk {
                row_range: start_point.row..end_point.row,
                diff_base_byte_range: start_base..end_base,
                buffer_range: start_anchor..end_anchor,
                secondary_status,
                secondary_diff_base_byte_range,
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
            let end_row = if range.end.column > 0 {
                range.end.row + 1
            } else {
                range.end.row
            };

            Some(DiffHunk {
                row_range: range.start.row..end_row,
                diff_base_byte_range: hunk.diff_base_byte_range.clone(),
                buffer_range: hunk.buffer_range.clone(),
                // The secondary status is not used by callers of this method.
                secondary_status: DiffHunkSecondaryStatus::None,
                secondary_diff_base_byte_range: None,
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
                        cmp::Ordering::Less => {
                            start.get_or_insert(new_hunk.buffer_range.start);
                            end.replace(new_hunk.buffer_range.end);
                            new_cursor.next(new_snapshot);
                        }
                        cmp::Ordering::Equal => {
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
                        cmp::Ordering::Greater => {
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
        diff_base: Option<Arc<String>>,
        language: Option<Arc<Language>>,
        language_registry: Option<Arc<LanguageRegistry>>,
        cx: &mut App,
    ) -> impl Future<Output = BufferDiffInner> {
        let diff_base =
            diff_base.map(|diff_base| (diff_base.clone(), Rope::from(diff_base.as_str())));
        let base_text_snapshot = diff_base.as_ref().map(|(_, diff_base)| {
            language::Buffer::build_snapshot(
                diff_base.clone(),
                language.clone(),
                language_registry.clone(),
                cx,
            )
        });
        let base_text_snapshot = cx.background_spawn(OptionFuture::from(base_text_snapshot));

        let hunks = cx.background_spawn({
            let buffer = buffer.clone();
            async move { compute_hunks(diff_base, buffer) }
        });

        async move {
            let (base_text, hunks) = futures::join!(base_text_snapshot, hunks);
            BufferDiffInner { base_text, hunks }
        }
    }

    fn build_with_base_buffer(
        buffer: text::BufferSnapshot,
        diff_base: Option<Arc<String>>,
        diff_base_buffer: Option<language::BufferSnapshot>,
        cx: &App,
    ) -> impl Future<Output = BufferDiffInner> {
        let diff_base = diff_base.clone().zip(
            diff_base_buffer
                .clone()
                .map(|buffer| buffer.as_rope().clone()),
        );
        cx.background_spawn(async move {
            BufferDiffInner {
                hunks: compute_hunks(diff_base, buffer),
                base_text: diff_base_buffer,
            }
        })
    }

    fn build_empty(buffer: &text::BufferSnapshot) -> BufferDiffInner {
        BufferDiffInner {
            hunks: SumTree::new(buffer),
            base_text: None,
        }
    }

    pub fn build_with_single_insertion(
        insertion_present_in_secondary_diff: bool,
        buffer: language::BufferSnapshot,
        cx: &mut App,
    ) -> BufferDiffSnapshot {
        let base_text = language::Buffer::build_empty_snapshot(cx);
        let hunks = SumTree::from_item(
            InternalDiffHunk {
                buffer_range: Anchor::MIN..Anchor::MAX,
                diff_base_byte_range: 0..0,
            },
            &base_text,
        );
        BufferDiffSnapshot {
            inner: BufferDiffInner {
                hunks: hunks.clone(),
                base_text: Some(base_text.clone()),
            },
            secondary_diff: Some(Box::new(BufferDiffSnapshot {
                inner: BufferDiffInner {
                    hunks: if insertion_present_in_secondary_diff {
                        hunks
                    } else {
                        SumTree::new(&buffer.text)
                    },
                    base_text: Some(if insertion_present_in_secondary_diff {
                        base_text
                    } else {
                        buffer
                    }),
                },
                secondary_diff: None,
                is_single_insertion: true,
            })),
            is_single_insertion: true,
        }
    }

    pub fn set_secondary_diff(&mut self, diff: Entity<BufferDiff>) {
        self.secondary_diff = Some(diff);
    }

    pub fn secondary_diff(&self) -> Option<Entity<BufferDiff>> {
        Some(self.secondary_diff.as_ref()?.clone())
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
                    this.base_text().cloned(),
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
        inner: BufferDiffInner,
        buffer: &text::BufferSnapshot,
    ) -> Option<Range<Anchor>> {
        let changed_range = match (self.inner.base_text.as_ref(), inner.base_text.as_ref()) {
            (None, None) => None,
            (Some(old), Some(new)) if old.remote_id() == new.remote_id() => {
                inner.compare(&self.inner, buffer)
            }
            _ => Some(text::Anchor::MIN..text::Anchor::MAX),
        };
        self.inner = inner;
        changed_range
    }

    pub fn base_text(&self) -> Option<&language::BufferSnapshot> {
        self.inner.base_text.as_ref()
    }

    pub fn snapshot(&self, cx: &App) -> BufferDiffSnapshot {
        BufferDiffSnapshot {
            inner: self.inner.clone(),
            secondary_diff: self
                .secondary_diff
                .as_ref()
                .map(|diff| Box::new(diff.read(cx).snapshot(cx))),
            is_single_insertion: false,
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn base_text_string(&self) -> Option<String> {
        self.inner.base_text.as_ref().map(|buffer| buffer.text())
    }

    pub fn new(buffer: &text::BufferSnapshot) -> Self {
        BufferDiff {
            buffer_id: buffer.remote_id(),
            inner: BufferDiff::build_empty(buffer),
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
        let base_text = self
            .inner
            .base_text
            .as_ref()
            .map(|base_text| base_text.text());
        let snapshot = BufferDiff::build_with_base_buffer(
            buffer.clone(),
            base_text.clone().map(Arc::new),
            self.inner.base_text.clone(),
            cx,
        );
        let snapshot = cx.background_executor().block(snapshot);
        let changed_range = self.set_state(snapshot, &buffer);
        cx.emit(BufferDiffEvent::DiffChanged { changed_range });
    }
}

impl DiffHunk {
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn deleted_none() -> Self {
        Self {
            kind: DiffHunkStatusKind::Deleted,
            secondary: DiffHunkSecondaryStatus::None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn added_none() -> Self {
        Self {
            kind: DiffHunkStatusKind::Added,
            secondary: DiffHunkSecondaryStatus::None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
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
                hunk.row_range.clone(),
                &diff_base[hunk.diff_base_byte_range.clone()],
                buffer
                    .text_for_range(
                        Point::new(hunk.row_range.start, 0)..Point::new(hunk.row_range.end, 0),
                    )
                    .collect::<String>(),
                hunk.status(),
            )
        })
        .collect::<Vec<_>>();

    let expected_hunks: Vec<_> = expected_hunks
        .iter()
        .map(|(r, s, h, status)| (r.clone(), *s, h.to_string(), *status))
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

        diff = BufferDiff::build_empty(&buffer);
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

        let empty_diff = BufferDiff::build_empty(&buffer);
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
    async fn test_secondary_edits_for_stage_unstage(cx: &mut TestAppContext, mut rng: StdRng) {
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
            index_text: &Entity<language::Buffer>,
            head_text: String,
            cx: &mut TestAppContext,
        ) -> BufferDiff {
            let inner = BufferDiff::build_sync(working_copy.text.clone(), head_text, cx);
            let secondary = BufferDiff {
                buffer_id: working_copy.remote_id(),
                inner: BufferDiff::build_sync(
                    working_copy.text.clone(),
                    index_text.read_with(cx, |index_text, _| index_text.text()),
                    cx,
                ),
                secondary_diff: None,
            };
            let secondary = cx.new(|_| secondary);
            BufferDiff {
                buffer_id: working_copy.remote_id(),
                inner,
                secondary_diff: Some(secondary),
            }
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
        let index_text = cx.new(|cx| {
            language::Buffer::local_normalized(
                if rng.gen() {
                    Rope::from(head_text.as_str())
                } else {
                    working_copy.as_rope().clone()
                },
                text::LineEnding::default(),
                cx,
            )
        });

        let mut diff = uncommitted_diff(&working_copy, &index_text, head_text.clone(), cx);
        let mut hunks = cx.update(|cx| {
            diff.hunks_intersecting_range(Anchor::MIN..Anchor::MAX, &working_copy, cx)
                .collect::<Vec<_>>()
        });
        if hunks.len() == 0 {
            return;
        }

        for _ in 0..operations {
            let i = rng.gen_range(0..hunks.len());
            let hunk = &mut hunks[i];
            let hunk_fields = (
                hunk.diff_base_byte_range.clone(),
                hunk.secondary_diff_base_byte_range.clone(),
                hunk.buffer_range.clone(),
            );
            let stage = match (
                hunk.secondary_status,
                hunk.secondary_diff_base_byte_range.clone(),
            ) {
                (DiffHunkSecondaryStatus::HasSecondaryHunk, Some(_)) => {
                    hunk.secondary_status = DiffHunkSecondaryStatus::None;
                    hunk.secondary_diff_base_byte_range = None;
                    true
                }
                (DiffHunkSecondaryStatus::None, None) => {
                    hunk.secondary_status = DiffHunkSecondaryStatus::HasSecondaryHunk;
                    // We don't look at this, just notice whether it's Some or not.
                    hunk.secondary_diff_base_byte_range = Some(17..17);
                    false
                }
                _ => unreachable!(),
            };

            let snapshot = cx.update(|cx| diff.snapshot(cx));
            let edits = snapshot.secondary_edits_for_stage_or_unstage(
                stage,
                [hunk_fields].into_iter(),
                &working_copy,
            );
            index_text.update(cx, |index_text, cx| {
                index_text.edit(edits, None, cx);
            });

            diff = uncommitted_diff(&working_copy, &index_text, head_text.clone(), cx);
            let found_hunks = cx.update(|cx| {
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
                assert_eq!(
                    expected_hunk.secondary_diff_base_byte_range.is_some(),
                    found_hunk.secondary_diff_base_byte_range.is_some()
                )
            }
            hunks = found_hunks;
        }
    }
}

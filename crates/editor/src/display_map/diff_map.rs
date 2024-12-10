use super::fold_map::{FoldBufferRows, FoldChunks, FoldEdit, FoldOffset, FoldSnapshot};
use crate::{FoldPoint, Highlights};
use collections::HashMap;
use gpui::{AppContext, Context as _, Model, ModelContext, Subscription};
use language::{BufferChunks, BufferId, Chunk};
use multi_buffer::{Anchor, AnchorRangeExt, MultiBuffer, ToOffset};
use project::buffer_store::BufferChangeSet;
use std::{cmp::Ordering, mem, ops::Range};
use sum_tree::{Cursor, SumTree, TreeMap};
use text::{Bias, Edit, Patch, Point, TextSummary, ToOffset as _};

struct DiffMap {
    snapshot: DiffMapSnapshot,
    multibuffer: Model<MultiBuffer>,
    diff_bases: HashMap<BufferId, ChangeSetState>,
    all_hunks_expanded: bool,
    edits_since_sync: Patch<DiffOffset>,
}

struct ChangeSetState {
    change_set: Model<BufferChangeSet>,
    _subscription: Subscription,
}

#[derive(Clone)]
struct DiffSnapshot {
    diff: git::diff::BufferDiff,
    base_text: language::BufferSnapshot,
}

#[derive(Clone)]
pub(crate) struct DiffMapSnapshot {
    diffs: TreeMap<BufferId, DiffSnapshot>,
    transforms: SumTree<DiffTransform>,
    fold_snapshot: FoldSnapshot,
}

#[derive(Debug, Clone)]
enum DiffTransform {
    BufferContent {
        summary: TextSummary,
    },
    DeletedHunk {
        summary: TextSummary,
        buffer_id: BufferId,
        base_text_byte_range: Range<usize>,
        base_text_start: Point,
    },
}

#[derive(Debug, Clone)]
struct DiffTransformSummary {
    input: TextSummary,
    output: TextSummary,
}

struct DiffMapChunks<'a> {
    snapshot: &'a DiffMapSnapshot,
    language_aware: bool,
    cursor: Cursor<'a, DiffTransform, (DiffOffset, FoldOffset)>,
    fold_chunks: FoldChunks<'a>,
    fold_chunk: Option<Chunk<'a>>,
    fold_offset: FoldOffset,
    offset: DiffOffset,
    end_offset: DiffOffset,
    diff_base_chunks: Option<(BufferId, BufferChunks<'a>)>,
}

struct DiffMapBufferRows<'a> {
    cursor: Cursor<'a, DiffTransform, DiffTransformSummary>,
    input_buffer_rows: FoldBufferRows<'a>,
}

pub type DiffEdit = text::Edit<DiffOffset>;

enum DiffMapOperation {
    BufferDiffUpdated {
        buffer_id: BufferId,
    },
    Edited {
        edits: Vec<FoldEdit>,
    },
    ExpandHunks {
        ranges: Vec<Range<multi_buffer::Anchor>>,
    },
    CollapseHunks {
        ranges: Vec<Range<multi_buffer::Anchor>>,
    },
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct DiffOffset(pub usize);

impl DiffMap {
    pub fn new(
        fold_snapshot: FoldSnapshot,
        multibuffer: Model<MultiBuffer>,
        cx: &mut AppContext,
    ) -> (Model<Self>, DiffMapSnapshot) {
        let snapshot = DiffMapSnapshot {
            diffs: TreeMap::default(),
            transforms: SumTree::new(&()),
            fold_snapshot,
        };

        let this = cx.new_model(|_| Self {
            multibuffer,
            snapshot: snapshot.clone(),
            all_hunks_expanded: false,
            diff_bases: HashMap::default(),
            edits_since_sync: Patch::default(),
        });

        (this, snapshot)
    }

    pub fn add_change_set(
        &mut self,
        change_set: Model<BufferChangeSet>,
        cx: &mut ModelContext<Self>,
    ) {
        let buffer_id = change_set.read(cx).buffer_id;
        self.buffer_diff_changed(change_set.clone(), cx);
        self.diff_bases.insert(
            buffer_id,
            ChangeSetState {
                _subscription: cx.observe(&change_set, Self::buffer_diff_changed),
                change_set,
            },
        );
    }

    pub fn sync(
        &mut self,
        fold_snapshot: FoldSnapshot,
        fold_edits: Vec<FoldEdit>,
    ) -> (DiffMapSnapshot, Vec<DiffEdit>) {
        let patch = mem::take(&mut self.edits_since_sync);
        (self.snapshot.clone(), patch.into_inner())
    }

    fn buffer_diff_changed(
        &mut self,
        change_set: Model<BufferChangeSet>,
        cx: &mut ModelContext<Self>,
    ) {
        let change_set = change_set.read(cx);
        let buffer_id = change_set.buffer_id;
        let diff = change_set.diff_to_buffer.clone();
        let base_text = change_set
            .base_text
            .as_ref()
            .map(|buffer| buffer.read(cx).snapshot());

        if let Some(base_text) = base_text.clone() {
            self.snapshot.diffs.insert(
                buffer_id,
                DiffSnapshot {
                    diff: diff.clone(),
                    base_text,
                },
            );
        } else {
            self.snapshot.diffs.remove(&buffer_id);
        }
        self.recompute_transforms(DiffMapOperation::BufferDiffUpdated { buffer_id }, cx);
    }

    pub(super) fn expand_diff_hunks(
        &mut self,
        ranges: Vec<Range<multi_buffer::Anchor>>,
        cx: &mut ModelContext<Self>,
    ) {
        self.recompute_transforms(DiffMapOperation::ExpandHunks { ranges }, cx);
    }

    pub(super) fn collapse_diff_hunks(
        &mut self,
        ranges: Vec<Range<multi_buffer::Anchor>>,
        cx: &mut ModelContext<Self>,
    ) {
        self.recompute_transforms(DiffMapOperation::CollapseHunks { ranges }, cx);
    }

    pub(super) fn set_all_hunks_expanded(&mut self, cx: &mut ModelContext<Self>) {
        self.all_hunks_expanded = true;
        self.recompute_transforms(
            DiffMapOperation::ExpandHunks {
                ranges: vec![Anchor::min()..Anchor::max()],
            },
            cx,
        );
    }

    fn recompute_transforms(
        &mut self,
        operation: DiffMapOperation,
        cx: &mut ModelContext<DiffMap>,
    ) {
        let multibuffer = self.multibuffer.read(cx);
        let multibuffer_snapshot = multibuffer.snapshot(cx);

        let affected_ranges: Vec<_> = match &operation {
            DiffMapOperation::BufferDiffUpdated { buffer_id } => {
                let multibuffer = self.multibuffer.read(cx);
                multibuffer
                    .ranges_for_buffer(*buffer_id, cx)
                    .into_iter()
                    .map(|(excerpt_id, range, buffer_range)| {
                        (
                            excerpt_id,
                            *buffer_id,
                            ToOffset::to_offset(&range.start, &multibuffer_snapshot)
                                ..ToOffset::to_offset(&range.end, &multibuffer_snapshot),
                            buffer_range,
                        )
                    })
                    .collect()
            }
            DiffMapOperation::Edited { edits } => todo!(),
            DiffMapOperation::ExpandHunks { ranges }
            | DiffMapOperation::CollapseHunks { ranges } => {
                let mut changed_ranges = Vec::new();
                for range in ranges.iter() {
                    let multibuffer_range = range.to_offset(&multibuffer_snapshot);
                    for (buffer, buffer_range, excerpt_id) in
                        multibuffer.range_to_buffer_ranges(range.clone(), cx)
                    {
                        let buffer = buffer.read(cx);
                        if let Some(excerpt_range) =
                            multibuffer_snapshot.range_for_excerpt::<usize>(excerpt_id)
                        {
                            changed_ranges.push((
                                excerpt_id,
                                buffer.remote_id(),
                                multibuffer_range.start.max(excerpt_range.start)
                                    ..multibuffer_range.end.min(excerpt_range.end),
                                buffer.anchor_before(buffer_range.start)
                                    ..buffer.anchor_after(buffer_range.end),
                            ));
                        }
                    }
                }
                changed_ranges
            }
        };

        let mut cursor = self
            .snapshot
            .transforms
            .cursor::<(FoldOffset, DiffOffset)>(&());
        let mut new_transforms = SumTree::default();
        let mut edits = Patch::default();

        for (excerpt_id, buffer_id, multibuffer_range, buffer_range) in affected_ranges {
            let Some(buffer) = self.multibuffer.read(cx).buffer(buffer_id) else {
                continue;
            };
            let change_set_state = self.snapshot.diffs.get(&buffer_id);

            let buffer = buffer.read(cx);
            let excerpt_start = buffer_range.start.to_offset(buffer);
            let affected_range_start = self
                .snapshot
                .fold_snapshot
                .make_fold_offset(multibuffer_range.start, Bias::Left);
            let affected_range_end = self
                .snapshot
                .fold_snapshot
                .make_fold_offset(multibuffer_range.end, Bias::Right);

            new_transforms.append(cursor.slice(&affected_range_start, Bias::Left, &()), &());

            if let Some(change_set_state) = change_set_state {
                let diff = &change_set_state.diff;
                let base_text = &change_set_state.base_text;

                for hunk in diff.hunks_intersecting_range(buffer_range.clone(), buffer) {
                    let hunk_start_anchor = multi_buffer::Anchor {
                        excerpt_id,
                        buffer_id: Some(buffer_id),
                        text_anchor: hunk.buffer_range.start,
                    };
                    let hunk_end_anchor = multi_buffer::Anchor {
                        excerpt_id,
                        buffer_id: Some(buffer_id),
                        text_anchor: hunk.buffer_range.end,
                    };
                    let hunk_start_multibuffer_offset = multibuffer_range.start
                        + (hunk.buffer_range.start.to_offset(buffer) - excerpt_start);
                    let hunk_start_fold_offset = self
                        .snapshot
                        .fold_snapshot
                        .make_fold_offset(hunk_start_multibuffer_offset, Bias::Left);

                    self.push_buffer_content_transform(&mut new_transforms, hunk_start_fold_offset);

                    let mut prev_old_transform_start = cursor.start().1;
                    while cursor.start().0 < hunk_start_fold_offset {
                        let Some(item) = cursor.item() else {
                            break;
                        };
                        if let DiffTransform::DeletedHunk {
                            base_text_byte_range,
                            ..
                        } = item
                        {
                            let old_range = prev_old_transform_start
                                ..prev_old_transform_start + DiffOffset(base_text_byte_range.len());
                            let new_offset = DiffOffset(new_transforms.summary().output.len);
                            let edit = Edit {
                                old: old_range,
                                new: new_offset..new_offset,
                            };
                            dbg!(&edit);
                            edits.push(edit);
                        }
                        prev_old_transform_start = cursor.start().1;
                        cursor.next(&());
                    }

                    let mut was_previously_expanded = false;
                    if let Some(item) = cursor.item() {
                        if let DiffTransform::DeletedHunk {
                            base_text_byte_range,
                            ..
                        } = item
                        {
                            if cursor.start().0 == hunk_start_fold_offset
                                && *base_text_byte_range == hunk.diff_base_byte_range
                            {
                                was_previously_expanded = true;
                            }
                        }
                    }

                    let mut should_expand_hunk = was_previously_expanded || self.all_hunks_expanded;
                    match &operation {
                        DiffMapOperation::ExpandHunks { ranges } => {
                            should_expand_hunk |= ranges.iter().any(|range| {
                                range.overlaps(
                                    &(hunk_start_anchor..hunk_end_anchor),
                                    &multibuffer_snapshot,
                                )
                            })
                        }
                        DiffMapOperation::CollapseHunks { ranges } => {
                            should_expand_hunk &= !ranges.iter().any(|range| {
                                range.overlaps(
                                    &(hunk_start_anchor..hunk_end_anchor),
                                    &multibuffer_snapshot,
                                )
                            })
                        }
                        _ => {}
                    };
                    if hunk.diff_base_byte_range.len() == 0 {
                        should_expand_hunk = false;
                    }

                    if should_expand_hunk {
                        let mut text_cursor = base_text.as_rope().cursor(0);
                        let base_text_start =
                            text_cursor.summary::<Point>(hunk.diff_base_byte_range.start);
                        let base_text_summary =
                            text_cursor.summary::<TextSummary>(hunk.diff_base_byte_range.end);

                        if !was_previously_expanded {
                            let old_offset = prev_old_transform_start;
                            let new_start = DiffOffset(new_transforms.summary().output.len);
                            let edit = Edit {
                                old: old_offset..old_offset,
                                new: new_start
                                    ..new_start + DiffOffset(hunk.diff_base_byte_range.len()),
                            };
                            dbg!(&edit);
                            edits.push(edit);
                        }

                        new_transforms.push(
                            DiffTransform::DeletedHunk {
                                base_text_byte_range: hunk.diff_base_byte_range.clone(),
                                summary: base_text_summary,
                                buffer_id,
                                base_text_start,
                            },
                            &(),
                        );
                    } else if was_previously_expanded {
                        let old_start = cursor.start().1;
                        let new_offset = DiffOffset(new_transforms.summary().output.len);
                        let edit = Edit {
                            old: old_start..old_start + DiffOffset(hunk.diff_base_byte_range.len()),
                            new: new_offset..new_offset,
                        };
                        dbg!(&edit);
                        edits.push(edit);
                        cursor.next(&());
                    }
                }

                while cursor.start().0 < affected_range_end {
                    let Some(item) = cursor.item() else {
                        break;
                    };
                    if let DiffTransform::DeletedHunk {
                        base_text_byte_range,
                        ..
                    } = item
                    {
                        let old_start = cursor.start().1;
                        let new_offset = DiffOffset(new_transforms.summary().output.len);
                        let edit = Edit {
                            old: old_start..old_start + DiffOffset(base_text_byte_range.len()),
                            new: new_offset..new_offset,
                        };
                        dbg!(&edit);
                        edits.push(edit);
                    }
                    cursor.next(&());
                }
            }

            self.push_buffer_content_transform(&mut new_transforms, affected_range_end);
        }

        self.edits_since_sync = self.edits_since_sync.compose(edits);

        new_transforms.append(cursor.suffix(&()), &());
        drop(cursor);
        self.snapshot.transforms = new_transforms;
        cx.notify();

        #[cfg(test)]
        self.check_invariants();
    }

    fn push_buffer_content_transform(
        &self,
        new_transforms: &mut SumTree<DiffTransform>,
        end_point: FoldOffset,
    ) {
        let summary_to_add = self.snapshot.fold_snapshot.text_summary_for_offset_range(
            FoldOffset(new_transforms.summary().input.len)..end_point,
        );
        if summary_to_add.len > 0 {
            let mut did_extend = false;
            new_transforms.update_last(
                |last_transform| {
                    if let DiffTransform::BufferContent { summary } = last_transform {
                        did_extend = true;
                        *summary += summary_to_add.clone();
                    }
                },
                &(),
            );
            if !did_extend {
                new_transforms.push(
                    DiffTransform::BufferContent {
                        summary: summary_to_add,
                    },
                    &(),
                )
            }
        }
    }

    #[cfg(test)]
    fn check_invariants(&self) {
        let snapshot = &self.snapshot;
        if snapshot.transforms.summary().input.len != snapshot.fold_snapshot.len().0 {
            panic!(
                "incorrect input length. expected {}, got {}. transforms: {:+?}",
                snapshot.fold_snapshot.len().0,
                snapshot.transforms.summary().input.len,
                snapshot.transforms.items(&()),
            );
        }
    }
}

impl DiffMapSnapshot {
    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(DiffOffset(0)..self.len(), false, Highlights::default())
            .map(|c| c.text)
            .collect()
    }

    #[cfg(test)]
    pub fn len(&self) -> DiffOffset {
        DiffOffset(self.transforms.summary().output.len)
    }

    pub fn to_fold_offset(&self, offset: DiffOffset) -> FoldOffset {
        let mut cursor = self.transforms.cursor::<(DiffOffset, FoldOffset)>(&());
        cursor.seek(&offset, Bias::Right, &());
        let mut fold_offset = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = offset.0 - cursor.start().0 .0;
            fold_offset.0 += overshoot;
        }
        fold_offset
    }

    pub fn chunks<'a>(
        &'a self,
        range: Range<DiffOffset>,
        language_aware: bool,
        highlights: Highlights<'a>,
    ) -> DiffMapChunks<'a> {
        let mut cursor = self.transforms.cursor::<(DiffOffset, FoldOffset)>(&());

        cursor.seek(&range.end, Bias::Right, &());
        let mut fold_end = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = range.end.0 - cursor.start().0 .0;
            fold_end.0 += overshoot;
        }

        cursor.seek(&range.start, Bias::Right, &());
        let mut fold_start = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = range.start.0 - cursor.start().0 .0;
            fold_start.0 += overshoot;
        }

        let fold_chunks =
            self.fold_snapshot
                .chunks(fold_start..fold_end, language_aware, highlights);

        DiffMapChunks {
            snapshot: self,
            language_aware,
            cursor,
            fold_chunk: None,
            fold_chunks,
            fold_offset: fold_start,
            offset: range.start,
            diff_base_chunks: None,
            end_offset: range.end,
        }
    }

    pub fn buffer_rows(&self, start_row: u32) -> DiffMapBufferRows {
        todo!()
    }
}

impl<'a> Iterator for DiffMapChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.end_offset {
            return None;
        }
        if self.offset == self.cursor.end(&()).0 {
            self.cursor.next(&());
        }

        let transform = self.cursor.item()?;

        match transform {
            DiffTransform::BufferContent { summary } => {
                let chunk = self
                    .fold_chunk
                    .get_or_insert_with(|| self.fold_chunks.next().unwrap());

                let chunk_end = self.offset + DiffOffset(chunk.text.len());
                let mut transform_end = self.cursor.start().0 + DiffOffset(summary.len);

                if transform_end > self.end_offset {
                    transform_end = self.end_offset
                }

                if transform_end < chunk_end {
                    let (before, after) = chunk.text.split_at(transform_end.0 - self.offset.0);
                    self.offset = transform_end;
                    chunk.text = after;
                    Some(Chunk {
                        text: before,
                        ..chunk.clone()
                    })
                } else {
                    self.offset = chunk_end;
                    self.fold_chunk.take()
                }
            }
            DiffTransform::DeletedHunk {
                summary,
                buffer_id,
                base_text_start,
                base_text_byte_range,
                ..
            } => {
                let base_buffer = &self.snapshot.diffs.get(&buffer_id)?.base_text;

                let diff_base_start_offset = base_text_byte_range.start;
                let diff_base_end_offset = base_text_byte_range.end;
                let diff_base_offset =
                    diff_base_start_offset + self.offset.0 - self.cursor.start().0 .0;

                let mut chunks = if let Some((_, mut chunks)) = self
                    .diff_base_chunks
                    .take()
                    .filter(|(id, _)| id == buffer_id)
                {
                    if chunks.offset() != diff_base_offset {
                        chunks.seek(diff_base_offset..diff_base_end_offset);
                    }
                    chunks
                } else {
                    base_buffer.chunks(diff_base_offset..diff_base_end_offset, self.language_aware)
                };

                let chunk = chunks.next()?;

                self.offset.0 += chunk.text.len();
                self.diff_base_chunks = Some((*buffer_id, chunks));
                Some(chunk)
            }
        }
    }
}

impl sum_tree::Item for DiffTransform {
    type Summary = DiffTransformSummary;

    fn summary(&self, _: &<Self::Summary as sum_tree::Summary>::Context) -> Self::Summary {
        match self {
            DiffTransform::BufferContent { summary } => DiffTransformSummary {
                input: summary.clone(),
                output: summary.clone(),
            },
            DiffTransform::DeletedHunk { summary, .. } => DiffTransformSummary {
                input: TextSummary::default(),
                output: summary.clone(),
            },
        }
    }
}

impl sum_tree::Summary for DiffTransformSummary {
    type Context = ();

    fn zero(_: &Self::Context) -> Self {
        DiffTransformSummary {
            input: TextSummary::default(),
            output: TextSummary::default(),
        }
    }

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        self.input += &summary.input;
        self.output += &summary.output;
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for FoldOffset {
    fn zero(_: &()) -> Self {
        FoldOffset(0)
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        self.0 += summary.input.len
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for DiffOffset {
    fn zero(_: &()) -> Self {
        DiffOffset(0)
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        self.0 += summary.output.len
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for FoldPoint {
    fn zero(_: &()) -> Self {
        FoldPoint(Point::zero())
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        self.0 += summary.input.lines
    }
}

impl<'a> sum_tree::SeekTarget<'a, DiffTransformSummary, DiffTransformSummary> for FoldPoint {
    fn cmp(&self, cursor_location: &DiffTransformSummary, _: &()) -> Ordering {
        Ord::cmp(&self.0, &cursor_location.input.lines)
    }
}

impl<'a> Iterator for DiffMapBufferRows<'a> {
    type Item = Option<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl std::ops::Add<DiffOffset> for DiffOffset {
    type Output = DiffOffset;

    fn add(self, rhs: DiffOffset) -> Self::Output {
        DiffOffset(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign<DiffOffset> for DiffOffset {
    fn add_assign(&mut self, rhs: DiffOffset) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub<DiffOffset> for DiffOffset {
    type Output = DiffOffset;

    fn sub(self, rhs: DiffOffset) -> Self::Output {
        DiffOffset(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display_map::{fold_map::FoldMap, inlay_map::InlayMap};
    use gpui::{AppContext, TestAppContext};
    use indoc::indoc;
    use multi_buffer::{Anchor, MultiBuffer};
    use project::Project;
    use settings::SettingsStore;

    #[gpui::test]
    fn test_basic_diff(cx: &mut TestAppContext) {
        cx.update(init_test);

        let text = indoc!(
            "
            ZERO
            one
            TWO
            three
            six
            "
        );
        let base_text = indoc!(
            "
            one
            two
            three
            four
            five
            six
            "
        );

        let buffer = cx.new_model(|cx| language::Buffer::local(text, cx));
        let change_set = cx.new_model(|cx| {
            BufferChangeSet::new_with_base_text(
                base_text.to_string(),
                buffer.read(cx).text_snapshot(),
                cx,
            )
        });

        let multibuffer = cx.new_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));
        let buffer_snapshot = multibuffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (diff_map, _) = cx.update(|cx| DiffMap::new(fold_snapshot.clone(), multibuffer, cx));
        diff_map.update(cx, |diff_map, cx| diff_map.add_change_set(change_set, cx));
        cx.run_until_parked();

        let (snapshot1, _) = diff_map.update(cx, |diff_map, _| {
            diff_map.sync(fold_snapshot.clone(), vec![])
        });

        assert_eq!(
            snapshot1.text(),
            indoc!(
                "
                ZERO
                one
                TWO
                three
                six
                "
            )
        );

        diff_map.update(cx, |diff_map, cx| {
            diff_map.expand_diff_hunks(vec![Anchor::min()..Anchor::max()], cx)
        });

        let (snapshot2, edits) = diff_map.update(cx, |diff_map, _| {
            diff_map.sync(fold_snapshot.clone(), vec![])
        });

        assert_eq!(
            snapshot2.text(),
            indoc!(
                "
                ZERO
                one
                two
                TWO
                three
                four
                five
                six
                "
            )
        );

        check_edits(&snapshot1, &snapshot2, &edits);

        diff_map.update(cx, |diff_map, cx| {
            diff_map.collapse_diff_hunks(vec![Anchor::min()..Anchor::max()], cx)
        });

        let (snapshot3, edits) = diff_map.update(cx, |diff_map, _| {
            diff_map.sync(fold_snapshot.clone(), vec![])
        });
        assert_eq!(
            snapshot3.text(),
            indoc!(
                "
                ZERO
                one
                TWO
                three
                six
                "
            )
        );
        check_edits(&snapshot2, &snapshot3, &edits);
    }

    #[track_caller]
    fn check_edits(
        old_snapshot: &DiffMapSnapshot,
        new_snapshot: &DiffMapSnapshot,
        edits: &[DiffEdit],
    ) {
        let mut text = old_snapshot.text();
        let new_text = new_snapshot.text();
        for edit in edits.iter().rev() {
            if !text.is_char_boundary(edit.old.start.0)
                || !text.is_char_boundary(edit.old.end.0)
                || !new_text.is_char_boundary(edit.new.start.0)
                || !new_text.is_char_boundary(edit.new.end.0)
            {
                panic!(
                    "invalid edits: {:?}\nold text: {:?}\nnew text: {:?}",
                    edits, text, new_text
                );
            }

            text.replace_range(
                edit.old.start.0..edit.old.end.0,
                &new_text[edit.new.start.0..edit.new.end.0],
            );
        }

        pretty_assertions::assert_eq!(text, new_text, "invalid edits: {:?}", edits);
    }

    fn init_test(cx: &mut AppContext) {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        language::init(cx);
        crate::init(cx);
        Project::init_settings(cx);
    }
}

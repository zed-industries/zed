use crate::{
    display_map::fold_map::{FoldBufferRows, FoldOffset, FoldSnapshot},
    FoldPoint, Highlights,
};
use collections::{HashMap, HashSet};
use gpui::{AppContext, Context as _, Model, ModelContext, Subscription};
use language::{BufferChunks, BufferId, Chunk};
use multi_buffer::MultiBuffer;
use project::buffer_store::BufferChangeSet;
use std::{cmp::Ordering, ops::Range};
use sum_tree::{Cursor, SumTree, TreeMap};
use text::{Bias, BufferSnapshot, Point, TextSummary, ToPoint};

use super::fold_map::{FoldChunks, FoldEdit};

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct DiffOffset(pub usize);

impl std::ops::Add<DiffOffset> for DiffOffset {
    type Output = DiffOffset;

    fn add(self, rhs: DiffOffset) -> Self::Output {
        DiffOffset(self.0 + rhs.0)
    }
}

struct DiffMap {
    snapshot: DiffMapSnapshot,
    multibuffer: Model<MultiBuffer>,
    diff_bases: HashMap<BufferId, ChangeSetState>,
    buffer_input_row_counts: Vec<(BufferId, u32)>,
    all_hunks_expanded: bool,
}

struct ChangeSetState {
    change_set: Model<BufferChangeSet>,
    last_version: Option<usize>,
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
        hunk_position: multi_buffer::Anchor,
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

        // Determine the extents of every run of excerpts associated with a
        // single buffer.
        let mut buffer_input_row_counts = Vec::new();
        let mut current_buffer_and_start_row = None;
        let fold_snapshot = &snapshot.fold_snapshot;
        let inlay_snapshot = &fold_snapshot.inlay_snapshot;
        let buffer_snapshot = &inlay_snapshot.buffer;
        for excerpt in buffer_snapshot.all_excerpts().map(Some).chain([None]) {
            let buffer_id = excerpt.as_ref().map(|e| e.buffer().remote_id());
            let position = excerpt.map_or(buffer_snapshot.max_point(), |e| e.start_point());
            let position = inlay_snapshot.to_inlay_point(position);
            let position = fold_snapshot.to_fold_point(position, Bias::Right);
            let row = position.row();
            if let Some((prev_buffer_id, prev_start_row)) = &current_buffer_and_start_row {
                if buffer_id != Some(*prev_buffer_id) {
                    buffer_input_row_counts.push((*prev_buffer_id, row - *prev_start_row));
                    current_buffer_and_start_row.take();
                }
            }
            if current_buffer_and_start_row.is_none() {
                if let Some(buffer_id) = buffer_id {
                    current_buffer_and_start_row = Some((buffer_id, row));
                }
            }
        }

        let this = cx.new_model(|_| Self {
            buffer_input_row_counts,
            multibuffer,
            snapshot: snapshot.clone(),
            all_hunks_expanded: false,
            diff_bases: HashMap::default(),
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
                last_version: None,
                change_set,
            },
        );
    }

    pub fn sync(
        &mut self,
        fold_snapshot: FoldSnapshot,
        mut fold_edits: Vec<FoldEdit>,
    ) -> (DiffMapSnapshot, Vec<DiffEdit>) {
        todo!()
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

        let Some(buffer) = self.multibuffer.read(cx).buffer(buffer_id) else {
            return;
        };

        let mut cursor = self.snapshot.transforms.cursor::<DiffTransformSummary>(&());
        let mut new_transforms = SumTree::default();

        let snapshot = buffer.read(cx);
        for (excerpt_id, multibuffer_range, buffer_range) in
            self.multibuffer.read(cx).ranges_for_buffer(buffer_id, cx)
        {
            let hunks = diff.hunks_intersecting_range(buffer_range.clone(), snapshot);
            let mut start = self
                .snapshot
                .fold_snapshot
                .make_fold_point(multibuffer_range.start, Bias::Left);
            let end = self
                .snapshot
                .fold_snapshot
                .make_fold_point(multibuffer_range.end, Bias::Right);

            new_transforms.append(cursor.slice(&start, Bias::Left, &()), &());
            start = FoldPoint(new_transforms.summary().input.lines);
            let mut old_tree = cursor.slice(&end, Bias::Right, &());
            if cursor.start().input.lines < end.0 {
                old_tree.extend(cursor.item().cloned(), &());
                cursor.next(&());
            }
            let old_expanded_hunk_anchors = old_tree
                .iter()
                .filter_map(|transform| {
                    if let DiffTransform::DeletedHunk { hunk_position, .. } = transform {
                        Some(*hunk_position)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>();

            let excerpt_start = buffer_range.start.to_point(snapshot);

            if let Some(base_text) = &base_text {
                for hunk in hunks {
                    let hunk_start_anchor = multi_buffer::Anchor {
                        excerpt_id,
                        buffer_id: Some(buffer_id),
                        text_anchor: hunk.buffer_range.start,
                    };
                    if !old_expanded_hunk_anchors.contains(&hunk_start_anchor)
                        && !self.all_hunks_expanded
                    {
                        continue;
                    }

                    if hunk.diff_base_byte_range.len() == 0 {
                        continue;
                    }
                    let mut text_cursor = base_text.as_rope().cursor(0);
                    let base_text_start =
                        text_cursor.summary::<Point>(hunk.diff_base_byte_range.start);
                    let base_text_summary = text_cursor.summary(hunk.diff_base_byte_range.end);

                    let hunk_start_in_excerpt =
                        hunk.buffer_range.start.to_point(snapshot) - excerpt_start;
                    let hunk_end_in_excerpt =
                        hunk.buffer_range.end.to_point(snapshot) - excerpt_start;
                    let hunk_start = multibuffer_range.start + hunk_start_in_excerpt;
                    let hunk_end = multibuffer_range.start + hunk_end_in_excerpt;
                    let hunk_start = self
                        .snapshot
                        .fold_snapshot
                        .make_fold_point(hunk_start, Bias::Left);
                    let hunk_end = self
                        .snapshot
                        .fold_snapshot
                        .make_fold_point(hunk_end, Bias::Left);

                    if hunk_start > start {
                        new_transforms.push(
                            DiffTransform::BufferContent {
                                summary: self
                                    .snapshot
                                    .fold_snapshot
                                    .text_summary_for_range(start..hunk_start),
                            },
                            &(),
                        );
                    }

                    start = hunk_start;

                    new_transforms.push(
                        DiffTransform::DeletedHunk {
                            hunk_position: hunk_start_anchor,
                            summary: base_text_summary,
                            base_text_start,
                        },
                        &(),
                    );
                }
            }

            if end > start {
                new_transforms.push(
                    DiffTransform::BufferContent {
                        summary: self
                            .snapshot
                            .fold_snapshot
                            .text_summary_for_range(start..end),
                    },
                    &(),
                );
            }
        }

        new_transforms.append(cursor.suffix(&()), &());

        drop(cursor);
        self.snapshot.transforms = new_transforms;

        #[cfg(test)]
        self.check_invariants();
    }

    pub(super) fn expand_diff_hunks(
        &mut self,
        multi_buffer_range: Range<usize>,
        cx: &mut ModelContext<Self>,
    ) {
    }

    pub(super) fn collapse_diff_hunks(
        &mut self,
        multi_buffer_range: Range<usize>,
        cx: &mut ModelContext<Self>,
    ) {
    }

    pub(super) fn set_all_hunks_expanded(&mut self, expand_all: bool, cx: &mut ModelContext<Self>) {
        self.all_hunks_expanded = expand_all;
        cx.notify()
    }

    fn snapshot(&self) -> DiffMapSnapshot {
        self.snapshot.clone()
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
                hunk_position,
                base_text_start,
            } => {
                let buffer_id = hunk_position.buffer_id?;
                let base_buffer = &self.snapshot.diffs.get(&buffer_id)?.base_text;

                let diff_base_start_offset = base_buffer.point_to_offset(*base_text_start);
                let diff_base_offset =
                    diff_base_start_offset + self.offset.0 - self.cursor.start().0 .0;
                let diff_base_end_offset = diff_base_start_offset + summary.len;

                let mut chunks = if let Some((_, mut chunks)) = self
                    .diff_base_chunks
                    .take()
                    .filter(|(id, _)| id == &buffer_id)
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
                self.diff_base_chunks = Some((buffer_id, chunks));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display_map::{fold_map::FoldMap, inlay_map::InlayMap};
    use gpui::{AppContext, TestAppContext};
    use indoc::indoc;
    use multi_buffer::MultiBuffer;
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

        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        let buffer_snapshot = buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let diff_map = cx.update(|cx| {
            let (diff_map, _) = DiffMap::new(fold_snapshot, buffer, cx);
            diff_map.update(cx, |diff_map, cx| {
                diff_map.set_all_hunks_expanded(true, cx);
            });
            diff_map
        });
        diff_map.update(cx, |diff_map, cx| diff_map.add_change_set(change_set, cx));
        cx.run_until_parked();

        assert_eq!(
            diff_map.update(cx, |diff_map, _| diff_map.snapshot().text()),
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

        diff_map.update(cx, |diff_map, cx| diff_map.set_all_hunks_expanded(true, cx));

        assert_eq!(
            diff_map.update(cx, |diff_map, cx| diff_map.snapshot().text()),
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
    }

    fn init_test(cx: &mut AppContext) {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        language::init(cx);
        crate::init(cx);
        Project::init_settings(cx);
    }
}

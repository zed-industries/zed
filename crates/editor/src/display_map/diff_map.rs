use super::fold_map::{FoldBufferRows, FoldChunks, FoldEdit, FoldOffset, FoldSnapshot};
use crate::{FoldPoint, Highlights};
use collections::HashMap;
use git::diff::BufferDiff;
use gpui::{AppContext, Context as _, Model, ModelContext, Subscription};
use language::{BufferChunks, BufferId, Chunk};
use multi_buffer::{AnchorRangeExt, ExcerptId, MultiBuffer};
use project::buffer_store::BufferChangeSet;
use std::{cmp::Ordering, mem, ops::Range};
use sum_tree::{Cursor, SumTree, TreeMap};
use text::{Bias, Edit, Patch, Point, TextSummary, ToPoint as _};

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
        let edits = patch.into_inner();
        (self.snapshot.clone(), edits)
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

        let multibuffer = self.multibuffer.read(cx);
        let changed_ranges = multibuffer
            .ranges_for_buffer(buffer_id, cx)
            .into_iter()
            .map(|(excerpt_id, range, buffer_range)| (excerpt_id, buffer_id, range, buffer_range))
            .collect();
        self.recompute_expanded_hunks(changed_ranges, cx);
    }

    pub(super) fn expand_diff_hunks(
        &mut self,
        ranges: Vec<Range<multi_buffer::Anchor>>,
        cx: &mut ModelContext<Self>,
    ) {
        let mut changed_ranges = Vec::new();
        {
            let multibuffer = self.multibuffer.read(cx);
            let snapshot = multibuffer.read(cx);
            for range in &ranges {
                let multibuffer_range = range.to_point(&*snapshot);
                for (buffer, buffer_range, excerpt_id) in
                    multibuffer.range_to_buffer_ranges(range.clone(), cx)
                {
                    let buffer = buffer.read(cx);
                    if let Some(excerpt_range) = snapshot.range_for_excerpt::<Point>(excerpt_id) {
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
        }

        self.recompute_expanded_hunks(changed_ranges, cx);
    }

    pub(super) fn collapse_diff_hunks(
        &mut self,
        multi_buffer_range: Range<usize>,
        cx: &mut ModelContext<Self>,
    ) {
    }

    pub(super) fn set_all_hunks_expanded(&mut self, expand_all: bool, cx: &mut ModelContext<Self>) {
        self.all_hunks_expanded = expand_all;
        let changes = self
            .multibuffer
            .read(cx)
            .read(cx)
            .all_excerpts()
            .map(|excerpt| {
                (
                    excerpt.id(),
                    excerpt.buffer().remote_id(),
                    excerpt.start_point()..excerpt.end_point(),
                    excerpt.buffer_range(),
                )
            })
            .collect();
        self.recompute_expanded_hunks(changes, cx);
    }

    fn recompute_expanded_hunks(
        &mut self,
        ranges: Vec<(ExcerptId, BufferId, Range<Point>, Range<text::Anchor>)>,
        cx: &mut ModelContext<DiffMap>,
    ) {
        let mut cursor = self
            .snapshot
            .transforms
            .cursor::<(FoldPoint, DiffOffset)>(&());
        let mut new_transforms = SumTree::default();

        for (excerpt_id, buffer_id, multibuffer_range, buffer_range) in ranges {
            let Some(buffer) = self.multibuffer.read(cx).buffer(buffer_id) else {
                continue;
            };
            let buffer = buffer.read(cx);

            let change_set_state = self.snapshot.diffs.get(&buffer_id);
            let diff = change_set_state
                .map(|state| state.diff.clone())
                .unwrap_or_else(|| BufferDiff::new(buffer));
            let base_text = change_set_state.map(|state| state.base_text.clone());

            let hunks = diff.hunks_intersecting_range(buffer_range.clone(), buffer);
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

            let mut old_expanded_hunks =
                HashMap::<multi_buffer::Anchor, Range<DiffOffset>>::default();
            while cursor.start().0 < end {
                let Some(item) = cursor.item() else {
                    break;
                };
                if let DiffTransform::DeletedHunk { hunk_position, .. } = item {
                    let range = cursor.start().1..cursor.end(&()).1;
                    old_expanded_hunks.insert(*hunk_position, range);
                }
                cursor.next(&());
            }

            let excerpt_start = buffer_range.start.to_point(buffer);
            let mut new_hunk_edits = Patch::default();

            if let Some(base_text) = &base_text {
                for hunk in hunks {
                    let hunk_start_anchor = multi_buffer::Anchor {
                        excerpt_id,
                        buffer_id: Some(buffer_id),
                        text_anchor: hunk.buffer_range.start,
                    };
                    let was_previously_expanded =
                        old_expanded_hunks.remove(&hunk_start_anchor).is_some();
                    if !was_previously_expanded && !self.all_hunks_expanded {
                        continue;
                    }

                    if hunk.diff_base_byte_range.len() == 0 {
                        continue;
                    }
                    let mut text_cursor = base_text.as_rope().cursor(0);
                    let base_text_start =
                        text_cursor.summary::<Point>(hunk.diff_base_byte_range.start);
                    let base_text_summary =
                        text_cursor.summary::<TextSummary>(hunk.diff_base_byte_range.end);

                    let hunk_start_in_excerpt =
                        hunk.buffer_range.start.to_point(buffer) - excerpt_start;
                    let hunk_start = multibuffer_range.start + hunk_start_in_excerpt;
                    let hunk_start = self
                        .snapshot
                        .fold_snapshot
                        .make_fold_point(hunk_start, Bias::Left);

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

                    if !was_previously_expanded {
                        let edit_start = DiffOffset(new_transforms.summary().output.len);

                        new_hunk_edits.push(DiffEdit {
                            old: edit_start..edit_start,
                            new: edit_start..(edit_start + DiffOffset(base_text_summary.len)),
                        });
                    }

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

            let mut old_hunk_edits = old_expanded_hunks
                .values()
                .map(|range| Edit {
                    old: range.clone(),
                    new: range.start..range.start,
                })
                .collect::<Vec<_>>();
            old_hunk_edits.sort_by(|a, b| a.old.start.cmp(&b.old.start));

            self.edits_since_sync = self
                .edits_since_sync
                .compose(Patch::new(old_hunk_edits))
                .compose(new_hunk_edits);

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
        cx.notify();

        #[cfg(test)]
        self.check_invariants();
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
        let (diff_map, _) = cx.update(|cx| DiffMap::new(fold_snapshot.clone(), buffer, cx));
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

        diff_map.update(cx, |diff_map, cx| diff_map.set_all_hunks_expanded(true, cx));

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
            diff_map.set_all_hunks_expanded(false, cx)
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

    fn check_edits(
        old_snapshot: &DiffMapSnapshot,
        new_snapshot: &DiffMapSnapshot,
        edits: &[DiffEdit],
    ) {
        let mut text = old_snapshot.text();
        let new_text = new_snapshot.text();
        for edit in edits.iter().rev() {
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
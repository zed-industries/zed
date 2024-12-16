use super::custom_highlights::CustomHighlightsChunks;
use crate::RowInfo;
use collections::HashMap;
use git::diff::DiffHunkStatus;
use gpui::{AppContext, Context as _, HighlightStyle, Model, ModelContext, Subscription};
use language::{BufferChunks, BufferId, Chunk};
use multi_buffer::{
    Anchor, AnchorRangeExt, MultiBuffer, MultiBufferDiffHunk, MultiBufferRow, MultiBufferRows,
    MultiBufferSnapshot, ToOffset, ToPoint,
};
use project::buffer_store::BufferChangeSet;
use std::{any::TypeId, mem, ops::Range, sync::Arc};
use sum_tree::{Cursor, SumTree, TreeMap};
use text::{Bias, Edit, Patch, Point, TextSummary, ToOffset as _};

pub(crate) struct DiffMap {
    snapshot: DiffMapSnapshot,
    multibuffer: Model<MultiBuffer>,
    diff_bases: HashMap<BufferId, ChangeSetState>,
    all_hunks_expanded: bool,
    edits_since_sync: Patch<DiffOffset>,
}

struct ChangeSetState {
    _change_set: Model<BufferChangeSet>,
    _subscription: Subscription,
}

#[derive(Clone)]
struct DiffSnapshot {
    diff: git::diff::BufferDiff,
    base_text: language::BufferSnapshot,
}

#[derive(Clone)]
pub struct DiffMapSnapshot {
    diffs: TreeMap<BufferId, DiffSnapshot>,
    transforms: SumTree<DiffTransform>,
    pub(crate) version: usize,
    pub(crate) buffer: MultiBufferSnapshot,
}

#[derive(Debug, Clone)]
enum DiffTransform {
    BufferContent {
        summary: TextSummary,
        is_inserted_hunk: bool,
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
    multibuffer_map: TextSummary,
    diff_map: TextSummary,
}

impl DiffTransformSummary {
    pub fn multibuffer_point(&self) -> Point {
        self.multibuffer_map.lines
    }
    pub fn multibuffer_offset(&self) -> usize {
        self.multibuffer_map.len
    }
    pub fn diff_point(&self) -> DiffPoint {
        DiffPoint(self.diff_map.lines)
    }
    pub fn diff_offset(&self) -> DiffOffset {
        DiffOffset(self.diff_map.len)
    }
}

pub struct DiffMapChunks<'a> {
    snapshot: &'a DiffMapSnapshot,
    language_aware: bool,
    cursor: Cursor<'a, DiffTransform, (DiffOffset, usize)>,
    multibuffer_chunks: CustomHighlightsChunks<'a>,
    multibuffer_chunk: Option<Chunk<'a>>,
    multibuffer_offset: usize,
    offset: DiffOffset,
    end_offset: DiffOffset,
    diff_base_chunks: Option<(BufferId, BufferChunks<'a>)>,
}

#[derive(Clone)]
pub struct DiffMapRows<'a> {
    cursor: Cursor<'a, DiffTransform, (DiffPoint, Point)>,
    diff_point: DiffPoint,
    input_buffer_rows: MultiBufferRows<'a>,
}

pub type DiffEdit = text::Edit<DiffOffset>;

enum ChangeKind {
    DiffUpdated,
    InputEdited,
    ExpandOrCollapseHunks { range: Range<usize>, expand: bool },
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct DiffOffset(pub usize);

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct DiffPoint(pub Point);

impl DiffPoint {
    pub fn new(row: u32, col: u32) -> Self {
        DiffPoint(Point::new(row, col))
    }

    pub fn row(&self) -> u32 {
        self.0.row
    }
    pub fn column(&self) -> u32 {
        self.0.column
    }
}

impl DiffMap {
    pub fn new(
        multibuffer: Model<MultiBuffer>,
        cx: &mut AppContext,
    ) -> (Model<Self>, DiffMapSnapshot) {
        let multibuffer_snapshot = multibuffer.read(cx).snapshot(cx);
        let snapshot = DiffMapSnapshot {
            diffs: TreeMap::default(),
            version: 0,
            transforms: SumTree::from_item(
                DiffTransform::BufferContent {
                    summary: multibuffer_snapshot.text_summary(),
                    is_inserted_hunk: false,
                },
                &(),
            ),
            buffer: multibuffer_snapshot,
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
                _change_set: change_set,
            },
        );
    }

    pub fn sync(
        &mut self,
        multibuffer_snapshot: MultiBufferSnapshot,
        buffer_edits: Vec<text::Edit<usize>>,
        cx: &mut ModelContext<Self>,
    ) -> (DiffMapSnapshot, Vec<DiffEdit>) {
        let changes = buffer_edits
            .iter()
            .map(|edit| (edit.clone(), ChangeKind::InputEdited))
            .collect::<Vec<_>>();

        self.snapshot.buffer = multibuffer_snapshot.clone();
        self.recompute_transforms(changes, cx);

        (
            self.snapshot.clone(),
            mem::take(&mut self.edits_since_sync).into_inner(),
        )
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
        let multibuffer_snapshot = self.snapshot.buffer();
        let changes = multibuffer
            .ranges_for_buffer(buffer_id, cx)
            .into_iter()
            .map(|(_, range, _)| {
                let multibuffer_start = multibuffer_snapshot.point_to_offset(range.start);
                let multibuffer_end = multibuffer_snapshot.point_to_offset(range.end);
                (
                    text::Edit {
                        old: multibuffer_start..multibuffer_end,
                        new: multibuffer_start..multibuffer_end,
                    },
                    ChangeKind::DiffUpdated,
                )
            })
            .collect();
        self.recompute_transforms(changes, cx);
    }

    pub(super) fn has_expanded_diff_hunks_in_ranges(
        &self,
        ranges: &[Range<multi_buffer::Anchor>],
    ) -> bool {
        let mut cursor = self.snapshot.transforms.cursor::<usize>(&());
        let multibuffer_snapshot = self.snapshot.buffer();
        for range in ranges {
            let range = range.to_point(multibuffer_snapshot);
            let start = multibuffer_snapshot.point_to_offset(Point::new(range.start.row, 0));
            let end = multibuffer_snapshot.point_to_offset(Point::new(range.end.row + 1, 0));
            let start = start.saturating_sub(1);
            let end = multibuffer_snapshot.len().min(end + 1);
            cursor.seek(&start, Bias::Right, &());
            while *cursor.start() < end {
                match cursor.item() {
                    Some(DiffTransform::DeletedHunk { .. })
                    | Some(DiffTransform::BufferContent {
                        is_inserted_hunk: true,
                        ..
                    }) => return true,
                    _ => {}
                }
                cursor.next(&());
            }
        }
        false
    }

    pub(super) fn expand_diff_hunks(
        &mut self,
        ranges: Vec<Range<multi_buffer::Anchor>>,
        cx: &mut ModelContext<Self>,
    ) {
        self.expand_or_collapse_diff_hunks(ranges, true, cx);
    }

    pub(super) fn collapse_diff_hunks(
        &mut self,
        ranges: Vec<Range<multi_buffer::Anchor>>,
        cx: &mut ModelContext<Self>,
    ) {
        self.expand_or_collapse_diff_hunks(ranges, false, cx);
    }

    pub(super) fn set_all_hunks_expanded(&mut self, cx: &mut ModelContext<Self>) {
        self.all_hunks_expanded = true;
        self.expand_or_collapse_diff_hunks(vec![Anchor::min()..Anchor::max()], true, cx);
    }

    fn expand_or_collapse_diff_hunks(
        &mut self,
        ranges: Vec<Range<multi_buffer::Anchor>>,
        expand: bool,
        cx: &mut ModelContext<Self>,
    ) {
        let multibuffer_snapshot = self.snapshot.buffer();
        let mut changes = Vec::new();
        for range in ranges.iter() {
            let multibuffer_start = range.start.to_point(multibuffer_snapshot);
            let multibuffer_end = range.end.to_point(multibuffer_snapshot);
            let multibuffer_start =
                multibuffer_snapshot.point_to_offset(Point::new(multibuffer_start.row, 0));
            let multibuffer_end =
                multibuffer_snapshot.point_to_offset(Point::new(multibuffer_end.row + 1, 0));
            let expanded_start = multibuffer_start.saturating_sub(1);
            let expanded_end = multibuffer_snapshot.len().min(multibuffer_end + 1);
            changes.push((
                text::Edit {
                    old: expanded_start..expanded_end,
                    new: expanded_start..expanded_end,
                },
                ChangeKind::ExpandOrCollapseHunks {
                    range: multibuffer_start..multibuffer_end,
                    expand,
                },
            ));
        }

        self.recompute_transforms(changes, cx);
    }

    fn recompute_transforms(
        &mut self,
        changes: Vec<(text::Edit<usize>, ChangeKind)>,
        cx: &mut ModelContext<DiffMap>,
    ) {
        let multibuffer = self.multibuffer.read(cx);
        let multibuffer_snapshot = self.snapshot.buffer();

        let mut cursor = self.snapshot.transforms.cursor::<(usize, DiffOffset)>(&());
        let mut new_transforms = SumTree::default();
        let mut edits = Patch::default();

        let mut changes = changes.into_iter().peekable();
        let mut delta = 0_isize;
        while let Some((mut edit, mut operation)) = changes.next() {
            let to_skip = cursor.slice(&edit.old.start, Bias::Right, &());
            self.append_transforms(&mut new_transforms, to_skip);

            let mut end_of_current_insert = 0;
            loop {
                let multibuffer_range = edit.new.clone();
                let edit_old_start =
                    cursor.start().1 + DiffOffset(edit.old.start - cursor.start().0);
                let mut edit_old_end =
                    cursor.start().1 + DiffOffset(edit.old.end - cursor.start().0);

                for (buffer, buffer_range, excerpt_id) in
                    multibuffer.range_to_buffer_ranges(multibuffer_range.clone(), cx)
                {
                    let excerpt_range = multibuffer_snapshot
                        .range_for_excerpt::<usize>(excerpt_id)
                        .unwrap();
                    let excerpt_buffer_range = multibuffer_snapshot
                        .buffer_range_for_excerpt(excerpt_id)
                        .unwrap();
                    let buffer_id = buffer.read(cx).remote_id();
                    let diff_state = self.snapshot.diffs.get(&buffer_id);

                    let buffer = buffer.read(cx);
                    let buffer_anchor_range = buffer.anchor_before(buffer_range.start)
                        ..buffer.anchor_after(buffer_range.end);
                    let change_start_buffer_offset = buffer_range.start;
                    if let Some(diff_state) = diff_state {
                        let diff = &diff_state.diff;
                        let base_text = &diff_state.base_text;

                        for hunk in diff.hunks_intersecting_range(buffer_anchor_range, buffer) {
                            let hunk_start_buffer_offset =
                                hunk.buffer_range.start.to_offset(buffer);
                            let hunk_end_buffer_offset = hunk.buffer_range.end.to_offset(buffer);

                            let excerpt_buffer_range_start_offset =
                                excerpt_buffer_range.start.to_offset(buffer);
                            let hunk_start_multibuffer_offset = excerpt_range.start
                                + hunk_start_buffer_offset
                                - excerpt_buffer_range_start_offset;

                            self.push_buffer_content_transform(
                                &mut new_transforms,
                                hunk_start_multibuffer_offset,
                                end_of_current_insert,
                            );

                            while cursor.end(&()).0 < hunk_start_multibuffer_offset
                                || (cursor.end(&()).0 == hunk_start_multibuffer_offset
                                    && cursor.start().0 < hunk_start_multibuffer_offset)
                            {
                                let Some(item) = cursor.item() else {
                                    break;
                                };
                                if let DiffTransform::DeletedHunk { .. } = item {
                                    let old_range = cursor.start().1..cursor.end(&()).1;
                                    let new_offset =
                                        DiffOffset((old_range.start.0 as isize + delta) as usize);
                                    delta -= (old_range.end - old_range.start).0 as isize;
                                    let edit = Edit {
                                        old: old_range,
                                        new: new_offset..new_offset,
                                    };
                                    edits.push(edit);
                                }
                                cursor.next(&());
                            }

                            let mut was_previously_expanded = false;
                            if cursor.start().0 == hunk_start_multibuffer_offset {
                                was_previously_expanded = match cursor.item() {
                                    Some(DiffTransform::DeletedHunk { .. }) => true,
                                    Some(DiffTransform::BufferContent {
                                        is_inserted_hunk, ..
                                    }) => *is_inserted_hunk,
                                    None => false,
                                };
                            }

                            let hunk_is_deletion =
                                hunk_start_buffer_offset == hunk_end_buffer_offset;

                            let mut should_expand_hunk =
                                was_previously_expanded || self.all_hunks_expanded;
                            if let ChangeKind::ExpandOrCollapseHunks { range, expand } = &operation
                            {
                                let intersects = hunk_is_deletion
                                    || (hunk_start_buffer_offset < range.end
                                        && hunk_end_buffer_offset > range.start);
                                if *expand {
                                    should_expand_hunk |= intersects;
                                } else {
                                    should_expand_hunk &= !intersects;
                                }
                            };

                            if should_expand_hunk {
                                if hunk.diff_base_byte_range.len() > 0
                                    && hunk_start_buffer_offset >= change_start_buffer_offset
                                {
                                    if !was_previously_expanded {
                                        let hunk_overshoot =
                                            hunk_start_multibuffer_offset - cursor.start().0;
                                        let old_offset =
                                            cursor.start().1 + DiffOffset(hunk_overshoot);
                                        let new_start =
                                            DiffOffset(new_transforms.summary().diff_map.len);
                                        let new_end =
                                            new_start + DiffOffset(hunk.diff_base_byte_range.len());
                                        delta += hunk.diff_base_byte_range.len() as isize;
                                        let edit = Edit {
                                            old: old_offset..old_offset,
                                            new: new_start..new_end,
                                        };
                                        edits.push(edit);
                                    }

                                    let mut text_cursor = base_text.as_rope().cursor(0);
                                    let base_text_start = text_cursor
                                        .summary::<Point>(hunk.diff_base_byte_range.start);
                                    let base_text_summary = text_cursor
                                        .summary::<TextSummary>(hunk.diff_base_byte_range.end);
                                    new_transforms.push(
                                        DiffTransform::DeletedHunk {
                                            base_text_byte_range: hunk.diff_base_byte_range.clone(),
                                            summary: base_text_summary,
                                            buffer_id,
                                            base_text_start,
                                        },
                                        &(),
                                    );
                                }

                                if hunk_end_buffer_offset > hunk_start_buffer_offset {
                                    let hunk_end_multibuffer_offset = excerpt_range.start
                                        + hunk_end_buffer_offset
                                        - excerpt_buffer_range_start_offset;
                                    end_of_current_insert = hunk_end_multibuffer_offset;
                                }

                                if was_previously_expanded {
                                    cursor.next(&());
                                }
                            }
                        }
                    }
                }

                while cursor.end(&()).0 <= edit.old.end {
                    let Some(item) = cursor.item() else {
                        break;
                    };
                    if let DiffTransform::DeletedHunk { .. } = item {
                        let old_range = cursor.start().1..cursor.end(&()).1;
                        let new_offset = DiffOffset((old_range.start.0 as isize + delta) as usize);
                        delta -= (old_range.end - old_range.start).0 as isize;
                        let edit = Edit {
                            old: old_range,
                            new: new_offset..new_offset,
                        };
                        edits.push(edit);
                    }

                    edit_old_end = cursor.start().1 + DiffOffset(edit.old.end - cursor.start().0);

                    cursor.next(&());
                }

                self.push_buffer_content_transform(
                    &mut new_transforms,
                    edit.new.end,
                    end_of_current_insert,
                );

                if let ChangeKind::InputEdited = operation {
                    let edit_new_start = DiffOffset((edit_old_start.0 as isize + delta) as usize);
                    delta += (edit.new.end - edit.new.start) as isize
                        - (edit.old.end - edit.old.start) as isize;
                    let edit_new_end = DiffOffset((edit_old_end.0 as isize + delta) as usize);
                    let edit = DiffEdit {
                        old: edit_old_start..edit_old_end,
                        new: edit_new_start..edit_new_end,
                    };
                    edits.push(edit);
                }

                if let Some((next_edit, _)) = changes.peek() {
                    if next_edit.old.start < cursor.end(&()).0 {
                        (edit, operation) = changes.next().unwrap();
                        continue;
                    }
                }

                let suffix = cursor.end(&()).0 - edit.old.end;
                let transform_end = new_transforms.summary().multibuffer_map.len + suffix;
                self.push_buffer_content_transform(
                    &mut new_transforms,
                    transform_end,
                    end_of_current_insert,
                );
                cursor.next(&());
                break;
            }
        }

        self.append_transforms(&mut new_transforms, cursor.suffix(&()));
        self.edits_since_sync = self.edits_since_sync.compose(edits);

        drop(cursor);
        self.snapshot.transforms = new_transforms;
        self.snapshot.version += 1;
        cx.notify();

        #[cfg(test)]
        self.check_invariants();
    }

    fn append_transforms(
        &self,
        new_transforms: &mut SumTree<DiffTransform>,
        subtree: SumTree<DiffTransform>,
    ) {
        if let Some(DiffTransform::BufferContent {
            is_inserted_hunk,
            summary,
        }) = subtree.first()
        {
            if self.extend_last_buffer_content_transform(
                new_transforms,
                *is_inserted_hunk,
                summary.clone(),
            ) {
                let mut cursor = subtree.cursor::<()>(&());
                cursor.next(&());
                cursor.next(&());
                new_transforms.append(cursor.suffix(&()), &());
                return;
            }
        }
        new_transforms.append(subtree, &());
    }

    fn push_buffer_content_transform(
        &self,
        new_transforms: &mut SumTree<DiffTransform>,
        end_offset: usize,
        end_of_current_inserted_hunk: usize,
    ) {
        for (end_offset, region_is_inserted_hunk) in [
            (end_offset.min(end_of_current_inserted_hunk), true),
            (end_offset, false),
        ] {
            let start_offset = new_transforms.summary().multibuffer_map.len;
            if end_offset <= start_offset {
                continue;
            }
            let summary_to_add = self
                .snapshot
                .buffer
                .text_summary_for_range::<TextSummary, _>(start_offset..end_offset);

            if !self.extend_last_buffer_content_transform(
                new_transforms,
                region_is_inserted_hunk,
                summary_to_add.clone(),
            ) {
                new_transforms.push(
                    DiffTransform::BufferContent {
                        summary: summary_to_add,
                        is_inserted_hunk: region_is_inserted_hunk,
                    },
                    &(),
                )
            }
        }
    }

    fn extend_last_buffer_content_transform(
        &self,
        new_transforms: &mut SumTree<DiffTransform>,
        region_is_inserted_hunk: bool,
        summary_to_add: TextSummary,
    ) -> bool {
        let mut did_extend = false;
        new_transforms.update_last(
            |last_transform| {
                if let DiffTransform::BufferContent {
                    summary,
                    is_inserted_hunk,
                } = last_transform
                {
                    if *is_inserted_hunk == region_is_inserted_hunk {
                        *summary += summary_to_add.clone();
                        did_extend = true;
                    }
                }
            },
            &(),
        );
        did_extend
    }

    #[cfg(test)]
    fn check_invariants(&self) {
        let snapshot = &self.snapshot;
        if snapshot.transforms.summary().multibuffer_map.len != snapshot.buffer.len() {
            panic!(
                "incorrect input length. expected {}, got {}. transforms: {:+?}",
                snapshot.buffer.len(),
                snapshot.transforms.summary().multibuffer_map.len,
                snapshot.transforms.items(&()),
            );
        }

        let mut prev_transform: Option<&DiffTransform> = None;
        for item in snapshot.transforms.iter() {
            if let DiffTransform::BufferContent {
                summary,
                is_inserted_hunk,
            } = item
            {
                if let Some(DiffTransform::BufferContent {
                    is_inserted_hunk: prev_is_inserted_hunk,
                    ..
                }) = prev_transform
                {
                    if *is_inserted_hunk == *prev_is_inserted_hunk {
                        panic!(
                            "multiple adjacent buffer content transforms with is_inserted_hunk = {is_inserted_hunk}. transforms: {:+?}",
                            snapshot.transforms.items(&()));
                    }
                }
                if summary.len == 0 && !snapshot.buffer().is_empty() {
                    panic!("empty buffer content transform");
                }
            }
            prev_transform = Some(item);
        }
    }
}

impl DiffMapSnapshot {
    pub fn diff_hunks_in_range<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
    ) -> impl Iterator<Item = MultiBufferDiffHunk> + 'a {
        let buffer_snapshot = self.buffer();
        let range = range.start.to_offset(buffer_snapshot)..range.end.to_offset(buffer_snapshot);
        buffer_snapshot
            .excerpts_for_range(range.clone())
            .filter_map(move |excerpt| {
                let buffer = excerpt.buffer();
                let buffer_id = buffer.remote_id();
                let diff = &self.diffs.get(&buffer_id)?.diff;
                let buffer_range = excerpt.map_range_to_buffer(range.clone());
                let buffer_range =
                    buffer.anchor_before(buffer_range.start)..buffer.anchor_after(buffer_range.end);
                Some(
                    diff.hunks_intersecting_range(buffer_range, excerpt.buffer())
                        .map(move |hunk| {
                            let start =
                                excerpt.map_point_from_buffer(Point::new(hunk.row_range.start, 0));
                            let end =
                                excerpt.map_point_from_buffer(Point::new(hunk.row_range.end, 0));
                            MultiBufferDiffHunk {
                                row_range: MultiBufferRow(start.row)..MultiBufferRow(end.row),
                                buffer_id,
                                buffer_range: hunk.buffer_range.clone(),
                                diff_base_byte_range: hunk.diff_base_byte_range.clone(),
                            }
                        }),
                )
            })
            .flatten()
    }

    pub fn diff_hunks_in_range_rev<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
    ) -> impl Iterator<Item = MultiBufferDiffHunk> + 'a {
        let buffer_snapshot = self.buffer();
        let range = range.start.to_offset(buffer_snapshot)..range.end.to_offset(buffer_snapshot);
        buffer_snapshot
            .excerpts_for_range_rev(range.clone())
            .filter_map(move |excerpt| {
                let buffer = excerpt.buffer();
                let buffer_id = buffer.remote_id();
                let diff = &self.diffs.get(&buffer_id)?.diff;
                let buffer_range = excerpt.map_range_to_buffer(range.clone());
                let buffer_range =
                    buffer.anchor_before(buffer_range.start)..buffer.anchor_after(buffer_range.end);
                Some(
                    diff.hunks_intersecting_range_rev(buffer_range, excerpt.buffer())
                        .map(move |hunk| {
                            let start_row = excerpt
                                .map_point_from_buffer(Point::new(hunk.row_range.start, 0))
                                .row;
                            let end_row = excerpt
                                .map_point_from_buffer(Point::new(hunk.row_range.end, 0))
                                .row;
                            MultiBufferDiffHunk {
                                row_range: MultiBufferRow(start_row)..MultiBufferRow(end_row),
                                buffer_id,
                                buffer_range: hunk.buffer_range.clone(),
                                diff_base_byte_range: hunk.diff_base_byte_range.clone(),
                            }
                        }),
                )
            })
            .flatten()
    }

    pub fn has_diff_hunks(&self) -> bool {
        self.diffs.values().any(|diff| !diff.diff.is_empty())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn text(&self) -> String {
        self.chunks(DiffOffset(0)..self.len(), false, None)
            .map(|c| c.text)
            .collect()
    }

    pub fn len(&self) -> DiffOffset {
        DiffOffset(self.transforms.summary().diff_map.len)
    }

    pub fn max_point(&self) -> DiffPoint {
        DiffPoint(self.transforms.summary().diff_map.lines)
    }

    pub fn text_summary(&self) -> TextSummary {
        self.transforms.summary().diff_map.clone()
    }

    pub fn text_summary_for_range(&self, range: Range<DiffOffset>) -> TextSummary {
        let mut cursor = self.transforms.cursor::<(DiffOffset, usize)>(&());
        cursor.seek(&range.start, Bias::Right, &());

        let Some(first_transform) = cursor.item() else {
            return TextSummary::default();
        };

        let (diff_transform_start, multibuffer_transform_start) = cursor.start().clone();
        let (diff_transform_end, _) = cursor.end(&());
        let diff_start = range.start;
        let diff_end = std::cmp::min(range.end, diff_transform_end);

        let mut result = match first_transform {
            DiffTransform::BufferContent { .. } => {
                let multibuffer_start =
                    multibuffer_transform_start + (diff_start - diff_transform_start).0;
                let multibuffer_end =
                    multibuffer_transform_start + (diff_end - diff_transform_start).0;

                self.buffer
                    .text_summary_for_range(multibuffer_start..multibuffer_end)
            }
            DiffTransform::DeletedHunk {
                buffer_id,
                base_text_byte_range,
                ..
            } => {
                let buffer_start =
                    base_text_byte_range.start + (diff_start - diff_transform_start).0;
                let buffer_end = base_text_byte_range.start + (diff_end - diff_transform_start).0;
                let Some(buffer_diff) = self.diffs.get(buffer_id) else {
                    panic!("{:?} is in non-extant deleted hunk", range.start)
                };

                buffer_diff
                    .base_text
                    .text_summary_for_range(buffer_start..buffer_end)
            }
        };
        if range.end < diff_transform_end {
            return result;
        }

        cursor.next(&());
        result = result + cursor.summary(&range.end, Bias::Right, &());

        let Some(last_transform) = cursor.item() else {
            return result;
        };

        let (diff_transform_start, multibuffer_transform_start) = cursor.start().clone();

        result += match last_transform {
            DiffTransform::BufferContent { .. } => {
                let multibuffer_end =
                    multibuffer_transform_start + (range.end - diff_transform_start).0;

                self.buffer.text_summary_for_range::<TextSummary, _>(
                    multibuffer_transform_start..multibuffer_end,
                )
            }
            DiffTransform::DeletedHunk {
                base_text_byte_range,
                buffer_id,
                ..
            } => {
                let buffer_end = base_text_byte_range.start + (range.end - diff_transform_start).0;
                let Some(buffer_diff) = self.diffs.get(buffer_id) else {
                    panic!("{:?} is in non-extant deleted hunk", range.end)
                };

                buffer_diff
                    .base_text
                    .text_summary_for_range(base_text_byte_range.start..buffer_end)
            }
        };

        result
    }

    pub fn buffer(&self) -> &MultiBufferSnapshot {
        &self.buffer
    }

    pub fn offset_to_point(&self, offset: DiffOffset) -> DiffPoint {
        let mut cursor = self.transforms.cursor::<DiffTransformSummary>(&());
        cursor.seek(&offset, Bias::Right, &());
        let start_transform = cursor.start();
        let overshoot = offset - start_transform.diff_offset();
        if overshoot.0 == 0 {
            return start_transform.diff_point();
        }

        match cursor.item() {
            Some(DiffTransform::BufferContent { .. }) => {
                let multibuffer_offset = start_transform.multibuffer_offset() + overshoot.0;
                let multibuffer_point = self.buffer.offset_to_point(multibuffer_offset);
                start_transform.diff_point()
                    + DiffPoint(multibuffer_point - start_transform.multibuffer_point())
            }
            Some(DiffTransform::DeletedHunk {
                buffer_id,
                base_text_start,
                base_text_byte_range,
                ..
            }) => {
                let Some(buffer_diff) = self.diffs.get(buffer_id) else {
                    panic!("{:?} is in non-extant deleted hunk", offset)
                };
                let buffer_offset = base_text_byte_range.start + overshoot.0;
                let buffer_point = buffer_diff.base_text.offset_to_point(buffer_offset);
                start_transform.diff_point() + DiffPoint(buffer_point - base_text_start)
            }
            None => {
                panic!("{:?} is past end of buffer", offset)
            }
        }
    }

    pub fn clip_point(&self, point: DiffPoint, bias: Bias) -> DiffPoint {
        let mut cursor = self.transforms.cursor::<DiffTransformSummary>(&());
        cursor.seek(&point, Bias::Right, &());
        let start_transform = cursor.start();
        let overshoot = point - start_transform.diff_point();
        if overshoot.0.is_zero() {
            return start_transform.diff_point();
        }

        match cursor.item() {
            Some(DiffTransform::BufferContent { .. }) => {
                let inlay_point = start_transform.multibuffer_point() + overshoot.0;
                let clipped = self.buffer.clip_point(inlay_point, bias);
                start_transform.diff_point()
                    + DiffPoint(clipped - start_transform.multibuffer_point())
            }
            Some(DiffTransform::DeletedHunk {
                buffer_id,
                base_text_start,
                ..
            }) => {
                let Some(buffer_diff) = self.diffs.get(buffer_id) else {
                    panic!("{:?} is in non-extant deleted hunk", point)
                };
                let buffer_point = *base_text_start + overshoot.0;
                let clipped = buffer_diff.base_text.clip_point(buffer_point, bias);
                start_transform.diff_point() + DiffPoint(clipped - base_text_start)
            }
            None => cursor.end(&()).diff_point(),
        }
    }

    pub fn point_to_offset(&self, point: DiffPoint) -> DiffOffset {
        let mut cursor = self.transforms.cursor::<DiffTransformSummary>(&());
        cursor.seek(&point, Bias::Right, &());
        let start_transform = cursor.start();
        let overshoot = point - start_transform.diff_point();
        if overshoot.0.is_zero() {
            return start_transform.diff_offset();
        }

        match cursor.item() {
            Some(DiffTransform::BufferContent { .. }) => {
                let multibuffer_point = start_transform.multibuffer_point() + overshoot.0;
                let multibuffer_offset = self.buffer.point_to_offset(multibuffer_point);
                start_transform.diff_offset()
                    + DiffOffset(multibuffer_offset - start_transform.multibuffer_offset())
            }
            Some(DiffTransform::DeletedHunk {
                buffer_id,
                base_text_start,
                base_text_byte_range,
                ..
            }) => {
                let Some(buffer_diff) = self.diffs.get(buffer_id) else {
                    panic!("{:?} is in non-extant deleted hunk", point)
                };
                let buffer_point = *base_text_start + overshoot.0;
                let buffer_offset = buffer_diff.base_text.point_to_offset(buffer_point);
                start_transform.diff_offset()
                    + DiffOffset(buffer_offset - base_text_byte_range.start)
            }
            None => {
                panic!("{:?} is past end of buffer", point)
            }
        }
    }

    pub fn to_multibuffer_offset(&self, offset: DiffOffset) -> usize {
        let mut cursor = self.transforms.cursor::<(DiffOffset, usize)>(&());
        cursor.seek(&offset, Bias::Right, &());
        let mut multibuffer_offset = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = offset.0 - cursor.start().0 .0;
            multibuffer_offset += overshoot;
        }
        multibuffer_offset
    }

    pub fn to_multibuffer_point(&self, point: DiffPoint) -> Point {
        let mut cursor = self.transforms.cursor::<(DiffPoint, Point)>(&());
        cursor.seek(&point, Bias::Right, &());
        let mut inlay_point = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = point.0 - cursor.start().0 .0;
            inlay_point += overshoot;
        }
        inlay_point
    }

    pub fn to_diff_offset(&self, multibuffer_offset: usize) -> DiffOffset {
        let mut cursor = self.transforms.cursor::<(usize, DiffOffset)>(&());
        cursor.seek(&multibuffer_offset, Bias::Right, &());
        let mut diff_offset = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = multibuffer_offset - cursor.start().0;
            diff_offset.0 += overshoot;
        }
        diff_offset
    }

    pub fn to_diff_point(&self, multibuffer_point: Point) -> DiffPoint {
        let mut cursor = self.transforms.cursor::<(Point, DiffPoint)>(&());
        cursor.seek(&multibuffer_point, Bias::Right, &());
        let mut diff_point = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = multibuffer_point - cursor.start().0;
            diff_point.0 += overshoot;
        }
        diff_point
    }

    pub(crate) fn chunks<'a>(
        &'a self,
        range: Range<DiffOffset>,
        language_aware: bool,
        text_highlights: Option<&'a TreeMap<TypeId, Arc<(HighlightStyle, Vec<Range<Anchor>>)>>>,
    ) -> DiffMapChunks<'a> {
        let mut cursor = self.transforms.cursor::<(DiffOffset, usize)>(&());

        cursor.seek(&range.end, Bias::Right, &());
        let mut multibuffer_end = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = range.end.0 - cursor.start().0 .0;
            multibuffer_end += overshoot;
        }

        cursor.seek(&range.start, Bias::Right, &());
        let mut multibuffer_start = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = range.start.0 - cursor.start().0 .0;
            multibuffer_start += overshoot;
        }

        let multibuffer_chunks = CustomHighlightsChunks::new(
            multibuffer_start..multibuffer_end,
            language_aware,
            text_highlights,
            &self.buffer,
        );

        DiffMapChunks {
            snapshot: self,
            language_aware,
            cursor,
            multibuffer_chunk: None,
            multibuffer_chunks,
            multibuffer_offset: multibuffer_start,
            offset: range.start,
            diff_base_chunks: None,
            end_offset: range.end,
        }
    }

    pub fn row_infos(&self, start_row: u32) -> DiffMapRows {
        if start_row > self.transforms.summary().diff_map.lines.row {
            panic!("invalid diff map row {}", start_row);
        }

        let diff_point = DiffPoint(Point::new(start_row, 0));
        let mut cursor = self.transforms.cursor::<(DiffPoint, Point)>(&());
        cursor.seek(&diff_point, Bias::Right, &());

        let (diff_transform_start, inlay_transform_start) = cursor.start().clone();

        let overshoot = if matches!(cursor.item(), Some(DiffTransform::BufferContent { .. })) {
            diff_point.row() - diff_transform_start.row()
        } else {
            0
        };
        let input_buffer_rows = self
            .buffer
            .buffer_rows(MultiBufferRow(inlay_transform_start.row + overshoot));

        DiffMapRows {
            diff_point,
            input_buffer_rows,
            cursor,
        }
    }
}

impl<'a> DiffMapChunks<'a> {
    pub fn seek(&mut self, range: Range<DiffOffset>) {
        self.cursor.seek(&range.end, Bias::Right, &());
        let mut inlay_end = self.cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = self.cursor.item() {
            let overshoot = range.end.0 - self.cursor.start().0 .0;
            inlay_end += overshoot;
        }

        self.cursor.seek(&range.start, Bias::Right, &());
        let mut inlay_start = self.cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = self.cursor.item() {
            let overshoot = range.start.0 - self.cursor.start().0 .0;
            inlay_start += overshoot;
        }

        self.multibuffer_chunks.seek(inlay_start..inlay_end);
        self.multibuffer_chunk.take();
        self.multibuffer_offset = inlay_start;
        self.offset = range.start;
        self.end_offset = range.end;
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
            DiffTransform::BufferContent { summary, .. } => {
                let chunk = self
                    .multibuffer_chunk
                    .get_or_insert_with(|| self.multibuffer_chunks.next().unwrap());

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
                    self.multibuffer_chunk.take()
                }
            }
            DiffTransform::DeletedHunk {
                buffer_id,
                base_text_byte_range,
                ..
            } => {
                let hunk_start_offset = self.cursor.start().0;
                let base_text_start_offset = base_text_byte_range.start;
                let base_text_end_offset = base_text_byte_range.end;
                let diff_base_end_offset = base_text_end_offset
                    .min(base_text_start_offset + self.end_offset.0 - hunk_start_offset.0);
                let diff_base_start_offset =
                    base_text_start_offset + self.offset.0 - hunk_start_offset.0;

                let mut chunks = if let Some((_, mut chunks)) = self
                    .diff_base_chunks
                    .take()
                    .filter(|(id, _)| id == buffer_id)
                {
                    if chunks.offset() != diff_base_start_offset {
                        chunks.seek(diff_base_start_offset..diff_base_end_offset);
                    }
                    chunks
                } else {
                    let base_buffer = &self.snapshot.diffs.get(&buffer_id)?.base_text;
                    base_buffer.chunks(
                        diff_base_start_offset..diff_base_end_offset,
                        self.language_aware,
                    )
                };

                let chunk = chunks.next()?;
                self.offset.0 += chunk.text.len();
                self.diff_base_chunks = Some((*buffer_id, chunks));
                Some(chunk)
            }
        }
    }
}

impl<'a> DiffMapRows<'a> {
    pub fn seek(&mut self, row: u32) {
        self.diff_point = DiffPoint::new(row, 0);
        self.cursor.seek(&self.diff_point, Bias::Right, &());
        let (diff_transform_start, inlay_transform_start) = self.cursor.start().clone();
        let overshoot = if matches!(
            self.cursor.item(),
            Some(DiffTransform::BufferContent { .. })
        ) {
            self.diff_point.row() - diff_transform_start.row()
        } else {
            0
        };
        self.input_buffer_rows
            .seek(MultiBufferRow(inlay_transform_start.row + overshoot));
    }
}

impl<'a> Iterator for DiffMapRows<'a> {
    type Item = RowInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.cursor.item() {
            Some(DiffTransform::DeletedHunk { .. }) => Some(RowInfo {
                buffer_row: None,
                diff_status: Some(DiffHunkStatus::Removed),
            }),
            Some(DiffTransform::BufferContent {
                is_inserted_hunk, ..
            }) => {
                let row = self.input_buffer_rows.next();
                row.map(|row| RowInfo {
                    buffer_row: row,
                    diff_status: if *is_inserted_hunk {
                        Some(DiffHunkStatus::Added)
                    } else {
                        None
                    },
                })
            }
            None => self.input_buffer_rows.next().map(|row| RowInfo {
                buffer_row: row,
                diff_status: None,
            }),
        };
        self.diff_point.0 += Point::new(1, 0);
        if self.diff_point >= self.cursor.end(&()).0 {
            self.cursor.next(&());
        }
        result
    }
}

impl sum_tree::Item for DiffTransform {
    type Summary = DiffTransformSummary;

    fn summary(&self, _: &<Self::Summary as sum_tree::Summary>::Context) -> Self::Summary {
        match self {
            DiffTransform::BufferContent { summary, .. } => DiffTransformSummary {
                multibuffer_map: summary.clone(),
                diff_map: summary.clone(),
            },
            DiffTransform::DeletedHunk { summary, .. } => DiffTransformSummary {
                multibuffer_map: TextSummary::default(),
                diff_map: summary.clone(),
            },
        }
    }
}

impl sum_tree::Summary for DiffTransformSummary {
    type Context = ();

    fn zero(_: &Self::Context) -> Self {
        DiffTransformSummary {
            multibuffer_map: TextSummary::default(),
            diff_map: TextSummary::default(),
        }
    }

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        self.multibuffer_map += &summary.multibuffer_map;
        self.diff_map += &summary.diff_map;
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for usize {
    fn zero(_: &()) -> Self {
        0
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        *self += summary.multibuffer_map.len
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for DiffOffset {
    fn zero(_: &()) -> Self {
        DiffOffset(0)
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        self.0 += summary.diff_map.len
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for Point {
    fn zero(_: &()) -> Self {
        Point::zero()
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        *self += summary.multibuffer_map.lines
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for DiffPoint {
    fn zero(_: &()) -> Self {
        DiffPoint(Point::zero())
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        self.0 += summary.diff_map.lines
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for TextSummary {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        *self += &summary.diff_map;
    }
}

impl<'a> sum_tree::SeekTarget<'a, DiffTransformSummary, DiffTransformSummary> for DiffPoint {
    fn cmp(
        &self,
        cursor_location: &DiffTransformSummary,
        _: &<DiffTransformSummary as sum_tree::Summary>::Context,
    ) -> std::cmp::Ordering {
        Ord::cmp(self, &cursor_location.diff_point())
    }
}

impl<'a> sum_tree::SeekTarget<'a, DiffTransformSummary, DiffTransformSummary> for DiffOffset {
    fn cmp(
        &self,
        cursor_location: &DiffTransformSummary,
        _: &<DiffTransformSummary as sum_tree::Summary>::Context,
    ) -> std::cmp::Ordering {
        Ord::cmp(self, &cursor_location.diff_offset())
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

impl std::ops::SubAssign<DiffOffset> for DiffOffset {
    fn sub_assign(&mut self, rhs: DiffOffset) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Add<DiffPoint> for DiffPoint {
    type Output = DiffPoint;

    fn add(self, rhs: DiffPoint) -> Self::Output {
        DiffPoint(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign<DiffPoint> for DiffPoint {
    fn add_assign(&mut self, rhs: DiffPoint) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub<DiffPoint> for DiffPoint {
    type Output = DiffPoint;

    fn sub(self, rhs: DiffPoint) -> Self::Output {
        DiffPoint(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, TestAppContext};
    use indoc::indoc;
    use language::Buffer;
    use multi_buffer::{Anchor, MultiBuffer};
    use project::Project;
    use settings::SettingsStore;
    use text::OffsetUtf16;

    #[gpui::test]
    fn test_basic_diff_map_updates(cx: &mut TestAppContext) {
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

        let (diff_map, mut snapshot, mut deps) = build_diff_map(text, Some(base_text), cx);
        assert_eq!(
            snapshot.text(),
            indoc!(
                "
                ZERO
                one
                TWO
                three
                six
                "
            ),
        );

        diff_map.update(cx, |diff_map, cx| {
            diff_map.expand_diff_hunks(vec![Anchor::min()..Anchor::max()], cx)
        });
        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                + ZERO
                  one
                - two
                + TWO
                  three
                - four
                - five
                  six
                "
            ),
        );

        assert_eq!(
            snapshot
                .row_infos(0)
                .map(|info| info.buffer_row)
                .collect::<Vec<_>>(),
            vec![
                Some(0),
                Some(1),
                None,
                Some(2),
                Some(3),
                None,
                None,
                Some(4),
                Some(5)
            ]
        );

        assert_chunks_in_ranges(&snapshot);
        assert_consistent_line_numbers(&snapshot);

        for (point, offset) in &[
            (
                DiffPoint::new(0, 0),
                DiffOffset(snapshot.text().find("ZERO").unwrap()),
            ),
            (
                DiffPoint::new(2, 2),
                DiffOffset(snapshot.text().find("two").unwrap() + 2),
            ),
            (
                DiffPoint::new(4, 3),
                DiffOffset(snapshot.text().find("three").unwrap() + 3),
            ),
            (DiffPoint::new(8, 0), DiffOffset(snapshot.text().len())),
        ] {
            let actual = snapshot.point_to_offset(*point);
            assert_eq!(actual, *offset, "for {:?}", point);
            let actual = snapshot.offset_to_point(*offset);
            assert_eq!(actual, *point, "for {:?}", offset);
        }

        diff_map.update(cx, |diff_map, cx| {
            diff_map.collapse_diff_hunks(vec![Anchor::min()..Anchor::max()], cx)
        });
        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                ZERO
                one
                TWO
                three
                six
                "
            ),
        );

        // Expand the first diff hunk
        diff_map.update(cx, |diff_map, cx| {
            let position = deps.multibuffer_snapshot.anchor_before(Point::new(2, 0));
            diff_map.expand_diff_hunks(vec![position..position], cx)
        });
        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                  ZERO
                  one
                - two
                + TWO
                  three
                  six
                "
            ),
        );

        // Expand the second diff hunk
        diff_map.update(cx, |diff_map, cx| {
            let position = deps.multibuffer_snapshot.anchor_before(Point::new(3, 0));
            diff_map.expand_diff_hunks(vec![position..position], cx)
        });
        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                  ZERO
                  one
                - two
                + TWO
                  three
                - four
                - five
                  six
                "
            ),
        );

        // Edit the buffer before the first hunk
        let edits = deps.update_buffer(cx, |buffer, cx| {
            buffer.edit_via_marked_text(
                indoc!(
                    "
                    ZERO
                    one« hundred
                      thousand»
                    TWO
                    three
                    six
                    "
                ),
                None,
                cx,
            );
        });

        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), edits, cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                  ZERO
                  one hundred
                    thousand
                - two
                + TWO
                  three
                - four
                - five
                  six
                "
            ),
        );

        // Recalculate the diff, changing the first diff hunk.
        let _ = deps.change_set.update(cx, |change_set, cx| {
            change_set.recalculate_diff(deps.buffer.read(cx).text_snapshot(), cx)
        });
        cx.run_until_parked();
        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), Vec::new(), cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                  ZERO
                  one hundred
                    thousand
                  TWO
                  three
                - four
                - five
                  six
                "
            ),
        );
    }

    #[gpui::test]
    fn test_diff_map_multiple_buffer_edits(cx: &mut TestAppContext) {
        cx.update(init_test);

        let text = "hello world";

        let (diff_map, mut snapshot, mut deps) = build_diff_map(text, None, cx);
        assert_eq!(snapshot.text(), "hello world");

        let edits = deps.update_buffer(cx, |buffer, cx| {
            buffer.edit([(4..5, "a"), (9..11, "k")], None, cx);
        });
        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot, edits, cx)
        });

        assert_new_snapshot(&mut snapshot, sync, indoc!("hella work"));
    }

    #[gpui::test]
    fn test_diff_map_clipping(cx: &mut TestAppContext) {
        cx.update(init_test);

        let text = indoc!(
            "₀
             ₃
             ₄
             ₅
             "
        );
        let base_text = indoc!(
            "₀
             ₁
             ₂
             ₃
             ₄
             "
        );

        let (diff_map, mut diff_snapshot, deps) = build_diff_map(text, Some(base_text), cx);
        diff_map.update(cx, |diff_map, cx| diff_map.set_all_hunks_expanded(cx));
        cx.run_until_parked();

        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot, vec![], cx)
        });

        assert_new_snapshot(
            &mut diff_snapshot,
            sync,
            indoc! {"
              ₀
            - ₁
            - ₂
              ₃
              ₄
            + ₅
            "},
        );

        for (point, (left, right)) in [
            (
                DiffPoint::new(0, 0), // start
                (DiffPoint::new(0, 0), DiffPoint::new(0, 0)),
            ),
            (
                DiffPoint::new(1, 1), // deleted
                (DiffPoint::new(1, 0), DiffPoint::new(1, 3)),
            ),
            (
                DiffPoint::new(3, 1), // unchanged
                (DiffPoint::new(3, 0), DiffPoint::new(3, 3)),
            ),
            (
                DiffPoint::new(5, 2), // inserted
                (DiffPoint::new(5, 0), DiffPoint::new(5, 3)),
            ),
            (
                DiffPoint::new(6, 0), // end
                (DiffPoint::new(6, 0), DiffPoint::new(6, 0)),
            ),
            (
                DiffPoint::new(7, 7), // beyond
                (DiffPoint::new(6, 0), DiffPoint::new(6, 0)),
            ),
        ] {
            assert_eq!(left, diff_snapshot.clip_point(point, Bias::Left));
            assert_eq!(right, diff_snapshot.clip_point(point, Bias::Right));
        }

        assert_eq!(
            diff_snapshot.text_summary_for_range(DiffOffset(0)..DiffOffset(0)),
            TextSummary::default()
        );
        assert_eq!(
            diff_snapshot.text_summary_for_range(diff_snapshot.len()..diff_snapshot.len()),
            TextSummary::default()
        );
        let full_summary = TextSummary {
            len: 24,
            len_utf16: OffsetUtf16(12),
            lines: Point { row: 6, column: 0 },
            first_line_chars: 1,
            last_line_chars: 0,
            last_line_len_utf16: 0,
            longest_row: 0,
            longest_row_chars: 1,
        };
        let partial_summary = TextSummary {
            len: 8,
            len_utf16: OffsetUtf16(4),
            lines: Point { row: 2, column: 0 },
            first_line_chars: 1,
            last_line_chars: 0,
            last_line_len_utf16: 0,
            longest_row: 0,
            longest_row_chars: 1,
        };

        let two = DiffOffset(diff_snapshot.text().find("₂").unwrap());
        let four = DiffOffset(diff_snapshot.text().find("₄").unwrap());

        assert_eq!(
            diff_snapshot.text_summary_for_range(DiffOffset(0)..diff_snapshot.len()),
            full_summary
        );
        assert_eq!(
            diff_snapshot.text_summary_for_range(DiffOffset(0)..two),
            partial_summary
        );
        assert_eq!(
            diff_snapshot.text_summary_for_range(two..four),
            partial_summary
        );
        assert_eq!(
            diff_snapshot.text_summary_for_range(four..diff_snapshot.len()),
            partial_summary
        );
    }

    #[gpui::test]
    fn test_empty_diff_map(cx: &mut TestAppContext) {
        cx.update(init_test);

        let (_diff_map, diff_snapshot, _deps) = build_diff_map("", None, cx);
        assert_eq!(
            diff_snapshot
                .row_infos(0)
                .map(|info| info.buffer_row)
                .collect::<Vec<_>>(),
            [Some(0)]
        );
    }

    #[gpui::test]
    fn test_expand_collapse_at_positions_adjacent_to_hunks(cx: &mut TestAppContext) {
        cx.update(init_test);

        let base_text = indoc!(
            "
            one
            two
            three
            four
            five
            six
            seven
            eight
            "
        );
        let text = indoc!(
            "
            one
            two
            five
            six; seven
            eight
            "
        );

        let (diff_map, mut snapshot, deps) = build_diff_map(text, Some(base_text), cx);

        // Expand at the line right below a deleted hunk.
        diff_map.update(cx, |diff_map, cx| {
            let point = deps.multibuffer_snapshot.anchor_before(Point::new(2, 0));
            diff_map.expand_diff_hunks(vec![point..point], cx)
        });

        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                  one
                  two
                - three
                - four
                  five
                  six; seven
                  eight
                "
            ),
        );

        // Collapse at the line right below a deleted hunk.
        diff_map.update(cx, |diff_map, cx| {
            let point = deps.multibuffer_snapshot.anchor_before(Point::new(2, 0));
            diff_map.collapse_diff_hunks(vec![point..point], cx)
        });

        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                  one
                  two
                  five
                  six; seven
                  eight
                "
            ),
        );

        // Expand at the line right above a deleted hunk.
        diff_map.update(cx, |diff_map, cx| {
            let point = deps.multibuffer_snapshot.anchor_before(Point::new(1, 0));
            diff_map.expand_diff_hunks(vec![point..point], cx)
        });

        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                  one
                  two
                - three
                - four
                  five
                  six; seven
                  eight
                "
            ),
        );

        eprintln!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        // Expand at the line right below a modified hunk. Should not expand anything.
        diff_map.update(cx, |diff_map, cx| {
            let point = deps.multibuffer_snapshot.anchor_before(Point::new(4, 0));
            diff_map.expand_diff_hunks(vec![point..point], cx)
        });

        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                  one
                  two
                - three
                - four
                  five
                  six; seven
                  eight
                "
            ),
        );
    }

    #[gpui::test]
    fn test_expand_collapse_insertion_hunk(cx: &mut TestAppContext) {
        cx.update(init_test);

        let base_text = indoc!(
            "
            one
            two
            seven
            eight
            "
        );
        let text = indoc!(
            "
            one
            two
            three
            four
            five
            six
            seven
            eight
            "
        );

        let (diff_map, mut snapshot, deps) = build_diff_map(text, Some(base_text), cx);

        // Expand at the line right right after a deleted hunk.
        diff_map.update(cx, |diff_map, cx| {
            let point = deps.multibuffer_snapshot.anchor_before(Point::new(2, 0));
            diff_map.expand_diff_hunks(vec![point..point], cx)
        });

        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                  one
                  two
                + three
                + four
                + five
                + six
                  seven
                  eight
                "
            ),
        );

        diff_map.update(cx, |diff_map, cx| {
            let point = deps.multibuffer_snapshot.anchor_before(Point::new(2, 0));
            diff_map.collapse_diff_hunks(vec![point..point], cx)
        });

        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                one
                two
                three
                four
                five
                six
                seven
                eight
                "
            ),
        );
    }

    #[gpui::test]
    fn test_edit_in_insertion_hunk(cx: &mut TestAppContext) {
        cx.update(init_test);

        let base_text = indoc!(
            "
            one
            two
            six
            seven
            "
        );
        let text = indoc!(
            "
            one
            two
            three
            four
            five
            six
            seven
            "
        );

        let (diff_map, mut snapshot, mut deps) = build_diff_map(text, Some(base_text), cx);

        // Expand the hunk
        diff_map.update(cx, |diff_map, cx| {
            diff_map.expand_diff_hunks(vec![Anchor::min()..Anchor::max()], cx)
        });

        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                  one
                  two
                + three
                + four
                + five
                  six
                  seven
                "
            ),
        );

        eprintln!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        let edits = deps.update_buffer(cx, |buffer, cx| {
            buffer.edit_via_marked_text(
                indoc!(
                    "
                    one
                    two
                    three«
                    !»
                    four
                    five
                    six
                    seven
                    "
                ),
                None,
                cx,
            )
        });

        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(deps.multibuffer_snapshot.clone(), edits, cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                  one
                  two
                + three
                + !
                + four
                + five
                  six
                  seven
                "
            ),
        );
    }

    #[track_caller]
    fn assert_new_snapshot(
        snapshot: &mut DiffMapSnapshot,
        (new_snapshot, edits): (DiffMapSnapshot, Vec<Edit<DiffOffset>>),
        expected_diff: &str,
    ) {
        let actual_text = new_snapshot.text();
        let line_infos = new_snapshot.row_infos(0).collect::<Vec<_>>();
        let has_diff = line_infos.iter().any(|info| info.diff_status.is_some());
        let actual_diff = actual_text
            .split('\n')
            .zip(line_infos)
            .map(|(line, info)| {
                let marker = match info.diff_status {
                    Some(DiffHunkStatus::Added) => "+ ",
                    Some(DiffHunkStatus::Removed) => "- ",
                    Some(DiffHunkStatus::Modified) => unreachable!(),
                    None => {
                        if has_diff {
                            "  "
                        } else {
                            ""
                        }
                    }
                };
                if line.is_empty() {
                    String::new()
                } else {
                    format!("{marker}{line}")
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        pretty_assertions::assert_eq!(actual_diff, expected_diff);
        check_edits(snapshot, &new_snapshot, &edits);
        *snapshot = new_snapshot;
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

    #[track_caller]
    fn assert_chunks_in_ranges(snapshot: &DiffMapSnapshot) {
        let full_text = snapshot.text();
        for ix in 0..full_text.len() {
            let offset = DiffOffset(ix);
            let mut chunks =
                snapshot.chunks(DiffOffset(0)..snapshot.len(), false, Default::default());
            chunks.seek(offset..snapshot.len());
            let tail = chunks.map(|chunk| chunk.text).collect::<String>();
            assert_eq!(tail, &full_text[ix..], "seek to range: {:?}", ix..);

            let tail = snapshot
                .chunks(offset..snapshot.len(), false, Default::default())
                .map(|chunk| chunk.text)
                .collect::<String>();
            assert_eq!(tail, &full_text[ix..], "start from range: {:?}", ix..);

            let head = snapshot
                .chunks(DiffOffset(0)..offset, false, Default::default())
                .map(|chunk| chunk.text)
                .collect::<String>();
            assert_eq!(head, &full_text[..ix], "start with range: {:?}", ..ix);
        }
    }

    #[track_caller]
    fn assert_consistent_line_numbers(snapshot: &DiffMapSnapshot) {
        let all_line_numbers = snapshot.row_infos(0).collect::<Vec<_>>();
        for start_row in 1..all_line_numbers.len() {
            let line_numbers = snapshot.row_infos(start_row as u32).collect::<Vec<_>>();
            assert_eq!(
                line_numbers,
                all_line_numbers[start_row..],
                "start_row: {start_row}"
            );

            for seek_row in 0..all_line_numbers.len() {
                let mut numbers = snapshot.row_infos(start_row as u32);
                numbers.seek(seek_row as u32);
                let line_numbers = numbers.collect::<Vec<_>>();
                assert_eq!(
                    line_numbers,
                    all_line_numbers[seek_row..],
                    "seek_row: {seek_row}, start_row: {start_row}"
                );
            }
        }
    }

    struct DiffMapDeps {
        buffer: Model<Buffer>,
        multibuffer: Model<MultiBuffer>,
        change_set: Model<BufferChangeSet>,
        multibuffer_snapshot: MultiBufferSnapshot,
        multibuffer_edits: text::Subscription,
    }

    fn build_diff_map(
        text: &str,
        base_text: Option<&str>,
        cx: &mut TestAppContext,
    ) -> (Model<DiffMap>, DiffMapSnapshot, DiffMapDeps) {
        let buffer = cx.new_model(|cx| Buffer::local(text, cx));

        let change_set = cx.new_model(|cx| {
            let text_snapshot = buffer.read(cx).text_snapshot();
            let mut change_set = BufferChangeSet::new(&text_snapshot);
            if let Some(base_text) = base_text {
                let _ = change_set.set_base_text(base_text.to_string(), text_snapshot, cx);
            }
            change_set
        });

        let multibuffer = cx.new_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        let (multibuffer_snapshot, multibuffer_edits) =
            multibuffer.update(cx, |buffer, cx| (buffer.snapshot(cx), buffer.subscribe()));

        let (diff_map, diff_map_snapshot) = cx.update(|cx| DiffMap::new(multibuffer.clone(), cx));
        diff_map.update(cx, |diff_map, cx| {
            diff_map.add_change_set(change_set.clone(), cx)
        });
        cx.run_until_parked();

        (
            diff_map,
            diff_map_snapshot,
            DiffMapDeps {
                buffer,
                multibuffer,
                change_set,
                multibuffer_snapshot,
                multibuffer_edits,
            },
        )
    }

    impl DiffMapDeps {
        fn update_buffer(
            &mut self,
            cx: &mut TestAppContext,
            f: impl FnOnce(&mut Buffer, &mut ModelContext<Buffer>),
        ) -> Vec<text::Edit<usize>> {
            self.buffer.update(cx, f);

            self.multibuffer_snapshot = self
                .multibuffer
                .read_with(cx, |buffer, cx| buffer.snapshot(cx));
            self.multibuffer_edits.consume().into_inner()
        }
    }

    fn init_test(cx: &mut AppContext) {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        language::init(cx);
        crate::init(cx);
        Project::init_settings(cx);
    }
}

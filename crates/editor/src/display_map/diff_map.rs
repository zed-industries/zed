use super::inlay_map::{InlayBufferRows, InlayChunks, InlayEdit, InlaySnapshot};
use crate::{Highlights, InlayOffset, InlayPoint};
use collections::HashMap;
use gpui::{AppContext, Context as _, Model, ModelContext, Subscription};
use language::{BufferChunks, BufferId, Chunk};
use multi_buffer::{Anchor, AnchorRangeExt, MultiBuffer, MultiBufferSnapshot, ToOffset};
use project::buffer_store::BufferChangeSet;
use std::{mem, ops::Range};
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
    inlay_snapshot: InlaySnapshot,
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
    inlay_map: TextSummary,
    diff_map: TextSummary,
}

impl DiffTransformSummary {
    pub fn inlay_point(&self) -> InlayPoint {
        InlayPoint(self.inlay_map.lines)
    }
    pub fn inlay_offset(&self) -> InlayOffset {
        InlayOffset(self.inlay_map.len)
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
    cursor: Cursor<'a, DiffTransform, (DiffOffset, InlayOffset)>,
    inlay_chunks: InlayChunks<'a>,
    inlay_chunk: Option<Chunk<'a>>,
    inlay_offset: InlayOffset,
    offset: DiffOffset,
    end_offset: DiffOffset,
    diff_base_chunks: Option<(BufferId, BufferChunks<'a>)>,
}

#[derive(Clone)]
pub struct DiffMapBufferRows<'a> {
    cursor: Cursor<'a, DiffTransform, (DiffPoint, InlayPoint)>,
    diff_point: DiffPoint,
    input_buffer_rows: InlayBufferRows<'a>,
}

pub type DiffEdit = text::Edit<DiffOffset>;

enum DiffMapOperation {
    BufferDiffUpdated {
        buffer_id: BufferId,
    },
    Edited {
        inlay_snapshot: InlaySnapshot,
        edits: Vec<InlayEdit>,
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
        inlay_snapshot: InlaySnapshot,
        multibuffer: Model<MultiBuffer>,
        cx: &mut AppContext,
    ) -> (Model<Self>, DiffMapSnapshot) {
        let snapshot = DiffMapSnapshot {
            diffs: TreeMap::default(),
            version: 0,
            transforms: SumTree::from_item(
                DiffTransform::BufferContent {
                    summary: inlay_snapshot.text_summary(),
                    is_inserted_hunk: false,
                },
                &(),
            ),
            inlay_snapshot,
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
        inlay_snapshot: InlaySnapshot,
        edits: Vec<InlayEdit>,
        cx: &mut ModelContext<Self>,
    ) -> (DiffMapSnapshot, Vec<DiffEdit>) {
        self.recompute_transforms(
            DiffMapOperation::Edited {
                inlay_snapshot,
                edits,
            },
            cx,
        );
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
        self.recompute_transforms(DiffMapOperation::BufferDiffUpdated { buffer_id }, cx);
    }

    pub(super) fn has_expanded_diff_hunks_in_ranges(
        &self,
        ranges: &[Range<multi_buffer::Anchor>],
    ) -> bool {
        let mut cursor = self.snapshot.transforms.cursor::<InlayOffset>(&());
        for range in ranges {
            let range = range.to_offset(self.snapshot.buffer());
            let inlay_start = self.snapshot.inlay_snapshot.to_inlay_offset(range.start);
            let inlay_end = self.snapshot.inlay_snapshot.to_inlay_offset(range.end);
            cursor.seek(&inlay_start, Bias::Right, &());
            while *cursor.start() < inlay_end {
                if let Some(DiffTransform::DeletedHunk { .. }) = cursor.item() {
                    return true;
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

        let changes: Vec<(InlayEdit, bool, Range<usize>)> = match &operation {
            DiffMapOperation::BufferDiffUpdated { buffer_id } => {
                let buffer_id = *buffer_id;
                let multibuffer = self.multibuffer.read(cx);
                multibuffer
                    .ranges_for_buffer(buffer_id, cx)
                    .into_iter()
                    .map(|(_, range, _)| {
                        let multibuffer_start =
                            ToOffset::to_offset(&range.start, &multibuffer_snapshot);
                        let multibuffer_end =
                            ToOffset::to_offset(&range.end, &multibuffer_snapshot);
                        let inlay_start = self
                            .snapshot
                            .inlay_snapshot
                            .to_inlay_offset(multibuffer_start);
                        let inlay_end = self
                            .snapshot
                            .inlay_snapshot
                            .to_inlay_offset(multibuffer_end);
                        (
                            InlayEdit {
                                old: inlay_start..inlay_end,
                                new: inlay_start..inlay_end,
                            },
                            false,
                            multibuffer_start..multibuffer_end,
                        )
                    })
                    .collect()
            }
            DiffMapOperation::Edited {
                inlay_snapshot,
                edits,
            } => {
                let mut changes = Vec::new();
                for edit in edits {
                    let multibuffer_start = inlay_snapshot.to_buffer_offset(edit.new.start);
                    let multibuffer_end = inlay_snapshot.to_buffer_offset(edit.new.end);
                    let multibuffer_range = multibuffer_start..multibuffer_end;
                    changes.push((edit.clone(), true, multibuffer_range))
                }
                self.snapshot.inlay_snapshot = inlay_snapshot.clone();
                changes
            }
            DiffMapOperation::ExpandHunks { ranges }
            | DiffMapOperation::CollapseHunks { ranges } => {
                let mut changes = Vec::new();
                for range in ranges.iter() {
                    let multibuffer_range = range.to_offset(&multibuffer_snapshot);
                    let inlay_start = self
                        .snapshot
                        .inlay_snapshot
                        .to_inlay_offset(multibuffer_range.start);
                    let inlay_end = self
                        .snapshot
                        .inlay_snapshot
                        .to_inlay_offset(multibuffer_range.end);
                    changes.push((
                        InlayEdit {
                            old: inlay_start..inlay_end,
                            new: inlay_start..inlay_end,
                        },
                        false,
                        multibuffer_range,
                    ));
                }
                changes
            }
        };

        let mut cursor = self
            .snapshot
            .transforms
            .cursor::<(InlayOffset, DiffOffset)>(&());
        let mut new_transforms = SumTree::default();
        let mut edits = Patch::default();

        let mut changes = changes.into_iter().peekable();
        while let Some((mut edit, mut is_inlay_edit, mut multibuffer_range)) = changes.next() {
            new_transforms.append(cursor.slice(&edit.old.start, Bias::Left, &()), &());

            let mut end_of_current_insert = InlayOffset(0);
            loop {
                let old_overshoot = (edit.old.start - cursor.start().0).0;
                let new_overshoot = edit.new.start.0 - new_transforms.summary().inlay_map.len;
                let diff_edit_old_start = cursor.start().1 + DiffOffset(old_overshoot);
                let diff_edit_new_start =
                    DiffOffset(new_transforms.summary().diff_map.len + new_overshoot);

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
                    let buffer_anchor_range = buffer.anchor_after(buffer_range.start)
                        ..buffer.anchor_before(buffer_range.end);
                    let change_start_buffer_offset = buffer_range.start;
                    if let Some(diff_state) = diff_state {
                        let diff = &diff_state.diff;
                        let base_text = &diff_state.base_text;

                        for hunk in diff.hunks_intersecting_range(buffer_anchor_range, buffer) {
                            let hunk_anchor_range = {
                                let start = multi_buffer::Anchor {
                                    excerpt_id,
                                    buffer_id: Some(buffer_id),
                                    text_anchor: hunk.buffer_range.start,
                                };
                                let end = multi_buffer::Anchor {
                                    excerpt_id,
                                    buffer_id: Some(buffer_id),
                                    text_anchor: hunk.buffer_range.end,
                                };
                                start..end
                            };

                            let hunk_start_buffer_offset =
                                hunk.buffer_range.start.to_offset(buffer);
                            if hunk_start_buffer_offset < change_start_buffer_offset {
                                continue;
                            }

                            let excerpt_buffer_range_start_offset =
                                excerpt_buffer_range.start.to_offset(buffer);
                            let hunk_start_multibuffer_offset = excerpt_range.start
                                + hunk_start_buffer_offset
                                - excerpt_buffer_range_start_offset;
                            let hunk_start_inlay_offset = self
                                .snapshot
                                .inlay_snapshot
                                .to_inlay_offset(hunk_start_multibuffer_offset);

                            self.push_buffer_content_transform(
                                &mut new_transforms,
                                hunk_start_inlay_offset,
                                end_of_current_insert,
                            );

                            while cursor.end(&()).0 <= hunk_start_inlay_offset {
                                let Some(item) = cursor.item() else {
                                    break;
                                };
                                if let DiffTransform::DeletedHunk { .. } = item {
                                    let new_offset =
                                        DiffOffset(new_transforms.summary().diff_map.len);
                                    let edit = Edit {
                                        old: cursor.start().1..cursor.end(&()).1,
                                        new: new_offset..new_offset,
                                    };
                                    edits.push(edit);
                                }
                                cursor.next(&());
                            }

                            let mut was_previously_expanded = false;
                            if let Some(item) = cursor.item() {
                                if let DiffTransform::DeletedHunk {
                                    base_text_byte_range,
                                    ..
                                } = item
                                {
                                    if cursor.start().0 == hunk_start_inlay_offset
                                        && *base_text_byte_range == hunk.diff_base_byte_range
                                    {
                                        was_previously_expanded = true;
                                    }
                                }
                            }

                            let mut should_expand_hunk =
                                was_previously_expanded || self.all_hunks_expanded;
                            match &operation {
                                DiffMapOperation::ExpandHunks { ranges } => {
                                    should_expand_hunk |= ranges.iter().any(|range| {
                                        range.overlaps(&hunk_anchor_range, &multibuffer_snapshot)
                                    })
                                }
                                DiffMapOperation::CollapseHunks { ranges } => {
                                    should_expand_hunk &= !ranges.iter().any(|range| {
                                        range.overlaps(&hunk_anchor_range, &multibuffer_snapshot)
                                    })
                                }
                                _ => {}
                            };

                            if should_expand_hunk {
                                if hunk.diff_base_byte_range.len() > 0 {
                                    if !was_previously_expanded {
                                        let hunk_overshoot =
                                            (hunk_start_inlay_offset - cursor.start().0).0;
                                        let old_offset =
                                            cursor.start().1 + DiffOffset(hunk_overshoot);
                                        let new_start =
                                            DiffOffset(new_transforms.summary().diff_map.len);
                                        let new_end =
                                            new_start + DiffOffset(hunk.diff_base_byte_range.len());
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

                                let hunk_end_buffer_offset =
                                    hunk.buffer_range.end.to_offset(buffer);

                                if hunk_end_buffer_offset > hunk_start_buffer_offset {
                                    let hunk_end_multibuffer_offset = excerpt_range.start
                                        + hunk_end_buffer_offset
                                        - excerpt_buffer_range_start_offset;
                                    let hunk_end_inlay_offset = self
                                        .snapshot
                                        .inlay_snapshot
                                        .to_inlay_offset(hunk_end_multibuffer_offset);
                                    end_of_current_insert = hunk_end_inlay_offset;
                                }
                            } else if was_previously_expanded {
                                let old_start = cursor.start().1;
                                let new_offset = DiffOffset(new_transforms.summary().diff_map.len);
                                let edit = Edit {
                                    old: old_start
                                        ..old_start + DiffOffset(hunk.diff_base_byte_range.len()),
                                    new: new_offset..new_offset,
                                };
                                edits.push(edit);
                                cursor.next(&());
                            }
                        }
                    }
                }

                while cursor.end(&()).0 <= edit.old.end {
                    let Some(item) = cursor.item() else {
                        break;
                    };
                    if let DiffTransform::DeletedHunk { .. } = item {
                        let new_offset = DiffOffset(new_transforms.summary().diff_map.len);
                        let edit = Edit {
                            old: cursor.start().1..cursor.end(&()).1,
                            new: new_offset..new_offset,
                        };
                        edits.push(edit);
                    }
                    cursor.next(&());
                }

                let old_overshoot = (edit.old.end - cursor.start().0).0;
                self.push_buffer_content_transform(
                    &mut new_transforms,
                    edit.new.end,
                    end_of_current_insert,
                );
                let diff_edit_old_end = cursor.start().1 + DiffOffset(old_overshoot);
                let diff_edit_new_end = DiffOffset(new_transforms.summary().diff_map.len);

                if is_inlay_edit {
                    edits.push(DiffEdit {
                        old: diff_edit_old_start..diff_edit_old_end,
                        new: diff_edit_new_start..diff_edit_new_end,
                    })
                }

                if let Some((next_edit, _, _)) = changes.peek() {
                    if next_edit.old.start < cursor.end(&()).0 {
                        (edit, is_inlay_edit, multibuffer_range) = changes.next().unwrap();
                        continue;
                    }
                }

                let suffix = (cursor.end(&()).0 - edit.old.end).0;
                let transform_end = InlayOffset(new_transforms.summary().inlay_map.len + suffix);
                self.push_buffer_content_transform(
                    &mut new_transforms,
                    transform_end,
                    end_of_current_insert,
                );
                cursor.next(&());
                break;
            }
        }

        self.edits_since_sync = self.edits_since_sync.compose(edits);

        new_transforms.append(cursor.suffix(&()), &());
        drop(cursor);
        self.snapshot.transforms = new_transforms;
        self.snapshot.version += 1;
        cx.notify();

        #[cfg(test)]
        self.check_invariants();
    }

    fn push_buffer_content_transform(
        &self,
        new_transforms: &mut SumTree<DiffTransform>,
        end_offset: InlayOffset,
        end_of_current_inserted_hunk: InlayOffset,
    ) {
        for (end_offset, region_is_inserted_hunk) in [
            (end_offset.min(end_of_current_inserted_hunk), true),
            (end_offset, false),
        ] {
            let start_offset = InlayOffset(new_transforms.summary().inlay_map.len);
            if end_offset <= start_offset {
                continue;
            }
            let summary_to_add = self
                .snapshot
                .inlay_snapshot
                .text_summary_for_range(start_offset..end_offset);

            let mut did_extend = false;
            new_transforms.update_last(
                |last_transform| {
                    if let DiffTransform::BufferContent {
                        summary,
                        is_inserted_hunk,
                    } = last_transform
                    {
                        if *is_inserted_hunk == region_is_inserted_hunk {
                            did_extend = true;
                            *summary += summary_to_add.clone();
                        }
                    }
                },
                &(),
            );
            if !did_extend {
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

    #[cfg(test)]
    fn check_invariants(&self) {
        let snapshot = &self.snapshot;
        if snapshot.transforms.summary().inlay_map.len != snapshot.inlay_snapshot.len().0 {
            panic!(
                "incorrect input length. expected {}, got {}. transforms: {:+?}",
                snapshot.inlay_snapshot.len().0,
                snapshot.transforms.summary().inlay_map.len,
                snapshot.transforms.items(&()),
            );
        }

        let mut prev_transform: Option<&DiffTransform> = None;
        for item in snapshot.transforms.iter() {
            if let DiffTransform::BufferContent {
                summary,
                is_inserted_hunk: is_new,
            } = item
            {
                if let Some(DiffTransform::BufferContent {
                    is_inserted_hunk: prev_is_new,
                    ..
                }) = prev_transform
                {
                    if *is_new == *prev_is_new {
                        panic!("multiple adjacent buffer content transforms with the same is_new value");
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
    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(DiffOffset(0)..self.len(), false, Highlights::default())
            .map(|c| c.text)
            .collect()
    }

    pub fn len(&self) -> DiffOffset {
        DiffOffset(self.transforms.summary().diff_map.len)
    }

    pub fn text_summary(&self) -> TextSummary {
        self.transforms.summary().diff_map.clone()
    }

    pub fn text_summary_for_range(&self, range: Range<DiffOffset>) -> TextSummary {
        let mut cursor = self.transforms.cursor::<(DiffOffset, InlayOffset)>(&());
        cursor.seek(&range.start, Bias::Right, &());

        let Some(first_transform) = cursor.item() else {
            return TextSummary::default();
        };

        let (diff_transform_start, inlay_transform_start) = cursor.start().clone();
        let (diff_transform_end, _) = cursor.end(&());
        let diff_start = range.start;
        let diff_end = std::cmp::min(range.end, diff_transform_end);

        let mut result = match first_transform {
            DiffTransform::BufferContent { .. } => {
                let inlay_start =
                    inlay_transform_start + InlayOffset((diff_start - diff_transform_start).0);
                let inlay_end =
                    inlay_transform_start + InlayOffset((diff_end - diff_transform_start).0);

                self.inlay_snapshot
                    .text_summary_for_range(inlay_start..inlay_end)
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

        let (diff_transform_start, inlay_transform_start) = cursor.start().clone();

        result += match last_transform {
            DiffTransform::BufferContent { .. } => {
                let inlay_end =
                    inlay_transform_start + InlayOffset((range.end - diff_transform_start).0);

                self.inlay_snapshot
                    .text_summary_for_range(inlay_transform_start..inlay_end)
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
        &self.inlay_snapshot.buffer
    }

    pub fn to_point(&self, offset: DiffOffset) -> DiffPoint {
        let mut cursor = self.transforms.cursor::<DiffTransformSummary>(&());
        cursor.seek(&offset, Bias::Right, &());
        let start_transform = cursor.start();
        let overshoot = offset - start_transform.diff_offset();
        if overshoot.0 == 0 {
            return start_transform.diff_point();
        }

        match cursor.item() {
            Some(DiffTransform::BufferContent { .. }) => {
                let inlay_offset = start_transform.inlay_offset() + InlayOffset(overshoot.0);
                let inlay_point = self.inlay_snapshot.to_point(inlay_offset);
                start_transform.diff_point()
                    + DiffPoint((inlay_point - start_transform.inlay_point()).0)
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
                let inlay_point = start_transform.inlay_point() + InlayPoint(overshoot.0);
                let clipped = self.inlay_snapshot.clip_point(inlay_point, bias);
                start_transform.diff_point()
                    + DiffPoint((clipped - start_transform.inlay_point()).0)
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

    pub fn to_offset(&self, point: DiffPoint) -> DiffOffset {
        let mut cursor = self.transforms.cursor::<DiffTransformSummary>(&());
        cursor.seek(&point, Bias::Right, &());
        let start_transform = cursor.start();
        let overshoot = point - start_transform.diff_point();
        if overshoot.0.is_zero() {
            return start_transform.diff_offset();
        }

        match cursor.item() {
            Some(DiffTransform::BufferContent { .. }) => {
                let inlay_point = start_transform.inlay_point() + InlayPoint(overshoot.0);
                let inlay_offset = self.inlay_snapshot.to_offset(inlay_point);
                start_transform.diff_offset()
                    + DiffOffset((inlay_offset - start_transform.inlay_offset()).0)
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

    pub fn to_inlay_offset(&self, offset: DiffOffset) -> InlayOffset {
        let mut cursor = self.transforms.cursor::<(DiffOffset, InlayOffset)>(&());
        cursor.seek(&offset, Bias::Right, &());
        let mut inlay_offset = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = offset.0 - cursor.start().0 .0;
            inlay_offset.0 += overshoot;
        }
        inlay_offset
    }

    pub fn to_inlay_point(&self, point: DiffPoint) -> InlayPoint {
        let mut cursor = self.transforms.cursor::<(DiffPoint, InlayPoint)>(&());
        cursor.seek(&point, Bias::Right, &());
        let mut inlay_point = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = point.0 - cursor.start().0 .0;
            inlay_point.0 += overshoot;
        }
        inlay_point
    }

    pub fn to_diff_offset(&self, offset: InlayOffset) -> DiffOffset {
        let mut cursor = self.transforms.cursor::<(InlayOffset, DiffOffset)>(&());
        cursor.seek(&offset, Bias::Right, &());
        let mut diff_offset = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = offset.0 - cursor.start().0 .0;
            diff_offset.0 += overshoot;
        }
        diff_offset
    }

    pub fn to_diff_point(&self, point: InlayPoint) -> DiffPoint {
        let mut cursor = self.transforms.cursor::<(InlayPoint, DiffPoint)>(&());
        cursor.seek(&point, Bias::Right, &());
        let mut diff_point = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = point.0 - cursor.start().0 .0;
            diff_point.0 += overshoot;
        }
        diff_point
    }

    pub fn make_diff_offset(&self, buffer_offset: usize) -> DiffOffset {
        self.to_diff_offset(self.inlay_snapshot.to_inlay_offset(buffer_offset))
    }

    pub fn make_diff_point(&self, buffer_point: Point) -> DiffPoint {
        self.to_diff_point(self.inlay_snapshot.to_inlay_point(buffer_point))
    }

    pub fn to_buffer_offset(&self, diff_offset: DiffOffset) -> usize {
        self.inlay_snapshot
            .to_buffer_offset(self.to_inlay_offset(diff_offset))
    }

    pub fn to_buffer_point(&self, diff_point: DiffPoint) -> Point {
        self.inlay_snapshot
            .to_buffer_point(self.to_inlay_point(diff_point))
    }

    pub(crate) fn chunks<'a>(
        &'a self,
        range: Range<DiffOffset>,
        language_aware: bool,
        highlights: Highlights<'a>,
    ) -> DiffMapChunks<'a> {
        let mut cursor = self.transforms.cursor::<(DiffOffset, InlayOffset)>(&());

        cursor.seek(&range.end, Bias::Right, &());
        let mut inlay_end = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = range.end.0 - cursor.start().0 .0;
            inlay_end.0 += overshoot;
        }

        cursor.seek(&range.start, Bias::Right, &());
        let mut inlay_start = cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = cursor.item() {
            let overshoot = range.start.0 - cursor.start().0 .0;
            inlay_start.0 += overshoot;
        }

        let inlay_chunks =
            self.inlay_snapshot
                .chunks(inlay_start..inlay_end, language_aware, highlights);

        DiffMapChunks {
            snapshot: self,
            language_aware,
            cursor,
            inlay_chunk: None,
            inlay_chunks,
            inlay_offset: inlay_start,
            offset: range.start,
            diff_base_chunks: None,
            end_offset: range.end,
        }
    }

    pub fn buffer_rows(&self, start_row: u32) -> DiffMapBufferRows {
        if start_row > self.transforms.summary().diff_map.lines.row {
            panic!("invalid diff map row {}", start_row);
        }

        let diff_point = DiffPoint(Point::new(start_row, 0));
        let mut cursor = self.transforms.cursor::<(DiffPoint, InlayPoint)>(&());
        cursor.seek(&diff_point, Bias::Right, &());

        let (diff_transform_start, inlay_transform_start) = cursor.start().clone();

        let overshoot = if matches!(cursor.item(), Some(DiffTransform::BufferContent { .. })) {
            diff_point.row() - diff_transform_start.row()
        } else {
            0
        };
        let input_buffer_rows = self
            .inlay_snapshot
            .buffer_rows(inlay_transform_start.row() + overshoot);

        DiffMapBufferRows {
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
            inlay_end.0 += overshoot;
        }

        self.cursor.seek(&range.start, Bias::Right, &());
        let mut inlay_start = self.cursor.start().1;
        if let Some(DiffTransform::BufferContent { .. }) = self.cursor.item() {
            let overshoot = range.start.0 - self.cursor.start().0 .0;
            inlay_start.0 += overshoot;
        }

        self.inlay_chunks.seek(inlay_start..inlay_end);
        self.inlay_chunk.take();
        self.inlay_offset = inlay_start;
        self.offset = range.start;
        self.end_offset = range.end;
    }

    pub fn offset(&self) -> DiffOffset {
        self.offset
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
                    .inlay_chunk
                    .get_or_insert_with(|| self.inlay_chunks.next().unwrap());

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
                    self.inlay_chunk.take()
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

impl<'a> DiffMapBufferRows<'a> {
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
            .seek(inlay_transform_start.row() + overshoot);
    }
}

impl<'a> Iterator for DiffMapBufferRows<'a> {
    type Item = Option<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if let Some(DiffTransform::DeletedHunk { .. }) = self.cursor.item() {
            Some(None)
        } else {
            self.input_buffer_rows.next()
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
                inlay_map: summary.clone(),
                diff_map: summary.clone(),
            },
            DiffTransform::DeletedHunk { summary, .. } => DiffTransformSummary {
                inlay_map: TextSummary::default(),
                diff_map: summary.clone(),
            },
        }
    }
}

impl sum_tree::Summary for DiffTransformSummary {
    type Context = ();

    fn zero(_: &Self::Context) -> Self {
        DiffTransformSummary {
            inlay_map: TextSummary::default(),
            diff_map: TextSummary::default(),
        }
    }

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        self.inlay_map += &summary.inlay_map;
        self.diff_map += &summary.diff_map;
    }
}

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for InlayOffset {
    fn zero(_: &()) -> Self {
        InlayOffset(0)
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        self.0 += summary.inlay_map.len
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

impl<'a> sum_tree::Dimension<'a, DiffTransformSummary> for InlayPoint {
    fn zero(_: &()) -> Self {
        InlayPoint(Point::zero())
    }

    fn add_summary(&mut self, summary: &'a DiffTransformSummary, _: &()) {
        self.0 += summary.inlay_map.lines
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
    use crate::display_map::inlay_map::InlayMap;
    use gpui::{AppContext, TestAppContext};
    use indoc::indoc;
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

        let buffer = cx.new_model(|cx| language::Buffer::local(text, cx));
        let change_set = cx.new_model(|cx| {
            BufferChangeSet::new_with_base_text(
                base_text.to_string(),
                buffer.read(cx).text_snapshot(),
                cx,
            )
        });

        let multibuffer = cx.new_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));
        let (multibuffer_snapshot, multibuffer_edits) =
            multibuffer.update(cx, |buffer, cx| (buffer.snapshot(cx), buffer.subscribe()));
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(multibuffer_snapshot.clone());
        let (diff_map, _) =
            cx.update(|cx| DiffMap::new(inlay_snapshot.clone(), multibuffer.clone(), cx));
        diff_map.update(cx, |diff_map, cx| diff_map.add_change_set(change_set, cx));
        cx.run_until_parked();

        let (mut snapshot, _) = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(inlay_snapshot.clone(), vec![], cx)
        });
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
            diff_map.sync(inlay_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
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
            ),
        );

        assert_chunks_in_ranges(&snapshot);

        assert_eq!(
            snapshot.buffer_rows(0).collect::<Vec<_>>(),
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

        assert_eq!(
            snapshot.buffer_rows(4).collect::<Vec<_>>(),
            vec![Some(3), None, None, Some(4), Some(5)]
        );
        assert_eq!(
            snapshot.buffer_rows(5).collect::<Vec<_>>(),
            vec![None, None, Some(4), Some(5)]
        );
        assert_eq!(
            snapshot.buffer_rows(6).collect::<Vec<_>>(),
            vec![None, Some(4), Some(5)]
        );

        let mut buffer_rows = snapshot.buffer_rows(0);
        buffer_rows.seek(7);
        assert_eq!(buffer_rows.next(), Some(Some(4)));
        buffer_rows.seek(6);
        assert_eq!(buffer_rows.next(), Some(None));
        buffer_rows.seek(5);
        assert_eq!(buffer_rows.next(), Some(None));
        buffer_rows.seek(4);
        assert_eq!(buffer_rows.next(), Some(Some(3)));
        drop(buffer_rows);

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
            let actual = snapshot.to_offset(*point);
            assert_eq!(actual, *offset, "for {:?}", point);
            let actual = snapshot.to_point(*offset);
            assert_eq!(actual, *point, "for {:?}", offset);
        }

        diff_map.update(cx, |diff_map, cx| {
            diff_map.collapse_diff_hunks(vec![Anchor::min()..Anchor::max()], cx)
        });
        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(inlay_snapshot.clone(), vec![], cx)
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

        diff_map.update(cx, |diff_map, cx| {
            diff_map.expand_diff_hunks(
                vec![
                    multibuffer_snapshot.anchor_before(Point::new(2, 0))
                        ..multibuffer_snapshot.anchor_before(Point::new(2, 0)),
                ],
                cx,
            )
        });
        let sync = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(inlay_snapshot.clone(), vec![], cx)
        });
        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                ZERO
                one
                two
                TWO
                three
                six
                "
            ),
        );

        buffer.update(cx, |buffer, cx| {
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

        let multibuffer_snapshot = multibuffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let (inlay_snapshot, edits) = inlay_map.sync(
            multibuffer_snapshot,
            multibuffer_edits.consume().into_inner(),
        );
        let sync = diff_map.update(cx, |diff_map, cx| diff_map.sync(inlay_snapshot, edits, cx));

        assert_new_snapshot(
            &mut snapshot,
            sync,
            indoc!(
                "
                ZERO
                one hundred
                  thousand
                two
                TWO
                three
                six
                "
            ),
        );
    }

    #[gpui::test]
    fn test_diff_map_multiple_buffer_edits(cx: &mut TestAppContext) {
        cx.update(init_test);

        let text = "hello world";
        let buffer = cx.new_model(|cx| language::Buffer::local(text, cx));

        let multibuffer = cx.new_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));
        let (multibuffer_snapshot, multibuffer_edits) =
            multibuffer.update(cx, |buffer, cx| (buffer.snapshot(cx), buffer.subscribe()));
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(multibuffer_snapshot.clone());
        let (diff_map, _) =
            cx.update(|cx| DiffMap::new(inlay_snapshot.clone(), multibuffer.clone(), cx));

        let (mut snapshot, _) = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(inlay_snapshot.clone(), vec![], cx)
        });
        assert_eq!(snapshot.text(), "hello world");

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(4..5, "a"), (9..11, "k")], None, cx);
        });
        let multibuffer_snapshot = multibuffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        let (inlay_snapshot, edits) = inlay_map.sync(
            multibuffer_snapshot,
            multibuffer_edits.consume().into_inner(),
        );
        let sync = diff_map.update(cx, |diff_map, cx| diff_map.sync(inlay_snapshot, edits, cx));

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

        let buffer = cx.new_model(|cx| language::Buffer::local(text, cx));
        let change_set = cx.new_model(|cx| {
            BufferChangeSet::new_with_base_text(
                base_text.to_string(),
                buffer.read(cx).text_snapshot(),
                cx,
            )
        });

        let multibuffer = cx.new_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));
        let (multibuffer_snapshot, _) =
            multibuffer.update(cx, |buffer, cx| (buffer.snapshot(cx), buffer.subscribe()));
        let (_, inlay_snapshot) = InlayMap::new(multibuffer_snapshot.clone());
        let (diff_map, _) =
            cx.update(|cx| DiffMap::new(inlay_snapshot.clone(), multibuffer.clone(), cx));
        diff_map.update(cx, |diff_map, cx| {
            diff_map.set_all_hunks_expanded(cx);
            diff_map.add_change_set(change_set, cx);
        });
        cx.run_until_parked();
        let (diff_snapshot, _) =
            diff_map.update(cx, |diff_map, cx| diff_map.sync(inlay_snapshot, vec![], cx));

        assert_eq!(
            diff_snapshot.text(),
            indoc! {"
            ₀
            ₁
            ₂
            ₃
            ₄
            ₅
        "}
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

        let text = "";

        let buffer = cx.new_model(|cx| language::Buffer::local(text, cx));
        let multibuffer = cx.new_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));
        let (multibuffer_snapshot, _multibuffer_edits) =
            multibuffer.update(cx, |buffer, cx| (buffer.snapshot(cx), buffer.subscribe()));
        let (_inlay_map, inlay_snapshot) = InlayMap::new(multibuffer_snapshot.clone());

        let (_diff_map, diff_snapshot) =
            cx.update(|cx| DiffMap::new(inlay_snapshot.clone(), multibuffer.clone(), cx));

        assert_eq!(diff_snapshot.buffer_rows(0).collect::<Vec<_>>(), [Some(0)]);
    }

    #[track_caller]
    fn assert_new_snapshot(
        snapshot: &mut DiffMapSnapshot,
        (new_snapshot, edits): (DiffMapSnapshot, Vec<Edit<DiffOffset>>),
        expected_text: &str,
    ) {
        assert_eq!(new_snapshot.text(), expected_text);
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

    fn init_test(cx: &mut AppContext) {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        language::init(cx);
        crate::init(cx);
        Project::init_settings(cx);
    }
}

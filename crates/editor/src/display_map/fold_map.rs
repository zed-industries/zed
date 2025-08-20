use crate::{InlayId, display_map::inlay_map::InlayChunk};

use super::{
    Highlights,
    inlay_map::{InlayBufferRows, InlayChunks, InlayEdit, InlayOffset, InlayPoint, InlaySnapshot},
};
use gpui::{AnyElement, App, ElementId, HighlightStyle, Pixels, Window};
use language::{Edit, HighlightId, Point, TextSummary};
use multi_buffer::{
    Anchor, AnchorRangeExt, MultiBufferRow, MultiBufferSnapshot, RowInfo, ToOffset,
};
use std::{
    any::TypeId,
    cmp::{self, Ordering},
    fmt, iter,
    ops::{Add, AddAssign, Deref, DerefMut, Range, Sub},
    sync::Arc,
    usize,
};
use sum_tree::{Bias, Cursor, Dimensions, FilterCursor, SumTree, Summary, TreeMap};
use ui::IntoElement as _;
use util::post_inc;

#[derive(Clone)]
pub struct FoldPlaceholder {
    /// Creates an element to represent this fold's placeholder.
    pub render: Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut App) -> AnyElement>,
    /// If true, the element is constrained to the shaped width of an ellipsis.
    pub constrain_width: bool,
    /// If true, merges the fold with an adjacent one.
    pub merge_adjacent: bool,
    /// Category of the fold. Useful for carefully removing from overlapping folds.
    pub type_tag: Option<TypeId>,
}

impl Default for FoldPlaceholder {
    fn default() -> Self {
        Self {
            render: Arc::new(|_, _, _| gpui::Empty.into_any_element()),
            constrain_width: true,
            merge_adjacent: true,
            type_tag: None,
        }
    }
}

impl FoldPlaceholder {
    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        Self {
            render: Arc::new(|_id, _range, _cx| gpui::Empty.into_any_element()),
            constrain_width: true,
            merge_adjacent: true,
            type_tag: None,
        }
    }
}

impl fmt::Debug for FoldPlaceholder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FoldPlaceholder")
            .field("constrain_width", &self.constrain_width)
            .finish()
    }
}

impl Eq for FoldPlaceholder {}

impl PartialEq for FoldPlaceholder {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.render, &other.render) && self.constrain_width == other.constrain_width
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct FoldPoint(pub Point);

impl FoldPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(Point::new(row, column))
    }

    pub fn row(self) -> u32 {
        self.0.row
    }

    pub fn column(self) -> u32 {
        self.0.column
    }

    pub fn row_mut(&mut self) -> &mut u32 {
        &mut self.0.row
    }

    #[cfg(test)]
    pub fn column_mut(&mut self) -> &mut u32 {
        &mut self.0.column
    }

    pub fn to_inlay_point(self, snapshot: &FoldSnapshot) -> InlayPoint {
        let mut cursor = snapshot
            .transforms
            .cursor::<Dimensions<FoldPoint, InlayPoint>>(&());
        cursor.seek(&self, Bias::Right);
        let overshoot = self.0 - cursor.start().0.0;
        InlayPoint(cursor.start().1.0 + overshoot)
    }

    pub fn to_offset(self, snapshot: &FoldSnapshot) -> FoldOffset {
        let mut cursor = snapshot
            .transforms
            .cursor::<Dimensions<FoldPoint, TransformSummary>>(&());
        cursor.seek(&self, Bias::Right);
        let overshoot = self.0 - cursor.start().1.output.lines;
        let mut offset = cursor.start().1.output.len;
        if !overshoot.is_zero() {
            let transform = cursor.item().expect("display point out of range");
            assert!(transform.placeholder.is_none());
            let end_inlay_offset = snapshot
                .inlay_snapshot
                .to_offset(InlayPoint(cursor.start().1.input.lines + overshoot));
            offset += end_inlay_offset.0 - cursor.start().1.input.len;
        }
        FoldOffset(offset)
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for FoldPoint {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.output.lines;
    }
}

pub(crate) struct FoldMapWriter<'a>(&'a mut FoldMap);

impl FoldMapWriter<'_> {
    pub(crate) fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = (Range<T>, FoldPlaceholder)>,
    ) -> (FoldSnapshot, Vec<FoldEdit>) {
        let mut edits = Vec::new();
        let mut folds = Vec::new();
        let snapshot = self.0.snapshot.inlay_snapshot.clone();
        for (range, fold_text) in ranges.into_iter() {
            let buffer = &snapshot.buffer;
            let range = range.start.to_offset(buffer)..range.end.to_offset(buffer);

            // Ignore any empty ranges.
            if range.start == range.end {
                continue;
            }

            // For now, ignore any ranges that span an excerpt boundary.
            let fold_range =
                FoldRange(buffer.anchor_after(range.start)..buffer.anchor_before(range.end));
            if fold_range.0.start.excerpt_id != fold_range.0.end.excerpt_id {
                continue;
            }

            folds.push(Fold {
                id: FoldId(post_inc(&mut self.0.next_fold_id.0)),
                range: fold_range,
                placeholder: fold_text,
            });

            let inlay_range =
                snapshot.to_inlay_offset(range.start)..snapshot.to_inlay_offset(range.end);
            edits.push(InlayEdit {
                old: inlay_range.clone(),
                new: inlay_range,
            });
        }

        let buffer = &snapshot.buffer;
        folds.sort_unstable_by(|a, b| sum_tree::SeekTarget::cmp(&a.range, &b.range, buffer));

        self.0.snapshot.folds = {
            let mut new_tree = SumTree::new(buffer);
            let mut cursor = self.0.snapshot.folds.cursor::<FoldRange>(buffer);
            for fold in folds {
                self.0.snapshot.fold_metadata_by_id.insert(
                    fold.id,
                    FoldMetadata {
                        range: fold.range.clone(),
                        width: None,
                    },
                );
                new_tree.append(cursor.slice(&fold.range, Bias::Right), buffer);
                new_tree.push(fold, buffer);
            }
            new_tree.append(cursor.suffix(), buffer);
            new_tree
        };

        let edits = consolidate_inlay_edits(edits);
        let edits = self.0.sync(snapshot.clone(), edits);
        (self.0.snapshot.clone(), edits)
    }

    /// Removes any folds with the given ranges.
    pub(crate) fn remove_folds<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        type_id: TypeId,
    ) -> (FoldSnapshot, Vec<FoldEdit>) {
        self.remove_folds_with(
            ranges,
            |fold| fold.placeholder.type_tag == Some(type_id),
            false,
        )
    }

    /// Removes any folds whose ranges intersect the given ranges.
    pub(crate) fn unfold_intersecting<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        inclusive: bool,
    ) -> (FoldSnapshot, Vec<FoldEdit>) {
        self.remove_folds_with(ranges, |_| true, inclusive)
    }

    /// Removes any folds that intersect the given ranges and for which the given predicate
    /// returns true.
    fn remove_folds_with<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        should_unfold: impl Fn(&Fold) -> bool,
        inclusive: bool,
    ) -> (FoldSnapshot, Vec<FoldEdit>) {
        let mut edits = Vec::new();
        let mut fold_ixs_to_delete = Vec::new();
        let snapshot = self.0.snapshot.inlay_snapshot.clone();
        let buffer = &snapshot.buffer;
        for range in ranges.into_iter() {
            let range = range.start.to_offset(buffer)..range.end.to_offset(buffer);
            let mut folds_cursor =
                intersecting_folds(&snapshot, &self.0.snapshot.folds, range.clone(), inclusive);
            while let Some(fold) = folds_cursor.item() {
                let offset_range =
                    fold.range.start.to_offset(buffer)..fold.range.end.to_offset(buffer);
                if should_unfold(fold) {
                    if offset_range.end > offset_range.start {
                        let inlay_range = snapshot.to_inlay_offset(offset_range.start)
                            ..snapshot.to_inlay_offset(offset_range.end);
                        edits.push(InlayEdit {
                            old: inlay_range.clone(),
                            new: inlay_range,
                        });
                    }
                    fold_ixs_to_delete.push(*folds_cursor.start());
                    self.0.snapshot.fold_metadata_by_id.remove(&fold.id);
                }
                folds_cursor.next();
            }
        }

        fold_ixs_to_delete.sort_unstable();
        fold_ixs_to_delete.dedup();

        self.0.snapshot.folds = {
            let mut cursor = self.0.snapshot.folds.cursor::<usize>(buffer);
            let mut folds = SumTree::new(buffer);
            for fold_ix in fold_ixs_to_delete {
                folds.append(cursor.slice(&fold_ix, Bias::Right), buffer);
                cursor.next();
            }
            folds.append(cursor.suffix(), buffer);
            folds
        };

        let edits = consolidate_inlay_edits(edits);
        let edits = self.0.sync(snapshot.clone(), edits);
        (self.0.snapshot.clone(), edits)
    }

    pub(crate) fn update_fold_widths(
        &mut self,
        new_widths: impl IntoIterator<Item = (ChunkRendererId, Pixels)>,
    ) -> (FoldSnapshot, Vec<FoldEdit>) {
        let mut edits = Vec::new();
        let inlay_snapshot = self.0.snapshot.inlay_snapshot.clone();
        let buffer = &inlay_snapshot.buffer;

        for (id, new_width) in new_widths {
            let ChunkRendererId::Fold(id) = id else {
                continue;
            };
            if let Some(metadata) = self.0.snapshot.fold_metadata_by_id.get(&id).cloned()
                && Some(new_width) != metadata.width
            {
                let buffer_start = metadata.range.start.to_offset(buffer);
                let buffer_end = metadata.range.end.to_offset(buffer);
                let inlay_range = inlay_snapshot.to_inlay_offset(buffer_start)
                    ..inlay_snapshot.to_inlay_offset(buffer_end);
                edits.push(InlayEdit {
                    old: inlay_range.clone(),
                    new: inlay_range.clone(),
                });

                self.0.snapshot.fold_metadata_by_id.insert(
                    id,
                    FoldMetadata {
                        range: metadata.range,
                        width: Some(new_width),
                    },
                );
            }
        }

        let edits = consolidate_inlay_edits(edits);
        let edits = self.0.sync(inlay_snapshot, edits);
        (self.0.snapshot.clone(), edits)
    }
}

/// Decides where the fold indicators should be; also tracks parts of a source file that are currently folded.
///
/// See the [`display_map` module documentation](crate::display_map) for more information.
pub(crate) struct FoldMap {
    snapshot: FoldSnapshot,
    next_fold_id: FoldId,
}

impl FoldMap {
    pub(crate) fn new(inlay_snapshot: InlaySnapshot) -> (Self, FoldSnapshot) {
        let this = Self {
            snapshot: FoldSnapshot {
                folds: SumTree::new(&inlay_snapshot.buffer),
                transforms: SumTree::from_item(
                    Transform {
                        summary: TransformSummary {
                            input: inlay_snapshot.text_summary(),
                            output: inlay_snapshot.text_summary(),
                        },
                        placeholder: None,
                    },
                    &(),
                ),
                inlay_snapshot: inlay_snapshot.clone(),
                version: 0,
                fold_metadata_by_id: TreeMap::default(),
            },
            next_fold_id: FoldId::default(),
        };
        let snapshot = this.snapshot.clone();
        (this, snapshot)
    }

    pub fn read(
        &mut self,
        inlay_snapshot: InlaySnapshot,
        edits: Vec<InlayEdit>,
    ) -> (FoldSnapshot, Vec<FoldEdit>) {
        let edits = self.sync(inlay_snapshot, edits);
        self.check_invariants();
        (self.snapshot.clone(), edits)
    }

    pub fn write(
        &mut self,
        inlay_snapshot: InlaySnapshot,
        edits: Vec<InlayEdit>,
    ) -> (FoldMapWriter<'_>, FoldSnapshot, Vec<FoldEdit>) {
        let (snapshot, edits) = self.read(inlay_snapshot, edits);
        (FoldMapWriter(self), snapshot, edits)
    }

    fn check_invariants(&self) {
        if cfg!(test) {
            assert_eq!(
                self.snapshot.transforms.summary().input.len,
                self.snapshot.inlay_snapshot.len().0,
                "transform tree does not match inlay snapshot's length"
            );

            let mut prev_transform_isomorphic = false;
            for transform in self.snapshot.transforms.iter() {
                if !transform.is_fold() && prev_transform_isomorphic {
                    panic!(
                        "found adjacent isomorphic transforms: {:?}",
                        self.snapshot.transforms.items(&())
                    );
                }
                prev_transform_isomorphic = !transform.is_fold();
            }

            let mut folds = self.snapshot.folds.iter().peekable();
            while let Some(fold) = folds.next() {
                if let Some(next_fold) = folds.peek() {
                    let comparison = fold.range.cmp(&next_fold.range, self.snapshot.buffer());
                    assert!(comparison.is_le());
                }
            }
        }
    }

    fn sync(
        &mut self,
        inlay_snapshot: InlaySnapshot,
        inlay_edits: Vec<InlayEdit>,
    ) -> Vec<FoldEdit> {
        if inlay_edits.is_empty() {
            if self.snapshot.inlay_snapshot.version != inlay_snapshot.version {
                self.snapshot.version += 1;
            }
            self.snapshot.inlay_snapshot = inlay_snapshot;
            Vec::new()
        } else {
            let mut inlay_edits_iter = inlay_edits.iter().cloned().peekable();

            let mut new_transforms = SumTree::<Transform>::default();
            let mut cursor = self.snapshot.transforms.cursor::<InlayOffset>(&());
            cursor.seek(&InlayOffset(0), Bias::Right);

            while let Some(mut edit) = inlay_edits_iter.next() {
                if let Some(item) = cursor.item()
                    && !item.is_fold()
                {
                    new_transforms.update_last(
                        |transform| {
                            if !transform.is_fold() {
                                transform.summary.add_summary(&item.summary, &());
                                cursor.next();
                            }
                        },
                        &(),
                    );
                }
                new_transforms.append(cursor.slice(&edit.old.start, Bias::Left), &());
                edit.new.start -= edit.old.start - *cursor.start();
                edit.old.start = *cursor.start();

                cursor.seek(&edit.old.end, Bias::Right);
                cursor.next();

                let mut delta = edit.new_len().0 as isize - edit.old_len().0 as isize;
                loop {
                    edit.old.end = *cursor.start();

                    if let Some(next_edit) = inlay_edits_iter.peek() {
                        if next_edit.old.start > edit.old.end {
                            break;
                        }

                        let next_edit = inlay_edits_iter.next().unwrap();
                        delta += next_edit.new_len().0 as isize - next_edit.old_len().0 as isize;

                        if next_edit.old.end >= edit.old.end {
                            edit.old.end = next_edit.old.end;
                            cursor.seek(&edit.old.end, Bias::Right);
                            cursor.next();
                        }
                    } else {
                        break;
                    }
                }

                edit.new.end =
                    InlayOffset(((edit.new.start + edit.old_len()).0 as isize + delta) as usize);

                let anchor = inlay_snapshot
                    .buffer
                    .anchor_before(inlay_snapshot.to_buffer_offset(edit.new.start));
                let mut folds_cursor = self
                    .snapshot
                    .folds
                    .cursor::<FoldRange>(&inlay_snapshot.buffer);
                folds_cursor.seek(&FoldRange(anchor..Anchor::max()), Bias::Left);

                let mut folds = iter::from_fn({
                    let inlay_snapshot = &inlay_snapshot;
                    move || {
                        let item = folds_cursor.item().map(|fold| {
                            let buffer_start = fold.range.start.to_offset(&inlay_snapshot.buffer);
                            let buffer_end = fold.range.end.to_offset(&inlay_snapshot.buffer);
                            (
                                fold.clone(),
                                inlay_snapshot.to_inlay_offset(buffer_start)
                                    ..inlay_snapshot.to_inlay_offset(buffer_end),
                            )
                        });
                        folds_cursor.next();
                        item
                    }
                })
                .peekable();

                while folds
                    .peek()
                    .is_some_and(|(_, fold_range)| fold_range.start < edit.new.end)
                {
                    let (fold, mut fold_range) = folds.next().unwrap();
                    let sum = new_transforms.summary();

                    assert!(fold_range.start.0 >= sum.input.len);

                    while folds.peek().is_some_and(|(next_fold, next_fold_range)| {
                        next_fold_range.start < fold_range.end
                            || (next_fold_range.start == fold_range.end
                                && fold.placeholder.merge_adjacent
                                && next_fold.placeholder.merge_adjacent)
                    }) {
                        let (_, next_fold_range) = folds.next().unwrap();
                        if next_fold_range.end > fold_range.end {
                            fold_range.end = next_fold_range.end;
                        }
                    }

                    if fold_range.start.0 > sum.input.len {
                        let text_summary = inlay_snapshot
                            .text_summary_for_range(InlayOffset(sum.input.len)..fold_range.start);
                        push_isomorphic(&mut new_transforms, text_summary);
                    }

                    if fold_range.end > fold_range.start {
                        const ELLIPSIS: &str = "⋯";

                        let fold_id = fold.id;
                        new_transforms.push(
                            Transform {
                                summary: TransformSummary {
                                    output: TextSummary::from(ELLIPSIS),
                                    input: inlay_snapshot
                                        .text_summary_for_range(fold_range.start..fold_range.end),
                                },
                                placeholder: Some(TransformPlaceholder {
                                    text: ELLIPSIS,
                                    renderer: ChunkRenderer {
                                        id: ChunkRendererId::Fold(fold.id),
                                        render: Arc::new(move |cx| {
                                            (fold.placeholder.render)(
                                                fold_id,
                                                fold.range.0.clone(),
                                                cx.context,
                                            )
                                        }),
                                        constrain_width: fold.placeholder.constrain_width,
                                        measured_width: self.snapshot.fold_width(&fold_id),
                                    },
                                }),
                            },
                            &(),
                        );
                    }
                }

                let sum = new_transforms.summary();
                if sum.input.len < edit.new.end.0 {
                    let text_summary = inlay_snapshot
                        .text_summary_for_range(InlayOffset(sum.input.len)..edit.new.end);
                    push_isomorphic(&mut new_transforms, text_summary);
                }
            }

            new_transforms.append(cursor.suffix(), &());
            if new_transforms.is_empty() {
                let text_summary = inlay_snapshot.text_summary();
                push_isomorphic(&mut new_transforms, text_summary);
            }

            drop(cursor);

            let mut fold_edits = Vec::with_capacity(inlay_edits.len());
            {
                let mut old_transforms = self
                    .snapshot
                    .transforms
                    .cursor::<Dimensions<InlayOffset, FoldOffset>>(&());
                let mut new_transforms =
                    new_transforms.cursor::<Dimensions<InlayOffset, FoldOffset>>(&());

                for mut edit in inlay_edits {
                    old_transforms.seek(&edit.old.start, Bias::Left);
                    if old_transforms.item().is_some_and(|t| t.is_fold()) {
                        edit.old.start = old_transforms.start().0;
                    }
                    let old_start =
                        old_transforms.start().1.0 + (edit.old.start - old_transforms.start().0).0;

                    old_transforms.seek_forward(&edit.old.end, Bias::Right);
                    if old_transforms.item().is_some_and(|t| t.is_fold()) {
                        old_transforms.next();
                        edit.old.end = old_transforms.start().0;
                    }
                    let old_end =
                        old_transforms.start().1.0 + (edit.old.end - old_transforms.start().0).0;

                    new_transforms.seek(&edit.new.start, Bias::Left);
                    if new_transforms.item().is_some_and(|t| t.is_fold()) {
                        edit.new.start = new_transforms.start().0;
                    }
                    let new_start =
                        new_transforms.start().1.0 + (edit.new.start - new_transforms.start().0).0;

                    new_transforms.seek_forward(&edit.new.end, Bias::Right);
                    if new_transforms.item().is_some_and(|t| t.is_fold()) {
                        new_transforms.next();
                        edit.new.end = new_transforms.start().0;
                    }
                    let new_end =
                        new_transforms.start().1.0 + (edit.new.end - new_transforms.start().0).0;

                    fold_edits.push(FoldEdit {
                        old: FoldOffset(old_start)..FoldOffset(old_end),
                        new: FoldOffset(new_start)..FoldOffset(new_end),
                    });
                }

                fold_edits = consolidate_fold_edits(fold_edits);
            }

            self.snapshot.transforms = new_transforms;
            self.snapshot.inlay_snapshot = inlay_snapshot;
            self.snapshot.version += 1;
            fold_edits
        }
    }
}

#[derive(Clone)]
pub struct FoldSnapshot {
    transforms: SumTree<Transform>,
    folds: SumTree<Fold>,
    fold_metadata_by_id: TreeMap<FoldId, FoldMetadata>,
    pub inlay_snapshot: InlaySnapshot,
    pub version: usize,
}

impl FoldSnapshot {
    pub fn buffer(&self) -> &MultiBufferSnapshot {
        &self.inlay_snapshot.buffer
    }

    fn fold_width(&self, fold_id: &FoldId) -> Option<Pixels> {
        self.fold_metadata_by_id.get(fold_id)?.width
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(FoldOffset(0)..self.len(), false, Highlights::default())
            .map(|c| c.text)
            .collect()
    }

    #[cfg(test)]
    pub fn fold_count(&self) -> usize {
        self.folds.items(&self.inlay_snapshot.buffer).len()
    }

    pub fn text_summary_for_range(&self, range: Range<FoldPoint>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self
            .transforms
            .cursor::<Dimensions<FoldPoint, InlayPoint>>(&());
        cursor.seek(&range.start, Bias::Right);
        if let Some(transform) = cursor.item() {
            let start_in_transform = range.start.0 - cursor.start().0.0;
            let end_in_transform = cmp::min(range.end, cursor.end().0).0 - cursor.start().0.0;
            if let Some(placeholder) = transform.placeholder.as_ref() {
                summary = TextSummary::from(
                    &placeholder.text
                        [start_in_transform.column as usize..end_in_transform.column as usize],
                );
            } else {
                let inlay_start = self
                    .inlay_snapshot
                    .to_offset(InlayPoint(cursor.start().1.0 + start_in_transform));
                let inlay_end = self
                    .inlay_snapshot
                    .to_offset(InlayPoint(cursor.start().1.0 + end_in_transform));
                summary = self
                    .inlay_snapshot
                    .text_summary_for_range(inlay_start..inlay_end);
            }
        }

        if range.end > cursor.end().0 {
            cursor.next();
            summary += &cursor
                .summary::<_, TransformSummary>(&range.end, Bias::Right)
                .output;
            if let Some(transform) = cursor.item() {
                let end_in_transform = range.end.0 - cursor.start().0.0;
                if let Some(placeholder) = transform.placeholder.as_ref() {
                    summary +=
                        TextSummary::from(&placeholder.text[..end_in_transform.column as usize]);
                } else {
                    let inlay_start = self.inlay_snapshot.to_offset(cursor.start().1);
                    let inlay_end = self
                        .inlay_snapshot
                        .to_offset(InlayPoint(cursor.start().1.0 + end_in_transform));
                    summary += self
                        .inlay_snapshot
                        .text_summary_for_range(inlay_start..inlay_end);
                }
            }
        }

        summary
    }

    pub fn to_fold_point(&self, point: InlayPoint, bias: Bias) -> FoldPoint {
        let mut cursor = self
            .transforms
            .cursor::<Dimensions<InlayPoint, FoldPoint>>(&());
        cursor.seek(&point, Bias::Right);
        if cursor.item().is_some_and(|t| t.is_fold()) {
            if bias == Bias::Left || point == cursor.start().0 {
                cursor.start().1
            } else {
                cursor.end().1
            }
        } else {
            let overshoot = point.0 - cursor.start().0.0;
            FoldPoint(cmp::min(cursor.start().1.0 + overshoot, cursor.end().1.0))
        }
    }

    pub fn len(&self) -> FoldOffset {
        FoldOffset(self.transforms.summary().output.len)
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let line_start = FoldPoint::new(row, 0).to_offset(self).0;
        let line_end = if row >= self.max_point().row() {
            self.len().0
        } else {
            FoldPoint::new(row + 1, 0).to_offset(self).0 - 1
        };
        (line_end - line_start) as u32
    }

    pub fn row_infos(&self, start_row: u32) -> FoldRows<'_> {
        if start_row > self.transforms.summary().output.lines.row {
            panic!("invalid display row {}", start_row);
        }

        let fold_point = FoldPoint::new(start_row, 0);
        let mut cursor = self
            .transforms
            .cursor::<Dimensions<FoldPoint, InlayPoint>>(&());
        cursor.seek(&fold_point, Bias::Left);

        let overshoot = fold_point.0 - cursor.start().0.0;
        let inlay_point = InlayPoint(cursor.start().1.0 + overshoot);
        let input_rows = self.inlay_snapshot.row_infos(inlay_point.row());

        FoldRows {
            fold_point,
            input_rows,
            cursor,
        }
    }

    pub fn max_point(&self) -> FoldPoint {
        FoldPoint(self.transforms.summary().output.lines)
    }

    #[cfg(test)]
    pub fn longest_row(&self) -> u32 {
        self.transforms.summary().output.longest_row
    }

    pub fn folds_in_range<T>(&self, range: Range<T>) -> impl Iterator<Item = &Fold>
    where
        T: ToOffset,
    {
        let buffer = &self.inlay_snapshot.buffer;
        let range = range.start.to_offset(buffer)..range.end.to_offset(buffer);
        let mut folds = intersecting_folds(&self.inlay_snapshot, &self.folds, range, false);
        iter::from_fn(move || {
            let item = folds.item();
            folds.next();
            item
        })
    }

    pub fn intersects_fold<T>(&self, offset: T) -> bool
    where
        T: ToOffset,
    {
        let buffer_offset = offset.to_offset(&self.inlay_snapshot.buffer);
        let inlay_offset = self.inlay_snapshot.to_inlay_offset(buffer_offset);
        let mut cursor = self.transforms.cursor::<InlayOffset>(&());
        cursor.seek(&inlay_offset, Bias::Right);
        cursor.item().is_some_and(|t| t.placeholder.is_some())
    }

    pub fn is_line_folded(&self, buffer_row: MultiBufferRow) -> bool {
        let mut inlay_point = self
            .inlay_snapshot
            .to_inlay_point(Point::new(buffer_row.0, 0));
        let mut cursor = self.transforms.cursor::<InlayPoint>(&());
        cursor.seek(&inlay_point, Bias::Right);
        loop {
            match cursor.item() {
                Some(transform) => {
                    let buffer_point = self.inlay_snapshot.to_buffer_point(inlay_point);
                    if buffer_point.row != buffer_row.0 {
                        return false;
                    } else if transform.placeholder.is_some() {
                        return true;
                    }
                }
                None => return false,
            }

            if cursor.end().row() == inlay_point.row() {
                cursor.next();
            } else {
                inlay_point.0 += Point::new(1, 0);
                cursor.seek(&inlay_point, Bias::Right);
            }
        }
    }

    pub(crate) fn chunks<'a>(
        &'a self,
        range: Range<FoldOffset>,
        language_aware: bool,
        highlights: Highlights<'a>,
    ) -> FoldChunks<'a> {
        let mut transform_cursor = self
            .transforms
            .cursor::<Dimensions<FoldOffset, InlayOffset>>(&());
        transform_cursor.seek(&range.start, Bias::Right);

        let inlay_start = {
            let overshoot = range.start.0 - transform_cursor.start().0.0;
            transform_cursor.start().1 + InlayOffset(overshoot)
        };

        let transform_end = transform_cursor.end();

        let inlay_end = if transform_cursor
            .item()
            .is_none_or(|transform| transform.is_fold())
        {
            inlay_start
        } else if range.end < transform_end.0 {
            let overshoot = range.end.0 - transform_cursor.start().0.0;
            transform_cursor.start().1 + InlayOffset(overshoot)
        } else {
            transform_end.1
        };

        FoldChunks {
            transform_cursor,
            inlay_chunks: self.inlay_snapshot.chunks(
                inlay_start..inlay_end,
                language_aware,
                highlights,
            ),
            inlay_chunk: None,
            inlay_offset: inlay_start,
            output_offset: range.start,
            max_output_offset: range.end,
        }
    }

    pub fn chars_at(&self, start: FoldPoint) -> impl '_ + Iterator<Item = char> {
        self.chunks(
            start.to_offset(self)..self.len(),
            false,
            Highlights::default(),
        )
        .flat_map(|chunk| chunk.text.chars())
    }

    #[cfg(test)]
    pub fn clip_offset(&self, offset: FoldOffset, bias: Bias) -> FoldOffset {
        if offset > self.len() {
            self.len()
        } else {
            self.clip_point(offset.to_point(self), bias).to_offset(self)
        }
    }

    pub fn clip_point(&self, point: FoldPoint, bias: Bias) -> FoldPoint {
        let mut cursor = self
            .transforms
            .cursor::<Dimensions<FoldPoint, InlayPoint>>(&());
        cursor.seek(&point, Bias::Right);
        if let Some(transform) = cursor.item() {
            let transform_start = cursor.start().0.0;
            if transform.placeholder.is_some() {
                if point.0 == transform_start || matches!(bias, Bias::Left) {
                    FoldPoint(transform_start)
                } else {
                    FoldPoint(cursor.end().0.0)
                }
            } else {
                let overshoot = InlayPoint(point.0 - transform_start);
                let inlay_point = cursor.start().1 + overshoot;
                let clipped_inlay_point = self.inlay_snapshot.clip_point(inlay_point, bias);
                FoldPoint(cursor.start().0.0 + (clipped_inlay_point - cursor.start().1).0)
            }
        } else {
            FoldPoint(self.transforms.summary().output.lines)
        }
    }
}

fn push_isomorphic(transforms: &mut SumTree<Transform>, summary: TextSummary) {
    let mut did_merge = false;
    transforms.update_last(
        |last| {
            if !last.is_fold() {
                last.summary.input += summary;
                last.summary.output += summary;
                did_merge = true;
            }
        },
        &(),
    );
    if !did_merge {
        transforms.push(
            Transform {
                summary: TransformSummary {
                    input: summary,
                    output: summary,
                },
                placeholder: None,
            },
            &(),
        )
    }
}

fn intersecting_folds<'a>(
    inlay_snapshot: &'a InlaySnapshot,
    folds: &'a SumTree<Fold>,
    range: Range<usize>,
    inclusive: bool,
) -> FilterCursor<'a, impl 'a + FnMut(&FoldSummary) -> bool, Fold, usize> {
    let buffer = &inlay_snapshot.buffer;
    let start = buffer.anchor_before(range.start.to_offset(buffer));
    let end = buffer.anchor_after(range.end.to_offset(buffer));
    let mut cursor = folds.filter::<_, usize>(buffer, move |summary| {
        let start_cmp = start.cmp(&summary.max_end, buffer);
        let end_cmp = end.cmp(&summary.min_start, buffer);

        if inclusive {
            start_cmp <= Ordering::Equal && end_cmp >= Ordering::Equal
        } else {
            start_cmp == Ordering::Less && end_cmp == Ordering::Greater
        }
    });
    cursor.next();
    cursor
}

fn consolidate_inlay_edits(mut edits: Vec<InlayEdit>) -> Vec<InlayEdit> {
    edits.sort_unstable_by(|a, b| {
        a.old
            .start
            .cmp(&b.old.start)
            .then_with(|| b.old.end.cmp(&a.old.end))
    });

    let _old_alloc_ptr = edits.as_ptr();
    let mut inlay_edits = edits.into_iter();

    if let Some(mut first_edit) = inlay_edits.next() {
        // This code relies on reusing allocations from the Vec<_> - at the time of writing .flatten() prevents them.
        #[allow(clippy::filter_map_identity)]
        let mut v: Vec<_> = inlay_edits
            .scan(&mut first_edit, |prev_edit, edit| {
                if prev_edit.old.end >= edit.old.start {
                    prev_edit.old.end = prev_edit.old.end.max(edit.old.end);
                    prev_edit.new.start = prev_edit.new.start.min(edit.new.start);
                    prev_edit.new.end = prev_edit.new.end.max(edit.new.end);
                    Some(None) // Skip this edit, it's merged
                } else {
                    let prev = std::mem::replace(*prev_edit, edit);
                    Some(Some(prev)) // Yield the previous edit
                }
            })
            .filter_map(|x| x)
            .collect();
        v.push(first_edit.clone());
        debug_assert_eq!(_old_alloc_ptr, v.as_ptr(), "Inlay edits were reallocated");
        v
    } else {
        vec![]
    }
}

fn consolidate_fold_edits(mut edits: Vec<FoldEdit>) -> Vec<FoldEdit> {
    edits.sort_unstable_by(|a, b| {
        a.old
            .start
            .cmp(&b.old.start)
            .then_with(|| b.old.end.cmp(&a.old.end))
    });
    let _old_alloc_ptr = edits.as_ptr();
    let mut fold_edits = edits.into_iter();

    if let Some(mut first_edit) = fold_edits.next() {
        // This code relies on reusing allocations from the Vec<_> - at the time of writing .flatten() prevents them.
        #[allow(clippy::filter_map_identity)]
        let mut v: Vec<_> = fold_edits
            .scan(&mut first_edit, |prev_edit, edit| {
                if prev_edit.old.end >= edit.old.start {
                    prev_edit.old.end = prev_edit.old.end.max(edit.old.end);
                    prev_edit.new.start = prev_edit.new.start.min(edit.new.start);
                    prev_edit.new.end = prev_edit.new.end.max(edit.new.end);
                    Some(None) // Skip this edit, it's merged
                } else {
                    let prev = std::mem::replace(*prev_edit, edit);
                    Some(Some(prev)) // Yield the previous edit
                }
            })
            .filter_map(|x| x)
            .collect();
        v.push(first_edit.clone());
        v
    } else {
        vec![]
    }
}

#[derive(Clone, Debug, Default)]
struct Transform {
    summary: TransformSummary,
    placeholder: Option<TransformPlaceholder>,
}

#[derive(Clone, Debug)]
struct TransformPlaceholder {
    text: &'static str,
    renderer: ChunkRenderer,
}

impl Transform {
    fn is_fold(&self) -> bool {
        self.placeholder.is_some()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TransformSummary {
    output: TextSummary,
    input: TextSummary,
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self, _cx: &()) -> Self::Summary {
        self.summary.clone()
    }
}

impl sum_tree::Summary for TransformSummary {
    type Context = ();

    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, other: &Self, _: &()) {
        self.input += &other.input;
        self.output += &other.output;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Ord, PartialOrd, Hash)]
pub struct FoldId(pub(super) usize);

impl From<FoldId> for ElementId {
    fn from(val: FoldId) -> Self {
        val.0.into()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fold {
    pub id: FoldId,
    pub range: FoldRange,
    pub placeholder: FoldPlaceholder,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FoldRange(Range<Anchor>);

impl Deref for FoldRange {
    type Target = Range<Anchor>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FoldRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for FoldRange {
    fn default() -> Self {
        Self(Anchor::min()..Anchor::max())
    }
}

#[derive(Clone, Debug)]
struct FoldMetadata {
    range: FoldRange,
    width: Option<Pixels>,
}

impl sum_tree::Item for Fold {
    type Summary = FoldSummary;

    fn summary(&self, _cx: &MultiBufferSnapshot) -> Self::Summary {
        FoldSummary {
            start: self.range.start,
            end: self.range.end,
            min_start: self.range.start,
            max_end: self.range.end,
            count: 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FoldSummary {
    start: Anchor,
    end: Anchor,
    min_start: Anchor,
    max_end: Anchor,
    count: usize,
}

impl Default for FoldSummary {
    fn default() -> Self {
        Self {
            start: Anchor::min(),
            end: Anchor::max(),
            min_start: Anchor::max(),
            max_end: Anchor::min(),
            count: 0,
        }
    }
}

impl sum_tree::Summary for FoldSummary {
    type Context = MultiBufferSnapshot;

    fn zero(_cx: &MultiBufferSnapshot) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, other: &Self, buffer: &Self::Context) {
        if other.min_start.cmp(&self.min_start, buffer) == Ordering::Less {
            self.min_start = other.min_start;
        }
        if other.max_end.cmp(&self.max_end, buffer) == Ordering::Greater {
            self.max_end = other.max_end;
        }

        #[cfg(debug_assertions)]
        {
            let start_comparison = self.start.cmp(&other.start, buffer);
            assert!(start_comparison <= Ordering::Equal);
            if start_comparison == Ordering::Equal {
                assert!(self.end.cmp(&other.end, buffer) >= Ordering::Equal);
            }
        }

        self.start = other.start;
        self.end = other.end;
        self.count += other.count;
    }
}

impl<'a> sum_tree::Dimension<'a, FoldSummary> for FoldRange {
    fn zero(_cx: &MultiBufferSnapshot) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a FoldSummary, _: &MultiBufferSnapshot) {
        self.0.start = summary.start;
        self.0.end = summary.end;
    }
}

impl sum_tree::SeekTarget<'_, FoldSummary, FoldRange> for FoldRange {
    fn cmp(&self, other: &Self, buffer: &MultiBufferSnapshot) -> Ordering {
        AnchorRangeExt::cmp(&self.0, &other.0, buffer)
    }
}

impl<'a> sum_tree::Dimension<'a, FoldSummary> for usize {
    fn zero(_cx: &MultiBufferSnapshot) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a FoldSummary, _: &MultiBufferSnapshot) {
        *self += summary.count;
    }
}

#[derive(Clone)]
pub struct FoldRows<'a> {
    cursor: Cursor<'a, Transform, Dimensions<FoldPoint, InlayPoint>>,
    input_rows: InlayBufferRows<'a>,
    fold_point: FoldPoint,
}

impl FoldRows<'_> {
    pub(crate) fn seek(&mut self, row: u32) {
        let fold_point = FoldPoint::new(row, 0);
        self.cursor.seek(&fold_point, Bias::Left);
        let overshoot = fold_point.0 - self.cursor.start().0.0;
        let inlay_point = InlayPoint(self.cursor.start().1.0 + overshoot);
        self.input_rows.seek(inlay_point.row());
        self.fold_point = fold_point;
    }
}

impl Iterator for FoldRows<'_> {
    type Item = RowInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let mut traversed_fold = false;
        while self.fold_point > self.cursor.end().0 {
            self.cursor.next();
            traversed_fold = true;
            if self.cursor.item().is_none() {
                break;
            }
        }

        if self.cursor.item().is_some() {
            if traversed_fold {
                self.input_rows.seek(self.cursor.start().1.0.row);
                self.input_rows.next();
            }
            *self.fold_point.row_mut() += 1;
            self.input_rows.next()
        } else {
            None
        }
    }
}

/// A chunk of a buffer's text, along with its syntax highlight and
/// diagnostic status.
#[derive(Clone, Debug, Default)]
pub struct Chunk<'a> {
    /// The text of the chunk.
    pub text: &'a str,
    /// The syntax highlighting style of the chunk.
    pub syntax_highlight_id: Option<HighlightId>,
    /// The highlight style that has been applied to this chunk in
    /// the editor.
    pub highlight_style: Option<HighlightStyle>,
    /// The severity of diagnostic associated with this chunk, if any.
    pub diagnostic_severity: Option<lsp::DiagnosticSeverity>,
    /// Whether this chunk of text is marked as unnecessary.
    pub is_unnecessary: bool,
    /// Whether this chunk of text should be underlined.
    pub underline: bool,
    /// Whether this chunk of text was originally a tab character.
    pub is_tab: bool,
    /// Whether this chunk of text was originally a tab character.
    pub is_inlay: bool,
    /// An optional recipe for how the chunk should be presented.
    pub renderer: Option<ChunkRenderer>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChunkRendererId {
    Fold(FoldId),
    Inlay(InlayId),
}

/// A recipe for how the chunk should be presented.
#[derive(Clone)]
pub struct ChunkRenderer {
    /// The id of the renderer associated with this chunk.
    pub id: ChunkRendererId,
    /// Creates a custom element to represent this chunk.
    pub render: Arc<dyn Send + Sync + Fn(&mut ChunkRendererContext) -> AnyElement>,
    /// If true, the element is constrained to the shaped width of the text.
    pub constrain_width: bool,
    /// The width of the element, as measured during the last layout pass.
    ///
    /// This is None if the element has not been laid out yet.
    pub measured_width: Option<Pixels>,
}

pub struct ChunkRendererContext<'a, 'b> {
    pub window: &'a mut Window,
    pub context: &'b mut App,
    pub max_width: Pixels,
}

impl fmt::Debug for ChunkRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ChunkRenderer")
            .field("constrain_width", &self.constrain_width)
            .finish()
    }
}

impl Deref for ChunkRendererContext<'_, '_> {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        self.context
    }
}

impl DerefMut for ChunkRendererContext<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.context
    }
}

pub struct FoldChunks<'a> {
    transform_cursor: Cursor<'a, Transform, Dimensions<FoldOffset, InlayOffset>>,
    inlay_chunks: InlayChunks<'a>,
    inlay_chunk: Option<(InlayOffset, InlayChunk<'a>)>,
    inlay_offset: InlayOffset,
    output_offset: FoldOffset,
    max_output_offset: FoldOffset,
}

impl FoldChunks<'_> {
    pub(crate) fn seek(&mut self, range: Range<FoldOffset>) {
        self.transform_cursor.seek(&range.start, Bias::Right);

        let inlay_start = {
            let overshoot = range.start.0 - self.transform_cursor.start().0.0;
            self.transform_cursor.start().1 + InlayOffset(overshoot)
        };

        let transform_end = self.transform_cursor.end();

        let inlay_end = if self
            .transform_cursor
            .item()
            .is_none_or(|transform| transform.is_fold())
        {
            inlay_start
        } else if range.end < transform_end.0 {
            let overshoot = range.end.0 - self.transform_cursor.start().0.0;
            self.transform_cursor.start().1 + InlayOffset(overshoot)
        } else {
            transform_end.1
        };

        self.inlay_chunks.seek(inlay_start..inlay_end);
        self.inlay_chunk = None;
        self.inlay_offset = inlay_start;
        self.output_offset = range.start;
        self.max_output_offset = range.end;
    }
}

impl<'a> Iterator for FoldChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_offset >= self.max_output_offset {
            return None;
        }

        let transform = self.transform_cursor.item()?;

        // If we're in a fold, then return the fold's display text and
        // advance the transform and buffer cursors to the end of the fold.
        if let Some(placeholder) = transform.placeholder.as_ref() {
            self.inlay_chunk.take();
            self.inlay_offset += InlayOffset(transform.summary.input.len);

            while self.inlay_offset >= self.transform_cursor.end().1
                && self.transform_cursor.item().is_some()
            {
                self.transform_cursor.next();
            }

            self.output_offset.0 += placeholder.text.len();
            return Some(Chunk {
                text: placeholder.text,
                renderer: Some(placeholder.renderer.clone()),
                ..Default::default()
            });
        }

        // When we reach a non-fold region, seek the underlying text
        // chunk iterator to the next unfolded range.
        if self.inlay_offset == self.transform_cursor.start().1
            && self.inlay_chunks.offset() != self.inlay_offset
        {
            let transform_start = self.transform_cursor.start();
            let transform_end = self.transform_cursor.end();
            let inlay_end = if self.max_output_offset < transform_end.0 {
                let overshoot = self.max_output_offset.0 - transform_start.0.0;
                transform_start.1 + InlayOffset(overshoot)
            } else {
                transform_end.1
            };

            self.inlay_chunks.seek(self.inlay_offset..inlay_end);
        }

        // Retrieve a chunk from the current location in the buffer.
        if self.inlay_chunk.is_none() {
            let chunk_offset = self.inlay_chunks.offset();
            self.inlay_chunk = self.inlay_chunks.next().map(|chunk| (chunk_offset, chunk));
        }

        // Otherwise, take a chunk from the buffer's text.
        if let Some((buffer_chunk_start, mut inlay_chunk)) = self.inlay_chunk.clone() {
            let chunk = &mut inlay_chunk.chunk;
            let buffer_chunk_end = buffer_chunk_start + InlayOffset(chunk.text.len());
            let transform_end = self.transform_cursor.end().1;
            let chunk_end = buffer_chunk_end.min(transform_end);

            chunk.text = &chunk.text
                [(self.inlay_offset - buffer_chunk_start).0..(chunk_end - buffer_chunk_start).0];

            if chunk_end == transform_end {
                self.transform_cursor.next();
            } else if chunk_end == buffer_chunk_end {
                self.inlay_chunk.take();
            }

            self.inlay_offset = chunk_end;
            self.output_offset.0 += chunk.text.len();
            return Some(Chunk {
                text: chunk.text,
                syntax_highlight_id: chunk.syntax_highlight_id,
                highlight_style: chunk.highlight_style,
                diagnostic_severity: chunk.diagnostic_severity,
                is_unnecessary: chunk.is_unnecessary,
                is_tab: chunk.is_tab,
                is_inlay: chunk.is_inlay,
                underline: chunk.underline,
                renderer: inlay_chunk.renderer,
            });
        }

        None
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct FoldOffset(pub usize);

impl FoldOffset {
    pub fn to_point(self, snapshot: &FoldSnapshot) -> FoldPoint {
        let mut cursor = snapshot
            .transforms
            .cursor::<Dimensions<FoldOffset, TransformSummary>>(&());
        cursor.seek(&self, Bias::Right);
        let overshoot = if cursor.item().is_none_or(|t| t.is_fold()) {
            Point::new(0, (self.0 - cursor.start().0.0) as u32)
        } else {
            let inlay_offset = cursor.start().1.input.len + self.0 - cursor.start().0.0;
            let inlay_point = snapshot.inlay_snapshot.to_point(InlayOffset(inlay_offset));
            inlay_point.0 - cursor.start().1.input.lines
        };
        FoldPoint(cursor.start().1.output.lines + overshoot)
    }

    #[cfg(test)]
    pub fn to_inlay_offset(self, snapshot: &FoldSnapshot) -> InlayOffset {
        let mut cursor = snapshot
            .transforms
            .cursor::<Dimensions<FoldOffset, InlayOffset>>(&());
        cursor.seek(&self, Bias::Right);
        let overshoot = self.0 - cursor.start().0.0;
        InlayOffset(cursor.start().1.0 + overshoot)
    }
}

impl Add for FoldOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for FoldOffset {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for FoldOffset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for FoldOffset {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.output.len;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for InlayPoint {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.input.lines;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for InlayOffset {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.input.len;
    }
}

pub type FoldEdit = Edit<FoldOffset>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MultiBuffer, ToPoint, display_map::inlay_map::InlayMap};
    use Bias::{Left, Right};
    use collections::HashSet;
    use rand::prelude::*;
    use settings::SettingsStore;
    use std::{env, mem};
    use text::Patch;
    use util::RandomCharIter;
    use util::test::sample_text;

    #[gpui::test]
    fn test_basic_folds(cx: &mut gpui::App) {
        init_test(cx);
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let mut map = FoldMap::new(inlay_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(inlay_snapshot, vec![]);
        let (snapshot2, edits) = writer.fold(vec![
            (Point::new(0, 2)..Point::new(2, 2), FoldPlaceholder::test()),
            (Point::new(2, 4)..Point::new(4, 1), FoldPlaceholder::test()),
        ]);
        assert_eq!(snapshot2.text(), "aa⋯cc⋯eeeee");
        assert_eq!(
            edits,
            &[
                FoldEdit {
                    old: FoldOffset(2)..FoldOffset(16),
                    new: FoldOffset(2)..FoldOffset(5),
                },
                FoldEdit {
                    old: FoldOffset(18)..FoldOffset(29),
                    new: FoldOffset(7)..FoldOffset(10)
                },
            ]
        );

        let buffer_snapshot = buffer.update(cx, |buffer, cx| {
            buffer.edit(
                vec![
                    (Point::new(0, 0)..Point::new(0, 1), "123"),
                    (Point::new(2, 3)..Point::new(2, 3), "123"),
                ],
                None,
                cx,
            );
            buffer.snapshot(cx)
        });

        let (inlay_snapshot, inlay_edits) =
            inlay_map.sync(buffer_snapshot, subscription.consume().into_inner());
        let (snapshot3, edits) = map.read(inlay_snapshot, inlay_edits);
        assert_eq!(snapshot3.text(), "123a⋯c123c⋯eeeee");
        assert_eq!(
            edits,
            &[
                FoldEdit {
                    old: FoldOffset(0)..FoldOffset(1),
                    new: FoldOffset(0)..FoldOffset(3),
                },
                FoldEdit {
                    old: FoldOffset(6)..FoldOffset(6),
                    new: FoldOffset(8)..FoldOffset(11),
                },
            ]
        );

        let buffer_snapshot = buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 6)..Point::new(4, 3), "456")], None, cx);
            buffer.snapshot(cx)
        });
        let (inlay_snapshot, inlay_edits) =
            inlay_map.sync(buffer_snapshot, subscription.consume().into_inner());
        let (snapshot4, _) = map.read(inlay_snapshot.clone(), inlay_edits);
        assert_eq!(snapshot4.text(), "123a⋯c123456eee");

        let (mut writer, _, _) = map.write(inlay_snapshot.clone(), vec![]);
        writer.unfold_intersecting(Some(Point::new(0, 4)..Point::new(0, 4)), false);
        let (snapshot5, _) = map.read(inlay_snapshot.clone(), vec![]);
        assert_eq!(snapshot5.text(), "123a⋯c123456eee");

        let (mut writer, _, _) = map.write(inlay_snapshot.clone(), vec![]);
        writer.unfold_intersecting(Some(Point::new(0, 4)..Point::new(0, 4)), true);
        let (snapshot6, _) = map.read(inlay_snapshot, vec![]);
        assert_eq!(snapshot6.text(), "123aaaaa\nbbbbbb\nccc123456eee");
    }

    #[gpui::test]
    fn test_adjacent_folds(cx: &mut gpui::App) {
        init_test(cx);
        let buffer = MultiBuffer::build_simple("abcdefghijkl", cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot);

        {
            let mut map = FoldMap::new(inlay_snapshot.clone()).0;

            let (mut writer, _, _) = map.write(inlay_snapshot.clone(), vec![]);
            writer.fold(vec![(5..8, FoldPlaceholder::test())]);
            let (snapshot, _) = map.read(inlay_snapshot.clone(), vec![]);
            assert_eq!(snapshot.text(), "abcde⋯ijkl");

            // Create an fold adjacent to the start of the first fold.
            let (mut writer, _, _) = map.write(inlay_snapshot.clone(), vec![]);
            writer.fold(vec![
                (0..1, FoldPlaceholder::test()),
                (2..5, FoldPlaceholder::test()),
            ]);
            let (snapshot, _) = map.read(inlay_snapshot.clone(), vec![]);
            assert_eq!(snapshot.text(), "⋯b⋯ijkl");

            // Create an fold adjacent to the end of the first fold.
            let (mut writer, _, _) = map.write(inlay_snapshot.clone(), vec![]);
            writer.fold(vec![
                (11..11, FoldPlaceholder::test()),
                (8..10, FoldPlaceholder::test()),
            ]);
            let (snapshot, _) = map.read(inlay_snapshot.clone(), vec![]);
            assert_eq!(snapshot.text(), "⋯b⋯kl");
        }

        {
            let mut map = FoldMap::new(inlay_snapshot.clone()).0;

            // Create two adjacent folds.
            let (mut writer, _, _) = map.write(inlay_snapshot.clone(), vec![]);
            writer.fold(vec![
                (0..2, FoldPlaceholder::test()),
                (2..5, FoldPlaceholder::test()),
            ]);
            let (snapshot, _) = map.read(inlay_snapshot, vec![]);
            assert_eq!(snapshot.text(), "⋯fghijkl");

            // Edit within one of the folds.
            let buffer_snapshot = buffer.update(cx, |buffer, cx| {
                buffer.edit([(0..1, "12345")], None, cx);
                buffer.snapshot(cx)
            });
            let (inlay_snapshot, inlay_edits) =
                inlay_map.sync(buffer_snapshot, subscription.consume().into_inner());
            let (snapshot, _) = map.read(inlay_snapshot, inlay_edits);
            assert_eq!(snapshot.text(), "12345⋯fghijkl");
        }
    }

    #[gpui::test]
    fn test_overlapping_folds(cx: &mut gpui::App) {
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let mut map = FoldMap::new(inlay_snapshot.clone()).0;
        let (mut writer, _, _) = map.write(inlay_snapshot.clone(), vec![]);
        writer.fold(vec![
            (Point::new(0, 2)..Point::new(2, 2), FoldPlaceholder::test()),
            (Point::new(0, 4)..Point::new(1, 0), FoldPlaceholder::test()),
            (Point::new(1, 2)..Point::new(3, 2), FoldPlaceholder::test()),
            (Point::new(3, 1)..Point::new(4, 1), FoldPlaceholder::test()),
        ]);
        let (snapshot, _) = map.read(inlay_snapshot, vec![]);
        assert_eq!(snapshot.text(), "aa⋯eeeee");
    }

    #[gpui::test]
    fn test_merging_folds_via_edit(cx: &mut gpui::App) {
        init_test(cx);
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let mut map = FoldMap::new(inlay_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(inlay_snapshot.clone(), vec![]);
        writer.fold(vec![
            (Point::new(0, 2)..Point::new(2, 2), FoldPlaceholder::test()),
            (Point::new(3, 1)..Point::new(4, 1), FoldPlaceholder::test()),
        ]);
        let (snapshot, _) = map.read(inlay_snapshot, vec![]);
        assert_eq!(snapshot.text(), "aa⋯cccc\nd⋯eeeee");

        let buffer_snapshot = buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 2)..Point::new(3, 1), "")], None, cx);
            buffer.snapshot(cx)
        });
        let (inlay_snapshot, inlay_edits) =
            inlay_map.sync(buffer_snapshot, subscription.consume().into_inner());
        let (snapshot, _) = map.read(inlay_snapshot, inlay_edits);
        assert_eq!(snapshot.text(), "aa⋯eeeee");
    }

    #[gpui::test]
    fn test_folds_in_range(cx: &mut gpui::App) {
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let mut map = FoldMap::new(inlay_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(inlay_snapshot.clone(), vec![]);
        writer.fold(vec![
            (Point::new(0, 2)..Point::new(2, 2), FoldPlaceholder::test()),
            (Point::new(0, 4)..Point::new(1, 0), FoldPlaceholder::test()),
            (Point::new(1, 2)..Point::new(3, 2), FoldPlaceholder::test()),
            (Point::new(3, 1)..Point::new(4, 1), FoldPlaceholder::test()),
        ]);
        let (snapshot, _) = map.read(inlay_snapshot, vec![]);
        let fold_ranges = snapshot
            .folds_in_range(Point::new(1, 0)..Point::new(1, 3))
            .map(|fold| {
                fold.range.start.to_point(&buffer_snapshot)
                    ..fold.range.end.to_point(&buffer_snapshot)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            fold_ranges,
            vec![
                Point::new(0, 2)..Point::new(2, 2),
                Point::new(1, 2)..Point::new(3, 2)
            ]
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random_folds(cx: &mut gpui::App, mut rng: StdRng) {
        init_test(cx);
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let len = rng.gen_range(0..10);
        let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
        let buffer = if rng.r#gen() {
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };
        let mut buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let mut map = FoldMap::new(inlay_snapshot.clone()).0;

        let (mut initial_snapshot, _) = map.read(inlay_snapshot, vec![]);
        let mut snapshot_edits = Vec::new();

        let mut next_inlay_id = 0;
        for _ in 0..operations {
            log::info!("text: {:?}", buffer_snapshot.text());
            let mut buffer_edits = Vec::new();
            let mut inlay_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=39 => {
                    snapshot_edits.extend(map.randomly_mutate(&mut rng));
                }
                40..=59 => {
                    let (_, edits) = inlay_map.randomly_mutate(&mut next_inlay_id, &mut rng);
                    inlay_edits = edits;
                }
                _ => buffer.update(cx, |buffer, cx| {
                    let subscription = buffer.subscribe();
                    let edit_count = rng.gen_range(1..=5);
                    buffer.randomly_mutate(&mut rng, edit_count, cx);
                    buffer_snapshot = buffer.snapshot(cx);
                    let edits = subscription.consume().into_inner();
                    log::info!("editing {:?}", edits);
                    buffer_edits.extend(edits);
                }),
            };

            let (inlay_snapshot, new_inlay_edits) =
                inlay_map.sync(buffer_snapshot.clone(), buffer_edits);
            log::info!("inlay text {:?}", inlay_snapshot.text());

            let inlay_edits = Patch::new(inlay_edits)
                .compose(new_inlay_edits)
                .into_inner();
            let (snapshot, edits) = map.read(inlay_snapshot.clone(), inlay_edits);
            snapshot_edits.push((snapshot.clone(), edits));

            let mut expected_text: String = inlay_snapshot.text().to_string();
            for fold_range in map.merged_folds().into_iter().rev() {
                let fold_inlay_start = inlay_snapshot.to_inlay_offset(fold_range.start);
                let fold_inlay_end = inlay_snapshot.to_inlay_offset(fold_range.end);
                expected_text.replace_range(fold_inlay_start.0..fold_inlay_end.0, "⋯");
            }

            assert_eq!(snapshot.text(), expected_text);
            log::info!(
                "fold text {:?} ({} lines)",
                expected_text,
                expected_text.matches('\n').count() + 1
            );

            let mut prev_row = 0;
            let mut expected_buffer_rows = Vec::new();
            for fold_range in map.merged_folds() {
                let fold_start = inlay_snapshot
                    .to_point(inlay_snapshot.to_inlay_offset(fold_range.start))
                    .row();
                let fold_end = inlay_snapshot
                    .to_point(inlay_snapshot.to_inlay_offset(fold_range.end))
                    .row();
                expected_buffer_rows.extend(
                    inlay_snapshot
                        .row_infos(prev_row)
                        .take((1 + fold_start - prev_row) as usize),
                );
                prev_row = 1 + fold_end;
            }
            expected_buffer_rows.extend(inlay_snapshot.row_infos(prev_row));

            assert_eq!(
                expected_buffer_rows.len(),
                expected_text.matches('\n').count() + 1,
                "wrong expected buffer rows {:?}. text: {:?}",
                expected_buffer_rows,
                expected_text
            );

            for (output_row, line) in expected_text.lines().enumerate() {
                let line_len = snapshot.line_len(output_row as u32);
                assert_eq!(line_len, line.len() as u32);
            }

            let longest_row = snapshot.longest_row();
            let longest_char_column = expected_text
                .split('\n')
                .nth(longest_row as usize)
                .unwrap()
                .chars()
                .count();
            let mut fold_point = FoldPoint::new(0, 0);
            let mut fold_offset = FoldOffset(0);
            let mut char_column = 0;
            for c in expected_text.chars() {
                let inlay_point = fold_point.to_inlay_point(&snapshot);
                let inlay_offset = fold_offset.to_inlay_offset(&snapshot);
                assert_eq!(
                    snapshot.to_fold_point(inlay_point, Right),
                    fold_point,
                    "{:?} -> fold point",
                    inlay_point,
                );
                assert_eq!(
                    inlay_snapshot.to_offset(inlay_point),
                    inlay_offset,
                    "inlay_snapshot.to_offset({:?})",
                    inlay_point,
                );
                assert_eq!(
                    fold_point.to_offset(&snapshot),
                    fold_offset,
                    "fold_point.to_offset({:?})",
                    fold_point,
                );

                if c == '\n' {
                    *fold_point.row_mut() += 1;
                    *fold_point.column_mut() = 0;
                    char_column = 0;
                } else {
                    *fold_point.column_mut() += c.len_utf8() as u32;
                    char_column += 1;
                }
                fold_offset.0 += c.len_utf8();
                if char_column > longest_char_column {
                    panic!(
                        "invalid longest row {:?} (chars {}), found row {:?} (chars: {})",
                        longest_row,
                        longest_char_column,
                        fold_point.row(),
                        char_column
                    );
                }
            }

            for _ in 0..5 {
                let mut start = snapshot
                    .clip_offset(FoldOffset(rng.gen_range(0..=snapshot.len().0)), Bias::Left);
                let mut end = snapshot
                    .clip_offset(FoldOffset(rng.gen_range(0..=snapshot.len().0)), Bias::Right);
                if start > end {
                    mem::swap(&mut start, &mut end);
                }

                let text = &expected_text[start.0..end.0];
                assert_eq!(
                    snapshot
                        .chunks(start..end, false, Highlights::default())
                        .map(|c| c.text)
                        .collect::<String>(),
                    text,
                );
            }

            let mut fold_row = 0;
            while fold_row < expected_buffer_rows.len() as u32 {
                assert_eq!(
                    snapshot.row_infos(fold_row).collect::<Vec<_>>(),
                    expected_buffer_rows[(fold_row as usize)..],
                    "wrong buffer rows starting at fold row {}",
                    fold_row,
                );
                fold_row += 1;
            }

            let folded_buffer_rows = map
                .merged_folds()
                .iter()
                .flat_map(|fold_range| {
                    let start_row = fold_range.start.to_point(&buffer_snapshot).row;
                    let end = fold_range.end.to_point(&buffer_snapshot);
                    if end.column == 0 {
                        start_row..end.row
                    } else {
                        start_row..end.row + 1
                    }
                })
                .collect::<HashSet<_>>();
            for row in 0..=buffer_snapshot.max_point().row {
                assert_eq!(
                    snapshot.is_line_folded(MultiBufferRow(row)),
                    folded_buffer_rows.contains(&row),
                    "expected buffer row {}{} to be folded",
                    row,
                    if folded_buffer_rows.contains(&row) {
                        ""
                    } else {
                        " not"
                    }
                );
            }

            for _ in 0..5 {
                let end =
                    buffer_snapshot.clip_offset(rng.gen_range(0..=buffer_snapshot.len()), Right);
                let start = buffer_snapshot.clip_offset(rng.gen_range(0..=end), Left);
                let expected_folds = map
                    .snapshot
                    .folds
                    .items(&buffer_snapshot)
                    .into_iter()
                    .filter(|fold| {
                        let start = buffer_snapshot.anchor_before(start);
                        let end = buffer_snapshot.anchor_after(end);
                        start.cmp(&fold.range.end, &buffer_snapshot) == Ordering::Less
                            && end.cmp(&fold.range.start, &buffer_snapshot) == Ordering::Greater
                    })
                    .collect::<Vec<_>>();

                assert_eq!(
                    snapshot
                        .folds_in_range(start..end)
                        .cloned()
                        .collect::<Vec<_>>(),
                    expected_folds
                );
            }

            let text = snapshot.text();
            for _ in 0..5 {
                let start_row = rng.gen_range(0..=snapshot.max_point().row());
                let start_column = rng.gen_range(0..=snapshot.line_len(start_row));
                let end_row = rng.gen_range(0..=snapshot.max_point().row());
                let end_column = rng.gen_range(0..=snapshot.line_len(end_row));
                let mut start =
                    snapshot.clip_point(FoldPoint::new(start_row, start_column), Bias::Left);
                let mut end = snapshot.clip_point(FoldPoint::new(end_row, end_column), Bias::Right);
                if start > end {
                    mem::swap(&mut start, &mut end);
                }

                let lines = start..end;
                let bytes = start.to_offset(&snapshot)..end.to_offset(&snapshot);
                assert_eq!(
                    snapshot.text_summary_for_range(lines),
                    TextSummary::from(&text[bytes.start.0..bytes.end.0])
                )
            }

            let mut text = initial_snapshot.text();
            for (snapshot, edits) in snapshot_edits.drain(..) {
                let new_text = snapshot.text();
                for edit in edits {
                    let old_bytes = edit.new.start.0..edit.new.start.0 + edit.old_len().0;
                    let new_bytes = edit.new.start.0..edit.new.end.0;
                    text.replace_range(old_bytes, &new_text[new_bytes]);
                }

                assert_eq!(text, new_text);
                initial_snapshot = snapshot;
            }
        }
    }

    #[gpui::test]
    fn test_buffer_rows(cx: &mut gpui::App) {
        let text = sample_text(6, 6, 'a') + "\n";
        let buffer = MultiBuffer::build_simple(&text, cx);

        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let mut map = FoldMap::new(inlay_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(inlay_snapshot.clone(), vec![]);
        writer.fold(vec![
            (Point::new(0, 2)..Point::new(2, 2), FoldPlaceholder::test()),
            (Point::new(3, 1)..Point::new(4, 1), FoldPlaceholder::test()),
        ]);

        let (snapshot, _) = map.read(inlay_snapshot, vec![]);
        assert_eq!(snapshot.text(), "aa⋯cccc\nd⋯eeeee\nffffff\n");
        assert_eq!(
            snapshot
                .row_infos(0)
                .map(|info| info.buffer_row)
                .collect::<Vec<_>>(),
            [Some(0), Some(3), Some(5), Some(6)]
        );
        assert_eq!(
            snapshot
                .row_infos(3)
                .map(|info| info.buffer_row)
                .collect::<Vec<_>>(),
            [Some(6)]
        );
    }

    fn init_test(cx: &mut gpui::App) {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
    }

    impl FoldMap {
        fn merged_folds(&self) -> Vec<Range<usize>> {
            let inlay_snapshot = self.snapshot.inlay_snapshot.clone();
            let buffer = &inlay_snapshot.buffer;
            let mut folds = self.snapshot.folds.items(buffer);
            // Ensure sorting doesn't change how folds get merged and displayed.
            folds.sort_by(|a, b| a.range.cmp(&b.range, buffer));
            let mut folds = folds
                .iter()
                .map(|fold| fold.range.start.to_offset(buffer)..fold.range.end.to_offset(buffer))
                .peekable();

            let mut merged_folds = Vec::new();
            while let Some(mut fold_range) = folds.next() {
                while let Some(next_range) = folds.peek() {
                    if fold_range.end >= next_range.start {
                        if next_range.end > fold_range.end {
                            fold_range.end = next_range.end;
                        }
                        folds.next();
                    } else {
                        break;
                    }
                }
                if fold_range.end > fold_range.start {
                    merged_folds.push(fold_range);
                }
            }
            merged_folds
        }

        pub fn randomly_mutate(
            &mut self,
            rng: &mut impl Rng,
        ) -> Vec<(FoldSnapshot, Vec<FoldEdit>)> {
            let mut snapshot_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=39 if !self.snapshot.folds.is_empty() => {
                    let inlay_snapshot = self.snapshot.inlay_snapshot.clone();
                    let buffer = &inlay_snapshot.buffer;
                    let mut to_unfold = Vec::new();
                    for _ in 0..rng.gen_range(1..=3) {
                        let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                        let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                        to_unfold.push(start..end);
                    }
                    let inclusive = rng.r#gen();
                    log::info!("unfolding {:?} (inclusive: {})", to_unfold, inclusive);
                    let (mut writer, snapshot, edits) = self.write(inlay_snapshot, vec![]);
                    snapshot_edits.push((snapshot, edits));
                    let (snapshot, edits) = writer.unfold_intersecting(to_unfold, inclusive);
                    snapshot_edits.push((snapshot, edits));
                }
                _ => {
                    let inlay_snapshot = self.snapshot.inlay_snapshot.clone();
                    let buffer = &inlay_snapshot.buffer;
                    let mut to_fold = Vec::new();
                    for _ in 0..rng.gen_range(1..=2) {
                        let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                        let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                        to_fold.push((start..end, FoldPlaceholder::test()));
                    }
                    log::info!("folding {:?}", to_fold);
                    let (mut writer, snapshot, edits) = self.write(inlay_snapshot, vec![]);
                    snapshot_edits.push((snapshot, edits));
                    let (snapshot, edits) = writer.fold(to_fold);
                    snapshot_edits.push((snapshot, edits));
                }
            }
            snapshot_edits
        }
    }
}

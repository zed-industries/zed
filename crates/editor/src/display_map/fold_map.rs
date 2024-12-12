use super::{
    diff_map::{
        DiffEdit, DiffMapBufferRows, DiffMapChunks, DiffMapSnapshot, DiffOffset, DiffPoint,
    },
    Highlights,
};
use gpui::{AnyElement, ElementId, WindowContext};
use language::{Chunk, ChunkRenderer, Edit, Point, TextSummary};
use multi_buffer::{Anchor, AnchorRangeExt, MultiBufferRow, MultiBufferSnapshot, ToOffset};
use std::{
    any::TypeId,
    cmp::{self, Ordering},
    fmt, iter,
    ops::{Add, AddAssign, Deref, DerefMut, Range, Sub},
    sync::Arc,
};
use sum_tree::{Bias, Cursor, FilterCursor, SumTree, Summary};
use ui::IntoElement as _;
use util::post_inc;

#[derive(Clone)]
pub struct FoldPlaceholder {
    /// Creates an element to represent this fold's placeholder.
    pub render: Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut WindowContext) -> AnyElement>,
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

    pub fn to_diff_point(self, snapshot: &FoldSnapshot) -> DiffPoint {
        let mut cursor = snapshot.transforms.cursor::<(FoldPoint, DiffPoint)>(&());
        cursor.seek(&self, Bias::Right, &());
        let overshoot = self.0 - cursor.start().0 .0;
        DiffPoint(cursor.start().1 .0 + overshoot)
    }

    pub fn to_offset(self, snapshot: &FoldSnapshot) -> FoldOffset {
        let mut cursor = snapshot
            .transforms
            .cursor::<(FoldPoint, TransformSummary)>(&());
        cursor.seek(&self, Bias::Right, &());
        let overshoot = self.0 - cursor.start().1.output.lines;
        let mut offset = cursor.start().1.output.len;
        if !overshoot.is_zero() {
            let transform = cursor.item().expect("display point out of range");
            assert!(transform.placeholder.is_none());
            let end_inlay_offset = snapshot
                .diff_map_snapshot
                .to_offset(DiffPoint(cursor.start().1.input.lines + overshoot));
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

impl<'a> FoldMapWriter<'a> {
    pub(crate) fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = (Range<T>, FoldPlaceholder)>,
    ) -> (FoldSnapshot, Vec<FoldEdit>) {
        let mut edits = Vec::new();
        let mut folds = Vec::new();
        let snapshot = self.0.snapshot.diff_map_snapshot.clone();
        let buffer = snapshot.buffer();
        for (range, fold_text) in ranges.into_iter() {
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

            let diff_range =
                snapshot.make_diff_offset(range.start)..snapshot.make_diff_offset(range.end);
            edits.push(DiffEdit {
                old: diff_range.clone(),
                new: diff_range,
            });
        }

        folds.sort_unstable_by(|a, b| sum_tree::SeekTarget::cmp(&a.range, &b.range, buffer));

        self.0.snapshot.folds = {
            let mut new_tree = SumTree::new(buffer);
            let mut cursor = self.0.snapshot.folds.cursor::<FoldRange>(buffer);
            for fold in folds {
                new_tree.append(cursor.slice(&fold.range, Bias::Right, buffer), buffer);
                new_tree.push(fold, buffer);
            }
            new_tree.append(cursor.suffix(buffer), buffer);
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
        let snapshot = self.0.snapshot.diff_map_snapshot.clone();
        let buffer = snapshot.buffer();
        for range in ranges.into_iter() {
            let range = range.start.to_offset(buffer)..range.end.to_offset(buffer);
            let mut folds_cursor =
                intersecting_folds(&snapshot, &self.0.snapshot.folds, range.clone(), inclusive);
            while let Some(fold) = folds_cursor.item() {
                let offset_range =
                    fold.range.start.to_offset(buffer)..fold.range.end.to_offset(buffer);
                if should_unfold(fold) {
                    if offset_range.end > offset_range.start {
                        let diff_range = snapshot.make_diff_offset(offset_range.start)
                            ..snapshot.make_diff_offset(offset_range.end);
                        edits.push(DiffEdit {
                            old: diff_range.clone(),
                            new: diff_range,
                        });
                    }
                    fold_ixs_to_delete.push(*folds_cursor.start());
                }
                folds_cursor.next(buffer);
            }
        }

        fold_ixs_to_delete.sort_unstable();
        fold_ixs_to_delete.dedup();

        self.0.snapshot.folds = {
            let mut cursor = self.0.snapshot.folds.cursor::<usize>(buffer);
            let mut folds = SumTree::new(buffer);
            for fold_ix in fold_ixs_to_delete {
                folds.append(cursor.slice(&fold_ix, Bias::Right, buffer), buffer);
                cursor.next(buffer);
            }
            folds.append(cursor.suffix(buffer), buffer);
            folds
        };

        let edits = consolidate_inlay_edits(edits);
        let edits = self.0.sync(snapshot.clone(), edits);
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
    pub(crate) fn new(diff_map_snapshot: DiffMapSnapshot) -> (Self, FoldSnapshot) {
        let this = Self {
            snapshot: FoldSnapshot {
                folds: SumTree::new(diff_map_snapshot.buffer()),
                transforms: SumTree::from_item(
                    Transform {
                        summary: TransformSummary {
                            input: diff_map_snapshot.text_summary(),
                            output: diff_map_snapshot.text_summary(),
                        },
                        placeholder: None,
                    },
                    &(),
                ),
                diff_map_snapshot: diff_map_snapshot.clone(),
                version: 0,
            },
            next_fold_id: FoldId::default(),
        };
        let snapshot = this.snapshot.clone();
        (this, snapshot)
    }

    pub fn read(
        &mut self,
        diff_map_snapshot: DiffMapSnapshot,
        edits: Vec<DiffEdit>,
    ) -> (FoldSnapshot, Vec<FoldEdit>) {
        let edits = self.sync(diff_map_snapshot, edits);
        self.check_invariants();
        (self.snapshot.clone(), edits)
    }

    pub fn write(
        &mut self,
        diff_map_snapshot: DiffMapSnapshot,
        edits: Vec<DiffEdit>,
    ) -> (FoldMapWriter, FoldSnapshot, Vec<FoldEdit>) {
        let (snapshot, edits) = self.read(diff_map_snapshot, edits);
        (FoldMapWriter(self), snapshot, edits)
    }

    fn check_invariants(&self) {
        if cfg!(test) {
            assert_eq!(
                self.snapshot.transforms.summary().input.len,
                self.snapshot.diff_map_snapshot.len().0,
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
                    let comparison = fold
                        .range
                        .cmp(&next_fold.range, &self.snapshot.diff_map_snapshot.buffer());
                    assert!(comparison.is_le());
                }
            }
        }
    }

    fn sync(
        &mut self,
        diff_map_snapshot: DiffMapSnapshot,
        diff_edits: Vec<DiffEdit>,
    ) -> Vec<FoldEdit> {
        if diff_edits.is_empty() {
            if self.snapshot.diff_map_snapshot.version != diff_map_snapshot.version {
                self.snapshot.version += 1;
            }
            self.snapshot.diff_map_snapshot = diff_map_snapshot;
            Vec::new()
        } else {
            let mut inlay_edits_iter = diff_edits.iter().cloned().peekable();

            let mut new_transforms = SumTree::<Transform>::default();
            let mut cursor = self.snapshot.transforms.cursor::<DiffOffset>(&());
            cursor.seek(&DiffOffset(0), Bias::Right, &());

            while let Some(mut edit) = inlay_edits_iter.next() {
                if let Some(item) = cursor.item() {
                    if !item.is_fold() {
                        new_transforms.update_last(
                            |transform| {
                                if !transform.is_fold() {
                                    transform.summary.add_summary(&item.summary, &());
                                    cursor.next(&());
                                }
                            },
                            &(),
                        );
                    }
                }
                new_transforms.append(cursor.slice(&edit.old.start, Bias::Left, &()), &());
                edit.new.start -= edit.old.start - *cursor.start();
                edit.old.start = *cursor.start();

                cursor.seek(&edit.old.end, Bias::Right, &());
                cursor.next(&());

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
                            cursor.seek(&edit.old.end, Bias::Right, &());
                            cursor.next(&());
                        }
                    } else {
                        break;
                    }
                }

                edit.new.end =
                    DiffOffset(((edit.new.start + edit.old_len()).0 as isize + delta) as usize);

                let anchor = diff_map_snapshot
                    .buffer()
                    .anchor_before(diff_map_snapshot.to_buffer_offset(edit.new.start));
                let mut folds_cursor = self
                    .snapshot
                    .folds
                    .cursor::<FoldRange>(&diff_map_snapshot.buffer());
                folds_cursor.seek(
                    &FoldRange(anchor..Anchor::max()),
                    Bias::Left,
                    &diff_map_snapshot.buffer(),
                );

                let mut folds = iter::from_fn({
                    let diff_map_snapshot = &diff_map_snapshot;
                    move || {
                        let item = folds_cursor.item().map(|fold| {
                            let buffer_start =
                                fold.range.start.to_offset(&diff_map_snapshot.buffer());
                            let buffer_end = fold.range.end.to_offset(&diff_map_snapshot.buffer());
                            (
                                fold.clone(),
                                diff_map_snapshot.make_diff_offset(buffer_start)
                                    ..diff_map_snapshot.make_diff_offset(buffer_end),
                            )
                        });
                        folds_cursor.next(&diff_map_snapshot.buffer());
                        item
                    }
                })
                .peekable();

                while folds
                    .peek()
                    .map_or(false, |(_, fold_range)| fold_range.start < edit.new.end)
                {
                    let (fold, mut fold_range) = folds.next().unwrap();
                    let sum = new_transforms.summary();

                    assert!(fold_range.start.0 >= sum.input.len);

                    while folds.peek().map_or(false, |(next_fold, next_fold_range)| {
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
                        let text_summary = diff_map_snapshot
                            .text_summary_for_range(DiffOffset(sum.input.len)..fold_range.start);
                        push_isomorphic(&mut new_transforms, text_summary);
                    }

                    if fold_range.end > fold_range.start {
                        const ELLIPSIS: &str = "⋯";

                        let fold_id = fold.id;
                        new_transforms.push(
                            Transform {
                                summary: TransformSummary {
                                    output: TextSummary::from(ELLIPSIS),
                                    input: diff_map_snapshot
                                        .text_summary_for_range(fold_range.start..fold_range.end),
                                },
                                placeholder: Some(TransformPlaceholder {
                                    text: ELLIPSIS,
                                    renderer: ChunkRenderer {
                                        render: Arc::new(move |cx| {
                                            (fold.placeholder.render)(
                                                fold_id,
                                                fold.range.0.clone(),
                                                cx,
                                            )
                                        }),
                                        constrain_width: fold.placeholder.constrain_width,
                                    },
                                }),
                            },
                            &(),
                        );
                    }
                }

                let sum = new_transforms.summary();
                if sum.input.len < edit.new.end.0 {
                    let text_summary = diff_map_snapshot
                        .text_summary_for_range(DiffOffset(sum.input.len)..edit.new.end);
                    push_isomorphic(&mut new_transforms, text_summary);
                }
            }

            new_transforms.append(cursor.suffix(&()), &());
            if new_transforms.is_empty() {
                let text_summary = diff_map_snapshot.text_summary();
                push_isomorphic(&mut new_transforms, text_summary);
            }

            drop(cursor);

            let mut fold_edits = Vec::with_capacity(diff_edits.len());
            {
                let mut old_transforms = self
                    .snapshot
                    .transforms
                    .cursor::<(DiffOffset, FoldOffset)>(&());
                let mut new_transforms = new_transforms.cursor::<(DiffOffset, FoldOffset)>(&());

                for mut edit in diff_edits {
                    old_transforms.seek(&edit.old.start, Bias::Left, &());
                    if old_transforms.item().map_or(false, |t| t.is_fold()) {
                        edit.old.start = old_transforms.start().0;
                    }
                    let old_start =
                        old_transforms.start().1 .0 + (edit.old.start - old_transforms.start().0).0;

                    old_transforms.seek_forward(&edit.old.end, Bias::Right, &());
                    if old_transforms.item().map_or(false, |t| t.is_fold()) {
                        old_transforms.next(&());
                        edit.old.end = old_transforms.start().0;
                    }
                    let old_end =
                        old_transforms.start().1 .0 + (edit.old.end - old_transforms.start().0).0;

                    new_transforms.seek(&edit.new.start, Bias::Left, &());
                    if new_transforms.item().map_or(false, |t| t.is_fold()) {
                        edit.new.start = new_transforms.start().0;
                    }
                    let new_start =
                        new_transforms.start().1 .0 + (edit.new.start - new_transforms.start().0).0;

                    new_transforms.seek_forward(&edit.new.end, Bias::Right, &());
                    if new_transforms.item().map_or(false, |t| t.is_fold()) {
                        new_transforms.next(&());
                        edit.new.end = new_transforms.start().0;
                    }
                    let new_end =
                        new_transforms.start().1 .0 + (edit.new.end - new_transforms.start().0).0;

                    fold_edits.push(FoldEdit {
                        old: FoldOffset(old_start)..FoldOffset(old_end),
                        new: FoldOffset(new_start)..FoldOffset(new_end),
                    });
                }

                fold_edits = consolidate_fold_edits(fold_edits);
            }

            self.snapshot.transforms = new_transforms;
            self.snapshot.diff_map_snapshot = diff_map_snapshot;
            self.snapshot.version += 1;
            fold_edits
        }
    }
}

#[derive(Clone)]
pub struct FoldSnapshot {
    transforms: SumTree<Transform>,
    folds: SumTree<Fold>,
    pub diff_map_snapshot: DiffMapSnapshot,
    pub version: usize,
}

impl FoldSnapshot {
    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(FoldOffset(0)..self.len(), false, Highlights::default())
            .map(|c| c.text)
            .collect()
    }

    pub fn buffer(&self) -> &MultiBufferSnapshot {
        self.diff_map_snapshot.buffer()
    }

    #[cfg(test)]
    pub fn fold_count(&self) -> usize {
        self.folds.items(self.diff_map_snapshot.buffer()).len()
    }

    pub fn text_summary_for_range(&self, range: Range<FoldPoint>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self.transforms.cursor::<(FoldPoint, DiffPoint)>(&());
        cursor.seek(&range.start, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let start_in_transform = range.start.0 - cursor.start().0 .0;
            let end_in_transform = cmp::min(range.end, cursor.end(&()).0).0 - cursor.start().0 .0;
            if let Some(placeholder) = transform.placeholder.as_ref() {
                summary = TextSummary::from(
                    &placeholder.text
                        [start_in_transform.column as usize..end_in_transform.column as usize],
                );
            } else {
                let inlay_start = self
                    .diff_map_snapshot
                    .to_offset(DiffPoint(cursor.start().1 .0 + start_in_transform));
                let inlay_end = self
                    .diff_map_snapshot
                    .to_offset(DiffPoint(cursor.start().1 .0 + end_in_transform));
                summary = self
                    .diff_map_snapshot
                    .text_summary_for_range(inlay_start..inlay_end);
            }
        }

        if range.end > cursor.end(&()).0 {
            cursor.next(&());
            summary += &cursor
                .summary::<_, TransformSummary>(&range.end, Bias::Right, &())
                .output;
            if let Some(transform) = cursor.item() {
                let end_in_transform = range.end.0 - cursor.start().0 .0;
                if let Some(placeholder) = transform.placeholder.as_ref() {
                    summary +=
                        TextSummary::from(&placeholder.text[..end_in_transform.column as usize]);
                } else {
                    let inlay_start = self.diff_map_snapshot.to_offset(cursor.start().1);
                    let inlay_end = self
                        .diff_map_snapshot
                        .to_offset(DiffPoint(cursor.start().1 .0 + end_in_transform));
                    summary += self
                        .diff_map_snapshot
                        .text_summary_for_range(inlay_start..inlay_end);
                }
            }
        }

        summary
    }

    pub fn text_summary_for_offset_range(&self, range: Range<FoldOffset>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self.transforms.cursor::<(FoldOffset, DiffOffset)>(&());
        cursor.seek(&range.start, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let start_in_transform = range.start.0 - cursor.start().0 .0;
            let end_in_transform = cmp::min(range.end, cursor.end(&()).0).0 - cursor.start().0 .0;
            if let Some(placeholder) = transform.placeholder.as_ref() {
                summary =
                    TextSummary::from(&placeholder.text[start_in_transform..end_in_transform]);
            } else {
                let inlay_start = DiffOffset(cursor.start().1 .0 + start_in_transform);
                let inlay_end = DiffOffset(cursor.start().1 .0 + end_in_transform);
                summary = self
                    .diff_map_snapshot
                    .text_summary_for_range(inlay_start..inlay_end);
            }
        }

        if range.end > cursor.end(&()).0 {
            cursor.next(&());
            summary += &cursor
                .summary::<_, TransformSummary>(&range.end, Bias::Right, &())
                .output;
            if let Some(transform) = cursor.item() {
                let end_in_transform = range.end.0 - cursor.start().0 .0;
                if let Some(placeholder) = transform.placeholder.as_ref() {
                    summary += TextSummary::from(&placeholder.text[..end_in_transform]);
                } else {
                    let inlay_start = cursor.start().1;
                    let inlay_end = DiffOffset(cursor.start().1 .0 + end_in_transform);
                    summary += self
                        .diff_map_snapshot
                        .text_summary_for_range(inlay_start..inlay_end);
                }
            }
        }

        summary
    }

    pub fn make_fold_point(&self, point: Point, bias: Bias) -> FoldPoint {
        self.to_fold_point(self.diff_map_snapshot.make_diff_point(point), bias)
    }

    pub fn make_fold_offset(&self, buffer_offset: usize, bias: Bias) -> FoldOffset {
        self.to_fold_offset(self.diff_map_snapshot.make_diff_offset(buffer_offset), bias)
    }

    pub fn to_fold_point(&self, point: DiffPoint, bias: Bias) -> FoldPoint {
        let mut cursor = self.transforms.cursor::<(DiffPoint, FoldPoint)>(&());
        cursor.seek(&point, Bias::Right, &());
        if cursor.item().map_or(false, |t| t.is_fold()) {
            if bias == Bias::Left || point == cursor.start().0 {
                cursor.start().1
            } else {
                cursor.end(&()).1
            }
        } else {
            let overshoot = point.0 - cursor.start().0 .0;
            FoldPoint(cmp::min(
                cursor.start().1 .0 + overshoot,
                cursor.end(&()).1 .0,
            ))
        }
    }

    pub fn to_fold_offset(&self, inlay_offset: DiffOffset, bias: Bias) -> FoldOffset {
        let mut cursor = self.transforms.cursor::<(DiffOffset, FoldOffset)>(&());
        cursor.seek(&inlay_offset, Bias::Right, &());
        if cursor.item().map_or(false, |t| t.is_fold()) {
            if bias == Bias::Left || inlay_offset == cursor.start().0 {
                cursor.start().1
            } else {
                cursor.end(&()).1
            }
        } else {
            let overshoot = inlay_offset.0 - cursor.start().0 .0;
            FoldOffset(cmp::min(
                cursor.start().1 .0 + overshoot,
                cursor.end(&()).1 .0,
            ))
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

    pub fn buffer_rows(&self, start_row: u32) -> FoldBufferRows {
        if start_row > self.transforms.summary().output.lines.row {
            panic!("invalid display row {}", start_row);
        }

        let fold_point = FoldPoint::new(start_row, 0);
        let mut cursor = self.transforms.cursor::<(FoldPoint, DiffPoint)>(&());
        cursor.seek(&fold_point, Bias::Left, &());

        let overshoot = fold_point.0 - cursor.start().0 .0;
        let diff_point = DiffPoint(cursor.start().1 .0 + overshoot);
        let input_buffer_rows = self.diff_map_snapshot.buffer_rows(diff_point.0.row);

        FoldBufferRows {
            fold_point,
            input_buffer_rows,
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
        let buffer = self.buffer();
        let range = range.start.to_offset(buffer)..range.end.to_offset(buffer);
        let mut folds = intersecting_folds(&self.diff_map_snapshot, &self.folds, range, false);
        iter::from_fn(move || {
            let item = folds.item();
            folds.next(buffer);
            item
        })
    }

    pub fn intersects_fold<T>(&self, offset: T) -> bool
    where
        T: ToOffset,
    {
        let buffer_offset = offset.to_offset(self.buffer());
        let diff_offset = self.diff_map_snapshot.make_diff_offset(buffer_offset);
        let mut cursor = self.transforms.cursor::<DiffOffset>(&());
        cursor.seek(&diff_offset, Bias::Right, &());
        cursor.item().map_or(false, |t| t.placeholder.is_some())
    }

    pub fn is_line_folded(&self, buffer_row: MultiBufferRow) -> bool {
        let mut diff_point = self
            .diff_map_snapshot
            .make_diff_point(Point::new(buffer_row.0, 0));
        let mut cursor = self.transforms.cursor::<DiffPoint>(&());
        cursor.seek(&diff_point, Bias::Right, &());
        loop {
            match cursor.item() {
                Some(transform) => {
                    let buffer_point = self.diff_map_snapshot.to_buffer_point(diff_point);
                    if buffer_point.row != buffer_row.0 {
                        return false;
                    } else if transform.placeholder.is_some() {
                        return true;
                    }
                }
                None => return false,
            }

            if cursor.end(&()).0.row == diff_point.0.row {
                cursor.next(&());
            } else {
                diff_point.0 += Point::new(1, 0);
                cursor.seek(&diff_point, Bias::Right, &());
            }
        }
    }

    pub(crate) fn chunks<'a>(
        &'a self,
        range: Range<FoldOffset>,
        language_aware: bool,
        highlights: Highlights<'a>,
    ) -> FoldChunks<'a> {
        let mut transform_cursor = self.transforms.cursor::<(FoldOffset, DiffOffset)>(&());
        transform_cursor.seek(&range.start, Bias::Right, &());

        let inlay_start = {
            let overshoot = range.start.0 - transform_cursor.start().0 .0;
            transform_cursor.start().1 + DiffOffset(overshoot)
        };

        let transform_end = transform_cursor.end(&());

        let inlay_end = if transform_cursor
            .item()
            .map_or(true, |transform| transform.is_fold())
        {
            inlay_start
        } else if range.end < transform_end.0 {
            let overshoot = range.end.0 - transform_cursor.start().0 .0;
            transform_cursor.start().1 + DiffOffset(overshoot)
        } else {
            transform_end.1
        };

        FoldChunks {
            transform_cursor,
            diff_map_chunks: self.diff_map_snapshot.chunks(
                inlay_start..inlay_end,
                language_aware,
                highlights,
            ),
            diff_chunk: None,
            diff_offset: inlay_start,
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
        let mut cursor = self.transforms.cursor::<(FoldPoint, DiffPoint)>(&());
        cursor.seek(&point, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let transform_start = cursor.start().0 .0;
            if transform.placeholder.is_some() {
                if point.0 == transform_start || matches!(bias, Bias::Left) {
                    FoldPoint(transform_start)
                } else {
                    FoldPoint(cursor.end(&()).0 .0)
                }
            } else {
                let overshoot = point.0 - transform_start;
                let diff_point = DiffPoint(cursor.start().1 .0 + overshoot);
                let clipped_diff_point = self.diff_map_snapshot.clip_point(diff_point, bias);
                FoldPoint(cursor.start().0 .0 + (clipped_diff_point - cursor.start().1).0)
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
                last.summary.input += summary.clone();
                last.summary.output += summary.clone();
                did_merge = true;
            }
        },
        &(),
    );
    if !did_merge {
        transforms.push(
            Transform {
                summary: TransformSummary {
                    input: summary.clone(),
                    output: summary,
                },
                placeholder: None,
            },
            &(),
        )
    }
}

fn intersecting_folds<'a>(
    diff_map_snapshot: &'a DiffMapSnapshot,
    folds: &'a SumTree<Fold>,
    range: Range<usize>,
    inclusive: bool,
) -> FilterCursor<'a, impl 'a + FnMut(&FoldSummary) -> bool, Fold, usize> {
    let buffer = diff_map_snapshot.buffer();
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
    cursor.next(buffer);
    cursor
}

fn consolidate_inlay_edits(mut edits: Vec<DiffEdit>) -> Vec<DiffEdit> {
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct FoldId(usize);

impl From<FoldId> for ElementId {
    fn from(val: FoldId) -> Self {
        ElementId::Integer(val.0)
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

impl<'a> sum_tree::SeekTarget<'a, FoldSummary, FoldRange> for FoldRange {
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
pub struct FoldBufferRows<'a> {
    cursor: Cursor<'a, Transform, (FoldPoint, DiffPoint)>,
    input_buffer_rows: DiffMapBufferRows<'a>,
    fold_point: FoldPoint,
}

impl<'a> FoldBufferRows<'a> {
    pub(crate) fn seek(&mut self, row: u32) {
        let fold_point = FoldPoint::new(row, 0);
        self.cursor.seek(&fold_point, Bias::Left, &());
        let overshoot = fold_point.0 - self.cursor.start().0 .0;
        let diff_point = DiffPoint(self.cursor.start().1 .0 + overshoot);
        self.input_buffer_rows.seek(diff_point.0.row);
        self.fold_point = fold_point;
    }
}

impl<'a> Iterator for FoldBufferRows<'a> {
    type Item = Option<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut traversed_fold = false;
        while self.fold_point > self.cursor.end(&()).0 {
            self.cursor.next(&());
            traversed_fold = true;
            if self.cursor.item().is_none() {
                break;
            }
        }

        if self.cursor.item().is_some() {
            if traversed_fold {
                self.input_buffer_rows.seek(self.cursor.start().1 .0.row);
                self.input_buffer_rows.next();
            }
            *self.fold_point.row_mut() += 1;
            self.input_buffer_rows.next().map(|info| info.row)
        } else {
            None
        }
    }
}

pub struct FoldChunks<'a> {
    transform_cursor: Cursor<'a, Transform, (FoldOffset, DiffOffset)>,
    diff_map_chunks: DiffMapChunks<'a>,
    diff_chunk: Option<(DiffOffset, Chunk<'a>)>,
    diff_offset: DiffOffset,
    output_offset: FoldOffset,
    max_output_offset: FoldOffset,
}

impl<'a> FoldChunks<'a> {
    pub(crate) fn seek(&mut self, range: Range<FoldOffset>) {
        self.transform_cursor.seek(&range.start, Bias::Right, &());

        let diff_start = {
            let overshoot = range.start.0 - self.transform_cursor.start().0 .0;
            self.transform_cursor.start().1 + DiffOffset(overshoot)
        };

        let transform_end = self.transform_cursor.end(&());

        let diff_end = if self
            .transform_cursor
            .item()
            .map_or(true, |transform| transform.is_fold())
        {
            diff_start
        } else if range.end < transform_end.0 {
            let overshoot = range.end.0 - self.transform_cursor.start().0 .0;
            self.transform_cursor.start().1 + DiffOffset(overshoot)
        } else {
            transform_end.1
        };

        self.diff_map_chunks.seek(diff_start..diff_end);
        self.diff_chunk = None;
        self.diff_offset = diff_start;
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
            self.diff_chunk.take();
            self.diff_offset += DiffOffset(transform.summary.input.len);

            while self.diff_offset >= self.transform_cursor.end(&()).1
                && self.transform_cursor.item().is_some()
            {
                self.transform_cursor.next(&());
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
        if self.diff_offset == self.transform_cursor.start().1
            && self.diff_map_chunks.offset() != self.diff_offset
        {
            let transform_start = self.transform_cursor.start();
            let transform_end = self.transform_cursor.end(&());
            let inlay_end = if self.max_output_offset < transform_end.0 {
                let overshoot = self.max_output_offset.0 - transform_start.0 .0;
                transform_start.1 + DiffOffset(overshoot)
            } else {
                transform_end.1
            };

            self.diff_map_chunks.seek(self.diff_offset..inlay_end);
        }

        // Retrieve a chunk from the current location in the buffer.
        if self.diff_chunk.is_none() {
            let chunk_offset = self.diff_map_chunks.offset();
            self.diff_chunk = self
                .diff_map_chunks
                .next()
                .map(|chunk| (chunk_offset, chunk));
        }

        // Otherwise, take a chunk from the buffer's text.
        if let Some((buffer_chunk_start, mut chunk)) = self.diff_chunk.clone() {
            let buffer_chunk_end = buffer_chunk_start + DiffOffset(chunk.text.len());
            let transform_end = self.transform_cursor.end(&()).1;
            let chunk_end = buffer_chunk_end.min(transform_end);

            chunk.text = &chunk.text
                [(self.diff_offset - buffer_chunk_start).0..(chunk_end - buffer_chunk_start).0];

            if chunk_end == transform_end {
                self.transform_cursor.next(&());
            } else if chunk_end == buffer_chunk_end {
                self.diff_chunk.take();
            }

            self.diff_offset = chunk_end;
            self.output_offset.0 += chunk.text.len();
            return Some(chunk);
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
            .cursor::<(FoldOffset, TransformSummary)>(&());
        cursor.seek(&self, Bias::Right, &());
        let overshoot = if cursor.item().map_or(true, |t| t.is_fold()) {
            Point::new(0, (self.0 - cursor.start().0 .0) as u32)
        } else {
            let inlay_offset = cursor.start().1.input.len + self.0 - cursor.start().0 .0;
            let inlay_point = snapshot
                .diff_map_snapshot
                .to_point(DiffOffset(inlay_offset));
            inlay_point.0 - cursor.start().1.input.lines
        };
        FoldPoint(cursor.start().1.output.lines + overshoot)
    }

    #[cfg(test)]
    pub fn to_diff_offset(self, snapshot: &FoldSnapshot) -> DiffOffset {
        let mut cursor = snapshot.transforms.cursor::<(FoldOffset, DiffOffset)>(&());
        cursor.seek(&self, Bias::Right, &());
        let overshoot = self.0 - cursor.start().0 .0;
        DiffOffset(cursor.start().1 .0 + overshoot)
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

impl<'a> sum_tree::Dimension<'a, TransformSummary> for DiffPoint {
    fn zero(_cx: &()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.input.lines;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for DiffOffset {
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
    use crate::{
        display_map::{diff_map::DiffMap, inlay_map::InlayMap},
        MultiBuffer, ToPoint,
    };
    use collections::HashSet;
    use rand::prelude::*;
    use settings::SettingsStore;
    use std::{env, mem};
    use util::test::sample_text;
    use util::RandomCharIter;
    use Bias::{Left, Right};

    #[gpui::test]
    fn test_basic_folds(cx: &mut gpui::AppContext) {
        init_test(cx);
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (diff_map, diff_snapshot) = DiffMap::new(inlay_snapshot, buffer.clone(), cx);
        let mut map = FoldMap::new(diff_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(diff_snapshot, vec![]);
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
        let (diff_snapshot, diff_edits) = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(inlay_snapshot, inlay_edits, cx)
        });
        let (snapshot3, edits) = map.read(diff_snapshot, diff_edits);
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
        let (diff_snapshot, diff_edits) = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(inlay_snapshot, inlay_edits, cx)
        });
        let (snapshot4, _) = map.read(diff_snapshot.clone(), diff_edits);
        assert_eq!(snapshot4.text(), "123a⋯c123456eee");

        let (mut writer, _, _) = map.write(diff_snapshot.clone(), vec![]);
        writer.unfold_intersecting(Some(Point::new(0, 4)..Point::new(0, 4)), false);
        let (snapshot5, _) = map.read(diff_snapshot.clone(), vec![]);
        assert_eq!(snapshot5.text(), "123a⋯c123456eee");

        let (mut writer, _, _) = map.write(diff_snapshot.clone(), vec![]);
        writer.unfold_intersecting(Some(Point::new(0, 4)..Point::new(0, 4)), true);
        let (snapshot6, _) = map.read(diff_snapshot, vec![]);
        assert_eq!(snapshot6.text(), "123aaaaa\nbbbbbb\nccc123456eee");
    }

    #[gpui::test]
    fn test_adjacent_folds(cx: &mut gpui::AppContext) {
        init_test(cx);
        let buffer = MultiBuffer::build_simple("abcdefghijkl", cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (diff_map, diff_snapshot) = DiffMap::new(inlay_snapshot, buffer.clone(), cx);

        {
            let mut map = FoldMap::new(diff_snapshot.clone()).0;

            let (mut writer, _, _) = map.write(diff_snapshot.clone(), vec![]);
            writer.fold(vec![(5..8, FoldPlaceholder::test())]);
            let (snapshot, _) = map.read(diff_snapshot.clone(), vec![]);
            assert_eq!(snapshot.text(), "abcde⋯ijkl");

            // Create an fold adjacent to the start of the first fold.
            let (mut writer, _, _) = map.write(diff_snapshot.clone(), vec![]);
            writer.fold(vec![
                (0..1, FoldPlaceholder::test()),
                (2..5, FoldPlaceholder::test()),
            ]);
            let (snapshot, _) = map.read(diff_snapshot.clone(), vec![]);
            assert_eq!(snapshot.text(), "⋯b⋯ijkl");

            // Create an fold adjacent to the end of the first fold.
            let (mut writer, _, _) = map.write(diff_snapshot.clone(), vec![]);
            writer.fold(vec![
                (11..11, FoldPlaceholder::test()),
                (8..10, FoldPlaceholder::test()),
            ]);
            let (snapshot, _) = map.read(diff_snapshot.clone(), vec![]);
            assert_eq!(snapshot.text(), "⋯b⋯kl");
        }

        {
            let mut map = FoldMap::new(diff_snapshot.clone()).0;

            // Create two adjacent folds.
            let (mut writer, _, _) = map.write(diff_snapshot.clone(), vec![]);
            writer.fold(vec![
                (0..2, FoldPlaceholder::test()),
                (2..5, FoldPlaceholder::test()),
            ]);
            let (snapshot, _) = map.read(diff_snapshot, vec![]);
            assert_eq!(snapshot.text(), "⋯fghijkl");

            // Edit within one of the folds.
            let buffer_snapshot = buffer.update(cx, |buffer, cx| {
                buffer.edit([(0..1, "12345")], None, cx);
                buffer.snapshot(cx)
            });
            let (inlay_snapshot, inlay_edits) =
                inlay_map.sync(buffer_snapshot, subscription.consume().into_inner());
            let (diff_snapshot, diff_edits) = diff_map.update(cx, |diff_map, cx| {
                diff_map.sync(inlay_snapshot, inlay_edits, cx)
            });
            let (snapshot, _) = map.read(diff_snapshot, diff_edits);
            assert_eq!(snapshot.text(), "12345⋯fghijkl");
        }
    }

    #[gpui::test]
    fn test_overlapping_folds(cx: &mut gpui::AppContext) {
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, diff_snapshot) = DiffMap::new(inlay_snapshot, buffer.clone(), cx);
        let mut map = FoldMap::new(diff_snapshot.clone()).0;
        let (mut writer, _, _) = map.write(diff_snapshot.clone(), vec![]);
        writer.fold(vec![
            (Point::new(0, 2)..Point::new(2, 2), FoldPlaceholder::test()),
            (Point::new(0, 4)..Point::new(1, 0), FoldPlaceholder::test()),
            (Point::new(1, 2)..Point::new(3, 2), FoldPlaceholder::test()),
            (Point::new(3, 1)..Point::new(4, 1), FoldPlaceholder::test()),
        ]);
        let (snapshot, _) = map.read(diff_snapshot, vec![]);
        assert_eq!(snapshot.text(), "aa⋯eeeee");
    }

    #[gpui::test]
    fn test_merging_folds_via_edit(cx: &mut gpui::AppContext) {
        init_test(cx);
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (diff_map, diff_snapshot) = DiffMap::new(inlay_snapshot, buffer.clone(), cx);
        let mut map = FoldMap::new(diff_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(diff_snapshot.clone(), vec![]);
        writer.fold(vec![
            (Point::new(0, 2)..Point::new(2, 2), FoldPlaceholder::test()),
            (Point::new(3, 1)..Point::new(4, 1), FoldPlaceholder::test()),
        ]);
        let (snapshot, _) = map.read(diff_snapshot.clone(), vec![]);
        assert_eq!(snapshot.text(), "aa⋯cccc\nd⋯eeeee");

        let buffer_snapshot = buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 2)..Point::new(3, 1), "")], None, cx);
            buffer.snapshot(cx)
        });
        let (inlay_snapshot, inlay_edits) =
            inlay_map.sync(buffer_snapshot, subscription.consume().into_inner());
        let (diff_snapshot, diff_edits) = diff_map.update(cx, |diff_map, cx| {
            diff_map.sync(inlay_snapshot, inlay_edits, cx)
        });
        let (snapshot, _) = map.read(diff_snapshot, diff_edits);
        assert_eq!(snapshot.text(), "aa⋯eeeee");
    }

    #[gpui::test]
    fn test_folds_in_range(cx: &mut gpui::AppContext) {
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, diff_snapshot) = DiffMap::new(inlay_snapshot, buffer.clone(), cx);
        let mut map = FoldMap::new(diff_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(diff_snapshot.clone(), vec![]);
        writer.fold(vec![
            (Point::new(0, 2)..Point::new(2, 2), FoldPlaceholder::test()),
            (Point::new(0, 4)..Point::new(1, 0), FoldPlaceholder::test()),
            (Point::new(1, 2)..Point::new(3, 2), FoldPlaceholder::test()),
            (Point::new(3, 1)..Point::new(4, 1), FoldPlaceholder::test()),
        ]);
        let (snapshot, _) = map.read(diff_snapshot.clone(), vec![]);
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
    fn test_random_folds(cx: &mut gpui::AppContext, mut rng: StdRng) {
        init_test(cx);
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let len = rng.gen_range(0..10);
        let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
        let buffer = if rng.gen() {
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };
        let mut buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (mut inlay_map, mut inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (diff_map, mut diff_snapshot) =
            DiffMap::new(inlay_snapshot.clone(), buffer.clone(), cx);
        let mut map = FoldMap::new(diff_snapshot.clone()).0;

        let (mut initial_snapshot, _) = map.read(diff_snapshot.clone(), vec![]);
        let mut snapshot_edits = Vec::new();

        let mut next_inlay_id = 0;
        for _ in 0..operations {
            log::info!("text: {:?}", buffer_snapshot.text());

            let mut inlay_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=39 => {
                    snapshot_edits.extend(map.randomly_mutate(&mut rng));
                }
                40..=59 => {
                    (inlay_snapshot, inlay_edits) =
                        inlay_map.randomly_mutate(&mut next_inlay_id, &mut rng);
                }
                _ => {
                    (inlay_snapshot, inlay_edits) = buffer.update(cx, |buffer, cx| {
                        let subscription = buffer.subscribe();
                        let edit_count = rng.gen_range(1..=5);
                        buffer.randomly_mutate(&mut rng, edit_count, cx);
                        buffer_snapshot = buffer.snapshot(cx);
                        let buffer_edits = subscription.consume().into_inner();
                        log::info!("editing {:?}", buffer_edits);
                        inlay_map.sync(buffer_snapshot.clone(), buffer_edits)
                    })
                }
            };

            log::info!("inlay text {:?}", inlay_snapshot.text());
            let (new_diff_snapshot, diff_edits) = diff_map.update(cx, |diff_map, cx| {
                diff_map.sync(inlay_snapshot.clone(), inlay_edits, cx)
            });
            diff_snapshot = new_diff_snapshot;

            let (snapshot, edits) = map.read(diff_snapshot.clone(), diff_edits);
            snapshot_edits.push((snapshot.clone(), edits));

            let mut expected_text: String = diff_snapshot.text().to_string();
            for fold_range in map.merged_folds().into_iter().rev() {
                let fold_inlay_start = diff_snapshot.make_diff_offset(fold_range.start);
                let fold_inlay_end = diff_snapshot.make_diff_offset(fold_range.end);
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
                        .buffer_rows(prev_row)
                        .take((1 + fold_start - prev_row) as usize),
                );
                prev_row = 1 + fold_end;
            }
            expected_buffer_rows.extend(inlay_snapshot.buffer_rows(prev_row));

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
                let diff_point = fold_point.to_diff_point(&snapshot);
                let diff_offset = fold_offset.to_diff_offset(&snapshot);
                assert_eq!(
                    snapshot.to_fold_point(diff_point, Right),
                    fold_point,
                    "{:?} -> fold point",
                    diff_point,
                );
                assert_eq!(
                    diff_snapshot.to_offset(diff_point),
                    diff_offset,
                    "diff_map_snapshot.to_offset({:?})",
                    diff_point,
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
                    snapshot.buffer_rows(fold_row).collect::<Vec<_>>(),
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
                let end =
                    snapshot.clip_offset(FoldOffset(rng.gen_range(0..=snapshot.len().0)), Right);
                let start = snapshot.clip_offset(FoldOffset(rng.gen_range(0..=end.0)), Left);
                let bytes = start..end;
                assert_eq!(
                    snapshot.text_summary_for_offset_range(bytes.clone()),
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
    fn test_buffer_rows(cx: &mut gpui::AppContext) {
        let text = sample_text(6, 6, 'a') + "\n";
        let buffer = MultiBuffer::build_simple(&text, cx);

        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, diff_snapshot) = DiffMap::new(inlay_snapshot.clone(), buffer.clone(), cx);
        let mut map = FoldMap::new(diff_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(diff_snapshot.clone(), vec![]);
        writer.fold(vec![
            (Point::new(0, 2)..Point::new(2, 2), FoldPlaceholder::test()),
            (Point::new(3, 1)..Point::new(4, 1), FoldPlaceholder::test()),
        ]);

        let (snapshot, _) = map.read(diff_snapshot, vec![]);
        assert_eq!(snapshot.text(), "aa⋯cccc\nd⋯eeeee\nffffff\n");
        assert_eq!(
            snapshot.buffer_rows(0).collect::<Vec<_>>(),
            [Some(0), Some(3), Some(5), Some(6)]
        );
        assert_eq!(snapshot.buffer_rows(3).collect::<Vec<_>>(), [Some(6)]);
    }

    fn init_test(cx: &mut gpui::AppContext) {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
    }

    impl FoldMap {
        fn merged_folds(&self) -> Vec<Range<usize>> {
            let diff_map_snapshot = self.snapshot.diff_map_snapshot.clone();
            let buffer = diff_map_snapshot.buffer();
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
                    let diff_map_snapshot = self.snapshot.diff_map_snapshot.clone();
                    let buffer = diff_map_snapshot.buffer();
                    let mut to_unfold = Vec::new();
                    for _ in 0..rng.gen_range(1..=3) {
                        let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                        let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                        to_unfold.push(start..end);
                    }
                    let inclusive = rng.gen();
                    log::info!("unfolding {:?} (inclusive: {})", to_unfold, inclusive);
                    let (mut writer, snapshot, edits) = self.write(diff_map_snapshot, vec![]);
                    snapshot_edits.push((snapshot, edits));
                    let (snapshot, edits) = writer.unfold_intersecting(to_unfold, inclusive);
                    snapshot_edits.push((snapshot, edits));
                }
                _ => {
                    let diff_map_snapshot = self.snapshot.diff_map_snapshot.clone();
                    let buffer = diff_map_snapshot.buffer();
                    let mut to_fold = Vec::new();
                    for _ in 0..rng.gen_range(1..=2) {
                        let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                        let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                        to_fold.push((start..end, FoldPlaceholder::test()));
                    }
                    log::info!("folding {:?}", to_fold);
                    let (mut writer, snapshot, edits) = self.write(diff_map_snapshot, vec![]);
                    snapshot_edits.push((snapshot, edits));
                    let (snapshot, edits) = writer.fold(to_fold);
                    snapshot_edits.push((snapshot, edits));
                }
            }
            snapshot_edits
        }
    }
}

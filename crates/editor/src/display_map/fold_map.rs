use super::TextHighlights;
use crate::{
    multi_buffer::MultiBufferRows, Anchor, AnchorRangeExt, MultiBufferChunks, MultiBufferSnapshot,
    ToOffset,
};
use collections::BTreeMap;
use gpui::fonts::HighlightStyle;
use language::{Chunk, Edit, Point, PointUtf16, TextSummary};
use parking_lot::Mutex;
use std::{
    any::TypeId,
    cmp::{self, Ordering},
    iter::{self, Peekable},
    ops::{Range, Sub},
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
    vec,
};
use sum_tree::{Bias, Cursor, FilterCursor, SumTree};

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct FoldPoint(pub super::Point);

impl FoldPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(super::Point::new(row, column))
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

    pub fn column_mut(&mut self) -> &mut u32 {
        &mut self.0.column
    }

    pub fn to_buffer_point(&self, snapshot: &FoldSnapshot) -> Point {
        let mut cursor = snapshot.transforms.cursor::<(FoldPoint, Point)>();
        cursor.seek(self, Bias::Right, &());
        let overshoot = self.0 - cursor.start().0 .0;
        cursor.start().1 + overshoot
    }

    pub fn to_buffer_offset(&self, snapshot: &FoldSnapshot) -> usize {
        let mut cursor = snapshot.transforms.cursor::<(FoldPoint, Point)>();
        cursor.seek(self, Bias::Right, &());
        let overshoot = self.0 - cursor.start().0 .0;
        snapshot
            .buffer_snapshot
            .point_to_offset(cursor.start().1 + overshoot)
    }

    pub fn to_offset(&self, snapshot: &FoldSnapshot) -> FoldOffset {
        let mut cursor = snapshot
            .transforms
            .cursor::<(FoldPoint, TransformSummary)>();
        cursor.seek(self, Bias::Right, &());
        let overshoot = self.0 - cursor.start().1.output.lines;
        let mut offset = cursor.start().1.output.bytes;
        if !overshoot.is_zero() {
            let transform = cursor.item().expect("display point out of range");
            assert!(transform.output_text.is_none());
            let end_buffer_offset = snapshot
                .buffer_snapshot
                .point_to_offset(cursor.start().1.input.lines + overshoot);
            offset += end_buffer_offset - cursor.start().1.input.bytes;
        }
        FoldOffset(offset)
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for FoldPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.output.lines;
    }
}

pub struct FoldMapWriter<'a>(&'a mut FoldMap);

impl<'a> FoldMapWriter<'a> {
    pub fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
    ) -> (FoldSnapshot, Vec<FoldEdit>) {
        let mut edits = Vec::new();
        let mut folds = Vec::new();
        let buffer = self.0.buffer.lock().clone();
        for range in ranges.into_iter() {
            let range = range.start.to_offset(&buffer)..range.end.to_offset(&buffer);

            // Ignore any empty ranges.
            if range.start == range.end {
                continue;
            }

            // For now, ignore any ranges that span an excerpt boundary.
            let fold = Fold(buffer.anchor_after(range.start)..buffer.anchor_before(range.end));
            if fold.0.start.excerpt_id() != fold.0.end.excerpt_id() {
                continue;
            }

            folds.push(fold);
            edits.push(text::Edit {
                old: range.clone(),
                new: range,
            });
        }

        folds.sort_unstable_by(|a, b| sum_tree::SeekTarget::cmp(a, b, &buffer));

        self.0.folds = {
            let mut new_tree = SumTree::new();
            let mut cursor = self.0.folds.cursor::<Fold>();
            for fold in folds {
                new_tree.push_tree(cursor.slice(&fold, Bias::Right, &buffer), &buffer);
                new_tree.push(fold, &buffer);
            }
            new_tree.push_tree(cursor.suffix(&buffer), &buffer);
            new_tree
        };

        consolidate_buffer_edits(&mut edits);
        let edits = self.0.sync(buffer.clone(), edits);
        let snapshot = FoldSnapshot {
            transforms: self.0.transforms.lock().clone(),
            folds: self.0.folds.clone(),
            buffer_snapshot: buffer,
            version: self.0.version.load(SeqCst),
        };
        (snapshot, edits)
    }

    pub fn unfold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        inclusive: bool,
    ) -> (FoldSnapshot, Vec<FoldEdit>) {
        let mut edits = Vec::new();
        let mut fold_ixs_to_delete = Vec::new();
        let buffer = self.0.buffer.lock().clone();
        for range in ranges.into_iter() {
            // Remove intersecting folds and add their ranges to edits that are passed to sync.
            let mut folds_cursor = intersecting_folds(&buffer, &self.0.folds, range, inclusive);
            while let Some(fold) = folds_cursor.item() {
                let offset_range = fold.0.start.to_offset(&buffer)..fold.0.end.to_offset(&buffer);
                if offset_range.end > offset_range.start {
                    edits.push(text::Edit {
                        old: offset_range.clone(),
                        new: offset_range,
                    });
                }
                fold_ixs_to_delete.push(*folds_cursor.start());
                folds_cursor.next(&buffer);
            }
        }

        fold_ixs_to_delete.sort_unstable();
        fold_ixs_to_delete.dedup();

        self.0.folds = {
            let mut cursor = self.0.folds.cursor::<usize>();
            let mut folds = SumTree::new();
            for fold_ix in fold_ixs_to_delete {
                folds.push_tree(cursor.slice(&fold_ix, Bias::Right, &buffer), &buffer);
                cursor.next(&buffer);
            }
            folds.push_tree(cursor.suffix(&buffer), &buffer);
            folds
        };

        consolidate_buffer_edits(&mut edits);
        let edits = self.0.sync(buffer.clone(), edits);
        let snapshot = FoldSnapshot {
            transforms: self.0.transforms.lock().clone(),
            folds: self.0.folds.clone(),
            buffer_snapshot: buffer,
            version: self.0.version.load(SeqCst),
        };
        (snapshot, edits)
    }
}

pub struct FoldMap {
    buffer: Mutex<MultiBufferSnapshot>,
    transforms: Mutex<SumTree<Transform>>,
    folds: SumTree<Fold>,
    version: AtomicUsize,
}

impl FoldMap {
    pub fn new(buffer: MultiBufferSnapshot) -> (Self, FoldSnapshot) {
        let this = Self {
            buffer: Mutex::new(buffer.clone()),
            folds: Default::default(),
            transforms: Mutex::new(SumTree::from_item(
                Transform {
                    summary: TransformSummary {
                        input: buffer.text_summary(),
                        output: buffer.text_summary(),
                    },
                    output_text: None,
                },
                &(),
            )),
            version: Default::default(),
        };

        let snapshot = FoldSnapshot {
            transforms: this.transforms.lock().clone(),
            folds: this.folds.clone(),
            buffer_snapshot: this.buffer.lock().clone(),
            version: this.version.load(SeqCst),
        };
        (this, snapshot)
    }

    pub fn read(
        &self,
        buffer: MultiBufferSnapshot,
        edits: Vec<Edit<usize>>,
    ) -> (FoldSnapshot, Vec<FoldEdit>) {
        let edits = self.sync(buffer, edits);
        self.check_invariants();
        let snapshot = FoldSnapshot {
            transforms: self.transforms.lock().clone(),
            folds: self.folds.clone(),
            buffer_snapshot: self.buffer.lock().clone(),
            version: self.version.load(SeqCst),
        };
        (snapshot, edits)
    }

    pub fn write(
        &mut self,
        buffer: MultiBufferSnapshot,
        edits: Vec<Edit<usize>>,
    ) -> (FoldMapWriter, FoldSnapshot, Vec<FoldEdit>) {
        let (snapshot, edits) = self.read(buffer, edits);
        (FoldMapWriter(self), snapshot, edits)
    }

    fn check_invariants(&self) {
        if cfg!(test) {
            assert_eq!(
                self.transforms.lock().summary().input.bytes,
                self.buffer.lock().len(),
                "transform tree does not match buffer's length"
            );

            let mut folds = self.folds.iter().peekable();
            while let Some(fold) = folds.next() {
                if let Some(next_fold) = folds.peek() {
                    let comparison = fold.0.cmp(&next_fold.0, &self.buffer.lock());
                    assert!(comparison.is_le());
                }
            }
        }
    }

    fn sync(
        &self,
        new_buffer: MultiBufferSnapshot,
        buffer_edits: Vec<text::Edit<usize>>,
    ) -> Vec<FoldEdit> {
        if buffer_edits.is_empty() {
            let mut buffer = self.buffer.lock();
            if buffer.edit_count() != new_buffer.edit_count()
                || buffer.parse_count() != new_buffer.parse_count()
                || buffer.diagnostics_update_count() != new_buffer.diagnostics_update_count()
                || buffer.trailing_excerpt_update_count()
                    != new_buffer.trailing_excerpt_update_count()
            {
                self.version.fetch_add(1, SeqCst);
            }
            *buffer = new_buffer;
            Vec::new()
        } else {
            let mut buffer_edits_iter = buffer_edits.iter().cloned().peekable();

            let mut new_transforms = SumTree::new();
            let mut transforms = self.transforms.lock();
            let mut cursor = transforms.cursor::<usize>();
            cursor.seek(&0, Bias::Right, &());

            while let Some(mut edit) = buffer_edits_iter.next() {
                new_transforms.push_tree(cursor.slice(&edit.old.start, Bias::Left, &()), &());
                edit.new.start -= edit.old.start - cursor.start();
                edit.old.start = *cursor.start();

                cursor.seek(&edit.old.end, Bias::Right, &());
                cursor.next(&());

                let mut delta = edit.new.len() as isize - edit.old.len() as isize;
                loop {
                    edit.old.end = *cursor.start();

                    if let Some(next_edit) = buffer_edits_iter.peek() {
                        if next_edit.old.start > edit.old.end {
                            break;
                        }

                        let next_edit = buffer_edits_iter.next().unwrap();
                        delta += next_edit.new.len() as isize - next_edit.old.len() as isize;

                        if next_edit.old.end >= edit.old.end {
                            edit.old.end = next_edit.old.end;
                            cursor.seek(&edit.old.end, Bias::Right, &());
                            cursor.next(&());
                        }
                    } else {
                        break;
                    }
                }

                edit.new.end = ((edit.new.start + edit.old.len()) as isize + delta) as usize;

                let anchor = new_buffer.anchor_before(edit.new.start);
                let mut folds_cursor = self.folds.cursor::<Fold>();
                folds_cursor.seek(&Fold(anchor..Anchor::max()), Bias::Left, &new_buffer);

                let mut folds = iter::from_fn({
                    let buffer = &new_buffer;
                    move || {
                        let item = folds_cursor
                            .item()
                            .map(|f| f.0.start.to_offset(buffer)..f.0.end.to_offset(buffer));
                        folds_cursor.next(buffer);
                        item
                    }
                })
                .peekable();

                while folds.peek().map_or(false, |fold| fold.start < edit.new.end) {
                    let mut fold = folds.next().unwrap();
                    let sum = new_transforms.summary();

                    assert!(fold.start >= sum.input.bytes);

                    while folds
                        .peek()
                        .map_or(false, |next_fold| next_fold.start <= fold.end)
                    {
                        let next_fold = folds.next().unwrap();
                        if next_fold.end > fold.end {
                            fold.end = next_fold.end;
                        }
                    }

                    if fold.start > sum.input.bytes {
                        let text_summary = new_buffer
                            .text_summary_for_range::<TextSummary, _>(sum.input.bytes..fold.start);
                        new_transforms.push(
                            Transform {
                                summary: TransformSummary {
                                    output: text_summary.clone(),
                                    input: text_summary,
                                },
                                output_text: None,
                            },
                            &(),
                        );
                    }

                    if fold.end > fold.start {
                        let output_text = "…";
                        new_transforms.push(
                            Transform {
                                summary: TransformSummary {
                                    output: TextSummary::from(output_text),
                                    input: new_buffer.text_summary_for_range(fold.start..fold.end),
                                },
                                output_text: Some(output_text),
                            },
                            &(),
                        );
                    }
                }

                let sum = new_transforms.summary();
                if sum.input.bytes < edit.new.end {
                    let text_summary = new_buffer
                        .text_summary_for_range::<TextSummary, _>(sum.input.bytes..edit.new.end);
                    new_transforms.push(
                        Transform {
                            summary: TransformSummary {
                                output: text_summary.clone(),
                                input: text_summary,
                            },
                            output_text: None,
                        },
                        &(),
                    );
                }
            }

            new_transforms.push_tree(cursor.suffix(&()), &());
            if new_transforms.is_empty() {
                let text_summary = new_buffer.text_summary();
                new_transforms.push(
                    Transform {
                        summary: TransformSummary {
                            output: text_summary.clone(),
                            input: text_summary,
                        },
                        output_text: None,
                    },
                    &(),
                );
            }

            drop(cursor);

            let mut fold_edits = Vec::with_capacity(buffer_edits.len());
            {
                let mut old_transforms = transforms.cursor::<(usize, FoldOffset)>();
                let mut new_transforms = new_transforms.cursor::<(usize, FoldOffset)>();

                for mut edit in buffer_edits {
                    old_transforms.seek(&edit.old.start, Bias::Left, &());
                    if old_transforms.item().map_or(false, |t| t.is_fold()) {
                        edit.old.start = old_transforms.start().0;
                    }
                    let old_start =
                        old_transforms.start().1 .0 + (edit.old.start - old_transforms.start().0);

                    old_transforms.seek_forward(&edit.old.end, Bias::Right, &());
                    if old_transforms.item().map_or(false, |t| t.is_fold()) {
                        old_transforms.next(&());
                        edit.old.end = old_transforms.start().0;
                    }
                    let old_end =
                        old_transforms.start().1 .0 + (edit.old.end - old_transforms.start().0);

                    new_transforms.seek(&edit.new.start, Bias::Left, &());
                    if new_transforms.item().map_or(false, |t| t.is_fold()) {
                        edit.new.start = new_transforms.start().0;
                    }
                    let new_start =
                        new_transforms.start().1 .0 + (edit.new.start - new_transforms.start().0);

                    new_transforms.seek_forward(&edit.new.end, Bias::Right, &());
                    if new_transforms.item().map_or(false, |t| t.is_fold()) {
                        new_transforms.next(&());
                        edit.new.end = new_transforms.start().0;
                    }
                    let new_end =
                        new_transforms.start().1 .0 + (edit.new.end - new_transforms.start().0);

                    fold_edits.push(FoldEdit {
                        old: FoldOffset(old_start)..FoldOffset(old_end),
                        new: FoldOffset(new_start)..FoldOffset(new_end),
                    });
                }

                consolidate_fold_edits(&mut fold_edits);
            }

            *transforms = new_transforms;
            *self.buffer.lock() = new_buffer;
            self.version.fetch_add(1, SeqCst);
            fold_edits
        }
    }
}

#[derive(Clone)]
pub struct FoldSnapshot {
    transforms: SumTree<Transform>,
    folds: SumTree<Fold>,
    buffer_snapshot: MultiBufferSnapshot,
    pub version: usize,
}

impl FoldSnapshot {
    pub fn buffer_snapshot(&self) -> &MultiBufferSnapshot {
        &self.buffer_snapshot
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(FoldOffset(0)..self.len(), false, None)
            .map(|c| c.text)
            .collect()
    }

    #[cfg(test)]
    pub fn fold_count(&self) -> usize {
        self.folds.items(&self.buffer_snapshot).len()
    }

    pub fn text_summary_for_range(&self, range: Range<FoldPoint>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self.transforms.cursor::<(FoldPoint, Point)>();
        cursor.seek(&range.start, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let start_in_transform = range.start.0 - cursor.start().0 .0;
            let end_in_transform = cmp::min(range.end, cursor.end(&()).0).0 - cursor.start().0 .0;
            if let Some(output_text) = transform.output_text {
                summary = TextSummary::from(
                    &output_text
                        [start_in_transform.column as usize..end_in_transform.column as usize],
                );
            } else {
                let buffer_start = cursor.start().1 + start_in_transform;
                let buffer_end = cursor.start().1 + end_in_transform;
                summary = self
                    .buffer_snapshot
                    .text_summary_for_range(buffer_start..buffer_end);
            }
        }

        if range.end > cursor.end(&()).0 {
            cursor.next(&());
            summary += &cursor
                .summary::<_, TransformSummary>(&range.end, Bias::Right, &())
                .output;
            if let Some(transform) = cursor.item() {
                let end_in_transform = range.end.0 - cursor.start().0 .0;
                if let Some(output_text) = transform.output_text {
                    summary += TextSummary::from(&output_text[..end_in_transform.column as usize]);
                } else {
                    let buffer_start = cursor.start().1;
                    let buffer_end = cursor.start().1 + end_in_transform;
                    summary += self
                        .buffer_snapshot
                        .text_summary_for_range::<TextSummary, _>(buffer_start..buffer_end);
                }
            }
        }

        summary
    }

    pub fn to_fold_point(&self, point: Point, bias: Bias) -> FoldPoint {
        let mut cursor = self.transforms.cursor::<(Point, FoldPoint)>();
        cursor.seek(&point, Bias::Right, &());
        if cursor.item().map_or(false, |t| t.is_fold()) {
            if bias == Bias::Left || point == cursor.start().0 {
                cursor.start().1
            } else {
                cursor.end(&()).1
            }
        } else {
            let overshoot = point - cursor.start().0;
            FoldPoint(cmp::min(
                cursor.start().1 .0 + overshoot,
                cursor.end(&()).1 .0,
            ))
        }
    }

    pub fn len(&self) -> FoldOffset {
        FoldOffset(self.transforms.summary().output.bytes)
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
        let mut cursor = self.transforms.cursor::<(FoldPoint, Point)>();
        cursor.seek(&fold_point, Bias::Left, &());

        let overshoot = fold_point.0 - cursor.start().0 .0;
        let buffer_point = cursor.start().1 + overshoot;
        let input_buffer_rows = self.buffer_snapshot.buffer_rows(buffer_point.row);

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

    pub fn folds_in_range<'a, T>(
        &'a self,
        range: Range<T>,
    ) -> impl Iterator<Item = &'a Range<Anchor>>
    where
        T: ToOffset,
    {
        let mut folds = intersecting_folds(&self.buffer_snapshot, &self.folds, range, false);
        iter::from_fn(move || {
            let item = folds.item().map(|f| &f.0);
            folds.next(&self.buffer_snapshot);
            item
        })
    }

    pub fn intersects_fold<T>(&self, offset: T) -> bool
    where
        T: ToOffset,
    {
        let offset = offset.to_offset(&self.buffer_snapshot);
        let mut cursor = self.transforms.cursor::<usize>();
        cursor.seek(&offset, Bias::Right, &());
        cursor.item().map_or(false, |t| t.output_text.is_some())
    }

    pub fn is_line_folded(&self, output_row: u32) -> bool {
        let mut cursor = self.transforms.cursor::<FoldPoint>();
        cursor.seek(&FoldPoint::new(output_row, 0), Bias::Right, &());
        while let Some(transform) = cursor.item() {
            if transform.output_text.is_some() {
                return true;
            }
            if cursor.end(&()).row() == output_row {
                cursor.next(&())
            } else {
                break;
            }
        }
        false
    }

    pub fn chars_at(&self, start: FoldPoint) -> impl '_ + Iterator<Item = char> {
        let start = start.to_offset(self);
        self.chunks(start..self.len(), false, None)
            .flat_map(|chunk| chunk.text.chars())
    }

    pub fn chunks<'a>(
        &'a self,
        range: Range<FoldOffset>,
        language_aware: bool,
        text_highlights: Option<&'a TextHighlights>,
    ) -> FoldChunks<'a> {
        let mut highlight_endpoints = Vec::new();
        let mut transform_cursor = self.transforms.cursor::<(FoldOffset, usize)>();

        let buffer_end = {
            transform_cursor.seek(&range.end, Bias::Right, &());
            let overshoot = range.end.0 - transform_cursor.start().0 .0;
            transform_cursor.start().1 + overshoot
        };

        let buffer_start = {
            transform_cursor.seek(&range.start, Bias::Right, &());
            let overshoot = range.start.0 - transform_cursor.start().0 .0;
            transform_cursor.start().1 + overshoot
        };

        if let Some(text_highlights) = text_highlights {
            if !text_highlights.is_empty() {
                while transform_cursor.start().0 < range.end {
                    if !transform_cursor.item().unwrap().is_fold() {
                        let transform_start = self
                            .buffer_snapshot
                            .anchor_after(cmp::max(buffer_start, transform_cursor.start().1));

                        let transform_end = {
                            let overshoot = range.end.0 - transform_cursor.start().0 .0;
                            self.buffer_snapshot.anchor_before(cmp::min(
                                transform_cursor.end(&()).1,
                                transform_cursor.start().1 + overshoot,
                            ))
                        };

                        for (tag, highlights) in text_highlights.iter() {
                            let style = highlights.0;
                            let ranges = &highlights.1;

                            let start_ix = match ranges.binary_search_by(|probe| {
                                let cmp = probe.end.cmp(&transform_start, &self.buffer_snapshot());
                                if cmp.is_gt() {
                                    Ordering::Greater
                                } else {
                                    Ordering::Less
                                }
                            }) {
                                Ok(i) | Err(i) => i,
                            };
                            for range in &ranges[start_ix..] {
                                if range
                                    .start
                                    .cmp(&transform_end, &self.buffer_snapshot)
                                    .is_ge()
                                {
                                    break;
                                }

                                highlight_endpoints.push(HighlightEndpoint {
                                    offset: range.start.to_offset(&self.buffer_snapshot),
                                    is_start: true,
                                    tag: *tag,
                                    style,
                                });
                                highlight_endpoints.push(HighlightEndpoint {
                                    offset: range.end.to_offset(&self.buffer_snapshot),
                                    is_start: false,
                                    tag: *tag,
                                    style,
                                });
                            }
                        }
                    }

                    transform_cursor.next(&());
                }
                highlight_endpoints.sort();
                transform_cursor.seek(&range.start, Bias::Right, &());
            }
        }

        FoldChunks {
            transform_cursor,
            buffer_chunks: self
                .buffer_snapshot
                .chunks(buffer_start..buffer_end, language_aware),
            buffer_chunk: None,
            buffer_offset: buffer_start,
            output_offset: range.start.0,
            max_output_offset: range.end.0,
            highlight_endpoints: highlight_endpoints.into_iter().peekable(),
            active_highlights: Default::default(),
        }
    }

    #[cfg(test)]
    pub fn clip_offset(&self, offset: FoldOffset, bias: Bias) -> FoldOffset {
        let mut cursor = self.transforms.cursor::<(FoldOffset, usize)>();
        cursor.seek(&offset, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let transform_start = cursor.start().0 .0;
            if transform.output_text.is_some() {
                if offset.0 == transform_start || matches!(bias, Bias::Left) {
                    FoldOffset(transform_start)
                } else {
                    FoldOffset(cursor.end(&()).0 .0)
                }
            } else {
                let overshoot = offset.0 - transform_start;
                let buffer_offset = cursor.start().1 + overshoot;
                let clipped_buffer_offset = self.buffer_snapshot.clip_offset(buffer_offset, bias);
                FoldOffset(
                    (offset.0 as isize + (clipped_buffer_offset as isize - buffer_offset as isize))
                        as usize,
                )
            }
        } else {
            FoldOffset(self.transforms.summary().output.bytes)
        }
    }

    pub fn clip_point(&self, point: FoldPoint, bias: Bias) -> FoldPoint {
        let mut cursor = self.transforms.cursor::<(FoldPoint, Point)>();
        cursor.seek(&point, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let transform_start = cursor.start().0 .0;
            if transform.output_text.is_some() {
                if point.0 == transform_start || matches!(bias, Bias::Left) {
                    FoldPoint(transform_start)
                } else {
                    FoldPoint(cursor.end(&()).0 .0)
                }
            } else {
                let overshoot = point.0 - transform_start;
                let buffer_position = cursor.start().1 + overshoot;
                let clipped_buffer_position =
                    self.buffer_snapshot.clip_point(buffer_position, bias);
                FoldPoint(cursor.start().0 .0 + (clipped_buffer_position - cursor.start().1))
            }
        } else {
            FoldPoint(self.transforms.summary().output.lines)
        }
    }
}

fn intersecting_folds<'a, T>(
    buffer: &'a MultiBufferSnapshot,
    folds: &'a SumTree<Fold>,
    range: Range<T>,
    inclusive: bool,
) -> FilterCursor<'a, impl 'a + FnMut(&FoldSummary) -> bool, Fold, usize>
where
    T: ToOffset,
{
    let start = buffer.anchor_before(range.start.to_offset(buffer));
    let end = buffer.anchor_after(range.end.to_offset(buffer));
    let mut cursor = folds.filter::<_, usize>(move |summary| {
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

fn consolidate_buffer_edits(edits: &mut Vec<text::Edit<usize>>) {
    edits.sort_unstable_by(|a, b| {
        a.old
            .start
            .cmp(&b.old.start)
            .then_with(|| b.old.end.cmp(&a.old.end))
    });

    let mut i = 1;
    while i < edits.len() {
        let edit = edits[i].clone();
        let prev_edit = &mut edits[i - 1];
        if prev_edit.old.end >= edit.old.start {
            prev_edit.old.end = prev_edit.old.end.max(edit.old.end);
            prev_edit.new.start = prev_edit.new.start.min(edit.new.start);
            prev_edit.new.end = prev_edit.new.end.max(edit.new.end);
            edits.remove(i);
            continue;
        }
        i += 1;
    }
}

fn consolidate_fold_edits(edits: &mut Vec<FoldEdit>) {
    edits.sort_unstable_by(|a, b| {
        a.old
            .start
            .cmp(&b.old.start)
            .then_with(|| b.old.end.cmp(&a.old.end))
    });

    let mut i = 1;
    while i < edits.len() {
        let edit = edits[i].clone();
        let prev_edit = &mut edits[i - 1];
        if prev_edit.old.end >= edit.old.start {
            prev_edit.old.end = prev_edit.old.end.max(edit.old.end);
            prev_edit.new.start = prev_edit.new.start.min(edit.new.start);
            prev_edit.new.end = prev_edit.new.end.max(edit.new.end);
            edits.remove(i);
            continue;
        }
        i += 1;
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Transform {
    summary: TransformSummary,
    output_text: Option<&'static str>,
}

impl Transform {
    fn is_fold(&self) -> bool {
        self.output_text.is_some()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TransformSummary {
    output: TextSummary,
    input: TextSummary,
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self) -> Self::Summary {
        self.summary.clone()
    }
}

impl sum_tree::Summary for TransformSummary {
    type Context = ();

    fn add_summary(&mut self, other: &Self, _: &()) {
        self.input += &other.input;
        self.output += &other.output;
    }
}

#[derive(Clone, Debug)]
struct Fold(Range<Anchor>);

impl Default for Fold {
    fn default() -> Self {
        Self(Anchor::min()..Anchor::max())
    }
}

impl sum_tree::Item for Fold {
    type Summary = FoldSummary;

    fn summary(&self) -> Self::Summary {
        FoldSummary {
            start: self.0.start.clone(),
            end: self.0.end.clone(),
            min_start: self.0.start.clone(),
            max_end: self.0.end.clone(),
            count: 1,
        }
    }
}

#[derive(Clone, Debug)]
struct FoldSummary {
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

    fn add_summary(&mut self, other: &Self, buffer: &MultiBufferSnapshot) {
        if other.min_start.cmp(&self.min_start, buffer) == Ordering::Less {
            self.min_start = other.min_start.clone();
        }
        if other.max_end.cmp(&self.max_end, buffer) == Ordering::Greater {
            self.max_end = other.max_end.clone();
        }

        #[cfg(debug_assertions)]
        {
            let start_comparison = self.start.cmp(&other.start, buffer);
            assert!(start_comparison <= Ordering::Equal);
            if start_comparison == Ordering::Equal {
                assert!(self.end.cmp(&other.end, buffer) >= Ordering::Equal);
            }
        }

        self.start = other.start.clone();
        self.end = other.end.clone();
        self.count += other.count;
    }
}

impl<'a> sum_tree::Dimension<'a, FoldSummary> for Fold {
    fn add_summary(&mut self, summary: &'a FoldSummary, _: &MultiBufferSnapshot) {
        self.0.start = summary.start.clone();
        self.0.end = summary.end.clone();
    }
}

impl<'a> sum_tree::SeekTarget<'a, FoldSummary, Fold> for Fold {
    fn cmp(&self, other: &Self, buffer: &MultiBufferSnapshot) -> Ordering {
        self.0.cmp(&other.0, buffer)
    }
}

impl<'a> sum_tree::Dimension<'a, FoldSummary> for usize {
    fn add_summary(&mut self, summary: &'a FoldSummary, _: &MultiBufferSnapshot) {
        *self += summary.count;
    }
}

pub struct FoldBufferRows<'a> {
    cursor: Cursor<'a, Transform, (FoldPoint, Point)>,
    input_buffer_rows: MultiBufferRows<'a>,
    fold_point: FoldPoint,
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
                self.input_buffer_rows.seek(self.cursor.start().1.row);
                self.input_buffer_rows.next();
            }
            *self.fold_point.row_mut() += 1;
            self.input_buffer_rows.next()
        } else {
            None
        }
    }
}

pub struct FoldChunks<'a> {
    transform_cursor: Cursor<'a, Transform, (FoldOffset, usize)>,
    buffer_chunks: MultiBufferChunks<'a>,
    buffer_chunk: Option<(usize, Chunk<'a>)>,
    buffer_offset: usize,
    output_offset: usize,
    max_output_offset: usize,
    highlight_endpoints: Peekable<vec::IntoIter<HighlightEndpoint>>,
    active_highlights: BTreeMap<Option<TypeId>, HighlightStyle>,
}

impl<'a> Iterator for FoldChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output_offset >= self.max_output_offset {
            return None;
        }

        let transform = if let Some(item) = self.transform_cursor.item() {
            item
        } else {
            return None;
        };

        // If we're in a fold, then return the fold's display text and
        // advance the transform and buffer cursors to the end of the fold.
        if let Some(output_text) = transform.output_text {
            self.buffer_chunk.take();
            self.buffer_offset += transform.summary.input.bytes;
            self.buffer_chunks.seek(self.buffer_offset);

            while self.buffer_offset >= self.transform_cursor.end(&()).1
                && self.transform_cursor.item().is_some()
            {
                self.transform_cursor.next(&());
            }

            self.output_offset += output_text.len();
            return Some(Chunk {
                text: output_text,
                syntax_highlight_id: None,
                highlight_style: None,
                diagnostic_severity: None,
                is_unnecessary: false,
            });
        }

        let mut next_highlight_endpoint = usize::MAX;
        while let Some(endpoint) = self.highlight_endpoints.peek().copied() {
            if endpoint.offset <= self.buffer_offset {
                if endpoint.is_start {
                    self.active_highlights.insert(endpoint.tag, endpoint.style);
                } else {
                    self.active_highlights.remove(&endpoint.tag);
                }
                self.highlight_endpoints.next();
            } else {
                next_highlight_endpoint = endpoint.offset;
                break;
            }
        }

        // Retrieve a chunk from the current location in the buffer.
        if self.buffer_chunk.is_none() {
            let chunk_offset = self.buffer_chunks.offset();
            self.buffer_chunk = self.buffer_chunks.next().map(|chunk| (chunk_offset, chunk));
        }

        // Otherwise, take a chunk from the buffer's text.
        if let Some((buffer_chunk_start, mut chunk)) = self.buffer_chunk {
            let buffer_chunk_end = buffer_chunk_start + chunk.text.len();
            let transform_end = self.transform_cursor.end(&()).1;
            let chunk_end = buffer_chunk_end
                .min(transform_end)
                .min(next_highlight_endpoint);

            chunk.text = &chunk.text
                [self.buffer_offset - buffer_chunk_start..chunk_end - buffer_chunk_start];

            if !self.active_highlights.is_empty() {
                let mut highlight_style = HighlightStyle::default();
                for active_highlight in self.active_highlights.values() {
                    highlight_style.highlight(*active_highlight);
                }
                chunk.highlight_style = Some(highlight_style);
            }

            if chunk_end == transform_end {
                self.transform_cursor.next(&());
            } else if chunk_end == buffer_chunk_end {
                self.buffer_chunk.take();
            }

            self.buffer_offset = chunk_end;
            self.output_offset += chunk.text.len();
            return Some(chunk);
        }

        None
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct HighlightEndpoint {
    offset: usize,
    is_start: bool,
    tag: Option<TypeId>,
    style: HighlightStyle,
}

impl PartialOrd for HighlightEndpoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HighlightEndpoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset
            .cmp(&other.offset)
            .then_with(|| other.is_start.cmp(&self.is_start))
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct FoldOffset(pub usize);

impl FoldOffset {
    pub fn to_point(&self, snapshot: &FoldSnapshot) -> FoldPoint {
        let mut cursor = snapshot
            .transforms
            .cursor::<(FoldOffset, TransformSummary)>();
        cursor.seek(self, Bias::Right, &());
        let overshoot = if cursor.item().map_or(true, |t| t.is_fold()) {
            Point::new(0, (self.0 - cursor.start().0 .0) as u32)
        } else {
            let buffer_offset = cursor.start().1.input.bytes + self.0 - cursor.start().0 .0;
            let buffer_point = snapshot.buffer_snapshot.offset_to_point(buffer_offset);
            buffer_point - cursor.start().1.input.lines
        };
        FoldPoint(cursor.start().1.output.lines + overshoot)
    }
}

impl Sub for FoldOffset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for FoldOffset {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.output.bytes;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for Point {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += &summary.input.lines;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for usize {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += &summary.input.bytes;
    }
}

pub type FoldEdit = Edit<FoldOffset>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MultiBuffer, ToPoint};
    use rand::prelude::*;
    use settings::Settings;
    use std::{cmp::Reverse, env, mem, sync::Arc};
    use sum_tree::TreeMap;
    use text::RandomCharIter;
    use util::test::sample_text;
    use Bias::{Left, Right};

    #[gpui::test]
    fn test_basic_folds(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let mut map = FoldMap::new(buffer_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(buffer_snapshot.clone(), vec![]);
        let (snapshot2, edits) = writer.fold(vec![
            Point::new(0, 2)..Point::new(2, 2),
            Point::new(2, 4)..Point::new(4, 1),
        ]);
        assert_eq!(snapshot2.text(), "aa…cc…eeeee");
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
                cx,
            );
            buffer.snapshot(cx)
        });
        let (snapshot3, edits) =
            map.read(buffer_snapshot.clone(), subscription.consume().into_inner());
        assert_eq!(snapshot3.text(), "123a…c123c…eeeee");
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
            buffer.edit([(Point::new(2, 6)..Point::new(4, 3), "456")], cx);
            buffer.snapshot(cx)
        });
        let (snapshot4, _) = map.read(buffer_snapshot.clone(), subscription.consume().into_inner());
        assert_eq!(snapshot4.text(), "123a…c123456eee");

        let (mut writer, _, _) = map.write(buffer_snapshot.clone(), vec![]);
        writer.unfold(Some(Point::new(0, 4)..Point::new(0, 4)), false);
        let (snapshot5, _) = map.read(buffer_snapshot.clone(), vec![]);
        assert_eq!(snapshot5.text(), "123a…c123456eee");

        let (mut writer, _, _) = map.write(buffer_snapshot.clone(), vec![]);
        writer.unfold(Some(Point::new(0, 4)..Point::new(0, 4)), true);
        let (snapshot6, _) = map.read(buffer_snapshot.clone(), vec![]);
        assert_eq!(snapshot6.text(), "123aaaaa\nbbbbbb\nccc123456eee");
    }

    #[gpui::test]
    fn test_adjacent_folds(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple("abcdefghijkl", cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let buffer_snapshot = buffer.read(cx).snapshot(cx);

        {
            let mut map = FoldMap::new(buffer_snapshot.clone()).0;

            let (mut writer, _, _) = map.write(buffer_snapshot.clone(), vec![]);
            writer.fold(vec![5..8]);
            let (snapshot, _) = map.read(buffer_snapshot.clone(), vec![]);
            assert_eq!(snapshot.text(), "abcde…ijkl");

            // Create an fold adjacent to the start of the first fold.
            let (mut writer, _, _) = map.write(buffer_snapshot.clone(), vec![]);
            writer.fold(vec![0..1, 2..5]);
            let (snapshot, _) = map.read(buffer_snapshot.clone(), vec![]);
            assert_eq!(snapshot.text(), "…b…ijkl");

            // Create an fold adjacent to the end of the first fold.
            let (mut writer, _, _) = map.write(buffer_snapshot.clone(), vec![]);
            writer.fold(vec![11..11, 8..10]);
            let (snapshot, _) = map.read(buffer_snapshot.clone(), vec![]);
            assert_eq!(snapshot.text(), "…b…kl");
        }

        {
            let mut map = FoldMap::new(buffer_snapshot.clone()).0;

            // Create two adjacent folds.
            let (mut writer, _, _) = map.write(buffer_snapshot.clone(), vec![]);
            writer.fold(vec![0..2, 2..5]);
            let (snapshot, _) = map.read(buffer_snapshot.clone(), vec![]);
            assert_eq!(snapshot.text(), "…fghijkl");

            // Edit within one of the folds.
            let buffer_snapshot = buffer.update(cx, |buffer, cx| {
                buffer.edit([(0..1, "12345")], cx);
                buffer.snapshot(cx)
            });
            let (snapshot, _) =
                map.read(buffer_snapshot.clone(), subscription.consume().into_inner());
            assert_eq!(snapshot.text(), "12345…fghijkl");
        }
    }

    #[gpui::test]
    fn test_overlapping_folds(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let mut map = FoldMap::new(buffer_snapshot.clone()).0;
        let (mut writer, _, _) = map.write(buffer_snapshot.clone(), vec![]);
        writer.fold(vec![
            Point::new(0, 2)..Point::new(2, 2),
            Point::new(0, 4)..Point::new(1, 0),
            Point::new(1, 2)..Point::new(3, 2),
            Point::new(3, 1)..Point::new(4, 1),
        ]);
        let (snapshot, _) = map.read(buffer_snapshot.clone(), vec![]);
        assert_eq!(snapshot.text(), "aa…eeeee");
    }

    #[gpui::test]
    fn test_merging_folds_via_edit(cx: &mut gpui::MutableAppContext) {
        cx.set_global(Settings::test(cx));
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let subscription = buffer.update(cx, |buffer, _| buffer.subscribe());
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let mut map = FoldMap::new(buffer_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(buffer_snapshot.clone(), vec![]);
        writer.fold(vec![
            Point::new(0, 2)..Point::new(2, 2),
            Point::new(3, 1)..Point::new(4, 1),
        ]);
        let (snapshot, _) = map.read(buffer_snapshot.clone(), vec![]);
        assert_eq!(snapshot.text(), "aa…cccc\nd…eeeee");

        let buffer_snapshot = buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 2)..Point::new(3, 1), "")], cx);
            buffer.snapshot(cx)
        });
        let (snapshot, _) = map.read(buffer_snapshot.clone(), subscription.consume().into_inner());
        assert_eq!(snapshot.text(), "aa…eeeee");
    }

    #[gpui::test]
    fn test_folds_in_range(cx: &mut gpui::MutableAppContext) {
        let buffer = MultiBuffer::build_simple(&sample_text(5, 6, 'a'), cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let mut map = FoldMap::new(buffer_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(buffer_snapshot.clone(), vec![]);
        writer.fold(vec![
            Point::new(0, 2)..Point::new(2, 2),
            Point::new(0, 4)..Point::new(1, 0),
            Point::new(1, 2)..Point::new(3, 2),
            Point::new(3, 1)..Point::new(4, 1),
        ]);
        let (snapshot, _) = map.read(buffer_snapshot.clone(), vec![]);
        let fold_ranges = snapshot
            .folds_in_range(Point::new(1, 0)..Point::new(1, 3))
            .map(|fold| fold.start.to_point(&buffer_snapshot)..fold.end.to_point(&buffer_snapshot))
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
    fn test_random_folds(cx: &mut gpui::MutableAppContext, mut rng: StdRng) {
        cx.set_global(Settings::test(cx));
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
        let mut map = FoldMap::new(buffer_snapshot.clone()).0;

        let (mut initial_snapshot, _) = map.read(buffer_snapshot.clone(), vec![]);
        let mut snapshot_edits = Vec::new();

        let mut highlights = TreeMap::default();
        let highlight_count = rng.gen_range(0_usize..10);
        let mut highlight_ranges = (0..highlight_count)
            .map(|_| buffer_snapshot.random_byte_range(0, &mut rng))
            .collect::<Vec<_>>();
        highlight_ranges.sort_by_key(|range| (range.start, Reverse(range.end)));
        log::info!("highlighting ranges {:?}", highlight_ranges);
        let highlight_ranges = highlight_ranges
            .into_iter()
            .map(|range| {
                buffer_snapshot.anchor_before(range.start)..buffer_snapshot.anchor_after(range.end)
            })
            .collect::<Vec<_>>();

        highlights.insert(
            Some(TypeId::of::<()>()),
            Arc::new((HighlightStyle::default(), highlight_ranges)),
        );

        for _ in 0..operations {
            log::info!("text: {:?}", buffer_snapshot.text());
            let mut buffer_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=59 => {
                    snapshot_edits.extend(map.randomly_mutate(&mut rng));
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

            let (snapshot, edits) = map.read(buffer_snapshot.clone(), buffer_edits);
            snapshot_edits.push((snapshot.clone(), edits));

            let mut expected_text: String = buffer_snapshot.text().to_string();
            for fold_range in map.merged_fold_ranges().into_iter().rev() {
                expected_text.replace_range(fold_range.start..fold_range.end, "…");
            }

            assert_eq!(snapshot.text(), expected_text);
            log::info!(
                "fold text {:?} ({} lines)",
                expected_text,
                expected_text.matches('\n').count() + 1
            );

            let mut prev_row = 0;
            let mut expected_buffer_rows = Vec::new();
            for fold_range in map.merged_fold_ranges().into_iter() {
                let fold_start = buffer_snapshot.offset_to_point(fold_range.start).row;
                let fold_end = buffer_snapshot.offset_to_point(fold_range.end).row;
                expected_buffer_rows.extend(
                    buffer_snapshot
                        .buffer_rows(prev_row)
                        .take((1 + fold_start - prev_row) as usize),
                );
                prev_row = 1 + fold_end;
            }
            expected_buffer_rows.extend(buffer_snapshot.buffer_rows(prev_row));

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
                let buffer_point = fold_point.to_buffer_point(&snapshot);
                let buffer_offset = buffer_point.to_offset(&buffer_snapshot);
                assert_eq!(
                    snapshot.to_fold_point(buffer_point, Right),
                    fold_point,
                    "{:?} -> fold point",
                    buffer_point,
                );
                assert_eq!(
                    fold_point.to_buffer_offset(&snapshot),
                    buffer_offset,
                    "fold_point.to_buffer_offset({:?})",
                    fold_point,
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
                        .chunks(start..end, false, Some(&highlights))
                        .map(|c| c.text)
                        .collect::<String>(),
                    text,
                );
            }

            let mut fold_row = 0;
            while fold_row < expected_buffer_rows.len() as u32 {
                fold_row = snapshot
                    .clip_point(FoldPoint::new(fold_row, 0), Bias::Right)
                    .row();
                assert_eq!(
                    snapshot.buffer_rows(fold_row).collect::<Vec<_>>(),
                    expected_buffer_rows[(fold_row as usize)..],
                    "wrong buffer rows starting at fold row {}",
                    fold_row,
                );
                fold_row += 1;
            }

            for fold_range in map.merged_fold_ranges() {
                let fold_point =
                    snapshot.to_fold_point(fold_range.start.to_point(&buffer_snapshot), Right);
                assert!(snapshot.is_line_folded(fold_point.row()));
            }

            for _ in 0..5 {
                let end =
                    buffer_snapshot.clip_offset(rng.gen_range(0..=buffer_snapshot.len()), Right);
                let start = buffer_snapshot.clip_offset(rng.gen_range(0..=end), Left);
                let expected_folds = map
                    .folds
                    .items(&buffer_snapshot)
                    .into_iter()
                    .filter(|fold| {
                        let start = buffer_snapshot.anchor_before(start);
                        let end = buffer_snapshot.anchor_after(end);
                        start.cmp(&fold.0.end, &buffer_snapshot) == Ordering::Less
                            && end.cmp(&fold.0.start, &buffer_snapshot) == Ordering::Greater
                    })
                    .map(|fold| fold.0)
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
    fn test_buffer_rows(cx: &mut gpui::MutableAppContext) {
        let text = sample_text(6, 6, 'a') + "\n";
        let buffer = MultiBuffer::build_simple(&text, cx);

        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let mut map = FoldMap::new(buffer_snapshot.clone()).0;

        let (mut writer, _, _) = map.write(buffer_snapshot.clone(), vec![]);
        writer.fold(vec![
            Point::new(0, 2)..Point::new(2, 2),
            Point::new(3, 1)..Point::new(4, 1),
        ]);

        let (snapshot, _) = map.read(buffer_snapshot.clone(), vec![]);
        assert_eq!(snapshot.text(), "aa…cccc\nd…eeeee\nffffff\n");
        assert_eq!(
            snapshot.buffer_rows(0).collect::<Vec<_>>(),
            [Some(0), Some(3), Some(5), Some(6)]
        );
        assert_eq!(snapshot.buffer_rows(3).collect::<Vec<_>>(), [Some(6)]);
    }

    impl FoldMap {
        fn merged_fold_ranges(&self) -> Vec<Range<usize>> {
            let buffer = self.buffer.lock().clone();
            let mut folds = self.folds.items(&buffer);
            // Ensure sorting doesn't change how folds get merged and displayed.
            folds.sort_by(|a, b| a.0.cmp(&b.0, &buffer));
            let mut fold_ranges = folds
                .iter()
                .map(|fold| fold.0.start.to_offset(&buffer)..fold.0.end.to_offset(&buffer))
                .peekable();

            let mut merged_ranges = Vec::new();
            while let Some(mut fold_range) = fold_ranges.next() {
                while let Some(next_range) = fold_ranges.peek() {
                    if fold_range.end >= next_range.start {
                        if next_range.end > fold_range.end {
                            fold_range.end = next_range.end;
                        }
                        fold_ranges.next();
                    } else {
                        break;
                    }
                }
                if fold_range.end > fold_range.start {
                    merged_ranges.push(fold_range);
                }
            }
            merged_ranges
        }

        pub fn randomly_mutate(
            &mut self,
            rng: &mut impl Rng,
        ) -> Vec<(FoldSnapshot, Vec<FoldEdit>)> {
            let mut snapshot_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=39 if !self.folds.is_empty() => {
                    let buffer = self.buffer.lock().clone();
                    let mut to_unfold = Vec::new();
                    for _ in 0..rng.gen_range(1..=3) {
                        let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                        let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                        to_unfold.push(start..end);
                    }
                    log::info!("unfolding {:?}", to_unfold);
                    let (mut writer, snapshot, edits) = self.write(buffer, vec![]);
                    snapshot_edits.push((snapshot, edits));
                    let (snapshot, edits) = writer.fold(to_unfold);
                    snapshot_edits.push((snapshot, edits));
                }
                _ => {
                    let buffer = self.buffer.lock().clone();
                    let mut to_fold = Vec::new();
                    for _ in 0..rng.gen_range(1..=2) {
                        let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                        let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                        to_fold.push(start..end);
                    }
                    log::info!("folding {:?}", to_fold);
                    let (mut writer, snapshot, edits) = self.write(buffer, vec![]);
                    snapshot_edits.push((snapshot, edits));
                    let (snapshot, edits) = writer.fold(to_fold);
                    snapshot_edits.push((snapshot, edits));
                }
            }
            snapshot_edits
        }
    }
}

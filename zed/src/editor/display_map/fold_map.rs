use super::{
    buffer::{AnchorRangeExt, TextSummary},
    Anchor, Buffer, Point as InputPoint, ToOffset,
};
use crate::{
    editor::buffer,
    settings::StyleId,
    sum_tree::{self, Cursor, FilterCursor, SumTree},
    time,
    util::Bias,
};
use gpui::{AppContext, ModelHandle};
use parking_lot::Mutex;
use std::{
    cmp::{self, Ordering},
    iter,
    ops::{Range, Sub},
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct OutputPoint(super::Point);

impl OutputPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(super::Point::new(row, column))
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
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
}

impl Sub<Self> for OutputPoint {
    type Output = OutputPoint;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

pub struct FoldMapWriter<'a>(&'a mut FoldMap);

impl<'a> FoldMapWriter<'a> {
    pub fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &AppContext,
    ) -> (Snapshot, Vec<Edit>) {
        let mut edits = Vec::new();
        let mut folds = Vec::new();
        let buffer = self.0.buffer.read(cx).snapshot();
        for range in ranges.into_iter() {
            let range = range.start.to_offset(&buffer)..range.end.to_offset(&buffer);
            if range.start != range.end {
                let fold = Fold(buffer.anchor_after(range.start)..buffer.anchor_before(range.end));
                folds.push(fold);
                edits.push(buffer::Edit {
                    old_bytes: range.clone(),
                    new_bytes: range.clone(),
                    ..Default::default()
                });
            }
        }

        folds.sort_unstable_by(|a, b| sum_tree::SeekDimension::cmp(a, b, &buffer));

        self.0.folds = {
            let mut new_tree = SumTree::new();
            let mut cursor = self.0.folds.cursor::<_, ()>();
            for fold in folds {
                new_tree.push_tree(cursor.slice(&fold, Bias::Right, &buffer), &buffer);
                new_tree.push(fold, &buffer);
            }
            new_tree.push_tree(cursor.suffix(&buffer), &buffer);
            new_tree
        };

        consolidate_input_edits(&mut edits);
        let edits = self.0.apply_edits(edits, cx);
        let snapshot = Snapshot {
            transforms: self.0.transforms.lock().clone(),
            folds: self.0.folds.clone(),
            buffer: self.0.buffer.read(cx).snapshot(),
            version: self.0.version.load(SeqCst),
        };
        (snapshot, edits)
    }

    pub fn unfold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &AppContext,
    ) -> (Snapshot, Vec<Edit>) {
        let mut edits = Vec::new();
        let mut fold_ixs_to_delete = Vec::new();
        let buffer = self.0.buffer.read(cx).snapshot();
        for range in ranges.into_iter() {
            // Remove intersecting folds and add their ranges to edits that are passed to apply_edits.
            let mut folds_cursor = intersecting_folds(&buffer, &self.0.folds, range);
            while let Some(fold) = folds_cursor.item() {
                let offset_range = fold.0.start.to_offset(&buffer)..fold.0.end.to_offset(&buffer);
                edits.push(buffer::Edit {
                    old_bytes: offset_range.clone(),
                    new_bytes: offset_range,
                    ..Default::default()
                });
                fold_ixs_to_delete.push(*folds_cursor.start());
                folds_cursor.next(&buffer);
            }
        }

        fold_ixs_to_delete.sort_unstable();
        fold_ixs_to_delete.dedup();

        self.0.folds = {
            let mut cursor = self.0.folds.cursor::<_, ()>();
            let mut folds = SumTree::new();
            for fold_ix in fold_ixs_to_delete {
                folds.push_tree(cursor.slice(&fold_ix, Bias::Right, &buffer), &buffer);
                cursor.next(&buffer);
            }
            folds.push_tree(cursor.suffix(&buffer), &buffer);
            folds
        };

        consolidate_input_edits(&mut edits);
        let edits = self.0.apply_edits(edits, cx);
        let snapshot = Snapshot {
            transforms: self.0.transforms.lock().clone(),
            folds: self.0.folds.clone(),
            buffer: self.0.buffer.read(cx).snapshot(),
            version: self.0.version.load(SeqCst),
        };
        (snapshot, edits)
    }
}

pub struct FoldMap {
    buffer: ModelHandle<Buffer>,
    transforms: Mutex<SumTree<Transform>>,
    folds: SumTree<Fold>,
    last_sync: Mutex<time::Global>,
    version: AtomicUsize,
}

impl FoldMap {
    pub fn new(buffer_handle: ModelHandle<Buffer>, cx: &AppContext) -> Self {
        let buffer = buffer_handle.read(cx);
        Self {
            buffer: buffer_handle,
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
            last_sync: Mutex::new(buffer.version()),
            version: AtomicUsize::new(0),
        }
    }

    pub fn read(&self, cx: &AppContext) -> (Snapshot, Vec<Edit>) {
        let edits = self.sync(cx);
        self.check_invariants(cx);
        let snapshot = Snapshot {
            transforms: self.transforms.lock().clone(),
            folds: self.folds.clone(),
            buffer: self.buffer.read(cx).snapshot(),
            version: self.version.load(SeqCst),
        };
        (snapshot, edits)
    }

    pub fn write(&mut self, cx: &AppContext) -> (FoldMapWriter, Snapshot, Vec<Edit>) {
        let (snapshot, edits) = self.read(cx);
        (FoldMapWriter(self), snapshot, edits)
    }

    fn sync(&self, cx: &AppContext) -> Vec<Edit> {
        let buffer = self.buffer.read(cx);
        let edits = buffer
            .edits_since(self.last_sync.lock().clone())
            .map(Into::into)
            .collect::<Vec<_>>();
        *self.last_sync.lock() = buffer.version();
        if edits.is_empty() {
            Vec::new()
        } else {
            self.apply_edits(edits, cx)
        }
    }

    fn check_invariants(&self, cx: &AppContext) {
        #[cfg(debug_assertions)]
        {
            let buffer = self.buffer.read(cx);
            assert_eq!(
                self.transforms.lock().summary().input.bytes,
                buffer.len(),
                "transform tree does not match buffer's length"
            );
        }
    }

    fn apply_edits(&self, input_edits: Vec<buffer::Edit>, cx: &AppContext) -> Vec<Edit> {
        let buffer = self.buffer.read(cx).snapshot();
        let mut input_edits_iter = input_edits.iter().cloned().peekable();

        let mut new_transforms = SumTree::new();
        let mut transforms = self.transforms.lock();
        let mut cursor = transforms.cursor::<usize, ()>();
        cursor.seek(&0, Bias::Right, &());

        while let Some(mut edit) = input_edits_iter.next() {
            new_transforms.push_tree(cursor.slice(&edit.old_bytes.start, Bias::Left, &()), &());
            edit.new_bytes.start -= edit.old_bytes.start - cursor.seek_start();
            edit.old_bytes.start = *cursor.seek_start();

            cursor.seek(&edit.old_bytes.end, Bias::Right, &());
            cursor.next(&());

            let mut delta = edit.delta();
            loop {
                edit.old_bytes.end = *cursor.seek_start();

                if let Some(next_edit) = input_edits_iter.peek() {
                    if next_edit.old_bytes.start > edit.old_bytes.end {
                        break;
                    }

                    let next_edit = input_edits_iter.next().unwrap();
                    delta += next_edit.delta();

                    if next_edit.old_bytes.end >= edit.old_bytes.end {
                        edit.old_bytes.end = next_edit.old_bytes.end;
                        cursor.seek(&edit.old_bytes.end, Bias::Right, &());
                        cursor.next(&());
                    }
                } else {
                    break;
                }
            }

            edit.new_bytes.end =
                ((edit.new_bytes.start + edit.deleted_bytes()) as isize + delta) as usize;

            let anchor = buffer.anchor_before(edit.new_bytes.start);
            let mut folds_cursor = self.folds.cursor::<_, ()>();
            folds_cursor.seek(&Fold(anchor..Anchor::max()), Bias::Left, &buffer);

            let mut folds = iter::from_fn({
                let buffer = &buffer;
                move || {
                    let item = folds_cursor
                        .item()
                        .map(|f| f.0.start.to_offset(buffer)..f.0.end.to_offset(buffer));
                    folds_cursor.next(buffer);
                    item
                }
            })
            .peekable();

            while folds
                .peek()
                .map_or(false, |fold| fold.start < edit.new_bytes.end)
            {
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
                    let text_summary = buffer.text_summary_for_range(sum.input.bytes..fold.start);
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
                    let chars = output_text.chars().count() as u32;
                    let lines = super::Point::new(0, output_text.len() as u32);
                    new_transforms.push(
                        Transform {
                            summary: TransformSummary {
                                output: TextSummary {
                                    bytes: output_text.len(),
                                    lines,
                                    first_line_chars: chars,
                                    last_line_chars: chars,
                                    longest_row: 0,
                                    longest_row_chars: chars,
                                },
                                input: buffer.text_summary_for_range(fold.start..fold.end),
                            },
                            output_text: Some(output_text),
                        },
                        &(),
                    );
                }
            }

            let sum = new_transforms.summary();
            if sum.input.bytes < edit.new_bytes.end {
                let text_summary =
                    buffer.text_summary_for_range(sum.input.bytes..edit.new_bytes.end);
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
            let text_summary = buffer.text_summary();
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

        let mut output_edits = Vec::with_capacity(input_edits.len());
        {
            let mut old_transforms = transforms.cursor::<usize, OutputOffset>();
            let mut new_transforms = new_transforms.cursor::<usize, OutputOffset>();

            for mut edit in input_edits {
                old_transforms.seek(&edit.old_bytes.start, Bias::Left, &());
                if old_transforms.item().map_or(false, |t| t.is_fold()) {
                    edit.old_bytes.start = *old_transforms.seek_start();
                }
                let old_start = old_transforms.sum_start().0
                    + (edit.old_bytes.start - old_transforms.seek_start());

                old_transforms.seek_forward(&edit.old_bytes.end, Bias::Right, &());
                if old_transforms.item().map_or(false, |t| t.is_fold()) {
                    old_transforms.next(&());
                    edit.old_bytes.end = *old_transforms.seek_start();
                }
                let old_end = old_transforms.sum_start().0
                    + (edit.old_bytes.end - old_transforms.seek_start());

                new_transforms.seek(&edit.new_bytes.start, Bias::Left, &());
                if new_transforms.item().map_or(false, |t| t.is_fold()) {
                    edit.new_bytes.start = *new_transforms.seek_start();
                }
                let new_start = new_transforms.sum_start().0
                    + (edit.new_bytes.start - new_transforms.seek_start());

                new_transforms.seek_forward(&edit.new_bytes.end, Bias::Right, &());
                if new_transforms.item().map_or(false, |t| t.is_fold()) {
                    new_transforms.next(&());
                    edit.new_bytes.end = *new_transforms.seek_start();
                }
                let new_end = new_transforms.sum_start().0
                    + (edit.new_bytes.end - new_transforms.seek_start());

                output_edits.push(Edit {
                    old_bytes: OutputOffset(old_start)..OutputOffset(old_end),
                    new_bytes: OutputOffset(new_start)..OutputOffset(new_end),
                });
            }

            consolidate_output_edits(&mut output_edits);
        }

        *transforms = new_transforms;
        self.version.fetch_add(1, SeqCst);
        output_edits
    }
}

#[derive(Clone)]
pub struct Snapshot {
    transforms: SumTree<Transform>,
    folds: SumTree<Fold>,
    buffer: buffer::Snapshot,
    pub version: usize,
}

impl Snapshot {
    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks_at(OutputOffset(0)).collect()
    }

    pub fn text_summary(&self) -> TextSummary {
        self.transforms.summary().output
    }

    pub fn text_summary_for_range(&self, range: Range<OutputPoint>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self.transforms.cursor::<OutputPoint, InputPoint>();
        cursor.seek(&range.start, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let start_in_transform = range.start.0 - cursor.seek_start().0;
            let end_in_transform =
                cmp::min(range.end, cursor.seek_end(&())).0 - cursor.seek_start().0;
            if let Some(output_text) = transform.output_text {
                summary = TextSummary::from(
                    &output_text
                        [start_in_transform.column as usize..end_in_transform.column as usize],
                );
            } else {
                let input_start = *cursor.sum_start() + start_in_transform;
                let input_end = *cursor.sum_start() + end_in_transform;
                summary = self.buffer.text_summary_for_range(input_start..input_end);
            }
        }

        if range.end > cursor.seek_end(&()) {
            cursor.next(&());
            summary += &cursor
                .summary::<TransformSummary>(&range.end, Bias::Right, &())
                .output;
            if let Some(transform) = cursor.item() {
                let end_in_transform = range.end.0 - cursor.seek_start().0;
                if let Some(output_text) = transform.output_text {
                    summary += TextSummary::from(&output_text[..end_in_transform.column as usize]);
                } else {
                    let input_start = *cursor.sum_start();
                    let input_end = *cursor.sum_start() + end_in_transform;
                    summary += self.buffer.text_summary_for_range(input_start..input_end);
                }
            }
        }

        summary
    }

    pub fn len(&self) -> usize {
        self.transforms.summary().output.bytes
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let line_start = self.to_output_offset(OutputPoint::new(row, 0)).0;
        let line_end = if row >= self.max_point().row() {
            self.len()
        } else {
            self.to_output_offset(OutputPoint::new(row + 1, 0)).0 - 1
        };
        (line_end - line_start) as u32
    }

    pub fn input_rows(&self, start_row: u32) -> InputRows {
        if start_row > self.transforms.summary().output.lines.row {
            panic!("invalid display row {}", start_row);
        }

        let output_point = OutputPoint::new(start_row, 0);
        let mut cursor = self.transforms.cursor();
        cursor.seek(&output_point, Bias::Left, &());
        InputRows {
            output_point,
            cursor,
        }
    }

    pub fn max_point(&self) -> OutputPoint {
        OutputPoint(self.transforms.summary().output.lines)
    }

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
        let mut folds = intersecting_folds(&self.buffer, &self.folds, range);
        iter::from_fn(move || {
            let item = folds.item().map(|f| &f.0);
            folds.next(&self.buffer);
            item
        })
    }

    pub fn intersects_fold<T>(&self, offset: T) -> bool
    where
        T: ToOffset,
    {
        let offset = offset.to_offset(&self.buffer);
        let mut cursor = self.transforms.cursor::<usize, ()>();
        cursor.seek(&offset, Bias::Right, &());
        cursor.item().map_or(false, |t| t.output_text.is_some())
    }

    pub fn is_line_folded(&self, output_row: u32) -> bool {
        let mut cursor = self.transforms.cursor::<OutputPoint, ()>();
        cursor.seek(&OutputPoint::new(output_row, 0), Bias::Right, &());
        while let Some(transform) = cursor.item() {
            if transform.output_text.is_some() {
                return true;
            }
            if cursor.seek_end(&()).row() == output_row {
                cursor.next(&())
            } else {
                break;
            }
        }
        false
    }

    pub fn chunks_at(&self, offset: OutputOffset) -> Chunks {
        let mut transform_cursor = self.transforms.cursor::<OutputOffset, usize>();
        transform_cursor.seek(&offset, Bias::Right, &());
        let overshoot = offset.0 - transform_cursor.seek_start().0;
        let input_offset = transform_cursor.sum_start() + overshoot;
        Chunks {
            transform_cursor,
            input_offset,
            input_chunks: self.buffer.text_for_range(input_offset..self.buffer.len()),
        }
    }

    pub fn highlighted_chunks(&mut self, range: Range<OutputOffset>) -> HighlightedChunks {
        let mut transform_cursor = self.transforms.cursor::<OutputOffset, usize>();

        transform_cursor.seek(&range.end, Bias::Right, &());
        let overshoot = range.end.0 - transform_cursor.seek_start().0;
        let input_end = transform_cursor.sum_start() + overshoot;

        transform_cursor.seek(&range.start, Bias::Right, &());
        let overshoot = range.start.0 - transform_cursor.seek_start().0;
        let input_start = transform_cursor.sum_start() + overshoot;

        HighlightedChunks {
            transform_cursor,
            input_offset: input_start,
            input_chunks: self
                .buffer
                .highlighted_text_for_range(input_start..input_end),
            input_chunk: None,
        }
    }

    pub fn chars_at<'a>(&'a self, point: OutputPoint) -> impl Iterator<Item = char> + 'a {
        let offset = self.to_output_offset(point);
        self.chunks_at(offset).flat_map(str::chars)
    }

    pub fn to_output_offset(&self, point: OutputPoint) -> OutputOffset {
        let mut cursor = self.transforms.cursor::<OutputPoint, TransformSummary>();
        cursor.seek(&point, Bias::Right, &());
        let overshoot = point.0 - cursor.sum_start().output.lines;
        let mut offset = cursor.sum_start().output.bytes;
        if !overshoot.is_zero() {
            let transform = cursor.item().expect("display point out of range");
            assert!(transform.output_text.is_none());
            let end_input_offset = self
                .buffer
                .to_offset(cursor.sum_start().input.lines + overshoot);
            offset += end_input_offset - cursor.sum_start().input.bytes;
        }
        OutputOffset(offset)
    }

    pub fn to_input_offset(&self, point: OutputPoint) -> usize {
        let mut cursor = self.transforms.cursor::<OutputPoint, InputPoint>();
        cursor.seek(&point, Bias::Right, &());
        let overshoot = point.0 - cursor.seek_start().0;
        self.buffer.to_offset(*cursor.sum_start() + overshoot)
    }

    pub fn to_input_point(&self, output_point: OutputPoint) -> InputPoint {
        let mut cursor = self.transforms.cursor::<OutputPoint, InputPoint>();
        cursor.seek(&output_point, Bias::Right, &());
        let overshoot = output_point.0 - cursor.seek_start().0;
        *cursor.sum_start() + overshoot
    }

    pub fn to_output_point(&self, point: InputPoint) -> OutputPoint {
        let mut cursor = self.transforms.cursor::<InputPoint, OutputPoint>();
        cursor.seek(&point, Bias::Right, &());
        let overshoot = point - cursor.seek_start();
        OutputPoint(cmp::min(
            cursor.sum_start().0 + overshoot,
            cursor.end(&()).0,
        ))
    }

    #[cfg(test)]
    pub fn clip_offset(&self, offset: OutputOffset, bias: Bias) -> OutputOffset {
        let mut cursor = self.transforms.cursor::<OutputOffset, usize>();
        cursor.seek(&offset, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let transform_start = cursor.seek_start().0;
            if transform.output_text.is_some() {
                if offset.0 == transform_start || matches!(bias, Bias::Left) {
                    OutputOffset(transform_start)
                } else {
                    OutputOffset(cursor.seek_end(&()).0)
                }
            } else {
                let overshoot = offset.0 - transform_start;
                let input_offset = cursor.sum_start() + overshoot;
                let clipped_input_offset = self.buffer.clip_offset(input_offset, bias);
                OutputOffset(
                    (offset.0 as isize + (clipped_input_offset as isize - input_offset as isize))
                        as usize,
                )
            }
        } else {
            OutputOffset(self.transforms.summary().output.bytes)
        }
    }

    pub fn clip_point(&self, point: OutputPoint, bias: Bias) -> OutputPoint {
        let mut cursor = self.transforms.cursor::<OutputPoint, InputPoint>();
        cursor.seek(&point, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let transform_start = cursor.seek_start().0;
            if transform.output_text.is_some() {
                if point.0 == transform_start || matches!(bias, Bias::Left) {
                    OutputPoint(transform_start)
                } else {
                    OutputPoint(cursor.seek_end(&()).0)
                }
            } else {
                let overshoot = point.0 - transform_start;
                let input_position = *cursor.sum_start() + overshoot;
                let clipped_input_position = self.buffer.clip_point(input_position, bias);
                OutputPoint::new(
                    point.row(),
                    ((point.column() as i32) + clipped_input_position.column as i32
                        - input_position.column as i32) as u32,
                )
            }
        } else {
            OutputPoint(self.transforms.summary().output.lines)
        }
    }
}

fn intersecting_folds<'a, T>(
    buffer: &'a buffer::Snapshot,
    folds: &'a SumTree<Fold>,
    range: Range<T>,
) -> FilterCursor<'a, impl 'a + Fn(&FoldSummary) -> bool, Fold, usize>
where
    T: ToOffset,
{
    let start = buffer.anchor_before(range.start.to_offset(buffer));
    let end = buffer.anchor_after(range.end.to_offset(buffer));
    folds.filter::<_, usize>(
        move |summary| {
            start.cmp(&summary.max_end, buffer).unwrap() == Ordering::Less
                && end.cmp(&summary.min_start, buffer).unwrap() == Ordering::Greater
        },
        buffer,
    )
}

fn consolidate_input_edits(edits: &mut Vec<buffer::Edit>) {
    edits.sort_unstable_by(|a, b| {
        a.old_bytes
            .start
            .cmp(&b.old_bytes.start)
            .then_with(|| b.old_bytes.end.cmp(&a.old_bytes.end))
    });

    let mut i = 1;
    while i < edits.len() {
        let edit = edits[i].clone();
        let prev_edit = &mut edits[i - 1];
        if prev_edit.old_bytes.end >= edit.old_bytes.start {
            prev_edit.old_bytes.end = prev_edit.old_bytes.end.max(edit.old_bytes.end);
            prev_edit.new_bytes.start = prev_edit.new_bytes.start.min(edit.new_bytes.start);
            prev_edit.new_bytes.end = prev_edit.new_bytes.end.max(edit.new_bytes.end);
            edits.remove(i);
            continue;
        }
        i += 1;
    }
}

fn consolidate_output_edits(edits: &mut Vec<Edit>) {
    edits.sort_unstable_by(|a, b| {
        a.old_bytes
            .start
            .cmp(&b.old_bytes.start)
            .then_with(|| b.old_bytes.end.cmp(&a.old_bytes.end))
    });

    let mut i = 1;
    while i < edits.len() {
        let edit = edits[i].clone();
        let prev_edit = &mut edits[i - 1];
        if prev_edit.old_bytes.end >= edit.old_bytes.start {
            prev_edit.old_bytes.end = prev_edit.old_bytes.end.max(edit.old_bytes.end);
            prev_edit.new_bytes.start = prev_edit.new_bytes.start.min(edit.new_bytes.start);
            prev_edit.new_bytes.end = prev_edit.new_bytes.end.max(edit.new_bytes.end);
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

impl<'a> sum_tree::Dimension<'a, TransformSummary> for TransformSummary {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        sum_tree::Summary::add_summary(self, summary, &());
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
    type Context = buffer::Snapshot;

    fn add_summary(&mut self, other: &Self, buffer: &buffer::Snapshot) {
        if other.min_start.cmp(&self.min_start, buffer).unwrap() == Ordering::Less {
            self.min_start = other.min_start.clone();
        }
        if other.max_end.cmp(&self.max_end, buffer).unwrap() == Ordering::Greater {
            self.max_end = other.max_end.clone();
        }

        #[cfg(debug_assertions)]
        {
            let start_comparison = self.start.cmp(&other.start, buffer).unwrap();
            assert!(start_comparison <= Ordering::Equal);
            if start_comparison == Ordering::Equal {
                assert!(self.end.cmp(&other.end, buffer).unwrap() >= Ordering::Equal);
            }
        }

        self.start = other.start.clone();
        self.end = other.end.clone();
        self.count += other.count;
    }
}

impl<'a> sum_tree::Dimension<'a, FoldSummary> for Fold {
    fn add_summary(&mut self, summary: &'a FoldSummary, _: &buffer::Snapshot) {
        self.0.start = summary.start.clone();
        self.0.end = summary.end.clone();
    }
}

impl<'a> sum_tree::SeekDimension<'a, FoldSummary> for Fold {
    fn cmp(&self, other: &Self, buffer: &buffer::Snapshot) -> Ordering {
        self.0.cmp(&other.0, buffer).unwrap()
    }
}

impl<'a> sum_tree::Dimension<'a, FoldSummary> for usize {
    fn add_summary(&mut self, summary: &'a FoldSummary, _: &buffer::Snapshot) {
        *self += summary.count;
    }
}

pub struct InputRows<'a> {
    cursor: Cursor<'a, Transform, OutputPoint, InputPoint>,
    output_point: OutputPoint,
}

impl<'a> Iterator for InputRows<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.output_point > self.cursor.seek_end(&()) {
            self.cursor.next(&());
            if self.cursor.item().is_none() {
                // TODO: Return a bool from next?
                break;
            }
        }

        if self.cursor.item().is_some() {
            let overshoot = self.output_point - *self.cursor.seek_start();
            let input_point = *self.cursor.sum_start() + overshoot.0;
            *self.output_point.row_mut() += 1;
            Some(input_point.row)
        } else {
            None
        }
    }
}

pub struct Chunks<'a> {
    transform_cursor: Cursor<'a, Transform, OutputOffset, usize>,
    input_chunks: buffer::Chunks<'a>,
    input_offset: usize,
}

impl<'a> Iterator for Chunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let transform = if let Some(item) = self.transform_cursor.item() {
            item
        } else {
            return None;
        };

        // If we're in a fold, then return the fold's display text and
        // advance the transform and buffer cursors to the end of the fold.
        if let Some(output_text) = transform.output_text {
            self.input_offset += transform.summary.input.bytes;
            self.input_chunks.seek(self.input_offset);

            while self.input_offset >= self.transform_cursor.end(&())
                && self.transform_cursor.item().is_some()
            {
                self.transform_cursor.next(&());
            }

            return Some(output_text);
        }

        // Otherwise, take a chunk from the buffer's text.
        if let Some(mut chunk) = self.input_chunks.peek() {
            let offset_in_chunk = self.input_offset - self.input_chunks.offset();
            chunk = &chunk[offset_in_chunk..];

            // Truncate the chunk so that it ends at the next fold.
            let region_end = self.transform_cursor.end(&()) - self.input_offset;
            if chunk.len() >= region_end {
                chunk = &chunk[0..region_end];
                self.transform_cursor.next(&());
            } else {
                self.input_chunks.next();
            }

            self.input_offset += chunk.len();
            return Some(chunk);
        }

        None
    }
}

pub struct HighlightedChunks<'a> {
    transform_cursor: Cursor<'a, Transform, OutputOffset, usize>,
    input_chunks: buffer::HighlightedChunks<'a>,
    input_chunk: Option<(usize, &'a str, StyleId)>,
    input_offset: usize,
}

impl<'a> Iterator for HighlightedChunks<'a> {
    type Item = (&'a str, StyleId);

    fn next(&mut self) -> Option<Self::Item> {
        let transform = if let Some(item) = self.transform_cursor.item() {
            item
        } else {
            return None;
        };

        // If we're in a fold, then return the fold's display text and
        // advance the transform and buffer cursors to the end of the fold.
        if let Some(output_text) = transform.output_text {
            self.input_chunk.take();
            self.input_offset += transform.summary.input.bytes;
            self.input_chunks.seek(self.input_offset);

            while self.input_offset >= self.transform_cursor.end(&())
                && self.transform_cursor.item().is_some()
            {
                self.transform_cursor.next(&());
            }

            return Some((output_text, StyleId::default()));
        }

        // Retrieve a chunk from the current location in the buffer.
        if self.input_chunk.is_none() {
            let chunk_offset = self.input_chunks.offset();
            self.input_chunk = self
                .input_chunks
                .next()
                .map(|(chunk, capture_ix)| (chunk_offset, chunk, capture_ix));
        }

        // Otherwise, take a chunk from the buffer's text.
        if let Some((chunk_offset, mut chunk, capture_ix)) = self.input_chunk {
            let offset_in_chunk = self.input_offset - chunk_offset;
            chunk = &chunk[offset_in_chunk..];

            // Truncate the chunk so that it ends at the next fold.
            let region_end = self.transform_cursor.end(&()) - self.input_offset;
            if chunk.len() >= region_end {
                chunk = &chunk[0..region_end];
                self.transform_cursor.next(&());
            } else {
                self.input_chunk.take();
            }

            self.input_offset += chunk.len();
            return Some((chunk, capture_ix));
        }

        None
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for OutputPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.output.lines;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct OutputOffset(pub usize);

impl OutputOffset {
    pub fn to_output_point(&self, snapshot: &Snapshot) -> OutputPoint {
        let mut cursor = snapshot
            .transforms
            .cursor::<OutputOffset, TransformSummary>();
        cursor.seek(self, Bias::Right, &());
        let overshoot = if cursor.item().map_or(true, |t| t.is_fold()) {
            InputPoint::new(0, (self.0 - cursor.seek_start().0) as u32)
        } else {
            let input_offset = cursor.sum_start().input.bytes + self.0 - cursor.seek_start().0;
            let input_point = snapshot.buffer.to_point(input_offset);
            input_point - cursor.sum_start().input.lines
        };
        OutputPoint(cursor.sum_start().output.lines + overshoot)
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for OutputOffset {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.output.bytes;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for InputPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += &summary.input.lines;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for usize {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += &summary.input.bytes;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edit {
    pub old_bytes: Range<OutputOffset>,
    pub new_bytes: Range<OutputOffset>,
}

impl Edit {
    pub fn delta(&self) -> isize {
        self.inserted_bytes() as isize - self.deleted_bytes() as isize
    }

    pub fn deleted_bytes(&self) -> usize {
        self.old_bytes.end.0 - self.old_bytes.start.0
    }

    pub fn inserted_bytes(&self) -> usize {
        self.new_bytes.end.0 - self.new_bytes.start.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::buffer::ToPoint;
    use crate::test::sample_text;
    use std::mem;

    #[gpui::test]
    fn test_basic_folds(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(5, 6), cx));
        let mut map = FoldMap::new(buffer.clone(), cx.as_ref());

        let (mut writer, _, _) = map.write(cx.as_ref());
        writer.fold(
            vec![
                InputPoint::new(0, 2)..InputPoint::new(2, 2),
                InputPoint::new(2, 4)..InputPoint::new(4, 1),
            ],
            cx.as_ref(),
        );
        let (snapshot2, edits) = map.read(cx.as_ref());
        assert_eq!(snapshot2.text(), "aa…cc…eeeee");
        assert_eq!(
            edits,
            &[
                Edit {
                    old_bytes: OutputOffset(2)..OutputOffset(16),
                    new_bytes: OutputOffset(2)..OutputOffset(5),
                },
                Edit {
                    old_bytes: OutputOffset(7)..OutputOffset(18),
                    new_bytes: OutputOffset(7)..OutputOffset(10)
                },
            ]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                vec![
                    InputPoint::new(0, 0)..InputPoint::new(0, 1),
                    InputPoint::new(2, 3)..InputPoint::new(2, 3),
                ],
                "123",
                cx,
            );
        });
        let (snapshot3, edits) = map.read(cx.as_ref());
        assert_eq!(snapshot3.text(), "123a…c123c…eeeee");
        assert_eq!(
            edits,
            &[
                Edit {
                    old_bytes: OutputOffset(0)..OutputOffset(1),
                    new_bytes: OutputOffset(0)..OutputOffset(3),
                },
                Edit {
                    old_bytes: OutputOffset(8)..OutputOffset(8),
                    new_bytes: OutputOffset(8)..OutputOffset(11),
                },
            ]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                vec![InputPoint::new(2, 6)..InputPoint::new(4, 3)],
                "456",
                cx,
            )
        });
        let (snapshot4, _) = map.read(cx.as_ref());
        assert_eq!(snapshot4.text(), "123a…c123456eee");

        let (mut writer, _, _) = map.write(cx.as_ref());
        writer.unfold(
            Some(InputPoint::new(0, 4)..InputPoint::new(0, 5)),
            cx.as_ref(),
        );
        let (snapshot5, _) = map.read(cx.as_ref());
        assert_eq!(snapshot5.text(), "123aaaaa\nbbbbbb\nccc123456eee");
    }

    #[gpui::test]
    fn test_adjacent_folds(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "abcdefghijkl", cx));

        {
            let mut map = FoldMap::new(buffer.clone(), cx.as_ref());

            let (mut writer, _, _) = map.write(cx.as_ref());
            writer.fold(vec![5..8], cx.as_ref());
            let (snapshot, _) = map.read(cx.as_ref());
            assert_eq!(snapshot.text(), "abcde…ijkl");

            // Create an fold adjacent to the start of the first fold.
            let (mut writer, _, _) = map.write(cx.as_ref());
            writer.fold(vec![0..1, 2..5], cx.as_ref());
            let (snapshot, _) = map.read(cx.as_ref());
            assert_eq!(snapshot.text(), "…b…ijkl");

            // Create an fold adjacent to the end of the first fold.
            let (mut writer, _, _) = map.write(cx.as_ref());
            writer.fold(vec![11..11, 8..10], cx.as_ref());
            let (snapshot, _) = map.read(cx.as_ref());
            assert_eq!(snapshot.text(), "…b…kl");
        }

        {
            let mut map = FoldMap::new(buffer.clone(), cx.as_ref());

            // Create two adjacent folds.
            let (mut writer, _, _) = map.write(cx.as_ref());
            writer.fold(vec![0..2, 2..5], cx.as_ref());
            let (snapshot, _) = map.read(cx.as_ref());
            assert_eq!(snapshot.text(), "…fghijkl");

            // Edit within one of the folds.
            buffer.update(cx, |buffer, cx| buffer.edit(vec![0..1], "12345", cx));
            let (snapshot, _) = map.read(cx.as_ref());
            assert_eq!(snapshot.text(), "12345…fghijkl");
        }
    }

    #[gpui::test]
    fn test_overlapping_folds(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(5, 6), cx));
        let mut map = FoldMap::new(buffer.clone(), cx.as_ref());
        let (mut writer, _, _) = map.write(cx.as_ref());
        writer.fold(
            vec![
                InputPoint::new(0, 2)..InputPoint::new(2, 2),
                InputPoint::new(0, 4)..InputPoint::new(1, 0),
                InputPoint::new(1, 2)..InputPoint::new(3, 2),
                InputPoint::new(3, 1)..InputPoint::new(4, 1),
            ],
            cx.as_ref(),
        );
        let (snapshot, _) = map.read(cx.as_ref());
        assert_eq!(snapshot.text(), "aa…eeeee");
    }

    #[gpui::test]
    fn test_merging_folds_via_edit(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(5, 6), cx));
        let mut map = FoldMap::new(buffer.clone(), cx.as_ref());

        let (mut writer, _, _) = map.write(cx.as_ref());
        writer.fold(
            vec![
                InputPoint::new(0, 2)..InputPoint::new(2, 2),
                InputPoint::new(3, 1)..InputPoint::new(4, 1),
            ],
            cx.as_ref(),
        );
        let (snapshot, _) = map.read(cx.as_ref());
        assert_eq!(snapshot.text(), "aa…cccc\nd…eeeee");

        buffer.update(cx, |buffer, cx| {
            buffer.edit(Some(InputPoint::new(2, 2)..InputPoint::new(3, 1)), "", cx)
        });
        let (snapshot, _) = map.read(cx.as_ref());
        assert_eq!(snapshot.text(), "aa…eeeee");
    }

    #[gpui::test]
    fn test_folds_in_range(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(5, 6), cx));
        let mut map = FoldMap::new(buffer.clone(), cx.as_ref());
        let buffer = buffer.read(cx);

        let (mut writer, _, _) = map.write(cx.as_ref());
        writer.fold(
            vec![
                InputPoint::new(0, 2)..InputPoint::new(2, 2),
                InputPoint::new(0, 4)..InputPoint::new(1, 0),
                InputPoint::new(1, 2)..InputPoint::new(3, 2),
                InputPoint::new(3, 1)..InputPoint::new(4, 1),
            ],
            cx.as_ref(),
        );
        let (snapshot, _) = map.read(cx.as_ref());
        let fold_ranges = snapshot
            .folds_in_range(InputPoint::new(1, 0)..InputPoint::new(1, 3))
            .map(|fold| fold.start.to_point(buffer)..fold.end.to_point(buffer))
            .collect::<Vec<_>>();
        assert_eq!(
            fold_ranges,
            vec![
                InputPoint::new(0, 2)..InputPoint::new(2, 2),
                InputPoint::new(1, 2)..InputPoint::new(3, 2)
            ]
        );
    }

    #[gpui::test]
    fn test_random_folds(cx: &mut gpui::MutableAppContext) {
        use crate::editor::ToPoint;
        use crate::util::RandomCharIter;
        use rand::prelude::*;
        use std::env;
        use Bias::{Left, Right};

        let iterations = env::var("ITERATIONS")
            .map(|i| i.parse().expect("invalid `ITERATIONS` variable"))
            .unwrap_or(100);
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);
        let seed_range = if let Ok(seed) = env::var("SEED") {
            let seed = seed.parse().expect("invalid `SEED` variable");
            seed..seed + 1
        } else {
            0..iterations
        };

        for seed in seed_range {
            dbg!(seed);
            let mut rng = StdRng::seed_from_u64(seed);

            let buffer = cx.add_model(|cx| {
                let len = rng.gen_range(0..10);
                let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
                Buffer::new(0, text, cx)
            });
            let mut map = FoldMap::new(buffer.clone(), cx.as_ref());

            let (mut initial_snapshot, _) = map.read(cx.as_ref());
            let mut snapshot_edits = Vec::new();

            for _ in 0..operations {
                log::info!("text: {:?}", buffer.read(cx).text());
                match rng.gen_range(0..=100) {
                    0..=34 => {
                        let buffer = buffer.read(cx);
                        let mut to_fold = Vec::new();
                        for _ in 0..rng.gen_range(1..=2) {
                            let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                            let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                            to_fold.push(start..end);
                        }
                        log::info!("folding {:?}", to_fold);
                        let (mut writer, snapshot, edits) = map.write(cx.as_ref());
                        snapshot_edits.push((snapshot, edits));
                        let (snapshot, edits) = writer.fold(to_fold, cx.as_ref());
                        snapshot_edits.push((snapshot, edits));
                    }
                    35..=59 if !map.folds.is_empty() => {
                        let buffer = buffer.read(cx);
                        let mut to_unfold = Vec::new();
                        for _ in 0..rng.gen_range(1..=3) {
                            let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                            let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                            to_unfold.push(start..end);
                        }
                        log::info!("unfolding {:?}", to_unfold);
                        let (mut writer, snapshot, edits) = map.write(cx.as_ref());
                        snapshot_edits.push((snapshot, edits));
                        let (snapshot, edits) = writer.fold(to_unfold, cx.as_ref());
                        snapshot_edits.push((snapshot, edits));
                    }
                    _ => {
                        let edits = buffer.update(cx, |buffer, cx| {
                            let start_version = buffer.version.clone();
                            let edit_count = rng.gen_range(1..=5);
                            buffer.randomly_edit(&mut rng, edit_count, cx);
                            buffer.edits_since(start_version).collect::<Vec<_>>()
                        });
                        log::info!("editing {:?}", edits);
                    }
                }

                let buffer = map.buffer.read(cx).snapshot();
                let mut expected_text: String = buffer.text().into();
                let mut expected_input_rows = Vec::new();
                let mut next_row = buffer.max_point().row;
                for fold_range in map.merged_fold_ranges(cx.as_ref()).into_iter().rev() {
                    let fold_start = buffer.point_for_offset(fold_range.start).unwrap();
                    let fold_end = buffer.point_for_offset(fold_range.end).unwrap();
                    expected_input_rows.extend((fold_end.row + 1..=next_row).rev());
                    next_row = fold_start.row;

                    expected_text.replace_range(fold_range.start..fold_range.end, "…");
                }
                expected_input_rows.extend((0..=next_row).rev());
                expected_input_rows.reverse();

                let (snapshot, edits) = map.read(cx.as_ref());
                assert_eq!(snapshot.text(), expected_text);
                snapshot_edits.push((snapshot.clone(), edits));

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
                let mut output_point = OutputPoint::new(0, 0);
                let mut output_offset = OutputOffset(0);
                let mut char_column = 0;
                for c in expected_text.chars() {
                    let input_point = snapshot.to_input_point(output_point);
                    let input_offset = input_point.to_offset(&buffer);
                    assert_eq!(
                        snapshot.to_output_point(input_point),
                        output_point,
                        "to_output_point({:?})",
                        input_point,
                    );
                    assert_eq!(
                        snapshot.to_input_offset(output_point),
                        input_offset,
                        "to_input_offset({:?})",
                        output_point,
                    );
                    assert_eq!(
                        snapshot.to_output_offset(output_point),
                        output_offset,
                        "to_output_offset({:?})",
                        output_point,
                    );

                    if c == '\n' {
                        *output_point.row_mut() += 1;
                        *output_point.column_mut() = 0;
                        char_column = 0;
                    } else {
                        *output_point.column_mut() += c.len_utf8() as u32;
                        char_column += 1;
                    }
                    output_offset.0 += c.len_utf8();
                    if char_column > longest_char_column {
                        panic!(
                            "invalid longest row {:?} (chars {}), found row {:?} (chars: {})",
                            longest_row,
                            longest_char_column,
                            output_point.row(),
                            char_column
                        );
                    }
                }

                for _ in 0..5 {
                    let offset = snapshot
                        .clip_offset(OutputOffset(rng.gen_range(0..=snapshot.len())), Bias::Right);
                    assert_eq!(
                        snapshot.chunks_at(offset).collect::<String>(),
                        &expected_text[offset.0..],
                    );
                }

                for (idx, input_row) in expected_input_rows.iter().enumerate() {
                    let output_row = snapshot
                        .to_output_point(InputPoint::new(*input_row, 0))
                        .row();
                    assert_eq!(
                        snapshot.input_rows(output_row).collect::<Vec<_>>(),
                        expected_input_rows[idx..],
                    );
                }

                for fold_range in map.merged_fold_ranges(cx.as_ref()) {
                    let output_point = snapshot.to_output_point(fold_range.start.to_point(&buffer));
                    assert!(snapshot.is_line_folded(output_point.row()));
                }

                for _ in 0..5 {
                    let end = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Right);
                    let start = buffer.clip_offset(rng.gen_range(0..=end), Left);
                    let expected_folds = map
                        .folds
                        .items(&buffer)
                        .into_iter()
                        .filter(|fold| {
                            let start = buffer.anchor_before(start);
                            let end = buffer.anchor_after(end);
                            start.cmp(&fold.0.end, &buffer).unwrap() == Ordering::Less
                                && end.cmp(&fold.0.start, &buffer).unwrap() == Ordering::Greater
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
                        snapshot.clip_point(OutputPoint::new(start_row, start_column), Bias::Left);
                    let mut end =
                        snapshot.clip_point(OutputPoint::new(end_row, end_column), Bias::Right);
                    if start > end {
                        mem::swap(&mut start, &mut end);
                    }

                    let lines = start..end;
                    let bytes = snapshot.to_output_offset(start)..snapshot.to_output_offset(end);
                    assert_eq!(
                        snapshot.text_summary_for_range(lines),
                        TextSummary::from(&text[bytes.start.0..bytes.end.0])
                    )
                }

                let mut text = initial_snapshot.text();
                for (snapshot, edits) in snapshot_edits.drain(..) {
                    let new_text = snapshot.text();
                    let mut delta = 0isize;
                    for edit in edits {
                        let old_bytes = ((edit.old_bytes.start.0 as isize) + delta) as usize
                            ..((edit.old_bytes.end.0 as isize) + delta) as usize;
                        let new_bytes = edit.new_bytes.start.0..edit.new_bytes.end.0;
                        delta += edit.delta();
                        text.replace_range(old_bytes, &new_text[new_bytes]);
                    }

                    assert_eq!(text, new_text);
                    initial_snapshot = snapshot;
                }
            }
        }
    }

    #[gpui::test]
    fn test_input_rows(cx: &mut gpui::MutableAppContext) {
        let text = sample_text(6, 6) + "\n";
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));

        let mut map = FoldMap::new(buffer.clone(), cx.as_ref());

        let (mut writer, _, _) = map.write(cx.as_ref());
        writer.fold(
            vec![
                InputPoint::new(0, 2)..InputPoint::new(2, 2),
                InputPoint::new(3, 1)..InputPoint::new(4, 1),
            ],
            cx.as_ref(),
        );

        let (snapshot, _) = map.read(cx.as_ref());
        assert_eq!(snapshot.text(), "aa…cccc\nd…eeeee\nffffff\n");
        assert_eq!(snapshot.input_rows(0).collect::<Vec<_>>(), [0, 3, 5, 6]);
        assert_eq!(snapshot.input_rows(3).collect::<Vec<_>>(), [6]);
    }

    impl FoldMap {
        fn merged_fold_ranges(&self, cx: &AppContext) -> Vec<Range<usize>> {
            let buffer = self.buffer.read(cx).snapshot();
            let mut folds = self.folds.items(&buffer);
            // Ensure sorting doesn't change how folds get merged and displayed.
            folds.sort_by(|a, b| a.0.cmp(&b.0, &buffer).unwrap());
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
    }
}

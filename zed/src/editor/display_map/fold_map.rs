use super::{
    buffer::{AnchorRangeExt, TextSummary},
    Anchor, Buffer, DisplayPoint, Point, ToOffset,
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
    collections::VecDeque,
    iter,
    ops::Range,
    sync::Arc,
};

pub struct FoldMap {
    buffer: ModelHandle<Buffer>,
    transforms: Mutex<SumTree<Transform>>,
    folds: SumTree<Fold>,
    last_sync: Mutex<time::Global>,
    history: Arc<Mutex<FoldMapHistory>>,
}

#[derive(Default)]
struct FoldMapHistory {
    edits: VecDeque<Edit>,
    oldest_edit_version: usize,
    snapshot_versions: Vec<usize>,
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
                        buffer: buffer.text_summary(),
                        display: buffer.text_summary(),
                    },
                    display_text: None,
                },
                &(),
            )),
            last_sync: Mutex::new(buffer.version()),
            history: Default::default(),
        }
    }

    pub fn snapshot(&self, cx: &AppContext) -> FoldMapSnapshot {
        self.sync(cx);
        let mut history = self.history.lock();
        let version = history.oldest_edit_version + history.edits.len();
        history.snapshot_versions.push(version);
        FoldMapSnapshot {
            transforms: self.transforms.lock().clone(),
            folds: self.folds.clone(),
            buffer: self.buffer.read(cx).snapshot(),
            history: self.history.clone(),
            version,
        }
    }

    pub fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &AppContext,
    ) {
        self.sync(cx);

        let mut edits = Vec::new();
        let mut folds = Vec::new();
        let buffer = self.buffer.read(cx).snapshot();
        for range in ranges.into_iter() {
            let range = range.start.to_offset(&buffer)..range.end.to_offset(&buffer);
            if range.start != range.end {
                let fold = Fold(buffer.anchor_after(range.start)..buffer.anchor_before(range.end));
                folds.push(fold);
                edits.push(Edit {
                    old_bytes: range.clone(),
                    new_bytes: range.clone(),
                });
            }
        }

        folds.sort_unstable_by(|a, b| sum_tree::SeekDimension::cmp(a, b, &buffer));

        self.folds = {
            let mut new_tree = SumTree::new();
            let mut cursor = self.folds.cursor::<_, ()>();
            for fold in folds {
                new_tree.push_tree(cursor.slice(&fold, Bias::Right, &buffer), &buffer);
                new_tree.push(fold, &buffer);
            }
            new_tree.push_tree(cursor.suffix(&buffer), &buffer);
            new_tree
        };

        self.consolidate_edits(&mut edits);
        self.apply_edits(edits, cx);
    }

    pub fn unfold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        cx: &AppContext,
    ) {
        self.sync(cx);

        let mut edits = Vec::new();
        let mut fold_ixs_to_delete = Vec::new();
        let buffer = self.buffer.read(cx).snapshot();
        for range in ranges.into_iter() {
            // Remove intersecting folds and add their ranges to edits that are passed to apply_edits.
            let mut folds_cursor = intersecting_folds(&buffer, &self.folds, range);
            while let Some(fold) = folds_cursor.item() {
                let offset_range = fold.0.start.to_offset(&buffer)..fold.0.end.to_offset(&buffer);
                edits.push(Edit {
                    old_bytes: offset_range.clone(),
                    new_bytes: offset_range,
                });
                fold_ixs_to_delete.push(*folds_cursor.start());
                folds_cursor.next(&buffer);
            }
        }

        fold_ixs_to_delete.sort_unstable();
        fold_ixs_to_delete.dedup();

        self.folds = {
            let mut cursor = self.folds.cursor::<_, ()>();
            let mut folds = SumTree::new();
            for fold_ix in fold_ixs_to_delete {
                folds.push_tree(cursor.slice(&fold_ix, Bias::Right, &buffer), &buffer);
                cursor.next(&buffer);
            }
            folds.push_tree(cursor.suffix(&buffer), &buffer);
            folds
        };

        self.consolidate_edits(&mut edits);
        self.apply_edits(edits, cx);
    }

    fn sync(&self, cx: &AppContext) {
        let buffer = self.buffer.read(cx);
        let edits = buffer
            .edits_since(self.last_sync.lock().clone())
            .map(Into::into)
            .collect::<Vec<_>>();
        if !edits.is_empty() {
            self.apply_edits(edits, cx);
        }
        *self.last_sync.lock() = buffer.version();
    }

    fn apply_edits(&self, mut edits: Vec<Edit>, cx: &AppContext) {
        let buffer = self.buffer.read(cx).snapshot();
        let mut edits_iter = edits.iter().cloned().peekable();

        let mut new_transforms = SumTree::new();
        let mut transforms = self.transforms.lock();
        let mut cursor = transforms.cursor::<usize, ()>();
        cursor.seek(&0, Bias::Right, &());

        while let Some(mut edit) = edits_iter.next() {
            new_transforms.push_tree(cursor.slice(&edit.old_bytes.start, Bias::Left, &()), &());
            edit.new_bytes.start -= edit.old_bytes.start - cursor.seek_start();
            edit.old_bytes.start = *cursor.seek_start();

            cursor.seek(&edit.old_bytes.end, Bias::Right, &());
            cursor.next(&());

            let mut delta = edit.delta();
            loop {
                edit.old_bytes.end = *cursor.seek_start();

                if let Some(next_edit) = edits_iter.peek() {
                    if next_edit.old_bytes.start > edit.old_bytes.end {
                        break;
                    }

                    let next_edit = edits_iter.next().unwrap();
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

                assert!(fold.start >= sum.buffer.bytes);

                while folds
                    .peek()
                    .map_or(false, |next_fold| next_fold.start <= fold.end)
                {
                    let next_fold = folds.next().unwrap();
                    if next_fold.end > fold.end {
                        fold.end = next_fold.end;
                    }
                }

                if fold.start > sum.buffer.bytes {
                    let text_summary = buffer.text_summary_for_range(sum.buffer.bytes..fold.start);
                    new_transforms.push(
                        Transform {
                            summary: TransformSummary {
                                display: text_summary.clone(),
                                buffer: text_summary,
                            },
                            display_text: None,
                        },
                        &(),
                    );
                }

                if fold.end > fold.start {
                    let display_text = "…";
                    let chars = display_text.chars().count() as u32;
                    let lines = Point::new(0, display_text.len() as u32);
                    new_transforms.push(
                        Transform {
                            summary: TransformSummary {
                                display: TextSummary {
                                    bytes: display_text.len(),
                                    lines,
                                    first_line_chars: chars,
                                    last_line_chars: chars,
                                    longest_row: 0,
                                    longest_row_chars: chars,
                                },
                                buffer: buffer.text_summary_for_range(fold.start..fold.end),
                            },
                            display_text: Some(display_text),
                        },
                        &(),
                    );
                }
            }

            let sum = new_transforms.summary();
            if sum.buffer.bytes < edit.new_bytes.end {
                let text_summary =
                    buffer.text_summary_for_range(sum.buffer.bytes..edit.new_bytes.end);
                new_transforms.push(
                    Transform {
                        summary: TransformSummary {
                            display: text_summary.clone(),
                            buffer: text_summary,
                        },
                        display_text: None,
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
                        display: text_summary.clone(),
                        buffer: text_summary,
                    },
                    display_text: None,
                },
                &(),
            );
        }

        drop(cursor);

        {
            let mut old_transforms = transforms.cursor::<usize, DisplayOffset>();
            let mut new_transforms = new_transforms.cursor::<usize, DisplayOffset>();

            let mut delta = 0;
            for edit in &mut edits {
                old_transforms.seek_forward(&edit.old_bytes.start, Bias::Right, &());
                edit.old_bytes.start = old_transforms.sum_start().0
                    + (edit.old_bytes.start - old_transforms.seek_start());
                old_transforms.seek_forward(&edit.old_bytes.end, Bias::Right, &());
                edit.old_bytes.end = old_transforms.sum_start().0
                    + (edit.old_bytes.end - old_transforms.seek_start());
                new_transforms.seek_forward(&edit.new_bytes.start, Bias::Right, &());
                edit.new_bytes.start = new_transforms.sum_start().0
                    + (edit.new_bytes.start - new_transforms.seek_start());
                new_transforms.seek_forward(&edit.new_bytes.end, Bias::Right, &());
                edit.new_bytes.end = new_transforms.sum_start().0
                    + (edit.new_bytes.end - new_transforms.seek_start());

                edit.old_bytes.start = (edit.old_bytes.start as isize + delta) as usize;
                edit.old_bytes.end = (edit.old_bytes.end as isize + delta) as usize;
                delta += edit.delta();
            }
        }

        *transforms = new_transforms;
        self.history.lock().edits.extend(edits);
    }

    fn consolidate_edits(&self, edits: &mut Vec<Edit>) {
        edits.sort_unstable_by(|a, b| {
            a.old_bytes
                .start
                .cmp(&b.old_bytes.start)
                .then_with(|| b.old_bytes.end.cmp(&a.old_bytes.end))
        });

        let mut i = 0;
        while i < edits.len() {
            let range = edits[i].old_bytes.clone();
            if i > 0 {
                if edits[i - 1].old_bytes.end >= range.start {
                    edits[i - 1].old_bytes.end = edits[i - 1].old_bytes.end.max(range.end);
                    edits[i - 1].new_bytes.end = edits[i - 1].new_bytes.end.max(range.end);
                    edits.remove(i);
                    continue;
                }
            }
            i += 1;
        }
    }
}

pub struct FoldMapSnapshot {
    transforms: SumTree<Transform>,
    folds: SumTree<Fold>,
    buffer: buffer::Snapshot,
    history: Arc<Mutex<FoldMapHistory>>,
    version: usize,
}

impl FoldMapSnapshot {
    pub fn edits_since_creation(&self) -> Vec<Edit> {
        let history = self.history.lock();
        let index = self.version - history.oldest_edit_version;
        history.edits.range(index..).cloned().collect()
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks_at(DisplayOffset(0)).collect()
    }

    pub fn len(&self) -> usize {
        self.transforms.summary().display.bytes
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let line_start = self.to_display_offset(DisplayPoint::new(row, 0)).0;
        let line_end = if row >= self.max_point().row() {
            self.len()
        } else {
            self.to_display_offset(DisplayPoint::new(row + 1, 0)).0 - 1
        };
        (line_end - line_start) as u32
    }

    pub fn buffer_rows(&self, start_row: u32) -> BufferRows {
        if start_row > self.transforms.summary().display.lines.row {
            panic!("invalid display row {}", start_row);
        }

        let display_point = Point::new(start_row, 0);
        let mut cursor = self.transforms.cursor();
        cursor.seek(&DisplayPoint(display_point), Bias::Left, &());

        BufferRows {
            display_point,
            cursor,
        }
    }

    pub fn max_point(&self) -> DisplayPoint {
        DisplayPoint(self.transforms.summary().display.lines)
    }

    pub fn longest_row(&self) -> u32 {
        self.transforms.summary().display.longest_row
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
        cursor.item().map_or(false, |t| t.display_text.is_some())
    }

    pub fn is_line_folded(&self, display_row: u32) -> bool {
        let mut cursor = self.transforms.cursor::<DisplayPoint, ()>();
        cursor.seek(&DisplayPoint::new(display_row, 0), Bias::Right, &());
        while let Some(transform) = cursor.item() {
            if transform.display_text.is_some() {
                return true;
            }
            if cursor.seek_end(&()).row() == display_row {
                cursor.next(&())
            } else {
                break;
            }
        }
        false
    }

    pub fn chunks_at(&self, offset: DisplayOffset) -> Chunks {
        let mut transform_cursor = self.transforms.cursor::<DisplayOffset, usize>();
        transform_cursor.seek(&offset, Bias::Right, &());
        let overshoot = offset.0 - transform_cursor.seek_start().0;
        let buffer_offset = transform_cursor.sum_start() + overshoot;
        Chunks {
            transform_cursor,
            buffer_offset,
            buffer_chunks: self.buffer.text_for_range(buffer_offset..self.buffer.len()),
        }
    }

    pub fn highlighted_chunks(&mut self, range: Range<DisplayOffset>) -> HighlightedChunks {
        let mut transform_cursor = self.transforms.cursor::<DisplayOffset, usize>();

        transform_cursor.seek(&range.end, Bias::Right, &());
        let overshoot = range.end.0 - transform_cursor.seek_start().0;
        let buffer_end = transform_cursor.sum_start() + overshoot;

        transform_cursor.seek(&range.start, Bias::Right, &());
        let overshoot = range.start.0 - transform_cursor.seek_start().0;
        let buffer_start = transform_cursor.sum_start() + overshoot;

        HighlightedChunks {
            transform_cursor,
            buffer_offset: buffer_start,
            buffer_chunks: self
                .buffer
                .highlighted_text_for_range(buffer_start..buffer_end),
            buffer_chunk: None,
        }
    }

    pub fn chars_at<'a>(&'a self, point: DisplayPoint) -> impl Iterator<Item = char> + 'a {
        let offset = self.to_display_offset(point);
        self.chunks_at(offset).flat_map(str::chars)
    }

    pub fn to_display_offset(&self, point: DisplayPoint) -> DisplayOffset {
        let mut cursor = self.transforms.cursor::<DisplayPoint, TransformSummary>();
        cursor.seek(&point, Bias::Right, &());
        let overshoot = point.0 - cursor.sum_start().display.lines;
        let mut offset = cursor.sum_start().display.bytes;
        if !overshoot.is_zero() {
            let transform = cursor.item().expect("display point out of range");
            assert!(transform.display_text.is_none());
            let end_buffer_offset = self
                .buffer
                .to_offset(cursor.sum_start().buffer.lines + overshoot);
            offset += end_buffer_offset - cursor.sum_start().buffer.bytes;
        }
        DisplayOffset(offset)
    }

    pub fn to_buffer_offset(&self, point: DisplayPoint) -> usize {
        let mut cursor = self.transforms.cursor::<DisplayPoint, Point>();
        cursor.seek(&point, Bias::Right, &());
        let overshoot = point.0 - cursor.seek_start().0;
        self.buffer.to_offset(*cursor.sum_start() + overshoot)
    }

    pub fn to_buffer_point(&self, display_point: DisplayPoint) -> Point {
        let mut cursor = self.transforms.cursor::<DisplayPoint, Point>();
        cursor.seek(&display_point, Bias::Right, &());
        let overshoot = display_point.0 - cursor.seek_start().0;
        *cursor.sum_start() + overshoot
    }

    pub fn to_display_point(&self, point: Point) -> DisplayPoint {
        let mut cursor = self.transforms.cursor::<Point, DisplayPoint>();
        cursor.seek(&point, Bias::Right, &());
        let overshoot = point - cursor.seek_start();
        DisplayPoint(cmp::min(
            cursor.sum_start().0 + overshoot,
            cursor.end(&()).0,
        ))
    }

    #[cfg(test)]
    pub fn clip_offset(&self, offset: DisplayOffset, bias: Bias) -> DisplayOffset {
        let mut cursor = self.transforms.cursor::<DisplayOffset, usize>();
        cursor.seek(&offset, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let transform_start = cursor.seek_start().0;
            if transform.display_text.is_some() {
                if offset.0 == transform_start || matches!(bias, Bias::Left) {
                    DisplayOffset(transform_start)
                } else {
                    DisplayOffset(cursor.seek_end(&()).0)
                }
            } else {
                let overshoot = offset.0 - transform_start;
                let buffer_offset = cursor.sum_start() + overshoot;
                let clipped_buffer_offset = self.buffer.clip_offset(buffer_offset, bias);
                DisplayOffset(
                    (offset.0 as isize + (clipped_buffer_offset as isize - buffer_offset as isize))
                        as usize,
                )
            }
        } else {
            DisplayOffset(self.transforms.summary().display.bytes)
        }
    }

    pub fn clip_point(&self, point: DisplayPoint, bias: Bias) -> DisplayPoint {
        let mut cursor = self.transforms.cursor::<DisplayPoint, Point>();
        cursor.seek(&point, Bias::Right, &());
        if let Some(transform) = cursor.item() {
            let transform_start = cursor.seek_start().0;
            if transform.display_text.is_some() {
                if point.0 == transform_start || matches!(bias, Bias::Left) {
                    DisplayPoint(transform_start)
                } else {
                    DisplayPoint(cursor.seek_end(&()).0)
                }
            } else {
                let overshoot = point.0 - transform_start;
                let buffer_position = *cursor.sum_start() + overshoot;
                let clipped_buffer_position = self.buffer.clip_point(buffer_position, bias);
                DisplayPoint::new(
                    point.row(),
                    ((point.column() as i32) + clipped_buffer_position.column as i32
                        - buffer_position.column as i32) as u32,
                )
            }
        } else {
            DisplayPoint(self.transforms.summary().display.lines)
        }
    }
}

impl Drop for FoldMapSnapshot {
    fn drop(&mut self) {
        let mut history = self.history.lock();

        // Remove this snapshot's version number from the set of currently-snapshotted
        // version numbers.
        let mut i = 0;
        let mut min_snapshot_version = None;
        while i < history.snapshot_versions.len() {
            let version = history.snapshot_versions[i];
            if version == self.version {
                history.snapshot_versions.remove(i);
            } else {
                if min_snapshot_version.map_or(true, |min_version| min_version > version) {
                    min_snapshot_version = Some(version);
                }
                i += 1;
            }
        }

        // Discard any edits that predate any outstanding snapshot.
        if let Some(min_snapshot_version) = min_snapshot_version {
            if min_snapshot_version > history.oldest_edit_version {
                let remove_count = min_snapshot_version - history.oldest_edit_version;
                drop(history.edits.drain(..remove_count));
                history.oldest_edit_version = min_snapshot_version;
            }
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Transform {
    summary: TransformSummary,
    display_text: Option<&'static str>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TransformSummary {
    display: TextSummary,
    buffer: TextSummary,
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
        self.buffer += &other.buffer;
        self.display += &other.display;
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

pub struct BufferRows<'a> {
    cursor: Cursor<'a, Transform, DisplayPoint, Point>,
    display_point: Point,
}

impl<'a> Iterator for BufferRows<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.display_point > self.cursor.seek_end(&()).0 {
            self.cursor.next(&());
            if self.cursor.item().is_none() {
                // TODO: Return a bool from next?
                break;
            }
        }

        if self.cursor.item().is_some() {
            let overshoot = self.display_point - self.cursor.seek_start().0;
            let buffer_point = *self.cursor.sum_start() + overshoot;
            self.display_point.row += 1;
            Some(buffer_point.row)
        } else {
            None
        }
    }
}

pub struct Chunks<'a> {
    transform_cursor: Cursor<'a, Transform, DisplayOffset, usize>,
    buffer_chunks: buffer::Chunks<'a>,
    buffer_offset: usize,
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
        if let Some(display_text) = transform.display_text {
            self.buffer_offset += transform.summary.buffer.bytes;
            self.buffer_chunks.seek(self.buffer_offset);

            while self.buffer_offset >= self.transform_cursor.end(&())
                && self.transform_cursor.item().is_some()
            {
                self.transform_cursor.next(&());
            }

            return Some(display_text);
        }

        // Otherwise, take a chunk from the buffer's text.
        if let Some(mut chunk) = self.buffer_chunks.peek() {
            let offset_in_chunk = self.buffer_offset - self.buffer_chunks.offset();
            chunk = &chunk[offset_in_chunk..];

            // Truncate the chunk so that it ends at the next fold.
            let region_end = self.transform_cursor.end(&()) - self.buffer_offset;
            if chunk.len() >= region_end {
                chunk = &chunk[0..region_end];
                self.transform_cursor.next(&());
            } else {
                self.buffer_chunks.next();
            }

            self.buffer_offset += chunk.len();
            return Some(chunk);
        }

        None
    }
}

pub struct HighlightedChunks<'a> {
    transform_cursor: Cursor<'a, Transform, DisplayOffset, usize>,
    buffer_chunks: buffer::HighlightedChunks<'a>,
    buffer_chunk: Option<(usize, &'a str, StyleId)>,
    buffer_offset: usize,
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
        if let Some(display_text) = transform.display_text {
            self.buffer_chunk.take();
            self.buffer_offset += transform.summary.buffer.bytes;
            self.buffer_chunks.seek(self.buffer_offset);

            while self.buffer_offset >= self.transform_cursor.end(&())
                && self.transform_cursor.item().is_some()
            {
                self.transform_cursor.next(&());
            }

            return Some((display_text, StyleId::default()));
        }

        // Retrieve a chunk from the current location in the buffer.
        if self.buffer_chunk.is_none() {
            let chunk_offset = self.buffer_chunks.offset();
            self.buffer_chunk = self
                .buffer_chunks
                .next()
                .map(|(chunk, capture_ix)| (chunk_offset, chunk, capture_ix));
        }

        // Otherwise, take a chunk from the buffer's text.
        if let Some((chunk_offset, mut chunk, capture_ix)) = self.buffer_chunk {
            let offset_in_chunk = self.buffer_offset - chunk_offset;
            chunk = &chunk[offset_in_chunk..];

            // Truncate the chunk so that it ends at the next fold.
            let region_end = self.transform_cursor.end(&()) - self.buffer_offset;
            if chunk.len() >= region_end {
                chunk = &chunk[0..region_end];
                self.transform_cursor.next(&());
            } else {
                self.buffer_chunk.take();
            }

            self.buffer_offset += chunk.len();
            return Some((chunk, capture_ix));
        }

        None
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for DisplayPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.display.lines;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct DisplayOffset(usize);

impl<'a> sum_tree::Dimension<'a, TransformSummary> for DisplayOffset {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += &summary.display.bytes;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for Point {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += &summary.buffer.lines;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for usize {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        *self += &summary.buffer.bytes;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edit {
    pub old_bytes: Range<usize>,
    pub new_bytes: Range<usize>,
}

impl Edit {
    pub fn delta(&self) -> isize {
        self.inserted_bytes() as isize - self.deleted_bytes() as isize
    }

    pub fn deleted_bytes(&self) -> usize {
        self.old_bytes.len()
    }

    pub fn inserted_bytes(&self) -> usize {
        self.new_bytes.len()
    }
}

impl From<buffer::Edit> for Edit {
    fn from(edit: buffer::Edit) -> Self {
        Self {
            old_bytes: edit.old_bytes,
            new_bytes: edit.new_bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::buffer::ToPoint;
    use crate::test::sample_text;

    #[gpui::test]
    fn test_basic_folds(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(5, 6), cx));
        let mut map = FoldMap::new(buffer.clone(), cx.as_ref());
        let snapshot1 = map.snapshot(cx.as_ref());

        map.fold(
            vec![
                Point::new(0, 2)..Point::new(2, 2),
                Point::new(2, 4)..Point::new(4, 1),
            ],
            cx.as_ref(),
        );
        let snapshot2 = map.snapshot(cx.as_ref());
        assert_eq!(snapshot2.text(), "aa…cc…eeeee");

        assert_eq!(snapshot2.edits_since_creation(), &[]);
        assert_eq!(
            snapshot1.edits_since_creation(),
            &[
                Edit {
                    old_bytes: 2..16,
                    new_bytes: 2..5,
                },
                Edit {
                    old_bytes: 7..18,
                    new_bytes: 7..10
                },
            ]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                vec![
                    Point::new(0, 0)..Point::new(0, 1),
                    Point::new(2, 3)..Point::new(2, 3),
                ],
                "123",
                cx,
            );
        });
        let snapshot3 = map.snapshot(cx.as_ref());
        assert_eq!(snapshot3.text(), "123a…c123c…eeeee");

        assert_eq!(snapshot3.edits_since_creation(), &[]);
        assert_eq!(
            snapshot2.edits_since_creation(),
            &[
                Edit {
                    old_bytes: 0..1,
                    new_bytes: 0..3,
                },
                Edit {
                    old_bytes: 8..8,
                    new_bytes: 8..11,
                },
            ]
        );
        assert_eq!(
            snapshot1.edits_since_creation(),
            &[
                Edit {
                    old_bytes: 2..16,
                    new_bytes: 2..5,
                },
                Edit {
                    old_bytes: 7..18,
                    new_bytes: 7..10
                },
                Edit {
                    old_bytes: 0..1,
                    new_bytes: 0..3,
                },
                Edit {
                    old_bytes: 8..8,
                    new_bytes: 8..11,
                },
            ]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit(vec![Point::new(2, 6)..Point::new(4, 3)], "456", cx)
        });
        let snapshot4 = map.snapshot(cx.as_ref());
        assert_eq!(snapshot4.text(), "123a…c123456eee");

        map.unfold(Some(Point::new(0, 4)..Point::new(0, 5)), cx.as_ref());
        let snapshot5 = map.snapshot(cx.as_ref());
        assert_eq!(snapshot5.text(), "123aaaaa\nbbbbbb\nccc123456eee");

        drop(snapshot1);
        drop(snapshot2);
        drop(snapshot3);
        drop(snapshot4);
        assert_eq!(map.history.lock().edits, &[]);
    }

    #[gpui::test]
    fn test_adjacent_folds(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "abcdefghijkl", cx));

        {
            let mut map = FoldMap::new(buffer.clone(), cx.as_ref());

            map.fold(vec![5..8], cx.as_ref());
            map.check_invariants(cx.as_ref());
            assert_eq!(map.snapshot(cx.as_ref()).text(), "abcde…ijkl");

            // Create an fold adjacent to the start of the first fold.
            map.fold(vec![0..1, 2..5], cx.as_ref());
            map.check_invariants(cx.as_ref());
            assert_eq!(map.snapshot(cx.as_ref()).text(), "…b…ijkl");

            // Create an fold adjacent to the end of the first fold.
            map.fold(vec![11..11, 8..10], cx.as_ref());
            map.check_invariants(cx.as_ref());
            assert_eq!(map.snapshot(cx.as_ref()).text(), "…b…kl");
        }

        {
            let mut map = FoldMap::new(buffer.clone(), cx.as_ref());

            // Create two adjacent folds.
            map.fold(vec![0..2, 2..5], cx.as_ref());
            map.check_invariants(cx.as_ref());
            assert_eq!(map.snapshot(cx.as_ref()).text(), "…fghijkl");

            // Edit within one of the folds.
            buffer.update(cx, |buffer, cx| {
                let version = buffer.version();
                buffer.edit(vec![0..1], "12345", cx);
                buffer.edits_since(version).collect::<Vec<_>>()
            });
            map.check_invariants(cx.as_ref());
            assert_eq!(map.snapshot(cx.as_ref()).text(), "12345…fghijkl");
        }
    }

    #[gpui::test]
    fn test_overlapping_folds(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(5, 6), cx));
        let mut map = FoldMap::new(buffer.clone(), cx.as_ref());
        map.fold(
            vec![
                Point::new(0, 2)..Point::new(2, 2),
                Point::new(0, 4)..Point::new(1, 0),
                Point::new(1, 2)..Point::new(3, 2),
                Point::new(3, 1)..Point::new(4, 1),
            ],
            cx.as_ref(),
        );
        assert_eq!(map.snapshot(cx.as_ref()).text(), "aa…eeeee");
    }

    #[gpui::test]
    fn test_merging_folds_via_edit(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(5, 6), cx));
        let mut map = FoldMap::new(buffer.clone(), cx.as_ref());

        map.fold(
            vec![
                Point::new(0, 2)..Point::new(2, 2),
                Point::new(3, 1)..Point::new(4, 1),
            ],
            cx.as_ref(),
        );
        assert_eq!(map.snapshot(cx.as_ref()).text(), "aa…cccc\nd…eeeee");

        buffer.update(cx, |buffer, cx| {
            buffer.edit(Some(Point::new(2, 2)..Point::new(3, 1)), "", cx)
        });
        assert_eq!(map.snapshot(cx.as_ref()).text(), "aa…eeeee");
    }

    #[gpui::test]
    fn test_folds_in_range(cx: &mut gpui::MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(5, 6), cx));
        let mut map = FoldMap::new(buffer.clone(), cx.as_ref());
        let buffer = buffer.read(cx);

        map.fold(
            vec![
                Point::new(0, 2)..Point::new(2, 2),
                Point::new(0, 4)..Point::new(1, 0),
                Point::new(1, 2)..Point::new(3, 2),
                Point::new(3, 1)..Point::new(4, 1),
            ],
            cx.as_ref(),
        );
        let fold_ranges = map
            .snapshot(cx.as_ref())
            .folds_in_range(Point::new(1, 0)..Point::new(1, 3))
            .map(|fold| fold.start.to_point(buffer)..fold.end.to_point(buffer))
            .collect::<Vec<_>>();
        assert_eq!(
            fold_ranges,
            vec![
                Point::new(0, 2)..Point::new(2, 2),
                Point::new(1, 2)..Point::new(3, 2)
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
                        map.fold(to_fold, cx.as_ref());
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
                        map.unfold(to_unfold, cx.as_ref());
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
                map.check_invariants(cx.as_ref());

                let buffer = map.buffer.read(cx).snapshot();
                let mut expected_text: String = buffer.text().into();
                let mut expected_buffer_rows = Vec::new();
                let mut next_row = buffer.max_point().row;
                for fold_range in map.merged_fold_ranges(cx.as_ref()).into_iter().rev() {
                    let fold_start = buffer.point_for_offset(fold_range.start).unwrap();
                    let fold_end = buffer.point_for_offset(fold_range.end).unwrap();
                    expected_buffer_rows.extend((fold_end.row + 1..=next_row).rev());
                    next_row = fold_start.row;

                    expected_text.replace_range(fold_range.start..fold_range.end, "…");
                }
                expected_buffer_rows.extend((0..=next_row).rev());
                expected_buffer_rows.reverse();

                assert_eq!(map.snapshot(cx.as_ref()).text(), expected_text);

                let snapshot = map.snapshot(cx.as_ref());
                for (display_row, line) in expected_text.lines().enumerate() {
                    let line_len = snapshot.line_len(display_row as u32);
                    assert_eq!(line_len, line.len() as u32);
                }

                let longest_row = snapshot.longest_row();
                let longest_char_column = expected_text
                    .split('\n')
                    .nth(longest_row as usize)
                    .unwrap()
                    .chars()
                    .count();
                let mut display_point = DisplayPoint::new(0, 0);
                let mut display_offset = DisplayOffset(0);
                let mut char_column = 0;
                for c in expected_text.chars() {
                    let buffer_point = snapshot.to_buffer_point(display_point);
                    let buffer_offset = buffer_point.to_offset(&buffer);
                    assert_eq!(
                        snapshot.to_display_point(buffer_point),
                        display_point,
                        "to_display_point({:?})",
                        buffer_point,
                    );
                    assert_eq!(
                        snapshot.to_buffer_offset(display_point),
                        buffer_offset,
                        "to_buffer_offset({:?})",
                        display_point,
                    );
                    assert_eq!(
                        snapshot.to_display_offset(display_point),
                        display_offset,
                        "to_display_offset({:?})",
                        display_point,
                    );

                    if c == '\n' {
                        *display_point.row_mut() += 1;
                        *display_point.column_mut() = 0;
                        char_column = 0;
                    } else {
                        *display_point.column_mut() += c.len_utf8() as u32;
                        char_column += 1;
                    }
                    display_offset.0 += c.len_utf8();
                    if char_column > longest_char_column {
                        panic!(
                            "invalid longest row {:?} (chars {}), found row {:?} (chars: {})",
                            longest_row,
                            longest_char_column,
                            display_point.row(),
                            char_column
                        );
                    }
                }

                let snapshot = map.snapshot(cx.as_ref());
                for _ in 0..5 {
                    let offset = snapshot.clip_offset(
                        DisplayOffset(rng.gen_range(0..=snapshot.len())),
                        Bias::Right,
                    );
                    assert_eq!(
                        snapshot.chunks_at(offset).collect::<String>(),
                        &expected_text[offset.0..],
                    );
                }

                for (idx, buffer_row) in expected_buffer_rows.iter().enumerate() {
                    let display_row = snapshot.to_display_point(Point::new(*buffer_row, 0)).row();
                    assert_eq!(
                        snapshot.buffer_rows(display_row).collect::<Vec<_>>(),
                        expected_buffer_rows[idx..],
                    );
                }

                for fold_range in map.merged_fold_ranges(cx.as_ref()) {
                    let display_point =
                        snapshot.to_display_point(fold_range.start.to_point(&buffer));
                    assert!(snapshot.is_line_folded(display_point.row()));
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
            }
        }
    }

    #[gpui::test]
    fn test_buffer_rows(cx: &mut gpui::MutableAppContext) {
        let text = sample_text(6, 6) + "\n";
        let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));

        let mut map = FoldMap::new(buffer.clone(), cx.as_ref());

        map.fold(
            vec![
                Point::new(0, 2)..Point::new(2, 2),
                Point::new(3, 1)..Point::new(4, 1),
            ],
            cx.as_ref(),
        );

        assert_eq!(
            map.snapshot(cx.as_ref()).text(),
            "aa…cccc\nd…eeeee\nffffff\n"
        );
        assert_eq!(
            map.snapshot(cx.as_ref()).buffer_rows(0).collect::<Vec<_>>(),
            vec![0, 3, 5, 6]
        );
        assert_eq!(
            map.snapshot(cx.as_ref()).buffer_rows(3).collect::<Vec<_>>(),
            vec![6]
        );
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

        fn check_invariants(&self, cx: &AppContext) {
            self.sync(cx);
            let buffer = self.buffer.read(cx);
            assert_eq!(
                self.transforms.lock().summary().buffer.bytes,
                buffer.len(),
                "transform tree does not match buffer's length"
            );
        }
    }
}

use super::{
    buffer, Anchor, AnchorRangeExt, Buffer, DisplayPoint, Edit, Point, TextSummary, ToOffset,
};
use crate::{
    sum_tree::{self, Cursor, SumTree},
    util::find_insertion_index,
};
use anyhow::{anyhow, Result};
use gpui::{AppContext, ModelHandle};
use std::{
    cmp::{self, Ordering},
    iter::Take,
    ops::Range,
};
use sum_tree::{Dimension, SeekBias};

pub struct FoldMap {
    buffer: ModelHandle<Buffer>,
    transforms: SumTree<Transform>,
    folds: Vec<Range<Anchor>>,
}

impl FoldMap {
    pub fn new(buffer: ModelHandle<Buffer>, app: &AppContext) -> Self {
        let text_summary = buffer.read(app).text_summary();
        Self {
            buffer,
            folds: Vec::new(),
            transforms: SumTree::from_item(Transform {
                summary: TransformSummary {
                    buffer: text_summary.clone(),
                    display: text_summary,
                },
                display_text: None,
            }),
        }
    }

    pub fn buffer_rows(&self, start_row: u32) -> Result<BufferRows> {
        if start_row > self.transforms.summary().display.lines.row {
            return Err(anyhow!("invalid display row {}", start_row));
        }

        let display_point = Point::new(start_row, 0);
        let mut cursor = self.transforms.cursor();
        cursor.seek(&DisplayPoint(display_point), SeekBias::Left);

        Ok(BufferRows {
            display_point,
            cursor,
        })
    }

    pub fn len(&self) -> usize {
        self.transforms.summary().display.chars
    }

    pub fn line_len(&self, row: u32, ctx: &AppContext) -> Result<u32> {
        let line_start = self.to_display_offset(DisplayPoint::new(row, 0), ctx)?.0;
        let line_end = if row >= self.max_point().row() {
            self.len()
        } else {
            self.to_display_offset(DisplayPoint::new(row + 1, 0), ctx)?
                .0
                - 1
        };

        Ok((line_end - line_start) as u32)
    }

    pub fn chars_at<'a>(&'a self, point: DisplayPoint, app: &'a AppContext) -> Result<Chars<'a>> {
        let offset = self.to_display_offset(point, app)?;
        let mut cursor = self.transforms.cursor();
        cursor.seek(&offset, SeekBias::Right);
        let buffer = self.buffer.read(app);
        Ok(Chars {
            cursor,
            offset: offset.0,
            buffer,
            buffer_chars: None,
        })
    }

    pub fn max_point(&self) -> DisplayPoint {
        DisplayPoint(self.transforms.summary().display.lines)
    }

    pub fn rightmost_point(&self) -> DisplayPoint {
        DisplayPoint(self.transforms.summary().display.rightmost_point)
    }

    pub fn folds_in_range<'a, T>(
        &'a self,
        range: Range<T>,
        app: &'a AppContext,
    ) -> Result<impl Iterator<Item = &'a Range<Anchor>>>
    where
        T: ToOffset,
    {
        let buffer = self.buffer.read(app);
        let range = buffer.anchor_before(range.start)?..buffer.anchor_before(range.end)?;
        Ok(self.folds.iter().filter(move |fold| {
            range.start.cmp(&fold.end, buffer).unwrap() == Ordering::Less
                && range.end.cmp(&fold.start, buffer).unwrap() == Ordering::Greater
        }))
    }

    pub fn fold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        app: &AppContext,
    ) -> Result<()> {
        let mut edits = Vec::new();
        let buffer = self.buffer.read(app);
        for range in ranges.into_iter() {
            let start = range.start.to_offset(buffer)?;
            let end = range.end.to_offset(buffer)?;
            edits.push(Edit {
                old_range: start..end,
                new_range: start..end,
            });

            let fold = buffer.anchor_after(start)?..buffer.anchor_before(end)?;
            let ix = find_insertion_index(&self.folds, |probe| probe.cmp(&fold, buffer))?;
            self.folds.insert(ix, fold);
        }
        edits.sort_unstable_by(|a, b| {
            a.old_range
                .start
                .cmp(&b.old_range.start)
                .then_with(|| b.old_range.end.cmp(&a.old_range.end))
        });

        self.apply_edits(&edits, app)?;
        Ok(())
    }

    pub fn unfold<T: ToOffset>(
        &mut self,
        ranges: impl IntoIterator<Item = Range<T>>,
        app: &AppContext,
    ) -> Result<()> {
        let buffer = self.buffer.read(app);

        let mut edits = Vec::new();
        for range in ranges.into_iter() {
            let start = buffer.anchor_before(range.start.to_offset(buffer)?)?;
            let end = buffer.anchor_after(range.end.to_offset(buffer)?)?;

            // Remove intersecting folds and add their ranges to edits that are passed to apply_edits
            self.folds.retain(|fold| {
                if fold.start.cmp(&end, buffer).unwrap() > Ordering::Equal
                    || fold.end.cmp(&start, buffer).unwrap() < Ordering::Equal
                {
                    true
                } else {
                    let offset_range =
                        fold.start.to_offset(buffer).unwrap()..fold.end.to_offset(buffer).unwrap();
                    edits.push(Edit {
                        old_range: offset_range.clone(),
                        new_range: offset_range,
                    });
                    false
                }
            });
        }

        self.apply_edits(&edits, app)?;
        Ok(())
    }

    pub fn is_line_folded(&self, display_row: u32) -> bool {
        let mut cursor = self.transforms.cursor::<DisplayPoint, DisplayPoint>();
        cursor.seek(&DisplayPoint::new(display_row, 0), SeekBias::Right);
        while let Some(transform) = cursor.item() {
            if transform.display_text.is_some() {
                return true;
            }
            if cursor.end().row() == display_row {
                cursor.next()
            } else {
                break;
            }
        }
        false
    }

    pub fn to_buffer_offset(&self, point: DisplayPoint, app: &AppContext) -> Result<usize> {
        let mut cursor = self.transforms.cursor::<DisplayPoint, TransformSummary>();
        cursor.seek(&point, SeekBias::Right);
        let overshoot = point.0 - cursor.start().display.lines;
        (cursor.start().buffer.lines + overshoot).to_offset(self.buffer.read(app))
    }

    pub fn to_display_offset(
        &self,
        point: DisplayPoint,
        app: &AppContext,
    ) -> Result<DisplayOffset> {
        let mut cursor = self.transforms.cursor::<DisplayPoint, TransformSummary>();
        cursor.seek(&point, SeekBias::Right);
        let overshoot = point.0 - cursor.start().display.lines;
        let mut offset = cursor.start().display.chars;
        if !overshoot.is_zero() {
            let transform = cursor
                .item()
                .ok_or_else(|| anyhow!("display point {:?} is out of range", point))?;
            assert!(transform.display_text.is_none());
            let end_buffer_offset =
                (cursor.start().buffer.lines + overshoot).to_offset(self.buffer.read(app))?;
            offset += end_buffer_offset - cursor.start().buffer.chars;
        }
        Ok(DisplayOffset(offset))
    }

    pub fn to_buffer_point(&self, display_point: DisplayPoint) -> Point {
        let mut cursor = self.transforms.cursor::<DisplayPoint, TransformSummary>();
        cursor.seek(&display_point, SeekBias::Right);
        let overshoot = display_point.0 - cursor.start().display.lines;
        cursor.start().buffer.lines + overshoot
    }

    pub fn to_display_point(&self, point: Point) -> DisplayPoint {
        let mut cursor = self.transforms.cursor::<Point, TransformSummary>();
        cursor.seek(&point, SeekBias::Right);
        let overshoot = point - cursor.start().buffer.lines;
        DisplayPoint(cmp::min(
            cursor.start().display.lines + overshoot,
            cursor.end().display.lines,
        ))
    }

    pub fn apply_edits(&mut self, edits: &[Edit], app: &AppContext) -> Result<()> {
        let buffer = self.buffer.read(app);
        let mut edits = edits.iter().cloned().peekable();

        let mut new_transforms = SumTree::new();
        let mut cursor = self.transforms.cursor::<usize, usize>();
        cursor.seek(&0, SeekBias::Right);

        while let Some(mut edit) = edits.next() {
            new_transforms.push_tree(cursor.slice(&edit.old_range.start, SeekBias::Left));
            edit.new_range.start -= edit.old_range.start - cursor.start();
            edit.old_range.start = *cursor.start();

            cursor.seek(&edit.old_range.end, SeekBias::Right);
            cursor.next();

            let mut delta = edit.delta();
            loop {
                edit.old_range.end = *cursor.start();

                if let Some(next_edit) = edits.peek() {
                    if next_edit.old_range.start > edit.old_range.end {
                        break;
                    }

                    let next_edit = edits.next().unwrap();
                    delta += next_edit.delta();

                    if next_edit.old_range.end >= edit.old_range.end {
                        edit.old_range.end = next_edit.old_range.end;
                        cursor.seek(&edit.old_range.end, SeekBias::Right);
                        cursor.next();
                    }
                } else {
                    break;
                }
            }

            edit.new_range.end =
                ((edit.new_range.start + edit.old_extent()) as isize + delta) as usize;

            let anchor = buffer.anchor_before(edit.new_range.start)?;
            let folds_start =
                find_insertion_index(&self.folds, |probe| probe.start.cmp(&anchor, buffer))?;
            let mut folds = self.folds[folds_start..]
                .iter()
                .map(|fold| {
                    fold.start.to_offset(buffer).unwrap()..fold.end.to_offset(buffer).unwrap()
                })
                .peekable();

            while folds
                .peek()
                .map_or(false, |fold| fold.start < edit.new_range.end)
            {
                let mut fold = folds.next().unwrap();
                let sum = new_transforms.summary();

                assert!(fold.start >= sum.buffer.chars);

                while folds
                    .peek()
                    .map_or(false, |next_fold| next_fold.start <= fold.end)
                {
                    let next_fold = folds.next().unwrap();
                    if next_fold.end > fold.end {
                        fold.end = next_fold.end;
                    }
                }

                if fold.start > sum.buffer.chars {
                    let text_summary = buffer.text_summary_for_range(sum.buffer.chars..fold.start);
                    new_transforms.push(Transform {
                        summary: TransformSummary {
                            display: text_summary.clone(),
                            buffer: text_summary,
                        },
                        display_text: None,
                    });
                }

                if fold.end > fold.start {
                    new_transforms.push(Transform {
                        summary: TransformSummary {
                            display: TextSummary {
                                chars: 1,
                                bytes: '…'.len_utf8(),
                                lines: Point::new(0, 1),
                                first_line_len: 1,
                                rightmost_point: Point::new(0, 1),
                            },
                            buffer: buffer.text_summary_for_range(fold.start..fold.end),
                        },
                        display_text: Some('…'),
                    });
                }
            }

            let sum = new_transforms.summary();
            if sum.buffer.chars < edit.new_range.end {
                let text_summary =
                    buffer.text_summary_for_range(sum.buffer.chars..edit.new_range.end);
                new_transforms.push(Transform {
                    summary: TransformSummary {
                        display: text_summary.clone(),
                        buffer: text_summary,
                    },
                    display_text: None,
                });
            }
        }

        new_transforms.push_tree(cursor.suffix());
        if new_transforms.is_empty() {
            let text_summary = buffer.text_summary();
            new_transforms.push(Transform {
                summary: TransformSummary {
                    display: text_summary.clone(),
                    buffer: text_summary,
                },
                display_text: None,
            });
        }

        drop(cursor);
        self.transforms = new_transforms;

        Ok(())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Transform {
    summary: TransformSummary,
    display_text: Option<char>,
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

impl<'a> std::ops::AddAssign<&'a Self> for TransformSummary {
    fn add_assign(&mut self, other: &'a Self) {
        self.buffer += &other.buffer;
        self.display += &other.display;
    }
}

impl<'a> Dimension<'a, TransformSummary> for TransformSummary {
    fn add_summary(&mut self, summary: &'a TransformSummary) {
        *self += summary;
    }
}

pub struct BufferRows<'a> {
    cursor: Cursor<'a, Transform, DisplayPoint, TransformSummary>,
    display_point: Point,
}

impl<'a> Iterator for BufferRows<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.display_point > self.cursor.end().display.lines {
            self.cursor.next();
            if self.cursor.item().is_none() {
                // TODO: Return a bool from next?
                break;
            }
        }

        if self.cursor.item().is_some() {
            let overshoot = self.display_point - self.cursor.start().display.lines;
            let buffer_point = self.cursor.start().buffer.lines + overshoot;
            self.display_point.row += 1;
            Some(buffer_point.row)
        } else {
            None
        }
    }
}

pub struct Chars<'a> {
    cursor: Cursor<'a, Transform, DisplayOffset, TransformSummary>,
    offset: usize,
    buffer: &'a Buffer,
    buffer_chars: Option<Take<buffer::CharIter<'a>>>,
}

impl<'a> Iterator for Chars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.buffer_chars.as_mut().and_then(|chars| chars.next()) {
            self.offset += 1;
            return Some(c);
        }

        while self.offset == self.cursor.end().display.chars && self.cursor.item().is_some() {
            self.cursor.next();
        }

        self.cursor.item().and_then(|transform| {
            if let Some(c) = transform.display_text {
                self.offset += 1;
                Some(c)
            } else {
                let overshoot = self.offset - self.cursor.start().display.chars;
                let buffer_start = self.cursor.start().buffer.chars + overshoot;
                let char_count = self.cursor.end().buffer.chars - buffer_start;
                self.buffer_chars =
                    Some(self.buffer.chars_at(buffer_start).unwrap().take(char_count));
                self.next()
            }
        })
    }
}

impl<'a> Dimension<'a, TransformSummary> for DisplayPoint {
    fn add_summary(&mut self, summary: &'a TransformSummary) {
        self.0 += &summary.display.lines;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct DisplayOffset(usize);

impl<'a> Dimension<'a, TransformSummary> for DisplayOffset {
    fn add_summary(&mut self, summary: &'a TransformSummary) {
        self.0 += &summary.display.chars;
    }
}

impl<'a> Dimension<'a, TransformSummary> for Point {
    fn add_summary(&mut self, summary: &'a TransformSummary) {
        *self += &summary.buffer.lines;
    }
}

impl<'a> Dimension<'a, TransformSummary> for usize {
    fn add_summary(&mut self, summary: &'a TransformSummary) {
        *self += &summary.buffer.chars;
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use super::*;
    use crate::test::sample_text;
    use buffer::ToPoint;
    use gpui::App;

    #[test]
    fn test_basic_folds() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| Buffer::new(0, sample_text(5, 6), ctx));
            let mut map = FoldMap::new(buffer.clone(), app.as_ref());

            map.fold(
                vec![
                    Point::new(0, 2)..Point::new(2, 2),
                    Point::new(2, 4)..Point::new(4, 1),
                ],
                app.as_ref(),
            )
            .unwrap();
            assert_eq!(map.text(app.as_ref()), "aa…cc…eeeee");

            let edits = buffer.update(app, |buffer, ctx| {
                let start_version = buffer.version.clone();
                buffer
                    .edit(
                        vec![
                            Point::new(0, 0)..Point::new(0, 1),
                            Point::new(2, 3)..Point::new(2, 3),
                        ],
                        "123",
                        Some(ctx),
                    )
                    .unwrap();
                buffer.edits_since(start_version).collect::<Vec<_>>()
            });

            map.apply_edits(&edits, app.as_ref()).unwrap();
            assert_eq!(map.text(app.as_ref()), "123a…c123c…eeeee");

            let edits = buffer.update(app, |buffer, ctx| {
                let start_version = buffer.version.clone();
                buffer
                    .edit(Some(Point::new(2, 6)..Point::new(4, 3)), "456", Some(ctx))
                    .unwrap();
                buffer.edits_since(start_version).collect::<Vec<_>>()
            });

            map.apply_edits(&edits, app.as_ref()).unwrap();
            assert_eq!(map.text(app.as_ref()), "123a…c123456eee");

            map.unfold(Some(Point::new(0, 4)..Point::new(0, 4)), app.as_ref())
                .unwrap();
            assert_eq!(map.text(app.as_ref()), "123aaaaa\nbbbbbb\nccc123456eee");
        });
    }

    #[test]
    fn test_adjacent_folds() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| Buffer::new(0, "abcdefghijkl", ctx));

            {
                let mut map = FoldMap::new(buffer.clone(), app.as_ref());

                map.fold(vec![5..8], app.as_ref()).unwrap();
                map.check_invariants(app.as_ref());
                assert_eq!(map.text(app.as_ref()), "abcde…ijkl");

                // Create an fold adjacent to the start of the first fold.
                map.fold(vec![0..1, 2..5], app.as_ref()).unwrap();
                map.check_invariants(app.as_ref());
                assert_eq!(map.text(app.as_ref()), "…b…ijkl");

                // Create an fold adjacent to the end of the first fold.
                map.fold(vec![11..11, 8..10], app.as_ref()).unwrap();
                map.check_invariants(app.as_ref());
                assert_eq!(map.text(app.as_ref()), "…b…kl");
            }

            {
                let mut map = FoldMap::new(buffer.clone(), app.as_ref());

                // Create two adjacent folds.
                map.fold(vec![0..2, 2..5], app.as_ref()).unwrap();
                map.check_invariants(app.as_ref());
                assert_eq!(map.text(app.as_ref()), "…fghijkl");

                // Edit within one of the folds.
                let edits = buffer.update(app, |buffer, ctx| {
                    let version = buffer.version();
                    buffer.edit(vec![0..1], "12345", Some(ctx)).unwrap();
                    buffer.edits_since(version).collect::<Vec<_>>()
                });
                map.apply_edits(edits.as_slice(), app.as_ref()).unwrap();
                map.check_invariants(app.as_ref());
                assert_eq!(map.text(app.as_ref()), "12345…fghijkl");
            }
        });
    }

    #[test]
    fn test_overlapping_folds() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| Buffer::new(0, sample_text(5, 6), ctx));
            let mut map = FoldMap::new(buffer.clone(), app.as_ref());
            map.fold(
                vec![
                    Point::new(0, 2)..Point::new(2, 2),
                    Point::new(0, 4)..Point::new(1, 0),
                    Point::new(1, 2)..Point::new(3, 2),
                    Point::new(3, 1)..Point::new(4, 1),
                ],
                app.as_ref(),
            )
            .unwrap();
            assert_eq!(map.text(app.as_ref()), "aa…eeeee");
        })
    }

    #[test]
    fn test_merging_folds_via_edit() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| Buffer::new(0, sample_text(5, 6), ctx));
            let mut map = FoldMap::new(buffer.clone(), app.as_ref());

            map.fold(
                vec![
                    Point::new(0, 2)..Point::new(2, 2),
                    Point::new(3, 1)..Point::new(4, 1),
                ],
                app.as_ref(),
            )
            .unwrap();
            assert_eq!(map.text(app.as_ref()), "aa…cccc\nd…eeeee");

            let edits = buffer.update(app, |buffer, ctx| {
                let start_version = buffer.version.clone();
                buffer
                    .edit(Some(Point::new(2, 2)..Point::new(3, 1)), "", Some(ctx))
                    .unwrap();
                buffer.edits_since(start_version).collect::<Vec<_>>()
            });

            map.apply_edits(&edits, app.as_ref()).unwrap();
            assert_eq!(map.text(app.as_ref()), "aa…eeeee");
        });
    }

    #[test]
    fn test_folds_in_range() {
        App::test((), |app| {
            let buffer = app.add_model(|ctx| Buffer::new(0, sample_text(5, 6), ctx));
            let mut map = FoldMap::new(buffer.clone(), app.as_ref());
            let buffer = buffer.read(app);

            map.fold(
                vec![
                    Point::new(0, 2)..Point::new(2, 2),
                    Point::new(0, 4)..Point::new(1, 0),
                    Point::new(1, 2)..Point::new(3, 2),
                    Point::new(3, 1)..Point::new(4, 1),
                ],
                app.as_ref(),
            )
            .unwrap();
            let fold_ranges = map
                .folds_in_range(Point::new(1, 0)..Point::new(1, 3), app.as_ref())
                .unwrap()
                .map(|fold| {
                    fold.start.to_point(buffer).unwrap()..fold.end.to_point(buffer).unwrap()
                })
                .collect::<Vec<_>>();
            assert_eq!(
                fold_ranges,
                vec![
                    Point::new(0, 2)..Point::new(2, 2),
                    Point::new(1, 2)..Point::new(3, 2)
                ]
            );
        });
    }

    #[test]
    fn test_random_folds() {
        use crate::editor::ToPoint;
        use crate::util::RandomCharIter;
        use rand::prelude::*;
        use std::env;

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
            println!("{:?}", seed);
            let mut rng = StdRng::seed_from_u64(seed);

            App::test((), |app| {
                let buffer = app.add_model(|ctx| {
                    let len = rng.gen_range(0..10);
                    let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
                    Buffer::new(0, text, ctx)
                });
                let mut map = FoldMap::new(buffer.clone(), app.as_ref());

                for _ in 0..operations {
                    log::info!("text: {:?}", buffer.read(app).text());
                    if rng.gen() {
                        let buffer = buffer.read(app);

                        let fold_count = rng.gen_range(1..=5);
                        let mut fold_ranges: Vec<Range<usize>> = Vec::new();
                        for _ in 0..fold_count {
                            let end = rng.gen_range(0..buffer.len() + 1);
                            let start = rng.gen_range(0..end + 1);
                            fold_ranges.push(start..end);
                        }
                        log::info!("folding {:?}", fold_ranges);
                        map.fold(fold_ranges.clone(), app.as_ref()).unwrap();
                    } else {
                        let edits = buffer.update(app, |buffer, ctx| {
                            let start_version = buffer.version.clone();
                            let edit_count = rng.gen_range(1..=5);
                            buffer.randomly_edit(&mut rng, edit_count, Some(ctx));
                            buffer.edits_since(start_version).collect::<Vec<_>>()
                        });
                        log::info!("editing {:?}", edits);
                        map.apply_edits(&edits, app.as_ref()).unwrap();
                    }
                    map.check_invariants(app.as_ref());

                    let buffer = map.buffer.read(app);
                    let mut expected_text = buffer.text();
                    let mut expected_buffer_rows = Vec::new();
                    let mut next_row = buffer.max_point().row;
                    for fold_range in map.merged_fold_ranges(app.as_ref()).into_iter().rev() {
                        let fold_start = buffer.point_for_offset(fold_range.start).unwrap();
                        let fold_end = buffer.point_for_offset(fold_range.end).unwrap();
                        expected_buffer_rows.extend((fold_end.row + 1..=next_row).rev());
                        next_row = fold_start.row;

                        expected_text.replace_range(fold_range.start..fold_range.end, "…");
                    }
                    expected_buffer_rows.extend((0..=next_row).rev());
                    expected_buffer_rows.reverse();

                    assert_eq!(map.text(app.as_ref()), expected_text);

                    for (display_row, line) in expected_text.lines().enumerate() {
                        let line_len = map.line_len(display_row as u32, app.as_ref()).unwrap();
                        assert_eq!(line_len, line.chars().count() as u32);
                    }

                    let mut display_point = DisplayPoint::new(0, 0);
                    let mut display_offset = DisplayOffset(0);
                    for c in expected_text.chars() {
                        let buffer_point = map.to_buffer_point(display_point);
                        let buffer_offset = buffer_point.to_offset(buffer).unwrap();
                        assert_eq!(map.to_display_point(buffer_point), display_point);
                        assert_eq!(
                            map.to_buffer_offset(display_point, app.as_ref()).unwrap(),
                            buffer_offset
                        );
                        assert_eq!(
                            map.to_display_offset(display_point, app.as_ref()).unwrap(),
                            display_offset
                        );

                        if c == '\n' {
                            *display_point.row_mut() += 1;
                            *display_point.column_mut() = 0;
                        } else {
                            *display_point.column_mut() += 1;
                        }
                        display_offset.0 += 1;
                    }

                    for (idx, buffer_row) in expected_buffer_rows.iter().enumerate() {
                        let display_row = map.to_display_point(Point::new(*buffer_row, 0)).row();
                        assert_eq!(
                            map.buffer_rows(display_row).unwrap().collect::<Vec<_>>(),
                            expected_buffer_rows[idx..],
                        );
                    }

                    for fold_range in map.merged_fold_ranges(app.as_ref()) {
                        let display_point =
                            map.to_display_point(fold_range.start.to_point(buffer).unwrap());
                        assert!(map.is_line_folded(display_point.row()));
                    }
                }
            });
        }
    }

    #[test]
    fn test_buffer_rows() {
        App::test((), |app| {
            let text = sample_text(6, 6) + "\n";
            let buffer = app.add_model(|ctx| Buffer::new(0, text, ctx));

            let mut map = FoldMap::new(buffer.clone(), app.as_ref());

            map.fold(
                vec![
                    Point::new(0, 2)..Point::new(2, 2),
                    Point::new(3, 1)..Point::new(4, 1),
                ],
                app.as_ref(),
            )
            .unwrap();

            assert_eq!(map.text(app.as_ref()), "aa…cccc\nd…eeeee\nffffff\n");
            assert_eq!(
                map.buffer_rows(0).unwrap().collect::<Vec<_>>(),
                vec![0, 3, 5, 6]
            );
            assert_eq!(map.buffer_rows(3).unwrap().collect::<Vec<_>>(), vec![6]);
        });
    }

    impl FoldMap {
        fn text(&self, app: &AppContext) -> String {
            self.chars_at(DisplayPoint(Point::zero()), app)
                .unwrap()
                .collect()
        }

        fn merged_fold_ranges(&self, app: &AppContext) -> Vec<Range<usize>> {
            let buffer = self.buffer.read(app);
            let mut fold_ranges = self
                .folds
                .iter()
                .map(|fold| {
                    fold.start.to_offset(buffer).unwrap()..fold.end.to_offset(buffer).unwrap()
                })
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

        fn check_invariants(&self, app: &AppContext) {
            let buffer = self.buffer.read(app);
            assert_eq!(
                self.transforms.summary().buffer.chars,
                buffer.len(),
                "transform tree does not match buffer's length"
            );

            let mut fold_ranges = Vec::new();
            let mut sorted_fold_ranges = Vec::new();
            for fold in &self.folds {
                let start = fold.start.to_offset(buffer).unwrap();
                let end = fold.end.to_offset(buffer).unwrap();
                fold_ranges.push(start..end);
                sorted_fold_ranges.push(start..end);
            }
            sorted_fold_ranges.sort_by_key(|fold| (fold.start, Reverse(fold.end)));
            assert_eq!(fold_ranges, sorted_fold_ranges);
        }
    }
}

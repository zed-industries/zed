use super::{
    fold_map::{FoldBufferRows, FoldChunks, FoldEdit, FoldOffset, FoldPoint, FoldSnapshot},
    TextHighlights,
};
use crate::{MultiBufferSnapshot, ToPoint};
use gpui::fonts::HighlightStyle;
use language::{Bias, Chunk, Edit, Patch, Point, Rope, TextSummary};
use parking_lot::Mutex;
use std::{
    cmp,
    ops::{Add, AddAssign, Range, Sub},
};
use util::post_inc;

pub type SuggestionEdit = Edit<SuggestionOffset>;

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct SuggestionOffset(pub usize);

impl Add for SuggestionOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for SuggestionOffset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign for SuggestionOffset {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct SuggestionPoint(pub Point);

impl SuggestionPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(Point::new(row, column))
    }

    pub fn row(self) -> u32 {
        self.0.row
    }

    pub fn column(self) -> u32 {
        self.0.column
    }
}

#[derive(Clone, Debug)]
pub struct Suggestion<T> {
    pub position: T,
    pub text: Rope,
}

pub struct SuggestionMap(Mutex<SuggestionSnapshot>);

impl SuggestionMap {
    pub fn new(fold_snapshot: FoldSnapshot) -> (Self, SuggestionSnapshot) {
        let snapshot = SuggestionSnapshot {
            fold_snapshot,
            suggestion: None,
            version: 0,
        };
        (Self(Mutex::new(snapshot.clone())), snapshot)
    }

    pub fn replace<T>(
        &self,
        new_suggestion: Option<Suggestion<T>>,
        fold_snapshot: FoldSnapshot,
        fold_edits: Vec<FoldEdit>,
    ) -> (
        SuggestionSnapshot,
        Vec<SuggestionEdit>,
        Option<Suggestion<FoldOffset>>,
    )
    where
        T: ToPoint,
    {
        let new_suggestion = new_suggestion.map(|new_suggestion| {
            let buffer_point = new_suggestion
                .position
                .to_point(fold_snapshot.buffer_snapshot());
            let fold_point = fold_snapshot.to_fold_point(buffer_point, Bias::Left);
            let fold_offset = fold_point.to_offset(&fold_snapshot);
            Suggestion {
                position: fold_offset,
                text: new_suggestion.text,
            }
        });

        let (_, edits) = self.sync(fold_snapshot, fold_edits);
        let mut snapshot = self.0.lock();

        let mut patch = Patch::new(edits);
        let old_suggestion = snapshot.suggestion.take();
        if let Some(suggestion) = &old_suggestion {
            patch = patch.compose([SuggestionEdit {
                old: SuggestionOffset(suggestion.position.0)
                    ..SuggestionOffset(suggestion.position.0 + suggestion.text.len()),
                new: SuggestionOffset(suggestion.position.0)
                    ..SuggestionOffset(suggestion.position.0),
            }]);
        }

        if let Some(suggestion) = new_suggestion.as_ref() {
            patch = patch.compose([SuggestionEdit {
                old: SuggestionOffset(suggestion.position.0)
                    ..SuggestionOffset(suggestion.position.0),
                new: SuggestionOffset(suggestion.position.0)
                    ..SuggestionOffset(suggestion.position.0 + suggestion.text.len()),
            }]);
        }

        snapshot.suggestion = new_suggestion;
        snapshot.version += 1;
        (snapshot.clone(), patch.into_inner(), old_suggestion)
    }

    pub fn sync(
        &self,
        fold_snapshot: FoldSnapshot,
        fold_edits: Vec<FoldEdit>,
    ) -> (SuggestionSnapshot, Vec<SuggestionEdit>) {
        let mut snapshot = self.0.lock();

        if snapshot.fold_snapshot.version != fold_snapshot.version {
            snapshot.version += 1;
        }

        let mut suggestion_edits = Vec::new();

        let mut suggestion_old_len = 0;
        let mut suggestion_new_len = 0;
        for fold_edit in fold_edits {
            let start = fold_edit.new.start;
            let end = FoldOffset(start.0 + fold_edit.old_len().0);
            if let Some(suggestion) = snapshot.suggestion.as_mut() {
                if end <= suggestion.position {
                    suggestion.position.0 += fold_edit.new_len().0;
                    suggestion.position.0 -= fold_edit.old_len().0;
                } else if start > suggestion.position {
                    suggestion_old_len = suggestion.text.len();
                    suggestion_new_len = suggestion_old_len;
                } else {
                    suggestion_old_len = suggestion.text.len();
                    snapshot.suggestion.take();
                    suggestion_edits.push(SuggestionEdit {
                        old: SuggestionOffset(fold_edit.old.start.0)
                            ..SuggestionOffset(fold_edit.old.end.0 + suggestion_old_len),
                        new: SuggestionOffset(fold_edit.new.start.0)
                            ..SuggestionOffset(fold_edit.new.end.0),
                    });
                    continue;
                }
            }

            suggestion_edits.push(SuggestionEdit {
                old: SuggestionOffset(fold_edit.old.start.0 + suggestion_old_len)
                    ..SuggestionOffset(fold_edit.old.end.0 + suggestion_old_len),
                new: SuggestionOffset(fold_edit.new.start.0 + suggestion_new_len)
                    ..SuggestionOffset(fold_edit.new.end.0 + suggestion_new_len),
            });
        }
        snapshot.fold_snapshot = fold_snapshot;

        (snapshot.clone(), suggestion_edits)
    }

    pub fn has_suggestion(&self) -> bool {
        let snapshot = self.0.lock();
        snapshot.suggestion.is_some()
    }
}

#[derive(Clone)]
pub struct SuggestionSnapshot {
    pub fold_snapshot: FoldSnapshot,
    pub suggestion: Option<Suggestion<FoldOffset>>,
    pub version: usize,
}

impl SuggestionSnapshot {
    pub fn buffer_snapshot(&self) -> &MultiBufferSnapshot {
        self.fold_snapshot.buffer_snapshot()
    }

    pub fn max_point(&self) -> SuggestionPoint {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let suggestion_point = suggestion.position.to_point(&self.fold_snapshot);
            let mut max_point = suggestion_point.0;
            max_point += suggestion.text.max_point();
            max_point += self.fold_snapshot.max_point().0 - suggestion_point.0;
            SuggestionPoint(max_point)
        } else {
            SuggestionPoint(self.fold_snapshot.max_point().0)
        }
    }

    pub fn len(&self) -> SuggestionOffset {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let mut len = suggestion.position.0;
            len += suggestion.text.len();
            len += self.fold_snapshot.len().0 - suggestion.position.0;
            SuggestionOffset(len)
        } else {
            SuggestionOffset(self.fold_snapshot.len().0)
        }
    }

    pub fn line_len(&self, row: u32) -> u32 {
        if let Some(suggestion) = &self.suggestion {
            let suggestion_start = suggestion.position.to_point(&self.fold_snapshot).0;
            let suggestion_end = suggestion_start + suggestion.text.max_point();

            if row < suggestion_start.row {
                self.fold_snapshot.line_len(row)
            } else if row > suggestion_end.row {
                self.fold_snapshot
                    .line_len(suggestion_start.row + (row - suggestion_end.row))
            } else {
                let mut result = suggestion.text.line_len(row - suggestion_start.row);
                if row == suggestion_start.row {
                    result += suggestion_start.column;
                }
                if row == suggestion_end.row {
                    result +=
                        self.fold_snapshot.line_len(suggestion_start.row) - suggestion_start.column;
                }
                result
            }
        } else {
            self.fold_snapshot.line_len(row)
        }
    }

    pub fn clip_point(&self, point: SuggestionPoint, bias: Bias) -> SuggestionPoint {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let suggestion_start = suggestion.position.to_point(&self.fold_snapshot).0;
            let suggestion_end = suggestion_start + suggestion.text.max_point();
            if point.0 <= suggestion_start {
                SuggestionPoint(self.fold_snapshot.clip_point(FoldPoint(point.0), bias).0)
            } else if point.0 > suggestion_end {
                let fold_point = self.fold_snapshot.clip_point(
                    FoldPoint(suggestion_start + (point.0 - suggestion_end)),
                    bias,
                );
                let suggestion_point = suggestion_end + (fold_point.0 - suggestion_start);
                if bias == Bias::Left && suggestion_point == suggestion_end {
                    SuggestionPoint(suggestion_start)
                } else {
                    SuggestionPoint(suggestion_point)
                }
            } else if bias == Bias::Left || suggestion_start == self.fold_snapshot.max_point().0 {
                SuggestionPoint(suggestion_start)
            } else {
                let fold_point = if self.fold_snapshot.line_len(suggestion_start.row)
                    > suggestion_start.column
                {
                    FoldPoint(suggestion_start + Point::new(0, 1))
                } else {
                    FoldPoint(suggestion_start + Point::new(1, 0))
                };
                let clipped_fold_point = self.fold_snapshot.clip_point(fold_point, bias);
                SuggestionPoint(suggestion_end + (clipped_fold_point.0 - suggestion_start))
            }
        } else {
            SuggestionPoint(self.fold_snapshot.clip_point(FoldPoint(point.0), bias).0)
        }
    }

    pub fn to_offset(&self, point: SuggestionPoint) -> SuggestionOffset {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let suggestion_start = suggestion.position.to_point(&self.fold_snapshot).0;
            let suggestion_end = suggestion_start + suggestion.text.max_point();

            if point.0 <= suggestion_start {
                SuggestionOffset(FoldPoint(point.0).to_offset(&self.fold_snapshot).0)
            } else if point.0 > suggestion_end {
                let fold_offset = FoldPoint(suggestion_start + (point.0 - suggestion_end))
                    .to_offset(&self.fold_snapshot);
                SuggestionOffset(fold_offset.0 + suggestion.text.len())
            } else {
                let offset_in_suggestion =
                    suggestion.text.point_to_offset(point.0 - suggestion_start);
                SuggestionOffset(suggestion.position.0 + offset_in_suggestion)
            }
        } else {
            SuggestionOffset(FoldPoint(point.0).to_offset(&self.fold_snapshot).0)
        }
    }

    pub fn to_point(&self, offset: SuggestionOffset) -> SuggestionPoint {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let suggestion_point_start = suggestion.position.to_point(&self.fold_snapshot).0;
            if offset.0 <= suggestion.position.0 {
                SuggestionPoint(FoldOffset(offset.0).to_point(&self.fold_snapshot).0)
            } else if offset.0 > (suggestion.position.0 + suggestion.text.len()) {
                let fold_point = FoldOffset(offset.0 - suggestion.text.len())
                    .to_point(&self.fold_snapshot)
                    .0;

                SuggestionPoint(
                    suggestion_point_start
                        + suggestion.text.max_point()
                        + (fold_point - suggestion_point_start),
                )
            } else {
                let point_in_suggestion = suggestion
                    .text
                    .offset_to_point(offset.0 - suggestion.position.0);
                SuggestionPoint(suggestion_point_start + point_in_suggestion)
            }
        } else {
            SuggestionPoint(FoldOffset(offset.0).to_point(&self.fold_snapshot).0)
        }
    }

    pub fn to_fold_point(&self, point: SuggestionPoint) -> FoldPoint {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let suggestion_start = suggestion.position.to_point(&self.fold_snapshot).0;
            let suggestion_end = suggestion_start + suggestion.text.max_point();

            if point.0 <= suggestion_start {
                FoldPoint(point.0)
            } else if point.0 > suggestion_end {
                FoldPoint(suggestion_start + (point.0 - suggestion_end))
            } else {
                FoldPoint(suggestion_start)
            }
        } else {
            FoldPoint(point.0)
        }
    }

    pub fn to_suggestion_point(&self, point: FoldPoint) -> SuggestionPoint {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let suggestion_start = suggestion.position.to_point(&self.fold_snapshot).0;

            if point.0 <= suggestion_start {
                SuggestionPoint(point.0)
            } else {
                let suggestion_end = suggestion_start + suggestion.text.max_point();
                SuggestionPoint(suggestion_end + (point.0 - suggestion_start))
            }
        } else {
            SuggestionPoint(point.0)
        }
    }

    pub fn text_summary_for_range(&self, range: Range<SuggestionPoint>) -> TextSummary {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let suggestion_start = suggestion.position.to_point(&self.fold_snapshot).0;
            let suggestion_end = suggestion_start + suggestion.text.max_point();
            let mut summary = TextSummary::default();

            let prefix_range =
                cmp::min(range.start.0, suggestion_start)..cmp::min(range.end.0, suggestion_start);
            if prefix_range.start < prefix_range.end {
                summary += self.fold_snapshot.text_summary_for_range(
                    FoldPoint(prefix_range.start)..FoldPoint(prefix_range.end),
                );
            }

            let suggestion_range =
                cmp::max(range.start.0, suggestion_start)..cmp::min(range.end.0, suggestion_end);
            if suggestion_range.start < suggestion_range.end {
                let point_range = suggestion_range.start - suggestion_start
                    ..suggestion_range.end - suggestion_start;
                let offset_range = suggestion.text.point_to_offset(point_range.start)
                    ..suggestion.text.point_to_offset(point_range.end);
                summary += suggestion
                    .text
                    .cursor(offset_range.start)
                    .summary::<TextSummary>(offset_range.end);
            }

            let suffix_range = cmp::max(range.start.0, suggestion_end)..range.end.0;
            if suffix_range.start < suffix_range.end {
                let start = suggestion_start + (suffix_range.start - suggestion_end);
                let end = suggestion_start + (suffix_range.end - suggestion_end);
                summary += self
                    .fold_snapshot
                    .text_summary_for_range(FoldPoint(start)..FoldPoint(end));
            }

            summary
        } else {
            self.fold_snapshot
                .text_summary_for_range(FoldPoint(range.start.0)..FoldPoint(range.end.0))
        }
    }

    pub fn chars_at(&self, start: SuggestionPoint) -> impl '_ + Iterator<Item = char> {
        let start = self.to_offset(start);
        self.chunks(start..self.len(), false, None, None)
            .flat_map(|chunk| chunk.text.chars())
    }

    pub fn chunks<'a>(
        &'a self,
        range: Range<SuggestionOffset>,
        language_aware: bool,
        text_highlights: Option<&'a TextHighlights>,
        suggestion_highlight: Option<HighlightStyle>,
    ) -> SuggestionChunks<'a> {
        if let Some(suggestion) = self.suggestion.as_ref() {
            let suggestion_range =
                suggestion.position.0..suggestion.position.0 + suggestion.text.len();

            let prefix_chunks = if range.start.0 < suggestion_range.start {
                Some(self.fold_snapshot.chunks(
                    FoldOffset(range.start.0)
                        ..cmp::min(FoldOffset(suggestion_range.start), FoldOffset(range.end.0)),
                    language_aware,
                    text_highlights,
                ))
            } else {
                None
            };

            let clipped_suggestion_range = cmp::max(range.start.0, suggestion_range.start)
                ..cmp::min(range.end.0, suggestion_range.end);
            let suggestion_chunks = if clipped_suggestion_range.start < clipped_suggestion_range.end
            {
                let start = clipped_suggestion_range.start - suggestion_range.start;
                let end = clipped_suggestion_range.end - suggestion_range.start;
                Some(suggestion.text.chunks_in_range(start..end))
            } else {
                None
            };

            let suffix_chunks = if range.end.0 > suggestion_range.end {
                let start = cmp::max(suggestion_range.end, range.start.0) - suggestion_range.len();
                let end = range.end.0 - suggestion_range.len();
                Some(self.fold_snapshot.chunks(
                    FoldOffset(start)..FoldOffset(end),
                    language_aware,
                    text_highlights,
                ))
            } else {
                None
            };

            SuggestionChunks {
                prefix_chunks,
                suggestion_chunks,
                suffix_chunks,
                highlight_style: suggestion_highlight,
            }
        } else {
            SuggestionChunks {
                prefix_chunks: Some(self.fold_snapshot.chunks(
                    FoldOffset(range.start.0)..FoldOffset(range.end.0),
                    language_aware,
                    text_highlights,
                )),
                suggestion_chunks: None,
                suffix_chunks: None,
                highlight_style: None,
            }
        }
    }

    pub fn buffer_rows<'a>(&'a self, row: u32) -> SuggestionBufferRows<'a> {
        let suggestion_range = if let Some(suggestion) = self.suggestion.as_ref() {
            let start = suggestion.position.to_point(&self.fold_snapshot).0;
            let end = start + suggestion.text.max_point();
            start.row..end.row
        } else {
            u32::MAX..u32::MAX
        };

        let fold_buffer_rows = if row <= suggestion_range.start {
            self.fold_snapshot.buffer_rows(row)
        } else if row > suggestion_range.end {
            self.fold_snapshot
                .buffer_rows(row - (suggestion_range.end - suggestion_range.start))
        } else {
            let mut rows = self.fold_snapshot.buffer_rows(suggestion_range.start);
            rows.next();
            rows
        };

        SuggestionBufferRows {
            current_row: row,
            suggestion_row_start: suggestion_range.start,
            suggestion_row_end: suggestion_range.end,
            fold_buffer_rows,
        }
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(Default::default()..self.len(), false, None, None)
            .map(|chunk| chunk.text)
            .collect()
    }
}

pub struct SuggestionChunks<'a> {
    prefix_chunks: Option<FoldChunks<'a>>,
    suggestion_chunks: Option<text::Chunks<'a>>,
    suffix_chunks: Option<FoldChunks<'a>>,
    highlight_style: Option<HighlightStyle>,
}

impl<'a> Iterator for SuggestionChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(chunks) = self.prefix_chunks.as_mut() {
            if let Some(chunk) = chunks.next() {
                return Some(chunk);
            } else {
                self.prefix_chunks = None;
            }
        }

        if let Some(chunks) = self.suggestion_chunks.as_mut() {
            if let Some(chunk) = chunks.next() {
                return Some(Chunk {
                    text: chunk,
                    syntax_highlight_id: None,
                    highlight_style: self.highlight_style,
                    diagnostic_severity: None,
                    is_unnecessary: false,
                });
            } else {
                self.suggestion_chunks = None;
            }
        }

        if let Some(chunks) = self.suffix_chunks.as_mut() {
            if let Some(chunk) = chunks.next() {
                return Some(chunk);
            } else {
                self.suffix_chunks = None;
            }
        }

        None
    }
}

#[derive(Clone)]
pub struct SuggestionBufferRows<'a> {
    current_row: u32,
    suggestion_row_start: u32,
    suggestion_row_end: u32,
    fold_buffer_rows: FoldBufferRows<'a>,
}

impl<'a> Iterator for SuggestionBufferRows<'a> {
    type Item = Option<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let row = post_inc(&mut self.current_row);
        if row <= self.suggestion_row_start || row > self.suggestion_row_end {
            self.fold_buffer_rows.next()
        } else {
            Some(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{display_map::fold_map::FoldMap, MultiBuffer};
    use gpui::MutableAppContext;
    use rand::{prelude::StdRng, Rng};
    use settings::Settings;
    use std::{
        env,
        ops::{Bound, RangeBounds},
    };

    #[gpui::test]
    fn test_basic(cx: &mut MutableAppContext) {
        let buffer = MultiBuffer::build_simple("abcdefghi", cx);
        let buffer_edits = buffer.update(cx, |buffer, _| buffer.subscribe());
        let (mut fold_map, fold_snapshot) = FoldMap::new(buffer.read(cx).snapshot(cx));
        let (suggestion_map, suggestion_snapshot) = SuggestionMap::new(fold_snapshot.clone());
        assert_eq!(suggestion_snapshot.text(), "abcdefghi");

        let (suggestion_snapshot, _, _) = suggestion_map.replace(
            Some(Suggestion {
                position: 3,
                text: "123\n456".into(),
            }),
            fold_snapshot,
            Default::default(),
        );
        assert_eq!(suggestion_snapshot.text(), "abc123\n456defghi");

        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [(0..0, "ABC"), (3..3, "DEF"), (4..4, "GHI"), (9..9, "JKL")],
                None,
                cx,
            )
        });
        let (fold_snapshot, fold_edits) = fold_map.read(
            buffer.read(cx).snapshot(cx),
            buffer_edits.consume().into_inner(),
        );
        let (suggestion_snapshot, _) = suggestion_map.sync(fold_snapshot.clone(), fold_edits);
        assert_eq!(suggestion_snapshot.text(), "ABCabcDEF123\n456dGHIefghiJKL");

        let (mut fold_map_writer, _, _) =
            fold_map.write(buffer.read(cx).snapshot(cx), Default::default());
        let (fold_snapshot, fold_edits) = fold_map_writer.fold([0..3]);
        let (suggestion_snapshot, _) = suggestion_map.sync(fold_snapshot, fold_edits);
        assert_eq!(suggestion_snapshot.text(), "⋯abcDEF123\n456dGHIefghiJKL");

        let (mut fold_map_writer, _, _) =
            fold_map.write(buffer.read(cx).snapshot(cx), Default::default());
        let (fold_snapshot, fold_edits) = fold_map_writer.fold([6..10]);
        let (suggestion_snapshot, _) = suggestion_map.sync(fold_snapshot, fold_edits);
        assert_eq!(suggestion_snapshot.text(), "⋯abc⋯GHIefghiJKL");
    }

    #[gpui::test(iterations = 100)]
    fn test_random_suggestions(cx: &mut MutableAppContext, mut rng: StdRng) {
        cx.set_global(Settings::test(cx));
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let len = rng.gen_range(0..30);
        let buffer = if rng.gen() {
            let text = util::RandomCharIter::new(&mut rng)
                .take(len)
                .collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };
        let mut buffer_snapshot = buffer.read(cx).snapshot(cx);
        log::info!("buffer text: {:?}", buffer_snapshot.text());

        let (mut fold_map, mut fold_snapshot) = FoldMap::new(buffer_snapshot.clone());
        let (suggestion_map, mut suggestion_snapshot) = SuggestionMap::new(fold_snapshot.clone());

        for _ in 0..operations {
            let mut suggestion_edits = Patch::default();

            let mut prev_suggestion_text = suggestion_snapshot.text();
            let mut buffer_edits = Vec::new();
            match rng.gen_range(0..=100) {
                0..=29 => {
                    let (_, edits) = suggestion_map.randomly_mutate(&mut rng);
                    suggestion_edits = suggestion_edits.compose(edits);
                }
                30..=59 => {
                    for (new_fold_snapshot, fold_edits) in fold_map.randomly_mutate(&mut rng) {
                        fold_snapshot = new_fold_snapshot;
                        let (_, edits) = suggestion_map.sync(fold_snapshot.clone(), fold_edits);
                        suggestion_edits = suggestion_edits.compose(edits);
                    }
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

            let (new_fold_snapshot, fold_edits) =
                fold_map.read(buffer_snapshot.clone(), buffer_edits);
            fold_snapshot = new_fold_snapshot;
            let (new_suggestion_snapshot, edits) =
                suggestion_map.sync(fold_snapshot.clone(), fold_edits);
            suggestion_snapshot = new_suggestion_snapshot;
            suggestion_edits = suggestion_edits.compose(edits);

            log::info!("buffer text: {:?}", buffer_snapshot.text());
            log::info!("folds text: {:?}", fold_snapshot.text());
            log::info!("suggestions text: {:?}", suggestion_snapshot.text());

            let mut expected_text = Rope::from(fold_snapshot.text().as_str());
            let mut expected_buffer_rows = fold_snapshot.buffer_rows(0).collect::<Vec<_>>();
            if let Some(suggestion) = suggestion_snapshot.suggestion.as_ref() {
                expected_text.replace(
                    suggestion.position.0..suggestion.position.0,
                    &suggestion.text.to_string(),
                );
                let suggestion_start = suggestion.position.to_point(&fold_snapshot).0;
                let suggestion_end = suggestion_start + suggestion.text.max_point();
                expected_buffer_rows.splice(
                    (suggestion_start.row + 1) as usize..(suggestion_start.row + 1) as usize,
                    (0..suggestion_end.row - suggestion_start.row).map(|_| None),
                );
            }
            assert_eq!(suggestion_snapshot.text(), expected_text.to_string());
            for row_start in 0..expected_buffer_rows.len() {
                assert_eq!(
                    suggestion_snapshot
                        .buffer_rows(row_start as u32)
                        .collect::<Vec<_>>(),
                    &expected_buffer_rows[row_start..],
                    "incorrect buffer rows starting at {}",
                    row_start
                );
            }

            for _ in 0..5 {
                let mut end = rng.gen_range(0..=suggestion_snapshot.len().0);
                end = expected_text.clip_offset(end, Bias::Right);
                let mut start = rng.gen_range(0..=end);
                start = expected_text.clip_offset(start, Bias::Right);

                let actual_text = suggestion_snapshot
                    .chunks(
                        SuggestionOffset(start)..SuggestionOffset(end),
                        false,
                        None,
                        None,
                    )
                    .map(|chunk| chunk.text)
                    .collect::<String>();
                assert_eq!(
                    actual_text,
                    expected_text.slice(start..end).to_string(),
                    "incorrect text in range {:?}",
                    start..end
                );

                let start_point = SuggestionPoint(expected_text.offset_to_point(start));
                let end_point = SuggestionPoint(expected_text.offset_to_point(end));
                assert_eq!(
                    suggestion_snapshot.text_summary_for_range(start_point..end_point),
                    expected_text.slice(start..end).summary()
                );
            }

            for edit in suggestion_edits.into_inner() {
                prev_suggestion_text.replace_range(
                    edit.new.start.0..edit.new.start.0 + edit.old_len().0,
                    &suggestion_snapshot.text()[edit.new.start.0..edit.new.end.0],
                );
            }
            assert_eq!(prev_suggestion_text, suggestion_snapshot.text());

            assert_eq!(expected_text.max_point(), suggestion_snapshot.max_point().0);
            assert_eq!(expected_text.len(), suggestion_snapshot.len().0);

            let mut suggestion_point = SuggestionPoint::default();
            let mut suggestion_offset = SuggestionOffset::default();
            for ch in expected_text.chars() {
                assert_eq!(
                    suggestion_snapshot.to_offset(suggestion_point),
                    suggestion_offset,
                    "invalid to_offset({:?})",
                    suggestion_point
                );
                assert_eq!(
                    suggestion_snapshot.to_point(suggestion_offset),
                    suggestion_point,
                    "invalid to_point({:?})",
                    suggestion_offset
                );
                assert_eq!(
                    suggestion_snapshot
                        .to_suggestion_point(suggestion_snapshot.to_fold_point(suggestion_point)),
                    suggestion_snapshot.clip_point(suggestion_point, Bias::Left),
                );

                let mut bytes = [0; 4];
                for byte in ch.encode_utf8(&mut bytes).as_bytes() {
                    suggestion_offset.0 += 1;
                    if *byte == b'\n' {
                        suggestion_point.0 += Point::new(1, 0);
                    } else {
                        suggestion_point.0 += Point::new(0, 1);
                    }

                    let clipped_left_point =
                        suggestion_snapshot.clip_point(suggestion_point, Bias::Left);
                    let clipped_right_point =
                        suggestion_snapshot.clip_point(suggestion_point, Bias::Right);
                    assert!(
                        clipped_left_point <= clipped_right_point,
                        "clipped left point {:?} is greater than clipped right point {:?}",
                        clipped_left_point,
                        clipped_right_point
                    );
                    assert_eq!(
                        clipped_left_point.0,
                        expected_text.clip_point(clipped_left_point.0, Bias::Left)
                    );
                    assert_eq!(
                        clipped_right_point.0,
                        expected_text.clip_point(clipped_right_point.0, Bias::Right)
                    );
                    assert!(clipped_left_point <= suggestion_snapshot.max_point());
                    assert!(clipped_right_point <= suggestion_snapshot.max_point());

                    if let Some(suggestion) = suggestion_snapshot.suggestion.as_ref() {
                        let suggestion_start = suggestion.position.to_point(&fold_snapshot).0;
                        let suggestion_end = suggestion_start + suggestion.text.max_point();
                        let invalid_range = (
                            Bound::Excluded(suggestion_start),
                            Bound::Included(suggestion_end),
                        );
                        assert!(
                            !invalid_range.contains(&clipped_left_point.0),
                            "clipped left point {:?} is inside invalid suggestion range {:?}",
                            clipped_left_point,
                            invalid_range
                        );
                        assert!(
                            !invalid_range.contains(&clipped_right_point.0),
                            "clipped right point {:?} is inside invalid suggestion range {:?}",
                            clipped_right_point,
                            invalid_range
                        );
                    }
                }
            }
        }
    }

    impl SuggestionMap {
        pub fn randomly_mutate(
            &self,
            rng: &mut impl Rng,
        ) -> (SuggestionSnapshot, Vec<SuggestionEdit>) {
            let fold_snapshot = self.0.lock().fold_snapshot.clone();
            let new_suggestion = if rng.gen_bool(0.3) {
                None
            } else {
                let index = rng.gen_range(0..=fold_snapshot.buffer_snapshot().len());
                let len = rng.gen_range(0..30);
                Some(Suggestion {
                    position: index,
                    text: util::RandomCharIter::new(rng)
                        .take(len)
                        .filter(|ch| *ch != '\r')
                        .collect::<String>()
                        .as_str()
                        .into(),
                })
            };

            log::info!("replacing suggestion with {:?}", new_suggestion);
            let (snapshot, edits, _) =
                self.replace(new_suggestion, fold_snapshot, Default::default());
            (snapshot, edits)
        }
    }
}

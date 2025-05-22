use super::{
    Highlights,
    fold_map::{self, Chunk, FoldChunks, FoldEdit, FoldPoint, FoldSnapshot},
};
use language::Point;
use multi_buffer::MultiBufferSnapshot;
use std::{cmp, mem, num::NonZeroU32, ops::Range};
use sum_tree::Bias;

const MAX_EXPANSION_COLUMN: u32 = 256;

/// Keeps track of hard tabs in a text buffer.
///
/// See the [`display_map` module documentation](crate::display_map) for more information.
pub struct TabMap(TabSnapshot);

impl TabMap {
    pub fn new(fold_snapshot: FoldSnapshot, tab_size: NonZeroU32) -> (Self, TabSnapshot) {
        let snapshot = TabSnapshot {
            fold_snapshot,
            tab_size,
            max_expansion_column: MAX_EXPANSION_COLUMN,
            version: 0,
        };
        (Self(snapshot.clone()), snapshot)
    }

    #[cfg(test)]
    pub fn set_max_expansion_column(&mut self, column: u32) -> TabSnapshot {
        self.0.max_expansion_column = column;
        self.0.clone()
    }

    pub fn sync(
        &mut self,
        fold_snapshot: FoldSnapshot,
        mut fold_edits: Vec<FoldEdit>,
        tab_size: NonZeroU32,
    ) -> (TabSnapshot, Vec<TabEdit>) {
        let old_snapshot = &mut self.0;
        let mut new_snapshot = TabSnapshot {
            fold_snapshot,
            tab_size,
            max_expansion_column: old_snapshot.max_expansion_column,
            version: old_snapshot.version,
        };

        if old_snapshot.fold_snapshot.version != new_snapshot.fold_snapshot.version {
            new_snapshot.version += 1;
        }

        let mut tab_edits = Vec::with_capacity(fold_edits.len());

        if old_snapshot.tab_size == new_snapshot.tab_size {
            // Expand each edit to include the next tab on the same line as the edit,
            // and any subsequent tabs on that line that moved across the tab expansion
            // boundary.
            for fold_edit in &mut fold_edits {
                let old_end = fold_edit.old.end.to_point(&old_snapshot.fold_snapshot);
                let old_end_row_successor_offset = cmp::min(
                    FoldPoint::new(old_end.row() + 1, 0),
                    old_snapshot.fold_snapshot.max_point(),
                )
                .to_offset(&old_snapshot.fold_snapshot);
                let new_end = fold_edit.new.end.to_point(&new_snapshot.fold_snapshot);

                let mut offset_from_edit = 0;
                let mut first_tab_offset = None;
                let mut last_tab_with_changed_expansion_offset = None;
                'outer: for chunk in old_snapshot.fold_snapshot.chunks(
                    fold_edit.old.end..old_end_row_successor_offset,
                    false,
                    Highlights::default(),
                ) {
                    for (ix, _) in chunk.text.match_indices('\t') {
                        let offset_from_edit = offset_from_edit + (ix as u32);
                        if first_tab_offset.is_none() {
                            first_tab_offset = Some(offset_from_edit);
                        }

                        let old_column = old_end.column() + offset_from_edit;
                        let new_column = new_end.column() + offset_from_edit;
                        let was_expanded = old_column < old_snapshot.max_expansion_column;
                        let is_expanded = new_column < new_snapshot.max_expansion_column;
                        if was_expanded != is_expanded {
                            last_tab_with_changed_expansion_offset = Some(offset_from_edit);
                        } else if !was_expanded && !is_expanded {
                            break 'outer;
                        }
                    }

                    offset_from_edit += chunk.text.len() as u32;
                    if old_end.column() + offset_from_edit >= old_snapshot.max_expansion_column
                        && new_end.column() + offset_from_edit >= new_snapshot.max_expansion_column
                    {
                        break;
                    }
                }

                if let Some(offset) = last_tab_with_changed_expansion_offset.or(first_tab_offset) {
                    fold_edit.old.end.0 += offset as usize + 1;
                    fold_edit.new.end.0 += offset as usize + 1;
                }
            }

            let _old_alloc_ptr = fold_edits.as_ptr();
            // Combine any edits that overlap due to the expansion.
            let mut fold_edits = fold_edits.into_iter();
            let fold_edits = if let Some(mut first_edit) = fold_edits.next() {
                // This code relies on reusing allocations from the Vec<_> - at the time of writing .flatten() prevents them.
                #[allow(clippy::filter_map_identity)]
                let mut v: Vec<_> = fold_edits
                    .scan(&mut first_edit, |state, edit| {
                        if state.old.end >= edit.old.start {
                            state.old.end = edit.old.end;
                            state.new.end = edit.new.end;
                            Some(None) // Skip this edit, it's merged
                        } else {
                            let new_state = edit.clone();
                            let result = Some(Some(state.clone())); // Yield the previous edit
                            **state = new_state;
                            result
                        }
                    })
                    .filter_map(|x| x)
                    .collect();
                v.push(first_edit);
                debug_assert_eq!(v.as_ptr(), _old_alloc_ptr, "Fold edits were reallocated");
                v
            } else {
                vec![]
            };

            for fold_edit in fold_edits {
                let old_start = fold_edit.old.start.to_point(&old_snapshot.fold_snapshot);
                let old_end = fold_edit.old.end.to_point(&old_snapshot.fold_snapshot);
                let new_start = fold_edit.new.start.to_point(&new_snapshot.fold_snapshot);
                let new_end = fold_edit.new.end.to_point(&new_snapshot.fold_snapshot);
                tab_edits.push(TabEdit {
                    old: old_snapshot.to_tab_point(old_start)..old_snapshot.to_tab_point(old_end),
                    new: new_snapshot.to_tab_point(new_start)..new_snapshot.to_tab_point(new_end),
                });
            }
        } else {
            new_snapshot.version += 1;
            tab_edits.push(TabEdit {
                old: TabPoint::zero()..old_snapshot.max_point(),
                new: TabPoint::zero()..new_snapshot.max_point(),
            });
        }

        *old_snapshot = new_snapshot;
        (old_snapshot.clone(), tab_edits)
    }
}

#[derive(Clone)]
pub struct TabSnapshot {
    pub fold_snapshot: FoldSnapshot,
    pub tab_size: NonZeroU32,
    pub max_expansion_column: u32,
    pub version: usize,
}

impl TabSnapshot {
    pub fn buffer_snapshot(&self) -> &MultiBufferSnapshot {
        &self.fold_snapshot.inlay_snapshot.buffer
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let max_point = self.max_point();
        if row < max_point.row() {
            self.to_tab_point(FoldPoint::new(row, self.fold_snapshot.line_len(row)))
                .0
                .column
        } else {
            max_point.column()
        }
    }

    pub fn text_summary(&self) -> TextSummary {
        self.text_summary_for_range(TabPoint::zero()..self.max_point())
    }

    pub fn text_summary_for_range(&self, range: Range<TabPoint>) -> TextSummary {
        let input_start = self.to_fold_point(range.start, Bias::Left).0;
        let input_end = self.to_fold_point(range.end, Bias::Right).0;
        let input_summary = self
            .fold_snapshot
            .text_summary_for_range(input_start..input_end);

        let mut first_line_chars = 0;
        let line_end = if range.start.row() == range.end.row() {
            range.end
        } else {
            self.max_point()
        };
        for c in self
            .chunks(range.start..line_end, false, Highlights::default())
            .flat_map(|chunk| chunk.text.chars())
        {
            if c == '\n' {
                break;
            }
            first_line_chars += 1;
        }

        let mut last_line_chars = 0;
        if range.start.row() == range.end.row() {
            last_line_chars = first_line_chars;
        } else {
            for _ in self
                .chunks(
                    TabPoint::new(range.end.row(), 0)..range.end,
                    false,
                    Highlights::default(),
                )
                .flat_map(|chunk| chunk.text.chars())
            {
                last_line_chars += 1;
            }
        }

        TextSummary {
            lines: range.end.0 - range.start.0,
            first_line_chars,
            last_line_chars,
            longest_row: input_summary.longest_row,
            longest_row_chars: input_summary.longest_row_chars,
        }
    }

    pub fn chunks<'a>(
        &'a self,
        range: Range<TabPoint>,
        language_aware: bool,
        highlights: Highlights<'a>,
    ) -> TabChunks<'a> {
        let (input_start, expanded_char_column, to_next_stop) =
            self.to_fold_point(range.start, Bias::Left);
        let input_column = input_start.column();
        let input_start = input_start.to_offset(&self.fold_snapshot);
        let input_end = self
            .to_fold_point(range.end, Bias::Right)
            .0
            .to_offset(&self.fold_snapshot);
        let to_next_stop = if range.start.0 + Point::new(0, to_next_stop) > range.end.0 {
            range.end.column() - range.start.column()
        } else {
            to_next_stop
        };

        TabChunks {
            snapshot: self,
            fold_chunks: self.fold_snapshot.chunks(
                input_start..input_end,
                language_aware,
                highlights,
            ),
            input_column,
            column: expanded_char_column,
            max_expansion_column: self.max_expansion_column,
            output_position: range.start.0,
            max_output_position: range.end.0,
            tab_size: self.tab_size,
            chunk: Chunk {
                text: &SPACES[0..(to_next_stop as usize)],
                is_tab: true,
                ..Default::default()
            },
            inside_leading_tab: to_next_stop > 0,
        }
    }

    pub fn rows(&self, row: u32) -> fold_map::FoldRows<'_> {
        self.fold_snapshot.row_infos(row)
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(
            TabPoint::zero()..self.max_point(),
            false,
            Highlights::default(),
        )
        .map(|chunk| chunk.text)
        .collect()
    }

    pub fn max_point(&self) -> TabPoint {
        self.to_tab_point(self.fold_snapshot.max_point())
    }

    pub fn clip_point(&self, point: TabPoint, bias: Bias) -> TabPoint {
        self.to_tab_point(
            self.fold_snapshot
                .clip_point(self.to_fold_point(point, bias).0, bias),
        )
    }

    pub fn to_tab_point(&self, input: FoldPoint) -> TabPoint {
        let chars = self.fold_snapshot.chars_at(FoldPoint::new(input.row(), 0));
        let expanded = self.expand_tabs(chars, input.column());
        TabPoint::new(input.row(), expanded)
    }

    pub fn to_fold_point(&self, output: TabPoint, bias: Bias) -> (FoldPoint, u32, u32) {
        let chars = self.fold_snapshot.chars_at(FoldPoint::new(output.row(), 0));
        let expanded = output.column();
        let (collapsed, expanded_char_column, to_next_stop) =
            self.collapse_tabs(chars, expanded, bias);
        (
            FoldPoint::new(output.row(), collapsed),
            expanded_char_column,
            to_next_stop,
        )
    }

    pub fn make_tab_point(&self, point: Point, bias: Bias) -> TabPoint {
        let inlay_point = self.fold_snapshot.inlay_snapshot.to_inlay_point(point);
        let fold_point = self.fold_snapshot.to_fold_point(inlay_point, bias);
        self.to_tab_point(fold_point)
    }

    pub fn to_point(&self, point: TabPoint, bias: Bias) -> Point {
        let fold_point = self.to_fold_point(point, bias).0;
        let inlay_point = fold_point.to_inlay_point(&self.fold_snapshot);
        self.fold_snapshot
            .inlay_snapshot
            .to_buffer_point(inlay_point)
    }

    fn expand_tabs(&self, chars: impl Iterator<Item = char>, column: u32) -> u32 {
        let tab_size = self.tab_size.get();

        let mut expanded_chars = 0;
        let mut expanded_bytes = 0;
        let mut collapsed_bytes = 0;
        let end_column = column.min(self.max_expansion_column);
        for c in chars {
            if collapsed_bytes >= end_column {
                break;
            }
            if c == '\t' {
                let tab_len = tab_size - expanded_chars % tab_size;
                expanded_bytes += tab_len;
                expanded_chars += tab_len;
            } else {
                expanded_bytes += c.len_utf8() as u32;
                expanded_chars += 1;
            }
            collapsed_bytes += c.len_utf8() as u32;
        }
        expanded_bytes + column.saturating_sub(collapsed_bytes)
    }

    fn collapse_tabs(
        &self,
        chars: impl Iterator<Item = char>,
        column: u32,
        bias: Bias,
    ) -> (u32, u32, u32) {
        let tab_size = self.tab_size.get();

        let mut expanded_bytes = 0;
        let mut expanded_chars = 0;
        let mut collapsed_bytes = 0;
        for c in chars {
            if expanded_bytes >= column {
                break;
            }
            if collapsed_bytes >= self.max_expansion_column {
                break;
            }

            if c == '\t' {
                let tab_len = tab_size - (expanded_chars % tab_size);
                expanded_chars += tab_len;
                expanded_bytes += tab_len;
                if expanded_bytes > column {
                    expanded_chars -= expanded_bytes - column;
                    return match bias {
                        Bias::Left => (collapsed_bytes, expanded_chars, expanded_bytes - column),
                        Bias::Right => (collapsed_bytes + 1, expanded_chars, 0),
                    };
                }
            } else {
                expanded_chars += 1;
                expanded_bytes += c.len_utf8() as u32;
            }

            if expanded_bytes > column && matches!(bias, Bias::Left) {
                expanded_chars -= 1;
                break;
            }

            collapsed_bytes += c.len_utf8() as u32;
        }
        (
            collapsed_bytes + column.saturating_sub(expanded_bytes),
            expanded_chars,
            0,
        )
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct TabPoint(pub Point);

impl TabPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self(Point::new(row, column))
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
}

impl From<Point> for TabPoint {
    fn from(point: Point) -> Self {
        Self(point)
    }
}

pub type TabEdit = text::Edit<TabPoint>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextSummary {
    pub lines: Point,
    pub first_line_chars: u32,
    pub last_line_chars: u32,
    pub longest_row: u32,
    pub longest_row_chars: u32,
}

impl<'a> From<&'a str> for TextSummary {
    fn from(text: &'a str) -> Self {
        let sum = text::TextSummary::from(text);

        TextSummary {
            lines: sum.lines,
            first_line_chars: sum.first_line_chars,
            last_line_chars: sum.last_line_chars,
            longest_row: sum.longest_row,
            longest_row_chars: sum.longest_row_chars,
        }
    }
}

impl<'a> std::ops::AddAssign<&'a Self> for TextSummary {
    fn add_assign(&mut self, other: &'a Self) {
        let joined_chars = self.last_line_chars + other.first_line_chars;
        if joined_chars > self.longest_row_chars {
            self.longest_row = self.lines.row;
            self.longest_row_chars = joined_chars;
        }
        if other.longest_row_chars > self.longest_row_chars {
            self.longest_row = self.lines.row + other.longest_row;
            self.longest_row_chars = other.longest_row_chars;
        }

        if self.lines.row == 0 {
            self.first_line_chars += other.first_line_chars;
        }

        if other.lines.row == 0 {
            self.last_line_chars += other.first_line_chars;
        } else {
            self.last_line_chars = other.last_line_chars;
        }

        self.lines += &other.lines;
    }
}

// Handles a tab width <= 16
const SPACES: &str = "                ";

pub struct TabChunks<'a> {
    snapshot: &'a TabSnapshot,
    fold_chunks: FoldChunks<'a>,
    chunk: Chunk<'a>,
    column: u32,
    max_expansion_column: u32,
    output_position: Point,
    input_column: u32,
    max_output_position: Point,
    tab_size: NonZeroU32,
    inside_leading_tab: bool,
}

impl TabChunks<'_> {
    pub(crate) fn seek(&mut self, range: Range<TabPoint>) {
        let (input_start, expanded_char_column, to_next_stop) =
            self.snapshot.to_fold_point(range.start, Bias::Left);
        let input_column = input_start.column();
        let input_start = input_start.to_offset(&self.snapshot.fold_snapshot);
        let input_end = self
            .snapshot
            .to_fold_point(range.end, Bias::Right)
            .0
            .to_offset(&self.snapshot.fold_snapshot);
        let to_next_stop = if range.start.0 + Point::new(0, to_next_stop) > range.end.0 {
            range.end.column() - range.start.column()
        } else {
            to_next_stop
        };

        self.fold_chunks.seek(input_start..input_end);
        self.input_column = input_column;
        self.column = expanded_char_column;
        self.output_position = range.start.0;
        self.max_output_position = range.end.0;
        self.chunk = Chunk {
            text: &SPACES[0..(to_next_stop as usize)],
            is_tab: true,
            ..Default::default()
        };
        self.inside_leading_tab = to_next_stop > 0;
    }
}

impl<'a> Iterator for TabChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk.text.is_empty() {
            if let Some(chunk) = self.fold_chunks.next() {
                self.chunk = chunk;
                if self.inside_leading_tab {
                    self.chunk.text = &self.chunk.text[1..];
                    self.inside_leading_tab = false;
                    self.input_column += 1;
                }
            } else {
                return None;
            }
        }

        for (ix, c) in self.chunk.text.char_indices() {
            match c {
                '\t' => {
                    if ix > 0 {
                        let (prefix, suffix) = self.chunk.text.split_at(ix);
                        self.chunk.text = suffix;
                        return Some(Chunk {
                            text: prefix,
                            ..self.chunk.clone()
                        });
                    } else {
                        self.chunk.text = &self.chunk.text[1..];
                        let tab_size = if self.input_column < self.max_expansion_column {
                            self.tab_size.get()
                        } else {
                            1
                        };
                        let mut len = tab_size - self.column % tab_size;
                        let next_output_position = cmp::min(
                            self.output_position + Point::new(0, len),
                            self.max_output_position,
                        );
                        len = next_output_position.column - self.output_position.column;
                        self.column += len;
                        self.input_column += 1;
                        self.output_position = next_output_position;
                        return Some(Chunk {
                            text: &SPACES[..len as usize],
                            is_tab: true,
                            ..self.chunk.clone()
                        });
                    }
                }
                '\n' => {
                    self.column = 0;
                    self.input_column = 0;
                    self.output_position += Point::new(1, 0);
                }
                _ => {
                    self.column += 1;
                    if !self.inside_leading_tab {
                        self.input_column += c.len_utf8() as u32;
                    }
                    self.output_position.column += c.len_utf8() as u32;
                }
            }
        }

        Some(mem::take(&mut self.chunk))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MultiBuffer,
        display_map::{fold_map::FoldMap, inlay_map::InlayMap},
    };
    use rand::{Rng, prelude::StdRng};

    #[gpui::test]
    fn test_expand_tabs(cx: &mut gpui::App) {
        let buffer = MultiBuffer::build_simple("", cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        assert_eq!(tab_snapshot.expand_tabs("\t".chars(), 0), 0);
        assert_eq!(tab_snapshot.expand_tabs("\t".chars(), 1), 4);
        assert_eq!(tab_snapshot.expand_tabs("\ta".chars(), 2), 5);
    }

    #[gpui::test]
    fn test_long_lines(cx: &mut gpui::App) {
        let max_expansion_column = 12;
        let input = "A\tBC\tDEF\tG\tHI\tJ\tK\tL\tM";
        let output = "A   BC  DEF G   HI J K L M";

        let buffer = MultiBuffer::build_simple(input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, mut tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        tab_snapshot.max_expansion_column = max_expansion_column;
        assert_eq!(tab_snapshot.text(), output);

        for (ix, c) in input.char_indices() {
            assert_eq!(
                tab_snapshot
                    .chunks(
                        TabPoint::new(0, ix as u32)..tab_snapshot.max_point(),
                        false,
                        Highlights::default(),
                    )
                    .map(|c| c.text)
                    .collect::<String>(),
                &output[ix..],
                "text from index {ix}"
            );

            if c != '\t' {
                let input_point = Point::new(0, ix as u32);
                let output_point = Point::new(0, output.find(c).unwrap() as u32);
                assert_eq!(
                    tab_snapshot.to_tab_point(FoldPoint(input_point)),
                    TabPoint(output_point),
                    "to_tab_point({input_point:?})"
                );
                assert_eq!(
                    tab_snapshot
                        .to_fold_point(TabPoint(output_point), Bias::Left)
                        .0,
                    FoldPoint(input_point),
                    "to_fold_point({output_point:?})"
                );
            }
        }
    }

    #[gpui::test]
    fn test_long_lines_with_character_spanning_max_expansion_column(cx: &mut gpui::App) {
        let max_expansion_column = 8;
        let input = "abcdefg⋯hij";

        let buffer = MultiBuffer::build_simple(input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, mut tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        tab_snapshot.max_expansion_column = max_expansion_column;
        assert_eq!(tab_snapshot.text(), input);
    }

    #[gpui::test]
    fn test_marking_tabs(cx: &mut gpui::App) {
        let input = "\t \thello";

        let buffer = MultiBuffer::build_simple(input, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot);
        let (_, tab_snapshot) = TabMap::new(fold_snapshot, 4.try_into().unwrap());

        assert_eq!(
            chunks(&tab_snapshot, TabPoint::zero()),
            vec![
                ("    ".to_string(), true),
                (" ".to_string(), false),
                ("   ".to_string(), true),
                ("hello".to_string(), false),
            ]
        );
        assert_eq!(
            chunks(&tab_snapshot, TabPoint::new(0, 2)),
            vec![
                ("  ".to_string(), true),
                (" ".to_string(), false),
                ("   ".to_string(), true),
                ("hello".to_string(), false),
            ]
        );

        fn chunks(snapshot: &TabSnapshot, start: TabPoint) -> Vec<(String, bool)> {
            let mut chunks = Vec::new();
            let mut was_tab = false;
            let mut text = String::new();
            for chunk in snapshot.chunks(start..snapshot.max_point(), false, Highlights::default())
            {
                if chunk.is_tab != was_tab {
                    if !text.is_empty() {
                        chunks.push((mem::take(&mut text), was_tab));
                    }
                    was_tab = chunk.is_tab;
                }
                text.push_str(chunk.text);
            }

            if !text.is_empty() {
                chunks.push((text, was_tab));
            }
            chunks
        }
    }

    #[gpui::test(iterations = 100)]
    fn test_random_tabs(cx: &mut gpui::App, mut rng: StdRng) {
        let tab_size = NonZeroU32::new(rng.gen_range(1..=4)).unwrap();
        let len = rng.gen_range(0..30);
        let buffer = if rng.r#gen() {
            let text = util::RandomCharIter::new(&mut rng)
                .take(len)
                .collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        log::info!("Buffer text: {:?}", buffer_snapshot.text());

        let (mut inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
        log::info!("InlayMap text: {:?}", inlay_snapshot.text());
        let (mut fold_map, _) = FoldMap::new(inlay_snapshot.clone());
        fold_map.randomly_mutate(&mut rng);
        let (fold_snapshot, _) = fold_map.read(inlay_snapshot, vec![]);
        log::info!("FoldMap text: {:?}", fold_snapshot.text());
        let (inlay_snapshot, _) = inlay_map.randomly_mutate(&mut 0, &mut rng);
        log::info!("InlayMap text: {:?}", inlay_snapshot.text());

        let (mut tab_map, _) = TabMap::new(fold_snapshot.clone(), tab_size);
        let tabs_snapshot = tab_map.set_max_expansion_column(32);

        let text = text::Rope::from(tabs_snapshot.text().as_str());
        log::info!(
            "TabMap text (tab size: {}): {:?}",
            tab_size,
            tabs_snapshot.text(),
        );

        for _ in 0..5 {
            let end_row = rng.gen_range(0..=text.max_point().row);
            let end_column = rng.gen_range(0..=text.line_len(end_row));
            let mut end = TabPoint(text.clip_point(Point::new(end_row, end_column), Bias::Right));
            let start_row = rng.gen_range(0..=text.max_point().row);
            let start_column = rng.gen_range(0..=text.line_len(start_row));
            let mut start =
                TabPoint(text.clip_point(Point::new(start_row, start_column), Bias::Left));
            if start > end {
                mem::swap(&mut start, &mut end);
            }

            let expected_text = text
                .chunks_in_range(text.point_to_offset(start.0)..text.point_to_offset(end.0))
                .collect::<String>();
            let expected_summary = TextSummary::from(expected_text.as_str());
            assert_eq!(
                tabs_snapshot
                    .chunks(start..end, false, Highlights::default())
                    .map(|c| c.text)
                    .collect::<String>(),
                expected_text,
                "chunks({:?}..{:?})",
                start,
                end
            );

            let mut actual_summary = tabs_snapshot.text_summary_for_range(start..end);
            if tab_size.get() > 1 && inlay_snapshot.text().contains('\t') {
                actual_summary.longest_row = expected_summary.longest_row;
                actual_summary.longest_row_chars = expected_summary.longest_row_chars;
            }
            assert_eq!(actual_summary, expected_summary);
        }

        for row in 0..=text.max_point().row {
            assert_eq!(
                tabs_snapshot.line_len(row),
                text.line_len(row),
                "line_len({row})"
            );
        }
    }
}

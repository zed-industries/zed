use super::fold_map::{self, FoldEdit, FoldPoint, FoldSnapshot};
use crate::MultiBufferSnapshot;
use language::{rope, Chunk};
use parking_lot::Mutex;
use std::{cmp, mem, ops::Range};
use sum_tree::Bias;
use text::Point;

pub struct TabMap(Mutex<TabSnapshot>);

impl TabMap {
    pub fn new(input: FoldSnapshot, tab_size: usize) -> (Self, TabSnapshot) {
        let snapshot = TabSnapshot {
            fold_snapshot: input,
            tab_size,
        };
        (Self(Mutex::new(snapshot.clone())), snapshot)
    }

    pub fn sync(
        &self,
        fold_snapshot: FoldSnapshot,
        mut fold_edits: Vec<FoldEdit>,
    ) -> (TabSnapshot, Vec<TabEdit>) {
        let mut old_snapshot = self.0.lock();
        let max_offset = old_snapshot.fold_snapshot.len();
        let new_snapshot = TabSnapshot {
            fold_snapshot,
            tab_size: old_snapshot.tab_size,
        };

        let mut tab_edits = Vec::with_capacity(fold_edits.len());
        for fold_edit in &mut fold_edits {
            let mut delta = 0;
            for chunk in old_snapshot
                .fold_snapshot
                .chunks(fold_edit.old.end..max_offset, false)
            {
                let patterns: &[_] = &['\t', '\n'];
                if let Some(ix) = chunk.text.find(patterns) {
                    if &chunk.text[ix..ix + 1] == "\t" {
                        fold_edit.old.end.0 += delta + ix + 1;
                        fold_edit.new.end.0 += delta + ix + 1;
                    }

                    break;
                }

                delta += chunk.text.len();
            }
        }

        let mut ix = 1;
        while ix < fold_edits.len() {
            let (prev_edits, next_edits) = fold_edits.split_at_mut(ix);
            let prev_edit = prev_edits.last_mut().unwrap();
            let edit = &next_edits[0];
            if prev_edit.old.end >= edit.old.start {
                prev_edit.old.end = edit.old.end;
                prev_edit.new.end = edit.new.end;
                fold_edits.remove(ix);
            } else {
                ix += 1;
            }
        }

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

        *old_snapshot = new_snapshot;
        (old_snapshot.clone(), tab_edits)
    }
}

#[derive(Clone)]
pub struct TabSnapshot {
    pub fold_snapshot: FoldSnapshot,
    pub tab_size: usize,
}

impl TabSnapshot {
    pub fn buffer_snapshot(&self) -> &MultiBufferSnapshot {
        self.fold_snapshot.buffer_snapshot()
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
            .chunks(range.start..line_end, false)
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
                .chunks(TabPoint::new(range.end.row(), 0)..range.end, false)
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

    pub fn version(&self) -> usize {
        self.fold_snapshot.version
    }

    pub fn chunks<'a>(&'a self, range: Range<TabPoint>, language_aware: bool) -> TabChunks<'a> {
        let (input_start, expanded_char_column, to_next_stop) =
            self.to_fold_point(range.start, Bias::Left);
        let input_start = input_start.to_offset(&self.fold_snapshot);
        let input_end = self
            .to_fold_point(range.end, Bias::Right)
            .0
            .to_offset(&self.fold_snapshot);
        let to_next_stop = if range.start.0 + Point::new(0, to_next_stop as u32) > range.end.0 {
            (range.end.column() - range.start.column()) as usize
        } else {
            to_next_stop
        };

        TabChunks {
            fold_chunks: self
                .fold_snapshot
                .chunks(input_start..input_end, language_aware),
            column: expanded_char_column,
            output_position: range.start.0,
            max_output_position: range.end.0,
            tab_size: self.tab_size,
            chunk: Chunk {
                text: &SPACES[0..to_next_stop],
                ..Default::default()
            },
            skip_leading_tab: to_next_stop > 0,
        }
    }

    pub fn buffer_rows(&self, row: u32) -> fold_map::FoldBufferRows {
        self.fold_snapshot.buffer_rows(row)
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks(TabPoint::zero()..self.max_point(), false)
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
        let expanded = Self::expand_tabs(chars, input.column() as usize, self.tab_size);
        TabPoint::new(input.row(), expanded as u32)
    }

    pub fn to_fold_point(&self, output: TabPoint, bias: Bias) -> (FoldPoint, usize, usize) {
        let chars = self.fold_snapshot.chars_at(FoldPoint::new(output.row(), 0));
        let expanded = output.column() as usize;
        let (collapsed, expanded_char_column, to_next_stop) =
            Self::collapse_tabs(chars, expanded, bias, self.tab_size);
        (
            FoldPoint::new(output.row(), collapsed as u32),
            expanded_char_column,
            to_next_stop,
        )
    }

    pub fn from_point(&self, point: Point, bias: Bias) -> TabPoint {
        self.to_tab_point(self.fold_snapshot.to_fold_point(point, bias))
    }

    pub fn to_point(&self, point: TabPoint, bias: Bias) -> Point {
        self.to_fold_point(point, bias)
            .0
            .to_buffer_point(&self.fold_snapshot)
    }

    fn expand_tabs(chars: impl Iterator<Item = char>, column: usize, tab_size: usize) -> usize {
        let mut expanded_chars = 0;
        let mut expanded_bytes = 0;
        let mut collapsed_bytes = 0;
        for c in chars {
            if collapsed_bytes == column {
                break;
            }
            if c == '\t' {
                let tab_len = tab_size - expanded_chars % tab_size;
                expanded_bytes += tab_len;
                expanded_chars += tab_len;
            } else {
                expanded_bytes += c.len_utf8();
                expanded_chars += 1;
            }
            collapsed_bytes += c.len_utf8();
        }
        expanded_bytes
    }

    fn collapse_tabs(
        mut chars: impl Iterator<Item = char>,
        column: usize,
        bias: Bias,
        tab_size: usize,
    ) -> (usize, usize, usize) {
        let mut expanded_bytes = 0;
        let mut expanded_chars = 0;
        let mut collapsed_bytes = 0;
        while let Some(c) = chars.next() {
            if expanded_bytes >= column {
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
                expanded_bytes += c.len_utf8();
            }

            if expanded_bytes > column && matches!(bias, Bias::Left) {
                expanded_chars -= 1;
                break;
            }

            collapsed_bytes += c.len_utf8();
        }
        (collapsed_bytes, expanded_chars, 0)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct TabPoint(pub super::Point);

impl TabPoint {
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
}

impl From<super::Point> for TabPoint {
    fn from(point: super::Point) -> Self {
        Self(point)
    }
}

pub type TabEdit = text::Edit<TabPoint>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextSummary {
    pub lines: super::Point,
    pub first_line_chars: u32,
    pub last_line_chars: u32,
    pub longest_row: u32,
    pub longest_row_chars: u32,
}

impl<'a> From<&'a str> for TextSummary {
    fn from(text: &'a str) -> Self {
        let sum = rope::TextSummary::from(text);

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
const SPACES: &'static str = "                ";

pub struct TabChunks<'a> {
    fold_chunks: fold_map::FoldChunks<'a>,
    chunk: Chunk<'a>,
    column: usize,
    output_position: Point,
    max_output_position: Point,
    tab_size: usize,
    skip_leading_tab: bool,
}

impl<'a> Iterator for TabChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk.text.is_empty() {
            if let Some(chunk) = self.fold_chunks.next() {
                self.chunk = chunk;
                if self.skip_leading_tab {
                    self.chunk.text = &self.chunk.text[1..];
                    self.skip_leading_tab = false;
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
                            ..self.chunk
                        });
                    } else {
                        self.chunk.text = &self.chunk.text[1..];
                        let mut len = self.tab_size - self.column % self.tab_size;
                        let next_output_position = cmp::min(
                            self.output_position + Point::new(0, len as u32),
                            self.max_output_position,
                        );
                        len = (next_output_position.column - self.output_position.column) as usize;
                        self.column += len;
                        self.output_position = next_output_position;
                        return Some(Chunk {
                            text: &SPACES[0..len],
                            ..self.chunk
                        });
                    }
                }
                '\n' => {
                    self.column = 0;
                    self.output_position += Point::new(1, 0);
                }
                _ => {
                    self.column += 1;
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
    use crate::{display_map::fold_map::FoldMap, MultiBuffer};
    use rand::{prelude::StdRng, Rng};
    use text::{RandomCharIter, Rope};

    #[test]
    fn test_expand_tabs() {
        assert_eq!(TabSnapshot::expand_tabs("\t".chars(), 0, 4), 0);
        assert_eq!(TabSnapshot::expand_tabs("\t".chars(), 1, 4), 4);
        assert_eq!(TabSnapshot::expand_tabs("\ta".chars(), 2, 4), 5);
    }

    #[gpui::test(iterations = 100)]
    fn test_random_tabs(cx: &mut gpui::MutableAppContext, mut rng: StdRng) {
        let tab_size = rng.gen_range(1..=4);
        let len = rng.gen_range(0..30);
        let buffer = if rng.gen() {
            let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        };
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        log::info!("Buffer text: {:?}", buffer_snapshot.text());

        let (mut fold_map, _) = FoldMap::new(buffer_snapshot.clone());
        fold_map.randomly_mutate(&mut rng);
        let (folds_snapshot, _) = fold_map.read(buffer_snapshot.clone(), vec![]);
        log::info!("FoldMap text: {:?}", folds_snapshot.text());

        let (_, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), tab_size);
        let text = Rope::from(tabs_snapshot.text().as_str());
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
                expected_text,
                tabs_snapshot
                    .chunks(start..end, false)
                    .map(|c| c.text)
                    .collect::<String>(),
                "chunks({:?}..{:?})",
                start,
                end
            );

            let mut actual_summary = tabs_snapshot.text_summary_for_range(start..end);
            if tab_size > 1 && folds_snapshot.text().contains('\t') {
                actual_summary.longest_row = expected_summary.longest_row;
                actual_summary.longest_row_chars = expected_summary.longest_row_chars;
            }

            assert_eq!(actual_summary, expected_summary,);
        }
    }
}

use super::fold_map::{self, FoldEdit, FoldPoint, Snapshot as FoldSnapshot, ToFoldPoint};
use buffer::Point;
use language::{rope, HighlightedChunk};
use parking_lot::Mutex;
use std::{mem, ops::Range};
use sum_tree::Bias;

pub struct TabMap(Mutex<Snapshot>);

impl TabMap {
    pub fn new(input: FoldSnapshot, tab_size: usize) -> (Self, Snapshot) {
        let snapshot = Snapshot {
            fold_snapshot: input,
            tab_size,
        };
        (Self(Mutex::new(snapshot.clone())), snapshot)
    }

    pub fn sync(
        &self,
        fold_snapshot: FoldSnapshot,
        mut fold_edits: Vec<FoldEdit>,
    ) -> (Snapshot, Vec<Edit>) {
        let mut old_snapshot = self.0.lock();
        let new_snapshot = Snapshot {
            fold_snapshot,
            tab_size: old_snapshot.tab_size,
        };

        let mut tab_edits = Vec::with_capacity(fold_edits.len());
        for fold_edit in &mut fold_edits {
            let mut delta = 0;
            for chunk in old_snapshot
                .fold_snapshot
                .chunks_at(fold_edit.old_bytes.end)
            {
                let patterns: &[_] = &['\t', '\n'];
                if let Some(ix) = chunk.find(patterns) {
                    if &chunk[ix..ix + 1] == "\t" {
                        fold_edit.old_bytes.end.0 += delta + ix + 1;
                        fold_edit.new_bytes.end.0 += delta + ix + 1;
                    }

                    break;
                }

                delta += chunk.len();
            }
        }

        let mut ix = 1;
        while ix < fold_edits.len() {
            let (prev_edits, next_edits) = fold_edits.split_at_mut(ix);
            let prev_edit = prev_edits.last_mut().unwrap();
            let edit = &next_edits[0];
            if prev_edit.old_bytes.end >= edit.old_bytes.start {
                prev_edit.old_bytes.end = edit.old_bytes.end;
                prev_edit.new_bytes.end = edit.new_bytes.end;
                fold_edits.remove(ix);
            } else {
                ix += 1;
            }
        }

        for fold_edit in fold_edits {
            let old_start = fold_edit
                .old_bytes
                .start
                .to_point(&old_snapshot.fold_snapshot);
            let old_end = fold_edit
                .old_bytes
                .end
                .to_point(&old_snapshot.fold_snapshot);
            let new_start = fold_edit
                .new_bytes
                .start
                .to_point(&new_snapshot.fold_snapshot);
            let new_end = fold_edit
                .new_bytes
                .end
                .to_point(&new_snapshot.fold_snapshot);
            tab_edits.push(Edit {
                old_lines: old_snapshot.to_tab_point(old_start)..old_snapshot.to_tab_point(old_end),
                new_lines: new_snapshot.to_tab_point(new_start)..new_snapshot.to_tab_point(new_end),
            });
        }

        *old_snapshot = new_snapshot;
        (old_snapshot.clone(), tab_edits)
    }
}

#[derive(Clone)]
pub struct Snapshot {
    pub fold_snapshot: FoldSnapshot,
    pub tab_size: usize,
}

impl Snapshot {
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
        let mut first_line_bytes = 0;
        for c in self.chunks_at(range.start).flat_map(|chunk| chunk.chars()) {
            if c == '\n'
                || (range.start.row() == range.end.row() && first_line_bytes == range.end.column())
            {
                break;
            }
            first_line_chars += 1;
            first_line_bytes += c.len_utf8() as u32;
        }

        let mut last_line_chars = 0;
        let mut last_line_bytes = 0;
        for c in self
            .chunks_at(TabPoint::new(range.end.row(), 0).max(range.start))
            .flat_map(|chunk| chunk.chars())
        {
            if last_line_bytes == range.end.column() {
                break;
            }
            last_line_chars += 1;
            last_line_bytes += c.len_utf8() as u32;
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

    pub fn chunks_at(&self, point: TabPoint) -> Chunks {
        let (point, expanded_char_column, to_next_stop) = self.to_fold_point(point, Bias::Left);
        let fold_chunks = self
            .fold_snapshot
            .chunks_at(point.to_offset(&self.fold_snapshot));
        Chunks {
            fold_chunks,
            column: expanded_char_column,
            tab_size: self.tab_size,
            chunk: &SPACES[0..to_next_stop],
            skip_leading_tab: to_next_stop > 0,
        }
    }

    pub fn highlighted_chunks(&mut self, range: Range<TabPoint>) -> HighlightedChunks {
        let (input_start, expanded_char_column, to_next_stop) =
            self.to_fold_point(range.start, Bias::Left);
        let input_start = input_start.to_offset(&self.fold_snapshot);
        let input_end = self
            .to_fold_point(range.end, Bias::Right)
            .0
            .to_offset(&self.fold_snapshot);
        HighlightedChunks {
            fold_chunks: self
                .fold_snapshot
                .highlighted_chunks(input_start..input_end),
            column: expanded_char_column,
            tab_size: self.tab_size,
            chunk: HighlightedChunk {
                text: &SPACES[0..to_next_stop],
                ..Default::default()
            },
            skip_leading_tab: to_next_stop > 0,
        }
    }

    pub fn buffer_rows(&self, row: u32) -> fold_map::BufferRows {
        self.fold_snapshot.buffer_rows(row)
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks_at(Default::default()).collect()
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

    pub fn from_point(&self, point: Point, bias: Bias) -> TabPoint {
        self.to_tab_point(point.to_fold_point(&self.fold_snapshot, bias))
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edit {
    pub old_lines: Range<TabPoint>,
    pub new_lines: Range<TabPoint>,
}

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

pub struct Chunks<'a> {
    fold_chunks: fold_map::Chunks<'a>,
    chunk: &'a str,
    column: usize,
    tab_size: usize,
    skip_leading_tab: bool,
}

impl<'a> Iterator for Chunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk.is_empty() {
            if let Some(chunk) = self.fold_chunks.next() {
                self.chunk = chunk;
                if self.skip_leading_tab {
                    self.chunk = &self.chunk[1..];
                    self.skip_leading_tab = false;
                }
            } else {
                return None;
            }
        }

        for (ix, c) in self.chunk.char_indices() {
            match c {
                '\t' => {
                    if ix > 0 {
                        let (prefix, suffix) = self.chunk.split_at(ix);
                        self.chunk = suffix;
                        return Some(prefix);
                    } else {
                        self.chunk = &self.chunk[1..];
                        let len = self.tab_size - self.column % self.tab_size;
                        self.column += len;
                        return Some(&SPACES[0..len]);
                    }
                }
                '\n' => self.column = 0,
                _ => self.column += 1,
            }
        }

        let result = Some(self.chunk);
        self.chunk = "";
        result
    }
}

pub struct HighlightedChunks<'a> {
    fold_chunks: fold_map::HighlightedChunks<'a>,
    chunk: HighlightedChunk<'a>,
    column: usize,
    tab_size: usize,
    skip_leading_tab: bool,
}

impl<'a> Iterator for HighlightedChunks<'a> {
    type Item = HighlightedChunk<'a>;

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
                        return Some(HighlightedChunk {
                            text: prefix,
                            ..self.chunk
                        });
                    } else {
                        self.chunk.text = &self.chunk.text[1..];
                        let len = self.tab_size - self.column % self.tab_size;
                        self.column += len;
                        return Some(HighlightedChunk {
                            text: &SPACES[0..len],
                            ..self.chunk
                        });
                    }
                }
                '\n' => self.column = 0,
                _ => self.column += 1,
            }
        }

        Some(mem::take(&mut self.chunk))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tabs() {
        assert_eq!(Snapshot::expand_tabs("\t".chars(), 0, 4), 0);
        assert_eq!(Snapshot::expand_tabs("\t".chars(), 1, 4), 4);
        assert_eq!(Snapshot::expand_tabs("\ta".chars(), 2, 4), 5);
    }
}

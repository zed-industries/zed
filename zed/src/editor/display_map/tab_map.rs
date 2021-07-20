use parking_lot::Mutex;

use super::fold_map::{
    Chunks as InputChunks, Edit as InputEdit, HighlightedChunks as InputHighlightedChunks,
    OutputOffset as InputOffset, OutputPoint as InputPoint, Snapshot as InputSnapshot,
};
use crate::{editor::rope, settings::StyleId, util::Bias};
use std::{
    mem,
    ops::{Add, AddAssign, Range},
};

pub struct TabMap(Mutex<Snapshot>);

impl TabMap {
    pub fn new(input: InputSnapshot, tab_size: usize) -> (Self, Snapshot) {
        let snapshot = Snapshot { input, tab_size };
        (Self(Mutex::new(snapshot.clone())), snapshot)
    }

    pub fn sync(
        &self,
        snapshot: InputSnapshot,
        input_edits: Vec<InputEdit>,
    ) -> (Snapshot, Vec<Edit>) {
        let mut old_snapshot = self.0.lock();
        let new_snapshot = Snapshot {
            input: snapshot,
            tab_size: old_snapshot.tab_size,
        };

        let mut output_edits = Vec::with_capacity(input_edits.len());
        for input_edit in input_edits {
            let old_start = input_edit.old_bytes.start.to_point(&old_snapshot.input);
            let old_end = input_edit.old_bytes.end.to_point(&old_snapshot.input);
            let new_start = input_edit.new_bytes.start.to_point(&new_snapshot.input);
            let new_end = input_edit.new_bytes.end.to_point(&new_snapshot.input);
            output_edits.push(Edit {
                old_lines: old_snapshot.to_output_point(old_start)
                    ..old_snapshot.to_output_point(old_end),
                new_lines: new_snapshot.to_output_point(new_start)
                    ..new_snapshot.to_output_point(new_end),
            });
        }

        *old_snapshot = new_snapshot;
        (old_snapshot.clone(), output_edits)
    }
}

#[derive(Clone)]
pub struct Snapshot {
    input: InputSnapshot,
    tab_size: usize,
}

impl Snapshot {
    pub fn text_summary(&self) -> TextSummary {
        // TODO: expand tabs on first and last line, ignoring the longest row.
        let summary = self.input.text_summary();
        TextSummary {
            lines: summary.lines,
            first_line_chars: summary.first_line_chars,
            last_line_chars: summary.last_line_chars,
            longest_row: summary.longest_row,
            longest_row_chars: summary.longest_row_chars,
        }
    }

    pub fn text_summary_for_range(&self, range: Range<OutputPoint>) -> TextSummary {
        // TODO: expand tabs on first and last line, ignoring the longest row.
        let start = self.to_input_point(range.start, Bias::Left).0;
        let end = self.to_input_point(range.end, Bias::Right).0;
        let summary = self.input.text_summary_for_range(start..end);
        TextSummary {
            lines: summary.lines,
            first_line_chars: summary.first_line_chars,
            last_line_chars: summary.last_line_chars,
            longest_row: summary.longest_row,
            longest_row_chars: summary.longest_row_chars,
        }
    }

    pub fn version(&self) -> usize {
        self.input.version
    }

    pub fn chunks_at(&self, point: OutputPoint) -> Chunks {
        let (point, expanded_char_column, to_next_stop) = self.to_input_point(point, Bias::Left);
        let fold_chunks = self.input.chunks_at(self.input.to_output_offset(point));
        Chunks {
            fold_chunks,
            column: expanded_char_column,
            tab_size: self.tab_size,
            chunk: &SPACES[0..to_next_stop],
            skip_leading_tab: to_next_stop > 0,
        }
    }

    pub fn highlighted_chunks_for_rows(&mut self, rows: Range<u32>) -> HighlightedChunks {
        let start = self.input.to_output_offset(InputPoint::new(rows.start, 0));
        let end = self
            .input
            .to_output_offset(InputPoint::new(rows.end, 0).min(self.input.max_point()));
        HighlightedChunks {
            input_chunks: self.input.highlighted_chunks(start..end),
            column: 0,
            tab_size: self.tab_size,
            chunk: "",
            style_id: Default::default(),
        }
    }

    #[cfg(test)]
    pub fn text(&self) -> String {
        self.chunks_at(Default::default()).collect()
    }

    pub fn len(&self) -> OutputOffset {
        self.to_output_offset(self.input.len())
    }

    pub fn line_len(&self, row: u32) -> u32 {
        self.to_output_point(InputPoint::new(row, self.input.line_len(row)))
            .column()
    }

    pub fn longest_row(&self) -> u32 {
        // TODO: Account for tab expansion.
        self.input.longest_row()
    }

    pub fn max_point(&self) -> OutputPoint {
        self.to_output_point(self.input.max_point())
    }

    pub fn clip_point(&self, point: OutputPoint, bias: Bias) -> OutputPoint {
        self.to_output_point(
            self.input
                .clip_point(self.to_input_point(point, bias).0, bias),
        )
    }

    pub fn to_output_offset(&self, input_offset: InputOffset) -> OutputOffset {
        let input_point = input_offset.to_point(&self.input);
        let input_row_start_offset = self
            .input
            .to_output_offset(InputPoint::new(input_point.row(), 0));
        let output_point = self.to_output_point(input_point);
        OutputOffset(input_row_start_offset.0 + output_point.column() as usize)
    }

    pub fn to_output_point(&self, input: InputPoint) -> OutputPoint {
        let chars = self.input.chars_at(InputPoint::new(input.row(), 0));
        let expanded = Self::expand_tabs(chars, input.column() as usize, self.tab_size);
        OutputPoint::new(input.row(), expanded as u32)
    }

    pub fn to_input_point(&self, output: OutputPoint, bias: Bias) -> (InputPoint, usize, usize) {
        let chars = self.input.chars_at(InputPoint::new(output.row(), 0));
        let expanded = output.column() as usize;
        let (collapsed, expanded_char_column, to_next_stop) =
            Self::collapse_tabs(chars, expanded, bias, self.tab_size);
        (
            InputPoint::new(output.row(), collapsed as u32),
            expanded_char_column,
            to_next_stop,
        )
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OutputOffset(pub usize);

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct OutputPoint(pub super::Point);

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

impl From<super::Point> for OutputPoint {
    fn from(point: super::Point) -> Self {
        Self(point)
    }
}

impl AddAssign<Self> for OutputPoint {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += &rhs.0;
    }
}

impl Add<Self> for OutputPoint {
    type Output = OutputPoint;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edit {
    pub old_lines: Range<OutputPoint>,
    pub new_lines: Range<OutputPoint>,
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
    fold_chunks: InputChunks<'a>,
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
    input_chunks: InputHighlightedChunks<'a>,
    chunk: &'a str,
    style_id: StyleId,
    column: usize,
    tab_size: usize,
}

impl<'a> Iterator for HighlightedChunks<'a> {
    type Item = (&'a str, StyleId);

    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk.is_empty() {
            if let Some((chunk, style_id)) = self.input_chunks.next() {
                self.chunk = chunk;
                self.style_id = style_id;
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
                        return Some((prefix, self.style_id));
                    } else {
                        self.chunk = &self.chunk[1..];
                        let len = self.tab_size - self.column % self.tab_size;
                        self.column += len;
                        return Some((&SPACES[0..len], self.style_id));
                    }
                }
                '\n' => self.column = 0,
                _ => self.column += 1,
            }
        }

        Some((mem::take(&mut self.chunk), mem::take(&mut self.style_id)))
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

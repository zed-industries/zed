use ordered_float::OrderedFloat;
use rope::{Point, Rope, TextSummary};
use std::collections::{BTreeSet, HashMap};
use std::{
    cmp,
    fmt::{self, Debug},
    ops::Range,
};

#[derive(Default)]
struct Matrix {
    cells: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new() -> Self {
        Self {
            cells: Vec::new(),
            rows: 0,
            cols: 0,
        }
    }

    fn resize(&mut self, rows: usize, cols: usize) {
        self.cells.resize(rows * cols, 0.);
        self.rows = rows;
        self.cols = cols;
    }

    fn swap_columns(&mut self, col1: usize, col2: usize) {
        if col1 == col2 {
            return;
        }

        if col1 >= self.cols {
            panic!("column out of bounds");
        }

        if col2 >= self.cols {
            panic!("column out of bounds");
        }

        unsafe {
            let ptr = self.cells.as_mut_ptr();
            std::ptr::swap_nonoverlapping(
                ptr.add(col1 * self.rows),
                ptr.add(col2 * self.rows),
                self.rows,
            );
        }
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        if row >= self.rows {
            panic!("row out of bounds")
        }

        if col >= self.cols {
            panic!("column out of bounds")
        }
        self.cells[col * self.rows + row]
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        if row >= self.rows {
            panic!("row out of bounds")
        }

        if col >= self.cols {
            panic!("column out of bounds")
        }

        self.cells[col * self.rows + row] = value;
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:5}", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum CharOperation {
    Insert { text: String },
    Delete { bytes: usize },
    Keep { bytes: usize },
}

#[derive(Default)]
pub struct StreamingDiff {
    old: Vec<char>,
    new: Vec<char>,
    scores: Matrix,
    old_text_ix: usize,
    new_text_ix: usize,
    equal_runs: HashMap<(usize, usize), u32>,
}

impl StreamingDiff {
    const INSERTION_SCORE: f64 = -1.;
    const DELETION_SCORE: f64 = -20.;
    const EQUALITY_BASE: f64 = 1.8;
    const MAX_EQUALITY_EXPONENT: i32 = 16;

    pub fn new(old: String) -> Self {
        let old = old.chars().collect::<Vec<_>>();
        let mut scores = Matrix::new();
        scores.resize(old.len() + 1, 1);
        for i in 0..=old.len() {
            scores.set(i, 0, i as f64 * Self::DELETION_SCORE);
        }
        Self {
            old,
            new: Vec::new(),
            scores,
            old_text_ix: 0,
            new_text_ix: 0,
            equal_runs: Default::default(),
        }
    }

    pub fn push_new(&mut self, text: &str) -> Vec<CharOperation> {
        self.new.extend(text.chars());
        self.scores.swap_columns(0, self.scores.cols - 1);
        self.scores
            .resize(self.old.len() + 1, self.new.len() - self.new_text_ix + 1);
        self.equal_runs.retain(|(_i, j), _| *j == self.new_text_ix);

        for j in self.new_text_ix + 1..=self.new.len() {
            let relative_j = j - self.new_text_ix;

            self.scores
                .set(0, relative_j, j as f64 * Self::INSERTION_SCORE);
            for i in 1..=self.old.len() {
                let insertion_score = self.scores.get(i, relative_j - 1) + Self::INSERTION_SCORE;
                let deletion_score = self.scores.get(i - 1, relative_j) + Self::DELETION_SCORE;
                let equality_score = if self.old[i - 1] == self.new[j - 1] {
                    let mut equal_run = self.equal_runs.get(&(i - 1, j - 1)).copied().unwrap_or(0);
                    equal_run += 1;
                    self.equal_runs.insert((i, j), equal_run);

                    let exponent = cmp::min(equal_run as i32 / 4, Self::MAX_EQUALITY_EXPONENT);
                    self.scores.get(i - 1, relative_j - 1) + Self::EQUALITY_BASE.powi(exponent)
                } else {
                    f64::NEG_INFINITY
                };

                let score = insertion_score.max(deletion_score).max(equality_score);
                self.scores.set(i, relative_j, score);
            }
        }

        let mut max_score = f64::NEG_INFINITY;
        let mut next_old_text_ix = self.old_text_ix;
        let next_new_text_ix = self.new.len();
        for i in self.old_text_ix..=self.old.len() {
            let score = self.scores.get(i, next_new_text_ix - self.new_text_ix);
            if score > max_score {
                max_score = score;
                next_old_text_ix = i;
            }
        }

        let hunks = self.backtrack(next_old_text_ix, next_new_text_ix);
        self.old_text_ix = next_old_text_ix;
        self.new_text_ix = next_new_text_ix;
        hunks
    }

    fn backtrack(&self, old_text_ix: usize, new_text_ix: usize) -> Vec<CharOperation> {
        let mut pending_insert: Option<Range<usize>> = None;
        let mut hunks = Vec::new();
        let mut i = old_text_ix;
        let mut j = new_text_ix;
        while (i, j) != (self.old_text_ix, self.new_text_ix) {
            let insertion_score = if j > self.new_text_ix {
                Some((i, j - 1))
            } else {
                None
            };
            let deletion_score = if i > self.old_text_ix {
                Some((i - 1, j))
            } else {
                None
            };
            let equality_score = if i > self.old_text_ix && j > self.new_text_ix {
                if self.old[i - 1] == self.new[j - 1] {
                    Some((i - 1, j - 1))
                } else {
                    None
                }
            } else {
                None
            };

            let (prev_i, prev_j) = [insertion_score, deletion_score, equality_score]
                .iter()
                .max_by_key(|cell| {
                    cell.map(|(i, j)| OrderedFloat(self.scores.get(i, j - self.new_text_ix)))
                })
                .unwrap()
                .unwrap();

            if prev_i == i && prev_j == j - 1 {
                if let Some(pending_insert) = pending_insert.as_mut() {
                    pending_insert.start = prev_j;
                } else {
                    pending_insert = Some(prev_j..j);
                }
            } else {
                if let Some(range) = pending_insert.take() {
                    hunks.push(CharOperation::Insert {
                        text: self.new[range].iter().collect(),
                    });
                }

                let char_len = self.old[i - 1].len_utf8();
                if prev_i == i - 1 && prev_j == j {
                    if let Some(CharOperation::Delete { bytes: len }) = hunks.last_mut() {
                        *len += char_len;
                    } else {
                        hunks.push(CharOperation::Delete { bytes: char_len })
                    }
                } else if let Some(CharOperation::Keep { bytes: len }) = hunks.last_mut() {
                    *len += char_len;
                } else {
                    hunks.push(CharOperation::Keep { bytes: char_len })
                }
            }

            i = prev_i;
            j = prev_j;
        }

        if let Some(range) = pending_insert.take() {
            hunks.push(CharOperation::Insert {
                text: self.new[range].iter().collect(),
            });
        }

        hunks.reverse();
        hunks
    }

    pub fn finish(self) -> Vec<CharOperation> {
        self.backtrack(self.old.len(), self.new.len())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineOperation {
    Insert { lines: u32 },
    Delete { lines: u32 },
    Keep { lines: u32 },
}

#[derive(Debug, Default)]
pub struct LineDiff {
    inserted_newline_at_end: bool,
    /// The extent of kept and deleted text.
    old_end: Point,
    /// The extent of kept and inserted text.
    new_end: Point,
    /// Deleted rows, expressed in terms of the old text.
    deleted_rows: BTreeSet<u32>,
    /// Inserted rows, expressed in terms of the new text.
    inserted_rows: BTreeSet<u32>,
    buffered_insert: String,
    /// After deleting a newline, we buffer deletion until we keep or insert a character.
    buffered_delete: usize,
}

impl LineDiff {
    pub fn push_char_operations<'a>(
        &mut self,
        operations: impl IntoIterator<Item = &'a CharOperation>,
        old_text: &Rope,
    ) {
        for operation in operations {
            self.push_char_operation(operation, old_text);
        }
    }

    pub fn push_char_operation(&mut self, operation: &CharOperation, old_text: &Rope) {
        match operation {
            CharOperation::Insert { text } => {
                self.flush_delete(old_text);

                if is_line_start(self.old_end) {
                    if let Some(newline_ix) = text.rfind('\n') {
                        let (prefix, suffix) = text.split_at(newline_ix + 1);
                        self.buffered_insert.push_str(prefix);
                        self.flush_insert(old_text);
                        self.buffered_insert.push_str(suffix);
                    } else {
                        self.buffered_insert.push_str(text);
                    }
                } else {
                    self.buffered_insert.push_str(text);
                    if !text.ends_with('\n') {
                        self.flush_insert(old_text);
                    }
                }
            }
            CharOperation::Delete { bytes } => {
                self.buffered_delete += bytes;

                let common_suffix_len = self.trim_buffered_end(old_text);
                self.flush_insert(old_text);

                if common_suffix_len > 0 || !is_line_end(self.old_end, old_text) {
                    self.flush_delete(old_text);
                    self.keep(common_suffix_len, old_text);
                }
            }
            CharOperation::Keep { bytes } => {
                self.flush_delete(old_text);
                self.flush_insert(old_text);
                self.keep(*bytes, old_text);
            }
        }
    }

    fn flush_insert(&mut self, old_text: &Rope) {
        if self.buffered_insert.is_empty() {
            return;
        }

        let new_start = self.new_end;
        let lines = TextSummary::from(self.buffered_insert.as_str()).lines;
        self.new_end += lines;

        if is_line_start(self.old_end) {
            if self.new_end.column == 0 {
                self.inserted_rows.extend(new_start.row..self.new_end.row);
            } else {
                self.deleted_rows.insert(self.old_end.row);
                self.inserted_rows.extend(new_start.row..=self.new_end.row);
            }
        } else if is_line_end(self.old_end, old_text) {
            if self.buffered_insert.starts_with('\n') {
                self.inserted_rows
                    .extend(new_start.row + 1..=self.new_end.row);
                self.inserted_newline_at_end = true;
            } else {
                if !self.inserted_newline_at_end {
                    self.deleted_rows.insert(self.old_end.row);
                }
                self.inserted_rows.extend(new_start.row..=self.new_end.row);
            }
        } else {
            self.deleted_rows.insert(self.old_end.row);
            self.inserted_rows.extend(new_start.row..=self.new_end.row);
        }

        self.buffered_insert.clear();
    }

    fn flush_delete(&mut self, old_text: &Rope) {
        if self.buffered_delete == 0 {
            return;
        }

        let old_start = self.old_end;
        self.old_end =
            old_text.offset_to_point(old_text.point_to_offset(self.old_end) + self.buffered_delete);

        if is_line_end(old_start, old_text) && is_line_end(self.old_end, old_text) {
            self.deleted_rows
                .extend(old_start.row + 1..=self.old_end.row);
        } else if is_line_start(old_start)
            && (is_line_start(self.old_end) && self.old_end < old_text.max_point())
            && self.new_end.column == 0
        {
            self.deleted_rows.extend(old_start.row..self.old_end.row);
        } else {
            self.inserted_rows.insert(self.new_end.row);
            self.deleted_rows.extend(old_start.row..=self.old_end.row);
        }

        self.inserted_newline_at_end = false;
        self.buffered_delete = 0;
    }

    fn keep(&mut self, bytes: usize, old_text: &Rope) {
        if bytes == 0 {
            return;
        }

        let lines =
            old_text.offset_to_point(old_text.point_to_offset(self.old_end) + bytes) - self.old_end;
        self.old_end += lines;
        self.new_end += lines;
        self.inserted_newline_at_end = false;
    }

    fn trim_buffered_end(&mut self, old_text: &Rope) -> usize {
        let old_start_offset = old_text.point_to_offset(self.old_end);
        let old_end_offset = old_start_offset + self.buffered_delete;

        let new_chars = self.buffered_insert.chars().rev();
        let old_chars = old_text
            .chunks_in_range(old_start_offset..old_end_offset)
            .flat_map(|chunk| chunk.chars().rev());

        let mut common_suffix_len = 0;
        for (new_ch, old_ch) in new_chars.zip(old_chars) {
            if new_ch == old_ch {
                common_suffix_len += new_ch.len_utf8();
            } else {
                break;
            }
        }

        self.buffered_delete -= common_suffix_len;
        self.buffered_insert
            .truncate(self.buffered_insert.len() - common_suffix_len);

        common_suffix_len
    }

    pub fn finish(&mut self, old_text: &Rope) {
        self.flush_insert(old_text);
        self.flush_delete(old_text);

        let old_start = self.old_end;
        self.old_end = old_text.max_point();
        self.new_end += self.old_end - old_start;
    }

    pub fn line_operations(&self) -> Vec<LineOperation> {
        let mut ops = Vec::new();
        let mut deleted_rows = self.deleted_rows.iter().copied().peekable();
        let mut inserted_rows = self.inserted_rows.iter().copied().peekable();
        let mut old_row = 0;
        let mut new_row = 0;

        while deleted_rows.peek().is_some() || inserted_rows.peek().is_some() {
            // Check for a run of deleted lines at current old row.
            if Some(old_row) == deleted_rows.peek().copied() {
                if let Some(LineOperation::Delete { lines }) = ops.last_mut() {
                    *lines += 1;
                } else {
                    ops.push(LineOperation::Delete { lines: 1 });
                }
                old_row += 1;
                deleted_rows.next();
            } else if Some(new_row) == inserted_rows.peek().copied() {
                if let Some(LineOperation::Insert { lines }) = ops.last_mut() {
                    *lines += 1;
                } else {
                    ops.push(LineOperation::Insert { lines: 1 });
                }
                new_row += 1;
                inserted_rows.next();
            } else {
                // Keep lines until the next deletion, insertion, or the end of the old text.
                let lines_to_next_deletion = inserted_rows
                    .peek()
                    .copied()
                    .unwrap_or(self.new_end.row + 1)
                    - new_row;
                let lines_to_next_insertion =
                    deleted_rows.peek().copied().unwrap_or(self.old_end.row + 1) - old_row;
                let kept_lines =
                    cmp::max(1, cmp::min(lines_to_next_insertion, lines_to_next_deletion));
                if kept_lines > 0 {
                    ops.push(LineOperation::Keep { lines: kept_lines });
                    old_row += kept_lines;
                    new_row += kept_lines;
                }
            }
        }

        if old_row < self.old_end.row + 1 {
            ops.push(LineOperation::Keep {
                lines: self.old_end.row + 1 - old_row,
            });
        }

        ops
    }
}

fn is_line_start(point: Point) -> bool {
    point.column == 0
}

fn is_line_end(point: Point, text: &Rope) -> bool {
    text.line_len(point.row) == point.column
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use std::env;

    #[test]
    fn test_delete_first_of_two_lines() {
        let old_text = "aaaa\nbbbb";
        let char_ops = vec![
            CharOperation::Delete { bytes: 5 },
            CharOperation::Keep { bytes: 4 },
        ];
        let expected_line_ops = vec![
            LineOperation::Delete { lines: 1 },
            LineOperation::Keep { lines: 1 },
        ];
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &expected_line_ops)
        );

        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(line_ops, expected_line_ops);
    }

    #[test]
    fn test_delete_second_of_two_lines() {
        let old_text = "aaaa\nbbbb";
        let char_ops = vec![
            CharOperation::Keep { bytes: 5 },
            CharOperation::Delete { bytes: 4 },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Keep { lines: 1 },
                LineOperation::Delete { lines: 1 },
                LineOperation::Insert { lines: 1 }
            ]
        );
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_add_new_line() {
        let old_text = "aaaa\nbbbb";
        let char_ops = vec![
            CharOperation::Keep { bytes: 9 },
            CharOperation::Insert {
                text: "\ncccc".into(),
            },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Keep { lines: 2 },
                LineOperation::Insert { lines: 1 }
            ]
        );
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_delete_line_in_middle() {
        let old_text = "aaaa\nbbbb\ncccc";
        let char_ops = vec![
            CharOperation::Keep { bytes: 5 },
            CharOperation::Delete { bytes: 5 },
            CharOperation::Keep { bytes: 4 },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Keep { lines: 1 },
                LineOperation::Delete { lines: 1 },
                LineOperation::Keep { lines: 1 }
            ]
        );
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_replace_line() {
        let old_text = "aaaa\nbbbb\ncccc";
        let char_ops = vec![
            CharOperation::Keep { bytes: 5 },
            CharOperation::Delete { bytes: 4 },
            CharOperation::Insert {
                text: "BBBB".into(),
            },
            CharOperation::Keep { bytes: 5 },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Keep { lines: 1 },
                LineOperation::Delete { lines: 1 },
                LineOperation::Insert { lines: 1 },
                LineOperation::Keep { lines: 1 }
            ]
        );
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_multiple_edits_on_different_lines() {
        let old_text = "aaaa\nbbbb\ncccc\ndddd";
        let char_ops = vec![
            CharOperation::Insert { text: "A".into() },
            CharOperation::Keep { bytes: 9 },
            CharOperation::Delete { bytes: 5 },
            CharOperation::Keep { bytes: 4 },
            CharOperation::Insert {
                text: "\nEEEE".into(),
            },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Delete { lines: 1 },
                LineOperation::Insert { lines: 1 },
                LineOperation::Keep { lines: 1 },
                LineOperation::Delete { lines: 2 },
                LineOperation::Insert { lines: 2 },
            ]
        );
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_edit_at_end_of_line() {
        let old_text = "aaaa\nbbbb\ncccc";
        let char_ops = vec![
            CharOperation::Keep { bytes: 4 },
            CharOperation::Insert { text: "A".into() },
            CharOperation::Keep { bytes: 10 },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Delete { lines: 1 },
                LineOperation::Insert { lines: 1 },
                LineOperation::Keep { lines: 2 }
            ]
        );
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_insert_newline_character() {
        let old_text = "aaaabbbb";
        let char_ops = vec![
            CharOperation::Keep { bytes: 4 },
            CharOperation::Insert { text: "\n".into() },
            CharOperation::Keep { bytes: 4 },
        ];
        let new_text = apply_char_operations(old_text, &char_ops);
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Delete { lines: 1 },
                LineOperation::Insert { lines: 2 }
            ]
        );
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_insert_newline_at_beginning() {
        let old_text = "aaaa\nbbbb";
        let char_ops = vec![
            CharOperation::Insert { text: "\n".into() },
            CharOperation::Keep { bytes: 9 },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Insert { lines: 1 },
                LineOperation::Keep { lines: 2 }
            ]
        );
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_delete_newline() {
        let old_text = "aaaa\nbbbb";
        let char_ops = vec![
            CharOperation::Keep { bytes: 4 },
            CharOperation::Delete { bytes: 1 },
            CharOperation::Keep { bytes: 4 },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Delete { lines: 2 },
                LineOperation::Insert { lines: 1 }
            ]
        );

        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_insert_multiple_newlines() {
        let old_text = "aaaa\nbbbb";
        let char_ops = vec![
            CharOperation::Keep { bytes: 5 },
            CharOperation::Insert {
                text: "\n\n".into(),
            },
            CharOperation::Keep { bytes: 4 },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Keep { lines: 1 },
                LineOperation::Insert { lines: 2 },
                LineOperation::Keep { lines: 1 }
            ]
        );
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_delete_multiple_newlines() {
        let old_text = "aaaa\n\n\nbbbb";
        let char_ops = vec![
            CharOperation::Keep { bytes: 5 },
            CharOperation::Delete { bytes: 2 },
            CharOperation::Keep { bytes: 4 },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Keep { lines: 1 },
                LineOperation::Delete { lines: 2 },
                LineOperation::Keep { lines: 1 }
            ]
        );
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_complex_scenario() {
        let old_text = "line1\nline2\nline3\nline4";
        let char_ops = vec![
            CharOperation::Keep { bytes: 6 },
            CharOperation::Insert {
                text: "inserted\n".into(),
            },
            CharOperation::Delete { bytes: 6 },
            CharOperation::Keep { bytes: 5 },
            CharOperation::Insert {
                text: "\nnewline".into(),
            },
            CharOperation::Keep { bytes: 6 },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Keep { lines: 1 },
                LineOperation::Delete { lines: 1 },
                LineOperation::Insert { lines: 1 },
                LineOperation::Keep { lines: 1 },
                LineOperation::Insert { lines: 1 },
                LineOperation::Keep { lines: 1 }
            ]
        );
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(new_text, "line1\ninserted\nline3\nnewline\nline4");
        assert_eq!(
            apply_line_operations(old_text, &new_text, &line_ops),
            new_text,
        );
    }

    #[test]
    fn test_cleaning_up_common_suffix() {
        let old_text = concat!(
            "        for y in 0..size.y() {\n",
            "            let a = 10;\n",
            "            let b = 20;\n",
            "        }",
        );
        let char_ops = [
            CharOperation::Keep { bytes: 8 },
            CharOperation::Insert { text: "let".into() },
            CharOperation::Insert {
                text: " mut".into(),
            },
            CharOperation::Insert { text: " y".into() },
            CharOperation::Insert { text: " =".into() },
            CharOperation::Insert { text: " 0".into() },
            CharOperation::Insert { text: ";".into() },
            CharOperation::Insert { text: "\n".into() },
            CharOperation::Insert {
                text: "        while".into(),
            },
            CharOperation::Insert { text: " y".into() },
            CharOperation::Insert {
                text: " < size".into(),
            },
            CharOperation::Insert { text: ".".into() },
            CharOperation::Insert { text: "y".into() },
            CharOperation::Insert { text: "()".into() },
            CharOperation::Insert { text: " {".into() },
            CharOperation::Insert { text: "\n".into() },
            CharOperation::Delete { bytes: 23 },
            CharOperation::Keep { bytes: 23 },
            CharOperation::Keep { bytes: 1 },
            CharOperation::Keep { bytes: 23 },
            CharOperation::Keep { bytes: 1 },
            CharOperation::Keep { bytes: 8 },
            CharOperation::Insert {
                text: "    y".into(),
            },
            CharOperation::Insert { text: " +=".into() },
            CharOperation::Insert { text: " 1".into() },
            CharOperation::Insert { text: ";".into() },
            CharOperation::Insert { text: "\n".into() },
            CharOperation::Insert {
                text: "        ".into(),
            },
            CharOperation::Keep { bytes: 1 },
        ];
        let line_ops = char_ops_to_line_ops(old_text, &char_ops);
        assert_eq!(
            line_ops,
            vec![
                LineOperation::Delete { lines: 1 },
                LineOperation::Insert { lines: 2 },
                LineOperation::Keep { lines: 2 },
                LineOperation::Delete { lines: 1 },
                LineOperation::Insert { lines: 2 },
            ]
        );
        let new_text = apply_char_operations(old_text, &char_ops);
        assert_eq!(
            new_text,
            apply_line_operations(old_text, &new_text, &line_ops)
        );
    }

    #[test]
    fn test_random_diffs() {
        random_test(|mut rng| {
            let old_text_len = env::var("OLD_TEXT_LEN")
                .map(|i| i.parse().expect("invalid `OLD_TEXT_LEN` variable"))
                .unwrap_or(10);

            let old = random_text(&mut rng, old_text_len);
            println!("old text: {:?}", old);

            let new = randomly_edit(&old, &mut rng);
            println!("new text: {:?}", new);

            let char_operations = random_streaming_diff(&mut rng, &old, &new);
            println!("char operations: {:?}", char_operations);

            // Use apply_char_operations to verify the result
            let patched = apply_char_operations(&old, &char_operations);
            assert_eq!(patched, new);

            // Test char_ops_to_line_ops
            let line_ops = char_ops_to_line_ops(&old, &char_operations);
            println!("line operations: {:?}", line_ops);
            let patched = apply_line_operations(&old, &new, &line_ops);
            assert_eq!(patched, new);
        });
    }

    fn char_ops_to_line_ops(old_text: &str, char_ops: &[CharOperation]) -> Vec<LineOperation> {
        let old_rope = Rope::from(old_text);
        let mut diff = LineDiff::default();
        for op in char_ops {
            diff.push_char_operation(op, &old_rope);
        }
        diff.finish(&old_rope);
        diff.line_operations()
    }

    fn random_streaming_diff(rng: &mut impl Rng, old: &str, new: &str) -> Vec<CharOperation> {
        let mut diff = StreamingDiff::new(old.to_string());
        let mut char_operations = Vec::new();
        let mut new_len = 0;

        while new_len < new.len() {
            let mut chunk_len = rng.gen_range(1..=new.len() - new_len);
            while !new.is_char_boundary(new_len + chunk_len) {
                chunk_len += 1;
            }
            let chunk = &new[new_len..new_len + chunk_len];
            let new_hunks = diff.push_new(chunk);
            char_operations.extend(new_hunks);
            new_len += chunk_len;
        }

        char_operations.extend(diff.finish());
        char_operations
    }

    fn random_test<F>(mut test_fn: F)
    where
        F: FnMut(StdRng),
    {
        let iterations = env::var("ITERATIONS")
            .map(|i| i.parse().expect("invalid `ITERATIONS` variable"))
            .unwrap_or(100);

        let seed: u64 = env::var("SEED")
            .map(|s| s.parse().expect("invalid `SEED` variable"))
            .unwrap_or(0);

        println!(
            "Running test with {} iterations and seed {}",
            iterations, seed
        );

        for i in 0..iterations {
            println!("Iteration {}", i + 1);
            let rng = StdRng::seed_from_u64(seed + i);
            test_fn(rng);
        }
    }

    fn apply_line_operations(old_text: &str, new_text: &str, line_ops: &[LineOperation]) -> String {
        let mut result: Vec<&str> = Vec::new();

        let old_lines: Vec<&str> = old_text.split('\n').collect();
        let new_lines: Vec<&str> = new_text.split('\n').collect();
        let mut old_start = 0_usize;
        let mut new_start = 0_usize;

        for op in line_ops {
            match op {
                LineOperation::Keep { lines } => {
                    let old_end = old_start + *lines as usize;
                    result.extend(&old_lines[old_start..old_end]);
                    old_start = old_end;
                    new_start += *lines as usize;
                }
                LineOperation::Delete { lines } => {
                    old_start += *lines as usize;
                }
                LineOperation::Insert { lines } => {
                    let new_end = new_start + *lines as usize;
                    result.extend(&new_lines[new_start..new_end]);
                    new_start = new_end;
                }
            }
        }

        result.join("\n")
    }

    #[test]
    fn test_apply_char_operations() {
        let old_text = "Hello, world!";
        let char_ops = vec![
            CharOperation::Keep { bytes: 7 },
            CharOperation::Delete { bytes: 5 },
            CharOperation::Insert {
                text: "Rust".to_string(),
            },
            CharOperation::Keep { bytes: 1 },
        ];
        let result = apply_char_operations(old_text, &char_ops);
        assert_eq!(result, "Hello, Rust!");
    }

    fn random_text(rng: &mut impl Rng, length: usize) -> String {
        util::RandomCharIter::new(rng).take(length).collect()
    }

    fn randomly_edit(text: &str, rng: &mut impl Rng) -> String {
        let mut result = String::from(text);
        let edit_count = rng.gen_range(1..=5);

        fn random_char_range(text: &str, rng: &mut impl Rng) -> (usize, usize) {
            let mut start = rng.gen_range(0..=text.len());
            while !text.is_char_boundary(start) {
                start -= 1;
            }
            let mut end = rng.gen_range(start..=text.len());
            while !text.is_char_boundary(end) {
                end += 1;
            }
            (start, end)
        }

        for _ in 0..edit_count {
            match rng.gen_range(0..3) {
                0 => {
                    // Insert
                    let (pos, _) = random_char_range(&result, rng);
                    let insert_len = rng.gen_range(1..=5);
                    let insert_text: String = random_text(rng, insert_len);
                    result.insert_str(pos, &insert_text);
                }
                1 => {
                    // Delete
                    if !result.is_empty() {
                        let (start, end) = random_char_range(&result, rng);
                        result.replace_range(start..end, "");
                    }
                }
                2 => {
                    // Replace
                    if !result.is_empty() {
                        let (start, end) = random_char_range(&result, rng);
                        let replace_len = end - start;
                        let replace_text: String = random_text(rng, replace_len);
                        result.replace_range(start..end, &replace_text);
                    }
                }
                _ => unreachable!(),
            }
        }

        result
    }

    fn apply_char_operations(old_text: &str, char_ops: &[CharOperation]) -> String {
        let mut result = String::new();
        let mut old_ix = 0;

        for operation in char_ops {
            match operation {
                CharOperation::Keep { bytes } => {
                    result.push_str(&old_text[old_ix..old_ix + bytes]);
                    old_ix += bytes;
                }
                CharOperation::Delete { bytes } => {
                    old_ix += bytes;
                }
                CharOperation::Insert { text } => {
                    result.push_str(text);
                }
            }
        }

        result
    }
}

//! `LineIndex` to make a line_offsets, each item is an byte offset (start from 0) of the beginning of the line.
//!
//! For example, the text: `"hello 你好\nworld"`, the line_offsets will store `[0, 13]`.
//!
//! Then `line_col` with a offset just need to find the line index by binary search.
//!
//! Inspired by rust-analyzer's `LineIndex`:
//! <https://github.com/rust-lang/rust/blob/1.67.0/src/tools/rust-analyzer/crates/ide-db/src/line_index.rs>
use alloc::vec::Vec;

#[derive(Clone)]
pub struct LineIndex {
    /// Offset (bytes) the the beginning of each line, zero-based
    line_offsets: Vec<usize>,
}

impl LineIndex {
    pub fn new(text: &str) -> LineIndex {
        let mut line_offsets: Vec<usize> = alloc::vec![0];

        let mut offset = 0;

        for c in text.chars() {
            offset += c.len_utf8();
            if c == '\n' {
                line_offsets.push(offset);
            }
        }

        LineIndex { line_offsets }
    }

    /// Returns (line, col) of pos.
    ///
    /// The pos is a byte offset, start from 0, e.g. "ab" is 2, "你好" is 6
    pub fn line_col(&self, input: &str, pos: usize) -> (usize, usize) {
        let line = self.line_offsets.partition_point(|&it| it <= pos) - 1;
        let first_offset = self.line_offsets[line];

        // Get line str from original input, then we can get column offset
        let line_str = &input[first_offset..pos];
        let col = line_str.chars().count();

        (line + 1, col + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::zero_prefixed_literal)]
    #[test]
    fn test_line_index() {
        let text = "hello 你好 A🎈C\nworld";
        let table = [
            (00, 1, 1, 'h'),
            (01, 1, 2, 'e'),
            (02, 1, 3, 'l'),
            (03, 1, 4, 'l'),
            (04, 1, 5, 'o'),
            (05, 1, 6, ' '),
            (06, 1, 7, '你'),
            (09, 1, 8, '好'),
            (12, 1, 9, ' '),
            (13, 1, 10, 'A'),
            (14, 1, 11, '🎈'),
            (18, 1, 12, 'C'),
            (19, 1, 13, '\n'),
            (20, 2, 1, 'w'),
            (21, 2, 2, 'o'),
            (22, 2, 3, 'r'),
            (23, 2, 4, 'l'),
            (24, 2, 5, 'd'),
        ];

        let index = LineIndex::new(text);
        for &(offset, line, col, c) in table.iter() {
            let res = index.line_col(text, offset);
            assert_eq!(
                (res.0, res.1),
                (line, col),
                "Expected: ({}, {}, {}, {:?})",
                offset,
                line,
                col,
                c
            );
        }
    }
}

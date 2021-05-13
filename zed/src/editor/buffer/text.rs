use ropey::{Rope, RopeSlice};

use super::Point;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextSummary {
    pub chars: usize,
    pub bytes: usize,
    pub lines: Point,
}

impl<'a> From<RopeSlice<'a>> for TextSummary {
    fn from(slice: RopeSlice<'a>) -> Self {
        let last_row = slice.len_lines() - 1;
        let last_column = slice.line(last_row).len_chars();
        Self {
            chars: slice.len_chars(),
            bytes: slice.len_bytes(),
            lines: Point::new(last_row as u32, last_column as u32),
        }
    }
}

impl<'a> From<&'a Rope> for TextSummary {
    fn from(text: &'a Rope) -> Self {
        Self::from(text.slice(..))
    }
}

impl<'a> std::ops::AddAssign<&'a Self> for TextSummary {
    fn add_assign(&mut self, other: &'a Self) {
        // let joined_line_len = self.lines.column + other.first_line_len;
        // if joined_line_len > self.rightmost_point.column {
        //     self.rightmost_point = Point::new(self.lines.row, joined_line_len);
        // }
        // if other.rightmost_point.column > self.rightmost_point.column {
        //     self.rightmost_point = self.lines + &other.rightmost_point;
        // }

        // if self.lines.row == 0 {
        //     self.first_line_len += other.first_line_len;
        // }

        self.chars += other.chars;
        self.bytes += other.bytes;
        self.lines += &other.lines;
    }
}

impl std::ops::AddAssign<Self> for TextSummary {
    fn add_assign(&mut self, other: Self) {
        *self += &other;
    }
}

use super::Point;
use crate::sum_tree::{self, SeekBias, SumTree};
use arrayvec::ArrayVec;
use std::{
    cmp,
    fmt::{self, Debug},
    ops::{Bound, Index, Range, RangeBounds},
    sync::Arc,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Run {
    Newline,
    Chars { len: usize, char_size: u8 },
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct ByteOffset(usize);

impl sum_tree::Item for Run {
    type Summary = TextSummary;

    fn summary(&self) -> Self::Summary {
        match *self {
            Run::Newline => TextSummary {
                chars: 1,
                bytes: 1,
                lines: Point::new(1, 0),
                first_line_len: 0,
                rightmost_point: Point::new(0, 0),
            },
            Run::Chars { len, char_size } => TextSummary {
                chars: len,
                bytes: len * char_size as usize,
                lines: Point::new(0, len as u32),
                first_line_len: len as u32,
                rightmost_point: Point::new(0, len as u32),
            },
        }
    }
}

impl Run {
    fn char_size(&self) -> u8 {
        match self {
            Run::Newline => 1,
            Run::Chars { char_size, .. } => *char_size,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextSummary {
    pub chars: usize,
    pub bytes: usize,
    pub lines: Point,
    pub first_line_len: u32,
    pub rightmost_point: Point,
}

impl<'a> std::ops::AddAssign<&'a Self> for TextSummary {
    fn add_assign(&mut self, other: &'a Self) {
        let joined_line_len = self.lines.column + other.first_line_len;
        if joined_line_len > self.rightmost_point.column {
            self.rightmost_point = Point::new(self.lines.row, joined_line_len);
        }
        if other.rightmost_point.column > self.rightmost_point.column {
            self.rightmost_point = self.lines + &other.rightmost_point;
        }

        if self.lines.row == 0 {
            self.first_line_len += other.first_line_len;
        }

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

impl<'a> sum_tree::Dimension<'a, TextSummary> for TextSummary {
    fn add_summary(&mut self, summary: &TextSummary) {
        *self += summary;
    }
}

impl<'a> sum_tree::Dimension<'a, TextSummary> for Point {
    fn add_summary(&mut self, summary: &TextSummary) {
        *self += &summary.lines;
    }
}

impl<'a> sum_tree::Dimension<'a, TextSummary> for ByteOffset {
    fn add_summary(&mut self, summary: &TextSummary) {
        self.0 += summary.bytes
    }
}

impl<'a> sum_tree::Dimension<'a, TextSummary> for usize {
    fn add_summary(&mut self, summary: &TextSummary) {
        *self += summary.chars;
    }
}

#[derive(Clone)]
pub struct Text {
    text: Arc<str>,
    runs: SumTree<Run>,
    range: Range<usize>,
}

impl From<String> for Text {
    fn from(text: String) -> Self {
        let mut runs = Vec::new();

        let mut chars_len = 0;
        let mut run_char_size = 0;
        let mut run_chars = 0;

        let mut chars = text.chars();
        loop {
            let ch = chars.next();
            let ch_size = ch.map_or(0, |ch| ch.len_utf8());
            if run_chars != 0 && (ch.is_none() || ch == Some('\n') || run_char_size != ch_size) {
                runs.push(Run::Chars {
                    len: run_chars,
                    char_size: run_char_size as u8,
                });
                run_chars = 0;
            }
            run_char_size = ch_size;

            match ch {
                Some('\n') => runs.push(Run::Newline),
                Some(_) => run_chars += 1,
                None => break,
            }
            chars_len += 1;
        }

        let mut tree = SumTree::new();
        tree.extend(runs);
        Text {
            text: text.into(),
            runs: tree,
            range: 0..chars_len,
        }
    }
}

impl<'a> From<&'a str> for Text {
    fn from(text: &'a str) -> Self {
        Self::from(String::from(text))
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Text").field(&self.text).finish()
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Eq for Text {}

impl<T: RangeBounds<usize>> Index<T> for Text {
    type Output = str;

    fn index(&self, range: T) -> &Self::Output {
        let start = match range.start_bound() {
            Bound::Included(start) => cmp::min(self.range.start + start, self.range.end),
            Bound::Excluded(_) => unimplemented!(),
            Bound::Unbounded => self.range.start,
        };
        let end = match range.end_bound() {
            Bound::Included(end) => cmp::min(self.range.start + end + 1, self.range.end),
            Bound::Excluded(end) => cmp::min(self.range.start + end, self.range.end),
            Bound::Unbounded => self.range.end,
        };

        let byte_start = self.abs_byte_offset_for_offset(start);
        let byte_end = self.abs_byte_offset_for_offset(end);
        &self.text[byte_start..byte_end]
    }
}

impl Text {
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    pub fn as_str(&self) -> &str {
        &self[..]
    }

    pub fn slice<T: RangeBounds<usize>>(&self, range: T) -> Text {
        let start = match range.start_bound() {
            Bound::Included(start) => cmp::min(self.range.start + start, self.range.end),
            Bound::Excluded(_) => unimplemented!(),
            Bound::Unbounded => self.range.start,
        };
        let end = match range.end_bound() {
            Bound::Included(end) => cmp::min(self.range.start + end + 1, self.range.end),
            Bound::Excluded(end) => cmp::min(self.range.start + end, self.range.end),
            Bound::Unbounded => self.range.end,
        };

        Text {
            text: self.text.clone(),
            runs: self.runs.clone(),
            range: start..end,
        }
    }

    pub fn line_len(&self, row: u32) -> u32 {
        let mut cursor = self.runs.cursor::<usize, Point>();
        cursor.seek(&self.range.start, SeekBias::Right);
        let absolute_row = cursor.start().row + row;

        let mut cursor = self.runs.cursor::<Point, usize>();
        cursor.seek(&Point::new(absolute_row, 0), SeekBias::Right);
        let prefix_len = self.range.start.saturating_sub(*cursor.start());
        let line_len = cursor.summary::<usize>(&Point::new(absolute_row + 1, 0), SeekBias::Left);
        let suffix_len = cursor.start().saturating_sub(self.range.end);

        line_len
            .saturating_sub(prefix_len)
            .saturating_sub(suffix_len) as u32
    }

    pub fn len(&self) -> usize {
        self.range.end - self.range.start
    }

    pub fn lines(&self) -> Point {
        self.abs_point_for_offset(self.range.end) - &self.abs_point_for_offset(self.range.start)
    }

    pub fn rightmost_point(&self) -> Point {
        let lines = self.lines();

        let mut candidates = ArrayVec::<[Point; 3]>::new();
        candidates.push(lines);
        if lines.row > 0 {
            candidates.push(Point::new(0, self.line_len(0)));
            if lines.row > 1 {
                let mut cursor = self.runs.cursor::<usize, Point>();
                cursor.seek(&self.range.start, SeekBias::Right);
                let absolute_start_row = cursor.start().row;

                let mut cursor = self.runs.cursor::<Point, usize>();
                cursor.seek(&Point::new(absolute_start_row + 1, 0), SeekBias::Right);
                let summary = cursor.summary::<TextSummary>(
                    &Point::new(absolute_start_row + lines.row, 0),
                    SeekBias::Left,
                );

                candidates.push(Point::new(1, 0) + &summary.rightmost_point);
            }
        }

        candidates.into_iter().max_by_key(|p| p.column).unwrap()
    }

    pub fn point_for_offset(&self, offset: usize) -> Point {
        self.abs_point_for_offset(self.range.start + offset)
            - &self.abs_point_for_offset(self.range.start)
    }

    pub fn offset_for_point(&self, point: Point) -> usize {
        let mut cursor = self.runs.cursor::<Point, TextSummary>();
        let abs_point = self.abs_point_for_offset(self.range.start) + &point;
        cursor.seek(&abs_point, SeekBias::Right);
        let overshoot = abs_point - &cursor.start().lines;
        let abs_offset = cursor.start().chars + overshoot.column as usize;
        abs_offset - self.range.start
    }

    pub fn summary(&self) -> TextSummary {
        TextSummary {
            chars: self.range.end - self.range.start,
            bytes: self.abs_byte_offset_for_offset(self.range.end)
                - self.abs_byte_offset_for_offset(self.range.start),
            lines: self.abs_point_for_offset(self.range.end)
                - &self.abs_point_for_offset(self.range.start),
            first_line_len: self.line_len(0),
            rightmost_point: self.rightmost_point(),
        }
    }

    fn abs_point_for_offset(&self, offset: usize) -> Point {
        let mut cursor = self.runs.cursor::<usize, TextSummary>();
        cursor.seek(&offset, SeekBias::Right);
        let overshoot = (offset - cursor.start().chars) as u32;
        cursor.start().lines + &Point::new(0, overshoot)
    }

    fn abs_byte_offset_for_offset(&self, offset: usize) -> usize {
        let mut cursor = self.runs.cursor::<usize, TextSummary>();
        cursor.seek(&offset, SeekBias::Right);
        let overshoot = offset - cursor.start().chars;
        cursor.start().bytes + overshoot * cursor.item().map_or(0, |run| run.char_size()) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_basic() {
        let text = Text::from(String::from("ab\ncd€\nfghij\nkl¢m"));
        assert_eq!(text.len(), 17);
        assert_eq!(text.as_str(), "ab\ncd€\nfghij\nkl¢m");
        assert_eq!(text.lines(), Point::new(3, 4));
        assert_eq!(text.line_len(0), 2);
        assert_eq!(text.line_len(1), 3);
        assert_eq!(text.line_len(2), 5);
        assert_eq!(text.line_len(3), 4);
        assert_eq!(text.rightmost_point(), Point::new(2, 5));

        let b_to_g = text.slice(1..9);
        assert_eq!(b_to_g.as_str(), "b\ncd€\nfg");
        assert_eq!(b_to_g.len(), 8);
        assert_eq!(b_to_g.lines(), Point::new(2, 2));
        assert_eq!(b_to_g.line_len(0), 1);
        assert_eq!(b_to_g.line_len(1), 3);
        assert_eq!(b_to_g.line_len(2), 2);
        assert_eq!(b_to_g.line_len(3), 0);
        assert_eq!(b_to_g.rightmost_point(), Point::new(1, 3));

        let d_to_i = text.slice(4..11);
        assert_eq!(d_to_i.as_str(), "d€\nfghi");
        assert_eq!(&d_to_i[1..5], "€\nfg");
        assert_eq!(d_to_i.len(), 7);
        assert_eq!(d_to_i.lines(), Point::new(1, 4));
        assert_eq!(d_to_i.line_len(0), 2);
        assert_eq!(d_to_i.line_len(1), 4);
        assert_eq!(d_to_i.line_len(2), 0);
        assert_eq!(d_to_i.rightmost_point(), Point::new(1, 4));

        let d_to_j = text.slice(4..=11);
        assert_eq!(d_to_j.as_str(), "d€\nfghij");
        assert_eq!(&d_to_j[1..], "€\nfghij");
        assert_eq!(d_to_j.len(), 8);
    }

    #[test]
    fn test_random() {
        use rand::prelude::*;

        for seed in 0..100 {
            println!("buffer::text seed: {}", seed);
            let rng = &mut StdRng::seed_from_u64(seed);

            let len = rng.gen_range(0..50);
            let mut string = String::new();
            for _ in 0..len {
                if rng.gen_ratio(1, 5) {
                    string.push('\n');
                } else {
                    string.push(rng.gen());
                }
            }
            let text = Text::from(string.clone());

            for _ in 0..10 {
                let start = rng.gen_range(0..text.len() + 1);
                let end = rng.gen_range(start..text.len() + 2);

                let string_slice = string
                    .chars()
                    .skip(start)
                    .take(end - start)
                    .collect::<String>();
                let expected_line_endpoints = string_slice
                    .split('\n')
                    .enumerate()
                    .map(|(row, line)| Point::new(row as u32, line.chars().count() as u32))
                    .collect::<Vec<_>>();
                let text_slice = text.slice(start..end);

                assert_eq!(text_slice.lines(), lines(&string_slice));

                let mut rightmost_points: HashSet<Point> = HashSet::new();
                for endpoint in &expected_line_endpoints {
                    if let Some(rightmost_point) = rightmost_points.iter().next().cloned() {
                        if endpoint.column > rightmost_point.column {
                            rightmost_points.clear();
                        }
                        if endpoint.column >= rightmost_point.column {
                            rightmost_points.insert(*endpoint);
                        }
                    } else {
                        rightmost_points.insert(*endpoint);
                    }

                    assert_eq!(text_slice.line_len(endpoint.row as u32), endpoint.column);
                }

                assert!(rightmost_points.contains(&text_slice.rightmost_point()));

                for _ in 0..10 {
                    let offset = rng.gen_range(0..string_slice.chars().count() + 1);
                    let point = lines(&string_slice.chars().take(offset).collect::<String>());
                    assert_eq!(text_slice.point_for_offset(offset), point);
                    assert_eq!(text_slice.offset_for_point(point), offset);
                    if offset < string_slice.chars().count() {
                        assert_eq!(
                            &text_slice[offset..offset + 1],
                            String::from_iter(string_slice.chars().nth(offset)).as_str()
                        );
                    }
                }
            }
        }
    }

    pub fn lines(s: &str) -> Point {
        let mut row = 0;
        let mut column = 0;
        for ch in s.chars() {
            if ch == '\n' {
                row += 1;
                column = 0;
            } else {
                column += 1;
            }
        }
        Point::new(row, column)
    }
}

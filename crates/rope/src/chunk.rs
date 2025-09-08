use crate::{OffsetUtf16, Point, PointUtf16, TextSummary, Unclipped};
use arrayvec::ArrayString;
use std::{cmp, ops::Range};
use sum_tree::Bias;
use unicode_segmentation::GraphemeCursor;
use util::debug_panic;

pub(crate) const MIN_BASE: usize = if cfg!(test) { 6 } else { 64 };
pub(crate) const MAX_BASE: usize = MIN_BASE * 2;

#[derive(Clone, Debug, Default)]
pub struct Chunk {
    chars: u128,
    chars_utf16: u128,
    newlines: u128,
    tabs: u128,
    pub text: ArrayString<MAX_BASE>,
}

impl Chunk {
    #[inline(always)]
    pub fn new(text: &str) -> Self {
        let mut this = Chunk::default();
        this.push_str(text);
        this
    }

    #[inline(always)]
    pub fn push_str(&mut self, text: &str) {
        for (char_ix, c) in text.char_indices() {
            let ix = self.text.len() + char_ix;
            self.chars |= 1 << ix;
            self.chars_utf16 |= 1 << ix;
            self.chars_utf16 |= (c.len_utf16() as u128) << ix;
            self.newlines |= ((c == '\n') as u128) << ix;
            self.tabs |= ((c == '\t') as u128) << ix;
        }
        self.text.push_str(text);
    }

    #[inline(always)]
    pub fn append(&mut self, slice: ChunkSlice) {
        if slice.is_empty() {
            return;
        };

        let base_ix = self.text.len();
        self.chars |= slice.chars << base_ix;
        self.chars_utf16 |= slice.chars_utf16 << base_ix;
        self.newlines |= slice.newlines << base_ix;
        self.tabs |= slice.tabs << base_ix;
        self.text.push_str(slice.text);
    }

    #[inline(always)]
    pub fn as_slice(&self) -> ChunkSlice<'_> {
        ChunkSlice {
            chars: self.chars,
            chars_utf16: self.chars_utf16,
            newlines: self.newlines,
            tabs: self.tabs,
            text: &self.text,
        }
    }

    #[inline(always)]
    pub fn slice(&self, range: Range<usize>) -> ChunkSlice<'_> {
        self.as_slice().slice(range)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkSlice<'a> {
    chars: u128,
    chars_utf16: u128,
    newlines: u128,
    tabs: u128,
    text: &'a str,
}

impl Into<Chunk> for ChunkSlice<'_> {
    fn into(self) -> Chunk {
        Chunk {
            chars: self.chars,
            chars_utf16: self.chars_utf16,
            newlines: self.newlines,
            tabs: self.tabs,
            text: self.text.try_into().unwrap(),
        }
    }
}

impl<'a> ChunkSlice<'a> {
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    #[inline(always)]
    pub fn is_char_boundary(self, offset: usize) -> bool {
        self.text.is_char_boundary(offset)
    }

    #[inline(always)]
    pub fn split_at(self, mid: usize) -> (ChunkSlice<'a>, ChunkSlice<'a>) {
        if mid == MAX_BASE {
            let left = self;
            let right = ChunkSlice {
                chars: 0,
                chars_utf16: 0,
                newlines: 0,
                tabs: 0,
                text: "",
            };
            (left, right)
        } else {
            let mask = (1u128 << mid) - 1;
            let (left_text, right_text) = self.text.split_at(mid);
            let left = ChunkSlice {
                chars: self.chars & mask,
                chars_utf16: self.chars_utf16 & mask,
                newlines: self.newlines & mask,
                tabs: self.tabs & mask,
                text: left_text,
            };
            let right = ChunkSlice {
                chars: self.chars >> mid,
                chars_utf16: self.chars_utf16 >> mid,
                newlines: self.newlines >> mid,
                tabs: self.tabs >> mid,
                text: right_text,
            };
            (left, right)
        }
    }

    #[inline(always)]
    pub fn slice(self, range: Range<usize>) -> Self {
        let mask = if range.end == MAX_BASE {
            u128::MAX
        } else {
            (1u128 << range.end) - 1
        };
        if range.start == MAX_BASE {
            Self {
                chars: 0,
                chars_utf16: 0,
                newlines: 0,
                tabs: 0,
                text: "",
            }
        } else {
            Self {
                chars: (self.chars & mask) >> range.start,
                chars_utf16: (self.chars_utf16 & mask) >> range.start,
                newlines: (self.newlines & mask) >> range.start,
                tabs: (self.tabs & mask) >> range.start,
                text: &self.text[range],
            }
        }
    }

    #[inline(always)]
    pub fn text_summary(&self) -> TextSummary {
        let mut chars = 0;
        let (longest_row, longest_row_chars) = self.longest_row(&mut chars);
        TextSummary {
            len: self.len(),
            chars,
            len_utf16: self.len_utf16(),
            lines: self.lines(),
            first_line_chars: self.first_line_chars(),
            last_line_chars: self.last_line_chars(),
            last_line_len_utf16: self.last_line_len_utf16(),
            longest_row,
            longest_row_chars,
        }
    }

    /// Get length in bytes
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Get length in UTF-16 code units
    #[inline(always)]
    pub fn len_utf16(&self) -> OffsetUtf16 {
        OffsetUtf16(self.chars_utf16.count_ones() as usize)
    }

    /// Get point representing number of lines and length of last line
    #[inline(always)]
    pub fn lines(&self) -> Point {
        let row = self.newlines.count_ones();
        let column = self.newlines.leading_zeros() - (u128::BITS - self.text.len() as u32);
        Point::new(row, column)
    }

    /// Get number of chars in first line
    #[inline(always)]
    pub fn first_line_chars(&self) -> u32 {
        if self.newlines == 0 {
            self.chars.count_ones()
        } else {
            let mask = (1u128 << self.newlines.trailing_zeros()) - 1;
            (self.chars & mask).count_ones()
        }
    }

    /// Get number of chars in last line
    #[inline(always)]
    pub fn last_line_chars(&self) -> u32 {
        if self.newlines == 0 {
            self.chars.count_ones()
        } else {
            let mask = !(u128::MAX >> self.newlines.leading_zeros());
            (self.chars & mask).count_ones()
        }
    }

    /// Get number of UTF-16 code units in last line
    #[inline(always)]
    pub fn last_line_len_utf16(&self) -> u32 {
        if self.newlines == 0 {
            self.chars_utf16.count_ones()
        } else {
            let mask = !(u128::MAX >> self.newlines.leading_zeros());
            (self.chars_utf16 & mask).count_ones()
        }
    }

    /// Get the longest row in the chunk and its length in characters.
    /// Calculate the total number of characters in the chunk along the way.
    #[inline(always)]
    pub fn longest_row(&self, total_chars: &mut usize) -> (u32, u32) {
        let mut chars = self.chars;
        let mut newlines = self.newlines;
        *total_chars = 0;
        let mut row = 0;
        let mut longest_row = 0;
        let mut longest_row_chars = 0;
        while newlines > 0 {
            let newline_ix = newlines.trailing_zeros();
            let row_chars = (chars & ((1 << newline_ix) - 1)).count_ones() as u8;
            *total_chars += usize::from(row_chars);
            if row_chars > longest_row_chars {
                longest_row = row;
                longest_row_chars = row_chars;
            }

            newlines >>= newline_ix;
            newlines >>= 1;
            chars >>= newline_ix;
            chars >>= 1;
            row += 1;
            *total_chars += 1;
        }

        let row_chars = chars.count_ones() as u8;
        *total_chars += usize::from(row_chars);
        if row_chars > longest_row_chars {
            (row, row_chars as u32)
        } else {
            (longest_row, longest_row_chars as u32)
        }
    }

    #[inline(always)]
    pub fn offset_to_point(&self, offset: usize) -> Point {
        let mask = if offset == MAX_BASE {
            u128::MAX
        } else {
            (1u128 << offset) - 1
        };
        let row = (self.newlines & mask).count_ones();
        let newline_ix = u128::BITS - (self.newlines & mask).leading_zeros();
        let column = (offset - newline_ix as usize) as u32;
        Point::new(row, column)
    }

    #[inline(always)]
    pub fn point_to_offset(&self, point: Point) -> usize {
        if point.row > self.lines().row {
            debug_panic!(
                "point {:?} extends beyond rows for string {:?}",
                point,
                self.text
            );
            return self.len();
        }

        let row_offset_range = self.offset_range_for_row(point.row);
        if point.column > row_offset_range.len() as u32 {
            debug_panic!(
                "point {:?} extends beyond row for string {:?}",
                point,
                self.text
            );
            row_offset_range.end
        } else {
            row_offset_range.start + point.column as usize
        }
    }

    #[inline(always)]
    pub fn offset_to_offset_utf16(&self, offset: usize) -> OffsetUtf16 {
        let mask = if offset == MAX_BASE {
            u128::MAX
        } else {
            (1u128 << offset) - 1
        };
        OffsetUtf16((self.chars_utf16 & mask).count_ones() as usize)
    }

    #[inline(always)]
    pub fn offset_utf16_to_offset(&self, target: OffsetUtf16) -> usize {
        if target.0 == 0 {
            0
        } else {
            let ix = nth_set_bit(self.chars_utf16, target.0) + 1;
            if ix == MAX_BASE {
                MAX_BASE
            } else {
                let utf8_additional_len = cmp::min(
                    (self.chars_utf16 >> ix).trailing_zeros() as usize,
                    self.text.len() - ix,
                );
                ix + utf8_additional_len
            }
        }
    }

    #[inline(always)]
    pub fn offset_to_point_utf16(&self, offset: usize) -> PointUtf16 {
        let mask = if offset == MAX_BASE {
            u128::MAX
        } else {
            (1u128 << offset) - 1
        };
        let row = (self.newlines & mask).count_ones();
        let newline_ix = u128::BITS - (self.newlines & mask).leading_zeros();
        let column = if newline_ix as usize == MAX_BASE {
            0
        } else {
            ((self.chars_utf16 & mask) >> newline_ix).count_ones()
        };
        PointUtf16::new(row, column)
    }

    #[inline(always)]
    pub fn point_to_point_utf16(&self, point: Point) -> PointUtf16 {
        self.offset_to_point_utf16(self.point_to_offset(point))
    }

    #[inline(always)]
    pub fn point_utf16_to_offset(&self, point: PointUtf16, clip: bool) -> usize {
        let lines = self.lines();
        if point.row > lines.row {
            if !clip {
                debug_panic!(
                    "point {:?} is beyond this chunk's extent {:?}",
                    point,
                    self.text
                );
            }
            return self.len();
        }

        let row_offset_range = self.offset_range_for_row(point.row);
        let line = self.slice(row_offset_range.clone());
        if point.column > line.last_line_len_utf16() {
            if !clip {
                debug_panic!(
                    "point {:?} is beyond the end of the line in chunk {:?}",
                    point,
                    self.text
                );
            }
            return line.len();
        }

        let mut offset = row_offset_range.start;
        if point.column > 0 {
            offset += line.offset_utf16_to_offset(OffsetUtf16(point.column as usize));
            if !self.text.is_char_boundary(offset) {
                offset -= 1;
                while !self.text.is_char_boundary(offset) {
                    offset -= 1;
                }
                if !clip {
                    debug_panic!(
                        "point {:?} is within character in chunk {:?}",
                        point,
                        self.text,
                    );
                }
            }
        }
        offset
    }

    #[inline(always)]
    pub fn unclipped_point_utf16_to_point(&self, point: Unclipped<PointUtf16>) -> Point {
        let max_point = self.lines();
        if point.0.row > max_point.row {
            return max_point;
        }

        let row_offset_range = self.offset_range_for_row(point.0.row);
        let line = self.slice(row_offset_range);
        if point.0.column == 0 {
            Point::new(point.0.row, 0)
        } else if point.0.column >= line.len_utf16().0 as u32 {
            Point::new(point.0.row, line.len() as u32)
        } else {
            let mut column = line.offset_utf16_to_offset(OffsetUtf16(point.0.column as usize));
            while !line.text.is_char_boundary(column) {
                column -= 1;
            }
            Point::new(point.0.row, column as u32)
        }
    }

    #[inline(always)]
    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        let max_point = self.lines();
        if point.row > max_point.row {
            return max_point;
        }

        let line = self.slice(self.offset_range_for_row(point.row));
        if point.column == 0 {
            point
        } else if point.column >= line.len() as u32 {
            Point::new(point.row, line.len() as u32)
        } else {
            let mut column = point.column as usize;
            let bytes = line.text.as_bytes();
            if bytes[column - 1] < 128 && bytes[column] < 128 {
                return Point::new(point.row, column as u32);
            }

            let mut grapheme_cursor = GraphemeCursor::new(column, bytes.len(), true);
            loop {
                if line.is_char_boundary(column)
                    && grapheme_cursor.is_boundary(line.text, 0).unwrap_or(false)
                {
                    break;
                }

                match bias {
                    Bias::Left => column -= 1,
                    Bias::Right => column += 1,
                }
                grapheme_cursor.set_cursor(column);
            }
            Point::new(point.row, column as u32)
        }
    }

    #[inline(always)]
    pub fn clip_point_utf16(&self, point: Unclipped<PointUtf16>, bias: Bias) -> PointUtf16 {
        let max_point = self.lines();
        if point.0.row > max_point.row {
            PointUtf16::new(max_point.row, self.last_line_len_utf16())
        } else {
            let line = self.slice(self.offset_range_for_row(point.0.row));
            let column = line.clip_offset_utf16(OffsetUtf16(point.0.column as usize), bias);
            PointUtf16::new(point.0.row, column.0 as u32)
        }
    }

    #[inline(always)]
    pub fn clip_offset_utf16(&self, target: OffsetUtf16, bias: Bias) -> OffsetUtf16 {
        if target == OffsetUtf16::default() {
            OffsetUtf16::default()
        } else if target >= self.len_utf16() {
            self.len_utf16()
        } else {
            let mut offset = self.offset_utf16_to_offset(target);
            while !self.text.is_char_boundary(offset) {
                if bias == Bias::Left {
                    offset -= 1;
                } else {
                    offset += 1;
                }
            }
            self.offset_to_offset_utf16(offset)
        }
    }

    #[inline(always)]
    fn offset_range_for_row(&self, row: u32) -> Range<usize> {
        let row_start = if row > 0 {
            nth_set_bit(self.newlines, row as usize) + 1
        } else {
            0
        };
        let row_len = if row_start == MAX_BASE {
            0
        } else {
            cmp::min(
                (self.newlines >> row_start).trailing_zeros(),
                (self.text.len() - row_start) as u32,
            )
        };
        row_start..row_start + row_len as usize
    }

    #[inline(always)]
    pub fn tabs(&self) -> Tabs {
        Tabs {
            tabs: self.tabs,
            chars: self.chars,
        }
    }
}

pub struct Tabs {
    tabs: u128,
    chars: u128,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TabPosition {
    pub byte_offset: usize,
    pub char_offset: usize,
}

impl Iterator for Tabs {
    type Item = TabPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tabs == 0 {
            return None;
        }

        let tab_offset = self.tabs.trailing_zeros() as usize;
        let chars_mask = (1 << tab_offset) - 1;
        let char_offset = (self.chars & chars_mask).count_ones() as usize;

        // Since tabs are 1 byte the tab offset is the same as the byte offset
        let position = TabPosition {
            byte_offset: tab_offset,
            char_offset,
        };
        // Remove the tab we've just seen
        self.tabs ^= 1 << tab_offset;

        Some(position)
    }
}

/// Finds the n-th bit that is set to 1.
#[inline(always)]
fn nth_set_bit(v: u128, n: usize) -> usize {
    let low = v as u64;
    let high = (v >> 64) as u64;

    let low_count = low.count_ones() as usize;
    if n > low_count {
        64 + nth_set_bit_u64(high, (n - low_count) as u64) as usize
    } else {
        nth_set_bit_u64(low, n as u64) as usize
    }
}

#[inline(always)]
fn nth_set_bit_u64(v: u64, mut n: u64) -> u64 {
    let v = v.reverse_bits();
    let mut s: u64 = 64;

    // Parallel bit count intermediates
    let a = v - ((v >> 1) & (u64::MAX / 3));
    let b = (a & (u64::MAX / 5)) + ((a >> 2) & (u64::MAX / 5));
    let c = (b + (b >> 4)) & (u64::MAX / 0x11);
    let d = (c + (c >> 8)) & (u64::MAX / 0x101);

    // Branchless select
    let t = (d >> 32) + (d >> 48);
    s -= (t.wrapping_sub(n) & 256) >> 3;
    n -= t & (t.wrapping_sub(n) >> 8);

    let t = (d >> (s - 16)) & 0xff;
    s -= (t.wrapping_sub(n) & 256) >> 4;
    n -= t & (t.wrapping_sub(n) >> 8);

    let t = (c >> (s - 8)) & 0xf;
    s -= (t.wrapping_sub(n) & 256) >> 5;
    n -= t & (t.wrapping_sub(n) >> 8);

    let t = (b >> (s - 4)) & 0x7;
    s -= (t.wrapping_sub(n) & 256) >> 6;
    n -= t & (t.wrapping_sub(n) >> 8);

    let t = (a >> (s - 2)) & 0x3;
    s -= (t.wrapping_sub(n) & 256) >> 7;
    n -= t & (t.wrapping_sub(n) >> 8);

    let t = (v >> (s - 1)) & 0x1;
    s -= (t.wrapping_sub(n) & 256) >> 8;

    65 - s - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use util::RandomCharIter;

    #[gpui::test(iterations = 100)]
    fn test_random_chunks(mut rng: StdRng) {
        let chunk_len = rng.random_range(0..=MAX_BASE);
        let text = RandomCharIter::new(&mut rng)
            .take(chunk_len)
            .collect::<String>();
        let mut ix = chunk_len;
        while !text.is_char_boundary(ix) {
            ix -= 1;
        }
        let text = &text[..ix];

        log::info!("Chunk: {:?}", text);
        let chunk = Chunk::new(text);
        verify_chunk(chunk.as_slice(), text);

        for _ in 0..10 {
            let mut start = rng.random_range(0..=chunk.text.len());
            let mut end = rng.random_range(start..=chunk.text.len());
            while !chunk.text.is_char_boundary(start) {
                start -= 1;
            }
            while !chunk.text.is_char_boundary(end) {
                end -= 1;
            }
            let range = start..end;
            log::info!("Range: {:?}", range);
            let text_slice = &text[range.clone()];
            let chunk_slice = chunk.slice(range);
            verify_chunk(chunk_slice, text_slice);
        }
    }

    #[gpui::test(iterations = 1000)]
    fn test_nth_set_bit_random(mut rng: StdRng) {
        let set_count = rng.random_range(0..=128);
        let mut set_bits = (0..128).choose_multiple(&mut rng, set_count);
        set_bits.sort();
        let mut n = 0;
        for ix in set_bits.iter().copied() {
            n |= 1 << ix;
        }

        for (mut ix, position) in set_bits.into_iter().enumerate() {
            ix += 1;
            assert_eq!(
                nth_set_bit(n, ix),
                position,
                "nth_set_bit({:0128b}, {})",
                n,
                ix
            );
        }
    }

    fn verify_chunk(chunk: ChunkSlice<'_>, text: &str) {
        let mut offset = 0;
        let mut offset_utf16 = OffsetUtf16(0);
        let mut point = Point::zero();
        let mut point_utf16 = PointUtf16::zero();

        log::info!("Verifying chunk {:?}", text);
        assert_eq!(chunk.offset_to_point(0), Point::zero());

        let mut expected_tab_positions = Vec::new();

        for (char_offset, c) in text.chars().enumerate() {
            let expected_point = chunk.offset_to_point(offset);
            assert_eq!(point, expected_point, "mismatch at offset {}", offset);
            assert_eq!(
                chunk.point_to_offset(point),
                offset,
                "mismatch at point {:?}",
                point
            );
            assert_eq!(
                chunk.offset_to_offset_utf16(offset),
                offset_utf16,
                "mismatch at offset {}",
                offset
            );
            assert_eq!(
                chunk.offset_utf16_to_offset(offset_utf16),
                offset,
                "mismatch at offset_utf16 {:?}",
                offset_utf16
            );
            assert_eq!(
                chunk.point_to_point_utf16(point),
                point_utf16,
                "mismatch at point {:?}",
                point
            );
            assert_eq!(
                chunk.point_utf16_to_offset(point_utf16, false),
                offset,
                "mismatch at point_utf16 {:?}",
                point_utf16
            );
            assert_eq!(
                chunk.unclipped_point_utf16_to_point(Unclipped(point_utf16)),
                point,
                "mismatch for unclipped_point_utf16_to_point at {:?}",
                point_utf16
            );

            assert_eq!(
                chunk.clip_point(point, Bias::Left),
                point,
                "incorrect left clip at {:?}",
                point
            );
            assert_eq!(
                chunk.clip_point(point, Bias::Right),
                point,
                "incorrect right clip at {:?}",
                point
            );

            for i in 1..c.len_utf8() {
                let test_point = Point::new(point.row, point.column + i as u32);
                assert_eq!(
                    chunk.clip_point(test_point, Bias::Left),
                    point,
                    "incorrect left clip within multi-byte char at {:?}",
                    test_point
                );
                assert_eq!(
                    chunk.clip_point(test_point, Bias::Right),
                    Point::new(point.row, point.column + c.len_utf8() as u32),
                    "incorrect right clip within multi-byte char at {:?}",
                    test_point
                );
            }

            for i in 1..c.len_utf16() {
                let test_point = Unclipped(PointUtf16::new(
                    point_utf16.row,
                    point_utf16.column + i as u32,
                ));
                assert_eq!(
                    chunk.unclipped_point_utf16_to_point(test_point),
                    point,
                    "incorrect unclipped_point_utf16_to_point within multi-byte char at {:?}",
                    test_point
                );
                assert_eq!(
                    chunk.clip_point_utf16(test_point, Bias::Left),
                    point_utf16,
                    "incorrect left clip_point_utf16 within multi-byte char at {:?}",
                    test_point
                );
                assert_eq!(
                    chunk.clip_point_utf16(test_point, Bias::Right),
                    PointUtf16::new(point_utf16.row, point_utf16.column + c.len_utf16() as u32),
                    "incorrect right clip_point_utf16 within multi-byte char at {:?}",
                    test_point
                );

                let test_offset = OffsetUtf16(offset_utf16.0 + i);
                assert_eq!(
                    chunk.clip_offset_utf16(test_offset, Bias::Left),
                    offset_utf16,
                    "incorrect left clip_offset_utf16 within multi-byte char at {:?}",
                    test_offset
                );
                assert_eq!(
                    chunk.clip_offset_utf16(test_offset, Bias::Right),
                    OffsetUtf16(offset_utf16.0 + c.len_utf16()),
                    "incorrect right clip_offset_utf16 within multi-byte char at {:?}",
                    test_offset
                );
            }

            if c == '\n' {
                point.row += 1;
                point.column = 0;
                point_utf16.row += 1;
                point_utf16.column = 0;
            } else {
                point.column += c.len_utf8() as u32;
                point_utf16.column += c.len_utf16() as u32;
            }

            if c == '\t' {
                expected_tab_positions.push(TabPosition {
                    byte_offset: offset,
                    char_offset,
                });
            }

            offset += c.len_utf8();
            offset_utf16.0 += c.len_utf16();
        }

        let final_point = chunk.offset_to_point(offset);
        assert_eq!(point, final_point, "mismatch at final offset {}", offset);
        assert_eq!(
            chunk.point_to_offset(point),
            offset,
            "mismatch at point {:?}",
            point
        );
        assert_eq!(
            chunk.offset_to_offset_utf16(offset),
            offset_utf16,
            "mismatch at offset {}",
            offset
        );
        assert_eq!(
            chunk.offset_utf16_to_offset(offset_utf16),
            offset,
            "mismatch at offset_utf16 {:?}",
            offset_utf16
        );
        assert_eq!(
            chunk.point_to_point_utf16(point),
            point_utf16,
            "mismatch at final point {:?}",
            point
        );
        assert_eq!(
            chunk.point_utf16_to_offset(point_utf16, false),
            offset,
            "mismatch at final point_utf16 {:?}",
            point_utf16
        );
        assert_eq!(
            chunk.unclipped_point_utf16_to_point(Unclipped(point_utf16)),
            point,
            "mismatch for unclipped_point_utf16_to_point at final point {:?}",
            point_utf16
        );
        assert_eq!(
            chunk.clip_point(point, Bias::Left),
            point,
            "incorrect left clip at final point {:?}",
            point
        );
        assert_eq!(
            chunk.clip_point(point, Bias::Right),
            point,
            "incorrect right clip at final point {:?}",
            point
        );
        assert_eq!(
            chunk.clip_point_utf16(Unclipped(point_utf16), Bias::Left),
            point_utf16,
            "incorrect left clip_point_utf16 at final point {:?}",
            point_utf16
        );
        assert_eq!(
            chunk.clip_point_utf16(Unclipped(point_utf16), Bias::Right),
            point_utf16,
            "incorrect right clip_point_utf16 at final point {:?}",
            point_utf16
        );
        assert_eq!(
            chunk.clip_offset_utf16(offset_utf16, Bias::Left),
            offset_utf16,
            "incorrect left clip_offset_utf16 at final offset {:?}",
            offset_utf16
        );
        assert_eq!(
            chunk.clip_offset_utf16(offset_utf16, Bias::Right),
            offset_utf16,
            "incorrect right clip_offset_utf16 at final offset {:?}",
            offset_utf16
        );

        // Verify length methods
        assert_eq!(chunk.len(), text.len());
        assert_eq!(
            chunk.len_utf16().0,
            text.chars().map(|c| c.len_utf16()).sum::<usize>()
        );

        // Verify line counting
        let lines = chunk.lines();
        let mut newline_count = 0;
        let mut last_line_len = 0;
        for c in text.chars() {
            if c == '\n' {
                newline_count += 1;
                last_line_len = 0;
            } else {
                last_line_len += c.len_utf8() as u32;
            }
        }
        assert_eq!(lines, Point::new(newline_count, last_line_len));

        // Verify first/last line chars
        if !text.is_empty() {
            let first_line = text.split('\n').next().unwrap();
            assert_eq!(chunk.first_line_chars(), first_line.chars().count() as u32);

            let last_line = text.split('\n').next_back().unwrap();
            assert_eq!(chunk.last_line_chars(), last_line.chars().count() as u32);
            assert_eq!(
                chunk.last_line_len_utf16(),
                last_line.chars().map(|c| c.len_utf16() as u32).sum::<u32>()
            );
        }

        // Verify longest row
        let (longest_row, longest_chars) = chunk.longest_row(&mut 0);
        let mut max_chars = 0;
        let mut current_row = 0;
        let mut current_chars = 0;
        let mut max_row = 0;

        for c in text.chars() {
            if c == '\n' {
                if current_chars > max_chars {
                    max_chars = current_chars;
                    max_row = current_row;
                }
                current_row += 1;
                current_chars = 0;
            } else {
                current_chars += 1;
            }
        }

        if current_chars > max_chars {
            max_chars = current_chars;
            max_row = current_row;
        }

        assert_eq!((max_row, max_chars as u32), (longest_row, longest_chars));
        assert_eq!(chunk.tabs().collect::<Vec<_>>(), expected_tab_positions);
    }
}

use crate::{OffsetUtf16, Point, PointUtf16, TextSummary, Unclipped};
use arrayvec::ArrayString;
use std::{cmp, ops::Range};
use sum_tree::Bias;
use unicode_segmentation::GraphemeCursor;
use util::debug_panic;

#[cfg(test)]
pub(crate) const MIN_BASE: usize = 6;

#[cfg(not(test))]
pub(crate) const MIN_BASE: usize = 32;

pub(crate) const MAX_BASE: usize = MIN_BASE * 2;

#[derive(Clone, Debug, Default)]
pub struct Chunk {
    chars: usize,
    chars_utf16: usize,
    tabs: usize,
    newlines: usize,
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
            self.chars_utf16 |= c.len_utf16() << ix;
            self.tabs |= ((c == '\t') as usize) << ix;
            self.newlines |= ((c == '\n') as usize) << ix;
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
        self.tabs |= slice.tabs << base_ix;
        self.newlines |= slice.newlines << base_ix;
        self.text.push_str(&slice.text);
    }

    #[inline(always)]
    pub fn as_slice(&self) -> ChunkSlice {
        ChunkSlice {
            chars: self.chars,
            chars_utf16: self.chars_utf16,
            tabs: self.tabs,
            newlines: self.newlines,
            text: &self.text,
        }
    }

    #[inline(always)]
    pub fn slice(&self, range: Range<usize>) -> ChunkSlice {
        self.as_slice().slice(range)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkSlice<'a> {
    chars: usize,
    chars_utf16: usize,
    tabs: usize,
    newlines: usize,
    text: &'a str,
}

impl<'a> Into<Chunk> for ChunkSlice<'a> {
    fn into(self) -> Chunk {
        Chunk {
            chars: self.chars,
            chars_utf16: self.chars_utf16,
            tabs: self.tabs,
            newlines: self.newlines,
            text: self.text.try_into().unwrap(),
        }
    }
}

impl<'a> ChunkSlice<'a> {
    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.text.is_empty()
    }

    #[inline(always)]
    pub fn is_char_boundary(self, offset: usize) -> bool {
        self.text.is_char_boundary(offset)
    }

    #[inline(always)]
    pub fn split_at(self, mid: usize) -> (ChunkSlice<'a>, ChunkSlice<'a>) {
        if mid == 64 {
            let left = self;
            let right = ChunkSlice {
                chars: 0,
                chars_utf16: 0,
                tabs: 0,
                newlines: 0,
                text: "",
            };
            (left, right)
        } else {
            let mask = ((1u128 << mid) - 1) as usize;
            let (left_text, right_text) = self.text.split_at(mid);
            let left = ChunkSlice {
                chars: self.chars & mask,
                chars_utf16: self.chars_utf16 & mask,
                tabs: self.tabs & mask,
                newlines: self.newlines & mask,
                text: left_text,
            };
            let right = ChunkSlice {
                chars: self.chars >> mid,
                chars_utf16: self.chars_utf16 >> mid,
                tabs: self.tabs >> mid,
                newlines: self.newlines >> mid,
                text: right_text,
            };
            (left, right)
        }
    }

    #[inline(always)]
    pub fn slice(self, range: Range<usize>) -> Self {
        let mask = ((1u128 << range.end) - 1) as usize;
        if range.start == 64 {
            Self {
                chars: 0,
                chars_utf16: 0,
                tabs: 0,
                newlines: 0,
                text: "",
            }
        } else {
            Self {
                chars: (self.chars & mask) >> range.start,
                chars_utf16: (self.chars_utf16 & mask) >> range.start,
                tabs: (self.tabs & mask) >> range.start,
                newlines: (self.newlines & mask) >> range.start,
                text: &self.text[range],
            }
        }
    }

    #[inline(always)]
    pub fn text_summary(&self) -> TextSummary {
        let (longest_row, longest_row_chars) = self.longest_row();
        TextSummary {
            len: self.len(),
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
        let column = self.newlines.leading_zeros() - (usize::BITS - self.text.len() as u32);
        Point::new(row, column)
    }

    /// Get number of chars in first line
    #[inline(always)]
    pub fn first_line_chars(&self) -> u32 {
        if self.newlines == 0 {
            self.chars.count_ones()
        } else {
            let mask = ((1u128 << self.newlines.trailing_zeros() as usize) - 1) as usize;
            (self.chars & mask).count_ones()
        }
    }

    /// Get number of chars in last line
    #[inline(always)]
    pub fn last_line_chars(&self) -> u32 {
        if self.newlines == 0 {
            self.chars.count_ones()
        } else {
            let mask = !(usize::MAX >> self.newlines.leading_zeros());
            (self.chars & mask).count_ones()
        }
    }

    /// Get number of UTF-16 code units in last line
    #[inline(always)]
    pub fn last_line_len_utf16(&self) -> u32 {
        if self.newlines == 0 {
            self.chars_utf16.count_ones()
        } else {
            let mask = !(usize::MAX >> self.newlines.leading_zeros());
            (self.chars_utf16 & mask).count_ones()
        }
    }

    /// Get the longest row in the chunk and its length in characters.
    #[inline(always)]
    pub fn longest_row(&self) -> (u32, u32) {
        let mut chars = self.chars;
        let mut newlines = self.newlines;
        let mut row = 0;
        let mut longest_row = 0;
        let mut longest_row_chars = 0;
        while newlines > 0 {
            let newline_ix = newlines.trailing_zeros();
            let row_chars = (chars & ((1 << newline_ix) - 1)).count_ones() as u8;
            if row_chars > longest_row_chars {
                longest_row = row;
                longest_row_chars = row_chars;
            }

            newlines >>= newline_ix;
            newlines >>= 1;
            chars >>= newline_ix;
            chars >>= 1;
            row += 1;
        }

        let row_chars = chars.count_ones() as u8;
        if row_chars > longest_row_chars {
            (row, row_chars as u32)
        } else {
            (longest_row, longest_row_chars as u32)
        }
    }

    #[inline(always)]
    pub fn offset_to_point(&self, offset: usize) -> Point {
        if !self.text.is_char_boundary(offset) {
            debug_panic!(
                "offset {:?} is not a char boundary for string {:?}",
                offset,
                self.text
            );
            return Point::zero();
        }

        let mask = ((1u128 << offset) - 1) as usize;
        let row = (self.newlines & mask).count_ones();
        let newline_ix = usize::BITS - (self.newlines & mask).leading_zeros();
        let column = (offset - newline_ix as usize) as u32;
        Point::new(row, column)
    }

    #[inline(always)]
    pub fn point_to_offset(&self, point: Point) -> usize {
        if point.row > self.newlines.count_ones() {
            debug_panic!(
                "point {:?} extends beyond rows for string {:?}",
                point,
                self.text
            );
            return 0;
        }

        let row_start_offset = if point.row > 0 {
            (nth_set_bit(self.newlines, point.row as usize) + 1) as usize
        } else {
            0
        };

        let newlines = if row_start_offset == usize::BITS as usize {
            0
        } else {
            self.newlines >> row_start_offset
        };
        let row_len = cmp::min(newlines.trailing_zeros(), self.text.len() as u32);
        if point.column > row_len {
            debug_panic!(
                "point {:?} extends beyond row for string {:?}",
                point,
                self.text
            );
            return row_start_offset + row_len as usize;
        }

        row_start_offset + point.column as usize
    }

    #[inline(always)]
    pub fn offset_to_offset_utf16(&self, offset: usize) -> OffsetUtf16 {
        let mask = ((1u128 << offset) - 1) as usize;
        OffsetUtf16((self.chars_utf16 & mask).count_ones() as usize)
    }

    #[inline(always)]
    pub fn offset_utf16_to_offset(&self, target: OffsetUtf16) -> usize {
        if target.0 == 0 {
            0
        } else {
            let ix = nth_set_bit(self.chars_utf16, target.0) + 1;
            if ix == 64 {
                64
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
        let mask = ((1u128 << offset) - 1) as usize;
        let row = (self.newlines & mask).count_ones();
        let newline_ix = usize::BITS - (self.newlines & mask).leading_zeros();
        let column = if newline_ix == 64 {
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

        let row_start_offset = if point.row > 0 {
            (nth_set_bit(self.newlines, point.row as usize) + 1) as usize
        } else {
            0
        };

        let row_len_utf8 = if row_start_offset == 64 {
            0
        } else {
            cmp::min(
                (self.newlines >> row_start_offset).trailing_zeros(),
                (self.text.len() - row_start_offset) as u32,
            )
        };
        let mask = ((1u128 << row_len_utf8) - 1) as usize;
        let row_chars_utf16 = if row_start_offset == 64 {
            0
        } else {
            (self.chars_utf16 >> row_start_offset) & mask
        };
        if point.column > row_chars_utf16.count_ones() {
            if !clip {
                debug_panic!(
                    "point {:?} is beyond the end of the line in chunk {:?}",
                    point,
                    self.text
                );
            }

            return row_start_offset + row_len_utf8 as usize;
        }

        let mut offset = row_start_offset;
        if point.column > 0 {
            let offset_within_row = nth_set_bit(row_chars_utf16, point.column as usize) + 1;
            offset += offset_within_row;
            if offset < 64 {
                offset += cmp::min(
                    (self.chars_utf16 >> offset).trailing_zeros() as usize,
                    self.text.len() - offset,
                );
            }

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

    pub fn unclipped_point_utf16_to_point(&self, target: Unclipped<PointUtf16>) -> Point {
        let mut point = Point::zero();
        let mut point_utf16 = PointUtf16::zero();

        for ch in self.text.chars() {
            if point_utf16 == target.0 {
                break;
            }

            if point_utf16 > target.0 {
                // If the point is past the end of a line or inside of a code point,
                // return the last valid point before the target.
                return point;
            }

            if ch == '\n' {
                point_utf16 += PointUtf16::new(1, 0);
                point += Point::new(1, 0);
            } else {
                point_utf16 += PointUtf16::new(0, ch.len_utf16() as u32);
                point += Point::new(0, ch.len_utf8() as u32);
            }
        }

        point
    }

    // todo!("use bitsets")
    pub fn clip_point(&self, target: Point, bias: Bias) -> Point {
        for (row, line) in self.text.split('\n').enumerate() {
            if row == target.row as usize {
                let bytes = line.as_bytes();
                let mut column = target.column.min(bytes.len() as u32) as usize;
                if column == 0
                    || column == bytes.len()
                    || (bytes[column - 1] < 128 && bytes[column] < 128)
                {
                    return Point::new(row as u32, column as u32);
                }

                let mut grapheme_cursor = GraphemeCursor::new(column, bytes.len(), true);
                loop {
                    if line.is_char_boundary(column)
                        && grapheme_cursor.is_boundary(line, 0).unwrap_or(false)
                    {
                        break;
                    }

                    match bias {
                        Bias::Left => column -= 1,
                        Bias::Right => column += 1,
                    }
                    grapheme_cursor.set_cursor(column);
                }
                return Point::new(row as u32, column as u32);
            }
        }
        unreachable!()
    }

    // todo!("use bitsets")
    pub fn clip_point_utf16(&self, target: Unclipped<PointUtf16>, bias: Bias) -> PointUtf16 {
        for (row, line) in self.text.split('\n').enumerate() {
            if row == target.0.row as usize {
                let mut code_units = line.encode_utf16();
                let mut column = code_units.by_ref().take(target.0.column as usize).count();
                if char::decode_utf16(code_units).next().transpose().is_err() {
                    match bias {
                        Bias::Left => column -= 1,
                        Bias::Right => column += 1,
                    }
                }
                return PointUtf16::new(row as u32, column as u32);
            }
        }
        unreachable!()
    }

    // todo!("use bitsets")
    pub fn clip_offset_utf16(&self, target: OffsetUtf16, bias: Bias) -> OffsetUtf16 {
        let mut code_units = self.text.encode_utf16();
        let mut offset = code_units.by_ref().take(target.0).count();
        if char::decode_utf16(code_units).next().transpose().is_err() {
            match bias {
                Bias::Left => offset -= 1,
                Bias::Right => offset += 1,
            }
        }
        OffsetUtf16(offset)
    }
}

/// Finds the n-th bit that is set to 1.
#[inline(always)]
fn nth_set_bit(v: usize, mut n: usize) -> usize {
    let v = v.reverse_bits();
    let mut s: usize = 64;
    let mut t: usize;

    // Parallel bit count intermediates
    let a = v - ((v >> 1) & usize::MAX / 3);
    let b = (a & usize::MAX / 5) + ((a >> 2) & usize::MAX / 5);
    let c = (b + (b >> 4)) & usize::MAX / 0x11;
    let d = (c + (c >> 8)) & usize::MAX / 0x101;
    t = (d >> 32) + (d >> 48);

    // Branchless select
    s -= ((t.wrapping_sub(n)) & 256) >> 3;
    n -= t & ((t.wrapping_sub(n)) >> 8);

    t = (d >> (s - 16)) & 0xff;
    s -= ((t.wrapping_sub(n)) & 256) >> 4;
    n -= t & ((t.wrapping_sub(n)) >> 8);

    t = (c >> (s - 8)) & 0xf;
    s -= ((t.wrapping_sub(n)) & 256) >> 5;
    n -= t & ((t.wrapping_sub(n)) >> 8);

    t = (b >> (s - 4)) & 0x7;
    s -= ((t.wrapping_sub(n)) & 256) >> 6;
    n -= t & ((t.wrapping_sub(n)) >> 8);

    t = (a >> (s - 2)) & 0x3;
    s -= ((t.wrapping_sub(n)) & 256) >> 7;
    n -= t & ((t.wrapping_sub(n)) >> 8);

    t = (v >> (s - 1)) & 0x1;
    s -= ((t.wrapping_sub(n)) & 256) >> 8;

    65 - s - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use util::RandomCharIter;

    #[gpui::test(iterations = 100)]
    fn test_random_chunks(mut rng: StdRng) {
        let max_len = std::env::var("CHUNK_MAX_LEN")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(64);
        let chunk_len = rng.gen_range(0..=max_len);
        let text = RandomCharIter::new(&mut rng)
            .take(chunk_len)
            .collect::<String>();
        let mut ix = chunk_len;
        while !text.is_char_boundary(ix) {
            ix -= 1;
        }
        let text = &text[..ix];

        log::info!("Chunk: {:?}", text);
        let chunk = Chunk::new(&text);
        verify_chunk(chunk.as_slice(), text);

        for _ in 0..10 {
            let mut start = rng.gen_range(0..=chunk.text.len());
            let mut end = rng.gen_range(start..=chunk.text.len());
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

    #[test]
    fn test_nth_set_bit() {
        assert_eq!(
            nth_set_bit(
                0b1000000000000000000000000000000000000000000000000000000000000000,
                1
            ),
            63
        );
        assert_eq!(
            nth_set_bit(
                0b1100000000000000000000000000000000000000000000000000000000000000,
                1
            ),
            62
        );
        assert_eq!(
            nth_set_bit(
                0b1100000000000000000000000000000000000000000000000000000000000000,
                2
            ),
            63
        );
        assert_eq!(
            nth_set_bit(
                0b0000000000000000000000000000000000000000000000000000000000000001,
                1
            ),
            0
        );
        assert_eq!(
            nth_set_bit(
                0b0000000000000000000000000000000000000000000000000000000000000011,
                2
            ),
            1
        );
        assert_eq!(
            nth_set_bit(
                0b0101010101010101010101010101010101010101010101010101010101010101,
                1
            ),
            0
        );
        assert_eq!(
            nth_set_bit(
                0b0101010101010101010101010101010101010101010101010101010101010101,
                32
            ),
            62
        );
        assert_eq!(
            nth_set_bit(
                0b1111111111111111111111111111111111111111111111111111111111111111,
                64
            ),
            63
        );
        assert_eq!(
            nth_set_bit(
                0b1111111111111111111111111111111111111111111111111111111111111111,
                1
            ),
            0
        );
        assert_eq!(
            nth_set_bit(
                0b1010101010101010101010101010101010101010101010101010101010101010,
                1
            ),
            1
        );
        assert_eq!(
            nth_set_bit(
                0b1111000011110000111100001111000011110000111100001111000011110000,
                8
            ),
            15
        );
    }

    fn verify_chunk(chunk: ChunkSlice<'_>, text: &str) {
        let mut offset = 0;
        let mut offset_utf16 = OffsetUtf16(0);
        let mut point = Point::zero();
        let mut point_utf16 = PointUtf16::zero();

        log::info!("Verifying chunk {:?}", text);
        assert_eq!(chunk.offset_to_point(0), Point::zero());

        for c in text.chars() {
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

            if c == '\n' {
                point.row += 1;
                point.column = 0;
                point_utf16.row += 1;
                point_utf16.column = 0;
            } else {
                point.column += c.len_utf8() as u32;
                point_utf16.column += c.len_utf16() as u32;
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

            let last_line = text.split('\n').last().unwrap();
            assert_eq!(chunk.last_line_chars(), last_line.chars().count() as u32);
            assert_eq!(
                chunk.last_line_len_utf16(),
                last_line.chars().map(|c| c.len_utf16() as u32).sum::<u32>()
            );
        }

        // Verify longest row
        let (longest_row, longest_chars) = chunk.longest_row();
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
    }
}

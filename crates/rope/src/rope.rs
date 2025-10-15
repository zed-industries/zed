mod chunk;
mod offset_utf16;
mod point;
mod point_utf16;
mod unclipped;

use chunk::Chunk;
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};
use smallvec::SmallVec;
use std::{
    cmp, fmt, io, mem,
    ops::{self, AddAssign, Range},
    str,
};
use sum_tree::{Bias, Dimension, Dimensions, SumTree};

pub use chunk::ChunkSlice;
pub use offset_utf16::OffsetUtf16;
pub use point::Point;
pub use point_utf16::PointUtf16;
pub use unclipped::Unclipped;

#[derive(Clone, Default)]
pub struct Rope {
    chunks: SumTree<Chunk>,
}

impl Rope {
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks that `index`-th byte is the first byte in a UTF-8 code point
    /// sequence or the end of the string.
    ///
    /// The start and end of the string (when `index == self.len()`) are
    /// considered to be boundaries.
    ///
    /// Returns `false` if `index` is greater than `self.len()`.
    pub fn is_char_boundary(&self, offset: usize) -> bool {
        if self.chunks.is_empty() {
            return offset == 0;
        }
        let mut cursor = self.chunks.cursor::<usize>(());
        cursor.seek(&offset, Bias::Left);
        let chunk_offset = offset - cursor.start();
        cursor
            .item()
            .map(|chunk| chunk.text.is_char_boundary(chunk_offset))
            .unwrap_or(false)
    }

    pub fn floor_char_boundary(&self, index: usize) -> usize {
        if index >= self.len() {
            self.len()
        } else {
            #[inline]
            pub(crate) const fn is_utf8_char_boundary(u8: u8) -> bool {
                // This is bit magic equivalent to: b < 128 || b >= 192
                (u8 as i8) >= -0x40
            }

            let mut cursor = self.chunks.cursor::<usize>(());
            cursor.seek(&index, Bias::Left);
            let chunk_offset = index - cursor.start();
            let lower_idx = cursor.item().map(|chunk| {
                let lower_bound = chunk_offset.saturating_sub(3);
                chunk
                    .text
                    .as_bytes()
                    .get(lower_bound..=chunk_offset)
                    .map(|it| {
                        let new_idx = it
                            .iter()
                            .rposition(|&b| is_utf8_char_boundary(b))
                            .unwrap_or(0);
                        lower_bound + new_idx
                    })
                    .unwrap_or(chunk.text.len())
            });
            lower_idx.map_or_else(|| self.len(), |idx| cursor.start() + idx)
        }
    }

    pub fn ceil_char_boundary(&self, index: usize) -> usize {
        if index > self.len() {
            self.len()
        } else {
            #[inline]
            pub(crate) const fn is_utf8_char_boundary(u8: u8) -> bool {
                // This is bit magic equivalent to: b < 128 || b >= 192
                (u8 as i8) >= -0x40
            }

            let mut cursor = self.chunks.cursor::<usize>(());
            cursor.seek(&index, Bias::Left);
            let chunk_offset = index - cursor.start();
            let upper_idx = cursor.item().map(|chunk| {
                let upper_bound = Ord::min(chunk_offset + 4, chunk.text.len());
                chunk.text.as_bytes()[chunk_offset..upper_bound]
                    .iter()
                    .position(|&b| is_utf8_char_boundary(b))
                    .map_or(upper_bound, |pos| pos + chunk_offset)
            });

            upper_idx.map_or_else(|| self.len(), |idx| cursor.start() + idx)
        }
    }

    pub fn append(&mut self, rope: Rope) {
        if let Some(chunk) = rope.chunks.first()
            && (self
                .chunks
                .last()
                .is_some_and(|c| c.text.len() < chunk::MIN_BASE)
                || chunk.text.len() < chunk::MIN_BASE)
        {
            self.push_chunk(chunk.as_slice());

            let mut chunks = rope.chunks.cursor::<()>(());
            chunks.next();
            chunks.next();
            self.chunks.append(chunks.suffix(), ());
        } else {
            self.chunks.append(rope.chunks, ());
        }
        self.check_invariants();
    }

    pub fn replace(&mut self, range: Range<usize>, text: &str) {
        let mut new_rope = Rope::new();
        let mut cursor = self.cursor(0);
        new_rope.append(cursor.slice(range.start));
        cursor.seek_forward(range.end);
        new_rope.push(text);
        new_rope.append(cursor.suffix());
        *self = new_rope;
    }

    pub fn slice(&self, range: Range<usize>) -> Rope {
        let mut cursor = self.cursor(0);
        cursor.seek_forward(range.start);
        cursor.slice(range.end)
    }

    pub fn slice_rows(&self, range: Range<u32>) -> Rope {
        // This would be more efficient with a forward advance after the first, but it's fine.
        let start = self.point_to_offset(Point::new(range.start, 0));
        let end = self.point_to_offset(Point::new(range.end, 0));
        self.slice(start..end)
    }

    pub fn push(&mut self, mut text: &str) {
        self.chunks.update_last(
            |last_chunk| {
                let split_ix = if last_chunk.text.len() + text.len() <= chunk::MAX_BASE {
                    text.len()
                } else {
                    let mut split_ix = cmp::min(
                        chunk::MIN_BASE.saturating_sub(last_chunk.text.len()),
                        text.len(),
                    );
                    while !text.is_char_boundary(split_ix) {
                        split_ix += 1;
                    }
                    split_ix
                };

                let (suffix, remainder) = text.split_at(split_ix);
                last_chunk.push_str(suffix);
                text = remainder;
            },
            (),
        );

        if text.len() > 2048 {
            return self.push_large(text);
        }
        let mut new_chunks = SmallVec::<[_; 16]>::new();

        while !text.is_empty() {
            let mut split_ix = cmp::min(chunk::MAX_BASE, text.len());
            while !text.is_char_boundary(split_ix) {
                split_ix -= 1;
            }
            let (chunk, remainder) = text.split_at(split_ix);
            new_chunks.push(chunk);
            text = remainder;
        }

        #[cfg(test)]
        const PARALLEL_THRESHOLD: usize = 4;
        #[cfg(not(test))]
        const PARALLEL_THRESHOLD: usize = 4 * (2 * sum_tree::TREE_BASE);

        if new_chunks.len() >= PARALLEL_THRESHOLD {
            self.chunks
                .par_extend(new_chunks.into_vec().into_par_iter().map(Chunk::new), ());
        } else {
            self.chunks
                .extend(new_chunks.into_iter().map(Chunk::new), ());
        }

        self.check_invariants();
    }

    /// A copy of `push` specialized for working with large quantities of text.
    fn push_large(&mut self, mut text: &str) {
        // To avoid frequent reallocs when loading large swaths of file contents,
        // we estimate worst-case `new_chunks` capacity;
        // Chunk is a fixed-capacity buffer. If a character falls on
        // chunk boundary, we push it off to the following chunk (thus leaving a small bit of capacity unfilled in current chunk).
        // Worst-case chunk count when loading a file is then a case where every chunk ends up with that unused capacity.
        // Since we're working with UTF-8, each character is at most 4 bytes wide. It follows then that the worst case is where
        // a chunk ends with 3 bytes of a 4-byte character. These 3 bytes end up being stored in the following chunk, thus wasting
        // 3 bytes of storage in current chunk.
        // For example, a 1024-byte string can occupy between 32 (full ASCII, 1024/32) and 36 (full 4-byte UTF-8, 1024 / 29 rounded up) chunks.
        const MIN_CHUNK_SIZE: usize = chunk::MAX_BASE - 3;

        // We also round up the capacity up by one, for a good measure; we *really* don't want to realloc here, as we assume that the # of characters
        // we're working with there is large.
        let capacity = text.len().div_ceil(MIN_CHUNK_SIZE);
        let mut new_chunks = Vec::with_capacity(capacity);

        while !text.is_empty() {
            let mut split_ix = cmp::min(chunk::MAX_BASE, text.len());
            while !text.is_char_boundary(split_ix) {
                split_ix -= 1;
            }
            let (chunk, remainder) = text.split_at(split_ix);
            new_chunks.push(chunk);
            text = remainder;
        }

        #[cfg(test)]
        const PARALLEL_THRESHOLD: usize = 4;
        #[cfg(not(test))]
        const PARALLEL_THRESHOLD: usize = 4 * (2 * sum_tree::TREE_BASE);

        if new_chunks.len() >= PARALLEL_THRESHOLD {
            self.chunks
                .par_extend(new_chunks.into_par_iter().map(Chunk::new), ());
        } else {
            self.chunks
                .extend(new_chunks.into_iter().map(Chunk::new), ());
        }

        self.check_invariants();
    }

    fn push_chunk(&mut self, mut chunk: ChunkSlice) {
        self.chunks.update_last(
            |last_chunk| {
                let split_ix = if last_chunk.text.len() + chunk.len() <= chunk::MAX_BASE {
                    chunk.len()
                } else {
                    let mut split_ix = cmp::min(
                        chunk::MIN_BASE.saturating_sub(last_chunk.text.len()),
                        chunk.len(),
                    );
                    while !chunk.is_char_boundary(split_ix) {
                        split_ix += 1;
                    }
                    split_ix
                };

                let (suffix, remainder) = chunk.split_at(split_ix);
                last_chunk.append(suffix);
                chunk = remainder;
            },
            (),
        );

        if !chunk.is_empty() {
            self.chunks.push(chunk.into(), ());
        }
    }

    pub fn push_front(&mut self, text: &str) {
        let suffix = mem::replace(self, Rope::from(text));
        self.append(suffix);
    }

    fn check_invariants(&self) {
        #[cfg(test)]
        {
            // Ensure all chunks except maybe the last one are not underflowing.
            // Allow some wiggle room for multibyte characters at chunk boundaries.
            let mut chunks = self.chunks.cursor::<()>(()).peekable();
            while let Some(chunk) = chunks.next() {
                if chunks.peek().is_some() {
                    assert!(chunk.text.len() + 3 >= chunk::MIN_BASE);
                }
            }
        }
    }

    pub fn summary(&self) -> TextSummary {
        self.chunks.summary().text
    }

    pub fn len(&self) -> usize {
        self.chunks.extent(())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn max_point(&self) -> Point {
        self.chunks.extent(())
    }

    pub fn max_point_utf16(&self) -> PointUtf16 {
        self.chunks.extent(())
    }

    pub fn cursor(&self, offset: usize) -> Cursor<'_> {
        Cursor::new(self, offset)
    }

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars_at(0)
    }

    pub fn chars_at(&self, start: usize) -> impl Iterator<Item = char> + '_ {
        self.chunks_in_range(start..self.len()).flat_map(str::chars)
    }

    pub fn reversed_chars_at(&self, start: usize) -> impl Iterator<Item = char> + '_ {
        self.reversed_chunks_in_range(0..start)
            .flat_map(|chunk| chunk.chars().rev())
    }

    pub fn bytes_in_range(&self, range: Range<usize>) -> Bytes<'_> {
        Bytes::new(self, range, false)
    }

    pub fn reversed_bytes_in_range(&self, range: Range<usize>) -> Bytes<'_> {
        Bytes::new(self, range, true)
    }

    pub fn chunks(&self) -> Chunks<'_> {
        self.chunks_in_range(0..self.len())
    }

    pub fn chunks_in_range(&self, range: Range<usize>) -> Chunks<'_> {
        Chunks::new(self, range, false)
    }

    pub fn reversed_chunks_in_range(&self, range: Range<usize>) -> Chunks<'_> {
        Chunks::new(self, range, true)
    }

    pub fn offset_to_offset_utf16(&self, offset: usize) -> OffsetUtf16 {
        if offset >= self.summary().len {
            return self.summary().len_utf16;
        }
        let mut cursor = self.chunks.cursor::<Dimensions<usize, OffsetUtf16>>(());
        cursor.seek(&offset, Bias::Left);
        let overshoot = offset - cursor.start().0;
        cursor.start().1
            + cursor.item().map_or(Default::default(), |chunk| {
                chunk.as_slice().offset_to_offset_utf16(overshoot)
            })
    }

    pub fn offset_utf16_to_offset(&self, offset: OffsetUtf16) -> usize {
        if offset >= self.summary().len_utf16 {
            return self.summary().len;
        }
        let mut cursor = self.chunks.cursor::<Dimensions<OffsetUtf16, usize>>(());
        cursor.seek(&offset, Bias::Left);
        let overshoot = offset - cursor.start().0;
        cursor.start().1
            + cursor.item().map_or(Default::default(), |chunk| {
                chunk.as_slice().offset_utf16_to_offset(overshoot)
            })
    }

    pub fn offset_to_point(&self, offset: usize) -> Point {
        if offset >= self.summary().len {
            return self.summary().lines;
        }
        let mut cursor = self.chunks.cursor::<Dimensions<usize, Point>>(());
        cursor.seek(&offset, Bias::Left);
        let overshoot = offset - cursor.start().0;
        cursor.start().1
            + cursor.item().map_or(Point::zero(), |chunk| {
                chunk.as_slice().offset_to_point(overshoot)
            })
    }

    pub fn offset_to_point_utf16(&self, offset: usize) -> PointUtf16 {
        if offset >= self.summary().len {
            return self.summary().lines_utf16();
        }
        let mut cursor = self.chunks.cursor::<Dimensions<usize, PointUtf16>>(());
        cursor.seek(&offset, Bias::Left);
        let overshoot = offset - cursor.start().0;
        cursor.start().1
            + cursor.item().map_or(PointUtf16::zero(), |chunk| {
                chunk.as_slice().offset_to_point_utf16(overshoot)
            })
    }

    pub fn point_to_point_utf16(&self, point: Point) -> PointUtf16 {
        if point >= self.summary().lines {
            return self.summary().lines_utf16();
        }
        let mut cursor = self.chunks.cursor::<Dimensions<Point, PointUtf16>>(());
        cursor.seek(&point, Bias::Left);
        let overshoot = point - cursor.start().0;
        cursor.start().1
            + cursor.item().map_or(PointUtf16::zero(), |chunk| {
                chunk.as_slice().point_to_point_utf16(overshoot)
            })
    }

    pub fn point_to_offset(&self, point: Point) -> usize {
        if point >= self.summary().lines {
            return self.summary().len;
        }
        let mut cursor = self.chunks.cursor::<Dimensions<Point, usize>>(());
        cursor.seek(&point, Bias::Left);
        let overshoot = point - cursor.start().0;
        cursor.start().1
            + cursor
                .item()
                .map_or(0, |chunk| chunk.as_slice().point_to_offset(overshoot))
    }

    pub fn point_utf16_to_offset(&self, point: PointUtf16) -> usize {
        self.point_utf16_to_offset_impl(point, false)
    }

    pub fn unclipped_point_utf16_to_offset(&self, point: Unclipped<PointUtf16>) -> usize {
        self.point_utf16_to_offset_impl(point.0, true)
    }

    fn point_utf16_to_offset_impl(&self, point: PointUtf16, clip: bool) -> usize {
        if point >= self.summary().lines_utf16() {
            return self.summary().len;
        }
        let mut cursor = self.chunks.cursor::<Dimensions<PointUtf16, usize>>(());
        cursor.seek(&point, Bias::Left);
        let overshoot = point - cursor.start().0;
        cursor.start().1
            + cursor.item().map_or(0, |chunk| {
                chunk.as_slice().point_utf16_to_offset(overshoot, clip)
            })
    }

    pub fn unclipped_point_utf16_to_point(&self, point: Unclipped<PointUtf16>) -> Point {
        if point.0 >= self.summary().lines_utf16() {
            return self.summary().lines;
        }
        let mut cursor = self.chunks.cursor::<Dimensions<PointUtf16, Point>>(());
        cursor.seek(&point.0, Bias::Left);
        let overshoot = Unclipped(point.0 - cursor.start().0);
        cursor.start().1
            + cursor.item().map_or(Point::zero(), |chunk| {
                chunk.as_slice().unclipped_point_utf16_to_point(overshoot)
            })
    }

    pub fn clip_offset(&self, offset: usize, bias: Bias) -> usize {
        match bias {
            Bias::Left => self.floor_char_boundary(offset),
            Bias::Right => self.ceil_char_boundary(offset),
        }
    }

    pub fn clip_offset_utf16(&self, offset: OffsetUtf16, bias: Bias) -> OffsetUtf16 {
        let mut cursor = self.chunks.cursor::<OffsetUtf16>(());
        cursor.seek(&offset, Bias::Right);
        if let Some(chunk) = cursor.item() {
            let overshoot = offset - cursor.start();
            *cursor.start() + chunk.as_slice().clip_offset_utf16(overshoot, bias)
        } else {
            self.summary().len_utf16
        }
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        let mut cursor = self.chunks.cursor::<Point>(());
        cursor.seek(&point, Bias::Right);
        if let Some(chunk) = cursor.item() {
            let overshoot = point - cursor.start();
            *cursor.start() + chunk.as_slice().clip_point(overshoot, bias)
        } else {
            self.summary().lines
        }
    }

    pub fn clip_point_utf16(&self, point: Unclipped<PointUtf16>, bias: Bias) -> PointUtf16 {
        let mut cursor = self.chunks.cursor::<PointUtf16>(());
        cursor.seek(&point.0, Bias::Right);
        if let Some(chunk) = cursor.item() {
            let overshoot = Unclipped(point.0 - cursor.start());
            *cursor.start() + chunk.as_slice().clip_point_utf16(overshoot, bias)
        } else {
            self.summary().lines_utf16()
        }
    }

    pub fn line_len(&self, row: u32) -> u32 {
        self.clip_point(Point::new(row, u32::MAX), Bias::Left)
            .column
    }
}

impl<'a> From<&'a str> for Rope {
    fn from(text: &'a str) -> Self {
        let mut rope = Self::new();
        rope.push(text);
        rope
    }
}

impl<'a> FromIterator<&'a str> for Rope {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut rope = Rope::new();
        for chunk in iter {
            rope.push(chunk);
        }
        rope
    }
}

impl From<String> for Rope {
    #[inline(always)]
    fn from(text: String) -> Self {
        Rope::from(text.as_str())
    }
}

impl From<&String> for Rope {
    #[inline(always)]
    fn from(text: &String) -> Self {
        Rope::from(text.as_str())
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for chunk in self.chunks() {
            write!(f, "{}", chunk)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Write as _;

        write!(f, "\"")?;
        let mut format_string = String::new();
        for chunk in self.chunks() {
            write!(&mut format_string, "{:?}", chunk)?;
            write!(f, "{}", &format_string[1..format_string.len() - 1])?;
            format_string.clear();
        }
        write!(f, "\"")?;
        Ok(())
    }
}

pub struct Cursor<'a> {
    rope: &'a Rope,
    chunks: sum_tree::Cursor<'a, 'static, Chunk, usize>,
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(rope: &'a Rope, offset: usize) -> Self {
        let mut chunks = rope.chunks.cursor(());
        chunks.seek(&offset, Bias::Right);
        Self {
            rope,
            chunks,
            offset,
        }
    }

    pub fn seek_forward(&mut self, end_offset: usize) {
        debug_assert!(end_offset >= self.offset);

        self.chunks.seek_forward(&end_offset, Bias::Right);
        self.offset = end_offset;
    }

    pub fn slice(&mut self, end_offset: usize) -> Rope {
        debug_assert!(
            end_offset >= self.offset,
            "cannot slice backwards from {} to {}",
            self.offset,
            end_offset
        );

        let mut slice = Rope::new();
        if let Some(start_chunk) = self.chunks.item() {
            let start_ix = self.offset - self.chunks.start();
            let end_ix = cmp::min(end_offset, self.chunks.end()) - self.chunks.start();
            slice.push_chunk(start_chunk.slice(start_ix..end_ix));
        }

        if end_offset > self.chunks.end() {
            self.chunks.next();
            slice.append(Rope {
                chunks: self.chunks.slice(&end_offset, Bias::Right),
            });
            if let Some(end_chunk) = self.chunks.item() {
                let end_ix = end_offset - self.chunks.start();
                slice.push_chunk(end_chunk.slice(0..end_ix));
            }
        }

        self.offset = end_offset;
        slice
    }

    pub fn summary<D: TextDimension>(&mut self, end_offset: usize) -> D {
        debug_assert!(end_offset >= self.offset);

        let mut summary = D::zero(());
        if let Some(start_chunk) = self.chunks.item() {
            let start_ix = self.offset - self.chunks.start();
            let end_ix = cmp::min(end_offset, self.chunks.end()) - self.chunks.start();
            summary.add_assign(&D::from_chunk(start_chunk.slice(start_ix..end_ix)));
        }

        if end_offset > self.chunks.end() {
            self.chunks.next();
            summary.add_assign(&self.chunks.summary(&end_offset, Bias::Right));
            if let Some(end_chunk) = self.chunks.item() {
                let end_ix = end_offset - self.chunks.start();
                summary.add_assign(&D::from_chunk(end_chunk.slice(0..end_ix)));
            }
        }

        self.offset = end_offset;
        summary
    }

    pub fn suffix(mut self) -> Rope {
        self.slice(self.rope.chunks.extent(()))
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

pub struct ChunkBitmaps<'a> {
    /// A slice of text up to 128 bytes in size
    pub text: &'a str,
    /// Bitmap of character locations in text. LSB ordered
    pub chars: u128,
    /// Bitmap of tab locations in text. LSB ordered
    pub tabs: u128,
}

#[derive(Clone)]
pub struct Chunks<'a> {
    chunks: sum_tree::Cursor<'a, 'static, Chunk, usize>,
    range: Range<usize>,
    offset: usize,
    reversed: bool,
}

impl<'a> Chunks<'a> {
    pub fn new(rope: &'a Rope, range: Range<usize>, reversed: bool) -> Self {
        let mut chunks = rope.chunks.cursor(());
        let offset = if reversed {
            chunks.seek(&range.end, Bias::Left);
            range.end
        } else {
            chunks.seek(&range.start, Bias::Right);
            range.start
        };
        let chunk_offset = offset - chunks.start();
        if let Some(chunk) = chunks.item()
            && !chunk.text.is_char_boundary(chunk_offset)
        {
            panic!("byte index {} is not a char boundary", offset);
        }
        Self {
            chunks,
            range,
            offset,
            reversed,
        }
    }

    fn offset_is_valid(&self) -> bool {
        if self.reversed {
            if self.offset <= self.range.start || self.offset > self.range.end {
                return false;
            }
        } else if self.offset < self.range.start || self.offset >= self.range.end {
            return false;
        }

        true
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn seek(&mut self, mut offset: usize) {
        offset = offset.clamp(self.range.start, self.range.end);

        if self.reversed {
            if offset > self.chunks.end() {
                self.chunks.seek_forward(&offset, Bias::Left);
            } else if offset <= *self.chunks.start() {
                self.chunks.seek(&offset, Bias::Left);
            }
        } else {
            if offset >= self.chunks.end() {
                self.chunks.seek_forward(&offset, Bias::Right);
            } else if offset < *self.chunks.start() {
                self.chunks.seek(&offset, Bias::Right);
            }
        };

        self.offset = offset;
    }

    pub fn set_range(&mut self, range: Range<usize>) {
        self.range = range.clone();
        self.seek(range.start);
    }

    /// Moves this cursor to the start of the next line in the rope.
    ///
    /// This method advances the cursor to the beginning of the next line.
    /// If the cursor is already at the end of the rope, this method does nothing.
    /// Reversed chunks iterators are not currently supported and will panic.
    ///
    /// Returns `true` if the cursor was successfully moved to the next line start,
    /// or `false` if the cursor was already at the end of the rope.
    pub fn next_line(&mut self) -> bool {
        assert!(!self.reversed);

        let mut found = false;
        if let Some(chunk) = self.peek() {
            if let Some(newline_ix) = chunk.find('\n') {
                self.offset += newline_ix + 1;
                found = self.offset <= self.range.end;
            } else {
                self.chunks
                    .search_forward(|summary| summary.text.lines.row > 0);
                self.offset = *self.chunks.start();

                if let Some(newline_ix) = self.peek().and_then(|chunk| chunk.find('\n')) {
                    self.offset += newline_ix + 1;
                    found = self.offset <= self.range.end;
                } else {
                    self.offset = self.chunks.end();
                }
            }

            if self.offset == self.chunks.end() {
                self.next();
            }
        }

        if self.offset > self.range.end {
            self.offset = cmp::min(self.offset, self.range.end);
            self.chunks.seek(&self.offset, Bias::Right);
        }

        found
    }

    /// Move this cursor to the preceding position in the rope that starts a new line.
    /// Reversed chunks iterators are not currently supported and will panic.
    ///
    /// If this cursor is not on the start of a line, it will be moved to the start of
    /// its current line. Otherwise it will be moved to the start of the previous line.
    /// It updates the cursor's position and returns true if a previous line was found,
    /// or false if the cursor was already at the start of the rope.
    pub fn prev_line(&mut self) -> bool {
        assert!(!self.reversed);

        let initial_offset = self.offset;

        if self.offset == *self.chunks.start() {
            self.chunks.prev();
        }

        if let Some(chunk) = self.chunks.item() {
            let mut end_ix = self.offset - *self.chunks.start();
            if chunk.text.as_bytes()[end_ix - 1] == b'\n' {
                end_ix -= 1;
            }

            if let Some(newline_ix) = chunk.text[..end_ix].rfind('\n') {
                self.offset = *self.chunks.start() + newline_ix + 1;
                if self.offset_is_valid() {
                    return true;
                }
            }
        }

        self.chunks
            .search_backward(|summary| summary.text.lines.row > 0);
        self.offset = *self.chunks.start();
        if let Some(chunk) = self.chunks.item()
            && let Some(newline_ix) = chunk.text.rfind('\n')
        {
            self.offset += newline_ix + 1;
            if self.offset_is_valid() {
                if self.offset == self.chunks.end() {
                    self.chunks.next();
                }

                return true;
            }
        }

        if !self.offset_is_valid() || self.chunks.item().is_none() {
            self.offset = self.range.start;
            self.chunks.seek(&self.offset, Bias::Right);
        }

        self.offset < initial_offset && self.offset == 0
    }

    /// Returns bitmaps that represent character positions and tab positions
    pub fn peek_with_bitmaps(&self) -> Option<ChunkBitmaps<'a>> {
        if !self.offset_is_valid() {
            return None;
        }

        let chunk = self.chunks.item()?;
        let chunk_start = *self.chunks.start();
        let slice_range = if self.reversed {
            let slice_start = cmp::max(chunk_start, self.range.start) - chunk_start;
            let slice_end = self.offset - chunk_start;
            slice_start..slice_end
        } else {
            let slice_start = self.offset - chunk_start;
            let slice_end = cmp::min(self.chunks.end(), self.range.end) - chunk_start;
            slice_start..slice_end
        };

        // slice range has a bounds between 0 and 128 in non test builds
        // We use a non wrapping sub because we want to overflow in the case where slice_range.end == 128
        // because that represents a full chunk and the bitmask shouldn't remove anything
        let bitmask = (1u128.unbounded_shl(slice_range.end as u32)).wrapping_sub(1);

        let chars = (chunk.chars() & bitmask) >> slice_range.start;
        let tabs = (chunk.tabs & bitmask) >> slice_range.start;

        Some(ChunkBitmaps {
            text: &chunk.text[slice_range],
            chars,
            tabs,
        })
    }

    pub fn peek(&self) -> Option<&'a str> {
        if !self.offset_is_valid() {
            return None;
        }

        let chunk = self.chunks.item()?;
        let chunk_start = *self.chunks.start();
        let slice_range = if self.reversed {
            let slice_start = cmp::max(chunk_start, self.range.start) - chunk_start;
            let slice_end = self.offset - chunk_start;
            slice_start..slice_end
        } else {
            let slice_start = self.offset - chunk_start;
            let slice_end = cmp::min(self.chunks.end(), self.range.end) - chunk_start;
            slice_start..slice_end
        };

        Some(&chunk.text[slice_range])
    }

    pub fn peek_tabs(&self) -> Option<ChunkBitmaps<'a>> {
        if !self.offset_is_valid() {
            return None;
        }

        let chunk = self.chunks.item()?;
        let chunk_start = *self.chunks.start();
        let slice_range = if self.reversed {
            let slice_start = cmp::max(chunk_start, self.range.start) - chunk_start;
            let slice_end = self.offset - chunk_start;
            slice_start..slice_end
        } else {
            let slice_start = self.offset - chunk_start;
            let slice_end = cmp::min(self.chunks.end(), self.range.end) - chunk_start;
            slice_start..slice_end
        };
        let chunk_start_offset = slice_range.start;
        let slice_text = &chunk.text[slice_range];

        // Shift the tabs to align with our slice window
        let shifted_tabs = chunk.tabs >> chunk_start_offset;
        let shifted_chars = chunk.chars() >> chunk_start_offset;

        Some(ChunkBitmaps {
            text: slice_text,
            chars: shifted_chars,
            tabs: shifted_tabs,
        })
    }

    pub fn lines(self) -> Lines<'a> {
        let reversed = self.reversed;
        Lines {
            chunks: self,
            current_line: String::new(),
            done: false,
            reversed,
        }
    }

    pub fn equals_str(&self, other: &str) -> bool {
        let chunk = self.clone();
        if chunk.reversed {
            let mut offset = other.len();
            for chunk in chunk {
                if other[0..offset].ends_with(chunk) {
                    offset -= chunk.len();
                } else {
                    return false;
                }
            }
            if offset != 0 {
                return false;
            }
        } else {
            let mut offset = 0;
            for chunk in chunk {
                if offset >= other.len() {
                    return false;
                }
                if other[offset..].starts_with(chunk) {
                    offset += chunk.len();
                } else {
                    return false;
                }
            }
            if offset != other.len() {
                return false;
            }
        }

        true
    }
}

pub struct ChunkWithBitmaps<'a>(pub Chunks<'a>);

impl<'a> Iterator for ChunkWithBitmaps<'a> {
    /// text, chars bitmap, tabs bitmap
    type Item = ChunkBitmaps<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk_bitmaps = self.0.peek_with_bitmaps()?;
        if self.0.reversed {
            self.0.offset -= chunk_bitmaps.text.len();
            if self.0.offset <= *self.0.chunks.start() {
                self.0.chunks.prev();
            }
        } else {
            self.0.offset += chunk_bitmaps.text.len();
            if self.0.offset >= self.0.chunks.end() {
                self.0.chunks.next();
            }
        }

        Some(chunk_bitmaps)
    }
}

impl<'a> Iterator for Chunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.peek()?;
        if self.reversed {
            self.offset -= chunk.len();
            if self.offset <= *self.chunks.start() {
                self.chunks.prev();
            }
        } else {
            self.offset += chunk.len();
            if self.offset >= self.chunks.end() {
                self.chunks.next();
            }
        }

        Some(chunk)
    }
}

pub struct Bytes<'a> {
    chunks: sum_tree::Cursor<'a, 'static, Chunk, usize>,
    range: Range<usize>,
    reversed: bool,
}

impl<'a> Bytes<'a> {
    pub fn new(rope: &'a Rope, range: Range<usize>, reversed: bool) -> Self {
        let mut chunks = rope.chunks.cursor(());
        if reversed {
            chunks.seek(&range.end, Bias::Left);
        } else {
            chunks.seek(&range.start, Bias::Right);
        }
        Self {
            chunks,
            range,
            reversed,
        }
    }

    pub fn peek(&self) -> Option<&'a [u8]> {
        let chunk = self.chunks.item()?;
        if self.reversed && self.range.start >= self.chunks.end() {
            return None;
        }
        let chunk_start = *self.chunks.start();
        if self.range.end <= chunk_start {
            return None;
        }
        let start = self.range.start.saturating_sub(chunk_start);
        let end = self.range.end - chunk_start;
        Some(&chunk.text.as_bytes()[start..chunk.text.len().min(end)])
    }
}

impl<'a> Iterator for Bytes<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();
        if result.is_some() {
            if self.reversed {
                self.chunks.prev();
            } else {
                self.chunks.next();
            }
        }
        result
    }
}

impl io::Read for Bytes<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(chunk) = self.peek() {
            let len = cmp::min(buf.len(), chunk.len());
            if self.reversed {
                buf[..len].copy_from_slice(&chunk[chunk.len() - len..]);
                buf[..len].reverse();
                self.range.end -= len;
            } else {
                buf[..len].copy_from_slice(&chunk[..len]);
                self.range.start += len;
            }

            if len == chunk.len() {
                if self.reversed {
                    self.chunks.prev();
                } else {
                    self.chunks.next();
                }
            }
            Ok(len)
        } else {
            Ok(0)
        }
    }
}

pub struct Lines<'a> {
    chunks: Chunks<'a>,
    current_line: String,
    done: bool,
    reversed: bool,
}

impl<'a> Lines<'a> {
    pub fn next(&mut self) -> Option<&str> {
        if self.done {
            return None;
        }

        self.current_line.clear();

        while let Some(chunk) = self.chunks.peek() {
            let chunk_lines = chunk.split('\n');
            if self.reversed {
                let mut chunk_lines = chunk_lines.rev().peekable();
                if let Some(chunk_line) = chunk_lines.next() {
                    let done = chunk_lines.peek().is_some();
                    if done {
                        self.chunks
                            .seek(self.chunks.offset() - chunk_line.len() - "\n".len());
                        if self.current_line.is_empty() {
                            return Some(chunk_line);
                        }
                    }
                    self.current_line.insert_str(0, chunk_line);
                    if done {
                        return Some(&self.current_line);
                    }
                }
            } else {
                let mut chunk_lines = chunk_lines.peekable();
                if let Some(chunk_line) = chunk_lines.next() {
                    let done = chunk_lines.peek().is_some();
                    if done {
                        self.chunks
                            .seek(self.chunks.offset() + chunk_line.len() + "\n".len());
                        if self.current_line.is_empty() {
                            return Some(chunk_line);
                        }
                    }
                    self.current_line.push_str(chunk_line);
                    if done {
                        return Some(&self.current_line);
                    }
                }
            }

            self.chunks.next();
        }

        self.done = true;
        Some(&self.current_line)
    }

    pub fn seek(&mut self, offset: usize) {
        self.chunks.seek(offset);
        self.current_line.clear();
        self.done = false;
    }

    pub fn offset(&self) -> usize {
        self.chunks.offset()
    }
}

impl sum_tree::Item for Chunk {
    type Summary = ChunkSummary;

    fn summary(&self, _cx: ()) -> Self::Summary {
        ChunkSummary {
            text: self.as_slice().text_summary(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ChunkSummary {
    text: TextSummary,
}

impl sum_tree::ContextLessSummary for ChunkSummary {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        self.text += &summary.text;
    }
}

/// Summary of a string of text.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TextSummary {
    /// Length in bytes.
    pub len: usize,
    /// Length in UTF-8.
    pub chars: usize,
    /// Length in UTF-16 code units
    pub len_utf16: OffsetUtf16,
    /// A point representing the number of lines and the length of the last line.
    ///
    /// In other words, it marks the point after the last byte in the text, (if
    /// EOF was a character, this would be its position).
    pub lines: Point,
    /// How many `char`s are in the first line
    pub first_line_chars: u32,
    /// How many `char`s are in the last line
    pub last_line_chars: u32,
    /// How many UTF-16 code units are in the last line
    pub last_line_len_utf16: u32,
    /// The row idx of the longest row
    pub longest_row: u32,
    /// How many `char`s are in the longest row
    pub longest_row_chars: u32,
}

impl TextSummary {
    pub fn lines_utf16(&self) -> PointUtf16 {
        PointUtf16 {
            row: self.lines.row,
            column: self.last_line_len_utf16,
        }
    }

    pub fn newline() -> Self {
        Self {
            len: 1,
            chars: 1,
            len_utf16: OffsetUtf16(1),
            first_line_chars: 0,
            last_line_chars: 0,
            last_line_len_utf16: 0,
            lines: Point::new(1, 0),
            longest_row: 0,
            longest_row_chars: 0,
        }
    }

    pub fn add_newline(&mut self) {
        self.len += 1;
        self.len_utf16 += OffsetUtf16(self.len_utf16.0 + 1);
        self.last_line_chars = 0;
        self.last_line_len_utf16 = 0;
        self.lines += Point::new(1, 0);
    }
}

impl<'a> From<&'a str> for TextSummary {
    fn from(text: &'a str) -> Self {
        let mut len_utf16 = OffsetUtf16(0);
        let mut lines = Point::new(0, 0);
        let mut first_line_chars = 0;
        let mut last_line_chars = 0;
        let mut last_line_len_utf16 = 0;
        let mut longest_row = 0;
        let mut longest_row_chars = 0;
        let mut chars = 0;
        for c in text.chars() {
            chars += 1;
            len_utf16.0 += c.len_utf16();

            if c == '\n' {
                lines += Point::new(1, 0);
                last_line_len_utf16 = 0;
                last_line_chars = 0;
            } else {
                lines.column += c.len_utf8() as u32;
                last_line_len_utf16 += c.len_utf16() as u32;
                last_line_chars += 1;
            }

            if lines.row == 0 {
                first_line_chars = last_line_chars;
            }

            if last_line_chars > longest_row_chars {
                longest_row = lines.row;
                longest_row_chars = last_line_chars;
            }
        }

        TextSummary {
            len: text.len(),
            chars,
            len_utf16,
            lines,
            first_line_chars,
            last_line_chars,
            last_line_len_utf16,
            longest_row,
            longest_row_chars,
        }
    }
}

impl sum_tree::ContextLessSummary for TextSummary {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        *self += summary;
    }
}

impl ops::Add<Self> for TextSummary {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        AddAssign::add_assign(&mut self, &rhs);
        self
    }
}

impl<'a> ops::AddAssign<&'a Self> for TextSummary {
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
            self.last_line_len_utf16 += other.last_line_len_utf16;
        } else {
            self.last_line_chars = other.last_line_chars;
            self.last_line_len_utf16 = other.last_line_len_utf16;
        }

        self.chars += other.chars;
        self.len += other.len;
        self.len_utf16 += other.len_utf16;
        self.lines += other.lines;
    }
}

impl ops::AddAssign<Self> for TextSummary {
    fn add_assign(&mut self, other: Self) {
        *self += &other;
    }
}

pub trait TextDimension:
    'static + Clone + Copy + Default + for<'a> Dimension<'a, ChunkSummary> + std::fmt::Debug
{
    fn from_text_summary(summary: &TextSummary) -> Self;
    fn from_chunk(chunk: ChunkSlice) -> Self;
    fn add_assign(&mut self, other: &Self);
}

impl<D1: TextDimension, D2: TextDimension> TextDimension for Dimensions<D1, D2, ()> {
    fn from_text_summary(summary: &TextSummary) -> Self {
        Dimensions(
            D1::from_text_summary(summary),
            D2::from_text_summary(summary),
            (),
        )
    }

    fn from_chunk(chunk: ChunkSlice) -> Self {
        Dimensions(D1::from_chunk(chunk), D2::from_chunk(chunk), ())
    }

    fn add_assign(&mut self, other: &Self) {
        self.0.add_assign(&other.0);
        self.1.add_assign(&other.1);
    }
}

impl<'a> sum_tree::Dimension<'a, ChunkSummary> for TextSummary {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ChunkSummary, _: ()) {
        *self += &summary.text;
    }
}

impl TextDimension for TextSummary {
    fn from_text_summary(summary: &TextSummary) -> Self {
        *summary
    }

    fn from_chunk(chunk: ChunkSlice) -> Self {
        chunk.text_summary()
    }

    fn add_assign(&mut self, other: &Self) {
        *self += other;
    }
}

impl<'a> sum_tree::Dimension<'a, ChunkSummary> for usize {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ChunkSummary, _: ()) {
        *self += summary.text.len;
    }
}

impl TextDimension for usize {
    fn from_text_summary(summary: &TextSummary) -> Self {
        summary.len
    }

    fn from_chunk(chunk: ChunkSlice) -> Self {
        chunk.len()
    }

    fn add_assign(&mut self, other: &Self) {
        *self += other;
    }
}

impl<'a> sum_tree::Dimension<'a, ChunkSummary> for OffsetUtf16 {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ChunkSummary, _: ()) {
        *self += summary.text.len_utf16;
    }
}

impl TextDimension for OffsetUtf16 {
    fn from_text_summary(summary: &TextSummary) -> Self {
        summary.len_utf16
    }

    fn from_chunk(chunk: ChunkSlice) -> Self {
        chunk.len_utf16()
    }

    fn add_assign(&mut self, other: &Self) {
        *self += other;
    }
}

impl<'a> sum_tree::Dimension<'a, ChunkSummary> for Point {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ChunkSummary, _: ()) {
        *self += summary.text.lines;
    }
}

impl TextDimension for Point {
    fn from_text_summary(summary: &TextSummary) -> Self {
        summary.lines
    }

    fn from_chunk(chunk: ChunkSlice) -> Self {
        chunk.lines()
    }

    fn add_assign(&mut self, other: &Self) {
        *self += other;
    }
}

impl<'a> sum_tree::Dimension<'a, ChunkSummary> for PointUtf16 {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ChunkSummary, _: ()) {
        *self += summary.text.lines_utf16();
    }
}

impl TextDimension for PointUtf16 {
    fn from_text_summary(summary: &TextSummary) -> Self {
        summary.lines_utf16()
    }

    fn from_chunk(chunk: ChunkSlice) -> Self {
        PointUtf16 {
            row: chunk.lines().row,
            column: chunk.last_line_len_utf16(),
        }
    }

    fn add_assign(&mut self, other: &Self) {
        *self += other;
    }
}

/// A pair of text dimensions in which only the first dimension is used for comparison,
/// but both dimensions are updated during addition and subtraction.
#[derive(Clone, Copy, Debug)]
pub struct DimensionPair<K, V> {
    pub key: K,
    pub value: Option<V>,
}

impl<K: Default, V: Default> Default for DimensionPair<K, V> {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Some(Default::default()),
        }
    }
}

impl<K, V> cmp::Ord for DimensionPair<K, V>
where
    K: cmp::Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K, V> cmp::PartialOrd for DimensionPair<K, V>
where
    K: cmp::PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K, V> cmp::PartialEq for DimensionPair<K, V>
where
    K: cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K, V> ops::Sub for DimensionPair<K, V>
where
    K: ops::Sub<K, Output = K>,
    V: ops::Sub<V, Output = V>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            key: self.key - rhs.key,
            value: self.value.zip(rhs.value).map(|(a, b)| a - b),
        }
    }
}

impl<K, V> cmp::Eq for DimensionPair<K, V> where K: cmp::Eq {}

impl<'a, K, V> sum_tree::Dimension<'a, ChunkSummary> for DimensionPair<K, V>
where
    K: sum_tree::Dimension<'a, ChunkSummary>,
    V: sum_tree::Dimension<'a, ChunkSummary>,
{
    fn zero(_cx: ()) -> Self {
        Self {
            key: K::zero(_cx),
            value: Some(V::zero(_cx)),
        }
    }

    fn add_summary(&mut self, summary: &'a ChunkSummary, _cx: ()) {
        self.key.add_summary(summary, _cx);
        if let Some(value) = &mut self.value {
            value.add_summary(summary, _cx);
        }
    }
}

impl<K, V> TextDimension for DimensionPair<K, V>
where
    K: TextDimension,
    V: TextDimension,
{
    fn add_assign(&mut self, other: &Self) {
        self.key.add_assign(&other.key);
        if let Some(value) = &mut self.value {
            if let Some(other_value) = other.value.as_ref() {
                value.add_assign(other_value);
            } else {
                self.value.take();
            }
        }
    }

    fn from_chunk(chunk: ChunkSlice) -> Self {
        Self {
            key: K::from_chunk(chunk),
            value: Some(V::from_chunk(chunk)),
        }
    }

    fn from_text_summary(summary: &TextSummary) -> Self {
        Self {
            key: K::from_text_summary(summary),
            value: Some(V::from_text_summary(summary)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Bias::{Left, Right};
    use rand::prelude::*;
    use std::{cmp::Ordering, env, io::Read};
    use util::RandomCharIter;

    #[ctor::ctor]
    fn init_logger() {
        zlog::init_test();
    }

    #[test]
    fn test_all_4_byte_chars() {
        let mut rope = Rope::new();
        let text = "🏀".repeat(256);
        rope.push(&text);
        assert_eq!(rope.text(), text);
    }

    #[test]
    fn test_clip() {
        let rope = Rope::from("🧘");

        assert_eq!(rope.clip_offset(1, Bias::Left), 0);
        assert_eq!(rope.clip_offset(1, Bias::Right), 4);
        assert_eq!(rope.clip_offset(5, Bias::Right), 4);

        assert_eq!(
            rope.clip_point(Point::new(0, 1), Bias::Left),
            Point::new(0, 0)
        );
        assert_eq!(
            rope.clip_point(Point::new(0, 1), Bias::Right),
            Point::new(0, 4)
        );
        assert_eq!(
            rope.clip_point(Point::new(0, 5), Bias::Right),
            Point::new(0, 4)
        );

        assert_eq!(
            rope.clip_point_utf16(Unclipped(PointUtf16::new(0, 1)), Bias::Left),
            PointUtf16::new(0, 0)
        );
        assert_eq!(
            rope.clip_point_utf16(Unclipped(PointUtf16::new(0, 1)), Bias::Right),
            PointUtf16::new(0, 2)
        );
        assert_eq!(
            rope.clip_point_utf16(Unclipped(PointUtf16::new(0, 3)), Bias::Right),
            PointUtf16::new(0, 2)
        );

        assert_eq!(
            rope.clip_offset_utf16(OffsetUtf16(1), Bias::Left),
            OffsetUtf16(0)
        );
        assert_eq!(
            rope.clip_offset_utf16(OffsetUtf16(1), Bias::Right),
            OffsetUtf16(2)
        );
        assert_eq!(
            rope.clip_offset_utf16(OffsetUtf16(3), Bias::Right),
            OffsetUtf16(2)
        );
    }

    #[test]
    fn test_prev_next_line() {
        let rope = Rope::from("abc\ndef\nghi\njkl");

        let mut chunks = rope.chunks();
        assert_eq!(chunks.peek().unwrap().chars().next().unwrap(), 'a');

        assert!(chunks.next_line());
        assert_eq!(chunks.peek().unwrap().chars().next().unwrap(), 'd');

        assert!(chunks.next_line());
        assert_eq!(chunks.peek().unwrap().chars().next().unwrap(), 'g');

        assert!(chunks.next_line());
        assert_eq!(chunks.peek().unwrap().chars().next().unwrap(), 'j');

        assert!(!chunks.next_line());
        assert_eq!(chunks.peek(), None);

        assert!(chunks.prev_line());
        assert_eq!(chunks.peek().unwrap().chars().next().unwrap(), 'j');

        assert!(chunks.prev_line());
        assert_eq!(chunks.peek().unwrap().chars().next().unwrap(), 'g');

        assert!(chunks.prev_line());
        assert_eq!(chunks.peek().unwrap().chars().next().unwrap(), 'd');

        assert!(chunks.prev_line());
        assert_eq!(chunks.peek().unwrap().chars().next().unwrap(), 'a');

        assert!(!chunks.prev_line());
        assert_eq!(chunks.peek().unwrap().chars().next().unwrap(), 'a');

        // Only return true when the cursor has moved to the start of a line
        let mut chunks = rope.chunks_in_range(5..7);
        chunks.seek(6);
        assert!(!chunks.prev_line());
        assert_eq!(chunks.peek().unwrap().chars().next().unwrap(), 'e');

        assert!(!chunks.next_line());
        assert_eq!(chunks.peek(), None);
    }

    #[test]
    fn test_lines() {
        let rope = Rope::from("abc\ndefg\nhi");
        let mut lines = rope.chunks().lines();
        assert_eq!(lines.next(), Some("abc"));
        assert_eq!(lines.next(), Some("defg"));
        assert_eq!(lines.next(), Some("hi"));
        assert_eq!(lines.next(), None);

        let rope = Rope::from("abc\ndefg\nhi\n");
        let mut lines = rope.chunks().lines();
        assert_eq!(lines.next(), Some("abc"));
        assert_eq!(lines.next(), Some("defg"));
        assert_eq!(lines.next(), Some("hi"));
        assert_eq!(lines.next(), Some(""));
        assert_eq!(lines.next(), None);

        let rope = Rope::from("abc\ndefg\nhi");
        let mut lines = rope.reversed_chunks_in_range(0..rope.len()).lines();
        assert_eq!(lines.next(), Some("hi"));
        assert_eq!(lines.next(), Some("defg"));
        assert_eq!(lines.next(), Some("abc"));
        assert_eq!(lines.next(), None);

        let rope = Rope::from("abc\ndefg\nhi\n");
        let mut lines = rope.reversed_chunks_in_range(0..rope.len()).lines();
        assert_eq!(lines.next(), Some(""));
        assert_eq!(lines.next(), Some("hi"));
        assert_eq!(lines.next(), Some("defg"));
        assert_eq!(lines.next(), Some("abc"));
        assert_eq!(lines.next(), None);

        let rope = Rope::from("abc\nlonger line test\nhi");
        let mut lines = rope.chunks().lines();
        assert_eq!(lines.next(), Some("abc"));
        assert_eq!(lines.next(), Some("longer line test"));
        assert_eq!(lines.next(), Some("hi"));
        assert_eq!(lines.next(), None);

        let rope = Rope::from("abc\nlonger line test\nhi");
        let mut lines = rope.reversed_chunks_in_range(0..rope.len()).lines();
        assert_eq!(lines.next(), Some("hi"));
        assert_eq!(lines.next(), Some("longer line test"));
        assert_eq!(lines.next(), Some("abc"));
        assert_eq!(lines.next(), None);
    }

    #[gpui::test(iterations = 100)]
    fn test_random_rope(mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let mut expected = String::new();
        let mut actual = Rope::new();
        for _ in 0..operations {
            let end_ix = clip_offset(&expected, rng.random_range(0..=expected.len()), Right);
            let start_ix = clip_offset(&expected, rng.random_range(0..=end_ix), Left);
            let len = rng.random_range(0..=64);
            let new_text: String = RandomCharIter::new(&mut rng).take(len).collect();

            let mut new_actual = Rope::new();
            let mut cursor = actual.cursor(0);
            new_actual.append(cursor.slice(start_ix));
            new_actual.push(&new_text);
            cursor.seek_forward(end_ix);
            new_actual.append(cursor.suffix());
            actual = new_actual;

            expected.replace_range(start_ix..end_ix, &new_text);

            assert_eq!(actual.text(), expected);
            log::info!("text: {:?}", expected);

            for _ in 0..5 {
                let end_ix = clip_offset(&expected, rng.random_range(0..=expected.len()), Right);
                let start_ix = clip_offset(&expected, rng.random_range(0..=end_ix), Left);

                let actual_text = actual.chunks_in_range(start_ix..end_ix).collect::<String>();
                assert_eq!(actual_text, &expected[start_ix..end_ix]);

                let mut actual_text = String::new();
                actual
                    .bytes_in_range(start_ix..end_ix)
                    .read_to_string(&mut actual_text)
                    .unwrap();
                assert_eq!(actual_text, &expected[start_ix..end_ix]);

                assert_eq!(
                    actual
                        .reversed_chunks_in_range(start_ix..end_ix)
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .rev()
                        .collect::<String>(),
                    &expected[start_ix..end_ix]
                );

                let mut expected_line_starts: Vec<_> = expected[start_ix..end_ix]
                    .match_indices('\n')
                    .map(|(index, _)| start_ix + index + 1)
                    .collect();

                let mut chunks = actual.chunks_in_range(start_ix..end_ix);

                let mut actual_line_starts = Vec::new();
                while chunks.next_line() {
                    actual_line_starts.push(chunks.offset());
                }
                assert_eq!(
                    actual_line_starts,
                    expected_line_starts,
                    "actual line starts != expected line starts when using next_line() for {:?} ({:?})",
                    &expected[start_ix..end_ix],
                    start_ix..end_ix
                );

                if start_ix < end_ix
                    && (start_ix == 0 || expected.as_bytes()[start_ix - 1] == b'\n')
                {
                    expected_line_starts.insert(0, start_ix);
                }
                // Remove the last index if it starts at the end of the range.
                if expected_line_starts.last() == Some(&end_ix) {
                    expected_line_starts.pop();
                }

                let mut actual_line_starts = Vec::new();
                while chunks.prev_line() {
                    actual_line_starts.push(chunks.offset());
                }
                actual_line_starts.reverse();
                assert_eq!(
                    actual_line_starts,
                    expected_line_starts,
                    "actual line starts != expected line starts when using prev_line() for {:?} ({:?})",
                    &expected[start_ix..end_ix],
                    start_ix..end_ix
                );

                // Check that next_line/prev_line work correctly from random positions
                let mut offset = rng.random_range(start_ix..=end_ix);
                while !expected.is_char_boundary(offset) {
                    offset -= 1;
                }
                chunks.seek(offset);

                for _ in 0..5 {
                    if rng.random() {
                        let expected_next_line_start = expected[offset..end_ix]
                            .find('\n')
                            .map(|newline_ix| offset + newline_ix + 1);

                        let moved = chunks.next_line();
                        assert_eq!(
                            moved,
                            expected_next_line_start.is_some(),
                            "unexpected result from next_line after seeking to {} in range {:?} ({:?})",
                            offset,
                            start_ix..end_ix,
                            &expected[start_ix..end_ix]
                        );
                        if let Some(expected_next_line_start) = expected_next_line_start {
                            assert_eq!(
                                chunks.offset(),
                                expected_next_line_start,
                                "invalid position after seeking to {} in range {:?} ({:?})",
                                offset,
                                start_ix..end_ix,
                                &expected[start_ix..end_ix]
                            );
                        } else {
                            assert_eq!(
                                chunks.offset(),
                                end_ix,
                                "invalid position after seeking to {} in range {:?} ({:?})",
                                offset,
                                start_ix..end_ix,
                                &expected[start_ix..end_ix]
                            );
                        }
                    } else {
                        let search_end = if offset > 0 && expected.as_bytes()[offset - 1] == b'\n' {
                            offset - 1
                        } else {
                            offset
                        };

                        let expected_prev_line_start = expected[..search_end]
                            .rfind('\n')
                            .and_then(|newline_ix| {
                                let line_start_ix = newline_ix + 1;
                                if line_start_ix >= start_ix {
                                    Some(line_start_ix)
                                } else {
                                    None
                                }
                            })
                            .or({
                                if offset > 0 && start_ix == 0 {
                                    Some(0)
                                } else {
                                    None
                                }
                            });

                        let moved = chunks.prev_line();
                        assert_eq!(
                            moved,
                            expected_prev_line_start.is_some(),
                            "unexpected result from prev_line after seeking to {} in range {:?} ({:?})",
                            offset,
                            start_ix..end_ix,
                            &expected[start_ix..end_ix]
                        );
                        if let Some(expected_prev_line_start) = expected_prev_line_start {
                            assert_eq!(
                                chunks.offset(),
                                expected_prev_line_start,
                                "invalid position after seeking to {} in range {:?} ({:?})",
                                offset,
                                start_ix..end_ix,
                                &expected[start_ix..end_ix]
                            );
                        } else {
                            assert_eq!(
                                chunks.offset(),
                                start_ix,
                                "invalid position after seeking to {} in range {:?} ({:?})",
                                offset,
                                start_ix..end_ix,
                                &expected[start_ix..end_ix]
                            );
                        }
                    }

                    assert!((start_ix..=end_ix).contains(&chunks.offset()));
                    if rng.random() {
                        offset = rng.random_range(start_ix..=end_ix);
                        while !expected.is_char_boundary(offset) {
                            offset -= 1;
                        }
                        chunks.seek(offset);
                    } else {
                        chunks.next();
                        offset = chunks.offset();
                        assert!((start_ix..=end_ix).contains(&chunks.offset()));
                    }
                }
            }

            let mut offset_utf16 = OffsetUtf16(0);
            let mut point = Point::new(0, 0);
            let mut point_utf16 = PointUtf16::new(0, 0);
            for (ix, ch) in expected.char_indices().chain(Some((expected.len(), '\0'))) {
                assert_eq!(actual.offset_to_point(ix), point, "offset_to_point({})", ix);
                assert_eq!(
                    actual.offset_to_point_utf16(ix),
                    point_utf16,
                    "offset_to_point_utf16({})",
                    ix
                );
                assert_eq!(
                    actual.point_to_offset(point),
                    ix,
                    "point_to_offset({:?})",
                    point
                );
                assert_eq!(
                    actual.point_utf16_to_offset(point_utf16),
                    ix,
                    "point_utf16_to_offset({:?})",
                    point_utf16
                );
                assert_eq!(
                    actual.offset_to_offset_utf16(ix),
                    offset_utf16,
                    "offset_to_offset_utf16({:?})",
                    ix
                );
                assert_eq!(
                    actual.offset_utf16_to_offset(offset_utf16),
                    ix,
                    "offset_utf16_to_offset({:?})",
                    offset_utf16
                );
                if ch == '\n' {
                    point += Point::new(1, 0);
                    point_utf16 += PointUtf16::new(1, 0);
                } else {
                    point.column += ch.len_utf8() as u32;
                    point_utf16.column += ch.len_utf16() as u32;
                }
                offset_utf16.0 += ch.len_utf16();
            }

            let mut offset_utf16 = OffsetUtf16(0);
            let mut point_utf16 = Unclipped(PointUtf16::zero());
            for unit in expected.encode_utf16() {
                let left_offset = actual.clip_offset_utf16(offset_utf16, Bias::Left);
                let right_offset = actual.clip_offset_utf16(offset_utf16, Bias::Right);
                assert!(right_offset >= left_offset);
                // Ensure translating UTF-16 offsets to UTF-8 offsets doesn't panic.
                actual.offset_utf16_to_offset(left_offset);
                actual.offset_utf16_to_offset(right_offset);

                let left_point = actual.clip_point_utf16(point_utf16, Bias::Left);
                let right_point = actual.clip_point_utf16(point_utf16, Bias::Right);
                assert!(right_point >= left_point);
                // Ensure translating valid UTF-16 points to offsets doesn't panic.
                actual.point_utf16_to_offset(left_point);
                actual.point_utf16_to_offset(right_point);

                offset_utf16.0 += 1;
                if unit == b'\n' as u16 {
                    point_utf16.0 += PointUtf16::new(1, 0);
                } else {
                    point_utf16.0 += PointUtf16::new(0, 1);
                }
            }

            for _ in 0..5 {
                let end_ix = clip_offset(&expected, rng.random_range(0..=expected.len()), Right);
                let start_ix = clip_offset(&expected, rng.random_range(0..=end_ix), Left);
                assert_eq!(
                    actual.cursor(start_ix).summary::<TextSummary>(end_ix),
                    TextSummary::from(&expected[start_ix..end_ix])
                );
            }

            let mut expected_longest_rows = Vec::new();
            let mut longest_line_len = -1_isize;
            for (row, line) in expected.split('\n').enumerate() {
                let row = row as u32;
                assert_eq!(
                    actual.line_len(row),
                    line.len() as u32,
                    "invalid line len for row {}",
                    row
                );

                let line_char_count = line.chars().count() as isize;
                match line_char_count.cmp(&longest_line_len) {
                    Ordering::Less => {}
                    Ordering::Equal => expected_longest_rows.push(row),
                    Ordering::Greater => {
                        longest_line_len = line_char_count;
                        expected_longest_rows.clear();
                        expected_longest_rows.push(row);
                    }
                }
            }

            let longest_row = actual.summary().longest_row;
            assert!(
                expected_longest_rows.contains(&longest_row),
                "incorrect longest row {}. expected {:?} with length {}",
                longest_row,
                expected_longest_rows,
                longest_line_len,
            );
        }
    }

    #[test]
    fn test_chunks_equals_str() {
        let text = "This is a multi-chunk\n& multi-line test string!";
        let rope = Rope::from(text);
        for start in 0..text.len() {
            for end in start..text.len() {
                let range = start..end;
                let correct_substring = &text[start..end];

                // Test that correct range returns true
                assert!(
                    rope.chunks_in_range(range.clone())
                        .equals_str(correct_substring)
                );
                assert!(
                    rope.reversed_chunks_in_range(range.clone())
                        .equals_str(correct_substring)
                );

                // Test that all other ranges return false (unless they happen to match)
                for other_start in 0..text.len() {
                    for other_end in other_start..text.len() {
                        if other_start == start && other_end == end {
                            continue;
                        }
                        let other_substring = &text[other_start..other_end];

                        // Only assert false if the substrings are actually different
                        if other_substring == correct_substring {
                            continue;
                        }
                        assert!(
                            !rope
                                .chunks_in_range(range.clone())
                                .equals_str(other_substring)
                        );
                        assert!(
                            !rope
                                .reversed_chunks_in_range(range.clone())
                                .equals_str(other_substring)
                        );
                    }
                }
            }
        }

        let rope = Rope::from("");
        assert!(rope.chunks_in_range(0..0).equals_str(""));
        assert!(rope.reversed_chunks_in_range(0..0).equals_str(""));
        assert!(!rope.chunks_in_range(0..0).equals_str("foo"));
        assert!(!rope.reversed_chunks_in_range(0..0).equals_str("foo"));
    }

    #[test]
    fn test_is_char_boundary() {
        let fixture = "地";
        let rope = Rope::from("地");
        for b in 0..=fixture.len() {
            assert_eq!(rope.is_char_boundary(b), fixture.is_char_boundary(b));
        }
        let fixture = "";
        let rope = Rope::from("");
        for b in 0..=fixture.len() {
            assert_eq!(rope.is_char_boundary(b), fixture.is_char_boundary(b));
        }
        let fixture = "🔴🟠🟡🟢🔵🟣⚫️⚪️🟤\n🏳️‍⚧️🏁🏳️‍🌈🏴‍☠️⛳️📬📭🏴🏳️🚩";
        let rope = Rope::from("🔴🟠🟡🟢🔵🟣⚫️⚪️🟤\n🏳️‍⚧️🏁🏳️‍🌈🏴‍☠️⛳️📬📭🏴🏳️🚩");
        for b in 0..=fixture.len() {
            assert_eq!(rope.is_char_boundary(b), fixture.is_char_boundary(b));
        }
    }

    #[test]
    fn test_floor_char_boundary() {
        // polyfill of str::floor_char_boundary
        fn floor_char_boundary(str: &str, index: usize) -> usize {
            if index >= str.len() {
                str.len()
            } else {
                let lower_bound = index.saturating_sub(3);
                let new_index = str.as_bytes()[lower_bound..=index]
                    .iter()
                    .rposition(|b| (*b as i8) >= -0x40);

                lower_bound + new_index.unwrap()
            }
        }

        let fixture = "地";
        let rope = Rope::from("地");
        for b in 0..=fixture.len() {
            assert_eq!(
                rope.floor_char_boundary(b),
                floor_char_boundary(&fixture, b)
            );
        }

        let fixture = "";
        let rope = Rope::from("");
        for b in 0..=fixture.len() {
            assert_eq!(
                rope.floor_char_boundary(b),
                floor_char_boundary(&fixture, b)
            );
        }

        let fixture = "🔴🟠🟡🟢🔵🟣⚫️⚪️🟤\n🏳️‍⚧️🏁🏳️‍🌈🏴‍☠️⛳️📬📭🏴🏳️🚩";
        let rope = Rope::from("🔴🟠🟡🟢🔵🟣⚫️⚪️🟤\n🏳️‍⚧️🏁🏳️‍🌈🏴‍☠️⛳️📬📭🏴🏳️🚩");
        for b in 0..=fixture.len() {
            assert_eq!(
                rope.floor_char_boundary(b),
                floor_char_boundary(&fixture, b)
            );
        }
    }

    #[test]
    fn test_ceil_char_boundary() {
        // polyfill of str::ceil_char_boundary
        fn ceil_char_boundary(str: &str, index: usize) -> usize {
            if index > str.len() {
                str.len()
            } else {
                let upper_bound = Ord::min(index + 4, str.len());
                str.as_bytes()[index..upper_bound]
                    .iter()
                    .position(|b| (*b as i8) >= -0x40)
                    .map_or(upper_bound, |pos| pos + index)
            }
        }

        let fixture = "地";
        let rope = Rope::from("地");
        for b in 0..=fixture.len() {
            assert_eq!(rope.ceil_char_boundary(b), ceil_char_boundary(&fixture, b));
        }

        let fixture = "";
        let rope = Rope::from("");
        for b in 0..=fixture.len() {
            assert_eq!(rope.ceil_char_boundary(b), ceil_char_boundary(&fixture, b));
        }

        let fixture = "🔴🟠🟡🟢🔵🟣⚫️⚪️🟤\n🏳️‍⚧️🏁🏳️‍🌈🏴‍☠️⛳️📬📭🏴🏳️🚩";
        let rope = Rope::from("🔴🟠🟡🟢🔵🟣⚫️⚪️🟤\n🏳️‍⚧️🏁🏳️‍🌈🏴‍☠️⛳️📬📭🏴🏳️🚩");
        for b in 0..=fixture.len() {
            assert_eq!(rope.ceil_char_boundary(b), ceil_char_boundary(&fixture, b));
        }
    }

    fn clip_offset(text: &str, mut offset: usize, bias: Bias) -> usize {
        while !text.is_char_boundary(offset) {
            match bias {
                Bias::Left => offset -= 1,
                Bias::Right => offset += 1,
            }
        }
        offset
    }

    impl Rope {
        fn text(&self) -> String {
            let mut text = String::new();
            for chunk in self.chunks.cursor::<()>(()) {
                text.push_str(&chunk.text);
            }
            text
        }
    }
}

use crate::PointUtf16;

use super::Point;
use arrayvec::ArrayString;
use smallvec::SmallVec;
use std::{cmp, fmt, io, mem, ops::Range, str};
use sum_tree::{Bias, Dimension, SumTree};

#[cfg(test)]
const CHUNK_BASE: usize = 6;

#[cfg(not(test))]
const CHUNK_BASE: usize = 16;

#[derive(Clone, Default, Debug)]
pub struct Rope {
    chunks: SumTree<Chunk>,
}

impl Rope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, rope: Rope) {
        let mut chunks = rope.chunks.cursor::<()>();
        chunks.next(&());
        if let Some(chunk) = chunks.item() {
            if self.chunks.last().map_or(false, |c| c.0.len() < CHUNK_BASE)
                || chunk.0.len() < CHUNK_BASE
            {
                self.push(&chunk.0);
                chunks.next(&());
            }
        }

        self.chunks.push_tree(chunks.suffix(&()), &());
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

    pub fn push(&mut self, text: &str) {
        let mut new_chunks = SmallVec::<[_; 16]>::new();
        let mut new_chunk = ArrayString::new();
        for ch in text.chars() {
            if new_chunk.len() + ch.len_utf8() > 2 * CHUNK_BASE {
                new_chunks.push(Chunk(new_chunk));
                new_chunk = ArrayString::new();
            }
            new_chunk.push(ch);
        }
        if !new_chunk.is_empty() {
            new_chunks.push(Chunk(new_chunk));
        }

        let mut new_chunks = new_chunks.into_iter();
        let mut first_new_chunk = new_chunks.next();
        self.chunks.update_last(
            |last_chunk| {
                if let Some(first_new_chunk_ref) = first_new_chunk.as_mut() {
                    if last_chunk.0.len() + first_new_chunk_ref.0.len() <= 2 * CHUNK_BASE {
                        last_chunk.0.push_str(&first_new_chunk.take().unwrap().0);
                    } else {
                        let mut text = ArrayString::<{ 4 * CHUNK_BASE }>::new();
                        text.push_str(&last_chunk.0);
                        text.push_str(&first_new_chunk_ref.0);
                        let (left, right) = text.split_at(find_split_ix(&text));
                        last_chunk.0.clear();
                        last_chunk.0.push_str(left);
                        first_new_chunk_ref.0.clear();
                        first_new_chunk_ref.0.push_str(right);
                    }
                }
            },
            &(),
        );

        self.chunks
            .extend(first_new_chunk.into_iter().chain(new_chunks), &());
        self.check_invariants();
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
            let mut chunks = self.chunks.cursor::<()>().peekable();
            while let Some(chunk) = chunks.next() {
                if chunks.peek().is_some() {
                    assert!(chunk.0.len() + 3 >= CHUNK_BASE);
                }
            }
        }
    }

    pub fn summary(&self) -> TextSummary {
        self.chunks.summary()
    }

    pub fn len(&self) -> usize {
        self.chunks.extent(&())
    }

    pub fn max_point(&self) -> Point {
        self.chunks.extent(&())
    }

    pub fn cursor(&self, offset: usize) -> Cursor {
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

    pub fn bytes_in_range(&self, range: Range<usize>) -> Bytes {
        Bytes::new(self, range)
    }

    pub fn chunks<'a>(&'a self) -> Chunks<'a> {
        self.chunks_in_range(0..self.len())
    }

    pub fn chunks_in_range(&self, range: Range<usize>) -> Chunks {
        Chunks::new(self, range, false)
    }

    pub fn reversed_chunks_in_range(&self, range: Range<usize>) -> Chunks {
        Chunks::new(self, range, true)
    }

    pub fn offset_to_point(&self, offset: usize) -> Point {
        if offset >= self.summary().bytes {
            return self.summary().lines;
        }
        let mut cursor = self.chunks.cursor::<(usize, Point)>();
        cursor.seek(&offset, Bias::Left, &());
        let overshoot = offset - cursor.start().0;
        cursor.start().1
            + cursor
                .item()
                .map_or(Point::zero(), |chunk| chunk.offset_to_point(overshoot))
    }

    pub fn offset_to_point_utf16(&self, offset: usize) -> PointUtf16 {
        if offset >= self.summary().bytes {
            return self.summary().lines_utf16;
        }
        let mut cursor = self.chunks.cursor::<(usize, PointUtf16)>();
        cursor.seek(&offset, Bias::Left, &());
        let overshoot = offset - cursor.start().0;
        cursor.start().1
            + cursor.item().map_or(PointUtf16::zero(), |chunk| {
                chunk.offset_to_point_utf16(overshoot)
            })
    }

    pub fn point_to_point_utf16(&self, point: Point) -> PointUtf16 {
        if point >= self.summary().lines {
            return self.summary().lines_utf16;
        }
        let mut cursor = self.chunks.cursor::<(Point, PointUtf16)>();
        cursor.seek(&point, Bias::Left, &());
        let overshoot = point - cursor.start().0;
        cursor.start().1
            + cursor.item().map_or(PointUtf16::zero(), |chunk| {
                chunk.point_to_point_utf16(overshoot)
            })
    }

    pub fn point_to_offset(&self, point: Point) -> usize {
        if point >= self.summary().lines {
            return self.summary().bytes;
        }
        let mut cursor = self.chunks.cursor::<(Point, usize)>();
        cursor.seek(&point, Bias::Left, &());
        let overshoot = point - cursor.start().0;
        cursor.start().1
            + cursor
                .item()
                .map_or(0, |chunk| chunk.point_to_offset(overshoot))
    }

    pub fn point_utf16_to_offset(&self, point: PointUtf16) -> usize {
        if point >= self.summary().lines_utf16 {
            return self.summary().bytes;
        }
        let mut cursor = self.chunks.cursor::<(PointUtf16, usize)>();
        cursor.seek(&point, Bias::Left, &());
        let overshoot = point - cursor.start().0;
        cursor.start().1
            + cursor
                .item()
                .map_or(0, |chunk| chunk.point_utf16_to_offset(overshoot))
    }

    pub fn point_utf16_to_point(&self, point: PointUtf16) -> Point {
        if point >= self.summary().lines_utf16 {
            return self.summary().lines;
        }
        let mut cursor = self.chunks.cursor::<(PointUtf16, Point)>();
        cursor.seek(&point, Bias::Left, &());
        let overshoot = point - cursor.start().0;
        cursor.start().1
            + cursor
                .item()
                .map_or(Point::zero(), |chunk| chunk.point_utf16_to_point(overshoot))
    }

    pub fn clip_offset(&self, mut offset: usize, bias: Bias) -> usize {
        let mut cursor = self.chunks.cursor::<usize>();
        cursor.seek(&offset, Bias::Left, &());
        if let Some(chunk) = cursor.item() {
            let mut ix = offset - cursor.start();
            while !chunk.0.is_char_boundary(ix) {
                match bias {
                    Bias::Left => {
                        ix -= 1;
                        offset -= 1;
                    }
                    Bias::Right => {
                        ix += 1;
                        offset += 1;
                    }
                }
            }
            offset
        } else {
            self.summary().bytes
        }
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        let mut cursor = self.chunks.cursor::<Point>();
        cursor.seek(&point, Bias::Right, &());
        if let Some(chunk) = cursor.item() {
            let overshoot = point - cursor.start();
            *cursor.start() + chunk.clip_point(overshoot, bias)
        } else {
            self.summary().lines
        }
    }

    pub fn clip_point_utf16(&self, point: PointUtf16, bias: Bias) -> PointUtf16 {
        let mut cursor = self.chunks.cursor::<PointUtf16>();
        cursor.seek(&point, Bias::Right, &());
        if let Some(chunk) = cursor.item() {
            let overshoot = point - cursor.start();
            *cursor.start() + chunk.clip_point_utf16(overshoot, bias)
        } else {
            self.summary().lines_utf16
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

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for chunk in self.chunks() {
            write!(f, "{}", chunk)?;
        }
        Ok(())
    }
}

pub struct Cursor<'a> {
    rope: &'a Rope,
    chunks: sum_tree::Cursor<'a, Chunk, usize>,
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(rope: &'a Rope, offset: usize) -> Self {
        let mut chunks = rope.chunks.cursor();
        chunks.seek(&offset, Bias::Right, &());
        Self {
            rope,
            chunks,
            offset,
        }
    }

    pub fn seek_forward(&mut self, end_offset: usize) {
        debug_assert!(end_offset >= self.offset);

        self.chunks.seek_forward(&end_offset, Bias::Right, &());
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
            let end_ix = cmp::min(end_offset, self.chunks.end(&())) - self.chunks.start();
            slice.push(&start_chunk.0[start_ix..end_ix]);
        }

        if end_offset > self.chunks.end(&()) {
            self.chunks.next(&());
            slice.append(Rope {
                chunks: self.chunks.slice(&end_offset, Bias::Right, &()),
            });
            if let Some(end_chunk) = self.chunks.item() {
                let end_ix = end_offset - self.chunks.start();
                slice.push(&end_chunk.0[..end_ix]);
            }
        }

        self.offset = end_offset;
        slice
    }

    pub fn summary<D: TextDimension>(&mut self, end_offset: usize) -> D {
        debug_assert!(end_offset >= self.offset);

        let mut summary = D::default();
        if let Some(start_chunk) = self.chunks.item() {
            let start_ix = self.offset - self.chunks.start();
            let end_ix = cmp::min(end_offset, self.chunks.end(&())) - self.chunks.start();
            summary.add_assign(&D::from_text_summary(&TextSummary::from(
                &start_chunk.0[start_ix..end_ix],
            )));
        }

        if end_offset > self.chunks.end(&()) {
            self.chunks.next(&());
            summary.add_assign(&self.chunks.summary(&end_offset, Bias::Right, &()));
            if let Some(end_chunk) = self.chunks.item() {
                let end_ix = end_offset - self.chunks.start();
                summary.add_assign(&D::from_text_summary(&TextSummary::from(
                    &end_chunk.0[..end_ix],
                )));
            }
        }

        self.offset = end_offset;
        summary
    }

    pub fn suffix(mut self) -> Rope {
        self.slice(self.rope.chunks.extent(&()))
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

pub struct Chunks<'a> {
    chunks: sum_tree::Cursor<'a, Chunk, usize>,
    range: Range<usize>,
    reversed: bool,
}

impl<'a> Chunks<'a> {
    pub fn new(rope: &'a Rope, range: Range<usize>, reversed: bool) -> Self {
        let mut chunks = rope.chunks.cursor();
        if reversed {
            chunks.seek(&range.end, Bias::Left, &());
        } else {
            chunks.seek(&range.start, Bias::Right, &());
        }
        Self {
            chunks,
            range,
            reversed,
        }
    }

    pub fn offset(&self) -> usize {
        if self.reversed {
            self.range.end.min(self.chunks.end(&()))
        } else {
            self.range.start.max(*self.chunks.start())
        }
    }

    pub fn seek(&mut self, offset: usize) {
        let bias = if self.reversed {
            Bias::Left
        } else {
            Bias::Right
        };

        if offset >= self.chunks.end(&()) {
            self.chunks.seek_forward(&offset, bias, &());
        } else {
            self.chunks.seek(&offset, bias, &());
        }

        if self.reversed {
            self.range.end = offset;
        } else {
            self.range.start = offset;
        }
    }

    pub fn peek(&self) -> Option<&'a str> {
        let chunk = self.chunks.item()?;
        if self.reversed && self.range.start >= self.chunks.end(&()) {
            return None;
        }
        let chunk_start = *self.chunks.start();
        if self.range.end <= chunk_start {
            return None;
        }

        let start = self.range.start.saturating_sub(chunk_start);
        let end = self.range.end - chunk_start;
        Some(&chunk.0[start..chunk.0.len().min(end)])
    }
}

impl<'a> Iterator for Chunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();
        if result.is_some() {
            if self.reversed {
                self.chunks.prev(&());
            } else {
                self.chunks.next(&());
            }
        }
        result
    }
}

pub struct Bytes<'a> {
    chunks: sum_tree::Cursor<'a, Chunk, usize>,
    range: Range<usize>,
}

impl<'a> Bytes<'a> {
    pub fn new(rope: &'a Rope, range: Range<usize>) -> Self {
        let mut chunks = rope.chunks.cursor();
        chunks.seek(&range.start, Bias::Right, &());
        Self { chunks, range }
    }

    pub fn peek(&self) -> Option<&'a [u8]> {
        let chunk = self.chunks.item()?;
        let chunk_start = *self.chunks.start();
        if self.range.end <= chunk_start {
            return None;
        }

        let start = self.range.start.saturating_sub(chunk_start);
        let end = self.range.end - chunk_start;
        Some(&chunk.0.as_bytes()[start..chunk.0.len().min(end)])
    }
}

impl<'a> Iterator for Bytes<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();
        if result.is_some() {
            self.chunks.next(&());
        }
        result
    }
}

impl<'a> io::Read for Bytes<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(chunk) = self.peek() {
            let len = cmp::min(buf.len(), chunk.len());
            buf[..len].copy_from_slice(&chunk[..len]);
            self.range.start += len;
            if len == chunk.len() {
                self.chunks.next(&());
            }
            Ok(len)
        } else {
            Ok(0)
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Chunk(ArrayString<{ 2 * CHUNK_BASE }>);

impl Chunk {
    fn offset_to_point(&self, target: usize) -> Point {
        let mut offset = 0;
        let mut point = Point::new(0, 0);
        for ch in self.0.chars() {
            if offset >= target {
                break;
            }

            if ch == '\n' {
                point.row += 1;
                point.column = 0;
            } else {
                point.column += ch.len_utf8() as u32;
            }
            offset += ch.len_utf8();
        }
        point
    }

    fn offset_to_point_utf16(&self, target: usize) -> PointUtf16 {
        let mut offset = 0;
        let mut point = PointUtf16::new(0, 0);
        for ch in self.0.chars() {
            if offset >= target {
                break;
            }

            if ch == '\n' {
                point.row += 1;
                point.column = 0;
            } else {
                point.column += ch.len_utf16() as u32;
            }
            offset += ch.len_utf8();
        }
        point
    }

    fn point_to_offset(&self, target: Point) -> usize {
        let mut offset = 0;
        let mut point = Point::new(0, 0);
        for ch in self.0.chars() {
            if point >= target {
                if point > target {
                    panic!("point {:?} is inside of character {:?}", target, ch);
                }
                break;
            }

            if ch == '\n' {
                point.row += 1;
                if point.row > target.row {
                    panic!(
                        "point {:?} is beyond the end of a line with length {}",
                        target, point.column
                    );
                }
                point.column = 0;
            } else {
                point.column += ch.len_utf8() as u32;
            }
            offset += ch.len_utf8();
        }
        offset
    }

    fn point_to_point_utf16(&self, target: Point) -> PointUtf16 {
        let mut point = Point::zero();
        let mut point_utf16 = PointUtf16::new(0, 0);
        for ch in self.0.chars() {
            if point >= target {
                break;
            }

            if ch == '\n' {
                point_utf16.row += 1;
                point_utf16.column = 0;
                point.row += 1;
                point.column = 0;
            } else {
                point_utf16.column += ch.len_utf16() as u32;
                point.column += ch.len_utf8() as u32;
            }
        }
        point_utf16
    }

    fn point_utf16_to_offset(&self, target: PointUtf16) -> usize {
        let mut offset = 0;
        let mut point = PointUtf16::new(0, 0);
        for ch in self.0.chars() {
            if point >= target {
                if point > target {
                    panic!("point {:?} is inside of character {:?}", target, ch);
                }
                break;
            }

            if ch == '\n' {
                point.row += 1;
                if point.row > target.row {
                    panic!(
                        "point {:?} is beyond the end of a line with length {}",
                        target, point.column
                    );
                }
                point.column = 0;
            } else {
                point.column += ch.len_utf16() as u32;
            }
            offset += ch.len_utf8();
        }
        offset
    }

    fn point_utf16_to_point(&self, target: PointUtf16) -> Point {
        let mut point = Point::zero();
        let mut point_utf16 = PointUtf16::zero();
        for ch in self.0.chars() {
            if point_utf16 >= target {
                if point_utf16 > target {
                    panic!("point {:?} is inside of character {:?}", target, ch);
                }
                break;
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

    fn clip_point(&self, target: Point, bias: Bias) -> Point {
        for (row, line) in self.0.split('\n').enumerate() {
            if row == target.row as usize {
                let mut column = target.column.min(line.len() as u32);
                while !line.is_char_boundary(column as usize) {
                    match bias {
                        Bias::Left => column -= 1,
                        Bias::Right => column += 1,
                    }
                }
                return Point::new(row as u32, column);
            }
        }
        unreachable!()
    }

    fn clip_point_utf16(&self, target: PointUtf16, bias: Bias) -> PointUtf16 {
        for (row, line) in self.0.split('\n').enumerate() {
            if row == target.row as usize {
                let mut code_units = line.encode_utf16();
                let mut column = code_units.by_ref().take(target.column as usize).count();
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
}

impl sum_tree::Item for Chunk {
    type Summary = TextSummary;

    fn summary(&self) -> Self::Summary {
        TextSummary::from(self.0.as_str())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextSummary {
    pub bytes: usize,
    pub lines: Point,
    pub lines_utf16: PointUtf16,
    pub first_line_chars: u32,
    pub last_line_chars: u32,
    pub longest_row: u32,
    pub longest_row_chars: u32,
}

impl<'a> From<&'a str> for TextSummary {
    fn from(text: &'a str) -> Self {
        let mut lines = Point::new(0, 0);
        let mut lines_utf16 = PointUtf16::new(0, 0);
        let mut first_line_chars = 0;
        let mut last_line_chars = 0;
        let mut longest_row = 0;
        let mut longest_row_chars = 0;
        for c in text.chars() {
            if c == '\n' {
                lines += Point::new(1, 0);
                lines_utf16 += PointUtf16::new(1, 0);
                last_line_chars = 0;
            } else {
                lines.column += c.len_utf8() as u32;
                lines_utf16.column += c.len_utf16() as u32;
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
            bytes: text.len(),
            lines,
            lines_utf16,
            first_line_chars,
            last_line_chars,
            longest_row,
            longest_row_chars,
        }
    }
}

impl sum_tree::Summary for TextSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        *self += summary;
    }
}

impl<'a> std::ops::Add<Self> for TextSummary {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign(&rhs);
        self
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

        self.bytes += other.bytes;
        self.lines += other.lines;
        self.lines_utf16 += other.lines_utf16;
    }
}

impl std::ops::AddAssign<Self> for TextSummary {
    fn add_assign(&mut self, other: Self) {
        *self += &other;
    }
}

pub trait TextDimension: 'static + for<'a> Dimension<'a, TextSummary> {
    fn from_text_summary(summary: &TextSummary) -> Self;
    fn add_assign(&mut self, other: &Self);
}

impl<'a, D1: TextDimension, D2: TextDimension> TextDimension for (D1, D2) {
    fn from_text_summary(summary: &TextSummary) -> Self {
        (
            D1::from_text_summary(summary),
            D2::from_text_summary(summary),
        )
    }

    fn add_assign(&mut self, other: &Self) {
        self.0.add_assign(&other.0);
        self.1.add_assign(&other.1);
    }
}

impl TextDimension for TextSummary {
    fn from_text_summary(summary: &TextSummary) -> Self {
        summary.clone()
    }

    fn add_assign(&mut self, other: &Self) {
        *self += other;
    }
}

impl<'a> sum_tree::Dimension<'a, TextSummary> for usize {
    fn add_summary(&mut self, summary: &'a TextSummary, _: &()) {
        *self += summary.bytes;
    }
}

impl TextDimension for usize {
    fn from_text_summary(summary: &TextSummary) -> Self {
        summary.bytes
    }

    fn add_assign(&mut self, other: &Self) {
        *self += other;
    }
}

impl<'a> sum_tree::Dimension<'a, TextSummary> for Point {
    fn add_summary(&mut self, summary: &'a TextSummary, _: &()) {
        *self += summary.lines;
    }
}

impl TextDimension for Point {
    fn from_text_summary(summary: &TextSummary) -> Self {
        summary.lines
    }

    fn add_assign(&mut self, other: &Self) {
        *self += other;
    }
}

impl<'a> sum_tree::Dimension<'a, TextSummary> for PointUtf16 {
    fn add_summary(&mut self, summary: &'a TextSummary, _: &()) {
        *self += summary.lines_utf16;
    }
}

impl TextDimension for PointUtf16 {
    fn from_text_summary(summary: &TextSummary) -> Self {
        summary.lines_utf16
    }

    fn add_assign(&mut self, other: &Self) {
        *self += other;
    }
}

fn find_split_ix(text: &str) -> usize {
    let mut ix = text.len() / 2;
    while !text.is_char_boundary(ix) {
        if ix < 2 * CHUNK_BASE {
            ix += 1;
        } else {
            ix = (text.len() / 2) - 1;
            break;
        }
    }
    while !text.is_char_boundary(ix) {
        ix -= 1;
    }

    debug_assert!(ix <= 2 * CHUNK_BASE);
    debug_assert!(text.len() - ix <= 2 * CHUNK_BASE);
    ix
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random_char_iter::RandomCharIter;
    use rand::prelude::*;
    use std::{cmp::Ordering, env, io::Read};
    use Bias::{Left, Right};

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
            rope.clip_point_utf16(PointUtf16::new(0, 1), Bias::Left),
            PointUtf16::new(0, 0)
        );
        assert_eq!(
            rope.clip_point_utf16(PointUtf16::new(0, 1), Bias::Right),
            PointUtf16::new(0, 2)
        );
        assert_eq!(
            rope.clip_point_utf16(PointUtf16::new(0, 3), Bias::Right),
            PointUtf16::new(0, 2)
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random_rope(mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let mut expected = String::new();
        let mut actual = Rope::new();
        for _ in 0..operations {
            let end_ix = clip_offset(&expected, rng.gen_range(0..=expected.len()), Right);
            let start_ix = clip_offset(&expected, rng.gen_range(0..=end_ix), Left);
            let len = rng.gen_range(0..=64);
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
                let end_ix = clip_offset(&expected, rng.gen_range(0..=expected.len()), Right);
                let start_ix = clip_offset(&expected, rng.gen_range(0..=end_ix), Left);

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
            }

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
                if ch == '\n' {
                    point += Point::new(1, 0);
                    point_utf16 += PointUtf16::new(1, 0);
                } else {
                    point.column += ch.len_utf8() as u32;
                    point_utf16.column += ch.len_utf16() as u32;
                }
            }

            let mut point_utf16 = PointUtf16::zero();
            for unit in expected.encode_utf16() {
                let left_point = actual.clip_point_utf16(point_utf16, Bias::Left);
                let right_point = actual.clip_point_utf16(point_utf16, Bias::Right);
                assert!(right_point >= left_point);
                // Ensure translating UTF-16 points to offsets doesn't panic.
                actual.point_utf16_to_offset(left_point);
                actual.point_utf16_to_offset(right_point);

                if unit == b'\n' as u16 {
                    point_utf16 += PointUtf16::new(1, 0);
                } else {
                    point_utf16 += PointUtf16::new(0, 1);
                }
            }

            for _ in 0..5 {
                let end_ix = clip_offset(&expected, rng.gen_range(0..=expected.len()), Right);
                let start_ix = clip_offset(&expected, rng.gen_range(0..=end_ix), Left);
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
            for chunk in self.chunks.cursor::<()>() {
                text.push_str(&chunk.0);
            }
            text
        }
    }
}

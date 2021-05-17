use super::Point;
use crate::sum_tree::{self, SeekBias, SumTree};
use arrayvec::ArrayString;
use smallvec::SmallVec;
use std::{cmp, iter::Skip, ops::Range, str};

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
        let mut chunks = rope.chunks.cursor::<(), ()>();
        chunks.next();
        if let Some(chunk) = chunks.item() {
            if self.chunks.last().map_or(false, |c| c.0.len() < CHUNK_BASE)
                || chunk.0.len() < CHUNK_BASE
            {
                self.push(&chunk.0);
                chunks.next();
            }
        }

        self.chunks.push_tree(chunks.suffix(&()), &());
        self.check_invariants();
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
                        let mut text = ArrayString::<[_; 4 * CHUNK_BASE]>::new();
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

    fn check_invariants(&self) {
        #[cfg(test)]
        {
            // Ensure all chunks except maybe the last one are not underflowing.
            // Allow some wiggle room for multibyte characters at chunk boundaries.
            let mut chunks = self.chunks.cursor::<(), ()>().peekable();
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
        self.chunks.extent()
    }

    pub fn max_point(&self) -> Point {
        self.chunks.extent()
    }

    pub fn cursor(&self, offset: usize) -> Cursor {
        Cursor::new(self, offset)
    }

    pub fn chars(&self) -> Chars {
        self.chars_at(0)
    }

    pub fn chars_at(&self, start: usize) -> Chars {
        Chars::new(self, start)
    }

    pub fn chunks<'a>(&'a self) -> ChunksIter<'a> {
        self.chunks_in_range(0..self.len())
    }

    pub fn chunks_in_range<'a>(&'a self, range: Range<usize>) -> ChunksIter<'a> {
        ChunksIter::new(self, range)
    }

    pub fn to_point(&self, offset: usize) -> Point {
        assert!(offset <= self.summary().bytes);
        let mut cursor = self.chunks.cursor::<usize, TextSummary>();
        cursor.seek(&offset, SeekBias::Left, &());
        let overshoot = offset - cursor.start().bytes;
        cursor.start().lines
            + cursor
                .item()
                .map_or(Point::zero(), |chunk| chunk.to_point(overshoot))
    }

    pub fn to_offset(&self, point: Point) -> usize {
        assert!(point <= self.summary().lines);
        let mut cursor = self.chunks.cursor::<Point, TextSummary>();
        cursor.seek(&point, SeekBias::Left, &());
        let overshoot = point - cursor.start().lines;
        cursor.start().bytes + cursor.item().map_or(0, |chunk| chunk.to_offset(overshoot))
    }

    pub fn next_char_boundary(&self, mut offset: usize) -> usize {
        assert!(offset <= self.summary().bytes);
        let mut cursor = self.chunks.cursor::<usize, usize>();
        cursor.seek(&offset, SeekBias::Left, &());
        if let Some(chunk) = cursor.item() {
            let mut ix = offset - cursor.start();
            while !chunk.0.is_char_boundary(ix) {
                ix += 1;
                offset += 1;
            }
        }
        offset
    }

    pub fn prev_char_boundary(&self, mut offset: usize) -> usize {
        assert!(offset <= self.summary().bytes);
        let mut cursor = self.chunks.cursor::<usize, usize>();
        cursor.seek(&offset, SeekBias::Left, &());
        if let Some(chunk) = cursor.item() {
            let mut ix = offset - cursor.start();
            while !chunk.0.is_char_boundary(ix) {
                ix -= 1;
                offset -= 1;
            }
        }
        offset
    }
}

impl<'a> From<&'a str> for Rope {
    fn from(text: &'a str) -> Self {
        let mut rope = Self::new();
        rope.push(text);
        rope
    }
}

pub struct Cursor<'a> {
    rope: &'a Rope,
    chunks: sum_tree::Cursor<'a, Chunk, usize, usize>,
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(rope: &'a Rope, offset: usize) -> Self {
        let mut chunks = rope.chunks.cursor();
        chunks.seek(&offset, SeekBias::Right, &());
        Self {
            rope,
            chunks,
            offset,
        }
    }

    pub fn seek_forward(&mut self, end_offset: usize) {
        debug_assert!(end_offset >= self.offset);

        self.chunks.seek_forward(&end_offset, SeekBias::Right, &());
        self.offset = end_offset;
    }

    pub fn slice(&mut self, end_offset: usize) -> Rope {
        debug_assert!(end_offset >= self.offset);

        let mut slice = Rope::new();
        if let Some(start_chunk) = self.chunks.item() {
            let start_ix = self.offset - self.chunks.start();
            let end_ix = cmp::min(end_offset, self.chunks.end()) - self.chunks.start();
            slice.push(&start_chunk.0[start_ix..end_ix]);
        }

        if end_offset > self.chunks.end() {
            self.chunks.next();
            slice.append(Rope {
                chunks: self.chunks.slice(&end_offset, SeekBias::Right, &()),
            });
            if let Some(end_chunk) = self.chunks.item() {
                let end_ix = end_offset - self.chunks.start();
                slice.push(&end_chunk.0[..end_ix]);
            }
        }

        self.offset = end_offset;
        slice
    }

    pub fn summary(&mut self, end_offset: usize) -> TextSummary {
        debug_assert!(end_offset >= self.offset);

        let mut summary = TextSummary::default();
        if let Some(start_chunk) = self.chunks.item() {
            let start_ix = self.offset - self.chunks.start();
            let end_ix = cmp::min(end_offset, self.chunks.end()) - self.chunks.start();
            summary = TextSummary::from(&start_chunk.0[start_ix..end_ix]);
        }

        if end_offset > self.chunks.end() {
            self.chunks.next();
            summary += &self.chunks.summary(&end_offset, SeekBias::Right, &());
            if let Some(end_chunk) = self.chunks.item() {
                let end_ix = end_offset - self.chunks.start();
                summary += TextSummary::from(&end_chunk.0[..end_ix]);
            }
        }

        summary
    }

    pub fn suffix(mut self) -> Rope {
        self.slice(self.rope.chunks.extent())
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

pub struct ChunksIter<'a> {
    chunks: sum_tree::Cursor<'a, Chunk, usize, usize>,
    range: Range<usize>,
}

impl<'a> ChunksIter<'a> {
    pub fn new(rope: &'a Rope, range: Range<usize>) -> Self {
        let mut chunks = rope.chunks.cursor();
        chunks.seek(&range.start, SeekBias::Right, &());
        Self { chunks, range }
    }

    pub fn offset(&self) -> usize {
        self.range.start.max(*self.chunks.start())
    }

    pub fn advance_to(&mut self, offset: usize) {
        if offset >= self.chunks.end() {
            self.chunks.seek_forward(&offset, SeekBias::Right, &());
            self.range.start = offset;
        }
    }

    pub fn peek(&self) -> Option<&'a str> {
        if let Some(chunk) = self.chunks.item() {
            let offset = *self.chunks.start();
            if self.range.end > offset {
                let start = self.range.start.saturating_sub(*self.chunks.start());
                let end = self.range.end - self.chunks.start();
                return Some(&chunk.0[start..chunk.0.len().min(end)]);
            }
        }
        None
    }
}

impl<'a> Iterator for ChunksIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();
        if result.is_some() {
            self.chunks.next();
        }
        result
    }
}

#[derive(Clone, Debug, Default)]
struct Chunk(ArrayString<[u8; 2 * CHUNK_BASE]>);

impl Chunk {
    fn to_point(&self, target: usize) -> Point {
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

    fn to_offset(&self, target: Point) -> usize {
        let mut offset = 0;
        let mut point = Point::new(0, 0);
        for ch in self.0.chars() {
            if point >= target {
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

        assert_eq!(point, target);
        offset
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
    pub first_line_len: u32,
    pub rightmost_point: Point,
}

impl<'a> From<&'a str> for TextSummary {
    fn from(text: &'a str) -> Self {
        let mut lines = Point::new(0, 0);
        let mut first_line_len = 0;
        let mut rightmost_point = Point::new(0, 0);
        for (i, line) in text.split('\n').enumerate() {
            if i > 0 {
                lines.row += 1;
            }
            lines.column = line.len() as u32;
            if i == 0 {
                first_line_len = lines.column;
            }
            if lines.column > rightmost_point.column {
                rightmost_point = lines;
            }
        }

        TextSummary {
            bytes: text.len(),
            lines,
            first_line_len,
            rightmost_point,
        }
    }
}

impl sum_tree::Summary for TextSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &Self::Context) {
        *self += summary;
    }
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
    fn add_summary(&mut self, summary: &'a TextSummary) {
        *self += summary;
    }
}

impl<'a> sum_tree::Dimension<'a, TextSummary> for usize {
    fn add_summary(&mut self, summary: &'a TextSummary) {
        *self += summary.bytes;
    }
}

impl<'a> sum_tree::Dimension<'a, TextSummary> for Point {
    fn add_summary(&mut self, summary: &'a TextSummary) {
        *self += &summary.lines;
    }
}

pub struct Chars<'a> {
    cursor: sum_tree::Cursor<'a, Chunk, usize, usize>,
    chars: Skip<str::Chars<'a>>,
}

impl<'a> Chars<'a> {
    pub fn new(rope: &'a Rope, start: usize) -> Self {
        let mut cursor = rope.chunks.cursor::<usize, usize>();
        cursor.seek(&start, SeekBias::Left, &());
        let chars = if let Some(chunk) = cursor.item() {
            let ix = start - cursor.start();
            assert_char_boundary(&chunk.0, ix);
            cursor.next();
            chunk.0.chars().skip(ix)
        } else {
            "".chars().skip(0)
        };

        Self { cursor, chars }
    }
}

impl<'a> Iterator for Chars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.chars.next() {
            Some(ch)
        } else if let Some(chunk) = self.cursor.item() {
            self.chars = chunk.0.chars().skip(0);
            self.cursor.next();
            Some(self.chars.next().unwrap())
        } else {
            None
        }
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

#[inline(always)]
fn assert_char_boundary(s: &str, ix: usize) {
    if !s.is_char_boundary(ix) {
        let mut char_start = ix;
        while !s.is_char_boundary(char_start) {
            char_start -= 1;
        }

        let ch = s[char_start..].chars().next().unwrap();
        let char_range = char_start..char_start + ch.len_utf8();
        panic!(
            "byte index {} is not a char boundary; it is inside {:?} (bytes {:?}) of `{}`",
            ix, ch, char_range, s
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::util::RandomCharIter;

    use super::*;
    use rand::prelude::*;
    use std::env;

    #[test]
    fn test_all_4_byte_chars() {
        let mut rope = Rope::new();
        let text = "🏀".repeat(256);
        rope.push(&text);
        assert_eq!(rope.text(), text);
    }

    #[test]
    fn test_random() {
        let iterations = env::var("ITERATIONS")
            .map(|i| i.parse().expect("invalid `ITERATIONS` variable"))
            .unwrap_or(100);
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);
        let seed_range = if let Ok(seed) = env::var("SEED") {
            let seed = seed.parse().expect("invalid `SEED` variable");
            seed..seed + 1
        } else {
            0..iterations
        };

        for seed in seed_range {
            dbg!(seed);
            let mut rng = StdRng::seed_from_u64(seed);
            let mut expected = String::new();
            let mut actual = Rope::new();
            for _ in 0..operations {
                let end_ix = actual.next_char_boundary(rng.gen_range(0..=expected.len()));
                let start_ix = actual.prev_char_boundary(rng.gen_range(0..=end_ix));
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
                    let end_ix = actual.next_char_boundary(rng.gen_range(0..=expected.len()));
                    let start_ix = actual.prev_char_boundary(rng.gen_range(0..=end_ix));
                    assert_eq!(
                        actual.chunks_in_range(start_ix..end_ix).collect::<String>(),
                        &expected[start_ix..end_ix]
                    );
                }

                let mut point = Point::new(0, 0);
                for (ix, ch) in expected.char_indices().chain(Some((expected.len(), '\0'))) {
                    assert_eq!(actual.to_point(ix), point, "to_point({})", ix);
                    assert_eq!(actual.to_offset(point), ix, "to_offset({:?})", point);
                    if ch == '\n' {
                        point.row += 1;
                        point.column = 0
                    } else {
                        point.column += ch.len_utf8() as u32;
                    }
                }

                for _ in 0..5 {
                    let end_ix = actual.next_char_boundary(rng.gen_range(0..=expected.len()));
                    let start_ix = actual.prev_char_boundary(rng.gen_range(0..=end_ix));
                    assert_eq!(
                        actual.cursor(start_ix).summary(end_ix),
                        TextSummary::from(&expected[start_ix..end_ix])
                    );
                }
            }
        }
    }

    impl Rope {
        fn text(&self) -> String {
            let mut text = String::new();
            for chunk in self.chunks.cursor::<(), ()>() {
                text.push_str(&chunk.0);
            }
            text
        }
    }
}

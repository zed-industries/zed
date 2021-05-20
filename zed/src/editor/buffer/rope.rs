use super::Point;
use crate::{
    editor::Bias,
    sum_tree::{self, SeekBias, SumTree},
};
use arrayvec::ArrayString;
use smallvec::SmallVec;
use std::{cmp, ops::Range, str};

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

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars_at(0)
    }

    pub fn chars_at(&self, start: usize) -> impl Iterator<Item = char> + '_ {
        self.chunks_in_range(start..self.len()).flat_map(str::chars)
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

    pub fn clip_offset(&self, mut offset: usize, bias: Bias) -> usize {
        let mut cursor = self.chunks.cursor::<usize, usize>();
        cursor.seek(&offset, SeekBias::Left, &());
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
        let mut cursor = self.chunks.cursor::<Point, Point>();
        cursor.seek(&point, SeekBias::Right, &());
        if let Some(chunk) = cursor.item() {
            let overshoot = point - cursor.start();
            *cursor.start() + chunk.clip_point(overshoot, bias)
        } else {
            self.summary().lines
        }
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
                if point > target {
                    panic!("point {:?} is inside of character {:?}", target, ch);
                }
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
        offset
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
    pub first_line_chars: u32,
    pub last_line_chars: u32,
    pub rightmost_row: u32,
    pub rightmost_row_chars: u32,
}

impl<'a> From<&'a str> for TextSummary {
    fn from(text: &'a str) -> Self {
        let mut lines = Point::new(0, 0);
        let mut first_line_chars = 0;
        let mut last_line_chars = 0;
        let mut rightmost_row = 0;
        let mut rightmost_row_chars = 0;
        for c in text.chars() {
            if c == '\n' {
                lines.row += 1;
                lines.column = 0;
                last_line_chars = 0;
            } else {
                lines.column += c.len_utf8() as u32;
                last_line_chars += 1;
            }

            if lines.row == 0 {
                first_line_chars = last_line_chars;
            }

            if last_line_chars > rightmost_row_chars {
                rightmost_row = lines.row;
                rightmost_row_chars = last_line_chars;
            }
        }

        TextSummary {
            bytes: text.len(),
            lines,
            first_line_chars,
            last_line_chars,
            rightmost_row,
            rightmost_row_chars,
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
        let joined_chars = self.last_line_chars + other.first_line_chars;
        if joined_chars > self.rightmost_row_chars {
            self.rightmost_row = self.lines.row;
            self.rightmost_row_chars = joined_chars;
        }
        if other.rightmost_row_chars > self.rightmost_row_chars {
            self.rightmost_row = self.lines.row + other.rightmost_row;
            self.rightmost_row_chars = other.rightmost_row_chars;
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
    use crate::util::RandomCharIter;
    use rand::prelude::*;
    use std::env;
    use Bias::{Left, Right};

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
                    let end_ix = clip_offset(&expected, rng.gen_range(0..=expected.len()), Right);
                    let start_ix = clip_offset(&expected, rng.gen_range(0..=end_ix), Left);
                    assert_eq!(
                        actual.cursor(start_ix).summary(end_ix),
                        TextSummary::from(&expected[start_ix..end_ix])
                    );
                }
            }
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
            for chunk in self.chunks.cursor::<(), ()>() {
                text.push_str(&chunk.0);
            }
            text
        }
    }
}

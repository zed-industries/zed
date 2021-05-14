use super::Point;
use crate::sum_tree::{self, SeekBias, SumTree};
use arrayvec::ArrayString;
use std::{cmp, ops::Range, str};

#[cfg(test)]
const CHUNK_BASE: usize = 2;

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

        while let Some((last_chunk, first_chunk)) = self.chunks.last().zip(chunks.item()) {
            if last_chunk.0.len() < CHUNK_BASE || first_chunk.0.len() < CHUNK_BASE {
                self.push(&first_chunk.0);
                chunks.next();
            } else {
                break;
            }
        }

        self.chunks.push_tree(chunks.suffix(&()), &());
        self.check_invariants();
    }

    pub fn push(&mut self, mut text: &str) {
        let mut suffix = ArrayString::<[_; CHUNK_BASE]>::new();
        self.chunks.with_last_mut(
            |chunk| {
                if chunk.0.len() + text.len() <= 2 * CHUNK_BASE {
                    chunk.0.push_str(text);
                    text = "";
                } else if chunk.0.len() < CHUNK_BASE {
                    let mut split_ix = CHUNK_BASE - chunk.0.len();
                    while !text.is_char_boundary(split_ix) {
                        split_ix += 1;
                    }
                    let split = text.split_at(split_ix);
                    chunk.0.push_str(split.0);
                    text = split.1;
                } else {
                    let mut split_ix = CHUNK_BASE;
                    while !chunk.0.is_char_boundary(split_ix) {
                        split_ix += 1;
                    }
                    suffix.push_str(&chunk.0[split_ix..]);
                    chunk.0.truncate(split_ix);
                }
            },
            &(),
        );

        let mut chunks = vec![];
        let mut chunk = ArrayString::new();
        for ch in suffix.chars().chain(text.chars()) {
            if chunk.len() + ch.len_utf8() > 2 * CHUNK_BASE {
                chunks.push(Chunk(chunk));
                chunk = ArrayString::new();
            }
            chunk.push(ch);
        }
        if !chunk.is_empty() {
            chunks.push(Chunk(chunk));
        }
        self.chunks.extend(chunks, &());
        self.check_invariants();
    }

    fn check_invariants(&self) {
        #[cfg(test)]
        {
            let mut chunks = self.chunks.cursor::<(), ()>().peekable();
            chunks.next();
            while let Some(chunk) = chunks.next() {
                if chunks.peek().is_some() {
                    assert!(
                        chunk.0.len() >= CHUNK_BASE,
                        "Underflowing chunk: {:?}\nChunks: {:?}",
                        chunk,
                        self.chunks.items()
                    );
                }
            }
        }
    }

    pub fn slice(&self, range: Range<usize>) -> Rope {
        let mut slice = Rope::new();
        let mut cursor = self.chunks.cursor::<usize, usize>();

        cursor.slice(&range.start, SeekBias::Left, &());
        if let Some(start_chunk) = cursor.item() {
            let start_ix = range.start - cursor.start();
            let end_ix = cmp::min(range.end, cursor.end()) - cursor.start();
            slice.push(&start_chunk.0[start_ix..end_ix]);
        }

        if range.end > cursor.end() {
            cursor.next();
            slice.append(Rope {
                chunks: cursor.slice(&range.end, SeekBias::Left, &()),
            });
            if let Some(end_chunk) = cursor.item() {
                slice.push(&end_chunk.0[..range.end - cursor.start()]);
            }
        }
        slice.check_invariants();
        slice
    }

    pub fn summary(&self) -> TextSummary {
        self.chunks.summary()
    }

    pub fn chars(&self) -> Chars {
        self.chars_at(0)
    }

    pub fn chars_at(&self, start: usize) -> Chars {
        Chars::new(self, start)
    }

    fn text(&self) -> String {
        let mut text = String::new();
        for chunk in self.chunks.cursor::<(), ()>() {
            text.push_str(&chunk.0);
        }
        text
    }
}

impl<'a> From<&'a str> for Rope {
    fn from(text: &'a str) -> Self {
        let mut rope = Self::new();
        rope.push(text);
        rope
    }
}

#[derive(Clone, Debug, Default)]
struct Chunk(ArrayString<[u8; 2 * CHUNK_BASE]>);

impl sum_tree::Item for Chunk {
    type Summary = TextSummary;

    fn summary(&self) -> Self::Summary {
        let mut chars = 0;
        let mut bytes = 0;
        let mut lines = Point::new(0, 0);
        let mut first_line_len = 0;
        let mut rightmost_point = Point::new(0, 0);
        for c in self.0.chars() {
            chars += 1;
            bytes += c.len_utf8();
            if c == '\n' {
                lines.row += 1;
                lines.column = 0;
            } else {
                lines.column += 1;
                if lines.row == 0 {
                    first_line_len = lines.column;
                }
                if lines.column > rightmost_point.column {
                    rightmost_point = lines;
                }
            }
        }

        TextSummary {
            chars,
            bytes,
            lines,
            first_line_len,
            rightmost_point,
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

impl<'a> sum_tree::Dimension<'a, TextSummary> for usize {
    fn add_summary(&mut self, summary: &'a TextSummary) {
        *self += summary.chars;
    }
}

pub struct Chars<'a> {
    cursor: sum_tree::Cursor<'a, Chunk, usize, usize>,
    chars: str::Chars<'a>,
}

impl<'a> Chars<'a> {
    pub fn new(rope: &'a Rope, start: usize) -> Self {
        let mut cursor = rope.chunks.cursor::<usize, usize>();
        cursor.slice(&start, SeekBias::Left, &());
        let chunk = cursor.item().expect("invalid index");
        let chars = chunk.0[start - cursor.start()..].chars();
        Self { cursor, chars }
    }
}

impl<'a> Iterator for Chars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.chars.next() {
            Some(ch)
        } else if let Some(chunk) = self.cursor.item() {
            self.chars = chunk.0.chars();
            self.cursor.next();
            Some(self.chars.next().unwrap())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::RandomCharIter;

    use super::*;
    use rand::prelude::*;
    use std::env;

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
                let end_ix = rng.gen_range(0..=expected.len());
                let start_ix = rng.gen_range(0..=end_ix);
                let len = rng.gen_range(0..=20);
                let new_text: String = RandomCharIter::new(&mut rng).take(len).collect();

                let mut new_actual = Rope::new();
                new_actual.append(actual.slice(0..start_ix));
                new_actual.push(&new_text);
                new_actual.append(actual.slice(end_ix..actual.summary().chars));
                actual = new_actual;

                let mut new_expected = String::new();
                new_expected.push_str(&expected[..start_ix]);
                new_expected.push_str(&new_text);
                new_expected.push_str(&expected[end_ix..]);
                expected = new_expected;

                assert_eq!(actual.text(), expected);
            }
        }
    }
}

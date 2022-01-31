use super::{Point, ToOffset};
use crate::{rope::TextDimension, BufferSnapshot};
use anyhow::Result;
use std::{cmp::Ordering, fmt::Debug, ops::Range};
use sum_tree::Bias;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Anchor {
    pub timestamp: clock::Local,
    pub offset: usize,
    pub bias: Bias,
}

impl Anchor {
    pub fn min() -> Self {
        Self {
            timestamp: clock::Local::MIN,
            offset: usize::MIN,
            bias: Bias::Left,
        }
    }

    pub fn max() -> Self {
        Self {
            timestamp: clock::Local::MAX,
            offset: usize::MAX,
            bias: Bias::Right,
        }
    }

    pub fn cmp(&self, other: &Anchor, buffer: &BufferSnapshot) -> Result<Ordering> {
        let fragment_id_comparison = if self.timestamp == other.timestamp {
            Ordering::Equal
        } else {
            buffer
                .fragment_id_for_anchor(self)
                .cmp(&buffer.fragment_id_for_anchor(other))
        };

        Ok(fragment_id_comparison
            .then_with(|| self.offset.cmp(&other.offset))
            .then_with(|| self.bias.cmp(&other.bias)))
    }

    pub fn bias(&self, bias: Bias, buffer: &BufferSnapshot) -> Anchor {
        if bias == Bias::Left {
            self.bias_left(buffer)
        } else {
            self.bias_right(buffer)
        }
    }

    pub fn bias_left(&self, buffer: &BufferSnapshot) -> Anchor {
        if self.bias == Bias::Left {
            self.clone()
        } else {
            buffer.anchor_before(self)
        }
    }

    pub fn bias_right(&self, buffer: &BufferSnapshot) -> Anchor {
        if self.bias == Bias::Right {
            self.clone()
        } else {
            buffer.anchor_after(self)
        }
    }

    pub fn summary<'a, D>(&self, content: &'a BufferSnapshot) -> D
    where
        D: TextDimension,
    {
        content.summary_for_anchor(self)
    }
}

pub trait AnchorRangeExt {
    fn cmp(&self, b: &Range<Anchor>, buffer: &BufferSnapshot) -> Result<Ordering>;
    fn to_offset(&self, content: &BufferSnapshot) -> Range<usize>;
    fn to_point(&self, content: &BufferSnapshot) -> Range<Point>;
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp(&self, other: &Range<Anchor>, buffer: &BufferSnapshot) -> Result<Ordering> {
        Ok(match self.start.cmp(&other.start, buffer)? {
            Ordering::Equal => other.end.cmp(&self.end, buffer)?,
            ord @ _ => ord,
        })
    }

    fn to_offset(&self, content: &BufferSnapshot) -> Range<usize> {
        self.start.to_offset(&content)..self.end.to_offset(&content)
    }

    fn to_point(&self, content: &BufferSnapshot) -> Range<Point> {
        self.start.summary::<Point>(&content)..self.end.summary::<Point>(&content)
    }
}

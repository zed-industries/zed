use super::{rope::TextDimension, Buffer, Point, Snapshot, ToOffset};
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

    pub fn cmp<'a>(&self, other: &Anchor, buffer: &Snapshot) -> Result<Ordering> {
        let offset_comparison = if self.timestamp == other.timestamp {
            self.offset.cmp(&other.offset)
        } else {
            buffer
                .full_offset_for_anchor(self)
                .cmp(&buffer.full_offset_for_anchor(other))
        };

        Ok(offset_comparison.then_with(|| self.bias.cmp(&other.bias)))
    }

    pub fn bias_left(&self, buffer: &Buffer) -> Anchor {
        if self.bias == Bias::Left {
            self.clone()
        } else {
            buffer.anchor_before(self)
        }
    }

    pub fn bias_right(&self, buffer: &Buffer) -> Anchor {
        if self.bias == Bias::Right {
            self.clone()
        } else {
            buffer.anchor_after(self)
        }
    }

    pub fn summary<'a, D>(&self, content: &'a Snapshot) -> D
    where
        D: TextDimension<'a>,
    {
        content.summary_for_anchor(self)
    }
}

pub trait AnchorRangeExt {
    fn cmp(&self, b: &Range<Anchor>, buffer: &Snapshot) -> Result<Ordering>;
    fn to_offset(&self, content: &Snapshot) -> Range<usize>;
    fn to_point(&self, content: &Snapshot) -> Range<Point>;
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp(&self, other: &Range<Anchor>, buffer: &Snapshot) -> Result<Ordering> {
        Ok(match self.start.cmp(&other.start, buffer)? {
            Ordering::Equal => other.end.cmp(&self.end, buffer)?,
            ord @ _ => ord,
        })
    }

    fn to_offset(&self, content: &Snapshot) -> Range<usize> {
        self.start.to_offset(&content)..self.end.to_offset(&content)
    }

    fn to_point(&self, content: &Snapshot) -> Range<Point> {
        self.start.summary::<Point>(&content)..self.end.summary::<Point>(&content)
    }
}

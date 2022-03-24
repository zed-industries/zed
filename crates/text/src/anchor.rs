use super::{Point, ToOffset};
use crate::{rope::TextDimension, BufferSnapshot, PointUtf16, ToPoint, ToPointUtf16};
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
    pub const MIN: Self = Self {
        timestamp: clock::Local::MIN,
        offset: usize::MIN,
        bias: Bias::Left,
    };

    pub const MAX: Self = Self {
        timestamp: clock::Local::MAX,
        offset: usize::MAX,
        bias: Bias::Right,
    };

    pub fn cmp(&self, other: &Anchor, buffer: &BufferSnapshot) -> Ordering {
        let fragment_id_comparison = if self.timestamp == other.timestamp {
            Ordering::Equal
        } else {
            buffer
                .fragment_id_for_anchor(self)
                .cmp(&buffer.fragment_id_for_anchor(other))
        };

        fragment_id_comparison
            .then_with(|| self.offset.cmp(&other.offset))
            .then_with(|| self.bias.cmp(&other.bias))
    }

    pub fn min(&self, other: &Self, buffer: &BufferSnapshot) -> Self {
        if self.cmp(other, buffer).is_le() {
            self.clone()
        } else {
            other.clone()
        }
    }

    pub fn max(&self, other: &Self, buffer: &BufferSnapshot) -> Self {
        if self.cmp(other, buffer).is_ge() {
            self.clone()
        } else {
            other.clone()
        }
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

pub trait OffsetRangeExt {
    fn to_offset(&self, snapshot: &BufferSnapshot) -> Range<usize>;
    fn to_point(&self, snapshot: &BufferSnapshot) -> Range<Point>;
    fn to_point_utf16(&self, snapshot: &BufferSnapshot) -> Range<PointUtf16>;
}

impl<T> OffsetRangeExt for Range<T>
where
    T: ToOffset,
{
    fn to_offset(&self, snapshot: &BufferSnapshot) -> Range<usize> {
        self.start.to_offset(snapshot)..self.end.to_offset(&snapshot)
    }

    fn to_point(&self, snapshot: &BufferSnapshot) -> Range<Point> {
        self.start.to_offset(snapshot).to_point(snapshot)
            ..self.end.to_offset(snapshot).to_point(snapshot)
    }

    fn to_point_utf16(&self, snapshot: &BufferSnapshot) -> Range<PointUtf16> {
        self.start.to_offset(snapshot).to_point_utf16(snapshot)
            ..self.end.to_offset(snapshot).to_point_utf16(snapshot)
    }
}

pub trait AnchorRangeExt {
    fn cmp(&self, b: &Range<Anchor>, buffer: &BufferSnapshot) -> Result<Ordering>;
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp(&self, other: &Range<Anchor>, buffer: &BufferSnapshot) -> Result<Ordering> {
        Ok(match self.start.cmp(&other.start, buffer) {
            Ordering::Equal => other.end.cmp(&self.end, buffer),
            ord @ _ => ord,
        })
    }
}

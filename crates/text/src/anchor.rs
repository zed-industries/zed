use crate::{
    BufferId, BufferSnapshot, Point, PointUtf16, TextDimension, ToOffset, ToPoint, ToPointUtf16,
    locator::Locator,
};
use std::{cmp::Ordering, fmt::Debug, ops::Range};
use sum_tree::{Bias, Dimensions};

/// A timestamped position in a buffer
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct Anchor {
    pub timestamp: clock::Lamport,
    /// The byte offset in the buffer
    pub offset: usize,
    /// Describes which character the anchor is biased towards
    pub bias: Bias,
    pub buffer_id: Option<BufferId>,
}

impl Anchor {
    pub const MIN: Self = Self {
        timestamp: clock::Lamport::MIN,
        offset: usize::MIN,
        bias: Bias::Left,
        buffer_id: None,
    };

    pub const MAX: Self = Self {
        timestamp: clock::Lamport::MAX,
        offset: usize::MAX,
        bias: Bias::Right,
        buffer_id: None,
    };

    pub fn cmp(&self, other: &Anchor, buffer: &BufferSnapshot) -> Ordering {
        let fragment_id_comparison = if self.timestamp == other.timestamp {
            Ordering::Equal
        } else {
            buffer
                .fragment_id_for_anchor(self)
                .cmp(buffer.fragment_id_for_anchor(other))
        };

        fragment_id_comparison
            .then_with(|| self.offset.cmp(&other.offset))
            .then_with(|| self.bias.cmp(&other.bias))
    }

    pub fn min(&self, other: &Self, buffer: &BufferSnapshot) -> Self {
        if self.cmp(other, buffer).is_le() {
            *self
        } else {
            *other
        }
    }

    pub fn max(&self, other: &Self, buffer: &BufferSnapshot) -> Self {
        if self.cmp(other, buffer).is_ge() {
            *self
        } else {
            *other
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
            *self
        } else {
            buffer.anchor_before(self)
        }
    }

    pub fn bias_right(&self, buffer: &BufferSnapshot) -> Anchor {
        if self.bias == Bias::Right {
            *self
        } else {
            buffer.anchor_after(self)
        }
    }

    pub fn summary<D>(&self, content: &BufferSnapshot) -> D
    where
        D: TextDimension,
    {
        content.summary_for_anchor(self)
    }

    /// Returns true when the [`Anchor`] is located inside a visible fragment.
    pub fn is_valid(&self, buffer: &BufferSnapshot) -> bool {
        if *self == Anchor::MIN || *self == Anchor::MAX {
            true
        } else if self.buffer_id != Some(buffer.remote_id) {
            false
        } else {
            let Some(fragment_id) = buffer.try_fragment_id_for_anchor(self) else {
                return false;
            };
            let mut fragment_cursor = buffer
                .fragments
                .cursor::<Dimensions<Option<&Locator>, usize>>(&None);
            fragment_cursor.seek(&Some(fragment_id), Bias::Left);
            fragment_cursor
                .item()
                .map_or(false, |fragment| fragment.visible)
        }
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
        self.start.to_offset(snapshot)..self.end.to_offset(snapshot)
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
    fn cmp(&self, b: &Range<Anchor>, buffer: &BufferSnapshot) -> Ordering;
    fn overlaps(&self, b: &Range<Anchor>, buffer: &BufferSnapshot) -> bool;
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp(&self, other: &Range<Anchor>, buffer: &BufferSnapshot) -> Ordering {
        match self.start.cmp(&other.start, buffer) {
            Ordering::Equal => other.end.cmp(&self.end, buffer),
            ord => ord,
        }
    }

    fn overlaps(&self, other: &Range<Anchor>, buffer: &BufferSnapshot) -> bool {
        self.start.cmp(&other.end, buffer).is_lt() && other.start.cmp(&self.end, buffer).is_lt()
    }
}

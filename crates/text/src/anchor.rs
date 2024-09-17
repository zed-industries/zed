use crate::{
    locator::Locator, BufferId, BufferSnapshot, Point, PointUtf16, TextDimension, ToOffset,
    ToPoint, ToPointUtf16,
};
use std::{cmp::Ordering, fmt::Debug, ops::Range};
use sum_tree::Bias;

/// A timestamped position in a buffer
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum Anchor {
    #[default]
    Start,
    End,
    Character {
        buffer_id: BufferId,
        insertion_id: clock::Lamport,
        offset: usize,
        bias: Bias,
    },
}

impl Anchor {
    pub fn cmp(&self, other: &Anchor, buffer: &BufferSnapshot) -> Ordering {
        match (self, other) {
            (Anchor::Start, Anchor::Start) | (Anchor::End, Anchor::End) => Ordering::Equal,
            (Anchor::Start, _) | (_, Anchor::End) => Ordering::Less,
            (_, Anchor::Start) | (Anchor::End, _) => Ordering::Greater,
            (
                Anchor::Character {
                    buffer_id,
                    insertion_id,
                    offset,
                    bias,
                },
                Anchor::Character {
                    buffer_id: other_buffer_id,
                    insertion_id: other_insertion_id,
                    offset: other_offset,
                    bias: other_bias,
                },
            ) => {
                debug_assert_eq!(
                    buffer_id, other_buffer_id,
                    "anchors belong to different buffers"
                );

                let fragment_id_comparison = if insertion_id == other_insertion_id {
                    Ordering::Equal
                } else {
                    buffer
                        .fragment_id_for_anchor(self)
                        .cmp(buffer.fragment_id_for_anchor(other))
                };

                fragment_id_comparison
                    .then_with(|| offset.cmp(&other_offset))
                    .then_with(|| bias.cmp(&other_bias))
            }
        }
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
        match self {
            Anchor::Start => Anchor::Start,
            Anchor::End => buffer.anchor_before(buffer.len()),
            Anchor::Character {
                buffer_id,
                insertion_id,
                offset,
                ..
            } => Anchor::Character {
                buffer_id: *buffer_id,
                insertion_id: *insertion_id,
                offset: *offset,
                bias: Bias::Left,
            },
        }
    }

    pub fn bias_right(&self, buffer: &BufferSnapshot) -> Anchor {
        match self {
            Anchor::Start => buffer.anchor_after(0),
            Anchor::End => Anchor::End,
            Anchor::Character {
                buffer_id,
                insertion_id,
                offset,
                ..
            } => Anchor::Character {
                buffer_id: *buffer_id,
                insertion_id: *insertion_id,
                offset: *offset,
                bias: Bias::Right,
            },
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
        match self {
            Anchor::Start | Anchor::End => true,
            Anchor::Character { buffer_id, .. } => {
                if *buffer_id == buffer.remote_id {
                    let fragment_id = buffer.fragment_id_for_anchor(self);
                    let mut fragment_cursor =
                        buffer.fragments.cursor::<(Option<&Locator>, usize)>();
                    fragment_cursor.seek(&Some(fragment_id), Bias::Left, &None);
                    fragment_cursor
                        .item()
                        .map_or(false, |fragment| fragment.visible)
                } else {
                    false
                }
            }
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
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp(&self, other: &Range<Anchor>, buffer: &BufferSnapshot) -> Ordering {
        match self.start.cmp(&other.start, buffer) {
            Ordering::Equal => other.end.cmp(&self.end, buffer),
            ord => ord,
        }
    }
}

use super::{ExcerptId, MultiBufferSnapshot, ToOffset, ToOffsetUtf16, ToPoint};
use language::{OffsetUtf16, Point, TextDimension};
use std::{
    cmp::Ordering,
    ops::{Range, Sub},
};
use sum_tree::Bias;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Anchor {
    Start,
    End,
    Text {
        excerpt_id: ExcerptId,
        text_anchor: text::Anchor,
    },
}

impl Anchor {
    pub fn excerpt_id(&self) -> ExcerptId {
        match self {
            Anchor::Start => ExcerptId::min(),
            Anchor::End => ExcerptId::max(),
            Anchor::Text { excerpt_id, .. } => *excerpt_id,
        }
    }

    pub fn cmp(&self, other: &Anchor, snapshot: &MultiBufferSnapshot) -> Ordering {
        match (self, other) {
            (Anchor::Start, Anchor::Start) | (Anchor::End, Anchor::End) => Ordering::Equal,
            (_, Anchor::Start) | (Anchor::End, _) => Ordering::Greater,
            (Anchor::Start, _) | (_, Anchor::End) => Ordering::Less,
            (
                Anchor::Text {
                    excerpt_id: id1,
                    text_anchor: anchor1,
                },
                Anchor::Text {
                    excerpt_id: id2,
                    text_anchor: anchor2,
                },
            ) => {
                let excerpt_id_cmp = id1.cmp(id2, snapshot);
                if excerpt_id_cmp.is_eq() {
                    if let Some(excerpt) = snapshot.excerpt(*id1) {
                        anchor1.cmp(anchor2, &excerpt.buffer)
                    } else {
                        Ordering::Equal
                    }
                } else {
                    excerpt_id_cmp
                }
            }
        }
    }

    pub fn bias(&self) -> Bias {
        match self {
            Anchor::Start => Bias::Left,
            Anchor::End => Bias::Right,
            Anchor::Text { text_anchor, .. } => match text_anchor {
                text::Anchor::Start { .. } => Bias::Left,
                text::Anchor::End { .. } => Bias::Right,
                text::Anchor::Character { bias, .. } => *bias,
            },
        }
    }

    pub fn bias_left(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        match self {
            Anchor::Start => *self,
            Anchor::End => snapshot.anchor_before(snapshot.len()),
            Anchor::Text {
                excerpt_id,
                text_anchor,
            } => {
                if let Some(excerpt) = snapshot.excerpt(*excerpt_id) {
                    Anchor::Text {
                        excerpt_id: *excerpt_id,
                        text_anchor: text_anchor.bias_left(&excerpt.buffer),
                    }
                } else {
                    *self
                }
            }
        }
    }

    pub fn bias_right(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        match self {
            Anchor::Start => snapshot.anchor_after(0),
            Anchor::End => *self,
            Anchor::Text {
                excerpt_id,
                text_anchor,
            } => {
                if let Some(excerpt) = snapshot.excerpt(*excerpt_id) {
                    Anchor::Text {
                        excerpt_id: *excerpt_id,
                        text_anchor: text_anchor.bias_right(&excerpt.buffer),
                    }
                } else {
                    *self
                }
            }
        }
    }

    pub fn summary<D>(&self, snapshot: &MultiBufferSnapshot) -> D
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        snapshot.summary_for_anchor(self)
    }

    pub fn is_valid(&self, snapshot: &MultiBufferSnapshot) -> bool {
        match self {
            Self::Start | Anchor::End => true,
            Anchor::Text {
                excerpt_id,
                text_anchor,
            } => {
                if let Some(excerpt) = snapshot.excerpt(*excerpt_id) {
                    excerpt.contains(self) && text_anchor.is_valid(&excerpt.buffer)
                } else {
                    false
                }
            }
        }
    }
}

impl ToOffset for Anchor {
    fn to_offset(&self, snapshot: &MultiBufferSnapshot) -> usize {
        self.summary(snapshot)
    }
}

impl ToOffsetUtf16 for Anchor {
    fn to_offset_utf16(&self, snapshot: &MultiBufferSnapshot) -> OffsetUtf16 {
        self.summary(snapshot)
    }
}

impl ToPoint for Anchor {
    fn to_point<'a>(&self, snapshot: &MultiBufferSnapshot) -> Point {
        self.summary(snapshot)
    }
}

pub trait AnchorRangeExt {
    fn cmp(&self, b: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> Ordering;
    fn overlaps(&self, b: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> bool;
    fn to_offset(&self, content: &MultiBufferSnapshot) -> Range<usize>;
    fn to_point(&self, content: &MultiBufferSnapshot) -> Range<Point>;
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp(&self, other: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> Ordering {
        match self.start.cmp(&other.start, buffer) {
            Ordering::Equal => other.end.cmp(&self.end, buffer),
            ord => ord,
        }
    }

    fn overlaps(&self, other: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> bool {
        self.end.cmp(&other.start, buffer).is_ge() && self.start.cmp(&other.end, buffer).is_le()
    }

    fn to_offset(&self, content: &MultiBufferSnapshot) -> Range<usize> {
        self.start.to_offset(content)..self.end.to_offset(content)
    }

    fn to_point(&self, content: &MultiBufferSnapshot) -> Range<Point> {
        self.start.to_point(content)..self.end.to_point(content)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
pub struct Offset(pub usize);

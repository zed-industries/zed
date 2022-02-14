use super::{ExcerptId, MultiBufferSnapshot, ToOffset, ToPoint};
use anyhow::Result;
use std::{
    cmp::Ordering,
    ops::{Range, Sub},
};
use sum_tree::Bias;
use text::{rope::TextDimension, Point};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Anchor {
    pub(crate) buffer_id: Option<usize>,
    pub(crate) excerpt_id: ExcerptId,
    pub(crate) text_anchor: text::Anchor,
}

impl Anchor {
    pub fn min() -> Self {
        Self {
            buffer_id: None,
            excerpt_id: ExcerptId::min(),
            text_anchor: text::Anchor::min(),
        }
    }

    pub fn max() -> Self {
        Self {
            buffer_id: None,
            excerpt_id: ExcerptId::max(),
            text_anchor: text::Anchor::max(),
        }
    }

    pub fn excerpt_id(&self) -> &ExcerptId {
        &self.excerpt_id
    }

    pub fn cmp<'a>(&self, other: &Anchor, snapshot: &MultiBufferSnapshot) -> Result<Ordering> {
        let excerpt_id_cmp = self.excerpt_id.cmp(&other.excerpt_id);
        if excerpt_id_cmp.is_eq() {
            if self.excerpt_id == ExcerptId::min() || self.excerpt_id == ExcerptId::max() {
                Ok(Ordering::Equal)
            } else if let Some((buffer_id, buffer_snapshot)) =
                snapshot.buffer_snapshot_for_excerpt(&self.excerpt_id)
            {
                // Even though the anchor refers to a valid excerpt the underlying buffer might have
                // changed. In that case, treat the anchor as if it were at the start of that
                // excerpt.
                if self.buffer_id == Some(buffer_id) && other.buffer_id == Some(buffer_id) {
                    self.text_anchor.cmp(&other.text_anchor, buffer_snapshot)
                } else if self.buffer_id == Some(buffer_id) {
                    Ok(Ordering::Greater)
                } else if other.buffer_id == Some(buffer_id) {
                    Ok(Ordering::Less)
                } else {
                    Ok(Ordering::Equal)
                }
            } else {
                Ok(Ordering::Equal)
            }
        } else {
            Ok(excerpt_id_cmp)
        }
    }

    pub fn bias_left(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        if self.text_anchor.bias != Bias::Left {
            if let Some((buffer_id, buffer_snapshot)) =
                snapshot.buffer_snapshot_for_excerpt(&self.excerpt_id)
            {
                if self.buffer_id == Some(buffer_id) {
                    return Self {
                        buffer_id: self.buffer_id,
                        excerpt_id: self.excerpt_id.clone(),
                        text_anchor: self.text_anchor.bias_left(buffer_snapshot),
                    };
                }
            }
        }
        self.clone()
    }

    pub fn bias_right(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        if self.text_anchor.bias != Bias::Right {
            if let Some((buffer_id, buffer_snapshot)) =
                snapshot.buffer_snapshot_for_excerpt(&self.excerpt_id)
            {
                if self.buffer_id == Some(buffer_id) {
                    return Self {
                        buffer_id: self.buffer_id,
                        excerpt_id: self.excerpt_id.clone(),
                        text_anchor: self.text_anchor.bias_right(buffer_snapshot),
                    };
                }
            }
        }
        self.clone()
    }

    pub fn summary<D>(&self, snapshot: &MultiBufferSnapshot) -> D
    where
        D: TextDimension + Ord + Sub<D, Output = D>,
    {
        snapshot.summary_for_anchor(self)
    }
}

impl ToOffset for Anchor {
    fn to_offset(&self, snapshot: &MultiBufferSnapshot) -> usize {
        self.summary(snapshot)
    }
}

impl ToPoint for Anchor {
    fn to_point<'a>(&self, snapshot: &MultiBufferSnapshot) -> Point {
        self.summary(snapshot)
    }
}

pub trait AnchorRangeExt {
    fn cmp(&self, b: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> Result<Ordering>;
    fn to_offset(&self, content: &MultiBufferSnapshot) -> Range<usize>;
    fn to_point(&self, content: &MultiBufferSnapshot) -> Range<Point>;
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp(&self, other: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> Result<Ordering> {
        Ok(match self.start.cmp(&other.start, buffer)? {
            Ordering::Equal => other.end.cmp(&self.end, buffer)?,
            ord @ _ => ord,
        })
    }

    fn to_offset(&self, content: &MultiBufferSnapshot) -> Range<usize> {
        self.start.to_offset(&content)..self.end.to_offset(&content)
    }

    fn to_point(&self, content: &MultiBufferSnapshot) -> Range<Point> {
        self.start.to_point(&content)..self.end.to_point(&content)
    }
}

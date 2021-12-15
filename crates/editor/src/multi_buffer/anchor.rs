use super::{ExcerptId, MultiBufferSnapshot, ToOffset, ToPoint};
use anyhow::{anyhow, Result};
use std::{
    cmp::Ordering,
    ops::{Range, Sub},
};
use sum_tree::Bias;
use text::{rope::TextDimension, Point};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Anchor {
    pub(crate) excerpt_id: ExcerptId,
    pub(crate) text_anchor: text::Anchor,
}

impl Anchor {
    pub fn new(excerpt_id: ExcerptId, text_anchor: text::Anchor) -> Self {
        Self {
            excerpt_id,
            text_anchor,
        }
    }

    pub fn min() -> Self {
        Self {
            excerpt_id: ExcerptId::min(),
            text_anchor: text::Anchor::min(),
        }
    }

    pub fn max() -> Self {
        Self {
            excerpt_id: ExcerptId::max(),
            text_anchor: text::Anchor::max(),
        }
    }

    pub fn cmp<'a>(&self, other: &Anchor, snapshot: &MultiBufferSnapshot) -> Result<Ordering> {
        let excerpt_id_cmp = self.excerpt_id.cmp(&other.excerpt_id);
        if excerpt_id_cmp.is_eq() {
            if self.excerpt_id == ExcerptId::min() || self.excerpt_id == ExcerptId::max() {
                Ok(Ordering::Equal)
            } else {
                self.text_anchor.cmp(
                    &other.text_anchor,
                    snapshot
                        .buffer_snapshot_for_excerpt(&self.excerpt_id)
                        .ok_or_else(|| anyhow!("excerpt {:?} not found", self.excerpt_id))?,
                )
            }
        } else {
            return Ok(excerpt_id_cmp);
        }
    }

    pub fn bias_left(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        if self.text_anchor.bias != Bias::Left {
            if let Some(buffer_snapshot) = snapshot.buffer_snapshot_for_excerpt(&self.excerpt_id) {
                return Self {
                    excerpt_id: self.excerpt_id.clone(),
                    text_anchor: self.text_anchor.bias_left(buffer_snapshot),
                };
            }
        }
        self.clone()
    }

    pub fn bias_right(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        if self.text_anchor.bias != Bias::Right {
            if let Some(buffer_snapshot) = snapshot.buffer_snapshot_for_excerpt(&self.excerpt_id) {
                return Self {
                    excerpt_id: self.excerpt_id.clone(),
                    text_anchor: self.text_anchor.bias_right(buffer_snapshot),
                };
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
    fn to_offset<'a>(&self, snapshot: &MultiBufferSnapshot) -> usize {
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

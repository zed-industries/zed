use crate::{MultiBufferDimension, MultiBufferOffset, MultiBufferOffsetUtf16};

use super::{ExcerptId, MultiBufferSnapshot, ToOffset, ToPoint};
use language::Point;
use std::{
    cmp::Ordering,
    ops::{AddAssign, Range, Sub},
};
use sum_tree::Bias;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Anchor {
    pub excerpt_id: ExcerptId,
    pub text_anchor: text::Anchor,
    pub diff_base_anchor: Option<text::Anchor>,
}

impl std::fmt::Debug for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_min() {
            return write!(f, "Anchor::min({:?})", self.text_anchor.buffer_id);
        }
        if self.is_max() {
            return write!(f, "Anchor::max({:?})", self.text_anchor.buffer_id);
        }

        f.debug_struct("Anchor")
            .field("excerpt_id", &self.excerpt_id)
            .field("text_anchor", &self.text_anchor)
            .field("diff_base_anchor", &self.diff_base_anchor)
            .finish()
    }
}

impl Anchor {
    pub fn with_diff_base_anchor(self, diff_base_anchor: text::Anchor) -> Self {
        Self {
            diff_base_anchor: Some(diff_base_anchor),
            ..self
        }
    }

    pub fn in_buffer(excerpt_id: ExcerptId, text_anchor: text::Anchor) -> Self {
        Self {
            excerpt_id,
            text_anchor,
            diff_base_anchor: None,
        }
    }

    pub fn range_in_buffer(excerpt_id: ExcerptId, range: Range<text::Anchor>) -> Range<Self> {
        Self::in_buffer(excerpt_id, range.start)..Self::in_buffer(excerpt_id, range.end)
    }

    pub fn min() -> Self {
        Self {
            excerpt_id: ExcerptId::min(),
            text_anchor: text::Anchor::MIN,
            diff_base_anchor: None,
        }
    }

    pub fn max() -> Self {
        Self {
            excerpt_id: ExcerptId::max(),
            text_anchor: text::Anchor::MAX,
            diff_base_anchor: None,
        }
    }

    pub fn is_min(&self) -> bool {
        self.excerpt_id == ExcerptId::min()
            && self.text_anchor.is_min()
            && self.diff_base_anchor.is_none()
    }

    pub fn is_max(&self) -> bool {
        self.excerpt_id == ExcerptId::max()
            && self.text_anchor.is_max()
            && self.diff_base_anchor.is_none()
    }

    pub fn cmp(&self, other: &Anchor, snapshot: &MultiBufferSnapshot) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        let self_excerpt_id = snapshot.latest_excerpt_id(self.excerpt_id);
        let other_excerpt_id = snapshot.latest_excerpt_id(other.excerpt_id);

        let excerpt_id_cmp = self_excerpt_id.cmp(&other_excerpt_id, snapshot);
        if excerpt_id_cmp.is_ne() {
            return excerpt_id_cmp;
        }
        if self_excerpt_id == ExcerptId::max()
            && self.text_anchor.is_max()
            && self.text_anchor.is_max()
            && self.diff_base_anchor.is_none()
            && other.diff_base_anchor.is_none()
        {
            return Ordering::Equal;
        }
        if let Some(excerpt) = snapshot.excerpt(self_excerpt_id) {
            let text_cmp = self.text_anchor.cmp(&other.text_anchor, &excerpt.buffer);
            if text_cmp.is_ne() {
                return text_cmp;
            }
            if (self.diff_base_anchor.is_some() || other.diff_base_anchor.is_some())
                && let Some(base_text) = snapshot
                    .diffs
                    .get(&excerpt.buffer_id)
                    .map(|diff| diff.base_text())
            {
                let self_anchor = self.diff_base_anchor.filter(|a| base_text.can_resolve(a));
                let other_anchor = other.diff_base_anchor.filter(|a| base_text.can_resolve(a));
                return match (self_anchor, other_anchor) {
                    (Some(a), Some(b)) => a.cmp(&b, base_text),
                    (Some(_), None) => match other.text_anchor.bias {
                        Bias::Left => Ordering::Greater,
                        Bias::Right => Ordering::Less,
                    },
                    (None, Some(_)) => match self.text_anchor.bias {
                        Bias::Left => Ordering::Less,
                        Bias::Right => Ordering::Greater,
                    },
                    (None, None) => Ordering::Equal,
                };
            }
        }
        Ordering::Equal
    }

    pub fn bias(&self) -> Bias {
        self.text_anchor.bias
    }

    pub fn bias_left(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        if self.text_anchor.bias != Bias::Left
            && let Some(excerpt) = snapshot.excerpt(self.excerpt_id)
        {
            return Self {
                excerpt_id: excerpt.id,
                text_anchor: self.text_anchor.bias_left(&excerpt.buffer),
                diff_base_anchor: self.diff_base_anchor.map(|a| {
                    if let Some(base_text) = snapshot
                        .diffs
                        .get(&excerpt.buffer_id)
                        .map(|diff| diff.base_text())
                        && a.buffer_id == Some(base_text.remote_id())
                    {
                        return a.bias_left(base_text);
                    }
                    a
                }),
            };
        }
        *self
    }

    pub fn bias_right(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        if self.text_anchor.bias != Bias::Right
            && let Some(excerpt) = snapshot.excerpt(self.excerpt_id)
        {
            return Self {
                excerpt_id: excerpt.id,
                text_anchor: self.text_anchor.bias_right(&excerpt.buffer),
                diff_base_anchor: self.diff_base_anchor.map(|a| {
                    if let Some(base_text) = snapshot
                        .diffs
                        .get(&excerpt.buffer_id)
                        .map(|diff| diff.base_text())
                        && a.buffer_id == Some(base_text.remote_id())
                    {
                        return a.bias_right(base_text);
                    }
                    a
                }),
            };
        }
        *self
    }

    pub fn summary<D>(&self, snapshot: &MultiBufferSnapshot) -> D
    where
        D: MultiBufferDimension
            + Ord
            + Sub<Output = D::TextDimension>
            + AddAssign<D::TextDimension>,
        D::TextDimension: Sub<Output = D::TextDimension> + Ord,
    {
        snapshot.summary_for_anchor(self)
    }

    pub fn is_valid(&self, snapshot: &MultiBufferSnapshot) -> bool {
        if self.is_min() || self.is_max() {
            true
        } else if let Some(excerpt) = snapshot.excerpt(self.excerpt_id) {
            (self.text_anchor == excerpt.range.context.start
                || self.text_anchor == excerpt.range.context.end
                || self.text_anchor.is_valid(&excerpt.buffer))
                && excerpt.contains(self)
        } else {
            false
        }
    }
}

impl ToOffset for Anchor {
    fn to_offset(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffset {
        self.summary(snapshot)
    }
    fn to_offset_utf16(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffsetUtf16 {
        self.summary(snapshot)
    }
}

impl ToPoint for Anchor {
    fn to_point<'a>(&self, snapshot: &MultiBufferSnapshot) -> Point {
        self.summary(snapshot)
    }
    fn to_point_utf16(&self, snapshot: &MultiBufferSnapshot) -> rope::PointUtf16 {
        self.summary(snapshot)
    }
}

pub trait AnchorRangeExt {
    fn cmp(&self, other: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> Ordering;
    fn includes(&self, other: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> bool;
    fn overlaps(&self, other: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> bool;
    fn to_offset(&self, content: &MultiBufferSnapshot) -> Range<MultiBufferOffset>;
    fn to_point(&self, content: &MultiBufferSnapshot) -> Range<Point>;
}

impl AnchorRangeExt for Range<Anchor> {
    fn cmp(&self, other: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> Ordering {
        match self.start.cmp(&other.start, buffer) {
            Ordering::Equal => other.end.cmp(&self.end, buffer),
            ord => ord,
        }
    }

    fn includes(&self, other: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> bool {
        self.start.cmp(&other.start, buffer).is_le() && other.end.cmp(&self.end, buffer).is_le()
    }

    fn overlaps(&self, other: &Range<Anchor>, buffer: &MultiBufferSnapshot) -> bool {
        self.end.cmp(&other.start, buffer).is_ge() && self.start.cmp(&other.end, buffer).is_le()
    }

    fn to_offset(&self, content: &MultiBufferSnapshot) -> Range<MultiBufferOffset> {
        self.start.to_offset(content)..self.end.to_offset(content)
    }

    fn to_point(&self, content: &MultiBufferSnapshot) -> Range<Point> {
        self.start.to_point(content)..self.end.to_point(content)
    }
}

use crate::{
    ExcerptSummary, MultiBufferDimension, MultiBufferOffset, MultiBufferOffsetUtf16, PathKey,
    PathKeyIndex, find_diff_state,
};

use super::{MultiBufferSnapshot, ToOffset, ToPoint};
use language::{BufferSnapshot, Point};
use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Range, Sub},
};
use sum_tree::Bias;
use text::BufferId;

/// A multibuffer anchor derived from an anchor into a specific excerpted buffer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ExcerptAnchor {
    pub(crate) text_anchor: text::Anchor,
    pub(crate) path: PathKeyIndex,
    pub(crate) diff_base_anchor: Option<text::Anchor>,
}

/// A stable reference to a position within a [`MultiBuffer`](super::MultiBuffer).
///
/// Unlike simple offsets, anchors remain valid as the text is edited, automatically
/// adjusting to reflect insertions and deletions around them.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Anchor {
    /// An anchor that always resolves to the start of the multibuffer.
    Min,
    /// An anchor that's attached to a specific excerpted buffer.
    Excerpt(ExcerptAnchor),
    /// An anchor that always resolves to the end of the multibuffer.
    Max,
}

pub(crate) enum AnchorSeekTarget<'a> {
    // buffer no longer exists at its original path key in the multibuffer
    Missing {
        path_key: &'a PathKey,
    },
    // we have excerpts for the buffer at the expected path key
    Excerpt {
        path_key: &'a PathKey,
        path_key_index: PathKeyIndex,
        anchor: text::Anchor,
        snapshot: &'a BufferSnapshot,
    },
    // no excerpts and it's a min or max anchor
    Empty,
}

impl std::fmt::Debug for AnchorSeekTarget<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Excerpt {
                path_key,
                path_key_index: _,
                anchor,
                snapshot: _,
            } => f
                .debug_struct("Excerpt")
                .field("path_key", path_key)
                .field("anchor", anchor)
                .finish(),
            Self::Missing { path_key } => f
                .debug_struct("Missing")
                .field("path_key", path_key)
                .finish(),
            Self::Empty => f.debug_struct("Empty").finish(),
        }
    }
}

impl std::fmt::Debug for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Anchor::Min => write!(f, "Anchor::Min"),
            Anchor::Max => write!(f, "Anchor::Max"),
            Anchor::Excerpt(excerpt_anchor) => write!(f, "{excerpt_anchor:?}"),
        }
    }
}

impl From<ExcerptAnchor> for Anchor {
    fn from(anchor: ExcerptAnchor) -> Self {
        Anchor::Excerpt(anchor)
    }
}

impl ExcerptAnchor {
    pub(crate) fn buffer_id(&self) -> BufferId {
        self.text_anchor.buffer_id
    }

    pub(crate) fn text_anchor(&self) -> text::Anchor {
        self.text_anchor
    }

    pub(crate) fn with_diff_base_anchor(mut self, diff_base_anchor: text::Anchor) -> Self {
        self.diff_base_anchor = Some(diff_base_anchor);
        self
    }

    pub(crate) fn cmp(&self, other: &Self, snapshot: &MultiBufferSnapshot) -> Ordering {
        let Some(self_path_key) = snapshot.path_keys.get_index(self.path.0 as usize) else {
            panic!("anchor's path was never added to multibuffer")
        };
        let Some(other_path_key) = snapshot.path_keys.get_index(other.path.0 as usize) else {
            panic!("anchor's path was never added to multibuffer")
        };

        match self_path_key.cmp(other_path_key) {
            Ordering::Equal => (),
            ordering => return ordering,
        }

        // in the case that you removed the buffer containing self,
        // and added the buffer containing other with the same path key
        // (ordering is arbitrary but consistent)
        if self.text_anchor.buffer_id != other.text_anchor.buffer_id {
            return self.text_anchor.buffer_id.cmp(&other.text_anchor.buffer_id);
        }

        // two anchors into the same buffer at the same path
        let Some(buffer) = snapshot
            .buffers
            .get(&self.text_anchor.buffer_id)
            .filter(|buffer_state| buffer_state.path_key == *self_path_key)
        else {
            // buffer no longer exists at the original path (which may have been reused for a different buffer),
            // so no way to compare the anchors
            return Ordering::Equal;
        };
        // two anchors into the same buffer at the same path that still exists at that path in the multibuffer
        let text_cmp = self
            .text_anchor()
            .cmp(&other.text_anchor(), &buffer.buffer_snapshot);
        if text_cmp != Ordering::Equal {
            return text_cmp;
        }

        if (self.diff_base_anchor.is_some() || other.diff_base_anchor.is_some())
            && let Some(base_text) = find_diff_state(&snapshot.diffs, self.text_anchor.buffer_id)
                .map(|diff| diff.base_text())
        {
            let self_anchor = self.diff_base_anchor.filter(|a| a.is_valid(base_text));
            let other_anchor = other.diff_base_anchor.filter(|a| a.is_valid(base_text));
            return match (self_anchor, other_anchor) {
                (Some(a), Some(b)) => a.cmp(&b, base_text),
                (Some(_), None) => match other.text_anchor().bias {
                    Bias::Left => Ordering::Greater,
                    Bias::Right => Ordering::Less,
                },
                (None, Some(_)) => match self.text_anchor().bias {
                    Bias::Left => Ordering::Less,
                    Bias::Right => Ordering::Greater,
                },
                (None, None) => Ordering::Equal,
            };
        }

        Ordering::Equal
    }

    fn bias_left(&self, snapshot: &MultiBufferSnapshot) -> Self {
        if self.text_anchor.bias == Bias::Left {
            return *self;
        }
        let Some(buffer) = snapshot.buffer_for_id(self.text_anchor.buffer_id) else {
            return *self;
        };
        let text_anchor = self.text_anchor().bias_left(&buffer);
        let ret = Self::in_buffer(self.path, text_anchor);
        if let Some(diff_base_anchor) = self.diff_base_anchor {
            if let Some(diff) = find_diff_state(&snapshot.diffs, self.text_anchor.buffer_id)
                && diff_base_anchor.is_valid(&diff.base_text())
            {
                ret.with_diff_base_anchor(diff_base_anchor.bias_left(diff.base_text()))
            } else {
                ret.with_diff_base_anchor(diff_base_anchor)
            }
        } else {
            ret
        }
    }

    fn bias_right(&self, snapshot: &MultiBufferSnapshot) -> Self {
        if self.text_anchor.bias == Bias::Right {
            return *self;
        }
        let Some(buffer) = snapshot.buffer_for_id(self.text_anchor.buffer_id) else {
            return *self;
        };
        let text_anchor = self.text_anchor().bias_right(&buffer);
        let ret = Self::in_buffer(self.path, text_anchor);
        if let Some(diff_base_anchor) = self.diff_base_anchor {
            if let Some(diff) = find_diff_state(&snapshot.diffs, self.text_anchor.buffer_id)
                && diff_base_anchor.is_valid(&diff.base_text())
            {
                ret.with_diff_base_anchor(diff_base_anchor.bias_right(diff.base_text()))
            } else {
                ret.with_diff_base_anchor(diff_base_anchor)
            }
        } else {
            ret
        }
    }

    #[track_caller]
    pub(crate) fn in_buffer(path: PathKeyIndex, text_anchor: text::Anchor) -> Self {
        ExcerptAnchor {
            path,
            diff_base_anchor: None,
            text_anchor,
        }
    }

    fn is_valid(&self, snapshot: &MultiBufferSnapshot) -> bool {
        let Some(target) = self.try_seek_target(snapshot) else {
            return false;
        };
        let Some(buffer_snapshot) = snapshot.buffer_for_id(self.buffer_id()) else {
            return false;
        };
        // Early check to avoid invalid comparisons when seeking
        if !buffer_snapshot.can_resolve(&self.text_anchor) {
            return false;
        }
        let mut cursor = snapshot.excerpts.cursor::<ExcerptSummary>(());
        cursor.seek(&target, Bias::Left);
        let Some(excerpt) = cursor.item() else {
            return false;
        };
        let is_valid = self.text_anchor == excerpt.range.context.start
            || self.text_anchor == excerpt.range.context.end
            || self.text_anchor.is_valid(&buffer_snapshot);
        is_valid
            && excerpt
                .range
                .context
                .start
                .cmp(&self.text_anchor(), buffer_snapshot)
                .is_le()
            && excerpt
                .range
                .context
                .end
                .cmp(&self.text_anchor(), buffer_snapshot)
                .is_ge()
    }

    pub(crate) fn seek_target<'a>(
        &self,
        snapshot: &'a MultiBufferSnapshot,
    ) -> AnchorSeekTarget<'a> {
        self.try_seek_target(snapshot)
            .expect("anchor is from different multi-buffer")
    }

    pub(crate) fn try_seek_target<'a>(
        &self,
        snapshot: &'a MultiBufferSnapshot,
    ) -> Option<AnchorSeekTarget<'a>> {
        let path_key = snapshot.try_path_for_anchor(*self)?;

        let Some(state) = snapshot
            .buffers
            .get(&self.buffer_id())
            .filter(|state| &state.path_key == path_key)
        else {
            return Some(AnchorSeekTarget::Missing { path_key });
        };

        Some(AnchorSeekTarget::Excerpt {
            path_key,
            path_key_index: self.path,
            anchor: self.text_anchor(),
            snapshot: &state.buffer_snapshot,
        })
    }
}

impl ToOffset for ExcerptAnchor {
    fn to_offset(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffset {
        Anchor::from(*self).to_offset(snapshot)
    }

    fn to_offset_utf16(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffsetUtf16 {
        Anchor::from(*self).to_offset_utf16(snapshot)
    }
}

impl ToPoint for ExcerptAnchor {
    fn to_point(&self, snapshot: &MultiBufferSnapshot) -> Point {
        Anchor::from(*self).to_point(snapshot)
    }

    fn to_point_utf16(&self, snapshot: &MultiBufferSnapshot) -> rope::PointUtf16 {
        Anchor::from(*self).to_point_utf16(snapshot)
    }
}

impl Anchor {
    pub fn is_min(&self) -> bool {
        matches!(self, Self::Min)
    }

    pub fn is_max(&self) -> bool {
        matches!(self, Self::Max)
    }

    pub(crate) fn in_buffer(path: PathKeyIndex, text_anchor: text::Anchor) -> Self {
        Self::Excerpt(ExcerptAnchor::in_buffer(path, text_anchor))
    }

    pub(crate) fn range_in_buffer(path: PathKeyIndex, range: Range<text::Anchor>) -> Range<Self> {
        Self::in_buffer(path, range.start)..Self::in_buffer(path, range.end)
    }

    pub fn cmp(&self, other: &Anchor, snapshot: &MultiBufferSnapshot) -> Ordering {
        match (self, other) {
            (Anchor::Min, Anchor::Min) => return Ordering::Equal,
            (Anchor::Max, Anchor::Max) => return Ordering::Equal,
            (Anchor::Min, _) => return Ordering::Less,
            (Anchor::Max, _) => return Ordering::Greater,
            (_, Anchor::Max) => return Ordering::Less,
            (_, Anchor::Min) => return Ordering::Greater,
            (Anchor::Excerpt(self_excerpt_anchor), Anchor::Excerpt(other_excerpt_anchor)) => {
                self_excerpt_anchor.cmp(other_excerpt_anchor, snapshot)
            }
        }
    }

    pub fn bias(&self) -> Bias {
        match self {
            Anchor::Min => Bias::Left,
            Anchor::Max => Bias::Right,
            Anchor::Excerpt(anchor) => anchor.text_anchor.bias,
        }
    }

    pub fn bias_left(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        match self {
            Anchor::Min => *self,
            Anchor::Max => snapshot.anchor_before(snapshot.max_point()),
            Anchor::Excerpt(anchor) => Anchor::Excerpt(anchor.bias_left(snapshot)),
        }
    }

    pub fn bias_right(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        match self {
            Anchor::Max => *self,
            Anchor::Min => snapshot.anchor_after(Point::zero()),
            Anchor::Excerpt(anchor) => Anchor::Excerpt(anchor.bias_right(snapshot)),
        }
    }

    pub fn summary<D>(&self, snapshot: &MultiBufferSnapshot) -> D
    where
        D: MultiBufferDimension
            + Ord
            + Sub<Output = D::TextDimension>
            + Sub<D::TextDimension, Output = D>
            + AddAssign<D::TextDimension>
            + Add<D::TextDimension, Output = D>,
        D::TextDimension: Sub<Output = D::TextDimension> + Ord,
    {
        snapshot.summary_for_anchor(self)
    }

    pub fn is_valid(&self, snapshot: &MultiBufferSnapshot) -> bool {
        match self {
            Anchor::Min | Anchor::Max => true,
            Anchor::Excerpt(excerpt_anchor) => excerpt_anchor.is_valid(snapshot),
        }
    }

    fn to_excerpt_anchor(&self, snapshot: &MultiBufferSnapshot) -> Option<ExcerptAnchor> {
        match self {
            Anchor::Min => {
                let excerpt = snapshot.excerpts.first()?;

                Some(ExcerptAnchor {
                    text_anchor: excerpt.range.context.start,
                    path: excerpt.path_key_index,
                    diff_base_anchor: None,
                })
            }
            Anchor::Excerpt(excerpt_anchor) => Some(*excerpt_anchor),
            Anchor::Max => {
                let excerpt = snapshot.excerpts.last()?;

                Some(ExcerptAnchor {
                    text_anchor: excerpt.range.context.end,
                    path: excerpt.path_key_index,
                    diff_base_anchor: None,
                })
            }
        }
    }

    pub(crate) fn seek_target<'a>(
        &self,
        snapshot: &'a MultiBufferSnapshot,
    ) -> AnchorSeekTarget<'a> {
        let Some(excerpt_anchor) = self.to_excerpt_anchor(snapshot) else {
            return AnchorSeekTarget::Empty;
        };

        excerpt_anchor.seek_target(snapshot)
    }

    pub(crate) fn excerpt_anchor(&self) -> Option<ExcerptAnchor> {
        match self {
            Anchor::Min | Anchor::Max => None,
            Anchor::Excerpt(excerpt_anchor) => Some(*excerpt_anchor),
        }
    }

    pub(crate) fn text_anchor(&self) -> Option<text::Anchor> {
        match self {
            Anchor::Min | Anchor::Max => None,
            Anchor::Excerpt(excerpt_anchor) => Some(excerpt_anchor.text_anchor()),
        }
    }

    pub fn opaque_id(&self) -> Option<[u8; 20]> {
        self.text_anchor().map(|a| a.opaque_id())
    }

    /// Note: anchor_to_buffer_anchor is probably what you want
    pub fn raw_text_anchor(&self) -> Option<text::Anchor> {
        match self {
            Anchor::Min | Anchor::Max => None,
            Anchor::Excerpt(excerpt_anchor) => Some(excerpt_anchor.text_anchor),
        }
    }

    pub(crate) fn try_seek_target<'a>(
        &self,
        snapshot: &'a MultiBufferSnapshot,
    ) -> Option<AnchorSeekTarget<'a>> {
        let Some(excerpt_anchor) = self.to_excerpt_anchor(snapshot) else {
            return Some(AnchorSeekTarget::Empty);
        };
        excerpt_anchor.try_seek_target(snapshot)
    }

    /// Returns the text anchor for this anchor.
    /// Panics if the anchor is from a different buffer.
    pub fn text_anchor_in(&self, buffer: &BufferSnapshot) -> text::Anchor {
        match self {
            Anchor::Min => text::Anchor::min_for_buffer(buffer.remote_id()),
            Anchor::Excerpt(excerpt_anchor) => {
                let text_anchor = excerpt_anchor.text_anchor;
                assert_eq!(text_anchor.buffer_id, buffer.remote_id());
                text_anchor
            }
            Anchor::Max => text::Anchor::max_for_buffer(buffer.remote_id()),
        }
    }

    pub fn diff_base_anchor(&self) -> Option<text::Anchor> {
        self.excerpt_anchor()?.diff_base_anchor
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn expect_text_anchor(&self) -> text::Anchor {
        self.excerpt_anchor().unwrap().text_anchor
    }

    pub fn with_diff_base_anchor(mut self, diff_base_anchor: text::Anchor) -> Self {
        match &mut self {
            Anchor::Min | Anchor::Max => {}
            Anchor::Excerpt(excerpt_anchor) => {
                excerpt_anchor.diff_base_anchor = Some(diff_base_anchor);
            }
        }
        self
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

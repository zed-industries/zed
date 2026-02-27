use crate::{
    ExcerptSummary, MultiBufferDimension, MultiBufferOffset, MultiBufferOffsetUtf16, PathKeyIndex,
};

use super::{MultiBufferSnapshot, ToOffset, ToPoint};
use language::Point;
use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Range, Sub},
};
use sum_tree::Bias;
use text::BufferId;
use util::debug_panic;

/// A stable reference to a position within a [`MultiBuffer`](super::MultiBuffer).
///
/// Unlike simple offsets, anchors remain valid as the text is edited, automatically
/// adjusting to reflect insertions and deletions around them.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Anchor {
    Min,
    Max,
    Text {
        /// The position within the excerpt's underlying buffer. This is a stable
        /// reference that remains valid as the buffer text is edited.
        timestamp: clock::Lamport,

        /// The byte offset into the text inserted in the operation
        /// at `timestamp`.
        offset: u32,
        /// Whether this anchor stays attached to the character *before* or *after*
        /// the offset.
        bias: Bias,
        buffer_id: BufferId,
        /// Refers to the path key that the buffer had when this anchor was created,
        /// so that ordering is stable when the path key for a buffer changes
        path: PathKeyIndex,
        /// When present, indicates this anchor points into deleted text within an
        /// expanded diff hunk. The anchor references a position in the diff base
        /// (original) text rather than the current buffer text. This is used when
        /// displaying inline diffs where deleted lines are shown.
        diff_base_anchor: Option<text::Anchor>,
    },
}

impl std::fmt::Debug for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_min() {
            return write!(f, "Anchor::Min");
        }
        if self.is_max() {
            return write!(f, "Anchor::Max");
        }

        f.debug_struct("Anchor")
            .field("text_anchor", &self.text_anchor().unwrap())
            .field("diff_base_anchor", &self.diff_base_anchor())
            .finish()
    }
}

impl Anchor {
    pub fn text_anchor(&self) -> Option<text::Anchor> {
        match self {
            Self::Min | Self::Max => None,
            Self::Text {
                timestamp,
                offset,
                bias,
                buffer_id,
                ..
            } => Some(text::Anchor::new(
                *timestamp,
                *offset,
                *bias,
                Some(*buffer_id),
            )),
        }
    }

    pub fn diff_base_anchor(&self) -> Option<text::Anchor> {
        match self {
            Self::Min | Self::Max => None,
            Self::Text {
                diff_base_anchor, ..
            } => *diff_base_anchor,
        }
    }

    pub fn with_diff_base_anchor(mut self, diff_base_anchor: text::Anchor) -> Self {
        match &mut self {
            Self::Min | Self::Max => {
                debug_panic!("with_diff_base_anchor called on min or max anchor");
            }
            Self::Text {
                diff_base_anchor: self_diff_base_anchor,
                ..
            } => {
                *self_diff_base_anchor = Some(diff_base_anchor);
            }
        };
        self
    }

    pub fn text(path: PathKeyIndex, text_anchor: text::Anchor) -> Self {
        let Some(buffer_id) = text_anchor.buffer_id else {
            panic!("text_anchor must have a buffer_id");
        };
        Self::Text {
            path,
            diff_base_anchor: None,
            timestamp: text_anchor.timestamp(),
            buffer_id,
            offset: text_anchor.offset,
            bias: text_anchor.bias,
        }
    }

    pub fn range_in_buffer(path: PathKeyIndex, range: Range<text::Anchor>) -> Range<Self> {
        Self::text(path, range.start)..Self::text(path, range.end)
    }

    pub fn min() -> Self {
        Self::Min
    }

    pub fn max() -> Self {
        Self::Max
    }

    pub fn is_min(&self) -> bool {
        matches!(self, Self::Min)
    }

    pub fn is_max(&self) -> bool {
        matches!(self, Self::Max)
    }

    pub fn cmp(&self, other: &Anchor, snapshot: &MultiBufferSnapshot) -> Ordering {
        let (self_text_anchor, self_path, other_text_anchor, other_path) = match (self, other) {
            (Anchor::Min, Anchor::Min) => return Ordering::Equal,
            (Anchor::Max, Anchor::Max) => return Ordering::Equal,
            (Anchor::Min, _) => return Ordering::Less,
            (Anchor::Max, _) => return Ordering::Greater,
            (_, Anchor::Max) => return Ordering::Less,
            (_, Anchor::Min) => return Ordering::Greater,
            (
                Anchor::Text {
                    path: self_path, ..
                },
                Anchor::Text {
                    path: other_path, ..
                },
            ) => (
                self.text_anchor().unwrap(),
                self_path,
                other.text_anchor().unwrap(),
                other_path,
            ),
        };
        let self_buffer_id = self_text_anchor.buffer_id.unwrap();
        let other_buffer_id = other_text_anchor.buffer_id.unwrap();

        let Some(self_path_key) = snapshot.path_keys_by_index.get(&self_path) else {
            panic!("anchor's path was never added to multibuffer")
        };
        let Some(other_path_key) = snapshot.path_keys_by_index.get(&other_path) else {
            panic!("anchor's path was never added to multibuffer")
        };

        if self_path_key.cmp(other_path_key) != Ordering::Equal {
            return self_path_key.cmp(other_path_key);
        }

        // in the case that you removed the buffer contianing self,
        // and added the buffer containing other with the same path key
        // (ordering is arbitrary but consistent)
        if self_buffer_id != other_buffer_id {
            return self_buffer_id.cmp(&other_buffer_id);
        }

        let Some(buffer) = snapshot.buffer_for_path(&self_path_key) else {
            return Ordering::Equal;
        };
        let text_cmp = self_text_anchor.cmp(&other_text_anchor, buffer);
        if text_cmp != Ordering::Equal {
            return text_cmp;
        }

        if (self.diff_base_anchor().is_some() || other.diff_base_anchor().is_some())
            && let Some(base_text) = snapshot
                .diffs
                .get(&self_buffer_id)
                .map(|diff| diff.base_text())
        {
            let self_anchor = self.diff_base_anchor().filter(|a| a.is_valid(base_text));
            let other_anchor = other.diff_base_anchor().filter(|a| a.is_valid(base_text));
            return match (self_anchor, other_anchor) {
                (Some(a), Some(b)) => a.cmp(&b, base_text),
                (Some(_), None) => match other_text_anchor.bias {
                    Bias::Left => Ordering::Greater,
                    Bias::Right => Ordering::Less,
                },
                (None, Some(_)) => match self_text_anchor.bias {
                    Bias::Left => Ordering::Less,
                    Bias::Right => Ordering::Greater,
                },
                (None, None) => Ordering::Equal,
            };
        }

        Ordering::Equal
    }

    pub fn bias(&self) -> Bias {
        match self {
            Anchor::Min => Bias::Left,
            Anchor::Max => Bias::Right,
            Anchor::Text { bias, .. } => *bias,
        }
    }

    pub fn bias_left(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        match self {
            Anchor::Min => *self,
            Anchor::Max => snapshot.anchor_before(snapshot.max_point()),
            Anchor::Text {
                path,
                bias,
                buffer_id,
                ..
            } => {
                if *bias == Bias::Left {
                    return *self;
                }
                let Some(buffer) = snapshot.buffer_for_id(*buffer_id) else {
                    return *self;
                };
                let text_anchor = self.text_anchor().unwrap().bias_left(&buffer);
                let ret = Self::text(*path, text_anchor);
                if let Some(diff_base_anchor) = self.diff_base_anchor() {
                    if let Some(diff) = snapshot.diffs.get(&buffer_id)
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
        }
    }

    pub fn bias_right(&self, snapshot: &MultiBufferSnapshot) -> Anchor {
        match self {
            Anchor::Min => *self,
            Anchor::Max => snapshot.anchor_after(Point::zero()),
            Anchor::Text {
                path,
                bias,
                buffer_id,
                ..
            } => {
                if *bias == Bias::Right {
                    return *self;
                }
                let Some(buffer) = snapshot.buffer_for_id(*buffer_id) else {
                    return *self;
                };
                let text_anchor = self.text_anchor().unwrap().bias_right(&buffer);
                let ret = Self::text(*path, text_anchor);
                if let Some(diff_base_anchor) = self.diff_base_anchor() {
                    if let Some(diff) = snapshot.diffs.get(&buffer_id)
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
        let Some(text_anchor) = self.text_anchor() else {
            return true;
        };
        let Some(buffer_id) = text_anchor.buffer_id else {
            debug_panic!("missing buffer_id for anchor");
            return false;
        };

        let Some(target) = snapshot.anchor_seek_target(*self) else {
            return false;
        };
        let mut cursor = snapshot.excerpts.cursor::<ExcerptSummary>(());
        cursor.seek(&target, Bias::Left);
        let Some(excerpt) = cursor.item() else {
            return false;
        };
        excerpt.buffer.remote_id() == buffer_id
            && excerpt
                .range
                .context
                .start
                .cmp(&text_anchor, &excerpt.buffer)
                .is_le()
            && excerpt
                .range
                .context
                .end
                .cmp(&text_anchor, &excerpt.buffer)
                .is_ge()
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

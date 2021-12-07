use std::{cmp::Ordering, fmt::Debug, ops::Range};
use sum_tree::Bias;
use text::{rope::TextDimension, Point, TextSummary};
use theme::SyntaxTheme;

use crate::Chunk;

pub trait DocumentSnapshot: 'static + Clone + Send + Unpin {
    type Anchor: DocumentAnchor<Snapshot = Self>;

    fn chunks<'a, T: ToDocumentOffset<Self>>(
        &'a self,
        range: Range<T>,
        theme: Option<&'a SyntaxTheme>,
    ) -> Box<dyn 'a + DocumentChunks<'a>>;
    fn text_summary(&self) -> TextSummary;
    fn text_summary_for_range<'a, D, O>(&'a self, range: Range<O>) -> D
    where
        D: TextDimension<'a>,
        O: ToDocumentOffset<Self>;
    fn len(&self) -> usize;
    fn anchor_before<T: ToDocumentOffset<Self>>(&self, position: T) -> Self::Anchor;
    fn anchor_after<T: ToDocumentOffset<Self>>(&self, position: T) -> Self::Anchor;
    fn clip_offset(&self, offset: usize, bias: Bias) -> usize;
    fn clip_point(&self, point: Point, bias: Bias) -> Point;
    fn to_offset(&self, point: Point) -> usize;
    fn to_point(&self, offset: usize) -> Point;
    fn parse_count(&self) -> usize;
    fn diagnostics_update_count(&self) -> usize;
}

pub trait ToDocumentOffset<S: DocumentSnapshot> {
    fn to_offset(&self, snapshot: &S) -> usize;
}

pub trait DocumentAnchor: Clone + Debug + Send + Sync + ToDocumentOffset<Self::Snapshot> {
    type Snapshot: DocumentSnapshot;

    fn min() -> Self;
    fn max() -> Self;
    fn cmp(&self, other: &Self, snapshot: &Self::Snapshot) -> Ordering;
}

pub trait DocumentChunks<'a>: Send + Iterator<Item = Chunk<'a>> {
    fn seek(&mut self, offset: usize);
    fn offset(&self) -> usize;
}

pub trait DocumentAnchorRangeExt<T: DocumentAnchor> {
    fn cmp(&self, b: &Range<T>, buffer: &T::Snapshot) -> Ordering;
    fn to_offset(&self, content: &T::Snapshot) -> Range<usize>;
}

impl<T: DocumentAnchor> DocumentAnchorRangeExt<T> for Range<T> {
    fn cmp(&self, other: &Range<T>, buffer: &T::Snapshot) -> Ordering {
        match self.start.cmp(&other.start, buffer) {
            Ordering::Equal => other.end.cmp(&self.end, buffer),
            ord @ _ => ord,
        }
    }

    fn to_offset(&self, content: &T::Snapshot) -> Range<usize> {
        self.start.to_offset(&content)..self.end.to_offset(&content)
    }
}

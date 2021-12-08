use crate::{rope::TextDimension, Point, PointUtf16, TextSummary};
use std::ops::Range;
use sum_tree::Bias;

pub trait Snapshot {
    fn line_len(&self, row: u32) -> u32;
    fn text_summary(&self) -> TextSummary;
    fn text_summary_for_range<'a, D, O>(&'a self, range: Range<O>) -> D
    where
        D: TextDimension,
        O: ToOffset;
    fn point_to_offset(&self, point: Point) -> usize;
    fn point_utf16_to_offset(&self, point: PointUtf16) -> usize;
    fn offset_to_point(&self, offset: usize) -> Point;
    fn clip_offset(&self, offset: usize, bias: Bias) -> usize;
    fn clip_point(&self, point: Point, bias: Bias) -> Point;
    fn clip_point_utf16(&self, point: PointUtf16, bias: Bias) -> PointUtf16;

    fn len(&self) -> usize {
        self.text_summary().bytes
    }

    fn max_point(&self) -> Point {
        self.text_summary().lines
    }
}

pub trait ToOffset: 'static {
    fn to_offset<'a, T: Snapshot>(&self, content: &T) -> usize;
}

pub trait ToPoint {
    fn to_point<'a, T: Snapshot>(&self, content: &T) -> Point;
}

impl ToOffset for Point {
    fn to_offset<'a, T: Snapshot>(&self, snapshot: &T) -> usize {
        snapshot.point_to_offset(*self)
    }
}

impl ToOffset for PointUtf16 {
    fn to_offset<'a, T: Snapshot>(&self, snapshot: &T) -> usize {
        snapshot.point_utf16_to_offset(*self)
    }
}

impl ToOffset for usize {
    fn to_offset<'a, T: Snapshot>(&self, snapshot: &T) -> usize {
        assert!(*self <= snapshot.len(), "offset is out of range");
        *self
    }
}

impl ToPoint for usize {
    fn to_point<'a, T: Snapshot>(&self, snapshot: &T) -> Point {
        snapshot.offset_to_point(*self)
    }
}

impl ToPoint for Point {
    fn to_point<'a, T: Snapshot>(&self, _: &T) -> Point {
        *self
    }
}

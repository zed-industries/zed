use crate::{
    buffer::{Point, Selection},
    traits::{Snapshot, ToOffset, ToPoint},
};
use std::ops::Range;

pub trait SelectionExt<T> {
    fn point_range<S>(&self, snapshot: &S) -> Range<Point>
    where
        S: Snapshot,
        T: ToPoint;
    fn offset_range<S>(&self, snapshot: &S) -> Range<usize>
    where
        S: Snapshot,
        T: ToOffset;
}

impl<T> SelectionExt<T> for Selection<T> {
    fn point_range<S>(&self, snapshot: &S) -> Range<Point>
    where
        S: Snapshot,
        T: ToPoint,
    {
        let start = self.start.to_point(snapshot);
        let end = self.end.to_point(snapshot);
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }

    fn offset_range<S>(&self, snapshot: &S) -> Range<usize>
    where
        S: Snapshot,
        T: ToOffset,
    {
        let start = self.start.to_offset(snapshot);
        let end = self.end.to_offset(snapshot);
        if self.reversed {
            end..start
        } else {
            start..end
        }
    }
}

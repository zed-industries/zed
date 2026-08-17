use crate::coord::ranged1d::{
    AsRangedCoord, DefaultFormatting, DiscreteRanged, KeyPointHint, Ranged,
};
use std::ops::Range;

/// A range that is defined by a slice of values.
///
/// Please note: the behavior of constructing an empty range may cause panic
#[derive(Clone)]
pub struct RangedSlice<'a, T: PartialEq>(&'a [T]);

impl<'a, T: PartialEq> Ranged for RangedSlice<'a, T> {
    type FormatOption = DefaultFormatting;
    type ValueType = &'a T;

    fn range(&self) -> Range<&'a T> {
        // If inner slice is empty, we should always panic
        &self.0[0]..&self.0[self.0.len() - 1]
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        match self.0.iter().position(|x| &x == value) {
            Some(pos) => {
                let pixel_span = limit.1 - limit.0;
                let value_span = self.0.len() - 1;
                (f64::from(limit.0)
                    + f64::from(pixel_span)
                        * (f64::from(pos as u32) / f64::from(value_span as u32)))
                .round() as i32
            }
            None => limit.0,
        }
    }

    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<Self::ValueType> {
        let max_points = hint.max_num_points();
        let mut ret = vec![];
        let intervals = (self.0.len() - 1) as f64;
        let step = (intervals / max_points as f64 + 1.0) as usize;
        for idx in (0..self.0.len()).step_by(step) {
            ret.push(&self.0[idx]);
        }
        ret
    }
}

impl<'a, T: PartialEq> DiscreteRanged for RangedSlice<'a, T> {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn index_of(&self, value: &&'a T) -> Option<usize> {
        self.0.iter().position(|x| &x == value)
    }

    fn from_index(&self, index: usize) -> Option<&'a T> {
        if self.0.len() <= index {
            return None;
        }
        Some(&self.0[index])
    }
}

impl<'a, T: PartialEq> From<&'a [T]> for RangedSlice<'a, T> {
    fn from(range: &'a [T]) -> Self {
        RangedSlice(range)
    }
}

impl<'a, T: PartialEq> AsRangedCoord for &'a [T] {
    type CoordDescType = RangedSlice<'a, T>;
    type Value = &'a T;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_slice_range() {
        let my_slice = [1, 2, 3, 0, -1, -2];
        let slice_range: RangedSlice<i32> = my_slice[..].into();

        assert_eq!(slice_range.range(), &1..&-2);
        assert_eq!(
            slice_range.key_points(6),
            my_slice.iter().collect::<Vec<_>>()
        );
        assert_eq!(slice_range.map(&&0, (0, 50)), 30);
    }

    #[test]
    fn test_slice_range_discrete() {
        let my_slice = [1, 2, 3, 0, -1, -2];
        let slice_range: RangedSlice<i32> = my_slice[..].into();

        assert_eq!(slice_range.size(), 6);
        assert_eq!(slice_range.index_of(&&3), Some(2));
        assert_eq!(slice_range.from_index(2), Some(&3));
    }
}

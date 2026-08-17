use crate::coord::ranged1d::{
    AsRangedCoord, KeyPointHint, NoDefaultFormatting, Ranged, ReversibleRanged, ValueFormatter,
};
use std::ops::Range;

/// The trait indicates the coordinate is discrete
/// This means we can bidirectionally map the range value to 0 to N
/// in which N is the number of distinct values of the range.
///
/// This is useful since for a histgoram, this is an abstraction of bucket.
pub trait DiscreteRanged
where
    Self: Ranged,
{
    /// Get the number of element in the range
    /// Note: we assume that all the ranged discrete coordinate has finite value
    ///
    /// - **returns** The number of values in the range
    fn size(&self) -> usize;

    /// Map a value to the index
    ///
    /// Note: This function doesn't guarantee return None when the value is out of range.
    /// The only way to confirm the value is in the range is to examining the return value isn't
    /// larger than self.size.
    ///
    /// - `value`: The value to map
    /// - **returns** The index of the value
    fn index_of(&self, value: &Self::ValueType) -> Option<usize>;

    /// Reverse map the index to the value
    ///
    /// Note: This function doesn't guarantee returning None when the index is out of range.
    ///
    /// - `value`: The index to map
    /// - **returns** The value
    // TODO: This doesn't follows rust's naming convention - however, this is a potential breaking
    // change, so postpone the fix to the next major release
    #[allow(clippy::wrong_self_convention)]
    fn from_index(&self, index: usize) -> Option<Self::ValueType>;

    /// Return a iterator that iterates over the all possible values
    ///
    /// - **returns** The value iterator
    fn values(&self) -> DiscreteValueIter<'_, Self>
    where
        Self: Sized,
    {
        DiscreteValueIter(self, 0, self.size())
    }

    /// Returns the previous value in this range
    ///
    /// Normally, it's based on the `from_index` and `index_of` function. But for
    /// some of the coord spec, it's possible that we value faster implementation.
    /// If this is the case, we can impelemnet the type specific impl for the `previous`
    /// and `next`.
    ///
    /// - `value`: The current value
    /// - **returns**: The value piror to current value
    fn previous(&self, value: &Self::ValueType) -> Option<Self::ValueType> {
        if let Some(idx) = self.index_of(value) {
            if idx > 0 {
                return self.from_index(idx - 1);
            }
        }
        None
    }

    /// Returns the next value in this range
    ///
    /// Normally, it's based on the `from_index` and `index_of` function. But for
    /// some of the coord spec, it's possible that we value faster implementation.
    /// If this is the case, we can impelemnet the type specific impl for the `previous`
    /// and `next`.
    ///
    /// - `value`: The current value
    /// - **returns**: The value next to current value
    fn next(&self, value: &Self::ValueType) -> Option<Self::ValueType> {
        if let Some(idx) = self.index_of(value) {
            if idx + 1 < self.size() {
                return self.from_index(idx + 1);
            }
        }
        None
    }
}

/// A `SegmentedCoord` is a decorator on any discrete coordinate specification.
/// This decorator will convert the discrete coordinate in two ways:
/// - Add an extra dummy element after all the values in original discrete coordinate
/// - Logically each value `v` from original coordinate system is mapped into an segment `[v, v+1)` where `v+1` denotes the successor of the `v`
/// - Introduce two types of values `SegmentValue::Exact(value)` which denotes the left end of value's segment and `SegmentValue::CenterOf(value)` which refers the center of the segment.
///   This is used in histogram types, which uses a discrete coordinate as the buckets.
///   The segmented coord always emits `CenterOf(value)` key points, thus it allows all the label and tick marks
///   of the coordinate rendered in the middle of each segment.
///   The corresponding trait [IntoSegmentedCoord](trait.IntoSegmentedCoord.html) is used to apply this decorator to coordinates.
#[derive(Clone)]
pub struct SegmentedCoord<D: DiscreteRanged>(D);

/// The trait for types that can decorated by [SegmentedCoord](struct.SegmentedCoord.html) decorator.
pub trait IntoSegmentedCoord: AsRangedCoord
where
    Self::CoordDescType: DiscreteRanged,
{
    /// Convert current ranged value into a segmented coordinate
    fn into_segmented(self) -> SegmentedCoord<Self::CoordDescType> {
        SegmentedCoord(self.into())
    }
}

impl<R: AsRangedCoord> IntoSegmentedCoord for R where R::CoordDescType: DiscreteRanged {}

/// The value that used by the segmented coordinate.
#[derive(Clone, Debug)]
pub enum SegmentValue<T> {
    /// Means we are referring the exact position of value `T`
    Exact(T),
    /// Means we are referring the center of position `T` and the successor of `T`
    CenterOf(T),
    /// Referring the last dummy element
    Last,
}

impl<T, D: DiscreteRanged + Ranged<ValueType = T>> ValueFormatter<SegmentValue<T>>
    for SegmentedCoord<D>
where
    D: ValueFormatter<T>,
{
    fn format(value: &SegmentValue<T>) -> String {
        match value {
            SegmentValue::Exact(ref value) => D::format(value),
            SegmentValue::CenterOf(ref value) => D::format(value),
            _ => "".to_string(),
        }
    }
}

impl<D: DiscreteRanged> Ranged for SegmentedCoord<D> {
    type FormatOption = NoDefaultFormatting;
    type ValueType = SegmentValue<D::ValueType>;

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        let margin = ((limit.1 - limit.0) as f32 / self.0.size() as f32).round() as i32;

        match value {
            SegmentValue::Exact(coord) => self.0.map(coord, (limit.0, limit.1 - margin)),
            SegmentValue::CenterOf(coord) => {
                let left = self.0.map(coord, (limit.0, limit.1 - margin));
                if let Some(idx) = self.0.index_of(coord) {
                    if idx + 1 < self.0.size() {
                        let right = self.0.map(
                            &self.0.from_index(idx + 1).unwrap(),
                            (limit.0, limit.1 - margin),
                        );
                        return (left + right) / 2;
                    }
                }
                left + margin / 2
            }
            SegmentValue::Last => limit.1,
        }
    }

    fn key_points<HintType: KeyPointHint>(&self, hint: HintType) -> Vec<Self::ValueType> {
        self.0
            .key_points(hint)
            .into_iter()
            .map(SegmentValue::CenterOf)
            .collect()
    }

    fn range(&self) -> Range<Self::ValueType> {
        let range = self.0.range();
        SegmentValue::Exact(range.start)..SegmentValue::Exact(range.end)
    }
}

impl<D: DiscreteRanged> DiscreteRanged for SegmentedCoord<D> {
    fn size(&self) -> usize {
        self.0.size() + 1
    }

    fn index_of(&self, value: &Self::ValueType) -> Option<usize> {
        match value {
            SegmentValue::Exact(value) => self.0.index_of(value),
            SegmentValue::CenterOf(value) => self.0.index_of(value),
            SegmentValue::Last => Some(self.0.size()),
        }
    }

    fn from_index(&self, idx: usize) -> Option<Self::ValueType> {
        match idx {
            idx if idx < self.0.size() => self.0.from_index(idx).map(SegmentValue::Exact),
            idx if idx == self.0.size() => Some(SegmentValue::Last),
            _ => None,
        }
    }
}

impl<T> From<T> for SegmentValue<T> {
    fn from(this: T) -> SegmentValue<T> {
        SegmentValue::Exact(this)
    }
}

impl<DC: DiscreteRanged> ReversibleRanged for DC {
    fn unmap(&self, input: i32, limit: (i32, i32)) -> Option<Self::ValueType> {
        let idx = (f64::from(input - limit.0) * (self.size() as f64) / f64::from(limit.1 - limit.0))
            .floor() as usize;
        self.from_index(idx)
    }
}

/// The iterator that can be used to iterate all the values defined by a discrete coordinate
pub struct DiscreteValueIter<'a, T: DiscreteRanged>(&'a T, usize, usize);

impl<'a, T: DiscreteRanged> Iterator for DiscreteValueIter<'a, T> {
    type Item = T::ValueType;
    fn next(&mut self) -> Option<T::ValueType> {
        if self.1 >= self.2 {
            return None;
        }
        let idx = self.1;
        self.1 += 1;
        self.0.from_index(idx)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_value_iter() {
        let range: crate::coord::ranged1d::types::RangedCoordi32 = (-10..10).into();

        let values: Vec<_> = range.values().collect();

        assert_eq!(21, values.len());

        for (expected, value) in (-10..=10).zip(values) {
            assert_eq!(expected, value);
        }
        assert_eq!(range.next(&5), Some(6));
        assert_eq!(range.next(&10), None);
        assert_eq!(range.previous(&-10), None);
        assert_eq!(range.previous(&10), Some(9));
    }

    #[test]
    fn test_centric_coord() {
        let coord = (0..10).into_segmented();

        assert_eq!(coord.size(), 12);
        for i in 0..=11 {
            match coord.from_index(i as usize) {
                Some(SegmentValue::Exact(value)) => assert_eq!(i, value),
                Some(SegmentValue::Last) => assert_eq!(i, 11),
                _ => panic!(),
            }
        }

        for (kps, idx) in coord.key_points(20).into_iter().zip(0..) {
            match kps {
                SegmentValue::CenterOf(value) if value <= 10 => assert_eq!(value, idx),
                _ => panic!(),
            }
        }

        assert_eq!(coord.map(&SegmentValue::CenterOf(0), (0, 24)), 1);
        assert_eq!(coord.map(&SegmentValue::Exact(0), (0, 24)), 0);
        assert_eq!(coord.map(&SegmentValue::Exact(1), (0, 24)), 2);
    }
}

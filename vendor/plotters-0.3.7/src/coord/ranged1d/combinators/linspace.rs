use crate::coord::ranged1d::types::RangedCoordusize;
use crate::coord::ranged1d::{
    AsRangedCoord, DiscreteRanged, KeyPointHint, NoDefaultFormatting, Ranged, ValueFormatter,
};
use std::cmp::{Ordering, PartialOrd};
use std::marker::PhantomData;
use std::ops::{Add, Range, Sub};

/// The type marker used to denote the rounding method.
/// Since we are mapping any range to a discrete range thus not all values are
/// perfect mapped to the grid points. In this case, this type marker gives hints
/// for the linspace coord for how to treat the non-grid-point values.
pub trait LinspaceRoundingMethod<V> {
    /// Search for the value within the given values array and rounding method
    ///
    /// - `values`: The values we want to search
    /// - `target`: The target value
    /// - `returns`: The index if we found the matching item, otherwise none
    fn search(values: &[V], target: &V) -> Option<usize>;
}

/// This type marker means linspace do the exact match for searching
/// which means if there's no value strictly equals to the target, the coord spec
/// reports not found result.
#[derive(Clone)]
pub struct Exact<V>(PhantomData<V>);

impl<V: PartialOrd> LinspaceRoundingMethod<V> for Exact<V> {
    fn search(values: &[V], target: &V) -> Option<usize> {
        values.iter().position(|x| target == x)
    }
}

/// This type marker means we round up the value. Which means we try to find a
/// minimal value in the values array that is greater or equal to the target.
#[derive(Clone)]
pub struct Ceil<V>(PhantomData<V>);

impl<V: PartialOrd> LinspaceRoundingMethod<V> for Ceil<V> {
    fn search(values: &[V], target: &V) -> Option<usize> {
        let ascending = if values.len() < 2 {
            true
        } else {
            values[0].partial_cmp(&values[1]) == Some(Ordering::Less)
        };

        match values.binary_search_by(|probe| {
            if ascending {
                probe.partial_cmp(target).unwrap()
            } else {
                target.partial_cmp(probe).unwrap()
            }
        }) {
            Ok(idx) => Some(idx),
            Err(idx) => {
                let offset = if ascending { 0 } else { 1 };

                if idx < offset || idx >= values.len() + offset {
                    return None;
                }
                Some(idx - offset)
            }
        }
    }
}

/// This means we use the round down. Which means we try to find a
/// maximum value in the values array that is less or equal to the target.
#[derive(Clone)]
pub struct Floor<V>(PhantomData<V>);

impl<V: PartialOrd> LinspaceRoundingMethod<V> for Floor<V> {
    fn search(values: &[V], target: &V) -> Option<usize> {
        let ascending = if values.len() < 2 {
            true
        } else {
            values[0].partial_cmp(&values[1]) == Some(Ordering::Less)
        };

        match values.binary_search_by(|probe| {
            if ascending {
                probe.partial_cmp(target).unwrap()
            } else {
                target.partial_cmp(probe).unwrap()
            }
        }) {
            Ok(idx) => Some(idx),
            Err(idx) => {
                let offset = if ascending { 1 } else { 0 };

                if idx < offset || idx >= values.len() + offset {
                    return None;
                }
                Some(idx - offset)
            }
        }
    }
}

/// This means we use the rounding. Which means we try to find the closet
/// value in the array that matches the target
#[derive(Clone)]
pub struct Round<V, S>(PhantomData<(V, S)>);

impl<V, S> LinspaceRoundingMethod<V> for Round<V, S>
where
    V: Add<S, Output = V> + PartialOrd + Sub<V, Output = S> + Clone,
    S: PartialOrd + Clone,
{
    fn search(values: &[V], target: &V) -> Option<usize> {
        let ascending = if values.len() < 2 {
            true
        } else {
            values[0].partial_cmp(&values[1]) == Some(Ordering::Less)
        };

        match values.binary_search_by(|probe| {
            if ascending {
                probe.partial_cmp(target).unwrap()
            } else {
                target.partial_cmp(probe).unwrap()
            }
        }) {
            Ok(idx) => Some(idx),
            Err(idx) => {
                if idx == 0 {
                    return Some(0);
                }

                if idx == values.len() {
                    return Some(idx - 1);
                }

                let left_delta = if ascending {
                    target.clone() - values[idx - 1].clone()
                } else {
                    values[idx - 1].clone() - target.clone()
                };
                let right_delta = if ascending {
                    values[idx].clone() - target.clone()
                } else {
                    target.clone() - values[idx].clone()
                };

                if left_delta.partial_cmp(&right_delta) == Some(Ordering::Less) {
                    Some(idx - 1)
                } else {
                    Some(idx)
                }
            }
        }
    }
}

/// The coordinate combinator that transform a continuous coordinate to a discrete coordinate
/// to a discrete coordinate by a giving step.
///
/// For example, range `0f32..100f32` is a continuous coordinate, thus this prevent us having a
/// histogram on it since Plotters doesn't know how to segment the range into buckets.
/// In this case, to get a histogram, we need to split the original range to a
/// set of discrete buckets (for example, 0.5 per bucket).
///
/// The linspace decorate abstracting this method. For example, we can have a discrete coordinate:
/// `(0f32..100f32).step(0.5)`.
///
/// Linspace also supports different types of bucket matching method - This configuration alters the behavior of
/// [DiscreteCoord::index_of](../trait.DiscreteCoord.html#tymethod.index_of) for Linspace coord spec
/// - **Flooring**, the value falls into the nearst bucket smaller than it. See [Linspace::use_floor](struct.Linspace.html#method.use_floor)
/// - **Round**,   the value falls into the nearst bucket. See [Linearspace::use_round](struct.Linspace.html#method.use_round)
/// - **Ceiling**, the value falls into the nearst bucket larger than itself. See [Linspace::use_ceil](struct.Linspace.html#method.use_ceil)
/// - **Exact Matchting**, the value must be exactly same as the butcket value.  See [Linspace::use_exact](struct.Linspace.html#method.use_exact)
#[derive(Clone)]
pub struct Linspace<T: Ranged, S: Clone, R: LinspaceRoundingMethod<T::ValueType>>
where
    T::ValueType: Add<S, Output = T::ValueType> + PartialOrd + Clone,
{
    step: S,
    inner: T,
    grid_value: Vec<T::ValueType>,
    _phatom: PhantomData<R>,
}

impl<T: Ranged, S: Clone, R: LinspaceRoundingMethod<T::ValueType>> Linspace<T, S, R>
where
    T::ValueType: Add<S, Output = T::ValueType> + PartialOrd + Clone,
{
    fn compute_grid_values(&mut self) {
        let range = self.inner.range();

        match (
            range.start.partial_cmp(&range.end),
            (range.start.clone() + self.step.clone()).partial_cmp(&range.end),
        ) {
            (Some(a), Some(b)) if a != b || a == Ordering::Equal || b == Ordering::Equal => (),
            (Some(a), Some(_)) => {
                let mut current = range.start;
                while current.partial_cmp(&range.end) == Some(a) {
                    self.grid_value.push(current.clone());
                    current = current + self.step.clone();
                }
            }
            _ => (),
        }
    }

    /// Set the linspace use the round up method for value matching
    ///
    /// - **returns**: The newly created linspace that uses new matching method
    pub fn use_ceil(self) -> Linspace<T, S, Ceil<T::ValueType>> {
        Linspace {
            step: self.step,
            inner: self.inner,
            grid_value: self.grid_value,
            _phatom: PhantomData,
        }
    }

    /// Set the linspace use the round down method for value matching
    ///
    /// - **returns**: The newly created linspace that uses new matching method
    pub fn use_floor(self) -> Linspace<T, S, Floor<T::ValueType>> {
        Linspace {
            step: self.step,
            inner: self.inner,
            grid_value: self.grid_value,
            _phatom: PhantomData,
        }
    }

    /// Set the linspace use the best match method for value matching
    ///
    /// - **returns**: The newly created linspace that uses new matching method
    pub fn use_round(self) -> Linspace<T, S, Round<T::ValueType, S>>
    where
        T::ValueType: Sub<T::ValueType, Output = S>,
        S: PartialOrd,
    {
        Linspace {
            step: self.step,
            inner: self.inner,
            grid_value: self.grid_value,
            _phatom: PhantomData,
        }
    }

    /// Set the linspace use the exact match method for value matching
    ///
    /// - **returns**: The newly created linspace that uses new matching method
    pub fn use_exact(self) -> Linspace<T, S, Exact<T::ValueType>>
    where
        T::ValueType: Sub<T::ValueType, Output = S>,
        S: PartialOrd,
    {
        Linspace {
            step: self.step,
            inner: self.inner,
            grid_value: self.grid_value,
            _phatom: PhantomData,
        }
    }
}

impl<T, R, S, RM> ValueFormatter<T> for Linspace<R, S, RM>
where
    R: Ranged<ValueType = T> + ValueFormatter<T>,
    RM: LinspaceRoundingMethod<T>,
    T: Add<S, Output = T> + PartialOrd + Clone,
    S: Clone,
{
    fn format(value: &T) -> String {
        R::format(value)
    }
}

impl<T: Ranged, S: Clone, R: LinspaceRoundingMethod<T::ValueType>> Ranged for Linspace<T, S, R>
where
    T::ValueType: Add<S, Output = T::ValueType> + PartialOrd + Clone,
{
    type FormatOption = NoDefaultFormatting;
    type ValueType = T::ValueType;

    fn range(&self) -> Range<T::ValueType> {
        self.inner.range()
    }

    fn map(&self, value: &T::ValueType, limit: (i32, i32)) -> i32 {
        self.inner.map(value, limit)
    }

    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<T::ValueType> {
        if self.grid_value.is_empty() {
            return vec![];
        }
        let idx_range: RangedCoordusize = (0..(self.grid_value.len() - 1)).into();

        idx_range
            .key_points(hint)
            .into_iter()
            .map(|x| self.grid_value[x].clone())
            .collect()
    }
}

impl<T: Ranged, S: Clone, R: LinspaceRoundingMethod<T::ValueType>> DiscreteRanged
    for Linspace<T, S, R>
where
    T::ValueType: Add<S, Output = T::ValueType> + PartialOrd + Clone,
{
    fn size(&self) -> usize {
        self.grid_value.len()
    }

    fn index_of(&self, value: &T::ValueType) -> Option<usize> {
        R::search(self.grid_value.as_ref(), value)
    }

    fn from_index(&self, idx: usize) -> Option<T::ValueType> {
        self.grid_value.get(idx).cloned()
    }
}

/// Makes a linspace coordinate from the ranged coordinates.
pub trait IntoLinspace: AsRangedCoord {
    /// Set the step value, make a linspace coordinate from the given range.
    /// By default the matching method use the exact match
    ///
    /// - `val`: The step value
    /// - **returns*: The newly created linspace
    fn step<S: Clone>(self, val: S) -> Linspace<Self::CoordDescType, S, Exact<Self::Value>>
    where
        Self::Value: Add<S, Output = Self::Value> + PartialOrd + Clone,
    {
        let mut ret = Linspace {
            step: val,
            inner: self.into(),
            grid_value: vec![],
            _phatom: PhantomData,
        };

        ret.compute_grid_values();

        ret
    }
}

impl<T: AsRangedCoord> IntoLinspace for T {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_float_linspace() {
        let coord = (0.0f64..100.0f64).step(0.1);

        assert_eq!(coord.map(&23.12, (0, 10000)), 2312);
        assert_eq!(coord.range(), 0.0..100.0);
        assert_eq!(coord.key_points(100000).len(), 1001);
        assert_eq!(coord.size(), 1001);
        assert_eq!(coord.index_of(&coord.from_index(230).unwrap()), Some(230));
        assert!((coord.from_index(230).unwrap() - 23.0).abs() < 1e-5);
    }

    #[test]
    fn test_rounding_methods() {
        let coord = (0.0f64..100.0f64).step(1.0);

        assert_eq!(coord.index_of(&1.0), Some(1));
        assert_eq!(coord.index_of(&1.2), None);

        let coord = coord.use_floor();
        assert_eq!(coord.index_of(&1.0), Some(1));
        assert_eq!(coord.index_of(&1.2), Some(1));
        assert_eq!(coord.index_of(&23.9), Some(23));
        assert_eq!(coord.index_of(&10000.0), Some(99));
        assert_eq!(coord.index_of(&-1.0), None);

        let coord = coord.use_ceil();
        assert_eq!(coord.index_of(&1.0), Some(1));
        assert_eq!(coord.index_of(&1.2), Some(2));
        assert_eq!(coord.index_of(&23.9), Some(24));
        assert_eq!(coord.index_of(&10000.0), None);
        assert_eq!(coord.index_of(&-1.0), Some(0));

        let coord = coord.use_round();
        assert_eq!(coord.index_of(&1.0), Some(1));
        assert_eq!(coord.index_of(&1.2), Some(1));
        assert_eq!(coord.index_of(&1.7), Some(2));
        assert_eq!(coord.index_of(&23.9), Some(24));
        assert_eq!(coord.index_of(&10000.0), Some(99));
        assert_eq!(coord.index_of(&-1.0), Some(0));

        let coord = (0.0f64..-100.0f64).step(-1.0);

        assert_eq!(coord.index_of(&-1.0), Some(1));
        assert_eq!(coord.index_of(&-1.2), None);

        let coord = coord.use_floor();
        assert_eq!(coord.index_of(&-1.0), Some(1));
        assert_eq!(coord.index_of(&-1.2), Some(2));
        assert_eq!(coord.index_of(&-23.9), Some(24));
        assert_eq!(coord.index_of(&-10000.0), None);
        assert_eq!(coord.index_of(&1.0), Some(0));

        let coord = coord.use_ceil();
        assert_eq!(coord.index_of(&-1.0), Some(1));
        assert_eq!(coord.index_of(&-1.2), Some(1));
        assert_eq!(coord.index_of(&-23.9), Some(23));
        assert_eq!(coord.index_of(&-10000.0), Some(99));
        assert_eq!(coord.index_of(&1.0), None);

        let coord = coord.use_round();
        assert_eq!(coord.index_of(&-1.0), Some(1));
        assert_eq!(coord.index_of(&-1.2), Some(1));
        assert_eq!(coord.index_of(&-1.7), Some(2));
        assert_eq!(coord.index_of(&-23.9), Some(24));
        assert_eq!(coord.index_of(&-10000.0), Some(99));
        assert_eq!(coord.index_of(&1.0), Some(0));
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_duration_linspace() {
        use chrono::Duration;
        let coord = (Duration::seconds(0)..Duration::seconds(100)).step(Duration::milliseconds(1));

        assert_eq!(coord.size(), 100_000);
        assert_eq!(coord.index_of(&coord.from_index(230).unwrap()), Some(230));
        assert_eq!(coord.key_points(10000000).len(), 100_000);
        assert_eq!(coord.range(), Duration::seconds(0)..Duration::seconds(100));
        assert_eq!(coord.map(&Duration::seconds(25), (0, 100_000)), 25000);
    }
}

use alloc::{collections::BTreeMap, vec::Vec};
use core::ops::{Range, RangeInclusive};

use super::{RangeInclusiveMap, RangeMap};

// A simple but infeasibly slow and memory-hungry
// version of `RangeInclusiveMap` for testing.
//
// Only understands `u32` keys, so that we don't
// have to be generic over `step`. This is just for
// testing, so it's fine.
//
// Note that even though this mirrors the semantics
// of `RangeInclusiveMap`, it can also be used to
// test `RangeMap` just fine.
#[derive(Eq, PartialEq, Debug)]
pub struct DenseU32RangeMap<V> {
    // Inner B-Tree map. Stores values and their keys
    // directly rather than as ranges.
    btm: BTreeMap<u32, V>,
}

impl<V> DenseU32RangeMap<V>
where
    V: PartialEq + Clone,
{
    pub fn new() -> DenseU32RangeMap<V> {
        DenseU32RangeMap {
            btm: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, range: RangeInclusive<u32>, value: V) {
        for k in range {
            self.btm.insert(k, value.clone());
        }
    }

    pub fn iter(&self) -> Iter<'_, V> {
        Iter {
            inner: self.btm.iter(),
            current: None,
        }
    }

    pub fn end_exclusive_iter(&self) -> EndExclusiveIter<'_, V> {
        EndExclusiveIter { inner: self.iter() }
    }

    // Vecs are easier to use for assertions than iterators,
    // because you don't have to consume them to compare them.
    pub fn to_vec(&self) -> Vec<(RangeInclusive<u32>, V)> {
        self.iter()
            .map(|(range, value)| (range, value.clone()))
            .collect()
    }

    pub fn to_end_exclusive_vec(&self) -> Vec<(Range<u32>, V)> {
        self.end_exclusive_iter()
            .map(|(range, value)| (range, value.clone()))
            .collect()
    }
}

impl<V> From<RangeMap<u32, V>> for DenseU32RangeMap<V>
where
    V: PartialEq + Clone,
{
    fn from(range_map: RangeMap<u32, V>) -> Self {
        let mut dense = Self::new();
        for (range, value) in range_map.iter() {
            // Convert to inclusive end.
            // (This is only valid because u32 has
            // the "successor function".)
            //
            // NOTE: Clippy's `range_minus_one` lint is a bit overzealous here,
            // because we _can't_ pass an open-ended range to `insert`.
            #[allow(clippy::range_minus_one)]
            dense.insert(range.start..=(range.end - 1), value.clone());
        }
        dense
    }
}

impl<V> From<RangeInclusiveMap<u32, V>> for DenseU32RangeMap<V>
where
    V: PartialEq + Clone,
{
    fn from(range_inclusive_map: RangeInclusiveMap<u32, V, u32>) -> Self {
        let mut dense = Self::new();
        for (range, value) in range_inclusive_map.iter() {
            dense.insert(range.clone(), value.clone());
        }
        dense
    }
}

pub struct Iter<'a, V> {
    inner: alloc::collections::btree_map::Iter<'a, u32, V>,
    // Current range being built for output.
    // We modify it as we iterate through the underlying
    // dense map.
    current: Option<(RangeInclusive<u32>, &'a V)>,
}

// Coalesce items from the underlying dense map as we go.
impl<'a, V> Iterator for Iter<'a, V>
where
    V: 'a + PartialEq,
{
    type Item = (RangeInclusive<u32>, &'a V);

    fn next(&mut self) -> Option<(RangeInclusive<u32>, &'a V)> {
        if let Some(next_dense) = self.inner.next() {
            if let Some(current) = &mut self.current {
                // We're already building a range. Can we extend it?
                if current.0.end() + 1 == *next_dense.0 && *current.1 == *next_dense.1 {
                    // This immediately follows the last item and the value is the same;
                    // we can extend it.
                    *current = (*current.0.start()..=*next_dense.0, current.1);
                    // Recurse until we find a way to flush it.
                    self.next()
                } else {
                    // There's a gap or the value differs; flush the one we were working on and start a new one.
                    let item_to_yield = current.clone();
                    *current = (*next_dense.0..=*next_dense.0, next_dense.1);
                    Some(item_to_yield)
                }
            } else {
                // We're not building a range yet; start a new one.
                self.current = Some((*next_dense.0..=*next_dense.0, next_dense.1));
                // Recurse until we find a way to flush it.
                self.next()
            }
        } else {
            // We reached the end of the underlying dense map.
            // Flush the item we were building, if any.
            self.current.take()
        }
    }
}

pub struct EndExclusiveIter<'a, V> {
    inner: Iter<'a, V>,
}

impl<'a, V> Iterator for EndExclusiveIter<'a, V>
where
    V: 'a + PartialEq,
{
    type Item = (Range<u32>, &'a V);

    fn next(&mut self) -> Option<(Range<u32>, &'a V)> {
        self.inner
            .next()
            .map(|(range, value)| ((*range.start())..(*range.end() + 1), value))
    }
}

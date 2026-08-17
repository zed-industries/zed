//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Strategies for generating values by taking samples of index ranges.
//!
//! Note that the strategies in this module are not native combinators; that
//! is, the input range is not itself a strategy, but is rather fixed when
//! the strategy is created.

use rand::Rng;

use core::fmt;
use core::hash::Hash;
use core::ops::Range;

use crate::bits::{BitSetLike, VarBitSet};
use crate::num::sample_uniform_incl;
use crate::sample::SizeRange;
use crate::std_facade::HashMap;
use crate::std_facade::Vec;
use crate::strategy::*;
use crate::test_runner::*;

/// Sample subsets whose size are within `size` from the given `range`.
///
/// This is roughly analogous to `rand::sample`, except that it samples _without_ replacement.
///
/// ## Panics
///
/// Panics if the maximum size implied by `size` is larger than the size of
/// `values`.
///
/// Panics if `size` is a zero-length range.
pub fn range_subset<T>(
    range: Range<T>,
    size: impl Into<SizeRange>,
) -> RangeSubset<T>
where
    T: Copy + Ord + fmt::Debug,
    Range<T>: ExactSizeIterator<Item = T>,
{
    let len = range.len();
    let size = size.into();

    size.assert_nonempty();
    assert!(
        size.end_incl() <= len,
        "Maximum size of subset {} exceeds length of input {}",
        size.end_incl(),
        len
    );
    RangeSubset { range, size }
}

/// Strategy to generate `Vec`s by sampling a subset from an index range.
///
/// This is created by the `range_subset` function in the same module.
#[derive(Debug)]
pub struct RangeSubset<T> {
    range: Range<T>,
    size: SizeRange,
}

impl<T> Strategy for RangeSubset<T>
where
    T: Copy + Eq + Hash + fmt::Debug,
    Range<T>: ExactSizeIterator<Item = T>,
{
    type Tree = RangeSubsetValueTree<T>;
    type Value = Vec<T>;

    fn new_tree(&self, runner: &mut TestRunner) -> Result<Self::Tree, Reason> {
        let (min_size, max_size) = (self.size.start(), self.size.end_incl());

        let count = sample_uniform_incl(runner, min_size, max_size);

        let range_len = self.range.len();

        let mut swaps: HashMap<T, T> = HashMap::default();

        let mut values: Vec<T> = Vec::default();

        let rng = runner.rng();

        // # Performance
        //
        // Thanks to specialization this `O(n)` access of `range.nth(…)` ends up being `O(1)`.

        // # Safety
        //
        // The offsets `i`/`j` get sampled from `0..count`/`0..range.len()`,
        // (where `0..count` is shorter, or equal in length to `0..range.len()`)
        // so unwrapping `range.nth(i).unwrap()` is safe:

        // Apply a Fisher-Yates shuffle:
        for i in 0..count {
            let j: usize = rng.random_range(i..range_len);

            let iv = self.range.clone().nth(i).unwrap();
            let vi = *swaps.get(&iv).unwrap_or(&iv);

            let jv = self.range.clone().nth(j).unwrap();
            let vj = *swaps.get(&jv).unwrap_or(&jv);

            swaps.insert(iv, vj);
            swaps.insert(jv, vi);
            values.push(vj);
        }

        let included_values = VarBitSet::saturated(count);

        Ok(RangeSubsetValueTree {
            values,
            included_values,
            shrink: 0,
            prev_shrink: None,
            min_size,
        })
    }
}

/// `RangeSubsetValueTree` corresponding to `RangeSubset`.
#[derive(Debug, Clone)]
pub struct RangeSubsetValueTree<T> {
    values: Vec<T>,
    included_values: VarBitSet,
    shrink: usize,
    prev_shrink: Option<usize>,
    min_size: usize,
}

impl<T> ValueTree for RangeSubsetValueTree<T>
where
    T: Copy + Eq + Hash + fmt::Debug,
    Range<T>: ExactSizeIterator<Item = T>,
{
    type Value = Vec<T>;

    fn current(&self) -> Self::Value {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(index, value)| {
                self.included_values.test(index).then_some(*value)
            })
            .collect()
    }

    fn simplify(&mut self) -> bool {
        if self.included_values.len() <= self.min_size {
            return false;
        }

        while self.shrink < self.values.len()
            && !self.included_values.test(self.shrink)
        {
            self.shrink += 1;
        }

        if self.shrink >= self.values.len() {
            self.prev_shrink = None;
            false
        } else {
            self.prev_shrink = Some(self.shrink);
            self.included_values.clear(self.shrink);
            self.shrink += 1;
            true
        }
    }

    fn complicate(&mut self) -> bool {
        if let Some(shrink) = self.prev_shrink.take() {
            self.included_values.set(shrink);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::std_facade::BTreeSet;

    use super::*;

    #[test]
    fn sample_range() {
        static INDICES: Range<usize> = 0..8;
        let mut size_counts: [usize; 8] = [0; 8];
        let mut value_counts: [usize; 8] = [0; 8];

        let mut runner = TestRunner::deterministic();
        let input = range_subset(INDICES.clone(), 3..7);

        for _ in 0..2048 {
            let value = input.new_tree(&mut runner).unwrap().current();
            // Generated the correct number of items
            assert!(value.len() >= 3 && value.len() < 7);
            // Chose distinct items
            assert_eq!(
                value.len(),
                value.iter().cloned().collect::<BTreeSet<_>>().len(),
                "output contains non-distinct items ({value:?})"
            );

            size_counts[value.len()] += 1;

            for value in value {
                value_counts[value] += 1;
            }
        }

        for i in 3..7 {
            assert!(
                size_counts[i] >= 256 && size_counts[i] < 1024,
                "size {} was chosen {} times",
                i,
                size_counts[i]
            );
        }

        for (ix, &v) in value_counts.iter().enumerate() {
            assert!(
                v >= 1024 && v < 1500,
                "Value {} was chosen {} times",
                ix,
                v
            );
        }
    }

    #[test]
    fn test_sample_sanity() {
        check_strategy_sanity(range_subset(0..5, 1..3), None);
    }

    #[test]
    fn subset_empty_range_works() {
        let mut runner = TestRunner::deterministic();
        let input = range_subset(0..0, 0..1);
        assert_eq!(
            Vec::<usize>::new(),
            input.new_tree(&mut runner).unwrap().current()
        );
    }

    #[test]
    fn subset_full_range_works() {
        let range = 1..4;
        let mut runner = TestRunner::deterministic();
        let input = range_subset(range.clone(), 3);
        let mut values = input.new_tree(&mut runner).unwrap().current();
        values.sort();
        assert_eq!(Vec::<usize>::from_iter(range), values);
    }
}

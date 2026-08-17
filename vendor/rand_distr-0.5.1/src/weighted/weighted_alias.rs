// Copyright 2019 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module contains an implementation of alias method for sampling random
//! indices with probabilities proportional to a collection of weights.

use super::Error;
use crate::{uniform::SampleUniform, Distribution, Uniform};
use alloc::{boxed::Box, vec, vec::Vec};
use core::fmt;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use rand::Rng;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A distribution using weighted sampling to pick a discretely selected item.
///
/// Sampling a [`WeightedAliasIndex<W>`] distribution returns the index of a randomly
/// selected element from the vector used to create the [`WeightedAliasIndex<W>`].
/// The chance of a given element being picked is proportional to the value of
/// the element. The weights can have any type `W` for which a implementation of
/// [`AliasableWeight`] exists.
///
/// # Performance
///
/// Given that `n` is the number of items in the vector used to create an
/// [`WeightedAliasIndex<W>`], it will require `O(n)` amount of memory.
/// More specifically it takes up some constant amount of memory plus
/// the vector used to create it and a [`Vec<u32>`] with capacity `n`.
///
/// Time complexity for the creation of a [`WeightedAliasIndex<W>`] is `O(n)`.
/// Sampling is `O(1)`, it makes a call to [`Uniform<u32>::sample`] and a call
/// to [`Uniform<W>::sample`].
///
/// # Example
///
/// ```
/// use rand_distr::weighted::WeightedAliasIndex;
/// use rand::prelude::*;
///
/// let choices = vec!['a', 'b', 'c'];
/// let weights = vec![2, 1, 1];
/// let dist = WeightedAliasIndex::new(weights).unwrap();
/// let mut rng = rand::rng();
/// for _ in 0..100 {
///     // 50% chance to print 'a', 25% chance to print 'b', 25% chance to print 'c'
///     println!("{}", choices[dist.sample(&mut rng)]);
/// }
///
/// let items = [('a', 0), ('b', 3), ('c', 7)];
/// let dist2 = WeightedAliasIndex::new(items.iter().map(|item| item.1).collect()).unwrap();
/// for _ in 0..100 {
///     // 0% chance to print 'a', 30% chance to print 'b', 70% chance to print 'c'
///     println!("{}", items[dist2.sample(&mut rng)].0);
/// }
/// ```
///
/// [`WeightedAliasIndex<W>`]: WeightedAliasIndex
/// [`Vec<u32>`]: Vec
/// [`Uniform<u32>::sample`]: Distribution::sample
/// [`Uniform<W>::sample`]: Distribution::sample
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(serialize = "W: Serialize, W::Sampler: Serialize"))
)]
#[cfg_attr(
    feature = "serde",
    serde(bound(deserialize = "W: Deserialize<'de>, W::Sampler: Deserialize<'de>"))
)]
pub struct WeightedAliasIndex<W: AliasableWeight> {
    aliases: Box<[u32]>,
    no_alias_odds: Box<[W]>,
    uniform_index: Uniform<u32>,
    uniform_within_weight_sum: Uniform<W>,
}

impl<W: AliasableWeight> WeightedAliasIndex<W> {
    /// Creates a new [`WeightedAliasIndex`].
    ///
    /// Error cases:
    /// -   [`Error::InvalidInput`] when `weights.len()` is zero or greater than `u32::MAX`.
    /// -   [`Error::InvalidWeight`] when a weight is not-a-number,
    ///     negative or greater than `max = W::MAX / weights.len()`.
    /// -   [`Error::InsufficientNonZero`] when the sum of all weights is zero.
    pub fn new(weights: Vec<W>) -> Result<Self, Error> {
        let n = weights.len();
        if n == 0 || n > u32::MAX as usize {
            return Err(Error::InvalidInput);
        }
        let n = n as u32;

        let max_weight_size = W::try_from_u32_lossy(n)
            .map(|n| W::MAX / n)
            .unwrap_or(W::ZERO);
        if !weights
            .iter()
            .all(|&w| W::ZERO <= w && w <= max_weight_size)
        {
            return Err(Error::InvalidWeight);
        }

        // The sum of weights will represent 100% of no alias odds.
        let weight_sum = AliasableWeight::sum(weights.as_slice());
        // Prevent floating point overflow due to rounding errors.
        let weight_sum = if weight_sum > W::MAX {
            W::MAX
        } else {
            weight_sum
        };
        if weight_sum == W::ZERO {
            return Err(Error::InsufficientNonZero);
        }

        // `weight_sum` would have been zero if `try_from_lossy` causes an error here.
        let n_converted = W::try_from_u32_lossy(n).unwrap();

        let mut no_alias_odds = weights.into_boxed_slice();
        for odds in no_alias_odds.iter_mut() {
            *odds *= n_converted;
            // Prevent floating point overflow due to rounding errors.
            *odds = if *odds > W::MAX { W::MAX } else { *odds };
        }

        /// This struct is designed to contain three data structures at once,
        /// sharing the same memory. More precisely it contains two linked lists
        /// and an alias map, which will be the output of this method. To keep
        /// the three data structures from getting in each other's way, it must
        /// be ensured that a single index is only ever in one of them at the
        /// same time.
        struct Aliases {
            aliases: Box<[u32]>,
            smalls_head: u32,
            bigs_head: u32,
        }

        impl Aliases {
            fn new(size: u32) -> Self {
                Aliases {
                    aliases: vec![0; size as usize].into_boxed_slice(),
                    smalls_head: u32::MAX,
                    bigs_head: u32::MAX,
                }
            }

            fn push_small(&mut self, idx: u32) {
                self.aliases[idx as usize] = self.smalls_head;
                self.smalls_head = idx;
            }

            fn push_big(&mut self, idx: u32) {
                self.aliases[idx as usize] = self.bigs_head;
                self.bigs_head = idx;
            }

            fn pop_small(&mut self) -> u32 {
                let popped = self.smalls_head;
                self.smalls_head = self.aliases[popped as usize];
                popped
            }

            fn pop_big(&mut self) -> u32 {
                let popped = self.bigs_head;
                self.bigs_head = self.aliases[popped as usize];
                popped
            }

            fn smalls_is_empty(&self) -> bool {
                self.smalls_head == u32::MAX
            }

            fn bigs_is_empty(&self) -> bool {
                self.bigs_head == u32::MAX
            }

            fn set_alias(&mut self, idx: u32, alias: u32) {
                self.aliases[idx as usize] = alias;
            }
        }

        let mut aliases = Aliases::new(n);

        // Split indices into those with small weights and those with big weights.
        for (index, &odds) in no_alias_odds.iter().enumerate() {
            if odds < weight_sum {
                aliases.push_small(index as u32);
            } else {
                aliases.push_big(index as u32);
            }
        }

        // Build the alias map by finding an alias with big weight for each index with
        // small weight.
        while !aliases.smalls_is_empty() && !aliases.bigs_is_empty() {
            let s = aliases.pop_small();
            let b = aliases.pop_big();

            aliases.set_alias(s, b);
            no_alias_odds[b as usize] =
                no_alias_odds[b as usize] - weight_sum + no_alias_odds[s as usize];

            if no_alias_odds[b as usize] < weight_sum {
                aliases.push_small(b);
            } else {
                aliases.push_big(b);
            }
        }

        // The remaining indices should have no alias odds of about 100%. This is due to
        // numeric accuracy. Otherwise they would be exactly 100%.
        while !aliases.smalls_is_empty() {
            no_alias_odds[aliases.pop_small() as usize] = weight_sum;
        }
        while !aliases.bigs_is_empty() {
            no_alias_odds[aliases.pop_big() as usize] = weight_sum;
        }

        // Prepare distributions for sampling. Creating them beforehand improves
        // sampling performance.
        let uniform_index = Uniform::new(0, n).unwrap();
        let uniform_within_weight_sum = Uniform::new(W::ZERO, weight_sum).unwrap();

        Ok(Self {
            aliases: aliases.aliases,
            no_alias_odds,
            uniform_index,
            uniform_within_weight_sum,
        })
    }
}

impl<W: AliasableWeight> Distribution<usize> for WeightedAliasIndex<W> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        let candidate = rng.sample(self.uniform_index);
        if rng.sample(&self.uniform_within_weight_sum) < self.no_alias_odds[candidate as usize] {
            candidate as usize
        } else {
            self.aliases[candidate as usize] as usize
        }
    }
}

impl<W: AliasableWeight> fmt::Debug for WeightedAliasIndex<W>
where
    W: fmt::Debug,
    Uniform<W>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WeightedAliasIndex")
            .field("aliases", &self.aliases)
            .field("no_alias_odds", &self.no_alias_odds)
            .field("uniform_index", &self.uniform_index)
            .field("uniform_within_weight_sum", &self.uniform_within_weight_sum)
            .finish()
    }
}

impl<W: AliasableWeight> Clone for WeightedAliasIndex<W>
where
    Uniform<W>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            aliases: self.aliases.clone(),
            no_alias_odds: self.no_alias_odds.clone(),
            uniform_index: self.uniform_index,
            uniform_within_weight_sum: self.uniform_within_weight_sum.clone(),
        }
    }
}

/// Weight bound for [`WeightedAliasIndex`]
///
/// Currently no guarantees on the correctness of [`WeightedAliasIndex`] are
/// given for custom implementations of this trait.
pub trait AliasableWeight:
    Sized
    + Copy
    + SampleUniform
    + PartialOrd
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Sum
{
    /// Maximum number representable by `Self`.
    const MAX: Self;

    /// Element of `Self` equivalent to 0.
    const ZERO: Self;

    /// Produce an instance of `Self` from a `u32` value, or return `None` if
    /// out of range. Loss of precision (where `Self` is a floating point type)
    /// is acceptable.
    fn try_from_u32_lossy(n: u32) -> Option<Self>;

    /// Sums all values in slice `values`.
    fn sum(values: &[Self]) -> Self {
        values.iter().copied().sum()
    }
}

macro_rules! impl_weight_for_float {
    ($T: ident) => {
        impl AliasableWeight for $T {
            const MAX: Self = $T::MAX;
            const ZERO: Self = 0.0;

            fn try_from_u32_lossy(n: u32) -> Option<Self> {
                Some(n as $T)
            }

            fn sum(values: &[Self]) -> Self {
                pairwise_sum(values)
            }
        }
    };
}

/// In comparison to naive accumulation, the pairwise sum algorithm reduces
/// rounding errors when there are many floating point values.
fn pairwise_sum<T: AliasableWeight>(values: &[T]) -> T {
    if values.len() <= 32 {
        values.iter().copied().sum()
    } else {
        let mid = values.len() / 2;
        let (a, b) = values.split_at(mid);
        pairwise_sum(a) + pairwise_sum(b)
    }
}

macro_rules! impl_weight_for_int {
    ($T: ident) => {
        impl AliasableWeight for $T {
            const MAX: Self = $T::MAX;
            const ZERO: Self = 0;

            fn try_from_u32_lossy(n: u32) -> Option<Self> {
                let n_converted = n as Self;
                if n_converted >= Self::ZERO && n_converted as u32 == n {
                    Some(n_converted)
                } else {
                    None
                }
            }
        }
    };
}

impl_weight_for_float!(f64);
impl_weight_for_float!(f32);
impl_weight_for_int!(usize);
impl_weight_for_int!(u128);
impl_weight_for_int!(u64);
impl_weight_for_int!(u32);
impl_weight_for_int!(u16);
impl_weight_for_int!(u8);
impl_weight_for_int!(i128);
impl_weight_for_int!(i64);
impl_weight_for_int!(i32);
impl_weight_for_int!(i16);
impl_weight_for_int!(i8);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg_attr(miri, ignore)] // Miri is too slow
    fn test_weighted_index_f32() {
        test_weighted_index(f32::into);

        // Floating point special cases
        assert_eq!(
            WeightedAliasIndex::new(vec![f32::INFINITY]).unwrap_err(),
            Error::InvalidWeight
        );
        assert_eq!(
            WeightedAliasIndex::new(vec![-0_f32]).unwrap_err(),
            Error::InsufficientNonZero
        );
        assert_eq!(
            WeightedAliasIndex::new(vec![-1_f32]).unwrap_err(),
            Error::InvalidWeight
        );
        assert_eq!(
            WeightedAliasIndex::new(vec![f32::NEG_INFINITY]).unwrap_err(),
            Error::InvalidWeight
        );
        assert_eq!(
            WeightedAliasIndex::new(vec![f32::NAN]).unwrap_err(),
            Error::InvalidWeight
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)] // Miri is too slow
    fn test_weighted_index_u128() {
        test_weighted_index(|x: u128| x as f64);
    }

    #[test]
    #[cfg_attr(miri, ignore)] // Miri is too slow
    fn test_weighted_index_i128() {
        test_weighted_index(|x: i128| x as f64);

        // Signed integer special cases
        assert_eq!(
            WeightedAliasIndex::new(vec![-1_i128]).unwrap_err(),
            Error::InvalidWeight
        );
        assert_eq!(
            WeightedAliasIndex::new(vec![i128::MIN]).unwrap_err(),
            Error::InvalidWeight
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)] // Miri is too slow
    fn test_weighted_index_u8() {
        test_weighted_index(u8::into);
    }

    #[test]
    #[cfg_attr(miri, ignore)] // Miri is too slow
    fn test_weighted_index_i8() {
        test_weighted_index(i8::into);

        // Signed integer special cases
        assert_eq!(
            WeightedAliasIndex::new(vec![-1_i8]).unwrap_err(),
            Error::InvalidWeight
        );
        assert_eq!(
            WeightedAliasIndex::new(vec![i8::MIN]).unwrap_err(),
            Error::InvalidWeight
        );
    }

    fn test_weighted_index<W: AliasableWeight, F: Fn(W) -> f64>(w_to_f64: F)
    where
        WeightedAliasIndex<W>: fmt::Debug,
    {
        const NUM_WEIGHTS: u32 = 10;
        const ZERO_WEIGHT_INDEX: u32 = 3;
        const NUM_SAMPLES: u32 = 15000;
        let mut rng = crate::test::rng(0x9c9fa0b0580a7031);

        let weights = {
            let mut weights = Vec::with_capacity(NUM_WEIGHTS as usize);
            let random_weight_distribution = Uniform::new_inclusive(
                W::ZERO,
                W::MAX / W::try_from_u32_lossy(NUM_WEIGHTS).unwrap(),
            )
            .unwrap();
            for _ in 0..NUM_WEIGHTS {
                weights.push(rng.sample(&random_weight_distribution));
            }
            weights[ZERO_WEIGHT_INDEX as usize] = W::ZERO;
            weights
        };
        let weight_sum = weights.iter().copied().sum::<W>();
        let expected_counts = weights
            .iter()
            .map(|&w| w_to_f64(w) / w_to_f64(weight_sum) * NUM_SAMPLES as f64)
            .collect::<Vec<f64>>();
        let weight_distribution = WeightedAliasIndex::new(weights).unwrap();

        let mut counts = vec![0; NUM_WEIGHTS as usize];
        for _ in 0..NUM_SAMPLES {
            counts[rng.sample(&weight_distribution)] += 1;
        }

        assert_eq!(counts[ZERO_WEIGHT_INDEX as usize], 0);
        for (count, expected_count) in counts.into_iter().zip(expected_counts) {
            let difference = (count as f64 - expected_count).abs();
            let max_allowed_difference = NUM_SAMPLES as f64 / NUM_WEIGHTS as f64 * 0.1;
            assert!(difference <= max_allowed_difference);
        }

        assert_eq!(
            WeightedAliasIndex::<W>::new(vec![]).unwrap_err(),
            Error::InvalidInput
        );
        assert_eq!(
            WeightedAliasIndex::new(vec![W::ZERO]).unwrap_err(),
            Error::InsufficientNonZero
        );
        assert_eq!(
            WeightedAliasIndex::new(vec![W::MAX, W::MAX]).unwrap_err(),
            Error::InvalidWeight
        );
    }

    #[test]
    fn value_stability() {
        fn test_samples<W: AliasableWeight>(
            weights: Vec<W>,
            buf: &mut [usize],
            expected: &[usize],
        ) {
            assert_eq!(buf.len(), expected.len());
            let distr = WeightedAliasIndex::new(weights).unwrap();
            let mut rng = crate::test::rng(0x9c9fa0b0580a7031);
            for r in buf.iter_mut() {
                *r = rng.sample(&distr);
            }
            assert_eq!(buf, expected);
        }

        let mut buf = [0; 10];
        test_samples(
            vec![1i32, 1, 1, 1, 1, 1, 1, 1, 1],
            &mut buf,
            &[6, 5, 7, 5, 8, 7, 6, 2, 3, 7],
        );
        test_samples(
            vec![0.7f32, 0.1, 0.1, 0.1],
            &mut buf,
            &[2, 0, 0, 0, 0, 0, 0, 0, 1, 3],
        );
        test_samples(
            vec![1.0f64, 0.999, 0.998, 0.997],
            &mut buf,
            &[2, 1, 2, 3, 2, 1, 3, 2, 1, 1],
        );
    }
}

//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::{Cell, Vec, VecDeque};

use rand::Rng;

use crate::num;
use crate::strategy::traits::*;
use crate::test_runner::*;

/// `Strategy` shuffle adaptor.
///
/// See `Strategy::prop_shuffle()`.
#[derive(Clone, Debug)]
#[must_use = "strategies do nothing unless used"]
pub struct Shuffle<S>(pub(super) S);

/// A value which can be used with the `prop_shuffle` combinator.
///
/// This is not a general-purpose trait. Its methods are prefixed with
/// `shuffle_` to avoid the compiler suggesting them or this trait as
/// corrections in errors.
pub trait Shuffleable {
    /// Return the length of this collection.
    fn shuffle_len(&self) -> usize;
    /// Swap the elements at the given indices.
    fn shuffle_swap(&mut self, a: usize, b: usize);
}

macro_rules! shuffleable {
    ($($t:tt)*) => {
        impl<T> Shuffleable for $($t)* {
            fn shuffle_len(&self) -> usize {
                self.len()
            }

            fn shuffle_swap(&mut self, a: usize, b: usize) {
                self.swap(a, b);
            }
        }
    }
}

shuffleable!([T]);
shuffleable!(Vec<T>);
shuffleable!(VecDeque<T>);
// Zero- and 1-length arrays aren't usefully shuffleable, but are included to
// simplify external macros that may try to use them anyway.
shuffleable!([T; 0]);
shuffleable!([T; 1]);
shuffleable!([T; 2]);
shuffleable!([T; 3]);
shuffleable!([T; 4]);
shuffleable!([T; 5]);
shuffleable!([T; 6]);
shuffleable!([T; 7]);
shuffleable!([T; 8]);
shuffleable!([T; 9]);
shuffleable!([T; 10]);
shuffleable!([T; 11]);
shuffleable!([T; 12]);
shuffleable!([T; 13]);
shuffleable!([T; 14]);
shuffleable!([T; 15]);
shuffleable!([T; 16]);
shuffleable!([T; 17]);
shuffleable!([T; 18]);
shuffleable!([T; 19]);
shuffleable!([T; 20]);
shuffleable!([T; 21]);
shuffleable!([T; 22]);
shuffleable!([T; 23]);
shuffleable!([T; 24]);
shuffleable!([T; 25]);
shuffleable!([T; 26]);
shuffleable!([T; 27]);
shuffleable!([T; 28]);
shuffleable!([T; 29]);
shuffleable!([T; 30]);
shuffleable!([T; 31]);
shuffleable!([T; 32]);

impl<S: Strategy> Strategy for Shuffle<S>
where
    S::Value: Shuffleable,
{
    type Tree = ShuffleValueTree<S::Tree>;
    type Value = S::Value;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        let rng = runner.new_rng();

        self.0.new_tree(runner).map(|inner| ShuffleValueTree {
            inner,
            rng,
            dist: Cell::new(None),
            simplifying_inner: false,
        })
    }
}

/// `ValueTree` shuffling adaptor.
///
/// See `Strategy::prop_shuffle()`.
#[derive(Clone, Debug)]
pub struct ShuffleValueTree<V> {
    inner: V,
    rng: TestRng,
    /// The maximum amount to move any one element during shuffling.
    ///
    /// This is `Cell` since we can't determine the bounds of the value until
    /// the first call to `current()`. (We technically _could_ by generating a
    /// value in `new_tree` and checking its length, but that would be a 100%
    /// slowdown.)
    dist: Cell<Option<num::usize::BinarySearch>>,
    /// Whether we've started simplifying `inner`. After this point, we can no
    /// longer simplify or complicate `dist`.
    simplifying_inner: bool,
}

impl<V: ValueTree> ShuffleValueTree<V>
where
    V::Value: Shuffleable,
{
    fn init_dist(&self, dflt: usize) -> usize {
        if self.dist.get().is_none() {
            self.dist.set(Some(num::usize::BinarySearch::new(dflt)));
        }

        self.dist.get().unwrap().current()
    }

    fn force_init_dist(&self) {
        if self.dist.get().is_none() {
            self.init_dist(self.current().shuffle_len());
        }
    }
}

impl<V: ValueTree> ValueTree for ShuffleValueTree<V>
where
    V::Value: Shuffleable,
{
    type Value = V::Value;

    fn current(&self) -> V::Value {
        let mut value = self.inner.current();
        let len = value.shuffle_len();
        // The maximum distance to swap elements. This could be larger than
        // `value` if `value` has reduced size during shrinking; that's OK,
        // since we only use this to filter swaps.
        let max_swap = self.init_dist(len);

        // If empty collection or all swaps will be filtered out, there's
        // nothing to shuffle.
        if 0 == len || 0 == max_swap {
            return value;
        }

        let mut rng = self.rng.clone();

        for start_index in 0..len - 1 {
            // Determine the other index to be swapped, then skip the swap if
            // it is too far. This ordering is critical, as it ensures that we
            // generate the same sequence of random numbers every time.
            let end_index = rng.random_range(start_index..len);
            if end_index - start_index <= max_swap {
                value.shuffle_swap(start_index, end_index);
            }
        }

        value
    }

    fn simplify(&mut self) -> bool {
        if self.simplifying_inner {
            self.inner.simplify()
        } else {
            // Ensure that we've initialised `dist` to *something* to give
            // consistent non-panicking behaviour even if called in an
            // unexpected sequence.
            self.force_init_dist();
            if self.dist.get_mut().as_mut().unwrap().simplify() {
                true
            } else {
                self.simplifying_inner = true;
                self.inner.simplify()
            }
        }
    }

    fn complicate(&mut self) -> bool {
        if self.simplifying_inner {
            self.inner.complicate()
        } else {
            self.force_init_dist();
            self.dist.get_mut().as_mut().unwrap().complicate()
        }
    }
}

#[cfg(test)]
mod test {
    use std::borrow::ToOwned;
    use std::collections::HashSet;
    use std::i32;

    use super::*;
    use crate::collection;
    use crate::strategy::just::Just;

    static VALUES: &'static [i32] = &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ];

    #[test]
    fn generates_different_permutations() {
        let mut runner = TestRunner::default();
        let mut seen = HashSet::<Vec<i32>>::new();

        let input = Just(VALUES.to_owned()).prop_shuffle();

        for _ in 0..1024 {
            let mut value = input.new_tree(&mut runner).unwrap().current();

            assert!(
                seen.insert(value.clone()),
                "Value {:?} generated more than once",
                value
            );

            value.sort();
            assert_eq!(VALUES, &value[..]);
        }
    }

    #[test]
    fn simplify_reduces_shuffle_amount() {
        let mut runner = TestRunner::default();

        let input = Just(VALUES.to_owned()).prop_shuffle();
        for _ in 0..1024 {
            let mut value = input.new_tree(&mut runner).unwrap();

            let mut prev_dist = i32::MAX;
            loop {
                let v = value.current();
                // Compute the "shuffle distance" by summing the absolute
                // distance of each element's displacement.
                let mut dist = 0;
                for (ix, &nominal) in v.iter().enumerate() {
                    dist += (nominal - ix as i32).abs();
                }

                assert!(
                    dist <= prev_dist,
                    "dist = {}, prev_dist = {}",
                    dist,
                    prev_dist
                );

                prev_dist = dist;
                if !value.simplify() {
                    break;
                }
            }

            // When fully simplified, the result is in the original order.
            assert_eq!(0, prev_dist);
        }
    }

    #[test]
    fn simplify_complicate_contract_upheld() {
        check_strategy_sanity(
            collection::vec(0i32..1000, 5..10).prop_shuffle(),
            None,
        );
    }
}

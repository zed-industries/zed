//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::{fmt, Arc, Vec};
use core::cmp::{max, min};
use core::u32;

#[cfg(not(feature = "std"))]
use num_traits::float::FloatCore;

use crate::num::sample_uniform;
use crate::strategy::{lazy::LazyValueTree, traits::*};
use crate::test_runner::*;

/// A **relative** `weight` of a particular `Strategy` corresponding to `T`
/// coupled with `T` itself. The weight is currently given in `u32`.
pub type W<T> = (u32, T);

/// A **relative** `weight` of a particular `Strategy` corresponding to `T`
/// coupled with `Arc<T>`. The weight is currently given in `u32`.
pub type WA<T> = (u32, Arc<T>);

/// A `Strategy` which picks from one of several delegate `Strategy`s.
///
/// See `Strategy::prop_union()`.
#[derive(Clone, Debug)]
#[must_use = "strategies do nothing unless used"]
pub struct Union<T: Strategy> {
    // In principle T could be any `Strategy + Clone`, but that isn't possible
    // for BC reasons with the 0.9 series.
    options: Vec<WA<T>>,
}

impl<T: Strategy> Union<T> {
    /// Create a strategy which selects uniformly from the given delegate
    /// strategies.
    ///
    /// When shrinking, after maximal simplification of the chosen element, the
    /// strategy will move to earlier options and continue simplification with
    /// those.
    ///
    /// ## Panics
    ///
    /// Panics if `options` is empty.
    pub fn new(options: impl IntoIterator<Item = T>) -> Self {
        let options: Vec<WA<T>> =
            options.into_iter().map(|v| (1, Arc::new(v))).collect();
        assert!(!options.is_empty());
        Self { options }
    }

    pub(crate) fn try_new<E>(
        it: impl Iterator<Item = Result<T, E>>,
    ) -> Result<Self, E> {
        let options: Vec<WA<T>> = it
            .map(|r| r.map(|v| (1, Arc::new(v))))
            .collect::<Result<_, _>>()?;

        assert!(!options.is_empty());
        Ok(Self { options })
    }

    /// Create a strategy which selects from the given delegate strategies.
    ///
    /// Each strategy is assigned a non-zero weight which determines how
    /// frequently that strategy is chosen. For example, a strategy with a
    /// weight of 2 will be chosen twice as frequently as one with a weight of
    /// 1\.
    ///
    /// ## Panics
    ///
    /// Panics if `options` is empty or any element has a weight of 0.
    ///
    /// Panics if the sum of the weights overflows a `u32`.
    pub fn new_weighted(options: Vec<W<T>>) -> Self {
        assert!(!options.is_empty());
        assert!(
            !options.iter().any(|&(w, _)| 0 == w),
            "Union option has a weight of 0"
        );
        assert!(
            options.iter().map(|&(w, _)| u64::from(w)).sum::<u64>()
                <= u64::from(u32::MAX),
            "Union weights overflow u32"
        );
        let options =
            options.into_iter().map(|(w, v)| (w, Arc::new(v))).collect();
        Self { options }
    }

    /// Add `other` as an additional alternate strategy with weight 1.
    pub fn or(mut self, other: T) -> Self {
        self.options.push((1, Arc::new(other)));
        self
    }
}

fn pick_weighted<I: Iterator<Item = u32>>(
    runner: &mut TestRunner,
    weights1: I,
    weights2: I,
) -> usize {
    let sum = weights1.map(u64::from).sum();
    let weighted_pick = sample_uniform(runner, 0, sum);
    weights2
        .scan(0u64, |state, w| {
            *state += u64::from(w);
            Some(*state)
        })
        .filter(|&v| v <= weighted_pick)
        .count()
}

impl<T: Strategy> Strategy for Union<T> {
    type Tree = UnionValueTree<T>;
    type Value = T::Value;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        fn extract_weight<V>(&(w, _): &WA<V>) -> u32 {
            w
        }

        let pick = pick_weighted(
            runner,
            self.options.iter().map(extract_weight::<T>),
            self.options.iter().map(extract_weight::<T>),
        );

        let mut options = Vec::with_capacity(pick);

        // Delay initialization for all options less than pick.
        for option in &self.options[0..pick] {
            options.push(LazyValueTree::new(Arc::clone(&option.1), runner));
        }

        // Initialize the tree at pick so at least one value is available. Note
        // that if generation for the value at pick fails, the entire strategy
        // will fail. This seems like the right call.
        options.push(LazyValueTree::new_initialized(
            self.options[pick].1.new_tree(runner)?,
        ));

        Ok(UnionValueTree {
            options,
            pick,
            min_pick: 0,
            prev_pick: None,
        })
    }
}

macro_rules! access_vec {
    ([$($muta:tt)*] $dst:ident = $this:expr, $ix:expr, $body:block) => {{
        let $dst = &$($muta)* $this.options[$ix];
        $body
    }}
}

/// `ValueTree` corresponding to `Union`.
pub struct UnionValueTree<T: Strategy> {
    options: Vec<LazyValueTree<T>>,
    // This struct maintains the invariant that between function calls,
    // `pick` and `prev_pick` (if Some) always point to initialized
    // trees.
    pick: usize,
    min_pick: usize,
    prev_pick: Option<usize>,
}

macro_rules! lazy_union_value_tree_body {
    ($typ:ty, $access:ident) => {
        type Value = $typ;

        fn current(&self) -> Self::Value {
            $access!([] opt = self, self.pick, {
                opt.as_inner().unwrap_or_else(||
                    panic!(
                        "value tree at self.pick = {} must be initialized",
                        self.pick,
                    )
                ).current()
            })
        }

        fn simplify(&mut self) -> bool {
            let orig_pick = self.pick;
            if $access!([mut] opt = self, orig_pick, {
                opt.as_inner_mut().unwrap_or_else(||
                    panic!(
                        "value tree at self.pick = {} must be initialized",
                        orig_pick,
                    )
                ).simplify()
            }) {
                self.prev_pick = None;
                return true;
            }

            assert!(
                self.pick >= self.min_pick,
                "self.pick = {} should never go below self.min_pick = {}",
                self.pick,
                self.min_pick,
            );
            if self.pick == self.min_pick {
                // No more simplification to be done.
                return false;
            }

            // self.prev_pick is always a valid pick.
            self.prev_pick = Some(self.pick);

            let mut next_pick = self.pick;
            while next_pick > self.min_pick {
                next_pick -= 1;
                let initialized = $access!([mut] opt = self, next_pick, {
                    opt.maybe_init();
                    opt.is_initialized()
                });
                if initialized {
                    // next_pick was correctly initialized above.
                    self.pick = next_pick;
                    return true;
                }
            }

            false
        }

        fn complicate(&mut self) -> bool {
            if let Some(pick) = self.prev_pick {
                // simplify() ensures that the previous pick was initialized.
                self.pick = pick;
                self.min_pick = pick;
                self.prev_pick = None;
                true
            } else {
                let pick = self.pick;
                $access!([mut] opt = self, pick, {
                    opt.as_inner_mut().unwrap_or_else(||
                        panic!(
                            "value tree at self.pick = {} must be initialized",
                            pick,
                        )
                    ).complicate()
                })
            }
        }
    }
}

impl<T: Strategy> ValueTree for UnionValueTree<T> {
    lazy_union_value_tree_body!(T::Value, access_vec);
}

impl<T: Strategy> Clone for UnionValueTree<T>
where
    T::Tree: Clone,
{
    fn clone(&self) -> Self {
        Self {
            options: self.options.clone(),
            pick: self.pick,
            min_pick: self.min_pick,
            prev_pick: self.prev_pick,
        }
    }
}

impl<T: Strategy> fmt::Debug for UnionValueTree<T>
where
    T::Tree: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnionValueTree")
            .field("options", &self.options)
            .field("pick", &self.pick)
            .field("min_pick", &self.min_pick)
            .field("prev_pick", &self.prev_pick)
            .finish()
    }
}

macro_rules! def_access_tuple {
    ($b:tt $name:ident, $($n:tt)*) => {
        macro_rules! $name {
            ([$b($b muta:tt)*] $b dst:ident = $b this:expr,
             $b ix:expr, $b body:block) => {
                match $b ix {
                    0 => {
                        let $b dst = &$b($b muta)* $b this.options.0;
                        $b body
                    },
                    $(
                        $n => {
                            if let Some(ref $b($b muta)* $b dst) =
                                $b this.options.$n
                            {
                                $b body
                            } else {
                                panic!("TupleUnion tried to access \
                                        uninitialised slot {}", $n)
                            }
                        },
                    )*
                    _ => panic!("TupleUnion tried to access out-of-range \
                                 slot {}", $b ix),
                }
            }
        }
    }
}

def_access_tuple!($ access_tuple2, 1);
def_access_tuple!($ access_tuple3, 1 2);
def_access_tuple!($ access_tuple4, 1 2 3);
def_access_tuple!($ access_tuple5, 1 2 3 4);
def_access_tuple!($ access_tuple6, 1 2 3 4 5);
def_access_tuple!($ access_tuple7, 1 2 3 4 5 6);
def_access_tuple!($ access_tuple8, 1 2 3 4 5 6 7);
def_access_tuple!($ access_tuple9, 1 2 3 4 5 6 7 8);
def_access_tuple!($ access_tupleA, 1 2 3 4 5 6 7 8 9);

/// Similar to `Union`, but internally uses a tuple to hold the strategies.
///
/// This allows better performance than vanilla `Union` since one does not need
/// to resort to boxing and dynamic dispatch to handle heterogeneous
/// strategies.
///
/// The difference between this and `TupleUnion` is that with this, value trees
/// for variants that aren't picked at first are generated lazily.
#[must_use = "strategies do nothing unless used"]
#[derive(Clone, Copy, Debug)]
pub struct TupleUnion<T>(T);

impl<T> TupleUnion<T> {
    /// Wrap `tuple` in a `TupleUnion`.
    ///
    /// The struct definition allows any `T` for `tuple`, but to be useful, it
    /// must be a 2- to 10-tuple of `(u32, Arc<impl Strategy>)` pairs where all
    /// strategies ultimately produce the same value. Each `u32` indicates the
    /// relative weight of its corresponding strategy.
    /// You may use `WA<S>` as an alias for `(u32, Arc<S>)`.
    ///
    /// Using this constructor directly is discouraged; prefer to use
    /// `prop_oneof!` since it is generally clearer.
    pub fn new(tuple: T) -> Self {
        TupleUnion(tuple)
    }
}

macro_rules! tuple_union {
    ($($gen:ident $ix:tt)*) => {
        impl<A : Strategy, $($gen: Strategy<Value = A::Value>),*>
        Strategy for TupleUnion<(WA<A>, $(WA<$gen>),*)> {
            type Tree = TupleUnionValueTree<
                (LazyValueTree<A>, $(Option<LazyValueTree<$gen>>),*)>;
            type Value = A::Value;

            fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                let weights = [((self.0).0).0, $(((self.0).$ix).0),*];
                let pick = pick_weighted(runner, weights.iter().cloned(),
                                         weights.iter().cloned());

                Ok(TupleUnionValueTree {
                    options: (
                        if 0 == pick {
                            LazyValueTree::new_initialized(
                                ((self.0).0).1.new_tree(runner)?)
                        } else {
                            LazyValueTree::new(
                                Arc::clone(&((self.0).0).1), runner)
                        },
                        $(
                        if $ix == pick {
                            Some(LazyValueTree::new_initialized(
                                 ((self.0).$ix).1.new_tree(runner)?))
                        } else if $ix < pick {
                            Some(LazyValueTree::new(
                                    Arc::clone(&((self.0).$ix).1), runner))
                        } else {
                            None
                        }),*),
                    pick: pick,
                    min_pick: 0,
                    prev_pick: None,
                })
            }
        }
    }
}

tuple_union!(B 1);
tuple_union!(B 1 C 2);
tuple_union!(B 1 C 2 D 3);
tuple_union!(B 1 C 2 D 3 E 4);
tuple_union!(B 1 C 2 D 3 E 4 F 5);
tuple_union!(B 1 C 2 D 3 E 4 F 5 G 6);
tuple_union!(B 1 C 2 D 3 E 4 F 5 G 6 H 7);
tuple_union!(B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8);
tuple_union!(B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9);

/// `ValueTree` type produced by `TupleUnion`.
#[derive(Clone, Copy, Debug)]
pub struct TupleUnionValueTree<T> {
    options: T,
    pick: usize,
    min_pick: usize,
    prev_pick: Option<usize>,
}

macro_rules! value_tree_tuple {
    ($access:ident, $($gen:ident)*) => {
        impl<A : Strategy, $($gen: Strategy<Value = A::Value>),*> ValueTree
        for TupleUnionValueTree<
            (LazyValueTree<A>, $(Option<LazyValueTree<$gen>>),*)
        > {
            lazy_union_value_tree_body!(A::Value, $access);
        }
    }
}

value_tree_tuple!(access_tuple2, B);
value_tree_tuple!(access_tuple3, B C);
value_tree_tuple!(access_tuple4, B C D);
value_tree_tuple!(access_tuple5, B C D E);
value_tree_tuple!(access_tuple6, B C D E F);
value_tree_tuple!(access_tuple7, B C D E F G);
value_tree_tuple!(access_tuple8, B C D E F G H);
value_tree_tuple!(access_tuple9, B C D E F G H I);
value_tree_tuple!(access_tupleA, B C D E F G H I J);

const WEIGHT_BASE: u32 = 0x8000_0000;

/// Convert a floating-point weight in the range (0.0,1.0) to a pair of weights
/// that can be used with `Union` and similar.
///
/// The first return value is the weight corresponding to `f`; the second
/// return value is the weight corresponding to `1.0 - f`.
///
/// This call does not make any guarantees as to what range of weights it may
/// produce, except that adding the two return values will never overflow a
/// `u32`. As such, it is generally not meaningful to combine any other weights
/// with the two returned.
///
/// ## Panics
///
/// Panics if `f` is not a real number between 0.0 and 1.0, both exclusive.
pub fn float_to_weight(f: f64) -> (u32, u32) {
    assert!(f > 0.0 && f < 1.0, "Invalid probability: {}", f);

    // Clamp to 1..WEIGHT_BASE-1 so that we never produce a weight of 0.
    let pos = max(
        1,
        min(WEIGHT_BASE - 1, (f * f64::from(WEIGHT_BASE)).round() as u32),
    );
    let neg = WEIGHT_BASE - pos;

    (pos, neg)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::strategy::just::Just;

    // FIXME(2018-06-01): figure out a way to run this test on no_std.
    // The problem is that the default seed is fixed and does not produce
    // enough passed tests. We need some universal source of non-determinism
    // for the seed, which is unlikely.
    #[cfg(feature = "std")]
    #[test]
    fn test_union() {
        let input = (10u32..20u32).prop_union(30u32..40u32);
        // Expect that 25% of cases pass (left input happens to be < 15, and
        // left is chosen as initial value). Of the 75% that fail, 50% should
        // converge to 15 and 50% to 30 (the latter because the left is beneath
        // the passing threshold).
        let mut passed = 0;
        let mut converged_low = 0;
        let mut converged_high = 0;
        let mut runner = TestRunner::deterministic();
        for _ in 0..256 {
            let case = input.new_tree(&mut runner).unwrap();
            let result = runner.run_one(case, |v| {
                prop_assert!(v < 15);
                Ok(())
            });

            match result {
                Ok(true) => passed += 1,
                Err(TestError::Fail(_, 15)) => converged_low += 1,
                Err(TestError::Fail(_, 30)) => converged_high += 1,
                e => panic!("Unexpected result: {:?}", e),
            }
        }

        assert!(passed >= 32 && passed <= 96, "Bad passed count: {}", passed);
        assert!(
            converged_low >= 32 && converged_low <= 160,
            "Bad converged_low count: {}",
            converged_low
        );
        assert!(
            converged_high >= 32 && converged_high <= 160,
            "Bad converged_high count: {}",
            converged_high
        );
    }

    #[test]
    fn test_union_weighted() {
        let input = Union::new_weighted(vec![
            (1, Just(0usize)),
            (2, Just(1usize)),
            (1, Just(2usize)),
        ]);

        let mut counts = [0, 0, 0];
        let mut runner = TestRunner::deterministic();
        for _ in 0..65536 {
            counts[input.new_tree(&mut runner).unwrap().current()] += 1;
        }

        println!("{:?}", counts);
        assert!(counts[0] > 0);
        assert!(counts[2] > 0);
        assert!(counts[1] > counts[0] * 3 / 2);
        assert!(counts[1] > counts[2] * 3 / 2);
    }

    #[test]
    fn test_union_sanity() {
        check_strategy_sanity(
            Union::new_weighted(vec![
                (1, 0i32..100),
                (2, 200i32..300),
                (1, 400i32..500),
            ]),
            None,
        );
    }

    // FIXME(2018-06-01): See note on `test_union`.
    #[cfg(feature = "std")]
    #[test]
    fn test_tuple_union() {
        let input = TupleUnion::new((
            (1, Arc::new(10u32..20u32)),
            (1, Arc::new(30u32..40u32)),
        ));
        // Expect that 25% of cases pass (left input happens to be < 15, and
        // left is chosen as initial value). Of the 75% that fail, 50% should
        // converge to 15 and 50% to 30 (the latter because the left is beneath
        // the passing threshold).
        let mut passed = 0;
        let mut converged_low = 0;
        let mut converged_high = 0;
        let mut runner = TestRunner::deterministic();
        for _ in 0..256 {
            let case = input.new_tree(&mut runner).unwrap();
            let result = runner.run_one(case, |v| {
                prop_assert!(v < 15);
                Ok(())
            });

            match result {
                Ok(true) => passed += 1,
                Err(TestError::Fail(_, 15)) => converged_low += 1,
                Err(TestError::Fail(_, 30)) => converged_high += 1,
                e => panic!("Unexpected result: {:?}", e),
            }
        }

        assert!(passed >= 32 && passed <= 96, "Bad passed count: {}", passed);
        assert!(
            converged_low >= 32 && converged_low <= 160,
            "Bad converged_low count: {}",
            converged_low
        );
        assert!(
            converged_high >= 32 && converged_high <= 160,
            "Bad converged_high count: {}",
            converged_high
        );
    }

    #[test]
    fn test_tuple_union_weighting() {
        let input = TupleUnion::new((
            (1, Arc::new(Just(0usize))),
            (2, Arc::new(Just(1usize))),
            (1, Arc::new(Just(2usize))),
        ));

        let mut counts = [0, 0, 0];
        let mut runner = TestRunner::deterministic();
        for _ in 0..65536 {
            counts[input.new_tree(&mut runner).unwrap().current()] += 1;
        }

        println!("{:?}", counts);
        assert!(counts[0] > 0);
        assert!(counts[2] > 0);
        assert!(counts[1] > counts[0] * 3 / 2);
        assert!(counts[1] > counts[2] * 3 / 2);
    }

    #[test]
    fn test_tuple_union_all_sizes() {
        let mut runner = TestRunner::deterministic();
        let r = Arc::new(1i32..10);

        macro_rules! test {
            ($($part:expr),*) => {{
                let input = TupleUnion::new((
                    $((1, $part.clone())),*,
                    (1, Arc::new(Just(0i32)))
                ));

                let mut pass = false;
                for _ in 0..1024 {
                    if 0 == input.new_tree(&mut runner).unwrap().current() {
                        pass = true;
                        break;
                    }
                }

                assert!(pass);
            }}
        }

        test!(r); // 2
        test!(r, r); // 3
        test!(r, r, r); // 4
        test!(r, r, r, r); // 5
        test!(r, r, r, r, r); // 6
        test!(r, r, r, r, r, r); // 7
        test!(r, r, r, r, r, r, r); // 8
        test!(r, r, r, r, r, r, r, r); // 9
        test!(r, r, r, r, r, r, r, r, r); // 10
    }

    #[test]
    fn test_tuple_union_sanity() {
        check_strategy_sanity(
            TupleUnion::new((
                (1, Arc::new(0i32..100i32)),
                (1, Arc::new(200i32..1000i32)),
                (1, Arc::new(2000i32..3000i32)),
            )),
            None,
        );
    }

    /// Test that unions work even if local filtering causes errors.
    #[test]
    fn test_filter_union_sanity() {
        let filter_strategy = (0u32..256).prop_filter("!%5", |&v| 0 != v % 5);
        check_strategy_sanity(
            Union::new(vec![filter_strategy; 8]),
            Some(filter_sanity_options()),
        );
    }

    /// Test that tuple unions work even if local filtering causes errors.
    #[test]
    fn test_filter_tuple_union_sanity() {
        let filter_strategy = (0u32..256).prop_filter("!%5", |&v| 0 != v % 5);
        check_strategy_sanity(
            TupleUnion::new((
                (1, Arc::new(filter_strategy.clone())),
                (1, Arc::new(filter_strategy.clone())),
                (1, Arc::new(filter_strategy.clone())),
                (1, Arc::new(filter_strategy.clone())),
            )),
            Some(filter_sanity_options()),
        );
    }

    fn filter_sanity_options() -> CheckStrategySanityOptions {
        CheckStrategySanityOptions {
            // Due to internal rejection sampling, `simplify()` can
            // converge back to what `complicate()` would do.
            strict_complicate_after_simplify: false,
            // Make failed filters return errors to test edge cases.
            error_on_local_rejects: true,
            ..CheckStrategySanityOptions::default()
        }
    }
}

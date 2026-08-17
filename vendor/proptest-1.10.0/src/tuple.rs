//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Support for combining strategies into tuples.
//!
//! There is no explicit "tuple strategy"; simply make a tuple containing the
//! strategy and that tuple is itself a strategy.

use crate::strategy::*;
use crate::test_runner::*;

/// Common `ValueTree` implementation for all tuple strategies.
#[derive(Clone, Copy, Debug)]
pub struct TupleValueTree<T> {
    tree: T,
    shrinker: u32,
    prev_shrinker: Option<u32>,
}

impl<T> TupleValueTree<T> {
    /// Create a new `TupleValueTree` wrapping `inner`.
    ///
    /// It only makes sense for `inner` to be a tuple of an arity for which the
    /// type implements `ValueTree`.
    pub fn new(inner: T) -> Self {
        TupleValueTree {
            tree: inner,
            shrinker: 0,
            prev_shrinker: None,
        }
    }
}

macro_rules! tuple {
    ($($fld:tt : $typ:ident),*) => {
        impl<$($typ : Strategy),*> Strategy for ($($typ,)*) {
            type Tree = TupleValueTree<($($typ::Tree,)*)>;
            type Value = ($($typ::Value,)*);

            fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                let values = ($(self.$fld.new_tree(runner)?,)*);
                Ok(TupleValueTree::new(values))
            }
        }

        impl<$($typ : ValueTree),*> ValueTree
        for TupleValueTree<($($typ,)*)> {
            type Value = ($($typ::Value,)*);

            fn current(&self) -> Self::Value {
                ($(self.tree.$fld.current(),)*)
            }

            fn simplify(&mut self) -> bool {
                $(
                    if $fld == self.shrinker {
                        if self.tree.$fld.simplify() {
                            self.prev_shrinker = Some(self.shrinker);
                            return true;
                        } else {
                            self.shrinker += 1;
                        }
                    }
                )*
                false
            }

            fn complicate(&mut self) -> bool {
                if let Some(shrinker) = self.prev_shrinker {$(
                    if $fld == shrinker {
                        if self.tree.$fld.complicate() {
                            self.shrinker = shrinker;
                            return true;
                        } else {
                            self.prev_shrinker = None;
                            return false;
                        }
                    }
                )*}
                false
            }
        }
    }
}

tuple!(0: A);
tuple!(0: A, 1: B);
tuple!(0: A, 1: B, 2: C);
tuple!(0: A, 1: B, 2: C, 3: D);
tuple!(0: A, 1: B, 2: C, 3: D, 4: E);
tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);
tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H);
tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I);
tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J);
tuple!(
    0: A,
    1: B,
    2: C,
    3: D,
    4: E,
    5: F,
    6: G,
    7: H,
    8: I,
    9: J,
    10: K
);
tuple!(
    0: A,
    1: B,
    2: C,
    3: D,
    4: E,
    5: F,
    6: G,
    7: H,
    8: I,
    9: J,
    10: K,
    11: L
);

#[cfg(test)]
mod test {
    use crate::strategy::*;

    use super::*;

    #[test]
    fn shrinks_fully_ltr() {
        fn pass(a: (i32, i32)) -> bool {
            a.0 * a.1 <= 9
        }

        let input = (0..32, 0..32);
        let mut runner = TestRunner::default();

        let mut cases_tested = 0;
        for _ in 0..256 {
            // Find a failing test case
            let mut case = input.new_tree(&mut runner).unwrap();
            if pass(case.current()) {
                continue;
            }

            loop {
                if pass(case.current()) {
                    if !case.complicate() {
                        break;
                    }
                } else {
                    if !case.simplify() {
                        break;
                    }
                }
            }

            let last = case.current();
            assert!(!pass(last));
            // Maximally shrunken
            assert!(pass((last.0 - 1, last.1)));
            assert!(pass((last.0, last.1 - 1)));

            cases_tested += 1;
        }

        assert!(cases_tested > 32, "Didn't find enough test cases");
    }

    #[test]
    fn test_sanity() {
        check_strategy_sanity((0i32..100, 0i32..1000, 0i32..10000), None);
    }
}

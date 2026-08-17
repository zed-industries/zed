//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Modified versions of the normal strategy combinators which take specialised
//! traits instead of normal functions.
//!
//! This entire module is strictly a workaround until
//! <https://github.com/rust-lang/rfcs/pull/1522> and
//! <https://github.com/rust-lang/rfcs/pull/2071> are available in stable. It
//! allows naming types built on the combinators without resorting to dynamic
//! dispatch or causing `Arc` to allocate space for a function pointer.
//!
//! External code is discouraged from using this module directly. It is
//! deliberately not exposed in a convenient way (i.e., via the `Strategy`
//! trait itself), but is nonetheless exposed since external trait implementors
//! may face the same issues.
//!
//! **This module is subject to removal at some point after the language
//! features linked above become stable.**

use crate::std_facade::fmt;

use crate::strategy::traits::*;
use crate::test_runner::*;

//==============================================================================
// Filter
//==============================================================================

/// Essentially `Fn (&T) -> bool`.
pub trait FilterFn<T> {
    /// Test whether `t` passes the filter.
    fn apply(&self, t: &T) -> bool;
}

/// Static version of `strategy::Filter`.
#[derive(Clone)]
#[must_use = "strategies do nothing unless used"]
pub struct Filter<S, F> {
    source: S,
    whence: Reason,
    fun: F,
}

impl<S, F> Filter<S, F> {
    /// Adapt strategy `source` to reject values which do not pass `filter`,
    /// using `whence` as the reported reason/location.
    pub fn new(source: S, whence: Reason, filter: F) -> Self {
        // NOTE: We don't use universal quantification R: Into<Reason>
        // since the module is not conveniently exposed.
        Filter {
            source,
            whence,
            fun: filter,
        }
    }
}

impl<S: fmt::Debug, F> fmt::Debug for Filter<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Filter")
            .field("source", &self.source)
            .field("whence", &self.whence)
            .field("fun", &"<function>")
            .finish()
    }
}

impl<S: Strategy, F: FilterFn<S::Value> + Clone> Strategy for Filter<S, F> {
    type Tree = Filter<S::Tree, F>;
    type Value = S::Value;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        loop {
            let val = self.source.new_tree(runner)?;
            if !self.fun.apply(&val.current()) {
                runner.reject_local(self.whence.clone())?;
            } else {
                return Ok(Filter {
                    source: val,
                    whence: "unused".into(),
                    fun: self.fun.clone(),
                });
            }
        }
    }
}

impl<S: ValueTree, F: FilterFn<S::Value>> Filter<S, F> {
    fn ensure_acceptable(&mut self) {
        while !self.fun.apply(&self.source.current()) {
            if !self.source.complicate() {
                panic!(
                    "Unable to complicate filtered strategy \
                     back into acceptable value"
                );
            }
        }
    }
}

impl<S: ValueTree, F: FilterFn<S::Value>> ValueTree for Filter<S, F> {
    type Value = S::Value;

    fn current(&self) -> S::Value {
        self.source.current()
    }

    fn simplify(&mut self) -> bool {
        if self.source.simplify() {
            self.ensure_acceptable();
            true
        } else {
            false
        }
    }

    fn complicate(&mut self) -> bool {
        if self.source.complicate() {
            self.ensure_acceptable();
            true
        } else {
            false
        }
    }
}

//==============================================================================
// Map
//==============================================================================

/// Essentially `Fn (T) -> Output`.
pub trait MapFn<T> {
    #[allow(missing_docs)]
    type Output: fmt::Debug;

    /// Map `T` to `Output`.
    fn apply(&self, t: T) -> Self::Output;
}

/// Static version of `strategy::Map`.
#[derive(Clone)]
#[must_use = "strategies do nothing unless used"]
pub struct Map<S, F> {
    source: S,
    fun: F,
}

impl<S, F> Map<S, F> {
    /// Adapt strategy `source` by applying `fun` to values it produces.
    pub fn new(source: S, fun: F) -> Self {
        Map { source, fun }
    }
}

impl<S: fmt::Debug, F> fmt::Debug for Map<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Map")
            .field("source", &self.source)
            .field("fun", &"<function>")
            .finish()
    }
}

impl<S: Strategy, F: Clone + MapFn<S::Value>> Strategy for Map<S, F> {
    type Tree = Map<S::Tree, F>;
    type Value = F::Output;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        self.source.new_tree(runner).map(|v| Map {
            source: v,
            fun: self.fun.clone(),
        })
    }
}

impl<S: ValueTree, F: MapFn<S::Value>> ValueTree for Map<S, F> {
    type Value = F::Output;

    fn current(&self) -> F::Output {
        self.fun.apply(self.source.current())
    }

    fn simplify(&mut self) -> bool {
        self.source.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.source.complicate()
    }
}

impl<I, O: fmt::Debug> MapFn<I> for fn(I) -> O {
    type Output = O;
    fn apply(&self, x: I) -> Self::Output {
        self(x)
    }
}

pub(crate) fn static_map<S: Strategy, O: fmt::Debug>(
    strat: S,
    fun: fn(S::Value) -> O,
) -> Map<S, fn(S::Value) -> O> {
    Map::new(strat, fun)
}

//==============================================================================
// Tests
//==============================================================================

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_static_filter() {
        #[derive(Clone, Copy, Debug)]
        struct MyFilter;
        impl FilterFn<i32> for MyFilter {
            fn apply(&self, &v: &i32) -> bool {
                0 == v % 3
            }
        }

        let input = Filter::new(0..256, "%3".into(), MyFilter);

        for _ in 0..256 {
            let mut runner = TestRunner::default();
            let mut case = input.new_tree(&mut runner).unwrap();

            assert!(0 == case.current() % 3);

            while case.simplify() {
                assert!(0 == case.current() % 3);
            }
            assert!(0 == case.current() % 3);
        }
    }

    #[test]
    fn test_static_map() {
        #[derive(Clone, Copy, Debug)]
        struct MyMap;
        impl MapFn<i32> for MyMap {
            type Output = i32;
            fn apply(&self, v: i32) -> i32 {
                v * 2
            }
        }

        let input = Map::new(0..10, MyMap);

        TestRunner::default()
            .run(&input, |v| {
                assert!(0 == v % 2);
                Ok(())
            })
            .unwrap();
    }
}

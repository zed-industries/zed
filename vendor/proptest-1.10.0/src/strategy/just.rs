//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::fmt;

use crate::strategy::{NewTree, Strategy, ValueTree};
use crate::test_runner::TestRunner;

macro_rules! noshrink {
    () => {
        fn simplify(&mut self) -> bool {
            false
        }
        fn complicate(&mut self) -> bool {
            false
        }
    };
}

//==============================================================================
// Just
//==============================================================================

/// A `Strategy` which always produces a single value and never
/// simplifies.
#[derive(Clone, Copy, Debug)]
#[must_use = "strategies do nothing unless used"]
pub struct Just<T: Clone + fmt::Debug>(
    /// The value produced by this strategy.
    pub T,
);

impl<T: Clone + fmt::Debug> Strategy for Just<T> {
    type Tree = Self;
    type Value = T;

    fn new_tree(&self, _: &mut TestRunner) -> NewTree<Self> {
        Ok(self.clone())
    }
}

impl<T: Clone + fmt::Debug> ValueTree for Just<T> {
    type Value = T;
    noshrink!();
    fn current(&self) -> T {
        self.0.clone()
    }
}

//==============================================================================
// LazyJust
//==============================================================================

/// A `Strategy` which always produces a single value and never
/// simplifies. If `T` is `Clone`, you should use `Just` instead.
///
/// This is a generalization of `Just` and works by calling
/// the provided `Fn () -> T` in `.current()` every time. This is not a
/// very interesting strategy, but is required in cases where `T` is
/// not `Clone`. It is also used in `proptest_derive` where we can't
/// assume that your type is `Clone`.
///
/// **It is important that the function used be pure.**
#[must_use = "strategies do nothing unless used"]
pub struct LazyJust<T, F: Fn() -> T> {
    /// The function executed in `.current()`.
    function: F,
}

/// Shorthand for `LazyJust<T, fn () -> T>`.
pub type LazyJustFn<V> = LazyJust<V, fn() -> V>;

impl<T, F: Fn() -> T> LazyJust<T, F> {
    /// Constructs a `LazyJust` strategy given the function/closure
    /// that produces the value.
    ///
    /// **It is important that the function used be pure.**
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<T: fmt::Debug, F: Clone + Fn() -> T> Strategy for LazyJust<T, F> {
    type Tree = Self;
    type Value = T;

    fn new_tree(&self, _: &mut TestRunner) -> NewTree<Self> {
        Ok(self.clone())
    }
}

impl<T: fmt::Debug, F: Fn() -> T> ValueTree for LazyJust<T, F> {
    type Value = T;
    noshrink!();
    fn current(&self) -> Self::Value {
        (self.function)()
    }
}

impl<T, F: Copy + Fn() -> T> Copy for LazyJust<T, F> {}

impl<T, F: Clone + Fn() -> T> Clone for LazyJust<T, F> {
    fn clone(&self) -> Self {
        Self {
            function: self.function.clone(),
        }
    }
}

impl<T, F: Fn() -> T> fmt::Debug for LazyJust<T, F> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("LazyJust")
            .field("function", &"<function>")
            .finish()
    }
}

//==============================================================================
// Any `fn () -> T` is a Strategy
//==============================================================================

// TODO: try 'F: Fn () -> T' instead when we've got specialization.

impl<T: fmt::Debug> Strategy for fn() -> T {
    type Tree = Self;
    type Value = T;

    fn new_tree(&self, _: &mut TestRunner) -> NewTree<Self> {
        Ok(*self)
    }
}

impl<T: fmt::Debug> ValueTree for fn() -> T {
    type Value = T;
    noshrink!();
    fn current(&self) -> Self::Value {
        self()
    }
}

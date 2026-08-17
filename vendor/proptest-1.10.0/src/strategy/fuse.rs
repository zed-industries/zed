//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::strategy::*;
use crate::test_runner::*;

/// Adaptor for `Strategy` and `ValueTree` which guards `simplify()` and
/// `complicate()` to avoid contract violations.
///
/// This can be used as an intermediate when the caller would otherwise need
/// its own separate state tracking, or as a workaround for a broken
/// `ValueTree` implementation.
///
/// This wrapper specifically has the following effects:
///
/// - Calling `complicate()` before `simplify()` was ever called does nothing
///   and returns `false`.
///
/// - Calling `simplify()` after it has returned `false` and no calls to
///   `complicate()` returned `true` does nothing and returns `false`.
///
/// - Calling `complicate()` after it has returned `false` and no calls to
///   `simplify()` returned `true` does nothing and returns `false`.
///
/// There is also limited functionality to alter the internal state to assist
/// in its usage as a state tracker.
///
/// Wrapping a `Strategy` in `Fuse` simply causes its `ValueTree` to also be
/// wrapped in `Fuse`.
///
/// While this is similar to `std::iter::Fuse`, it is not exposed as a method
/// on `Strategy` since the vast majority of proptest should never need this
/// functionality; it mainly concerns implementors of strategies.
#[derive(Debug, Clone, Copy)]
#[must_use = "strategies do nothing unless used"]
pub struct Fuse<T> {
    inner: T,
    may_simplify: bool,
    may_complicate: bool,
}

impl<T> Fuse<T> {
    /// Wrap the given `T` in `Fuse`.
    pub fn new(inner: T) -> Self {
        Fuse {
            inner,
            may_simplify: true,
            may_complicate: false,
        }
    }
}

impl<T: Strategy> Strategy for Fuse<T> {
    type Tree = Fuse<T::Tree>;
    type Value = T::Value;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        self.inner.new_tree(runner).map(Fuse::new)
    }
}

impl<T: ValueTree> Fuse<T> {
    /// Return whether a call to `simplify()` may be productive.
    ///
    /// Formally, this is true if one of the following holds:
    ///
    /// - `simplify()` has never been called.
    /// - The most recent call to `simplify()` returned `true`.
    /// - `complicate()` has been called more recently than `simplify()` and
    ///   the last call returned `true`.
    pub fn may_simplify(&self) -> bool {
        self.may_simplify
    }

    /// Disallow any further calls to `simplify()` until a call to
    /// `complicate()` returns `true`.
    pub fn disallow_simplify(&mut self) {
        self.may_simplify = false;
    }

    /// Return whether a call to `complicate()` may be productive.
    ///
    /// Formally, this is true if one of the following holds:
    ///
    /// - The most recent call to `complicate()` returned `true`.
    /// - `simplify()` has been called more recently than `complicate()` and
    ///   the last call returned `true`.
    pub fn may_complicate(&self) -> bool {
        self.may_complicate
    }

    /// Disallow any further calls to `complicate()` until a call to
    /// `simplify()` returns `true`.
    pub fn disallow_complicate(&mut self) {
        self.may_complicate = false;
    }

    /// Prevent any further shrinking operations from occurring.
    pub fn freeze(&mut self) {
        self.disallow_simplify();
        self.disallow_complicate();
    }
}

impl<T: ValueTree> ValueTree for Fuse<T> {
    type Value = T::Value;

    fn current(&self) -> T::Value {
        self.inner.current()
    }

    fn simplify(&mut self) -> bool {
        if self.may_simplify {
            if self.inner.simplify() {
                self.may_complicate = true;
                true
            } else {
                self.may_simplify = false;
                false
            }
        } else {
            false
        }
    }

    fn complicate(&mut self) -> bool {
        if self.may_complicate {
            if self.inner.complicate() {
                self.may_simplify = true;
                true
            } else {
                self.may_complicate = false;
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct StrictValueTree {
        min: u32,
        curr: u32,
        max: u32,
        ready: bool,
    }

    impl StrictValueTree {
        fn new(start: u32) -> Self {
            StrictValueTree {
                min: 0,
                curr: start,
                max: start,
                ready: false,
            }
        }
    }

    impl ValueTree for StrictValueTree {
        type Value = u32;

        fn current(&self) -> u32 {
            self.curr
        }

        fn simplify(&mut self) -> bool {
            assert!(self.min <= self.curr);
            if self.curr > self.min {
                self.max = self.curr;
                self.curr -= 1;
                self.ready = true;
                true
            } else {
                self.min += 1;
                false
            }
        }

        fn complicate(&mut self) -> bool {
            assert!(self.max >= self.curr);
            assert!(self.ready);
            if self.curr < self.max {
                self.curr += 1;
                true
            } else {
                self.max -= 1;
                false
            }
        }
    }

    #[test]
    fn test_sanity() {
        check_strategy_sanity(Fuse::new(0i32..100i32), None);
    }

    #[test]
    fn guards_bad_transitions() {
        let mut vt = Fuse::new(StrictValueTree::new(5));
        assert!(!vt.complicate());
        assert_eq!(5, vt.current());

        assert!(vt.simplify()); // 0, 4, 5
        assert!(vt.simplify()); // 0, 3, 4
        assert!(vt.simplify()); // 0, 2, 3
        assert!(vt.simplify()); // 0, 1, 2
        assert!(vt.simplify()); // 0, 0, 1
        assert_eq!(0, vt.current());
        assert!(!vt.simplify()); // 1, 0, 1
        assert!(!vt.simplify()); // 1, 0, 1
        assert_eq!(0, vt.current());
        assert!(vt.complicate()); // 1, 1, 1
        assert_eq!(1, vt.current());
        assert!(!vt.complicate()); // 1, 1, 0
        assert!(!vt.complicate()); // 1, 1, 0
        assert_eq!(1, vt.current());
    }
}

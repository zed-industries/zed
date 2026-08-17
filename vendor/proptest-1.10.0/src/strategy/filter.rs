//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::{fmt, Arc};

use crate::strategy::traits::*;
use crate::test_runner::*;

/// `Strategy` and `ValueTree` filter adaptor.
///
/// See `Strategy::prop_filter()`.
#[must_use = "strategies do nothing unless used"]
pub struct Filter<S, F> {
    pub(super) source: S,
    pub(super) whence: Reason,
    pub(super) fun: Arc<F>,
}

impl<S, F> Filter<S, F> {
    pub(super) fn new(source: S, whence: Reason, fun: F) -> Self {
        Self {
            source,
            whence,
            fun: Arc::new(fun),
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

impl<S: Clone, F> Clone for Filter<S, F> {
    fn clone(&self) -> Self {
        Filter {
            source: self.source.clone(),
            whence: "unused".into(),
            fun: Arc::clone(&self.fun),
        }
    }
}

impl<S: Strategy, F: Fn(&S::Value) -> bool> Strategy for Filter<S, F> {
    type Tree = Filter<S::Tree, F>;
    type Value = S::Value;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        loop {
            let val = self.source.new_tree(runner)?;
            if !(self.fun)(&val.current()) {
                runner.reject_local(self.whence.clone())?;
            } else {
                return Ok(Filter {
                    source: val,
                    whence: self.whence.clone(),
                    fun: Arc::clone(&self.fun),
                });
            }
        }
    }
}

impl<S: ValueTree, F: Fn(&S::Value) -> bool> Filter<S, F> {
    fn ensure_acceptable(&mut self) {
        while !(self.fun)(&self.source.current()) {
            if !self.source.complicate() {
                panic!(
                    "Unable to complicate filtered strategy \
                     back into acceptable value"
                );
            }
        }
    }
}

impl<S: ValueTree, F: Fn(&S::Value) -> bool> ValueTree for Filter<S, F> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filter() {
        let input = (0..256).prop_filter("%3", |&v| 0 == v % 3);

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
    fn test_filter_sanity() {
        check_strategy_sanity(
            (0..256).prop_filter("!%5", |&v| 0 != v % 5),
            Some(CheckStrategySanityOptions {
                // Due to internal rejection sampling, `simplify()` can
                // converge back to what `complicate()` would do.
                strict_complicate_after_simplify: false,
                ..CheckStrategySanityOptions::default()
            }),
        );
    }
}

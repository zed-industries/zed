//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::{fmt, Arc, Cell};

use crate::strategy::traits::*;
use crate::test_runner::*;

/// `Strategy` and `ValueTree` filter_map adaptor.
///
/// See `Strategy::prop_filter_map()`.
#[must_use = "strategies do nothing unless used"]
pub struct FilterMap<S, F> {
    pub(super) source: S,
    pub(super) whence: Reason,
    pub(super) fun: Arc<F>,
}

impl<S, F> FilterMap<S, F> {
    pub(super) fn new(source: S, whence: Reason, fun: F) -> Self {
        Self {
            source,
            whence,
            fun: Arc::new(fun),
        }
    }
}

impl<S: fmt::Debug, F> fmt::Debug for FilterMap<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FilterMap")
            .field("source", &self.source)
            .field("whence", &self.whence)
            .field("fun", &"<function>")
            .finish()
    }
}

impl<S: Clone, F> Clone for FilterMap<S, F> {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            whence: self.whence.clone(),
            fun: Arc::clone(&self.fun),
        }
    }
}

impl<S: Strategy, F: Fn(S::Value) -> Option<O>, O: fmt::Debug> Strategy
    for FilterMap<S, F>
{
    type Tree = FilterMapValueTree<S::Tree, F, O>;
    type Value = O;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        loop {
            let val = self.source.new_tree(runner)?;
            if let Some(current) = (self.fun)(val.current()) {
                return Ok(FilterMapValueTree::new(val, &self.fun, current));
            } else {
                runner.reject_local(self.whence.clone())?;
            }
        }
    }
}

/// `ValueTree` corresponding to `FilterMap`.
pub struct FilterMapValueTree<V, F, O> {
    source: V,
    current: Cell<Option<O>>,
    fun: Arc<F>,
}

impl<V: Clone + ValueTree, F: Fn(V::Value) -> Option<O>, O> Clone
    for FilterMapValueTree<V, F, O>
{
    fn clone(&self) -> Self {
        Self::new(self.source.clone(), &self.fun, self.fresh_current())
    }
}

impl<V: fmt::Debug, F, O> fmt::Debug for FilterMapValueTree<V, F, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FilterMapValueTree")
            .field("source", &self.source)
            .field("current", &"<current>")
            .field("fun", &"<function>")
            .finish()
    }
}

impl<V: ValueTree, F: Fn(V::Value) -> Option<O>, O>
    FilterMapValueTree<V, F, O>
{
    fn new(source: V, fun: &Arc<F>, current: O) -> Self {
        Self {
            source,
            current: Cell::new(Some(current)),
            fun: Arc::clone(fun),
        }
    }

    fn fresh_current(&self) -> O {
        (self.fun)(self.source.current())
            .expect("internal logic error; this is a bug!")
    }

    fn ensure_acceptable(&mut self) {
        loop {
            if let Some(current) = (self.fun)(self.source.current()) {
                // Found an acceptable element!
                self.current = Cell::new(Some(current));
                break;
            } else if !self.source.complicate() {
                panic!(
                    "Unable to complicate filtered strategy \
                     back into acceptable value"
                );
            }
        }
    }
}

impl<V: ValueTree, F: Fn(V::Value) -> Option<O>, O: fmt::Debug> ValueTree
    for FilterMapValueTree<V, F, O>
{
    type Value = O;

    fn current(&self) -> O {
        // Optimization: we avoid the else branch in most success cases
        // thereby avoiding to call the closure and the source tree.
        if let Some(current) = self.current.replace(None) {
            current
        } else {
            self.fresh_current()
        }
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
    fn test_filter_map() {
        let input = (0..256).prop_filter_map("%3 + 1", |v| {
            if 0 == v % 3 {
                Some(v + 1)
            } else {
                None
            }
        });

        for _ in 0..256 {
            let mut runner = TestRunner::default();
            let mut case = input.new_tree(&mut runner).unwrap();

            assert_eq!(0, (case.current() - 1) % 3);

            while case.simplify() {
                assert_eq!(0, (case.current() - 1) % 3);
            }
            assert_eq!(0, (case.current() - 1) % 3);
        }
    }

    #[test]
    fn test_filter_map_sanity() {
        check_strategy_sanity(
            (0..256).prop_filter_map("!%5 * 2", |v| {
                if 0 != v % 5 {
                    Some(v * 2)
                } else {
                    None
                }
            }),
            Some(CheckStrategySanityOptions {
                // Due to internal rejection sampling, `simplify()` can
                // converge back to what `complicate()` would do.
                strict_complicate_after_simplify: false,
                ..CheckStrategySanityOptions::default()
            }),
        );
    }
}

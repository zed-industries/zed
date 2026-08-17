//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::Arc;
use core::fmt;
use core::marker::PhantomData;

use crate::strategy::traits::*;
use crate::test_runner::*;

//==============================================================================
// Map
//==============================================================================

/// `Strategy` and `ValueTree` map adaptor.
///
/// See `Strategy::prop_map()`.
#[must_use = "strategies do nothing unless used"]
pub struct Map<S, F> {
    pub(super) source: S,
    pub(super) fun: Arc<F>,
}

impl<S: fmt::Debug, F> fmt::Debug for Map<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Map")
            .field("source", &self.source)
            .field("fun", &"<function>")
            .finish()
    }
}

impl<S: Clone, F> Clone for Map<S, F> {
    fn clone(&self) -> Self {
        Map {
            source: self.source.clone(),
            fun: Arc::clone(&self.fun),
        }
    }
}

impl<S: Strategy, O: fmt::Debug, F: Fn(S::Value) -> O> Strategy for Map<S, F> {
    type Tree = Map<S::Tree, F>;
    type Value = O;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        self.source.new_tree(runner).map(|v| Map {
            source: v,
            fun: Arc::clone(&self.fun),
        })
    }
}

impl<S: ValueTree, O: fmt::Debug, F: Fn(S::Value) -> O> ValueTree
    for Map<S, F>
{
    type Value = O;

    fn current(&self) -> O {
        (self.fun)(self.source.current())
    }

    fn simplify(&mut self) -> bool {
        self.source.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.source.complicate()
    }
}

//==============================================================================
// MapInto
//==============================================================================

// NOTE: Since this is external stable API,
// we avoid relying on the Map in `statics`.

/// `Strategy` and `ValueTree` map into adaptor.
///
/// See `Strategy::prop_map_into()`.
#[must_use = "strategies do nothing unless used"]
pub struct MapInto<S, O> {
    pub(super) source: S,
    pub(super) output: PhantomData<O>,
}

impl<S, O> MapInto<S, O> {
    /// Construct a `MapInto` mapper from an `S` strategy into a strategy
    /// producing `O`s.
    pub(super) fn new(source: S) -> Self {
        Self {
            source,
            output: PhantomData,
        }
    }
}

impl<S: fmt::Debug, O> fmt::Debug for MapInto<S, O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MapInto")
            .field("source", &self.source)
            .finish()
    }
}

impl<S: Clone, O> Clone for MapInto<S, O> {
    fn clone(&self) -> Self {
        Self::new(self.source.clone())
    }
}

impl<S: Strategy, O: fmt::Debug> Strategy for MapInto<S, O>
where
    S::Value: Into<O>,
{
    type Tree = MapInto<S::Tree, O>;
    type Value = O;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        self.source.new_tree(runner).map(MapInto::new)
    }
}

impl<S: ValueTree, O: fmt::Debug> ValueTree for MapInto<S, O>
where
    S::Value: Into<O>,
{
    type Value = O;

    fn current(&self) -> O {
        self.source.current().into()
    }

    fn simplify(&mut self) -> bool {
        self.source.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.source.complicate()
    }
}

//==============================================================================
// Perturb
//==============================================================================

/// `Strategy` perturbation adaptor.
///
/// See `Strategy::prop_perturb()`.
#[must_use = "strategies do nothing unless used"]
pub struct Perturb<S, F> {
    pub(super) source: S,
    pub(super) fun: Arc<F>,
}

impl<S: fmt::Debug, F> fmt::Debug for Perturb<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Perturb")
            .field("source", &self.source)
            .field("fun", &"<function>")
            .finish()
    }
}

impl<S: Clone, F> Clone for Perturb<S, F> {
    fn clone(&self) -> Self {
        Perturb {
            source: self.source.clone(),
            fun: Arc::clone(&self.fun),
        }
    }
}

impl<S: Strategy, O: fmt::Debug, F: Fn(S::Value, TestRng) -> O> Strategy
    for Perturb<S, F>
{
    type Tree = PerturbValueTree<S::Tree, F>;
    type Value = O;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        let rng = runner.new_rng();

        self.source.new_tree(runner).map(|source| PerturbValueTree {
            source,
            rng,
            fun: Arc::clone(&self.fun),
        })
    }
}

/// `ValueTree` perturbation adaptor.
///
/// See `Strategy::prop_perturb()`.
pub struct PerturbValueTree<S, F> {
    source: S,
    fun: Arc<F>,
    rng: TestRng,
}

impl<S: fmt::Debug, F> fmt::Debug for PerturbValueTree<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PerturbValueTree")
            .field("source", &self.source)
            .field("fun", &"<function>")
            .field("rng", &self.rng)
            .finish()
    }
}

impl<S: Clone, F> Clone for PerturbValueTree<S, F> {
    fn clone(&self) -> Self {
        PerturbValueTree {
            source: self.source.clone(),
            fun: Arc::clone(&self.fun),
            rng: self.rng.clone(),
        }
    }
}

impl<S: ValueTree, O: fmt::Debug, F: Fn(S::Value, TestRng) -> O> ValueTree
    for PerturbValueTree<S, F>
{
    type Value = O;

    fn current(&self) -> O {
        (self.fun)(self.source.current(), self.rng.clone())
    }

    fn simplify(&mut self) -> bool {
        self.source.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.source.complicate()
    }
}

//==============================================================================
// Tests
//==============================================================================

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use rand::RngCore;

    use super::*;
    use crate::strategy::just::Just;

    #[test]
    fn test_map() {
        TestRunner::default()
            .run(&(0..10).prop_map(|v| v * 2), |v| {
                assert!(0 == v % 2);
                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn test_map_into() {
        TestRunner::default()
            .run(&(0..10u8).prop_map_into::<usize>(), |v| {
                assert!(v < 10);
                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn perturb_uses_same_rng_every_time() {
        let mut runner = TestRunner::default();
        let input = Just(1).prop_perturb(|v, mut rng| v + rng.next_u32());

        for _ in 0..16 {
            let value = input.new_tree(&mut runner).unwrap();
            assert_eq!(value.current(), value.current());
        }
    }

    #[test]
    fn perturb_uses_varying_random_seeds() {
        let mut runner = TestRunner::default();
        let input = Just(1).prop_perturb(|v, mut rng| v + rng.next_u32());

        let mut seen = HashSet::new();
        for _ in 0..64 {
            seen.insert(input.new_tree(&mut runner).unwrap().current());
        }

        assert_eq!(64, seen.len());
    }
}

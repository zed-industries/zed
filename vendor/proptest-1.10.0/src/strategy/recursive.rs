//-
// Copyright 2017 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::{fmt, Arc, Box, Vec};

use crate::strategy::traits::*;
use crate::strategy::unions::float_to_weight;
use crate::test_runner::*;

/// Return type from `Strategy::prop_recursive()`.
#[must_use = "strategies do nothing unless used"]
pub struct Recursive<T, F> {
    base: BoxedStrategy<T>,
    recurse: Arc<F>,
    depth: u32,
    desired_size: u32,
    expected_branch_size: u32,
}

impl<T: fmt::Debug, F> fmt::Debug for Recursive<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Recursive")
            .field("base", &self.base)
            .field("recurse", &"<function>")
            .field("depth", &self.depth)
            .field("desired_size", &self.desired_size)
            .field("expected_branch_size", &self.expected_branch_size)
            .finish()
    }
}

impl<T, F> Clone for Recursive<T, F> {
    fn clone(&self) -> Self {
        Recursive {
            base: self.base.clone(),
            recurse: Arc::clone(&self.recurse),
            depth: self.depth,
            desired_size: self.desired_size,
            expected_branch_size: self.expected_branch_size,
        }
    }
}

impl<
        T: fmt::Debug + 'static,
        R: Strategy<Value = T> + 'static,
        F: Fn(BoxedStrategy<T>) -> R,
    > Recursive<T, F>
{
    pub(super) fn new(
        base: impl Strategy<Value = T> + 'static,
        depth: u32,
        desired_size: u32,
        expected_branch_size: u32,
        recurse: F,
    ) -> Self {
        Self {
            base: base.boxed(),
            recurse: Arc::new(recurse),
            depth,
            desired_size,
            expected_branch_size,
        }
    }
}

impl<
        T: fmt::Debug + 'static,
        R: Strategy<Value = T> + 'static,
        F: Fn(BoxedStrategy<T>) -> R,
    > Strategy for Recursive<T, F>
{
    type Tree = Box<dyn ValueTree<Value = T>>;
    type Value = T;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        // Since the generator is stateless, we can't implement any "absolutely
        // X many items" rule. We _can_, however, with extremely high
        // probability, obtain a value near what we want by using decaying
        // probabilities of branching as we go down the tree.
        //
        // We are given a target size S and a branch size K (branch size =
        // expected number of items immediately below each branch). We select
        // some probability P for each level.
        //
        // A single level l is thus expected to hold PlK branches. Each of
        // those will have P(l+1)K child branches of their own, so there are
        // PlP(l+1)K² second-level branches. The total branches in the tree is
        // thus (Σ PlK^l) for l from 0 to infinity. Each level is expected to
        // hold K items, so the total number of items is simply K times the
        // number of branches, or (K Σ PlK^l). So we want to find a P sequence
        // such that (lim (K Σ PlK^l) = S), or more simply,
        // (lim Σ PlK^l = S/K).
        //
        // Let Q be a second probability sequence such that Pl = Ql/K^l. This
        // changes the formulation to (lim Σ Ql = S/K). The series Σ0.5^(l+1)
        // converges on 1.0, so we can let Ql = S/K * 0.5^(l+1), and so
        // Pl = S/K^(l+1) * 0.5^(l+1) = S / (2K) ^ (l+1)
        //
        // We don't actually have infinite levels here since we _can_ easily
        // cap to a fixed max depth, so this will be a minor underestimate. We
        // also clamp all probabilities to 0.9 to ensure that we can't end up
        // with levels which are always pure branches, which further
        // underestimates size.

        let mut branch_probabilities = Vec::new();
        let mut k2 = u64::from(self.expected_branch_size) * 2;
        for _ in 0..self.depth {
            branch_probabilities.push(f64::from(self.desired_size) / k2 as f64);
            k2 = k2.saturating_mul(u64::from(self.expected_branch_size) * 2);
        }

        let mut strat = self.base.clone();
        while let Some(branch_probability) = branch_probabilities.pop() {
            let recursed = (self.recurse)(strat.clone());
            let recursive_choice = recursed.boxed();
            let non_recursive_choice = strat;
            // Clamp the maximum branch probability to 0.9 to ensure we can
            // generate non-recursive cases reasonably often.
            let branch_probability = branch_probability.min(0.9);
            let (weight_branch, weight_leaf) =
                float_to_weight(branch_probability);
            let branch = prop_oneof![
                weight_leaf => non_recursive_choice,
                weight_branch => recursive_choice,
            ];
            strat = branch.boxed();
        }

        strat.new_tree(runner)
    }
}

#[cfg(test)]
mod test {
    use std::cmp::max;

    use super::*;
    use crate::strategy::just::Just;

    #[derive(Clone, Debug, PartialEq)]
    enum Tree {
        Leaf,
        Branch(Vec<Tree>),
    }

    impl Tree {
        fn stats(&self) -> (u32, u32) {
            match *self {
                Tree::Leaf => (0, 1),
                Tree::Branch(ref children) => {
                    let mut depth = 0;
                    let mut count = 0;
                    for child in children {
                        let (d, c) = child.stats();
                        depth = max(d, depth);
                        count += c;
                    }

                    (depth + 1, count + 1)
                }
            }
        }
    }

    #[test]
    fn test_recursive() {
        let mut max_depth = 0;
        let mut max_count = 0;

        let strat = Just(Tree::Leaf).prop_recursive(4, 64, 16, |element| {
            crate::collection::vec(element, 8..16).prop_map(Tree::Branch)
        });

        let mut runner = TestRunner::deterministic();
        for _ in 0..65536 {
            let tree = strat.new_tree(&mut runner).unwrap().current();
            let (depth, count) = tree.stats();
            assert!(depth <= 4, "Got depth {}", depth);
            assert!(count <= 128, "Got count {}", count);
            max_depth = max(depth, max_depth);
            max_count = max(count, max_count);
        }

        assert!(max_depth >= 3, "Only got max depth {}", max_depth);
        assert!(max_count > 48, "Only got max count {}", max_count);
    }

    #[test]
    fn simplifies_to_non_recursive() {
        let strat = Just(Tree::Leaf).prop_recursive(4, 64, 16, |element| {
            crate::collection::vec(element, 8..16).prop_map(Tree::Branch)
        });

        let mut runner = TestRunner::deterministic();
        for _ in 0..256 {
            let mut value = strat.new_tree(&mut runner).unwrap();
            while value.simplify() {}

            assert_eq!(Tree::Leaf, value.current());
        }
    }
}

//! Fix-point analyses on the IR using the "monotone framework".
//!
//! A lattice is a set with a partial ordering between elements, where there is
//! a single least upper bound and a single greatest least bound for every
//! subset. We are dealing with finite lattices, which means that it has a
//! finite number of elements, and it follows that there exists a single top and
//! a single bottom member of the lattice. For example, the power set of a
//! finite set forms a finite lattice where partial ordering is defined by set
//! inclusion, that is `a <= b` if `a` is a subset of `b`. Here is the finite
//! lattice constructed from the set {0,1,2}:
//!
//! ```text
//!                    .----- Top = {0,1,2} -----.
//!                   /            |              \
//!                  /             |               \
//!                 /              |                \
//!              {0,1} -------.  {0,2}  .--------- {1,2}
//!                |           \ /   \ /             |
//!                |            /     \              |
//!                |           / \   / \             |
//!               {0} --------'   {1}   `---------- {2}
//!                 \              |                /
//!                  \             |               /
//!                   \            |              /
//!                    `------ Bottom = {} ------'
//! ```
//!
//! A monotone function `f` is a function where if `x <= y`, then it holds that
//! `f(x) <= f(y)`. It should be clear that running a monotone function to a
//! fix-point on a finite lattice will always terminate: `f` can only "move"
//! along the lattice in a single direction, and therefore can only either find
//! a fix-point in the middle of the lattice or continue to the top or bottom
//! depending if it is ascending or descending the lattice respectively.
//!
//! For a deeper introduction to the general form of this kind of analysis, see
//! [Static Program Analysis by Anders Møller and Michael I. Schwartzbach][spa].
//!
//! [spa]: https://cs.au.dk/~amoeller/spa/spa.pdf

// Re-export individual analyses.
mod template_params;
pub(crate) use self::template_params::UsedTemplateParameters;
mod derive;
pub use self::derive::DeriveTrait;
pub(crate) use self::derive::{as_cannot_derive_set, CannotDerive};
mod has_vtable;
pub(crate) use self::has_vtable::{
    HasVtable, HasVtableAnalysis, HasVtableResult,
};
mod has_destructor;
pub(crate) use self::has_destructor::HasDestructorAnalysis;
mod has_type_param_in_array;
pub(crate) use self::has_type_param_in_array::HasTypeParameterInArray;
mod has_float;
pub(crate) use self::has_float::HasFloat;
mod sizedness;
pub(crate) use self::sizedness::{
    Sizedness, SizednessAnalysis, SizednessResult,
};

use crate::ir::context::{BindgenContext, ItemId};

use crate::ir::traversal::{EdgeKind, Trace};
use crate::HashMap;
use std::fmt;
use std::ops;

/// An analysis in the monotone framework.
///
/// Implementors of this trait must maintain the following two invariants:
///
/// 1. The concrete data must be a member of a finite-height lattice.
/// 2. The concrete `constrain` method must be monotone: that is,
///    if `x <= y`, then `constrain(x) <= constrain(y)`.
///
/// If these invariants do not hold, iteration to a fix-point might never
/// complete.
///
/// For a simple example analysis, see the `ReachableFrom` type in the `tests`
/// module below.
pub(crate) trait MonotoneFramework: Sized + fmt::Debug {
    /// The type of node in our dependency graph.
    ///
    /// This is just generic (and not `ItemId`) so that we can easily unit test
    /// without constructing real `Item`s and their `ItemId`s.
    type Node: Copy;

    /// Any extra data that is needed during computation.
    ///
    /// Again, this is just generic (and not `&BindgenContext`) so that we can
    /// easily unit test without constructing real `BindgenContext`s full of
    /// real `Item`s and real `ItemId`s.
    type Extra: Sized;

    /// The final output of this analysis. Once we have reached a fix-point, we
    /// convert `self` into this type, and return it as the final result of the
    /// analysis.
    type Output: From<Self> + fmt::Debug;

    /// Construct a new instance of this analysis.
    fn new(extra: Self::Extra) -> Self;

    /// Get the initial set of nodes from which to start the analysis. Unless
    /// you are sure of some domain-specific knowledge, this should be the
    /// complete set of nodes.
    fn initial_worklist(&self) -> Vec<Self::Node>;

    /// Update the analysis for the given node.
    ///
    /// If this results in changing our internal state (ie, we discovered that
    /// we have not reached a fix-point and iteration should continue), return
    /// `ConstrainResult::Changed`. Otherwise, return `ConstrainResult::Same`.
    /// When `constrain` returns `ConstrainResult::Same` for all nodes in the
    /// set, we have reached a fix-point and the analysis is complete.
    fn constrain(&mut self, node: Self::Node) -> ConstrainResult;

    /// For each node `d` that depends on the given `node`'s current answer when
    /// running `constrain(d)`, call `f(d)`. This informs us which new nodes to
    /// queue up in the worklist when `constrain(node)` reports updated
    /// information.
    fn each_depending_on<F>(&self, node: Self::Node, f: F)
    where
        F: FnMut(Self::Node);
}

/// Whether an analysis's `constrain` function modified the incremental results
/// or not.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub(crate) enum ConstrainResult {
    /// The incremental results were updated, and the fix-point computation
    /// should continue.
    Changed,

    /// The incremental results were not updated.
    #[default]
    Same,
}

impl ops::BitOr for ConstrainResult {
    type Output = Self;

    fn bitor(self, rhs: ConstrainResult) -> Self::Output {
        if self == ConstrainResult::Changed || rhs == ConstrainResult::Changed {
            ConstrainResult::Changed
        } else {
            ConstrainResult::Same
        }
    }
}

impl ops::BitOrAssign for ConstrainResult {
    fn bitor_assign(&mut self, rhs: ConstrainResult) {
        *self = *self | rhs;
    }
}

/// Run an analysis in the monotone framework.
pub(crate) fn analyze<Analysis>(extra: Analysis::Extra) -> Analysis::Output
where
    Analysis: MonotoneFramework,
{
    let mut analysis = Analysis::new(extra);
    let mut worklist = analysis.initial_worklist();

    while let Some(node) = worklist.pop() {
        if let ConstrainResult::Changed = analysis.constrain(node) {
            analysis.each_depending_on(node, |needs_work| {
                worklist.push(needs_work);
            });
        }
    }

    analysis.into()
}

/// Generate the dependency map for analysis
pub(crate) fn generate_dependencies<F>(
    ctx: &BindgenContext,
    consider_edge: F,
) -> HashMap<ItemId, Vec<ItemId>>
where
    F: Fn(EdgeKind) -> bool,
{
    let mut dependencies = HashMap::default();

    for &item in ctx.allowlisted_items() {
        dependencies.entry(item).or_insert_with(Vec::new);

        {
            // We reverse our natural IR graph edges to find dependencies
            // between nodes.
            item.trace(
                ctx,
                &mut |sub_item: ItemId, edge_kind| {
                    if ctx.allowlisted_items().contains(&sub_item) &&
                        consider_edge(edge_kind)
                    {
                        dependencies
                            .entry(sub_item)
                            .or_insert_with(Vec::new)
                            .push(item);
                    }
                },
                &(),
            );
        }
    }
    dependencies
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HashSet;

    // Here we find the set of nodes that are reachable from any given
    // node. This is a lattice mapping nodes to subsets of all nodes. Our join
    // function is set union.
    //
    // This is our test graph:
    //
    //     +---+                    +---+
    //     |   |                    |   |
    //     | 1 |               .----| 2 |
    //     |   |               |    |   |
    //     +---+               |    +---+
    //       |                 |      ^
    //       |                 |      |
    //       |      +---+      '------'
    //       '----->|   |
    //              | 3 |
    //       .------|   |------.
    //       |      +---+      |
    //       |        ^        |
    //       v        |        v
    //     +---+      |      +---+    +---+
    //     |   |      |      |   |    |   |
    //     | 4 |      |      | 5 |--->| 6 |
    //     |   |      |      |   |    |   |
    //     +---+      |      +---+    +---+
    //       |        |        |        |
    //       |        |        |        v
    //       |      +---+      |      +---+
    //       |      |   |      |      |   |
    //       '----->| 7 |<-----'      | 8 |
    //              |   |             |   |
    //              +---+             +---+
    //
    // And here is the mapping from a node to the set of nodes that are
    // reachable from it within the test graph:
    //
    //     1: {3,4,5,6,7,8}
    //     2: {2}
    //     3: {3,4,5,6,7,8}
    //     4: {3,4,5,6,7,8}
    //     5: {3,4,5,6,7,8}
    //     6: {8}
    //     7: {3,4,5,6,7,8}
    //     8: {}

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    struct Node(usize);

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct Graph(HashMap<Node, Vec<Node>>);

    impl Graph {
        fn make_test_graph() -> Graph {
            let mut g = Graph::default();
            g.0.insert(Node(1), vec![Node(3)]);
            g.0.insert(Node(2), vec![Node(2)]);
            g.0.insert(Node(3), vec![Node(4), Node(5)]);
            g.0.insert(Node(4), vec![Node(7)]);
            g.0.insert(Node(5), vec![Node(6), Node(7)]);
            g.0.insert(Node(6), vec![Node(8)]);
            g.0.insert(Node(7), vec![Node(3)]);
            g.0.insert(Node(8), vec![]);
            g
        }

        fn reverse(&self) -> Graph {
            let mut reversed = Graph::default();
            for (node, edges) in &self.0 {
                reversed.0.entry(*node).or_insert_with(Vec::new);
                for referent in edges {
                    reversed
                        .0
                        .entry(*referent)
                        .or_insert_with(Vec::new)
                        .push(*node);
                }
            }
            reversed
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct ReachableFrom<'a> {
        reachable: HashMap<Node, HashSet<Node>>,
        graph: &'a Graph,
        reversed: Graph,
    }

    impl<'a> MonotoneFramework for ReachableFrom<'a> {
        type Node = Node;
        type Extra = &'a Graph;
        type Output = HashMap<Node, HashSet<Node>>;

        fn new(graph: &'a Graph) -> Self {
            let reversed = graph.reverse();
            ReachableFrom {
                reachable: Default::default(),
                graph,
                reversed,
            }
        }

        fn initial_worklist(&self) -> Vec<Node> {
            self.graph.0.keys().copied().collect()
        }

        fn constrain(&mut self, node: Node) -> ConstrainResult {
            // The set of nodes reachable from a node `x` is
            //
            //     reachable(x) = s_0 U s_1 U ... U reachable(s_0) U reachable(s_1) U ...
            //
            // where there exist edges from `x` to each of `s_0, s_1, ...`.
            //
            // Yes, what follows is a **terribly** inefficient set union
            // implementation. Don't copy this code outside of this test!

            let original_size = self.reachable.entry(node).or_default().len();

            for sub_node in &self.graph.0[&node] {
                self.reachable.get_mut(&node).unwrap().insert(*sub_node);

                let sub_reachable =
                    self.reachable.entry(*sub_node).or_default().clone();

                for transitive in sub_reachable {
                    self.reachable.get_mut(&node).unwrap().insert(transitive);
                }
            }

            let new_size = self.reachable[&node].len();
            if original_size == new_size {
                ConstrainResult::Same
            } else {
                ConstrainResult::Changed
            }
        }

        fn each_depending_on<F>(&self, node: Node, mut f: F)
        where
            F: FnMut(Node),
        {
            for dep in &self.reversed.0[&node] {
                f(*dep);
            }
        }
    }

    impl<'a> From<ReachableFrom<'a>> for HashMap<Node, HashSet<Node>> {
        fn from(reachable: ReachableFrom<'a>) -> Self {
            reachable.reachable
        }
    }

    #[test]
    fn monotone() {
        let g = Graph::make_test_graph();
        let reachable = analyze::<ReachableFrom>(&g);
        println!("reachable = {reachable:#?}");

        fn nodes<A>(nodes: A) -> HashSet<Node>
        where
            A: AsRef<[usize]>,
        {
            nodes.as_ref().iter().copied().map(Node).collect()
        }

        let mut expected = HashMap::default();
        expected.insert(Node(1), nodes([3, 4, 5, 6, 7, 8]));
        expected.insert(Node(2), nodes([2]));
        expected.insert(Node(3), nodes([3, 4, 5, 6, 7, 8]));
        expected.insert(Node(4), nodes([3, 4, 5, 6, 7, 8]));
        expected.insert(Node(5), nodes([3, 4, 5, 6, 7, 8]));
        expected.insert(Node(6), nodes([8]));
        expected.insert(Node(7), nodes([3, 4, 5, 6, 7, 8]));
        expected.insert(Node(8), nodes([]));
        println!("expected = {expected:#?}");

        assert_eq!(reachable, expected);
    }
}

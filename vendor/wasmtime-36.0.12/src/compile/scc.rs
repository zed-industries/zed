//! Strongly-connected components.
//!
//! This module implements [Tarjan's algorithm] for finding strongly-connected
//! components.
//!
//! This algorithm takes `O(V+E)` time and uses `O(V+E)` space.
//!
//! Tarjan's algorithm is usually presented as a recursive algorithm, but we do
//! not trust the input and cannot recurse over it for fear of blowing the
//! stack. Therefore, this implementation is iterative.
//!
//! [Tarjan's algorithm]: https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm

use super::*;
use std::{
    collections::BTreeSet,
    fmt::{self, Debug},
    hash::Hash,
    ops::Range,
};
use wasmtime_environ::{
    EntityRef, EntitySet, PrimaryMap, SecondaryMap, packed_option::PackedOption,
};

/// A strongly-connected component.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scc(u32);
wasmtime_environ::entity_impl!(Scc);

/// The set of strongly-connected components for a graph of `Node`s.
pub struct StronglyConnectedComponents<Node>
where
    Node: EntityRef,
{
    /// A map from a component to the range of `self.component_nodes` that make
    /// up that component's nodes.
    components: PrimaryMap<Scc, Range<u32>>,

    /// The data storage for the values of `self.components`.
    component_nodes: Vec<Node>,
}

impl<Node> Debug for StronglyConnectedComponents<Node>
where
    Node: EntityRef + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Map<'a, Node: EntityRef + Debug>(&'a StronglyConnectedComponents<Node>);

        impl<'a, Node> Debug for Map<'a, Node>
        where
            Node: EntityRef + Debug,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> fmt::Result {
                f.debug_map().entries(self.0.iter()).finish()
            }
        }

        f.debug_struct("StronglyConnectedComponents")
            .field("components", &Map(self))
            .finish()
    }
}

impl<Node> StronglyConnectedComponents<Node>
where
    Node: EntityRef + std::fmt::Debug,
{
    /// Find the strongly-connected for the given graph.
    pub fn new<I, F, S>(nodes: I, successors: F) -> Self
    where
        I: IntoIterator<Item = Node>,
        F: Fn(Node) -> S,
        S: Iterator<Item = Node>,
    {
        let nodes = nodes.into_iter();

        // The resulting components and their nodes.
        let mut component_nodes = vec![];
        let mut components = PrimaryMap::<Scc, Range<u32>>::new();

        // The DFS index counter.
        let mut index = NonMaxU32::default();

        // The DFS index and the earliest on-stack node reachable from each
        // node.
        let (min, max) = nodes.size_hint();
        let capacity = max.unwrap_or_else(|| 2 * min);
        let mut indices = SecondaryMap::<Node, Option<NonMaxU32>>::with_capacity(capacity);
        let mut lowlinks = SecondaryMap::<Node, Option<NonMaxU32>>::with_capacity(capacity);

        // The stack of nodes we are currently finding an SCC for. Not the same
        // as the DFS stack: we only pop from this stack once we find the root
        // of an SCC.
        let mut stack = vec![];
        let mut on_stack = EntitySet::<Node>::new();

        let mut dfs = Dfs::new(nodes);
        while let Some(event) = dfs.next(
            &successors,
            // We have seen the node before if we have assigned it a DFS index.
            |node| indices[node].is_some(),
        ) {
            match event {
                DfsEvent::Pre(node) => {
                    debug_assert!(indices[node].is_none());
                    debug_assert!(lowlinks[node].is_none());

                    // Assign an index to this node.
                    indices[node] = Some(index);

                    // Its current lowlink is itself. This will get updated to
                    // be accurate as we visit the node's successors.
                    lowlinks[node] = Some(index);

                    // Increment the DFS counter.
                    index = NonMaxU32::new(index.get() + 1).unwrap();

                    // Push the node onto the SCC stack.
                    stack.push(node);
                    let is_newly_on_stack = on_stack.insert(node);
                    debug_assert!(is_newly_on_stack);
                }

                DfsEvent::AfterEdge(node, succ) => {
                    debug_assert!(indices[node].is_some());
                    debug_assert!(lowlinks[node].is_some());
                    debug_assert!(lowlinks[node] <= indices[node]);
                    debug_assert!(indices[succ].is_some());
                    debug_assert!(lowlinks[succ].is_some());
                    debug_assert!(lowlinks[succ] <= indices[succ]);

                    // If the successor is still on the SCC stack, then it is
                    // part of the same SCC as this node, so propagate its
                    // lowlink to this node's lowlink.
                    if on_stack.contains(succ) {
                        lowlinks[node] = Some(std::cmp::min(
                            lowlinks[node].unwrap(),
                            lowlinks[succ].unwrap(),
                        ));
                    }
                }

                DfsEvent::Post(node) => {
                    debug_assert!(indices[node].is_some());
                    debug_assert!(lowlinks[node].is_some());

                    // If this node's index is the same as its lowlink, then it
                    // is the root of a component. Pop this component's elements
                    // from the SCC stack and push them into our result data
                    // structures.
                    if indices[node] == lowlinks[node] {
                        let mut done = false;
                        components.push(extend_with_range(
                            &mut component_nodes,
                            std::iter::from_fn(|| {
                                if done {
                                    return None;
                                }
                                let v = stack.pop().unwrap();
                                let was_on_stack = on_stack.remove(v);
                                debug_assert!(was_on_stack);
                                if v == node {
                                    done = true;
                                }
                                Some(v)
                            }),
                        ));
                    }
                }
            }
        }

        Self {
            components,
            component_nodes,
        }
    }

    /// Get the number of components.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    fn node_range(&self, range: Range<u32>) -> &[Node] {
        let start = usize::try_from(range.start).unwrap();
        let end = usize::try_from(range.end).unwrap();
        &self.component_nodes[start..end]
    }

    /// Iterate over each strongly-connnected component and the `Node`s that are
    /// members of it.
    ///
    /// Iteration happens in reverse-topological order (successors are visited
    /// before predecessors in the resulting SCC DAG).
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (Scc, &[Node])> + '_ {
        self.components
            .iter()
            .map(|(component, range)| (component, self.node_range(range.clone())))
    }

    /// Iterate over each strongly-connected component.
    ///
    /// Iteration happens in reverse-topological order (successors are visited
    /// before predecessors in the resulting SCC DAG).
    pub fn keys(&self) -> impl ExactSizeIterator<Item = Scc> {
        self.components.keys()
    }

    /// Iterate over the `Node`s that make up each strongly-connected component.
    ///
    /// Iteration happens in reverse-topological order (successors are visited
    /// before predecessors in the resulting SCC DAG).
    #[cfg(test)]
    pub fn values(&self) -> impl ExactSizeIterator<Item = &[Node]> + '_ {
        self.components
            .values()
            .map(|range| self.node_range(range.clone()))
    }

    /// Get the `Node`s that make up the given strongly-connected component.
    pub fn nodes(&self, component: Scc) -> &[Node] {
        let range = self.components[component].clone();
        self.node_range(range)
    }

    /// Get this set of strongly-connected components' "evaporation".
    ///
    /// Given an input graph `G`, we can construct a new graph where the new
    /// graph's nodes are the strongly-connected components of `G` and there is
    /// an edge `scc(a) --> scc(b)` for every edge `a --> b` in the input graph
    /// `G` where `scc(a) != scc(b)` and `scc` is the function from a node to
    /// its strongly-connected component. This new graph is known as the
    /// "condensation" of `G`.
    ///
    /// In the following diagram, the solid lines are the input graph and the
    /// dotted lines show its condensation:
    ///
    /// ```ignore
    /// ...............
    /// :             :
    /// :    ,-----.  :
    /// :    |     |  :
    /// :    V     |  :
    /// :  +---+   |  :
    /// :  | a |---'  :
    /// :  +---+      :
    /// :    |        :
    /// :....|........:
    ///      |  :
    ///      |  :
    ///      |  :
    ///      |  V
    /// .....|..............       ....................
    /// :    |             :       :                  :
    /// :    V             :......>:                  :
    /// :  +---+    +---+  :       :  +---+    +---+  :
    /// :  | b |<-->| c |------------>| d |<-->| e |  :
    /// :  +---+    +---+  :       :  +---+    +---+  :
    /// :    |             :       :             |    :
    /// :....|.............:       :.............|....:
    ///      |  :                          :     |
    ///      |  :                          :     |
    ///      |  :                          :     |
    ///      |  V                          :     |
    /// .....|........................     :     |
    /// :    |                       :     :     |
    /// :    | ,----------------.    :     :     |
    /// :    | |                |    :<....:     |
    /// :    V |                V    :           |
    /// :   +---+    +---+    +---+  :           |
    /// :   | f |<---| g |<---| h |<-------------'
    /// :   +---+    +---+    +---+  :
    /// :                            :
    /// :............................:
    /// ```
    ///
    /// Note that a graph's condensation is always acyclic, because the
    /// strongly-conneted components encapsulate and hide any cycles from the
    /// input graph.
    ///
    /// I am not aware of an existing name for the reverse (or transpose)
    /// condensation, where each of the condensation's edges `a --> b` are
    /// flipped into `b --> a`, so, at cfallin's brilliant suggestion, I have
    /// decided to call it an "evaporation".
    ///
    /// In the context of a call graph, the condensation allows us to derive a
    /// partial dependency ordering for bottom-up inlining:
    ///
    /// * An edge `scc({a,b,...}) --> scc({c,d,...})` means that the functions
    ///   `{c,d,...}` should be processed for inlining before functions
    ///   `{a,b,...}`, since some functions in `{a,b,...}` call some functions
    ///   in `{c,d,...}` and we might want to inline these calls but only after
    ///   `{c,d,...}` have already had their calls inlined.
    ///
    /// * Functions within an SCC are unordered (and we probably don't want to
    ///   inline between them at all, or only want to do so in a very limited
    ///   manner, since their members are mutually recursive).
    ///
    /// A call graph's evaporation, by flipping edges from pointing to an SCC's
    /// dependencies to pointing to its dependers, allows us to answer the
    /// question "given that I've finished processing calls in `scc({e,f,...})`
    /// for inlining, which other SCCs are now potentially ready for
    /// processing?".
    pub fn evaporation<F, S>(&self, successors: F) -> Evaporation
    where
        F: Fn(Node) -> S,
        S: Iterator<Item = Node>,
    {
        log::trace!("Creating the evaporation of {self:#?}");

        let node_to_component: SecondaryMap<Node, PackedOption<Scc>> = self
            .iter()
            .flat_map(|(c, nodes)| {
                nodes
                    .iter()
                    .copied()
                    .map(move |node| (node, PackedOption::from(Some(c))))
            })
            .collect();

        // Create a set of all reverse dependencies. This set contains an entry
        // `(to, from)` for all `from --> to` edges in the SCC's condensation.
        //
        // Note that we use a `BTreeSet` so that the results are ordered, all
        // edges `a --> *` are contiguous for each component `a`, and we can
        // therefore pack them densely in the resulting `Evaporation`.
        let mut reverse_edge_set = BTreeSet::<(Scc, Scc)>::new();
        for (from_component, nodes) in self.iter() {
            for node in nodes {
                for succ in successors(*node) {
                    let to_component = node_to_component[succ].unwrap();
                    if to_component == from_component {
                        continue;
                    }
                    reverse_edge_set.insert((to_component, from_component));
                }
            }
        }

        // Convert the `reverse_edge_set` into our densely-packed
        // representation.
        let mut reverse_edges =
            SecondaryMap::<Scc, Range<u32>>::with_capacity(self.components.len());
        let mut reverse_edge_elems = Vec::with_capacity(reverse_edge_set.len());
        for (to_node, from_node) in reverse_edge_set {
            let range = extend_with_range(&mut reverse_edge_elems, Some(from_node));

            if reverse_edges[to_node] == Range::default() {
                reverse_edges[to_node] = range;
            } else {
                debug_assert_eq!(reverse_edges[to_node].end, range.start);
                reverse_edges[to_node].end = range.end;
            }
        }

        let result = Evaporation {
            reverse_edges,
            reverse_edge_elems,
        };
        log::trace!("  -> {result:#?}");
        result
    }
}

/// The "evaporation" of a graph and its strongly-connected components.
///
/// Created by `StronglyConnectedComponents::evaporation`; see that method's
/// documentation for more details.
pub struct Evaporation {
    reverse_edges: SecondaryMap<Scc, Range<u32>>,
    reverse_edge_elems: Vec<Scc>,
}

impl Debug for Evaporation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Map<'a>(&'a Evaporation);

        impl<'a> Debug for Map<'a> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_map()
                    .entries(
                        self.0
                            .reverse_edges
                            .keys()
                            .map(|k| (k, self.0.reverse_edges(k))),
                    )
                    .finish()
            }
        }

        f.debug_struct("Evaporation")
            .field("reverse_edges", &Map(self))
            .finish()
    }
}

impl Evaporation {
    /// Get the reverse dependencies of the given strongly-connected component.
    ///
    /// The result is all of the SCCs whose members have edges in the original
    /// graph to one or more of this SCC's members.
    pub fn reverse_edges(&self, component: Scc) -> &[Scc] {
        let Range { start, end } = self.reverse_edges[component].clone();
        let start = usize::try_from(start).unwrap();
        let end = usize::try_from(end).unwrap();
        &self.reverse_edge_elems[start..end]
    }
}

/// An iterative depth-first traversal.
struct Dfs<Node> {
    stack: Vec<DfsEvent<Node>>,
}

impl<Node> Dfs<Node> {
    fn new(roots: impl IntoIterator<Item = Node>) -> Self {
        Self {
            stack: roots.into_iter().map(|v| DfsEvent::Pre(v)).collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DfsEvent<Node> {
    /// The first time seeing this node.
    Pre(Node),

    /// After having just visited the given edge.
    AfterEdge(Node, Node),

    /// Finished visiting this node and all of its successors.
    Post(Node),
}

impl<Node> Dfs<Node>
where
    Node: Copy + std::fmt::Debug,
{
    /// Pump the traversal, yielding the next `DfsEvent`.
    fn next<S>(
        &mut self,
        successors: impl Fn(Node) -> S,
        seen: impl Fn(Node) -> bool,
    ) -> Option<DfsEvent<Node>>
    where
        S: Iterator<Item = Node>,
    {
        loop {
            let event = self.stack.pop()?;

            if let DfsEvent::Pre(node) = event {
                if seen(node) {
                    continue;
                }

                let successors = successors(node);

                let (min, max) = successors.size_hint();
                let estimated_successors_len = max.unwrap_or_else(|| 2 * min);
                self.stack.reserve(
                    // We push an after-edge and pre event for each successor.
                    2 * estimated_successors_len
                        // And we push one post event for this node.
                        + 1,
                );

                self.stack.push(DfsEvent::Post(node));
                for succ in successors {
                    self.stack.push(DfsEvent::AfterEdge(node, succ));
                    if !seen(succ) {
                        self.stack.push(DfsEvent::Pre(succ));
                    }
                }
            }

            return Some(event);
        }
    }
}

mod non_max {
    use std::num::NonZeroU32;

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct NonMaxU32(NonZeroU32);

    impl Default for NonMaxU32 {
        fn default() -> Self {
            Self::new(0).unwrap()
        }
    }

    impl core::fmt::Debug for NonMaxU32 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_tuple("NonMaxU32").field(&self.get()).finish()
        }
    }

    impl NonMaxU32 {
        pub fn new(x: u32) -> Option<Self> {
            if x == u32::MAX {
                None
            } else {
                // Safety: We know that `x+1` is non-zero because it will not
                // overflow because `x` is not `u32::MAX`.
                Some(Self(unsafe { NonZeroU32::new_unchecked(x + 1) }))
            }
        }

        pub fn get(&self) -> u32 {
            self.0.get() - 1
        }
    }
}
use non_max::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Node(u32);
    wasmtime_environ::entity_impl!(Node);

    #[derive(Debug)]
    struct Graph {
        edges: SecondaryMap<Node, Vec<Node>>,
    }

    impl Default for Graph {
        fn default() -> Self {
            let _ = env_logger::try_init();
            Self {
                edges: Default::default(),
            }
        }
    }

    impl Graph {
        fn define(&mut self, node: u32, edges: impl IntoIterator<Item = u32>) -> &mut Self {
            assert!(self.edges[Node::from_u32(node)].is_empty());
            self.edges[Node::from_u32(node)].extend(edges.into_iter().map(|v| Node::from_u32(v)));
            self
        }

        fn edges(&self, node: Node) -> impl Iterator<Item = Node> {
            self.edges[node].iter().copied()
        }
    }

    fn components(graph: &Graph) -> Vec<Vec<u32>> {
        let components = StronglyConnectedComponents::new(graph.edges.keys(), |v| graph.edges(v));
        components
            .values()
            .map(|vs| vs.iter().map(|v| v.as_u32()).collect::<Vec<_>>())
            .collect()
    }

    #[test]
    fn test_empty() {
        let graph = Graph::default();
        assert!(components(&graph).is_empty());
    }

    #[test]
    fn test_single_node() {
        // +---+
        // | 0 |
        // +---+
        let mut graph = Graph::default();
        graph.define(0, []);

        assert_eq!(components(&graph), vec![vec![0]]);
    }

    #[test]
    fn test_single_node_cycle() {
        //   ,---.
        //   |   |
        //   V   |
        // +---+ |
        // | 0 |-'
        // +---+
        let mut graph = Graph::default();
        graph.define(0, [0]);

        assert_eq!(components(&graph), vec![vec![0]]);
    }

    #[test]
    fn test_disconnected_nodes() {
        // +---+     +---+
        // | 0 |     | 1 |
        // +---+     +---+
        let mut graph = Graph::default();
        graph.define(0, []);
        graph.define(1, []);
        assert_eq!(components(&graph), vec![vec![1], vec![0]]);
    }

    #[test]
    fn test_chained_nodes() {
        // +---+   +---+   +---+   +---+
        // | 0 |<--| 1 |<--| 2 |<--| 3 |
        // +---+   +---+   +---+   +---+
        let mut graph = Graph::default();
        graph.define(0, []);
        graph.define(1, [0]);
        graph.define(2, [1]);
        graph.define(3, [2]);
        assert_eq!(components(&graph), vec![vec![0], vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_simple_multi_node_cycle() {
        //   ,-----------------------.
        //   |                       |
        //   |                       V
        // +---+   +---+   +---+   +---+
        // | 0 |<--| 1 |<--| 2 |<--| 3 |
        // +---+   +---+   +---+   +---+
        let mut graph = Graph::default();
        graph.define(0, [3]);
        graph.define(1, [0]);
        graph.define(2, [1]);
        graph.define(3, [2]);
        assert_eq!(components(&graph), vec![vec![0, 1, 2, 3]]);
    }

    #[test]
    fn test_complicated_multi_node_cycle() {
        //   ,---------------.
        //   |               |
        //   |               V
        // +---+   +---+   +---+   +---+   +---+
        // | 0 |<--| 1 |<--| 2 |<--| 3 |<--| 4 |
        // +---+   +---+   +---+   +---+   +---+
        //                   |               ^
        //                   |               |
        //                   `---------------'
        let mut graph = Graph::default();
        graph.define(0, [3]);
        graph.define(1, [0]);
        graph.define(2, [1, 4]);
        graph.define(3, [2]);
        graph.define(4, [3]);
        assert_eq!(components(&graph), vec![vec![0, 1, 2, 3, 4]]);
    }

    #[test]
    fn test_disconnected_cycles() {
        // +---+           +---+
        // | 0 |           | 1 |
        // +---+           +---+
        //   ^               ^
        //   |               |
        //   V               V
        // +---+           +---+
        // | 2 |           | 3 |
        // +---+           +---+
        let mut graph = Graph::default();
        graph.define(0, [2]);
        graph.define(1, [3]);
        graph.define(2, [0]);
        graph.define(3, [1]);
        assert_eq!(components(&graph), vec![vec![1, 3], vec![0, 2]]);
    }

    #[test]
    fn test_chain_of_cycles() {
        //   ,-----.
        //   |     |
        //   V     |
        // +---+   |
        // | 0 |---'
        // +---+
        //   |
        //   V
        // +---+    +---+
        // | 1 |<-->| 2 |
        // +---+    +---+
        //  |
        //  | ,----------------.
        //  | |                |
        //  V |                V
        // +---+    +---+    +---+
        // | 3 |<---| 4 |<---| 5 |
        // +---+    +---+    +---+
        let mut graph = Graph::default();
        graph.define(0, [0, 1]);
        graph.define(1, [2, 3]);
        graph.define(2, [1]);
        graph.define(3, [5]);
        graph.define(4, [3]);
        graph.define(5, [4]);
        assert_eq!(components(&graph), vec![vec![3, 4, 5], vec![1, 2], vec![0]]);
    }

    #[test]
    fn test_multiple_edges_to_same_component() {
        // +---+           +---+
        // | 0 |           | 1 |
        // +---+           +---+
        //   ^               ^
        //   |               |
        //   V               V
        // +---+           +---+
        // | 2 |           | 3 |
        // +---+           +---+
        //   |               |
        //   `------. ,------'
        //          | |
        //          V V
        //         +---+
        //         | 4 |
        //         +---+
        //           ^
        //           |
        //           V
        //         +---+
        //         | 5 |
        //         +---+
        let mut graph = Graph::default();
        graph.define(0, [2]);
        graph.define(1, [3]);
        graph.define(2, [0, 4]);
        graph.define(3, [1, 4]);
        graph.define(4, [5]);
        graph.define(5, [4]);
        assert_eq!(components(&graph), vec![vec![4, 5], vec![1, 3], vec![0, 2]]);
    }

    #[test]
    fn test_duplicate_edges() {
        // +---+           +---+
        // | 0 |           | 1 |
        // +---+           +---+
        //   ^               ^
        //   |               |
        //   V               V
        // +---+           +---+
        // | 2 |---------->| 3 |
        // +---+           +---+
        //   |               ^
        //   `---------------'
        let mut graph = Graph::default();
        graph.define(0, [2]);
        graph.define(1, [3]);
        graph.define(2, [0, 3, 3]);
        graph.define(3, [1]);
        assert_eq!(components(&graph), vec![vec![1, 3], vec![0, 2]]);
    }
}

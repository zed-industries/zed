use crate::grammar::repr::*;
use crate::lr1::core::*;
use crate::lr1::lookahead::Lookahead;
use petgraph::graph::NodeIndex;
use petgraph::prelude::*;
use petgraph::{EdgeDirection, Graph};

// Each state `s` corresponds to the node in the graph with index
// `s`. The edges are the shift transitions.
pub struct StateGraph {
    graph: Graph<(), Symbol>,
}

impl StateGraph {
    pub fn new<L>(states: &[State<L>]) -> StateGraph
    where
        L: Lookahead,
    {
        let nodes = states.len();
        let edges = states.iter().map(|s| s.shifts.len() + s.gotos.len()).sum();
        let mut graph = Graph::with_capacity(nodes, edges);

        // First, create the nodes.
        for i in 0..states.len() {
            let j = graph.add_node(());
            assert_eq!(i, j.index());
        }

        // Add in the edges.
        for (i, state) in states.iter().enumerate() {
            // Successors of a node arise from:
            // - shifts (found in the `conflicts` and `tokens` maps)
            // - gotos (found in the `gotos` map)
            let shifts = state
                .shifts
                .iter()
                .map(|(terminal, &state)| (Symbol::Terminal(terminal.clone()), state));
            let gotos = state
                .gotos
                .iter()
                .map(|(nt, &state)| (Symbol::Nonterminal(nt.clone()), state));
            let it = shifts.chain(gotos).map(|(symbol, successor)| {
                (NodeIndex::new(i), NodeIndex::new(successor.0), symbol)
            });
            graph.extend_with_edges(it);
        }

        StateGraph { graph }
    }

    /// Given a list of symbols `[X, Y, Z]`, traces back from
    /// `initial_state_index` to find the set of states whence we
    /// could have arrived at `initial_state_index` after pushing `X`,
    /// `Y`, and `Z`.
    pub fn trace_back(
        &self,
        initial_state_index: StateIndex,
        initial_symbols: &[Symbol],
    ) -> Vec<StateIndex> {
        let mut stack = vec![(initial_state_index, initial_symbols)];
        let mut result = vec![];
        while let Some((state_index, symbols)) = stack.pop() {
            if let Some((head, tail)) = symbols.split_last() {
                stack.extend(
                    self.graph
                        .edges_directed(NodeIndex::new(state_index.0), EdgeDirection::Incoming)
                        .filter(|edge| edge.weight() == head)
                        .map(|edge| (StateIndex(edge.source().index()), tail)),
                );
            } else {
                result.push(state_index);
            }
        }
        result.sort();
        result.dedup();
        result
    }

    pub fn successors(&self, state_index: StateIndex) -> impl Iterator<Item = StateIndex> + '_ {
        self.graph
            .edges_directed(NodeIndex::new(state_index.0), EdgeDirection::Outgoing)
            .map(|edge| StateIndex(edge.target().index()))
    }

    pub fn predecessors<'a>(
        &'a self,
        state_index: StateIndex,
        symbol: &'a Symbol,
    ) -> impl Iterator<Item = StateIndex> + 'a {
        self.graph
            .edges_directed(NodeIndex::new(state_index.0), EdgeDirection::Incoming)
            .filter(move |edge| edge.weight() == symbol)
            .map(|edge| StateIndex(edge.source().index()))
    }
}

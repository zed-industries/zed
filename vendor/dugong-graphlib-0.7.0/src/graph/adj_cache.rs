//! Adjacency caches used by [`Graph`](super::Graph).
//!
//! These caches exist purely as an optimization: many Dagre algorithms query successors /
//! predecessors repeatedly, and scanning all edges each time is O(E) per query.

#[derive(Debug, Clone)]
pub(in crate::graph) struct DirectedAdjCache {
    pub(in crate::graph) generation: u64,
    pub(in crate::graph) out_offsets: Vec<usize>,
    pub(in crate::graph) out_edges: Vec<usize>,
    pub(in crate::graph) in_offsets: Vec<usize>,
    pub(in crate::graph) in_edges: Vec<usize>,
}

impl DirectedAdjCache {
    pub(in crate::graph) fn out_edges(&self, v_ix: usize) -> &[usize] {
        let start = self.out_offsets[v_ix];
        let end = self.out_offsets[v_ix + 1];
        &self.out_edges[start..end]
    }

    pub(in crate::graph) fn in_edges(&self, v_ix: usize) -> &[usize] {
        let start = self.in_offsets[v_ix];
        let end = self.in_offsets[v_ix + 1];
        &self.in_edges[start..end]
    }
}

#[derive(Debug, Clone)]
pub(in crate::graph) struct UndirectedAdjCache {
    pub(in crate::graph) generation: u64,
    pub(in crate::graph) offsets: Vec<usize>,
    pub(in crate::graph) edges: Vec<usize>,
}

impl UndirectedAdjCache {
    pub(in crate::graph) fn edges(&self, v_ix: usize) -> &[usize] {
        let start = self.offsets[v_ix];
        let end = self.offsets[v_ix + 1];
        &self.edges[start..end]
    }
}

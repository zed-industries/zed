//! Internal storage entries for [`Graph`](super::Graph).

use super::EdgeKey;

#[derive(Debug, Clone)]
pub(in crate::graph) struct NodeEntry<N> {
    pub(in crate::graph) id: String,
    pub(in crate::graph) label: N,
}

#[derive(Debug, Clone)]
pub(in crate::graph) struct EdgeEntry<E> {
    pub(in crate::graph) key: EdgeKey,
    pub(in crate::graph) v_ix: usize,
    pub(in crate::graph) w_ix: usize,
    pub(in crate::graph) label: E,
}

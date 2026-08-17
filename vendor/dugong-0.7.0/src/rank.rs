//! Ranking algorithms (network simplex, tight tree, longest path).
//!
//! Ported from Dagre's `rank.js` and related helpers. The implementation here is parity-oriented
//! (deterministic and defensive) to support headless diagram rendering.

pub mod feasible_tree;
pub mod network_simplex;
pub mod tree;
pub mod util;

pub fn rank(g: &mut crate::graphlib::Graph<crate::NodeLabel, crate::EdgeLabel, crate::GraphLabel>) {
    let ranker = g.graph().ranker.clone();
    match ranker.as_deref() {
        Some("network-simplex") => network_simplex::network_simplex(g),
        Some("tight-tree") => {
            util::longest_path(g);
            let _ = feasible_tree::feasible_tree(g);
        }
        Some("longest-path") => util::longest_path(g),
        Some("none") => {}
        _ => network_simplex::network_simplex(g),
    }
}

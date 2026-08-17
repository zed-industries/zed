//! Layer graph construction for ordering sweeps.
//!
//! This mirrors upstream Dagre's `buildLayerGraph` helper, materializing a rank-local graph used
//! for barycenter-based sorting.

use super::types::OrderNodeLite;
use super::{
    LayerGraphLabel, OrderEdgeWeight, OrderNodeLabel, OrderNodeRange, Relationship, WeightLabel,
};
use crate::graphlib::{Graph, GraphOptions};

pub fn build_layer_graph<N, E, G>(
    g: &Graph<N, E, G>,
    rank: i32,
    relationship: Relationship,
    nodes_with_rank: Option<&[String]>,
) -> Graph<N, WeightLabel, LayerGraphLabel>
where
    N: Default + Clone + 'static + OrderNodeRange,
    E: Default + 'static + OrderEdgeWeight,
    G: Default,
{
    let root = create_root_node(g);
    build_layer_graph_with_root(g, rank, relationship, &root, nodes_with_rank)
}

pub(super) fn build_layer_graph_with_root<N, E, G>(
    g: &Graph<N, E, G>,
    rank: i32,
    relationship: Relationship,
    root: &str,
    nodes_with_rank: Option<&[String]>,
) -> Graph<N, WeightLabel, LayerGraphLabel>
where
    N: Default + Clone + 'static + OrderNodeRange,
    E: Default + 'static + OrderEdgeWeight,
    G: Default,
{
    let mut result: Graph<N, WeightLabel, LayerGraphLabel> = Graph::new(GraphOptions {
        compound: true,
        multigraph: false,
        ..Default::default()
    });
    result.set_graph(LayerGraphLabel {
        root: root.to_string(),
    });
    result.set_node(root.to_string(), N::default());

    let mut visit_node = |v: &str| {
        let node = g.node(v).cloned().unwrap_or_default();
        let parent = g.parent(v);

        let in_range = node.rank() == Some(rank)
            || (node.min_rank().is_some()
                && node.max_rank().is_some()
                && node.min_rank().is_some_and(|min| min <= rank)
                && node.max_rank().is_some_and(|max| rank <= max));

        if !in_range {
            return;
        }

        result.set_node(v.to_string(), node.clone());
        result.set_parent_ref(v, parent.unwrap_or(root));

        match relationship {
            Relationship::InEdges => {
                g.for_each_in_edge(v, None, |ek, lbl| {
                    let u = ek.v.as_str();

                    if !result.has_node(u) {
                        let lbl = g.node(u).cloned().unwrap_or_default();
                        result.set_node(u.to_string(), lbl);
                    }

                    let existing_weight = result.edge(u, v, None).map(|l| l.weight).unwrap_or(0.0);
                    let weight = lbl.weight();
                    result.set_edge_with_label(
                        u.to_string(),
                        v.to_string(),
                        WeightLabel {
                            weight: weight + existing_weight,
                        },
                    );
                });
            }
            Relationship::OutEdges => {
                // Reverse out-edges so `barycenter(...)` can always read `in_edges(...)`.
                g.for_each_out_edge(v, None, |ek, lbl| {
                    let u = ek.w.as_str();

                    if !result.has_node(u) {
                        let lbl = g.node(u).cloned().unwrap_or_default();
                        result.set_node(u.to_string(), lbl);
                    }

                    let existing_weight = result.edge(u, v, None).map(|l| l.weight).unwrap_or(0.0);
                    let weight = lbl.weight();
                    result.set_edge_with_label(
                        u.to_string(),
                        v.to_string(),
                        WeightLabel {
                            weight: weight + existing_weight,
                        },
                    );
                });
            }
        }

        if node.has_min_rank() {
            let override_label = node.subgraph_layer_label(rank);
            result.set_node(v.to_string(), override_label);
        }
    };

    match nodes_with_rank {
        Some(vs) => {
            for v in vs {
                visit_node(v.as_str());
            }
        }
        None => {
            for v in g.nodes() {
                visit_node(v);
            }
        }
    }

    result
}

pub(super) fn build_layer_graph_with_root_lite_ix<N, E, G>(
    g: &Graph<N, E, G>,
    rank: i32,
    relationship: Relationship,
    root: &str,
    nodes_with_rank: &[usize],
) -> Graph<OrderNodeLite, WeightLabel, LayerGraphLabel>
where
    N: Default + 'static + OrderNodeLabel,
    E: Default + 'static + OrderEdgeWeight,
    G: Default,
{
    let root_id = root.to_string();
    let mut result: Graph<OrderNodeLite, WeightLabel, LayerGraphLabel> = Graph::with_capacity(
        GraphOptions {
            compound: true,
            multigraph: false,
            ..Default::default()
        },
        // Root + at least current-rank nodes; edges can add adjacent nodes.
        (nodes_with_rank.len() + 1).saturating_mul(2),
        // Edges in the layer graph are limited to incident edges of current-rank nodes. Avoid
        // reserving based on the full graph's edge count (can be much larger due to dummy nodes).
        nodes_with_rank.len().saturating_mul(4).saturating_add(8),
    );
    result.set_graph(LayerGraphLabel {
        root: root_id.clone(),
    });
    result.set_node(root_id.clone(), OrderNodeLite::default());

    // Note: `nodes_with_rank` is derived from `nodes_by_rank` in `order(...)`, which already
    // materializes nodes that are in-range for this rank (including subgraph nodes with
    // `min_rank..=max_rank`). Avoid re-checking rank ranges here.
    let mut visit_node = |v: &str| {
        let Some(node) = g.node(v) else {
            return;
        };
        let parent = g.parent(v);

        let lbl = if node.has_min_rank() {
            OrderNodeLite::subgraph_layer_label(node, rank)
        } else {
            OrderNodeLite::from_node(node)
        };

        result.set_node(v.to_string(), lbl);
        result.set_parent_ref(v, parent.unwrap_or(root_id.as_str()));

        match relationship {
            Relationship::InEdges => {
                g.for_each_in_edge(v, None, |ek, lbl| {
                    let u = ek.v.as_str();

                    if !result.has_node(u) {
                        let u_lbl = g.node(u).map(OrderNodeLite::from_node).unwrap_or_default();
                        result.set_node(u.to_string(), u_lbl);
                    }

                    let existing_weight = result.edge(u, v, None).map(|l| l.weight).unwrap_or(0.0);
                    let weight = lbl.weight();
                    result.set_edge_with_label(
                        u.to_string(),
                        v.to_string(),
                        WeightLabel {
                            weight: weight + existing_weight,
                        },
                    );
                });
            }
            Relationship::OutEdges => {
                // Reverse out-edges so `barycenter(...)` can always read `in_edges(...)`.
                g.for_each_out_edge(v, None, |ek, lbl| {
                    let u = ek.w.as_str();

                    if !result.has_node(u) {
                        let u_lbl = g.node(u).map(OrderNodeLite::from_node).unwrap_or_default();
                        result.set_node(u.to_string(), u_lbl);
                    }

                    let existing_weight = result.edge(u, v, None).map(|l| l.weight).unwrap_or(0.0);
                    let weight = lbl.weight();
                    result.set_edge_with_label(
                        u.to_string(),
                        v.to_string(),
                        WeightLabel {
                            weight: weight + existing_weight,
                        },
                    );
                });
            }
        }
    };

    for &ix in nodes_with_rank {
        let Some(v) = g.node_id_by_ix(ix) else {
            continue;
        };
        visit_node(v);
    }

    result
}

pub(super) fn create_root_node<N, E, G>(g: &Graph<N, E, G>) -> String
where
    N: Default + 'static,
    E: Default + 'static,
    G: Default,
{
    loop {
        let v = crate::util::unique_id("_root");
        if !g.has_node(&v) {
            return v;
        }
    }
}

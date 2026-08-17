//! Self-edge extraction and reinsertion.
//!
//! Upstream Dagre temporarily removes self-loop edges before ranking/normalization and later
//! re-inserts them via dummy `"selfedge"` nodes during BK positioning. This keeps the ranker
//! constraints valid and makes self-loops deterministic.

use crate::graphlib::Graph;
use crate::{EdgeLabel, GraphLabel, NodeLabel, Point, SelfEdge};

pub fn remove_self_edges(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let self_loop_keys: Vec<_> = g.edges().filter(|ek| ek.v == ek.w).cloned().collect();
    for ek in self_loop_keys {
        if ek.v != ek.w {
            continue;
        }
        let Some(label) = g.edge_by_key(&ek).cloned() else {
            continue;
        };
        if let Some(n) = g.node_mut(&ek.v) {
            n.self_edges.push(SelfEdge {
                edge_obj: ek.clone(),
                label,
            });
        }
        let _ = g.remove_edge_key(&ek);
    }
}

pub fn insert_self_edges(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let layering = crate::util::build_layer_matrix(g);
    for layer in layering {
        let mut extra: usize = 0;
        for (idx, node_id) in layer.iter().enumerate() {
            let Some(rank) = g.node(node_id).and_then(|n| n.rank) else {
                continue;
            };

            if let Some(n) = g.node_mut(node_id) {
                n.order = Some(idx + extra);
            }

            let self_edges = g
                .node(node_id)
                .map(|n| n.self_edges.clone())
                .unwrap_or_default();
            if self_edges.is_empty() {
                continue;
            }
            if let Some(n) = g.node_mut(node_id) {
                n.self_edges.clear();
            }

            for se in self_edges {
                extra += 1;
                let selfedge_id = crate::util::unique_id("_se");
                g.set_node(
                    selfedge_id.clone(),
                    NodeLabel {
                        width: se.label.width,
                        height: se.label.height,
                        rank: Some(rank),
                        order: Some(idx + extra),
                        dummy: Some("selfedge".to_string()),
                        edge_label: Some(se.label.clone()),
                        edge_obj: Some(se.edge_obj.clone()),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

pub fn position_self_edges(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let mut ids: Vec<String> = Vec::new();
    g.for_each_node(|id, n| {
        if n.dummy.as_deref() == Some("selfedge") {
            ids.push(id.to_string());
        }
    });
    for id in ids {
        let Some(node) = g.node(&id).cloned() else {
            continue;
        };
        if node.dummy.as_deref() != Some("selfedge") {
            continue;
        }
        let (Some(x), Some(y)) = (node.x, node.y) else {
            continue;
        };
        let Some(edge_obj) = node.edge_obj.clone() else {
            continue;
        };
        let Some(mut label) = node.edge_label.clone() else {
            continue;
        };
        let Some(v_node) = g.node(&edge_obj.v) else {
            continue;
        };
        let (Some(vx), Some(vy)) = (v_node.x, v_node.y) else {
            continue;
        };

        // Match upstream Dagre (`positionSelfEdges`): do not apply any extra snapping before
        // computing the 2/3 and 5/6 fractions.
        let i = vx + v_node.width / 2.0;
        let a = vy;
        let o = x - i;
        let l = v_node.height / 2.0;

        label.points = vec![
            Point {
                x: i + 2.0 * o / 3.0,
                y: a - l,
            },
            Point {
                x: i + 5.0 * o / 6.0,
                y: a - l,
            },
            Point { x: i + o, y: a },
            Point {
                x: i + 5.0 * o / 6.0,
                y: a + l,
            },
            Point {
                x: i + 2.0 * o / 3.0,
                y: a + l,
            },
        ];
        label.x = Some(x);
        label.y = Some(y);

        g.set_edge_named(
            edge_obj.v.clone(),
            edge_obj.w.clone(),
            edge_obj.name.clone(),
            Some(label),
        );
        let _ = g.remove_node(&id);
    }
}

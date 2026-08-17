//! Compound-graph helpers for layout pipelines.

use crate::graphlib;
use crate::{EdgeLabel, GraphLabel, NodeLabel};

pub(super) fn remove_border_nodes(g: &mut graphlib::Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    // First pass: update compound-node geometry from its border nodes.
    let compound_nodes: Vec<String> = g
        .nodes()
        .filter(|v| !g.children(v).is_empty())
        .map(|s| s.to_string())
        .collect();
    for v in &compound_nodes {
        let Some(node) = g.node(v).cloned() else {
            continue;
        };
        let (Some(bt), Some(bb)) = (node.border_top.clone(), node.border_bottom.clone()) else {
            continue;
        };

        let Some(t) = g.node(&bt) else {
            continue;
        };
        let Some(b) = g.node(&bb) else {
            continue;
        };
        let (Some(ty), Some(by)) = (t.y, b.y) else {
            continue;
        };

        // Dagre derives cluster width from the span of border segments across *all* ranks.
        // Using only the last border node can over-estimate geometry when border nodes drift
        // across ranks (notably in extracted subgraphs without external edges).
        let mut lx: f64 = f64::INFINITY;
        for id in node.border_left.iter().filter_map(|v| v.as_ref()) {
            let Some(n) = g.node(id) else {
                continue;
            };
            let Some(x) = n.x else {
                continue;
            };
            lx = lx.min(x);
        }
        let mut rx: f64 = f64::NEG_INFINITY;
        for id in node.border_right.iter().filter_map(|v| v.as_ref()) {
            let Some(n) = g.node(id) else {
                continue;
            };
            let Some(x) = n.x else {
                continue;
            };
            rx = rx.max(x);
        }
        if !lx.is_finite() || !rx.is_finite() {
            continue;
        }

        let width = (rx - lx).abs();
        let height = (by - ty).abs();
        if let Some(n) = g.node_mut(v) {
            n.width = width;
            n.height = height;
            n.x = Some(lx + width / 2.0);
            n.y = Some(ty + height / 2.0);
        }
    }

    // Second pass: remove all border dummy nodes.
    let mut to_remove: Vec<String> = Vec::new();
    g.for_each_node(|id, n| {
        if n.dummy.as_deref() == Some("border") {
            to_remove.push(id.to_string());
        }
    });
    for v in to_remove {
        let _ = g.remove_node(&v);
    }
}

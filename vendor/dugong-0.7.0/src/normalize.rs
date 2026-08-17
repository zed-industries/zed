//! Normalize long edges by inserting dummy nodes.
//!
//! This mirrors Dagre's `normalize.js` / `undo.js` behavior. It materializes intermediate
//! `"edge"` / `"edge-label"` nodes so ranking/ordering/positioning can treat long edges as a
//! chain of rank-adjacent edges.

use crate::graphlib::{EdgeKey, Graph};
use crate::{EdgeLabel, GraphLabel, NodeLabel, Point};

fn add_dummy_node(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    label: NodeLabel,
    prefix: &str,
) -> Option<String> {
    if !g.has_node(prefix) {
        g.set_node(prefix, label);
        return Some(prefix.to_string());
    }
    for i in 1usize.. {
        let v = format!("{prefix}{i}");
        if !g.has_node(&v) {
            g.set_node(&v, label.clone());
            return Some(v);
        }
    }
    None
}

pub fn run(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    g.graph_mut().dummy_chains.clear();
    let to_normalize: Vec<_> = g
        .edges()
        .filter(|e| {
            let v_rank = g.node(&e.v).and_then(|n| n.rank).unwrap_or(0);
            let w_rank = g.node(&e.w).and_then(|n| n.rank).unwrap_or(0);
            w_rank != v_rank + 1
        })
        .cloned()
        .collect();
    for e in to_normalize {
        normalize_edge(g, e);
    }
}

fn normalize_edge(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>, e: EdgeKey) {
    let v = e.v.clone();
    let w = e.w.clone();
    let name = e.name.clone();

    let v_rank = g.node(&v).and_then(|n| n.rank).unwrap_or(0);
    let w_rank = g.node(&w).and_then(|n| n.rank).unwrap_or(0);
    let Some(mut edge_label) = g.edge_by_key(&e).cloned() else {
        return;
    };
    let label_rank = edge_label.label_rank;

    if w_rank == v_rank + 1 {
        return;
    }

    let _ = g.remove_edge_key(&e);

    edge_label.points.clear();

    let mut prev = v;
    let mut first_dummy: Option<String> = None;
    let mut r = v_rank + 1;

    while r < w_rank {
        let Some(dummy_id) = add_dummy_node(
            g,
            NodeLabel {
                width: 0.0,
                height: 0.0,
                rank: Some(r),
                dummy: Some("edge".to_string()),
                edge_label: Some(edge_label.clone()),
                edge_obj: Some(e.clone()),
                ..Default::default()
            },
            "_d",
        ) else {
            break;
        };

        if first_dummy.is_none() {
            first_dummy = Some(dummy_id.clone());
            g.graph_mut().dummy_chains.push(dummy_id.clone());
        }

        if label_rank == Some(r) {
            if let Some(n) = g.node_mut(&dummy_id) {
                n.width = edge_label.width;
                n.height = edge_label.height;
                n.dummy = Some("edge-label".to_string());
                n.labelpos = Some(edge_label.labelpos);
            }
        }

        g.set_edge_named(
            prev.clone(),
            dummy_id.clone(),
            name.clone(),
            Some(EdgeLabel {
                weight: edge_label.weight,
                ..Default::default()
            }),
        );
        prev = dummy_id;
        r += 1;
    }

    g.set_edge_named(
        prev,
        w,
        name,
        Some(EdgeLabel {
            weight: edge_label.weight,
            ..Default::default()
        }),
    );
}

pub fn undo(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let chains = g.graph().dummy_chains.clone();
    for start in chains {
        let Some(start_node) = g.node(&start) else {
            continue;
        };
        let Some(mut orig_label) = start_node.edge_label.clone() else {
            continue;
        };
        let Some(edge_obj) = start_node.edge_obj.clone() else {
            continue;
        };

        let mut v = start.clone();
        while let Some(node) = g.node(&v) {
            if node.dummy.is_none() {
                break;
            }
            let w = g
                .first_successor(&v)
                .map(|s| s.to_string())
                .unwrap_or_default();

            if let (Some(x), Some(y)) = (node.x, node.y) {
                orig_label.points.push(Point { x, y });
                if node.dummy.as_deref() == Some("edge-label") {
                    orig_label.x = Some(x);
                    orig_label.y = Some(y);
                    orig_label.width = node.width;
                    orig_label.height = node.height;
                }
            }

            let _ = g.remove_node(&v);
            v = w;
            if v.is_empty() {
                break;
            }
        }

        g.set_edge_key(edge_obj, orig_label);
    }
}

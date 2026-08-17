//! Break cycles by reversing a feedback arc set (FAS).
//!
//! This mirrors Dagre's `acyclic.js`. Mermaid uses the default (DFS-based) variant by default,
//! but can opt into the greedy strategy.

use crate::graphlib::{EdgeKey, Graph};
use crate::{EdgeLabel, GraphLabel, NodeLabel};
use std::collections::BTreeSet;

pub fn run(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let fas = if g
        .graph()
        .acyclicer
        .as_deref()
        .is_some_and(|s| s == "greedy")
    {
        crate::greedy_fas::greedy_fas_with_weight(g, |lbl: &EdgeLabel| {
            if !lbl.weight.is_finite() {
                return 0;
            }
            lbl.weight.round() as i64
        })
    } else {
        dfs_fas(g)
    };

    for e in fas.into_iter().filter(|e| e.v != e.w) {
        let Some(label) = g.edge_by_key(&e).cloned() else {
            continue;
        };
        let _ = g.remove_edge_key(&e);

        let mut label = label;
        label.forward_name = e.name.clone();
        label.reversed = true;

        let Some(name) = unique_rev_name(g, &e.w, &e.v) else {
            continue;
        };
        g.set_edge_named(e.w, e.v, Some(name), Some(label));
    }
}

pub fn undo(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let edge_keys = g.edge_keys();
    for e in edge_keys {
        let Some(label) = g.edge_by_key(&e).cloned() else {
            continue;
        };
        if !label.reversed {
            continue;
        }
        let _ = g.remove_edge_key(&e);

        let mut label = label;
        let forward_name = label.forward_name.take();
        label.reversed = false;
        label.points.reverse();
        g.set_edge_named(e.w, e.v, forward_name, Some(label));
    }
}

fn unique_rev_name(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    v: &str,
    w: &str,
) -> Option<String> {
    for i in 1usize.. {
        let candidate = format!("rev{i}");
        if !g.has_edge(v, w, Some(&candidate)) {
            return Some(candidate);
        }
    }
    None
}

fn dfs_fas(g: &Graph<NodeLabel, EdgeLabel, GraphLabel>) -> Vec<EdgeKey> {
    // Ported from Dagre `lib/acyclic.js` (dfsFAS) as used by Mermaid `@11.12.2`.
    let mut fas: Vec<EdgeKey> = Vec::new();
    let mut stack: BTreeSet<String> = BTreeSet::new();
    let mut visited: BTreeSet<String> = BTreeSet::new();

    struct DfsFrame {
        v: String,
        edges: Vec<EdgeKey>,
        next_edge: usize,
    }

    fn push_frame(
        g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
        v: String,
        visited: &mut BTreeSet<String>,
        stack: &mut BTreeSet<String>,
        frames: &mut Vec<DfsFrame>,
    ) {
        visited.insert(v.clone());
        stack.insert(v.clone());
        frames.push(DfsFrame {
            edges: g.out_edges(&v, None),
            v,
            next_edge: 0,
        });
    }

    fn dfs_iterative(
        g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
        root: &str,
        visited: &mut BTreeSet<String>,
        stack: &mut BTreeSet<String>,
        fas: &mut Vec<EdgeKey>,
    ) {
        if visited.contains(root) {
            return;
        }

        let mut frames: Vec<DfsFrame> = Vec::new();
        push_frame(g, root.to_string(), visited, stack, &mut frames);

        while !frames.is_empty() {
            let next = {
                let frame = match frames.last_mut() {
                    Some(frame) => frame,
                    None => break,
                };
                if frame.next_edge < frame.edges.len() {
                    let edge = frame.edges[frame.next_edge].clone();
                    frame.next_edge += 1;
                    Some(edge)
                } else {
                    None
                }
            };

            let Some(e) = next else {
                let Some(frame) = frames.pop() else {
                    break;
                };
                stack.remove(&frame.v);
                continue;
            };

            if e.v == e.w {
                continue;
            }
            if stack.contains(&e.w) {
                fas.push(e);
            } else if !visited.contains(&e.w) {
                push_frame(g, e.w.clone(), visited, stack, &mut frames);
            }
        }
    }

    // Dagre's `dfsFAS` iterates nodes in `g.nodes()` order (insertion order).
    for v in g.nodes() {
        dfs_iterative(g, v, &mut visited, &mut stack, &mut fas);
    }
    fas
}

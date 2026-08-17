//! Nesting graph construction for compound graphs.
//!
//! This mirrors Dagre's `nesting-graph.js`: it creates a synthetic root, adds border nodes for
//! clusters, and injects nesting edges so the ranker sees a connected graph.

use crate::graphlib::{EdgeKey, Graph, alg};
use crate::{EdgeLabel, GraphLabel, NodeLabel};
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;

#[derive(Default)]
struct DummyNodeIdGen {
    next_suffix: FxHashMap<&'static str, usize>,
}

impl DummyNodeIdGen {
    fn unique_id(
        &mut self,
        g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
        prefix: &'static str,
    ) -> String {
        let suffix = match self.next_suffix.get(&prefix).copied() {
            Some(v) => v,
            None => {
                if !g.has_node(prefix) {
                    self.next_suffix.insert(prefix, 1);
                    return prefix.to_string();
                }
                self.next_suffix.insert(prefix, 1);
                1
            }
        };

        // Keep the exact legacy naming scheme (`prefix`, `prefix1`, `prefix2`, ...) but avoid
        // scanning from `1` on every call (which is O(n^2) with repeated allocations).
        let mut next = suffix;
        loop {
            let id = format!("{prefix}{next}");
            if !g.has_node(&id) {
                self.next_suffix.insert(prefix, next + 1);
                return id;
            }
            next += 1;
        }
    }
}

fn add_dummy_node(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    ids: &mut DummyNodeIdGen,
    dummy: &str,
    mut label: NodeLabel,
    name: &'static str,
) -> String {
    let id = ids.unique_id(g, name);
    label.dummy = Some(dummy.to_string());
    g.set_node(id.clone(), label);
    id
}

fn add_border_node(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    ids: &mut DummyNodeIdGen,
    prefix: &'static str,
) -> String {
    add_dummy_node(
        g,
        ids,
        "border",
        NodeLabel {
            width: 0.0,
            height: 0.0,
            ..Default::default()
        },
        prefix,
    )
}

fn tree_depths(g: &Graph<NodeLabel, EdgeLabel, GraphLabel>) -> BTreeMap<String, usize> {
    let mut out: BTreeMap<String, usize> = BTreeMap::new();
    let mut stack: Vec<(String, usize)> = g
        .children_root()
        .into_iter()
        .rev()
        .map(|v| (v.to_string(), 1))
        .collect();

    while let Some((v, depth)) = stack.pop() {
        out.insert(v.clone(), depth);
        let children: Vec<String> = g.children_iter(&v).map(|s| s.to_string()).collect();
        for child in children.into_iter().rev() {
            stack.push((child, depth + 1));
        }
    }

    out
}

enum NestingDfsFrame {
    Enter(String),
    LinkChild {
        parent: String,
        top: String,
        bottom: String,
        child: String,
    },
    LinkRoot {
        node: String,
        top: String,
    },
}

fn add_root_leaf_edge(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    ctx: &NestingDfsCtx<'_>,
    v: &str,
) {
    if v != ctx.root {
        g.set_edge_with_label(
            ctx.root,
            v,
            EdgeLabel {
                weight: 0.0,
                minlen: ctx.node_sep,
                ..Default::default()
            },
        );
    }
}

fn add_child_nesting_edges(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    ctx: &NestingDfsCtx<'_>,
    parent: &str,
    top: &str,
    bottom: &str,
    child: &str,
) {
    let child_node = g.node(child).cloned().unwrap_or_default();
    let child_top = child_node
        .border_top
        .as_deref()
        .unwrap_or(child)
        .to_string();
    let child_bottom = child_node
        .border_bottom
        .as_deref()
        .unwrap_or(child)
        .to_string();
    let this_weight = if child_node.border_top.is_some() {
        ctx.weight
    } else {
        2.0 * ctx.weight
    };
    let minlen = if child_top != child_bottom {
        1usize
    } else {
        let dv = ctx.depths.get(parent).copied().unwrap_or(1);
        ctx.height.saturating_sub(dv).saturating_add(1)
    };

    g.set_edge_with_label(
        top.to_string(),
        child_top,
        EdgeLabel {
            weight: this_weight,
            minlen,
            nesting_edge: true,
            ..Default::default()
        },
    );
    g.set_edge_with_label(
        child_bottom,
        bottom.to_string(),
        EdgeLabel {
            weight: this_weight,
            minlen,
            nesting_edge: true,
            ..Default::default()
        },
    );
}

fn add_root_cluster_edge(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    ctx: &NestingDfsCtx<'_>,
    v: &str,
    top: &str,
) {
    if g.parent(v).is_none() {
        let dv = ctx.depths.get(v).copied().unwrap_or(1);
        g.set_edge_with_label(
            ctx.root,
            top,
            EdgeLabel {
                weight: 0.0,
                minlen: ctx.height + dv,
                nesting_edge: true,
                ..Default::default()
            },
        );
    }
}

fn nesting_dfs(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    ctx: &NestingDfsCtx<'_>,
    ids: &mut DummyNodeIdGen,
    root_child: String,
) {
    let mut stack = vec![NestingDfsFrame::Enter(root_child)];

    while let Some(frame) = stack.pop() {
        match frame {
            NestingDfsFrame::Enter(v) => {
                let children: Vec<String> = g.children_iter(&v).map(|s| s.to_string()).collect();
                if children.is_empty() {
                    add_root_leaf_edge(g, ctx, &v);
                    continue;
                }

                let top = add_border_node(g, ids, "_bt");
                let bottom = add_border_node(g, ids, "_bb");

                g.set_parent_ref(top.as_str(), &v);
                if let Some(lbl) = g.node_mut(&v) {
                    lbl.border_top = Some(top.clone());
                }
                g.set_parent_ref(bottom.as_str(), &v);
                if let Some(lbl) = g.node_mut(&v) {
                    lbl.border_bottom = Some(bottom.clone());
                }

                stack.push(NestingDfsFrame::LinkRoot {
                    node: v.clone(),
                    top: top.clone(),
                });
                for child in children.into_iter().rev() {
                    stack.push(NestingDfsFrame::LinkChild {
                        parent: v.clone(),
                        top: top.clone(),
                        bottom: bottom.clone(),
                        child: child.clone(),
                    });
                    stack.push(NestingDfsFrame::Enter(child));
                }
            }
            NestingDfsFrame::LinkChild {
                parent,
                top,
                bottom,
                child,
            } => add_child_nesting_edges(g, ctx, &parent, &top, &bottom, &child),
            NestingDfsFrame::LinkRoot { node, top } => add_root_cluster_edge(g, ctx, &node, &top),
        }
    }
}

fn sum_weights(g: &Graph<NodeLabel, EdgeLabel, GraphLabel>) -> f64 {
    let mut out: f64 = 0.0;
    g.for_each_edge(|_k, e| out += e.weight);
    out
}

struct NestingDfsCtx<'a> {
    root: &'a str,
    node_sep: usize,
    weight: f64,
    height: usize,
    depths: &'a BTreeMap<String, usize>,
}

pub fn run(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let mut ids = DummyNodeIdGen::default();
    let root = add_dummy_node(
        g,
        &mut ids,
        "root",
        NodeLabel {
            ..Default::default()
        },
        "_root",
    );

    let depths = tree_depths(g);
    let height = depths
        .values()
        .copied()
        .max()
        .unwrap_or(1)
        .saturating_sub(1);
    let node_sep = 2 * height + 1;

    if let Some(gl) = g.graph_mut().nesting_root.replace(root.clone()) {
        let _ = gl;
    }

    g.for_each_edge_mut(|_k, e| {
        e.minlen *= node_sep.max(1);
    });

    let weight = sum_weights(g) + 1.0;
    let ctx = NestingDfsCtx {
        root: &root,
        node_sep,
        weight,
        height,
        depths: &depths,
    };

    let children = g
        .children_root()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    for child in children {
        nesting_dfs(g, &ctx, &mut ids, child);
    }

    g.graph_mut().node_rank_factor = Some(node_sep);

    // Dagre assumes the nesting graph pass makes the graph connected before ranking.
    // Our upstream parity tests include cases where the input graph is not fully connected
    // by the nesting edges alone (e.g. edges incident on cluster nodes). Connect any
    // remaining components through the nesting root so network-simplex does not panic.
    let comps = alg::components(g);
    if comps.len() > 1 {
        for comp in comps {
            if comp.iter().any(|v| v == &root) {
                continue;
            }
            let Some(v) = comp.first() else {
                continue;
            };
            if v == &root {
                continue;
            }
            if g.edge(&root, v, None).is_some() {
                continue;
            }
            g.set_edge_with_label(
                root.clone(),
                v.clone(),
                EdgeLabel {
                    weight: 0.0,
                    // Match Dagre's nesting graph behavior: connect components through the
                    // nesting root using the same `nodeSep`-scaled minlen so rank constraints
                    // remain consistent with compound graphs.
                    minlen: node_sep.max(1),
                    nesting_edge: true,
                    ..Default::default()
                },
            );
        }
    }
}

pub fn cleanup(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let root = g.graph().nesting_root.clone();
    if let Some(root) = root {
        let _ = g.remove_node(&root);
        g.graph_mut().nesting_root = None;
    }

    let mut to_remove: Vec<EdgeKey> = Vec::new();
    g.for_each_edge(|k, e| {
        if e.nesting_edge {
            to_remove.push(k.clone());
        }
    });
    for k in to_remove {
        let _ = g.remove_edge_key(&k);
    }
}

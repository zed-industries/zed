//! Feasible tree construction used by the network simplex ranker.

use super::tree;
use crate::graphlib::{Graph, GraphOptions};
use crate::{EdgeLabel, GraphLabel, NodeLabel};

pub fn feasible_tree(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
) -> Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()> {
    let mut rank_by_ix: Vec<i32> = Vec::new();
    g.for_each_node_ix(|ix, _id, lbl| {
        if ix >= rank_by_ix.len() {
            rank_by_ix.resize(ix + 1, 0);
        }
        rank_by_ix[ix] = lbl.rank.unwrap_or(0);
    });
    let mut in_tree_by_ix: Vec<bool> = vec![false; rank_by_ix.len()];
    let mut tree_g_ixs: Vec<usize> = Vec::new();

    let mut t: Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()> = Graph::new(GraphOptions {
        directed: false,
        ..Default::default()
    });

    let Some(start) = g.nodes().next().map(|s| s.to_string()) else {
        return t;
    };
    let size = g.node_count();
    t.set_node(start.clone(), tree::TreeNodeLabel::default());
    if let Some(ix) = g.node_ix(&start) {
        if ix >= in_tree_by_ix.len() {
            in_tree_by_ix.resize(ix + 1, false);
            rank_by_ix.resize(ix + 1, 0);
        }
        in_tree_by_ix[ix] = true;
        tree_g_ixs.push(ix);
    }

    while tight_tree(&mut t, g, &rank_by_ix, &mut in_tree_by_ix, &mut tree_g_ixs) < size {
        let Some((slack, in_v)) = find_min_slack_edge(g, &rank_by_ix, &in_tree_by_ix) else {
            // Disconnected graphs can occur in downstream usage. Dagre effectively works
            // per component; here we create a forest by starting a new component root.
            let mut next_root: Option<(usize, String)> = None;
            g.for_each_node_ix(|ix, id, _lbl| {
                if next_root.is_some() {
                    return;
                }
                if in_tree_by_ix.get(ix).copied().unwrap_or(false) {
                    return;
                }
                next_root = Some((ix, id.to_string()));
            });
            let Some((ix, next_root)) = next_root else {
                break;
            };
            if ix >= in_tree_by_ix.len() {
                in_tree_by_ix.resize(ix + 1, false);
                rank_by_ix.resize(ix + 1, 0);
            }
            in_tree_by_ix[ix] = true;
            tree_g_ixs.push(ix);
            t.set_node(next_root, tree::TreeNodeLabel::default());
            continue;
        };
        let delta = if in_v { slack } else { -slack };
        shift_ranks(g, &mut rank_by_ix, &tree_g_ixs, delta);
    }

    t
}

fn tight_tree(
    t: &mut Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    rank_by_ix: &[i32],
    in_tree_by_ix: &mut Vec<bool>,
    tree_g_ixs: &mut Vec<usize>,
) -> usize {
    if g.is_directed() {
        let mut stack_ix: Vec<usize> = Vec::new();
        stack_ix.extend(tree_g_ixs.iter().copied());
        while let Some(v_ix) = stack_ix.pop() {
            let Some(v_id) = g.node_id_by_ix(v_ix) else {
                continue;
            };
            let v = v_id.to_string();

            g.for_each_out_edge_ix(v_ix, None, |tail_ix, head_ix, _ek, lbl| {
                if in_tree_by_ix.get(head_ix).copied().unwrap_or(false) {
                    return;
                }

                let tail_rank = rank_by_ix.get(tail_ix).copied().unwrap_or(0);
                let head_rank = rank_by_ix.get(head_ix).copied().unwrap_or(0);
                let minlen: i32 = lbl.minlen.max(1) as i32;
                let slack = head_rank - tail_rank - minlen;
                if slack == 0 {
                    let Some(w_id) = g.node_id_by_ix(head_ix) else {
                        return;
                    };
                    let w = w_id.to_string();

                    stack_ix.push(head_ix);
                    if head_ix >= in_tree_by_ix.len() {
                        in_tree_by_ix.resize(head_ix + 1, false);
                    }
                    in_tree_by_ix[head_ix] = true;
                    tree_g_ixs.push(head_ix);
                    t.set_edge(v.clone(), w);
                }
            });

            g.for_each_in_edge_ix(v_ix, None, |tail_ix, head_ix, _ek, lbl| {
                debug_assert_eq!(head_ix, v_ix);
                if in_tree_by_ix.get(tail_ix).copied().unwrap_or(false) {
                    return;
                }

                let tail_rank = rank_by_ix.get(tail_ix).copied().unwrap_or(0);
                let head_rank = rank_by_ix.get(head_ix).copied().unwrap_or(0);
                let minlen: i32 = lbl.minlen.max(1) as i32;
                let slack = head_rank - tail_rank - minlen;
                if slack == 0 {
                    let Some(w_id) = g.node_id_by_ix(tail_ix) else {
                        return;
                    };
                    let w = w_id.to_string();

                    stack_ix.push(tail_ix);
                    if tail_ix >= in_tree_by_ix.len() {
                        in_tree_by_ix.resize(tail_ix + 1, false);
                    }
                    in_tree_by_ix[tail_ix] = true;
                    tree_g_ixs.push(tail_ix);
                    t.set_edge(v.clone(), w);
                }
            });
        }
    } else {
        let roots: Vec<String> = t.node_ids();
        for root in roots {
            let mut stack: Vec<String> = vec![root];

            while let Some(v) = stack.pop() {
                g.for_each_out_edge(&v, None, |ek, lbl| {
                    let w = if v == ek.v {
                        ek.w.as_str()
                    } else {
                        ek.v.as_str()
                    };
                    if t.has_node(w) {
                        return;
                    }

                    let minlen: i32 = lbl.minlen.max(1) as i32;

                    let Some(tail_ix) = g.node_ix(&ek.v) else {
                        return;
                    };
                    let Some(head_ix) = g.node_ix(&ek.w) else {
                        return;
                    };
                    let tail_rank = rank_by_ix.get(tail_ix).copied().unwrap_or(0);
                    let head_rank = rank_by_ix.get(head_ix).copied().unwrap_or(0);
                    let slack = head_rank - tail_rank - minlen;
                    if slack == 0 {
                        let w = w.to_string();
                        stack.push(w.clone());
                        if let Some(w_ix) = g.node_ix(&w) {
                            if w_ix >= in_tree_by_ix.len() {
                                in_tree_by_ix.resize(w_ix + 1, false);
                            }
                            in_tree_by_ix[w_ix] = true;
                            tree_g_ixs.push(w_ix);
                        }
                        t.set_edge(v.clone(), w);
                    }
                });
            }
        }
    }
    t.node_count()
}

fn find_min_slack_edge(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    rank_by_ix: &[i32],
    in_tree_by_ix: &[bool],
) -> Option<(i32, bool)> {
    let mut best: Option<(i32, bool)> = None;
    g.for_each_edge_ix(|v_ix, w_ix, _key, lbl| {
        let in_v = in_tree_by_ix.get(v_ix).copied().unwrap_or(false);
        let in_w = in_tree_by_ix.get(w_ix).copied().unwrap_or(false);
        if in_v == in_w {
            return;
        }

        let v_rank = rank_by_ix.get(v_ix).copied().unwrap_or(0);
        let w_rank = rank_by_ix.get(w_ix).copied().unwrap_or(0);
        let minlen: i32 = lbl.minlen.max(1) as i32;
        let slack = w_rank - v_rank - minlen;

        match &best {
            Some((best_slack, _)) if slack >= *best_slack => {}
            _ => best = Some((slack, in_v)),
        }
    });
    best
}

fn shift_ranks(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    rank_by_ix: &mut Vec<i32>,
    tree_g_ixs: &[usize],
    delta: i32,
) {
    for &ix in tree_g_ixs {
        if ix >= rank_by_ix.len() {
            rank_by_ix.resize(ix + 1, 0);
        }
        let new_rank = rank_by_ix[ix] + delta;
        rank_by_ix[ix] = new_rank;
        if let Some(label) = g.node_label_mut_by_ix(ix) {
            label.rank = Some(new_rank);
        }
    }
}

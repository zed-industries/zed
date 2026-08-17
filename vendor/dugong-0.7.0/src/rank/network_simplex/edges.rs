//! Network simplex edge exchange helpers.
//!
//! This contains the classic Dagre-style `leave_edge` / `enter_edge` / `exchange_edges` routines,
//! which are kept for parity and for potential debugging, even though the main implementation
//! in this crate uses a faster incremental update path.

use super::tree;
use crate::graphlib::{EdgeKey, Graph, alg};
use crate::{EdgeLabel, GraphLabel, NodeLabel};

pub fn leave_edge(t: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>) -> Option<EdgeKey> {
    t.edges()
        .find(|e| {
            t.edge_by_key(e)
                .map(|lbl| lbl.cutvalue < 0.0)
                .unwrap_or(false)
        })
        .cloned()
}

pub fn enter_edge(
    t: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    rank_by_ix: &[i32],
    edge: &EdgeKey,
) -> EdgeKey {
    let (v, w) = if let (Some(v_ix), Some(w_ix)) = (g.node_ix(&edge.v), g.node_ix(&edge.w)) {
        if g.has_edge_ix(v_ix, w_ix) {
            (edge.v.as_str(), edge.w.as_str())
        } else {
            (edge.w.as_str(), edge.v.as_str())
        }
    } else if g.has_edge(&edge.v, &edge.w, None) {
        (edge.v.as_str(), edge.w.as_str())
    } else {
        (edge.w.as_str(), edge.v.as_str())
    };

    let mut t_low_by_gix: Vec<i32> = Vec::new();
    let mut t_lim_by_gix: Vec<i32> = Vec::new();
    let mut t_has_by_gix: Vec<bool> = Vec::new();
    for id in t.nodes() {
        let Some(lbl) = t.node(id) else {
            continue;
        };
        let Some(g_ix) = g.node_ix(id) else {
            continue;
        };
        if g_ix >= t_has_by_gix.len() {
            t_has_by_gix.resize(g_ix + 1, false);
            t_low_by_gix.resize(g_ix + 1, 0);
            t_lim_by_gix.resize(g_ix + 1, 0);
        }
        t_has_by_gix[g_ix] = true;
        t_low_by_gix[g_ix] = lbl.low;
        t_lim_by_gix[g_ix] = lbl.lim;
    }

    let Some(v_gix) = g.node_ix(v) else {
        return edge.clone();
    };
    let Some(w_gix) = g.node_ix(w) else {
        return edge.clone();
    };

    if !t_has_by_gix.get(v_gix).copied().unwrap_or(false) {
        return edge.clone();
    }
    if !t_has_by_gix.get(w_gix).copied().unwrap_or(false) {
        return edge.clone();
    }

    let v_low = t_low_by_gix.get(v_gix).copied().unwrap_or(0);
    let v_lim = t_lim_by_gix.get(v_gix).copied().unwrap_or(0);
    let w_low = t_low_by_gix.get(w_gix).copied().unwrap_or(0);
    let w_lim = t_lim_by_gix.get(w_gix).copied().unwrap_or(0);

    let ((tail_low, tail_lim), flip) = if v_lim > w_lim {
        ((w_low, w_lim), true)
    } else {
        ((v_low, v_lim), false)
    };

    let mut best: Option<(i32, EdgeKey)> = None;
    g.for_each_edge_ix(|g_v_ix, g_w_ix, key, lbl| {
        if !t_has_by_gix.get(g_v_ix).copied().unwrap_or(false) {
            return;
        };
        if !t_has_by_gix.get(g_w_ix).copied().unwrap_or(false) {
            return;
        };
        let v_lim = t_lim_by_gix.get(g_v_ix).copied().unwrap_or(0);
        let w_lim = t_lim_by_gix.get(g_w_ix).copied().unwrap_or(0);
        let v_desc = tail_low <= v_lim && v_lim <= tail_lim;
        let w_desc = tail_low <= w_lim && w_lim <= tail_lim;

        if flip == v_desc && flip != w_desc {
            let v_rank = rank_by_ix.get(g_v_ix).copied().unwrap_or(0);
            let w_rank = rank_by_ix.get(g_w_ix).copied().unwrap_or(0);
            let minlen: i32 = (lbl.minlen.max(1)) as i32;
            let slack = w_rank - v_rank - minlen;

            match &best {
                Some((best_slack, _)) if slack >= *best_slack => {}
                _ => best = Some((slack, key.clone())),
            }
        }
    });

    best.map(|(_, e)| e).unwrap_or_else(|| edge.clone())
}

pub fn exchange_edges(
    t: &mut Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    rank_by_ix: &mut Vec<i32>,
    e: &EdgeKey,
    f: &EdgeKey,
) {
    let _ = t.remove_edge(&e.v, &e.w, None);
    t.set_edge(f.v.clone(), f.w.clone());
    super::init_low_lim_values(t, None);
    super::init_cut_values(t, g);
    update_ranks(t, g, rank_by_ix);
}

fn update_ranks(
    t: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    rank_by_ix: &mut Vec<i32>,
) {
    let Some(root) = t
        .nodes()
        .find(|v| t.node(v).map(|lbl| lbl.parent.is_none()).unwrap_or(false))
        .or_else(|| t.nodes().next())
    else {
        return;
    };

    let vs = alg::preorder(t, &[root]);
    for v in vs.into_iter().skip(1) {
        let Some(parent) = t.node(&v).and_then(|lbl| lbl.parent.clone()) else {
            continue;
        };

        let Some(v_ix) = g.node_ix(&v) else {
            continue;
        };
        let Some(parent_ix) = g.node_ix(&parent) else {
            continue;
        };
        let (minlen, flipped) = if let Some(e) = g.edge_by_endpoints_ix(v_ix, parent_ix) {
            (e.minlen as i32, false)
        } else if let Some(e) = g.edge_by_endpoints_ix(parent_ix, v_ix) {
            (e.minlen as i32, true)
        } else {
            continue;
        };

        let parent_rank = rank_by_ix.get(parent_ix).copied().unwrap_or(0);
        let rank = if flipped {
            parent_rank + minlen
        } else {
            parent_rank - minlen
        };
        if let Some(node) = g.node_mut(&v) {
            node.rank = Some(rank);
        }
        if let Some(ix) = g.node_ix(&v) {
            if ix >= rank_by_ix.len() {
                rank_by_ix.resize(ix + 1, 0);
            }
            rank_by_ix[ix] = rank;
        }
    }
}

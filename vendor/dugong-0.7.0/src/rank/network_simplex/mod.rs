//! Network simplex ranker (Dagre-compatible).

use super::{feasible_tree, tree, util};
use crate::graphlib::{EdgeKey, Graph, alg};
use crate::{EdgeLabel, GraphLabel, NodeLabel};

mod edges;

#[derive(Debug, Clone, Copy)]
struct DfsFrame {
    v_ix: usize,
    parent_ix: Option<usize>,
    low: i32,
    next_neighbor: usize,
}

#[derive(Debug, Clone)]
struct TreeState {
    /// Tree node index -> graph node index.
    g_ix_by_t_ix: Vec<Option<usize>>,
    /// Graph node index -> tree node index.
    t_ix_by_g_ix: Vec<Option<usize>>,

    parent_t_ix: Vec<Option<usize>>,
    low: Vec<i32>,
    lim: Vec<i32>,

    /// Cut value for the tree edge between this node and its parent (roots have 0.0).
    cut_to_parent: Vec<f64>,

    roots: Vec<usize>,

    // Reused scratch buffers to avoid repeated allocations in the simplex loop.
    node_ixs: Vec<usize>,
    roots_to_visit: Vec<usize>,
    visited: Vec<bool>,
    neighbors: Vec<Vec<usize>>,
    dfs_stack: Vec<DfsFrame>,

    in_tree_by_g_ix: Vec<bool>,
    low_by_g_ix: Vec<i32>,
    lim_by_g_ix: Vec<i32>,
    parent_g_ix_by_g_ix: Vec<Option<usize>>,
    cut_to_parent_by_g_ix: Vec<f64>,
    parent_edge_present_by_t_ix: Vec<bool>,
    parent_edge_weight_by_t_ix: Vec<f64>,
    child_is_tail_to_parent_by_t_ix: Vec<bool>,
    g_ix_by_lim: Vec<Option<usize>>,
    tail_g_ixs: Vec<usize>,

    children: Vec<Vec<usize>>,
    postorder: Vec<usize>,
    post_stack: Vec<(usize, usize)>,
    rank_stack: Vec<usize>,

    tree_edge_ends_in_order: Vec<(usize, usize)>,
    leave_edge_ends_in_order: Option<(usize, usize)>,
}

impl TreeState {
    fn new(
        t: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
        g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    ) -> Self {
        let mut max_t_ix: usize = 0;
        t.for_each_node_ix(|t_ix, _id, _lbl| {
            max_t_ix = max_t_ix.max(t_ix);
        });
        let t_len = max_t_ix.saturating_add(1);

        let mut max_g_ix: usize = 0;
        g.for_each_node_ix(|g_ix, _id, _lbl| {
            max_g_ix = max_g_ix.max(g_ix);
        });
        let g_len = max_g_ix.saturating_add(1);

        let mut g_ix_by_t_ix: Vec<Option<usize>> = vec![None; t_len];
        let mut t_ix_by_g_ix: Vec<Option<usize>> = vec![None; g_len];
        t.for_each_node_ix(|t_ix, id, _lbl| {
            let Some(g_ix) = g.node_ix(id) else {
                return;
            };
            if t_ix >= g_ix_by_t_ix.len() {
                g_ix_by_t_ix.resize(t_ix + 1, None);
            }
            g_ix_by_t_ix[t_ix] = Some(g_ix);
            if g_ix >= t_ix_by_g_ix.len() {
                t_ix_by_g_ix.resize(g_ix + 1, None);
            }
            t_ix_by_g_ix[g_ix] = Some(t_ix);
        });

        Self {
            g_ix_by_t_ix,
            t_ix_by_g_ix,
            parent_t_ix: vec![None; t_len],
            low: vec![0; t_len],
            lim: vec![0; t_len],
            cut_to_parent: vec![0.0; t_len],
            roots: Vec::new(),
            node_ixs: Vec::new(),
            roots_to_visit: Vec::new(),
            visited: vec![false; t_len],
            neighbors: vec![Vec::new(); t_len],
            dfs_stack: Vec::new(),
            in_tree_by_g_ix: vec![false; g_len],
            low_by_g_ix: vec![0; g_len],
            lim_by_g_ix: vec![0; g_len],
            parent_g_ix_by_g_ix: vec![None; g_len],
            cut_to_parent_by_g_ix: vec![0.0; g_len],
            parent_edge_present_by_t_ix: vec![false; t_len],
            parent_edge_weight_by_t_ix: vec![0.0; t_len],
            child_is_tail_to_parent_by_t_ix: vec![true; t_len],
            g_ix_by_lim: vec![None; t_len + 1],
            tail_g_ixs: Vec::new(),
            children: vec![Vec::new(); t_len],
            postorder: Vec::new(),
            post_stack: Vec::new(),
            rank_stack: Vec::new(),
            tree_edge_ends_in_order: Vec::new(),
            leave_edge_ends_in_order: None,
        }
    }

    fn node_low_lim_by_gix(&self, g_ix: usize) -> Option<(i32, i32)> {
        if !self.in_tree_by_g_ix.get(g_ix).copied().unwrap_or(false) {
            return None;
        }
        Some((
            self.low_by_g_ix.get(g_ix).copied()?,
            self.lim_by_g_ix.get(g_ix).copied()?,
        ))
    }

    fn rebuild(
        &mut self,
        t: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
        g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
        root: Option<&str>,
    ) {
        // The tree structure changes across iterations, but the node set is stable.
        let mut max_t_ix: usize = 0;
        t.for_each_node_ix(|t_ix, _id, _lbl| {
            max_t_ix = max_t_ix.max(t_ix);
        });
        let t_len = max_t_ix.saturating_add(1);

        self.parent_t_ix.resize(t_len, None);
        self.low.resize(t_len, 0);
        self.lim.resize(t_len, 0);
        self.cut_to_parent.resize(t_len, 0.0);
        self.roots.clear();
        self.parent_t_ix.fill(None);
        self.low.fill(0);
        self.lim.fill(0);
        self.cut_to_parent.fill(0.0);

        // Rebuild index mappings defensively in case `t` changed shape.
        self.g_ix_by_t_ix.resize(t_len, None);
        self.g_ix_by_t_ix.fill(None);
        self.t_ix_by_g_ix.fill(None);

        self.node_ixs.clear();
        t.for_each_node_ix(|t_ix, id, _lbl| {
            let g_ix = g.node_ix(id);
            self.g_ix_by_t_ix[t_ix] = g_ix;
            if let Some(g_ix) = g_ix {
                if g_ix >= self.t_ix_by_g_ix.len() {
                    self.t_ix_by_g_ix.resize(g_ix + 1, None);
                }
                self.t_ix_by_g_ix[g_ix] = Some(t_ix);
            }
            self.node_ixs.push(t_ix);
        });

        // Build a stable adjacency list for the current tree edges.
        self.neighbors.resize_with(t_len, Vec::new);
        self.neighbors.truncate(t_len);
        for ns in &mut self.neighbors {
            ns.clear();
        }
        self.tree_edge_ends_in_order.clear();
        self.tree_edge_ends_in_order.reserve(t.edge_count());
        t.for_each_edge_ix(|v_ix, w_ix, _key, _lbl| {
            if v_ix >= self.neighbors.len() || w_ix >= self.neighbors.len() {
                return;
            }
            self.neighbors[v_ix].push(w_ix);
            self.neighbors[w_ix].push(v_ix);
            self.tree_edge_ends_in_order.push((v_ix, w_ix));
        });

        self.visited.resize(t_len, false);
        self.visited.fill(false);
        let mut next_lim: i32 = 1;

        let preferred_root_ix: Option<usize> = root
            .and_then(|id| t.node_ix(id))
            .or_else(|| self.node_ixs.first().copied());

        self.roots_to_visit.clear();
        if let Some(ix) = preferred_root_ix {
            self.roots_to_visit.push(ix);
        }
        self.roots_to_visit.extend(self.node_ixs.iter().copied());

        for &start_ix in &self.roots_to_visit {
            if start_ix >= self.visited.len() || self.visited[start_ix] {
                continue;
            }
            if t.node_id_by_ix(start_ix).is_none() {
                continue;
            }
            self.roots.push(start_ix);
            self.visited[start_ix] = true;

            self.dfs_stack.clear();
            self.dfs_stack.push(DfsFrame {
                v_ix: start_ix,
                parent_ix: None,
                low: next_lim,
                next_neighbor: 0,
            });

            while !self.dfs_stack.is_empty() {
                let next_child = {
                    let Some(top) = self.dfs_stack.last_mut() else {
                        break;
                    };
                    self.neighbors
                        .get(top.v_ix)
                        .and_then(|ns| ns.get(top.next_neighbor))
                        .copied()
                        .inspect(|_| top.next_neighbor += 1)
                        .map(|w_ix| (w_ix, top.v_ix, top.parent_ix))
                };

                if let Some((w_ix, parent_v_ix, parent_ix)) = next_child {
                    if parent_ix.is_some_and(|p| p == w_ix) {
                        continue;
                    }
                    if w_ix >= self.visited.len() || self.visited[w_ix] {
                        continue;
                    }
                    self.visited[w_ix] = true;
                    self.parent_t_ix[w_ix] = Some(parent_v_ix);
                    self.dfs_stack.push(DfsFrame {
                        v_ix: w_ix,
                        parent_ix: Some(parent_v_ix),
                        low: next_lim,
                        next_neighbor: 0,
                    });
                    continue;
                }

                let Some(frame) = self.dfs_stack.pop() else {
                    break;
                };
                let DfsFrame {
                    v_ix,
                    parent_ix: _,
                    low,
                    next_neighbor: _,
                } = frame;
                self.low[v_ix] = low;
                self.lim[v_ix] = next_lim;
                next_lim += 1;
            }
        }

        self.in_tree_by_g_ix.resize(self.t_ix_by_g_ix.len(), false);
        self.low_by_g_ix.resize(self.t_ix_by_g_ix.len(), 0);
        self.lim_by_g_ix.resize(self.t_ix_by_g_ix.len(), 0);
        self.parent_g_ix_by_g_ix
            .resize(self.t_ix_by_g_ix.len(), None);
        self.cut_to_parent_by_g_ix
            .resize(self.t_ix_by_g_ix.len(), 0.0);
        self.in_tree_by_g_ix.fill(false);
        self.low_by_g_ix.fill(0);
        self.lim_by_g_ix.fill(0);
        self.parent_g_ix_by_g_ix.fill(None);
        // `cut_to_parent_by_g_ix` is written in postorder during `rebuild_cut_values`;
        // roots are never read through this mapping, so we don't need to clear the whole
        // buffer here.

        for &t_ix in &self.node_ixs {
            let Some(g_ix) = self.g_ix_by_t_ix.get(t_ix).copied().flatten() else {
                continue;
            };
            if g_ix >= self.in_tree_by_g_ix.len() {
                continue;
            }
            self.in_tree_by_g_ix[g_ix] = true;
            self.low_by_g_ix[g_ix] = self.low.get(t_ix).copied().unwrap_or(0);
            self.lim_by_g_ix[g_ix] = self.lim.get(t_ix).copied().unwrap_or(0);
        }

        self.parent_edge_present_by_t_ix.resize(t_len, false);
        self.parent_edge_weight_by_t_ix.resize(t_len, 0.0);
        self.child_is_tail_to_parent_by_t_ix.resize(t_len, true);
        self.parent_edge_present_by_t_ix.fill(false);
        self.parent_edge_weight_by_t_ix.fill(0.0);
        self.child_is_tail_to_parent_by_t_ix.fill(true);

        for (child_tix, parent_tix) in self.parent_t_ix.iter().copied().enumerate() {
            let Some(parent_tix) = parent_tix else {
                continue;
            };
            let Some(child_gix) = self.g_ix_by_t_ix.get(child_tix).copied().flatten() else {
                continue;
            };
            let Some(parent_gix) = self.g_ix_by_t_ix.get(parent_tix).copied().flatten() else {
                continue;
            };
            if child_gix < self.parent_g_ix_by_g_ix.len() {
                self.parent_g_ix_by_g_ix[child_gix] = Some(parent_gix);
            }

            let (present, weight, child_is_tail) =
                if let Some(e) = g.edge_by_endpoints_ix(child_gix, parent_gix) {
                    (true, e.weight, true)
                } else if let Some(e) = g.edge_by_endpoints_ix(parent_gix, child_gix) {
                    (true, e.weight, false)
                } else {
                    (false, 0.0, true)
                };

            if child_tix < self.parent_edge_present_by_t_ix.len() {
                self.parent_edge_present_by_t_ix[child_tix] = present;
                self.parent_edge_weight_by_t_ix[child_tix] = weight;
                self.child_is_tail_to_parent_by_t_ix[child_tix] = child_is_tail;
            }
        }

        self.g_ix_by_lim.resize(t_len + 1, None);
        self.g_ix_by_lim.fill(None);
        for &t_ix in &self.node_ixs {
            let Some(g_ix) = self.g_ix_by_t_ix.get(t_ix).copied().flatten() else {
                continue;
            };
            let lim = self.lim.get(t_ix).copied().unwrap_or(0);
            let Ok(lim) = usize::try_from(lim) else {
                continue;
            };
            if lim < self.g_ix_by_lim.len() {
                self.g_ix_by_lim[lim] = Some(g_ix);
            }
        }

        self.rebuild_cut_values(t, g);
    }

    fn rebuild_cut_values(
        &mut self,
        t: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
        g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    ) {
        let t_len = self.parent_t_ix.len();
        self.children.resize_with(t_len, Vec::new);
        self.children.truncate(t_len);
        for ch in &mut self.children {
            ch.clear();
        }
        for (child_ix, parent_ix) in self.parent_t_ix.iter().copied().enumerate() {
            let Some(parent_ix) = parent_ix else {
                continue;
            };
            if parent_ix < self.children.len() {
                self.children[parent_ix].push(child_ix);
            }
        }

        // Postorder traversal for each tree component.
        self.postorder.clear();
        for &root_ix in &self.roots {
            if root_ix >= t_len {
                continue;
            }
            if t.node_id_by_ix(root_ix).is_none() {
                continue;
            }

            self.post_stack.clear();
            self.post_stack.push((root_ix, 0));
            while let Some((v_ix, idx)) = self.post_stack.last_mut() {
                let next_child = self
                    .children
                    .get(*v_ix)
                    .and_then(|ch| ch.get(*idx))
                    .copied();
                if let Some(w_ix) = next_child {
                    *idx += 1;
                    self.post_stack.push((w_ix, 0));
                    continue;
                }
                let Some((v_ix, _idx)) = self.post_stack.pop() else {
                    break;
                };
                self.postorder.push(v_ix);
            }
        }

        self.cut_to_parent_by_g_ix
            .resize(self.t_ix_by_g_ix.len(), 0.0);
        // `cut_to_parent_by_g_ix` is populated in postorder below; we intentionally avoid
        // clearing the whole buffer here to keep rebuild costs down.
        for &child_tix in &self.postorder {
            if self.parent_t_ix.get(child_tix).copied().flatten().is_none() {
                continue;
            }
            let cut = self.calc_cut_value_by_tix(t, g, child_tix);
            if child_tix < self.cut_to_parent.len() {
                self.cut_to_parent[child_tix] = cut;
            }
            if let Some(child_gix) = self.g_ix_by_t_ix.get(child_tix).copied().flatten() {
                if child_gix < self.cut_to_parent_by_g_ix.len() {
                    self.cut_to_parent_by_g_ix[child_gix] = cut;
                }
            }
        }

        self.leave_edge_ends_in_order = None;
        for &(u_ix, v_ix) in &self.tree_edge_ends_in_order {
            let child_tix = if self.parent_t_ix.get(u_ix).copied().flatten() == Some(v_ix) {
                Some(u_ix)
            } else if self.parent_t_ix.get(v_ix).copied().flatten() == Some(u_ix) {
                Some(v_ix)
            } else {
                None
            };
            let Some(child_tix) = child_tix else {
                continue;
            };
            if self.cut_to_parent.get(child_tix).copied().unwrap_or(0.0) < 0.0 {
                self.leave_edge_ends_in_order = Some((u_ix, v_ix));
                break;
            }
        }
    }

    fn calc_cut_value_by_tix(
        &self,
        _t: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
        g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
        child_tix: usize,
    ) -> f64 {
        let Some(parent_tix) = self.parent_t_ix.get(child_tix).copied().flatten() else {
            return 0.0;
        };
        let Some(child_gix) = self.g_ix_by_t_ix.get(child_tix).copied().flatten() else {
            return 0.0;
        };
        let Some(parent_gix) = self.g_ix_by_t_ix.get(parent_tix).copied().flatten() else {
            return 0.0;
        };

        if !self
            .parent_edge_present_by_t_ix
            .get(child_tix)
            .copied()
            .unwrap_or(false)
        {
            return 0.0;
        }
        let child_is_tail = self
            .child_is_tail_to_parent_by_t_ix
            .get(child_tix)
            .copied()
            .unwrap_or(true);
        let mut cut_value = self
            .parent_edge_weight_by_t_ix
            .get(child_tix)
            .copied()
            .unwrap_or(0.0);

        if g.is_directed() {
            let parent_g_ix_by_g_ix = &self.parent_g_ix_by_g_ix;
            let cut_to_parent_by_g_ix = &self.cut_to_parent_by_g_ix;
            let out_sign: f64 = if child_is_tail { 1.0 } else { -1.0 };
            let in_sign: f64 = -out_sign;

            g.for_each_out_edge_ix(child_gix, None, |_tail_ix, head_ix, _ek, lbl| {
                if head_ix == parent_gix {
                    return;
                }

                cut_value += out_sign * lbl.weight;

                let (Some(parent), Some(other_cut_value)) = (
                    parent_g_ix_by_g_ix.get(head_ix),
                    cut_to_parent_by_g_ix.get(head_ix),
                ) else {
                    return;
                };
                if *parent == Some(child_gix) {
                    cut_value += -out_sign * *other_cut_value;
                }
            });

            g.for_each_in_edge_ix(child_gix, None, |tail_ix, _head_ix, _ek, lbl| {
                if tail_ix == parent_gix {
                    return;
                }

                cut_value += in_sign * lbl.weight;

                let (Some(parent), Some(other_cut_value)) = (
                    parent_g_ix_by_g_ix.get(tail_ix),
                    cut_to_parent_by_g_ix.get(tail_ix),
                ) else {
                    return;
                };
                if *parent == Some(child_gix) {
                    cut_value += -in_sign * *other_cut_value;
                }
            });
        }

        cut_value
    }

    fn find_leave_edge_in_insertion_order(
        &self,
        _t: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
    ) -> Option<(usize, usize)> {
        self.leave_edge_ends_in_order
    }
}

pub fn network_simplex(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let mut simplified = crate::util::simplify(g);
    util::longest_path(&mut simplified);
    let mut t = feasible_tree::feasible_tree(&mut simplified);
    let mut t_state = TreeState::new(&t, &simplified);
    t_state.rebuild(&t, &simplified, None);

    let mut rank_by_ix: Vec<i32> = Vec::new();
    simplified.for_each_node_ix(|g_ix, _id, lbl| {
        if g_ix >= rank_by_ix.len() {
            rank_by_ix.resize(g_ix + 1, 0);
        }
        rank_by_ix[g_ix] = lbl.rank.unwrap_or(0);
    });

    while let Some((leave_u_tix, leave_v_tix)) = t_state.find_leave_edge_in_insertion_order(&t) {
        let leave_u_id = t.node_id_by_ix(leave_u_tix);
        let leave_v_id = t.node_id_by_ix(leave_v_tix);
        let Some((leave_u_id, leave_v_id)) = leave_u_id.zip(leave_v_id) else {
            break;
        };
        let leave_u_id = leave_u_id.to_string();
        let leave_v_id = leave_v_id.to_string();
        let f = enter_edge_fast(
            &mut t_state,
            &simplified,
            &rank_by_ix,
            leave_u_tix,
            leave_v_tix,
        );

        let _ = t.remove_edge(&leave_u_id, &leave_v_id, None);
        t.set_edge(f.v, f.w);

        t_state.rebuild(&t, &simplified, None);
        update_ranks_fast(&mut t_state, &mut simplified, &mut rank_by_ix);
    }

    for v in g.node_ids() {
        if let Some(rank) = simplified.node(&v).and_then(|n| n.rank) {
            if let Some(lbl) = g.node_mut(&v) {
                lbl.rank = Some(rank);
            }
        }
    }
}

fn enter_edge_fast(
    t_state: &mut TreeState,
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    rank_by_ix: &[i32],
    leave_u_tix: usize,
    leave_v_tix: usize,
) -> EdgeKey {
    let fallback = EdgeKey {
        v: t_state
            .g_ix_by_t_ix
            .get(leave_u_tix)
            .copied()
            .flatten()
            .and_then(|ix| g.node_id_by_ix(ix))
            .unwrap_or("")
            .to_string(),
        w: t_state
            .g_ix_by_t_ix
            .get(leave_v_tix)
            .copied()
            .flatten()
            .and_then(|ix| g.node_id_by_ix(ix))
            .unwrap_or("")
            .to_string(),
        name: None,
    };
    let Some(leave_u_gix) = t_state.g_ix_by_t_ix.get(leave_u_tix).copied().flatten() else {
        return fallback;
    };
    let Some(leave_v_gix) = t_state.g_ix_by_t_ix.get(leave_v_tix).copied().flatten() else {
        return fallback;
    };

    // Orient the leaving tree edge according to the graph's directed edge direction.
    let (v_gix, w_gix) = if g.has_edge_ix(leave_u_gix, leave_v_gix) {
        (leave_u_gix, leave_v_gix)
    } else {
        (leave_v_gix, leave_u_gix)
    };

    let Some((v_low, v_lim)) = t_state.node_low_lim_by_gix(v_gix) else {
        return fallback;
    };
    let Some((w_low, w_lim)) = t_state.node_low_lim_by_gix(w_gix) else {
        return fallback;
    };

    let ((tail_low, tail_lim), flip) = if v_lim > w_lim {
        ((w_low, w_lim), true)
    } else {
        ((v_low, v_lim), false)
    };

    let is_in_tail = |t_state: &TreeState, g_ix: usize| -> bool {
        if !t_state.in_tree_by_g_ix.get(g_ix).copied().unwrap_or(false) {
            return false;
        }
        let lim = t_state.lim_by_g_ix.get(g_ix).copied().unwrap_or(0);
        tail_low <= lim && lim <= tail_lim
    };

    t_state.tail_g_ixs.clear();
    let Ok(tail_low) = usize::try_from(tail_low) else {
        return fallback;
    };
    let Ok(tail_lim) = usize::try_from(tail_lim) else {
        return fallback;
    };
    if tail_low == 0 || tail_low > tail_lim {
        return fallback;
    }
    let max_lim = t_state.g_ix_by_lim.len().saturating_sub(1);
    let tail_lim = tail_lim.min(max_lim);

    t_state
        .tail_g_ixs
        .reserve(tail_lim.saturating_sub(tail_low) + 1);
    for lim in tail_low..=tail_lim {
        let Some(g_ix) = t_state.g_ix_by_lim.get(lim).copied().flatten() else {
            continue;
        };
        t_state.tail_g_ixs.push(g_ix);
    }

    let mut best: Option<(i32, usize)> = None;

    if g.is_directed() {
        if !flip {
            for &head_gix in &t_state.tail_g_ixs {
                g.for_each_in_edge_entry_ix(
                    head_gix,
                    None,
                    |edge_ix, tail_ix, head_ix, _key, lbl| {
                        debug_assert_eq!(head_ix, head_gix);
                        // Skip re-adding the leaving edge.
                        if (tail_ix == leave_u_gix && head_ix == leave_v_gix)
                            || (tail_ix == leave_v_gix && head_ix == leave_u_gix)
                        {
                            return;
                        }
                        if is_in_tail(&*t_state, tail_ix) {
                            return;
                        }
                        if !t_state
                            .in_tree_by_g_ix
                            .get(tail_ix)
                            .copied()
                            .unwrap_or(false)
                        {
                            return;
                        }

                        let v_rank = rank_by_ix.get(tail_ix).copied().unwrap_or(0);
                        let w_rank = rank_by_ix.get(head_ix).copied().unwrap_or(0);
                        let minlen: i32 = (lbl.minlen.max(1)) as i32;
                        let slack = w_rank - v_rank - minlen;
                        match &best {
                            Some((best_slack, _)) if slack >= *best_slack => {}
                            _ => best = Some((slack, edge_ix)),
                        }
                    },
                );
            }
        } else {
            for &tail_gix in &t_state.tail_g_ixs {
                g.for_each_out_edge_entry_ix(
                    tail_gix,
                    None,
                    |edge_ix, tail_ix, head_ix, _key, lbl| {
                        debug_assert_eq!(tail_ix, tail_gix);
                        // Skip re-adding the leaving edge.
                        if (tail_ix == leave_u_gix && head_ix == leave_v_gix)
                            || (tail_ix == leave_v_gix && head_ix == leave_u_gix)
                        {
                            return;
                        }
                        if is_in_tail(&*t_state, head_ix) {
                            return;
                        }
                        if !t_state
                            .in_tree_by_g_ix
                            .get(head_ix)
                            .copied()
                            .unwrap_or(false)
                        {
                            return;
                        }

                        let v_rank = rank_by_ix.get(tail_ix).copied().unwrap_or(0);
                        let w_rank = rank_by_ix.get(head_ix).copied().unwrap_or(0);
                        let minlen: i32 = (lbl.minlen.max(1)) as i32;
                        let slack = w_rank - v_rank - minlen;
                        match &best {
                            Some((best_slack, _)) if slack >= *best_slack => {}
                            _ => best = Some((slack, edge_ix)),
                        }
                    },
                );
            }
        }
    } else {
        g.for_each_edge_entry_ix(|edge_ix, g_v_ix, g_w_ix, _key, lbl| {
            let v_desc = is_in_tail(&*t_state, g_v_ix);
            let w_desc = is_in_tail(&*t_state, g_w_ix);
            if flip == v_desc && flip != w_desc {
                let v_rank = rank_by_ix.get(g_v_ix).copied().unwrap_or(0);
                let w_rank = rank_by_ix.get(g_w_ix).copied().unwrap_or(0);
                let minlen: i32 = (lbl.minlen.max(1)) as i32;
                let slack = w_rank - v_rank - minlen;
                match &best {
                    Some((best_slack, _)) if slack >= *best_slack => {}
                    _ => best = Some((slack, edge_ix)),
                }
            }
        });
    }

    let Some((_, edge_ix)) = best else {
        return fallback;
    };
    g.edge_key_by_ix(edge_ix).cloned().unwrap_or(fallback)
}

fn update_ranks_fast(
    t_state: &mut TreeState,
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    rank_by_ix: &mut Vec<i32>,
) {
    for &root_tix in &t_state.roots {
        if t_state
            .g_ix_by_t_ix
            .get(root_tix)
            .copied()
            .flatten()
            .is_none()
        {
            continue;
        }

        t_state.rank_stack.clear();
        t_state.rank_stack.push(root_tix);

        while let Some(parent_tix) = t_state.rank_stack.pop() {
            let Some(parent_gix) = t_state.g_ix_by_t_ix.get(parent_tix).copied().flatten() else {
                continue;
            };

            let parent_rank = rank_by_ix.get(parent_gix).copied().unwrap_or(0);
            let Some(ch) = t_state.children.get(parent_tix) else {
                continue;
            };
            for &child_tix in ch {
                let Some(child_gix) = t_state.g_ix_by_t_ix.get(child_tix).copied().flatten() else {
                    continue;
                };

                let (minlen, flipped) =
                    if let Some(e) = g.edge_by_endpoints_ix(child_gix, parent_gix) {
                        (e.minlen as i32, false)
                    } else if let Some(e) = g.edge_by_endpoints_ix(parent_gix, child_gix) {
                        (e.minlen as i32, true)
                    } else {
                        continue;
                    };

                let rank = if flipped {
                    parent_rank + minlen
                } else {
                    parent_rank - minlen
                };

                if let Some(node) = g.node_label_mut_by_ix(child_gix) {
                    node.rank = Some(rank);
                }

                if child_gix >= rank_by_ix.len() {
                    rank_by_ix.resize(child_gix + 1, 0);
                }
                rank_by_ix[child_gix] = rank;
                t_state.rank_stack.push(child_tix);
            }
        }
    }
}

pub fn init_low_lim_values(
    tree: &mut Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
    root: Option<&str>,
) {
    let Some(root) = root
        .map(|s| s.to_string())
        .or_else(|| tree.nodes().next().map(|s| s.to_string()))
    else {
        return;
    };
    let Some(root_ix) = tree.node_ix(&root) else {
        return;
    };

    #[derive(Debug)]
    struct Frame {
        v_ix: usize,
        parent_ix: Option<usize>,
        low: i32,
        neighbors: Vec<usize>,
        next_neighbor: usize,
    }

    fn ensure_bool_len(v: &mut Vec<bool>, ix: usize) {
        if ix >= v.len() {
            v.resize(ix + 1, false);
        }
    }

    fn push_frame(
        tree: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
        visited: &mut Vec<bool>,
        stack: &mut Vec<Frame>,
        v_ix: usize,
        parent_ix: Option<usize>,
        next_lim: i32,
    ) {
        ensure_bool_len(visited, v_ix);
        visited[v_ix] = true;

        let mut neighbors: Vec<usize> = Vec::new();
        tree.for_each_neighbor_ix(v_ix, |w_ix| {
            if parent_ix.is_some_and(|p| p == w_ix) {
                return;
            }
            neighbors.push(w_ix);
        });

        stack.push(Frame {
            v_ix,
            parent_ix,
            low: next_lim,
            neighbors,
            next_neighbor: 0,
        });
    }

    let mut visited: Vec<bool> = Vec::new();
    let mut stack: Vec<Frame> = Vec::new();
    let mut next_lim: i32 = 1;
    push_frame(&*tree, &mut visited, &mut stack, root_ix, None, next_lim);

    while !stack.is_empty() {
        let next_child = {
            let Some(top) = stack.last_mut() else {
                break;
            };
            top.neighbors
                .get(top.next_neighbor)
                .copied()
                .inspect(|_| top.next_neighbor += 1)
                .map(|w_ix| (w_ix, top.v_ix))
        };

        if let Some((w_ix, parent_ix)) = next_child {
            ensure_bool_len(&mut visited, w_ix);
            if visited[w_ix] {
                continue;
            }
            push_frame(
                &*tree,
                &mut visited,
                &mut stack,
                w_ix,
                Some(parent_ix),
                next_lim,
            );
            continue;
        }

        let Some(frame) = stack.pop() else {
            break;
        };
        let Frame {
            v_ix,
            parent_ix,
            low,
            neighbors: _,
            next_neighbor: _,
        } = frame;

        let parent = parent_ix
            .and_then(|p| tree.node_id_by_ix(p))
            .map(|p| p.to_string());
        if let Some(label) = tree.node_label_mut_by_ix(v_ix) {
            label.low = low;
            label.lim = next_lim;
            label.parent = parent;
        }

        next_lim += 1;
    }
}

pub fn init_cut_values(
    t: &mut Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
) {
    let mut vs: Vec<String> = {
        let roots: Vec<&str> = t.nodes().collect();
        alg::postorder(t, &roots)
    };
    let _ = vs.pop();
    for v in vs {
        assign_cut_value(t, g, &v);
    }
}

fn assign_cut_value(
    t: &mut Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    child: &str,
) {
    let Some(parent) = t.node(child).and_then(|lbl| lbl.parent.clone()) else {
        return;
    };
    let cutvalue = calc_cut_value(t, g, child);
    if let Some(edge) = t.edge_mut(child, &parent, None) {
        edge.cutvalue = cutvalue;
    }
}

pub fn calc_cut_value(
    t: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    child: &str,
) -> f64 {
    let Some(parent) = t.node(child).and_then(|lbl| lbl.parent.as_deref()) else {
        return 0.0;
    };

    let mut child_is_tail = true;
    let graph_edge = if g.is_directed() {
        let Some(child_ix) = g.node_ix(child) else {
            return 0.0;
        };
        let Some(parent_ix) = g.node_ix(parent) else {
            return 0.0;
        };

        if let Some(e) = g.edge_by_endpoints_ix(child_ix, parent_ix) {
            e
        } else {
            child_is_tail = false;
            let Some(e) = g.edge_by_endpoints_ix(parent_ix, child_ix) else {
                return 0.0;
            };
            e
        }
    } else {
        let mut graph_edge = g.edge(child, parent, None);
        if graph_edge.is_none() {
            child_is_tail = false;
            graph_edge = g.edge(parent, child, None);
        }
        let Some(graph_edge) = graph_edge else {
            return 0.0;
        };
        graph_edge
    };

    let mut cut_value = graph_edge.weight;

    if g.is_directed() {
        let Some(child_ix) = g.node_ix(child) else {
            return cut_value;
        };
        let parent_ix = g.node_ix(parent);

        g.for_each_out_edge_ix(child_ix, None, |_tail_ix, head_ix, _ek, lbl| {
            if parent_ix.is_some_and(|p| head_ix == p) {
                return;
            }
            let Some(other) = g.node_id_by_ix(head_ix) else {
                return;
            };

            let points_to_head = child_is_tail;
            cut_value += if points_to_head {
                lbl.weight
            } else {
                -lbl.weight
            };

            if let Some(other_edge) = t.edge(child, other, None) {
                let other_cut_value = other_edge.cutvalue;
                cut_value += if points_to_head {
                    -other_cut_value
                } else {
                    other_cut_value
                };
            }
        });

        g.for_each_in_edge_ix(child_ix, None, |tail_ix, _head_ix, _ek, lbl| {
            if parent_ix.is_some_and(|p| tail_ix == p) {
                return;
            }
            let Some(other) = g.node_id_by_ix(tail_ix) else {
                return;
            };

            let points_to_head = !child_is_tail;
            cut_value += if points_to_head {
                lbl.weight
            } else {
                -lbl.weight
            };

            if let Some(other_edge) = t.edge(child, other, None) {
                let other_cut_value = other_edge.cutvalue;
                cut_value += if points_to_head {
                    -other_cut_value
                } else {
                    other_cut_value
                };
            }
        });
    } else {
        g.for_each_out_edge(child, None, |ek, lbl| {
            let other = ek.w.as_str();
            if other == parent {
                return;
            }

            let points_to_head = child_is_tail;
            cut_value += if points_to_head {
                lbl.weight
            } else {
                -lbl.weight
            };

            if let Some(other_edge) = t.edge(child, other, None) {
                let other_cut_value = other_edge.cutvalue;
                cut_value += if points_to_head {
                    -other_cut_value
                } else {
                    other_cut_value
                };
            }
        });

        g.for_each_in_edge(child, None, |ek, lbl| {
            let other = ek.v.as_str();
            if other == parent {
                return;
            }

            let points_to_head = !child_is_tail;
            cut_value += if points_to_head {
                lbl.weight
            } else {
                -lbl.weight
            };

            if let Some(other_edge) = t.edge(child, other, None) {
                let other_cut_value = other_edge.cutvalue;
                cut_value += if points_to_head {
                    -other_cut_value
                } else {
                    other_cut_value
                };
            }
        });
    }

    cut_value
}

pub fn leave_edge(t: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>) -> Option<EdgeKey> {
    edges::leave_edge(t)
}

pub fn enter_edge(
    t: &Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    rank_by_ix: &[i32],
    edge: &EdgeKey,
) -> EdgeKey {
    edges::enter_edge(t, g, rank_by_ix, edge)
}

pub fn exchange_edges(
    t: &mut Graph<tree::TreeNodeLabel, tree::TreeEdgeLabel, ()>,
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    rank_by_ix: &mut Vec<i32>,
    e: &EdgeKey,
    f: &EdgeKey,
) {
    edges::exchange_edges(t, g, rank_by_ix, e, f);
}

// NOTE: Dagre treats the feasible tree as an undirected structure. We consider an edge to be a
// tree edge if it exists in `t` (queried via `t.edge(u, v, None)` in the hot loops).

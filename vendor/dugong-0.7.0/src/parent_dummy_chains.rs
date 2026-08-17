//! Re-parent dummy chains in compound graphs.
//!
//! Dagre assigns dummy `"edge"` nodes to the most appropriate parent cluster based on the LCA
//! between the edge endpoints and the cluster min/max ranks. This mirrors upstream behavior.

use crate::graphlib::Graph;
use crate::{EdgeLabel, GraphLabel, NodeLabel};
use std::collections::BTreeMap;

pub fn parent_dummy_chains(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let postorder_nums = postorder(g);

    let chains = g.graph().dummy_chains.clone();
    for mut v in chains {
        let Some(node) = g.node(&v) else {
            continue;
        };
        let Some(edge_obj) = node.edge_obj.clone() else {
            continue;
        };

        let path_data = find_path(g, &postorder_nums, &edge_obj.v, &edge_obj.w);
        let path = path_data.path;
        let lca = path_data.lca;

        let mut path_idx: usize = 0;
        let mut path_v = path.get(path_idx).cloned().unwrap_or(None);
        let mut ascending = true;

        while v != edge_obj.w {
            let rank = g.node(&v).and_then(|n| n.rank).unwrap_or(0);

            if ascending {
                while path_v != lca
                    && path_v
                        .as_deref()
                        .and_then(|pv| g.node(pv))
                        .and_then(|n| n.max_rank)
                        .unwrap_or(i32::MAX / 2)
                        < rank
                {
                    path_idx += 1;
                    path_v = path.get(path_idx).cloned().unwrap_or(None);
                }

                if path_v == lca {
                    ascending = false;
                }
            }

            if !ascending {
                while path_idx + 1 < path.len()
                    && path
                        .get(path_idx + 1)
                        .and_then(|p| p.as_ref())
                        .and_then(|pv| g.node(pv))
                        .and_then(|n| n.min_rank)
                        .unwrap_or(i32::MIN / 2)
                        <= rank
                {
                    path_idx += 1;
                }
                path_v = path.get(path_idx).cloned().unwrap_or(None);
            }

            match &path_v {
                Some(parent) => {
                    g.set_parent_ref(v.as_str(), parent.as_str());
                }
                None => {
                    g.clear_parent(&v);
                }
            }

            let Some(next) = g.first_successor(&v).map(|s| s.to_string()) else {
                break;
            };
            v = next;
        }
    }
}

struct PostorderNum {
    low: usize,
    lim: usize,
}

struct PathData {
    path: Vec<Option<String>>,
    lca: Option<String>,
}

fn find_path(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    postorder_nums: &BTreeMap<String, PostorderNum>,
    v: &str,
    w: &str,
) -> PathData {
    let v_po = &postorder_nums[v];
    let w_po = &postorder_nums[w];
    let low = v_po.low.min(w_po.low);
    let lim = v_po.lim.max(w_po.lim);

    // Traverse up from v to find the LCA.
    let mut v_path: Vec<Option<String>> = Vec::new();
    let mut parent = Some(v.to_string());
    let lca: Option<String>;
    loop {
        parent = parent
            .as_deref()
            .and_then(|p| g.parent(p))
            .map(|s| s.to_string());
        v_path.push(parent.clone());
        let Some(p) = parent.clone() else {
            lca = None;
            break;
        };
        let po = &postorder_nums[&p];
        if !(po.low > low || lim > po.lim) {
            lca = Some(p);
            break;
        }
    }

    // Traverse from w to LCA.
    let mut w_path: Vec<Option<String>> = Vec::new();
    let mut cur = w.to_string();
    loop {
        let p = g.parent(&cur).map(|s| s.to_string());
        if p == lca {
            break;
        }
        if p.is_none() {
            break;
        }
        w_path.push(p.clone());
        if let Some(p) = p {
            cur = p;
        } else {
            break;
        }
    }

    let mut path = v_path;
    w_path.reverse();
    path.extend(w_path);
    PathData { path, lca }
}

fn postorder(g: &Graph<NodeLabel, EdgeLabel, GraphLabel>) -> BTreeMap<String, PostorderNum> {
    let mut result: BTreeMap<String, PostorderNum> = BTreeMap::new();
    let mut lim: usize = 0;

    let mut stack: Vec<(String, bool, usize)> = g
        .children_root()
        .into_iter()
        .rev()
        .map(|v| (v.to_string(), false, 0))
        .collect();

    while let Some((v, expanded, low)) = stack.pop() {
        if expanded {
            result.insert(v, PostorderNum { low, lim });
            lim += 1;
            continue;
        }

        let low = lim;
        stack.push((v.clone(), true, low));
        for child in g.children(&v).into_iter().rev() {
            stack.push((child.to_string(), false, 0));
        }
    }

    result
}

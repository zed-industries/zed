//! Add border segments for compound graphs.
//!
//! Dagre materializes per-rank `"border"` dummy nodes along each cluster so the ordering and
//! positioning steps can route edges around clusters. This mirrors upstream `add-border-segments.js`.

use crate::graphlib::Graph;
use crate::{EdgeLabel, GraphLabel, NodeLabel};
use rustc_hash::FxHashMap;

#[derive(Default)]
struct DummyNodeIdGen {
    next_suffix: FxHashMap<&'static str, usize>,
}

impl DummyNodeIdGen {
    fn add_dummy_node(
        &mut self,
        g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
        label: NodeLabel,
        prefix: &'static str,
    ) -> String {
        let suffix = match self.next_suffix.get(&prefix).copied() {
            Some(v) => v,
            None => {
                if !g.has_node(prefix) {
                    g.set_node(prefix, label);
                    self.next_suffix.insert(prefix, 1);
                    return prefix.to_string();
                }
                self.next_suffix.insert(prefix, 1);
                1
            }
        };

        // The legacy port used `for i in 1.. { format!("{prefix}{i}") ; has_node(...) }`,
        // which is O(n^2) and alloc-heavy. Keep the exact naming scheme but use a per-prefix
        // monotonic counter to make the common case O(1).
        let mut next = suffix;
        loop {
            let id = format!("{prefix}{next}");
            if !g.has_node(&id) {
                g.set_node(id.clone(), label);
                self.next_suffix.insert(prefix, next + 1);
                return id;
            }
            next += 1;
        }
    }
}

pub fn add_border_segments(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    if !g.options().compound {
        return;
    }

    let roots: Vec<String> = g
        .children_root()
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let mut ids = DummyNodeIdGen::default();

    let mut stack: Vec<(String, bool)> = roots.into_iter().rev().map(|v| (v, false)).collect();
    while let Some((v, expanded)) = stack.pop() {
        if expanded {
            add_border_segments_for_node(g, &v, &mut ids);
            continue;
        }

        stack.push((v.clone(), true));
        let children: Vec<String> = g.children_iter(&v).map(|s| s.to_string()).collect();
        for child in children.into_iter().rev() {
            stack.push((child, false));
        }
    }
}

fn add_border_segments_for_node(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    v: &str,
    ids: &mut DummyNodeIdGen,
) {
    let Some((min_rank, max_rank)) = g.node(v).and_then(|n| Some((n.min_rank?, n.max_rank?)))
    else {
        return;
    };

    let max_rank_usize: usize = max_rank.max(0) as usize;
    if let Some(n) = g.node_mut(v) {
        n.border_left = vec![None; max_rank_usize + 1];
        n.border_right = vec![None; max_rank_usize + 1];
    }

    let mut prev_left: Option<String> = None;
    let mut prev_right: Option<String> = None;

    for rank in min_rank..=max_rank {
        let left = add_border_node(g, ids, "borderLeft", "_bl", v, rank, true);
        if let Some(prev) = prev_left {
            g.set_edge_with_label(
                prev,
                left.clone(),
                EdgeLabel {
                    weight: 1.0,
                    ..Default::default()
                },
            );
        }
        prev_left = Some(left);

        let right = add_border_node(g, ids, "borderRight", "_br", v, rank, false);
        if let Some(prev) = prev_right {
            g.set_edge_with_label(
                prev,
                right.clone(),
                EdgeLabel {
                    weight: 1.0,
                    ..Default::default()
                },
            );
        }
        prev_right = Some(right);
    }
}

fn add_border_node(
    g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    ids: &mut DummyNodeIdGen,
    prop: &str,
    prefix: &'static str,
    sg: &str,
    rank: i32,
    is_left: bool,
) -> String {
    let curr = ids.add_dummy_node(
        g,
        NodeLabel {
            width: 0.0,
            height: 0.0,
            rank: Some(rank),
            dummy: Some("border".to_string()),
            border_type: Some(prop.to_string()),
            ..Default::default()
        },
        prefix,
    );

    if let Some(n) = g.node_mut(sg) {
        let idx = rank.max(0) as usize;
        if is_left {
            n.border_left[idx] = Some(curr.clone());
        } else {
            n.border_right[idx] = Some(curr.clone());
        }
    }

    g.set_parent_ref(curr.as_str(), sg);
    curr
}

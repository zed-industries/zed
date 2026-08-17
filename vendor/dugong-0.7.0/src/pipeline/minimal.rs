//! Minimal, parity-oriented layout.
//!
//! This pipeline is intentionally lightweight and defensive. It is used by tests and by early
//! parity work where we want deterministic output without pulling the full Dagre pipeline.

use crate::graphlib;
use crate::{EdgeLabel, GraphLabel, LabelPos, NodeLabel, Point, RankDir, acyclic};

pub fn layout(g: &mut graphlib::Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    // Dagre breaks cycles before ranking. Keep self-loops out of the feedback arc set since
    // reversing a self-loop doesn't make the graph acyclic, and self-loops should not constrain
    // rank assignment.
    acyclic::run(g);

    let graph = g.graph().clone();
    let edge_keys: Vec<graphlib::EdgeKey> = g.edges().cloned().collect();

    let mut max_edge_label_width: f64 = 0.0;
    let mut max_edge_label_height: f64 = 0.0;
    for e in &edge_keys {
        if let Some(lbl) = g.edge(&e.v, &e.w, e.name.as_deref()) {
            max_edge_label_width = max_edge_label_width.max(lbl.width);
            max_edge_label_height = max_edge_label_height.max(lbl.height);
        }
    }

    // A minimal parity-oriented approximation:
    // - in TB/BT: long edge labels tend to push nodes apart horizontally (cross-axis)
    // - in LR/RL: long edge labels tend to push ranks apart horizontally (axis)
    let node_sep = match graph.rankdir {
        RankDir::TB | RankDir::BT => graph.nodesep.max(max_edge_label_width),
        RankDir::LR | RankDir::RL => graph.nodesep.max(max_edge_label_height),
    };
    let rank_sep = match graph.rankdir {
        RankDir::TB | RankDir::BT => graph.ranksep,
        RankDir::LR | RankDir::RL => graph.ranksep.max(max_edge_label_width),
    };

    let node_ids: Vec<String> = g.nodes().map(|s| s.to_string()).collect();
    let node_ids: Vec<String> = node_ids
        .into_iter()
        .filter(|id| !g.options().compound || g.children(id).is_empty())
        .collect();

    let mut indegree: rustc_hash::FxHashMap<String, usize> =
        node_ids.iter().map(|id| (id.clone(), 0)).collect();
    for e in g.edges() {
        if e.v == e.w {
            continue;
        }
        if let Some(v) = indegree.get_mut(&e.w) {
            *v += 1;
        }
    }

    // Deterministic Kahn order: initial nodes in insertion order.
    let mut queue: std::collections::VecDeque<String> = node_ids
        .iter()
        .filter(|id| indegree.get(*id).copied().unwrap_or(0) == 0)
        .cloned()
        .collect();

    let mut topo: Vec<String> = Vec::new();
    while let Some(n) = queue.pop_front() {
        topo.push(n.clone());

        // Traverse outgoing edges in edge insertion order.
        let mut out: Vec<String> = Vec::new();
        for e in g.edges() {
            if e.v == n {
                if e.v == e.w {
                    continue;
                }
                out.push(e.w.clone());
            }
        }
        for w in out {
            if let Some(v) = indegree.get_mut(&w) {
                *v = v.saturating_sub(1);
                if *v == 0 {
                    queue.push_back(w);
                }
            }
        }
    }

    // If the graph has a cycle, fall back to insertion order for now.
    if topo.len() != node_ids.len() {
        topo = node_ids.clone();
    }

    let mut rank: rustc_hash::FxHashMap<String, usize> =
        node_ids.iter().map(|id| (id.clone(), 0)).collect();
    for n in &topo {
        let r = rank.get(n).copied().unwrap_or(0);
        for e in g.edges() {
            if e.v != *n {
                continue;
            }
            if e.v == e.w {
                continue;
            }
            let minlen = g
                .edge(&e.v, &e.w, e.name.as_deref())
                .map(|l| l.minlen)
                .unwrap_or(1)
                .max(1);
            let next = r.saturating_add(minlen);
            let entry = rank.entry(e.w.clone()).or_insert(0);
            if next > *entry {
                *entry = next;
            }
        }
    }

    if g.options().compound {
        // Compact ranks inside compound nodes where a common rank is feasible, to minimize cluster height.
        // This is a small parity-oriented step to match upstream Dagre behavior for subgraphs.
        let parents: Vec<String> = g
            .node_ids()
            .into_iter()
            .filter(|id| !g.children(id).is_empty())
            .collect();

        for parent in parents {
            let children: Vec<String> = g
                .children(&parent)
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            let targets: Vec<String> = children
                .into_iter()
                .filter(|c| rank.contains_key(c))
                .collect();
            if targets.len() < 2 {
                continue;
            }

            // Preserve insertion order (children are stored deterministically).
            let mut min_needed: usize = 0;
            let mut max_allowed: usize = usize::MAX / 4;

            for child in &targets {
                let mut min_rank: usize = 0;
                for ek in g.in_edges(child, None) {
                    let Some(&pred_rank) = rank.get(&ek.v) else {
                        continue;
                    };
                    let minlen = g.edge_by_key(&ek).map(|e| e.minlen).unwrap_or(1).max(1);
                    min_rank = min_rank.max(pred_rank.saturating_add(minlen));
                }

                let mut max_rank: usize = usize::MAX / 4;
                for ek in g.out_edges(child, None) {
                    let Some(&succ_rank) = rank.get(&ek.w) else {
                        continue;
                    };
                    let minlen = g.edge_by_key(&ek).map(|e| e.minlen).unwrap_or(1).max(1);
                    let upper = succ_rank.saturating_sub(minlen);
                    max_rank = max_rank.min(upper);
                }

                min_needed = min_needed.max(min_rank);
                max_allowed = max_allowed.min(max_rank);
            }

            if min_needed <= max_allowed {
                for child in &targets {
                    rank.insert(child.clone(), min_needed);
                }
            }
        }
    }

    let max_rank = rank.values().copied().max().unwrap_or(0);
    let mut ranks: Vec<Vec<String>> = vec![Vec::new(); max_rank + 1];
    for id in &node_ids {
        let r = rank.get(id).copied().unwrap_or(0);
        if let Some(layer) = ranks.get_mut(r) {
            layer.push(id.clone());
        }
    }

    fn node_size(g: &graphlib::Graph<NodeLabel, EdgeLabel, GraphLabel>, id: &str) -> (f64, f64) {
        match g.node(id) {
            Some(n) => (n.width, n.height),
            None => (0.0, 0.0),
        }
    }

    let mut gap_extra: Vec<f64> = vec![0.0; ranks.len().saturating_sub(1)];
    for e in g.edges() {
        if e.v == e.w {
            continue;
        }
        let Some(v_rank) = rank.get(&e.v).copied() else {
            continue;
        };
        let Some(w_rank) = rank.get(&e.w).copied() else {
            continue;
        };
        if w_rank != v_rank.saturating_add(1) {
            continue;
        }
        let Some(lbl) = g.edge(&e.v, &e.w, e.name.as_deref()) else {
            continue;
        };
        if lbl.height <= 0.0 {
            continue;
        }
        if let Some(extra) = gap_extra.get_mut(v_rank) {
            *extra = extra.max(lbl.height);
        }
    }

    let mut rank_heights: Vec<f64> = Vec::with_capacity(ranks.len());
    let mut rank_widths: Vec<f64> = Vec::with_capacity(ranks.len());
    for ids in &ranks {
        let mut h: f64 = 0.0;
        let mut w: f64 = 0.0;
        for (i, id) in ids.iter().enumerate() {
            let (nw, nh) = node_size(g, id);
            h = h.max(nh);
            w += nw;
            if i + 1 < ids.len() {
                w += node_sep;
            }
        }
        rank_heights.push(h);
        rank_widths.push(w);
    }
    let max_rank_width = rank_widths.iter().copied().fold(0.0_f64, |a, b| a.max(b));

    let mut y_cursor: f64 = 0.0;
    for (rank_idx, ids) in ranks.iter().enumerate() {
        let rank_h = rank_heights[rank_idx];
        let y = y_cursor + rank_h / 2.0;

        let rank_w = rank_widths[rank_idx];
        let mut x_cursor = (max_rank_width - rank_w) / 2.0;
        for id in ids {
            let (nw, _) = node_size(g, id);
            let x = x_cursor + nw / 2.0;
            if let Some(n) = g.node_mut(id) {
                n.x = Some(x);
                n.y = Some(y);
            }
            x_cursor += nw + node_sep;
        }

        y_cursor += rank_h;
        if rank_idx + 1 < ranks.len() {
            y_cursor += rank_sep + gap_extra.get(rank_idx).copied().unwrap_or(0.0);
        }
    }

    let total_height = y_cursor;

    for e in &edge_keys {
        let Some((sx, sy, sw, sh)) = g
            .node(&e.v)
            .map(|n| (n.x.unwrap_or(0.0), n.y.unwrap_or(0.0), n.width, n.height))
        else {
            continue;
        };
        let Some((tx, ty, tw, th)) = g
            .node(&e.w)
            .map(|n| (n.x.unwrap_or(0.0), n.y.unwrap_or(0.0), n.width, n.height))
        else {
            continue;
        };

        let Some(lbl) = g.edge_mut(&e.v, &e.w, e.name.as_deref()) else {
            continue;
        };
        lbl.points.clear();
        lbl.x = None;
        lbl.y = None;

        if e.v == e.w {
            // A minimal self-loop shape that satisfies upstream dagre invariants:
            // - TB/BT: all points are to the right of the node center (x > node.x)
            // - LR/RL: after rankdir transforms, all points are below the node center (y > node.y)
            // and all points stay within the node's height/2 on the cross-axis.
            let x0 = sx + sw / 2.0 + graph.edgesep.max(1.0);
            let x1 = x0 + graph.edgesep.max(1.0);
            let y0 = sy;
            let y_top = sy - sh / 2.0;
            let y_bot = sy + sh / 2.0;

            lbl.points.extend([
                Point { x: x0, y: y0 },
                Point { x: x0, y: y_top },
                Point { x: x1, y: y_top },
                Point { x: x1, y: y0 },
                Point { x: x1, y: y_bot },
                Point { x: x0, y: y_bot },
                Point { x: x0, y: y0 },
            ]);

            continue;
        }

        let start = Point {
            x: sx,
            y: sy + sh / 2.0,
        };
        let end = Point {
            x: tx,
            y: ty - th / 2.0,
        };

        let minlen = lbl.minlen.max(1);
        let count = 2 * minlen + 1;
        for i in 0..count {
            let t = (i as f64) / ((count - 1) as f64);
            lbl.points.push(Point {
                x: start.x + (end.x - start.x) * t,
                y: start.y + (end.y - start.y) * t,
            });
        }

        if lbl.width > 0.0 || lbl.height > 0.0 {
            if let Some(mid) = lbl.points.get(count / 2).copied() {
                let mut ex = mid.x;
                let ey = mid.y;
                match lbl.labelpos {
                    LabelPos::C => {}
                    LabelPos::L => ex -= lbl.labeloffset + lbl.width / 2.0,
                    LabelPos::R => ex += lbl.labeloffset + lbl.width / 2.0,
                }
                lbl.x = Some(ex);
                lbl.y = Some(ey);
            }
        }

        let _ = (sw, tw);
    }

    match graph.rankdir {
        RankDir::TB => {}
        RankDir::BT => {
            for id in &node_ids {
                if let Some(n) = g.node_mut(id) {
                    if let Some(y) = n.y {
                        n.y = Some(total_height - y);
                    }
                }
            }
            for e in &edge_keys {
                if let Some(lbl) = g.edge_mut(&e.v, &e.w, e.name.as_deref()) {
                    for p in &mut lbl.points {
                        p.y = total_height - p.y;
                    }
                    if let Some(y) = lbl.y {
                        lbl.y = Some(total_height - y);
                    }
                }
            }
        }
        RankDir::LR => {
            for id in &node_ids {
                if let Some(n) = g.node_mut(id) {
                    let (Some(x), Some(y)) = (n.x, n.y) else {
                        continue;
                    };
                    n.x = Some(y);
                    n.y = Some(x);
                }
            }
            for e in &edge_keys {
                if let Some(lbl) = g.edge_mut(&e.v, &e.w, e.name.as_deref()) {
                    for p in &mut lbl.points {
                        (p.x, p.y) = (p.y, p.x);
                    }
                    if let (Some(x), Some(y)) = (lbl.x, lbl.y) {
                        lbl.x = Some(y);
                        lbl.y = Some(x);
                    }
                }
            }
        }
        RankDir::RL => {
            for id in &node_ids {
                if let Some(n) = g.node_mut(id) {
                    let (Some(x), Some(y)) = (n.x, n.y) else {
                        continue;
                    };
                    n.x = Some(total_height - y);
                    n.y = Some(x);
                }
            }
            for e in &edge_keys {
                if let Some(lbl) = g.edge_mut(&e.v, &e.w, e.name.as_deref()) {
                    for p in &mut lbl.points {
                        let new_x = total_height - p.y;
                        (p.x, p.y) = (new_x, p.x);
                    }
                    if let (Some(x), Some(y)) = (lbl.x, lbl.y) {
                        lbl.x = Some(total_height - y);
                        lbl.y = Some(x);
                    }
                }
            }
        }
    }

    // Restore original edge directions and names, keeping computed points.
    acyclic::undo(g);
}

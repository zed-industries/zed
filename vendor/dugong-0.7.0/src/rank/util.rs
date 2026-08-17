//! Rank helpers (longest-path, slack).

use crate::graphlib::{EdgeKey, Graph};
use crate::{EdgeLabel, GraphLabel, NodeLabel};
use rustc_hash::FxHashMap as HashMap;

pub fn longest_path(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    struct Frame {
        v: String,
        edges: Vec<EdgeKey>,
        next_edge: usize,
        rank: Option<i32>,
        incoming_minlen: Option<i32>,
    }

    fn apply_candidate(rank: &mut Option<i32>, candidate: i32) {
        *rank = Some(match *rank {
            Some(current) => current.min(candidate),
            None => candidate,
        });
    }

    let sources: Vec<String> = g.sources().into_iter().map(|s| s.to_string()).collect();
    let mut visited: HashMap<String, i32> = HashMap::default();
    for v in sources {
        if visited.contains_key(&v) {
            continue;
        }

        let mut stack = vec![Frame {
            edges: g.out_edges(&v, None),
            v,
            next_edge: 0,
            rank: None,
            incoming_minlen: None,
        }];

        while let Some(frame) = stack.last_mut() {
            if let Some(rank) = visited.get(frame.v.as_str()).copied() {
                let incoming_minlen = frame.incoming_minlen;
                let _ = stack.pop();
                if let (Some(parent), Some(minlen)) = (stack.last_mut(), incoming_minlen) {
                    apply_candidate(&mut parent.rank, rank - minlen);
                }
                continue;
            }

            if frame.next_edge < frame.edges.len() {
                let edge = frame.edges[frame.next_edge].clone();
                frame.next_edge += 1;
                let minlen = g
                    .edge_by_key(&edge)
                    .map(|lbl| lbl.minlen as i32)
                    .unwrap_or(1);
                if let Some(child_rank) = visited.get(edge.w.as_str()).copied() {
                    apply_candidate(&mut frame.rank, child_rank - minlen);
                } else {
                    stack.push(Frame {
                        edges: g.out_edges(&edge.w, None),
                        v: edge.w,
                        next_edge: 0,
                        rank: None,
                        incoming_minlen: Some(minlen),
                    });
                }
                continue;
            }

            let Some(frame) = stack.pop() else {
                break;
            };
            let rank = frame.rank.unwrap_or(0);
            if let Some(label) = g.node_mut(&frame.v) {
                label.rank = Some(rank);
            }
            visited.insert(frame.v, rank);
            if let (Some(parent), Some(minlen)) = (stack.last_mut(), frame.incoming_minlen) {
                apply_candidate(&mut parent.rank, rank - minlen);
            }
        }
    }
}

pub fn slack(g: &Graph<NodeLabel, EdgeLabel, GraphLabel>, e: &EdgeKey) -> i32 {
    // Be defensive: callers can provide arbitrary graphs. Missing nodes/ranks are treated
    // as `0` so layout can degrade gracefully instead of panicking.
    let w_rank = g.node(&e.w).and_then(|n| n.rank).unwrap_or(0);
    let v_rank = g.node(&e.v).and_then(|n| n.rank).unwrap_or(0);
    let minlen: i32 = g.edge_by_key(e).map(|lbl| lbl.minlen as i32).unwrap_or(1);
    w_rank - v_rank - minlen
}

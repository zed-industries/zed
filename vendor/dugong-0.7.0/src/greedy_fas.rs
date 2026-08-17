//! Greedy feedback arc set (FAS) selection.
//!
//! Ported from Dagre's `greedy-fas.js`. This is used by `acyclic` when `acyclicer=greedy`.

use crate::graphlib::{EdgeKey, Graph};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::collections::{VecDeque, hash_map::Entry};

pub fn greedy_fas<N, E, G>(g: &Graph<N, E, G>) -> Vec<EdgeKey>
where
    N: Default + 'static,
    E: Default + 'static,
    G: Default,
{
    greedy_fas_with_weight(g, |_| 1)
}

pub fn greedy_fas_with_weight<N, E, G>(
    g: &Graph<N, E, G>,
    weight_fn: impl Fn(&E) -> i64,
) -> Vec<EdgeKey>
where
    N: Default + 'static,
    E: Default + 'static,
    G: Default,
{
    if g.node_count() <= 1 {
        return Vec::new();
    }

    // Aggregate multi-edges into a simple graph with summed weights.
    //
    // Note: Upstream Dagre (JS) preserves insertion order for `g.nodes()` / `g.edges()` and
    // the derived `inEdges(v)` / `outEdges(v)` traversals. GreedyFAS is sensitive to that
    // ordering because it uses stable queues (List.enqueue + List.dequeue).
    //
    // For parity, keep node initialization in `g.node_ids()` order and keep the aggregated
    // adjacency order based on the *first occurrence* of each `(v, w)` in `g.edges()`.
    let node_ids = g.node_ids();
    let mut in_w: HashMap<String, i64> = HashMap::default();
    let mut out_w: HashMap<String, i64> = HashMap::default();
    for v in &node_ids {
        in_w.insert(v.clone(), 0);
        out_w.insert(v.clone(), 0);
    }

    let mut edge_w: HashMap<(String, String), i64> = HashMap::default();
    let mut edge_order: Vec<(String, String)> = Vec::new();
    let mut max_in: i64 = 0;
    let mut max_out: i64 = 0;

    for e in g.edges() {
        let w = g.edge_by_key(e).map(&weight_fn).unwrap_or(1);
        let key = (e.v.clone(), e.w.clone());
        match edge_w.entry(key.clone()) {
            Entry::Vacant(v) => {
                v.insert(w);
                edge_order.push(key);
            }
            Entry::Occupied(mut o) => {
                *o.get_mut() += w;
            }
        }
        let o = out_w.entry(e.v.clone()).or_insert(0);
        *o += w;
        max_out = max_out.max(*o);
        let i = in_w.entry(e.w.clone()).or_insert(0);
        *i += w;
        max_in = max_in.max(*i);
    }

    let bucket_len: usize = (max_out + max_in + 3).max(3) as usize;
    let zero_idx: i64 = max_in + 1;
    let mut buckets: Vec<VecDeque<String>> = (0..bucket_len).map(|_| VecDeque::new()).collect();
    let mut bucket_of: HashMap<String, usize> = HashMap::default();

    for v in &node_ids {
        assign_bucket(v, &in_w, &out_w, &mut buckets, zero_idx, &mut bucket_of);
    }

    // Build adjacency for the aggregated graph (for efficient updates).
    let mut in_edges: HashMap<String, Vec<(String, i64)>> = HashMap::default();
    let mut out_edges: HashMap<String, Vec<(String, i64)>> = HashMap::default();
    for (v, w) in &edge_order {
        let wgt = edge_w.get(&(v.clone(), w.clone())).copied().unwrap_or(0);
        out_edges
            .entry(v.clone())
            .or_default()
            .push((w.clone(), wgt));
        in_edges
            .entry(w.clone())
            .or_default()
            .push((v.clone(), wgt));
    }

    let mut alive: HashSet<String> = node_ids.iter().cloned().collect();
    let mut results: Vec<(String, String)> = Vec::new();

    struct Work<'a> {
        alive: &'a mut HashSet<String>,
        buckets: &'a mut [VecDeque<String>],
        zero_idx: i64,
        bucket_of: &'a mut HashMap<String, usize>,
        in_w: &'a mut HashMap<String, i64>,
        out_w: &'a mut HashMap<String, i64>,
        in_edges: &'a HashMap<String, Vec<(String, i64)>>,
        out_edges: &'a HashMap<String, Vec<(String, i64)>>,
    }

    impl Work<'_> {
        fn pop_bucket(&mut self, idx: usize) -> Option<String> {
            pop_bucket(&mut self.buckets[idx], &*self.alive)
        }

        fn remove_node(&mut self, v: &str) {
            self.remove_node_inner(v, None);
        }

        fn remove_node_collect_predecessors(&mut self, v: &str, preds: &mut Vec<(String, String)>) {
            self.remove_node_inner(v, Some(preds));
        }

        fn remove_node_inner(
            &mut self,
            v: &str,
            collect_predecessors: Option<&mut Vec<(String, String)>>,
        ) {
            if !self.alive.remove(v) {
                return;
            }

            if let Some(preds) = collect_predecessors {
                if let Some(ins) = self.in_edges.get(v) {
                    for (u, _) in ins {
                        if self.alive.contains(u) {
                            preds.push((u.clone(), v.to_string()));
                        }
                    }
                }
            }

            if let Some(ins) = self.in_edges.get(v) {
                for (u, wgt) in ins {
                    if !self.alive.contains(u) {
                        continue;
                    }
                    if let Some(o) = self.out_w.get_mut(u) {
                        *o -= *wgt;
                    }
                    assign_bucket(
                        u,
                        &*self.in_w,
                        &*self.out_w,
                        self.buckets,
                        self.zero_idx,
                        self.bucket_of,
                    );
                }
            }

            if let Some(outs) = self.out_edges.get(v) {
                for (w, wgt) in outs {
                    if !self.alive.contains(w) {
                        continue;
                    }
                    if let Some(i) = self.in_w.get_mut(w) {
                        *i -= *wgt;
                    }
                    assign_bucket(
                        w,
                        &*self.in_w,
                        &*self.out_w,
                        self.buckets,
                        self.zero_idx,
                        self.bucket_of,
                    );
                }
            }

            self.in_w.remove(v);
            self.out_w.remove(v);
            self.bucket_of.remove(v);
        }
    }

    let mut work = Work {
        alive: &mut alive,
        buckets: &mut buckets,
        zero_idx,
        bucket_of: &mut bucket_of,
        in_w: &mut in_w,
        out_w: &mut out_w,
        in_edges: &in_edges,
        out_edges: &out_edges,
    };

    while !work.alive.is_empty() {
        // Drain sinks (out == 0).
        while let Some(v) = work.pop_bucket(0) {
            work.remove_node(&v);
        }

        // Drain sources (in == 0).
        let last = work.buckets.len() - 1;
        while let Some(v) = work.pop_bucket(last) {
            work.remove_node(&v);
        }

        if work.alive.is_empty() {
            break;
        }

        // Pick a node from the highest non-extreme bucket and collect its predecessor edges.
        let mut picked: Option<String> = None;
        for i in (1..last).rev() {
            if let Some(v) = work.pop_bucket(i) {
                picked = Some(v);
                break;
            }
        }

        let Some(v) = picked else {
            // Should not happen, but avoid an infinite loop.
            let v = node_ids
                .iter()
                .find(|id| work.alive.contains(*id))
                .cloned()
                .or_else(|| work.alive.iter().next().cloned());
            let Some(v) = v else {
                break;
            };
            work.remove_node(&v);
            continue;
        };

        let mut preds: Vec<(String, String)> = Vec::new();
        work.remove_node_collect_predecessors(&v, &mut preds);
        results.extend(preds);
    }

    // Expand multi-edges back to concrete edge keys from the original graph.
    let mut out: Vec<EdgeKey> = Vec::new();
    for (v, w) in results {
        out.extend(g.out_edges(&v, Some(&w)));
    }
    out
}

fn pop_bucket(bucket: &mut VecDeque<String>, alive: &HashSet<String>) -> Option<String> {
    while let Some(v) = bucket.pop_back() {
        if alive.contains(&v) {
            return Some(v);
        }
    }
    None
}

fn assign_bucket(
    v: &str,
    in_w: &HashMap<String, i64>,
    out_w: &HashMap<String, i64>,
    buckets: &mut [VecDeque<String>],
    zero_idx: i64,
    bucket_of: &mut HashMap<String, usize>,
) {
    if let Some(prev) = bucket_of.get(v).copied() {
        if let Some(pos) = buckets[prev].iter().position(|x| x == v) {
            buckets[prev].remove(pos);
        }
    }

    let in_v = in_w.get(v).copied().unwrap_or(0);
    let out_v = out_w.get(v).copied().unwrap_or(0);
    let idx: usize = if out_v == 0 {
        0
    } else if in_v == 0 {
        buckets.len() - 1
    } else {
        let raw = out_v - in_v + zero_idx;
        raw.clamp(0, (buckets.len() - 1) as i64) as usize
    };

    buckets[idx].push_front(v.to_string());
    bucket_of.insert(v.to_string(), idx);
}

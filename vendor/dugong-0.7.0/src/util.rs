//! Miscellaneous helpers from Dagre's `util.js`.
//!
//! Note: upstream sometimes `throw`s on invalid inputs. In this Rust port we prefer best-effort
//! behavior (no panics on user-controlled input) so downstream libraries can render diagrams
//! without crashing.

use crate::graphlib::{Graph, GraphOptions};
use crate::{EdgeLabel, GraphLabel, NodeLabel, Point};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use web_time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub fn simplify<N, G>(g: &Graph<N, EdgeLabel, G>) -> Graph<N, EdgeLabel, G>
where
    N: Default + Clone + 'static,
    G: Default + Clone,
{
    let mut simplified: Graph<N, EdgeLabel, G> = Graph::new(GraphOptions {
        multigraph: false,
        compound: false,
        ..Default::default()
    });
    simplified.set_graph(g.graph().clone());

    for v in g.node_ids() {
        if let Some(lbl) = g.node(&v) {
            simplified.set_node(v, lbl.clone());
        }
    }

    // Merge multi-edges deterministically while avoiding per-edge String clones.
    let mut merged: BTreeMap<(usize, usize), (f64, usize)> = BTreeMap::new();
    g.for_each_edge_ix(|v_ix, w_ix, _key, lbl| {
        let entry = merged.entry((v_ix, w_ix)).or_insert((0.0, 1));
        entry.0 += lbl.weight;
        entry.1 = entry.1.max(lbl.minlen.max(1));
    });

    let mut merged_edges: Vec<(String, String, f64, usize)> = Vec::with_capacity(merged.len());
    for ((v_ix, w_ix), (weight, minlen)) in merged {
        let Some(v) = g.node_id_by_ix(v_ix) else {
            continue;
        };
        let Some(w) = g.node_id_by_ix(w_ix) else {
            continue;
        };
        merged_edges.push((v.to_string(), w.to_string(), weight, minlen));
    }
    merged_edges.sort_by(|a, b| (a.0.as_str(), a.1.as_str()).cmp(&(b.0.as_str(), b.1.as_str())));

    for (v, w, weight, minlen) in merged_edges {
        simplified.set_edge_with_label(
            v,
            w,
            EdgeLabel {
                weight,
                minlen,
                ..Default::default()
            },
        );
    }

    simplified
}

pub fn as_non_compound_graph<N, E, G>(g: &Graph<N, E, G>) -> Graph<N, E, G>
where
    N: Default + Clone + 'static,
    E: Default + Clone + 'static,
    G: Default + Clone,
{
    let mut simplified: Graph<N, E, G> = Graph::new(GraphOptions {
        multigraph: g.options().multigraph,
        compound: false,
        ..Default::default()
    });
    simplified.set_graph(g.graph().clone());

    for v in g.node_ids() {
        if g.children(&v).is_empty() {
            if let Some(lbl) = g.node(&v) {
                simplified.set_node(v, lbl.clone());
            }
        }
    }

    for e in g.edges() {
        if let Some(lbl) = g.edge_by_key(e) {
            simplified.set_edge_named(e.v.clone(), e.w.clone(), e.name.clone(), Some(lbl.clone()));
        }
    }

    simplified
}

pub fn successor_weights<G>(
    g: &Graph<NodeLabel, EdgeLabel, G>,
) -> BTreeMap<String, BTreeMap<String, f64>>
where
    G: Default,
{
    let mut out: BTreeMap<String, BTreeMap<String, f64>> = BTreeMap::new();
    for v in g.node_ids() {
        let mut map: BTreeMap<String, f64> = BTreeMap::new();
        for e in g.out_edges(&v, None) {
            let w = e.w.clone();
            let weight = g.edge_by_key(&e).map(|lbl| lbl.weight).unwrap_or(0.0);
            *map.entry(w).or_insert(0.0) += weight;
        }
        out.insert(v, map);
    }
    out
}

pub fn predecessor_weights<G>(
    g: &Graph<NodeLabel, EdgeLabel, G>,
) -> BTreeMap<String, BTreeMap<String, f64>>
where
    G: Default,
{
    let mut out: BTreeMap<String, BTreeMap<String, f64>> = BTreeMap::new();
    for v in g.node_ids() {
        let mut map: BTreeMap<String, f64> = BTreeMap::new();
        for e in g.in_edges(&v, None) {
            let u = e.v.clone();
            let weight = g.edge_by_key(&e).map(|lbl| lbl.weight).unwrap_or(0.0);
            *map.entry(u).or_insert(0.0) += weight;
        }
        out.insert(v, map);
    }
    out
}

pub fn intersect_rect(rect: Rect, point: Point) -> Point {
    let x = rect.x;
    let y = rect.y;

    let dx = point.x - x;
    let dy = point.y - y;
    let mut w = rect.width / 2.0;
    let mut h = rect.height / 2.0;

    // Upstream throws here. In headless Rust usage this can become input-reachable for degenerate
    // edges, so return a deterministic point on the right edge instead.
    if dx == 0.0 && dy == 0.0 {
        return Point { x: x + w, y };
    }

    let (sx, sy) = if dy.abs() * w > dx.abs() * h {
        if dy < 0.0 {
            h = -h;
        }
        (h * dx / dy, h)
    } else {
        if dx < 0.0 {
            w = -w;
        }
        (w, w * dy / dx)
    };

    Point {
        x: x + sx,
        y: y + sy,
    }
}

pub fn build_layer_matrix<E, G>(g: &Graph<NodeLabel, E, G>) -> Vec<Vec<String>>
where
    E: Default + 'static,
    G: Default,
{
    let mut min_rank: i32 = i32::MAX;
    let mut max_rank: i32 = i32::MIN;
    let mut entries: Vec<(i32, usize, String)> = Vec::new();

    for id in g.nodes() {
        let Some(node) = g.node(id) else {
            continue;
        };
        let Some(rank) = node.rank else {
            continue;
        };
        min_rank = min_rank.min(rank);
        max_rank = max_rank.max(rank);
        entries.push((rank, node.order.unwrap_or(0), id.to_string()));
    }

    if max_rank == i32::MIN {
        return Vec::new();
    }

    let shift = if min_rank < 0 { -min_rank } else { 0 };
    let len = (max_rank + shift + 1).max(0) as usize;
    let mut layers: Vec<Vec<(usize, String)>> = vec![Vec::new(); len];

    for (rank, order, id) in entries {
        let idx = (rank + shift).max(0) as usize;
        if idx < layers.len() {
            layers[idx].push((order, id));
        }
    }

    layers
        .into_iter()
        .map(|mut layer| {
            layer.sort_by_key(|(o, _)| *o);
            layer.into_iter().map(|(_, id)| id).collect()
        })
        .collect()
}

pub fn time_to_writer<T>(name: &str, writer: &mut dyn std::io::Write, f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let out = f();
    let ms = start.elapsed().as_millis();
    let _ = writeln!(writer, "{name} time: {ms}ms");
    let _ = writer.flush();
    out
}

pub fn time<T>(name: &str, f: impl FnOnce() -> T) -> T {
    let mut stdout = std::io::stdout();
    time_to_writer(name, &mut stdout, f)
}

pub fn normalize_ranks<E, G>(g: &mut Graph<NodeLabel, E, G>)
where
    E: Default + 'static,
    G: Default,
{
    let mut min_rank: i32 = i32::MAX;
    g.for_each_node(|_id, n| {
        if let Some(rank) = n.rank {
            min_rank = min_rank.min(rank);
        }
    });
    if min_rank == i32::MAX {
        return;
    }
    g.for_each_node_mut(|_id, n| {
        if let Some(rank) = n.rank {
            n.rank = Some(rank - min_rank);
        }
    });
}

pub fn remove_empty_ranks(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let Some(factor) = g.graph().node_rank_factor.filter(|&f| f > 0) else {
        return;
    };

    let mut offset: i32 = i32::MAX;
    g.for_each_node(|_id, n| {
        if let Some(rank) = n.rank {
            offset = offset.min(rank);
        }
    });
    if offset == i32::MAX {
        return;
    }

    let mut max_idx: usize = 0;
    let mut layers: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    g.for_each_node(|id, n| {
        let Some(rank) = n.rank else {
            return;
        };
        let idx = (rank - offset).max(0) as usize;
        max_idx = max_idx.max(idx);
        layers.entry(idx).or_default().push(id.to_string());
    });

    let mut delta: i32 = 0;
    for i in 0..=max_idx {
        if !layers.contains_key(&i) && i % factor != 0 {
            delta -= 1;
            continue;
        }
        if delta == 0 {
            continue;
        }
        if let Some(vs) = layers.get(&i) {
            for v in vs {
                if let Some(n) = g.node_mut(v) {
                    if let Some(rank) = n.rank {
                        n.rank = Some(rank + delta);
                    }
                }
            }
        }
    }
}

static UNIQUE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn unique_id(prefix: impl ToString) -> String {
    let id = UNIQUE_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{}{}", prefix.to_string(), id)
}

pub fn range(limit: i32) -> Vec<i32> {
    range_with(0, limit, 1)
}

pub fn range_start(start: i32, limit: i32) -> Vec<i32> {
    range_with(start, limit, 1)
}

pub fn range_with(start: i32, limit: i32, step: i32) -> Vec<i32> {
    if step == 0 {
        return Vec::new();
    }
    let mut out: Vec<i32> = Vec::new();
    let mut i = start;
    if step > 0 {
        while i < limit {
            out.push(i);
            i += step;
        }
    } else {
        while limit < i {
            out.push(i);
            i += step;
        }
    }
    out
}

pub fn map_values<K, V, R>(obj: &BTreeMap<K, V>, f: impl Fn(&V, &K) -> R) -> BTreeMap<K, R>
where
    K: Ord + Clone,
{
    obj.iter().map(|(k, v)| (k.clone(), f(v, k))).collect()
}

pub fn map_values_prop(
    obj: &BTreeMap<String, serde_json::Value>,
    prop: &str,
) -> BTreeMap<String, serde_json::Value> {
    obj.iter()
        .map(|(k, v)| {
            let value = v.get(prop).cloned().unwrap_or(serde_json::Value::Null);
            (k.clone(), value)
        })
        .collect()
}

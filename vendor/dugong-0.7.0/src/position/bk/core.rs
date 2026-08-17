//! Brandes & Köpf (BK) horizontal compaction.
//!
//! This module is a parity-oriented port of Dagre's `position/bk` helpers.
//!
//! Note: this file is being split into submodules to keep individual algorithms focused.

use crate::graphlib::{Graph, GraphOptions};
use crate::{EdgeLabel, GraphLabel, NodeLabel};
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;
use std::collections::{BTreeMap, BTreeSet};

use super::util::{sep, width};

pub type Conflicts = BTreeMap<String, BTreeSet<String>>;

pub fn add_conflict(conflicts: &mut Conflicts, v: &str, w: &str) {
    let (v, w) = if v <= w { (v, w) } else { (w, v) };
    conflicts
        .entry(v.to_string())
        .or_default()
        .insert(w.to_string());
}

pub fn has_conflict(conflicts: &Conflicts, v: &str, w: &str) -> bool {
    let (v, w) = if v <= w { (v, w) } else { (w, v) };
    conflicts.get(v).map(|m| m.contains(w)).unwrap_or(false)
}

type ConflictsRef<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn add_conflict_ref<'a>(conflicts: &mut ConflictsRef<'a>, v: &'a str, w: &'a str) {
    let (v, w) = if v <= w { (v, w) } else { (w, v) };
    conflicts.entry(v).or_default().insert(w);
}

fn has_conflict_ref(conflicts: &ConflictsRef<'_>, v: &str, w: &str) -> bool {
    let (v, w) = if v <= w { (v, w) } else { (w, v) };
    conflicts.get(v).is_some_and(|m| m.contains(w))
}

fn first_dummy_predecessor<'a>(
    g: &'a Graph<NodeLabel, EdgeLabel, GraphLabel>,
    v: &str,
) -> Option<&'a str> {
    let mut out: Option<&'a str> = None;
    g.for_each_predecessor(v, |u| {
        if out.is_some() {
            return;
        }
        if g.node(u).map(|n| n.dummy.is_some()).unwrap_or(false) {
            out = Some(u);
        }
    });
    out
}

pub fn find_type1_conflicts(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &[Vec<String>],
) -> Conflicts {
    let mut conflicts: Conflicts = BTreeMap::new();
    if layering.is_empty() {
        return conflicts;
    }

    for i in 1..layering.len() {
        let prev_layer = &layering[i - 1];
        let layer = &layering[i];

        let mut k0: usize = 0;
        let mut scan_pos: usize = 0;
        let prev_layer_len = prev_layer.len();
        let last_node = layer.last().map(|s| s.as_str());

        for (idx, v) in layer.iter().enumerate() {
            let w = find_other_inner_segment_node(g, v);
            let k1 = w
                .as_deref()
                .and_then(|w| g.node(w))
                .and_then(|n| n.order)
                .unwrap_or(prev_layer_len);

            if w.is_some() || last_node == Some(v.as_str()) {
                for scan_node in layer.iter().skip(scan_pos).take(idx + 1 - scan_pos) {
                    let scan_dummy = g
                        .node(scan_node)
                        .map(|n| n.dummy.is_some())
                        .unwrap_or(false);
                    g.for_each_predecessor(scan_node, |u| {
                        let Some(u_label) = g.node(u) else {
                            return;
                        };
                        let u_pos = u_label.order.unwrap_or(0);
                        let u_dummy = u_label.dummy.is_some();

                        if (u_pos < k0 || k1 < u_pos) && !(u_dummy && scan_dummy) {
                            add_conflict(&mut conflicts, u, scan_node);
                        }
                    });
                }
                scan_pos = idx + 1;
                k0 = k1;
            }
        }
    }

    conflicts
}

pub fn find_type2_conflicts(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &[Vec<String>],
) -> Conflicts {
    let mut conflicts: Conflicts = BTreeMap::new();
    if layering.is_empty() {
        return conflicts;
    }

    fn scan(
        g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
        conflicts: &mut Conflicts,
        south: &[String],
        south_pos: usize,
        south_end: usize,
        prev_north_border: isize,
        next_north_border: isize,
    ) {
        for v in south.iter().take(south_end).skip(south_pos) {
            let v_dummy = g.node(v).and_then(|n| n.dummy.as_deref());
            if v_dummy.is_some() {
                g.for_each_predecessor(v, |u| {
                    let Some(u_node) = g.node(u) else {
                        return;
                    };
                    if u_node.dummy.is_some() {
                        let u_order = u_node.order.unwrap_or(0) as isize;
                        if u_order < prev_north_border || u_order > next_north_border {
                            add_conflict(conflicts, u, v);
                        }
                    }
                });
            }
        }
    }

    for i in 1..layering.len() {
        let north = &layering[i - 1];
        let south = &layering[i];

        let mut prev_north_pos: isize = -1;
        let mut next_north_pos: Option<isize> = None;
        let mut south_pos: usize = 0;

        for (south_lookahead, v) in south.iter().enumerate() {
            let is_border = g
                .node(v)
                .and_then(|n| n.dummy.as_deref())
                .is_some_and(|d| d == "border");
            if is_border {
                let mut first: Option<&str> = None;
                g.for_each_predecessor(v, |u| {
                    if first.is_none() {
                        first = Some(u);
                    }
                });
                if let Some(u) = first {
                    next_north_pos = g.node(u).and_then(|n| n.order).map(|n| n as isize);
                    scan(
                        g,
                        &mut conflicts,
                        south,
                        south_pos,
                        south_lookahead,
                        prev_north_pos,
                        next_north_pos.unwrap_or(-1),
                    );
                    south_pos = south_lookahead;
                    prev_north_pos = next_north_pos.unwrap_or(prev_north_pos);
                }
            }

            scan(
                g,
                &mut conflicts,
                south,
                south_pos,
                south.len(),
                next_north_pos.unwrap_or(-1),
                north.len() as isize,
            );
        }
    }

    conflicts
}

fn find_other_inner_segment_node(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    v: &str,
) -> Option<String> {
    if g.node(v).map(|n| n.dummy.is_some()).unwrap_or(false) {
        return first_dummy_predecessor(g, v).map(|u| u.to_string());
    }
    None
}

fn find_other_inner_segment_node_ref<'a>(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    v: &'a str,
    canon: &HashMap<&'a str, &'a str>,
) -> Option<&'a str> {
    if g.node(v).map(|n| n.dummy.is_some()).unwrap_or(false) {
        let u = first_dummy_predecessor(g, v)?;
        canon.get(u).copied()
    } else {
        None
    }
}

fn find_type1_conflicts_ref<'a>(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &[Vec<&'a str>],
    canon: &HashMap<&'a str, &'a str>,
) -> ConflictsRef<'a> {
    let mut conflicts: ConflictsRef<'a> = HashMap::default();
    if layering.is_empty() {
        return conflicts;
    }

    for i in 1..layering.len() {
        let prev_layer = &layering[i - 1];
        let layer = &layering[i];

        let mut k0: usize = 0;
        let mut scan_pos: usize = 0;
        let prev_layer_len = prev_layer.len();
        let last_node = layer.last().copied();

        for (idx, &v) in layer.iter().enumerate() {
            let w = find_other_inner_segment_node_ref(g, v, canon);
            let k1 = w
                .and_then(|w| g.node(w))
                .and_then(|n| n.order)
                .unwrap_or(prev_layer_len);

            if w.is_some() || last_node == Some(v) {
                for &scan_node in layer.iter().skip(scan_pos).take(idx + 1 - scan_pos) {
                    let scan_dummy = g
                        .node(scan_node)
                        .map(|n| n.dummy.is_some())
                        .unwrap_or(false);
                    g.for_each_predecessor(scan_node, |u| {
                        let Some(&u) = canon.get(u) else {
                            return;
                        };
                        let Some(u_label) = g.node(u) else {
                            return;
                        };
                        let u_pos = u_label.order.unwrap_or(0);
                        let u_dummy = u_label.dummy.is_some();

                        if (u_pos < k0 || k1 < u_pos) && !(u_dummy && scan_dummy) {
                            add_conflict_ref(&mut conflicts, u, scan_node);
                        }
                    });
                }
                scan_pos = idx + 1;
                k0 = k1;
            }
        }
    }

    conflicts
}

fn find_type2_conflicts_ref<'a>(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &[Vec<&'a str>],
    canon: &HashMap<&'a str, &'a str>,
) -> ConflictsRef<'a> {
    let mut conflicts: ConflictsRef<'a> = HashMap::default();
    if layering.is_empty() {
        return conflicts;
    }

    fn scan<'a>(
        g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
        canon: &HashMap<&'a str, &'a str>,
        conflicts: &mut ConflictsRef<'a>,
        south: &[&'a str],
        south_range: std::ops::Range<usize>,
        prev_north_border: isize,
        next_north_border: isize,
    ) {
        for &v in south.iter().take(south_range.end).skip(south_range.start) {
            let v_dummy = g.node(v).and_then(|n| n.dummy.as_deref());
            if v_dummy.is_some() {
                g.for_each_predecessor(v, |u| {
                    let Some(&u) = canon.get(u) else {
                        return;
                    };
                    let Some(u_node) = g.node(u) else {
                        return;
                    };
                    if u_node.dummy.is_some() {
                        let u_order = u_node.order.unwrap_or(0) as isize;
                        if u_order < prev_north_border || u_order > next_north_border {
                            add_conflict_ref(conflicts, u, v);
                        }
                    }
                });
            }
        }
    }

    for i in 1..layering.len() {
        let north = &layering[i - 1];
        let south = &layering[i];

        let mut prev_north_pos: isize = -1;
        let mut next_north_pos: Option<isize> = None;
        let mut south_pos: usize = 0;

        for (south_lookahead, &v) in south.iter().enumerate() {
            let is_border = g
                .node(v)
                .and_then(|n| n.dummy.as_deref())
                .is_some_and(|d| d == "border");
            if is_border {
                let mut first: Option<&str> = None;
                g.for_each_predecessor(v, |u| {
                    if first.is_none() {
                        first = Some(u);
                    }
                });
                if let Some(u) = first.and_then(|u| canon.get(u).copied()) {
                    next_north_pos = g.node(u).and_then(|n| n.order).map(|n| n as isize);
                    scan(
                        g,
                        canon,
                        &mut conflicts,
                        south,
                        south_pos..south_lookahead,
                        prev_north_pos,
                        next_north_pos.unwrap_or(-1),
                    );
                    south_pos = south_lookahead;
                    prev_north_pos = next_north_pos.unwrap_or(prev_north_pos);
                }
            }

            scan(
                g,
                canon,
                &mut conflicts,
                south,
                south_pos..south.len(),
                next_north_pos.unwrap_or(-1),
                north.len() as isize,
            );
        }
    }

    conflicts
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alignment {
    pub root: HashMap<String, String>,
    pub align: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
struct AlignmentRef<'a> {
    root: HashMap<&'a str, &'a str>,
    align: HashMap<&'a str, &'a str>,
}

pub fn vertical_alignment<F>(
    _g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &[Vec<String>],
    conflicts: &Conflicts,
    neighbor_fn: F,
) -> Alignment
where
    F: Fn(&str) -> Vec<String>,
{
    let mut root: HashMap<String, String> = HashMap::default();
    let mut align: HashMap<String, String> = HashMap::default();
    let mut pos: HashMap<String, usize> = HashMap::default();

    for layer in layering {
        for (order, v) in layer.iter().enumerate() {
            root.insert(v.clone(), v.clone());
            align.insert(v.clone(), v.clone());
            pos.insert(v.clone(), order);
        }
    }

    for layer in layering {
        let mut prev_idx: isize = -1;
        for v in layer {
            let mut ws = neighbor_fn(v);
            if ws.is_empty() {
                continue;
            }
            ws.sort_by_key(|w| pos.get(w).copied().unwrap_or(usize::MAX));

            let mp = (ws.len() - 1) as f64 / 2.0;
            let i0 = mp.floor() as usize;
            let i1 = mp.ceil() as usize;

            for w in ws.iter().take(i1 + 1).skip(i0) {
                let v_align = align.get(v).cloned().unwrap_or_else(|| v.clone());
                let w_pos = pos.get(w).copied().unwrap_or(usize::MAX) as isize;
                if v_align == *v && prev_idx < w_pos && !has_conflict(conflicts, v, w) {
                    align.insert(w.clone(), v.clone());
                    let w_root = root.get(w).cloned().unwrap_or_else(|| w.clone());
                    align.insert(v.clone(), w_root.clone());
                    root.insert(v.clone(), w_root);
                    prev_idx = w_pos;
                }
            }
        }
    }

    Alignment { root, align }
}

fn vertical_alignment_ref_fast<'a>(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &[Vec<&'a str>],
    conflicts: &ConflictsRef<'a>,
    canon: &HashMap<&'a str, &'a str>,
    use_predecessors: bool,
) -> AlignmentRef<'a> {
    let mut root: HashMap<&'a str, &'a str> = HashMap::default();
    let mut align: HashMap<&'a str, &'a str> = HashMap::default();
    let mut pos: HashMap<&'a str, usize> = HashMap::default();

    for layer in layering {
        for (order, &v) in layer.iter().enumerate() {
            root.insert(v, v);
            align.insert(v, v);
            pos.insert(v, order);
        }
    }

    let mut ws_raw: Vec<&str> = Vec::new();
    let mut ws: Vec<&'a str> = Vec::new();
    for layer in layering {
        let mut prev_idx: isize = -1;
        for &v in layer {
            ws_raw.clear();
            if use_predecessors {
                g.extend_predecessors(v, &mut ws_raw);
            } else {
                g.extend_successors(v, &mut ws_raw);
            }
            if ws_raw.is_empty() {
                continue;
            }

            ws.clear();
            for w in &ws_raw {
                let Some(&w) = canon.get(*w) else {
                    continue;
                };
                ws.push(w);
            }
            if ws.is_empty() {
                continue;
            }

            ws.sort_by_key(|w| pos.get(w).copied().unwrap_or(usize::MAX));

            let mp = (ws.len() - 1) as f64 / 2.0;
            let i0 = mp.floor() as usize;
            let i1 = mp.ceil() as usize;

            for &w in ws.iter().take(i1 + 1).skip(i0) {
                let v_align = align.get(v).copied().unwrap_or(v);
                let w_pos = pos.get(w).copied().unwrap_or(usize::MAX) as isize;
                if v_align == v && prev_idx < w_pos && !has_conflict_ref(conflicts, v, w) {
                    align.insert(w, v);
                    let w_root = root.get(w).copied().unwrap_or(w);
                    align.insert(v, w_root);
                    root.insert(v, w_root);
                    prev_idx = w_pos;
                }
            }
        }
    }

    AlignmentRef { root, align }
}

fn horizontal_compaction_ref_fast<'a>(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &[Vec<&'a str>],
    root: &HashMap<&'a str, &'a str>,
    align: &HashMap<&'a str, &'a str>,
    reverse_sep: bool,
) -> HashMap<&'a str, f64> {
    let border_type = if reverse_sep {
        "borderLeft"
    } else {
        "borderRight"
    };

    let mut root_to_idx: HashMap<&'a str, usize> = HashMap::default();
    let mut idx_to_root: Vec<&'a str> = Vec::new();
    let mut edges: HashMap<(usize, usize), f64> = HashMap::default();

    let mut ensure_root = |id: &'a str| -> usize {
        if let Some(&idx) = root_to_idx.get(id) {
            return idx;
        }
        let idx = idx_to_root.len();
        idx_to_root.push(id);
        root_to_idx.insert(id, idx);
        idx
    };

    for layer in layering {
        let mut u: Option<&'a str> = None;
        for &v in layer {
            let v_root = root.get(v).copied().unwrap_or(v);
            let v_idx = ensure_root(v_root);

            if let Some(u) = u {
                let u_root = root.get(u).copied().unwrap_or(u);
                let u_idx = ensure_root(u_root);
                let sep = sep(g, v, u, reverse_sep);
                edges
                    .entry((u_idx, v_idx))
                    .and_modify(|w| {
                        if sep > *w {
                            *w = sep;
                        }
                    })
                    .or_insert(sep);
            }

            u = Some(v);
        }
    }

    let node_count = idx_to_root.len();
    let mut preds: Vec<Vec<(usize, f64)>> = vec![Vec::new(); node_count];
    let mut succs: Vec<Vec<(usize, f64)>> = vec![Vec::new(); node_count];
    for ((u, v), w) in edges {
        preds[v].push((u, w));
        succs[u].push((v, w));
    }

    let mut xs: Vec<f64> = vec![0.0; node_count];

    // First pass: assign smallest coordinates (postorder over predecessors)
    {
        let mut scheduled: Vec<bool> = vec![false; node_count];
        let mut stack: Vec<(usize, bool)> = (0..node_count).map(|i| (i, false)).collect();
        while let Some((elem, expanded)) = stack.pop() {
            if expanded {
                let mut best: f64 = 0.0;
                for &(p, w) in &preds[elem] {
                    best = best.max(xs[p] + w);
                }
                xs[elem] = best;
                continue;
            }

            if scheduled[elem] {
                continue;
            }
            scheduled[elem] = true;

            stack.push((elem, true));
            for &(p, _w) in &preds[elem] {
                stack.push((p, false));
            }
        }
    }

    // Second pass: assign greatest coordinates (postorder over successors)
    {
        let mut scheduled: Vec<bool> = vec![false; node_count];
        let mut stack: Vec<(usize, bool)> = (0..node_count).map(|i| (i, false)).collect();
        while let Some((elem, expanded)) = stack.pop() {
            if expanded {
                let mut min: f64 = f64::INFINITY;
                for &(w, weight) in &succs[elem] {
                    min = min.min(xs[w] - weight);
                }

                let Some(node) = g.node(idx_to_root[elem]) else {
                    continue;
                };

                if min.is_finite() && node.border_type.as_deref() != Some(border_type) {
                    xs[elem] = xs[elem].max(min);
                }
                continue;
            }

            if scheduled[elem] {
                continue;
            }
            scheduled[elem] = true;

            stack.push((elem, true));
            for &(w, _weight) in &succs[elem] {
                stack.push((w, false));
            }
        }
    }

    // Assign x coordinates to all nodes based on their block root.
    let mut out: HashMap<&'a str, f64> = HashMap::default();
    for (&v, &r) in align {
        let block = root.get(v).copied().unwrap_or(r);
        let x = root_to_idx
            .get(block)
            .copied()
            .and_then(|idx| xs.get(idx).copied())
            .unwrap_or(0.0);
        out.insert(v, x);
    }
    out
}

fn find_smallest_width_alignment_ref<'a>(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    xss: &HashMap<&'static str, HashMap<&'a str, f64>>,
) -> HashMap<&'a str, f64> {
    let mut best_width: f64 = f64::INFINITY;
    let mut best: HashMap<&'a str, f64> = HashMap::default();

    for key in ["ul", "ur", "dl", "dr"] {
        let Some(xs) = xss.get(key) else {
            continue;
        };
        let mut max: f64 = f64::NEG_INFINITY;
        let mut min: f64 = f64::INFINITY;
        for (&v, &x) in xs {
            let half_w = width(g, v) / 2.0;
            max = max.max(x + half_w);
            min = min.min(x - half_w);
        }
        let w = max - min;
        if w < best_width {
            best_width = w;
            best = xs.clone();
        }
    }

    best
}

fn align_coordinates_ref<'a>(
    xss: &mut HashMap<&'static str, HashMap<&'a str, f64>>,
    align_to: &HashMap<&'a str, f64>,
) {
    let align_to_min = align_to.values().copied().fold(f64::INFINITY, f64::min);
    let align_to_max = align_to.values().copied().fold(f64::NEG_INFINITY, f64::max);

    for key in ["ul", "ur", "dl", "dr"] {
        let Some(xs) = xss.get_mut(key) else {
            continue;
        };

        let xs_min = xs.values().copied().fold(f64::INFINITY, f64::min);
        let xs_max = xs.values().copied().fold(f64::NEG_INFINITY, f64::max);

        let mut delta = align_to_min - xs_min;
        if key.ends_with('r') {
            delta = align_to_max - xs_max;
        }

        if delta != 0.0 {
            for x in xs.values_mut() {
                *x += delta;
            }
        }
    }
}

fn balance_ref<'a>(
    xss: &HashMap<&'static str, HashMap<&'a str, f64>>,
    align: Option<&str>,
) -> HashMap<&'a str, f64> {
    let Some(xs_ul) = xss.get("ul") else {
        return HashMap::default();
    };

    let align_key = align.map(|a| a.to_ascii_lowercase());

    let mut out: HashMap<&'a str, f64> = HashMap::default();
    for &v in xs_ul.keys() {
        if let Some(key) = align_key.as_deref() {
            let x = xss
                .get(key)
                .and_then(|xs| xs.get(v).copied())
                .unwrap_or(0.0);
            out.insert(v, x);
            continue;
        }

        let mut vals: Vec<f64> = xss.values().filter_map(|xs| xs.get(v).copied()).collect();
        vals.sort_by(|a, b| a.total_cmp(b));
        if vals.len() >= 4 {
            out.insert(v, (vals[1] + vals[2]) / 2.0);
        }
    }
    out
}

fn position_x_with_layering_ref<'a>(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &'a [Vec<String>],
) -> HashMap<&'a str, f64> {
    if layering.is_empty() {
        return HashMap::default();
    }

    let mut canon: HashMap<&'a str, &'a str> = HashMap::default();
    let layering_ref: Vec<Vec<&'a str>> = layering
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|id| {
                    let id = id.as_str();
                    canon.insert(id, id);
                    id
                })
                .collect()
        })
        .collect();

    let mut conflicts = find_type1_conflicts_ref(g, &layering_ref, &canon);
    let type2 = find_type2_conflicts_ref(g, &layering_ref, &canon);
    for (v, ws) in type2 {
        for w in ws {
            add_conflict_ref(&mut conflicts, v, w);
        }
    }

    let mut xss: HashMap<&'static str, HashMap<&'a str, f64>> = HashMap::default();

    for reverse_layers in [false, true] {
        for reverse_inner in [false, true] {
            let key: &'static str = match (reverse_layers, reverse_inner) {
                (false, false) => "ul",
                (false, true) => "ur",
                (true, false) => "dl",
                (true, true) => "dr",
            };

            let mut adjusted_layering: Vec<Vec<&'a str>> = Vec::with_capacity(layering_ref.len());
            if reverse_layers {
                for layer in layering_ref.iter().rev() {
                    if reverse_inner {
                        adjusted_layering.push(layer.iter().rev().copied().collect());
                    } else {
                        adjusted_layering.push(layer.clone());
                    }
                }
            } else {
                for layer in layering_ref.iter() {
                    if reverse_inner {
                        adjusted_layering.push(layer.iter().rev().copied().collect());
                    } else {
                        adjusted_layering.push(layer.clone());
                    }
                }
            }

            let align = vertical_alignment_ref_fast(
                g,
                &adjusted_layering,
                &conflicts,
                &canon,
                !reverse_layers,
            );
            let mut xs = horizontal_compaction_ref_fast(
                g,
                &adjusted_layering,
                &align.root,
                &align.align,
                reverse_inner,
            );
            if reverse_inner {
                for x in xs.values_mut() {
                    *x = -*x;
                }
            }

            xss.insert(key, xs);
        }
    }

    let smallest = find_smallest_width_alignment_ref(g, &xss);
    align_coordinates_ref(&mut xss, &smallest);
    balance_ref(&xss, g.graph().align.as_deref())
}

pub fn position_x_with_layering(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &[Vec<String>],
) -> HashMap<String, f64> {
    position_x_with_layering_ref(g, layering)
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
}

pub fn horizontal_compaction(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &[Vec<String>],
    root: &HashMap<String, String>,
    align: &HashMap<String, String>,
    reverse_sep: bool,
) -> HashMap<String, f64> {
    let root_ref: HashMap<&'_ str, &'_ str> =
        root.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let align_ref: HashMap<&'_ str, &'_ str> = align
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    horizontal_compaction_ref(g, layering, &root_ref, &align_ref, reverse_sep)
}

fn horizontal_compaction_ref<'a>(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &[Vec<String>],
    root: &HashMap<&'a str, &'a str>,
    align: &HashMap<&'a str, &'a str>,
    reverse_sep: bool,
) -> HashMap<String, f64> {
    let mut xs: HashMap<String, f64> = HashMap::default();
    let block_g = build_block_graph_ref(g, layering, root, reverse_sep);
    let border_type = if reverse_sep {
        "borderLeft"
    } else {
        "borderRight"
    };

    fn iterate_predecessors<'a, F>(block_g: &'a Graph<(), f64, ()>, mut set_xs: F)
    where
        F: FnMut(&'a str),
    {
        let mut stack: Vec<&'a str> = block_g.nodes().collect();
        let mut entered: HashSet<&'a str> = HashSet::default();
        let mut scratch: Vec<&'a str> = Vec::new();

        while let Some(elem) = stack.pop() {
            if entered.contains(elem) {
                set_xs(elem);
                continue;
            }

            entered.insert(elem);
            stack.push(elem);

            scratch.clear();
            block_g.extend_predecessors(elem, &mut scratch);
            stack.extend(scratch.iter().copied());
        }
    }

    fn iterate_successors<'a, F>(block_g: &'a Graph<(), f64, ()>, mut set_xs: F)
    where
        F: FnMut(&'a str),
    {
        let mut stack: Vec<&'a str> = block_g.nodes().collect();
        let mut entered: HashSet<&'a str> = HashSet::default();
        let mut scratch: Vec<&'a str> = Vec::new();

        while let Some(elem) = stack.pop() {
            if entered.contains(elem) {
                set_xs(elem);
                continue;
            }

            entered.insert(elem);
            stack.push(elem);

            scratch.clear();
            block_g.extend_successors(elem, &mut scratch);
            stack.extend(scratch.iter().copied());
        }
    }

    // First pass: assign smallest coordinates
    {
        let mut set = |elem: &str| {
            let mut best: f64 = 0.0;
            block_g.for_each_in_edge(elem, None, |ek, w| {
                let x_v = xs.get(&ek.v).copied().unwrap_or(0.0);
                best = best.max(x_v + *w);
            });
            xs.insert(elem.to_string(), best);
        };
        iterate_predecessors(&block_g, &mut set);
    }

    // Second pass: assign greatest coordinates
    {
        let mut set = |elem: &str| {
            let mut min: f64 = f64::INFINITY;
            block_g.for_each_out_edge(elem, None, |ek, w| {
                let x_w = xs.get(&ek.w).copied().unwrap_or(0.0);
                min = min.min(x_w - *w);
            });

            let node = g.node(elem);
            let Some(node) = node else {
                return;
            };
            if min.is_finite() && node.border_type.as_deref() != Some(border_type) {
                let cur = xs.get(elem).copied().unwrap_or(0.0);
                xs.insert(elem.to_string(), cur.max(min));
            }
        };
        iterate_successors(&block_g, &mut set);
    }

    // Assign x coordinates to all nodes based on their block root.
    let mut out: HashMap<String, f64> = HashMap::default();
    for (&v, &r) in align {
        let x = xs
            .get(root.get(v).copied().unwrap_or(r))
            .copied()
            .unwrap_or(0.0);
        out.insert(v.to_string(), x);
    }
    out
}

fn build_block_graph_ref<'a>(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    layering: &[Vec<String>],
    root: &HashMap<&'a str, &'a str>,
    reverse_sep: bool,
) -> Graph<(), f64, ()> {
    let mut block_graph: Graph<(), f64, ()> = Graph::new(GraphOptions::default());
    for layer in layering {
        let mut u: Option<&str> = None;
        for v in layer {
            let v = v.as_str();
            let v_root = root.get(v).copied().unwrap_or(v);
            block_graph.ensure_node(v_root.to_string());

            if let Some(u) = u {
                let u_root = root.get(u).copied().unwrap_or(u);
                let prev_max = block_graph
                    .edge(u_root, v_root, None)
                    .copied()
                    .unwrap_or(0.0);
                let sep = sep(g, v, u, reverse_sep);
                block_graph.set_edge_with_label(
                    u_root.to_string(),
                    v_root.to_string(),
                    sep.max(prev_max),
                );
            }

            u = Some(v);
        }
    }
    block_graph
}

pub fn find_smallest_width_alignment(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    xss: &HashMap<String, HashMap<String, f64>>,
) -> HashMap<String, f64> {
    let mut best_width: f64 = f64::INFINITY;
    let mut best: HashMap<String, f64> = HashMap::default();

    // Match upstream dagre: ties are resolved by a stable iteration order over alignments.
    // The canonical order is: `ul`, `ur`, `dl`, `dr` (insertion order in upstream).
    for key in ["ul", "ur", "dl", "dr"] {
        let Some(xs) = xss.get(key) else {
            continue;
        };
        let mut max: f64 = f64::NEG_INFINITY;
        let mut min: f64 = f64::INFINITY;
        for (v, x) in xs {
            let half_w = width(g, v) / 2.0;
            max = max.max(x + half_w);
            min = min.min(x - half_w);
        }
        let w = max - min;
        if w < best_width {
            best_width = w;
            best = xs.clone();
        }
    }

    best
}

pub fn align_coordinates(
    xss: &mut HashMap<String, HashMap<String, f64>>,
    align_to: &HashMap<String, f64>,
) {
    let align_to_min = align_to.values().copied().fold(f64::INFINITY, f64::min);
    let align_to_max = align_to.values().copied().fold(f64::NEG_INFINITY, f64::max);

    for (vert, horiz) in [("u", "l"), ("u", "r"), ("d", "l"), ("d", "r")] {
        let key = format!("{vert}{horiz}");
        let Some(xs) = xss.get(&key).cloned() else {
            continue;
        };

        let xs_min = xs.values().copied().fold(f64::INFINITY, f64::min);
        let xs_max = xs.values().copied().fold(f64::NEG_INFINITY, f64::max);

        let mut delta = align_to_min - xs_min;
        if horiz != "l" {
            delta = align_to_max - xs_max;
        }

        if delta != 0.0 {
            xss.insert(key, xs.into_iter().map(|(v, x)| (v, x + delta)).collect());
        }
    }
}

pub fn balance(
    xss: &HashMap<String, HashMap<String, f64>>,
    align: Option<&str>,
) -> HashMap<String, f64> {
    let Some(xs_ul) = xss.get("ul") else {
        return HashMap::default();
    };

    let align_key = align.map(|a| a.to_ascii_lowercase());

    let mut out: HashMap<String, f64> = HashMap::default();
    for v in xs_ul.keys() {
        if let Some(key) = align_key.as_deref() {
            let x = xss
                .get(key)
                .and_then(|xs| xs.get(v))
                .copied()
                .unwrap_or(0.0);
            out.insert(v.clone(), x);
            continue;
        }

        let mut vals: Vec<f64> = xss.values().filter_map(|xs| xs.get(v).copied()).collect();
        vals.sort_by(|a, b| a.total_cmp(b));
        if vals.len() >= 4 {
            out.insert(v.clone(), (vals[1] + vals[2]) / 2.0);
        }
    }
    out
}

pub fn position_x(g: &Graph<NodeLabel, EdgeLabel, GraphLabel>) -> HashMap<String, f64> {
    let layering = crate::util::build_layer_matrix(g);
    position_x_with_layering(g, &layering)
}

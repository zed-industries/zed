//! Barycenter computation and conflict resolution.
//!
//! Ported from Dagre's `barycenter`, `resolveConflicts`, and `sortSubgraph` helpers.

use super::{OrderEdgeWeight, OrderNodeLabel};
use crate::graphlib::Graph;
use rustc_hash::FxHashMap as HashMap;
use web_time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub struct BarycenterEntry {
    pub v: String,
    pub barycenter: Option<f64>,
    pub weight: Option<f64>,
}

pub fn barycenter<N, E, G>(g: &Graph<N, E, G>, movable: &[String]) -> Vec<BarycenterEntry>
where
    N: Default + OrderNodeLabel + 'static,
    E: Default + OrderEdgeWeight + 'static,
    G: Default,
{
    movable
        .iter()
        .map(|v| {
            let Some(v_ix) = g.node_ix(v.as_str()) else {
                return BarycenterEntry {
                    v: v.clone(),
                    barycenter: None,
                    weight: None,
                };
            };

            let mut saw_edge = false;
            let mut sum: f64 = 0.0;
            let mut weight: f64 = 0.0;
            g.for_each_in_edge_ix(v_ix, None, |u_ix, _w_ix, _ek, lbl| {
                saw_edge = true;
                let edge_weight = lbl.weight();
                let u_order = g
                    .node_label_by_ix(u_ix)
                    .and_then(|n| n.order())
                    .map(|n| n as f64)
                    .unwrap_or(0.0);
                sum += edge_weight * u_order;
                weight += edge_weight;
            });
            if !saw_edge {
                return BarycenterEntry {
                    v: v.clone(),
                    barycenter: None,
                    weight: None,
                };
            }

            BarycenterEntry {
                v: v.clone(),
                barycenter: Some(sum / weight),
                weight: Some(weight),
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct SortEntry {
    pub vs: Vec<String>,
    pub i: usize,
    pub barycenter: Option<f64>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone)]
struct ConflictEntry {
    indegree: usize,
    ins: Vec<usize>,
    outs: Vec<usize>,
    vs: Vec<usize>,
    i: usize,
    barycenter: Option<f64>,
    weight: Option<f64>,
    merged: bool,
}

pub fn resolve_conflicts<N, E, G>(
    entries: &[BarycenterEntry],
    cg: &Graph<N, E, G>,
) -> Vec<SortEntry>
where
    N: Default + 'static,
    E: Default + 'static,
    G: Default,
{
    let mut id_to_ix: HashMap<&str, usize> = HashMap::default();
    let mut conflicts: Vec<ConflictEntry> = Vec::with_capacity(entries.len());
    for (ix, entry) in entries.iter().enumerate() {
        id_to_ix.insert(entry.v.as_str(), ix);
        conflicts.push(ConflictEntry {
            indegree: 0,
            ins: Vec::new(),
            outs: Vec::new(),
            vs: vec![ix],
            i: ix,
            barycenter: entry.barycenter,
            weight: entry.weight,
            merged: false,
        });
    }

    for e in cg.edges() {
        let Some(&v_ix) = id_to_ix.get(e.v.as_str()) else {
            continue;
        };
        let Some(&w_ix) = id_to_ix.get(e.w.as_str()) else {
            continue;
        };

        conflicts[w_ix].indegree += 1;
        conflicts[v_ix].outs.push(w_ix);
    }

    // Dagre/Graphlib effectively preserves insertion order when building `sourceSet`. The
    // conflict resolution loop consumes this list via `pop()` (LIFO), so the push order is a
    // load-bearing tie-breaker for symmetric graphs. Prefer the input slice order here to
    // match upstream and avoid mirrored / drifted layouts.
    let mut source_set: Vec<usize> = Vec::with_capacity(entries.len());
    for (ix, _entry) in entries.iter().enumerate() {
        if conflicts[ix].indegree == 0 {
            source_set.push(ix);
        }
    }

    let mut processed: Vec<usize> = Vec::new();
    while let Some(v_ix) = source_set.pop() {
        processed.push(v_ix);

        let ins = std::mem::take(&mut conflicts[v_ix].ins);

        // Match upstream `.reverse().forEach(...)` on the "in" list.
        for u in ins.into_iter().rev() {
            if conflicts[u].merged {
                continue;
            }
            let u_bary = conflicts[u].barycenter;
            let v_bary = conflicts[v_ix].barycenter;
            let should_merge = match (u_bary, v_bary) {
                (None, _) => true,
                (_, None) => true,
                (Some(ub), Some(vb)) => ub >= vb,
            };
            if should_merge {
                merge_conflict_entries(&mut conflicts, v_ix, u);
            }
        }

        let outs = std::mem::take(&mut conflicts[v_ix].outs);
        for w_ix in outs {
            conflicts[w_ix].ins.push(v_ix);
            conflicts[w_ix].indegree = conflicts[w_ix].indegree.saturating_sub(1);
            if conflicts[w_ix].indegree == 0 {
                source_set.push(w_ix);
            }
        }
    }

    let mut out: Vec<SortEntry> = Vec::new();
    for id in processed {
        let entry = &conflicts[id];
        if entry.merged {
            continue;
        }
        let mut vs: Vec<String> = Vec::with_capacity(entry.vs.len());
        for &ix in &entry.vs {
            vs.push(entries[ix].v.clone());
        }
        out.push(SortEntry {
            vs,
            i: entry.i,
            barycenter: entry.barycenter,
            weight: entry.weight,
        });
    }
    out
}

// The conflict resolution algorithm needs a helper that can mutate two entries in-place.
// We keep it as a standalone function to make the port easy to review.
fn merge_conflict_entries(mapped: &mut [ConflictEntry], target: usize, source: usize) {
    if target == source {
        return;
    }

    let (t, s) = if target < source {
        let (left, right) = mapped.split_at_mut(source);
        (&mut left[target], &mut right[0])
    } else {
        let (left, right) = mapped.split_at_mut(target);
        (&mut right[0], &mut left[source])
    };

    let target_bary = t.barycenter;
    let target_weight = t.weight;
    let source_bary = s.barycenter;
    let source_weight = s.weight;
    let source_i = s.i;

    let mut sum: f64 = 0.0;
    let mut weight: f64 = 0.0;
    if let (Some(b), Some(w)) = (target_bary, target_weight) {
        if w != 0.0 {
            sum += b * w;
            weight += w;
        }
    }
    if let (Some(b), Some(w)) = (source_bary, source_weight) {
        if w != 0.0 {
            sum += b * w;
            weight += w;
        }
    }

    let mut merged_vs = std::mem::take(&mut s.vs);
    merged_vs.extend(std::mem::take(&mut t.vs));
    t.vs = merged_vs;

    if weight != 0.0 {
        t.barycenter = Some(sum / weight);
        t.weight = Some(weight);
    }
    t.i = t.i.min(source_i);

    s.merged = true;
}

#[derive(Debug, Clone, PartialEq)]
pub struct SortResult {
    pub vs: Vec<String>,
    pub barycenter: Option<f64>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BarycenterEntryIx {
    pub v_ix: usize,
    pub barycenter: Option<f64>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SortEntryIx {
    pub vs: Vec<usize>,
    pub i: usize,
    pub barycenter: Option<f64>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SortResultIx {
    pub vs: Vec<usize>,
    pub barycenter: Option<f64>,
    pub weight: Option<f64>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct SortSubgraphTimings {
    pub total: Duration,
    pub build_movable: Duration,
    pub barycenter: Duration,
    pub resolve_conflicts: Duration,
    pub expand_subgraphs: Duration,
    pub sort: Duration,
    pub border_adjust: Duration,
}

pub fn sort(entries: &[SortEntry], bias_right: bool) -> SortResult {
    let mut total_len: usize = 0;
    let mut sortable: Vec<usize> = Vec::new();
    let mut unsortable: Vec<usize> = Vec::new();

    for (ix, entry) in entries.iter().enumerate() {
        total_len += entry.vs.len();
        if entry.barycenter.is_some() {
            sortable.push(ix);
        } else {
            unsortable.push(ix);
        }
    }

    unsortable.sort_by(|&a, &b| entries[b].i.cmp(&entries[a].i));

    sortable.sort_by(|&a, &b| {
        let a_entry = &entries[a];
        let b_entry = &entries[b];
        let a_bc = a_entry.barycenter.unwrap_or(0.0);
        let b_bc = b_entry.barycenter.unwrap_or(0.0);
        if a_bc < b_bc {
            std::cmp::Ordering::Less
        } else if a_bc > b_bc {
            std::cmp::Ordering::Greater
        } else if !bias_right {
            a_entry.i.cmp(&b_entry.i)
        } else {
            b_entry.i.cmp(&a_entry.i)
        }
    });

    let mut out: Vec<String> = Vec::with_capacity(total_len);
    let mut sum: f64 = 0.0;
    let mut weight: f64 = 0.0;
    let mut vs_index: usize = 0;

    fn consume_unsortable(
        out: &mut Vec<String>,
        entries: &[SortEntry],
        unsortable: &mut Vec<usize>,
        mut index: usize,
    ) -> usize {
        while let Some(&last_ix) = unsortable.last() {
            let last = &entries[last_ix];
            if last.i > index {
                break;
            }
            let Some(last_ix) = unsortable.pop() else {
                break;
            };
            out.extend(entries[last_ix].vs.iter().cloned());
            index += 1;
        }
        index
    }

    vs_index = consume_unsortable(&mut out, entries, &mut unsortable, vs_index);

    for entry_ix in sortable {
        let entry = &entries[entry_ix];
        vs_index += entry.vs.len();
        out.extend(entry.vs.iter().cloned());
        if let (Some(bc), Some(w)) = (entry.barycenter, entry.weight) {
            sum += bc * w;
            weight += w;
        }
        vs_index = consume_unsortable(&mut out, entries, &mut unsortable, vs_index);
    }

    if weight != 0.0 {
        SortResult {
            vs: out,
            barycenter: Some(sum / weight),
            weight: Some(weight),
        }
    } else {
        SortResult {
            vs: out,
            barycenter: None,
            weight: None,
        }
    }
}

pub(crate) fn sort_ix(entries: &[SortEntryIx], bias_right: bool) -> SortResultIx {
    let mut total_len: usize = 0;
    let mut sortable: Vec<usize> = Vec::new();
    let mut unsortable: Vec<usize> = Vec::new();

    for (ix, entry) in entries.iter().enumerate() {
        total_len += entry.vs.len();
        if entry.barycenter.is_some() {
            sortable.push(ix);
        } else {
            unsortable.push(ix);
        }
    }

    unsortable.sort_by(|&a, &b| entries[b].i.cmp(&entries[a].i));

    sortable.sort_by(|&a, &b| {
        let a_entry = &entries[a];
        let b_entry = &entries[b];
        let a_bc = a_entry.barycenter.unwrap_or(0.0);
        let b_bc = b_entry.barycenter.unwrap_or(0.0);
        if a_bc < b_bc {
            std::cmp::Ordering::Less
        } else if a_bc > b_bc {
            std::cmp::Ordering::Greater
        } else if !bias_right {
            a_entry.i.cmp(&b_entry.i)
        } else {
            b_entry.i.cmp(&a_entry.i)
        }
    });

    let mut out: Vec<usize> = Vec::with_capacity(total_len);
    let mut sum: f64 = 0.0;
    let mut weight: f64 = 0.0;
    let mut vs_index: usize = 0;

    fn consume_unsortable(
        out: &mut Vec<usize>,
        entries: &[SortEntryIx],
        unsortable: &mut Vec<usize>,
        mut index: usize,
    ) -> usize {
        while let Some(&last_ix) = unsortable.last() {
            let last = &entries[last_ix];
            if last.i > index {
                break;
            }
            let Some(last_ix) = unsortable.pop() else {
                break;
            };
            out.extend(entries[last_ix].vs.iter().copied());
            index += 1;
        }
        index
    }

    vs_index = consume_unsortable(&mut out, entries, &mut unsortable, vs_index);

    for entry_ix in sortable {
        let entry = &entries[entry_ix];
        vs_index += entry.vs.len();
        out.extend(entry.vs.iter().copied());
        if let (Some(bc), Some(w)) = (entry.barycenter, entry.weight) {
            sum += bc * w;
            weight += w;
        }
        vs_index = consume_unsortable(&mut out, entries, &mut unsortable, vs_index);
    }

    if weight != 0.0 {
        SortResultIx {
            vs: out,
            barycenter: Some(sum / weight),
            weight: Some(weight),
        }
    } else {
        SortResultIx {
            vs: out,
            barycenter: None,
            weight: None,
        }
    }
}

pub fn sort_subgraph<N, E, G, CN, CE, CG>(
    g: &Graph<N, E, G>,
    v: &str,
    cg: &Graph<CN, CE, CG>,
    bias_right: bool,
) -> SortResult
where
    N: Default + OrderNodeLabel + Clone + 'static,
    E: Default + OrderEdgeWeight + 'static,
    G: Default,
    CN: Default + 'static,
    CE: Default + 'static,
    CG: Default,
{
    struct SortSubgraphFrame {
        v: String,
        barycenters: Vec<BarycenterEntry>,
        border_left: Option<String>,
        border_right: Option<String>,
    }

    enum SortSubgraphStep {
        Enter(String),
        Exit(SortSubgraphFrame),
    }

    let root_id = v.to_string();
    let mut root_result: Option<SortResult> = None;
    let mut results: HashMap<String, SortResult> = HashMap::default();
    let mut stack = vec![SortSubgraphStep::Enter(root_id.clone())];

    while let Some(step) = stack.pop() {
        match step {
            SortSubgraphStep::Enter(v) => {
                let mut movable: Vec<String> =
                    g.children(&v).into_iter().map(|s| s.to_string()).collect();

                let (border_left, border_right) = g.node(&v).map_or((None, None), |node| {
                    (
                        node.border_left().map(|s| s.to_string()),
                        node.border_right().map(|s| s.to_string()),
                    )
                });

                if let (Some(bl), Some(br)) = (border_left.as_deref(), border_right.as_deref()) {
                    movable.retain(|w| w != bl && w != br);
                }

                let barycenters = barycenter(g, &movable);
                let child_ids = barycenters
                    .iter()
                    .filter(|entry| !g.children(&entry.v).is_empty())
                    .map(|entry| entry.v.clone())
                    .collect::<Vec<_>>();

                stack.push(SortSubgraphStep::Exit(SortSubgraphFrame {
                    v,
                    barycenters,
                    border_left,
                    border_right,
                }));
                for child in child_ids.into_iter().rev() {
                    stack.push(SortSubgraphStep::Enter(child));
                }
            }
            SortSubgraphStep::Exit(mut frame) => {
                let mut subgraphs: HashMap<String, SortResult> = HashMap::default();
                for entry in &mut frame.barycenters {
                    let Some(subgraph_result) = results.get(&entry.v).cloned() else {
                        continue;
                    };
                    if subgraph_result.barycenter.is_some() {
                        merge_barycenters(entry, &subgraph_result);
                    }
                    subgraphs.insert(entry.v.clone(), subgraph_result);
                }

                let mut entries = resolve_conflicts(&frame.barycenters, cg);
                expand_subgraphs(&mut entries, &subgraphs);
                let mut result = sort(&entries, bias_right);

                if let (Some(bl), Some(br)) =
                    (frame.border_left.as_deref(), frame.border_right.as_deref())
                {
                    let mut out: Vec<String> = Vec::with_capacity(result.vs.len() + 2);
                    out.push(bl.to_string());
                    out.extend(result.vs);
                    out.push(br.to_string());
                    result.vs = out;

                    if let (Some(bl_pred), Some(br_pred)) =
                        (g.first_predecessor(bl), g.first_predecessor(br))
                    {
                        let bl_order = g.node(bl_pred).and_then(|n| n.order()).unwrap_or(0) as f64;
                        let br_order = g.node(br_pred).and_then(|n| n.order()).unwrap_or(0) as f64;

                        let bc = result.barycenter.unwrap_or(0.0);
                        let w = result.weight.unwrap_or(0.0);
                        let denom = w + 2.0;
                        result.barycenter = Some((bc * w + bl_order + br_order) / denom);
                        result.weight = Some(denom);
                    }
                }

                if frame.v == root_id {
                    root_result = Some(result.clone());
                }
                results.insert(frame.v, result);
            }
        }
    }

    root_result.unwrap_or(SortResult {
        vs: Vec::new(),
        barycenter: None,
        weight: None,
    })
}

pub(crate) fn sort_subgraph_ix<N, E, G, CN, CE, CG>(
    g: &Graph<N, E, G>,
    v: &str,
    cg: &Graph<CN, CE, CG>,
    bias_right: bool,
) -> SortResultIx
where
    N: Default + OrderNodeLabel + Clone + 'static,
    E: Default + OrderEdgeWeight + 'static,
    G: Default,
    CN: Default + 'static,
    CE: Default + 'static,
    CG: Default,
{
    sort_subgraph_ix_iterative(g, v, cg, bias_right, None, false)
}

pub(crate) fn sort_subgraph_with_timings_ix<N, E, G, CN, CE, CG>(
    g: &Graph<N, E, G>,
    v: &str,
    cg: &Graph<CN, CE, CG>,
    bias_right: bool,
    timings: &mut SortSubgraphTimings,
) -> SortResultIx
where
    N: Default + OrderNodeLabel + Clone + 'static,
    E: Default + OrderEdgeWeight + 'static,
    G: Default,
    CN: Default + 'static,
    CE: Default + 'static,
    CG: Default,
{
    sort_subgraph_ix_iterative(g, v, cg, bias_right, Some(timings), true)
}

fn sort_subgraph_ix_iterative<N, E, G, CN, CE, CG>(
    g: &Graph<N, E, G>,
    v: &str,
    cg: &Graph<CN, CE, CG>,
    bias_right: bool,
    mut timings: Option<&mut SortSubgraphTimings>,
    sort_movable_by_order: bool,
) -> SortResultIx
where
    N: Default + OrderNodeLabel + Clone + 'static,
    E: Default + OrderEdgeWeight + 'static,
    G: Default,
    CN: Default + 'static,
    CE: Default + 'static,
    CG: Default,
{
    struct SortSubgraphFrame {
        v: String,
        barycenters: Vec<BarycenterEntryIx>,
        border_left: Option<String>,
        border_right: Option<String>,
        border_left_ix: Option<usize>,
        border_right_ix: Option<usize>,
    }

    enum SortSubgraphStep {
        Enter(String),
        Exit(SortSubgraphFrame),
    }

    let total_start = timings.is_some().then(Instant::now);
    let root_id = v.to_string();
    let mut root_result: Option<SortResultIx> = None;
    let mut results: HashMap<usize, SortResultIx> = HashMap::default();
    let mut stack = vec![SortSubgraphStep::Enter(root_id.clone())];

    while let Some(step) = stack.pop() {
        match step {
            SortSubgraphStep::Enter(v) => {
                let build_movable_start = timings.is_some().then(Instant::now);
                let mut movable: Vec<usize> =
                    g.children_iter(&v).filter_map(|s| g.node_ix(s)).collect();

                let (border_left, border_right) = g.node(&v).map_or((None, None), |node| {
                    (
                        node.border_left().map(str::to_string),
                        node.border_right().map(str::to_string),
                    )
                });

                let border_left_ix = border_left.as_deref().and_then(|s| g.node_ix(s));
                let border_right_ix = border_right.as_deref().and_then(|s| g.node_ix(s));

                if border_left_ix.is_some() && border_right_ix.is_some() {
                    movable.retain(|&w_ix| {
                        Some(w_ix) != border_left_ix && Some(w_ix) != border_right_ix
                    });
                }

                if sort_movable_by_order {
                    movable.sort_by_key(|&w_ix| {
                        let order = g
                            .node_label_by_ix(w_ix)
                            .and_then(|n| n.order())
                            .unwrap_or(usize::MAX);
                        (order, w_ix)
                    });
                }

                if let (Some(start), Some(t)) = (build_movable_start, timings.as_deref_mut()) {
                    t.build_movable += start.elapsed();
                }

                let barycenter_start = timings.is_some().then(Instant::now);
                let barycenters = barycenter_ix(g, &movable);
                if let (Some(start), Some(t)) = (barycenter_start, timings.as_deref_mut()) {
                    t.barycenter += start.elapsed();
                }

                let child_ids = barycenters
                    .iter()
                    .filter_map(|entry| {
                        let entry_id = g.node_id_by_ix(entry.v_ix)?;
                        g.children_iter(entry_id)
                            .next()
                            .is_some()
                            .then(|| entry_id.to_string())
                    })
                    .collect::<Vec<_>>();

                stack.push(SortSubgraphStep::Exit(SortSubgraphFrame {
                    v,
                    barycenters,
                    border_left,
                    border_right,
                    border_left_ix,
                    border_right_ix,
                }));
                for child in child_ids.into_iter().rev() {
                    stack.push(SortSubgraphStep::Enter(child));
                }
            }
            SortSubgraphStep::Exit(mut frame) => {
                let mut subgraphs: HashMap<usize, SortResultIx> = HashMap::default();
                for entry in &mut frame.barycenters {
                    let Some(subgraph_result) = results.get(&entry.v_ix).cloned() else {
                        continue;
                    };
                    if subgraph_result.barycenter.is_some() {
                        merge_barycenters_ix(entry, &subgraph_result);
                    }
                    subgraphs.insert(entry.v_ix, subgraph_result);
                }

                let resolve_start = timings.is_some().then(Instant::now);
                let mut entries = resolve_conflicts_ix(g, &frame.barycenters, cg);
                if let (Some(start), Some(t)) = (resolve_start, timings.as_deref_mut()) {
                    t.resolve_conflicts += start.elapsed();
                }

                let expand_start = timings.is_some().then(Instant::now);
                expand_subgraphs_ix(&mut entries, &subgraphs);
                if let (Some(start), Some(t)) = (expand_start, timings.as_deref_mut()) {
                    t.expand_subgraphs += start.elapsed();
                }

                let sort_start = timings.is_some().then(Instant::now);
                let mut result = sort_ix(&entries, bias_right);
                if let (Some(start), Some(t)) = (sort_start, timings.as_deref_mut()) {
                    t.sort += start.elapsed();
                }

                if let (Some(bl), Some(br)) =
                    (frame.border_left.as_deref(), frame.border_right.as_deref())
                {
                    if let (Some(bl_ix), Some(br_ix)) =
                        (frame.border_left_ix, frame.border_right_ix)
                    {
                        let border_start = timings.is_some().then(Instant::now);
                        let mut out: Vec<usize> = Vec::with_capacity(result.vs.len() + 2);
                        out.push(bl_ix);
                        out.extend(result.vs);
                        out.push(br_ix);
                        result.vs = out;

                        if let (Some(bl_pred), Some(br_pred)) =
                            (g.first_predecessor(bl), g.first_predecessor(br))
                        {
                            if let (Some(bl_pred_ix), Some(br_pred_ix)) =
                                (g.node_ix(bl_pred), g.node_ix(br_pred))
                            {
                                let bl_order =
                                    g.node_label_by_ix(bl_pred_ix)
                                        .and_then(|n| n.order())
                                        .unwrap_or(0) as f64;
                                let br_order =
                                    g.node_label_by_ix(br_pred_ix)
                                        .and_then(|n| n.order())
                                        .unwrap_or(0) as f64;

                                let bc = result.barycenter.unwrap_or(0.0);
                                let w = result.weight.unwrap_or(0.0);
                                let denom = w + 2.0;
                                result.barycenter = Some((bc * w + bl_order + br_order) / denom);
                                result.weight = Some(denom);
                            }
                        }
                        if let (Some(start), Some(t)) = (border_start, timings.as_deref_mut()) {
                            t.border_adjust += start.elapsed();
                        }
                    }
                }

                if frame.v == root_id {
                    root_result = Some(result.clone());
                }
                if let Some(v_ix) = g.node_ix(&frame.v) {
                    results.insert(v_ix, result);
                }
            }
        }
    }

    if let (Some(start), Some(t)) = (total_start, timings.as_deref_mut()) {
        t.total += start.elapsed();
    }

    root_result.unwrap_or(SortResultIx {
        vs: Vec::new(),
        barycenter: None,
        weight: None,
    })
}

fn expand_subgraphs(entries: &mut [SortEntry], subgraphs: &HashMap<String, SortResult>) {
    for entry in entries {
        let mut out: Vec<String> = Vec::new();
        for v in &entry.vs {
            if let Some(sg) = subgraphs.get(v) {
                out.extend(sg.vs.iter().cloned());
            } else {
                out.push(v.clone());
            }
        }
        entry.vs = out;
    }
}

fn expand_subgraphs_ix(entries: &mut [SortEntryIx], subgraphs: &HashMap<usize, SortResultIx>) {
    for entry in entries {
        let mut out: Vec<usize> = Vec::new();
        for &v_ix in &entry.vs {
            if let Some(sg) = subgraphs.get(&v_ix) {
                out.extend(sg.vs.iter().copied());
            } else {
                out.push(v_ix);
            }
        }
        entry.vs = out;
    }
}

fn merge_barycenters(target: &mut BarycenterEntry, other: &SortResult) {
    let Some(other_bc) = other.barycenter else {
        return;
    };
    let other_w = other.weight.unwrap_or(0.0);

    if let (Some(bc), Some(w)) = (target.barycenter, target.weight) {
        let denom = w + other_w;
        target.barycenter = Some((bc * w + other_bc * other_w) / denom);
        target.weight = Some(denom);
    } else {
        target.barycenter = Some(other_bc);
        target.weight = Some(other_w);
    }
}

fn merge_barycenters_ix(target: &mut BarycenterEntryIx, other: &SortResultIx) {
    let Some(other_bc) = other.barycenter else {
        return;
    };
    let other_w = other.weight.unwrap_or(0.0);

    if let (Some(bc), Some(w)) = (target.barycenter, target.weight) {
        let denom = w + other_w;
        target.barycenter = Some((bc * w + other_bc * other_w) / denom);
        target.weight = Some(denom);
    } else {
        target.barycenter = Some(other_bc);
        target.weight = Some(other_w);
    }
}

fn barycenter_ix<N, E, G>(g: &Graph<N, E, G>, movable_ix: &[usize]) -> Vec<BarycenterEntryIx>
where
    N: Default + OrderNodeLabel + 'static,
    E: Default + OrderEdgeWeight + 'static,
    G: Default,
{
    movable_ix
        .iter()
        .map(|&v_ix| {
            let mut saw_edge = false;
            let mut sum: f64 = 0.0;
            let mut weight: f64 = 0.0;
            g.for_each_in_edge_ix(v_ix, None, |u_ix, _w_ix, _ek, lbl| {
                saw_edge = true;
                let edge_weight = lbl.weight();
                let u_order = g
                    .node_label_by_ix(u_ix)
                    .and_then(|n| n.order())
                    .map(|n| n as f64)
                    .unwrap_or(0.0);
                sum += edge_weight * u_order;
                weight += edge_weight;
            });

            if !saw_edge {
                return BarycenterEntryIx {
                    v_ix,
                    barycenter: None,
                    weight: None,
                };
            }

            BarycenterEntryIx {
                v_ix,
                barycenter: Some(sum / weight),
                weight: Some(weight),
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct ConflictEntryIx {
    indegree: usize,
    ins: Vec<usize>,
    outs: Vec<usize>,
    vs: Vec<usize>,
    i: usize,
    barycenter: Option<f64>,
    weight: Option<f64>,
    merged: bool,
}

fn resolve_conflicts_ix<N, E, G, CN, CE, CG>(
    g: &Graph<N, E, G>,
    entries: &[BarycenterEntryIx],
    cg: &Graph<CN, CE, CG>,
) -> Vec<SortEntryIx>
where
    N: Default + 'static,
    E: Default + 'static,
    G: Default,
    CN: Default + 'static,
    CE: Default + 'static,
    CG: Default,
{
    let max_v_ix = entries.iter().map(|e| e.v_ix).max().unwrap_or(0);
    let mut node_ix_to_entry_ix: Vec<Option<usize>> = vec![None; max_v_ix.saturating_add(1)];
    let mut conflicts: Vec<ConflictEntryIx> = Vec::with_capacity(entries.len());
    for (ix, entry) in entries.iter().enumerate() {
        if let Some(slot) = node_ix_to_entry_ix.get_mut(entry.v_ix) {
            *slot = Some(ix);
        }
        conflicts.push(ConflictEntryIx {
            indegree: 0,
            ins: Vec::new(),
            outs: Vec::new(),
            vs: vec![ix],
            i: ix,
            barycenter: entry.barycenter,
            weight: entry.weight,
            merged: false,
        });
    }

    let mut cg_ix_to_entry_ix: Vec<Option<usize>> = Vec::new();
    cg.for_each_node_ix(|cg_ix, node_id, _| {
        let Some(g_node_ix) = g.node_ix(node_id) else {
            return;
        };
        let Some(Some(entry_ix)) = node_ix_to_entry_ix.get(g_node_ix).copied() else {
            return;
        };
        if cg_ix_to_entry_ix.len() <= cg_ix {
            cg_ix_to_entry_ix.resize(cg_ix + 1, None);
        }
        cg_ix_to_entry_ix[cg_ix] = Some(entry_ix);
    });

    cg.for_each_edge_ix(|v_cg_ix, w_cg_ix, _ek, _lbl| {
        let Some(&Some(v_ix)) = cg_ix_to_entry_ix.get(v_cg_ix) else {
            return;
        };
        let Some(&Some(w_ix)) = cg_ix_to_entry_ix.get(w_cg_ix) else {
            return;
        };

        conflicts[w_ix].indegree += 1;
        conflicts[v_ix].outs.push(w_ix);
    });

    let mut source_set: Vec<usize> = Vec::new();
    for (ix, conflict) in conflicts.iter().enumerate() {
        if conflict.indegree == 0 {
            source_set.push(ix);
        }
    }

    let mut processed: Vec<usize> = Vec::new();
    while let Some(v_ix) = source_set.pop() {
        processed.push(v_ix);

        let ins = std::mem::take(&mut conflicts[v_ix].ins);

        for u in ins.into_iter().rev() {
            if conflicts[u].merged {
                continue;
            }
            let u_bary = conflicts[u].barycenter;
            let v_bary = conflicts[v_ix].barycenter;
            let should_merge = match (u_bary, v_bary) {
                (None, _) => true,
                (_, None) => true,
                (Some(ub), Some(vb)) => ub >= vb,
            };
            if should_merge {
                merge_conflict_entries_ix(&mut conflicts, v_ix, u);
            }
        }

        let outs = std::mem::take(&mut conflicts[v_ix].outs);
        for w_ix in outs {
            conflicts[w_ix].ins.push(v_ix);
            conflicts[w_ix].indegree = conflicts[w_ix].indegree.saturating_sub(1);
            if conflicts[w_ix].indegree == 0 {
                source_set.push(w_ix);
            }
        }
    }

    let mut out: Vec<SortEntryIx> = Vec::new();
    for id in processed {
        let entry = &conflicts[id];
        if entry.merged {
            continue;
        }
        let mut vs: Vec<usize> = Vec::with_capacity(entry.vs.len());
        for &ix in &entry.vs {
            vs.push(entries[ix].v_ix);
        }
        out.push(SortEntryIx {
            vs,
            i: entry.i,
            barycenter: entry.barycenter,
            weight: entry.weight,
        });
    }
    out
}

fn merge_conflict_entries_ix(mapped: &mut [ConflictEntryIx], target: usize, source: usize) {
    if target == source {
        return;
    }

    let (t, s) = if target < source {
        let (left, right) = mapped.split_at_mut(source);
        (&mut left[target], &mut right[0])
    } else {
        let (left, right) = mapped.split_at_mut(target);
        (&mut right[0], &mut left[source])
    };

    let target_bary = t.barycenter;
    let target_weight = t.weight;
    let source_bary = s.barycenter;
    let source_weight = s.weight;
    let source_i = s.i;

    let mut sum: f64 = 0.0;
    let mut weight: f64 = 0.0;
    if let (Some(b), Some(w)) = (target_bary, target_weight) {
        if w != 0.0 {
            sum += b * w;
            weight += w;
        }
    }
    if let (Some(b), Some(w)) = (source_bary, source_weight) {
        if w != 0.0 {
            sum += b * w;
            weight += w;
        }
    }

    let mut merged_vs = std::mem::take(&mut s.vs);
    merged_vs.extend(std::mem::take(&mut t.vs));
    t.vs = merged_vs;

    if weight != 0.0 {
        t.barycenter = Some(sum / weight);
        t.weight = Some(weight);
    }
    t.i = t.i.min(source_i);

    s.merged = true;
}

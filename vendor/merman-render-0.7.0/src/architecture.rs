use crate::architecture_metrics::{
    architecture_cytoscape_child_contribution_bounds, architecture_cytoscape_child_label_bounds,
    architecture_measure_cytoscape_node_bbox_extras, architecture_node_bbox_extras_to_manatee,
};
use crate::config::config_f64;
use crate::json::from_value_ref;
use crate::model::{
    ArchitectureCompoundBounds, ArchitectureCytoscapeServiceBounds,
    ArchitectureCytoscapeServiceLabelMetrics, ArchitectureDiagramLayout,
    ArchitectureFcoseDebugNodeBounds, ArchitectureFcoseDebugStage, ArchitectureFcoseRelocateDebug,
    Bounds, LayoutEdge, LayoutNode, LayoutPoint,
};
use crate::text::{TextMeasurer, TextStyle};
use crate::{Error, Result};
use indexmap::IndexMap;
use merman_core::diagrams::architecture::ArchitectureDiagramRenderModel;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::Deserialize;
use serde_json::Value;

fn architecture_relative_placement_constraints<'a>(
    spatial_maps: &[IndexMap<&'a str, (i32, i32)>],
    node_index_by_id: &FxHashMap<&'a str, usize>,
    gap: f64,
) -> Vec<manatee::algo::fcose::IndexedRelativePlacementConstraint> {
    let mut relative: Vec<manatee::algo::fcose::IndexedRelativePlacementConstraint> = Vec::new();

    for spatial_map in spatial_maps {
        let mut inv: FxHashMap<(i32, i32), &str> = FxHashMap::default();
        inv.reserve(spatial_map.len().saturating_mul(2));
        for (id, (x, y)) in spatial_map.iter() {
            inv.insert((*x, *y), *id);
        }

        let mut pos_queue: std::collections::VecDeque<(i32, i32)> =
            std::collections::VecDeque::new();
        let mut visited_pos: FxHashSet<(i32, i32)> = FxHashSet::default();
        visited_pos.reserve(spatial_map.len().saturating_mul(2));
        pos_queue.push_back((0, 0));

        // Preserve Mermaid's direction iteration order: L, R, T, B.
        const DIRS: [(char, (i32, i32)); 4] =
            [('L', (-1, 0)), ('R', (1, 0)), ('T', (0, 1)), ('B', (0, -1))];

        while let Some(curr) = pos_queue.pop_front() {
            // Mermaid marks the current grid position as visited but does not skip duplicate
            // queued positions on pop. That preserves duplicate relative constraints when a
            // node is reached through two paths before its neighbors are visited.
            visited_pos.insert(curr);
            let Some(&curr_id) = inv.get(&curr) else {
                continue;
            };
            for (dir, (sx, sy)) in DIRS {
                let new_pos = (curr.0 + sx, curr.1 + sy);
                let Some(&new_id) = inv.get(&new_pos) else {
                    continue;
                };
                if visited_pos.contains(&new_pos) {
                    continue;
                }
                pos_queue.push_back(new_pos);
                let Some(&curr_idx) = node_index_by_id.get(curr_id) else {
                    continue;
                };
                let Some(&new_idx) = node_index_by_id.get(new_id) else {
                    continue;
                };

                // `ArchitectureDirectionName[dir] = newId`
                // `ArchitectureDirectionName[getOppositeArchitectureDirection(dir)] = currId`
                let c = match dir {
                    'L' => manatee::algo::fcose::IndexedRelativePlacementConstraint {
                        left: Some(new_idx),
                        right: Some(curr_idx),
                        top: None,
                        bottom: None,
                        gap,
                    },
                    'R' => manatee::algo::fcose::IndexedRelativePlacementConstraint {
                        left: Some(curr_idx),
                        right: Some(new_idx),
                        top: None,
                        bottom: None,
                        gap,
                    },
                    'T' => manatee::algo::fcose::IndexedRelativePlacementConstraint {
                        left: None,
                        right: None,
                        top: Some(new_idx),
                        bottom: Some(curr_idx),
                        gap,
                    },
                    'B' => manatee::algo::fcose::IndexedRelativePlacementConstraint {
                        left: None,
                        right: None,
                        top: Some(curr_idx),
                        bottom: Some(new_idx),
                        gap,
                    },
                    _ => continue,
                };
                relative.push(c);
            }
        }
    }

    relative
}

fn config_bool(cfg: &Value, path: &[&str]) -> Option<bool> {
    let mut cur = cfg;
    for k in path {
        cur = cur.get(*k)?;
    }
    cur.as_bool()
}

#[derive(Debug, Clone, Deserialize)]
struct ArchitectureNodeModel {
    id: String,
    #[serde(rename = "type")]
    node_type: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default, rename = "in")]
    in_group: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ArchitectureEdgeModel {
    #[serde(rename = "lhsId", alias = "lhs")]
    lhs_id: String,
    #[serde(rename = "rhsId", alias = "rhs")]
    rhs_id: String,
    #[serde(default, rename = "lhsDir")]
    lhs_dir: Option<String>,
    #[serde(default, rename = "rhsDir")]
    rhs_dir: Option<String>,
    #[serde(default)]
    title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ArchitectureGroupModel {
    id: String,
    #[serde(default, rename = "in")]
    in_group: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ArchitectureModel {
    #[serde(default)]
    nodes: Vec<ArchitectureNodeModel>,
    #[serde(default)]
    groups: Vec<ArchitectureGroupModel>,
    #[serde(default)]
    edges: Vec<ArchitectureEdgeModel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArchitectureNodeType {
    Service,
    Junction,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    L,
    R,
    T,
    B,
}

impl Dir {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            'L' => Some(Self::L),
            'R' => Some(Self::R),
            'T' => Some(Self::T),
            'B' => Some(Self::B),
            _ => None,
        }
    }

    fn is_x(self) -> bool {
        matches!(self, Self::L | Self::R)
    }
}

fn dir_pair_key(source: Dir, target: Dir) -> Option<&'static str> {
    match (source, target) {
        (Dir::L, Dir::R) => Some("LR"),
        (Dir::L, Dir::T) => Some("LT"),
        (Dir::L, Dir::B) => Some("LB"),
        (Dir::R, Dir::L) => Some("RL"),
        (Dir::R, Dir::T) => Some("RT"),
        (Dir::R, Dir::B) => Some("RB"),
        (Dir::T, Dir::L) => Some("TL"),
        (Dir::T, Dir::R) => Some("TR"),
        (Dir::T, Dir::B) => Some("TB"),
        (Dir::B, Dir::L) => Some("BL"),
        (Dir::B, Dir::R) => Some("BR"),
        (Dir::B, Dir::T) => Some("BT"),
        _ => None,
    }
}

fn shift_position_by_arch_pair(x: i32, y: i32, pair: &str) -> (i32, i32) {
    // Port of Mermaid@11.12.2 `shiftPositionByArchitectureDirectionPair`.
    let bytes = pair.as_bytes();
    if bytes.len() != 2 {
        return (x, y);
    }
    let lhs = match bytes[0] as char {
        'L' => Dir::L,
        'R' => Dir::R,
        'T' => Dir::T,
        'B' => Dir::B,
        _ => return (x, y),
    };
    let rhs = match bytes[1] as char {
        'L' => Dir::L,
        'R' => Dir::R,
        'T' => Dir::T,
        'B' => Dir::B,
        _ => return (x, y),
    };

    if lhs.is_x() {
        if !rhs.is_x() {
            (
                x + if lhs == Dir::L { -1 } else { 1 },
                y + if rhs == Dir::T { 1 } else { -1 },
            )
        } else {
            (x + if lhs == Dir::L { -1 } else { 1 }, y)
        }
    } else if rhs.is_x() {
        (
            x + if rhs == Dir::L { 1 } else { -1 },
            y + if lhs == Dir::T { 1 } else { -1 },
        )
    } else {
        (x, y + if lhs == Dir::T { 1 } else { -1 })
    }
}

fn anchor_from_dir(dir: Dir) -> manatee::Anchor {
    match dir {
        Dir::L => manatee::Anchor::Left,
        Dir::R => manatee::Anchor::Right,
        Dir::T => manatee::Anchor::Top,
        Dir::B => manatee::Anchor::Bottom,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GroupAlignment {
    Horizontal,
    Vertical,
    Bend,
}

fn dir_alignment(a: Option<char>, b: Option<char>) -> GroupAlignment {
    let (Some(a), Some(b)) = (a.and_then(Dir::from_char), b.and_then(Dir::from_char)) else {
        return GroupAlignment::Bend;
    };
    if a.is_x() != b.is_x() {
        GroupAlignment::Bend
    } else if a.is_x() {
        GroupAlignment::Horizontal
    } else {
        GroupAlignment::Vertical
    }
}

fn flatten_alignments(
    alignment_obj: &IndexMap<i32, IndexMap<String, Vec<usize>>>,
    alignment_dir: GroupAlignment,
    group_alignments: &std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, GroupAlignment>,
    >,
) -> Vec<Vec<usize>> {
    // Mirror Mermaid's `flattenAlignments(...)` + `Object.values(...)` ordering.
    //
    // Mermaid uses plain JS objects keyed by row/col number. Enumeration order puts
    // non-negative integer keys first (ascending), then other string keys (insertion
    // order). We reproduce that here to keep the first element of each alignment group
    // stable, since `cose-base` uses it to seed dummy-node positions.
    fn js_object_dir_order(obj: &IndexMap<i32, IndexMap<String, Vec<usize>>>) -> Vec<i32> {
        let mut non_neg: Vec<i32> = Vec::new();
        let mut other: Vec<i32> = Vec::new();
        for &k in obj.keys() {
            if k >= 0 {
                non_neg.push(k);
            } else {
                other.push(k);
            }
        }
        non_neg.sort_unstable();
        non_neg.extend(other);
        non_neg
    }

    fn is_js_array_index_key(k: &str) -> Option<u32> {
        if k.is_empty() {
            return None;
        }
        if k.as_bytes().iter().all(|b| b.is_ascii_digit()) {
            return k.parse::<u32>().ok();
        }
        None
    }

    let mut prev: IndexMap<String, Vec<usize>> = IndexMap::new();

    for dir in js_object_dir_order(alignment_obj) {
        let Some(alignments) = alignment_obj.get(&dir) else {
            continue;
        };
        let mut cnt = 0usize;
        let mut arr: Vec<(String, Vec<usize>)> = alignments
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        if arr.len() == 1 {
            if let Some((_, node_ids)) = arr.pop() {
                prev.insert(dir.to_string(), node_ids);
            }
            continue;
        }
        for i in 0..arr.len().saturating_sub(1) {
            for j in (i + 1)..arr.len() {
                let (a_group_id, a_node_ids) = &arr[i];
                let (b_group_id, b_node_ids) = &arr[j];
                let alignment = group_alignments
                    .get(a_group_id)
                    .and_then(|m| m.get(b_group_id))
                    .copied();

                if alignment == Some(alignment_dir)
                    || a_group_id == "default"
                    || b_group_id == "default"
                {
                    prev.entry(dir.to_string())
                        .or_default()
                        .extend(a_node_ids.iter().cloned());
                    prev.entry(dir.to_string())
                        .or_default()
                        .extend(b_node_ids.iter().cloned());
                } else {
                    let key_a = format!("{dir}-{cnt}");
                    cnt += 1;
                    prev.insert(key_a, a_node_ids.clone());
                    let key_b = format!("{dir}-{cnt}");
                    cnt += 1;
                    prev.insert(key_b, b_node_ids.clone());
                }
            }
        }
    }

    // `Object.values(prev)` ordering.
    let mut numeric_keys: Vec<(u32, String)> = Vec::new();
    let mut other_keys: Vec<String> = Vec::new();
    for k in prev.keys() {
        if let Some(ix) = is_js_array_index_key(k.as_str()) {
            numeric_keys.push((ix, k.clone()));
        } else {
            other_keys.push(k.clone());
        }
    }
    numeric_keys.sort_by_key(|(ix, _)| *ix);

    let mut out: Vec<Vec<usize>> = Vec::new();
    for (_, k) in numeric_keys {
        if let Some(v) = prev.get(&k) {
            out.push(v.clone());
        }
    }
    for k in other_keys {
        if let Some(v) = prev.get(&k) {
            out.push(v.clone());
        }
    }
    out
}

#[derive(Debug, Clone, Copy)]
struct ArchitectureNodeView<'a> {
    id: &'a str,
    node_type: ArchitectureNodeType,
    title: Option<&'a str>,
    in_group: Option<&'a str>,
}

#[derive(Debug, Clone, Copy)]
struct ArchitectureGroupView<'a> {
    id: &'a str,
    in_group: Option<&'a str>,
}

#[derive(Debug, Clone, Copy)]
struct ArchitectureEdgeView<'a> {
    lhs_id: &'a str,
    rhs_id: &'a str,
    lhs_dir: Option<char>,
    rhs_dir: Option<char>,
    title: Option<&'a str>,
}

#[derive(Debug, Clone)]
struct ArchitectureModelView<'a> {
    nodes: Vec<ArchitectureNodeView<'a>>,
    groups: Vec<ArchitectureGroupView<'a>>,
    edges: Vec<ArchitectureEdgeView<'a>>,
}

impl<'a> ArchitectureModelView<'a> {
    fn from_json(model: &'a ArchitectureModel) -> Self {
        let nodes = model
            .nodes
            .iter()
            .map(|n| ArchitectureNodeView {
                id: n.id.as_str(),
                node_type: match n.node_type.as_str() {
                    "service" => ArchitectureNodeType::Service,
                    "junction" => ArchitectureNodeType::Junction,
                    _ => ArchitectureNodeType::Other,
                },
                title: n.title.as_deref(),
                in_group: n.in_group.as_deref(),
            })
            .collect();

        let groups = model
            .groups
            .iter()
            .map(|g| ArchitectureGroupView {
                id: g.id.as_str(),
                in_group: g.in_group.as_deref(),
            })
            .collect();

        let edges = model
            .edges
            .iter()
            .map(|e| ArchitectureEdgeView {
                lhs_id: e.lhs_id.as_str(),
                rhs_id: e.rhs_id.as_str(),
                lhs_dir: e.lhs_dir.as_deref().and_then(|s| s.chars().next()),
                rhs_dir: e.rhs_dir.as_deref().and_then(|s| s.chars().next()),
                title: e.title.as_deref(),
            })
            .collect();

        Self {
            nodes,
            groups,
            edges,
        }
    }

    fn from_typed(model: &'a ArchitectureDiagramRenderModel) -> Self {
        let nodes = model
            .nodes
            .iter()
            .map(|n| ArchitectureNodeView {
                id: n.id.as_str(),
                node_type: match n.node_type {
                    merman_core::diagrams::architecture::ArchitectureRenderNodeType::Service => {
                        ArchitectureNodeType::Service
                    }
                    merman_core::diagrams::architecture::ArchitectureRenderNodeType::Junction => {
                        ArchitectureNodeType::Junction
                    }
                },
                title: n.title.as_deref(),
                in_group: n.in_group.as_deref(),
            })
            .collect();

        let groups = model
            .groups
            .iter()
            .map(|g| ArchitectureGroupView {
                id: g.id.as_str(),
                in_group: g.in_group.as_deref(),
            })
            .collect();

        let edges = model
            .edges
            .iter()
            .map(|e| ArchitectureEdgeView {
                lhs_id: e.lhs_id.as_str(),
                rhs_id: e.rhs_id.as_str(),
                lhs_dir: Some(e.lhs_dir),
                rhs_dir: Some(e.rhs_dir),
                title: e.title.as_deref(),
            })
            .collect();

        Self {
            nodes,
            groups,
            edges,
        }
    }
}

struct ArchitectureFcoseNodeBoundsExtrasInput<'m, 'a> {
    model: &'m ArchitectureModelView<'a>,
    text_measurer: &'m dyn TextMeasurer,
    icon_size: f64,
    font_size_px: f64,
    font_family: &'m str,
}

fn architecture_cytoscape_text_style(font_size_px: f64, font_family: &str) -> TextStyle {
    TextStyle {
        font_family: Some(font_family.to_string()),
        font_size: font_size_px,
        font_weight: None,
    }
}

fn architecture_cytoscape_edge_text_style() -> TextStyle {
    TextStyle {
        // Mermaid's Architecture Cytoscape stylesheet sets `font-size` only on `node[label]`;
        // `edge[label]` keeps Cytoscape's default 16px sans-serif label style.
        font_family: Some("sans-serif".to_string()),
        ..TextStyle::default()
    }
}

fn architecture_fcose_node_bounds_extras<'a>(
    input: ArchitectureFcoseNodeBoundsExtrasInput<'_, 'a>,
) -> FxHashMap<&'a str, manatee::BoundsExtras> {
    // Capture per-node service label extents for the FCoSE port. These extras do not change
    // layout node size, but they let manatee approximate Cytoscape's
    // `compound-sizing-wrt-labels: include` behavior when computing compound and element bboxes.
    //
    // Relocation-centering stays inside manatee's indexed graph adapter; keeping it out of this
    // renderer-side helper avoids a second, unused pre-layout bbox model.
    let ArchitectureFcoseNodeBoundsExtrasInput {
        model,
        text_measurer,
        icon_size,
        font_size_px,
        font_family,
    } = input;
    let text_style = architecture_cytoscape_text_style(font_size_px, font_family);

    let mut node_title: FxHashMap<&str, &str> = FxHashMap::default();
    node_title.reserve(model.nodes.len().saturating_mul(2));

    for n in &model.nodes {
        if let Some(t) = n.title {
            node_title.insert(n.id, t);
        }
    }

    let mut node_bounds_extras: FxHashMap<&str, manatee::BoundsExtras> = FxHashMap::default();
    node_bounds_extras.reserve(model.nodes.len().saturating_mul(2));
    for n in &model.nodes {
        let title = node_title.get(n.id).copied();
        let bounds_extras = architecture_measure_cytoscape_node_bbox_extras(
            title,
            text_measurer,
            &text_style,
            icon_size,
            font_size_px,
        );
        node_bounds_extras.insert(
            n.id,
            architecture_node_bbox_extras_to_manatee(bounds_extras),
        );
    }

    node_bounds_extras
}

#[derive(Debug, Clone)]
struct ArchitectureFcoseInputPlan<'a> {
    node_ids: Vec<&'a str>,
    compound_ids: Vec<&'a str>,
    node_index_by_id: FxHashMap<&'a str, usize>,
    spatial_maps: Vec<IndexMap<&'a str, (i32, i32)>>,
    graph: manatee::algo::fcose::IndexedGraph,
    options: manatee::algo::fcose::IndexedFcoseOptions,
    default_edge_length: f64,
    compound_padding_px: f64,
}

struct ArchitectureFcoseInputPlanInput<'m, 'a> {
    model: &'m ArchitectureModelView<'a>,
    layout_nodes: &'m [LayoutNode],
    node_bounds_extras: &'m FxHashMap<&'a str, manatee::BoundsExtras>,
    text_measurer: &'m dyn TextMeasurer,
    icon_size: f64,
    padding_px: f64,
    ideal_edge_length_multiplier: f64,
    same_group_edge_elasticity: f64,
    fcose_randomize: bool,
    fcose_node_separation: f64,
    fcose_num_iter: usize,
    fcose_seed: u64,
}

fn build_architecture_fcose_input_plan<'a>(
    input: ArchitectureFcoseInputPlanInput<'_, 'a>,
) -> Result<ArchitectureFcoseInputPlan<'a>> {
    let ArchitectureFcoseInputPlanInput {
        model,
        layout_nodes,
        node_bounds_extras,
        text_measurer,
        icon_size,
        padding_px,
        ideal_edge_length_multiplier,
        same_group_edge_elasticity,
        fcose_randomize,
        fcose_node_separation,
        fcose_num_iter,
        fcose_seed,
    } = input;

    if layout_nodes.len() != model.nodes.len() {
        return Err(Error::InvalidModel {
            message: format!(
                "architecture FCoSE input node count mismatch: model={} layout={}",
                model.nodes.len(),
                layout_nodes.len()
            ),
        });
    }

    let node_ids: Vec<&str> = model.nodes.iter().map(|n| n.id).collect();
    let compound_ids: Vec<&str> = model.groups.iter().map(|g| g.id).collect();

    for (idx, (model_node, layout_node)) in model.nodes.iter().zip(layout_nodes).enumerate() {
        if layout_node.id != model_node.id {
            return Err(Error::InvalidModel {
                message: format!(
                    "architecture FCoSE input node order mismatch at {idx}: model={} layout={}",
                    model_node.id, layout_node.id
                ),
            });
        }
    }

    let mut incident_edges: FxHashMap<&str, Vec<usize>> = FxHashMap::default();
    incident_edges.reserve(model.nodes.len().saturating_mul(2));
    for (edge_idx, e) in model.edges.iter().enumerate() {
        incident_edges.entry(e.lhs_id).or_default().push(edge_idx);
        incident_edges.entry(e.rhs_id).or_default().push(edge_idx);
    }

    let mut adj_list: FxHashMap<&str, IndexMap<&'static str, &str>> = FxHashMap::default();
    adj_list.reserve(model.nodes.len().saturating_mul(2));
    for &id in &node_ids {
        let mut adj: IndexMap<&'static str, &str> = IndexMap::new();
        let Some(edges) = incident_edges.get(id) else {
            adj_list.insert(id, adj);
            continue;
        };
        for &edge_idx in edges {
            let e = &model.edges[edge_idx];
            let (rhs_id, lhs_dir, rhs_dir) = if e.lhs_id == id {
                (e.rhs_id, e.lhs_dir, e.rhs_dir)
            } else {
                (e.lhs_id, e.rhs_dir, e.lhs_dir)
            };
            let (Some(lhs_dir), Some(rhs_dir)) = (
                lhs_dir.and_then(Dir::from_char),
                rhs_dir.and_then(Dir::from_char),
            ) else {
                continue;
            };
            let Some(pair) = dir_pair_key(lhs_dir, rhs_dir) else {
                continue;
            };
            if let Some(existing) = adj.get_mut(pair) {
                *existing = rhs_id;
            } else {
                adj.insert(pair, rhs_id);
            }
        }
        adj_list.insert(id, adj);
    }

    // Deterministic component discovery: mimic Mermaid's `Object.keys(notVisited)[0]` by walking
    // node order and taking the first not-yet-visited id for each component.
    let mut spatial_maps: Vec<IndexMap<&str, (i32, i32)>> = Vec::new();
    let mut visited: FxHashSet<&str> = FxHashSet::default();
    visited.reserve(model.nodes.len().saturating_mul(2));
    for &start_id in &node_ids {
        if visited.contains(start_id) {
            continue;
        }

        let mut spatial: IndexMap<&str, (i32, i32)> = IndexMap::new();
        spatial.insert(start_id, (0, 0));

        let mut queue: std::collections::VecDeque<&str> = std::collections::VecDeque::new();
        queue.push_back(start_id);

        while let Some(id) = queue.pop_front() {
            if !visited.insert(id) {
                continue;
            }
            let Some(&(pos_x, pos_y)) = spatial.get(id) else {
                continue;
            };
            let Some(adj) = adj_list.get(id) else {
                continue;
            };
            for (&pair, &rhs_id) in adj.iter() {
                if visited.contains(rhs_id) {
                    continue;
                }
                let (nx, ny) = shift_position_by_arch_pair(pos_x, pos_y, pair);
                // Mermaid updates `spatialMap[rhsId]` even if the node is already enqueued,
                // because `visited[rhsId]` is only set when the node is dequeued.
                spatial.insert(rhs_id, (nx, ny));
                queue.push_back(rhs_id);
            }
        }

        spatial_maps.push(spatial);
    }

    let mut node_group: std::collections::BTreeMap<&str, Option<&str>> =
        std::collections::BTreeMap::new();
    for n in &model.nodes {
        node_group.insert(n.id, n.in_group);
    }

    let mut node_index_by_id: FxHashMap<&str, usize> = FxHashMap::default();
    node_index_by_id.reserve(model.nodes.len().saturating_mul(2));
    for (idx, &id) in node_ids.iter().enumerate() {
        node_index_by_id.insert(id, idx);
    }

    let mut compound_index_by_id: FxHashMap<&str, usize> = FxHashMap::default();
    compound_index_by_id.reserve(model.groups.len().saturating_mul(2));
    for (idx, g) in model.groups.iter().enumerate() {
        compound_index_by_id.insert(g.id, idx);
    }

    // Track how groups connect (used when flattening alignment arrays across groups).
    //
    // Mermaid builds this while reducing `this.nodes` and each node's `service.edges` list in
    // `ArchitectureDB.getDataStructures()`. The same edge can therefore update the map once
    // per endpoint, and later endpoint traversal overwrites earlier alignment values. Do not
    // collapse this to a single global edge pass: fixtures with mixed core/data edges rely on
    // the source traversal order to decide which group alignment survives.
    let mut group_alignments: std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, GroupAlignment>,
    > = std::collections::BTreeMap::new();
    for &id in &node_ids {
        let Some(edge_indices) = incident_edges.get(id) else {
            continue;
        };
        for &edge_idx in edge_indices {
            let e = &model.edges[edge_idx];
            let Some(lhs_group) = node_group.get(e.lhs_id).and_then(|v| *v) else {
                continue;
            };
            let Some(rhs_group) = node_group.get(e.rhs_id).and_then(|v| *v) else {
                continue;
            };
            if lhs_group == rhs_group {
                continue;
            }
            let alignment = dir_alignment(e.lhs_dir, e.rhs_dir);
            if alignment == GroupAlignment::Bend {
                continue;
            }
            group_alignments
                .entry(lhs_group.to_string())
                .or_default()
                .insert(rhs_group.to_string(), alignment);
            group_alignments
                .entry(rhs_group.to_string())
                .or_default()
                .insert(lhs_group.to_string(), alignment);
        }
    }

    let mut horizontal_all: Vec<Vec<usize>> = Vec::new();
    let mut vertical_all: Vec<Vec<usize>> = Vec::new();
    for spatial_map in &spatial_maps {
        let mut horizontal_alignments: IndexMap<i32, IndexMap<String, Vec<usize>>> =
            IndexMap::new();
        let mut vertical_alignments: IndexMap<i32, IndexMap<String, Vec<usize>>> = IndexMap::new();

        for (id, (x, y)) in spatial_map {
            let id = *id;
            let Some(&node_idx) = node_index_by_id.get(id) else {
                continue;
            };
            let node_group = node_group
                .get(id)
                .and_then(|v| *v)
                .unwrap_or("default")
                .to_string();

            horizontal_alignments
                .entry(*y)
                .or_default()
                .entry(node_group.clone())
                .or_default()
                .push(node_idx);

            vertical_alignments
                .entry(*x)
                .or_default()
                .entry(node_group)
                .or_default()
                .push(node_idx);
        }

        let horiz_map = flatten_alignments(
            &horizontal_alignments,
            GroupAlignment::Horizontal,
            &group_alignments,
        );
        let vert_map = flatten_alignments(
            &vertical_alignments,
            GroupAlignment::Vertical,
            &group_alignments,
        );

        for v in &horiz_map {
            if v.len() > 1 {
                horizontal_all.push(v.clone());
            }
        }
        for v in &vert_map {
            if v.len() > 1 {
                vertical_all.push(v.clone());
            }
        }
    }

    // RelativePlacementConstraint (gap between borders).
    //
    // Upstream Mermaid derives these by BFS over immediate grid neighbors, starting from the
    // spatial origin `(0, 0)`. We mirror that behavior so constraints match Cytoscape's FCoSE
    // input even when the underlying spatial map discovery is approximate.
    let gap = ideal_edge_length_multiplier * icon_size;
    let relative =
        architecture_relative_placement_constraints(&spatial_maps, &node_index_by_id, gap);

    let mut edges: Vec<manatee::algo::fcose::IndexedEdge> = Vec::new();
    let mut default_edge_length_sum = 0.0f64;
    let mut default_edge_length_cnt = 0.0f64;
    let edge_text_style = architecture_cytoscape_edge_text_style();

    // Cytoscape FCoSE de-duplicates multiple edges between the same two nodes when building
    // its internal layout graph:
    //
    // `sourceNode.getEdgesBetween(targetNode).length == 0`
    //
    // This means bidirectional/multi edges still render in the final SVG, but only the first
    // edge between each undirected node pair contributes forces to the layout.
    //
    // Without this, our spring forces can cancel in small symmetric graphs, which makes the
    // final spacing (and thus the root `viewBox/max-width`) diverge from Mermaid baselines.
    let mut seen_undirected_layout_edges: FxHashSet<(usize, usize)> = FxHashSet::default();

    for e in &model.edges {
        let Some(&a_idx) = node_index_by_id.get(e.lhs_id) else {
            return Err(Error::InvalidModel {
                message: format!("edge lhs node not found: {}", e.lhs_id),
            });
        };
        let Some(&b_idx) = node_index_by_id.get(e.rhs_id) else {
            return Err(Error::InvalidModel {
                message: format!("edge rhs node not found: {}", e.rhs_id),
            });
        };
        let (k1, k2) = if a_idx <= b_idx {
            (a_idx, b_idx)
        } else {
            (b_idx, a_idx)
        };
        if !seen_undirected_layout_edges.insert((k1, k2)) {
            continue;
        }

        let lhs_g = node_group.get(e.lhs_id).and_then(|v| *v);
        let rhs_g = node_group.get(e.rhs_id).and_then(|v| *v);
        let same_parent = lhs_g == rhs_g;

        let base_ideal_length = if same_parent {
            ideal_edge_length_multiplier * icon_size
        } else {
            0.5 * icon_size
        };
        default_edge_length_sum += base_ideal_length;
        default_edge_length_cnt += 1.0;

        let elasticity = if same_parent {
            same_group_edge_elasticity
        } else {
            0.001
        };

        let source_anchor = e.lhs_dir.and_then(Dir::from_char).map(anchor_from_dir);
        let target_anchor = e.rhs_dir.and_then(Dir::from_char).map(anchor_from_dir);
        let curve_style_segments = match (
            e.lhs_dir.and_then(Dir::from_char),
            e.rhs_dir.and_then(Dir::from_char),
        ) {
            (Some(a), Some(b)) => a.is_x() != b.is_x(),
            _ => false,
        };

        let (label_width, label_height) = match e.title.map(str::trim).filter(|t| !t.is_empty()) {
            Some(label) => {
                let m = text_measurer.measure(label, &edge_text_style);
                let w = m.width.max(0.0);
                // Cytoscape edge label bounding boxes are slightly taller than the measured
                // font metrics height (roughly `fontSize + 1px` at Mermaid defaults).
                let h = (m.height + 1.0).max(0.0);
                (Some(w), Some(h))
            }
            None => (None, None),
        };
        edges.push(manatee::algo::fcose::IndexedEdge {
            source: a_idx,
            target: b_idx,
            label_width,
            label_height,
            source_anchor,
            target_anchor,
            curve_style_segments,
            ideal_length: base_ideal_length,
            elasticity,
        });
    }

    let default_edge_length = if default_edge_length_cnt > 0.0 {
        default_edge_length_sum / default_edge_length_cnt
    } else {
        50.0
    };

    let mut indexed_nodes: Vec<manatee::algo::fcose::IndexedNode> =
        Vec::with_capacity(layout_nodes.len());
    for (idx, n) in layout_nodes.iter().enumerate() {
        let model_node = &model.nodes[idx];
        let parent =
            match model_node.in_group {
                Some(group_id) => Some(*compound_index_by_id.get(group_id).ok_or_else(|| {
                    Error::InvalidModel {
                        message: format!("node parent group not found: {}/{}", n.id, group_id),
                    }
                })?),
                None => None,
            };
        indexed_nodes.push(manatee::algo::fcose::IndexedNode {
            parent,
            width: n.width,
            height: n.height,
            // Mermaid Architecture feeds Cytoscape node `position()` values directly
            // into the SVG `translate(x,y)` for the icon box (i.e. it treats the
            // Cytoscape "center" as a top-left anchor). This keeps the coordinate convention
            // consistent across nodes, edges, and viewBox in upstream baselines.
            x: n.x,
            y: n.y,
            bounds_extras: node_bounds_extras
                .get(model_node.id)
                .copied()
                .unwrap_or_default(),
        });
    }

    let mut indexed_compounds: Vec<manatee::algo::fcose::IndexedCompound> =
        Vec::with_capacity(model.groups.len());
    for g in &model.groups {
        let parent =
            match g.in_group {
                Some(parent_id) => Some(*compound_index_by_id.get(parent_id).ok_or_else(|| {
                    Error::InvalidModel {
                        message: format!("compound parent group not found: {}/{}", g.id, parent_id),
                    }
                })?),
                None => None,
            };
        indexed_compounds.push(manatee::algo::fcose::IndexedCompound { parent });
    }

    let graph = manatee::algo::fcose::IndexedGraph {
        nodes: indexed_nodes,
        edges,
        compounds: indexed_compounds,
    };

    // Mermaid Architecture styles group nodes with `padding: ${db.getConfigField('padding')}px`
    // before running FCoSE, and CoSE uses that per-compound padding when updating bounds.
    let compound_padding_px = padding_px;
    let options = manatee::algo::fcose::IndexedFcoseOptions {
        alignment_constraint: Some(manatee::algo::fcose::IndexedAlignmentConstraint {
            horizontal: horizontal_all,
            vertical: vertical_all,
        }),
        relative_placement_constraint: relative,
        default_edge_length: Some(default_edge_length),
        randomize: fcose_randomize,
        node_separation: Some(fcose_node_separation),
        num_iter: Some(fcose_num_iter),
        compound_padding: Some(compound_padding_px),
        relocate_center: None,
        // Mermaid Architecture runs the layout twice (`layout.run()` inside `layoutstop`),
        // which advances the seeded RNG stream and can change final positions.
        rerun: true,
        // Mermaid@11.15 wraps FCoSE in a seeded `Math.random()` helper. Seed 0 opts out
        // upstream; the Rust port keeps a deterministic fallback for headless repeatability.
        random_seed: fcose_seed,
        // The shipped Mermaid 11.15 Architecture render path consumes two seeded random
        // values before the first FCoSE constraint shuffle even when `randomize=false`.
        random_seed_offset: Some(2),
    };

    Ok(ArchitectureFcoseInputPlan {
        node_ids,
        compound_ids,
        node_index_by_id,
        spatial_maps,
        graph,
        options,
        default_edge_length,
        compound_padding_px,
    })
}

fn architecture_cytoscape_service_bounds<'a>(
    model: &ArchitectureModelView<'a>,
    nodes: &[LayoutNode],
    text_measurer: &dyn TextMeasurer,
    icon_size: f64,
    font_size_px: f64,
    font_family: &str,
) -> Vec<ArchitectureCytoscapeServiceBounds> {
    let text_style = architecture_cytoscape_text_style(font_size_px, font_family);
    let mut node_by_id: FxHashMap<&str, &LayoutNode> = FxHashMap::default();
    node_by_id.reserve(nodes.len().saturating_mul(2));
    for node in nodes {
        node_by_id.insert(node.id.as_str(), node);
    }

    let mut out = Vec::new();
    for node in &model.nodes {
        if node.node_type != ArchitectureNodeType::Service {
            continue;
        }
        let Some(layout_node) = node_by_id.get(node.id).copied() else {
            continue;
        };
        let body_bounds = Bounds {
            min_x: layout_node.x,
            min_y: layout_node.y,
            max_x: layout_node.x + icon_size,
            max_y: layout_node.y + icon_size,
        };
        let label_bounds = architecture_cytoscape_child_label_bounds(
            node.title,
            text_measurer,
            &text_style,
            font_size_px,
        );
        let label_metrics = label_bounds.map(|label| ArchitectureCytoscapeServiceLabelMetrics {
            text_width: label.metrics.width,
            half_width: label.half_width,
            applied_scale: label.metrics.applied_scale,
        });
        let contribution =
            architecture_cytoscape_child_contribution_bounds(&body_bounds, label_bounds.as_ref());
        out.push(ArchitectureCytoscapeServiceBounds {
            id: node.id.to_string(),
            in_group: node.in_group.map(str::to_string),
            body_bounds: contribution.body_bounds,
            label_bounds: contribution.label_bounds,
            label_metrics,
            union_bounds: contribution.union_bounds,
        });
    }
    out
}

fn compute_bounds(nodes: &[LayoutNode], edges: &[LayoutEdge]) -> Option<Bounds> {
    let mut pts: Vec<(f64, f64)> = Vec::new();
    for n in nodes {
        // Architecture renderer uses top-left anchored `translate(x, y)` for nodes.
        pts.push((n.x, n.y));
        pts.push((n.x + n.width, n.y + n.height));
    }
    for e in edges {
        for p in &e.points {
            pts.push((p.x, p.y));
        }
    }
    Bounds::from_points(pts)
}

fn architecture_bounds_from_layout_rect(rect: manatee::graph::LayoutRect) -> Bounds {
    Bounds {
        min_x: rect.left,
        min_y: rect.top,
        max_x: rect.left + rect.width,
        max_y: rect.top + rect.height,
    }
}

#[derive(Debug, Clone, Default)]
struct ArchitectureFcoseResultProjection {
    compound_bounds: Vec<ArchitectureCompoundBounds>,
    debug_stages: Vec<ArchitectureFcoseDebugStage>,
}

fn project_architecture_fcose_result(
    plan: &ArchitectureFcoseInputPlan<'_>,
    nodes: &mut [LayoutNode],
    result: manatee::algo::fcose::IndexedLayoutResult,
) -> ArchitectureFcoseResultProjection {
    for (idx, n) in nodes.iter_mut().enumerate() {
        if let Some(p) = result.node_positions.get(idx) {
            n.x = p.x;
            n.y = p.y;
        }
    }

    let mut compound_bounds = Vec::with_capacity(plan.compound_ids.len());
    for (idx, group_id) in plan.compound_ids.iter().enumerate() {
        if let Some(b) = result.compound_bounds.get(idx) {
            compound_bounds.push(ArchitectureCompoundBounds {
                id: (*group_id).to_string(),
                bounds: architecture_bounds_from_layout_rect(*b),
            });
        }
    }

    let mut debug_stages = Vec::with_capacity(result.debug_stages.len());
    for stage in result.debug_stages {
        let node_displacements = stage.node_displacements;
        let stage_nodes = stage
            .node_bounds
            .into_iter()
            .enumerate()
            .filter_map(|(idx, b)| {
                let displacement = node_displacements
                    .get(idx)
                    .map(|p| LayoutPoint { x: p.x, y: p.y });
                if let Some(node_id) = plan.node_ids.get(idx) {
                    Some(ArchitectureFcoseDebugNodeBounds {
                        id: (*node_id).to_string(),
                        kind: "node".to_string(),
                        bounds: architecture_bounds_from_layout_rect(b),
                        displacement,
                    })
                } else {
                    let group_idx = idx.checked_sub(plan.node_ids.len())?;
                    plan.compound_ids.get(group_idx).map(|group_id| {
                        ArchitectureFcoseDebugNodeBounds {
                            id: (*group_id).to_string(),
                            kind: "group".to_string(),
                            bounds: architecture_bounds_from_layout_rect(b),
                            displacement,
                        }
                    })
                }
            })
            .collect();
        let stage_compound_bounds = stage
            .compound_bounds
            .into_iter()
            .enumerate()
            .filter_map(|(idx, b)| {
                plan.compound_ids
                    .get(idx)
                    .map(|group_id| ArchitectureCompoundBounds {
                        id: (*group_id).to_string(),
                        bounds: architecture_bounds_from_layout_rect(b),
                    })
            })
            .collect();
        debug_stages.push(ArchitectureFcoseDebugStage {
            run_index: stage.run_index,
            tag: stage.tag,
            iterations: stage.iterations,
            bbox: stage.bbox.map(architecture_bounds_from_layout_rect),
            nodes: stage_nodes,
            compound_bounds: stage_compound_bounds,
            relocate: stage.relocate.map(|r| ArchitectureFcoseRelocateDebug {
                original_center: LayoutPoint {
                    x: r.original_center.x,
                    y: r.original_center.y,
                },
                rect_center: LayoutPoint {
                    x: r.rect_center.x,
                    y: r.rect_center.y,
                },
                delta: LayoutPoint {
                    x: r.delta.x,
                    y: r.delta.y,
                },
            }),
        });
    }

    ArchitectureFcoseResultProjection {
        compound_bounds,
        debug_stages,
    }
}

pub fn layout_architecture_diagram(
    model: &Value,
    effective_config: &Value,
    _text_measurer: &dyn TextMeasurer,
    use_manatee_layout: bool,
) -> Result<ArchitectureDiagramLayout> {
    let model_json: ArchitectureModel = from_value_ref(model)?;
    let model_view = ArchitectureModelView::from_json(&model_json);
    layout_architecture_diagram_model(
        &model_view,
        effective_config,
        _text_measurer,
        use_manatee_layout,
    )
}

pub fn layout_architecture_diagram_typed(
    model: &ArchitectureDiagramRenderModel,
    effective_config: &Value,
    text_measurer: &dyn TextMeasurer,
    use_manatee_layout: bool,
) -> Result<ArchitectureDiagramLayout> {
    let model = ArchitectureModelView::from_typed(model);
    layout_architecture_diagram_model(&model, effective_config, text_measurer, use_manatee_layout)
}

fn layout_architecture_diagram_model(
    model: &ArchitectureModelView<'_>,
    effective_config: &Value,
    text_measurer: &dyn TextMeasurer,
    use_manatee_layout: bool,
) -> Result<ArchitectureDiagramLayout> {
    let timing_enabled = std::env::var("MERMAN_ARCHITECTURE_LAYOUT_TIMING")
        .ok()
        .as_deref()
        == Some("1");
    #[derive(Debug, Default, Clone)]
    struct ArchitectureLayoutTimings {
        total: web_time::Duration,
        build_adjacency_and_components: web_time::Duration,
        positions_and_centering: web_time::Duration,
        emit_nodes: web_time::Duration,
        manatee_prepare: web_time::Duration,
        manatee_layout: web_time::Duration,
        build_edges: web_time::Duration,
        bounds: web_time::Duration,
    }
    let mut timings = ArchitectureLayoutTimings::default();
    let total_start = timing_enabled.then(web_time::Instant::now);

    let icon_size = config_f64(effective_config, &["architecture", "iconSize"]).unwrap_or(80.0);
    let icon_size = icon_size.max(1.0);
    let half_icon = icon_size / 2.0;
    let padding_px = config_f64(effective_config, &["architecture", "padding"]).unwrap_or(40.0);
    let padding_px = padding_px.max(0.0);
    let font_size_px = config_f64(effective_config, &["architecture", "fontSize"]).unwrap_or(16.0);
    let font_size_px = font_size_px.max(1.0);
    let font_family = crate::config::config_font_family_css(effective_config);
    let fcose_randomize =
        config_bool(effective_config, &["architecture", "randomize"]).unwrap_or(false);
    let fcose_node_separation = config_f64(effective_config, &["architecture", "nodeSeparation"])
        .filter(|v| v.is_finite() && *v > 0.0)
        .unwrap_or(75.0);
    let ideal_edge_length_multiplier = config_f64(
        effective_config,
        &["architecture", "idealEdgeLengthMultiplier"],
    )
    .filter(|v| v.is_finite() && *v > 0.0)
    .unwrap_or(1.5);
    let same_group_edge_elasticity =
        config_f64(effective_config, &["architecture", "edgeElasticity"])
            .filter(|v| v.is_finite() && *v >= 0.0)
            .unwrap_or(0.45);
    let fcose_num_iter = config_f64(effective_config, &["architecture", "numIter"])
        .filter(|v| v.is_finite() && *v >= 1.0)
        .map(|v| v.round() as usize)
        .unwrap_or(2500);
    let fcose_seed = config_f64(effective_config, &["architecture", "seed"])
        .filter(|v| v.is_finite() && *v >= 1.0)
        .map(|v| v.round() as u64)
        .unwrap_or(1);

    let node_bounds_extras =
        architecture_fcose_node_bounds_extras(ArchitectureFcoseNodeBoundsExtrasInput {
            model,
            text_measurer,
            icon_size,
            font_size_px,
            font_family: font_family.as_str(),
        });
    if std::env::var("MERMAN_ARCH_DEBUG_NODE_BOUNDS_EXTRAS")
        .ok()
        .as_deref()
        == Some("1")
    {
        eprintln!(
            "[arch-node-bounds-extras] icon_size={:.3} font_size={:.3} nodes={} extras={}",
            icon_size,
            font_size_px,
            model.nodes.len(),
            node_bounds_extras.len(),
        );
    }

    let mut nodes: Vec<LayoutNode> = Vec::new();

    let positions_start = timing_enabled.then(web_time::Instant::now);
    if let Some(s) = positions_start {
        timings.positions_and_centering = s.elapsed();
    }

    // Emit nodes in Mermaid model order (stable for snapshots and close to upstream).
    let emit_nodes_start = timing_enabled.then(web_time::Instant::now);
    for n in &model.nodes {
        match n.node_type {
            ArchitectureNodeType::Service | ArchitectureNodeType::Junction => {}
            other => {
                return Err(Error::InvalidModel {
                    message: format!("unsupported architecture node type: {other:?}"),
                });
            }
        }

        nodes.push(LayoutNode {
            id: n.id.to_string(),
            // Cytoscape nodes default to `{ x: 0, y: 0 }` centers before the first layout run.
            // Our SVG model uses a top-left anchored `<g transform="translate(x,y)">` for the
            // 80x80 icon box, so convert `(0,0)` center into top-left.
            x: 0.0,
            y: 0.0,
            width: icon_size,
            height: icon_size,
            is_cluster: false,
            label_width: None,
            label_height: None,
        });
    }
    if let Some(s) = emit_nodes_start {
        timings.emit_nodes = s.elapsed();
    }

    let mut fcose_compound_bounds: Vec<ArchitectureCompoundBounds> = Vec::new();
    let mut fcose_debug_stages: Vec<ArchitectureFcoseDebugStage> = Vec::new();

    if use_manatee_layout && !nodes.is_empty() {
        let manatee_prepare_start = timing_enabled.then(web_time::Instant::now);
        let plan = build_architecture_fcose_input_plan(ArchitectureFcoseInputPlanInput {
            model,
            layout_nodes: &nodes,
            node_bounds_extras: &node_bounds_extras,
            text_measurer,
            icon_size,
            padding_px,
            ideal_edge_length_multiplier,
            same_group_edge_elasticity,
            fcose_randomize,
            fcose_node_separation,
            fcose_num_iter,
            fcose_seed,
        })?;

        if std::env::var("MERMAN_ARCH_DEBUG_FCOSE_CONSTRAINTS")
            .ok()
            .as_deref()
            == Some("1")
        {
            eprintln!(
                "[arch-fcose] nodes={} edges={} compounds={} spatial_maps={} index_entries={} default_edge_length={:.6} compound_padding={:.6}",
                plan.graph.nodes.len(),
                plan.graph.edges.len(),
                plan.graph.compounds.len(),
                plan.spatial_maps.len(),
                plan.node_index_by_id.len(),
                plan.default_edge_length,
                plan.compound_padding_px,
            );
            if let Some(a) = &plan.options.alignment_constraint {
                eprintln!("[arch-fcose] alignment.horizontal={:?}", a.horizontal);
                eprintln!("[arch-fcose] alignment.vertical={:?}", a.vertical);
            }
            eprintln!(
                "[arch-fcose] relative_placement_constraint={:?}",
                plan.options.relative_placement_constraint
            );
        }

        if let Some(s) = manatee_prepare_start {
            timings.manatee_prepare = s.elapsed();
        }
        let manatee_layout_start = timing_enabled.then(web_time::Instant::now);
        let result =
            manatee::algo::fcose::layout_indexed(&plan.graph, &plan.options).map_err(|e| {
                Error::InvalidModel {
                    message: format!("manatee layout failed: {e}"),
                }
            })?;
        if let Some(s) = manatee_layout_start {
            timings.manatee_layout = s.elapsed();
        }

        let projection = project_architecture_fcose_result(&plan, &mut nodes, result);
        fcose_compound_bounds = projection.compound_bounds;
        fcose_debug_stages = projection.debug_stages;
    }

    let cytoscape_service_bounds = architecture_cytoscape_service_bounds(
        model,
        &nodes,
        text_measurer,
        icon_size,
        font_size_px,
        font_family.as_str(),
    );

    let build_edges_start = timing_enabled.then(web_time::Instant::now);
    let mut node_by_id: FxHashMap<&str, &LayoutNode> = FxHashMap::default();
    node_by_id.reserve(nodes.len());
    for n in &nodes {
        node_by_id.insert(n.id.as_str(), n);
    }

    let mut edges: Vec<LayoutEdge> = Vec::new();
    for (idx, e) in model.edges.iter().enumerate() {
        let Some(&a) = node_by_id.get(e.lhs_id) else {
            return Err(Error::InvalidModel {
                message: format!("edge lhs node not found: {}", e.lhs_id),
            });
        };
        let Some(&b) = node_by_id.get(e.rhs_id) else {
            return Err(Error::InvalidModel {
                message: format!("edge rhs node not found: {}", e.rhs_id),
            });
        };

        fn endpoint(
            x: f64,
            y: f64,
            dir: Option<char>,
            icon_size: f64,
            half_icon: f64,
        ) -> (f64, f64) {
            match dir {
                Some('L') => (x, y + half_icon),
                Some('R') => (x + icon_size, y + half_icon),
                Some('T') => (x + half_icon, y),
                Some('B') => (x + half_icon, y + icon_size),
                _ => (x + half_icon, y + half_icon),
            }
        }

        let (sx, sy) = endpoint(a.x, a.y, e.lhs_dir, icon_size, half_icon);
        let (tx, ty) = endpoint(b.x, b.y, e.rhs_dir, icon_size, half_icon);

        fn cytoscape_segments_weight_distance_for_point(
            source: (f64, f64),
            target: (f64, f64),
            point: (f64, f64),
        ) -> Option<(f64, f64)> {
            // Mermaid Architecture uses Cytoscape `curve-style: segments` for XY edges and derives
            // `segment-weights`/`segment-distances` from a chosen 90° bend point.
            //
            // Reference: `repo-ref/mermaid/packages/mermaid/src/diagrams/architecture/architectureRenderer.ts`
            let (s_x, s_y) = source;
            let (t_x, t_y) = target;
            let (p_x, p_y) = point;

            if s_x == t_x || s_y == t_y {
                return None;
            }

            let denom_x = s_x - t_x;
            if denom_x == 0.0 {
                return None;
            }

            let slope = (s_y - t_y) / denom_x;
            let d =
                (p_y - s_y + ((s_x - p_x) * (s_y - t_y)) / denom_x) / (1.0 + slope * slope).sqrt();

            let w = ((p_y - s_y).powi(2) + (p_x - s_x).powi(2) - d.powi(2))
                .max(0.0)
                .sqrt();
            let dist_ab = ((t_x - s_x).powi(2) + (t_y - s_y).powi(2)).sqrt();
            if dist_ab == 0.0 {
                return None;
            }
            let mut w = w / dist_ab;

            // Ensure that the sign of `d` matches the left/right side of the line from source to
            // target, and that the sign of `w` matches whether the point is "behind" the source.
            let delta1 = (t_x - s_x) * (p_y - s_y) - (t_y - s_y) * (p_x - s_x);
            let delta1 = if delta1 >= 0.0 { 1.0 } else { -1.0 };
            let delta2 = (t_x - s_x) * (p_x - s_x) + (t_y - s_y) * (p_y - s_y);
            let delta2 = if delta2 >= 0.0 { 1.0 } else { -1.0 };

            let d = d.abs() * delta1;
            w *= delta2;

            Some((w, d))
        }

        fn cytoscape_segments_point_from_weight_distance(
            source: (f64, f64),
            target: (f64, f64),
            weight: f64,
            distance: f64,
        ) -> Option<(f64, f64)> {
            // Cytoscape "segments" curve point (for a single segment) is defined by:
            // - `weight`: normalized distance along the source->target vector
            // - `distance`: signed perpendicular offset from the line
            //
            // We reconstruct the implied bend point so our headless routing matches the
            // upstream browser output.
            let (s_x, s_y) = source;
            let (t_x, t_y) = target;
            let dx = t_x - s_x;
            let dy = t_y - s_y;
            let dist_ab = (dx * dx + dy * dy).sqrt();
            if dist_ab == 0.0 {
                return None;
            }

            let ux = dx / dist_ab;
            let uy = dy / dist_ab;
            // Left-hand normal of the line.
            let nx = -uy;
            let ny = ux;

            let along = weight * dist_ab;
            Some((
                s_x + ux * along + nx * distance,
                s_y + uy * along + ny * distance,
            ))
        }
        // Mirror Mermaid Architecture edge routing:
        //
        // - Non-XY edges use Cytoscape `curve-style: straight`, and Mermaid draws a 3-point
        //   polyline using `edge.midpoint()`, which is the midpoint of the straight segment.
        // - XY edges (`curve-style: segments`) are post-processed to create a single 90° bend.
        //   Mermaid then draws a 3-point polyline where the midpoint corresponds to that bend.
        //
        // Note: Group/junction endpoint shifts are applied later during SVG emission; these
        // layout points represent the raw Cytoscape endpoints.
        let is_xy = match (
            e.lhs_dir.and_then(Dir::from_char),
            e.rhs_dir.and_then(Dir::from_char),
        ) {
            (Some(a), Some(b)) => a.is_x() != b.is_x(),
            _ => false,
        };
        let mid = if is_xy {
            let (point_x, point_y) = if matches!(e.lhs_dir, Some('T' | 'B')) {
                (sx, ty)
            } else {
                (tx, sy)
            };
            let (w, d) = cytoscape_segments_weight_distance_for_point(
                (sx, sy),
                (tx, ty),
                (point_x, point_y),
            )
            .unwrap_or((0.0, 0.0));
            let (mx, my) = cytoscape_segments_point_from_weight_distance((sx, sy), (tx, ty), w, d)
                .unwrap_or((point_x, point_y));
            LayoutPoint { x: mx, y: my }
        } else {
            LayoutPoint {
                x: (sx + tx) / 2.0,
                y: (sy + ty) / 2.0,
            }
        };
        edges.push(LayoutEdge {
            id: format!("edge-{idx}"),
            from: e.lhs_id.to_string(),
            to: e.rhs_id.to_string(),
            from_cluster: None,
            to_cluster: None,
            points: vec![
                LayoutPoint { x: sx, y: sy },
                mid,
                LayoutPoint { x: tx, y: ty },
            ],
            label: None,
            start_label_left: None,
            start_label_right: None,
            end_label_left: None,
            end_label_right: None,
            start_marker: None,
            end_marker: None,
            stroke_dasharray: None,
        });
    }
    if let Some(s) = build_edges_start {
        timings.build_edges = s.elapsed();
    }

    let bounds_start = timing_enabled.then(web_time::Instant::now);
    let bounds = compute_bounds(&nodes, &edges);
    if let Some(s) = bounds_start {
        timings.bounds = s.elapsed();
    }

    if let Some(s) = total_start {
        timings.total = s.elapsed();
        eprintln!(
            "[layout-timing] diagram=architecture total={:?} adjacency={:?} positions={:?} emit_nodes={:?} manatee_prepare={:?} manatee_layout={:?} build_edges={:?} bounds={:?} nodes={} edges={} groups={} use_manatee_layout={}",
            timings.total,
            timings.build_adjacency_and_components,
            timings.positions_and_centering,
            timings.emit_nodes,
            timings.manatee_prepare,
            timings.manatee_layout,
            timings.build_edges,
            timings.bounds,
            nodes.len(),
            edges.len(),
            model.groups.len(),
            use_manatee_layout,
        );
    }

    Ok(ArchitectureDiagramLayout {
        nodes,
        edges,
        cytoscape_service_bounds,
        fcose_compound_bounds,
        fcose_debug_stages,
        bounds,
    })
}

#[cfg(test)]
mod tests {
    fn layout_node(id: &str, width: f64, height: f64) -> crate::model::LayoutNode {
        crate::model::LayoutNode {
            id: id.to_string(),
            x: 0.0,
            y: 0.0,
            width,
            height,
            is_cluster: false,
            label_width: None,
            label_height: None,
        }
    }

    fn layout_rect(left: f64, top: f64, width: f64, height: f64) -> manatee::LayoutRect {
        manatee::LayoutRect {
            left,
            top,
            width,
            height,
        }
    }

    fn build_test_plan<'a>(
        model: &'a super::ArchitectureModelView<'a>,
        layout_nodes: &[crate::model::LayoutNode],
        node_bounds_extras: &rustc_hash::FxHashMap<&'a str, manatee::BoundsExtras>,
    ) -> super::ArchitectureFcoseInputPlan<'a> {
        let measurer = crate::text::DeterministicTextMeasurer::default();
        super::build_architecture_fcose_input_plan(super::ArchitectureFcoseInputPlanInput {
            model,
            layout_nodes,
            node_bounds_extras,
            text_measurer: &measurer,
            icon_size: 80.0,
            padding_px: 40.0,
            ideal_edge_length_multiplier: 1.5,
            same_group_edge_elasticity: 0.45,
            fcose_randomize: false,
            fcose_node_separation: 75.0,
            fcose_num_iter: 2500,
            fcose_seed: 1,
        })
        .expect("build architecture FCoSE input plan")
    }

    #[test]
    fn architecture_fcose_input_plan_preserves_minimal_graph_order_and_edge_input() {
        let model = super::ArchitectureModelView {
            nodes: vec![
                super::ArchitectureNodeView {
                    id: "api",
                    node_type: super::ArchitectureNodeType::Service,
                    title: Some("API"),
                    in_group: None,
                },
                super::ArchitectureNodeView {
                    id: "db",
                    node_type: super::ArchitectureNodeType::Service,
                    title: Some("DB"),
                    in_group: None,
                },
            ],
            groups: Vec::new(),
            edges: vec![super::ArchitectureEdgeView {
                lhs_id: "api",
                rhs_id: "db",
                lhs_dir: Some('R'),
                rhs_dir: Some('L'),
                title: Some("reads"),
            }],
        };
        let layout_nodes = vec![
            layout_node("api", 80.0, 80.0),
            layout_node("db", 80.0, 80.0),
        ];
        let node_bounds_extras = rustc_hash::FxHashMap::default();

        let plan = build_test_plan(&model, &layout_nodes, &node_bounds_extras);

        assert_eq!(plan.node_ids, vec!["api", "db"]);
        assert_eq!(plan.node_index_by_id.get("api"), Some(&0));
        assert_eq!(plan.node_index_by_id.get("db"), Some(&1));
        assert_eq!(plan.graph.nodes.len(), 2);
        assert_eq!(plan.graph.nodes[0].width, 80.0);
        assert_eq!(plan.graph.nodes[0].height, 80.0);
        assert_eq!(plan.graph.edges.len(), 1);
        assert_eq!(plan.graph.edges[0].source, 0);
        assert_eq!(plan.graph.edges[0].target, 1);
        assert_eq!(
            plan.graph.edges[0].source_anchor,
            Some(manatee::Anchor::Right)
        );
        assert_eq!(
            plan.graph.edges[0].target_anchor,
            Some(manatee::Anchor::Left)
        );
        assert!(plan.graph.edges[0].label_width.unwrap_or_default() > 0.0);
        assert_eq!(plan.options.default_edge_length, Some(120.0));
    }

    #[test]
    fn architecture_fcose_input_plan_preserves_nested_compound_parents() {
        let model = super::ArchitectureModelView {
            nodes: vec![
                super::ArchitectureNodeView {
                    id: "api",
                    node_type: super::ArchitectureNodeType::Service,
                    title: None,
                    in_group: Some("platform"),
                },
                super::ArchitectureNodeView {
                    id: "db",
                    node_type: super::ArchitectureNodeType::Service,
                    title: None,
                    in_group: Some("core"),
                },
            ],
            groups: vec![
                super::ArchitectureGroupView {
                    id: "core",
                    in_group: None,
                },
                super::ArchitectureGroupView {
                    id: "platform",
                    in_group: Some("core"),
                },
            ],
            edges: Vec::new(),
        };
        let layout_nodes = vec![
            layout_node("api", 80.0, 80.0),
            layout_node("db", 80.0, 80.0),
        ];
        let node_bounds_extras = rustc_hash::FxHashMap::default();

        let plan = build_test_plan(&model, &layout_nodes, &node_bounds_extras);

        assert_eq!(plan.compound_ids, vec!["core", "platform"]);
        assert_eq!(plan.graph.compounds.len(), 2);
        assert_eq!(plan.graph.compounds[0].parent, None);
        assert_eq!(plan.graph.compounds[1].parent, Some(0));
        assert_eq!(plan.graph.nodes[0].parent, Some(1));
        assert_eq!(plan.graph.nodes[1].parent, Some(0));
    }

    #[test]
    fn architecture_fcose_input_plan_uses_layout_node_size_and_bounds_extras() {
        let model = super::ArchitectureModelView {
            nodes: vec![super::ArchitectureNodeView {
                id: "api",
                node_type: super::ArchitectureNodeType::Service,
                title: Some("API"),
                in_group: None,
            }],
            groups: Vec::new(),
            edges: Vec::new(),
        };
        let layout_nodes = vec![layout_node("api", 96.0, 72.0)];
        let mut node_bounds_extras = rustc_hash::FxHashMap::default();
        node_bounds_extras.insert(
            "api",
            manatee::BoundsExtras {
                left: 5.0,
                right: 6.0,
                top: 7.0,
                bottom: 8.0,
            },
        );

        let plan = build_test_plan(&model, &layout_nodes, &node_bounds_extras);

        let node = plan.graph.nodes[0];
        assert_eq!(node.width, 96.0);
        assert_eq!(node.height, 72.0);
        assert_eq!(node.bounds_extras.left, 5.0);
        assert_eq!(node.bounds_extras.right, 6.0);
        assert_eq!(node.bounds_extras.top, 7.0);
        assert_eq!(node.bounds_extras.bottom, 8.0);
    }

    #[test]
    fn architecture_fcose_input_plan_deduplicates_undirected_layout_edges() {
        let model = super::ArchitectureModelView {
            nodes: vec![
                super::ArchitectureNodeView {
                    id: "api",
                    node_type: super::ArchitectureNodeType::Service,
                    title: None,
                    in_group: None,
                },
                super::ArchitectureNodeView {
                    id: "db",
                    node_type: super::ArchitectureNodeType::Service,
                    title: None,
                    in_group: None,
                },
            ],
            groups: Vec::new(),
            edges: vec![
                super::ArchitectureEdgeView {
                    lhs_id: "api",
                    rhs_id: "db",
                    lhs_dir: Some('R'),
                    rhs_dir: Some('L'),
                    title: None,
                },
                super::ArchitectureEdgeView {
                    lhs_id: "db",
                    rhs_id: "api",
                    lhs_dir: Some('L'),
                    rhs_dir: Some('R'),
                    title: None,
                },
            ],
        };
        let layout_nodes = vec![
            layout_node("api", 80.0, 80.0),
            layout_node("db", 80.0, 80.0),
        ];
        let node_bounds_extras = rustc_hash::FxHashMap::default();

        let plan = build_test_plan(&model, &layout_nodes, &node_bounds_extras);

        assert_eq!(plan.graph.edges.len(), 1);
        assert_eq!(plan.graph.edges[0].source, 0);
        assert_eq!(plan.graph.edges[0].target, 1);
    }

    #[test]
    fn architecture_fcose_result_projection_updates_nodes_and_maps_debug_ids() {
        let model = super::ArchitectureModelView {
            nodes: vec![
                super::ArchitectureNodeView {
                    id: "api",
                    node_type: super::ArchitectureNodeType::Service,
                    title: None,
                    in_group: Some("core"),
                },
                super::ArchitectureNodeView {
                    id: "db",
                    node_type: super::ArchitectureNodeType::Service,
                    title: None,
                    in_group: None,
                },
            ],
            groups: vec![super::ArchitectureGroupView {
                id: "core",
                in_group: None,
            }],
            edges: Vec::new(),
        };
        let mut layout_nodes = vec![
            layout_node("api", 80.0, 80.0),
            layout_node("db", 80.0, 80.0),
        ];
        let node_bounds_extras = rustc_hash::FxHashMap::default();
        let plan = build_test_plan(&model, &layout_nodes, &node_bounds_extras);

        let result = manatee::algo::fcose::IndexedLayoutResult {
            node_positions: vec![
                manatee::Point { x: 10.0, y: 20.0 },
                manatee::Point { x: 30.0, y: 40.0 },
            ],
            compound_positions: vec![manatee::Point { x: 50.0, y: 60.0 }],
            compound_bounds: vec![layout_rect(5.0, 6.0, 100.0, 120.0)],
            debug_stages: vec![manatee::algo::fcose::IndexedFcoseDebugStage {
                run_index: 1,
                tag: "final".to_string(),
                iterations: Some(7),
                bbox: Some(layout_rect(0.0, 0.0, 160.0, 180.0)),
                node_bounds: vec![
                    layout_rect(10.0, 20.0, 80.0, 80.0),
                    layout_rect(30.0, 40.0, 80.0, 80.0),
                    layout_rect(5.0, 6.0, 100.0, 120.0),
                ],
                node_displacements: vec![
                    manatee::Point { x: 1.0, y: 2.0 },
                    manatee::Point { x: 3.0, y: 4.0 },
                    manatee::Point { x: 5.0, y: 6.0 },
                ],
                compound_bounds: vec![layout_rect(5.0, 6.0, 100.0, 120.0)],
                relocate: Some(manatee::algo::fcose::IndexedFcoseRelocateDebug {
                    original_center: manatee::Point { x: 10.0, y: 20.0 },
                    rect_center: manatee::Point { x: 30.0, y: 40.0 },
                    delta: manatee::Point { x: 2.0, y: 3.0 },
                }),
            }],
        };

        let projection = super::project_architecture_fcose_result(&plan, &mut layout_nodes, result);

        assert_eq!((layout_nodes[0].x, layout_nodes[0].y), (10.0, 20.0));
        assert_eq!((layout_nodes[1].x, layout_nodes[1].y), (30.0, 40.0));
        assert_eq!(projection.compound_bounds.len(), 1);
        assert_eq!(projection.compound_bounds[0].id, "core");
        assert_eq!(projection.compound_bounds[0].bounds.min_x, 5.0);
        assert_eq!(projection.compound_bounds[0].bounds.max_y, 126.0);

        let stage = &projection.debug_stages[0];
        assert_eq!(stage.run_index, 1);
        assert_eq!(stage.tag, "final");
        assert_eq!(stage.nodes.len(), 3);
        assert_eq!(stage.nodes[0].id, "api");
        assert_eq!(stage.nodes[0].kind, "node");
        assert_eq!(stage.nodes[1].id, "db");
        assert_eq!(stage.nodes[2].id, "core");
        assert_eq!(stage.nodes[2].kind, "group");
        let group_displacement = stage.nodes[2]
            .displacement
            .as_ref()
            .expect("group displacement");
        assert_eq!((group_displacement.x, group_displacement.y), (5.0, 6.0));
        assert_eq!(stage.compound_bounds[0].id, "core");
        let relocate = stage.relocate.as_ref().expect("relocate debug");
        assert_eq!((relocate.delta.x, relocate.delta.y), (2.0, 3.0));
    }

    #[test]
    fn architecture_fcose_node_bounds_extras_feed_label_bounds() {
        let model = super::ArchitectureModelView {
            nodes: vec![super::ArchitectureNodeView {
                id: "api",
                node_type: super::ArchitectureNodeType::Service,
                title: Some("API"),
                in_group: Some("core"),
            }],
            groups: vec![super::ArchitectureGroupView {
                id: "core",
                in_group: None,
            }],
            edges: Vec::new(),
        };
        let measurer = crate::text::DeterministicTextMeasurer::default();

        let node_bounds_extras = super::architecture_fcose_node_bounds_extras(
            super::ArchitectureFcoseNodeBoundsExtrasInput {
                model: &model,
                text_measurer: &measurer,
                icon_size: 80.0,
                font_size_px: 16.0,
                font_family: crate::config::MERMAID_DEFAULT_FONT_FAMILY_CSS,
            },
        );
        let extras = node_bounds_extras.get("api").expect("api node extras");

        assert_eq!(extras.top, 1.0);
        assert_eq!(extras.bottom, 18.0);
        assert_eq!(extras.left, 1.0);
        assert_eq!(extras.right, 1.0);
    }

    #[test]
    fn architecture_fcose_edge_label_style_keeps_cytoscape_defaults() {
        let node_style =
            super::architecture_cytoscape_text_style(18.0, r#""IBM Plex Sans",Arial,sans-serif"#);
        let edge_style = super::architecture_cytoscape_edge_text_style();

        assert_eq!(node_style.font_size, 18.0);
        assert_eq!(
            node_style.font_family.as_deref(),
            Some(r#""IBM Plex Sans",Arial,sans-serif"#)
        );
        assert_eq!(edge_style.font_size, 16.0);
        assert_eq!(edge_style.font_family.as_deref(), Some("sans-serif"));
    }

    #[test]
    fn architecture_relative_constraints_preserve_mermaid_duplicate_bfs_pops() {
        let mut spatial_map = indexmap::IndexMap::new();
        spatial_map.insert("ingress", (0, 0));
        spatial_map.insert("fork", (1, 0));
        spatial_map.insert("auth", (2, 0));
        spatial_map.insert("api", (1, -1));
        spatial_map.insert("join", (2, -1));
        spatial_map.insert("db", (3, -1));
        spatial_map.insert("cache", (2, -2));

        let mut node_index_by_id = rustc_hash::FxHashMap::default();
        for (idx, id) in ["ingress", "auth", "api", "db", "cache", "fork", "join"]
            .into_iter()
            .enumerate()
        {
            node_index_by_id.insert(id, idx);
        }

        let constraints = super::architecture_relative_placement_constraints(
            &[spatial_map],
            &node_index_by_id,
            120.0,
        );

        assert_eq!(constraints.len(), 9);
        assert_eq!(
            constraints
                .iter()
                .filter(|c| c.left == Some(6) && c.right == Some(3))
                .count(),
            2,
            "Mermaid processes the duplicate queued join position before db is visited",
        );
        assert_eq!(
            constraints
                .iter()
                .filter(|c| c.top == Some(6) && c.bottom == Some(4))
                .count(),
            2,
            "Mermaid processes the duplicate queued join position before cache is visited",
        );
    }
}

//! State diagram layout implementation (stateDiagram-v2).

use crate::generated::state_text_overrides_11_12_2 as state_text_overrides;
use crate::model::{
    Bounds, LayoutCluster, LayoutEdge, LayoutLabel, LayoutNode, LayoutPoint, StateDiagramV2Layout,
};
use crate::text::{TextMeasurer, TextStyle, WrapMode};
use crate::{Error, Result};
use dugong::graphlib::{EdgeKey, Graph, GraphOptions};
use dugong::{EdgeLabel, GraphLabel, LabelPos, NodeLabel, RankDir};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};

// Mermaid@11.12.2 renders end states (`[*]`) as a path-based circle whose `getBBox().width`
// is slightly larger than the nominal diameter (14px). Mermaid feeds that measured bbox into
// Dagre, which can shift the end node center by ~0.0066px and affect root `viewBox/max-width`.
use super::config::*;
use super::{STATE_END_NODE_DAGRE_WIDTH_PX_11_12_2, StateDiagramModel, StateNode};

struct PreparedGraph {
    graph: Graph<NodeLabel, EdgeLabel, GraphLabel>,
    extracted: BTreeMap<String, PreparedGraph>,
    root_cluster_id: Option<String>,
}

impl Drop for PreparedGraph {
    fn drop(&mut self) {
        let extracted = std::mem::take(&mut self.extracted);
        let mut stack: Vec<PreparedGraph> = extracted.into_values().collect();
        while let Some(mut graph) = stack.pop() {
            let children = std::mem::take(&mut graph.extracted);
            stack.extend(children.into_values());
        }
    }
}

type Rect = merman_core::geom::Box2;

#[derive(Debug, Clone)]
struct EdgeSegment {
    original_id: String,
    segment: i32,
    original_from: String,
    original_to: String,
    from_cluster: Option<String>,
    to_cluster: Option<String>,
    points: Vec<LayoutPoint>,
    label: Option<LayoutLabel>,
}

#[derive(Debug, Clone)]
struct LayoutFragments {
    nodes: HashMap<String, LayoutNode>,
    edge_segments: Vec<EdgeSegment>,
}

struct StateDagreInput {
    graph: Graph<NodeLabel, EdgeLabel, GraphLabel>,
    hidden_prefixes: Vec<String>,
    dagre_id_by_semantic_id: HashMap<String, String>,
    dir_by_dagre_id: HashMap<String, Option<String>>,
    text_style: TextStyle,
    wrap_mode: WrapMode,
    html_labels: bool,
}

fn get_extras_string(
    extras: &std::collections::BTreeMap<String, Value>,
    key: &str,
) -> Option<String> {
    extras
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn set_extras_string(
    extras: &mut std::collections::BTreeMap<String, Value>,
    key: &str,
    value: &str,
) {
    extras.insert(key.to_string(), Value::String(value.to_string()));
}

fn set_extras_i32(extras: &mut std::collections::BTreeMap<String, Value>, key: &str, value: i32) {
    extras.insert(key.to_string(), Value::Number(value.into()));
}

fn edge_label_metrics(
    label: &str,
    measurer: &dyn TextMeasurer,
    text_style: &TextStyle,
    wrap_mode: WrapMode,
) -> (f64, f64) {
    if label.trim().is_empty() {
        return (0.0, 0.0);
    }
    // Mermaid stores sanitized labels that can contain HTML entities like `&lt;`. In the browser,
    // those are decoded before layout/measurement, so decode them here to avoid skewing widths.
    let decoded = decode_html_entities_once(label);
    let wrapping_width = state_text_overrides::state_edge_label_max_width_px();
    let mut metrics = measurer.measure_wrapped(
        decoded.as_ref(),
        text_style,
        Some(wrapping_width),
        wrap_mode,
    );
    // For SVG edge labels, `createText(..., addSvgBackground=true)` adds a background rect with a
    // 2px padding.
    if wrap_mode == WrapMode::SvgLike {
        metrics.width += 4.0;
        metrics.height += 4.0;
    }

    if wrap_mode == WrapMode::HtmlLike {
        // Mermaid DOM measurements routinely land on a 1/64px lattice.
        metrics.width = crate::text::round_to_1_64_px(metrics.width);
        if wrapping_width.is_finite() && wrapping_width > 0.0 {
            metrics.width = metrics.width.min(wrapping_width);
        }

        let trimmed = decoded.as_ref().trim();
        if let Some(w) =
            state_text_overrides::lookup_state_edge_label_width_px(text_style.font_size, trimmed)
        {
            metrics.width = w;
        }
    }
    (metrics.width.max(0.0), metrics.height.max(0.0))
}

fn node_label_metrics(
    label: &str,
    wrapping_width: f64,
    node_css_compiled_styles: &[String],
    node_css_styles: &[String],
    measurer: &dyn TextMeasurer,
    text_style: &TextStyle,
    wrap_mode: WrapMode,
) -> (f64, f64) {
    fn parse_css_px_f64(v: &str) -> Option<f64> {
        let t = v.trim();
        let t = t.trim_end_matches(';').trim();
        let t = t.trim_end_matches("!important").trim();
        let t = t.trim_end_matches("px").trim();
        t.parse::<f64>().ok()
    }

    fn parse_text_style_overrides(
        compiled: &[String],
        direct: &[String],
    ) -> (Option<String>, bool, Option<f64>, Option<String>) {
        let mut weight: Option<String> = None;
        let mut italic: bool = false;
        let mut font_size_px: Option<f64> = None;
        let mut font_family: Option<String> = None;

        for raw in compiled.iter().chain(direct.iter()) {
            let raw = raw.trim().trim_end_matches(';').trim();
            if raw.is_empty() {
                continue;
            }
            let Some((k, v)) = raw.split_once(':') else {
                continue;
            };
            let key = k.trim().to_ascii_lowercase();
            let val = v.trim();
            match key.as_str() {
                "font-weight" => {
                    let val = val.trim_end_matches("!important").trim();
                    if !val.is_empty() {
                        weight = Some(val.to_string());
                    }
                }
                "font-style" => {
                    let val = val
                        .trim_end_matches("!important")
                        .trim()
                        .to_ascii_lowercase();
                    if val.contains("italic") || val.contains("oblique") {
                        italic = true;
                    }
                }
                "font-size" => {
                    if let Some(px) = parse_css_px_f64(val) {
                        if px.is_finite() && px > 0.0 {
                            font_size_px = Some(px);
                        }
                    }
                }
                "font-family" => {
                    let val = val.trim_end_matches("!important").trim();
                    if !val.is_empty() {
                        font_family = Some(val.to_string());
                    }
                }
                _ => {}
            }
        }

        (weight, italic, font_size_px, font_family)
    }

    let decoded = decode_html_entities_once(label);
    let (weight, italic, font_size_px, font_family) =
        parse_text_style_overrides(node_css_compiled_styles, node_css_styles);
    let mut style = text_style.clone();
    if let Some(px) = font_size_px {
        style.font_size = px;
    }
    if let Some(ff) = font_family {
        style.font_family = Some(ff);
    }
    style.font_weight = weight;

    let mut metrics =
        measurer.measure_wrapped(decoded.as_ref(), &style, Some(wrapping_width), wrap_mode);

    if italic && wrap_mode == WrapMode::HtmlLike {
        metrics.width +=
            crate::text::mermaid_default_italic_width_delta_px(decoded.as_ref(), &style);
    }

    if wrap_mode == WrapMode::HtmlLike {
        metrics.width += crate::text::mermaid_default_bold_width_delta_px(decoded.as_ref(), &style);
    }

    if wrap_mode == WrapMode::HtmlLike && wrapping_width.is_finite() && wrapping_width > 0.0 {
        // Mermaid HTML labels are effectively clamped by CSS `max-width`. Any additional width
        // adjustments (italic/bold deltas) must not exceed that wrapping width.
        metrics.width = metrics.width.min(wrapping_width);
    }

    if wrap_mode == WrapMode::HtmlLike {
        // Mermaid DOM measurements routinely land on a 1/64px lattice.
        metrics.width = crate::text::round_to_1_64_px(metrics.width);
        if wrapping_width.is_finite() && wrapping_width > 0.0 {
            metrics.width = metrics.width.min(wrapping_width);
        }
    }

    if wrap_mode == WrapMode::HtmlLike {
        let has_metrics_style = italic
            || style
                .font_weight
                .as_deref()
                .is_some_and(|s| !s.trim().is_empty())
            || font_size_px.is_some()
            || text_style
                .font_family
                .as_deref()
                .zip(style.font_family.as_deref())
                .is_some_and(|(a, b)| a.trim() != b.trim());
        if !has_metrics_style {
            let trimmed = decoded.as_ref().trim();
            if let Some(w) =
                crate::generated::state_text_overrides_11_12_2::lookup_state_node_label_width_px(
                    style.font_size,
                    trimmed,
                )
            {
                metrics.width = w;
            }
        }

        let trimmed = decoded.as_ref().trim();
        let bold = style
            .font_weight
            .as_deref()
            .is_some_and(|s| s.to_ascii_lowercase().contains("bold"));
        if let Some(w) =
            crate::generated::state_text_overrides_11_12_2::lookup_state_node_label_width_px_styled(
                style.font_size,
                trimmed,
                bold,
                italic,
            )
        {
            metrics.width = w;
        }
    }

    if wrap_mode == WrapMode::HtmlLike {
        let has_classdef_border_style = node_css_compiled_styles
            .iter()
            .any(|s| s.trim_start().to_ascii_lowercase().starts_with("border:"));

        // Mermaid@11.12.2 browser baselines show a surprising `getBoundingClientRect()` inflation
        // for `classDef`-styled border nodes: even a single-line `<p>` label can measure as `72px`
        // tall. Mirror that behavior here to avoid relying on string-keyed height overrides.
        if has_classdef_border_style && (style.font_size - 16.0).abs() <= 0.01 {
            let trimmed = decoded.as_ref().trim();
            let is_single_line = !trimmed.contains('\n')
                && !trimmed.to_ascii_lowercase().contains("<br")
                && !trimmed.is_empty();
            if is_single_line && (metrics.height - 24.0).abs() <= 0.01 {
                metrics.height = metrics.height.max(72.0);
            }
        }
    }
    (metrics.width.max(0.0), metrics.height.max(0.0))
}

fn title_label_metrics(
    label: &str,
    measurer: &dyn TextMeasurer,
    text_style: &TextStyle,
    wrap_mode: WrapMode,
) -> (f64, f64) {
    // Mermaid state diagram cluster titles use `createLabel(...)` (nowrap) rather than
    // `createText(...)` (width constrained).
    let decoded = decode_html_entities_once(label);
    let mut metrics = measurer.measure_wrapped(decoded.as_ref(), text_style, None, wrap_mode);

    if wrap_mode == WrapMode::HtmlLike {
        // Mermaid DOM measurements routinely land on a 1/64px lattice.
        metrics.width = crate::text::round_to_1_64_px(metrics.width);
    }

    (metrics.width.max(0.0), metrics.height.max(0.0))
}

fn extract_descendants(
    graph: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    id: &str,
    out: &mut Vec<String>,
) {
    let mut visited: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = graph
        .children(id)
        .iter()
        .rev()
        .map(|s| s.to_string())
        .collect();
    while let Some(node) = stack.pop() {
        if !visited.insert(node.clone()) {
            continue;
        }
        out.push(node.clone());
        let children = graph.children(&node);
        for child in children.iter().rev() {
            stack.push(child.to_string());
        }
    }
}

fn is_descendant(descendants: &HashMap<String, HashSet<String>>, id: &str, ancestor: &str) -> bool {
    descendants
        .get(ancestor)
        .is_some_and(|set| set.contains(id))
}

fn find_common_edges(
    graph: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    id1: &str,
    id2: &str,
) -> Vec<(String, String)> {
    let edges1: Vec<(String, String)> = graph
        .edge_keys()
        .into_iter()
        .filter(|e| e.v == id1 || e.w == id1)
        .map(|e| (e.v, e.w))
        .collect();
    let edges2: Vec<(String, String)> = graph
        .edge_keys()
        .into_iter()
        .filter(|e| e.v == id2 || e.w == id2)
        .map(|e| (e.v, e.w))
        .collect();

    let edges1_prim: Vec<(String, String)> = edges1
        .into_iter()
        .map(|(v, w)| {
            (
                if v == id1 { id2.to_string() } else { v },
                // Mermaid's `findCommonEdges(...)` has an asymmetry here: it maps the `w` side
                // back to `id1` rather than `id2` (Mermaid@11.12.2).
                if w == id1 { id1.to_string() } else { w },
            )
        })
        .collect();

    let mut out = Vec::new();
    for e1 in edges1_prim {
        if edges2.contains(&e1) {
            out.push(e1);
        }
    }
    out
}

fn find_non_cluster_child(
    graph: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    id: &str,
    cluster_id: &str,
) -> Option<String> {
    let children = graph.children(id);
    if children.is_empty() {
        return Some(id.to_string());
    }
    let mut reserve: Option<String> = None;
    let mut visited: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = children.iter().rev().map(|s| s.to_string()).collect();
    while let Some(node) = stack.pop() {
        if !visited.insert(node.clone()) {
            continue;
        }
        let children = graph.children(&node);
        if !children.is_empty() {
            for child in children.iter().rev() {
                stack.push(child.to_string());
            }
            continue;
        }
        let common_edges = find_common_edges(graph, cluster_id, &node);
        if !common_edges.is_empty() {
            reserve = Some(node);
        } else {
            return Some(node);
        }
    }
    reserve
}

fn prepare_graph(
    graph: Graph<NodeLabel, EdgeLabel, GraphLabel>,
    cluster_dir: &impl Fn(&str) -> Option<String>,
    root_cluster_id: Option<String>,
) -> Result<PreparedGraph> {
    let mut root = PreparedGraph {
        graph,
        extracted: BTreeMap::new(),
        root_cluster_id,
    };

    let mut stack: Vec<Vec<String>> = vec![Vec::new()];
    while let Some(path) = stack.pop() {
        let prepared = prepared_graph_at_path_mut(&mut root, &path)?;
        let mut child_ids = prepare_graph_one_level(prepared, cluster_dir)?;
        child_ids.reverse();
        for child_id in child_ids {
            let mut child_path = path.clone();
            child_path.push(child_id);
            stack.push(child_path);
        }
    }

    Ok(root)
}

fn prepared_graph_at_path_mut<'a>(
    root: &'a mut PreparedGraph,
    path: &[String],
) -> Result<&'a mut PreparedGraph> {
    let mut current = root;
    for id in path {
        current = current
            .extracted
            .get_mut(id)
            .ok_or_else(|| Error::InvalidModel {
                message: format!("missing prepared cluster graph: {id}"),
            })?;
    }
    Ok(current)
}

fn prepare_graph_one_level(
    prepared: &mut PreparedGraph,
    cluster_dir: &impl Fn(&str) -> Option<String>,
) -> Result<Vec<String>> {
    let graph = &mut prepared.graph;
    let cluster_ids: Vec<String> = graph
        .node_ids()
        .into_iter()
        .filter(|id| !graph.children(id).is_empty())
        .collect();

    let mut descendants: HashMap<String, HashSet<String>> = HashMap::new();
    let mut external: HashMap<String, bool> =
        cluster_ids.iter().map(|id| (id.clone(), false)).collect();

    let edge_keys = graph.edge_keys();
    if !edge_keys.is_empty() {
        for id in &cluster_ids {
            let mut vec: Vec<String> = Vec::new();
            extract_descendants(graph, id, &mut vec);
            descendants.insert(id.clone(), vec.into_iter().collect());
        }

        for id in &cluster_ids {
            for e in &edge_keys {
                let d1 = is_descendant(&descendants, &e.v, id);
                let d2 = is_descendant(&descendants, &e.w, id);
                if d1 ^ d2 {
                    external.insert(id.clone(), true);
                    break;
                }
            }
        }
    }

    let mut anchor: HashMap<String, String> = HashMap::new();
    if !edge_keys.is_empty() {
        for id in &cluster_ids {
            let Some(a) = find_non_cluster_child(graph, id, id) else {
                continue;
            };
            anchor.insert(id.clone(), a);
        }
    }

    // Adjust edges that touch cluster ids by rewriting them to anchor nodes.
    //
    // Match Mermaid `adjustClustersAndEdges(graph)`: edges incident on cluster nodes are removed
    // and re-inserted even when their endpoints do not change. This affects edge insertion order
    // and can change deterministic tie-breaking in Dagre's acyclic pass.
    for key in edge_keys {
        let mut from_cluster: Option<String> = None;
        let mut to_cluster: Option<String> = None;
        let mut v = key.v.clone();
        let mut w = key.w.clone();

        let touches_cluster =
            cluster_ids.iter().any(|c| c == &v) || cluster_ids.iter().any(|c| c == &w);
        if !touches_cluster {
            continue;
        }

        if cluster_ids.iter().any(|c| c == &v) && *external.get(&v).unwrap_or(&false) {
            if let Some(a) = anchor.get(&v) {
                from_cluster = Some(v.clone());
                v = a.clone();
            }
        }
        if cluster_ids.iter().any(|c| c == &w) && *external.get(&w).unwrap_or(&false) {
            if let Some(a) = anchor.get(&w) {
                to_cluster = Some(w.clone());
                w = a.clone();
            }
        }

        let Some(old_label) = graph.edge_by_key(&key).cloned() else {
            continue;
        };
        let _ = graph.remove_edge_key(&key);

        let mut new_label = old_label;
        if let Some(fc) = from_cluster.as_deref() {
            set_extras_string(&mut new_label.extras, "fromCluster", fc);
        }
        if let Some(tc) = to_cluster.as_deref() {
            set_extras_string(&mut new_label.extras, "toCluster", tc);
        }
        graph.set_edge_named(v, w, key.name.clone(), Some(new_label));
    }

    // Extract clusters without external connections into subgraphs for nested layout.
    //
    // Mermaid@11.12.2 `dagre-wrapper` extractor does not require clusters to be root-level. It
    // extracts any cluster node that has children and no external connections, then relies on the
    // nested render pass to (optionally) inject the cluster root node back into the subgraph
    // for sizing/padding.
    let mut candidate_roots: Vec<String> = Vec::new();
    for id in graph.node_ids() {
        if graph.children(&id).is_empty() {
            continue;
        }
        if *external.get(&id).unwrap_or(&false) {
            continue;
        }
        candidate_roots.push(id);
    }
    fn cluster_depth(g: &Graph<NodeLabel, EdgeLabel, GraphLabel>, id: &str) -> usize {
        let mut depth = 0usize;
        let mut cur = id;
        while let Some(parent) = g.parent(cur) {
            depth += 1;
            cur = parent;
            if depth > 128 {
                break;
            }
        }
        depth
    }
    candidate_roots.sort_by(|a, b| {
        cluster_depth(&graph, a)
            .cmp(&cluster_depth(&graph, b))
            .then(a.cmp(b))
    });

    let mut child_ids = Vec::new();
    for cluster_id in candidate_roots {
        if !graph.has_node(&cluster_id) || graph.children(&cluster_id).is_empty() {
            continue;
        }
        let parent_dir = graph.graph().rankdir;
        let requested = cluster_dir(&cluster_id).map(|d| rank_dir_from(&d));
        // Mermaid keeps nested state graphs in the same rank direction by default. Only apply
        // a different direction when explicitly requested by the cluster itself.
        let dir = requested.unwrap_or(parent_dir);
        let nodesep = graph.graph().nodesep;
        let ranksep = graph.graph().ranksep + 25.0;
        let marginx = graph.graph().marginx;
        let marginy = graph.graph().marginy;

        let mut subgraph = extract_cluster_graph(&cluster_id, graph)?;
        subgraph.graph_mut().rankdir = dir;
        subgraph.graph_mut().nodesep = nodesep;
        subgraph.graph_mut().ranksep = ranksep;
        subgraph.graph_mut().marginx = marginx;
        subgraph.graph_mut().marginy = marginy;

        prepared.extracted.insert(
            cluster_id.clone(),
            PreparedGraph {
                graph: subgraph,
                extracted: BTreeMap::new(),
                root_cluster_id: Some(cluster_id.clone()),
            },
        );
        child_ids.push(cluster_id);
    }

    Ok(child_ids)
}

fn extract_cluster_graph(
    cluster_id: &str,
    graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
) -> Result<Graph<NodeLabel, EdgeLabel, GraphLabel>> {
    if graph.children(cluster_id).is_empty() {
        return Err(Error::InvalidModel {
            message: format!("cluster has no children: {cluster_id}"),
        });
    }

    // Mermaid's cluster extractor uses a somewhat surprising copy algorithm:
    // - It walks leaf nodes in a deterministic-but-mutation-sensitive order.
    // - For each leaf, it calls `graph.edges(node)` (Graphlib ignores the argument and returns
    //   *all* edges), inserting edges opportunistically while the source graph is being mutated.
    //
    // This affects edge insertion order in the extracted graph and can change Dagre's cycle
    // breaking tie-breakers (notably for cyclic-special self-loop expansions). Mirror that
    // behavior for parity.
    let mut descendants: Vec<String> = Vec::new();
    extract_descendants(graph, cluster_id, &mut descendants);
    let descendants_set: HashSet<String> = descendants.iter().cloned().collect();

    fn edge_in_cluster(ek: &EdgeKey, root_id: &str, descendants: &HashSet<String>) -> bool {
        if ek.v == root_id || ek.w == root_id {
            return false;
        }
        descendants.contains(&ek.v) || descendants.contains(&ek.w)
    }

    let mut sub = Graph::<NodeLabel, EdgeLabel, GraphLabel>::new(GraphOptions {
        directed: true,
        multigraph: true,
        compound: true,
    });

    struct CopyFrame {
        current_cluster_id: String,
        nodes: Vec<String>,
        next_index: usize,
    }

    let root_nodes: Vec<String> = graph
        .children(cluster_id)
        .iter()
        .map(|s| s.to_string())
        .collect();
    let mut stack = vec![CopyFrame {
        current_cluster_id: cluster_id.to_string(),
        nodes: root_nodes,
        next_index: 0,
    }];

    while !stack.is_empty() {
        let frame_idx = stack.len() - 1;
        let Some((node, current_cluster_id)) = ({
            let frame = &mut stack[frame_idx];
            if frame.next_index >= frame.nodes.len() {
                None
            } else {
                let node = frame.nodes[frame.next_index].clone();
                frame.next_index += 1;
                Some((node, frame.current_cluster_id.clone()))
            }
        }) else {
            stack.pop();
            continue;
        };

        if !graph.has_node(&node) {
            continue;
        }

        if !graph.children(&node).is_empty() {
            let mut child_nodes: Vec<String> = graph
                .children(&node)
                .iter()
                .map(|s| s.to_string())
                .collect();
            if node != cluster_id {
                child_nodes.push(node.clone());
            }
            stack.push(CopyFrame {
                current_cluster_id: node,
                nodes: child_nodes,
                next_index: 0,
            });
            continue;
        }

        let data = graph.node(&node).cloned().unwrap_or_default();
        sub.set_node(node.clone(), data);

        if let Some(parent) = graph.parent(&node) {
            if parent != cluster_id {
                sub.set_parent(node.clone(), parent.to_string());
            }
        }
        if current_cluster_id != cluster_id && node != current_cluster_id {
            sub.set_parent(node.clone(), current_cluster_id);
        }

        // NOTE: Mermaid uses `graph.edges(node)` but Graphlib ignores the argument and
        // returns all edges. Mirror that by iterating the full edge set each time.
        let edge_keys = graph.edge_keys();
        for ek in edge_keys {
            if !edge_in_cluster(&ek, cluster_id, &descendants_set) {
                continue;
            }
            let Some(label) = graph.edge_by_key(&ek).cloned() else {
                continue;
            };
            sub.set_edge_named(ek.v, ek.w, ek.name, Some(label));
        }

        let _ = graph.remove_node(&node);
    }

    Ok(sub)
}

/// Debug-only helper: extracts a cluster subgraph the same way `prepare_graph(...)` does.
#[doc(hidden)]
pub fn debug_extract_state_diagram_v2_cluster_graph(
    graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    cluster_id: &str,
) -> Result<Graph<NodeLabel, EdgeLabel, GraphLabel>> {
    extract_cluster_graph(cluster_id, graph)
}

fn inject_root_cluster_node(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>, root_id: &str) {
    if !g.has_node(root_id) {
        g.set_node(
            root_id.to_string(),
            NodeLabel {
                width: 1.0,
                height: 1.0,
                ..Default::default()
            },
        );
    }

    let node_ids: Vec<String> = g.node_ids().into_iter().map(|s| s.to_string()).collect();
    for v in node_ids {
        if v == root_id {
            continue;
        }
        if g.parent(&v).is_none() {
            g.set_parent(v, root_id.to_string());
        }
    }
}

fn layout_prepared(prepared: &mut PreparedGraph) -> Result<(LayoutFragments, Rect)> {
    let mut stack: Vec<(Vec<String>, bool)> = vec![(Vec::new(), false)];
    let mut completed: HashMap<Vec<String>, (LayoutFragments, Rect)> = HashMap::new();

    while let Some((path, visited)) = stack.pop() {
        let child_ids: Vec<String> = {
            let node = prepared_graph_at_path_mut(prepared, &path)?;
            node.extracted.keys().cloned().collect()
        };

        if visited {
            let mut extracted_fragments = HashMap::new();
            for child_id in child_ids {
                let mut child_path = path.clone();
                child_path.push(child_id.clone());
                let Some(result) = completed.remove(&child_path) else {
                    return Err(Error::InvalidModel {
                        message: format!("missing prepared cluster layout: {child_id}"),
                    });
                };
                extracted_fragments.insert(child_id, result);
            }

            let node = prepared_graph_at_path_mut(prepared, &path)?;
            let result = layout_prepared_node(node, extracted_fragments)?;
            completed.insert(path, result);
            continue;
        }

        stack.push((path.clone(), true));
        for child_id in child_ids.into_iter().rev() {
            let mut child_path = path.clone();
            child_path.push(child_id);
            stack.push((child_path, false));
        }
    }

    completed
        .remove(&Vec::new())
        .ok_or_else(|| Error::InvalidModel {
            message: "missing prepared root layout".to_string(),
        })
}

fn layout_prepared_node(
    prepared: &mut PreparedGraph,
    extracted_fragments: HashMap<String, (LayoutFragments, Rect)>,
) -> Result<(LayoutFragments, Rect)> {
    if let Some(root_id) = prepared.root_cluster_id.clone() {
        // Mermaid's dagre-wrapper nested render pass injects the parent cluster node into the
        // extracted graph and parents top-level nodes to it. This is required for Dagre’s
        // compound border nodes to yield the same “outer padding” used by upstream when sizing
        // clusterNode placeholders via `updateNodeBounds(...)`.
        inject_root_cluster_node(&mut prepared.graph, &root_id);
    }

    let mut fragments = LayoutFragments {
        nodes: HashMap::new(),
        edge_segments: Vec::new(),
    };

    for (id, (_sub_frag, bounds)) in &extracted_fragments {
        let Some(n) = prepared.graph.node_mut(id) else {
            return Err(Error::InvalidModel {
                message: format!("missing cluster placeholder node: {id}"),
            });
        };
        n.width = bounds.width().max(1.0);
        n.height = bounds.height().max(1.0);
    }

    // State diagrams use Mermaid's unified dagre renderer, so we want the more complete
    // Dagre-ish pipeline here (edge label proxies, BK positioning, etc).
    dugong::layout_dagreish(&mut prepared.graph);

    for id in prepared.graph.node_ids() {
        let Some(n) = prepared.graph.node(&id) else {
            continue;
        };
        fragments.nodes.insert(
            id.clone(),
            LayoutNode {
                id: id.clone(),
                x: n.x.unwrap_or(0.0),
                y: n.y.unwrap_or(0.0),
                width: n.width,
                height: n.height,
                is_cluster: false,
                label_width: None,
                label_height: None,
            },
        );
    }

    for key in prepared.graph.edge_keys() {
        let Some(e) = prepared.graph.edge_by_key(&key) else {
            continue;
        };
        let original_id = get_extras_string(&e.extras, "originalId").unwrap_or_else(|| {
            key.name
                .clone()
                .unwrap_or_else(|| format!("edge:{}:{}", key.v, key.w))
        });
        let segment = e
            .extras
            .get("segment")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;
        let original_from =
            get_extras_string(&e.extras, "originalFrom").unwrap_or_else(|| key.v.clone());
        let original_to =
            get_extras_string(&e.extras, "originalTo").unwrap_or_else(|| key.w.clone());
        let from_cluster = get_extras_string(&e.extras, "fromCluster");
        let to_cluster = get_extras_string(&e.extras, "toCluster");

        // Mermaid's dagre wrapper emits "edgeLabel" placeholder groups even when the visible
        // label is empty. Dagre still assigns an `(x, y)` label position for those edges, and the
        // placeholders can affect the root `svg.getBBox()` (and therefore `viewBox/max-width`).
        //
        // Preserve the label center even when `width/height` are 0 so downstream renderers can
        // place the placeholders like upstream.
        let label = match (e.x, e.y) {
            (Some(x), Some(y)) => Some(LayoutLabel {
                x,
                y,
                width: e.width.max(0.0),
                height: e.height.max(0.0),
            }),
            _ => None,
        };

        let points = e
            .points
            .iter()
            .map(|p| LayoutPoint { x: p.x, y: p.y })
            .collect::<Vec<_>>();

        fragments.edge_segments.push(EdgeSegment {
            original_id,
            segment,
            original_from,
            original_to,
            from_cluster,
            to_cluster,
            points,
            label,
        });
    }

    // Merge extracted fragments into this graph, translating them by the cluster placeholder
    // position.
    for (cluster_id, (mut sub_frag, sub_bounds)) in extracted_fragments {
        let Some(cluster_node) = fragments.nodes.get(&cluster_id).cloned() else {
            return Err(Error::InvalidModel {
                message: format!("missing cluster placeholder layout: {cluster_id}"),
            });
        };
        let (sub_cx, sub_cy) = sub_bounds.center();
        let dx = cluster_node.x - sub_cx;
        let dy = cluster_node.y - sub_cy;

        for n in sub_frag.nodes.values_mut() {
            n.x += dx;
            n.y += dy;
        }
        for seg in &mut sub_frag.edge_segments {
            for p in &mut seg.points {
                p.x += dx;
                p.y += dy;
            }
            if let Some(l) = seg.label.as_mut() {
                l.x += dx;
                l.y += dy;
            }
        }

        fragments.nodes.extend(sub_frag.nodes);
        fragments.edge_segments.extend(sub_frag.edge_segments);
    }

    let mut points: Vec<(f64, f64)> = Vec::new();
    for n in fragments.nodes.values() {
        let r = Rect::from_center(n.x, n.y, n.width, n.height);
        points.push((r.min_x(), r.min_y()));
        points.push((r.max_x(), r.max_y()));
    }
    for e in &fragments.edge_segments {
        for p in &e.points {
            points.push((p.x, p.y));
        }
        if let Some(l) = &e.label {
            let r = Rect::from_center(l.x, l.y, l.width, l.height);
            points.push((r.min_x(), r.min_y()));
            points.push((r.max_x(), r.max_y()));
        }
    }
    let bounds = Bounds::from_points(points)
        .map(|b| Rect::from_min_max(b.min_x, b.min_y, b.max_x, b.max_y))
        .unwrap_or_else(|| Rect::from_min_max(0.0, 0.0, 0.0, 0.0));

    Ok((fragments, bounds))
}

fn merge_edge_segments(mut segments: Vec<EdgeSegment>) -> Vec<LayoutEdge> {
    segments.sort_by(|a, b| {
        a.original_id
            .cmp(&b.original_id)
            .then_with(|| a.segment.cmp(&b.segment))
    });

    let mut out: Vec<LayoutEdge> = Vec::new();
    let mut i = 0usize;
    while i < segments.len() {
        let id = segments[i].original_id.clone();
        let from = segments[i].original_from.clone();
        let to = segments[i].original_to.clone();

        let mut from_cluster = segments[i].from_cluster.clone();
        let mut to_cluster = segments[i].to_cluster.clone();

        let mut points: Vec<LayoutPoint> = Vec::new();
        let mut label: Option<LayoutLabel> = None;

        while i < segments.len() && segments[i].original_id == id {
            let seg = &segments[i];
            if from_cluster.is_none() {
                from_cluster = seg.from_cluster.clone();
            }
            if to_cluster.is_none() {
                to_cluster = seg.to_cluster.clone();
            }
            if label.is_none() {
                label = seg.label.clone();
            }

            for (idx, p) in seg.points.iter().enumerate() {
                if points.is_empty() {
                    points.push(p.clone());
                    continue;
                }
                if idx == 0
                    && points.last().is_some_and(|last| {
                        (last.x - p.x).abs() < 1e-9 && (last.y - p.y).abs() < 1e-9
                    })
                {
                    continue;
                }
                points.push(p.clone());
            }

            i += 1;
        }

        out.push(LayoutEdge {
            id,
            from,
            to,
            from_cluster,
            to_cluster,
            points,
            label,
            start_label_left: None,
            start_label_right: None,
            end_label_left: None,
            end_label_right: None,
            start_marker: None,
            end_marker: None,
            stroke_dasharray: None,
        });
    }

    out
}

pub fn layout_state_diagram_v2(
    semantic: &Value,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<StateDiagramV2Layout> {
    let model: StateDiagramModel = crate::json::from_value_ref(semantic)?;
    layout_state_diagram_v2_inner(&model, effective_config, measurer)
}

pub fn layout_state_diagram_v2_typed(
    model: &StateDiagramModel,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<StateDiagramV2Layout> {
    layout_state_diagram_v2_inner(model, effective_config, measurer)
}

fn state_hidden_prefixes(model: &StateDiagramModel) -> Vec<String> {
    let mut hidden_prefixes: Vec<String> = Vec::new();
    for (id, st) in &model.states {
        let Some(note) = st.note.as_ref() else {
            continue;
        };
        if note.text.trim().is_empty() {
            continue;
        }
        if note.position.is_none() {
            hidden_prefixes.push(id.clone());
        }
    }
    hidden_prefixes
}

fn state_is_hidden_id(prefixes: &[String], id: &str) -> bool {
    prefixes.iter().any(|p| {
        if id == p {
            return true;
        }
        id.strip_prefix(p)
            .is_some_and(|rest| rest.starts_with("----"))
    })
}

fn dagre_id_for_node(n: &StateNode) -> String {
    if n.dom_id.trim().is_empty() {
        n.id.clone()
    } else {
        n.dom_id.clone()
    }
}

fn build_state_diagram_v2_dagre_input(
    model: &StateDiagramModel,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<StateDagreInput> {
    // Mermaid accepts some historical "floating note" syntaxes in the parser but does not render them.
    // Keep them in the semantic model/snapshots, but exclude them from layout so they do not shift
    // visible nodes/edges (and therefore do not affect root viewBox/max-width parity).
    let hidden_prefixes = state_hidden_prefixes(model);

    let mut dagre_id_by_semantic_id: HashMap<String, String> = HashMap::new();
    let mut dir_by_dagre_id: HashMap<String, Option<String>> = HashMap::new();
    for n in &model.nodes {
        let dagre_id = dagre_id_for_node(n);
        dagre_id_by_semantic_id.insert(n.id.clone(), dagre_id.clone());
        dir_by_dagre_id.insert(dagre_id, n.dir.as_ref().map(|s| normalize_dir(s)));
    }

    let diagram_dir = rank_dir_from(&model.direction);
    let nodesep = config_f64(effective_config, &["state", "nodeSpacing"]).unwrap_or(50.0);
    let ranksep = config_f64(effective_config, &["state", "rankSpacing"]).unwrap_or(50.0);
    let html_labels = config_bool(effective_config, &["flowchart", "htmlLabels"]).unwrap_or(true);
    let wrap_mode = if html_labels {
        WrapMode::HtmlLike
    } else {
        WrapMode::SvgLike
    };
    let wrapping_width = crate::state::state_html_label_wrapping_width(effective_config);
    let state_padding = config_f64(effective_config, &["state", "padding"]).unwrap_or(8.0);
    let text_style = state_text_style(effective_config);

    let mut graph = Graph::<NodeLabel, EdgeLabel, GraphLabel>::new(GraphOptions {
        directed: true,
        multigraph: true,
        compound: true,
    });
    graph.set_graph(GraphLabel {
        rankdir: diagram_dir,
        nodesep,
        ranksep,
        marginx: 8.0,
        marginy: 8.0,
        // Mermaid `@11.12.2` dagre-wrapper renderer does not set `ranker`, so Dagre defaults to
        // `network-simplex`.
        ..Default::default()
    });

    // Pre-size nodes (leaf nodes only). Cluster nodes start with a tiny placeholder size.
    for n in &model.nodes {
        if state_is_hidden_id(&hidden_prefixes, n.id.as_str()) {
            continue;
        }
        let dagre_id = dagre_id_by_semantic_id
            .get(&n.id)
            .cloned()
            .unwrap_or_else(|| n.id.clone());
        if state_node_is_effective_group(n) {
            graph.set_node(
                dagre_id,
                NodeLabel {
                    width: 1.0,
                    height: 1.0,
                    ..Default::default()
                },
            );
            continue;
        }

        let padding = n.padding.unwrap_or(state_padding).max(0.0);
        let label_text = n
            .label
            .as_ref()
            .map(value_to_label_text)
            .unwrap_or_else(|| n.id.clone());

        let (w, h) = match n.shape.as_str() {
            "stateStart" => (14.0, 14.0),
            "stateEnd" => (STATE_END_NODE_DAGRE_WIDTH_PX_11_12_2, 14.0),
            "choice" => (28.0, 28.0),
            "fork" | "join" => {
                let (mut width, mut height) = if matches!(diagram_dir, RankDir::LR | RankDir::RL) {
                    (10.0, 70.0)
                } else {
                    (70.0, 10.0)
                };
                width += state_padding / 2.0;
                height += state_padding / 2.0;
                (width, height)
            }
            "note" => {
                let (mut tw, th) = node_label_metrics(
                    &label_text,
                    wrapping_width,
                    &n.css_compiled_styles,
                    &n.css_styles,
                    measurer,
                    &text_style,
                    wrap_mode,
                );
                if wrap_mode == WrapMode::HtmlLike {
                    let decoded = decode_html_entities_once(&label_text);
                    if let Some(w) = state_text_overrides::lookup_state_note_label_width_px(
                        text_style.font_size,
                        decoded.as_ref().trim(),
                    ) {
                        tw = w;
                    }
                }
                (tw + padding * 2.0, th + padding * 2.0)
            }
            "rectWithTitle" => {
                let desc = n
                    .description
                    .as_ref()
                    .map(|v| v.join("\n"))
                    .unwrap_or_default();
                // Mermaid's `rectWithTitle` nodes render two HTML `<span>` labels with
                // `display:inline-block; padding-right:1px; white-space:nowrap;` and no explicit
                // `line-height`. Empirically, their measured height matches SVG `getBBox()` height
                // (1.1875em at 16px -> 19px), not the 1.5em HTML `<p>` line-height used by most
                // other state labels.
                let (title_w, title_h) =
                    title_label_metrics(&label_text, measurer, &text_style, WrapMode::SvgLike);
                let (desc_w, desc_h) =
                    title_label_metrics(&desc, measurer, &text_style, WrapMode::SvgLike);

                // Mirror `padding-right: 1px` in upstream HTML.
                let title_w = state_text_overrides::rect_with_title_span_effective_width_px(
                    text_style.font_size,
                    label_text.trim(),
                    title_w,
                );
                let desc_w = state_text_overrides::rect_with_title_span_effective_width_px(
                    text_style.font_size,
                    desc.trim(),
                    desc_w,
                );
                let title_h = state_text_overrides::rect_with_title_span_effective_height_px(
                    text_style.font_size,
                    label_text.trim(),
                    title_h,
                );
                let desc_h = state_text_overrides::rect_with_title_span_effective_height_px(
                    text_style.font_size,
                    desc.trim(),
                    desc_h,
                );

                let inner_w = title_w.max(desc_w);
                let top_pad = state_text_overrides::state_rect_with_title_top_pad_px(padding);
                let bottom_pad = state_text_overrides::state_rect_with_title_bottom_pad_px(padding);
                let gap = state_text_overrides::state_rect_with_title_gap_px(padding);
                let h = top_pad + title_h.max(0.0) + gap + desc_h.max(0.0) + bottom_pad;
                let w = inner_w + padding;
                (w.max(1.0), h.max(1.0))
            }
            "rect" => {
                let (tw, th) = node_label_metrics(
                    &label_text,
                    wrapping_width,
                    &n.css_compiled_styles,
                    &n.css_styles,
                    measurer,
                    &text_style,
                    wrap_mode,
                );
                // Mermaid converts `rect` into `roundedRect` when rx/ry is set.
                let has_rounding = n.rx.unwrap_or(0.0) > 0.0 && n.ry.unwrap_or(0.0) > 0.0;
                let pad_x = if has_rounding { padding } else { padding * 2.0 };
                let pad_y = padding;
                (tw + pad_x * 2.0, th + pad_y * 2.0)
            }
            other => {
                return Err(Error::InvalidModel {
                    message: format!("unsupported state node shape: {other}"),
                });
            }
        };

        graph.set_node(
            dagre_id,
            NodeLabel {
                width: w.max(1.0),
                height: h.max(1.0),
                ..Default::default()
            },
        );
    }

    if graph.options().compound {
        for n in &model.nodes {
            if state_is_hidden_id(&hidden_prefixes, n.id.as_str()) {
                continue;
            }
            if let Some(p) = n.parent_id.as_ref() {
                if state_is_hidden_id(&hidden_prefixes, p.as_str()) {
                    continue;
                }
                let child_id = dagre_id_by_semantic_id
                    .get(&n.id)
                    .cloned()
                    .unwrap_or_else(|| n.id.clone());
                let parent_id = dagre_id_by_semantic_id
                    .get(p)
                    .cloned()
                    .unwrap_or_else(|| p.clone());
                graph.set_parent(child_id, parent_id);
            }
        }
    }

    // Add edges. For self-loops, split into 3 edges with 2 tiny dummy nodes (Mermaid wrapper
    // behavior).
    for e in &model.edges {
        if state_is_hidden_id(&hidden_prefixes, e.id.as_str())
            || state_is_hidden_id(&hidden_prefixes, e.start.as_str())
            || state_is_hidden_id(&hidden_prefixes, e.end.as_str())
        {
            continue;
        }
        let (lw, lh) = edge_label_metrics(&e.label, measurer, &text_style, wrap_mode);
        let mut base = EdgeLabel {
            width: lw,
            height: lh,
            labelpos: LabelPos::C,
            labeloffset: 10.0,
            minlen: 1,
            weight: 1.0,
            ..Default::default()
        };
        set_extras_string(&mut base.extras, "originalId", &e.id);
        set_extras_string(&mut base.extras, "originalFrom", &e.start);
        set_extras_string(&mut base.extras, "originalTo", &e.end);
        set_extras_i32(&mut base.extras, "segment", 0);

        if e.start != e.end {
            let start_id = dagre_id_by_semantic_id
                .get(&e.start)
                .cloned()
                .unwrap_or_else(|| e.start.clone());
            let end_id = dagre_id_by_semantic_id
                .get(&e.end)
                .cloned()
                .unwrap_or_else(|| e.end.clone());
            graph.set_edge_named(start_id, end_id, Some(e.id.clone()), Some(base));
            continue;
        }

        let node_id = e.start.clone();
        let node_dagre_id = dagre_id_by_semantic_id
            .get(&node_id)
            .cloned()
            .unwrap_or_else(|| node_id.clone());
        let id1 = format!("{node_id}-cyclic-special-1");
        let idm = format!("{node_id}-cyclic-special-mid");
        let id2 = format!("{node_id}-cyclic-special-2");
        // Mermaid uses fixed self-loop helper node ids (`${nodeId}---${nodeId}---{1|2}`), not
        // per-edge ids. This means multiple self-loop transitions on the same node collide in the
        // layout graph; match upstream behavior for parity.
        let special1 = format!("{node_id}---{node_id}---1");
        let special2 = format!("{node_id}---{node_id}---2");

        graph.set_node(
            special1.clone(),
            NodeLabel {
                // Mermaid's renderer initially seeds these dummy nodes with `10x10`, but then
                // `labelRect` renders them as `0.1x0.1` and `updateNodeBounds(...)` overwrites
                // `node.width/height` *before* Dagre layout runs.
                //
                // Mirror the effective size seen by Dagre to keep cyclic self-loop layouts and
                // root viewBox parity stable.
                width: 0.1,
                height: 0.1,
                ..Default::default()
            },
        );
        graph.set_node(
            special2.clone(),
            NodeLabel {
                width: 0.1,
                height: 0.1,
                ..Default::default()
            },
        );
        if let Some(parent) = graph.parent(&node_dagre_id).map(|s| s.to_string()) {
            graph.set_parent(special1.clone(), parent.clone());
            graph.set_parent(special2.clone(), parent);
        }

        let mut edge1 = base.clone();
        edge1.width = 0.0;
        edge1.height = 0.0;
        set_extras_i32(&mut edge1.extras, "segment", 0);
        set_extras_string(&mut edge1.extras, "originalId", &id1);

        let mut edge_mid = base.clone();
        set_extras_i32(&mut edge_mid.extras, "segment", 1);
        set_extras_string(&mut edge_mid.extras, "originalId", &idm);

        let mut edge2 = base.clone();
        edge2.width = 0.0;
        edge2.height = 0.0;
        set_extras_i32(&mut edge2.extras, "segment", 2);
        set_extras_string(&mut edge2.extras, "originalId", &id2);

        // Mermaid uses different edge *names* (graphlib multigraph keys) from the edge `.id`
        // property for cyclic-special helper edges. This impacts edge iteration order and can
        // affect Dagre's cycle-breaking tie-breakers. Match Mermaid@11.12.2 exactly, including
        // the upstream typo in `-cyc<lic-special-2`.
        let name1 = format!("{node_id}-cyclic-special-0");
        let name_mid = format!("{node_id}-cyclic-special-1");
        let name2 = format!("{node_id}-cyc<lic-special-2");

        graph.set_edge_named(
            node_dagre_id.clone(),
            special1.clone(),
            Some(name1),
            Some(edge1),
        );
        graph.set_edge_named(special1, special2.clone(), Some(name_mid), Some(edge_mid));
        graph.set_edge_named(special2, node_dagre_id, Some(name2), Some(edge2));
    }

    Ok(StateDagreInput {
        graph,
        hidden_prefixes,
        dagre_id_by_semantic_id,
        dir_by_dagre_id,
        text_style,
        wrap_mode,
        html_labels,
    })
}

fn layout_state_diagram_v2_inner(
    model: &StateDiagramModel,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<StateDiagramV2Layout> {
    validate_state_parent_cycles(model)?;
    let StateDagreInput {
        graph,
        hidden_prefixes,
        dagre_id_by_semantic_id,
        dir_by_dagre_id,
        text_style,
        wrap_mode,
        html_labels,
    } = build_state_diagram_v2_dagre_input(model, effective_config, measurer)?;

    let cluster_dir =
        |id: &str| -> Option<String> { dir_by_dagre_id.get(id).and_then(|v| v.clone()) };

    let mut prepared = prepare_graph(graph, &cluster_dir, None)?;
    let (fragments, _layout_bounds) = layout_prepared(&mut prepared)?;

    let semantic_ids: HashSet<&str> = model
        .nodes
        .iter()
        .filter(|n| !state_is_hidden_id(&hidden_prefixes, n.id.as_str()))
        .map(|n| n.id.as_str())
        .collect();

    // Build output nodes from semantic nodes only.
    let mut out_nodes: Vec<LayoutNode> = Vec::new();
    for n in &model.nodes {
        if state_is_hidden_id(&hidden_prefixes, n.id.as_str()) {
            continue;
        }
        let dagre_id = dagre_id_by_semantic_id
            .get(&n.id)
            .map(|s| s.as_str())
            .unwrap_or(n.id.as_str());
        let Some(pos) = fragments.nodes.get(dagre_id) else {
            return Err(Error::InvalidModel {
                message: format!("missing positioned node: {}", n.id),
            });
        };

        if !state_node_is_effective_group(n) {
            out_nodes.push(LayoutNode {
                id: n.id.clone(),
                x: pos.x,
                y: pos.y,
                width: pos.width,
                height: pos.height,
                is_cluster: false,
                label_width: None,
                label_height: None,
            });
        }
    }

    // Preserve Mermaid's hidden self-loop helper nodes (`${nodeId}---${nodeId}---{1|2}`).
    //
    // These nodes are not part of the semantic model and are not rendered as visible nodes, but
    // Mermaid's SVG output uses their positioned bounding boxes to place `0.1 x 0.1` placeholder
    // rects which can affect `svg.getBBox()` and therefore the root `viewBox/max-width`.
    let mut helper_ids: HashSet<String> = HashSet::new();
    for e in &model.edges {
        if state_is_hidden_id(&hidden_prefixes, e.id.as_str())
            || state_is_hidden_id(&hidden_prefixes, e.start.as_str())
            || state_is_hidden_id(&hidden_prefixes, e.end.as_str())
        {
            continue;
        }
        if e.start != e.end {
            continue;
        }
        let node_id = e.start.as_str();
        helper_ids.insert(format!("{node_id}---{node_id}---1"));
        helper_ids.insert(format!("{node_id}---{node_id}---2"));
    }
    for id in helper_ids {
        let Some(pos) = fragments.nodes.get(&id) else {
            continue;
        };
        out_nodes.push(LayoutNode {
            id,
            x: pos.x,
            y: pos.y,
            width: pos.width,
            height: pos.height,
            is_cluster: false,
            label_width: None,
            label_height: None,
        });
    }

    let mut clusters: Vec<LayoutCluster> = Vec::new();
    for n in &model.nodes {
        if state_is_hidden_id(&hidden_prefixes, n.id.as_str()) {
            continue;
        }
        if !state_node_is_effective_group(n) {
            continue;
        }
        let dagre_id = dagre_id_by_semantic_id
            .get(&n.id)
            .map(|s| s.as_str())
            .unwrap_or(n.id.as_str());
        let Some(pos) = fragments.nodes.get(dagre_id) else {
            return Err(Error::InvalidModel {
                message: format!("missing positioned cluster node: {}", n.id),
            });
        };

        let mut title = n
            .label
            .as_ref()
            .map(value_to_label_text)
            .unwrap_or_default();
        if title.trim().is_empty() {
            title = n.id.clone();
        }
        let pad = n.padding.unwrap_or(8.0).max(0.0);
        let (tw, th) = if title.trim().is_empty() {
            (0.0, 0.0)
        } else {
            title_label_metrics(&title, measurer, &text_style, wrap_mode)
        };

        // Mermaid expands cluster width to ensure the title fits, but does not re-run Dagre after
        // that adjustment (so child node positions remain unchanged).
        let min_cluster_width = if title.trim().is_empty() {
            0.0
        } else {
            (tw + pad).max(0.0)
        };
        let rect = Rect::from_center(pos.x, pos.y, pos.width.max(min_cluster_width), pos.height);
        let (cx, cy) = rect.center();

        let title_top_adjust = if html_labels { 0.0 } else { 3.0 };
        let title_label = LayoutLabel {
            x: cx,
            y: rect.min_y() + 1.0 - title_top_adjust + th / 2.0,
            width: tw,
            height: th,
        };

        let diff = match n.shape.as_str() {
            "divider" => -pad,
            "noteGroup" => 0.0,
            _ => {
                let padded_label_width = tw + pad;
                if rect.width() <= padded_label_width {
                    (padded_label_width - rect.width()) / 2.0 - pad
                } else {
                    -pad
                }
            }
        };
        let offset_y = if n.shape == "roundedWithTitle" {
            th - pad / 2.0
        } else {
            0.0
        };

        let requested_dir = n.dir.as_ref().map(|s| normalize_dir(s));
        let effective_dir = requested_dir
            .clone()
            .unwrap_or_else(|| normalize_dir(&model.direction));

        clusters.push(LayoutCluster {
            id: n.id.clone(),
            x: cx,
            y: cy,
            width: rect.width(),
            height: rect.height(),
            diff,
            offset_y,
            title,
            title_label,
            requested_dir,
            effective_dir,
            padding: pad,
            title_margin_top: 0.0,
            title_margin_bottom: 0.0,
        });

        out_nodes.push(LayoutNode {
            id: n.id.clone(),
            x: cx,
            y: cy,
            width: rect.width(),
            height: rect.height(),
            is_cluster: true,
            label_width: None,
            label_height: None,
        });
    }

    out_nodes.sort_by(|a, b| a.id.cmp(&b.id));
    clusters.sort_by(|a, b| a.id.cmp(&b.id));

    let mut out_edges = merge_edge_segments(
        fragments
            .edge_segments
            .into_iter()
            .filter(|s| {
                semantic_ids.contains(s.original_from.as_str())
                    && semantic_ids.contains(s.original_to.as_str())
            })
            .collect(),
    );

    // Mermaid adjusts the first/last edge points by intersecting the polyline with the node's
    // rendered shape. For rounded state nodes, Mermaid uses a polygon intersection that relies on
    // the historical `intersect-line.js` rounding behavior (producing systematic half-pixel offsets).
    // Our layout engine emits continuous intersections; post-process endpoints to match upstream.
    {
        type Point = merman_core::geom::Point;

        fn same_sign(a: f64, b: f64) -> bool {
            a * b > 0.0
        }

        fn mermaid_intersect_line(p1: Point, p2: Point, q1: Point, q2: Point) -> Option<Point> {
            // Port of Mermaid@11.12.2 `intersect-line.js` (Graphics Gems II).
            let a1 = p2.y - p1.y;
            let b1 = p1.x - p2.x;
            let c1 = p2.x * p1.y - p1.x * p2.y;

            let r3 = a1 * q1.x + b1 * q1.y + c1;
            let r4 = a1 * q2.x + b1 * q2.y + c1;
            if r3 != 0.0 && r4 != 0.0 && same_sign(r3, r4) {
                return None;
            }

            let a2 = q2.y - q1.y;
            let b2 = q1.x - q2.x;
            let c2 = q2.x * q1.y - q1.x * q2.y;

            let r1 = a2 * p1.x + b2 * p1.y + c2;
            let r2 = a2 * p2.x + b2 * p2.y + c2;
            let epsilon = 1e-6;
            if r1.abs() < epsilon && r2.abs() < epsilon && same_sign(r1, r2) {
                return None;
            }

            let denom = a1 * b2 - a2 * b1;
            if denom == 0.0 {
                return None;
            }

            let offset = (denom / 2.0).abs();

            let mut num = b1 * c2 - b2 * c1;
            let x = if num < 0.0 {
                (num - offset) / denom
            } else {
                (num + offset) / denom
            };

            num = a2 * c1 - a1 * c2;
            let y = if num < 0.0 {
                (num - offset) / denom
            } else {
                (num + offset) / denom
            };

            Some(merman_core::geom::point(x, y))
        }

        fn mermaid_arc_points(
            x1: f64,
            y1: f64,
            x2: f64,
            y2: f64,
            rx: f64,
            ry: f64,
            clockwise: bool,
        ) -> Vec<Point> {
            // Port of Mermaid@11.12.2 `roundedRect.ts` `generateArcPoints(...)` (20 points).
            let num_points = 20usize;
            let mid_x = (x1 + x2) / 2.0;
            let mid_y = (y1 + y2) / 2.0;
            let ang = (y2 - y1).atan2(x2 - x1);
            let dx = (x2 - x1) / 2.0;
            let dy = (y2 - y1) / 2.0;
            let tx = dx / rx;
            let ty = dy / ry;
            let dist = (tx * tx + ty * ty).sqrt();
            if dist > 1.0 {
                return Vec::new();
            }
            let scaled_center_dist = (1.0 - dist * dist).sqrt();
            let center_x =
                mid_x + scaled_center_dist * ry * ang.sin() * if clockwise { -1.0 } else { 1.0 };
            let center_y =
                mid_y - scaled_center_dist * rx * ang.cos() * if clockwise { -1.0 } else { 1.0 };

            let start_angle = ((y1 - center_y) / ry).atan2((x1 - center_x) / rx);
            let end_angle = ((y2 - center_y) / ry).atan2((x2 - center_x) / rx);

            let mut angle_range = end_angle - start_angle;
            if clockwise && angle_range < 0.0 {
                angle_range += std::f64::consts::TAU;
            }
            if !clockwise && angle_range > 0.0 {
                angle_range -= std::f64::consts::TAU;
            }

            let mut out = Vec::with_capacity(num_points);
            for i in 0..num_points {
                let t = i as f64 / (num_points - 1) as f64;
                let a = start_angle + t * angle_range;
                out.push(merman_core::geom::point(
                    center_x + rx * a.cos(),
                    center_y + ry * a.sin(),
                ));
            }
            out
        }

        fn mermaid_rounded_rect_points(w: f64, h: f64) -> Vec<Point> {
            // Port of Mermaid@11.12.2 `roundedRect.ts` geometry (taper+arc polygon).
            let radius = 5.0;
            let taper = 5.0;

            let mut points: Vec<Point> = Vec::new();
            points.push(merman_core::geom::point(-w / 2.0 + taper, -h / 2.0));
            points.push(merman_core::geom::point(w / 2.0 - taper, -h / 2.0));
            points.extend(mermaid_arc_points(
                w / 2.0 - taper,
                -h / 2.0,
                w / 2.0,
                -h / 2.0 + taper,
                radius,
                radius,
                true,
            ));

            points.push(merman_core::geom::point(w / 2.0, -h / 2.0 + taper));
            points.push(merman_core::geom::point(w / 2.0, h / 2.0 - taper));
            points.extend(mermaid_arc_points(
                w / 2.0,
                h / 2.0 - taper,
                w / 2.0 - taper,
                h / 2.0,
                radius,
                radius,
                true,
            ));

            points.push(merman_core::geom::point(w / 2.0 - taper, h / 2.0));
            points.push(merman_core::geom::point(-w / 2.0 + taper, h / 2.0));
            points.extend(mermaid_arc_points(
                -w / 2.0 + taper,
                h / 2.0,
                -w / 2.0,
                h / 2.0 - taper,
                radius,
                radius,
                true,
            ));

            points.push(merman_core::geom::point(-w / 2.0, h / 2.0 - taper));
            points.push(merman_core::geom::point(-w / 2.0, -h / 2.0 + taper));
            points.extend(mermaid_arc_points(
                -w / 2.0,
                -h / 2.0 + taper,
                -w / 2.0 + taper,
                -h / 2.0,
                radius,
                radius,
                true,
            ));

            points
        }

        fn mermaid_choice_points(w: f64, h: f64) -> Vec<Point> {
            // Mermaid stateDiagram-v2 "choice" nodes are diamonds.
            vec![
                merman_core::geom::point(0.0, -h / 2.0),
                merman_core::geom::point(w / 2.0, 0.0),
                merman_core::geom::point(0.0, h / 2.0),
                merman_core::geom::point(-w / 2.0, 0.0),
            ]
        }

        fn mermaid_intersect_polygon(
            node: Point,
            w: f64,
            h: f64,
            poly: &[Point],
            point: Point,
        ) -> Point {
            if poly.is_empty() {
                return node;
            }

            let mut min_x = f64::INFINITY;
            let mut min_y = f64::INFINITY;
            for p in poly {
                min_x = min_x.min(p.x);
                min_y = min_y.min(p.y);
            }

            let left = node.x - w / 2.0 - min_x;
            let top = node.y - h / 2.0 - min_y;

            let mut intersections: Vec<Point> = Vec::new();
            for i in 0..poly.len() {
                let p1 = poly[i];
                let p2 = poly[if i + 1 < poly.len() { i + 1 } else { 0 }];
                let q1 = merman_core::geom::point(left + p1.x, top + p1.y);
                let q2 = merman_core::geom::point(left + p2.x, top + p2.y);
                if let Some(hit) = mermaid_intersect_line(node, point, q1, q2) {
                    intersections.push(hit);
                }
            }

            if intersections.is_empty() {
                return node;
            }

            intersections.sort_by(|a, b| {
                let da = ((a.x - point.x).powi(2) + (a.y - point.y).powi(2)).sqrt();
                let db = ((b.x - point.x).powi(2) + (b.y - point.y).powi(2)).sqrt();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            });

            intersections[0]
        }

        fn mermaid_intersect_circle(node: Point, r: f64, point: Point) -> Point {
            // Port of Mermaid@11.12.2 `intersect-ellipse.js`.
            let cx = node.x;
            let cy = node.y;
            let px = cx - point.x;
            let py = cy - point.y;
            let det = (r * r * py * py + r * r * px * px).sqrt();
            if det == 0.0 {
                return node;
            }
            let mut dx = ((r * r * px) / det).abs();
            if point.x < cx {
                dx = -dx;
            }
            let mut dy = ((r * r * py) / det).abs();
            if point.y < cy {
                dy = -dy;
            }
            merman_core::geom::point(cx + dx, cy + dy)
        }

        let layout_nodes: HashMap<&str, &LayoutNode> =
            out_nodes.iter().map(|n| (n.id.as_str(), n)).collect();
        let semantic_nodes: HashMap<&str, &StateNode> =
            model.nodes.iter().map(|n| (n.id.as_str(), n)).collect();

        for e in &mut out_edges {
            if e.points.len() < 2 {
                continue;
            }
            if e.from == e.to {
                continue;
            }
            let Some(start_ln) = layout_nodes.get(e.from.as_str()).copied() else {
                continue;
            };
            let Some(end_ln) = layout_nodes.get(e.to.as_str()).copied() else {
                continue;
            };
            let Some(start_sn) = semantic_nodes.get(e.from.as_str()).copied() else {
                continue;
            };
            let Some(end_sn) = semantic_nodes.get(e.to.as_str()).copied() else {
                continue;
            };

            let start_target = if e.points.len() >= 3 {
                e.points[1].clone()
            } else {
                e.points[e.points.len() - 1].clone()
            };
            let end_target = if e.points.len() >= 3 {
                e.points[e.points.len() - 2].clone()
            } else {
                e.points[0].clone()
            };

            let start_center = merman_core::geom::point(start_ln.x, start_ln.y);
            let end_center = merman_core::geom::point(end_ln.x, end_ln.y);

            let start_target = merman_core::geom::point(start_target.x, start_target.y);
            let end_target = merman_core::geom::point(end_target.x, end_target.y);

            let start_hit = match start_sn.shape.as_str() {
                "stateStart" | "stateEnd" => {
                    mermaid_intersect_circle(start_center, 7.0, start_target)
                }
                "choice" => {
                    let poly =
                        mermaid_choice_points(start_ln.width.max(1.0), start_ln.height.max(1.0));
                    mermaid_intersect_polygon(
                        start_center,
                        start_ln.width.max(1.0),
                        start_ln.height.max(1.0),
                        &poly,
                        start_target,
                    )
                }
                // `rect` with rx/ry becomes `roundedRect` in Mermaid.
                "rect" if start_sn.rx.unwrap_or(0.0) > 0.0 && start_sn.ry.unwrap_or(0.0) > 0.0 => {
                    let poly = mermaid_rounded_rect_points(
                        start_ln.width.max(1.0),
                        start_ln.height.max(1.0),
                    );
                    mermaid_intersect_polygon(
                        start_center,
                        start_ln.width.max(1.0),
                        start_ln.height.max(1.0),
                        &poly,
                        start_target,
                    )
                }
                _ => start_center,
            };
            let end_hit = match end_sn.shape.as_str() {
                "stateStart" | "stateEnd" => mermaid_intersect_circle(end_center, 7.0, end_target),
                "choice" => {
                    let poly = mermaid_choice_points(end_ln.width.max(1.0), end_ln.height.max(1.0));
                    mermaid_intersect_polygon(
                        end_center,
                        end_ln.width.max(1.0),
                        end_ln.height.max(1.0),
                        &poly,
                        end_target,
                    )
                }
                "rect" if end_sn.rx.unwrap_or(0.0) > 0.0 && end_sn.ry.unwrap_or(0.0) > 0.0 => {
                    let poly =
                        mermaid_rounded_rect_points(end_ln.width.max(1.0), end_ln.height.max(1.0));
                    mermaid_intersect_polygon(
                        end_center,
                        end_ln.width.max(1.0),
                        end_ln.height.max(1.0),
                        &poly,
                        end_target,
                    )
                }
                _ => end_center,
            };

            if let Some(p0) = e.points.first_mut() {
                p0.x = start_hit.x;
                p0.y = start_hit.y;
            }
            if let Some(pn) = e.points.last_mut() {
                pn.x = end_hit.x;
                pn.y = end_hit.y;
            }
        }
    }
    out_edges.sort_by(|a, b| a.id.cmp(&b.id));

    let bounds = {
        let mut points: Vec<(f64, f64)> = Vec::new();
        for n in &out_nodes {
            let r = Rect::from_center(n.x, n.y, n.width, n.height);
            points.push((r.min_x(), r.min_y()));
            points.push((r.max_x(), r.max_y()));
        }
        for e in &out_edges {
            for p in &e.points {
                points.push((p.x, p.y));
            }
            if let Some(l) = &e.label {
                let r = Rect::from_center(l.x, l.y, l.width, l.height);
                points.push((r.min_x(), r.min_y()));
                points.push((r.max_x(), r.max_y()));
            }
        }
        Bounds::from_points(points)
    };

    Ok(StateDiagramV2Layout {
        nodes: out_nodes,
        edges: out_edges,
        clusters,
        bounds,
    })
}

fn validate_state_parent_cycles(model: &StateDiagramModel) -> Result<()> {
    let parent_by_id: HashMap<&str, &str> = model
        .nodes
        .iter()
        .filter_map(|node| {
            node.parent_id
                .as_deref()
                .map(|parent| (node.id.as_str(), parent))
        })
        .collect();
    for node in &model.nodes {
        let mut current = Some(node.id.as_str());
        let mut seen: HashSet<&str> = HashSet::new();
        while let Some(id) = current {
            if !seen.insert(id) {
                return Err(Error::InvalidModel {
                    message: format!("state parent cycle involving {id}"),
                });
            }
            current = parent_by_id.get(id).copied();
        }
    }
    Ok(())
}

/// Debug-only helper: builds the Dagre input graph for stateDiagram-v2 *before* layout runs.
///
/// This shares the same graph construction path as `layout_state_diagram_v2_inner`.
/// It is used by `xtask` to compare `dugong` against Mermaid's JS Dagre implementation
/// (`dagre-d3-es`) at the layout output layer (nodes/edges/points) rather than at the SVG layer.
#[doc(hidden)]
pub fn debug_build_state_diagram_v2_dagre_graph(
    semantic: &Value,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<Graph<NodeLabel, EdgeLabel, GraphLabel>> {
    let model: StateDiagramModel = crate::json::from_value_ref(semantic)?;
    Ok(build_state_diagram_v2_dagre_input(&model, effective_config, measurer)?.graph)
}

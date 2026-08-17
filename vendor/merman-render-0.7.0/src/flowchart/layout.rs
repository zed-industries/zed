use crate::config::{config_f64, config_string};
use crate::math::MathRenderer;
use crate::model::{
    FlowchartV2Layout, LayoutCluster, LayoutEdge, LayoutLabel, LayoutNode, LayoutPoint,
};
use crate::text::{TextMeasurer, TextStyle, WrapMode};
use crate::{Error, Result};
use dugong::graphlib::{Graph, GraphOptions};
use dugong::{EdgeLabel, GraphLabel, LabelPos, NodeLabel, RankDir};
use merman_core::MermaidConfig;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

use super::label::compute_bounds;
use super::node::{NodeLayoutDimensionsRequest, node_layout_dimensions};
use super::{FlowEdge, FlowSubgraph, FlowchartV2Model};
use super::{
    FlowchartLabelMetricsRequest, flowchart_effective_font_style_for_classes,
    flowchart_effective_font_style_for_node_classes, flowchart_effective_html_labels,
    flowchart_effective_node_html_labels, flowchart_effective_text_style_for_classes,
    flowchart_effective_text_style_for_node_classes, flowchart_html_label_measurement_base_style,
    flowchart_label_metrics_for_layout, flowchart_label_plain_text_for_layout,
    flowchart_node_has_span_css_height_parity, flowchart_whole_label_font_style_requests_italic,
};

fn flowchart_svg_plain_computed_width_px(
    measurer: &dyn TextMeasurer,
    plain: &str,
    style: &TextStyle,
    max_width_px: Option<f64>,
) -> f64 {
    let wrapped_lines =
        crate::text::wrap_svg_text_lines_by_measurement(measurer, plain, style, max_width_px, true);
    let mut width: f64 = 0.0;
    for line in wrapped_lines {
        width = width.max(measurer.measure_svg_text_computed_length_px(line.trim_end(), style));
    }
    crate::text::round_to_1_64_px(width)
}

fn rank_dir_from_flow(direction: &str) -> RankDir {
    match direction.trim().to_uppercase().as_str() {
        "TB" | "TD" => RankDir::TB,
        "BT" => RankDir::BT,
        "LR" => RankDir::LR,
        "RL" => RankDir::RL,
        _ => RankDir::TB,
    }
}

fn flowchart_dagre_spacing_or_default(config: &Value, key: &str, default: f64) -> f64 {
    let Some(raw) = config
        .get("flowchart")
        .and_then(|flowchart| flowchart.get(key))
    else {
        return default;
    };

    // Mermaid Flowchart assigns `conf?.nodeSpacing || 50` and `conf?.rankSpacing || 50`,
    // so numeric zero falls back to the default instead of becoming a real Dagre separation.
    if raw.is_number() {
        return raw
            .as_f64()
            .filter(|value| *value != 0.0)
            .unwrap_or(default);
    }

    config_f64(config, &["flowchart", key]).unwrap_or(default)
}

fn normalize_dir(s: &str) -> String {
    s.trim().to_uppercase()
}

fn toggled_dir(parent: &str) -> String {
    let parent = normalize_dir(parent);
    if parent == "TB" || parent == "TD" {
        "LR".to_string()
    } else {
        "TB".to_string()
    }
}

fn flow_dir_from_rankdir(rankdir: RankDir) -> &'static str {
    match rankdir {
        RankDir::TB => "TB",
        RankDir::BT => "BT",
        RankDir::LR => "LR",
        RankDir::RL => "RL",
    }
}

fn effective_cluster_dir(sg: &FlowSubgraph, parent_dir: &str, inherit_dir: bool) -> String {
    if let Some(dir) = sg.dir.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        return normalize_dir(dir);
    }
    if inherit_dir {
        return normalize_dir(parent_dir);
    }
    toggled_dir(parent_dir)
}

fn compute_effective_dir_by_id(
    subgraphs_by_id: &HashMap<String, FlowSubgraph>,
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    diagram_dir: &str,
    inherit_dir: bool,
) -> HashMap<String, String> {
    fn compute_one_iterative(
        id: &str,
        subgraphs_by_id: &HashMap<String, FlowSubgraph>,
        g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
        diagram_dir: &str,
        inherit_dir: bool,
        memo: &mut HashMap<String, String>,
    ) {
        if memo.contains_key(id) {
            return;
        }

        let mut path: Vec<String> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        let mut cur = id.to_string();
        let mut parent_dir = loop {
            if let Some(dir) = memo.get(&cur) {
                break dir.clone();
            }
            if !seen.insert(cur.clone()) {
                let dir = toggled_dir(diagram_dir);
                memo.insert(cur.clone(), dir.clone());
                break dir;
            }

            path.push(cur.clone());
            let Some(parent) = g.parent(&cur).filter(|p| subgraphs_by_id.contains_key(*p)) else {
                break normalize_dir(diagram_dir);
            };
            cur = parent.to_string();
        };

        for node in path.into_iter().rev() {
            if let Some(dir) = memo.get(&node) {
                parent_dir = dir.clone();
                continue;
            }

            let dir = subgraphs_by_id
                .get(&node)
                .map(|sg| effective_cluster_dir(sg, &parent_dir, inherit_dir))
                .unwrap_or_else(|| toggled_dir(&parent_dir));
            memo.insert(node, dir.clone());
            parent_dir = dir;
        }
    }

    let mut memo: HashMap<String, String> = HashMap::new();
    for id in subgraphs_by_id.keys() {
        compute_one_iterative(id, subgraphs_by_id, g, diagram_dir, inherit_dir, &mut memo);
    }
    memo
}

fn dir_to_rankdir(dir: &str) -> RankDir {
    match normalize_dir(dir).as_str() {
        "TB" | "TD" => RankDir::TB,
        "BT" => RankDir::BT,
        "LR" => RankDir::LR,
        "RL" => RankDir::RL,
        _ => RankDir::TB,
    }
}

fn edge_label_is_non_empty(edge: &FlowEdge) -> bool {
    edge.label
        .as_deref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
}

fn first_parent_cycle_assignment<'a, I>(
    order: I,
    parent_by_id: &HashMap<String, String>,
) -> Option<(String, String)>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut assigned_parent: HashMap<String, String> = HashMap::new();

    for child in order {
        let Some(parent) = parent_by_id.get(child) else {
            continue;
        };

        let mut current = Some(parent.as_str());
        while let Some(id) = current {
            if id == child {
                return Some((child.to_string(), parent.clone()));
            }
            current = assigned_parent.get(id).map(String::as_str);
        }

        assigned_parent.insert(child.to_string(), parent.clone());
    }

    None
}

fn lowest_common_parent(
    g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    a: &str,
    b: &str,
) -> Option<String> {
    if !g.options().compound {
        return None;
    }

    let mut ancestors: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut cur = g.parent(a);
    while let Some(p) = cur {
        ancestors.insert(p.to_string());
        cur = g.parent(p);
    }

    let mut cur = g.parent(b);
    while let Some(p) = cur {
        if ancestors.contains(p) {
            return Some(p.to_string());
        }
        cur = g.parent(p);
    }

    None
}

fn extract_descendants(id: &str, g: &Graph<NodeLabel, EdgeLabel, GraphLabel>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = g.children(id).iter().map(|s| s.to_string()).collect();
    while let Some(node) = stack.pop() {
        if !visited.insert(node.clone()) {
            continue;
        }
        for child in g.children(&node) {
            stack.push(child.to_string());
        }
        out.push(node);
    }

    out
}

fn edge_in_cluster(
    edge: &dugong::graphlib::EdgeKey,
    cluster_id: &str,
    descendants: &std::collections::HashMap<String, Vec<String>>,
) -> bool {
    if edge.v == cluster_id || edge.w == cluster_id {
        return false;
    }
    let Some(cluster_desc) = descendants.get(cluster_id) else {
        return false;
    };
    cluster_desc.contains(&edge.v) || cluster_desc.contains(&edge.w)
}

#[derive(Debug, Clone)]
struct FlowchartClusterDbEntry {
    anchor_id: String,
    external_connections: bool,
}

fn flowchart_find_common_edges(
    graph: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
    id1: &str,
    id2: &str,
) -> Vec<(String, String)> {
    let edges1: Vec<(String, String)> = graph
        .edge_keys()
        .into_iter()
        .filter(|edge| edge.v == id1 || edge.w == id1)
        .map(|edge| (edge.v, edge.w))
        .collect();
    let edges2: Vec<(String, String)> = graph
        .edge_keys()
        .into_iter()
        .filter(|edge| edge.v == id2 || edge.w == id2)
        .map(|edge| (edge.v, edge.w))
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

fn flowchart_find_non_cluster_child(
    id: &str,
    graph: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
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
        let common_edges = flowchart_find_common_edges(graph, cluster_id, &node);
        if !common_edges.is_empty() {
            reserve = Some(node);
        } else {
            return Some(node);
        }
    }

    reserve
}

fn adjust_flowchart_clusters_and_edges(
    graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
) -> std::collections::HashMap<String, bool> {
    use serde_json::Value;

    fn is_descendant(
        node_id: &str,
        cluster_id: &str,
        descendants: &std::collections::HashMap<String, Vec<String>>,
    ) -> bool {
        descendants
            .get(cluster_id)
            .is_some_and(|ds| ds.iter().any(|n| n == node_id))
    }

    let mut descendants: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let mut cluster_db: std::collections::HashMap<String, FlowchartClusterDbEntry> =
        std::collections::HashMap::new();

    for id in graph.node_ids() {
        if graph.children(&id).is_empty() {
            continue;
        }
        descendants.insert(id.clone(), extract_descendants(&id, graph));
        let anchor_id =
            flowchart_find_non_cluster_child(&id, graph, &id).unwrap_or_else(|| id.clone());
        cluster_db.insert(
            id,
            FlowchartClusterDbEntry {
                anchor_id,
                external_connections: false,
            },
        );
    }

    for id in cluster_db.keys().cloned().collect::<Vec<_>>() {
        if graph.children(&id).is_empty() {
            continue;
        }
        let mut has_external = false;
        for e in graph.edges() {
            let d1 = is_descendant(e.v.as_str(), id.as_str(), &descendants);
            let d2 = is_descendant(e.w.as_str(), id.as_str(), &descendants);
            if d1 ^ d2 {
                has_external = true;
                break;
            }
        }
        if let Some(entry) = cluster_db.get_mut(&id) {
            entry.external_connections = has_external;
        }
    }

    for id in cluster_db.keys().cloned().collect::<Vec<_>>() {
        let Some(non_cluster_child) = cluster_db.get(&id).map(|e| e.anchor_id.clone()) else {
            continue;
        };
        let parent = graph.parent(&non_cluster_child);
        if parent.is_some_and(|p| p != id.as_str())
            && parent.is_some_and(|p| cluster_db.contains_key(p))
            && parent.is_some_and(|p| !cluster_db.get(p).is_some_and(|e| e.external_connections))
        {
            if let Some(p) = parent {
                if let Some(entry) = cluster_db.get_mut(&id) {
                    entry.anchor_id = p.to_string();
                }
            }
        }
    }

    fn get_anchor_id(
        id: &str,
        cluster_db: &std::collections::HashMap<String, FlowchartClusterDbEntry>,
    ) -> String {
        let Some(entry) = cluster_db.get(id) else {
            return id.to_string();
        };
        if !entry.external_connections {
            return id.to_string();
        }
        entry.anchor_id.clone()
    }

    let edge_keys = graph.edge_keys();
    for ek in edge_keys {
        if !cluster_db.contains_key(&ek.v) && !cluster_db.contains_key(&ek.w) {
            continue;
        }

        let Some(mut edge_label) = graph.edge_by_key(&ek).cloned() else {
            continue;
        };

        let v = get_anchor_id(&ek.v, &cluster_db);
        let w = get_anchor_id(&ek.w, &cluster_db);

        // Match Mermaid `adjustClustersAndEdges`: edges that touch cluster nodes are removed and
        // re-inserted even when their endpoints do not change. This affects edge iteration order
        // and therefore cycle-breaking determinism in Dagre's acyclic pass.
        let _ = graph.remove_edge_key(&ek);

        if v != ek.v {
            if let Some(parent) = graph.parent(&v) {
                if let Some(entry) = cluster_db.get_mut(parent) {
                    entry.external_connections = true;
                }
            }
            edge_label
                .extras
                .insert("fromCluster".to_string(), Value::String(ek.v.clone()));
        }

        if w != ek.w {
            if let Some(parent) = graph.parent(&w) {
                if let Some(entry) = cluster_db.get_mut(parent) {
                    entry.external_connections = true;
                }
            }
            edge_label
                .extras
                .insert("toCluster".to_string(), Value::String(ek.w.clone()));
        }

        graph.set_edge_named(v, w, ek.name, Some(edge_label));
    }

    cluster_db
        .into_iter()
        .map(|(id, entry)| (id, entry.external_connections))
        .collect()
}

fn copy_cluster(
    cluster_id: &str,
    graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    new_graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    root_id: &str,
    descendants: &std::collections::HashMap<String, Vec<String>>,
) {
    #[derive(Debug)]
    struct CopyFrame {
        node: String,
        owner_cluster: String,
        expanded: bool,
    }

    let mut nodes: Vec<(String, String)> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut stack: Vec<CopyFrame> = Vec::new();
    if cluster_id != root_id {
        stack.push(CopyFrame {
            node: cluster_id.to_string(),
            owner_cluster: cluster_id.to_string(),
            expanded: true,
        });
    }
    stack.extend(graph.children(cluster_id).iter().rev().map(|s| CopyFrame {
        node: s.to_string(),
        owner_cluster: cluster_id.to_string(),
        expanded: false,
    }));

    while let Some(frame) = stack.pop() {
        if frame.expanded {
            nodes.push((frame.node, frame.owner_cluster));
            continue;
        }
        if !visited.insert(frame.node.clone()) {
            continue;
        }

        let children = graph.children(&frame.node);
        if children.is_empty() {
            nodes.push((frame.node, frame.owner_cluster));
            continue;
        }

        stack.push(CopyFrame {
            node: frame.node.clone(),
            owner_cluster: frame.node.clone(),
            expanded: true,
        });
        for child in children.iter().rev() {
            stack.push(CopyFrame {
                node: child.to_string(),
                owner_cluster: frame.node.clone(),
                expanded: false,
            });
        }
    }

    for (node, owner_cluster) in nodes {
        if !graph.has_node(&node) {
            continue;
        }

        let data = graph.node(&node).cloned().unwrap_or_default();
        new_graph.set_node(node.clone(), data);

        if let Some(parent) = graph.parent(&node) {
            if parent != root_id {
                new_graph.set_parent(node.clone(), parent.to_string());
            }
        }
        if owner_cluster != root_id && node != owner_cluster {
            new_graph.set_parent(node.clone(), owner_cluster);
        }

        // Copy edges for this extracted cluster.
        //
        // Mermaid's implementation calls `graph.edges(node)` (note: on dagre-d3-es Graphlib,
        // `edges()` ignores the argument and returns *all* edges). Because the source graph is
        // mutated as nodes are removed, this makes edge insertion order sensitive to the leaf
        // traversal order, which in turn can affect deterministic tie-breaking in Dagre's
        // acyclic/ranking steps.
        //
        // Reference:
        // - `packages/mermaid/src/rendering-util/layout-algorithms/dagre/mermaid-graphlib.js`
        for ek in graph.edge_keys() {
            if !edge_in_cluster(&ek, root_id, descendants) {
                continue;
            }
            if new_graph.has_edge(&ek.v, &ek.w, ek.name.as_deref()) {
                continue;
            }
            let Some(lbl) = graph.edge_by_key(&ek).cloned() else {
                continue;
            };
            new_graph.set_edge_named(ek.v, ek.w, ek.name, Some(lbl));
        }

        let _ = graph.remove_node(&node);
    }
}

fn extract_clusters_recursively(
    graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
    subgraphs_by_id: &std::collections::HashMap<String, FlowSubgraph>,
    external_connections_by_id: &std::collections::HashMap<String, bool>,
    extracted: &mut std::collections::HashMap<String, Graph<NodeLabel, EdgeLabel, GraphLabel>>,
    _depth: usize,
) {
    struct ExtractFrame {
        id: String,
        graph: Graph<NodeLabel, EdgeLabel, GraphLabel>,
        expanded: bool,
    }

    fn extract_one_level(
        graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
        subgraphs_by_id: &std::collections::HashMap<String, FlowSubgraph>,
        external_connections_by_id: &std::collections::HashMap<String, bool>,
    ) -> Vec<(String, Graph<NodeLabel, EdgeLabel, GraphLabel>)> {
        let node_ids = graph.node_ids();
        let mut descendants: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        for id in &node_ids {
            if graph.children(id).is_empty() {
                continue;
            }
            descendants.insert(id.clone(), extract_descendants(id, graph));
        }

        let mut extracted_here: Vec<(String, Graph<NodeLabel, EdgeLabel, GraphLabel>)> = Vec::new();

        let candidates: Vec<String> = node_ids
            .into_iter()
            .filter(|id| graph.has_node(id))
            .filter(|id| !graph.children(id).is_empty())
            // Mermaid's extractor does not recompute external connections after
            // `adjustClustersAndEdges` rewrites cluster endpoints to anchor children. It uses the
            // global `clusterDb.externalConnections` flag computed before those rewrites, then
            // recurses into extracted graphs with the same `clusterDb`.
            //
            // Reference:
            // - `packages/mermaid/src/rendering-util/layout-algorithms/dagre/mermaid-graphlib.js`
            .filter(|id| {
                external_connections_by_id
                    .get(id.as_str())
                    .is_some_and(|external| !external)
            })
            .collect();

        for id in candidates {
            if !graph.has_node(&id) {
                continue;
            }
            if graph.children(&id).is_empty() {
                continue;
            }

            let mut cluster_graph: Graph<NodeLabel, EdgeLabel, GraphLabel> =
                Graph::new(GraphOptions {
                    multigraph: true,
                    compound: true,
                    directed: true,
                });

            // Mermaid's `extractor(...)` uses:
            // - `clusterData.dir` when explicitly set for the subgraph
            // - otherwise: toggle relative to the current graph's rankdir (TB<->LR)
            let dir = subgraphs_by_id
                .get(&id)
                .and_then(|sg| sg.dir.as_deref())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(normalize_dir)
                .unwrap_or_else(|| toggled_dir(flow_dir_from_rankdir(graph.graph().rankdir)));

            cluster_graph.set_graph(GraphLabel {
                rankdir: dir_to_rankdir(&dir),
                // Mermaid's cluster extractor initializes subgraphs with a fixed dagre config
                // (nodesep/ranksep=50, marginx/marginy=8). Before each recursive render Mermaid then
                // overrides `nodesep` to the parent graph value and `ranksep` to `parent.ranksep + 25`.
                //
                // We model that in headless mode by keeping the extractor defaults here, then applying
                // the per-depth override inside `layout_graph_with_recursive_clusters(...)` right
                // before laying out each extracted graph.
                //
                // Reference:
                // - `packages/mermaid/src/rendering-util/layout-algorithms/dagre/mermaid-graphlib.js`
                // - `packages/mermaid/src/rendering-util/layout-algorithms/dagre/index.js`
                nodesep: 50.0,
                ranksep: 50.0,
                marginx: 8.0,
                marginy: 8.0,
                acyclicer: None,
                ..Default::default()
            });

            copy_cluster(&id, graph, &mut cluster_graph, &id, &descendants);
            extracted_here.push((id, cluster_graph));
        }

        extracted_here
    }

    let extracted_here = extract_one_level(graph, subgraphs_by_id, external_connections_by_id);
    let mut stack: Vec<ExtractFrame> = extracted_here
        .into_iter()
        .rev()
        .map(|(id, graph)| ExtractFrame {
            id,
            graph,
            expanded: false,
        })
        .collect();

    while let Some(mut frame) = stack.pop() {
        if frame.expanded {
            extracted.insert(frame.id, frame.graph);
            continue;
        }

        let children = extract_one_level(
            &mut frame.graph,
            subgraphs_by_id,
            external_connections_by_id,
        );
        frame.expanded = true;
        stack.push(frame);
        stack.extend(children.into_iter().rev().map(|(id, graph)| ExtractFrame {
            id,
            graph,
            expanded: false,
        }));
    }
}

pub fn layout_flowchart_v2(
    semantic: &Value,
    effective_config: &MermaidConfig,
    measurer: &dyn TextMeasurer,
    math_renderer: Option<&(dyn MathRenderer + Send + Sync)>,
) -> Result<FlowchartV2Layout> {
    let timing_enabled = std::env::var("MERMAN_FLOWCHART_LAYOUT_TIMING")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let total_start = timing_enabled.then(web_time::Instant::now);

    let deserialize_start = timing_enabled.then(web_time::Instant::now);
    let model: FlowchartV2Model = crate::json::from_value_ref(semantic)?;
    let deserialize = deserialize_start.map(|s| s.elapsed()).unwrap_or_default();

    layout_flowchart_v2_with_model(
        &model,
        effective_config,
        measurer,
        math_renderer,
        timing_enabled,
        total_start,
        deserialize,
    )
}

pub fn layout_flowchart_v2_typed(
    model: &FlowchartV2Model,
    effective_config: &MermaidConfig,
    measurer: &dyn TextMeasurer,
    math_renderer: Option<&(dyn MathRenderer + Send + Sync)>,
) -> Result<FlowchartV2Layout> {
    let timing_enabled = std::env::var("MERMAN_FLOWCHART_LAYOUT_TIMING")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let total_start = timing_enabled.then(web_time::Instant::now);

    layout_flowchart_v2_with_model(
        model,
        effective_config,
        measurer,
        math_renderer,
        timing_enabled,
        total_start,
        web_time::Duration::default(),
    )
}

fn layout_flowchart_v2_with_model(
    model: &FlowchartV2Model,
    effective_config: &MermaidConfig,
    measurer: &dyn TextMeasurer,
    math_renderer: Option<&(dyn MathRenderer + Send + Sync)>,
    timing_enabled: bool,
    total_start: Option<web_time::Instant>,
    deserialize: web_time::Duration,
) -> Result<FlowchartV2Layout> {
    #[derive(Debug, Default, Clone)]
    struct FlowchartLayoutTimings {
        total: web_time::Duration,
        deserialize: web_time::Duration,
        expand_self_loops: web_time::Duration,
        build_graph: web_time::Duration,
        extract_clusters: web_time::Duration,
        dom_order: web_time::Duration,
        layout_recursive: web_time::Duration,
        dagre_calls: u32,
        dagre_total: web_time::Duration,
        place_graph: web_time::Duration,
        build_output: web_time::Duration,
    }

    let mut timings = FlowchartLayoutTimings {
        deserialize,
        ..Default::default()
    };

    let effective_config_value = effective_config.as_value();

    // Mermaid's dagre adapter expands self-loop edges into a chain of two special label nodes plus
    // three edges. This avoids `v == w` edges in Dagre and is required for SVG parity (Mermaid
    // uses `*-cyclic-special-*` ids when rendering self-loops).
    let expand_self_loops_start = timing_enabled.then(web_time::Instant::now);
    let self_loop_count = model.edges.iter().filter(|e| e.from == e.to).count();
    let mut render_edges: Vec<std::borrow::Cow<'_, FlowEdge>> =
        Vec::with_capacity(model.edges.len() + self_loop_count * 3);
    let mut self_loop_label_node_ids: Vec<String> = Vec::new();
    let mut self_loop_label_node_id_set: std::collections::HashSet<String> =
        std::collections::HashSet::new();
    for e in &model.edges {
        if e.from != e.to {
            render_edges.push(std::borrow::Cow::Borrowed(e));
            continue;
        }

        let helper_edges = super::flowchart_self_loop_helper_edges(
            e,
            super::FlowchartSelfLoopEdgeOptions::layout(),
        );
        if self_loop_label_node_id_set.insert(helper_edges.special_id_1.clone()) {
            self_loop_label_node_ids.push(helper_edges.special_id_1.clone());
        }
        if self_loop_label_node_id_set.insert(helper_edges.special_id_2.clone()) {
            self_loop_label_node_ids.push(helper_edges.special_id_2.clone());
        }

        // Mermaid clears the label text on the end segments, but keeps the label (if any) on the
        // mid edge (`edgeMid` is a structuredClone of the original edge without label changes).
        render_edges.push(std::borrow::Cow::Owned(helper_edges.edge1));
        render_edges.push(std::borrow::Cow::Owned(helper_edges.edge_mid));
        render_edges.push(std::borrow::Cow::Owned(helper_edges.edge2));
    }
    if let Some(s) = expand_self_loops_start {
        timings.expand_self_loops = s.elapsed();
    }

    let build_graph_start = timing_enabled.then(web_time::Instant::now);

    let nodesep = flowchart_dagre_spacing_or_default(effective_config_value, "nodeSpacing", 50.0);
    let ranksep = flowchart_dagre_spacing_or_default(effective_config_value, "rankSpacing", 50.0);
    // Mermaid's default config sets `flowchart.padding` to 15.
    let node_padding =
        config_f64(effective_config_value, &["flowchart", "padding"]).unwrap_or(15.0);
    // Used by a few flowchart-v2 shapes (notably `forkJoin.ts`) to inflate Dagre node dimensions.
    // Mermaid default config sets `state.padding` to 8.
    let state_padding = config_f64(effective_config_value, &["state", "padding"]).unwrap_or(8.0);
    let wrapping_width =
        config_f64(effective_config_value, &["flowchart", "wrappingWidth"]).unwrap_or(200.0);
    // Mermaid measures edge labels via `createText(...)` without overriding the default
    // wrapping width (200px), independent of `flowchart.wrappingWidth`.
    let edge_label_wrapping_width = 200.0;
    // Mermaid 11.15 wraps markdown subgraph titles through `createText(...)` but renders
    // non-markdown titles through deprecated `createLabel(... width=Infinity)`.
    let cluster_title_wrapping_width = 200.0;
    // Mermaid 11.15 has asymmetric Flowchart html-label semantics:
    // node shapes use root `htmlLabels` directly, while edge and cluster labels use
    // `getEffectiveHtmlLabels(...)` and still honor deprecated `flowchart.htmlLabels`.
    let node_html_labels = flowchart_effective_node_html_labels(effective_config_value);
    let flowchart_html_labels = flowchart_effective_html_labels(effective_config_value);
    let edge_html_labels = flowchart_html_labels;
    let cluster_html_labels = edge_html_labels;
    let node_html_label_css_parity = node_html_labels && flowchart_html_labels;
    let node_wrap_mode = if node_html_labels {
        WrapMode::HtmlLike
    } else {
        WrapMode::SvgLike
    };
    let cluster_wrap_mode = if cluster_html_labels {
        WrapMode::HtmlLike
    } else {
        WrapMode::SvgLike
    };
    let edge_wrap_mode = if edge_html_labels {
        WrapMode::HtmlLike
    } else {
        WrapMode::SvgLike
    };
    // Mermaid FlowDB encodes subgraph nodes with a fixed `padding: 8` in `data4Layout.nodes`.
    // That value is separate from `flowchart.padding` (node padding) and `nodeSpacing`/`rankSpacing`.
    let cluster_padding = 8.0;
    let title_margin_top = config_f64(
        effective_config_value,
        &["flowchart", "subGraphTitleMargin", "top"],
    )
    .unwrap_or(0.0);
    let title_margin_bottom = config_f64(
        effective_config_value,
        &["flowchart", "subGraphTitleMargin", "bottom"],
    )
    .unwrap_or(0.0);
    let title_total_margin = title_margin_top + title_margin_bottom;
    let y_shift = title_total_margin / 2.0;
    let inherit_dir = effective_config_value
        .get("flowchart")
        .and_then(|v| v.get("inheritDir"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    fn normalize_css_font_family(font_family: &str) -> String {
        let s = font_family.trim().trim_end_matches(';').trim();
        if s.is_empty() {
            return String::new();
        }

        let mut parts: Vec<String> = Vec::new();
        let mut cur = String::new();
        let mut in_single = false;
        let mut in_double = false;

        for ch in s.chars() {
            match ch {
                '\'' if !in_double => {
                    in_single = !in_single;
                    cur.push(ch);
                }
                '"' if !in_single => {
                    in_double = !in_double;
                    cur.push(ch);
                }
                ',' if !in_single && !in_double => {
                    let p = cur.trim();
                    if !p.is_empty() {
                        parts.push(p.to_string());
                    }
                    cur.clear();
                }
                _ => cur.push(ch),
            }
        }

        let p = cur.trim();
        if !p.is_empty() {
            parts.push(p.to_string());
        }
        parts.join(",")
    }

    fn parse_font_size_px(v: &serde_json::Value) -> Option<f64> {
        if let Some(n) = v.as_f64() {
            return Some(n);
        }
        if let Some(n) = v.as_i64() {
            return Some(n as f64);
        }
        if let Some(n) = v.as_u64() {
            return Some(n as f64);
        }
        let s = v.as_str()?.trim();
        if s.is_empty() {
            return None;
        }
        let mut num = String::new();
        for (idx, ch) in s.chars().enumerate() {
            if ch.is_ascii_digit() {
                num.push(ch);
                continue;
            }
            if idx == 0 && (ch == '-' || ch == '+') {
                num.push(ch);
                continue;
            }
            break;
        }
        if num.trim().is_empty() {
            return None;
        }
        num.parse::<f64>().ok()
    }

    let default_theme_font_family = "\"trebuchet ms\",verdana,arial,sans-serif".to_string();
    let theme_font_family =
        config_string(effective_config_value, &["themeVariables", "fontFamily"])
            .map(|s| normalize_css_font_family(&s));
    let top_font_family = config_string(effective_config_value, &["fontFamily"])
        .map(|s| normalize_css_font_family(&s));
    let font_family = Some(match (top_font_family, theme_font_family) {
        (Some(top), Some(theme)) if theme == default_theme_font_family => top,
        (_, Some(theme)) => theme,
        (Some(top), None) => top,
        (None, None) => default_theme_font_family,
    });
    let font_size = effective_config_value
        .get("themeVariables")
        .and_then(|tv| tv.get("fontSize"))
        .and_then(parse_font_size_px)
        .unwrap_or(16.0);
    let font_weight = config_string(effective_config_value, &["fontWeight"]);
    let text_style = TextStyle {
        font_family,
        font_size,
        font_weight,
    };
    let html_label_text_style =
        flowchart_html_label_measurement_base_style(&text_style, effective_config_value);
    let node_label_base_style = if node_wrap_mode == WrapMode::HtmlLike {
        &html_label_text_style
    } else {
        &text_style
    };
    let cluster_label_base_style = if cluster_wrap_mode == WrapMode::HtmlLike {
        &html_label_text_style
    } else {
        &text_style
    };
    let edge_label_base_style = if edge_wrap_mode == WrapMode::HtmlLike {
        &html_label_text_style
    } else {
        &text_style
    };

    let diagram_direction = normalize_dir(model.direction.as_deref().unwrap_or("TB"));
    let has_subgraphs = !model.subgraphs.is_empty();
    let mut subgraphs_by_id: std::collections::HashMap<String, FlowSubgraph> =
        std::collections::HashMap::new();
    for sg in &model.subgraphs {
        subgraphs_by_id.insert(sg.id.clone(), sg.clone());
    }
    let subgraph_ids: std::collections::HashSet<&str> =
        model.subgraphs.iter().map(|sg| sg.id.as_str()).collect();
    let subgraph_id_set: std::collections::HashSet<String> =
        model.subgraphs.iter().map(|sg| sg.id.clone()).collect();
    let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
        multigraph: true,
        // Mermaid's Dagre adapter always enables `compound: true`, even if there are no explicit
        // subgraphs. This also allows `nestingGraph.run` to connect components during ranking.
        compound: true,
        directed: true,
    });
    g.set_graph(GraphLabel {
        rankdir: rank_dir_from_flow(&diagram_direction),
        nodesep,
        ranksep,
        marginx: 8.0,
        marginy: 8.0,
        acyclicer: None,
        ..Default::default()
    });

    let mut empty_subgraph_ids: Vec<String> = Vec::new();
    let mut cluster_node_labels: std::collections::HashMap<String, NodeLabel> =
        std::collections::HashMap::new();
    for sg in &model.subgraphs {
        if sg.nodes.is_empty() {
            // Mermaid renders empty subgraphs as regular nodes. Keep the semantic `subgraph`
            // definition around for styling/title, but size + lay it out as a leaf node.
            empty_subgraph_ids.push(sg.id.clone());
            continue;
        }
        // Mermaid does not pre-size compound (subgraph) nodes based on title metrics for Dagre
        // layout. Their dimensions are computed from children (border nodes) and then adjusted at
        // render time for title width and configured margins.
        cluster_node_labels.insert(sg.id.clone(), NodeLabel::default());
    }

    let mut leaf_node_labels: std::collections::HashMap<String, NodeLabel> =
        std::collections::HashMap::new();
    let mut leaf_label_metrics_by_id: HashMap<String, (f64, f64)> = HashMap::new();
    leaf_label_metrics_by_id.reserve(model.nodes.len() + empty_subgraph_ids.len());
    for n in &model.nodes {
        // Mermaid treats the subgraph id as the "group node" id (a cluster can be referenced in
        // edges). Avoid introducing a separate leaf node that would collide with the cluster node
        // of the same id.
        if subgraph_ids.contains(n.id.as_str()) {
            continue;
        }
        let raw_label = n.label.as_deref().unwrap_or(&n.id);
        let label_type = n.label_type.as_deref().unwrap_or("text");
        let node_text_style = flowchart_effective_text_style_for_node_classes(
            node_label_base_style,
            &model.class_defs,
            &n.classes,
            &n.styles,
        );
        let node_font_style = flowchart_effective_font_style_for_node_classes(
            &model.class_defs,
            &n.classes,
            &n.styles,
        );
        let mut metrics = flowchart_label_metrics_for_layout(FlowchartLabelMetricsRequest {
            measurer,
            raw_label,
            label_type,
            style: node_text_style.as_ref(),
            max_width_px: Some(wrapping_width),
            wrap_mode: node_wrap_mode,
            config: effective_config,
            math_renderer,
            preserve_string_whitespace_height: node_html_label_css_parity,
            whole_label_font_style: node_font_style.as_deref(),
        });
        let span_css_height_parity =
            flowchart_node_has_span_css_height_parity(&model.class_defs, &n.classes);
        if node_html_label_css_parity && span_css_height_parity {
            crate::text::flowchart_apply_mermaid_styled_node_height_parity(
                &mut metrics,
                node_text_style.as_ref(),
            );
        }
        if node_wrap_mode == WrapMode::SvgLike
            && label_type != "markdown"
            && !raw_label.contains('<')
            && !raw_label.contains('>')
            && matches!(
                n.layout_shape.as_deref().unwrap_or("squareRect"),
                "squareRect"
            )
        {
            let plain = crate::flowchart::flowchart_label_plain_text_for_layout(
                raw_label, label_type, false,
            );
            metrics.width = flowchart_svg_plain_computed_width_px(
                measurer,
                &plain,
                node_text_style.as_ref(),
                Some(wrapping_width),
            );
        }
        leaf_label_metrics_by_id.insert(n.id.clone(), (metrics.width, metrics.height));
        let (width, height) = node_layout_dimensions(NodeLayoutDimensionsRequest {
            layout_shape: n.layout_shape.as_deref(),
            layout_direction: &diagram_direction,
            metrics,
            padding: node_padding,
            state_padding,
            wrap_mode: node_wrap_mode,
            node_icon: n.icon.as_deref(),
            node_img: n.img.as_deref(),
            node_pos: n.pos.as_deref(),
            node_asset_width: n.asset_width,
            node_asset_height: n.asset_height,
        });
        leaf_node_labels.insert(
            n.id.clone(),
            NodeLabel {
                width,
                height,
                ..Default::default()
            },
        );
    }
    for sg in &model.subgraphs {
        if !sg.nodes.is_empty() {
            continue;
        }
        let label_type = sg.label_type.as_deref().unwrap_or("text");
        let sg_text_style = flowchart_effective_text_style_for_classes(
            cluster_label_base_style,
            &model.class_defs,
            &sg.classes,
            &sg.styles,
        );
        let sg_font_style =
            flowchart_effective_font_style_for_classes(&model.class_defs, &sg.classes, &sg.styles);
        let metrics = flowchart_label_metrics_for_layout(FlowchartLabelMetricsRequest {
            measurer,
            raw_label: &sg.title,
            label_type,
            style: sg_text_style.as_ref(),
            max_width_px: Some(cluster_title_wrapping_width),
            wrap_mode: node_wrap_mode,
            config: effective_config,
            math_renderer,
            preserve_string_whitespace_height: node_html_label_css_parity,
            whole_label_font_style: sg_font_style.as_deref(),
        });
        leaf_label_metrics_by_id.insert(sg.id.clone(), (metrics.width, metrics.height));
        let (width, height) = node_layout_dimensions(NodeLayoutDimensionsRequest {
            layout_shape: Some("squareRect"),
            layout_direction: &diagram_direction,
            metrics,
            padding: cluster_padding,
            state_padding,
            wrap_mode: node_wrap_mode,
            node_icon: None,
            node_img: None,
            node_pos: None,
            node_asset_width: None,
            node_asset_height: None,
        });
        leaf_node_labels.insert(
            sg.id.clone(),
            NodeLabel {
                width,
                height,
                ..Default::default()
            },
        );
    }

    // Mermaid constructs the Dagre graph by:
    // 1) inserting subgraph (cluster) nodes first (in reverse `subgraphs[]` order), then
    // 2) inserting vertex nodes (in FlowDB `Map` insertion order),
    // and setting `parentId` as each node is inserted.
    //
    // Matching this order matters because Graphlib insertion order can affect compound-graph
    // child ordering, anchor selection and deterministic tie-breaking in layout.
    let mut inserted: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut parent_assigned: std::collections::HashSet<String> = std::collections::HashSet::new();
    let insert_one = |id: &str,
                      g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
                      inserted: &mut std::collections::HashSet<String>| {
        if inserted.contains(id) {
            return;
        }
        if let Some(lbl) = cluster_node_labels.get(id).cloned() {
            g.set_node(id.to_string(), lbl);
            inserted.insert(id.to_string());
            return;
        }
        if let Some(lbl) = leaf_node_labels.get(id).cloned() {
            g.set_node(id.to_string(), lbl);
            inserted.insert(id.to_string());
        }
    };

    if has_subgraphs {
        // Match Mermaid's `FlowDB.getData()` parent assignment: build `parentId` by iterating
        // subgraphs in reverse order and recording each membership.
        let mut parent_by_id: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for sg in model.subgraphs.iter().rev() {
            for child in &sg.nodes {
                parent_by_id.insert(child.clone(), sg.id.clone());
            }
        }

        // Cyclic subgraph membership such as `A contains B` and `B contains A` is valid syntax but
        // cannot be represented in Graphlib's compound parent tree. Detect it before calling
        // `set_parent`, but keep Mermaid-compatible error wording for the offending assignment.
        if let Some((child, parent)) = first_parent_cycle_assignment(
            model.subgraphs.iter().rev().map(|sg| sg.id.as_str()),
            &parent_by_id,
        ) {
            return Err(Error::InvalidModel {
                message: format!("Setting {parent} as parent of {child} would create a cycle"),
            });
        }

        let insert_with_parent =
            |id: &str,
             g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
             inserted: &mut std::collections::HashSet<String>,
             parent_assigned: &mut std::collections::HashSet<String>| {
                insert_one(id, g, inserted);
                if !parent_assigned.insert(id.to_string()) {
                    return;
                }
                if let Some(p) = parent_by_id.get(id).cloned() {
                    g.set_parent(id.to_string(), p);
                }
            };

        for sg in model.subgraphs.iter().rev() {
            insert_with_parent(sg.id.as_str(), &mut g, &mut inserted, &mut parent_assigned);
        }
        for n in &model.nodes {
            insert_with_parent(n.id.as_str(), &mut g, &mut inserted, &mut parent_assigned);
        }
        for id in &model.vertex_calls {
            insert_with_parent(id.as_str(), &mut g, &mut inserted, &mut parent_assigned);
        }
    } else {
        // No subgraphs: insertion order still matters for deterministic Dagre tie-breaking.
        for n in &model.nodes {
            insert_one(n.id.as_str(), &mut g, &mut inserted);
        }
        for id in &model.vertex_calls {
            insert_one(id.as_str(), &mut g, &mut inserted);
        }
    }

    // Materialize self-loop helper label nodes and place them in the same parent cluster as the
    // base node (if any), matching Mermaid `@11.12.2` dagre layout adapter behavior.
    for id in &self_loop_label_node_ids {
        if !g.has_node(id) {
            g.set_node(
                id.clone(),
                NodeLabel {
                    // Mermaid initializes these labelRect nodes at 10x10, but then immediately
                    // runs `insertNode(...)` + `updateNodeBounds(...)` before Dagre layout. For an
                    // empty `labelRect`, the measured bbox collapses to ~0.1x0.1 and that is what
                    // Dagre actually sees for spacing. Match that here for layout parity.
                    width: 0.1_f32 as f64,
                    height: 0.1_f32 as f64,
                    ..Default::default()
                },
            );
        }
        let Some((base, _)) = id.split_once("---") else {
            continue;
        };
        if let Some(p) = g.parent(base) {
            g.set_parent(id.clone(), p.to_string());
        }
    }

    let effective_dir_by_id = if has_subgraphs {
        compute_effective_dir_by_id(&subgraphs_by_id, &g, &diagram_direction, inherit_dir)
    } else {
        HashMap::new()
    };

    // Map SVG edge ids to the multigraph key used by the Dagre layout graph. Most edges use their
    // `id` as the key, but Mermaid uses distinct keys for the self-loop special edges and we also
    // want deterministic ordering under our BTree-backed graph storage.
    let mut edge_key_by_id: HashMap<String, String> = HashMap::new();
    let mut edge_id_by_key: HashMap<String, String> = HashMap::new();

    for e in &render_edges {
        // Mermaid uses distinct graphlib multigraph keys for self-loop helper edges.
        // Reference: `packages/mermaid/src/rendering-util/layout-algorithms/dagre/index.js`
        let edge_key = if let Some(prefix) = e.id.strip_suffix("-cyclic-special-1") {
            format!("{prefix}-cyclic-special-0")
        } else if let Some(prefix) = e.id.strip_suffix("-cyclic-special-mid") {
            format!("{prefix}-cyclic-special-1")
        } else if let Some(prefix) = e.id.strip_suffix("-cyclic-special-2") {
            // Mermaid contains this typo in the edge key (note the `<`):
            // `nodeId + '-cyc<lic-special-2'`
            format!("{prefix}-cyc<lic-special-2")
        } else {
            e.id.clone()
        };
        edge_key_by_id.insert(e.id.clone(), edge_key.clone());
        edge_id_by_key.insert(edge_key.clone(), e.id.clone());

        let from = e.from.clone();
        let to = e.to.clone();

        if edge_label_is_non_empty(e) {
            let label_text = e.label.as_deref().unwrap_or_default();
            let label_type = e.label_type.as_deref().unwrap_or("text");
            let edge_text_style = flowchart_effective_text_style_for_classes(
                edge_label_base_style,
                &model.class_defs,
                &e.classes,
                &e.style,
            );
            let edge_font_style =
                flowchart_effective_font_style_for_classes(&model.class_defs, &e.classes, &e.style);
            let metrics = if label_type == "markdown" && edge_wrap_mode != WrapMode::HtmlLike {
                let mut metrics = crate::text::measure_wrapped_markdown_with_flowchart_bold_deltas(
                    measurer,
                    label_text,
                    edge_text_style.as_ref(),
                    Some(edge_label_wrapping_width),
                    edge_wrap_mode,
                );
                if flowchart_whole_label_font_style_requests_italic(edge_font_style.as_deref()) {
                    let plain = flowchart_label_plain_text_for_layout(
                        label_text,
                        label_type,
                        edge_wrap_mode == WrapMode::HtmlLike,
                    );
                    let italic_delta = crate::text::mermaid_default_italic_width_delta_px(
                        &plain,
                        edge_text_style.as_ref(),
                    );
                    if italic_delta > 0.0 {
                        metrics.width = crate::text::round_to_1_64_px(metrics.width + italic_delta);
                    }
                }
                metrics
            } else {
                flowchart_label_metrics_for_layout(FlowchartLabelMetricsRequest {
                    measurer,
                    raw_label: label_text,
                    label_type,
                    style: edge_text_style.as_ref(),
                    max_width_px: Some(edge_label_wrapping_width),
                    wrap_mode: edge_wrap_mode,
                    config: effective_config,
                    math_renderer,
                    preserve_string_whitespace_height: false,
                    whole_label_font_style: edge_font_style.as_deref(),
                })
            };
            let (label_width, label_height) = if edge_html_labels {
                (metrics.width.max(1.0), metrics.height.max(1.0))
            } else {
                // Mermaid's SVG edge-labels include a padded background rect (+2px left/right and
                // +2px top/bottom).
                (
                    (metrics.width + 4.0).max(1.0),
                    (metrics.height + 4.0).max(1.0),
                )
            };

            let minlen = e.length.max(1);
            let el = EdgeLabel {
                width: label_width,
                height: label_height,
                labelpos: LabelPos::C,
                // Dagre layout defaults `labeloffset` to 10 when unspecified.
                labeloffset: 10.0,
                minlen,
                weight: 1.0,
                ..Default::default()
            };

            g.set_edge_named(from, to, Some(edge_key), Some(el));
        } else {
            let el = EdgeLabel {
                width: 0.0,
                height: 0.0,
                labelpos: LabelPos::C,
                // Dagre layout defaults `labeloffset` to 10 when unspecified.
                labeloffset: 10.0,
                minlen: e.length.max(1),
                weight: 1.0,
                ..Default::default()
            };
            g.set_edge_named(from, to, Some(edge_key), Some(el));
        }
    }

    let external_connections_by_id = if has_subgraphs {
        adjust_flowchart_clusters_and_edges(&mut g)
    } else {
        std::collections::HashMap::new()
    };

    let mut edge_endpoints_by_id: HashMap<String, (String, String)> = HashMap::new();
    for ek in g.edge_keys() {
        let Some(edge_key) = ek.name.as_deref() else {
            continue;
        };
        let edge_id = edge_id_by_key
            .get(edge_key)
            .cloned()
            .unwrap_or_else(|| edge_key.to_string());
        edge_endpoints_by_id.insert(edge_id, (ek.v.clone(), ek.w.clone()));
    }

    if let Some(s) = build_graph_start {
        timings.build_graph = s.elapsed();
    }

    let mut extracted_graphs: std::collections::HashMap<
        String,
        Graph<NodeLabel, EdgeLabel, GraphLabel>,
    > = std::collections::HashMap::new();
    if has_subgraphs {
        let extract_start = timing_enabled.then(web_time::Instant::now);
        extract_clusters_recursively(
            &mut g,
            &subgraphs_by_id,
            &external_connections_by_id,
            &mut extracted_graphs,
            0,
        );
        if let Some(s) = extract_start {
            timings.extract_clusters = s.elapsed();
        }
    }

    // Mermaid's flowchart-v2 renderer inserts node DOM elements in `graph.nodes()` order before
    // running Dagre layout, including for recursively extracted cluster graphs. Capture that
    // insertion order per root so the headless SVG matches strict DOM expectations.
    let mut dom_node_order_by_root: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let dom_order_start = timing_enabled.then(web_time::Instant::now);
    dom_node_order_by_root.insert(String::new(), g.node_ids());
    for (id, cg) in &extracted_graphs {
        dom_node_order_by_root.insert(id.clone(), cg.node_ids());
    }
    if let Some(s) = dom_order_start {
        timings.dom_order = s.elapsed();
    }

    type Rect = merman_core::geom::Box2;

    struct ClusterTitleMetricsContext<'a> {
        subgraphs_by_id: &'a std::collections::HashMap<String, FlowSubgraph>,
        class_defs: &'a indexmap::IndexMap<String, Vec<String>>,
        measurer: &'a dyn TextMeasurer,
        text_style: &'a TextStyle,
        html_label_text_style: &'a TextStyle,
        title_wrapping_width: f64,
        wrap_mode: WrapMode,
        config: &'a MermaidConfig,
        math_renderer: Option<&'a (dyn MathRenderer + Send + Sync)>,
    }

    fn cluster_title_metrics_for_layout(
        id: &str,
        ctx: &ClusterTitleMetricsContext<'_>,
    ) -> Option<(f64, f64)> {
        let sg = ctx.subgraphs_by_id.get(id)?;
        let label_type = sg.label_type.as_deref().unwrap_or("text");
        let title_font_style =
            flowchart_effective_font_style_for_classes(ctx.class_defs, &sg.classes, &sg.styles);
        let title_width_limit = (label_type == "markdown").then_some(ctx.title_wrapping_width);
        let metrics = flowchart_label_metrics_for_layout(FlowchartLabelMetricsRequest {
            measurer: ctx.measurer,
            raw_label: &sg.title,
            label_type,
            style: if ctx.wrap_mode == WrapMode::HtmlLike {
                ctx.html_label_text_style
            } else {
                ctx.text_style
            },
            max_width_px: title_width_limit,
            wrap_mode: ctx.wrap_mode,
            config: ctx.config,
            math_renderer: ctx.math_renderer,
            preserve_string_whitespace_height: false,
            whole_label_font_style: title_font_style.as_deref(),
        });
        Some((metrics.width.max(1.0), metrics.height.max(1.0)))
    }

    fn extracted_graph_bbox_rect(
        g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
        title_total_margin: f64,
        extracted: &std::collections::HashMap<String, Graph<NodeLabel, EdgeLabel, GraphLabel>>,
        subgraph_id_set: &std::collections::HashSet<String>,
        title_metrics_ctx: &ClusterTitleMetricsContext<'_>,
        cluster_padding: f64,
    ) -> Option<Rect> {
        fn graph_content_rect(
            g: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
            extracted: &std::collections::HashMap<String, Graph<NodeLabel, EdgeLabel, GraphLabel>>,
            subgraph_id_set: &std::collections::HashSet<String>,
            title_total_margin: f64,
            title_metrics_ctx: &ClusterTitleMetricsContext<'_>,
            cluster_padding: f64,
        ) -> Option<Rect> {
            let mut out: Option<Rect> = None;
            for id in g.node_ids() {
                let Some(n) = g.node(&id) else { continue };
                let (Some(x), Some(y)) = (n.x, n.y) else {
                    continue;
                };
                let mut width = n.width;
                let mut height = n.height;
                let is_cluster_node = extracted.contains_key(&id) && g.children(&id).is_empty();
                let is_non_recursive_cluster =
                    subgraph_id_set.contains(&id) && !g.children(&id).is_empty();

                // Mermaid increases cluster node height by `subGraphTitleTotalMargin` *after* Dagre
                // layout (just before rendering), and `updateNodeBounds(...)` measures the DOM
                // bbox after that expansion. Mirror that here for non-recursive clusters.
                //
                // For leaf clusterNodes (recursively rendered clusters), the node's width/height
                // comes directly from `updateNodeBounds(...)`, so do not add margins again.
                if !is_cluster_node && is_non_recursive_cluster {
                    if title_total_margin > 0.0 {
                        height = (height + title_total_margin).max(1.0);
                    }
                    if let Some((title_w, title_h)) =
                        cluster_title_metrics_for_layout(&id, title_metrics_ctx)
                    {
                        width = width.max(title_w + cluster_padding);
                        height = height.max(title_h + title_total_margin);
                    }
                }

                let r = Rect::from_center(x, y, width, height);
                if let Some(ref mut cur) = out {
                    cur.union(r);
                } else {
                    out = Some(r);
                }
            }
            for ek in g.edge_keys() {
                let Some(e) = g.edge_by_key(&ek) else {
                    continue;
                };
                let (Some(x), Some(y)) = (e.x, e.y) else {
                    continue;
                };
                if e.width <= 0.0 && e.height <= 0.0 {
                    continue;
                }
                let r = Rect::from_center(x, y, e.width, e.height);
                if let Some(ref mut cur) = out {
                    cur.union(r);
                } else {
                    out = Some(r);
                }
            }
            out
        }

        graph_content_rect(
            g,
            extracted,
            subgraph_id_set,
            title_total_margin,
            title_metrics_ctx,
            cluster_padding,
        )
    }

    fn apply_mermaid_subgraph_title_shifts(
        graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
        extracted: &std::collections::HashMap<String, Graph<NodeLabel, EdgeLabel, GraphLabel>>,
        subgraph_id_set: &std::collections::HashSet<String>,
        y_shift: f64,
    ) {
        if y_shift.abs() < 1e-9 {
            return;
        }

        // Mermaid v11.12.2 adjusts Y positions after Dagre layout:
        // - regular nodes: +subGraphTitleTotalMargin/2
        // - clusterNode nodes (recursively rendered clusters): +subGraphTitleTotalMargin
        // - pure cluster nodes (non-recursive clusters): no y-shift (but height grows elsewhere)
        for id in graph.node_ids() {
            // A cluster is only a Mermaid "clusterNode" placeholder if it is a leaf in the
            // current graph. Extracted graphs contain an injected parent cluster node with the
            // same id (and children), which must follow the pure-cluster path.
            let is_cluster_node = extracted.contains_key(&id) && graph.children(&id).is_empty();
            let delta_y = if is_cluster_node {
                y_shift * 2.0
            } else if subgraph_id_set.contains(&id) && !graph.children(&id).is_empty() {
                0.0
            } else {
                y_shift
            };
            if delta_y.abs() > 1e-9 {
                let Some(y) = graph.node(&id).and_then(|n| n.y) else {
                    continue;
                };
                if let Some(n) = graph.node_mut(&id) {
                    n.y = Some(y + delta_y);
                }
            }
        }

        // Mermaid shifts all edge points and the edge label position by +subGraphTitleTotalMargin/2.
        for ek in graph.edge_keys() {
            let Some(e) = graph.edge_mut_by_key(&ek) else {
                continue;
            };
            if let Some(y) = e.y {
                e.y = Some(y + y_shift);
            }
            for p in &mut e.points {
                p.y += y_shift;
            }
        }
    }

    struct RecursiveLayoutContext<'a> {
        extracted:
            &'a mut std::collections::HashMap<String, Graph<NodeLabel, EdgeLabel, GraphLabel>>,
        subgraph_id_set: &'a std::collections::HashSet<String>,
        y_shift: f64,
        cluster_node_labels: &'a std::collections::HashMap<String, NodeLabel>,
        title_total_margin: f64,
        title_metrics_ctx: &'a ClusterTitleMetricsContext<'a>,
        cluster_padding: f64,
        timings: &'a mut FlowchartLayoutTimings,
        timing_enabled: bool,
    }

    fn layout_graph_with_recursive_clusters(
        graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
        graph_cluster_id: Option<&str>,
        _depth: usize,
        ctx: &mut RecursiveLayoutContext<'_>,
    ) {
        #[derive(Clone)]
        struct LayoutFrame {
            cluster_id: Option<String>,
            expanded: bool,
        }

        fn recursive_child_ids(
            graph: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
            extracted: &std::collections::HashMap<String, Graph<NodeLabel, EdgeLabel, GraphLabel>>,
        ) -> Vec<String> {
            graph
                .node_ids()
                .into_iter()
                // Only recurse into extracted graphs for leaf cluster nodes ("clusterNode" in
                // Mermaid). Child graphs also get their parent cluster node injected, with
                // children, and must not recurse back into themselves.
                .filter(|id| graph.children(id).is_empty() && extracted.contains_key(id))
                .collect()
        }

        fn inject_parent_cluster(
            graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
            cluster_id: &str,
            ctx: &RecursiveLayoutContext<'_>,
        ) {
            if !graph.has_node(cluster_id) {
                let lbl = ctx
                    .cluster_node_labels
                    .get(cluster_id)
                    .cloned()
                    .unwrap_or_default();
                graph.set_node(cluster_id.to_string(), lbl);
            }
            let node_ids = graph.node_ids();
            for node_id in node_ids {
                if node_id == cluster_id {
                    continue;
                }
                if graph.parent(&node_id).is_none() {
                    graph.set_parent(node_id, cluster_id.to_string());
                }
            }
        }

        fn update_child_cluster_bounds(
            graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
            ctx: &RecursiveLayoutContext<'_>,
        ) {
            let ids = recursive_child_ids(graph, ctx.extracted);
            for id in ids {
                let Some(child) = ctx.extracted.get(&id) else {
                    continue;
                };

                // In Mermaid, `updateNodeBounds(...)` measures the recursively rendered `<g
                // class="root">` group. In that render path, the child graph contains a node
                // matching the cluster id (inserted via `graph.setNode(parentCluster.id, ...)`),
                // whose computed compound bounds correspond to the cluster box measured in the DOM.
                if let Some(r) = extracted_graph_bbox_rect(
                    child,
                    ctx.title_total_margin,
                    ctx.extracted,
                    ctx.subgraph_id_set,
                    ctx.title_metrics_ctx,
                    ctx.cluster_padding,
                ) {
                    if let Some(n) = graph.node_mut(&id) {
                        n.width = r.width().max(1.0);
                        n.height = r.height().max(1.0);
                    }
                } else if let Some(n_child) = child.node(&id) {
                    if let Some(n) = graph.node_mut(&id) {
                        n.width = n_child.width.max(1.0);
                        n.height = n_child.height.max(1.0);
                    }
                }
            }
        }

        fn layout_one_graph(
            graph: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>,
            ctx: &mut RecursiveLayoutContext<'_>,
        ) {
            if ctx.timing_enabled {
                ctx.timings.dagre_calls += 1;
                let start = web_time::Instant::now();
                dugong::layout_dagreish(graph);
                ctx.timings.dagre_total += start.elapsed();
            } else {
                dugong::layout_dagreish(graph);
            }
            apply_mermaid_subgraph_title_shifts(
                graph,
                ctx.extracted,
                ctx.subgraph_id_set,
                ctx.y_shift,
            );
        }

        let mut stack = vec![LayoutFrame {
            cluster_id: graph_cluster_id.map(str::to_string),
            expanded: false,
        }];

        while let Some(frame) = stack.pop() {
            if !frame.expanded {
                let Some((child_ids, parent_nodesep, parent_ranksep)) = (match &frame.cluster_id {
                    Some(id) => ctx.extracted.get(id).map(|g| {
                        (
                            recursive_child_ids(g, ctx.extracted),
                            g.graph().nodesep,
                            g.graph().ranksep,
                        )
                    }),
                    None => Some((
                        recursive_child_ids(graph, ctx.extracted),
                        graph.graph().nodesep,
                        graph.graph().ranksep,
                    )),
                }) else {
                    continue;
                };

                stack.push(LayoutFrame {
                    cluster_id: frame.cluster_id,
                    expanded: true,
                });

                for child_id in child_ids.iter().rev() {
                    // Match Mermaid `recursiveRender` behavior: before laying out a recursively
                    // rendered cluster graph, override `nodesep` to the parent graph spacing and
                    // `ranksep` to `parent.ranksep + 25`. This compounds for nested recursive
                    // clusters.
                    if let Some(child) = ctx.extracted.get_mut(child_id) {
                        child.graph_mut().nodesep = parent_nodesep;
                        child.graph_mut().ranksep = parent_ranksep + 25.0;
                    }
                    stack.push(LayoutFrame {
                        cluster_id: Some(child_id.clone()),
                        expanded: false,
                    });
                }
                continue;
            }

            if let Some(cluster_id) = frame.cluster_id {
                let Some(mut current) = ctx.extracted.remove(&cluster_id) else {
                    continue;
                };
                update_child_cluster_bounds(&mut current, ctx);
                inject_parent_cluster(&mut current, &cluster_id, ctx);
                layout_one_graph(&mut current, ctx);
                ctx.extracted.insert(cluster_id, current);
            } else {
                update_child_cluster_bounds(graph, ctx);
                layout_one_graph(graph, ctx);
            }
        }
    }

    let layout_start = timing_enabled.then(web_time::Instant::now);
    {
        let title_metrics_ctx = ClusterTitleMetricsContext {
            subgraphs_by_id: &subgraphs_by_id,
            class_defs: &model.class_defs,
            measurer,
            text_style: &text_style,
            html_label_text_style: &html_label_text_style,
            title_wrapping_width: cluster_title_wrapping_width,
            wrap_mode: cluster_wrap_mode,
            config: effective_config,
            math_renderer,
        };
        let mut recursive_layout_ctx = RecursiveLayoutContext {
            extracted: &mut extracted_graphs,
            subgraph_id_set: &subgraph_id_set,
            y_shift,
            cluster_node_labels: &cluster_node_labels,
            title_total_margin,
            title_metrics_ctx: &title_metrics_ctx,
            cluster_padding,
            timings: &mut timings,
            timing_enabled,
        };
        layout_graph_with_recursive_clusters(&mut g, None, 0, &mut recursive_layout_ctx);
    }
    if let Some(s) = layout_start {
        timings.layout_recursive = s.elapsed();
    }

    let mut leaf_rects: std::collections::HashMap<String, Rect> = std::collections::HashMap::new();
    let mut base_pos: std::collections::HashMap<String, (f64, f64)> =
        std::collections::HashMap::new();
    let mut edge_override_points: std::collections::HashMap<String, Vec<LayoutPoint>> =
        std::collections::HashMap::new();
    let mut edge_override_label: std::collections::HashMap<String, Option<LayoutLabel>> =
        std::collections::HashMap::new();
    let mut edge_override_from_cluster: std::collections::HashMap<String, Option<String>> =
        std::collections::HashMap::new();
    let mut edge_override_to_cluster: std::collections::HashMap<String, Option<String>> =
        std::collections::HashMap::new();
    let mut leaf_node_ids: std::collections::HashSet<String> = model
        .nodes
        .iter()
        .filter(|n| !subgraph_ids.contains(n.id.as_str()))
        .map(|n| n.id.clone())
        .collect();
    for id in &self_loop_label_node_ids {
        leaf_node_ids.insert(id.clone());
    }
    for id in &empty_subgraph_ids {
        leaf_node_ids.insert(id.clone());
    }

    struct PlaceGraphInputs<'a> {
        edge_id_by_key: &'a std::collections::HashMap<String, String>,
        extracted_graphs:
            &'a std::collections::HashMap<String, Graph<NodeLabel, EdgeLabel, GraphLabel>>,
        subgraph_ids: &'a std::collections::HashSet<&'a str>,
        leaf_node_ids: &'a std::collections::HashSet<String>,
    }

    struct PlaceGraphOutputs<'a> {
        base_pos: &'a mut std::collections::HashMap<String, (f64, f64)>,
        leaf_rects: &'a mut std::collections::HashMap<String, Rect>,
        cluster_rects_from_graph: &'a mut std::collections::HashMap<String, Rect>,
        extracted_cluster_rects: &'a mut std::collections::HashMap<String, Rect>,
        extracted_cluster_base_widths: &'a mut std::collections::HashMap<String, f64>,
        edge_override_points: &'a mut std::collections::HashMap<String, Vec<LayoutPoint>>,
        edge_override_label: &'a mut std::collections::HashMap<String, Option<LayoutLabel>>,
        edge_override_from_cluster: &'a mut std::collections::HashMap<String, Option<String>>,
        edge_override_to_cluster: &'a mut std::collections::HashMap<String, Option<String>>,
    }

    fn place_graph(
        graph: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
        offset: (f64, f64),
        is_root: bool,
        inputs: &PlaceGraphInputs<'_>,
        out: &mut PlaceGraphOutputs<'_>,
    ) {
        struct PlaceFrame<'a> {
            graph: &'a Graph<NodeLabel, EdgeLabel, GraphLabel>,
            offset: (f64, f64),
            is_root: bool,
        }

        fn subtree_rect_iterative(
            graph: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
            id: &str,
        ) -> Option<Rect> {
            let mut visiting: std::collections::HashSet<String> = std::collections::HashSet::new();
            if !visiting.insert(id.to_string()) {
                return None;
            }
            let mut out: Option<Rect> = None;
            let mut stack: Vec<String> = graph.children(id).iter().map(|s| s.to_string()).collect();
            while let Some(node) = stack.pop() {
                if let Some(n) = graph.node(&node) {
                    if let (Some(x), Some(y)) = (n.x, n.y) {
                        let r = Rect::from_center(x, y, n.width, n.height);
                        if let Some(ref mut cur) = out {
                            cur.union(r);
                        } else {
                            out = Some(r);
                        }
                    }
                }
                for child in graph.children(&node) {
                    if visiting.insert(child.to_string()) {
                        stack.push(child.to_string());
                    }
                }
            }
            out
        }

        let mut stack = vec![PlaceFrame {
            graph,
            offset,
            is_root,
        }];
        while let Some(frame) = stack.pop() {
            for id in frame.graph.node_ids() {
                let Some(n) = frame.graph.node(&id) else {
                    continue;
                };
                let x = n.x.unwrap_or(0.0) + frame.offset.0;
                let y = n.y.unwrap_or(0.0) + frame.offset.1;
                if inputs.leaf_node_ids.contains(&id) {
                    out.base_pos.insert(id.clone(), (x, y));
                    out.leaf_rects
                        .insert(id.clone(), Rect::from_center(x, y, n.width, n.height));
                    continue;
                }
            }

            // Capture the layout-computed compound bounds for non-extracted clusters.
            //
            // Upstream Dagre computes compound-node geometry from border nodes and then removes the
            // border dummy nodes (`removeBorderNodes`). Our dugong parity pipeline mirrors that, so
            // prefer the compound node's own x/y/width/height when available.
            for id in frame.graph.node_ids() {
                if !inputs.subgraph_ids.contains(id.as_str()) {
                    continue;
                }
                if inputs.extracted_graphs.contains_key(&id) {
                    continue;
                }
                if out.cluster_rects_from_graph.contains_key(&id) {
                    continue;
                }
                if let Some(n) = frame.graph.node(&id) {
                    if let (Some(x), Some(y)) = (n.x, n.y) {
                        if n.width > 0.0 && n.height > 0.0 {
                            let mut r = Rect::from_center(x, y, n.width, n.height);
                            r.translate(frame.offset.0, frame.offset.1);
                            out.cluster_rects_from_graph.insert(id, r);
                            continue;
                        }
                    }
                }

                let Some(mut r) = subtree_rect_iterative(frame.graph, &id) else {
                    continue;
                };
                r.translate(frame.offset.0, frame.offset.1);
                out.cluster_rects_from_graph.insert(id, r);
            }

            for ek in frame.graph.edge_keys() {
                let Some(edge_key) = ek.name.as_deref() else {
                    continue;
                };
                let edge_id = inputs
                    .edge_id_by_key
                    .get(edge_key)
                    .map(String::as_str)
                    .unwrap_or(edge_key);
                let Some(lbl) = frame.graph.edge_by_key(&ek) else {
                    continue;
                };

                if let (Some(x), Some(y)) = (lbl.x, lbl.y) {
                    if lbl.width > 0.0 || lbl.height > 0.0 {
                        let lx = x + frame.offset.0;
                        let ly = y + frame.offset.1;
                        let leaf_id = format!("edge-label::{edge_id}");
                        out.base_pos.insert(leaf_id.clone(), (lx, ly));
                        out.leaf_rects
                            .insert(leaf_id, Rect::from_center(lx, ly, lbl.width, lbl.height));
                    }
                }

                if !frame.is_root {
                    let points = lbl
                        .points
                        .iter()
                        .map(|p| LayoutPoint {
                            x: p.x + frame.offset.0,
                            y: p.y + frame.offset.1,
                        })
                        .collect::<Vec<_>>();
                    let label_pos = match (lbl.x, lbl.y) {
                        (Some(x), Some(y)) if lbl.width > 0.0 || lbl.height > 0.0 => {
                            Some(LayoutLabel {
                                x: x + frame.offset.0,
                                y: y + frame.offset.1,
                                width: lbl.width,
                                height: lbl.height,
                            })
                        }
                        _ => None,
                    };
                    out.edge_override_points.insert(edge_id.to_string(), points);
                    out.edge_override_label
                        .insert(edge_id.to_string(), label_pos);
                    let from_cluster = lbl
                        .extras
                        .get("fromCluster")
                        .and_then(|v| v.as_str().map(|s| s.to_string()));
                    let to_cluster = lbl
                        .extras
                        .get("toCluster")
                        .and_then(|v| v.as_str().map(|s| s.to_string()));
                    out.edge_override_from_cluster
                        .insert(edge_id.to_string(), from_cluster);
                    out.edge_override_to_cluster
                        .insert(edge_id.to_string(), to_cluster);
                }
            }

            let mut child_frames = Vec::new();
            for id in frame.graph.node_ids() {
                // Only recurse into extracted graphs for leaf cluster nodes ("clusterNode" in Mermaid).
                // The recursively rendered graph itself also contains a node with the same id (the
                // parent cluster node injected before layout), which has children and must not recurse.
                if !frame.graph.children(&id).is_empty() {
                    continue;
                }
                let Some(child) = inputs.extracted_graphs.get(&id) else {
                    continue;
                };
                let Some(n) = frame.graph.node(&id) else {
                    continue;
                };
                let (Some(px), Some(py)) = (n.x, n.y) else {
                    continue;
                };
                let parent_x = px + frame.offset.0;
                let parent_y = py + frame.offset.1;
                let Some(cnode) = child.node(&id) else {
                    continue;
                };
                let (Some(cx), Some(cy)) = (cnode.x, cnode.y) else {
                    continue;
                };
                let child_offset = (parent_x - cx, parent_y - cy);
                // The extracted cluster's footprint in the parent graph is the clusterNode itself.
                // Our recursive layout step updates the parent graph's node `width/height` to match
                // Mermaid's `updateNodeBounds(...)` behavior (including any title margin). Avoid
                // adding `title_total_margin` again here.
                let r = Rect::from_center(parent_x, parent_y, n.width, n.height);
                out.extracted_cluster_rects.insert(id.clone(), r);
                out.extracted_cluster_base_widths
                    .insert(id.clone(), cnode.width.max(1.0));
                child_frames.push(PlaceFrame {
                    graph: child,
                    offset: child_offset,
                    is_root: false,
                });
            }
            for child in child_frames.into_iter().rev() {
                stack.push(child);
            }
        }
    }

    let mut cluster_rects_from_graph: std::collections::HashMap<String, Rect> =
        std::collections::HashMap::new();
    let mut extracted_cluster_rects: std::collections::HashMap<String, Rect> =
        std::collections::HashMap::new();
    let mut extracted_cluster_base_widths: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    let place_start = timing_enabled.then(web_time::Instant::now);
    {
        let place_graph_inputs = PlaceGraphInputs {
            edge_id_by_key: &edge_id_by_key,
            extracted_graphs: &extracted_graphs,
            subgraph_ids: &subgraph_ids,
            leaf_node_ids: &leaf_node_ids,
        };
        let mut place_graph_outputs = PlaceGraphOutputs {
            base_pos: &mut base_pos,
            leaf_rects: &mut leaf_rects,
            cluster_rects_from_graph: &mut cluster_rects_from_graph,
            extracted_cluster_rects: &mut extracted_cluster_rects,
            extracted_cluster_base_widths: &mut extracted_cluster_base_widths,
            edge_override_points: &mut edge_override_points,
            edge_override_label: &mut edge_override_label,
            edge_override_from_cluster: &mut edge_override_from_cluster,
            edge_override_to_cluster: &mut edge_override_to_cluster,
        };
        place_graph(
            &g,
            (0.0, 0.0),
            true,
            &place_graph_inputs,
            &mut place_graph_outputs,
        );
    }
    if let Some(s) = place_start {
        timings.place_graph = s.elapsed();
    }

    let build_output_start = timing_enabled.then(web_time::Instant::now);

    let mut extra_children: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let labeled_edges: std::collections::HashSet<&str> = render_edges
        .iter()
        .filter(|e| edge_label_is_non_empty(e))
        .map(|e| e.id.as_str())
        .collect();

    fn collect_extra_children(
        graph: &Graph<NodeLabel, EdgeLabel, GraphLabel>,
        edge_id_by_key: &std::collections::HashMap<String, String>,
        labeled_edges: &std::collections::HashSet<&str>,
        implicit_root: Option<&str>,
        out: &mut std::collections::HashMap<String, Vec<String>>,
    ) {
        for ek in graph.edge_keys() {
            let Some(edge_key) = ek.name.as_deref() else {
                continue;
            };
            let edge_id = edge_id_by_key
                .get(edge_key)
                .map(String::as_str)
                .unwrap_or(edge_key);
            if !labeled_edges.contains(edge_id) {
                continue;
            }
            // Mermaid's recursive cluster extractor removes the root cluster node from the
            // extracted graph. In that case, the "lowest common parent" of edges whose endpoints
            // belong to the extracted cluster becomes `None`, even though the label should still
            // participate in the extracted cluster's bounding box. Use `implicit_root` to map
            // those labels back to the extracted cluster id.
            let parent = lowest_common_parent(graph, &ek.v, &ek.w)
                .or_else(|| implicit_root.map(|s| s.to_string()));
            let Some(parent) = parent else {
                continue;
            };
            out.entry(parent)
                .or_default()
                .push(format!("edge-label::{edge_id}"));
        }
    }

    collect_extra_children(
        &g,
        &edge_id_by_key,
        &labeled_edges,
        None,
        &mut extra_children,
    );
    for (cluster_id, cg) in &extracted_graphs {
        collect_extra_children(
            cg,
            &edge_id_by_key,
            &labeled_edges,
            Some(cluster_id.as_str()),
            &mut extra_children,
        );
    }

    // Ensure Mermaid-style self-loop helper nodes participate in cluster bounding/packing.
    // These nodes are not part of the semantic `subgraph ... end` membership list, but are
    // parented into the same clusters as their base node.
    for id in &self_loop_label_node_ids {
        if let Some(p) = g.parent(id) {
            extra_children
                .entry(p.to_string())
                .or_default()
                .push(id.clone());
        }
    }

    let mut out_nodes: Vec<LayoutNode> = Vec::new();
    for n in &model.nodes {
        if subgraph_ids.contains(n.id.as_str()) {
            continue;
        }
        let (x, y) = base_pos
            .get(&n.id)
            .copied()
            .ok_or_else(|| Error::InvalidModel {
                message: format!("missing positioned node {}", n.id),
            })?;
        let (width, height) = leaf_rects
            .get(&n.id)
            .map(|r| (r.width(), r.height()))
            .ok_or_else(|| Error::InvalidModel {
                message: format!("missing sized node {}", n.id),
            })?;
        out_nodes.push(LayoutNode {
            id: n.id.clone(),
            x,
            y,
            width,
            height,
            is_cluster: false,
            label_width: leaf_label_metrics_by_id.get(&n.id).map(|v| v.0),
            label_height: leaf_label_metrics_by_id.get(&n.id).map(|v| v.1),
        });
    }
    for id in &empty_subgraph_ids {
        let (x, y) = base_pos
            .get(id)
            .copied()
            .ok_or_else(|| Error::InvalidModel {
                message: format!("missing positioned node {id}"),
            })?;
        let (width, height) = leaf_rects
            .get(id)
            .map(|r| (r.width(), r.height()))
            .ok_or_else(|| Error::InvalidModel {
                message: format!("missing sized node {id}"),
            })?;
        out_nodes.push(LayoutNode {
            id: id.clone(),
            x,
            y,
            width,
            height,
            is_cluster: false,
            label_width: leaf_label_metrics_by_id.get(id).map(|v| v.0),
            label_height: leaf_label_metrics_by_id.get(id).map(|v| v.1),
        });
    }
    for id in &self_loop_label_node_ids {
        let (x, y) = base_pos
            .get(id)
            .copied()
            .ok_or_else(|| Error::InvalidModel {
                message: format!("missing positioned node {id}"),
            })?;
        let (width, height) = leaf_rects
            .get(id)
            .map(|r| (r.width(), r.height()))
            .ok_or_else(|| Error::InvalidModel {
                message: format!("missing sized node {id}"),
            })?;
        out_nodes.push(LayoutNode {
            id: id.clone(),
            x,
            y,
            width,
            height,
            is_cluster: false,
            label_width: None,
            label_height: None,
        });
    }

    let mut clusters: Vec<LayoutCluster> = Vec::new();

    let mut cluster_rects: std::collections::HashMap<String, Rect> =
        std::collections::HashMap::new();
    let mut cluster_base_widths: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    let mut visiting: std::collections::HashSet<String> = std::collections::HashSet::new();

    struct ClusterRectContext<'a> {
        subgraphs_by_id: &'a std::collections::HashMap<String, FlowSubgraph>,
        class_defs: &'a indexmap::IndexMap<String, Vec<String>>,
        leaf_rects: &'a std::collections::HashMap<String, Rect>,
        extra_children: &'a std::collections::HashMap<String, Vec<String>>,
        measurer: &'a dyn TextMeasurer,
        text_style: &'a TextStyle,
        html_label_text_style: &'a TextStyle,
        title_wrapping_width: f64,
        wrap_mode: WrapMode,
        config: &'a MermaidConfig,
        math_renderer: Option<&'a (dyn MathRenderer + Send + Sync)>,
        cluster_padding: f64,
        title_total_margin: f64,
    }

    struct ClusterRectState<'a> {
        cluster_rects: &'a mut std::collections::HashMap<String, Rect>,
        cluster_base_widths: &'a mut std::collections::HashMap<String, f64>,
        visiting: &'a mut std::collections::HashSet<String>,
    }

    fn compute_cluster_rect(
        id: &str,
        ctx: &ClusterRectContext<'_>,
        state: &mut ClusterRectState<'_>,
    ) -> Result<(Rect, f64)> {
        if let Some(r) = state.cluster_rects.get(id).copied() {
            let base_width = state
                .cluster_base_widths
                .get(id)
                .copied()
                .unwrap_or_else(|| r.width());
            return Ok((r, base_width));
        }

        struct ClusterRectFrame {
            id: String,
            expanded: bool,
        }

        let mut stack = vec![ClusterRectFrame {
            id: id.to_string(),
            expanded: false,
        }];
        while let Some(frame) = stack.pop() {
            if state.cluster_rects.contains_key(&frame.id) {
                continue;
            }

            if !frame.expanded {
                if !state.visiting.insert(frame.id.clone()) {
                    return Err(Error::InvalidModel {
                        message: format!("cycle in subgraph membership involving {}", frame.id),
                    });
                }

                let Some(sg) = ctx.subgraphs_by_id.get(&frame.id) else {
                    return Err(Error::InvalidModel {
                        message: format!("missing subgraph definition for {}", frame.id),
                    });
                };

                stack.push(ClusterRectFrame {
                    id: frame.id.clone(),
                    expanded: true,
                });
                for member in sg.nodes.iter().rev() {
                    if ctx.subgraphs_by_id.contains_key(member)
                        && !state.cluster_rects.contains_key(member)
                    {
                        stack.push(ClusterRectFrame {
                            id: member.clone(),
                            expanded: false,
                        });
                    }
                }
                continue;
            }

            let Some(sg) = ctx.subgraphs_by_id.get(&frame.id) else {
                return Err(Error::InvalidModel {
                    message: format!("missing subgraph definition for {}", frame.id),
                });
            };

            let mut content: Option<Rect> = None;
            for member in &sg.nodes {
                let member_rect = if let Some(r) = ctx.leaf_rects.get(member).copied() {
                    Some(r)
                } else if ctx.subgraphs_by_id.contains_key(member) {
                    Some(state.cluster_rects.get(member).copied().ok_or_else(|| {
                        Error::InvalidModel {
                            message: format!("missing computed subgraph rect for {member}"),
                        }
                    })?)
                } else {
                    None
                };

                if let Some(r) = member_rect {
                    if let Some(ref mut cur) = content {
                        cur.union(r);
                    } else {
                        content = Some(r);
                    }
                }
            }

            if let Some(extra) = ctx.extra_children.get(&frame.id) {
                for child in extra {
                    if let Some(r) = ctx.leaf_rects.get(child).copied() {
                        if let Some(ref mut cur) = content {
                            cur.union(r);
                        } else {
                            content = Some(r);
                        }
                    }
                }
            }

            let label_type = sg.label_type.as_deref().unwrap_or("text");
            let title_width_limit = (label_type == "markdown").then_some(ctx.title_wrapping_width);
            let title_font_style =
                flowchart_effective_font_style_for_classes(ctx.class_defs, &sg.classes, &sg.styles);
            let title_metrics = flowchart_label_metrics_for_layout(FlowchartLabelMetricsRequest {
                measurer: ctx.measurer,
                raw_label: &sg.title,
                label_type,
                style: if ctx.wrap_mode == WrapMode::HtmlLike {
                    ctx.html_label_text_style
                } else {
                    ctx.text_style
                },
                max_width_px: title_width_limit,
                wrap_mode: ctx.wrap_mode,
                config: ctx.config,
                math_renderer: ctx.math_renderer,
                preserve_string_whitespace_height: false,
                whole_label_font_style: title_font_style.as_deref(),
            });
            let mut rect = if let Some(r) = content {
                r
            } else {
                Rect::from_center(
                    0.0,
                    0.0,
                    title_metrics.width.max(1.0),
                    title_metrics.height.max(1.0),
                )
            };

            // Expand to provide the cluster's internal padding.
            rect.pad(ctx.cluster_padding);

            // Mermaid computes `node.diff` using the pre-widened layout node width, then may widen the
            // rect to fit the label bbox during rendering.
            let base_width = rect.width();

            // Mermaid cluster "rect" rendering widens to fit the raw title bbox, plus a small
            // horizontal inset. Empirically (Mermaid@11.12.2 fixtures), this behaves like
            // `title_width + cluster_padding` when the title is wider than the content.
            let min_width = title_metrics.width.max(1.0) + ctx.cluster_padding;
            if rect.width() < min_width {
                let (cx, cy) = rect.center();
                rect = Rect::from_center(cx, cy, min_width, rect.height());
            }

            // Extend height to reserve space for subgraph title margins (Mermaid does this after layout).
            if ctx.title_total_margin > 0.0 {
                let (cx, cy) = rect.center();
                rect =
                    Rect::from_center(cx, cy, rect.width(), rect.height() + ctx.title_total_margin);
            }

            // Keep the cluster tall enough to accommodate the title bbox if needed.
            let min_height = title_metrics.height.max(1.0) + ctx.title_total_margin;
            if rect.height() < min_height {
                let (cx, cy) = rect.center();
                rect = Rect::from_center(cx, cy, rect.width(), min_height);
            }

            state.visiting.remove(&frame.id);
            state.cluster_rects.insert(frame.id.clone(), rect);
            state.cluster_base_widths.insert(frame.id, base_width);
        }

        let rect = state
            .cluster_rects
            .get(id)
            .copied()
            .ok_or_else(|| Error::InvalidModel {
                message: format!("missing computed subgraph rect for {id}"),
            })?;
        let base_width = state
            .cluster_base_widths
            .get(id)
            .copied()
            .unwrap_or_else(|| rect.width());
        Ok((rect, base_width))
    }

    struct ClusterTitleAdjustContext<'a> {
        class_defs: &'a indexmap::IndexMap<String, Vec<String>>,
        measurer: &'a dyn TextMeasurer,
        text_style: &'a TextStyle,
        html_label_text_style: &'a TextStyle,
        title_wrapping_width: f64,
        wrap_mode: WrapMode,
        config: &'a MermaidConfig,
        math_renderer: Option<&'a (dyn MathRenderer + Send + Sync)>,
        title_total_margin: f64,
        cluster_padding: f64,
    }

    fn adjust_cluster_rect_for_title(
        mut rect: Rect,
        sg: &FlowSubgraph,
        title: &str,
        label_type: &str,
        add_title_total_margin: bool,
        ctx: &ClusterTitleAdjustContext<'_>,
    ) -> Rect {
        let title_width_limit = (label_type == "markdown").then_some(ctx.title_wrapping_width);
        let title_font_style =
            flowchart_effective_font_style_for_classes(ctx.class_defs, &sg.classes, &sg.styles);
        let title_metrics = flowchart_label_metrics_for_layout(FlowchartLabelMetricsRequest {
            measurer: ctx.measurer,
            raw_label: title,
            label_type,
            style: if ctx.wrap_mode == WrapMode::HtmlLike {
                ctx.html_label_text_style
            } else {
                ctx.text_style
            },
            max_width_px: title_width_limit,
            wrap_mode: ctx.wrap_mode,
            config: ctx.config,
            math_renderer: ctx.math_renderer,
            preserve_string_whitespace_height: false,
            whole_label_font_style: title_font_style.as_deref(),
        });
        let title_w = title_metrics.width.max(1.0);
        let title_h = title_metrics.height.max(1.0);

        // Mermaid cluster "rect" widens to fit the raw title bbox (no added padding),
        // even when the cluster bounds come from Dagre border nodes.
        let min_w = title_w + ctx.cluster_padding;
        if rect.width() < min_w {
            let (cx, cy) = rect.center();
            rect = Rect::from_center(cx, cy, min_w, rect.height());
        }

        // Mermaid adds `subGraphTitleTotalMargin` to cluster height after layout.
        if add_title_total_margin && ctx.title_total_margin > 0.0 {
            let (cx, cy) = rect.center();
            rect = Rect::from_center(cx, cy, rect.width(), rect.height() + ctx.title_total_margin);
        }

        // Keep the cluster tall enough for the title bbox (including title margins).
        let min_h = title_h + ctx.title_total_margin;
        if rect.height() < min_h {
            let (cx, cy) = rect.center();
            rect = Rect::from_center(cx, cy, rect.width(), min_h);
        }

        rect
    }

    let cluster_rect_ctx = ClusterRectContext {
        subgraphs_by_id: &subgraphs_by_id,
        class_defs: &model.class_defs,
        leaf_rects: &leaf_rects,
        extra_children: &extra_children,
        measurer,
        text_style: &text_style,
        html_label_text_style: &html_label_text_style,
        title_wrapping_width: cluster_title_wrapping_width,
        wrap_mode: cluster_wrap_mode,
        config: effective_config,
        math_renderer,
        cluster_padding,
        title_total_margin,
    };
    let mut cluster_rect_state = ClusterRectState {
        cluster_rects: &mut cluster_rects,
        cluster_base_widths: &mut cluster_base_widths,
        visiting: &mut visiting,
    };
    let title_adjust_ctx = ClusterTitleAdjustContext {
        class_defs: &model.class_defs,
        measurer,
        text_style: &text_style,
        html_label_text_style: &html_label_text_style,
        title_wrapping_width: cluster_title_wrapping_width,
        wrap_mode: cluster_wrap_mode,
        config: effective_config,
        math_renderer,
        title_total_margin,
        cluster_padding,
    };

    for sg in &model.subgraphs {
        if sg.nodes.is_empty() {
            continue;
        }

        let (rect, base_width) = if extracted_graphs.contains_key(&sg.id) {
            // For extracted (recursive) clusters, match Mermaid's `updateNodeBounds(...)` intent by
            // taking the rendered child-graph content bbox (including border nodes) as the cluster
            // node's bounds.
            let rect = extracted_cluster_rects
                .get(&sg.id)
                .copied()
                .unwrap_or_else(|| {
                    compute_cluster_rect(&sg.id, &cluster_rect_ctx, &mut cluster_rect_state)
                        .map(|v| v.0)
                        .unwrap_or_else(|_| Rect::from_center(0.0, 0.0, 1.0, 1.0))
                });
            let base_width = extracted_cluster_base_widths
                .get(&sg.id)
                .copied()
                .unwrap_or_else(|| rect.width());
            let rect = adjust_cluster_rect_for_title(
                rect,
                sg,
                &sg.title,
                sg.label_type.as_deref().unwrap_or("text"),
                false,
                &title_adjust_ctx,
            );
            (rect, base_width)
        } else if let Some(r) = cluster_rects_from_graph.get(&sg.id).copied() {
            let base_width = r.width();
            let rect = adjust_cluster_rect_for_title(
                r,
                sg,
                &sg.title,
                sg.label_type.as_deref().unwrap_or("text"),
                true,
                &title_adjust_ctx,
            );
            (rect, base_width)
        } else {
            compute_cluster_rect(&sg.id, &cluster_rect_ctx, &mut cluster_rect_state)?
        };
        let (cx, cy) = rect.center();

        let label_type = sg.label_type.as_deref().unwrap_or("text");
        let title_width_limit = (label_type == "markdown").then_some(cluster_title_wrapping_width);
        let title_font_style =
            flowchart_effective_font_style_for_classes(&model.class_defs, &sg.classes, &sg.styles);
        let mut title_metrics = flowchart_label_metrics_for_layout(FlowchartLabelMetricsRequest {
            measurer,
            raw_label: &sg.title,
            label_type,
            style: if cluster_wrap_mode == WrapMode::HtmlLike {
                &html_label_text_style
            } else {
                &text_style
            },
            max_width_px: title_width_limit,
            wrap_mode: cluster_wrap_mode,
            config: effective_config,
            math_renderer,
            preserve_string_whitespace_height: false,
            whole_label_font_style: title_font_style.as_deref(),
        });
        if cluster_wrap_mode == crate::text::WrapMode::SvgLike && label_type == "markdown" {
            // Cluster titles with markdown emphasis are rendered as SVG `<text>/<tspan>` runs and
            // measured via browser `getBBox()` in upstream Mermaid. For italic `<em>` titles, the
            // 1/64px-snapped markdown width can be just enough to shift the left-aligned label
            // transform across a strict-XML rounding boundary; use a tighter lattice width probe.
            let has_emphasis = crate::text::mermaid_markdown_to_wrapped_word_lines(
                measurer,
                &sg.title,
                &text_style,
                title_width_limit,
                cluster_wrap_mode,
            )
            .iter()
            .any(|line| {
                line.iter()
                    .any(|(_, ty)| *ty == crate::text::MermaidMarkdownWordType::Em)
            });
            if has_emphasis {
                title_metrics.width = crate::text::measure_markdown_svg_like_precise_width_px(
                    measurer,
                    &sg.title,
                    &text_style,
                    title_width_limit,
                );
            }
        }
        let title_label = LayoutLabel {
            x: cx,
            y: cy - rect.height() / 2.0 + title_margin_top + title_metrics.height / 2.0,
            width: title_metrics.width,
            height: title_metrics.height,
        };

        // `dagre-wrapper/clusters.js` (shape `rect`) sets `padding = 0 * node.padding`.
        // The cluster label is positioned at `node.x - bbox.width/2`, and `node.diff` is:
        // - `(bbox.width - node.width)/2 - node.padding/2` when the box widens to fit the title
        // - otherwise `-node.padding/2`.
        let title_w = title_metrics.width.max(1.0);
        let diff = if base_width <= title_w {
            (title_w - base_width) / 2.0 - cluster_padding / 2.0
        } else {
            -cluster_padding / 2.0
        };
        let offset_y = title_metrics.height - cluster_padding / 2.0;

        let effective_dir = effective_dir_by_id
            .get(&sg.id)
            .cloned()
            .unwrap_or_else(|| effective_cluster_dir(sg, &diagram_direction, inherit_dir));

        clusters.push(LayoutCluster {
            id: sg.id.clone(),
            x: cx,
            y: cy,
            width: rect.width(),
            height: rect.height(),
            diff,
            offset_y,
            title: sg.title.clone(),
            title_label,
            requested_dir: sg.dir.as_ref().map(|s| normalize_dir(s)),
            effective_dir,
            padding: cluster_padding,
            title_margin_top,
            title_margin_bottom,
        });

        out_nodes.push(LayoutNode {
            id: sg.id.clone(),
            x: cx,
            // Mermaid does not shift pure cluster nodes by `subGraphTitleTotalMargin / 2`.
            y: cy,
            width: rect.width(),
            height: rect.height(),
            is_cluster: true,
            label_width: None,
            label_height: None,
        });
    }
    clusters.sort_by(|a, b| a.id.cmp(&b.id));

    let mut out_edges: Vec<LayoutEdge> = Vec::new();
    for e in &render_edges {
        let (points, label_pos, mut from_cluster, mut to_cluster) =
            if let Some(points) = edge_override_points.get(&e.id) {
                let from_cluster = edge_override_from_cluster
                    .get(&e.id)
                    .cloned()
                    .unwrap_or(None);
                let to_cluster = edge_override_to_cluster.get(&e.id).cloned().unwrap_or(None);
                (
                    points.clone(),
                    edge_override_label.get(&e.id).cloned().unwrap_or(None),
                    from_cluster,
                    to_cluster,
                )
            } else {
                let (v, w) = edge_endpoints_by_id
                    .get(&e.id)
                    .cloned()
                    .unwrap_or_else(|| (e.from.clone(), e.to.clone()));
                let edge_key = edge_key_by_id
                    .get(&e.id)
                    .map(String::as_str)
                    .unwrap_or(e.id.as_str());
                let Some(label) = g.edge(&v, &w, Some(edge_key)) else {
                    return Err(Error::InvalidModel {
                        message: format!("missing layout edge {}", e.id),
                    });
                };
                let from_cluster = label
                    .extras
                    .get("fromCluster")
                    .and_then(|v| v.as_str().map(|s| s.to_string()));
                let to_cluster = label
                    .extras
                    .get("toCluster")
                    .and_then(|v| v.as_str().map(|s| s.to_string()));
                let points = label
                    .points
                    .iter()
                    .map(|p| LayoutPoint { x: p.x, y: p.y })
                    .collect::<Vec<_>>();
                let label_pos = match (label.x, label.y) {
                    (Some(x), Some(y)) if label.width > 0.0 || label.height > 0.0 => {
                        Some(LayoutLabel {
                            x,
                            y,
                            width: label.width,
                            height: label.height,
                        })
                    }
                    _ => None,
                };
                (points, label_pos, from_cluster, to_cluster)
            };

        // Match Mermaid's dagre adapter: self-loop special edges on group nodes are annotated with
        // `fromCluster` / `toCluster` so downstream renderers can clip routes to the cluster
        // boundary.
        if subgraph_ids.contains(e.from.as_str()) && e.id.ends_with("-cyclic-special-1") {
            from_cluster = Some(e.from.clone());
        }
        if subgraph_ids.contains(e.to.as_str()) && e.id.ends_with("-cyclic-special-2") {
            to_cluster = Some(e.to.clone());
        }

        out_edges.push(LayoutEdge {
            id: e.id.clone(),
            from: e.from.clone(),
            to: e.to.clone(),
            from_cluster,
            to_cluster,
            points,
            label: label_pos,
            start_label_left: None,
            start_label_right: None,
            end_label_left: None,
            end_label_right: None,
            start_marker: None,
            end_marker: None,
            stroke_dasharray: None,
        });
    }

    // Mermaid's flowchart renderer uses shape-specific intersection functions for edge endpoints
    // (e.g. diamond nodes). Our Dagre-ish layout currently treats all nodes as rectangles, so the
    // first/last points can land on the bounding box rather than the actual polygon boundary.
    //
    // Adjust the first/last edge points to match Mermaid's shape intersection behavior for the
    // shapes that materially differ from rectangles.
    let mut node_shape_by_id: HashMap<&str, &str> = HashMap::new();
    for n in &model.nodes {
        if let Some(s) = n.layout_shape.as_deref() {
            node_shape_by_id.insert(n.id.as_str(), s);
        }
    }
    let mut layout_node_by_id: HashMap<&str, &LayoutNode> = HashMap::new();
    for n in &out_nodes {
        layout_node_by_id.insert(n.id.as_str(), n);
    }

    fn diamond_intersection(node: &LayoutNode, toward: &LayoutPoint) -> Option<LayoutPoint> {
        let vx = toward.x - node.x;
        let vy = toward.y - node.y;
        if !(vx.is_finite() && vy.is_finite()) {
            return None;
        }
        if vx.abs() <= 1e-12 && vy.abs() <= 1e-12 {
            return None;
        }
        let hw = (node.width / 2.0).max(1e-9);
        let hh = (node.height / 2.0).max(1e-9);
        let denom = vx.abs() / hw + vy.abs() / hh;
        if !(denom.is_finite() && denom > 0.0) {
            return None;
        }
        let t = 1.0 / denom;
        Some(LayoutPoint {
            x: node.x + vx * t,
            y: node.y + vy * t,
        })
    }

    for e in &mut out_edges {
        if e.points.len() < 2 {
            continue;
        }

        if let Some(node) = layout_node_by_id.get(e.from.as_str()) {
            if !node.is_cluster {
                let shape = node_shape_by_id
                    .get(e.from.as_str())
                    .copied()
                    .unwrap_or("squareRect");
                if matches!(shape, "diamond" | "question" | "diam") {
                    if let Some(p) = diamond_intersection(node, &e.points[1]) {
                        e.points[0] = p;
                    }
                }
            }
        }
        if let Some(node) = layout_node_by_id.get(e.to.as_str()) {
            if !node.is_cluster {
                let shape = node_shape_by_id
                    .get(e.to.as_str())
                    .copied()
                    .unwrap_or("squareRect");
                if matches!(shape, "diamond" | "question" | "diam") {
                    let n = e.points.len();
                    if let Some(p) = diamond_intersection(node, &e.points[n - 2]) {
                        e.points[n - 1] = p;
                    }
                }
            }
        }
    }

    let bounds = compute_bounds(&out_nodes, &out_edges);

    if let Some(s) = build_output_start {
        timings.build_output = s.elapsed();
    }
    if let Some(s) = total_start {
        timings.total = s.elapsed();
        let dagre_overhead = timings
            .layout_recursive
            .checked_sub(timings.dagre_total)
            .unwrap_or_default();
        eprintln!(
            "[layout-timing] diagram=flowchart-v2 total={:?} deserialize={:?} expand_self_loops={:?} build_graph={:?} extract_clusters={:?} dom_order={:?} layout_recursive={:?} dagre_calls={} dagre_total={:?} dagre_overhead={:?} place_graph={:?} build_output={:?}",
            timings.total,
            timings.deserialize,
            timings.expand_self_loops,
            timings.build_graph,
            timings.extract_clusters,
            timings.dom_order,
            timings.layout_recursive,
            timings.dagre_calls,
            timings.dagre_total,
            dagre_overhead,
            timings.place_graph,
            timings.build_output,
        );
    }

    Ok(FlowchartV2Layout {
        nodes: out_nodes,
        edges: out_edges,
        clusters,
        bounds,
        dom_node_order_by_root,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    const DEEP_SUBGRAPH_DEPTH: usize = 10_000;

    #[derive(Debug)]
    struct DeepTraversalOutcome {
        descendant_count: usize,
        anchor: Option<String>,
        root_dir: Option<String>,
        child_dir: Option<String>,
        copied_node_count: usize,
    }

    fn compound_graph() -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
        Graph::new(GraphOptions {
            multigraph: true,
            compound: true,
            directed: true,
        })
    }

    fn deep_compound_graph(depth: usize) -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
        let mut graph = compound_graph();
        for i in (0..depth).rev() {
            graph.set_parent(format!("n{}", i + 1), format!("n{i}"));
        }
        graph
    }

    fn deep_subgraphs(depth: usize) -> HashMap<String, FlowSubgraph> {
        let mut subgraphs = HashMap::with_capacity(depth);
        for i in 0..depth {
            subgraphs.insert(
                format!("n{i}"),
                FlowSubgraph {
                    id: format!("n{i}"),
                    title: format!("n{i}"),
                    dir: None,
                    label_type: None,
                    classes: Vec::new(),
                    styles: Vec::new(),
                    nodes: vec![format!("n{}", i + 1)],
                },
            );
        }
        subgraphs
    }

    #[test]
    fn flowchart_cluster_traversals_handle_deep_subgraphs_with_small_stack() {
        let (tx, rx) = mpsc::channel();
        std::thread::Builder::new()
            .name("flowchart-deep-subgraph-traversal".to_string())
            .stack_size(512 * 1024)
            .spawn(move || {
                let mut graph = deep_compound_graph(DEEP_SUBGRAPH_DEPTH);
                let descendants = extract_descendants("n0", &graph);
                let anchor = flowchart_find_non_cluster_child("n0", &graph, "n0");
                let dirs = compute_effective_dir_by_id(
                    &deep_subgraphs(DEEP_SUBGRAPH_DEPTH),
                    &graph,
                    "TB",
                    false,
                );

                let mut descendants_by_id = HashMap::new();
                descendants_by_id.insert("n0".to_string(), descendants.clone());
                let mut copied = compound_graph();
                copy_cluster("n0", &mut graph, &mut copied, "n0", &descendants_by_id);

                tx.send(DeepTraversalOutcome {
                    descendant_count: descendants.len(),
                    anchor,
                    root_dir: dirs.get("n0").cloned(),
                    child_dir: dirs.get("n1").cloned(),
                    copied_node_count: copied.node_count(),
                })
                .unwrap();
            })
            .expect("spawn deep subgraph traversal test");

        let outcome = rx
            .recv_timeout(Duration::from_secs(15))
            .expect("deep subgraph traversal should finish without stack overflow");

        assert_eq!(outcome.descendant_count, DEEP_SUBGRAPH_DEPTH);
        assert_eq!(outcome.anchor, Some(format!("n{DEEP_SUBGRAPH_DEPTH}")));
        assert_eq!(outcome.root_dir.as_deref(), Some("LR"));
        assert_eq!(outcome.child_dir.as_deref(), Some("TB"));
        assert_eq!(outcome.copied_node_count, DEEP_SUBGRAPH_DEPTH);
    }

    #[test]
    fn extract_descendants_handles_deeply_nested_subgraphs() {
        let (tx, rx) = mpsc::channel();
        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(move || {
                let mut g = Graph::new(GraphOptions {
                    compound: true,
                    ..Default::default()
                });
                for i in (0..DEEP_SUBGRAPH_DEPTH).rev() {
                    g.set_parent(format!("n{}", i + 1), format!("n{i}"));
                }
                let _ = tx.send(extract_descendants("n0", &g));
            })
            .unwrap();
        let descendants = rx
            .recv_timeout(Duration::from_secs(2))
            .expect("extract_descendants overflowed the stack on deep nesting");
        assert_eq!(descendants.len(), DEEP_SUBGRAPH_DEPTH);
    }
}

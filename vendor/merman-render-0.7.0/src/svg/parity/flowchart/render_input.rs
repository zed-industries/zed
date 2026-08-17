//! Flowchart render-time edge and helper-node preparation.

use std::borrow::Cow;
use std::collections::BTreeSet;

use rustc_hash::FxHashSet;

pub(in crate::svg::parity::flowchart) struct FlowchartRenderInputs<'a> {
    pub render_edges: Vec<Cow<'a, crate::flowchart::FlowEdge>>,
    pub extra_nodes: Vec<crate::flowchart::FlowNode>,
}

pub(in crate::svg::parity::flowchart) fn prepare_flowchart_render_inputs<'a>(
    model: &'a crate::flowchart::FlowchartV2Model,
) -> FlowchartRenderInputs<'a> {
    // Mermaid expands self-loop edges into a chain of helper nodes plus `*-cyclic-special-*` edge
    // segments during Dagre layout. Replicate that expansion here so rendered SVG ids match.
    let self_loop_count = model.edges.iter().filter(|e| e.from == e.to).count();
    let mut render_edges: Vec<Cow<'a, crate::flowchart::FlowEdge>> =
        Vec::with_capacity(model.edges.len() + self_loop_count * 3);
    let mut self_loop_label_node_ids: BTreeSet<String> = BTreeSet::new();
    for e in &model.edges {
        if e.from != e.to {
            render_edges.push(Cow::Borrowed(e));
            continue;
        }

        let helper_edges = crate::flowchart::flowchart_self_loop_helper_edges(
            e,
            crate::flowchart::FlowchartSelfLoopEdgeOptions::svg_render(),
        );
        self_loop_label_node_ids.insert(helper_edges.special_id_1.clone());
        self_loop_label_node_ids.insert(helper_edges.special_id_2.clone());

        render_edges.push(Cow::Owned(helper_edges.edge1));
        render_edges.push(Cow::Owned(helper_edges.edge_mid));
        render_edges.push(Cow::Owned(helper_edges.edge2));
    }

    // Mermaid's `adjustClustersAndEdges(graph)` rewrites edges that connect directly to cluster
    // nodes by removing and re-adding them (after swapping endpoints to anchor nodes). This has a
    // visible side-effect: those edges end up later in `graph.edges()` insertion order, so the
    // DOM emitted under `.edgePaths` / `.edgeLabels` matches that stable partition.
    let cluster_ids_with_children: FxHashSet<&str> = model
        .subgraphs
        .iter()
        .filter(|sg| !sg.nodes.is_empty())
        .map(|sg| sg.id.as_str())
        .collect();
    if !cluster_ids_with_children.is_empty() && render_edges.len() >= 2 {
        let mut normal: Vec<Cow<'a, crate::flowchart::FlowEdge>> =
            Vec::with_capacity(render_edges.len());
        let mut cluster: Vec<Cow<'a, crate::flowchart::FlowEdge>> = Vec::new();
        for e in render_edges {
            let edge = e.as_ref();
            if cluster_ids_with_children.contains(edge.from.as_str())
                || cluster_ids_with_children.contains(edge.to.as_str())
            {
                cluster.push(e);
            } else {
                normal.push(e);
            }
        }
        normal.extend(cluster);
        render_edges = normal;
    }

    let mut extra_nodes: Vec<crate::flowchart::FlowNode> =
        Vec::with_capacity(self_loop_label_node_ids.len());
    for id in &self_loop_label_node_ids {
        extra_nodes.push(crate::flowchart::FlowNode {
            id: id.clone(),
            label: Some(String::new()),
            label_type: None,
            layout_shape: None,
            icon: None,
            form: None,
            pos: None,
            img: None,
            constraint: None,
            asset_width: None,
            asset_height: None,
            classes: Vec::new(),
            styles: Vec::new(),
            have_callback: false,
            link: None,
            link_target: None,
        });
    }

    FlowchartRenderInputs {
        render_edges,
        extra_nodes,
    }
}

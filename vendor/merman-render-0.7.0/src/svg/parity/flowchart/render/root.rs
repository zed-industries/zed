//! Flowchart root renderer.

use super::super::*;

pub(in crate::svg::parity::flowchart) struct FlowchartRootRenderSession<'details, 'cache> {
    pub(in crate::svg::parity::flowchart) timing_enabled: bool,
    pub(in crate::svg::parity::flowchart) details: &'details mut FlowchartRenderDetails,
    pub(in crate::svg::parity::flowchart) edge_cache:
        Option<&'cache FxHashMap<&'cache str, FlowchartEdgePathCacheEntry>>,
}

struct FlowchartRootFrame<'a> {
    cluster_id: Option<&'a str>,
    parent_origin_x: f64,
    parent_origin_y: f64,
    origin_x: f64,
    origin_y: f64,
    content_origin_y: f64,
    dom_order: Vec<&'a str>,
    next_dom_index: usize,
    initialized: bool,
    nested_start: Option<web_time::Instant>,
}

impl<'a> FlowchartRootFrame<'a> {
    fn new(
        cluster_id: Option<&'a str>,
        parent_origin_x: f64,
        parent_origin_y: f64,
        nested_start: Option<web_time::Instant>,
    ) -> Self {
        Self {
            cluster_id,
            parent_origin_x,
            parent_origin_y,
            origin_x: parent_origin_x,
            origin_y: parent_origin_y,
            content_origin_y: parent_origin_y,
            dom_order: Vec::new(),
            next_dom_index: 0,
            initialized: false,
            nested_start,
        }
    }
}

pub(in crate::svg::parity::flowchart) fn render_flowchart_root(
    out: &mut String,
    ctx: &FlowchartRenderCtx<'_>,
    cluster_id: Option<&str>,
    parent_origin_x: f64,
    parent_origin_y: f64,
    session: &mut FlowchartRootRenderSession<'_, '_>,
) {
    let mut stack = vec![FlowchartRootFrame::new(
        cluster_id,
        parent_origin_x,
        parent_origin_y,
        None,
    )];

    while let Some(frame) = stack.pop() {
        let mut frame = Some(frame);
        if !frame.as_ref().is_some_and(|frame| frame.initialized) {
            if let Some(frame) = frame.as_mut() {
                initialize_flowchart_root_frame(out, ctx, session, frame);
            }
        }

        let mut pushed_nested = false;
        while frame
            .as_ref()
            .is_some_and(|frame| frame.next_dom_index < frame.dom_order.len())
        {
            let id = {
                let Some(frame) = frame.as_mut() else {
                    break;
                };
                let id = frame.dom_order[frame.next_dom_index];
                frame.next_dom_index += 1;
                id
            };

            if ctx
                .subgraphs_by_id
                .get(id)
                .is_some_and(|sg| !sg.nodes.is_empty())
            {
                // Non-recursive clusters render as cluster boxes (in `.clusters`) and do not emit a
                // node DOM element. Recursive clusters render as nested `.root` groups.
                if ctx.recursive_clusters.contains(id) {
                    let nested_start = session.timing_enabled.then(web_time::Instant::now);
                    if let Some(parent) = frame.take() {
                        let child = FlowchartRootFrame::new(
                            Some(id),
                            parent.origin_x,
                            parent.origin_y,
                            nested_start,
                        );
                        stack.push(parent);
                        stack.push(child);
                        pushed_nested = true;
                    }
                    break;
                }
                continue;
            }

            let node_start = session.timing_enabled.then(web_time::Instant::now);
            let Some(current) = frame.as_ref() else {
                break;
            };
            render_flowchart_node(
                out,
                ctx,
                id,
                current.origin_x,
                current.content_origin_y,
                session.timing_enabled,
                &mut *session.details,
            );
            if let Some(s) = node_start {
                session.details.nodes += s.elapsed();
            }
        }

        if pushed_nested {
            continue;
        }

        if let Some(frame) = frame.take() {
            out.push_str("</g></g>");
            if let Some(start) = frame.nested_start {
                session.details.nested_roots += start.elapsed();
            }
        }
    }
}

fn initialize_flowchart_root_frame<'a>(
    out: &mut String,
    ctx: &'a FlowchartRenderCtx<'a>,
    session: &mut FlowchartRootRenderSession<'_, '_>,
    frame: &mut FlowchartRootFrame<'a>,
) {
    session.details.root_calls += 1;

    let (origin_x, origin_y, transform_attr) = if let Some(cid) = frame.cluster_id {
        if let Some(off) = flowchart_cluster_root_offsets(ctx, cid) {
            let rel_x = off.origin_x - frame.parent_origin_x;
            let rel_y = off.abs_top_transform - frame.parent_origin_y;
            (
                off.origin_x,
                off.origin_y,
                format!(
                    r#" transform="translate({},{})""#,
                    fmt_display(rel_x),
                    fmt_display(rel_y)
                ),
            )
        } else {
            // Fallback: keep the group in the parent's coordinate space.
            (
                frame.parent_origin_x,
                frame.parent_origin_y,
                r#" transform="translate(0,0)""#.to_string(),
            )
        }
    } else {
        (0.0, 0.0, String::new())
    };

    frame.origin_x = origin_x;
    frame.origin_y = origin_y;
    frame.content_origin_y = origin_y;

    let _ = write!(out, r#"<g class="root"{}>"#, transform_attr);

    let _g_clusters = detail_guard(session.timing_enabled, &mut session.details.clusters);
    let mut clusters_to_draw: Vec<&LayoutCluster> = Vec::new();
    if let Some(cid) = frame.cluster_id {
        if ctx
            .subgraphs_by_id
            .get(cid)
            .is_some_and(|sg| sg.nodes.is_empty())
        {
            // Empty subgraphs are rendered as plain nodes in Mermaid (see flowchart-v2.spec.js
            // outgoing-links-4 baseline), so they should not emit cluster boxes.
        } else if let Some(cluster) = ctx.layout_clusters_by_id.get(cid) {
            clusters_to_draw.push(cluster);
        }
    }
    for id in ctx.subgraphs_by_id.keys() {
        if frame.cluster_id.is_some_and(|cid| cid == *id) {
            continue;
        }
        if ctx
            .subgraphs_by_id
            .get(id)
            .is_some_and(|sg| sg.nodes.is_empty())
        {
            continue;
        }
        if ctx.recursive_clusters.contains(id) {
            continue;
        }
        if flowchart_effective_parent(ctx, id) == frame.cluster_id {
            if let Some(cluster) = ctx.layout_clusters_by_id.get(*id) {
                clusters_to_draw.push(cluster);
            }
        }
    }
    if clusters_to_draw.is_empty() {
        out.push_str(r#"<g class="clusters"/>"#);
    } else {
        // Mermaid emits clusters by traversing the Dagre graph hierarchy (pre-order over
        // `graph.children()`), which in practice matches a stable bottom-to-top ordering in the
        // upstream SVG baselines (see `flowchart-v2 outgoing-links-*` fixtures).
        fn is_ancestor(parent: &FxHashMap<&str, &str>, ancestor: &str, node: &str) -> bool {
            let mut cur: Option<&str> = Some(node);
            while let Some(id) = cur {
                let Some(p) = parent.get(id).copied() else {
                    break;
                };
                if p == ancestor {
                    return true;
                }
                cur = Some(p);
            }
            false
        }

        clusters_to_draw.sort_by(|a, b| {
            if a.id != b.id {
                if is_ancestor(&ctx.parent, &a.id, &b.id) {
                    return std::cmp::Ordering::Less;
                }
                if is_ancestor(&ctx.parent, &b.id, &a.id) {
                    return std::cmp::Ordering::Greater;
                }
            }

            let a_top_y = a.y - a.height / 2.0;
            let b_top_y = b.y - b.height / 2.0;
            let a_top_x = a.x - a.width / 2.0;
            let b_top_x = b.x - b.width / 2.0;
            let a_idx = ctx
                .subgraph_order
                .iter()
                .position(|id| *id == a.id.as_str());
            let b_idx = ctx
                .subgraph_order
                .iter()
                .position(|id| *id == b.id.as_str());
            if let (Some(ai), Some(bi)) = (a_idx, b_idx) {
                // Mermaid's cluster insertion order tracks the order in which subgraphs are
                // defined/registered, but for SVG output the baselines match a reverse (last
                // defined first) ordering for sibling cluster boxes.
                bi.cmp(&ai)
                    .then_with(|| b_top_y.total_cmp(&a_top_y))
                    .then_with(|| b_top_x.total_cmp(&a_top_x))
                    .then_with(|| a.id.cmp(&b.id))
            } else {
                b_top_y
                    .total_cmp(&a_top_y)
                    .then_with(|| b_top_x.total_cmp(&a_top_x))
                    .then_with(|| a.id.cmp(&b.id))
            }
        });
        out.push_str(r#"<g class="clusters">"#);
        for cluster in clusters_to_draw {
            render_flowchart_cluster(out, ctx, cluster, origin_x, frame.content_origin_y);
        }
        out.push_str("</g>");
    }
    drop(_g_clusters);

    let _g_edges_select = detail_guard(session.timing_enabled, &mut session.details.edges_select);
    let edges = flowchart_edges_for_root(ctx, frame.cluster_id);
    drop(_g_edges_select);

    let _g_edge_paths = detail_guard(session.timing_enabled, &mut session.details.edge_paths);
    if edges.is_empty() {
        out.push_str(r#"<g class="edgePaths"/>"#);
    } else {
        out.push_str(r#"<g class="edgePaths">"#);
        let mut scratch = FlowchartEdgeDataPointsScratch::default();
        for e in &edges {
            render_flowchart_edge_path(
                out,
                ctx,
                e,
                origin_x,
                frame.content_origin_y,
                &mut scratch,
                session.edge_cache,
            );
        }
        out.push_str("</g>");
    }
    drop(_g_edge_paths);

    let _g_edge_labels = detail_guard(session.timing_enabled, &mut session.details.edge_labels);
    if edges.is_empty() {
        out.push_str(r#"<g class="edgeLabels"/>"#);
    } else {
        fn edge_label_is_empty(
            ctx: &FlowchartRenderCtx<'_>,
            edge: &crate::flowchart::FlowEdge,
        ) -> bool {
            let label_text = edge.label.as_deref().unwrap_or_default();
            let label_type = edge.label_type.as_deref().unwrap_or("text");
            let label_plain =
                flowchart_label_plain_text(label_text, label_type, ctx.edge_html_labels);
            label_plain.trim().is_empty() && label_text.trim().is_empty()
        }

        out.push_str(r#"<g class="edgeLabels">"#);
        if !ctx.edge_html_labels {
            // Mermaid's `createText(..., useHtmlLabels=false)` always creates a background `<rect>`,
            // but for empty labels it returns the `<text>` element instead of the wrapper `<g>`.
            // The unused wrapper `<g>` (with the `background` rect) remains as a direct child
            // under `.edgeLabels`. Mirror this by emitting one rect-group per empty label.
            for e in &edges {
                if edge_label_is_empty(ctx, e) {
                    out.push_str(r#"<g><rect class="background" style="stroke: none"/></g>"#);
                }
            }
            for e in &edges {
                render_flowchart_edge_label(
                    out,
                    ctx,
                    e,
                    origin_x,
                    frame.content_origin_y,
                    session.edge_cache,
                );
            }
        } else {
            // Mermaid emits HTML edge-label wrappers in graph edge order. Empty labels stay in
            // place as zero-sized foreignObjects instead of being partitioned ahead of labels.
            for e in &edges {
                render_flowchart_edge_label(
                    out,
                    ctx,
                    e,
                    origin_x,
                    frame.content_origin_y,
                    session.edge_cache,
                );
            }
        }
        out.push_str("</g>");
    }
    drop(_g_edge_labels);

    out.push_str(r#"<g class="nodes">"#);

    // Mermaid inserts node DOM elements in `graph.nodes()` insertion order while recursively
    // rendering extracted cluster graphs. Our layout captures that order per extracted root.
    let _g_dom_order = detail_guard(session.timing_enabled, &mut session.details.dom_order);
    let mut dom_order: Vec<&str> = ctx
        .dom_node_order_by_root
        .get(frame.cluster_id.unwrap_or(""))
        .map(|ids| ids.iter().map(|s| s.as_str()).collect())
        .unwrap_or_default();
    if !dom_order.is_empty() {
        // Some upstream flowchart-v2 configurations can produce a DOM registration order that
        // only includes non-recursive clusters (clusters with external edges). These clusters do
        // not emit a node DOM element, so relying on the raw order would produce an empty
        // `.nodes` group. Fall back to our effective-parent ordering in that case.
        let mut emits_anything = false;
        for id in &dom_order {
            if ctx
                .subgraphs_by_id
                .get(id)
                .is_some_and(|sg| !sg.nodes.is_empty())
            {
                if ctx.recursive_clusters.contains(id) {
                    emits_anything = true;
                    break;
                }
                continue;
            }
            emits_anything = true;
            break;
        }
        if !emits_anything {
            dom_order.clear();
        }
    }

    if dom_order.is_empty() {
        // Fallback for v1 layouts: approximate by appending extracted cluster roots after
        // regular nodes.
        dom_order = flowchart_root_children_nodes(ctx, frame.cluster_id);
        dom_order.extend(flowchart_root_children_clusters(ctx, frame.cluster_id));
    }
    drop(_g_dom_order);

    frame.dom_order = dom_order;
    frame.initialized = true;
}

pub(super) fn flowchart_wrap_svg_text_lines(
    measurer: &dyn TextMeasurer,
    text: &str,
    style: &crate::text::TextStyle,
    max_width_px: Option<f64>,
    break_long_words: bool,
) -> Vec<String> {
    crate::text::wrap_svg_text_lines_by_measurement(
        measurer,
        text,
        style,
        max_width_px,
        break_long_words,
    )
}

use std::fmt::Write as _;

use super::defs::prepare_flowchart_defs;
use super::document::{FlowchartSvgDocumentRequest, prepare_flowchart_svg_document};
use super::render_config::{FlowchartRenderConfig, prepare_flowchart_render_config};
use super::render_input::{FlowchartRenderInputs, prepare_flowchart_render_inputs};
use super::viewbox::{
    FlowchartRenderedBoundsRequest, FlowchartViewboxBounds, FlowchartViewboxBoundsRequest,
    prepare_flowchart_rendered_bounds, prepare_flowchart_viewbox_bounds,
};
use super::*;

pub(super) fn render_flowchart_v2_svg(
    layout: &FlowchartV2Layout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let config = merman_core::MermaidConfig::from_value(effective_config.clone());
    render_flowchart_v2_svg_with_config(layout, semantic, &config, diagram_title, measurer, options)
}

#[inline]
fn section<'a>(
    enabled: bool,
    dst: &'a mut web_time::Duration,
) -> Option<super::super::timing::TimingGuard<'a>> {
    enabled.then(|| super::super::timing::TimingGuard::new(dst))
}

pub(super) fn render_flowchart_v2_svg_model_with_config(
    layout: &FlowchartV2Layout,
    model: &crate::flowchart::FlowchartV2Model,
    effective_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let timing_enabled = super::super::timing::render_timing_enabled();
    let mut timings = super::super::timing::RenderTimings::default();
    let total_start = web_time::Instant::now();

    render_flowchart_v2_svg_with_config_inner(
        layout,
        model,
        effective_config,
        diagram_title,
        measurer,
        options,
        FlowchartSvgTiming {
            enabled: timing_enabled,
            timings: &mut timings,
            total_start,
        },
    )
}

pub(super) fn render_flowchart_v2_svg_with_config(
    layout: &FlowchartV2Layout,
    semantic: &serde_json::Value,
    effective_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let timing_enabled = super::super::timing::render_timing_enabled();
    let mut timings = super::super::timing::RenderTimings::default();
    let total_start = web_time::Instant::now();

    let model: crate::flowchart::FlowchartV2Model = {
        let _g = section(timing_enabled, &mut timings.deserialize_model);
        crate::json::from_value_ref(semantic)?
    };

    render_flowchart_v2_svg_with_config_inner(
        layout,
        &model,
        effective_config,
        diagram_title,
        measurer,
        options,
        FlowchartSvgTiming {
            enabled: timing_enabled,
            timings: &mut timings,
            total_start,
        },
    )
}

struct FlowchartSvgTiming<'a> {
    enabled: bool,
    timings: &'a mut super::super::timing::RenderTimings,
    total_start: web_time::Instant,
}

fn render_flowchart_v2_svg_with_config_inner(
    layout: &FlowchartV2Layout,
    model: &crate::flowchart::FlowchartV2Model,
    effective_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
    timing: FlowchartSvgTiming<'_>,
) -> Result<String> {
    let timing_enabled = timing.enabled;
    let timings = timing.timings;
    let total_start = timing.total_start;

    let effective_config_value = effective_config.as_value();

    let diagram_id = options.diagram_id.as_deref().unwrap_or("merman");
    let diagram_type = "flowchart-v2";

    let _g_build_ctx = section(timing_enabled, &mut timings.build_ctx);

    let FlowchartRenderInputs {
        render_edges,
        extra_nodes,
    } = prepare_flowchart_render_inputs(model);

    let FlowchartRenderConfig {
        font_family,
        font_size,
        wrapping_width,
        node_html_labels,
        edge_html_labels,
        node_wrap_mode,
        edge_wrap_mode,
        diagram_padding,
        use_max_width,
        title_top_margin,
        node_padding,
        text_style,
        html_label_text_style,
        default_edge_interpolate,
        default_edge_style,
        node_border_color,
        node_fill_color,
    } = prepare_flowchart_render_config(model, effective_config_value);

    let mut nodes_by_id: FxHashMap<&str, &crate::flowchart::FlowNode> =
        FxHashMap::with_capacity_and_hasher(
            model.nodes.len() + extra_nodes.len(),
            Default::default(),
        );
    for n in &model.nodes {
        nodes_by_id.insert(n.id.as_str(), n);
    }
    for n in &extra_nodes {
        let _ = nodes_by_id.entry(n.id.as_str()).or_insert(n);
    }

    let edge_order: Vec<&str> = render_edges
        .iter()
        .map(|e| e.as_ref().id.as_str())
        .collect();
    let mut edges_by_id: FxHashMap<&str, &crate::flowchart::FlowEdge> =
        FxHashMap::with_capacity_and_hasher(render_edges.len(), Default::default());
    for e in &render_edges {
        let edge = e.as_ref();
        edges_by_id.insert(edge.id.as_str(), edge);
    }

    let subgraph_order: Vec<&str> = model.subgraphs.iter().map(|s| s.id.as_str()).collect();
    let mut subgraphs_by_id: FxHashMap<&str, &crate::flowchart::FlowSubgraph> =
        FxHashMap::with_capacity_and_hasher(model.subgraphs.len(), Default::default());
    for sg in &model.subgraphs {
        subgraphs_by_id.insert(sg.id.as_str(), sg);
    }

    let mut parent: FxHashMap<&str, &str> = FxHashMap::default();
    for sg in &model.subgraphs {
        let sg_id = sg.id.as_str();
        for child in &sg.nodes {
            parent.insert(child.as_str(), sg_id);
        }
    }
    for n in &extra_nodes {
        let id = n.id.as_str();
        let Some((base, _)) = id.split_once("---") else {
            continue;
        };
        if let Some(&p) = parent.get(base) {
            parent.insert(id, p);
        }
    }

    let mut recursive_clusters: FxHashSet<&str> = FxHashSet::default();
    for sg in model.subgraphs.iter() {
        if sg.nodes.is_empty() {
            continue;
        }
        let mut external = false;
        for e in &render_edges {
            let e = e.as_ref();
            // Match Mermaid `adjustClustersAndEdges` / flowchart-v2 behavior: a cluster is
            // considered to have external connections when an edge crosses its descendant boundary.
            let from_in = flowchart_is_strict_descendant(&parent, e.from.as_str(), sg.id.as_str());
            let to_in = flowchart_is_strict_descendant(&parent, e.to.as_str(), sg.id.as_str());
            if from_in != to_in {
                external = true;
                break;
            }
        }
        if !external {
            recursive_clusters.insert(sg.id.as_str());
        }
    }

    let mut layout_nodes_by_id: FxHashMap<&str, &LayoutNode> =
        FxHashMap::with_capacity_and_hasher(layout.nodes.len(), Default::default());
    for n in &layout.nodes {
        layout_nodes_by_id.insert(n.id.as_str(), n);
    }

    let mut layout_edges_by_id: FxHashMap<&str, &crate::model::LayoutEdge> =
        FxHashMap::with_capacity_and_hasher(layout.edges.len(), Default::default());
    for e in &layout.edges {
        layout_edges_by_id.insert(e.id.as_str(), e);
    }

    let mut layout_clusters_by_id: FxHashMap<&str, &LayoutCluster> =
        FxHashMap::with_capacity_and_hasher(layout.clusters.len(), Default::default());
    for c in &layout.clusters {
        layout_clusters_by_id.insert(c.id.as_str(), c);
    }

    // Mermaid flowchart-v2 does not translate the root `.root` group; node/edge coordinates are
    // already in the Dagre coordinate space (including Dagre's fixed `marginx/marginy=8`).
    // `diagramPadding` is applied only when computing the final SVG viewBox.
    let tx = 0.0;
    let ty = 0.0;

    let node_dom_index = flowchart_node_dom_indices(model);

    let ctx = FlowchartRenderCtx {
        diagram_id,
        tx,
        ty,
        measurer,
        config: effective_config,
        math_renderer: options.math_renderer.as_deref(),
        icon_registry: options.icon_registry.as_deref(),
        node_html_labels,
        edge_html_labels,
        class_defs: &model.class_defs,
        node_border_color,
        node_fill_color,
        default_edge_interpolate,
        default_edge_style,
        trace_edge_id: std::env::var("MERMAN_TRACE_FLOWCHART_EDGE").ok(),
        subgraph_order,
        edge_order,
        nodes_by_id,
        edges_by_id,
        subgraphs_by_id,
        tooltips: &model.tooltips,
        recursive_clusters,
        parent,
        layout_nodes_by_id,
        layout_edges_by_id,
        layout_clusters_by_id,
        dom_node_order_by_root: &layout.dom_node_order_by_root,
        node_dom_index,
        node_padding,
        wrapping_width,
        node_wrap_mode,
        edge_wrap_mode,
        text_style,
        html_label_text_style,
    };

    let mut edge_path_cache: FxHashMap<&str, FlowchartEdgePathCacheEntry> =
        FxHashMap::with_capacity_and_hasher(render_edges.len(), Default::default());

    let subgraph_title_y_shift = {
        let top = config_f64(
            effective_config_value,
            &["flowchart", "subGraphTitleMargin", "top"],
        )
        .unwrap_or(0.0)
        .max(0.0);
        let bottom = config_f64(
            effective_config_value,
            &["flowchart", "subGraphTitleMargin", "bottom"],
        )
        .unwrap_or(0.0)
        .max(0.0);
        (top + bottom) / 2.0
    };

    fn self_loop_label_base_node_id(id: &str) -> Option<&str> {
        let mut parts = id.split("---");
        let a = parts.next()?;
        let b = parts.next()?;
        let n = parts.next()?;
        if parts.next().is_some() {
            return None;
        }
        if a != b {
            return None;
        }
        if n != "1" && n != "2" {
            return None;
        }
        Some(a)
    }

    drop(_g_build_ctx);

    let mut detail = FlowchartRenderDetails::default();
    let mut viewbox_edge_curve_bounds = web_time::Duration::ZERO;
    let _g_viewbox = section(timing_enabled, &mut timings.viewbox);

    let effective_parent_for_id = |id: &str| -> Option<&str> {
        let mut cur = ctx.parent.get(id).copied();
        if cur.is_none() {
            if let Some(base) = self_loop_label_base_node_id(id) {
                cur = ctx.parent.get(base).copied();
            }
        }
        while let Some(p) = cur {
            if ctx.subgraphs_by_id.contains_key(p) && !ctx.recursive_clusters.contains(p) {
                cur = ctx.parent.get(p).copied();
                continue;
            }
            return Some(p);
        }
        None
    };

    let bounds = prepare_flowchart_rendered_bounds(
        FlowchartRenderedBoundsRequest {
            ctx: &ctx,
            layout,
            subgraph_title_y_shift,
        },
        &effective_parent_for_id,
    );
    let FlowchartViewboxBounds {
        diagram_title,
        title_anchor_x,
        bbox_min_x,
        bbox_min_y,
        bbox_max_x,
        bbox_max_y,
    } = prepare_flowchart_viewbox_bounds(
        FlowchartViewboxBoundsRequest {
            ctx: &ctx,
            render_edges: &render_edges,
            base_bounds: bounds,
            diagram_title,
            font_family: &font_family,
            title_top_margin,
            timing_enabled,
            viewbox_edge_curve_bounds: &mut viewbox_edge_curve_bounds,
            detail: &mut detail,
            edge_path_cache: &mut edge_path_cache,
        },
        &effective_parent_for_id,
    );

    let document = prepare_flowchart_svg_document(FlowchartSvgDocumentRequest {
        diagram_id,
        diagram_type,
        model,
        use_max_width,
        apply_root_overrides: options.apply_root_overrides,
        diagram_padding,
        bbox_min_x,
        bbox_min_y,
        bbox_max_x,
        bbox_max_y,
    });

    drop(_g_viewbox);
    let _g_render_svg = section(timing_enabled, &mut timings.render_svg);

    let css = flowchart_css(
        diagram_id,
        effective_config_value,
        &font_family,
        font_size,
        &model.class_defs,
    );

    let estimated_svg_bytes = 2048usize
        + css.len()
        + layout.nodes.len().saturating_mul(256)
        + render_edges.len().saturating_mul(256)
        + layout.clusters.len().saturating_mul(128);
    let mut out = String::with_capacity(estimated_svg_bytes);

    document.push_root_open(&mut out);
    document.push_accessibility_metadata(&mut out);
    out.push_str("<style>");
    out.push_str(&css);
    out.push_str("</style>");
    push_flowchart_shadow_defs(&mut out, diagram_id, effective_config_value);

    let defs = prepare_flowchart_defs(diagram_id, &ctx);

    out.push_str("<g>");
    defs.push_base_markers(&mut out);
    let mut root_session = FlowchartRootRenderSession {
        timing_enabled,
        details: &mut detail,
        edge_cache: Some(&edge_path_cache),
    };
    render_flowchart_root(&mut out, &ctx, None, 0.0, 0.0, &mut root_session);

    defs.push_extra_markers(&mut out);
    out.push_str("</g>");
    push_flowchart_gradient(&mut out, diagram_id, effective_config_value);
    if let Some(title) = diagram_title.as_deref() {
        let title_x = title_anchor_x;
        let title_y = -title_top_margin;
        let _ = write!(
            &mut out,
            r#"<text text-anchor="middle" x="{}" y="{}" class="flowchartTitleText">{}</text>"#,
            fmt(title_x),
            fmt(title_y),
            escape_xml(title)
        );
    }
    out.push_str("</svg>\n");

    drop(_g_render_svg);
    timings.total = total_start.elapsed();
    if timing_enabled {
        eprintln!(
            "[render-timing] diagram=flowchart-v2 total={:?} deserialize={:?} build_ctx={:?} viewbox={:?} viewbox_edge_curve_bounds={:?} viewbox_edge_curve_lca={:?} viewbox_edge_curve_offsets={:?} viewbox_edge_curve_geom={:?} viewbox_edge_curve_bbox_union={:?} viewbox_edge_curve_geom_calls={} viewbox_edge_curve_geom_skipped_bounds={} render_svg={:?} finalize={:?} root_calls={} clusters={:?} edges_select={:?} edge_paths={:?} edge_labels={:?} dom_order={:?} nodes={:?} node_style_compile={:?} node_roughjs={:?} node_roughjs_calls={} node_label_html={:?} node_label_html_calls={} nested_roots={:?}",
            timings.total,
            timings.deserialize_model,
            timings.build_ctx,
            timings.viewbox,
            viewbox_edge_curve_bounds,
            detail.viewbox_edge_curve_lca,
            detail.viewbox_edge_curve_offsets,
            detail.viewbox_edge_curve_geom,
            detail.viewbox_edge_curve_bbox_union,
            detail.viewbox_edge_curve_geom_calls,
            detail.viewbox_edge_curve_geom_skipped_bounds,
            timings.render_svg,
            timings.finalize_svg,
            detail.root_calls,
            detail.clusters,
            detail.edges_select,
            detail.edge_paths,
            detail.edge_labels,
            detail.dom_order,
            detail.nodes,
            detail.node_style_compile,
            detail.node_roughjs,
            detail.node_roughjs_calls,
            detail.node_label_html,
            detail.node_label_html_calls,
            detail.nested_roots,
        );
    }
    Ok(out)
}

fn push_flowchart_shadow_defs(
    out: &mut String,
    diagram_id: &str,
    effective_config_value: &serde_json::Value,
) {
    let flood_color = effective_config_value
        .get("theme")
        .and_then(|v| v.as_str())
        .filter(|theme| theme.contains("dark"))
        .map(|_| "#FFFFFF")
        .unwrap_or("#000000");
    let diagram_id = escape_xml(diagram_id);
    let _ = write!(
        out,
        r#"<defs><filter id="{}-drop-shadow" height="130%" width="130%"><feDropShadow dx="4" dy="4" stdDeviation="0" flood-opacity="0.06" flood-color="{}"/></filter></defs><defs><filter id="{}-drop-shadow-small" height="150%" width="150%"><feDropShadow dx="2" dy="2" stdDeviation="0" flood-opacity="0.06" flood-color="{}"/></filter></defs>"#,
        diagram_id.as_str(),
        flood_color,
        diagram_id.as_str(),
        flood_color
    );
}

fn push_flowchart_gradient(
    out: &mut String,
    diagram_id: &str,
    effective_config_value: &serde_json::Value,
) {
    if !config_bool(effective_config_value, &["themeVariables", "useGradient"]).unwrap_or(false) {
        return;
    }

    let gradient_start =
        config_string(effective_config_value, &["themeVariables", "gradientStart"])
            .or_else(|| {
                config_string(
                    effective_config_value,
                    &["themeVariables", "primaryBorderColor"],
                )
            })
            .unwrap_or_else(|| "#9370DB".to_string());
    let gradient_stop = config_string(effective_config_value, &["themeVariables", "gradientStop"])
        .or_else(|| {
            config_string(
                effective_config_value,
                &["themeVariables", "secondaryBorderColor"],
            )
        })
        .unwrap_or_else(|| gradient_start.clone());

    let diagram_id = escape_xml(diagram_id);
    let gradient_start = escape_xml(&gradient_start);
    let gradient_stop = escape_xml(&gradient_stop);
    let _ = write!(
        out,
        r#"<linearGradient id="{}-gradient" gradientUnits="objectBoundingBox" x1="0%" y1="0%" x2="100%" y2="0%"><stop offset="0%" stop-color="{}" stop-opacity="1"/><stop offset="100%" stop-color="{}" stop-opacity="1"/></linearGradient>"#,
        diagram_id.as_str(),
        gradient_start.as_str(),
        gradient_stop.as_str()
    );
}

use super::super::timing::{RenderTimings, TimingGuard, render_timing_enabled};
use super::groups::{
    ClassClusterEdgeGroupsRenderContext, ClassClusterEdgeGroupsRenderState,
    render_class_cluster_edge_groups,
};
use super::namespace::{ClassNamespaceRenderMode, class_namespace_render_mode};
use super::nodes::{
    ClassNodesRenderContext, ClassNodesRenderState, render_class_namespace_subgraph_body,
    render_class_nodes,
};
use super::root::{CLASS_GRAPH_MARGIN_PX, write_class_svg_root_open};
use super::settings::ClassRenderSettings;
use super::viewbox::{ClassViewBoxContext, class_viewbox_attrs};
use super::*;

pub(super) fn render_class_diagram_v2_svg_impl(
    layout: &ClassDiagramV2Layout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model: ClassSvgModel = crate::json::from_value_ref(semantic)?;
    render_class_diagram_v2_svg_model_impl(
        layout,
        &model,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub(super) fn render_class_diagram_v2_svg_model_impl(
    layout: &ClassDiagramV2Layout,
    model: &ClassSvgModel,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    render_class_diagram_v2_svg_model_impl_inner(
        layout,
        model,
        effective_config,
        None,
        diagram_title,
        measurer,
        options,
    )
}

pub(super) fn render_class_diagram_v2_svg_model_impl_with_config(
    layout: &ClassDiagramV2Layout,
    model: &ClassSvgModel,
    effective_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    render_class_diagram_v2_svg_model_impl_inner(
        layout,
        model,
        effective_config.as_value(),
        Some(effective_config),
        diagram_title,
        measurer,
        options,
    )
}

fn render_class_diagram_v2_svg_model_impl_inner(
    layout: &ClassDiagramV2Layout,
    model: &ClassSvgModel,
    effective_config: &serde_json::Value,
    borrowed_sanitize_config: Option<&merman_core::MermaidConfig>,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let timing_enabled = render_timing_enabled();
    let total_start = timing_enabled.then(web_time::Instant::now);
    let mut timings = RenderTimings::default();

    let mut detail = ClassRenderDetails::default();

    let diagram_id = options.diagram_id.as_deref().unwrap_or("merman");
    let aria_roledescription = options.aria_roledescription.as_deref().unwrap_or("class");
    let mut sanitize_config: Option<merman_core::MermaidConfig> = None;

    let build_ctx_guard = timing_enabled.then(|| TimingGuard::new(&mut timings.build_ctx));
    let settings = ClassRenderSettings::from_config(effective_config);

    // Mermaid's class renderer uses Dagre with fixed `marginx/marginy=8`, then calls
    // `setupGraphViewbox(svg, padding=conf.diagramPadding)` which computes the final SVG viewBox
    // from `svg.getBBox()`.
    //
    // Our headless layout output is margin-free, so re-introduce Dagre's margin at render time to
    // match upstream SVG coordinates and viewport sizing.
    let content_tx = CLASS_GRAPH_MARGIN_PX;
    let content_ty = CLASS_GRAPH_MARGIN_PX;

    // Mermaid derives the final viewport using `svg.getBBox()` (after rendering). We don't have a
    // browser DOM, so approximate the effective bbox by accumulating bounds for the elements we
    // emit (using the exact same `d` strings we output for paths).
    let mut content_bounds: Option<Bounds> = None;

    let render_guard = timing_enabled.then(|| TimingGuard::new(&mut timings.render_svg));
    let estimated_svg_bytes = 2048usize
        + model.classes.len().saturating_mul(512)
        + model.relations.len().saturating_mul(384)
        + model.notes.len().saturating_mul(256)
        + model.namespaces.len().saturating_mul(128);
    let mut out = String::with_capacity(estimated_svg_bytes);
    let root_open = write_class_svg_root_open(&mut out, model, diagram_id, aria_roledescription)?;

    // Mermaid emits a single `<style>` element with diagram-scoped CSS.
    let css = class_css(
        diagram_id,
        effective_config,
        settings
            .text_style
            .font_family
            .as_deref()
            .unwrap_or("\"trebuchet ms\", verdana, arial, sans-serif"),
        settings.font_size_css.as_str(),
    );
    out.push_str("<style>");
    out.push_str(&css);
    out.push_str("</style>");

    // Mermaid wraps diagram content (defs + root) in a single `<g>` element.
    out.push_str("<g>");
    class_markers(&mut out, diagram_id, aria_roledescription);

    let ClassRenderLookups {
        class_nodes_by_id,
        relations_by_id,
        relation_index_by_id,
        note_by_id,
        iface_by_id,
    } = ClassRenderLookups::new(model);

    out.push_str(r#"<g class="root">"#);

    // Mermaid sometimes emits a nested dagre-d3 `root` wrapper (translated by -8px on the x-axis).
    // In that mode, the outer `clusters/edgePaths/edgeLabels` groups are empty placeholders, and
    // all cluster + edge rendering happens inside the nested wrapper under `<g class="nodes">`.
    //
    // This affects DOM parity for namespace-heavy diagrams. See upstream fixtures:
    // - `upstream_cypress_classdiagram_handdrawn_v3_spec_hd_should_add_classes_namespaces_039`
    // - `upstream_docs_classdiagram_define_namespace_035`
    // - `upstream_cypress_classdiagram_v2_spec_renders_a_class_diagram_with_nested_namespaces_and_relationships_035`
    let ClassNamespaceRenderMode {
        single_namespace_id,
        wrap_nodes_root,
        nodes_root_dx,
        nodes_root_dy,
        render_namespaces_as_subgraphs,
    } = class_namespace_render_mode(model, &class_nodes_by_id, CLASS_GRAPH_MARGIN_PX);

    drop(build_ctx_guard);

    let marker_url_prefix = {
        let mut out = String::new();
        let _ = write!(&mut out, "{}", escape_attr_display(diagram_id));
        out.push('_');
        let _ = write!(&mut out, "{}", escape_attr_display(aria_roledescription));
        out.push('-');
        out
    };

    let group_ctx = ClassClusterEdgeGroupsRenderContext {
        clusters: &layout.clusters,
        edges: &layout.edges,
        relations_by_id: &relations_by_id,
        relation_index_by_id: &relation_index_by_id,
        marker_url_prefix: &marker_url_prefix,
        diagram_id,
        content_tx,
        content_ty,
        edge_use_html_labels: settings.edge_use_html_labels,
        look: settings.look.as_str(),
        timing_enabled,
    };

    if wrap_nodes_root {
        out.push_str(r#"<g class="clusters"/><g class="edgePaths"/><g class="edgeLabels"/>"#);
    } else if render_namespaces_as_subgraphs {
        out.push_str(r#"<g class="clusters"/>"#);
    } else {
        render_class_cluster_edge_groups(
            ClassClusterEdgeGroupsRenderState {
                out: &mut out,
                content_bounds: &mut content_bounds,
                detail: &mut detail,
            },
            &group_ctx,
            0.0,
            0.0,
            true,
        );
    }

    // Nodes.
    let nodes_start = timing_enabled.then(web_time::Instant::now);

    if wrap_nodes_root {
        out.push_str(r#"<g class="nodes">"#);
        let _ = write!(
            &mut out,
            r#"<g class="root" transform="translate({}, {})">"#,
            fmt(nodes_root_dx),
            fmt(nodes_root_dy)
        );
        render_class_cluster_edge_groups(
            ClassClusterEdgeGroupsRenderState {
                out: &mut out,
                content_bounds: &mut content_bounds,
                detail: &mut detail,
            },
            &group_ctx,
            nodes_root_dx,
            nodes_root_dy,
            true,
        );
        out.push_str(r#"<g class="nodes">"#);
    }

    let nodes_ctx = ClassNodesRenderContext {
        layout,
        model,
        class_nodes_by_id: &class_nodes_by_id,
        note_by_id: &note_by_id,
        iface_by_id: &iface_by_id,
        settings: &settings,
        effective_config,
        diagram_id,
        measurer,
        content_tx,
        content_ty,
        timing_enabled,
        wrap_nodes_root,
        single_namespace_id,
        render_namespaces_as_subgraphs,
        nodes_root_dx,
        nodes_root_dy,
    };
    if render_namespaces_as_subgraphs {
        render_class_namespace_subgraph_body(
            ClassNodesRenderState {
                out: &mut out,
                content_bounds: &mut content_bounds,
                detail: &mut detail,
                sanitize_config: &mut sanitize_config,
                borrowed_sanitize_config,
            },
            &nodes_ctx,
            &group_ctx,
        );
    } else {
        if !wrap_nodes_root {
            out.push_str(r#"<g class="nodes">"#);
        }
        render_class_nodes(
            ClassNodesRenderState {
                out: &mut out,
                content_bounds: &mut content_bounds,
                detail: &mut detail,
                sanitize_config: &mut sanitize_config,
                borrowed_sanitize_config,
            },
            &nodes_ctx,
        );
        out.push_str("</g>"); // outer nodes
    }
    out.push_str("</g>"); // root
    out.push_str("</g>"); // wrapper
    if let Some(s) = nodes_start {
        detail.nodes += s.elapsed();
    }

    push_class_gradient(&mut out, diagram_id, effective_config);

    drop(render_guard);
    let viewbox_guard = timing_enabled.then(|| TimingGuard::new(&mut timings.viewbox));

    let viewbox_attrs = class_viewbox_attrs(ClassViewBoxContext {
        model,
        content_bounds,
        viewport_padding: settings.viewport_padding,
        diagram_title,
        has_acc_title: root_open.has_acc_title,
        has_acc_descr: root_open.has_acc_descr,
    });

    // Mermaid renders the diagram title as a direct child of `<svg>` (outside the wrapper `<g>`),
    // centered in the root viewport.
    if let Some(title) = viewbox_attrs.title.as_ref() {
        let _ = write!(
            &mut out,
            r#"<text text-anchor="middle" x="{}" y="{}" class="classDiagramTitleText">{}</text>"#,
            fmt(title.x),
            fmt(title.y),
            escape_xml_display(title.text)
        );
    }

    drop(viewbox_guard);
    let finalize_guard = timing_enabled.then(|| TimingGuard::new(&mut timings.finalize_svg));

    // Avoid a full-string scan + allocation for placeholder replacement by patching the initial
    // `<svg ...>` attributes in-place.
    out.replace_range(
        root_open.viewbox_placeholder_range,
        viewbox_attrs.view_box_attr.as_str(),
    );
    out.replace_range(
        root_open.max_width_placeholder_range,
        viewbox_attrs.max_w_attr.as_str(),
    );

    push_class_shadow_defs(&mut out, diagram_id, effective_config);
    out.push_str("</svg>");
    drop(finalize_guard);

    if let Some(s) = total_start {
        timings.total = s.elapsed();
        emit_class_render_timing(&timings, &detail, layout);
    }
    Ok(out)
}

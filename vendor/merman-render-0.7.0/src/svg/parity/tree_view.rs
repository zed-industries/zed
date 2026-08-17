use super::*;
use merman_core::diagrams::tree_view::TreeViewDiagramRenderModel;

pub(super) fn render_tree_view_diagram_svg(
    layout: &TreeViewDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model: TreeViewDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    render_tree_view_diagram_svg_model(layout, &model, effective_config, options)
}

pub(super) fn render_tree_view_diagram_svg_model(
    layout: &TreeViewDiagramLayout,
    model: &TreeViewDiagramRenderModel,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let diagram_id = options.diagram_id.as_deref().unwrap_or("treeView");
    let diagram_id_esc = escape_xml(diagram_id);
    let acc_title = model
        .acc_title
        .as_deref()
        .map(str::trim)
        .filter(|t| !t.is_empty());
    let acc_descr = model
        .acc_descr
        .as_deref()
        .map(str::trim)
        .filter(|d| !d.is_empty());
    let aria_labelledby = acc_title.map(|_| format!("chart-title-{diagram_id_esc}"));
    let aria_describedby = acc_descr.map(|_| format!("chart-desc-{diagram_id_esc}"));
    let root_bounds = root_svg::DiagramBounds::from_view_box(
        -layout.line_thickness / 2.0,
        0.0,
        layout.total_width,
        layout.total_height,
    );
    let root_overrides = if options.apply_root_overrides {
        root_svg::resolve_root_overrides(None, None)
    } else {
        None
    };
    let viewport_plan = root_svg::build_root_viewport_plan(
        root_bounds,
        root_overrides.as_ref(),
        layout.use_max_width,
    );

    let mut out = String::new();
    root_svg::push_svg_root_open_with_viewport_plan(
        &mut out,
        root_svg::SvgRootAttrs {
            aria_labelledby: aria_labelledby.as_deref(),
            aria_describedby: aria_describedby.as_deref(),
            trailing_newline: false,
            ..root_svg::SvgRootAttrs::new(diagram_id, "treeView")
        },
        &viewport_plan,
    );

    let css = tree_view_css(effective_config);
    if let Some(title) = acc_title {
        let _ = write!(
            &mut out,
            r#"<title id="chart-title-{}">{}</title>"#,
            diagram_id_esc,
            escape_xml_display(title)
        );
    }
    if let Some(descr) = acc_descr {
        let _ = write!(
            &mut out,
            r#"<desc id="chart-desc-{}">{}</desc>"#,
            diagram_id_esc,
            escape_xml_display(descr)
        );
    }
    let _ = write!(&mut out, "<style>{css}</style>");
    out.push_str("<g/>");
    out.push_str(r#"<g class="tree-view">"#);
    for node in &layout.nodes {
        let _ = write!(
            &mut out,
            r#"<text dominant-baseline="middle" class="treeView-node-label" x="{}" y="{}">{}</text>"#,
            fmt(node.label_x),
            fmt(node.label_y),
            escape_xml(&node.name)
        );
    }
    for line in &layout.lines {
        let _ = write!(
            &mut out,
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke-width="{}" class="treeView-node-line"></line>"#,
            fmt(line.x1),
            fmt(line.y1),
            fmt(line.x2),
            fmt(line.y2),
            fmt(line.stroke_width)
        );
    }
    out.push_str("</g></svg>\n");
    Ok(out)
}

fn tree_view_css(effective_config: &serde_json::Value) -> String {
    let theme = PresentationTheme::new(effective_config).tree_view();

    format!(
        ".treeView-node-label {{ font-size: {}; fill: {}; }} .treeView-node-line {{ stroke: {}; }}",
        theme.label_font_size_css, theme.label_color, theme.line_color
    )
}

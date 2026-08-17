//! Flowchart cluster renderer.

use super::super::*;

const FLOWCHART_CLUSTER_TITLE_WRAP_WIDTH: f64 = 200.0;

pub(in crate::svg::parity) fn render_flowchart_cluster(
    out: &mut String,
    ctx: &FlowchartRenderCtx<'_>,
    cluster: &LayoutCluster,
    origin_x: f64,
    origin_y: f64,
) {
    let Some(sg) = ctx.subgraphs_by_id.get(cluster.id.as_str()) else {
        return;
    };
    if sg.nodes.is_empty() {
        return;
    }

    let compiled_styles = flowchart_compile_styles(ctx.class_defs, &sg.classes, &sg.styles, &[]);
    let rect_style = compiled_styles.node_style.trim();
    let label_style = compiled_styles.label_style.trim();

    let left = (cluster.x - cluster.width / 2.0) + ctx.tx - origin_x;
    let top = (cluster.y - cluster.height / 2.0) + ctx.ty - origin_y;
    let rect_w = cluster.width.max(1.0);
    let rect_h = cluster.height.max(1.0);
    let label_top = top + cluster.title_margin_top.max(0.0);

    let label_type = sg.label_type.as_deref().unwrap_or("text");

    let mut class_attr = String::new();
    for c in &sg.classes {
        let c = c.trim();
        if c.is_empty() {
            continue;
        }
        if !class_attr.is_empty() {
            class_attr.push(' ');
        }
        class_attr.push_str(c);
    }
    if !class_attr.is_empty() {
        class_attr.push(' ');
    }
    class_attr.push_str("cluster");
    let data_look = flowchart_config_look(ctx.config);

    // Mermaid renders subgraph titles using the same `flowchart.htmlLabels` toggle as edge labels.
    if !ctx.edge_html_labels {
        let label_w = cluster.title_label.width.max(0.0);
        let label_left = left + rect_w / 2.0 - label_w / 2.0;
        let title_text = flowchart_label_plain_text(&cluster.title, label_type, false);
        let _ = write!(
            out,
            r#"<g class="{}" id="{}-{}" data-look="{}"><rect style="{}" x="{}" y="{}" width="{}" height="{}"/><g class="cluster-label" transform="translate({},{})"><g><rect class="background" style="stroke: none"/>"#,
            escape_xml_display(&class_attr),
            escape_xml_display(ctx.diagram_id),
            escape_xml_display(&cluster.id),
            escape_xml_display(data_look),
            escape_xml_display(rect_style),
            fmt_display(left),
            fmt_display(top),
            fmt_display(rect_w),
            fmt_display(rect_h),
            fmt_display(label_left),
            fmt_display(label_top)
        );
        if label_type == "markdown" {
            write_flowchart_svg_text_markdown(out, &cluster.title, true);
        } else {
            write_flowchart_svg_text(out, &title_text, true);
        }
        out.push_str("</g></g></g>");
        return;
    }

    let title_html =
        flowchart_label_html(&cluster.title, label_type, ctx.config, ctx.math_renderer);
    let label_w = cluster.title_label.width.max(0.0);
    let label_h = cluster.title_label.height.max(0.0);
    let label_left = left + rect_w / 2.0 - label_w / 2.0;

    let span_style_attr = OptionalStyleXmlAttr(label_style);
    let div_style = if label_type != "markdown" {
        "display: table-cell; white-space: nowrap; line-height: 1.5;".to_string()
    } else if label_w >= FLOWCHART_CLUSTER_TITLE_WRAP_WIDTH - 1e-3 {
        format!(
            "display: table; white-space: break-spaces; line-height: 1.5; max-width: {mw}px; text-align: center; width: {mw}px;",
            mw = fmt_display(FLOWCHART_CLUSTER_TITLE_WRAP_WIDTH)
        )
    } else {
        format!(
            "display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {mw}px; text-align: center;",
            mw = fmt_display(FLOWCHART_CLUSTER_TITLE_WRAP_WIDTH)
        )
    };

    let _ = write!(
        out,
        r#"<g class="{}" id="{}-{}" data-look="{}"><rect style="{}" x="{}" y="{}" width="{}" height="{}"/><g class="cluster-label" transform="translate({},{})"><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="{}"><span class="nodeLabel"{}>{}</span></div></foreignObject></g></g>"#,
        escape_xml_display(&class_attr),
        escape_xml_display(ctx.diagram_id),
        escape_xml_display(&cluster.id),
        escape_xml_display(data_look),
        escape_xml_display(rect_style),
        fmt_display(left),
        fmt_display(top),
        fmt_display(rect_w),
        fmt_display(rect_h),
        fmt_display(label_left),
        fmt_display(label_top),
        fmt_display(label_w),
        fmt_display(label_h),
        escape_xml_display(&div_style),
        span_style_attr,
        title_html
    );
}

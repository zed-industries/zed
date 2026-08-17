//! Flowchart v2 stadium shape.

use std::fmt::Write as _;

use crate::svg::parity::{escape_attr, fmt, fmt_display};

use super::super::geom::path_from_points;
use super::super::roughjs::roughjs_paths_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn render_stadium(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    // Port of Mermaid `@11.12.2` `stadium.ts` points + `createPathFromPoints`.
    // Note that Mermaid's `generateCirclePoints()` pushes negated coordinates.
    fn generate_circle_points(
        center_x: f64,
        center_y: f64,
        radius: f64,
        num_points: usize,
        start_angle_deg: f64,
        end_angle_deg: f64,
    ) -> Vec<(f64, f64)> {
        let start = start_angle_deg.to_radians();
        let end = end_angle_deg.to_radians();
        let angle_range = end - start;
        let step = angle_range / (num_points.saturating_sub(1).max(1) as f64);
        let mut pts: Vec<(f64, f64)> = Vec::with_capacity(num_points);
        for i in 0..num_points {
            let angle = start + (i as f64) * step;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            pts.push((-x, -y));
        }
        pts
    }

    // Mermaid flowchart-v2 updates `node.width/height` from the rendered rough path bbox
    // (`updateNodeBounds`) before running Dagre layout. That bbox is narrower than the
    // theoretical `(text bbox + padding)` width used to generate the stadium points. The
    // SVG path is still generated from the theoretical width, so we recompute it here.
    let node_text_style = crate::flowchart::flowchart_effective_text_style_for_node_classes(
        &ctx.text_style,
        ctx.class_defs,
        common.node_classes,
        &[],
    );
    let node_font_style = crate::flowchart::flowchart_effective_font_style_for_node_classes(
        ctx.class_defs,
        common.node_classes,
        common.node_styles,
    );
    let mut metrics = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: ctx.measurer,
            raw_label: label.text,
            label_type: label.label_type,
            style: &node_text_style,
            max_width_px: Some(ctx.wrapping_width),
            wrap_mode: ctx.node_wrap_mode,
            config: ctx.config,
            math_renderer: ctx.math_renderer,
            preserve_string_whitespace_height: ctx.node_html_labels && ctx.edge_html_labels,
            whole_label_font_style: node_font_style.as_deref(),
        },
    );
    let span_css_height_parity = crate::flowchart::flowchart_node_has_span_css_height_parity(
        ctx.class_defs,
        common.node_classes,
    );
    if ctx.node_html_labels && ctx.edge_html_labels && span_css_height_parity {
        crate::text::flowchart_apply_mermaid_styled_node_height_parity(
            &mut metrics,
            &node_text_style,
        );
    }
    let (render_w, render_h) = crate::flowchart::flowchart_node_render_dimensions(
        Some("stadium"),
        metrics,
        ctx.node_padding,
    );

    let w = render_w.max(1.0);
    let h = render_h.max(1.0);
    let radius = h / 2.0;

    let mut pts: Vec<(f64, f64)> = Vec::new();
    pts.push((-w / 2.0 + radius, -h / 2.0));
    pts.push((w / 2.0 - radius, -h / 2.0));
    pts.extend(generate_circle_points(
        -w / 2.0 + radius,
        0.0,
        radius,
        50,
        90.0,
        270.0,
    ));
    pts.push((w / 2.0 - radius, h / 2.0));
    pts.extend(generate_circle_points(
        w / 2.0 - radius,
        0.0,
        radius,
        50,
        270.0,
        450.0,
    ));
    let path_data = path_from_points(&pts);

    if let Some((fill_d, stroke_d)) =
        super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
            roughjs_paths_for_svg_path(
                &path_data,
                common.fill_color,
                common.stroke_color,
                common.stroke_width,
                common.stroke_dasharray,
                common.hand_drawn_seed,
            )
        })
    {
        out.push_str(r#"<g class="basic label-container outer-path">"#);
        let _ = write!(
            out,
            r#"<path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/>"#,
            escape_attr(&fill_d),
            escape_attr(common.fill_color),
            escape_attr(common.style)
        );
        let _ = write!(
            out,
            r#"<path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/>"#,
            escape_attr(&stroke_d),
            escape_attr(common.stroke_color),
            fmt_display(common.stroke_width as f64),
            escape_attr(common.stroke_dasharray),
            escape_attr(common.style)
        );
        out.push_str("</g>");
    } else {
        let _ = write!(
            out,
            r#"<rect class="basic label-container" style="{}" x="{}" y="{}" width="{}" height="{}" rx="{}" ry="{}"/>"#,
            escape_attr(common.style),
            fmt(-w / 2.0),
            fmt(-h / 2.0),
            fmt(w),
            fmt(h),
            fmt(radius),
            fmt(radius)
        );
    }
}

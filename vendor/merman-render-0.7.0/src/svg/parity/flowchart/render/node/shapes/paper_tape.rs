//! Flowchart v2 paper tape (flag) shape.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::fmt_display;

use super::super::geom::{generate_full_sine_wave_points, path_from_points};
use super::super::helpers;
use super::super::roughjs::roughjs_paths_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn render_paper_tape(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    let metrics = helpers::compute_node_label_metrics(
        ctx,
        Some(common.layout_node),
        label.text,
        label.label_type,
        common.node_classes,
        common.node_styles,
    );

    let p = ctx.node_padding;
    let w = (metrics.width + 2.0 * p).max(0.0);
    let h = (metrics.height + p).max(0.0);
    let wave_amplitude = h / 8.0;
    let final_h = h + wave_amplitude * 2.0;

    let mut points: Vec<(f64, f64)> = Vec::new();
    points.push((-w / 2.0, final_h / 2.0));
    points.extend(generate_full_sine_wave_points(
        -w / 2.0,
        final_h / 2.0,
        w / 2.0,
        final_h / 2.0,
        wave_amplitude,
        1.0,
    ));
    points.push((w / 2.0, -final_h / 2.0));
    points.extend(generate_full_sine_wave_points(
        w / 2.0,
        -final_h / 2.0,
        -w / 2.0,
        -final_h / 2.0,
        wave_amplitude,
        -1.0,
    ));

    let path_data = path_from_points(&points);
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
        out.push_str(r#"<g class="basic label-container">"#);
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
    }
}

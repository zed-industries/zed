//! Flowchart v2 wave edged rectangle (Document).

use std::fmt::Write as _;

use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::util;

use super::super::geom::{generate_full_sine_wave_points, path_from_points};
use super::super::helpers;
use super::super::roughjs::roughjs_paths_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn render_wave_document(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &mut super::super::FlowchartNodeLabelState<'_>,
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
    // Mermaid creates this shape once during the DOM-backed measurement pass, then Dagre uses the
    // resulting `getBBox()` as `node.width/height`. Re-feeding those bbox dimensions into our
    // separate render pass would inflate the wave geometry a second time.
    let w = (metrics.width + 2.0 * p).max(0.0);
    let h = (metrics.height + 2.0 * p).max(0.0);
    let wave_amplitude = h / 8.0;
    let final_h = h + wave_amplitude;

    // Mermaid keeps a minimum width (14px) for wave edged rectangles.
    let min_width = 14.0;
    let extra_w = ((min_width - w).max(0.0)) / 2.0;

    let mut points: Vec<(f64, f64)> = Vec::new();
    points.push((-w / 2.0 - extra_w, final_h / 2.0));
    points.extend(generate_full_sine_wave_points(
        -w / 2.0 - extra_w,
        final_h / 2.0,
        w / 2.0 + extra_w,
        final_h / 2.0,
        wave_amplitude,
        0.8,
    ));
    points.push((w / 2.0 + extra_w, -final_h / 2.0));
    points.push((-w / 2.0 - extra_w, -final_h / 2.0));

    let path_data = path_from_points(&points);
    let (fill_d, stroke_d) =
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
        .unwrap_or_else(|| ("M0,0".to_string(), "M0,0".to_string()));
    let _ = write!(
        out,
        r##"<g class="basic label-container outer-path" transform="translate(0,{})"><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/></g>"##,
        util::fmt(-wave_amplitude / 2.0),
        escape_attr(&fill_d),
        escape_attr(common.fill_color),
        escape_attr(common.style),
        escape_attr(&stroke_d),
        escape_attr(common.stroke_color),
        util::fmt_display(common.stroke_width as f64),
        escape_attr(common.stroke_dasharray),
        escape_attr(common.style),
    );

    // Mirror Mermaid `waveEdgedRectangle.ts` label placement.
    label.dx = -w / 2.0 + p + metrics.width / 2.0;
    label.dy = -h / 2.0 + p - wave_amplitude + metrics.height / 2.0;
}

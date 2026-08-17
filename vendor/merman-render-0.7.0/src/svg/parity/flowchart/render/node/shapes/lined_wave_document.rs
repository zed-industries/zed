//! Flowchart v2 lined wave edged rectangle (Lined document).

use std::fmt::Write as _;

use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::util;

use super::super::geom::generate_full_sine_wave_points;
use super::super::helpers;
use super::super::roughjs::roughjs_paths_for_polygon;

pub(in crate::svg::parity::flowchart::render::node) fn render_lined_wave_document(
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
    let w = (metrics.width + 2.0 * p).max(0.0);
    let h = (metrics.height + 2.0 * p).max(0.0);
    let wave_amplitude = if common.look_is_neo() {
        h / 4.0
    } else {
        h / 8.0
    };
    let final_h = h + wave_amplitude;
    let ext = (w / 2.0) * 0.1;

    // Mermaid nudges label by half the left extension, and shifts it up by waveAmplitude/2.
    label.dx = ext / 2.0;
    label.dy = -wave_amplitude / 2.0;

    let mut points: Vec<(f64, f64)> = Vec::new();
    points.push((-w / 2.0 - ext, -final_h / 2.0));
    points.push((-w / 2.0 - ext, final_h / 2.0));
    points.extend(generate_full_sine_wave_points(
        -w / 2.0 - ext,
        final_h / 2.0,
        w / 2.0 + ext,
        final_h / 2.0,
        wave_amplitude,
        0.8,
    ));
    points.push((w / 2.0 + ext, -final_h / 2.0));
    points.push((-w / 2.0 - ext, -final_h / 2.0));
    points.push((-w / 2.0, -final_h / 2.0));
    points.push((-w / 2.0, (final_h / 2.0) * 1.1));
    points.push((-w / 2.0, -final_h / 2.0));

    let (fill_d, stroke_d) =
        super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
            roughjs_paths_for_polygon(
                &points,
                common.fill_color,
                common.stroke_color,
                common.stroke_width,
                common.hand_drawn_seed,
            )
        })
        .unwrap_or_else(|| ("M0,0".to_string(), "M0,0".to_string()));
    let _ = write!(
        out,
        r##"<g class="basic label-container outer-path" transform="translate(0,{})"><path d="{}" stroke="none" stroke-width="0" fill="{}" fill-rule="evenodd" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/></g>"##,
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
}

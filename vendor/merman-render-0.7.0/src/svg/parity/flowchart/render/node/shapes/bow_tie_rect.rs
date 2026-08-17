//! Flowchart v2 bow tie rectangle (Stored data).

use std::fmt::Write as _;

use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::util;

use super::super::geom::{arc_points, path_from_points};
use super::super::helpers;
use super::super::roughjs::roughjs_paths_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn render_bow_tie_rect(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    let metrics = helpers::compute_node_label_metrics(
        ctx,
        None,
        label.text,
        label.label_type,
        common.node_classes,
        common.node_styles,
    );

    let p = ctx.node_padding;
    let w = metrics.width + 2.0 * p;
    let h = metrics.height + p;
    let ry = h / 2.0;
    let rx = ry / (2.5 + h / 50.0);

    let mut points: Vec<(f64, f64)> = Vec::new();
    points.push((w / 2.0, -h / 2.0));
    points.push((-w / 2.0, -h / 2.0));
    points.extend(arc_points(
        -w / 2.0,
        -h / 2.0,
        -w / 2.0,
        h / 2.0,
        rx,
        ry,
        false,
    ));
    points.push((w / 2.0, h / 2.0));
    points.extend(arc_points(
        w / 2.0,
        h / 2.0,
        w / 2.0,
        -h / 2.0,
        rx,
        ry,
        true,
    ));

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
        r##"<g class="basic label-container outer-path" transform="translate({},0)"><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/></g>"##,
        util::fmt(rx / 2.0),
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

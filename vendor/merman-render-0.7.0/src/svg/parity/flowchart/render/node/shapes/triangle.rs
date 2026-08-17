//! Flowchart v2 triangle (Extract).

use std::fmt::Write as _;

use crate::flowchart::flowchart_effective_text_style_for_node_classes;
use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::util;

use super::super::geom::path_from_points;
use super::super::helpers;
use super::super::roughjs::roughjs_paths_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn render_triangle_extract(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &mut super::super::FlowchartNodeLabelState<'_>,
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
    let w = metrics.width + p;
    let h = w + metrics.height;
    let tw = w + metrics.height;
    let pts = vec![(0.0, 0.0), (tw, 0.0), (tw / 2.0, -h)];
    let path_data = path_from_points(&pts);
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
        r#"<g transform="translate({},{})" class="outer-path"><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/></g>"#,
        util::fmt(-h / 2.0),
        util::fmt(h / 2.0),
        escape_attr(&fill_d),
        escape_attr(common.fill_color),
        escape_attr(common.style),
        escape_attr(&stroke_d),
        escape_attr(common.stroke_color),
        util::fmt(common.stroke_width as f64),
        escape_attr(common.stroke_dasharray),
        escape_attr(common.style),
    );

    let node_text_style = flowchart_effective_text_style_for_node_classes(
        &ctx.text_style,
        ctx.class_defs,
        common.node_classes,
        common.node_styles,
    );
    let bbox_y_offset = if ctx.node_html_labels {
        0.0
    } else {
        crate::text::svg_create_text_bbox_y_offset_px(&node_text_style)
    };
    let padding_term = if ctx.node_html_labels { p / 2.0 } else { p };
    label.dy = h / 2.0 - metrics.height / 2.0 - padding_term + bbox_y_offset;
}

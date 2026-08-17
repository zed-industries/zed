//! Flowchart v2 rounded rectangle shape.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::flowchart::flowchart_config_diagram_look;
use crate::svg::parity::{fmt, fmt_display};

use super::super::geom::{arc_points, path_from_points};
use super::super::roughjs::roughjs_paths_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn render_rounded_rect(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    let w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let radius = 5.0;
    let taper = 5.0;

    let mut pts: Vec<(f64, f64)> = Vec::new();
    pts.push((-w / 2.0 + taper, -h / 2.0));
    pts.push((w / 2.0 - taper, -h / 2.0));
    pts.extend(arc_points(
        w / 2.0 - taper,
        -h / 2.0,
        w / 2.0,
        -h / 2.0 + taper,
        radius,
        radius,
        true,
    ));
    pts.push((w / 2.0, -h / 2.0 + taper));
    pts.push((w / 2.0, h / 2.0 - taper));
    pts.extend(arc_points(
        w / 2.0,
        h / 2.0 - taper,
        w / 2.0 - taper,
        h / 2.0,
        radius,
        radius,
        true,
    ));
    pts.push((w / 2.0 - taper, h / 2.0));
    pts.push((-w / 2.0 + taper, h / 2.0));
    pts.extend(arc_points(
        -w / 2.0 + taper,
        h / 2.0,
        -w / 2.0,
        h / 2.0 - taper,
        radius,
        radius,
        true,
    ));
    pts.push((-w / 2.0, h / 2.0 - taper));
    pts.push((-w / 2.0, -h / 2.0 + taper));
    pts.extend(arc_points(
        -w / 2.0,
        -h / 2.0 + taper,
        -w / 2.0 + taper,
        -h / 2.0,
        radius,
        radius,
        true,
    ));
    let path_data = path_from_points(&pts);

    let rough_paths = if flowchart_config_diagram_look(ctx.config).is_hand_drawn() {
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
    } else {
        None
    };

    if let Some((fill_d, stroke_d)) = rough_paths {
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
            r#"<rect class="basic label-container" style="{}" x="{}" y="{}" width="{}" height="{}" rx="5" ry="5"/>"#,
            escape_attr(common.style),
            fmt(-w / 2.0),
            fmt(-h / 2.0),
            fmt(w),
            fmt(h)
        );
    }
}

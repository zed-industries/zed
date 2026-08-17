//! Flowchart v2 odd shape.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::{OptionalStyleAttr, escape_attr};
use crate::svg::parity::{fmt, fmt_display};

use super::super::geom::path_from_points;
use super::super::roughjs::roughjs_paths_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn render_odd(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &mut super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    let total_w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let w = (total_w - h / 4.0).max(1.0);
    let x = -w / 2.0;
    let y = -h / 2.0;
    let notch = y / 2.0;
    let dx = -notch / 2.0;
    label.dx = dx;

    let pts: Vec<(f64, f64)> = vec![(x + notch, y), (x, 0.0), (x + notch, -y), (-x, -y), (-x, y)];
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
        let _ = write!(
            out,
            r#"<g class="basic label-container outer-path" transform="translate({},0)">"#,
            fmt(dx)
        );
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
            r#"<polygon points="{},{} {},{} {},{} {},{} {},{}" class="label-container outer-path" transform="translate({},{})"{} />"#,
            fmt(x + notch),
            fmt(y),
            fmt(x),
            fmt(0.0),
            fmt(x + notch),
            fmt(-y),
            fmt(-x),
            fmt(-y),
            fmt(-x),
            fmt(y),
            fmt(dx),
            fmt(0.0),
            OptionalStyleAttr(common.style)
        );
    }
}

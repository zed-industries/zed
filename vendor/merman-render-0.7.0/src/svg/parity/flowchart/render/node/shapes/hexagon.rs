//! Flowchart v2 hexagon shape.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::{OptionalStyleAttr, escape_attr};
use crate::svg::parity::fmt_display;

use super::super::geom::path_from_points;
use super::super::roughjs::roughjs_paths_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn render_hexagon(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    let w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let f = if common.look_is_neo() { 3.5 } else { 4.0 };
    let m = h / f;

    let pts: Vec<(f64, f64)> = vec![
        (m, 0.0),
        (w - m, 0.0),
        (w, -h / 2.0),
        (w - m, -h),
        (m, -h),
        (0.0, -h / 2.0),
    ];
    let path_data = path_from_points(&pts);

    let rough_paths = if common.look_is_hand_drawn() {
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
        let _ = write!(
            out,
            r#"<g class="basic label-container" transform="translate({},{})">"#,
            fmt_display(-w / 2.0),
            fmt_display(h / 2.0)
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
            r#"<polygon points="{},{} {},{} {},{} {},{} {},{} {},{}" class="label-container" transform="translate({},{})"{} />"#,
            fmt_display(m),
            fmt_display(0.0),
            fmt_display(w - m),
            fmt_display(0.0),
            fmt_display(w),
            fmt_display(-h / 2.0),
            fmt_display(w - m),
            fmt_display(-h),
            fmt_display(m),
            fmt_display(-h),
            fmt_display(0.0),
            fmt_display(-h / 2.0),
            fmt_display(-w / 2.0),
            fmt_display(h / 2.0),
            OptionalStyleAttr(common.style)
        );
    }
}

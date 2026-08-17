//! Flowchart v2 window pane shape.

use std::fmt::Write as _;

use crate::svg::parity::{escape_attr, fmt};

use super::super::roughjs::roughjs_paths_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn render_window_pane(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &mut super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    // Mermaid `windowPane.ts` (non-handDrawn): RoughJS multi-subpath with `roughness=0` + a
    // fixed `rectOffset=10` and a translation of `(+5, +5)`.
    let rect_offset = 10.0;
    let out_w = common.layout_node.width.max(1.0);
    let out_h = common.layout_node.height.max(1.0);
    let w = (out_w - rect_offset).max(1.0);
    let h = (out_h - rect_offset).max(1.0);
    let x = -w / 2.0;
    let y = -h / 2.0;

    // Label transform includes the same `rectOffset/2` shift as the container.
    label.dx = rect_offset / 2.0;
    label.dy = rect_offset / 2.0;

    let path_data = format!(
        "M{},{} L{},{} L{},{} L{},{} L{},{} M{},{} L{},{} M{},{} L{},{}",
        fmt(x - rect_offset),
        fmt(y - rect_offset),
        fmt(x + w),
        fmt(y - rect_offset),
        fmt(x + w),
        fmt(y + h),
        fmt(x - rect_offset),
        fmt(y + h),
        fmt(x - rect_offset),
        fmt(y - rect_offset),
        fmt(x - rect_offset),
        fmt(y),
        fmt(x + w),
        fmt(y),
        fmt(x),
        fmt(y - rect_offset),
        fmt(x),
        fmt(y + h),
    );

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
            r#"<g transform="translate({},{})" class="basic label-container outer-path">"#,
            fmt(rect_offset / 2.0),
            fmt(rect_offset / 2.0)
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
            fmt(common.stroke_width as f64),
            escape_attr(common.stroke_dasharray),
            escape_attr(common.style)
        );
        out.push_str("</g>");
    }
}

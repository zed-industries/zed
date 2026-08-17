//! Flowchart v2 stacked rectangle shape.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::{OptionalStyleAttr, escape_attr};
use crate::svg::parity::fmt_display;

use super::super::geom::path_from_points;
use super::super::roughjs::roughjs_paths_for_svg_path;

fn write_stacked_rectangle_rough_path_group(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    fill_d: &str,
    stroke_d: &str,
) {
    if common.look_is_hand_drawn() {
        let _ = write!(
            out,
            r#"<g><path d="{}" stroke="none" stroke-width="0" fill="{}"{} /><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}"{} /></g>"#,
            escape_attr(fill_d),
            escape_attr(common.fill_color),
            OptionalStyleAttr(common.style),
            escape_attr(stroke_d),
            escape_attr(common.stroke_color),
            fmt_display(common.stroke_width as f64),
            escape_attr(common.stroke_dasharray),
            OptionalStyleAttr(common.style),
        );
        return;
    }

    let _ = write!(
        out,
        r#"<g><path d="{} {}" fill="{}" fill-opacity="1" stroke="{}" stroke-opacity="1" stroke-width="{}"{} /></g>"#,
        escape_attr(fill_d),
        escape_attr(stroke_d),
        escape_attr(common.fill_color),
        escape_attr(common.stroke_color),
        fmt_display(common.stroke_width as f64),
        OptionalStyleAttr(common.style),
    );
}

pub(in crate::svg::parity::flowchart::render::node) fn render_stacked_rectangle(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &mut super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    let rect_offset = 5.0;
    let total_w = common.layout_node.width.max(1.0);
    let total_h = common.layout_node.height.max(1.0);
    let w = (total_w - 2.0 * rect_offset).max(1.0);
    let h = (total_h - 2.0 * rect_offset).max(1.0);
    let x = -w / 2.0;
    let y = -h / 2.0;

    label.dx = -rect_offset;
    label.dy = rect_offset;

    let outer_points = vec![
        (x - rect_offset, y + rect_offset),
        (x - rect_offset, y + h + rect_offset),
        (x + w - rect_offset, y + h + rect_offset),
        (x + w - rect_offset, y + h),
        (x + w, y + h),
        (x + w, y + h - rect_offset),
        (x + w + rect_offset, y + h - rect_offset),
        (x + w + rect_offset, y - rect_offset),
        (x + rect_offset, y - rect_offset),
        (x + rect_offset, y),
        (x, y),
        (x, y + rect_offset),
    ];

    let inner_points = vec![
        (x, y + rect_offset),
        (x + w - rect_offset, y + rect_offset),
        (x + w - rect_offset, y + h),
        (x + w, y + h),
        (x + w, y),
        (x, y),
    ];

    let outer_path = path_from_points(&outer_points);
    let inner_path = path_from_points(&inner_points);

    out.push_str(r#"<g class="basic label-container outer-path">"#);
    if let Some((fill_d, stroke_d)) =
        super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
            roughjs_paths_for_svg_path(
                &outer_path,
                common.fill_color,
                common.stroke_color,
                common.stroke_width,
                common.stroke_dasharray,
                common.hand_drawn_seed,
            )
        })
    {
        write_stacked_rectangle_rough_path_group(out, common, &fill_d, &stroke_d);
    }
    if let Some((fill_d, stroke_d)) =
        super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
            roughjs_paths_for_svg_path(
                &inner_path,
                common.fill_color,
                common.stroke_color,
                common.stroke_width,
                common.stroke_dasharray,
                common.hand_drawn_seed,
            )
        })
    {
        write_stacked_rectangle_rough_path_group(out, common, &fill_d, &stroke_d);
    }
    out.push_str("</g>");
}

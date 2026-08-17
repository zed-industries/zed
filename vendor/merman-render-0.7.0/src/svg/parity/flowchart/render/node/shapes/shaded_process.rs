//! Flowchart v2 shaded process / lined rectangle.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::fmt_display;

use super::super::roughjs::roughjs_paths_for_polygon;

pub(in crate::svg::parity::flowchart::render::node) fn render_shaded_process(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &mut super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    // Mermaid `shadedProcess.ts`:
    // - outer bbox includes an extra 8px on both sides (and an internal vertical line),
    // - label is nudged +4px on x (handled by caller).
    let out_w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let w = (out_w - 16.0).max(1.0);
    let x = -out_w / 2.0 + 8.0;
    let y = -h / 2.0;
    label.dx = 4.0;
    let pts: Vec<(f64, f64)> = vec![
        (x, y),
        (x + w + 8.0, y),
        (x + w + 8.0, y + h),
        (x - 8.0, y + h),
        (x - 8.0, y),
        (x, y),
        (x, y + h),
    ];
    let (fill_d, stroke_d) =
        super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
            roughjs_paths_for_polygon(
                &pts,
                common.fill_color,
                common.stroke_color,
                common.stroke_width,
                common.hand_drawn_seed,
            )
        })
        .unwrap_or_else(|| ("M0,0".to_string(), "M0,0".to_string()));
    let _ = write!(
        out,
        r##"<g class="basic label-container outer-path" style=""><path d="{}" stroke="none" stroke-width="0" fill="{}" fill-rule="evenodd" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/></g>"##,
        escape_attr(&fill_d),
        escape_attr(common.fill_color),
        escape_attr(common.style),
        escape_attr(&stroke_d),
        escape_attr(common.stroke_color),
        fmt_display(common.stroke_width as f64),
        escape_attr(common.stroke_dasharray),
        escape_attr(common.style),
    );
}

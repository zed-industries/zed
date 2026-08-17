//! Flowchart v2 subroutine shape.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::OptionalStyleAttr;
use crate::svg::parity::fmt_display;

pub(in crate::svg::parity::flowchart::render::node) fn render_subroutine(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
) {
    // Mermaid `subroutine.ts` (non-handDrawn): polygon via `insertPolygonShape(...)`.
    let total_w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let w = (total_w - 16.0).max(1.0);

    let pts: Vec<(f64, f64)> = vec![
        (0.0, 0.0),
        (w, 0.0),
        (w, -h),
        (0.0, -h),
        (0.0, 0.0),
        (-8.0, 0.0),
        (w + 8.0, 0.0),
        (w + 8.0, -h),
        (-8.0, -h),
        (-8.0, 0.0),
    ];
    let mut points_attr = String::new();
    for (idx, (px, py)) in pts.iter().copied().enumerate() {
        if idx > 0 {
            points_attr.push(' ');
        }
        let _ = write!(&mut points_attr, "{},{}", fmt_display(px), fmt_display(py));
    }
    let _ = write!(
        out,
        r#"<polygon points="{}" class="label-container" transform="translate({},{})"{} />"#,
        points_attr,
        fmt_display(-w / 2.0),
        fmt_display(h / 2.0),
        OptionalStyleAttr(common.style)
    );
}

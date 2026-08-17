//! Flowchart v2 lean/trapezoid polygon shapes.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::OptionalStyleAttr;
use crate::svg::parity::fmt_display;

pub(in crate::svg::parity::flowchart::render::node) fn render_lean_right(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
) {
    // Mermaid `leanRight.ts` (non-handDrawn): polygon via `insertPolygonShape(...)`.
    let total_w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let w = (total_w - h).max(1.0);
    let dx = (3.0 * h) / 6.0;
    let pts: Vec<(f64, f64)> = vec![(-dx, 0.0), (w, 0.0), (w + dx, -h), (0.0, -h)];
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

pub(in crate::svg::parity::flowchart::render::node) fn render_lean_left(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
) {
    // Mermaid `leanLeft.ts` (non-handDrawn): polygon via `insertPolygonShape(...)`.
    let total_w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let w = (total_w - h).max(1.0);
    let dx = (3.0 * h) / 6.0;
    let pts: Vec<(f64, f64)> = vec![(0.0, 0.0), (w + dx, 0.0), (w, -h), (-dx, -h)];
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

pub(in crate::svg::parity::flowchart::render::node) fn render_trapezoid(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
) {
    // Mermaid `trapezoid.ts` (non-handDrawn): polygon via `insertPolygonShape(...)`.
    let total_w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let w = (total_w - h).max(1.0);
    let dx = (3.0 * h) / 6.0;
    let pts: Vec<(f64, f64)> = vec![(-dx, 0.0), (w + dx, 0.0), (w, -h), (0.0, -h)];
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

pub(in crate::svg::parity::flowchart::render::node) fn render_inv_trapezoid(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
) {
    // Mermaid `invertedTrapezoid.ts` (non-handDrawn): polygon via `insertPolygonShape(...)`.
    let total_w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let w = (total_w - h).max(1.0);
    let dx = (3.0 * h) / 6.0;
    let pts: Vec<(f64, f64)> = vec![(0.0, 0.0), (w, 0.0), (w + dx, -h), (-dx, -h)];
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

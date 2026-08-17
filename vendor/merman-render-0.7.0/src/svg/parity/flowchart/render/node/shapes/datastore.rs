//! Flowchart v2 datastore shape.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::fmt;

pub(in crate::svg::parity::flowchart::render::node) fn render_datastore(
    out: &mut String,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
) {
    let w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let _ = write!(
        out,
        r#"<rect class="basic label-container" style="{}" rx="0" ry="0" x="{}" y="{}" width="{}" height="{}" stroke-dasharray="{} {}"/>"#,
        escape_attr(common.style),
        fmt(-w / 2.0),
        fmt(-h / 2.0),
        fmt(w),
        fmt(h),
        fmt(w),
        fmt(h)
    );
}

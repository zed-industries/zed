//! Flowchart v2 note shape.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::escape_attr;
use crate::svg::parity::util;
use crate::svg::parity::{fmt, fmt_display};

use super::super::roughjs::{RoughRectSpec, roughjs_paths_for_rect};

pub(in crate::svg::parity::flowchart::render::node) fn render_note(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) {
    let w = common.layout_node.width.max(1.0);
    let h = common.layout_node.height.max(1.0);
    let x = -w / 2.0;
    let y = -h / 2.0;

    let note_fill = util::theme_color(ctx.config.as_value(), "noteBkgColor", "#fff5ad");
    let note_stroke = util::theme_color(ctx.config.as_value(), "noteBorderColor", "#aaaa33");

    if let Some((fill_d, stroke_d)) =
        super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
            roughjs_paths_for_rect(RoughRectSpec {
                x,
                y,
                w,
                h,
                fill: &note_fill,
                stroke: &note_stroke,
                stroke_width: common.stroke_width,
                seed: common.hand_drawn_seed,
            })
        })
    {
        let _ = write!(out, r#"<g class="basic label-container outer-path">"#);
        let _ = write!(
            out,
            r#"<path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/>"#,
            escape_attr(&fill_d),
            escape_attr(&note_fill),
            escape_attr(common.style)
        );
        let _ = write!(
            out,
            r#"<path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}" style="{}"/>"#,
            escape_attr(&stroke_d),
            escape_attr(&note_stroke),
            fmt_display(common.stroke_width as f64),
            escape_attr(common.stroke_dasharray),
            escape_attr(common.style)
        );
        out.push_str("</g>");
    } else {
        // Fallback: basic rect.
        let _ = write!(
            out,
            r#"<rect class="basic label-container outer-path" style="{}" x="{}" y="{}" width="{}" height="{}"/>"#,
            escape_attr(common.style),
            fmt(x),
            fmt(y),
            fmt(w),
            fmt(h)
        );
    }
}

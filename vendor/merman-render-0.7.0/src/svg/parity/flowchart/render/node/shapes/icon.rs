//! Flowchart v2 icon shape.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::{
    escape_attr, flowchart_label_html, flowchart_label_plain_text,
};
use crate::svg::parity::fmt;

pub(in crate::svg::parity::flowchart::render::node) fn try_render_icon(
    out: &mut String,
    ctx: &crate::svg::parity::flowchart::types::FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &super::super::FlowchartNodeLabelState<'_>,
    details: &mut crate::svg::parity::flowchart::types::FlowchartRenderDetails,
) -> bool {
    // Port of Mermaid `icon.ts` (`icon-shape default`).
    if let Some(icon_name) = common.node_icon.filter(|s| !s.trim().is_empty()) {
        let label_text_plain =
            flowchart_label_plain_text(label.text, label.label_type, ctx.node_html_labels);
        let has_label = !label_text_plain.trim().is_empty();
        let label_padding = if has_label { 8.0 } else { 0.0 };
        let top_label = common.node_pos == Some("t");

        let asset_h = common.node_asset_height.unwrap_or(48.0).max(1.0);
        let asset_w = common.node_asset_width.unwrap_or(48.0).max(1.0);
        let icon_size = asset_h.max(asset_w);

        let height = icon_size;
        let width = icon_size;
        let x = -width / 2.0;
        let y = -height / 2.0;

        let node_font_style = crate::flowchart::flowchart_effective_font_style_for_node_classes(
            ctx.class_defs,
            common.node_classes,
            common.node_styles,
        );
        let mut metrics = crate::flowchart::flowchart_label_metrics_for_layout(
            crate::flowchart::FlowchartLabelMetricsRequest {
                measurer: ctx.measurer,
                raw_label: label.text,
                label_type: label.label_type,
                style: if ctx.node_wrap_mode == crate::text::WrapMode::HtmlLike {
                    &ctx.html_label_text_style
                } else {
                    &ctx.text_style
                },
                max_width_px: Some(ctx.wrapping_width),
                wrap_mode: ctx.node_wrap_mode,
                config: ctx.config,
                math_renderer: ctx.math_renderer,
                preserve_string_whitespace_height: ctx.node_html_labels && ctx.edge_html_labels,
                whole_label_font_style: node_font_style.as_deref(),
            },
        );
        if !has_label {
            metrics.width = 0.0;
            metrics.height = 0.0;
        }

        // Mermaid's `labelHelper(...)` wraps icon labels in `.labelBkg` (2px padding).
        let label_bbox_w = metrics.width + if has_label { 4.0 } else { 0.0 };
        let label_bbox_h = metrics.height + if has_label { 4.0 } else { 0.0 };

        let outer_w = width.max(label_bbox_w);
        let outer_h = height + label_bbox_h + label_padding;

        let icon_dy = if top_label {
            label_bbox_h / 2.0 + label_padding / 2.0
        } else {
            -label_bbox_h / 2.0 - label_padding / 2.0
        };

        // Icon border/background (Mermaid uses RoughJS `rc.rectangle(...)` with stroke/fill "none").
        let icon_path = format!(
            "M{} {} L{} {} L{} {} L{} {}",
            fmt(x),
            fmt(y),
            fmt(x + width),
            fmt(y),
            fmt(x + width),
            fmt(y + height),
            fmt(x),
            fmt(y + height)
        );
        let _ = write!(out, r#"<g transform="translate(0,{})">"#, fmt(icon_dy));
        let _ = write!(
            out,
            r#"<path d="{}" stroke="none" stroke-width="0" fill="none"/>"#,
            escape_attr(&icon_path),
        );
        out.push_str("</g>");

        let outer_x0 = -outer_w / 2.0;
        let outer_y0 = -outer_h / 2.0;
        let outer_path = format!(
            "M{} {} L{} {} L{} {} L{} {}",
            fmt(outer_x0),
            fmt(outer_y0),
            fmt(outer_x0 + outer_w),
            fmt(outer_y0),
            fmt(outer_x0 + outer_w),
            fmt(outer_y0 + outer_h),
            fmt(outer_x0),
            fmt(outer_y0 + outer_h)
        );

        let label_html =
            super::super::helpers::timed_node_label_html(common.timing_enabled, details, || {
                flowchart_label_html(label.text, label.label_type, ctx.config, ctx.math_renderer)
            });
        let label_y = if top_label {
            -outer_h / 2.0
        } else {
            outer_h / 2.0 - label_bbox_h
        };
        let _ = write!(
            out,
            r#"<g class="label" style="" transform="translate({},{})"><rect/><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" style="display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {}px; text-align: center;"><span class="{}">{}</span></div></foreignObject></g>"#,
            fmt(-label_bbox_w / 2.0),
            fmt(label_y),
            fmt(label_bbox_w),
            fmt(label_bbox_h),
            fmt(ctx.wrapping_width),
            super::super::helpers::flowchart_node_label_span_class(label.label_type),
            label_html
        );

        // Outer bbox helper node (transparent fill, no stroke) — emitted after the label group.
        let _ = write!(
            out,
            r#"<g><path d="{}" stroke="none" stroke-width="0" fill="transparent"/></g>"#,
            escape_attr(&outer_path)
        );

        let icon_tx = -icon_size / 2.0;
        let icon_ty = icon_dy - icon_size / 2.0;
        let icon_svg = super::super::helpers::icon_svg_or_placeholder(ctx, icon_name, icon_size);
        let _ = write!(
            out,
            r#"<g transform="translate({},{})" style="color: {};"><g>{}</g></g>"#,
            fmt(icon_tx),
            fmt(icon_ty),
            escape_attr(common.stroke_color),
            icon_svg,
        );

        out.push_str("</g>");
        if common.wrapped_in_a {
            out.push_str("</a>");
        }
        return true;
    }

    false
}

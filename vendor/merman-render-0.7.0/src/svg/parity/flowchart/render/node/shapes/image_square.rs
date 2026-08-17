//! Flowchart v2 image square shape.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::types::{FlowchartRenderCtx, FlowchartRenderDetails};
use crate::svg::parity::flowchart::{flowchart_label_html, flowchart_label_plain_text};
use crate::svg::parity::{escape_xml_display, fmt_display};

use super::super::roughjs::roughjs_stroke_path_for_svg_path;

pub(in crate::svg::parity::flowchart::render::node) fn try_render_image_square(
    out: &mut String,
    ctx: &FlowchartRenderCtx<'_>,
    common: &super::super::FlowchartNodeRenderCommon<'_>,
    label: &super::super::FlowchartNodeLabelState<'_>,
    details: &mut FlowchartRenderDetails,
) -> bool {
    // Port of Mermaid `imageSquare.ts` (`image-shape default`).
    if let Some(img_href) = common.node_img.filter(|s| !s.trim().is_empty()) {
        let label_text_plain =
            flowchart_label_plain_text(label.text, label.label_type, ctx.node_html_labels);
        let has_label = !label_text_plain.trim().is_empty();
        let label_padding = if has_label { 8.0 } else { 0.0 };
        let top_label = common.node_pos == Some("t");

        let assumed_aspect_ratio = 1.0f64;
        let asset_h = common.node_asset_height.unwrap_or(60.0).max(1.0);
        let asset_w = common.node_asset_width.unwrap_or(asset_h).max(1.0);
        let aspect_ratio = if asset_h > 0.0 {
            asset_w / asset_h
        } else {
            assumed_aspect_ratio
        };

        let default_width = ctx.wrapping_width.max(0.0);
        let image_raw_width = asset_w.max(if has_label { default_width } else { 0.0 });

        let constraint_on = common.node_constraint == Some("on");
        let image_width = if constraint_on && common.node_asset_height.is_some() {
            asset_h * aspect_ratio
        } else {
            image_raw_width
        };
        let image_height = if constraint_on {
            if aspect_ratio != 0.0 {
                image_width / aspect_ratio
            } else {
                asset_h
            }
        } else {
            asset_h
        };

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

        // Mermaid's `labelHelper(...)` wraps image labels in `.labelBkg`; the flowchart
        // stylesheet adds 2px padding to the nested `<p>`, so DOM `getBBox()` includes +4px.
        let label_bbox_w = metrics.width + if has_label { 4.0 } else { 0.0 };
        let label_bbox_h = metrics.height + if has_label { 4.0 } else { 0.0 };

        let outer_w = image_width.max(label_bbox_w);
        let outer_h = image_height + label_bbox_h + label_padding;

        let x0 = -image_width / 2.0;
        let y0 = -image_height / 2.0;
        // Mermaid `imageSquare` fills with a straight rect (not rough).
        let rect_fill_path = format!(
            "M{} {} L{} {} L{} {} L{} {}",
            fmt_display(x0),
            fmt_display(y0),
            fmt_display(x0 + image_width),
            fmt_display(y0),
            fmt_display(x0 + image_width),
            fmt_display(y0 + image_height),
            fmt_display(x0),
            fmt_display(y0 + image_height)
        );
        // Stroke uses RoughJS and must be a closed path so the left edge is included.
        let rect_stroke_path = format!(
            "M{} {} L{} {} L{} {} L{} {} L{} {}",
            fmt_display(x0),
            fmt_display(y0),
            fmt_display(x0 + image_width),
            fmt_display(y0),
            fmt_display(x0 + image_width),
            fmt_display(y0 + image_height),
            fmt_display(x0),
            fmt_display(y0 + image_height),
            fmt_display(x0),
            fmt_display(y0)
        );

        let icon_dy = if top_label {
            label_bbox_h / 2.0 + label_padding / 2.0
        } else {
            -label_bbox_h / 2.0 - label_padding / 2.0
        };
        let _ = write!(
            out,
            r#"<g transform="translate(0,{})">"#,
            fmt_display(icon_dy)
        );
        let _ = write!(
            out,
            r#"<path d="{}" stroke="none" stroke-width="0" fill="{}"/>"#,
            escape_xml_display(&rect_fill_path),
            escape_xml_display(common.fill_color)
        );
        if let Some(stroke_d) =
            super::super::helpers::timed_node_roughjs(common.timing_enabled, details, || {
                roughjs_stroke_path_for_svg_path(
                    &rect_stroke_path,
                    common.stroke_color,
                    common.stroke_width,
                    common.stroke_dasharray,
                    common.hand_drawn_seed,
                )
            })
        {
            let _ = write!(
                out,
                r#"<path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="{}"/>"#,
                escape_xml_display(&stroke_d),
                escape_xml_display(common.stroke_color),
                fmt_display(common.stroke_width as f64),
                escape_xml_display(common.stroke_dasharray)
            );
        }
        out.push_str("</g>");

        // Label group uses a background class in Mermaid's image/icon helpers.
        let label_html =
            super::super::helpers::timed_node_label_html(common.timing_enabled, details, || {
                flowchart_label_html(label.text, label.label_type, ctx.config, ctx.math_renderer)
            });
        let label_dy = if top_label {
            -image_height / 2.0 - label_bbox_h / 2.0 - label_padding / 2.0
        } else {
            image_height / 2.0 - label_bbox_h / 2.0 + label_padding / 2.0
        };
        let _ = write!(
            out,
            concat!(
                r#"<g class="label" style="" transform="translate({},{})">"#,
                r#"<rect/>"#,
                r#"<foreignObject width="{}" height="{}">"#,
                r#"<div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg" "#,
                r#"style="display: table-cell; white-space: nowrap; line-height: 1.5; "#,
                r#"max-width: {}px; text-align: center;"><span class="{}">{}</span></div>"#,
                r#"</foreignObject></g>"#
            ),
            fmt_display(-label_bbox_w / 2.0),
            fmt_display(label_dy),
            fmt_display(label_bbox_w),
            fmt_display(label_bbox_h),
            fmt_display(ctx.wrapping_width),
            super::super::helpers::flowchart_node_label_span_class(label.label_type),
            label_html
        );

        let outer_x0 = -outer_w / 2.0;
        let outer_y0 = -outer_h / 2.0;
        let outer_path = format!(
            "M{} {} L{} {} L{} {} L{} {}",
            outer_x0,
            outer_y0,
            outer_x0 + outer_w,
            outer_y0,
            outer_x0 + outer_w,
            outer_y0 + outer_h,
            outer_x0,
            outer_y0 + outer_h
        );
        let _ = write!(
            out,
            r#"<g><path d="{}" stroke="none" stroke-width="0" fill="none"/></g>"#,
            escape_xml_display(&outer_path)
        );

        let img_translate_y = if top_label {
            outer_h / 2.0 - image_height
        } else {
            -outer_h / 2.0
        };
        let _ = write!(
            out,
            r#"<image href="{}" width="{}" height="{}" preserveAspectRatio="none" transform="translate({},{})"/>"#,
            escape_xml_display(img_href),
            fmt_display(image_width),
            fmt_display(image_height),
            fmt_display(-image_width / 2.0),
            fmt_display(img_translate_y)
        );

        out.push_str("</g>");
        if common.wrapped_in_a {
            out.push_str("</a>");
        }
        return true;
    } else {
        // Fall back to a normal node if the image URL is missing.
        let w = common.layout_node.width.max(1.0);
        let h = common.layout_node.height.max(1.0);
        let _ = write!(
            out,
            r#"<rect class="basic label-container" style="{}" x="{}" y="{}" width="{}" height="{}"/>"#,
            escape_xml_display(common.style),
            fmt_display(-w / 2.0),
            fmt_display(-h / 2.0),
            fmt_display(w),
            fmt_display(h)
        );
        // Keep default label rendering.
    }

    false
}

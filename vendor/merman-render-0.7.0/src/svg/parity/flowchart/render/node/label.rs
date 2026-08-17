//! Flowchart node label renderer.

use std::fmt::Write as _;

use crate::svg::parity::flowchart::label::{flowchart_label_html, flowchart_label_plain_text};
use crate::svg::parity::flowchart::style::FlowchartCompiledStyles;
use crate::svg::parity::flowchart::types::{FlowchartRenderCtx, FlowchartRenderDetails};
use crate::svg::parity::flowchart::util::{OptionalStyleXmlAttr, flowchart_html_contains_img_tag};
use crate::svg::parity::flowchart::write_flowchart_svg_text;
use crate::svg::parity::flowchart::write_flowchart_svg_text_markdown_wrapped;
use crate::svg::parity::{escape_xml_display, fmt_display};

use super::super::root::flowchart_wrap_svg_text_lines;

pub(in crate::svg::parity::flowchart::render::node) fn render_flowchart_node_label(
    out: &mut String,
    ctx: &FlowchartRenderCtx<'_>,
    common: &super::FlowchartNodeRenderCommon<'_>,
    label: &super::FlowchartNodeLabelState<'_>,
    compiled_styles: &FlowchartCompiledStyles,
    details: &mut FlowchartRenderDetails,
) {
    let label_text_plain =
        flowchart_label_plain_text(label.text, label.label_type, ctx.node_html_labels);
    let label_base_style = if ctx.node_wrap_mode == crate::text::WrapMode::HtmlLike {
        &ctx.html_label_text_style
    } else {
        &ctx.text_style
    };
    let node_text_style = crate::flowchart::flowchart_effective_text_style_for_node_classes(
        label_base_style,
        ctx.class_defs,
        common.node_classes,
        common.node_styles,
    );
    let node_font_style = crate::flowchart::flowchart_effective_font_style_for_node_classes(
        ctx.class_defs,
        common.node_classes,
        common.node_styles,
    );
    let is_markdown_label = label.label_type == "markdown";
    let has_literal_backticks = !is_markdown_label && label.text.contains('`');
    let has_markdown_marker = label.text.contains("**")
        || label.text.contains("__")
        || label.text.contains('*')
        || label.text.contains('_');
    let renders_markdown_like =
        is_markdown_label || (!has_literal_backticks && has_markdown_marker);
    let mut label_dy = label.dy;
    if !ctx.node_html_labels
        && renders_markdown_like
        && crate::text::mermaid_markdown_to_lines(label.text, true).len() > 1
        && matches!(
            common.shape,
            "doc"
                | "document"
                | "lin-cyl"
                | "disk"
                | "lined-cylinder"
                | "tag-doc"
                | "tagged-document"
                | "docs"
                | "documents"
                | "st-doc"
                | "stacked-document"
                | "div-rect"
                | "div-proc"
                | "divided-rectangle"
                | "divided-process"
                | "win-pane"
                | "internal-storage"
                | "window-pane"
        )
    {
        // Mermaid shape renderers override `labelHelper(...)`'s default centering using
        // `-bbox.y`. Chromium reports these wrapped SVG markdown labels with a small positive
        // `getBBox().y`, so model that render-time offset here instead of baking literal `-1`s
        // into individual shapes.
        label_dy -= crate::text::svg_create_text_bbox_y_offset_px(&node_text_style);
    }
    let mut metrics = if let (Some(w), Some(h)) = (
        common.layout_node.label_width,
        common.layout_node.label_height,
    ) {
        // Layout already had to measure labels to compute node sizes. Carry those metrics forward so
        // render does not repeat expensive HTML/markdown measurement work.
        crate::text::TextMetrics {
            width: w,
            height: h,
            line_count: 0,
        }
    } else {
        let mut metrics = crate::flowchart::flowchart_label_metrics_for_layout(
            crate::flowchart::FlowchartLabelMetricsRequest {
                measurer: ctx.measurer,
                raw_label: label.text,
                label_type: label.label_type,
                style: &node_text_style,
                max_width_px: Some(ctx.wrapping_width),
                wrap_mode: ctx.node_wrap_mode,
                config: ctx.config,
                math_renderer: ctx.math_renderer,
                preserve_string_whitespace_height: ctx.node_html_labels && ctx.edge_html_labels,
                whole_label_font_style: node_font_style.as_deref(),
            },
        );
        let span_css_height_parity = crate::flowchart::flowchart_node_has_span_css_height_parity(
            ctx.class_defs,
            common.node_classes,
        );
        if ctx.node_html_labels && ctx.edge_html_labels && span_css_height_parity {
            crate::text::flowchart_apply_mermaid_styled_node_height_parity(
                &mut metrics,
                &node_text_style,
            );
        }
        metrics
    };
    let label_has_visual_content = flowchart_html_contains_img_tag(label.text)
        || (label.label_type == "markdown" && label.text.contains("!["));
    if label_text_plain.trim().is_empty() && !label_has_visual_content {
        metrics.width = 0.0;
        metrics.height = 0.0;
    }
    let label_group_class = if common.shape == "note" {
        "label noteLabel"
    } else {
        "label"
    };
    if !ctx.node_html_labels {
        let _ = write!(
            out,
            r#"<g class="{}" style="{}" transform="translate({},{})"><rect/><g><rect class="background" style="stroke: none"/>"#,
            label_group_class,
            escape_xml_display(&compiled_styles.label_style),
            fmt_display(label.dx),
            fmt_display(-metrics.height / 2.0 + label_dy)
        );
        if label.label_type == "markdown" {
            write_flowchart_svg_text_markdown_wrapped(
                out,
                label.text,
                true,
                ctx.measurer,
                &node_text_style,
                Some(ctx.wrapping_width),
            );
        } else {
            let wrapped = flowchart_wrap_svg_text_lines(
                ctx.measurer,
                &label_text_plain,
                &node_text_style,
                Some(ctx.wrapping_width),
                true,
            )
            .join("\n");
            write_flowchart_svg_text(out, &wrapped, true);
        }
        out.push_str("</g></g></g>");
    } else {
        let label_html =
            super::helpers::timed_node_label_html(common.timing_enabled, details, || {
                flowchart_label_html(label.text, label.label_type, ctx.config, ctx.math_renderer)
            });
        let span_style_attr = OptionalStyleXmlAttr(compiled_styles.label_style.as_str());
        let is_math_html_label = ctx.node_wrap_mode == crate::text::WrapMode::HtmlLike
            && label.text.contains("$$")
            && ctx.math_renderer.is_some();

        let needs_wrap = if ctx.node_wrap_mode == crate::text::WrapMode::HtmlLike {
            if is_math_html_label {
                metrics.width >= ctx.wrapping_width - 0.01
            } else {
                let has_inline_style_tags =
                    ctx.node_html_labels && label.label_type != "markdown" && {
                        let lower = label_html.to_ascii_lowercase();
                        crate::text::flowchart_html_has_inline_style_tags(&lower)
                    };

                let raw = if label.label_type == "markdown" {
                    crate::text::measure_markdown_with_flowchart_bold_deltas(
                        ctx.measurer,
                        label.text,
                        &node_text_style,
                        None,
                        ctx.node_wrap_mode,
                    )
                    .width
                } else if has_inline_style_tags {
                    crate::text::measure_html_with_flowchart_bold_deltas(
                        ctx.measurer,
                        &label_html,
                        &node_text_style,
                        None,
                        ctx.node_wrap_mode,
                    )
                    .width
                } else {
                    ctx.measurer
                        .measure_wrapped(
                            &label_text_plain,
                            &node_text_style,
                            None,
                            ctx.node_wrap_mode,
                        )
                        .width
                };
                raw > ctx.wrapping_width
            }
        } else {
            false
        };

        let mut div_style = crate::svg::parity::flowchart::style::flowchart_label_div_style_prefix(
            compiled_styles,
            true,
        );
        if needs_wrap {
            let _ = write!(
                &mut div_style,
                "display: table; white-space: break-spaces; line-height: 1.5; max-width: {mw}px; text-align: center; width: {mw}px;",
                mw = fmt_display(ctx.wrapping_width)
            );
        } else {
            let _ = write!(
                &mut div_style,
                "display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {mw}px; text-align: center;",
                mw = fmt_display(ctx.wrapping_width)
            );
        }
        let _ = write!(
            out,
            r#"<g class="{}" style="{}" transform="translate({},{})"><rect/><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="{}"><span class="{}"{}>{}</span></div></foreignObject></g></g>"#,
            label_group_class,
            escape_xml_display(&compiled_styles.label_style),
            fmt_display(-metrics.width / 2.0 + label.dx),
            fmt_display(-metrics.height / 2.0 + label_dy),
            fmt_display(metrics.width),
            fmt_display(metrics.height),
            escape_xml_display(&div_style),
            super::helpers::flowchart_node_label_span_class(label.label_type),
            span_style_attr,
            label_html
        );
    }
    if common.wrapped_in_a {
        out.push_str("</a>");
    }
}

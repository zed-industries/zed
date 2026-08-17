//! Node-level helpers (link sanitization, class building, placeholders).

use crate::svg::parity::flowchart::types::{FlowchartRenderCtx, FlowchartRenderDetails};
use crate::svg::parity::util::escape_attr_display;
use crate::svg::parity::{escape_xml_display, escape_xml_into, fmt_display};
use std::fmt::Write as _;

pub(in crate::svg::parity::flowchart::render::node) fn icon_svg_or_placeholder(
    ctx: &FlowchartRenderCtx<'_>,
    icon_name: &str,
    icon_size: f64,
) -> String {
    ctx.icon_registry
        .and_then(|registry| registry.svg_for(icon_name, icon_size, icon_size, None, None))
        .unwrap_or_else(|| placeholder_icon_svg(icon_size))
}

fn placeholder_icon_svg(icon_size: f64) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{s}" height="{s}" viewBox="0 0 80 80"><g><rect width="80" height="80" style="fill: #087ebf; stroke-width: 0px;"/><text transform="translate(21.16 64.67)" style="fill: #fff; font-family: ArialMT, Arial; font-size: 67.75px;"><tspan x="0" y="0">?</tspan></text></g></svg>"#,
        s = fmt_display(icon_size)
    )
}

fn is_self_loop_label_node_id(id: &str) -> bool {
    let mut parts = id.split("---");
    let Some(a) = parts.next() else {
        return false;
    };
    let Some(b) = parts.next() else {
        return false;
    };
    let Some(n) = parts.next() else {
        return false;
    };
    parts.next().is_none() && a == b && (n == "1" || n == "2")
}

pub(super) fn try_render_self_loop_label_placeholder(
    out: &mut String,
    node_id: &str,
    x: f64,
    y: f64,
) -> bool {
    if !is_self_loop_label_node_id(node_id) {
        return false;
    }

    let _ = write!(
        out,
        r#"<g class="label edgeLabel" id="{}" transform="translate({},{})"><rect width="0.1" height="0.1"/><g class="label" style="" transform="translate(0,0)"><rect/><foreignObject width="0" height="0"><div xmlns="http://www.w3.org/1999/xhtml" style="display: table-cell; white-space: nowrap; line-height: 1.5; max-width: 10px; text-align: center;"><span class="nodeLabel"></span></div></foreignObject></g></g>"#,
        escape_xml_display(node_id),
        fmt_display(x),
        fmt_display(y)
    );
    true
}

fn href_is_safe_in_strict_mode(href: &str, config: &merman_core::MermaidConfig) -> bool {
    let _config = config;

    let href = href.trim();
    if href.is_empty() {
        return false;
    }

    let lower = href.to_ascii_lowercase();
    if lower.starts_with('#')
        || lower.starts_with("mailto:")
        || lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("//")
        || lower.starts_with('/')
        || lower.starts_with("./")
        || lower.starts_with("../")
    {
        return true;
    }

    // Mermaid's SVG output does not include `xlink:href` for unknown schemes (e.g. `notes://...`
    // or `javascript:...`). Treat any explicit scheme as unsafe unless it matched the allow-list
    // above.
    let scheme_end = lower.find(['/', '?', '#']).unwrap_or(lower.len());
    !lower[..scheme_end].contains(':')
}

fn write_class_attr(out: &mut String, base: &str, classes: &[String]) {
    escape_xml_into(out, base);
    for c in classes {
        let t = c.trim();
        if t.is_empty() {
            continue;
        }
        out.push(' ');
        escape_xml_into(out, t);
    }
}

pub(super) struct NodeWrapperAttrs<'a> {
    pub(super) diagram_id: &'a str,
    pub(super) node_id: &'a str,
    pub(super) dom_idx: Option<usize>,
    pub(super) class_attr_base: &'a str,
    pub(super) node_classes: &'a [String],
    pub(super) wrapped_in_a: bool,
    pub(super) href: Option<&'a str>,
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) tooltip_enabled: bool,
    pub(super) tooltip: &'a str,
    pub(super) look: &'a str,
}

pub(super) fn open_node_wrapper(out: &mut String, attrs: NodeWrapperAttrs<'_>) {
    let NodeWrapperAttrs {
        diagram_id,
        node_id,
        dom_idx,
        class_attr_base,
        node_classes,
        wrapped_in_a,
        href,
        x,
        y,
        tooltip_enabled,
        tooltip,
        look,
    } = attrs;

    if wrapped_in_a {
        if let Some(href) = href {
            out.push_str(r#"<a xlink:href=""#);
            escape_xml_into(out, href);
            out.push_str(r#"" transform="translate("#);
            crate::svg::parity::util::fmt_into(out, x);
            out.push(',');
            crate::svg::parity::util::fmt_into(out, y);
            out.push_str(r#")" data-look=""#);
            escape_xml_into(out, look);
            out.push_str(r#"">"#);
        } else {
            out.push_str(r#"<a transform="translate("#);
            crate::svg::parity::util::fmt_into(out, x);
            out.push(',');
            crate::svg::parity::util::fmt_into(out, y);
            out.push_str(r#")" data-look=""#);
            escape_xml_into(out, look);
            out.push_str(r#"">"#);
        }
        out.push_str(r#"<g class=""#);
        write_class_attr(out, class_attr_base, node_classes);
        if let Some(dom_idx) = dom_idx {
            out.push_str(r#"" id=""#);
            escape_xml_into(out, diagram_id);
            out.push_str(r#"-flowchart-"#);
            escape_xml_into(out, node_id);
            let _ = write!(out, "-{dom_idx}\"");
        } else {
            out.push_str(r#"" id=""#);
            escape_xml_into(out, diagram_id);
            out.push('-');
            escape_xml_into(out, node_id);
            out.push('"');
        }
    } else {
        out.push_str(r#"<g class=""#);
        write_class_attr(out, class_attr_base, node_classes);
        if let Some(dom_idx) = dom_idx {
            out.push_str(r#"" id=""#);
            escape_xml_into(out, diagram_id);
            out.push_str(r#"-flowchart-"#);
            escape_xml_into(out, node_id);
            let _ = write!(out, r#"-{dom_idx}" transform="translate("#);
            crate::svg::parity::util::fmt_into(out, x);
            out.push(',');
            crate::svg::parity::util::fmt_into(out, y);
            out.push_str(r#")" data-look=""#);
            escape_xml_into(out, look);
            out.push('"');
        } else {
            out.push_str(r#"" id=""#);
            escape_xml_into(out, diagram_id);
            out.push('-');
            escape_xml_into(out, node_id);
            out.push_str(r#"" transform="translate("#);
            crate::svg::parity::util::fmt_into(out, x);
            out.push(',');
            crate::svg::parity::util::fmt_into(out, y);
            out.push_str(r#")" data-look=""#);
            escape_xml_into(out, look);
            out.push('"');
        }
    }
    if tooltip_enabled {
        let _ = write!(out, r#" title="{}""#, escape_attr_display(tooltip));
    }
    out.push('>');
}

pub(super) fn flowchart_node_label_span_class(label_type: &str) -> &'static str {
    if label_type == "markdown" {
        "nodeLabel markdown-node-label"
    } else {
        "nodeLabel"
    }
}

pub(super) fn timed_node_roughjs<T>(
    timing_enabled: bool,
    details: &mut FlowchartRenderDetails,
    f: impl FnOnce() -> T,
) -> T {
    if timing_enabled {
        details.node_roughjs_calls += 1;
        let start = web_time::Instant::now();
        let out = f();
        details.node_roughjs += start.elapsed();
        out
    } else {
        f()
    }
}

pub(super) fn timed_node_label_html<T>(
    timing_enabled: bool,
    details: &mut FlowchartRenderDetails,
    f: impl FnOnce() -> T,
) -> T {
    if timing_enabled {
        details.node_label_html_calls += 1;
        let start = web_time::Instant::now();
        let out = f();
        details.node_label_html += start.elapsed();
        out
    } else {
        f()
    }
}

pub(super) struct ResolvedNodeRenderInfo<'a> {
    pub(super) dom_idx: Option<usize>,
    pub(super) class_attr_base: &'static str,
    pub(super) wrapped_in_a: bool,
    pub(super) href: Option<&'a str>,
    pub(super) label_text: &'a str,
    pub(super) label_text_is_node_id: bool,
    pub(super) label_type: &'a str,
    pub(super) shape: &'a str,
    pub(super) node_icon: Option<&'a str>,
    pub(super) node_img: Option<&'a str>,
    pub(super) node_pos: Option<&'a str>,
    pub(super) node_constraint: Option<&'a str>,
    pub(super) node_asset_width: Option<f64>,
    pub(super) node_asset_height: Option<f64>,
    pub(super) node_styles: &'a [String],
    pub(super) node_classes: &'a [String],
}

pub(super) fn resolve_node_render_info<'a>(
    ctx: &'a FlowchartRenderCtx<'a>,
    node_id: &str,
) -> Option<ResolvedNodeRenderInfo<'a>> {
    if let Some(sg) = ctx.subgraphs_by_id.get(node_id) {
        if sg.nodes.is_empty() {
            return Some(ResolvedNodeRenderInfo {
                dom_idx: None,
                class_attr_base: "node",
                wrapped_in_a: false,
                href: None,
                label_text: sg.title.as_str(),
                label_text_is_node_id: false,
                label_type: sg.label_type.as_deref().unwrap_or("text"),
                shape: "squareRect",
                node_icon: None,
                node_img: None,
                node_pos: None,
                node_constraint: None,
                node_asset_width: None,
                node_asset_height: None,
                node_styles: &sg.styles,
                node_classes: &sg.classes,
            });
        }
    }

    if let Some(node) = ctx.nodes_by_id.get(node_id) {
        let dom_idx = Some(ctx.node_dom_index.get(node_id).copied().unwrap_or(0));
        let shape = node.layout_shape.as_deref().unwrap_or("squareRect");

        // Mermaid flowchart-v2 uses a distinct wrapper class for icon/image nodes.
        let class_attr_base = if shape == "imageSquare" {
            "image-shape default"
        } else if shape == "icon" || shape.starts_with("icon") {
            "icon-shape default"
        } else {
            "node default"
        };

        let link = node
            .link
            .as_deref()
            .map(|u| u.trim())
            .filter(|u| !u.is_empty());
        let link_present = link.is_some();
        // Mermaid sanitizes unsafe URLs (e.g. `javascript:` in strict mode) into
        // `about:blank`, but the resulting SVG `<a>` carries no `xlink:href` attribute.
        let href = link
            .filter(|u| *u != "about:blank")
            .filter(|u| href_is_safe_in_strict_mode(u, ctx.config));
        // Mermaid wraps nodes in `<a>` only when a link is present. Callback-based
        // interactions (`click A someFn`) still mark the node as clickable, but do not
        // emit an anchor element in the SVG.
        let wrapped_in_a = link_present;

        let (label_text, label_text_is_node_id) = if let Some(v) = node.label.as_deref() {
            (v, false)
        } else {
            ("", true)
        };

        Some(ResolvedNodeRenderInfo {
            dom_idx,
            class_attr_base,
            wrapped_in_a,
            href,
            label_text,
            label_text_is_node_id,
            label_type: node.label_type.as_deref().unwrap_or("text"),
            shape,
            node_icon: node.icon.as_deref(),
            node_img: node.img.as_deref(),
            node_pos: node.pos.as_deref(),
            node_constraint: node.constraint.as_deref(),
            node_asset_width: node.asset_width,
            node_asset_height: node.asset_height,
            node_styles: &node.styles,
            node_classes: &node.classes,
        })
    } else {
        None
    }
}

pub(in crate::svg::parity::flowchart::render::node) fn compute_node_label_metrics(
    ctx: &FlowchartRenderCtx<'_>,
    layout_node: Option<&crate::model::LayoutNode>,
    label_text: &str,
    label_type: &str,
    node_classes: &[String],
    node_styles: &[String],
) -> crate::text::TextMetrics {
    // Shared across many Flowchart v2 shape renderers.
    //
    // Keep behavior identical to the inlined implementations to preserve Mermaid SVG parity.
    let label_text_plain = crate::svg::parity::flowchart::flowchart_label_plain_text(
        label_text,
        label_type,
        ctx.node_html_labels,
    );
    let label_base_style = if ctx.node_wrap_mode == crate::text::WrapMode::HtmlLike {
        &ctx.html_label_text_style
    } else {
        &ctx.text_style
    };
    let node_text_style = crate::flowchart::flowchart_effective_text_style_for_node_classes(
        label_base_style,
        ctx.class_defs,
        node_classes,
        node_styles,
    );
    let node_font_style = crate::flowchart::flowchart_effective_font_style_for_node_classes(
        ctx.class_defs,
        node_classes,
        node_styles,
    );
    let mut metrics = if let Some(layout_node) = layout_node {
        if let (Some(width), Some(height)) = (layout_node.label_width, layout_node.label_height) {
            crate::text::TextMetrics {
                width,
                height,
                line_count: 0,
            }
        } else {
            let mut metrics = crate::flowchart::flowchart_label_metrics_for_layout(
                crate::flowchart::FlowchartLabelMetricsRequest {
                    measurer: ctx.measurer,
                    raw_label: label_text,
                    label_type,
                    style: &node_text_style,
                    max_width_px: Some(ctx.wrapping_width),
                    wrap_mode: ctx.node_wrap_mode,
                    config: ctx.config,
                    math_renderer: ctx.math_renderer,
                    preserve_string_whitespace_height: ctx.node_html_labels && ctx.edge_html_labels,
                    whole_label_font_style: node_font_style.as_deref(),
                },
            );

            let span_css_height_parity =
                crate::flowchart::flowchart_node_has_span_css_height_parity(
                    ctx.class_defs,
                    node_classes,
                );
            if ctx.node_html_labels && ctx.edge_html_labels && span_css_height_parity {
                crate::text::flowchart_apply_mermaid_styled_node_height_parity(
                    &mut metrics,
                    &node_text_style,
                );
            }
            metrics
        }
    } else {
        let mut metrics = crate::flowchart::flowchart_label_metrics_for_layout(
            crate::flowchart::FlowchartLabelMetricsRequest {
                measurer: ctx.measurer,
                raw_label: label_text,
                label_type,
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
            node_classes,
        );
        if ctx.node_html_labels && ctx.edge_html_labels && span_css_height_parity {
            crate::text::flowchart_apply_mermaid_styled_node_height_parity(
                &mut metrics,
                &node_text_style,
            );
        }
        metrics
    };

    let label_has_visual_content =
        super::super::super::util::flowchart_html_contains_img_tag(label_text)
            || (label_type == "markdown" && label_text.contains("!["));
    if label_text_plain.trim().is_empty() && !label_has_visual_content {
        metrics.width = 0.0;
        metrics.height = 0.0;
    }

    metrics
}

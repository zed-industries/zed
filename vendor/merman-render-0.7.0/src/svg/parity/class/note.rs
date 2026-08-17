use crate::entities::decode_entities_minimal_cow;
use crate::model::{Bounds, LayoutNode};
use crate::text::{MermaidMarkdownWordType, TextMeasurer, TextStyle, WrapMode};
use std::fmt::Write as _;
use web_time::Duration;

use super::super::{escape_attr_display, escape_xml_into, fmt, theme_color};
use super::ClassSvgNote;
use super::bounds::{include_path_bounds, include_xywh};
use super::label::class_note_html_div_style;
use super::node::ClassNodeRenderPosition;
use super::rough::{class_rough_rect_stroke_path_and_bounds, class_rough_seed};

pub(super) struct ClassNoteRenderContext<'a> {
    pub diagram_id: &'a str,
    pub effective_config: &'a serde_json::Value,
    pub measurer: &'a dyn TextMeasurer,
    pub text_style: &'a TextStyle,
    pub line_height: f64,
    pub use_html_labels: bool,
    pub look: &'a str,
    pub timing_enabled: bool,
}

pub(super) struct ClassNoteRenderState<'a> {
    pub out: &'a mut String,
    pub content_bounds: &'a mut Option<Bounds>,
    pub sanitize_config: &'a mut Option<merman_core::MermaidConfig>,
    pub borrowed_sanitize_config: Option<&'a merman_core::MermaidConfig>,
}

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct ClassNoteRenderStats {
    pub notes_sanitize: Duration,
    pub path_bounds: Duration,
    pub path_bounds_calls: usize,
}

pub(super) fn render_class_note_node(
    state: ClassNoteRenderState<'_>,
    note: &ClassSvgNote,
    layout_node: &LayoutNode,
    position: ClassNodeRenderPosition,
    ctx: &ClassNoteRenderContext<'_>,
) -> ClassNoteRenderStats {
    let out = &mut *state.out;
    let content_bounds = &mut *state.content_bounds;
    let sanitize_config = &mut *state.sanitize_config;
    let borrowed_sanitize_config = state.borrowed_sanitize_config;
    let mut stats = ClassNoteRenderStats::default();

    let note_src = note.text.trim();
    let note_text = decode_entities_minimal_cow(note_src);
    let (label_w_raw, label_h_raw) = if ctx.use_html_labels {
        match (layout_node.label_width, layout_node.label_height) {
            (Some(w), Some(h)) => (w, h),
            _ => {
                let note_html_config = class_note_sanitize_config(
                    borrowed_sanitize_config,
                    sanitize_config,
                    ctx.effective_config,
                );
                let metrics = crate::class::class_html_measure_note_metrics(
                    ctx.measurer,
                    ctx.text_style,
                    note_src,
                    note_html_config,
                );
                (metrics.width, metrics.height)
            }
        }
    } else {
        let mut metrics =
            ctx.measurer
                .measure_wrapped(&note_text, ctx.text_style, None, WrapMode::SvgLike);
        if let Some(width) = crate::class::class_svg_single_line_plain_label_width_px(
            note_text.as_ref(),
            ctx.measurer,
            ctx.text_style,
        ) {
            metrics.width = width;
        }
        (metrics.width, metrics.height)
    };
    let label_w = label_w_raw.max(1.0);
    let label_h = if ctx.use_html_labels {
        label_h_raw.max(ctx.line_height).max(1.0)
    } else {
        label_h_raw.max(1.0)
    };
    let w = layout_node.width.max(1.0);
    let h = layout_node.height.max(1.0);
    let left = -w / 2.0;
    let top = -h / 2.0;
    let label_x = -label_w / 2.0;
    let label_y = if ctx.use_html_labels {
        -label_h / 2.0
    } else {
        -label_h / 2.0 - crate::class::class_svg_create_text_bbox_y_offset_px(ctx.text_style)
    };
    let (note_stroke_d, note_stroke_pb) = class_rough_rect_stroke_path_and_bounds(
        left,
        top,
        w,
        h,
        class_rough_seed(ctx.diagram_id, &note.id),
    );
    include_xywh(
        content_bounds,
        position.node_bounds_tx + left,
        position.node_bounds_ty + top,
        w,
        h,
    );
    include_xywh(
        content_bounds,
        position.node_bounds_tx + label_x,
        position.node_bounds_ty + label_y,
        label_w,
        label_h,
    );
    let path_bounds_start = ctx.timing_enabled.then(web_time::Instant::now);
    include_path_bounds(
        content_bounds,
        &note_stroke_pb,
        position.node_bounds_tx,
        position.node_bounds_ty,
    );
    if let Some(s) = path_bounds_start {
        stats.path_bounds += s.elapsed();
        stats.path_bounds_calls += 1;
    }

    let note_fill = theme_color(ctx.effective_config, "noteBkgColor", "#fff5ad");
    let note_stroke = theme_color(ctx.effective_config, "noteBorderColor", "#aaaa33");
    let note_shape_style = format!("fill:{note_fill} !important;stroke:{note_stroke} !important");
    if ctx.use_html_labels {
        let note_div_style = class_note_html_div_style(label_w, 200);
        let _ = write!(
            out,
            r##"<g class="node undefined" id="{}-{}" data-look="{}" transform="translate({}, {})"><g class="basic label-container outer-path"><path d="M{} {} L{} {} L{} {} L{} {}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="1.3" fill="none" stroke-dasharray="0 0" style="{}"/></g><g class="label noteLabel" style="text-align:left !important;white-space:nowrap !important" transform="translate({}, {})"><rect/><foreignObject width="{}" height="{}"><div style="{}" xmlns="http://www.w3.org/1999/xhtml"><span style="text-align:left !important;white-space:nowrap !important" class="nodeLabel markdown-node-label"><p>"##,
            escape_attr_display(ctx.diagram_id),
            escape_attr_display(&note.id),
            escape_attr_display(ctx.look),
            fmt(position.node_tx),
            fmt(position.node_ty),
            fmt(left),
            fmt(top),
            fmt(left + w),
            fmt(top),
            fmt(left + w),
            fmt(top + h),
            fmt(left),
            fmt(top + h),
            escape_attr_display(&note_fill),
            escape_attr_display(&note_shape_style),
            escape_attr_display(&note_stroke_d),
            escape_attr_display(&note_stroke),
            escape_attr_display(&note_shape_style),
            fmt(label_x),
            fmt(label_y),
            fmt(label_w),
            fmt(label_h),
            escape_attr_display(&note_div_style),
        );
        let sanitize_start = ctx.timing_enabled.then(web_time::Instant::now);
        let note_html_config = class_note_sanitize_config(
            borrowed_sanitize_config,
            sanitize_config,
            ctx.effective_config,
        );
        let note_html = crate::class::class_note_html_fragment(note_src, note_html_config);
        if let Some(s) = sanitize_start {
            stats.notes_sanitize += s.elapsed();
        }
        out.push_str(&note_html);
        out.push_str("</p></span></div></foreignObject></g></g>");
    } else {
        let note_label_style = "text-align:left !important;white-space:nowrap !important";
        let _ = write!(
            out,
            r##"<g class="node undefined" id="{}-{}" data-look="{}" transform="translate({}, {})"><g class="basic label-container outer-path"><path d="M{} {} L{} {} L{} {} L{} {}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="1.3" fill="none" stroke-dasharray="0 0" style="{}"/></g><g class="label noteLabel" style="{}" transform="translate({}, {})"><rect/><g><rect class="background" style="stroke: none"/>"##,
            escape_attr_display(ctx.diagram_id),
            escape_attr_display(&note.id),
            escape_attr_display(ctx.look),
            fmt(position.node_tx),
            fmt(position.node_ty),
            fmt(left),
            fmt(top),
            fmt(left + w),
            fmt(top),
            fmt(left + w),
            fmt(top + h),
            fmt(left),
            fmt(top + h),
            escape_attr_display(&note_fill),
            escape_attr_display(&note_shape_style),
            escape_attr_display(&note_stroke_d),
            escape_attr_display(&note_stroke),
            escape_attr_display(&note_shape_style),
            escape_attr_display(note_label_style),
            fmt(label_x),
            fmt(label_y),
        );
        write_class_svg_text_markdown_with_style(out, note_text.as_ref(), note_label_style);
        out.push_str("</g></g></g>");
    }

    stats
}

fn class_note_sanitize_config<'a>(
    borrowed_sanitize_config: Option<&'a merman_core::MermaidConfig>,
    owned_sanitize_config: &'a mut Option<merman_core::MermaidConfig>,
    effective_config: &serde_json::Value,
) -> &'a merman_core::MermaidConfig {
    if let Some(config) = borrowed_sanitize_config {
        return config;
    }
    owned_sanitize_config
        .get_or_insert_with(|| merman_core::MermaidConfig::from_value(effective_config.clone()))
}

fn write_class_svg_text_markdown_with_style(out: &mut String, markdown: &str, style: &str) {
    let markdown = markdown
        .strip_prefix('`')
        .and_then(|s| s.strip_suffix('`'))
        .unwrap_or(markdown);
    let _ = write!(
        out,
        r#"<text y="-10.1" style="{}">"#,
        escape_attr_display(style)
    );

    let lines = crate::text::mermaid_markdown_to_lines(markdown, true);
    if lines.len() == 1 && lines[0].is_empty() {
        out.push_str(r#"<tspan class="row text-outer-tspan" x="0" y="-0.1em" dy="1.1em"/>"#);
        out.push_str("</text>");
        return;
    }

    for (idx, words) in lines.iter().enumerate() {
        if idx == 0 {
            out.push_str(r#"<tspan class="row text-outer-tspan" x="0" y="-0.1em" dy="1.1em">"#);
        } else {
            let y_em = if idx == 1 {
                "1em".to_string()
            } else {
                format!("{:.1}em", 1.0 + (idx as f64 - 1.0) * 1.1)
            };
            let _ = write!(
                out,
                r#"<tspan class="row text-outer-tspan" x="0" y="{}" dy="1.1em">"#,
                y_em
            );
        }

        for (word_idx, (word, ty)) in words.iter().enumerate() {
            let is_strong = *ty == MermaidMarkdownWordType::Strong;
            let is_em = *ty == MermaidMarkdownWordType::Em;
            let font_style = if is_em { "italic" } else { "normal" };
            let font_weight = if is_strong { "bold" } else { "normal" };
            let _ = write!(
                out,
                r#"<tspan font-style="{}" class="text-inner-tspan" font-weight="{}">"#,
                font_style, font_weight
            );
            if word_idx == 0 {
                escape_xml_into(out, word);
            } else {
                out.push(' ');
                escape_xml_into(out, word);
            }
            out.push_str("</tspan>");
        }

        out.push_str("</tspan>");
    }

    out.push_str("</text>");
}

use super::super::*;

pub(super) struct ClassInlineStyles<'a> {
    pub style_attr: String,
    pub fill: Option<&'a str>,
    pub stroke: Option<&'a str>,
    pub stroke_width: Option<&'a str>,
    pub stroke_dasharray: Option<&'a str>,
}

pub(super) fn render_class_html_label(
    out: &mut String,
    span_class: &str,
    text: &str,
    include_p: bool,
    extra_span_class: Option<&str>,
    span_style: Option<&str>,
) {
    fn is_simple_plain_label(text: &str) -> bool {
        // Fast-path for the common case: no Markdown tokens and no hard/soft line breaks.
        // This avoids pulldown-cmark overhead while producing the same XHTML fragment Mermaid
        // would emit for plain text labels.
        if text.to_ascii_lowercase().contains("<br") {
            return false;
        }
        let bytes = text.as_bytes();
        !bytes.iter().any(|&b| {
            matches!(
                b,
                b'\n' | b'\r' | b'*' | b'_' | b'`' | b'~' | b'|' | b'[' | b']'
            )
        })
    }

    out.push_str(r#"<span class=""#);
    escape_xml_into(out, span_class);
    if let Some(extra) = extra_span_class.map(|s| s.trim()).filter(|s| !s.is_empty()) {
        out.push(' ');
        escape_xml_into(out, extra);
    }
    let span_style = span_style.map(str::trim).unwrap_or("");
    if span_class == "nodeLabel" || !span_style.is_empty() {
        out.push_str(r#"" style=""#);
        super::super::util::escape_attr_into(out, span_style);
        out.push_str(r#"">"#);
    } else {
        out.push_str(r#"">"#);
    }

    if is_simple_plain_label(text) {
        if include_p {
            out.push_str("<p>");
            escape_xml_into(out, text);
            out.push_str("</p>");
        } else {
            escape_xml_into(out, text);
        }
        out.push_str("</span>");
        return;
    }

    let html = crate::text::mermaid_markdown_to_xhtml_label_fragment(text, true);
    if include_p {
        out.push_str(&html);
    } else {
        let inner = html
            .strip_prefix("<p>")
            .and_then(|s| s.strip_suffix("</p>"))
            .unwrap_or(html.as_str());
        out.push_str(inner);
    }
    out.push_str("</span>");
}

pub(super) fn write_class_svg_text_markdown(out: &mut String, markdown: &str, include_style: bool) {
    crate::svg::parity::flowchart::write_flowchart_svg_text_markdown(out, markdown, include_style);
}

pub(super) fn write_class_svg_edge_text(out: &mut String, text: &str, include_style: bool) {
    crate::svg::parity::flowchart::write_flowchart_svg_text_centered(out, text, include_style);
}

pub(super) fn write_class_svg_edge_text_markdown(
    out: &mut String,
    markdown: &str,
    include_style: bool,
) {
    crate::svg::parity::flowchart::write_flowchart_svg_text_markdown_centered(
        out,
        markdown,
        include_style,
    );
}

pub(super) fn class_html_div_style(width: f64, max_width_px: i64) -> String {
    let max_width_px = max_width_px.max(0);
    if width >= max_width_px as f64 - 0.01 {
        format!(
            "display: table; white-space: break-spaces; line-height: 1.5; max-width: {max_width_px}px; text-align: center; width: {max_width_px}px;"
        )
    } else {
        format!(
            "display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {max_width_px}px; text-align: center;"
        )
    }
}

pub(super) fn class_note_html_div_style(width: f64, max_width_px: i64) -> String {
    let max_width_px = max_width_px.max(0);
    if width >= max_width_px as f64 - 0.01 {
        format!(
            "text-align: center; white-space: break-spaces; display: table; line-height: 1.5; max-width: {max_width_px}px; width: {max_width_px}px;"
        )
    } else {
        format!(
            "text-align: center; white-space: nowrap; display: table-cell; line-height: 1.5; max-width: {max_width_px}px;"
        )
    }
}

pub(super) fn class_html_label_max_width_px(width: f64, is_bold: bool) -> i64 {
    width.max(0.0).ceil() as i64 + if is_bold { 51 } else { 50 }
}

pub(super) fn class_html_label_metrics(
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    text: &str,
    max_width_px: i64,
    css_style: &str,
) -> crate::text::TextMetrics {
    let mut metrics = crate::class::class_html_measure_label_metrics(
        measurer,
        style,
        text,
        max_width_px,
        css_style,
    );
    if let Some(width) =
        crate::class::class_html_known_rendered_width_override_px(text, style, false)
    {
        metrics.width = width;
    }
    metrics
}

pub(super) fn class_html_title_metrics(
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    text: &str,
    max_width_px: i64,
) -> crate::text::TextMetrics {
    let markdown = crate::text::DeterministicTextMeasurer::normalized_text_lines(text)
        .into_iter()
        .map(|line| format!("**{line}**"))
        .collect::<Vec<_>>()
        .join("\n");
    crate::text::measure_markdown_with_flowchart_bold_deltas(
        measurer,
        markdown.as_str(),
        style,
        Some(max_width_px.max(1) as f64),
        WrapMode::HtmlLike,
    )
}

pub(super) fn class_svg_label_rect(
    metrics: &crate::text::TextMetrics,
    y_offset: f64,
) -> Option<super::Rect> {
    if !(metrics.width.is_finite() && metrics.height.is_finite()) {
        return None;
    }
    let w = metrics.width.max(0.0);
    let h = metrics.height.max(0.0);
    if w <= 0.0 || h <= 0.0 {
        return None;
    }
    let lines = metrics.line_count.max(1) as f64;
    let y = y_offset - (h / (2.0 * lines));
    Some(super::Rect::from_min_max(0.0, y, w, y + h))
}

pub(super) fn wrap_class_svg_text_like_mermaid(
    text: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    wrap_probe_font_size: f64,
    bold: bool,
) -> String {
    let Some(wrap_width_px) =
        mermaid_class_svg_create_text_width_px(measurer, text, style, wrap_probe_font_size)
    else {
        return text.to_string();
    };

    // Vendored font metrics do not line up with Chromium's SVG `getComputedTextLength()` exactly.
    // Most default-font SVG labels need a small inflation, but Mermaid's Class path can also
    // measure the wrap width with a smaller top-level `fontSize` while rendering the final SVG text
    // with an explicit larger `themeVariables.fontSize` px value. That case needs slack so type
    // suffixes stay on the same outer tspan row as upstream.
    let computed_len_fudge = if bold {
        1.0
    } else if wrap_probe_font_size < style.font_size {
        0.9
    } else if style.font_size >= 20.0 {
        1.0
    } else {
        1.02
    };

    let mut lines: Vec<String> = Vec::new();
    for line in crate::text::DeterministicTextMeasurer::normalized_text_lines(text) {
        let mut tokens = std::collections::VecDeque::from(
            crate::text::DeterministicTextMeasurer::split_line_to_words(&line),
        );
        let mut cur = String::new();

        while let Some(tok) = tokens.pop_front() {
            if cur.is_empty() && tok == " " {
                continue;
            }

            let candidate = format!("{cur}{tok}");
            let candidate_w =
                class_svg_text_computed_length_px(measurer, candidate.trim_end(), style, bold)
                    * computed_len_fudge;
            if candidate_w <= wrap_width_px {
                cur = candidate;
                continue;
            }

            if !cur.trim().is_empty() {
                lines.push(cur.trim_end().to_string());
                cur.clear();
                tokens.push_front(tok);
                continue;
            }

            if tok == " " {
                continue;
            }

            let chars = tok.chars().collect::<Vec<_>>();
            let mut cut = 1usize;
            while cut < chars.len() {
                let head: String = chars[..cut].iter().collect();
                let head_w =
                    class_svg_text_computed_length_px(measurer, head.as_str(), style, bold)
                        * computed_len_fudge;
                if head_w > wrap_width_px {
                    break;
                }
                cut += 1;
            }
            cut = cut.saturating_sub(1).max(1);
            let head: String = chars[..cut].iter().collect();
            let tail: String = chars[cut..].iter().collect();
            lines.push(head);
            if !tail.is_empty() {
                tokens.push_front(tail);
            }
        }

        if !cur.trim().is_empty() {
            lines.push(cur.trim_end().to_string());
        }
    }

    if lines.len() <= 1 {
        text.to_string()
    } else {
        lines.join("\n")
    }
}

pub(super) fn round_to_1_1024_px_ties_to_even(v: f64) -> f64 {
    if !(v.is_finite() && v >= 0.0) {
        return 0.0;
    }
    let x = v * 1024.0;
    let f = x.floor();
    let frac = x - f;
    let i = if frac < 0.5 {
        f
    } else if frac > 0.5 {
        f + 1.0
    } else {
        let fi = f as i64;
        if fi % 2 == 0 { f } else { f + 1.0 }
    };
    let out = i / 1024.0;
    if out == -0.0 { 0.0 } else { out }
}

pub(super) fn bolder_delta_scale_for_svg(font_size: f64) -> f64 {
    let fs = font_size.max(1.0);
    if fs <= 16.0 {
        1.0
    } else if fs >= 24.0 {
        0.6
    } else {
        1.0 - (fs - 16.0) * (0.4 / 8.0)
    }
}

fn mermaid_class_svg_create_text_width_px(
    measurer: &dyn TextMeasurer,
    text: &str,
    style: &TextStyle,
    wrap_probe_font_size: f64,
) -> Option<f64> {
    let wrap_probe_font_size = wrap_probe_font_size.max(1.0);
    // Mermaid `calculateTextWidth(...)` selects between `sans-serif` and the configured font
    // family using `calculateTextDimensions(...)` (it does *not* always take the max width).
    // Replicate that selection logic so SVG-label wrapping matches upstream fixtures.
    #[derive(Clone, Copy)]
    struct Dim {
        width: f64,
        height: f64,
        line_height: f64,
    }
    fn dim_for(measurer: &dyn TextMeasurer, text: &str, style: &TextStyle) -> Dim {
        let width = measurer
            .measure_svg_simple_text_bbox_width_px(text, style)
            .max(0.0)
            .round();
        let height = measurer
            .measure_wrapped(text, style, None, WrapMode::SvgLike)
            .height
            .max(0.0)
            .round();
        Dim {
            width,
            height,
            line_height: height,
        }
    }

    let wrap_probe_style = TextStyle {
        font_family: style
            .font_family
            .clone()
            .or_else(|| Some("Arial".to_string())),
        font_size: wrap_probe_font_size,
        font_weight: None,
    };
    let sans_probe_style = TextStyle {
        font_family: Some("sans-serif".to_string()),
        font_size: wrap_probe_font_size,
        font_weight: None,
    };
    let dims = [
        dim_for(measurer, text, &sans_probe_style),
        dim_for(measurer, text, &wrap_probe_style),
    ];
    let pick_sans = dims[1].height.is_nan()
        || dims[1].width.is_nan()
        || dims[1].line_height.is_nan()
        || (dims[0].height > dims[1].height
            && dims[0].width > dims[1].width
            && dims[0].line_height > dims[1].line_height);
    let w = dims[if pick_sans { 0 } else { 1 }].width + 50.0;
    if w.is_finite() && w > 0.0 {
        Some(w)
    } else {
        None
    }
}

fn class_svg_text_computed_length_px(
    measurer: &dyn TextMeasurer,
    text: &str,
    style: &TextStyle,
    bold: bool,
) -> f64 {
    if bold {
        let bold_style = TextStyle {
            font_family: style.font_family.clone(),
            font_size: style.font_size,
            font_weight: Some("bolder".to_string()),
        };
        measurer.measure_svg_text_computed_length_px(text, &bold_style)
    } else {
        measurer.measure_svg_text_computed_length_px(text, style)
    }
}

pub(super) fn class_apply_inline_styles<'a>(
    node: &'a super::ClassSvgNode,
) -> ClassInlineStyles<'a> {
    let mut style_attr = String::new();
    let mut fill: Option<&str> = None;
    let mut stroke: Option<&str> = None;
    let mut stroke_width: Option<&str> = None;
    let mut stroke_dasharray: Option<&str> = None;

    for raw in &node.styles {
        let trimmed = raw.trim().trim_end_matches(';').trim();
        if trimmed.is_empty() {
            continue;
        }
        if !style_attr.is_empty() {
            style_attr.push(';');
        }
        style_attr.push_str(trimmed);

        let Some((k, v)) = trimmed.split_once(':') else {
            continue;
        };
        let key = k.trim();
        let val = v.trim().trim_end_matches(';').trim();
        if key.eq_ignore_ascii_case("fill") && !val.is_empty() {
            fill = Some(val);
        }
        if key.eq_ignore_ascii_case("stroke") && !val.is_empty() {
            stroke = Some(val);
        }
        if key.eq_ignore_ascii_case("stroke-width") && !val.is_empty() {
            stroke_width = Some(val);
        }
        if key.eq_ignore_ascii_case("stroke-dasharray") && !val.is_empty() {
            stroke_dasharray = Some(val);
        }
    }

    ClassInlineStyles {
        style_attr,
        fill,
        stroke,
        stroke_width,
        stroke_dasharray,
    }
}

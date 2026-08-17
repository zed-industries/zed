use super::super::*;
use crate::sequence::sequence_text_line_step_px;
use std::collections::VecDeque;

pub(super) struct LoopTextRenderContext<'a> {
    pub(super) measurer: &'a dyn TextMeasurer,
    pub(super) style: &'a TextStyle,
}

impl<'a> LoopTextRenderContext<'a> {
    pub(super) fn new(measurer: &'a dyn TextMeasurer, style: &'a TextStyle) -> Self {
        Self { measurer, style }
    }
}

pub(super) fn display_block_label(raw_label: &str, always_show: bool) -> Option<String> {
    let decoded = merman_core::entities::decode_mermaid_entities_to_unicode(raw_label);
    let t = decoded.as_ref().trim();
    if t.is_empty() {
        if always_show {
            // Mermaid renders empty block labels as a zero-width space inside `<tspan>`.
            Some("\u{200B}".to_string())
        } else {
            None
        }
    } else {
        Some(bracketize(t))
    }
}

pub(super) fn wrap_svg_text_lines(
    text: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    max_width: Option<f64>,
) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for line in crate::text::split_html_br_lines(text) {
        if let Some(w) = max_width {
            lines.extend(wrap_svg_text_line(line, measurer, style, w));
        } else {
            lines.push(line.to_string());
        }
    }
    if lines.is_empty() {
        vec!["".to_string()]
    } else {
        lines
    }
}

pub(super) fn write_loop_text_lines(
    out: &mut String,
    ctx: &LoopTextRenderContext<'_>,
    x: f64,
    y0: f64,
    max_width: Option<f64>,
    text: &str,
    use_tspan: bool,
) {
    let line_step = sequence_text_line_step_px(ctx.style.font_size);
    let lines = wrap_svg_text_lines(text, ctx.measurer, ctx.style, max_width);
    for (i, line) in lines.into_iter().enumerate() {
        let y = y0 + (i as f64) * line_step;
        if use_tspan {
            let _ = write!(
                out,
                r#"<text x="{x}" y="{y}" text-anchor="middle" class="loopText" style="font-size: {fs}px; font-weight: 400;"><tspan x="{x}">{text}</tspan></text>"#,
                x = fmt(x),
                y = fmt(y),
                fs = fmt(ctx.style.font_size),
                text = escape_xml(&line)
            );
        } else {
            let _ = write!(
                out,
                r#"<text x="{x}" y="{y}" text-anchor="middle" class="loopText" style="font-size: {fs}px; font-weight: 400;">{text}</text>"#,
                x = fmt(x),
                y = fmt(y),
                fs = fmt(ctx.style.font_size),
                text = escape_xml(&line)
            );
        }
    }
}

pub(super) fn write_section_title_lines(
    out: &mut String,
    ctx: &LoopTextRenderContext<'_>,
    x: f64,
    y0: f64,
    max_width: Option<f64>,
    text: &str,
) {
    let line_step = sequence_text_line_step_px(ctx.style.font_size);
    let lines = wrap_svg_text_lines(text, ctx.measurer, ctx.style, max_width);
    for (i, line) in lines.into_iter().enumerate() {
        let y = y0 + (i as f64) * line_step;
        let _ = write!(
            out,
            r#"<text x="{x}" y="{y}" text-anchor="middle" class="sectionTitle" style="font-size: {fs}px; font-weight: 400;">{text}</text>"#,
            x = fmt(x),
            y = fmt(y),
            fs = fmt(ctx.style.font_size),
            text = escape_xml(&line)
        );
    }
}

fn bracketize(s: &str) -> String {
    let t = s.trim();
    if t.is_empty() {
        return "\u{200B}".to_string();
    }
    if t.starts_with('[') && t.ends_with(']') {
        return t.to_string();
    }
    format!("[{t}]")
}

fn split_line_to_words(text: &str) -> Vec<String> {
    let parts = text.split(' ').collect::<Vec<_>>();
    let mut out: Vec<String> = Vec::new();
    for part in parts {
        if !part.is_empty() {
            out.push(part.to_string());
        }
        out.push(" ".to_string());
    }
    while out.last().is_some_and(|s| s == " ") {
        out.pop();
    }
    out
}

fn wrap_svg_text_line(
    line: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    max_width: f64,
) -> Vec<String> {
    if !max_width.is_finite() || max_width <= 0.0 {
        return vec![line.to_string()];
    }

    // Browser SVG bbox wrapping is a little tighter than the vendored font model for narrow
    // Sequence frame labels. Keep this bounded to narrow frames; wider loop labels such as
    // `[Authentication check]` fit upstream and must not be forced to wrap.
    let narrow_frame_pad = if max_width <= 160.0 { 15.0 } else { 0.0 };
    let max_width = (max_width - narrow_frame_pad).max(1.0);

    fn svg_bbox_width_px(measurer: &dyn TextMeasurer, style: &TextStyle, text: &str) -> f64 {
        measurer
            .measure_svg_simple_text_bbox_width_for_wrap_px(text, style)
            .max(0.0)
    }

    let mut tokens = VecDeque::from(split_line_to_words(line));
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut force_break_after_next_non_space: bool = false;

    while let Some(tok) = tokens.pop_front() {
        if cur.is_empty() && tok == " " {
            continue;
        }

        let candidate = format!("{cur}{tok}");
        if svg_bbox_width_px(measurer, style, &candidate) <= max_width {
            cur = candidate;
            if force_break_after_next_non_space && tok != " " {
                out.push(cur.trim_end().to_string());
                cur.clear();
                force_break_after_next_non_space = false;
            }
            continue;
        }

        if !cur.trim().is_empty() {
            out.push(cur.trim_end().to_string());
            cur.clear();
            tokens.push_front(tok);
            continue;
        }

        if tok == " " {
            continue;
        }

        // `tok` itself does not fit on an empty line; split by characters.
        let chars = tok.chars().collect::<Vec<_>>();
        let mut cut = 1usize;
        while cut < chars.len() {
            let mut head: String = chars[..cut].iter().collect();
            let tail_len = chars.len().saturating_sub(cut);
            let should_hyphenate = tail_len > 0
                && !head.ends_with('-')
                && head
                    .chars()
                    .last()
                    .is_some_and(|ch| ch.is_ascii_alphanumeric());
            if should_hyphenate {
                head.push('-');
            }
            if svg_bbox_width_px(measurer, style, &head) > max_width {
                break;
            }
            cut += 1;
        }
        cut = cut.saturating_sub(1).max(1);
        let mut head: String = chars[..cut].iter().collect();
        let tail: String = chars[cut..].iter().collect();
        let mut hyphenated = false;
        if !tail.is_empty()
            && !head.ends_with('-')
            && head
                .chars()
                .last()
                .is_some_and(|ch| ch.is_ascii_alphanumeric())
        {
            head.push('-');
            if svg_bbox_width_px(measurer, style, &head) <= max_width {
                hyphenated = true;
            } else {
                head.pop();
            }
        }
        out.push(head);
        if !tail.is_empty() {
            tokens.push_front(tail);
            if hyphenated {
                force_break_after_next_non_space = true;
            }
        }
    }

    if !cur.trim().is_empty() {
        out.push(cur.trim_end().to_string());
    }

    if out.is_empty() {
        vec!["".to_string()]
    } else {
        out
    }
}

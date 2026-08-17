//! Flowchart-specific HTML label measurement parity helpers.

use super::{TextMetrics, TextStyle};

pub fn flowchart_html_line_height_px(font_size_px: f64) -> f64 {
    (font_size_px.max(1.0) * 1.5).max(1.0)
}

pub fn flowchart_apply_mermaid_string_whitespace_height_parity(
    metrics: &mut TextMetrics,
    raw_label: &str,
    style: &TextStyle,
) {
    if metrics.width <= 0.0 && metrics.height <= 0.0 {
        return;
    }

    // Mermaid FlowDB preserves leading/trailing whitespace when the label comes from a quoted
    // string (e.g. `[" test "]`). Upstream SVG baselines (Mermaid@11.12.3) show that DOM
    // measurement can allocate extra vertical space in some cases even though the rendered HTML
    // collapses whitespace.
    //
    // In practice, this "extra line height" behavior is only observed for labels with *both*
    // leading and trailing whitespace (e.g. `" test "`). Trailing-only whitespace (e.g.
    // `"Ends with spaces  "`) does not inflate height in upstream baselines.
    let bytes = raw_label.as_bytes();
    if bytes.is_empty() {
        return;
    }
    let leading_ws = matches!(bytes.first(), Some(b' ' | b'\t'));
    let trailing_ws = matches!(bytes.last(), Some(b' ' | b'\t'));
    if !(leading_ws && trailing_ws) {
        return;
    }

    let line_h = flowchart_html_line_height_px(style.font_size);
    metrics.height += 2.0 * line_h;
    metrics.line_count = metrics.line_count.saturating_add(2);
}

pub fn flowchart_apply_mermaid_styled_node_height_parity(
    metrics: &mut TextMetrics,
    style: &TextStyle,
) {
    if metrics.width <= 0.0 && metrics.height <= 0.0 {
        return;
    }

    // Mermaid@11.12.2 HTML label measurement for styled flowchart nodes (nodes with inline style or
    // classDef-applied style) often results in a 3-line label box, even when the label is a single
    // line. This is observable in upstream SVG fixtures (e.g.
    // `upstream_flow_style_inline_class_variants_spec` where `test` inside `:::exClass` becomes a
    // 72px-tall label box, yielding a 102px node height with padding).
    //
    // Model this as "at least 3 lines" in headless metrics so layout and foreignObject sizing match.
    let min_lines = 3usize;
    if metrics.line_count >= min_lines {
        return;
    }

    let line_h = flowchart_html_line_height_px(style.font_size);
    let extra = min_lines - metrics.line_count;
    metrics.height += extra as f64 * line_h;
    metrics.line_count = min_lines;
}

pub fn flowchart_html_has_inline_style_tags(lower_html: &str) -> bool {
    // Detect Mermaid HTML inline styling tags in a way that avoids false positives like
    // `<br>` matching `<b`.
    //
    // We keep this intentionally lightweight (no full HTML parser); for our purposes we only
    // need to decide whether the label needs the special inline-style measurement path.
    let bytes = lower_html.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] != b'<' {
            i += 1;
            continue;
        }
        i += 1;
        if i >= bytes.len() {
            break;
        }
        if bytes[i] == b'!' || bytes[i] == b'?' {
            continue;
        }
        if bytes[i] == b'/' {
            i += 1;
        }
        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
            i += 1;
        }
        if start == i {
            continue;
        }
        let name = &lower_html[start..i];
        if matches!(name, "strong" | "b" | "em" | "i") {
            return true;
        }
    }
    false
}

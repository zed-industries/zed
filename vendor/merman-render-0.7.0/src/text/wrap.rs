//! Text wrapping helpers used across diagrams.
//!
//! This module intentionally mirrors Mermaid behavior (including quirks) for parity.

use super::{
    DeterministicTextMeasurer, TextMeasurer, TextStyle, WrapMode, estimate_char_width_em,
    estimate_line_width_px,
};

pub fn ceil_to_1_64_px(v: f64) -> f64 {
    if !(v.is_finite() && v >= 0.0) {
        return 0.0;
    }
    // Avoid "ceil to next 1/64" due to tiny FP drift (e.g. `...0000000002` over the exact
    // lattice). Upstream Mermaid fixtures frequently land exactly on the 1/64px grid.
    let x = v * 64.0;
    let r = x.round();
    if (x - r).abs() < 1e-4 {
        return r / 64.0;
    }
    ((x) - 1e-5).ceil() / 64.0
}

pub fn round_to_1_64_px(v: f64) -> f64 {
    if !(v.is_finite() && v >= 0.0) {
        return 0.0;
    }
    let x = v * 64.0;
    let r = (x + 0.5).floor();
    r / 64.0
}

pub fn round_to_1_64_px_ties_to_even(v: f64) -> f64 {
    if !(v.is_finite() && v >= 0.0) {
        return 0.0;
    }
    let x = v * 64.0;
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
    let out = i / 64.0;
    if out == -0.0 { 0.0 } else { out }
}

pub fn wrap_text_lines_px(
    text: &str,
    style: &TextStyle,
    max_width_px: Option<f64>,
    wrap_mode: WrapMode,
) -> Vec<String> {
    let font_size = style.font_size.max(1.0);
    let max_width_px = max_width_px.filter(|w| w.is_finite() && *w > 0.0);
    let break_long_words = wrap_mode == WrapMode::SvgLike;

    fn split_token_to_width_px(tok: &str, max_width_px: f64, font_size: f64) -> (String, String) {
        let max_em = max_width_px / font_size;
        let mut em = 0.0;
        let chars = tok.chars().collect::<Vec<_>>();
        let mut split_at = 0usize;
        for (idx, ch) in chars.iter().enumerate() {
            em += estimate_char_width_em(*ch);
            if em > max_em && idx > 0 {
                break;
            }
            split_at = idx + 1;
            if em >= max_em {
                break;
            }
        }
        if split_at == 0 {
            split_at = 1.min(chars.len());
        }
        let head = chars.iter().take(split_at).collect::<String>();
        let tail = chars.iter().skip(split_at).collect::<String>();
        (head, tail)
    }

    fn wrap_line_to_width_px(
        line: &str,
        max_width_px: f64,
        font_size: f64,
        break_long_words: bool,
    ) -> Vec<String> {
        let mut tokens =
            std::collections::VecDeque::from(DeterministicTextMeasurer::split_line_to_words(line));
        let mut out: Vec<String> = Vec::new();
        let mut cur = String::new();

        while let Some(tok) = tokens.pop_front() {
            if cur.is_empty() && tok == " " {
                continue;
            }

            let candidate = format!("{cur}{tok}");
            let candidate_trimmed = candidate.trim_end();
            if estimate_line_width_px(candidate_trimmed, font_size) <= max_width_px {
                cur = candidate;
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

            if !break_long_words {
                out.push(tok);
            } else {
                let (head, tail) = split_token_to_width_px(&tok, max_width_px, font_size);
                out.push(head);
                if !tail.is_empty() {
                    tokens.push_front(tail);
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

    let mut lines: Vec<String> = Vec::new();
    for line in DeterministicTextMeasurer::normalized_text_lines(text) {
        if let Some(w) = max_width_px {
            lines.extend(wrap_line_to_width_px(&line, w, font_size, break_long_words));
        } else {
            lines.push(line);
        }
    }

    if lines.is_empty() {
        vec!["".to_string()]
    } else {
        lines
    }
}

/// Wraps SVG-like text into lines using the provided [`TextMeasurer`] for width decisions.
///
/// This mirrors Mermaid's `wrapLabel(...)` behavior at a high level (greedy word wrapping), but
/// delegates width measurements to the active measurer so diagram-specific SVG bbox overrides can
/// affect wrapping breakpoints.
pub fn wrap_text_lines_measurer(
    text: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    max_width_px: Option<f64>,
) -> Vec<String> {
    fn wrap_line(
        line: &str,
        measurer: &dyn TextMeasurer,
        style: &TextStyle,
        max_width_px: f64,
    ) -> Vec<String> {
        use std::collections::VecDeque;

        if !max_width_px.is_finite() || max_width_px <= 0.0 {
            return vec![line.to_string()];
        }

        let mut tokens = VecDeque::from(DeterministicTextMeasurer::split_line_to_words(line));
        let mut out: Vec<String> = Vec::new();
        let mut cur = String::new();

        while let Some(tok) = tokens.pop_front() {
            if cur.is_empty() && tok == " " {
                continue;
            }

            let candidate = format!("{cur}{tok}");
            if measurer.measure(candidate.trim_end(), style).width <= max_width_px {
                cur = candidate;
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

            // Token itself does not fit on an empty line; split by characters.
            let chars = tok.chars().collect::<Vec<_>>();
            let mut cut = 1usize;
            while cut < chars.len() {
                let head: String = chars[..cut].iter().collect();
                if measurer.measure(&head, style).width > max_width_px {
                    break;
                }
                cut += 1;
            }
            cut = cut.saturating_sub(1).max(1);
            let head: String = chars[..cut].iter().collect();
            let tail: String = chars[cut..].iter().collect();
            out.push(head);
            if !tail.is_empty() {
                tokens.push_front(tail);
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

    let mut out: Vec<String> = Vec::new();
    for line in split_html_br_lines(text) {
        if let Some(w) = max_width_px {
            out.extend(wrap_line(line, measurer, style, w));
        } else {
            out.push(line.to_string());
        }
    }
    if out.is_empty() {
        vec!["".to_string()]
    } else {
        out
    }
}

/// Wraps flowchart-style SVG text using the same width probe Mermaid uses for emitted `<text>`.
///
/// The helper is shared by layout and SVG emission so wrapped cluster titles, node labels, and edge
/// labels agree on both line breaks and the width used for root-bounds derivation.
pub(crate) fn wrap_svg_text_lines_by_measurement(
    measurer: &dyn TextMeasurer,
    text: &str,
    style: &TextStyle,
    max_width_px: Option<f64>,
    break_long_words: bool,
) -> Vec<String> {
    const EPS_PX: f64 = 0.125;
    let max_width_px = max_width_px.filter(|w| w.is_finite() && *w > 0.0);

    fn measure_w_px(measurer: &dyn TextMeasurer, style: &TextStyle, s: &str) -> f64 {
        measurer.measure(s, style).width
    }

    fn split_token_to_width_px(
        measurer: &dyn TextMeasurer,
        style: &TextStyle,
        tok: &str,
        max_width_px: f64,
    ) -> (String, String) {
        if max_width_px <= 0.0 {
            return (tok.to_string(), String::new());
        }
        let chars = tok.chars().collect::<Vec<_>>();
        if chars.is_empty() {
            return (String::new(), String::new());
        }

        let mut split_at = 1usize;
        for i in 1..=chars.len() {
            let head = chars[..i].iter().collect::<String>();
            let w = measure_w_px(measurer, style, &head);
            if w.is_finite() && w <= max_width_px + EPS_PX {
                split_at = i;
            } else {
                break;
            }
        }
        let head = chars[..split_at].iter().collect::<String>();
        let tail = chars[split_at..].iter().collect::<String>();
        (head, tail)
    }

    fn wrap_line_to_width_px(
        measurer: &dyn TextMeasurer,
        style: &TextStyle,
        line: &str,
        max_width_px: f64,
        break_long_words: bool,
    ) -> Vec<String> {
        let mut tokens =
            std::collections::VecDeque::from(DeterministicTextMeasurer::split_line_to_words(line));
        let mut out: Vec<String> = Vec::new();
        let mut cur = String::new();

        while let Some(tok) = tokens.pop_front() {
            if cur.is_empty() && tok == " " {
                continue;
            }

            let candidate = format!("{cur}{tok}");
            let candidate_trimmed = candidate.trim_end();
            if measure_w_px(measurer, style, candidate_trimmed) <= max_width_px + EPS_PX {
                cur = candidate;
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

            if measure_w_px(measurer, style, tok.as_str()) <= max_width_px + EPS_PX {
                cur = tok;
                continue;
            }

            if !break_long_words {
                out.push(tok);
                continue;
            }

            let (head, tail) = split_token_to_width_px(measurer, style, &tok, max_width_px);
            out.push(head);
            if !tail.is_empty() {
                tokens.push_front(tail);
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

    let mut lines = Vec::new();
    for line in DeterministicTextMeasurer::normalized_text_lines(text) {
        if let Some(w) = max_width_px {
            lines.extend(wrap_line_to_width_px(
                measurer,
                style,
                &line,
                w,
                break_long_words,
            ));
        } else {
            lines.push(line);
        }
    }

    if lines.is_empty() {
        vec!["".to_string()]
    } else {
        lines
    }
}

/// Splits a Mermaid label into lines using Mermaid's `<br>`-style line breaks.
///
/// Mirrors Mermaid's `lineBreakRegex = /<br\\s*\\/?>/gi` behavior:
/// - allows ASCII whitespace between `br` and the optional `/` or `>`
/// - does not accept extra characters (e.g. `<br \\t/>` with a literal backslash)
pub fn split_html_br_lines(text: &str) -> Vec<&str> {
    let b = text.as_bytes();
    let mut parts: Vec<&str> = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;
    while i + 3 < b.len() {
        if b[i] != b'<' {
            i += 1;
            continue;
        }
        let b1 = b[i + 1];
        let b2 = b[i + 2];
        if !matches!(b1, b'b' | b'B') || !matches!(b2, b'r' | b'R') {
            i += 1;
            continue;
        }
        let mut j = i + 3;
        while j < b.len() && matches!(b[j], b' ' | b'\t' | b'\r' | b'\n') {
            j += 1;
        }
        if j < b.len() && b[j] == b'/' {
            j += 1;
        }
        if j < b.len() && b[j] == b'>' {
            parts.push(&text[start..i]);
            start = j + 1;
            i = start;
            continue;
        }
        i += 1;
    }
    parts.push(&text[start..]);
    parts
}

const EXACT_SVG_WRAP_SINGLE_LINE_GUARD_PX: f64 = 4.0;

fn exact_svg_single_line_evidence_fits(
    label: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    max_width_px: f64,
) -> bool {
    if label.is_empty() || !max_width_px.is_finite() || max_width_px <= 0.0 {
        return false;
    }

    let exact_w = measurer
        .measure_svg_simple_text_bbox_width_px(label, style)
        .floor()
        .max(0.0);
    let probe_w = measurer
        .measure_svg_simple_text_bbox_width_for_wrap_px(label, style)
        .floor()
        .max(0.0);

    exact_w + EXACT_SVG_WRAP_SINGLE_LINE_GUARD_PX <= max_width_px
        && exact_w + EXACT_SVG_WRAP_SINGLE_LINE_GUARD_PX < probe_w
}

/// Wraps a label using Mermaid's `wrapLabel(...)` logic, producing wrapped *lines*.
///
/// This is used by Sequence diagrams (Mermaid@11.x) when `wrap: true` is enabled and when actor
/// descriptions are marked `wrap: true` by the DB layer.
pub fn wrap_label_like_mermaid_lines(
    label: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    max_width_px: f64,
) -> Vec<String> {
    if label.is_empty() {
        return Vec::new();
    }
    if !max_width_px.is_finite() || max_width_px <= 0.0 {
        return vec![label.to_string()];
    }

    // Mermaid short-circuits wrapping if the label already contains `<br>` breaks.
    if split_html_br_lines(label).len() > 1 {
        return split_html_br_lines(label)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
    }
    if exact_svg_single_line_evidence_fits(label, measurer, style, max_width_px) {
        return vec![label.to_string()];
    }

    fn w_px(measurer: &dyn TextMeasurer, style: &TextStyle, s: &str) -> f64 {
        // Upstream uses `calculateTextWidth(...)` which rounds the SVG bbox width.
        measurer
            .measure_svg_simple_text_bbox_width_for_wrap_px(s, style)
            .round()
    }

    fn break_string_like_mermaid(
        word: &str,
        max_width_px: f64,
        measurer: &dyn TextMeasurer,
        style: &TextStyle,
    ) -> (Vec<String>, String) {
        let chars: Vec<char> = word.chars().collect();
        let mut lines: Vec<String> = Vec::new();
        let mut current = String::new();
        for (idx, ch) in chars.iter().enumerate() {
            let next_line = format!("{current}{ch}");
            let line_w = w_px(measurer, style, &next_line);
            if line_w >= max_width_px {
                let is_last = idx + 1 == chars.len();
                if is_last {
                    lines.push(next_line);
                } else {
                    lines.push(format!("{next_line}-"));
                }
                current.clear();
            } else {
                current = next_line;
            }
        }
        (lines, current)
    }

    // Mermaid splits on ASCII spaces and drops empty chunks (collapsing multiple spaces).
    let words: Vec<&str> = label.split(' ').filter(|w| !w.is_empty()).collect();
    if words.is_empty() {
        return vec![label.to_string()];
    }

    let mut completed: Vec<String> = Vec::new();
    let mut next_line = String::new();
    for (idx, word) in words.iter().enumerate() {
        let word_len = w_px(measurer, style, &format!("{word} "));
        let next_len = w_px(measurer, style, &next_line);
        if word_len > max_width_px {
            let (hyphenated, remaining) =
                break_string_like_mermaid(word, max_width_px, measurer, style);
            completed.push(next_line.clone());
            completed.extend(hyphenated);
            next_line = remaining;
        } else if next_len + word_len >= max_width_px {
            completed.push(next_line.clone());
            next_line = (*word).to_string();
        } else if next_line.is_empty() {
            next_line = (*word).to_string();
        } else {
            next_line.push(' ');
            next_line.push_str(word);
        }

        let is_last = idx + 1 == words.len();
        if is_last {
            completed.push(next_line.clone());
        }
    }

    completed.into_iter().filter(|l| !l.is_empty()).collect()
}

/// A variant of [`wrap_label_like_mermaid_lines`] that uses `TextMeasurer::measure(...)` widths
/// (advance-like) rather than SVG bbox widths for wrap decisions.
///
/// This exists to match Mermaid Sequence message wrapping behavior in environments where SVG bbox
/// measurements differ slightly from the vendored bbox tables.
pub fn wrap_label_like_mermaid_lines_relaxed(
    label: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    max_width_px: f64,
) -> Vec<String> {
    if label.is_empty() {
        return Vec::new();
    }
    if !max_width_px.is_finite() || max_width_px <= 0.0 {
        return vec![label.to_string()];
    }

    if split_html_br_lines(label).len() > 1 {
        return split_html_br_lines(label)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
    }
    if exact_svg_single_line_evidence_fits(label, measurer, style, max_width_px) {
        return vec![label.to_string()];
    }

    fn w_px(measurer: &dyn TextMeasurer, style: &TextStyle, s: &str) -> f64 {
        measurer.measure(s, style).width.round()
    }

    fn break_string_like_mermaid(
        word: &str,
        max_width_px: f64,
        measurer: &dyn TextMeasurer,
        style: &TextStyle,
    ) -> (Vec<String>, String) {
        let chars: Vec<char> = word.chars().collect();
        let mut lines: Vec<String> = Vec::new();
        let mut current = String::new();
        for (idx, ch) in chars.iter().enumerate() {
            let next_line = format!("{current}{ch}");
            let line_w = w_px(measurer, style, &next_line);
            if line_w >= max_width_px {
                let is_last = idx + 1 == chars.len();
                if is_last {
                    lines.push(next_line);
                } else {
                    lines.push(format!("{next_line}-"));
                }
                current.clear();
            } else {
                current = next_line;
            }
        }
        (lines, current)
    }

    let words: Vec<&str> = label.split(' ').filter(|w| !w.is_empty()).collect();
    if words.is_empty() {
        return vec![label.to_string()];
    }

    let mut completed: Vec<String> = Vec::new();
    let mut next_line = String::new();
    for (idx, word) in words.iter().enumerate() {
        let word_len = w_px(measurer, style, &format!("{word} "));
        let next_len = w_px(measurer, style, &next_line);
        if word_len > max_width_px {
            let (hyphenated, remaining) =
                break_string_like_mermaid(word, max_width_px, measurer, style);
            completed.push(next_line.clone());
            completed.extend(hyphenated);
            next_line = remaining;
        } else if next_len + word_len >= max_width_px {
            completed.push(next_line.clone());
            next_line = (*word).to_string();
        } else if next_line.is_empty() {
            next_line = (*word).to_string();
        } else {
            next_line.push(' ');
            next_line.push_str(word);
        }

        let is_last = idx + 1 == words.len();
        if is_last {
            completed.push(next_line.clone());
        }
    }

    completed.into_iter().filter(|l| !l.is_empty()).collect()
}

/// A variant of [`wrap_label_like_mermaid_lines`] that floors width probes instead of rounding.
///
/// Mermaid uses `Math.round(getBBox().width)` for `calculateTextWidth(...)`, but flooring can be
/// closer to upstream SVG baselines for some wrapped Sequence message labels when our vendored
/// tables land slightly above the browser-reported integer width.
pub fn wrap_label_like_mermaid_lines_floored_bbox(
    label: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    max_width_px: f64,
) -> Vec<String> {
    if label.is_empty() {
        return Vec::new();
    }
    if !max_width_px.is_finite() || max_width_px <= 0.0 {
        return vec![label.to_string()];
    }

    if split_html_br_lines(label).len() > 1 {
        return split_html_br_lines(label)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
    }
    if exact_svg_single_line_evidence_fits(label, measurer, style, max_width_px) {
        return vec![label.to_string()];
    }

    fn w_px(measurer: &dyn TextMeasurer, style: &TextStyle, s: &str) -> f64 {
        measurer
            .measure_svg_simple_text_bbox_width_for_wrap_px(s, style)
            .floor()
    }

    fn break_string_like_mermaid(
        word: &str,
        max_width_px: f64,
        measurer: &dyn TextMeasurer,
        style: &TextStyle,
    ) -> (Vec<String>, String) {
        let chars: Vec<char> = word.chars().collect();
        let mut lines: Vec<String> = Vec::new();
        let mut current = String::new();
        for (idx, ch) in chars.iter().enumerate() {
            let next_line = format!("{current}{ch}");
            let line_w = w_px(measurer, style, &next_line);
            if line_w >= max_width_px {
                let is_last = idx + 1 == chars.len();
                if is_last {
                    lines.push(next_line);
                } else {
                    lines.push(format!("{next_line}-"));
                }
                current.clear();
            } else {
                current = next_line;
            }
        }
        (lines, current)
    }

    let words: Vec<&str> = label.split(' ').filter(|w| !w.is_empty()).collect();
    if words.is_empty() {
        return vec![label.to_string()];
    }

    let mut completed: Vec<String> = Vec::new();
    let mut next_line = String::new();
    for (idx, word) in words.iter().enumerate() {
        let word_len = w_px(measurer, style, &format!("{word} "));
        let next_len = w_px(measurer, style, &next_line);
        if word_len > max_width_px {
            let (hyphenated, remaining) =
                break_string_like_mermaid(word, max_width_px, measurer, style);
            completed.push(next_line.clone());
            completed.extend(hyphenated);
            next_line = remaining;
        } else if next_len + word_len >= max_width_px {
            completed.push(next_line.clone());
            next_line = (*word).to_string();
        } else if next_line.is_empty() {
            next_line = (*word).to_string();
        } else {
            next_line.push(' ');
            next_line.push_str(word);
        }

        let is_last = idx + 1 == words.len();
        if is_last {
            completed.push(next_line.clone());
        }
    }

    completed.into_iter().filter(|l| !l.is_empty()).collect()
}

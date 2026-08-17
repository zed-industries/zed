//! Flowchart label rendering helpers (HTML/SVG text).

use super::*;

pub(in crate::svg::parity) fn flowchart_label_html(
    label: &str,
    label_type: &str,
    config: &merman_core::MermaidConfig,
    math_renderer: Option<&(dyn crate::math::MathRenderer + Send + Sync)>,
) -> String {
    flowchart_label_html_impl(label, label_type, config, math_renderer, false)
}

pub(in crate::svg::parity) fn flowchart_edge_label_html(
    label: &str,
    label_type: &str,
    config: &merman_core::MermaidConfig,
    math_renderer: Option<&(dyn crate::math::MathRenderer + Send + Sync)>,
) -> String {
    flowchart_label_html_impl(label, label_type, config, math_renderer, true)
}

fn flowchart_label_html_impl(
    label: &str,
    label_type: &str,
    config: &merman_core::MermaidConfig,
    math_renderer: Option<&(dyn crate::math::MathRenderer + Send + Sync)>,
    force_non_markdown_paragraph: bool,
) -> String {
    if label.trim().is_empty() {
        return String::new();
    }

    fn xhtml_fix_fragment(input: &str) -> String {
        // `foreignObject` content lives in an XML document, so:
        // - void tags must be self-closed (`<br />`, not `<br>`)
        // - stray `<` / `>` in text must be entity-escaped (`&lt;`, `&gt;`)
        //
        // Mermaid's SVG baselines follow these rules.
        let input = input
            .replace("<br>", "<br />")
            .replace("<br/>", "<br />")
            .replace("<br >", "<br />")
            .replace("</br>", "<br />")
            .replace("</br/>", "<br />")
            .replace("</br />", "<br />")
            .replace("</br >", "<br />");

        fn is_xhtml_void_tag(name: &str) -> bool {
            matches!(
                name,
                "br" | "img"
                    | "hr"
                    | "input"
                    | "meta"
                    | "link"
                    | "source"
                    | "area"
                    | "base"
                    | "col"
                    | "embed"
                    | "param"
                    | "track"
                    | "wbr"
            )
        }

        fn xhtml_self_close_void_tag(tag: &str) -> String {
            if !tag.ends_with('>') {
                return tag.to_string();
            }
            let mut inner = tag[..tag.len() - 1].to_string();
            while inner.ends_with(|c: char| c.is_whitespace()) {
                inner.pop();
            }
            if inner.ends_with('/') {
                // Normalize to `<tag ... />` (space before `/`) to match upstream SVG baselines.
                while inner.ends_with('/') {
                    inner.pop();
                }
                while inner.ends_with(|c: char| c.is_whitespace()) {
                    inner.pop();
                }
                inner.push_str(" /");
                inner.push('>');
                return inner;
            }
            inner.push_str(" /");
            inner.push('>');
            inner
        }

        let mut out = String::with_capacity(input.len());
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '<' => {
                    let next = chars.peek().copied();
                    if !matches!(
                        next,
                        Some(n) if n.is_ascii_alphabetic() || matches!(n, '/' | '!' | '?')
                    ) {
                        out.push_str("&lt;");
                        continue;
                    }

                    let mut tag = String::from("<");
                    let mut saw_end = false;
                    for c in chars.by_ref() {
                        tag.push(c);
                        if c == '>' {
                            saw_end = true;
                            break;
                        }
                    }
                    if !saw_end {
                        out.push_str("&lt;");
                        out.push_str(&tag[1..]);
                        continue;
                    }

                    let tag_trim = tag.trim();
                    let inner = tag_trim
                        .trim_start_matches('<')
                        .trim_end_matches('>')
                        .trim();
                    let is_closing = inner.starts_with('/');
                    let name = inner
                        .trim_start_matches('/')
                        .trim_end_matches('/')
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_ascii_lowercase();
                    if !is_closing && is_xhtml_void_tag(&name) {
                        out.push_str(&xhtml_self_close_void_tag(tag_trim));
                    } else {
                        out.push_str(tag_trim);
                    }
                }
                '>' => out.push_str("&gt;"),
                '&' => {
                    // Preserve entities already encoded by the sanitizer.
                    let mut tail = String::new();
                    let mut ok = false;
                    for _ in 0..32 {
                        match chars.peek().copied() {
                            Some(';') => {
                                chars.next();
                                tail.push(';');
                                ok = true;
                                break;
                            }
                            Some(c)
                                if c.is_ascii_alphanumeric() || matches!(c, '#' | 'x' | 'X') =>
                            {
                                chars.next();
                                tail.push(c);
                            }
                            _ => break,
                        }
                    }
                    if ok {
                        out.push('&');
                        out.push_str(&tail);
                    } else {
                        out.push_str("&amp;");
                        out.push_str(&tail);
                    }
                }
                _ => out.push(ch),
            }
        }

        out
    }

    fn normalize_flowchart_img_tags(input: &str, fixed_width: bool) -> String {
        // Mermaid flowchart-v2 adds inline styles to `<img>` tags inside HTML labels to constrain
        // their layout. The SVG baseline uses XHTML, so we also self-close the tags later.
        if !input.to_ascii_lowercase().contains("<img") {
            return input.to_string();
        }

        let style = if fixed_width {
            "display: flex; flex-direction: column; min-width: 80px; max-width: 80px;"
        } else {
            "display: flex; flex-direction: column; width: 100%;"
        };

        fn extract_img_src(tag: &str) -> Option<String> {
            let lower = tag.to_ascii_lowercase();
            let idx = lower.find("src=")?;
            let rest = &tag[idx + 4..];
            let rest = rest.trim_start();
            let quote = rest.chars().next()?;
            if quote != '"' && quote != '\'' {
                return None;
            }
            let mut val = String::new();
            let mut it = rest.chars();
            let _ = it.next(); // consume quote
            for ch in it {
                if ch == quote {
                    break;
                }
                val.push(ch);
            }
            let val = val.trim().to_string();
            if val.is_empty() { None } else { Some(val) }
        }

        let mut out = String::with_capacity(input.len());
        let bytes = input.as_bytes();
        let mut i = 0usize;
        while i < bytes.len() {
            if bytes[i] == b'<' && i + 3 < bytes.len() {
                let rest = &input[i..];
                let rest_lower = rest.to_ascii_lowercase();
                if rest_lower.starts_with("<img") {
                    let Some(rel_end) = rest.find('>') else {
                        out.push_str(rest);
                        break;
                    };
                    let tag = &rest[..=rel_end];
                    let src = extract_img_src(tag);
                    out.push_str("<img");
                    if let Some(src) = src {
                        let _ = write!(out, r#" src="{}""#, escape_attr(&src));
                    }
                    out.push_str(r#" style=""#);
                    out.push_str(style);
                    out.push('"');
                    out.push('>');
                    i += rel_end + 1;
                    continue;
                }
            }
            let Some(ch) = input[i..].chars().next() else {
                break;
            };
            out.push(ch);
            i += ch.len_utf8();
        }
        out
    }

    fn is_single_img_label(label: &str) -> bool {
        let t = label.trim();
        let lower = t.to_ascii_lowercase();
        if !lower.starts_with("<img") {
            return false;
        }
        let Some(end) = t.find('>') else {
            return false;
        };
        t[end + 1..].trim().is_empty()
    }

    fn trim_markdown_trailing_newlines(
        input: std::borrow::Cow<'_, str>,
    ) -> std::borrow::Cow<'_, str> {
        if input.ends_with('\n') {
            std::borrow::Cow::Owned(input.trim_end_matches('\n').to_string())
        } else {
            input
        }
    }

    fn replace_non_markdown_html_line_breaks(input: &str) -> String {
        if input.contains("\\n") || input.contains('\n') {
            input.replace("\\n", "<br />").replace('\n', "<br />")
        } else {
            input.to_string()
        }
    }

    if let Some(r) = math_renderer {
        if label.contains("$$") {
            if let Some(html) = r.render_html_label(label, config) {
                return xhtml_fix_fragment(&merman_core::sanitize::sanitize_text(&html, config));
            }
        }
    }

    fn mermaid_markdown_to_html_minimal(
        markdown: &str,
        markdown_auto_wrap: bool,
        wants_p: bool,
    ) -> String {
        if !wants_p {
            return markdown.replace("\r\n", "\n");
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Ty {
            Strong,
            Em,
        }

        fn is_punctuation(ch: char) -> bool {
            !ch.is_whitespace() && !ch.is_alphanumeric()
        }

        fn mermaid_delim_can_open_close(
            ch: char,
            prev: Option<char>,
            next: Option<char>,
        ) -> (bool, bool) {
            let prev_is_ws = prev.is_none_or(|c| c.is_whitespace());
            let next_is_ws = next.is_none_or(|c| c.is_whitespace());
            let prev_is_punct = prev.is_some_and(is_punctuation);
            let next_is_punct = next.is_some_and(is_punctuation);

            let left_flanking = !next_is_ws && (!next_is_punct || prev_is_ws || prev_is_punct);
            let right_flanking = !prev_is_ws && (!prev_is_punct || next_is_ws || next_is_punct);

            if ch == '_' {
                let can_open = left_flanking && (!right_flanking || prev_is_ws || prev_is_punct);
                let can_close = right_flanking && (!left_flanking || next_is_ws || next_is_punct);
                (can_open, can_close)
            } else {
                (left_flanking, right_flanking)
            }
        }

        let s = markdown.replace("\r\n", "\n");
        let chars: Vec<char> = s.chars().collect();

        fn open_tag(ty: Ty) -> &'static str {
            match ty {
                Ty::Strong => "<strong>",
                Ty::Em => "<em>",
            }
        }

        fn close_tag(ty: Ty) -> &'static str {
            match ty {
                Ty::Strong => "</strong>",
                Ty::Em => "</em>",
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct Delim {
            ty: Ty,
            ch: char,
            run_len: usize,
            token_index: usize,
        }

        let mut tokens: Vec<String> = Vec::with_capacity(16);
        tokens.push("<p>".to_string());

        let mut text_buf = String::new();
        let flush_text = |tokens: &mut Vec<String>, text_buf: &mut String| {
            if !text_buf.is_empty() {
                tokens.push(std::mem::take(text_buf));
            }
        };

        let mut stack: Vec<Delim> = Vec::new();

        let mut i = 0usize;
        while i < chars.len() {
            let ch = chars[i];

            if ch == '\n' {
                let mut j = i;
                while j < chars.len() && chars[j] == '\n' {
                    j += 1;
                }
                let newline_count = j - i;

                if newline_count >= 2 {
                    while text_buf.ends_with(' ') {
                        text_buf.pop();
                    }
                    flush_text(&mut tokens, &mut text_buf);
                    tokens.push("</p><p>".to_string());
                    i = j;
                    while i < chars.len() && chars[i] == ' ' {
                        i += 1;
                    }
                    continue;
                }

                flush_text(&mut tokens, &mut text_buf);
                tokens.push("<br/>".to_string());
                i += 1;
                while i < chars.len() && chars[i] == ' ' {
                    i += 1;
                }
                continue;
            }

            if ch == '<' {
                if let Some(end_rel) = chars[i..].iter().position(|c| *c == '>') {
                    let end = i + end_rel;
                    flush_text(&mut tokens, &mut text_buf);
                    let mut tag = String::new();
                    for c in &chars[i..=end] {
                        tag.push(*c);
                    }
                    tokens.push(tag);
                    i = end + 1;
                    continue;
                }
            }

            if ch == '*' || ch == '_' {
                let run_len = if i + 1 < chars.len() && chars[i + 1] == ch {
                    2
                } else {
                    1
                };
                let want = if run_len == 2 { Ty::Strong } else { Ty::Em };
                let prev = if i > 0 { Some(chars[i - 1]) } else { None };
                let next = if i + run_len < chars.len() {
                    Some(chars[i + run_len])
                } else {
                    None
                };
                let (can_open, can_close) = mermaid_delim_can_open_close(ch, prev, next);

                flush_text(&mut tokens, &mut text_buf);

                let delim_text: String = std::iter::repeat_n(ch, run_len).collect();

                if can_close
                    && stack
                        .last()
                        .is_some_and(|d| d.ty == want && d.ch == ch && d.run_len == run_len)
                {
                    if let Some(opener) = stack.pop() {
                        tokens[opener.token_index] = open_tag(want).to_string();
                        tokens.push(close_tag(want).to_string());
                        i += run_len;
                        continue;
                    }
                }
                if can_open {
                    let token_index = tokens.len();
                    tokens.push(delim_text);
                    stack.push(Delim {
                        ty: want,
                        ch,
                        run_len,
                        token_index,
                    });
                    i += run_len;
                    continue;
                }

                tokens.push(delim_text);
                i += run_len;
                continue;
            }

            if ch == ' ' && !markdown_auto_wrap {
                text_buf.push_str("&nbsp;");
            } else {
                text_buf.push(ch);
            }
            i += 1;
        }

        while text_buf.ends_with(' ') {
            text_buf.pop();
        }
        flush_text(&mut tokens, &mut text_buf);
        tokens.push("</p>".to_string());
        tokens.concat()
    }

    match label_type {
        "markdown" => {
            let decoded = decode_mermaid_entities_for_render_text(label);
            let decoded = if decoded.contains("\\\\") {
                std::borrow::Cow::Owned(decoded.replace("\\\\", "\\"))
            } else {
                decoded
            };
            let decoded = trim_markdown_trailing_newlines(decoded);
            let markdown_auto_wrap = config
                .as_value()
                .get("markdownAutoWrap")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(true);
            let html_out = if crate::text::mermaid_markdown_contains_raw_blocks(decoded.as_ref()) {
                crate::text::mermaid_markdown_to_html_label_fragment(
                    decoded.as_ref(),
                    markdown_auto_wrap,
                )
            } else {
                let wants_p = crate::text::mermaid_markdown_wants_paragraph_wrap(decoded.as_ref());
                mermaid_markdown_to_html_minimal(decoded.as_ref(), markdown_auto_wrap, wants_p)
            };
            let html_out = html_out.trim().to_string();
            let html_out = crate::text::replace_fontawesome_icons(&html_out);
            xhtml_fix_fragment(&merman_core::sanitize::sanitize_text(&html_out, config))
        }
        _ => {
            let label = if label.contains("\r\n") {
                label.replace("\r\n", "\n")
            } else {
                label.to_string()
            };
            let label = crate::flowchart::flowchart_decode_label_escapes(&label);
            let label = if label_type == "string" {
                label.trim().to_string()
            } else {
                label
            };
            let label = label.trim_end_matches('\n');
            let wants_p = force_non_markdown_paragraph
                || crate::text::mermaid_markdown_wants_paragraph_wrap(label);
            let label = crate::flowchart::flowchart_normalize_plain_multiline_label_for_html(label);
            let label = label.as_ref();

            // Fast path for the overwhelmingly common case: plain text labels (no HTML, no
            // entities, no Mermaid icon syntax). In upstream Mermaid, these go through
            // `sanitizeText(...)` but the output is unchanged; skipping the HTML sanitizer here is
            // a large win in flowcharts with many nodes.
            if !label.contains('<')
                && !label.contains('>')
                && !label.contains('&')
                && !label.contains(":fa-")
            {
                let inner = if wants_p {
                    replace_non_markdown_html_line_breaks(label)
                } else {
                    label.to_string()
                };
                if wants_p {
                    return format!("<p>{inner}</p>");
                }
                return inner;
            }

            let label = if wants_p {
                replace_non_markdown_html_line_breaks(label)
            } else {
                label.to_string()
            };
            let fixed_img_width = is_single_img_label(&label);
            let label = normalize_flowchart_img_tags(&label, fixed_img_width);
            let wrapped = if !wants_p {
                label
            } else {
                format!("<p>{}</p>", label)
            };
            let wrapped = if wrapped.contains(":fa-") {
                crate::text::replace_fontawesome_icons(&wrapped)
            } else {
                wrapped
            };
            xhtml_fix_fragment(&merman_core::sanitize::sanitize_text(&wrapped, config))
        }
    }
}

pub(in crate::svg::parity) fn flowchart_label_plain_text(
    label: &str,
    label_type: &str,
    html_labels: bool,
) -> String {
    crate::flowchart::flowchart_label_plain_text_for_layout(label, label_type, html_labels)
}

pub(in crate::svg::parity) fn write_flowchart_svg_text(
    out: &mut String,
    text: &str,
    include_style: bool,
) {
    write_flowchart_svg_text_impl(out, text, include_style, false, true);
}

pub(in crate::svg::parity) fn write_flowchart_svg_text_centered(
    out: &mut String,
    text: &str,
    include_style: bool,
) {
    write_flowchart_svg_text_impl(out, text, include_style, true, true);
}

fn open_flowchart_svg_text(out: &mut String, include_style: bool, center_text: bool) {
    // Mirrors Mermaid's SVG text structure when `flowchart.htmlLabels=false`.
    match (include_style, center_text) {
        (true, true) => out.push_str(r#"<text y="-10.1" style="" text-anchor="middle">"#),
        (true, false) => out.push_str(r#"<text y="-10.1" style="">"#),
        (false, true) => out.push_str(r#"<text y="-10.1" text-anchor="middle">"#),
        (false, false) => out.push_str(r#"<text y="-10.1">"#),
    }
}

fn flowchart_outer_tspan_class(include_row_class: bool) -> &'static str {
    if include_row_class {
        "row text-outer-tspan"
    } else {
        "text-outer-tspan"
    }
}

fn write_empty_flowchart_tspan(out: &mut String, center_text: bool, include_row_class: bool) {
    let outer_class = flowchart_outer_tspan_class(include_row_class);
    if center_text {
        let _ = write!(
            out,
            r#"<tspan class="{}" x="0" y="-0.1em" dy="1.1em" text-anchor="middle"/>"#,
            outer_class
        );
    } else {
        let _ = write!(
            out,
            r#"<tspan class="{}" x="0" y="-0.1em" dy="1.1em"/>"#,
            outer_class
        );
    }
}

fn open_flowchart_tspan(out: &mut String, idx: usize, center_text: bool, include_row_class: bool) {
    let text_anchor = if center_text {
        r#" text-anchor="middle""#
    } else {
        ""
    };
    let outer_class = flowchart_outer_tspan_class(include_row_class);
    if idx == 0 {
        let _ = write!(
            out,
            r#"<tspan class="{}" x="0" y="-0.1em" dy="1.1em"{}>"#,
            outer_class, text_anchor
        );
    } else {
        // Mermaid sets an absolute `y` for each subsequent line, then uses `dy="1.1em"` as
        // the line-height increment. This yields `y="1em"` for the 2nd line and `y="2.1em"`
        // for the 3rd line, etc.
        let y_em = if idx == 1 {
            "1em".to_string()
        } else {
            format!("{:.1}em", 1.0 + (idx as f64 - 1.0) * 1.1)
        };
        let _ = write!(
            out,
            r#"<tspan class="{}" x="0" y="{}" dy="1.1em"{}>"#,
            outer_class, y_em, text_anchor
        );
    }
}

fn write_flowchart_svg_text_impl(
    out: &mut String,
    text: &str,
    include_style: bool,
    center_text: bool,
    include_row_class: bool,
) {
    open_flowchart_svg_text(out, include_style, center_text);

    let lines = crate::text::DeterministicTextMeasurer::normalized_text_lines(text);
    let allow_simple_markdown = !text.contains('`');
    if lines.len() == 1 && lines[0].is_empty() {
        write_empty_flowchart_tspan(out, center_text, include_row_class);
        out.push_str("</text>");
        return;
    }

    fn split_mermaid_escaped_tag_tokens(line: &str) -> Option<Vec<String>> {
        // Mermaid’s SVG text renderer tokenizes a simple HTML-tag wrapper even when htmlLabels are
        // disabled, resulting in 3 inner <tspan> runs like:
        //   `<strong>Haiya</strong>` -> `<strong>` + ` Haiya` + ` </strong>`
        // (all still rendered as escaped text).
        let line = line.trim_end();
        if !line.starts_with('<') || !line.ends_with('>') {
            return None;
        }
        let open_end = line.find('>')?;
        let open_tag = &line[..=open_end];
        if open_tag.starts_with("</") {
            return None;
        }
        let open_inner = open_tag.trim_start_matches('<').trim_end_matches('>');
        let tag_name = open_inner
            .split_whitespace()
            .next()
            .filter(|s| !s.is_empty())?;
        let close_tag = format!("</{tag_name}>");
        if !line.ends_with(&close_tag) {
            return None;
        }
        let inner = &line[open_end + 1..line.len() - close_tag.len()];
        Some(vec![
            open_tag.to_string(),
            inner.trim().to_string(),
            close_tag,
        ])
    }

    fn strip_simple_markdown_word(word: &str) -> (std::borrow::Cow<'_, str>, bool, bool) {
        // Mermaid flowchart-v2 SVG labels apply a small subset of Markdown styling even when the
        // FlowDB label type is `text` (not `markdown`), e.g. `**bold**`.
        if word.len() >= 4 && word.starts_with("**") && word.ends_with("**") {
            let inner = &word[2..word.len() - 2];
            if !inner.is_empty() {
                return (std::borrow::Cow::Borrowed(inner), true, false);
            }
        }
        if word.len() >= 2 && word.starts_with('*') && word.ends_with('*') {
            let inner = &word[1..word.len() - 1];
            if !inner.is_empty() {
                return (std::borrow::Cow::Borrowed(inner), false, true);
            }
        }
        if word.len() >= 2 && word.starts_with('_') && word.ends_with('_') {
            let inner = &word[1..word.len() - 1];
            if !inner.is_empty() {
                return (std::borrow::Cow::Borrowed(inner), false, true);
            }
        }
        (std::borrow::Cow::Borrowed(word), false, false)
    }

    for (idx, line) in lines.iter().enumerate() {
        open_flowchart_tspan(out, idx, center_text, include_row_class);
        let words: Vec<String> = split_mermaid_escaped_tag_tokens(line).unwrap_or_else(|| {
            line.split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
        });
        for (word_idx, word) in words.iter().enumerate() {
            let (word, is_strong, is_em) = if allow_simple_markdown {
                strip_simple_markdown_word(word)
            } else {
                (std::borrow::Cow::Borrowed(word.as_str()), false, false)
            };
            let font_style = if is_em { "italic" } else { "normal" };
            let font_weight = if is_strong { "bold" } else { "normal" };
            let _ = write!(
                out,
                r#"<tspan font-style="{}" class="text-inner-tspan" font-weight="{}">"#,
                font_style, font_weight
            );
            if word_idx == 0 {
                escape_xml_into(out, word.as_ref());
            } else {
                out.push(' ');
                escape_xml_into(out, word.as_ref());
            }
            out.push_str("</tspan>");
        }
        out.push_str("</tspan>");
    }

    out.push_str("</text>");
}

fn normalized_markdown_label(markdown: &str) -> &str {
    markdown
        .strip_prefix('`')
        .and_then(|s| s.strip_suffix('`'))
        .unwrap_or(markdown)
}

fn markdown_to_svg_word_lines(markdown: &str) -> Vec<Vec<(String, bool, bool)>> {
    crate::text::mermaid_markdown_to_lines(markdown, true)
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|(w, ty)| {
                    let is_strong = ty == crate::text::MermaidMarkdownWordType::Strong;
                    let is_em = ty == crate::text::MermaidMarkdownWordType::Em;
                    (w, is_strong, is_em)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn markdown_to_wrapped_svg_word_lines(
    measurer: &dyn crate::text::TextMeasurer,
    markdown: &str,
    style: &crate::text::TextStyle,
    max_width_px: Option<f64>,
) -> Vec<Vec<(String, bool, bool)>> {
    crate::text::mermaid_markdown_to_wrapped_word_lines(
        measurer,
        markdown,
        style,
        max_width_px,
        crate::text::WrapMode::SvgLike,
    )
    .into_iter()
    .map(|line| {
        line.into_iter()
            .map(|(w, ty)| {
                let is_strong = ty == crate::text::MermaidMarkdownWordType::Strong;
                let is_em = ty == crate::text::MermaidMarkdownWordType::Em;
                (w, is_strong, is_em)
            })
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()
}

fn write_flowchart_svg_text_markdown_lines(
    out: &mut String,
    lines: &[Vec<(String, bool, bool)>],
    include_style: bool,
    center_text: bool,
    include_row_class: bool,
) {
    open_flowchart_svg_text(out, include_style, center_text);

    if lines.len() == 1 && lines[0].is_empty() {
        write_empty_flowchart_tspan(out, center_text, include_row_class);
        out.push_str("</text>");
        return;
    }

    for (idx, words) in lines.iter().enumerate() {
        open_flowchart_tspan(out, idx, center_text, include_row_class);

        for (word_idx, (word, is_strong, is_em)) in words.iter().enumerate() {
            let font_style = if *is_em { "italic" } else { "normal" };
            let font_weight = if *is_strong { "bold" } else { "normal" };
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

pub(in crate::svg::parity) fn write_flowchart_svg_text_markdown(
    out: &mut String,
    markdown: &str,
    include_style: bool,
) {
    let markdown = normalized_markdown_label(markdown);
    let lines = markdown_to_svg_word_lines(markdown);
    write_flowchart_svg_text_markdown_lines(out, &lines, include_style, false, true);
}

pub(in crate::svg::parity) fn write_flowchart_svg_text_markdown_centered(
    out: &mut String,
    markdown: &str,
    include_style: bool,
) {
    let markdown = normalized_markdown_label(markdown);
    let lines = markdown_to_svg_word_lines(markdown);
    write_flowchart_svg_text_markdown_lines(out, &lines, include_style, true, true);
}

pub(in crate::svg::parity) fn write_flowchart_svg_text_markdown_wrapped_centered(
    out: &mut String,
    markdown: &str,
    include_style: bool,
    measurer: &dyn crate::text::TextMeasurer,
    style: &crate::text::TextStyle,
    max_width_px: Option<f64>,
) {
    let markdown = normalized_markdown_label(markdown);
    let lines = markdown_to_wrapped_svg_word_lines(measurer, markdown, style, max_width_px);
    write_flowchart_svg_text_markdown_lines(out, &lines, include_style, true, true);
}

pub(in crate::svg::parity) fn write_flowchart_svg_text_markdown_wrapped(
    out: &mut String,
    markdown: &str,
    include_style: bool,
    measurer: &dyn crate::text::TextMeasurer,
    style: &crate::text::TextStyle,
    max_width_px: Option<f64>,
) {
    let markdown = normalized_markdown_label(markdown);
    let lines = markdown_to_wrapped_svg_word_lines(measurer, markdown, style, max_width_px);
    write_flowchart_svg_text_markdown_lines(out, &lines, include_style, false, true);
}

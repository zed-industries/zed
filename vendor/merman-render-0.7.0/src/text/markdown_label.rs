//! Mermaid HTML/XHTML label fragment helpers.

fn mermaid_markdown_line_starts_raw_block(line: &str) -> bool {
    let line = line.trim_end();
    if line.is_empty() {
        return false;
    }

    // Markdown block constructs generally allow up to 3 leading spaces.
    let mut i = 0usize;
    for ch in line.chars() {
        if ch == ' ' && i < 3 {
            i += 1;
            continue;
        }
        break;
    }
    let s = &line[i.min(line.len())..];
    let line_trim = s.trim();
    if line_trim.is_empty() {
        return false;
    }

    if line_trim.starts_with('#') || line_trim.starts_with('>') {
        return true;
    }
    if line_trim.starts_with("```") || line_trim.starts_with("~~~") {
        return true;
    }

    if line_trim.len() >= 3 {
        let no_spaces: String = line_trim.chars().filter(|c| !c.is_whitespace()).collect();
        let ch = no_spaces.chars().next().unwrap_or('\0');
        if (ch == '-' || ch == '_' || ch == '*')
            && no_spaces.chars().all(|c| c == ch)
            && no_spaces.len() >= 3
        {
            return true;
        }
    }

    let bytes = line_trim.as_bytes();
    let mut j = 0usize;
    while j < bytes.len() && bytes[j].is_ascii_digit() {
        j += 1;
    }
    if j > 0 && j + 1 < bytes.len() && (bytes[j] == b'.' || bytes[j] == b')') {
        let next = bytes[j + 1];
        if next == b' ' || next == b'\t' {
            return true;
        }
    }

    if bytes.len() >= 2 {
        let first = bytes[0];
        let second = bytes[1];
        if (first == b'-' || first == b'*' || first == b'+') && (second == b' ' || second == b'\t')
        {
            return true;
        }
    }

    false
}

pub(crate) fn mermaid_markdown_contains_raw_blocks(markdown: &str) -> bool {
    markdown
        .replace("\r\n", "\n")
        .lines()
        .any(mermaid_markdown_line_starts_raw_block)
}

fn mermaid_markdown_paragraph_to_html(label: &str, markdown_auto_wrap: bool) -> String {
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

    let s = label.replace("\r\n", "\n");
    let chars: Vec<char> = s.chars().collect();
    let mut tokens: Vec<String> = Vec::with_capacity(16);
    tokens.push("<p>".to_string());

    let mut text_buf = String::new();
    let flush_text = |tokens: &mut Vec<String>, text_buf: &mut String| {
        if !text_buf.is_empty() {
            tokens.push(std::mem::take(text_buf));
        }
    };

    let mut stack: Vec<Delim> = Vec::new();
    let mut in_code_span = false;
    let mut i = 0usize;
    while i < chars.len() {
        let ch = chars[i];

        if ch == '\n' {
            while text_buf.ends_with(' ') {
                text_buf.pop();
            }
            flush_text(&mut tokens, &mut text_buf);
            tokens.push("<br/>".to_string());
            i += 1;
            while i < chars.len() && chars[i] == ' ' {
                i += 1;
            }
            continue;
        }

        if ch == '`' {
            text_buf.push(ch);
            in_code_span = !in_code_span;
            i += 1;
            continue;
        }

        if in_code_span {
            if ch == ' ' && !markdown_auto_wrap {
                text_buf.push_str("&nbsp;");
            } else {
                text_buf.push(ch);
            }
            i += 1;
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
                if tag.eq_ignore_ascii_case("<br>")
                    || tag.eq_ignore_ascii_case("<br/>")
                    || tag.eq_ignore_ascii_case("<br />")
                    || tag.eq_ignore_ascii_case("</br>")
                    || tag.eq_ignore_ascii_case("</br/>")
                    || tag.eq_ignore_ascii_case("</br />")
                    || tag.eq_ignore_ascii_case("</br >")
                {
                    tokens.push("<br />".to_string());
                } else {
                    tokens.push(tag);
                }
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
            if ch == '*' && can_close {
                if run_len == 1
                    && stack
                        .last()
                        .is_some_and(|d| d.ty == Ty::Strong && d.ch == '*' && d.run_len == 2)
                {
                    if let Some(opener) = stack.pop() {
                        tokens[opener.token_index] = format!("*{}", open_tag(Ty::Em));
                        tokens.push(close_tag(Ty::Em).to_string());
                        i += 1;
                        continue;
                    }
                }
                if run_len == 2
                    && stack
                        .last()
                        .is_some_and(|d| d.ty == Ty::Em && d.ch == '*' && d.run_len == 1)
                {
                    if let Some(opener) = stack.pop() {
                        tokens[opener.token_index] = open_tag(Ty::Em).to_string();
                        tokens.push(close_tag(Ty::Em).to_string());
                        tokens.push("*".to_string());
                        i += 2;
                        continue;
                    }
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

fn mermaid_collapse_raw_html_label_text(markdown: &str) -> String {
    let mut out = String::with_capacity(markdown.len());
    let mut pending_space = false;
    for ch in markdown.chars() {
        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && !out.is_empty() {
            out.push(' ');
        }
        pending_space = false;
        out.push(ch);
    }
    out.trim().to_string()
}

/// Approximate the final browser DOM fragment that Mermaid HTML labels produce for Markdown.
///
/// Mermaid's `markdownToHTML()` returns raw block Markdown for unsupported constructs (lists,
/// headings, fenced blocks, etc.). Once that HTML is inserted into a `<span>` inside a
/// `foreignObject`, browser whitespace collapsing turns those raw block lines into plain inline
/// text. We reproduce that post-DOM shape here so layout measurement and strict SVG parity stay in
/// sync.
pub(crate) fn mermaid_markdown_to_html_label_fragment(
    markdown: &str,
    markdown_auto_wrap: bool,
) -> String {
    let markdown = markdown.replace("\r\n", "\n");
    if markdown.is_empty() {
        return String::new();
    }

    let lines: Vec<&str> = markdown.split('\n').collect();
    let mut out = String::new();
    let mut paragraph_lines: Vec<&str> = Vec::new();
    let mut i = 0usize;

    while i < lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            if !paragraph_lines.is_empty() {
                out.push_str(&mermaid_markdown_paragraph_to_html(
                    &paragraph_lines.join("\n"),
                    markdown_auto_wrap,
                ));
                paragraph_lines.clear();
            }
            i += 1;
            continue;
        }

        if mermaid_markdown_line_starts_raw_block(line) {
            if !paragraph_lines.is_empty() {
                out.push_str(&mermaid_markdown_paragraph_to_html(
                    &paragraph_lines.join("\n"),
                    markdown_auto_wrap,
                ));
                paragraph_lines.clear();
            }

            let mut raw_block = String::from(line);
            i += 1;
            while i < lines.len() {
                let next = lines[i];
                if next.trim().is_empty() {
                    break;
                }
                if mermaid_markdown_line_starts_raw_block(next) {
                    raw_block.push('\n');
                    raw_block.push_str(next);
                    i += 1;
                    continue;
                }
                break;
            }
            out.push_str(&mermaid_collapse_raw_html_label_text(&raw_block));
            continue;
        }

        paragraph_lines.push(line);
        i += 1;
    }

    if !paragraph_lines.is_empty() {
        out.push_str(&mermaid_markdown_paragraph_to_html(
            &paragraph_lines.join("\n"),
            markdown_auto_wrap,
        ));
    }

    out
}
fn escape_xml_text_preserving_entities(raw: &str) -> String {
    fn is_valid_entity(entity: &str) -> bool {
        if entity.is_empty() {
            return false;
        }
        if let Some(hex) = entity
            .strip_prefix("#x")
            .or_else(|| entity.strip_prefix("#X"))
        {
            return !hex.is_empty() && hex.chars().all(|c| c.is_ascii_hexdigit());
        }
        if let Some(dec) = entity.strip_prefix('#') {
            return !dec.is_empty() && dec.chars().all(|c| c.is_ascii_digit());
        }
        let mut it = entity.chars();
        let Some(first) = it.next() else {
            return false;
        };
        if !first.is_ascii_alphabetic() {
            return false;
        }
        it.all(|c| c.is_ascii_alphanumeric())
    }

    fn escape_xml_segment(out: &mut String, raw: &str) {
        for ch in raw.chars() {
            match ch {
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                _ => out.push(ch),
            }
        }
    }

    let mut out = String::with_capacity(raw.len());
    let mut i = 0usize;
    while let Some(rel) = raw[i..].find('&') {
        let amp = i + rel;
        escape_xml_segment(&mut out, &raw[i..amp]);
        let tail = &raw[amp + 1..];
        if let Some(semi_rel) = tail.find(';') {
            let semi = amp + 1 + semi_rel;
            let entity = &raw[amp + 1..semi];
            if is_valid_entity(entity) {
                out.push('&');
                out.push_str(entity);
                out.push(';');
                i = semi + 1;
                continue;
            }
        }
        out.push_str("&amp;");
        i = amp + 1;
    }
    escape_xml_segment(&mut out, &raw[i..]);
    out
}

fn mermaid_markdown_paragraph_to_xhtml(label: &str, markdown_auto_wrap: bool) -> String {
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

    let s = label.replace("\r\n", "\n");
    let chars: Vec<char> = s.chars().collect();
    let mut tokens: Vec<String> = Vec::with_capacity(16);
    tokens.push("<p>".to_string());

    let mut text_buf = String::new();
    let flush_text = |tokens: &mut Vec<String>, text_buf: &mut String| {
        if text_buf.is_empty() {
            return;
        }
        let raw = std::mem::take(text_buf);
        tokens.push(escape_xml_text_preserving_entities(&raw));
    };

    let mut stack: Vec<Delim> = Vec::new();
    let mut in_code_span = false;
    let mut i = 0usize;
    while i < chars.len() {
        let ch = chars[i];

        if ch == '\n' {
            while text_buf.ends_with(' ') {
                text_buf.pop();
            }
            flush_text(&mut tokens, &mut text_buf);
            tokens.push("<br/>".to_string());
            i += 1;
            while i < chars.len() && chars[i] == ' ' {
                i += 1;
            }
            continue;
        }

        if ch == '`' {
            text_buf.push(ch);
            in_code_span = !in_code_span;
            i += 1;
            continue;
        }

        if in_code_span {
            if ch == ' ' && !markdown_auto_wrap {
                text_buf.push_str("&nbsp;");
            } else {
                text_buf.push(ch);
            }
            i += 1;
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
                if tag.eq_ignore_ascii_case("<br>")
                    || tag.eq_ignore_ascii_case("<br/>")
                    || tag.eq_ignore_ascii_case("<br />")
                    || tag.eq_ignore_ascii_case("</br>")
                    || tag.eq_ignore_ascii_case("</br/>")
                    || tag.eq_ignore_ascii_case("</br />")
                    || tag.eq_ignore_ascii_case("</br >")
                {
                    tokens.push("<br/>".to_string());
                } else {
                    tokens.push(tag);
                }
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
            if ch == '*' && can_close {
                if run_len == 1
                    && stack
                        .last()
                        .is_some_and(|d| d.ty == Ty::Strong && d.ch == '*' && d.run_len == 2)
                {
                    if let Some(opener) = stack.pop() {
                        tokens[opener.token_index] = format!("*{}", open_tag(Ty::Em));
                        tokens.push(close_tag(Ty::Em).to_string());
                        i += 1;
                        continue;
                    }
                }
                if run_len == 2
                    && stack
                        .last()
                        .is_some_and(|d| d.ty == Ty::Em && d.ch == '*' && d.run_len == 1)
                {
                    if let Some(opener) = stack.pop() {
                        tokens[opener.token_index] = open_tag(Ty::Em).to_string();
                        tokens.push(close_tag(Ty::Em).to_string());
                        tokens.push("*".to_string());
                        i += 2;
                        continue;
                    }
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

/// XHTML-safe variant of Mermaid HTML-label Markdown rendering for diagrams that inject the
/// fragment directly into `<foreignObject>` content without running the flowchart sanitizer path.
pub(crate) fn mermaid_markdown_to_xhtml_label_fragment(
    markdown: &str,
    markdown_auto_wrap: bool,
) -> String {
    let markdown = markdown.replace("\r\n", "\n");
    if markdown.is_empty() {
        return String::new();
    }

    let lines: Vec<&str> = markdown.split('\n').collect();
    let mut out = String::new();
    let mut paragraph_lines: Vec<&str> = Vec::new();
    let mut i = 0usize;

    while i < lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            if !paragraph_lines.is_empty() {
                out.push_str(&mermaid_markdown_paragraph_to_xhtml(
                    &paragraph_lines.join("\n"),
                    markdown_auto_wrap,
                ));
                paragraph_lines.clear();
            }
            i += 1;
            continue;
        }

        if mermaid_markdown_line_starts_raw_block(line) {
            if !paragraph_lines.is_empty() {
                out.push_str(&mermaid_markdown_paragraph_to_xhtml(
                    &paragraph_lines.join("\n"),
                    markdown_auto_wrap,
                ));
                paragraph_lines.clear();
            }

            let mut raw_block = String::from(line);
            i += 1;
            while i < lines.len() {
                let next = lines[i];
                if next.trim().is_empty() {
                    break;
                }
                if mermaid_markdown_line_starts_raw_block(next) {
                    raw_block.push('\n');
                    raw_block.push_str(next);
                    i += 1;
                    continue;
                }
                break;
            }
            out.push_str(&escape_xml_text_preserving_entities(
                &mermaid_collapse_raw_html_label_text(&raw_block),
            ));
            continue;
        }

        paragraph_lines.push(line);
        i += 1;
    }

    if !paragraph_lines.is_empty() {
        out.push_str(&mermaid_markdown_paragraph_to_xhtml(
            &paragraph_lines.join("\n"),
            markdown_auto_wrap,
        ));
    }

    out
}

/// Heuristic: whether Mermaid's upstream `markdownToHTML()` would wrap the given label into a
/// `<p>...</p>` wrapper when `htmlLabels=true`.
///
/// Mermaid@11.12.2 uses `marked.lexer(markdown)` and only explicitly formats a small subset of
/// token types (`paragraph`, `strong`, `em`, `text`, `html`, `escape`). For unsupported *block*
/// tokens (e.g. ordered/unordered lists, headings, fenced code blocks), Mermaid falls back to
/// emitting the raw Markdown without a surrounding `<p>` wrapper.
///
/// We don't embed `marked` in Rust; instead we match the small set of block starters that would
/// make the top-level token *not* be a paragraph. This keeps our SVG DOM parity stable for cases
/// like `1. foo` (ordered list) where upstream renders the raw text inside `<span class="edgeLabel">`.
pub(crate) fn mermaid_markdown_wants_paragraph_wrap(markdown: &str) -> bool {
    let s = markdown.trim_start();
    if s.is_empty() {
        return true;
    }

    let mut i = 0usize;
    for ch in s.chars() {
        if ch == ' ' && i < 3 {
            i += 1;
            continue;
        }
        break;
    }
    let s = &s[i.min(s.len())..];
    let line = s.lines().next().unwrap_or(s).trim_end();
    !mermaid_markdown_line_starts_raw_block(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_label_fragment_collapses_mixed_list_blocks_like_browser_dom() {
        let input = "Hello\n  - l1\n  - l2";
        assert!(mermaid_markdown_contains_raw_blocks(input));
        assert_eq!(
            mermaid_markdown_to_html_label_fragment(input, true),
            "<p>Hello</p>- l1 - l2"
        );
    }

    #[test]
    fn xhtml_label_fragment_preserves_inline_br_listish_continuations() {
        let input = "Hello<br/>- l1<br/>- l2";
        assert_eq!(
            mermaid_markdown_to_xhtml_label_fragment(input, true),
            "<p>Hello<br/>- l1<br/>- l2</p>"
        );
    }

    #[test]
    fn xhtml_label_fragment_normalizes_raw_br_variants() {
        let input = "Hello<br>world";
        assert_eq!(
            mermaid_markdown_to_xhtml_label_fragment(input, true),
            "<p>Hello<br/>world</p>"
        );
    }

    #[test]
    fn html_label_fragment_preserves_inline_code_literals() {
        let input = "inline: `**not bold**`";
        assert_eq!(
            mermaid_markdown_to_html_label_fragment(input, true),
            "<p>inline: `**not bold**`</p>"
        );
    }

    #[test]
    fn xhtml_label_fragment_preserves_inline_code_literals() {
        let input = "inline: `**not bold**`";
        assert_eq!(
            mermaid_markdown_to_xhtml_label_fragment(input, true),
            "<p>inline: `**not bold**`</p>"
        );
    }

    #[test]
    fn html_label_fragment_reinterprets_partial_star_strong_like_mermaid() {
        let input = "+inline: **bold*";
        assert_eq!(
            mermaid_markdown_to_html_label_fragment(input, true),
            "<p>+inline: *<em>bold</em></p>"
        );
    }

    #[test]
    fn xhtml_label_fragment_reinterprets_partial_star_strong_like_mermaid() {
        let input = "+inline: **bold*";
        assert_eq!(
            mermaid_markdown_to_xhtml_label_fragment(input, true),
            "<p>+inline: *<em>bold</em></p>"
        );
    }
}

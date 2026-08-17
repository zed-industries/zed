//! Mermaid-like Markdown tokenization helpers.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MermaidMarkdownWordType {
    Normal,
    Strong,
    Em,
}

/// Minimal, deterministic subset of Mermaid's `markdownToLines(...)` output.
///
/// This aims to match Mermaid's token boundaries for emphasis/strong delimiters (including `_`
/// behavior) well enough to reproduce upstream SVG-label layout and baseline DOM.
pub(crate) fn mermaid_markdown_to_lines(
    markdown: &str,
    markdown_auto_wrap: bool,
) -> Vec<Vec<(String, MermaidMarkdownWordType)>> {
    fn preprocess_mermaid_markdown(markdown: &str, markdown_auto_wrap: bool) -> String {
        let markdown = markdown.replace("\r\n", "\n");

        // Mermaid preprocessing:
        // - Replace `<br/>` with `\n`
        // - Replace multiple newlines with a single newline
        // - Dedent common indentation
        let mut s = markdown
            .replace("<br/>", "\n")
            .replace("<br />", "\n")
            .replace("<br>", "\n")
            .replace("</br>", "\n")
            .replace("</br/>", "\n")
            .replace("</br />", "\n")
            .replace("</br >", "\n");

        // Collapse multiple consecutive newlines to a single `\n`.
        let mut collapsed = String::with_capacity(s.len());
        let mut prev_nl = false;
        for ch in s.chars() {
            if ch == '\n' {
                if prev_nl {
                    continue;
                }
                prev_nl = true;
                collapsed.push('\n');
            } else {
                prev_nl = false;
                collapsed.push(ch);
            }
        }
        s = collapsed;

        // Dedent: remove the smallest common leading indentation of non-empty lines.
        let lines: Vec<&str> = s.split('\n').collect();
        let mut min_indent: Option<usize> = None;
        for l in &lines {
            if l.trim().is_empty() {
                continue;
            }
            let indent = l
                .chars()
                .take_while(|c| *c == ' ' || *c == '\t')
                .map(|c| if c == '\t' { 4 } else { 1 })
                .sum::<usize>();
            min_indent = Some(min_indent.map_or(indent, |m| m.min(indent)));
        }
        let min_indent = min_indent.unwrap_or(0);
        if min_indent > 0 {
            let mut dedented = String::with_capacity(s.len());
            for (idx, l) in lines.iter().enumerate() {
                if idx > 0 {
                    dedented.push('\n');
                }
                let mut remaining = min_indent;
                let mut it = l.chars().peekable();
                while remaining > 0 {
                    match it.peek().copied() {
                        Some(' ') => {
                            let _ = it.next();
                            remaining = remaining.saturating_sub(1);
                        }
                        Some('\t') => {
                            let _ = it.next();
                            remaining = remaining.saturating_sub(4);
                        }
                        _ => break,
                    }
                }
                for ch in it {
                    dedented.push(ch);
                }
            }
            s = dedented;
        }

        if !markdown_auto_wrap {
            s = s.replace(' ', "&nbsp;");
        }
        s
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum DelimKind {
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

    // Mermaid wraps SVG-label Markdown strings in single backticks; strip to avoid inline-code
    // suppressing `**`/`_` formatting.
    let markdown = markdown
        .strip_prefix('`')
        .and_then(|s| s.strip_suffix('`'))
        .unwrap_or(markdown);

    let pre = preprocess_mermaid_markdown(markdown, markdown_auto_wrap);
    let chars: Vec<char> = pre.chars().collect();

    let mut out: Vec<Vec<(String, MermaidMarkdownWordType)>> = vec![Vec::new()];
    let mut line_idx: usize = 0;

    let mut stack: Vec<MermaidMarkdownWordType> = vec![MermaidMarkdownWordType::Normal];
    let mut word = String::new();
    let mut word_ty = MermaidMarkdownWordType::Normal;
    let mut in_code_span = false;

    fn line_mut(
        out: &mut Vec<Vec<(String, MermaidMarkdownWordType)>>,
        line_idx: usize,
    ) -> &mut Vec<(String, MermaidMarkdownWordType)> {
        if out.len() <= line_idx {
            out.resize_with(line_idx + 1, Vec::new);
        }
        &mut out[line_idx]
    }

    let flush_word = |out: &mut Vec<Vec<(String, MermaidMarkdownWordType)>>,
                      line_idx: &mut usize,
                      word: &mut String,
                      word_ty: MermaidMarkdownWordType| {
        if word.is_empty() {
            return;
        }
        let mut w = std::mem::take(word);
        if w.contains("&#39;") {
            w = w.replace("&#39;", "'");
        }
        line_mut(out, *line_idx).push((w, word_ty));
    };

    let mut i = 0usize;
    while i < chars.len() {
        let ch = chars[i];

        if ch == '\n' {
            flush_word(&mut out, &mut line_idx, &mut word, word_ty);
            word_ty = *stack.last().unwrap_or(&MermaidMarkdownWordType::Normal);
            line_idx += 1;
            out.push(Vec::new());
            i += 1;
            continue;
        }
        if ch == ' ' {
            flush_word(&mut out, &mut line_idx, &mut word, word_ty);
            word_ty = *stack.last().unwrap_or(&MermaidMarkdownWordType::Normal);
            i += 1;
            continue;
        }

        if ch == '<' {
            if let Some(end) = chars[i..].iter().position(|c| *c == '>') {
                let end = i + end;
                let html: String = chars[i..=end].iter().collect();
                flush_word(&mut out, &mut line_idx, &mut word, word_ty);
                if html.eq_ignore_ascii_case("<br>")
                    || html.eq_ignore_ascii_case("<br/>")
                    || html.eq_ignore_ascii_case("<br />")
                    || html.eq_ignore_ascii_case("</br>")
                    || html.eq_ignore_ascii_case("</br/>")
                    || html.eq_ignore_ascii_case("</br />")
                    || html.eq_ignore_ascii_case("</br >")
                {
                    word_ty = *stack.last().unwrap_or(&MermaidMarkdownWordType::Normal);
                    line_idx += 1;
                    out.push(Vec::new());
                } else {
                    line_mut(&mut out, line_idx).push((html, MermaidMarkdownWordType::Normal));
                    word_ty = *stack.last().unwrap_or(&MermaidMarkdownWordType::Normal);
                }
                i = end + 1;
                continue;
            }
        }

        if ch == '`' {
            if word.is_empty() {
                word_ty = *stack.last().unwrap_or(&MermaidMarkdownWordType::Normal);
            }
            word.push(ch);
            in_code_span = !in_code_span;
            i += 1;
            continue;
        }

        if ch == '*' || ch == '_' {
            if in_code_span {
                if word.is_empty() {
                    word_ty = *stack.last().unwrap_or(&MermaidMarkdownWordType::Normal);
                }
                word.push(ch);
                i += 1;
                continue;
            }
            let run_len = if i + 1 < chars.len() && chars[i + 1] == ch {
                2
            } else {
                1
            };
            let kind = if run_len == 2 {
                DelimKind::Strong
            } else {
                DelimKind::Em
            };
            let prev = if i > 0 { Some(chars[i - 1]) } else { None };
            let next = if i + run_len < chars.len() {
                Some(chars[i + run_len])
            } else {
                None
            };
            let (can_open, can_close) = mermaid_delim_can_open_close(ch, prev, next);

            let want_ty = match kind {
                DelimKind::Strong => MermaidMarkdownWordType::Strong,
                DelimKind::Em => MermaidMarkdownWordType::Em,
            };
            let cur_ty = *stack.last().unwrap_or(&MermaidMarkdownWordType::Normal);

            if can_close && cur_ty == want_ty {
                flush_word(&mut out, &mut line_idx, &mut word, word_ty);
                stack.pop();
                word_ty = *stack.last().unwrap_or(&MermaidMarkdownWordType::Normal);
                i += run_len;
                continue;
            }
            if can_open {
                flush_word(&mut out, &mut line_idx, &mut word, word_ty);
                stack.push(want_ty);
                word_ty = *stack.last().unwrap_or(&MermaidMarkdownWordType::Normal);
                i += run_len;
                continue;
            }

            // Treat the delimiter run as literal if it can't open/close. Mermaid's upstream
            // behavior does not reinterpret a failed `__` run as two separate `_` runs (e.g.
            // `a__b` must remain literal, not split into `a_` + `_b_`).
            if word.is_empty() {
                word_ty = *stack.last().unwrap_or(&MermaidMarkdownWordType::Normal);
            }
            for _ in 0..run_len {
                word.push(ch);
            }
            i += run_len;
            continue;
        }

        if word.is_empty() {
            word_ty = *stack.last().unwrap_or(&MermaidMarkdownWordType::Normal);
        }
        word.push(ch);
        i += 1;
    }

    flush_word(&mut out, &mut line_idx, &mut word, word_ty);
    if out.is_empty() {
        out.push(Vec::new());
    }
    while out.last().is_some_and(|l| l.is_empty()) && out.len() > 1 {
        out.pop();
    }
    out
}

pub(crate) fn mermaid_markdown_contains_html_tags(markdown: &str) -> bool {
    pulldown_cmark::Parser::new_ext(
        markdown,
        pulldown_cmark::Options::ENABLE_TABLES
            | pulldown_cmark::Options::ENABLE_STRIKETHROUGH
            | pulldown_cmark::Options::ENABLE_TASKLISTS,
    )
    .any(|ev| {
        matches!(
            ev,
            pulldown_cmark::Event::Html(_) | pulldown_cmark::Event::InlineHtml(_)
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn underscore_delimiters_match_mermaid() {
        use MermaidMarkdownWordType::*;

        assert_eq!(
            mermaid_markdown_to_lines("`a__b`", true),
            vec![vec![("a__b".to_string(), Normal)]]
        );
        assert_eq!(
            mermaid_markdown_to_lines("`_a_b_`", true),
            vec![vec![("a_b".to_string(), Em)]]
        );
        assert_eq!(
            mermaid_markdown_to_lines("`_a__b_`", true),
            vec![vec![("a__b".to_string(), Em)]]
        );
        assert_eq!(
            mermaid_markdown_to_lines("`__a__`", true),
            vec![vec![("a".to_string(), Strong)]]
        );
    }

    #[test]
    fn inline_code_suppresses_emphasis_delimiters() {
        use MermaidMarkdownWordType::*;

        // Mermaid CLI baselines (class diagram HTML labels) preserve backticks and do not
        // interpret `**...**` inside them as strong/emphasis.
        assert_eq!(
            mermaid_markdown_to_lines("inline: `**not bold**`", true),
            vec![vec![
                ("inline:".to_string(), Normal),
                ("`**not".to_string(), Normal),
                ("bold**`".to_string(), Normal),
            ]]
        );
    }

    #[test]
    fn html_tags_after_newline_stay_on_current_markdown_line() {
        use MermaidMarkdownWordType::*;

        assert_eq!(
            mermaid_markdown_to_lines("alpha\n<strong>bravo</strong>", true),
            vec![
                vec![("alpha".to_string(), Normal)],
                vec![
                    ("<strong>".to_string(), Normal),
                    ("bravo".to_string(), Normal),
                    ("</strong>".to_string(), Normal),
                ],
            ]
        );
    }
}

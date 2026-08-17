use super::TitleKind;

pub(super) fn title_kind_str(kind: &TitleKind) -> &'static str {
    match kind {
        TitleKind::Text => "text",
        TitleKind::String => "string",
        TitleKind::Markdown => "markdown",
    }
}

pub(super) fn unquote(s: &str) -> String {
    let bytes = s.as_bytes();
    if bytes.len() >= 2 && bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"' {
        return s[1..s.len() - 1].to_string();
    }
    if bytes.len() >= 2 && bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\'' {
        return s[1..s.len() - 1].to_string();
    }
    s.to_string()
}

pub(super) fn parse_label_text(raw: &str) -> (String, TitleKind) {
    let trimmed = raw.trim();
    let quoted = (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''));
    let unquoted = if quoted {
        // Mermaid flowchart quoted labels are treated as raw text with surrounding quotes stripped.
        // Do not interpret backslash escapes here: fixtures rely on sequences like `\\n`, `\\t`,
        // `\\nabla`, and Windows paths (e.g. `C:\\Temp\\...`) being preserved verbatim.
        unquote(trimmed)
    } else {
        trimmed.to_string()
    };

    let (no_backticks, is_markdown) = strip_wrapping_backticks(unquoted.trim());
    if is_markdown {
        return (no_backticks, TitleKind::Markdown);
    }
    if quoted {
        return (unquoted, TitleKind::String);
    }
    (unquoted, TitleKind::Text)
}

pub(super) fn parse_edge_label_text(raw: &str) -> (String, TitleKind) {
    let trimmed = raw.trim();
    let quoted = (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''));

    if quoted {
        return parse_label_text(trimmed);
    }

    // Mermaid flowchart edge labels only enter Markdown-string mode via the lexer's `MD_STR`
    // token, i.e. a double-quoted string whose payload is wrapped in backticks:
    //   -- "`edge **label**`" -->
    //
    // Bare pipe labels like `-->|`edge **label**`|` keep the backticks literally and stay `text`.
    (trimmed.to_string(), TitleKind::Text)
}

pub(super) fn strip_wrapping_backticks(s: &str) -> (String, bool) {
    let trimmed = s.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('`') && trimmed.ends_with('`') {
        return (trimmed[1..trimmed.len() - 1].to_string(), true);
    }
    (trimmed.to_string(), false)
}

#[cfg(test)]
mod tests {
    use super::{parse_edge_label_text, parse_label_text};
    use crate::diagrams::flowchart::TitleKind;

    #[test]
    fn parse_label_text_keeps_backslashes_in_string_labels() {
        let (text, kind) = parse_label_text(r#""Path: C:\\Temp\\merman\\out.svg (Windows-style)""#);
        assert_eq!(kind, TitleKind::String);
        assert_eq!(text, r#"Path: C:\\Temp\\merman\\out.svg (Windows-style)"#);
    }

    #[test]
    fn parse_label_text_does_not_treat_tex_commands_as_escapes() {
        let (text, kind) = parse_label_text(r#""$$\nabla\therefore\alpha$$""#);
        assert_eq!(kind, TitleKind::String);
        assert_eq!(text, r#"$$\nabla\therefore\alpha$$"#);
    }

    #[test]
    fn parse_edge_label_text_keeps_unquoted_backticks_literal() {
        let (text, kind) =
            parse_edge_label_text(r#"`This is **bold** </br>and <strong>strong</strong>`"#);
        assert_eq!(kind, TitleKind::Text);
        assert_eq!(
            text,
            r#"`This is **bold** </br>and <strong>strong</strong>`"#
        );
    }

    #[test]
    fn parse_edge_label_text_keeps_unquoted_partial_markdown_literal() {
        let (text, kind) = parse_edge_label_text(r#"`**bold*`"#);
        assert_eq!(kind, TitleKind::Text);
        assert_eq!(text, r#"`**bold*`"#);
    }

    #[test]
    fn parse_edge_label_text_supports_quoted_markdown_strings() {
        let (text, kind) = parse_edge_label_text(r#""`Bold **edge label**`""#);
        assert_eq!(kind, TitleKind::Markdown);
        assert_eq!(text, r#"Bold **edge label**"#);
    }
}

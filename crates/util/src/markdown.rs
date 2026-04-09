use std::fmt::{Display, Formatter};

/// Generates a URL-friendly slug from heading text (e.g. "Hello World" → "hello-world").
pub fn generate_heading_slug(text: &str) -> String {
    text.trim()
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                Some(c.to_lowercase().next().unwrap_or(c))
            } else if c == ' ' {
                Some('-')
            } else {
                None
            }
        })
        .collect()
}

/// Returns true if the URL starts with a URI scheme (RFC 3986 §3.1).
fn has_uri_scheme(url: &str) -> bool {
    let mut chars = url.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    for c in chars {
        if c == ':' {
            return true;
        }
        if !(c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.') {
            return false;
        }
    }
    false
}

/// Splits a relative URL into its path and `#fragment` parts.
/// Absolute URLs are returned as-is with no fragment.
pub fn split_local_url_fragment(url: &str) -> (&str, Option<&str>) {
    if has_uri_scheme(url) {
        return (url, None);
    }
    match url.find('#') {
        Some(pos) => {
            let path = &url[..pos];
            let fragment = &url[pos + 1..];
            (
                path,
                if fragment.is_empty() {
                    None
                } else {
                    Some(fragment)
                },
            )
        }
        None => (url, None),
    }
}

/// Indicates that the wrapped `String` is markdown text.
#[derive(Debug, Clone)]
pub struct MarkdownString(pub String);

impl Display for MarkdownString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Escapes markdown special characters in markdown text blocks. Markdown code blocks follow
/// different rules and `MarkdownInlineCode` or `MarkdownCodeBlock` should be used in that case.
///
/// Also escapes the following markdown extensions:
///
/// * `^` for superscripts
/// * `$` for inline math
/// * `~` for strikethrough
///
/// Escape of some characters is unnecessary, because while they are involved in markdown syntax,
/// the other characters involved are escaped:
///
/// * `!`, `]`, `(`, and `)` are used in link syntax, but `[` is escaped so these are parsed as
///   plaintext.
///
/// * `;` is used in HTML entity syntax, but `&` is escaped, so they are parsed as plaintext.
///
/// TODO: There is one escape this doesn't do currently. Period after numbers at the start of the
/// line (`[0-9]*\.`) should also be escaped to avoid it being interpreted as a list item.
pub struct MarkdownEscaped<'a>(pub &'a str);

/// Implements `Display` to format markdown inline code (wrapped in backticks), handling code that
/// contains backticks and spaces. All whitespace is treated as a single space character. For text
/// that does not contain whitespace other than ' ', this escaping roundtrips through
/// pulldown-cmark.
///
/// When used in tables, `|` should be escaped like `\|` in the text provided to this function.
pub struct MarkdownInlineCode<'a>(pub &'a str);

/// Implements `Display` to format markdown code blocks, wrapped in 3 or more backticks as needed.
pub struct MarkdownCodeBlock<'a> {
    pub tag: &'a str,
    pub text: &'a str,
}

impl Display for MarkdownEscaped<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut start_of_unescaped = None;
        for (ix, c) in self.0.char_indices() {
            match c {
                // Always escaped.
                '\\' | '`' | '*' | '_' | '[' | '^' | '$' | '~' | '&' |
                // TODO: these only need to be escaped when they are the first non-whitespace
                // character of the line of a block. There should probably be both an `escape_block`
                // which does this and an `escape_inline` method which does not escape these.
                '#' | '+' | '=' | '-' => {
                    match start_of_unescaped {
                        None => {}
                        Some(start_of_unescaped) => {
                            write!(formatter, "{}", &self.0[start_of_unescaped..ix])?;
                        }
                    }
                    write!(formatter, "\\")?;
                    // Can include this char in the "unescaped" text since a
                    // backslash was just emitted.
                    start_of_unescaped = Some(ix);
                }
                // Escaped since `<` is used in opening HTML tags. `&lt;` is used since Markdown
                // supports HTML entities, and this allows the text to be used directly in HTML.
                '<' => {
                    match start_of_unescaped {
                        None => {}
                        Some(start_of_unescaped) => {
                            write!(formatter, "{}", &self.0[start_of_unescaped..ix])?;
                        }
                    }
                    write!(formatter, "&lt;")?;
                    start_of_unescaped = None;
                }
                // Escaped since `>` is used for blockquotes. `&gt;` is used since Markdown supports
                // HTML entities, and this allows the text to be used directly in HTML.
                '>' => {
                    match start_of_unescaped {
                        None => {}
                        Some(start_of_unescaped) => {
                            write!(formatter, "{}", &self.0[start_of_unescaped..ix])?;
                        }
                    }
                    write!(formatter, "&gt;")?;
                    start_of_unescaped = None;
                }
                _ => {
                    if start_of_unescaped.is_none() {
                        start_of_unescaped = Some(ix);
                    }
                }
            }
        }
        if let Some(start_of_unescaped) = start_of_unescaped {
            write!(formatter, "{}", &self.0[start_of_unescaped..])?;
        }
        Ok(())
    }
}

impl Display for MarkdownInlineCode<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        // Apache License 2.0, same as this crate.
        //
        // Copied from `pulldown-cmark-to-cmark-20.0.0` with modifications:
        //
        // * Handling of all whitespace. pulldown-cmark-to-cmark is anticipating
        // `Code` events parsed by pulldown-cmark.
        //
        // https://github.com/Byron/pulldown-cmark-to-cmark/blob/3c850de2d3d1d79f19ca5f375e1089a653cf3ff7/src/lib.rs#L290

        let mut all_whitespace = true;
        let text = self
            .0
            .chars()
            .map(|c| {
                if c.is_whitespace() {
                    ' '
                } else {
                    all_whitespace = false;
                    c
                }
            })
            .collect::<String>();

        // When inline code has leading and trailing ' ' characters, additional space is needed
        // to escape it, unless all characters are space.
        if all_whitespace {
            write!(formatter, "`{text}`")
        } else {
            // More backticks are needed to delimit the inline code than the maximum number of
            // backticks in a consecutive run.
            let backticks = "`".repeat(count_max_consecutive_chars(&text, '`') + 1);
            let space = match text.as_bytes() {
                &[b'`', ..] | &[.., b'`'] => " ", // Space needed to separate backtick.
                &[b' ', .., b' '] => " ",         // Space needed to escape inner space.
                _ => "",                          // No space needed.
            };
            write!(formatter, "{backticks}{space}{text}{space}{backticks}")
        }
    }
}

impl Display for MarkdownCodeBlock<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let tag = self.tag;
        let text = self.text;
        let backticks = "`".repeat(3.max(count_max_consecutive_chars(text, '`') + 1));
        write!(formatter, "{backticks}{tag}\n{text}\n{backticks}\n")
    }
}

// Copied from `pulldown-cmark-to-cmark-20.0.0` with changed names.
// https://github.com/Byron/pulldown-cmark-to-cmark/blob/3c850de2d3d1d79f19ca5f375e1089a653cf3ff7/src/lib.rs#L1063
// Apache License 2.0, same as this code.
fn count_max_consecutive_chars(text: &str, search: char) -> usize {
    let mut in_search_chars = false;
    let mut max_count = 0;
    let mut cur_count = 0;

    for ch in text.chars() {
        if ch == search {
            cur_count += 1;
            in_search_chars = true;
        } else if in_search_chars {
            max_count = max_count.max(cur_count);
            cur_count = 0;
            in_search_chars = false;
        }
    }
    max_count.max(cur_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_escaped() {
        let input = r#"
        # Heading

        Another heading
        ===

        Another heading variant
        ---

        Paragraph with [link](https://example.com) and `code`, *emphasis*, and ~strikethrough~.

        ```
        code block
        ```

        List with varying leaders:
          - Item 1
          * Item 2
          + Item 3

        Some math:  $`\sqrt{3x-1}+(1+x)^2`$

        HTML entity: &nbsp;
        "#;

        let expected = r#"
        \# Heading

        Another heading
        \=\=\=

        Another heading variant
        \-\-\-

        Paragraph with \[link](https://example.com) and \`code\`, \*emphasis\*, and \~strikethrough\~.

        \`\`\`
        code block
        \`\`\`

        List with varying leaders:
          \- Item 1
          \* Item 2
          \+ Item 3

        Some math:  \$\`\\sqrt{3x\-1}\+(1\+x)\^2\`\$

        HTML entity: \&nbsp;
        "#;

        assert_eq!(MarkdownEscaped(input).to_string(), expected);
    }

    #[test]
    fn test_markdown_inline_code() {
        assert_eq!(MarkdownInlineCode(" ").to_string(), "` `");
        assert_eq!(MarkdownInlineCode("text").to_string(), "`text`");
        assert_eq!(MarkdownInlineCode("text ").to_string(), "`text `");
        assert_eq!(MarkdownInlineCode(" text ").to_string(), "`  text  `");
        assert_eq!(MarkdownInlineCode("`").to_string(), "`` ` ``");
        assert_eq!(MarkdownInlineCode("``").to_string(), "``` `` ```");
        assert_eq!(MarkdownInlineCode("`text`").to_string(), "`` `text` ``");
        assert_eq!(
            MarkdownInlineCode("some `text` no leading or trailing backticks").to_string(),
            "``some `text` no leading or trailing backticks``"
        );
    }

    #[test]
    fn test_count_max_consecutive_chars() {
        assert_eq!(
            count_max_consecutive_chars("``a```b``", '`'),
            3,
            "the highest seen consecutive segment of backticks counts"
        );
        assert_eq!(
            count_max_consecutive_chars("```a``b`", '`'),
            3,
            "it can't be downgraded later"
        );
    }

    #[test]
    fn test_split_local_url_fragment() {
        assert_eq!(split_local_url_fragment("#heading"), ("", Some("heading")));
        assert_eq!(
            split_local_url_fragment("./file.md#heading"),
            ("./file.md", Some("heading"))
        );
        assert_eq!(split_local_url_fragment("./file.md"), ("./file.md", None));
        assert_eq!(
            split_local_url_fragment("https://example.com#frag"),
            ("https://example.com#frag", None)
        );
        assert_eq!(
            split_local_url_fragment("mailto:user@example.com"),
            ("mailto:user@example.com", None)
        );
        assert_eq!(split_local_url_fragment("#"), ("", None));
        assert_eq!(
            split_local_url_fragment("../other.md#section"),
            ("../other.md", Some("section"))
        );
        assert_eq!(
            split_local_url_fragment("123:not-a-scheme#frag"),
            ("123:not-a-scheme", Some("frag"))
        );
    }

    #[test]
    fn test_generate_heading_slug() {
        assert_eq!(generate_heading_slug("Hello World"), "hello-world");
        assert_eq!(generate_heading_slug("Hello  World"), "hello--world");
        assert_eq!(generate_heading_slug("Hello-World"), "hello-world");
        assert_eq!(
            generate_heading_slug("Some **bold** text"),
            "some-bold-text"
        );
        assert_eq!(generate_heading_slug("Let's try with Ü"), "lets-try-with-ü");
        assert_eq!(
            generate_heading_slug("heading with 123 numbers"),
            "heading-with-123-numbers"
        );
        assert_eq!(
            generate_heading_slug("What about (parens)?"),
            "what-about-parens"
        );
        assert_eq!(
            generate_heading_slug("  leading spaces  "),
            "leading-spaces"
        );
    }
}

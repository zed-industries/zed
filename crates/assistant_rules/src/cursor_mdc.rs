use crate::{RulesContent, WhenIncluded};
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingFrontmatter,
}

/// Parses Cursor *.mdc rule files which consist of frontmatter followed by content. While the
/// frontmatter looks like YAML, it is not. Instead each field is on a single line and has no
/// escaping rules (there is no way to have a file glob that involves ","), and there are no
/// parse errors.
pub fn parse(source: &str) -> Result<RulesContent, ParseError> {
    let mut line_ranges = line_ranges(source);
    let first_line_range = line_ranges.next().unwrap();
    if !is_delimiter_line(source, first_line_range.clone()) {
        return Err(ParseError::MissingFrontmatter);
    }

    let mut description = None;
    let mut globs = None;
    let mut always_apply = false;
    let mut text_start = None;
    for line_range in line_ranges {
        if is_delimiter_line(source, line_range.clone()) {
            text_start = Some(line_range.end + 1);
            break;
        }
        let line = source[line_range].trim_start();
        if let Some(value) = parse_field(line, "description") {
            description = Some(value.to_string());
        }
        if let Some(value) = parse_field(line, "globs") {
            globs = Some(value);
        }
        if let Some(value) = parse_field(line, "alwaysApply") {
            always_apply = value == "true";
        }
    }
    let Some(text_start) = text_start else {
        return Err(ParseError::MissingFrontmatter);
    };

    let when_included = if always_apply {
        WhenIncluded::Always
    } else if let Some(globs) = globs {
        // These are not trimmed as they do actually match spaces, even though the Cursor UI doesn't
        // allow entering spaces.
        WhenIncluded::AutoAttached {
            globs: globs
                .split(',')
                .map(|glob| glob.to_string())
                .collect::<Vec<_>>(),
        }
    } else if description.is_some() {
        WhenIncluded::AgentRequested
    } else {
        WhenIncluded::Manual
    };

    let text = source.get(text_start..).unwrap_or_default().to_string();

    Ok(RulesContent {
        when_included,
        description,
        text,
    })
}

fn parse_field<'a>(line: &'a str, name: &'a str) -> Option<&'a str> {
    line.strip_prefix(name)
        .and_then(|suffix| suffix.trim().strip_prefix(":").map(str::trim))
        .filter(|value| !value.is_empty())
}

fn is_delimiter_line(source: &str, line_range: Range<usize>) -> bool {
    const FRONTMATTER_DELIMITER: &str = "---";
    line_range.end - line_range.start >= FRONTMATTER_DELIMITER.len()
        && &source[line_range.start..line_range.start + FRONTMATTER_DELIMITER.len()]
            == FRONTMATTER_DELIMITER
        && source[line_range.start + FRONTMATTER_DELIMITER.len()..line_range.end]
            .chars()
            .all(char::is_whitespace)
}

fn line_ranges(text: &str) -> impl Iterator<Item = Range<usize>> + '_ {
    let mut line_start = 0;
    text.match_indices('\n')
        .map(move |(offset, _)| {
            let range = line_start..offset;
            line_start = offset + 1;
            range
        })
        .chain(
            std::iter::once_with(move || {
                if line_start < text.len() || line_start == 0 {
                    Some(line_start..text.len())
                } else {
                    None
                }
            })
            .flatten(),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_always_included() {
        let text = indoc! {r#"
            ---
            description: some description
            globs: *.rs
            alwaysApply: true
            ---
            Rule body text"#};

        let rule = parse(text).unwrap();
        assert_eq!(
            rule,
            RulesContent {
                when_included: WhenIncluded::Always,
                description: Some("some description".into()),
                text: "Rule body text".into(),
            }
        );
    }

    #[test]
    fn test_parse_auto_attached() {
        let text = indoc! {r#"
            ---
            globs: *.rs, spaces in glob ,*.md
            ---
            Rule body text"#};

        let rule = parse(text).unwrap();
        assert_eq!(
            rule,
            RulesContent {
                when_included: WhenIncluded::AutoAttached {
                    globs: vec!["*.rs".into(), " spaces in glob ".into(), "*.md".into()]
                },
                description: None,
                text: "Rule body text".into(),
            }
        );
    }

    #[test]
    fn test_parse_rule_type_agent_requested() {
        let text = indoc! {r#"
            ---
            description: some description
            ---
            Rule body text"#};

        let rule = parse(text).unwrap();
        assert_eq!(
            rule,
            RulesContent {
                when_included: WhenIncluded::AgentRequested,
                description: Some("some description".into()),
                text: "Rule body text".into(),
            }
        );
    }

    #[test]
    fn test_parse_rule_type_manual() {
        let text = indoc! {r#"
            ---
            alwaysApply: false
            ---
            Rule body text"#};

        let rule = parse(text).unwrap();
        assert_eq!(
            rule,
            RulesContent {
                when_included: WhenIncluded::Manual,
                description: None,
                text: "Rule body text".into(),
            }
        );
    }

    #[test]
    fn test_parse_rule_whitespace() {
        // Experimentally, spaces are allowed before fields and ":", but not before "---".
        let text = indoc! {r#"
            ---
            globs        :   *.rs
            description  :   some description
            ---
            Rule body text"#};

        let rule = parse(text).unwrap();
        assert_eq!(
            rule,
            RulesContent {
                when_included: WhenIncluded::AutoAttached {
                    globs: vec!["*.rs".into()]
                },
                description: Some("some description".into()),
                text: "Rule body text".into(),
            }
        );
    }

    #[test]
    fn test_parse_no_frontmatter() {
        let text = "Invalid content without frontmatter";
        assert_eq!(parse(text), Err(ParseError::MissingFrontmatter));
    }

    #[test]
    fn test_parse_invalid_end_delimeter() {
        let text = indoc! {r#"
            ---
            description  :   some description
                ---
            Invalid end delimeter
            "#};
        assert_eq!(parse(text), Err(ParseError::MissingFrontmatter));
    }
}

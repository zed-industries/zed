//! Implements parsing for shell glob and extglob patterns.

use crate::error;

/// Represents the kind of an extended glob.
pub enum ExtendedGlobKind {
    /// The `+` extended glob; matches one or more occurrences of the inner pattern.
    Plus,
    /// The `@` extended glob; allows matching an alternation of inner patterns.
    At,
    /// The `!` extended glob; matches the negation of the inner pattern.
    Exclamation,
    /// The `?` extended glob; matches zero or one occurrence of the inner pattern.
    Question,
    /// The `*` extended glob; matches zero or more occurrences of the inner pattern.
    Star,
}

/// Converts a shell pattern to a regular expression string.
///
/// # Arguments
///
/// * `pattern` - The shell pattern to convert.
/// * `enable_extended_globbing` - Whether to enable extended globbing (extglob).
pub fn pattern_to_regex_str(
    pattern: &str,
    enable_extended_globbing: bool,
) -> Result<String, error::WordParseError> {
    let regex_str = pattern_to_regex_translator::pattern(pattern, enable_extended_globbing)
        .map_err(|e| error::WordParseError::Pattern(e.into()))?;
    Ok(regex_str)
}

peg::parser! {
    grammar pattern_to_regex_translator(enable_extended_globbing: bool) for str {
        pub(crate) rule pattern() -> String =
            pieces:(pattern_piece()*) {
                pieces.join("")
            }

        rule pattern_piece() -> String =
            escape_sequence() /
            bracket_expression() /
            extglob_enabled() s:extended_glob_pattern() { s } /
            wildcard() /
            [c if regex_char_needs_escaping(c)] {
                let mut s = '\\'.to_string();
                s.push(c);
                s
            } /
            [c] { c.to_string() }

        rule escape_sequence() -> String =
            sequence:$(['\\'] [c if regex_char_needs_escaping(c)]) { sequence.to_owned() } /
            ['\\'] [c] { c.to_string() }

        rule bracket_expression() -> String =
            "[" invert:(invert_char()?) members:bracket_member()+ "]" {
                let mut members = members;
                if invert.is_some() {
                    members.insert(0, String::from("^"));
                }

                std::format!("[{}]", members.join(""))
            }

        rule invert_char() -> bool =
            ['!' | '^'] { true }

        rule bracket_member() -> String =
            char_class_expression() /
            char_range() /
            char_list()

        rule char_class_expression() -> String =
            e:$("[:" char_class() ":]") { e.to_owned() }

        rule char_class() =
            "alnum" / "alpha" / "blank" / "cntrl" / "digit" / "graph" / "lower" / "print" / "punct" / "space" / "upper"/ "xdigit"

        rule char_range() -> String =
            range:$([_] "-" [c if c != ']']) { range.to_owned() }

        rule char_list() -> String =
            chars:$([c if c != ']']+) { escape_char_class_char_list(chars) }

        rule wildcard() -> String =
            "?" { String::from(".") } /
            "*" { String::from(".*") }

        rule extglob_enabled() -> () =
            &[_] {? if enable_extended_globbing { Ok(()) } else { Err("extglob disabled") } }

        pub(crate) rule extended_glob_pattern() -> String =
            kind:extended_glob_prefix() "(" branches:extended_glob_body() ")" {
                let mut s = String::new();

                // fancy_regex uses ?! to indicate a negative lookahead.
                if matches!(kind, ExtendedGlobKind::Exclamation) {
                    if !branches.is_empty() {
                        s.push_str("(?:(?!");
                        s.push_str(&branches.join("|"));
                        s.push_str(").*|(?>");
                        s.push_str(&branches.join("|"));
                        s.push_str(").+?|)");
                    } else {
                        s.push_str("(?:.+)");
                    }
                } else {
                    s.push('(');
                    s.push_str(&branches.join("|"));
                    s.push(')');

                    match kind {
                        ExtendedGlobKind::Plus => s.push('+'),
                        ExtendedGlobKind::Question => s.push('?'),
                        ExtendedGlobKind::Star => s.push('*'),
                        ExtendedGlobKind::At | ExtendedGlobKind::Exclamation => (),
                    }
                }

                s
            }

        rule extended_glob_prefix() -> ExtendedGlobKind =
            "+" { ExtendedGlobKind::Plus } /
            "@" { ExtendedGlobKind::At } /
            "!" { ExtendedGlobKind::Exclamation } /
            "?" { ExtendedGlobKind::Question } /
            "*" { ExtendedGlobKind::Star }

        pub(crate) rule extended_glob_body() -> Vec<String> =
            // Cover case with *no* branches.
            &[')'] { vec![] } /
            // Otherwise, look for branches separated by '|'.
            extended_glob_branch() ** "|"

        rule extended_glob_branch() -> String =
            // Cover case of empty branch.
            &['|' | ')'] { String::new() } /
            pieces:(!['|' | ')'] piece:pattern_piece() { piece })+ {
                pieces.join("")
            }
    }
}

/// Returns whether or not a given character needs to be escaped in a regular expression.
///
/// # Arguments
///
/// * `c` - The character to check.
pub const fn regex_char_needs_escaping(c: char) -> bool {
    matches!(
        c,
        '[' | ']' | '(' | ')' | '{' | '}' | '*' | '?' | '.' | '+' | '^' | '$' | '|' | '\\'
    )
}

fn escape_char_class_char_list(s: &str) -> String {
    s.replace('[', r"\[")
}

#[cfg(test)]
#[expect(clippy::panic_in_result_fn)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_bracket_exprs() -> Result<()> {
        assert_eq!(pattern_to_regex_str("[a-z]", true)?, "[a-z]");
        assert_eq!(pattern_to_regex_str("[abc]", true)?, "[abc]");
        assert_eq!(pattern_to_regex_str(r"[\(]", true)?, r"[\(]");
        assert_eq!(pattern_to_regex_str(r"[(]", true)?, "[(]");
        assert_eq!(pattern_to_regex_str("[[:digit:]]", true)?, "[[:digit:]]");
        assert_eq!(pattern_to_regex_str(r"[-(),!]*", true)?, r"[-(),!].*");
        assert_eq!(pattern_to_regex_str(r"[-\(\),\!]*", true)?, r"[-\(\),\!].*");
        Ok(())
    }

    #[test]
    fn test_extended_glob() -> Result<()> {
        assert_eq!(
            pattern_to_regex_translator::extended_glob_pattern("@(a|b)", true)?,
            "(a|b)"
        );

        assert_eq!(
            pattern_to_regex_translator::extended_glob_pattern("@(|a)", true)?,
            "(|a)"
        );

        assert_eq!(
            pattern_to_regex_translator::extended_glob_pattern("@(|)", true)?,
            "(|)"
        );

        assert_eq!(
            pattern_to_regex_translator::extended_glob_body("ab|ac", true)?,
            vec!["ab", "ac"],
        );

        assert_eq!(
            pattern_to_regex_translator::extended_glob_pattern("*(ab|ac)", true)?,
            "(ab|ac)*"
        );

        assert_eq!(
            pattern_to_regex_translator::extended_glob_body("", true)?,
            Vec::<String>::new(),
        );

        Ok(())
    }
}

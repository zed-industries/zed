//! Utilities for manipulating C/C++ comments.

/// The type of a comment.
#[derive(Debug, PartialEq, Eq)]
enum Kind {
    /// A `///` comment, or something of the like.
    /// All lines in a comment should start with the same symbol.
    SingleLines,
    /// A `/**` comment, where each other line can start with `*` and the
    /// entire block ends with `*/`.
    MultiLine,
}

/// Preprocesses a C/C++ comment so that it is a valid Rust comment.
pub(crate) fn preprocess(comment: &str) -> String {
    match kind(comment) {
        Some(Kind::SingleLines) => preprocess_single_lines(comment),
        Some(Kind::MultiLine) => preprocess_multi_line(comment),
        None => comment.to_owned(),
    }
}

/// Gets the kind of the doc comment, if it is one.
fn kind(comment: &str) -> Option<Kind> {
    if comment.starts_with("/*") {
        Some(Kind::MultiLine)
    } else if comment.starts_with("//") {
        Some(Kind::SingleLines)
    } else {
        None
    }
}

/// Preprocesses multiple single line comments.
///
/// Handles lines starting with both `//` and `///`.
fn preprocess_single_lines(comment: &str) -> String {
    debug_assert!(comment.starts_with("//"), "comment is not single line");

    let lines: Vec<_> = comment
        .lines()
        .map(|l| l.trim().trim_start_matches('/'))
        .collect();
    lines.join("\n")
}

fn preprocess_multi_line(comment: &str) -> String {
    let comment = comment
        .trim_start_matches('/')
        .trim_end_matches('/')
        .trim_end_matches('*');

    // Strip any potential `*` characters preceding each line.
    let mut lines: Vec<_> = comment
        .lines()
        .map(|line| line.trim().trim_start_matches('*').trim_start_matches('!'))
        .skip_while(|line| line.trim().is_empty()) // Skip the first empty lines.
        .collect();

    // Remove the trailing line corresponding to the `*/`.
    if lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }

    lines.join("\n")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn picks_up_single_and_multi_line_doc_comments() {
        assert_eq!(kind("/// hello"), Some(Kind::SingleLines));
        assert_eq!(kind("/** world */"), Some(Kind::MultiLine));
    }

    #[test]
    fn processes_single_lines_correctly() {
        assert_eq!(preprocess("///"), "");
        assert_eq!(preprocess("/// hello"), " hello");
        assert_eq!(preprocess("// hello"), " hello");
        assert_eq!(preprocess("//    hello"), "    hello");
    }

    #[test]
    fn processes_multi_lines_correctly() {
        assert_eq!(preprocess("/**/"), "");

        assert_eq!(
            preprocess("/** hello \n * world \n * foo \n */"),
            " hello\n world\n foo"
        );

        assert_eq!(
            preprocess("/**\nhello\n*world\n*foo\n*/"),
            "hello\nworld\nfoo"
        );
    }
}

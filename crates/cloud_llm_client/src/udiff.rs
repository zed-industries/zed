use std::{borrow::Cow, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum DiffLine<'a> {
    OldPath { path: Cow<'a, str> },
    NewPath { path: Cow<'a, str> },
    HunkHeader(Option<HunkLocation>),
    Context(&'a str),
    Deletion(&'a str),
    Addition(&'a str),
    Garbage(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct HunkLocation {
    start_line_old: u32,
    count_old: u32,
    start_line_new: u32,
    count_new: u32,
}

impl<'a> DiffLine<'a> {
    pub fn parse(line: &'a str) -> Self {
        Self::try_parse(line).unwrap_or(Self::Garbage(line))
    }

    fn try_parse(line: &'a str) -> Option<Self> {
        if let Some(header) = line.strip_prefix("---").and_then(eat_required_whitespace) {
            let path = parse_header_path("a/", header);
            Some(Self::OldPath { path })
        } else if let Some(header) = line.strip_prefix("+++").and_then(eat_required_whitespace) {
            Some(Self::NewPath {
                path: parse_header_path("b/", header),
            })
        } else if let Some(header) = line.strip_prefix("@@").and_then(eat_required_whitespace) {
            if header.starts_with("...") {
                return Some(Self::HunkHeader(None));
            }

            let (start_line_old, header) = header.strip_prefix('-')?.split_once(',')?;
            let mut parts = header.split_ascii_whitespace();
            let count_old = parts.next()?;
            let (start_line_new, count_new) = parts.next()?.strip_prefix('+')?.split_once(',')?;

            Some(Self::HunkHeader(Some(HunkLocation {
                start_line_old: start_line_old.parse::<u32>().ok()?.saturating_sub(1),
                count_old: count_old.parse().ok()?,
                start_line_new: start_line_new.parse::<u32>().ok()?.saturating_sub(1),
                count_new: count_new.parse().ok()?,
            })))
        } else if let Some(deleted_header) = line.strip_prefix("-") {
            Some(Self::Deletion(deleted_header))
        } else if line.is_empty() {
            Some(Self::Context(""))
        } else if let Some(context) = line.strip_prefix(" ") {
            Some(Self::Context(context))
        } else {
            Some(Self::Addition(line.strip_prefix("+")?))
        }
    }
}

impl<'a> Display for DiffLine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiffLine::OldPath { path } => write!(f, "--- {path}"),
            DiffLine::NewPath { path } => write!(f, "+++ {path}"),
            DiffLine::HunkHeader(Some(hunk_location)) => {
                write!(
                    f,
                    "@@ -{},{} +{},{} @@",
                    hunk_location.start_line_old + 1,
                    hunk_location.count_old,
                    hunk_location.start_line_new + 1,
                    hunk_location.count_new
                )
            }
            DiffLine::HunkHeader(None) => write!(f, "@@ ... @@"),
            DiffLine::Context(content) => write!(f, " {content}"),
            DiffLine::Deletion(content) => write!(f, "-{content}"),
            DiffLine::Addition(content) => write!(f, "+{content}"),
            DiffLine::Garbage(line) => write!(f, "{line}"),
        }
    }
}

fn parse_header_path<'a>(strip_prefix: &'static str, header: &'a str) -> Cow<'a, str> {
    if !header.contains(['"', '\\']) {
        let path = header.split_ascii_whitespace().next().unwrap_or(header);
        return Cow::Borrowed(path.strip_prefix(strip_prefix).unwrap_or(path));
    }

    let mut path = String::with_capacity(header.len());
    let mut in_quote = false;
    let mut chars = header.chars().peekable();
    let mut strip_prefix = Some(strip_prefix);

    while let Some(char) = chars.next() {
        if char == '"' {
            in_quote = !in_quote;
        } else if char == '\\' {
            let Some(&next_char) = chars.peek() else {
                break;
            };
            chars.next();
            path.push(next_char);
        } else if char.is_ascii_whitespace() && !in_quote {
            break;
        } else {
            path.push(char);
        }

        if let Some(prefix) = strip_prefix
            && path == prefix
        {
            strip_prefix.take();
            path.clear();
        }
    }

    Cow::Owned(path)
}

fn eat_required_whitespace(header: &str) -> Option<&str> {
    let trimmed = header.trim_ascii_start();

    if trimmed.len() == header.len() {
        None
    } else {
        Some(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_lines_simple() {
        let input = indoc! {"
            diff --git a/text.txt b/text.txt
            index 86c770d..a1fd855 100644
            --- a/file.txt
            +++ b/file.txt
            @@ -1,2 +1,3 @@
             context
            -deleted
            +inserted
            garbage

            --- b/file.txt
            +++ a/file.txt
        "};

        let lines = input.lines().map(DiffLine::parse).collect::<Vec<_>>();

        pretty_assertions::assert_eq!(
            lines,
            &[
                DiffLine::Garbage("diff --git a/text.txt b/text.txt"),
                DiffLine::Garbage("index 86c770d..a1fd855 100644"),
                DiffLine::OldPath {
                    path: "file.txt".into()
                },
                DiffLine::NewPath {
                    path: "file.txt".into()
                },
                DiffLine::HunkHeader(Some(HunkLocation {
                    start_line_old: 0,
                    count_old: 2,
                    start_line_new: 0,
                    count_new: 3
                })),
                DiffLine::Context("context"),
                DiffLine::Deletion("deleted"),
                DiffLine::Addition("inserted"),
                DiffLine::Garbage("garbage"),
                DiffLine::Context(""),
                DiffLine::OldPath {
                    path: "b/file.txt".into()
                },
                DiffLine::NewPath {
                    path: "a/file.txt".into()
                },
            ]
        );
    }

    #[test]
    fn file_header_extra_space() {
        let options = ["--- file", "---   file", "---\tfile"];

        for option in options {
            pretty_assertions::assert_eq!(
                DiffLine::parse(option),
                DiffLine::OldPath {
                    path: "file".into()
                },
                "{option}",
            );
        }
    }

    #[test]
    fn hunk_header_extra_space() {
        let options = [
            "@@ -1,2 +1,3 @@",
            "@@  -1,2  +1,3 @@",
            "@@\t-1,2\t+1,3\t@@",
            "@@ -1,2  +1,3 @@",
            "@@ -1,2   +1,3 @@",
            "@@ -1,2 +1,3   @@",
            "@@ -1,2 +1,3 @@ garbage",
        ];

        for option in options {
            pretty_assertions::assert_eq!(
                DiffLine::parse(option),
                DiffLine::HunkHeader(Some(HunkLocation {
                    start_line_old: 0,
                    count_old: 2,
                    start_line_new: 0,
                    count_new: 3
                })),
                "{option}",
            );
        }
    }

    #[test]
    fn hunk_header_without_location() {
        pretty_assertions::assert_eq!(DiffLine::parse("@@ ... @@"), DiffLine::HunkHeader(None));
    }

    #[test]
    fn test_parse_path() {
        assert_eq!(parse_header_path("a/", "foo.txt"), "foo.txt");
        assert_eq!(
            parse_header_path("a/", "foo/bar/baz.txt"),
            "foo/bar/baz.txt"
        );
        assert_eq!(parse_header_path("a/", "a/foo.txt"), "foo.txt");
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt"),
            "foo/bar/baz.txt"
        );

        // Extra
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt  2025"),
            "foo/bar/baz.txt"
        );
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt\t2025"),
            "foo/bar/baz.txt"
        );
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/baz.txt \""),
            "foo/bar/baz.txt"
        );

        // Quoted
        assert_eq!(
            parse_header_path("a/", "a/foo/bar/\"baz quox.txt\""),
            "foo/bar/baz quox.txt"
        );
        assert_eq!(
            parse_header_path("a/", "\"a/foo/bar/baz quox.txt\""),
            "foo/bar/baz quox.txt"
        );
        assert_eq!(
            parse_header_path("a/", "\"foo/bar/baz quox.txt\""),
            "foo/bar/baz quox.txt"
        );
        assert_eq!(parse_header_path("a/", "\"whatever 🤷\""), "whatever 🤷");
        assert_eq!(
            parse_header_path("a/", "\"foo/bar/baz quox.txt\"  2025"),
            "foo/bar/baz quox.txt"
        );
        // unescaped quotes are dropped
        assert_eq!(parse_header_path("a/", "foo/\"bar\""), "foo/bar");

        // Escaped
        assert_eq!(
            parse_header_path("a/", "\"foo/\\\"bar\\\"/baz.txt\""),
            "foo/\"bar\"/baz.txt"
        );
        assert_eq!(
            parse_header_path("a/", "\"C:\\\\Projects\\\\My App\\\\old file.txt\""),
            "C:\\Projects\\My App\\old file.txt"
        );
    }
}

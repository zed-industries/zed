use once_cell::sync::Lazy;
use prost_types::source_code_info::Location;
#[cfg(feature = "cleanup-markdown")]
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use regex::Regex;

/// Comments on a Protobuf item.
#[derive(Debug, Default, Clone)]
pub struct Comments {
    /// Leading detached blocks of comments.
    pub leading_detached: Vec<Vec<String>>,

    /// Leading comments.
    pub leading: Vec<String>,

    /// Trailing comments.
    pub trailing: Vec<String>,
}

impl Comments {
    pub(crate) fn from_location(location: &Location) -> Comments {
        let leading_detached = location
            .leading_detached_comments
            .iter()
            .map(get_lines)
            .collect();
        let leading = location
            .leading_comments
            .as_ref()
            .map_or(Vec::new(), get_lines);
        let trailing = location
            .trailing_comments
            .as_ref()
            .map_or(Vec::new(), get_lines);
        Comments {
            leading_detached,
            leading,
            trailing,
        }
    }

    /// Appends the comments to a buffer with indentation.
    ///
    /// Each level of indentation corresponds to four space (' ') characters.
    pub fn append_with_indent(&self, indent_level: u8, buf: &mut String) {
        // Append blocks of detached comments.
        for detached_block in &self.leading_detached {
            for line in detached_block {
                for _ in 0..indent_level {
                    buf.push_str("    ");
                }
                buf.push_str("//");
                buf.push_str(&Self::sanitize_line(line));
                buf.push('\n');
            }
            buf.push('\n');
        }

        // Append leading comments.
        for line in &self.leading {
            for _ in 0..indent_level {
                buf.push_str("    ");
            }
            buf.push_str("///");
            buf.push_str(&Self::sanitize_line(line));
            buf.push('\n');
        }

        // Append an empty comment line if there are leading and trailing comments.
        if !self.leading.is_empty() && !self.trailing.is_empty() {
            for _ in 0..indent_level {
                buf.push_str("    ");
            }
            buf.push_str("///\n");
        }

        // Append trailing comments.
        for line in &self.trailing {
            for _ in 0..indent_level {
                buf.push_str("    ");
            }
            buf.push_str("///");
            buf.push_str(&Self::sanitize_line(line));
            buf.push('\n');
        }
    }

    /// Checks whether a RustDoc line should be indented.
    ///
    /// Lines should be indented if:
    /// - they are non-empty, AND
    ///   - they don't already start with a space
    ///     OR
    ///   - they start with several spaces.
    ///
    /// The last condition can happen in the case of multi-line Markdown lists
    /// such as:
    ///
    /// - this is a list
    ///   where some elements spans multiple lines
    /// - but not all elements
    fn should_indent(sanitized_line: &str) -> bool {
        let mut chars = sanitized_line.chars();
        chars
            .next()
            .map_or(false, |c| c != ' ' || chars.next() == Some(' '))
    }

    /// Sanitizes the line for rustdoc by performing the following operations:
    ///     - escape urls as <http://foo.com>
    ///     - escape `[` & `]` if not already escaped and not followed by a parenthesis or bracket
    fn sanitize_line(line: &str) -> String {
        static RULE_URL: Lazy<Regex> = Lazy::new(|| Regex::new(r"https?://[^\s)]+").unwrap());
        static RULE_BRACKETS: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(^|[^\]\\])\[(([^\]]*[^\\])?)\]([^(\[]|$)").unwrap());

        let mut s = RULE_URL.replace_all(line, r"<$0>").to_string();
        s = RULE_BRACKETS.replace_all(&s, r"$1\[$2\]$4").to_string();
        if Self::should_indent(&s) {
            s.insert(0, ' ');
        }
        s
    }
}

/// A service descriptor.
#[derive(Debug, Clone)]
pub struct Service {
    /// The service name in Rust style.
    pub name: String,
    /// The service name as it appears in the .proto file.
    pub proto_name: String,
    /// The package name as it appears in the .proto file.
    pub package: String,
    /// The service comments.
    pub comments: Comments,
    /// The service methods.
    pub methods: Vec<Method>,
    /// The service options.
    pub options: prost_types::ServiceOptions,
}

/// A service method descriptor.
#[derive(Debug, Clone)]
pub struct Method {
    /// The name of the method in Rust style.
    pub name: String,
    /// The name of the method as it appears in the .proto file.
    pub proto_name: String,
    /// The method comments.
    pub comments: Comments,
    /// The input Rust type.
    pub input_type: String,
    /// The output Rust type.
    pub output_type: String,
    /// The input Protobuf type.
    pub input_proto_type: String,
    /// The output Protobuf type.
    pub output_proto_type: String,
    /// The method options.
    pub options: prost_types::MethodOptions,
    /// Identifies if client streams multiple client messages.
    pub client_streaming: bool,
    /// Identifies if server streams multiple server messages.
    pub server_streaming: bool,
}

#[cfg(not(feature = "cleanup-markdown"))]
fn get_lines<S>(comments: S) -> Vec<String>
where
    S: AsRef<str>,
{
    comments.as_ref().lines().map(str::to_owned).collect()
}

#[cfg(feature = "cleanup-markdown")]
fn get_lines<S>(comments: S) -> Vec<String>
where
    S: AsRef<str>,
{
    let comments = comments.as_ref();
    let mut buffer = String::with_capacity(comments.len() + 256);
    let opts = pulldown_cmark_to_cmark::Options {
        code_block_token_count: 3,
        ..Default::default()
    };
    match pulldown_cmark_to_cmark::cmark_with_options(
        Parser::new_ext(comments, Options::all() - Options::ENABLE_SMART_PUNCTUATION).map(
            |event| {
                fn map_codeblock(kind: CodeBlockKind) -> CodeBlockKind {
                    match kind {
                        CodeBlockKind::Fenced(s) => {
                            if s.as_ref() == "rust" {
                                CodeBlockKind::Fenced("compile_fail".into())
                            } else {
                                CodeBlockKind::Fenced(format!("text,{}", s).into())
                            }
                        }
                        CodeBlockKind::Indented => CodeBlockKind::Fenced("text".into()),
                    }
                }
                match event {
                    Event::Start(Tag::CodeBlock(kind)) => {
                        Event::Start(Tag::CodeBlock(map_codeblock(kind)))
                    }
                    Event::End(Tag::CodeBlock(kind)) => {
                        Event::End(Tag::CodeBlock(map_codeblock(kind)))
                    }
                    e => e,
                }
            },
        ),
        &mut buffer,
        opts,
    ) {
        Ok(_) => buffer.lines().map(str::to_owned).collect(),
        Err(_) => comments.lines().map(str::to_owned).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_append_with_indent_leaves_prespaced_lines() {
        struct TestCases {
            name: &'static str,
            input: String,
            expected: String,
        }

        let tests = vec![
            TestCases {
                name: "existing_space",
                input: " A line with a single leading space.".to_string(),
                expected: "/// A line with a single leading space.\n".to_string(),
            },
            TestCases {
                name: "non_existing_space",
                input: "A line without a single leading space.".to_string(),
                expected: "/// A line without a single leading space.\n".to_string(),
            },
            TestCases {
                name: "empty",
                input: "".to_string(),
                expected: "///\n".to_string(),
            },
            TestCases {
                name: "multiple_leading_spaces",
                input: "  a line with several leading spaces, such as in a markdown list"
                    .to_string(),
                expected: "///   a line with several leading spaces, such as in a markdown list\n"
                    .to_string(),
            },
        ];
        for t in tests {
            let input = Comments {
                leading_detached: vec![],
                leading: vec![],
                trailing: vec![t.input],
            };

            let mut actual = "".to_string();
            input.append_with_indent(0, &mut actual);

            assert_eq!(t.expected, actual, "failed {}", t.name);
        }
    }

    #[test]
    fn test_comment_append_with_indent_sanitizes_comment_doc_url() {
        struct TestCases {
            name: &'static str,
            input: String,
            expected: String,
        }

        let tests = vec![
            TestCases {
                name: "valid_http",
                input: "See https://www.rust-lang.org/".to_string(),
                expected: "/// See <https://www.rust-lang.org/>\n".to_string(),
            },
            TestCases {
                name: "valid_https",
                input: "See https://www.rust-lang.org/".to_string(),
                expected: "/// See <https://www.rust-lang.org/>\n".to_string(),
            },
            TestCases {
                name: "valid_https_parenthesis",
                input: "See (https://www.rust-lang.org/)".to_string(),
                expected: "/// See (<https://www.rust-lang.org/>)\n".to_string(),
            },
            TestCases {
                name: "invalid",
                input: "See note://abc".to_string(),
                expected: "/// See note://abc\n".to_string(),
            },
        ];
        for t in tests {
            let input = Comments {
                leading_detached: vec![],
                leading: vec![],
                trailing: vec![t.input],
            };

            let mut actual = "".to_string();
            input.append_with_indent(0, &mut actual);

            assert_eq!(t.expected, actual, "failed {}", t.name);
        }
    }

    #[test]
    fn test_comment_append_with_indent_sanitizes_square_brackets() {
        struct TestCases {
            name: &'static str,
            input: String,
            expected: String,
        }

        let tests = vec![
            TestCases {
                name: "valid_brackets",
                input: "foo [bar] baz".to_string(),
                expected: "/// foo \\[bar\\] baz\n".to_string(),
            },
            TestCases {
                name: "invalid_start_bracket",
                input: "foo [= baz".to_string(),
                expected: "/// foo [= baz\n".to_string(),
            },
            TestCases {
                name: "invalid_end_bracket",
                input: "foo =] baz".to_string(),
                expected: "/// foo =] baz\n".to_string(),
            },
            TestCases {
                name: "invalid_bracket_combination",
                input: "[0, 9)".to_string(),
                expected: "/// [0, 9)\n".to_string(),
            },
            TestCases {
                name: "valid_brackets_parenthesis",
                input: "foo [bar](bar) baz".to_string(),
                expected: "/// foo [bar](bar) baz\n".to_string(),
            },
            TestCases {
                name: "valid_brackets_end",
                input: "foo [bar]".to_string(),
                expected: "/// foo \\[bar\\]\n".to_string(),
            },
            TestCases {
                name: "valid_brackets_no_parenthesis",
                input: "foo [bar]baz".to_string(),
                expected: "/// foo \\[bar\\]baz\n".to_string(),
            },
            TestCases {
                name: "valid_empty_brackets",
                input: "foo []".to_string(),
                expected: "/// foo \\[\\]\n".to_string(),
            },
            TestCases {
                name: "valid_empty_brackets_parenthesis",
                input: "foo []()".to_string(),
                expected: "/// foo []()\n".to_string(),
            },
            TestCases {
                name: "valid_brackets_brackets",
                input: "foo [bar][bar] baz".to_string(),
                expected: "/// foo [bar][bar] baz\n".to_string(),
            },
            TestCases {
                name: "valid_brackets_brackets_end",
                input: "foo [bar][baz]".to_string(),
                expected: "/// foo [bar][baz]\n".to_string(),
            },
            TestCases {
                name: "valid_brackets_brackets_all",
                input: "[bar][baz]".to_string(),
                expected: "/// [bar][baz]\n".to_string(),
            },
            TestCases {
                name: "escaped_brackets",
                input: "\\[bar\\]\\[baz\\]".to_string(),
                expected: "/// \\[bar\\]\\[baz\\]\n".to_string(),
            },
            TestCases {
                name: "escaped_empty_brackets",
                input: "\\[\\]\\[\\]".to_string(),
                expected: "/// \\[\\]\\[\\]\n".to_string(),
            },
        ];
        for t in tests {
            let input = Comments {
                leading_detached: vec![],
                leading: vec![],
                trailing: vec![t.input],
            };

            let mut actual = "".to_string();
            input.append_with_indent(0, &mut actual);

            assert_eq!(t.expected, actual, "failed {}", t.name);
        }
    }

    #[test]
    fn test_codeblocks() {
        struct TestCase {
            name: &'static str,
            input: &'static str,
            #[allow(unused)]
            cleanedup_expected: Vec<&'static str>,
        }

        let tests = vec![
            TestCase {
                name: "unlabelled_block",
                input: "    thingy\n",
                cleanedup_expected: vec!["", "```text", "thingy", "```"],
            },
            TestCase {
                name: "rust_block",
                input: "```rust\nfoo.bar()\n```\n",
                cleanedup_expected: vec!["", "```compile_fail", "foo.bar()", "```"],
            },
            TestCase {
                name: "js_block",
                input: "```javascript\nfoo.bar()\n```\n",
                cleanedup_expected: vec!["", "```text,javascript", "foo.bar()", "```"],
            },
        ];

        for t in tests {
            let loc = Location {
                path: vec![],
                span: vec![],
                leading_comments: Some(t.input.into()),
                trailing_comments: None,
                leading_detached_comments: vec![],
            };
            let comments = Comments::from_location(&loc);
            #[cfg(feature = "cleanup-markdown")]
            let expected = t.cleanedup_expected;
            #[cfg(not(feature = "cleanup-markdown"))]
            let expected: Vec<&str> = t.input.lines().collect();
            assert_eq!(expected, comments.leading, "failed {}", t.name);
        }
    }
}

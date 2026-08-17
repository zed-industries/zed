use lazy_static::lazy_static;
use prost_types::source_code_info::Location;
use regex::Regex;

/// Comments on a Protobuf item.
#[derive(Debug)]
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
        fn get_lines<S>(comments: S) -> Vec<String>
        where
            S: AsRef<str>,
        {
            comments.as_ref().lines().map(str::to_owned).collect()
        }

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

    /// Sanitizes the line for rustdoc by performing the following operations:
    ///     - escape urls as <http://foo.com>
    ///     - escape `[` & `]`
    fn sanitize_line(line: &str) -> String {
        lazy_static! {
            static ref RULE_URL: Regex = Regex::new(r"https?://[^\s)]+").unwrap();
            static ref RULE_BRACKETS: Regex = Regex::new(r"(\[)(\S+)(])").unwrap();
        }

        let mut s = RULE_URL.replace_all(line, r"<$0>").to_string();
        s = RULE_BRACKETS.replace_all(&s, r"\$1$2\$3").to_string();
        s
    }
}

/// A service descriptor.
#[derive(Debug)]
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
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

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
                expected: "///See <https://www.rust-lang.org/>\n".to_string(),
            },
            TestCases {
                name: "valid_https",
                input: "See https://www.rust-lang.org/".to_string(),
                expected: "///See <https://www.rust-lang.org/>\n".to_string(),
            },
            TestCases {
                name: "valid_https_parenthesis",
                input: "See (https://www.rust-lang.org/)".to_string(),
                expected: "///See (<https://www.rust-lang.org/>)\n".to_string(),
            },
            TestCases {
                name: "invalid",
                input: "See note://abc".to_string(),
                expected: "///See note://abc\n".to_string(),
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
                expected: "///foo \\[bar\\] baz\n".to_string(),
            },
            TestCases {
                name: "invalid_start_bracket",
                input: "foo [= baz".to_string(),
                expected: "///foo [= baz\n".to_string(),
            },
            TestCases {
                name: "invalid_end_bracket",
                input: "foo =] baz".to_string(),
                expected: "///foo =] baz\n".to_string(),
            },
            TestCases {
                name: "invalid_bracket_combination",
                input: "[0, 9)".to_string(),
                expected: "///[0, 9)\n".to_string(),
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
}

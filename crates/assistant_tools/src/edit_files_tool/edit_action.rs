use std::{
    mem::take,
    ops::Range,
    path::{Path, PathBuf},
};
use util::ResultExt;

/// Represents an edit action to be performed on a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditAction {
    /// Replace specific content in a file with new content
    Replace {
        file_path: PathBuf,
        old: String,
        new: String,
    },
    /// Write content to a file (create or overwrite)
    Write { file_path: PathBuf, content: String },
}

impl EditAction {
    pub fn file_path(&self) -> &Path {
        match self {
            EditAction::Replace { file_path, .. } => file_path,
            EditAction::Write { file_path, .. } => file_path,
        }
    }
}

/// Parses edit actions from an LLM response.
/// See system.md for more details on the format.
#[derive(Debug)]
pub struct EditActionParser {
    state: State,
    line: usize,
    column: usize,
    marker_ix: usize,
    action_source: Vec<u8>,
    fence_start_offset: usize,
    block_range: Range<usize>,
    old_range: Range<usize>,
    new_range: Range<usize>,
    errors: Vec<ParseError>,
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    /// Anywhere outside an action
    Default,
    /// After opening ```, in optional language tag
    OpenFence,
    /// In SEARCH marker
    SearchMarker,
    /// In search block or divider
    SearchBlock,
    /// In replace block or REPLACE marker
    ReplaceBlock,
    /// In closing ```
    CloseFence,
}

impl EditActionParser {
    /// Creates a new `EditActionParser`
    pub fn new() -> Self {
        Self {
            state: State::Default,
            line: 1,
            column: 0,
            action_source: Vec::new(),
            fence_start_offset: 0,
            marker_ix: 0,
            block_range: Range::default(),
            old_range: Range::default(),
            new_range: Range::default(),
            errors: Vec::new(),
        }
    }

    /// Processes a chunk of input text and returns any completed edit actions.
    ///
    /// This method can be called repeatedly with fragments of input. The parser
    /// maintains its state between calls, allowing you to process streaming input
    /// as it becomes available. Actions are only inserted once they are fully parsed.
    ///
    /// If a block fails to parse, it will simply be skipped and an error will be recorded.
    /// All errors can be accessed through the `EditActionsParser::errors` method.
    pub fn parse_chunk(&mut self, input: &str) -> Vec<(EditAction, String)> {
        use State::*;

        const FENCE: &[u8] = b"```";
        const SEARCH_MARKER: &[u8] = b"<<<<<<< SEARCH";
        const DIVIDER: &[u8] = b"=======";
        const NL_DIVIDER: &[u8] = b"\n=======";
        const REPLACE_MARKER: &[u8] = b">>>>>>> REPLACE";
        const NL_REPLACE_MARKER: &[u8] = b"\n>>>>>>> REPLACE";

        let mut actions = Vec::new();

        for byte in input.bytes() {
            // Update line and column tracking
            if byte == b'\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }

            let action_offset = self.action_source.len();

            match &self.state {
                Default => match self.match_marker(byte, FENCE, false) {
                    MarkerMatch::Complete => {
                        self.fence_start_offset = action_offset + 1 - FENCE.len();
                        self.to_state(OpenFence);
                    }
                    MarkerMatch::Partial => {}
                    MarkerMatch::None => {
                        if self.marker_ix > 0 {
                            self.marker_ix = 0;
                        } else if self.action_source.ends_with(b"\n") {
                            self.action_source.clear();
                        }
                    }
                },
                OpenFence => {
                    // skip language tag
                    if byte == b'\n' {
                        self.to_state(SearchMarker);
                    }
                }
                SearchMarker => {
                    if self.expect_marker(byte, SEARCH_MARKER, true) {
                        self.to_state(SearchBlock);
                    }
                }
                SearchBlock => {
                    if self.extend_block_range(byte, DIVIDER, NL_DIVIDER) {
                        self.old_range = take(&mut self.block_range);
                        self.to_state(ReplaceBlock);
                    }
                }
                ReplaceBlock => {
                    if self.extend_block_range(byte, REPLACE_MARKER, NL_REPLACE_MARKER) {
                        self.new_range = take(&mut self.block_range);
                        self.to_state(CloseFence);
                    }
                }
                CloseFence => {
                    if self.expect_marker(byte, FENCE, false) {
                        self.action_source.push(byte);

                        if let Some(action) = self.action() {
                            actions.push(action);
                        }

                        self.errors();
                        self.reset();

                        continue;
                    }
                }
            };

            self.action_source.push(byte);
        }

        actions
    }

    /// Returns a reference to the errors encountered during parsing.
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    fn action(&mut self) -> Option<(EditAction, String)> {
        let old_range = take(&mut self.old_range);
        let new_range = take(&mut self.new_range);

        let action_source = take(&mut self.action_source);
        let action_source = String::from_utf8(action_source).log_err()?;

        let mut file_path_bytes = action_source[..self.fence_start_offset].to_owned();

        if file_path_bytes.ends_with("\n") {
            file_path_bytes.pop();
            if file_path_bytes.ends_with("\r") {
                file_path_bytes.pop();
            }
        }

        let file_path = PathBuf::from(file_path_bytes);

        if old_range.is_empty() {
            return Some((
                EditAction::Write {
                    file_path,
                    content: action_source[new_range].to_owned(),
                },
                action_source,
            ));
        }

        let old = action_source[old_range].to_owned();
        let new = action_source[new_range].to_owned();

        let action = EditAction::Replace {
            file_path,
            old,
            new,
        };

        Some((action, action_source))
    }

    fn to_state(&mut self, state: State) {
        self.state = state;
        self.marker_ix = 0;
    }

    fn reset(&mut self) {
        self.action_source.clear();
        self.block_range = Range::default();
        self.old_range = Range::default();
        self.new_range = Range::default();
        self.fence_start_offset = 0;
        self.marker_ix = 0;
        self.to_state(State::Default);
    }

    fn expect_marker(&mut self, byte: u8, marker: &'static [u8], trailing_newline: bool) -> bool {
        match self.match_marker(byte, marker, trailing_newline) {
            MarkerMatch::Complete => true,
            MarkerMatch::Partial => false,
            MarkerMatch::None => {
                self.errors.push(ParseError {
                    line: self.line,
                    column: self.column,
                    expected: marker,
                    found: byte,
                });

                self.reset();
                false
            }
        }
    }

    fn extend_block_range(&mut self, byte: u8, marker: &[u8], nl_marker: &[u8]) -> bool {
        let marker = if self.block_range.is_empty() {
            // do not require another newline if block is empty
            marker
        } else {
            nl_marker
        };

        let offset = self.action_source.len();

        match self.match_marker(byte, marker, true) {
            MarkerMatch::Complete => {
                if self.action_source[self.block_range.clone()].ends_with(b"\r") {
                    self.block_range.end -= 1;
                }

                true
            }
            MarkerMatch::Partial => false,
            MarkerMatch::None => {
                if self.marker_ix > 0 {
                    self.marker_ix = 0;
                    self.block_range.end = offset;

                    // The beginning of marker might match current byte
                    match self.match_marker(byte, marker, true) {
                        MarkerMatch::Complete => return true,
                        MarkerMatch::Partial => return false,
                        MarkerMatch::None => { /* no match, keep collecting */ }
                    }
                }

                if self.block_range.is_empty() {
                    self.block_range.start = offset;
                }
                self.block_range.end = offset + 1;

                false
            }
        }
    }

    fn match_marker(&mut self, byte: u8, marker: &[u8], trailing_newline: bool) -> MarkerMatch {
        if trailing_newline && self.marker_ix >= marker.len() {
            if byte == b'\n' {
                MarkerMatch::Complete
            } else if byte == b'\r' {
                MarkerMatch::Partial
            } else {
                MarkerMatch::None
            }
        } else if byte == marker[self.marker_ix] {
            self.marker_ix += 1;

            if self.marker_ix < marker.len() || trailing_newline {
                MarkerMatch::Partial
            } else {
                MarkerMatch::Complete
            }
        } else {
            MarkerMatch::None
        }
    }
}

#[derive(Debug)]
enum MarkerMatch {
    None,
    Partial,
    Complete,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    line: usize,
    column: usize,
    expected: &'static [u8],
    found: u8,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "input:{}:{}: Expected marker {:?}, found {:?}",
            self.line,
            self.column,
            String::from_utf8_lossy(self.expected),
            self.found as char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use util::line_endings;

    #[test]
    fn test_simple_edit_action() {
        let input = r#"src/main.rs
```
<<<<<<< SEARCH
fn original() {}
=======
fn replacement() {}
>>>>>>> REPLACE
```
"#;

        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(input);

        assert_no_errors(&parser);
        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0].0,
            EditAction::Replace {
                file_path: PathBuf::from("src/main.rs"),
                old: "fn original() {}".to_string(),
                new: "fn replacement() {}".to_string(),
            }
        );
    }

    #[test]
    fn test_with_language_tag() {
        let input = r#"src/main.rs
```rust
<<<<<<< SEARCH
fn original() {}
=======
fn replacement() {}
>>>>>>> REPLACE
```
"#;

        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(input);

        assert_no_errors(&parser);
        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0].0,
            EditAction::Replace {
                file_path: PathBuf::from("src/main.rs"),
                old: "fn original() {}".to_string(),
                new: "fn replacement() {}".to_string(),
            }
        );
    }

    #[test]
    fn test_with_surrounding_text() {
        let input = r#"Here's a modification I'd like to make to the file:

src/main.rs
```rust
<<<<<<< SEARCH
fn original() {}
=======
fn replacement() {}
>>>>>>> REPLACE
```

This change makes the function better.
"#;

        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(input);

        assert_no_errors(&parser);
        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0].0,
            EditAction::Replace {
                file_path: PathBuf::from("src/main.rs"),
                old: "fn original() {}".to_string(),
                new: "fn replacement() {}".to_string(),
            }
        );
    }

    #[test]
    fn test_multiple_edit_actions() {
        let input = r#"First change:
src/main.rs
```
<<<<<<< SEARCH
fn original() {}
=======
fn replacement() {}
>>>>>>> REPLACE
```

Second change:
src/utils.rs
```rust
<<<<<<< SEARCH
fn old_util() -> bool { false }
=======
fn new_util() -> bool { true }
>>>>>>> REPLACE
```
"#;

        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(input);

        assert_no_errors(&parser);
        assert_eq!(actions.len(), 2);

        let (action, _) = &actions[0];
        assert_eq!(
            action,
            &EditAction::Replace {
                file_path: PathBuf::from("src/main.rs"),
                old: "fn original() {}".to_string(),
                new: "fn replacement() {}".to_string(),
            }
        );
        let (action2, _) = &actions[1];
        assert_eq!(
            action2,
            &EditAction::Replace {
                file_path: PathBuf::from("src/utils.rs"),
                old: "fn old_util() -> bool { false }".to_string(),
                new: "fn new_util() -> bool { true }".to_string(),
            }
        );
    }

    #[test]
    fn test_multiline() {
        let input = r#"src/main.rs
```rust
<<<<<<< SEARCH
fn original() {
    println!("This is the original function");
    let x = 42;
    if x > 0 {
        println!("Positive number");
    }
}
=======
fn replacement() {
    println!("This is the replacement function");
    let x = 100;
    if x > 50 {
        println!("Large number");
    } else {
        println!("Small number");
    }
}
>>>>>>> REPLACE
```
"#;

        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(input);

        assert_no_errors(&parser);
        assert_eq!(actions.len(), 1);

        let (action, _) = &actions[0];
        assert_eq!(
            action,
            &EditAction::Replace {
                file_path: PathBuf::from("src/main.rs"),
                old: "fn original() {\n    println!(\"This is the original function\");\n    let x = 42;\n    if x > 0 {\n        println!(\"Positive number\");\n    }\n}".to_string(),
                new: "fn replacement() {\n    println!(\"This is the replacement function\");\n    let x = 100;\n    if x > 50 {\n        println!(\"Large number\");\n    } else {\n        println!(\"Small number\");\n    }\n}".to_string(),
            }
        );
    }

    #[test]
    fn test_write_action() {
        let input = r#"Create a new main.rs file:

src/main.rs
```rust
<<<<<<< SEARCH
=======
fn new_function() {
    println!("This function is being added");
}
>>>>>>> REPLACE
```
"#;

        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(input);

        assert_no_errors(&parser);
        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0].0,
            EditAction::Write {
                file_path: PathBuf::from("src/main.rs"),
                content: "fn new_function() {\n    println!(\"This function is being added\");\n}"
                    .to_string(),
            }
        );
    }

    #[test]
    fn test_empty_replace() {
        let input = r#"src/main.rs
```rust
<<<<<<< SEARCH
fn this_will_be_deleted() {
    println!("Deleting this function");
}
=======
>>>>>>> REPLACE
```
"#;

        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(&input);

        assert_no_errors(&parser);
        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0].0,
            EditAction::Replace {
                file_path: PathBuf::from("src/main.rs"),
                old: "fn this_will_be_deleted() {\n    println!(\"Deleting this function\");\n}"
                    .to_string(),
                new: "".to_string(),
            }
        );

        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(&input.replace("\n", "\r\n"));
        assert_no_errors(&parser);
        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0].0,
            EditAction::Replace {
                file_path: PathBuf::from("src/main.rs"),
                old:
                    "fn this_will_be_deleted() {\r\n    println!(\"Deleting this function\");\r\n}"
                        .to_string(),
                new: "".to_string(),
            }
        );
    }

    #[test]
    fn test_empty_both() {
        let input = r#"src/main.rs
```rust
<<<<<<< SEARCH
=======
>>>>>>> REPLACE
```
"#;

        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(input);

        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0].0,
            EditAction::Write {
                file_path: PathBuf::from("src/main.rs"),
                content: String::new(),
            }
        );
        assert_no_errors(&parser);
    }

    #[test]
    fn test_resumability() {
        let input_part1 = r#"src/main.rs
```rust
<<<<<<< SEARCH
fn ori"#;

        let input_part2 = r#"ginal() {}
=======
fn replacement() {}"#;

        let input_part3 = r#"
>>>>>>> REPLACE
```
"#;

        let mut parser = EditActionParser::new();
        let actions1 = parser.parse_chunk(input_part1);
        assert_no_errors(&parser);
        assert_eq!(actions1.len(), 0);

        let actions2 = parser.parse_chunk(input_part2);
        // No actions should be complete yet
        assert_no_errors(&parser);
        assert_eq!(actions2.len(), 0);

        let actions3 = parser.parse_chunk(input_part3);
        // The third chunk should complete the action
        assert_no_errors(&parser);
        assert_eq!(actions3.len(), 1);
        let (action, _) = &actions3[0];
        assert_eq!(
            action,
            &EditAction::Replace {
                file_path: PathBuf::from("src/main.rs"),
                old: "fn original() {}".to_string(),
                new: "fn replacement() {}".to_string(),
            }
        );
    }

    #[test]
    fn test_parser_state_preservation() {
        let mut parser = EditActionParser::new();
        let actions1 = parser.parse_chunk("src/main.rs\n```rust\n<<<<<<< SEARCH\n");

        // Check parser is in the correct state
        assert_no_errors(&parser);
        assert_eq!(parser.state, State::SearchBlock);
        assert_eq!(
            parser.action_source,
            b"src/main.rs\n```rust\n<<<<<<< SEARCH\n"
        );

        // Continue parsing
        let actions2 = parser.parse_chunk("original code\n=======\n");

        assert_no_errors(&parser);
        assert_eq!(parser.state, State::ReplaceBlock);
        assert_eq!(
            &parser.action_source[parser.old_range.clone()],
            b"original code"
        );

        let actions3 = parser.parse_chunk("replacement code\n>>>>>>> REPLACE\n```\n");

        // After complete parsing, state should reset
        assert_no_errors(&parser);
        assert_eq!(parser.state, State::Default);
        assert_eq!(parser.action_source, b"\n");
        assert!(parser.old_range.is_empty());
        assert!(parser.new_range.is_empty());

        assert_eq!(actions1.len(), 0);
        assert_eq!(actions2.len(), 0);
        assert_eq!(actions3.len(), 1);
    }

    #[test]
    fn test_invalid_search_marker() {
        let input = r#"src/main.rs
```rust
<<<<<<< WRONG_MARKER
fn original() {}
=======
fn replacement() {}
>>>>>>> REPLACE
```
"#;

        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(input);
        assert_eq!(actions.len(), 0);

        assert_eq!(parser.errors().len(), 1);
        let error = &parser.errors()[0];

        assert_eq!(
            error.to_string(),
            "input:3:9: Expected marker \"<<<<<<< SEARCH\", found 'W'"
        );
    }

    #[test]
    fn test_missing_closing_fence() {
        let input = r#"src/main.rs
```rust
<<<<<<< SEARCH
fn original() {}
=======
fn replacement() {}
>>>>>>> REPLACE
<!-- Missing closing fence -->

src/utils.rs
```rust
<<<<<<< SEARCH
fn utils_func() {}
=======
fn new_utils_func() {}
>>>>>>> REPLACE
```
"#;

        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(input);

        // Only the second block should be parsed
        assert_eq!(actions.len(), 1);
        let (action, _) = &actions[0];
        assert_eq!(
            action,
            &EditAction::Replace {
                file_path: PathBuf::from("src/utils.rs"),
                old: "fn utils_func() {}".to_string(),
                new: "fn new_utils_func() {}".to_string(),
            }
        );
        assert_eq!(parser.errors().len(), 1);
        assert_eq!(
            parser.errors()[0].to_string(),
            "input:8:1: Expected marker \"```\", found '<'"
        );

        // The parser should continue after an error
        assert_eq!(parser.state, State::Default);
    }

    const SYSTEM_PROMPT: &str = include_str!("./edit_prompt.md");

    #[test]
    fn test_parse_examples_in_system_prompt() {
        let mut parser = EditActionParser::new();
        let actions = parser.parse_chunk(SYSTEM_PROMPT);
        assert_examples_in_system_prompt(&actions, parser.errors());
    }

    #[gpui::test(iterations = 10)]
    fn test_random_chunking_of_system_prompt(mut rng: StdRng) {
        let mut parser = EditActionParser::new();
        let mut remaining = SYSTEM_PROMPT;
        let mut actions = Vec::with_capacity(5);

        while !remaining.is_empty() {
            let chunk_size = rng.gen_range(1..=std::cmp::min(remaining.len(), 100));

            let (chunk, rest) = remaining.split_at(chunk_size);

            let chunk_actions = parser.parse_chunk(chunk);
            actions.extend(chunk_actions);
            remaining = rest;
        }

        assert_examples_in_system_prompt(&actions, parser.errors());
    }

    fn assert_examples_in_system_prompt(actions: &[(EditAction, String)], errors: &[ParseError]) {
        assert_eq!(actions.len(), 5);

        assert_eq!(
            actions[0].0,
            EditAction::Replace {
                file_path: PathBuf::from("mathweb/flask/app.py"),
                old: "from flask import Flask".to_string(),
                new: line_endings!("import math\nfrom flask import Flask").to_string(),
            },
        );

        assert_eq!(
            actions[1].0,
            EditAction::Replace {
                file_path: PathBuf::from("mathweb/flask/app.py"),
                old: line_endings!("def factorial(n):\n    \"compute factorial\"\n\n    if n == 0:\n        return 1\n    else:\n        return n * factorial(n-1)\n").to_string(),
                new: "".to_string(),
            }
        );

        assert_eq!(
            actions[2].0,
            EditAction::Replace {
                file_path: PathBuf::from("mathweb/flask/app.py"),
                old: "    return str(factorial(n))".to_string(),
                new: "    return str(math.factorial(n))".to_string(),
            },
        );

        assert_eq!(
            actions[3].0,
            EditAction::Write {
                file_path: PathBuf::from("hello.py"),
                content: line_endings!(
                    "def hello():\n    \"print a greeting\"\n\n    print(\"hello\")"
                )
                .to_string(),
            },
        );

        assert_eq!(
            actions[4].0,
            EditAction::Replace {
                file_path: PathBuf::from("main.py"),
                old: line_endings!(
                    "def hello():\n    \"print a greeting\"\n\n    print(\"hello\")"
                )
                .to_string(),
                new: "from hello import hello".to_string(),
            },
        );

        // The system prompt includes some text that would produce errors
        assert_eq!(
            errors[0].to_string(),
            "input:102:1: Expected marker \"<<<<<<< SEARCH\", found '3'"
        );
        #[cfg(not(windows))]
        assert_eq!(
            errors[1].to_string(),
            "input:109:0: Expected marker \"<<<<<<< SEARCH\", found '\\n'"
        );
        #[cfg(windows)]
        assert_eq!(
            errors[1].to_string(),
            "input:108:1: Expected marker \"<<<<<<< SEARCH\", found '\\r'"
        );
    }

    #[test]
    fn test_print_error() {
        let input = r#"src/main.rs
```rust
<<<<<<< WRONG_MARKER
fn original() {}
=======
fn replacement() {}
>>>>>>> REPLACE
```
"#;

        let mut parser = EditActionParser::new();
        parser.parse_chunk(input);

        assert_eq!(parser.errors().len(), 1);
        let error = &parser.errors()[0];
        let expected_error = r#"input:3:9: Expected marker "<<<<<<< SEARCH", found 'W'"#;

        assert_eq!(format!("{}", error), expected_error);
    }

    // helpers

    fn assert_no_errors(parser: &EditActionParser) {
        let errors = parser.errors();

        assert!(
            errors.is_empty(),
            "Expected no errors, but found:\n\n{}",
            errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}

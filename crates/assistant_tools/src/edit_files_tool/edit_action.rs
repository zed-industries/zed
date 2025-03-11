use util::ResultExt;

/// Represents an edit action to be performed on a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditAction {
    /// Replace specific content in a file with new content
    Replace {
        file_path: String,
        old: String,
        new: String,
    },
    /// Write content to a file (create or overwrite)
    Write { file_path: String, content: String },
}

impl EditAction {
    pub fn file_path(&self) -> &str {
        match self {
            EditAction::Replace { file_path, .. } => file_path,
            EditAction::Write { file_path, .. } => file_path,
        }
    }
}

/// Parses edit actions from an LLM response.
///
/// A response might include many edit actions with the following format:
///
/// Replace content:
///
/// src/main.rs
/// ```
/// <<<<<<< SEARCH
/// fn original() {}
/// =======
/// fn replacement() {}
/// >>>>>>> REPLACE
/// ```
///
/// Write new content:
///
/// src/main.rs
/// ```
/// <<<<<<< SEARCH
/// =======
/// fn new_function() {}
/// >>>>>>> REPLACE
/// ```
#[derive(Debug)]
pub struct EditActionParser {
    state: State,
    pre_fence_line: Vec<u8>,
    marker_ix: usize,
    offset: usize,
    old_bytes: Vec<u8>,
    new_bytes: Vec<u8>,
    errors: Vec<(usize, ParseError)>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ExpectedMarker { expected: &'static [u8], found: u8 },
    NoOp,
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
            pre_fence_line: Vec::new(),
            marker_ix: 0,
            offset: 0,
            old_bytes: Vec::new(),
            new_bytes: Vec::new(),
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
    pub fn parse_chunk<'a>(
        &mut self,
        input: &str,
        actions: &'a mut Vec<EditAction>,
    ) -> &'a [EditAction] {
        use State::*;

        const FENCE: &[u8] = b"\n```";
        const SEARCH_MARKER: &[u8] = b"<<<<<<< SEARCH\n";
        const DIVIDER: &[u8] = b"=======\n";
        const NL_DIVIDER: &[u8] = b"\n=======\n";
        const REPLACE_MARKER: &[u8] = b">>>>>>> REPLACE";
        const NL_REPLACE_MARKER: &[u8] = b"\n>>>>>>> REPLACE";

        let start_index = actions.len();

        for byte in input.bytes() {
            match self.state {
                Default => match match_marker(byte, FENCE, &mut self.marker_ix) {
                    MarkerMatch::Complete => {
                        self.to_state(OpenFence);
                    }
                    MarkerMatch::Partial => {}
                    MarkerMatch::None => {
                        if self.marker_ix > 0 {
                            self.marker_ix = 0;
                            self.pre_fence_line.clear();
                        }

                        if byte != b'\n' {
                            self.pre_fence_line.push(byte);
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
                    if self.expect_marker(byte, SEARCH_MARKER) {
                        self.to_state(SearchBlock);
                    }
                }
                SearchBlock => {
                    if collect_until_marker(
                        byte,
                        DIVIDER,
                        NL_DIVIDER,
                        &mut self.marker_ix,
                        &mut self.old_bytes,
                    ) {
                        self.to_state(ReplaceBlock);
                    }
                }
                ReplaceBlock => {
                    if collect_until_marker(
                        byte,
                        REPLACE_MARKER,
                        NL_REPLACE_MARKER,
                        &mut self.marker_ix,
                        &mut self.new_bytes,
                    ) {
                        self.to_state(CloseFence);
                    }
                }
                CloseFence => {
                    if self.expect_marker(byte, FENCE) {
                        if let Some(action) = self.action() {
                            actions.push(action);
                        }
                        self.reset();
                    }
                }
            };

            self.offset += 1;
        }

        &actions[start_index..]
    }

    /// Returns a reference to the errors encountered during parsing.
    pub fn errors(&self) -> &[(usize, ParseError)] {
        &self.errors
    }

    fn action(&mut self) -> Option<EditAction> {
        if self.old_bytes.is_empty() && self.new_bytes.is_empty() {
            self.errors.push((self.offset, ParseError::NoOp));
            return None;
        }

        let file_path = String::from_utf8(std::mem::take(&mut self.pre_fence_line)).log_err()?;
        let content = String::from_utf8(std::mem::take(&mut self.new_bytes)).log_err()?;

        if self.old_bytes.is_empty() {
            Some(EditAction::Write { file_path, content })
        } else {
            let old = String::from_utf8(std::mem::take(&mut self.old_bytes)).log_err()?;

            Some(EditAction::Replace {
                file_path,
                old,
                new: content,
            })
        }
    }

    fn expect_marker(&mut self, byte: u8, marker: &'static [u8]) -> bool {
        match match_marker(byte, marker, &mut self.marker_ix) {
            MarkerMatch::Complete => true,
            MarkerMatch::Partial => false,
            MarkerMatch::None => {
                self.errors.push((
                    self.offset,
                    ParseError::ExpectedMarker {
                        expected: marker,
                        found: byte,
                    },
                ));
                self.reset();
                false
            }
        }
    }

    fn to_state(&mut self, state: State) {
        self.state = state;
        self.marker_ix = 0;
    }

    fn reset(&mut self) {
        self.pre_fence_line.clear();
        self.old_bytes.clear();
        self.new_bytes.clear();
        self.to_state(State::Default);
    }
}

#[derive(Debug)]
enum MarkerMatch {
    None,
    Partial,
    Complete,
}

fn match_marker(byte: u8, marker: &[u8], marker_ix: &mut usize) -> MarkerMatch {
    if byte == marker[*marker_ix] {
        *marker_ix += 1;

        if *marker_ix >= marker.len() {
            MarkerMatch::Complete
        } else {
            MarkerMatch::Partial
        }
    } else {
        MarkerMatch::None
    }
}

fn collect_until_marker(
    byte: u8,
    marker: &[u8],
    nl_marker: &[u8],
    marker_ix: &mut usize,
    buf: &mut Vec<u8>,
) -> bool {
    let marker = if buf.is_empty() {
        // do not require another newline if block is empty
        marker
    } else {
        nl_marker
    };

    match match_marker(byte, marker, marker_ix) {
        MarkerMatch::Complete => true,
        MarkerMatch::Partial => false,
        MarkerMatch::None => {
            if *marker_ix > 0 {
                buf.extend_from_slice(&marker[..*marker_ix]);
                *marker_ix = 0;

                // The beginning of marker might match current byte
                match match_marker(byte, marker, marker_ix) {
                    MarkerMatch::Complete => return true,
                    MarkerMatch::Partial => return false,
                    MarkerMatch::None => { /* no match, keep collecting */ }
                }
            }

            buf.push(byte);

            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

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
        let mut actions = vec![];
        parser.parse_chunk(input, &mut actions);

        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0],
            EditAction::Replace {
                file_path: "src/main.rs".to_string(),
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
        let mut actions = vec![];
        parser.parse_chunk(input, &mut actions);

        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0],
            EditAction::Replace {
                file_path: "src/main.rs".to_string(),
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
        let mut actions = vec![];
        parser.parse_chunk(input, &mut actions);

        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0],
            EditAction::Replace {
                file_path: "src/main.rs".to_string(),
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
        let mut actions = vec![];
        parser.parse_chunk(input, &mut actions);

        assert_eq!(actions.len(), 2);
        assert_eq!(
            actions[0],
            EditAction::Replace {
                file_path: "src/main.rs".to_string(),
                old: "fn original() {}".to_string(),
                new: "fn replacement() {}".to_string(),
            }
        );
        assert_eq!(
            actions[1],
            EditAction::Replace {
                file_path: "src/utils.rs".to_string(),
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
        let mut actions = vec![];
        parser.parse_chunk(input, &mut actions);

        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0],
            EditAction::Replace {
                file_path: "src/main.rs".to_string(),
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
        let mut actions = vec![];
        parser.parse_chunk(input, &mut actions);

        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0],
            EditAction::Write {
                file_path: "src/main.rs".to_string(),
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
        let mut actions = vec![];
        parser.parse_chunk(input, &mut actions);

        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0],
            EditAction::Replace {
                file_path: "src/main.rs".to_string(),
                old: "fn this_will_be_deleted() {\n    println!(\"Deleting this function\");\n}"
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
        let mut actions = vec![];
        parser.parse_chunk(input, &mut actions);

        // Should not create an action when both sections are empty
        assert_eq!(actions.len(), 0);

        // Check that the NoOp error was added
        assert_eq!(parser.errors().len(), 1);
        match parser.errors()[0].1 {
            ParseError::NoOp => {}
            _ => panic!("Expected NoOp error"),
        }
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
        let mut actions = vec![];

        parser.parse_chunk(input_part1, &mut actions);
        assert_eq!(actions.len(), 0);

        parser.parse_chunk(input_part2, &mut actions);
        // No actions should be complete yet
        assert_eq!(actions.len(), 0);

        parser.parse_chunk(input_part3, &mut actions);
        // The third chunk should complete the action
        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0],
            EditAction::Replace {
                file_path: "src/main.rs".to_string(),
                old: "fn original() {}".to_string(),
                new: "fn replacement() {}".to_string(),
            }
        );
    }

    #[test]
    fn test_parser_state_preservation() {
        let mut parser = EditActionParser::new();
        let mut actions = vec![];
        parser.parse_chunk("src/main.rs\n```rust\n<<<<<<< SEARCH\n", &mut actions);

        // Check parser is in the correct state
        assert_eq!(parser.state, State::SearchBlock);
        assert_eq!(parser.pre_fence_line, b"src/main.rs");

        // Continue parsing
        parser.parse_chunk("original code\n=======\n", &mut actions);
        assert_eq!(parser.state, State::ReplaceBlock);
        assert_eq!(parser.old_bytes, b"original code");

        parser.parse_chunk("replacement code\n>>>>>>> REPLACE\n```\n", &mut actions);

        // After complete parsing, state should reset
        assert_eq!(parser.state, State::Default);
        assert!(parser.pre_fence_line.is_empty());
        assert!(parser.old_bytes.is_empty());
        assert!(parser.new_bytes.is_empty());
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
        let mut actions = vec![];
        parser.parse_chunk(input, &mut actions);
        assert_eq!(actions.len(), 0);

        assert_eq!(parser.errors().len(), 1);
        let error = &parser.errors()[0];

        assert_eq!(error.0, 28);
        assert_eq!(
            error.1,
            ParseError::ExpectedMarker {
                expected: b"<<<<<<< SEARCH\n",
                found: b'W'
            }
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
        let mut actions = vec![];
        parser.parse_chunk(input, &mut actions);

        // Only the second block should be parsed
        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0],
            EditAction::Replace {
                file_path: "src/utils.rs".to_string(),
                old: "fn utils_func() {}".to_string(),
                new: "fn new_utils_func() {}".to_string(),
            }
        );

        // The parser should continue after an error
        assert_eq!(parser.state, State::Default);
    }

    const SYSTEM_PROMPT: &str = include_str!("./system.md");

    #[test]
    fn test_parse_examples_in_system_prompt() {
        let mut parser = EditActionParser::new();
        let mut actions = vec![];
        parser.parse_chunk(SYSTEM_PROMPT, &mut actions);
        assert_examples_in_system_prompt(&actions, parser.errors());
    }

    #[test]
    fn test_random_chunking_of_system_prompt() {
        use rand::Rng;

        // Run the test multiple times with different random chunks
        for _ in 0..5 {
            let mut parser = EditActionParser::new();
            let mut remaining = SYSTEM_PROMPT;
            let mut actions = Vec::with_capacity(5);

            while !remaining.is_empty() {
                let mut rng = rand::thread_rng();
                let chunk_size = rng.gen_range(1..=std::cmp::min(remaining.len(), 100));

                let (chunk, rest) = remaining.split_at(chunk_size);

                parser.parse_chunk(chunk, &mut actions);
                remaining = rest;
            }

            assert_examples_in_system_prompt(&actions, parser.errors());
        }
    }

    fn assert_examples_in_system_prompt(actions: &[EditAction], errors: &[(usize, ParseError)]) {
        assert_eq!(actions.len(), 5);

        assert_eq!(
            actions[0],
            EditAction::Replace {
                file_path: "mathweb/flask/app.py".to_string(),
                old: "from flask import Flask".to_string(),
                new: "import math\nfrom flask import Flask".to_string(),
            }
        );

        assert_eq!(
                    actions[1],
                    EditAction::Replace {
                        file_path: "mathweb/flask/app.py".to_string(),
                        old: "def factorial(n):\n    \"compute factorial\"\n\n    if n == 0:\n        return 1\n    else:\n        return n * factorial(n-1)\n".to_string(),
                        new: "".to_string(),
                    }
                );

        assert_eq!(
            actions[2],
            EditAction::Replace {
                file_path: "mathweb/flask/app.py".to_string(),
                old: "    return str(factorial(n))".to_string(),
                new: "    return str(math.factorial(n))".to_string(),
            }
        );

        assert_eq!(
            actions[3],
            EditAction::Write {
                file_path: "hello.py".to_string(),
                content: "def hello():\n    \"print a greeting\"\n\n    print(\"hello\")"
                    .to_string(),
            }
        );

        assert_eq!(
            actions[4],
            EditAction::Replace {
                file_path: "main.py".to_string(),
                old: "def hello():\n    \"print a greeting\"\n\n    print(\"hello\")".to_string(),
                new: "from hello import hello".to_string(),
            }
        );

        // Ensure we have no parsing errors
        assert!(errors.is_empty(), "Parsing errors found: {:?}", errors);
    }
}

use util::ResultExt;

#[derive(Debug, Clone, PartialEq, Eq)]
enum EditAction {
    Replace {
        file_path: String,
        old: String,
        new: String,
    },
    Write {
        file_path: String,
        content: String,
    },
}

#[derive(Debug)]
struct EditActionParser {
    pre_fence_line: Vec<u8>,
    marker_ix: usize,
    old_bytes: Vec<u8>,
    new_bytes: Vec<u8>,
    state: State,
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
    fn new() -> Self {
        Self {
            pre_fence_line: Vec::new(),
            marker_ix: 0,
            old_bytes: Vec::new(),
            new_bytes: Vec::new(),
            state: State::Default,
        }
    }

    fn parse_chunk(&mut self, input: &str) -> Vec<EditAction> {
        use State::*;

        let mut actions = vec![];

        const FENCE: &[u8] = b"\n```";
        const SEARCH_MARKER: &[u8] = b"<<<<<<< SEARCH\n";
        const DIVIDER: &[u8] = b"=======\n";
        const NL_DIVIDER: &[u8] = b"\n=======\n";
        const REPLACE_MARKER: &[u8] = b"\n>>>>>>> REPLACE";

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
                    let divider = if self.old_bytes.is_empty() {
                        // do not require another newline if search block is empty
                        DIVIDER
                    } else {
                        NL_DIVIDER
                    };

                    if collect_until_marker(byte, divider, &mut self.marker_ix, &mut self.old_bytes)
                    {
                        self.to_state(ReplaceBlock);
                    }
                }
                ReplaceBlock => {
                    if collect_until_marker(
                        byte,
                        REPLACE_MARKER,
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
        }

        actions
    }

    fn action(&mut self) -> Option<EditAction> {
        if self.old_bytes.is_empty() && self.new_bytes.is_empty() {
            // todo! record az error
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

    fn expect_marker(&mut self, byte: u8, marker: &[u8]) -> bool {
        match match_marker(byte, marker, &mut self.marker_ix) {
            MarkerMatch::Complete => true,
            MarkerMatch::Partial => false,
            MarkerMatch::None => {
                // todo az: record error
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

fn collect_until_marker(byte: u8, marker: &[u8], marker_ix: &mut usize, buf: &mut Vec<u8>) -> bool {
    // this can't be a method because we'd need to have two mutable references on self
    match match_marker(byte, marker, marker_ix) {
        MarkerMatch::Complete => true,
        MarkerMatch::Partial => false,
        MarkerMatch::None => {
            if *marker_ix > 0 {
                buf.extend_from_slice(&marker[..*marker_ix]);
                *marker_ix = 0;
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
        let actions = parser.parse_chunk(input);

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
        let actions = parser.parse_chunk(input);

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
        let actions = parser.parse_chunk(input);

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
        let actions = parser.parse_chunk(input);

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
        let actions = parser.parse_chunk(input);

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
        let actions = parser.parse_chunk(input);

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
        let actions2 = parser.parse_chunk(input_part2);
        let actions3 = parser.parse_chunk(input_part3);

        // No actions should be complete yet
        assert_eq!(actions1.len(), 0);
        assert_eq!(actions2.len(), 0);

        // The third chunk should complete the action
        assert_eq!(actions3.len(), 1);
        assert_eq!(
            actions3[0],
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
        parser.parse_chunk("src/main.rs\n```rust\n<<<<<<< SEARCH\n");

        // Check parser is in the correct state
        assert_eq!(parser.state, State::SearchBlock);
        assert_eq!(parser.pre_fence_line, b"src/main.rs");

        // Continue parsing
        parser.parse_chunk("original code\n=======\n");
        assert_eq!(parser.state, State::ReplaceBlock);
        assert_eq!(parser.old_bytes, b"original code");

        parser.parse_chunk("replacement code\n>>>>>>> REPLACE\n```\n");

        // After complete parsing, state should reset
        assert_eq!(parser.state, State::Default);
        assert!(parser.pre_fence_line.is_empty());
        assert!(parser.old_bytes.is_empty());
        assert!(parser.new_bytes.is_empty());
    }
}

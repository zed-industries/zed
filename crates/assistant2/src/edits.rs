#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct EditBlock {
    file_path: String,
    search: String,
    replace: String,
}

#[derive(Debug)]
struct EditBlockParser {
    pre_fence_line: Vec<u8>,
    marker_ix: usize,
    search: Vec<u8>,
    replace: Vec<u8>,
    state: State,
}

const FENCE: &[u8] = b"\n```";
const BLOCK_START: &[u8] = b"<<<<<<< SEARCH\n";
const BLOCK_DIVIDER: &[u8] = b"\n=======\n";
const BLOCK_END: &[u8] = b"\n>>>>>>> REPLACE";

#[derive(Debug, PartialEq, Eq)]
enum State {
    Default,
    OpenFence,
    SearchMarker,
    SearchBlock,
    ReplaceBlock,
    CloseFence,
}

impl EditBlockParser {
    fn new() -> Self {
        Self {
            pre_fence_line: Vec::new(),
            marker_ix: 0,
            search: Vec::new(),
            replace: Vec::new(),
            state: State::Default,
        }
    }

    fn parse_chunk(&mut self, input: &str) -> Vec<EditBlock> {
        use State::*;

        let mut blocks = vec![];

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
                    if self.expect_marker(byte, BLOCK_START) {
                        self.to_state(SearchBlock);
                    }
                }
                SearchBlock => {
                    if collect_until_marker(
                        byte,
                        BLOCK_DIVIDER,
                        &mut self.marker_ix,
                        &mut self.search,
                    ) {
                        self.to_state(ReplaceBlock);
                    }
                }
                ReplaceBlock => {
                    if collect_until_marker(byte, BLOCK_END, &mut self.marker_ix, &mut self.replace)
                    {
                        self.to_state(CloseFence);
                    }
                }
                CloseFence => {
                    if self.expect_marker(byte, FENCE) {
                        if let (Ok(file_path), Ok(search), Ok(replace)) = (
                            String::from_utf8(std::mem::take(&mut self.pre_fence_line)),
                            String::from_utf8(std::mem::take(&mut self.search)),
                            String::from_utf8(std::mem::take(&mut self.replace)),
                        ) {
                            blocks.push(EditBlock {
                                file_path,
                                search,
                                replace,
                            })
                        }

                        self.reset();
                    }
                }
            };
        }

        blocks
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
        self.search.clear();
        self.replace.clear();
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
    fn test_simple_edit_block() {
        let input = r#"src/main.rs
```
<<<<<<< SEARCH
fn original() {}
=======
fn replacement() {}
>>>>>>> REPLACE
```
"#;

        let mut parser = EditBlockParser::new();
        let blocks = parser.parse_chunk(input);

        dbg!(parser);

        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0],
            EditBlock {
                file_path: "src/main.rs".to_string(),
                search: "fn original() {}".to_string(),
                replace: "fn replacement() {}".to_string(),
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

        let mut parser = EditBlockParser::new();
        let blocks = parser.parse_chunk(input);

        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0],
            EditBlock {
                file_path: "src/main.rs".to_string(),
                search: "fn original() {}".to_string(),
                replace: "fn replacement() {}".to_string(),
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

        let mut parser = EditBlockParser::new();
        let blocks = parser.parse_chunk(input);

        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0],
            EditBlock {
                file_path: "src/main.rs".to_string(),
                search: "fn original() {}".to_string(),
                replace: "fn replacement() {}".to_string(),
            }
        );
    }

    #[test]
    fn test_multiple_edit_blocks() {
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

        let mut parser = EditBlockParser::new();
        let blocks = parser.parse_chunk(input);

        assert_eq!(blocks.len(), 2);
        assert_eq!(
            blocks[0],
            EditBlock {
                file_path: "src/main.rs".to_string(),
                search: "fn original() {}".to_string(),
                replace: "fn replacement() {}".to_string(),
            }
        );
        assert_eq!(
            blocks[1],
            EditBlock {
                file_path: "src/utils.rs".to_string(),
                search: "fn old_util() -> bool { false }".to_string(),
                replace: "fn new_util() -> bool { true }".to_string(),
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

        let mut parser = EditBlockParser::new();
        let blocks = parser.parse_chunk(input);

        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0],
            EditBlock {
                file_path: "src/main.rs".to_string(),
                search: "fn original() {\n    println!(\"This is the original function\");\n    let x = 42;\n    if x > 0 {\n        println!(\"Positive number\");\n    }\n}".to_string(),
                replace: "fn replacement() {\n    println!(\"This is the replacement function\");\n    let x = 100;\n    if x > 50 {\n        println!(\"Large number\");\n    } else {\n        println!(\"Small number\");\n    }\n}".to_string(),
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

        let mut parser = EditBlockParser::new();
        let blocks1 = parser.parse_chunk(input_part1);
        let blocks2 = parser.parse_chunk(input_part2);
        let blocks3 = parser.parse_chunk(input_part3);

        // No blocks should be complete yet
        assert_eq!(blocks1.len(), 0);
        assert_eq!(blocks2.len(), 0);

        // The third chunk should complete the block
        assert_eq!(blocks3.len(), 1);
        assert_eq!(
            blocks3[0],
            EditBlock {
                file_path: "src/main.rs".to_string(),
                search: "fn original() {}".to_string(),
                replace: "fn replacement() {}".to_string(),
            }
        );
    }

    #[test]
    fn test_parser_state_preservation() {
        let mut parser = EditBlockParser::new();
        parser.parse_chunk("src/main.rs\n```rust\n<<<<<<< SEARCH\n");

        // Check parser is in the correct state
        assert_eq!(parser.state, State::SearchBlock);
        assert_eq!(parser.pre_fence_line, b"src/main.rs");

        // Continue parsing
        parser.parse_chunk("original code\n=======\n");
        assert_eq!(parser.state, State::ReplaceBlock);
        assert_eq!(parser.search, b"original code");

        parser.parse_chunk("replacement code\n>>>>>>> REPLACE\n```\n");

        // After complete parsing, state should reset
        assert_eq!(parser.state, State::Default);
        assert!(parser.pre_fence_line.is_empty());
        assert!(parser.search.is_empty());
        assert!(parser.replace.is_empty());
    }
}

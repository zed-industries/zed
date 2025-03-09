pub const SCRIPT_START_TAG: &str = "<eval type=\"lua\">";
pub const SCRIPT_END_TAG: &str = "</eval>";

const START_TAG: &[u8] = SCRIPT_START_TAG.as_bytes();
const END_TAG: &[u8] = SCRIPT_END_TAG.as_bytes();

/// Parses a script tag in an assistant message as it is being streamed.
pub struct ScriptTagParser {
    state: State,
    buffer: Vec<u8>,
    tag_match_ix: usize,
}

enum State {
    Unstarted,
    Streaming,
    Ended,
}

#[derive(Debug, PartialEq)]
pub struct ChunkOutput {
    /// The chunk with script tags removed.
    pub content: String,
    /// The full script tag content. `None` until closed.
    pub script_source: Option<String>,
}

impl ScriptTagParser {
    /// Create a new script tag parser.
    pub fn new() -> Self {
        Self {
            state: State::Unstarted,
            buffer: Vec::new(),
            tag_match_ix: 0,
        }
    }

    /// Returns true if the parser has found a script tag.
    pub fn found_script(&self) -> bool {
        match self.state {
            State::Unstarted => false,
            State::Streaming | State::Ended => true,
        }
    }

    /// Process a new chunk of input, splitting it into surrounding content and script source.
    pub fn parse_chunk(&mut self, input: &str) -> ChunkOutput {
        let mut content = Vec::with_capacity(input.len());

        for byte in input.bytes() {
            match self.state {
                State::Unstarted => {
                    if collect_until_tag(byte, START_TAG, &mut self.tag_match_ix, &mut content) {
                        self.state = State::Streaming;
                        self.buffer = Vec::with_capacity(1024);
                        self.tag_match_ix = 0;
                    }
                }
                State::Streaming => {
                    if collect_until_tag(byte, END_TAG, &mut self.tag_match_ix, &mut self.buffer) {
                        self.state = State::Ended;
                    }
                }
                State::Ended => content.push(byte),
            }
        }

        let content = unsafe { String::from_utf8_unchecked(content) };

        let script_source = if matches!(self.state, State::Ended) && !self.buffer.is_empty() {
            let source = unsafe { String::from_utf8_unchecked(std::mem::take(&mut self.buffer)) };

            Some(source)
        } else {
            None
        };

        ChunkOutput {
            content,
            script_source,
        }
    }
}

fn collect_until_tag(byte: u8, tag: &[u8], tag_match_ix: &mut usize, buffer: &mut Vec<u8>) -> bool {
    // this can't be a method because it'd require a mutable borrow on both self and self.buffer

    if match_tag_byte(byte, tag, tag_match_ix) {
        *tag_match_ix >= tag.len()
    } else {
        if *tag_match_ix > 0 {
            // push the partially matched tag to the buffer
            buffer.extend_from_slice(&tag[..*tag_match_ix]);
            *tag_match_ix = 0;

            // the tag might start to match again
            if match_tag_byte(byte, tag, tag_match_ix) {
                return *tag_match_ix >= tag.len();
            }
        }

        buffer.push(byte);

        false
    }
}

fn match_tag_byte(byte: u8, tag: &[u8], tag_match_ix: &mut usize) -> bool {
    if byte == tag[*tag_match_ix] {
        *tag_match_ix += 1;
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_complete_tag() {
        let mut parser = ScriptTagParser::new();
        let input = "<eval type=\"lua\">print(\"Hello, World!\")</eval>";
        let result = parser.parse_chunk(input);
        assert_eq!(result.content, "");
        assert_eq!(
            result.script_source,
            Some("print(\"Hello, World!\")".to_string())
        );
    }

    #[test]
    fn test_no_tag() {
        let mut parser = ScriptTagParser::new();
        let input = "No tags here, just plain text";
        let result = parser.parse_chunk(input);
        assert_eq!(result.content, "No tags here, just plain text");
        assert_eq!(result.script_source, None);
    }

    #[test]
    fn test_partial_end_tag() {
        let mut parser = ScriptTagParser::new();

        // Start the tag
        let result = parser.parse_chunk("<eval type=\"lua\">let x = '</e");
        assert_eq!(result.content, "");
        assert_eq!(result.script_source, None);

        // Finish with the rest
        let result = parser.parse_chunk("val' + 'not the end';</eval>");
        assert_eq!(result.content, "");
        assert_eq!(
            result.script_source,
            Some("let x = '</eval' + 'not the end';".to_string())
        );
    }

    #[test]
    fn test_text_before_and_after_tag() {
        let mut parser = ScriptTagParser::new();
        let input = "Before tag <eval type=\"lua\">print(\"Hello\")</eval> After tag";
        let result = parser.parse_chunk(input);
        assert_eq!(result.content, "Before tag  After tag");
        assert_eq!(result.script_source, Some("print(\"Hello\")".to_string()));
    }

    #[test]
    fn test_multiple_chunks_with_surrounding_text() {
        let mut parser = ScriptTagParser::new();

        // First chunk with text before
        let result = parser.parse_chunk("Before script <eval type=\"lua\">local x = 10");
        assert_eq!(result.content, "Before script ");
        assert_eq!(result.script_source, None);

        // Second chunk with script content
        let result = parser.parse_chunk("\nlocal y = 20");
        assert_eq!(result.content, "");
        assert_eq!(result.script_source, None);

        // Last chunk with text after
        let result = parser.parse_chunk("\nprint(x + y)</eval> After script");
        assert_eq!(result.content, " After script");
        assert_eq!(
            result.script_source,
            Some("local x = 10\nlocal y = 20\nprint(x + y)".to_string())
        );

        let result = parser.parse_chunk(" there's more text");
        assert_eq!(result.content, " there's more text");
        assert_eq!(result.script_source, None);
    }

    #[test]
    fn test_partial_start_tag_matching() {
        let mut parser = ScriptTagParser::new();

        // partial match of start tag...
        let result = parser.parse_chunk("<ev");
        assert_eq!(result.content, "");

        // ...that's abandandoned when the < of a real tag is encountered
        let result = parser.parse_chunk("<eval type=\"lua\">script content</eval>");
        // ...so it gets pushed to content
        assert_eq!(result.content, "<ev");
        // ...and the real tag is parsed correctly
        assert_eq!(result.script_source, Some("script content".to_string()));
    }

    #[test]
    fn test_random_chunked_parsing() {
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};
        use std::time::{SystemTime, UNIX_EPOCH};

        let test_inputs = [
            "Before <eval type=\"lua\">print(\"Hello\")</eval> After",
            "No tags here at all",
            "<eval type=\"lua\">local x = 10\nlocal y = 20\nprint(x + y)</eval>",
            "Text <eval type=\"lua\">if true then\nprint(\"nested </e\")\nend</eval> more",
        ];

        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        eprintln!("Using random seed: {}", seed);
        let mut rng = StdRng::seed_from_u64(seed);

        for test_input in &test_inputs {
            let mut reference_parser = ScriptTagParser::new();
            let expected = reference_parser.parse_chunk(test_input);

            let mut chunked_parser = ScriptTagParser::new();
            let mut remaining = test_input.as_bytes();
            let mut actual_content = String::new();
            let mut actual_script = None;

            while !remaining.is_empty() {
                let chunk_size = rng.gen_range(1..=remaining.len().min(5));
                let (chunk, rest) = remaining.split_at(chunk_size);
                remaining = rest;

                let chunk_str = std::str::from_utf8(chunk).unwrap();
                let result = chunked_parser.parse_chunk(chunk_str);

                actual_content.push_str(&result.content);
                if result.script_source.is_some() {
                    actual_script = result.script_source;
                }
            }

            assert_eq!(actual_content, expected.content);
            assert_eq!(actual_script, expected.script_source);
        }
    }
}

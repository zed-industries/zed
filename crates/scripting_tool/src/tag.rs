pub const SCRIPT_START_TAG: &str = "<eval type=\"lua\">";
pub const SCRIPT_END_TAG: &str = "</eval>";

const START_TAG: &[u8] = SCRIPT_START_TAG.as_bytes();
const END_TAG: &[u8] = SCRIPT_END_TAG.as_bytes();

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
    pub content: String,
    pub script_source: Option<String>,
}

impl ScriptTagParser {
    pub fn new() -> Self {
        Self {
            state: State::Unstarted,
            buffer: Vec::new(),
            tag_match_ix: 0,
        }
    }

    pub fn found_script(&self) -> bool {
        match self.state {
            State::Unstarted => false,
            State::Streaming | State::Ended => true,
        }
    }

    pub fn parse_chunk(&mut self, input: &str) -> ChunkOutput {
        let mut content = Vec::with_capacity(input.len());

        for byte in input.bytes() {
            match self.state {
                State::Unstarted => match self.match_tag(byte, START_TAG) {
                    TagMatch::None => {
                        if self.tag_match_ix > 0 {
                            content.extend_from_slice(&START_TAG[..self.tag_match_ix]);
                            self.tag_match_ix = 0;
                        }

                        content.push(byte)
                    }
                    TagMatch::Partial => {}
                    TagMatch::Complete => {
                        self.state = State::Streaming;
                        self.buffer = Vec::with_capacity(1024);
                        self.tag_match_ix = 0;
                    }
                },
                State::Streaming => {
                    // TODO: find some way to escape tag?
                    match self.match_tag(byte, END_TAG) {
                        TagMatch::Complete => {
                            self.state = State::Ended;
                        }
                        TagMatch::Partial => {}
                        TagMatch::None => {
                            if self.tag_match_ix > 0 {
                                // If tag didn't match completely, we assume it's part of the script source
                                self.buffer.extend_from_slice(&END_TAG[..self.tag_match_ix]);
                                self.tag_match_ix = 0;

                                // tag beginning might match current byte
                                match self.match_tag(byte, END_TAG) {
                                    TagMatch::Complete => {
                                        self.state = State::Ended;
                                        continue;
                                    }
                                    TagMatch::Partial => continue,
                                    TagMatch::None => { /* no match, keep collecting */ }
                                }
                            }

                            self.buffer.push(byte);
                        }
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

    fn match_tag(&mut self, byte: u8, tag: &[u8]) -> TagMatch {
        if byte == tag[self.tag_match_ix] {
            self.tag_match_ix += 1;

            if self.tag_match_ix >= tag.len() {
                TagMatch::Complete
            } else {
                TagMatch::Partial
            }
        } else {
            TagMatch::None
        }
    }
}

#[derive(Debug)]
enum TagMatch {
    None,
    Partial,
    Complete,
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
}

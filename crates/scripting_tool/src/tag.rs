const START_TAG: &[u8] = b"<eval type=\"lua\">";
const END_TAG: &[u8] = b"</eval>";

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

impl ScriptTagParser {
    pub fn new() -> Self {
        Self {
            state: State::Unstarted,
            buffer: Vec::new(),
            tag_match_ix: 0,
        }
    }

    pub fn parse_chunk(&mut self, input: &str) -> Option<String> {
        for byte in input.bytes() {
            match self.state {
                State::Unstarted => match self.match_tag(byte, START_TAG) {
                    TagMatch::None | TagMatch::Partial => {}
                    TagMatch::Complete => {
                        self.state = State::Streaming;
                        self.buffer = Vec::with_capacity(1024);
                        self.tag_match_ix = 0;
                    }
                },
                State::Streaming => {
                    // TODO: find some way to escape tag?
                    match self.match_tag(byte, END_TAG) {
                        TagMatch::Complete => return Some(self.ended()),
                        TagMatch::Partial => {}
                        TagMatch::None => {
                            if self.tag_match_ix > 0 {
                                // If tag didn't match completely, we assume it's part of the script source
                                self.buffer.extend_from_slice(&END_TAG[..self.tag_match_ix]);
                                self.tag_match_ix = 0;

                                // tag beginning might match current byte
                                match self.match_tag(byte, END_TAG) {
                                    TagMatch::Complete => return Some(self.ended()),
                                    TagMatch::Partial => continue,
                                    TagMatch::None => { /* no match, keep collecting */ }
                                }
                            }

                            self.buffer.push(byte);
                        }
                    }
                }
                State::Ended => return None,
            }
        }

        return None;
    }

    fn ended(&mut self) -> String {
        self.state = State::Ended;
        self.tag_match_ix = 0;

        let buffer = std::mem::take(&mut self.buffer);
        unsafe { String::from_utf8_unchecked(buffer) }
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
        assert_eq!(result, Some("print(\"Hello, World!\")".to_string()));
    }

    #[test]
    fn test_parse_multiple_chunks() {
        let mut parser = ScriptTagParser::new();

        // First chunk
        let result = parser.parse_chunk("<eval type=\"lua\">print(");
        assert_eq!(result, None);

        // Second chunk
        let result = parser.parse_chunk("\"Hello, World!\")</eval>");
        assert_eq!(result, Some("print(\"Hello, World!\")".to_string()));

        // After done
        let result = parser.parse_chunk("more content");
        assert_eq!(result, None);
    }

    #[test]
    fn test_no_tag() {
        let mut parser = ScriptTagParser::new();
        let input = "No tags here, just plain text";
        let result = parser.parse_chunk(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_partial_end_tag() {
        let mut parser = ScriptTagParser::new();

        // Start the tag
        let result = parser.parse_chunk("<eval type=\"lua\">let x = '</e");
        assert_eq!(result, None);

        // Finish with the rest
        let result = parser.parse_chunk("val' + 'not the end';</eval>");
        assert_eq!(
            result,
            Some("let x = '</eval' + 'not the end';".to_string())
        );
    }
}

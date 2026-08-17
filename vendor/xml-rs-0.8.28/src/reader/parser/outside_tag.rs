use crate::common::is_whitespace_char;
use crate::reader::error::SyntaxError;
use crate::reader::events::XmlEvent;
use crate::reader::lexer::Token;

use super::{
    ClosingTagSubstate, DoctypeSubstate, Encountered, OpeningTagSubstate,
    ProcessingInstructionSubstate, PullParser, Result, State,
};

impl PullParser {
    pub fn outside_tag(&mut self, t: Token) -> Option<Result> {
        match t {
            Token::Character(c) => {
                if is_whitespace_char(c) {
                    // skip whitespace outside of the root element
                    if (self.config.c.trim_whitespace && self.buf.is_empty()) ||
                        (self.depth() == 0 && self.config.c.ignore_root_level_whitespace) {
                            return None;
                    }
                } else {
                    self.inside_whitespace = false;
                    if self.depth() == 0 {
                        return Some(self.error(SyntaxError::UnexpectedTokenOutsideRoot(t)));
                    }
                }

                if !self.is_valid_xml_char_not_restricted(c) {
                    return Some(self.error(SyntaxError::InvalidCharacterEntity(c as u32)));
                }

                if self.buf.is_empty() {
                    self.push_pos();
                } else if self.buf.len() > self.config.max_data_length {
                    return Some(self.error(SyntaxError::ExceededConfiguredLimit));
                }
                self.buf.push(c);
                None
            },

            Token::CommentEnd | Token::TagEnd | Token::EqualsSign |
            Token::DoubleQuote | Token::SingleQuote |
            Token::ProcessingInstructionEnd | Token::EmptyTagEnd => {
                if self.depth() == 0 {
                    return Some(self.error(SyntaxError::UnexpectedTokenOutsideRoot(t)));
                }
                self.inside_whitespace = false;

                if let Some(s) = t.as_static_str() {
                    if self.buf.is_empty() {
                        self.push_pos();
                    } else if self.buf.len() > self.config.max_data_length {
                        return Some(self.error(SyntaxError::ExceededConfiguredLimit));
                    }

                    self.buf.push_str(s);
                }
                None
            },

            Token::ReferenceStart if self.depth() > 0 => {
                self.state_after_reference = State::OutsideTag;
                self.into_state_continue(State::InsideReference)
            },

            Token::ReferenceEnd if self.depth() > 0 => { // Semi-colon in a text outside an entity
                self.inside_whitespace = false;
                if self.buf.len() > self.config.max_data_length {
                    return Some(self.error(SyntaxError::ExceededConfiguredLimit));
                }
                Token::ReferenceEnd.push_to_string(&mut self.buf);
                None
            },

            Token::CommentStart if self.config.c.coalesce_characters && self.config.c.ignore_comments => {
                let next_event = self.set_encountered(Encountered::Comment);
                // We need to switch the lexer into a comment mode inside comments
                self.into_state(State::InsideComment, next_event)
            }

            Token::CDataStart if self.depth() > 0 && self.config.c.coalesce_characters && self.config.c.cdata_to_characters => {
                if self.buf.is_empty() {
                    self.push_pos(); // CDataEnd will pop pos if the buffer remains empty
                }
                // if coalescing chars, continue without event
                self.into_state_continue(State::InsideCData)
            },

            _ => {
                // Encountered some markup event, flush the buffer as characters
                // or a whitespace
                let mut next_event = if self.buf_has_data() {
                    let buf = self.take_buf();
                    if self.inside_whitespace && self.config.c.trim_whitespace {
                        // there will be no event emitted for this, but start of buffering has pushed a pos
                        self.next_pos();
                        None
                    } else if self.inside_whitespace && !self.config.c.whitespace_to_characters {
                        debug_assert!(buf.chars().all(|ch| ch.is_whitespace()), "ws={buf:?}");
                        Some(Ok(XmlEvent::Whitespace(buf)))
                    } else if self.config.c.trim_whitespace {
                        Some(Ok(XmlEvent::Characters(buf.trim_matches(is_whitespace_char).into())))
                    } else {
                        Some(Ok(XmlEvent::Characters(buf)))
                    }
                } else { None };
                self.inside_whitespace = true;  // Reset inside_whitespace flag

                // pos is popped whenever an event is emitted, so pushes must happen only if there will be an event to balance it
                // and ignored comments don't pop
                if t != Token::CommentStart || !self.config.c.ignore_comments {
                    self.push_pos();
                }
                match t {
                    Token::OpeningTagStart if self.depth() > 0 || self.encountered < Encountered::Element || self.config.allow_multiple_root_elements => {
                        if let Some(e) = self.set_encountered(Encountered::Element) {
                            next_event = Some(e);
                        }
                        self.nst.push_empty();
                        self.into_state(State::InsideOpeningTag(OpeningTagSubstate::InsideName), next_event)
                    },

                    Token::ClosingTagStart if self.depth() > 0 =>
                        self.into_state(State::InsideClosingTag(ClosingTagSubstate::CTInsideName), next_event),

                    Token::CommentStart => {
                        if let Some(e) = self.set_encountered(Encountered::Comment) {
                            next_event = Some(e);
                        }
                        // We need to switch the lexer into a comment mode inside comments
                        self.into_state(State::InsideComment, next_event)
                    },

                    Token::DoctypeStart if self.encountered < Encountered::Doctype => {
                        if let Some(e) = self.set_encountered(Encountered::Doctype) {
                            next_event = Some(e);
                        }
                        self.data.doctype = Some(Token::DoctypeStart.to_string());

                        // We don't have a doctype event so skip this position
                        // FIXME: update when we have a doctype event
                        self.next_pos();
                        self.into_state(State::InsideDoctype(DoctypeSubstate::Outside), next_event)
                    },

                    Token::ProcessingInstructionStart =>
                        self.into_state(State::InsideProcessingInstruction(ProcessingInstructionSubstate::PIInsideName), next_event),

                    Token::CDataStart if self.depth() > 0 => {
                        self.into_state(State::InsideCData, next_event)
                    },

                    _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
                }
            },
        }
    }

    pub fn document_start(&mut self, t: Token) -> Option<Result> {
        debug_assert!(self.encountered < Encountered::Declaration);

        match t {
            Token::Character(c) => {
                let next_event = self.set_encountered(Encountered::AnyChars);

                if !is_whitespace_char(c) {
                    return Some(self.error(SyntaxError::UnexpectedTokenOutsideRoot(t)));
                }
                self.inside_whitespace = true;

                // skip whitespace outside of the root element
                if (self.config.c.trim_whitespace && self.buf.is_empty()) ||
                    (self.depth() == 0 && self.config.c.ignore_root_level_whitespace) {
                        return self.into_state(State::OutsideTag, next_event);
                }

                self.push_pos();
                self.buf.push(c);
                self.into_state(State::OutsideTag, next_event)
            },

            Token::CommentStart => {
                let next_event = self.set_encountered(Encountered::Comment);
                self.into_state(State::InsideComment, next_event)
            },

            Token::OpeningTagStart => {
                let next_event = self.set_encountered(Encountered::Element);
                self.nst.push_empty();
                self.into_state(State::InsideOpeningTag(OpeningTagSubstate::InsideName), next_event)
            },

            Token::DoctypeStart => {
                let next_event = self.set_encountered(Encountered::Doctype);
                self.data.doctype = Some(Token::DoctypeStart.to_string());

                // We don't have a doctype event so skip this position
                // FIXME: update when we have a doctype event
                self.next_pos();
                self.into_state(State::InsideDoctype(DoctypeSubstate::Outside), next_event)
            },

            Token::ProcessingInstructionStart => {
                self.push_pos();
                self.into_state_continue(State::InsideProcessingInstruction(ProcessingInstructionSubstate::PIInsideName))
            },

            _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
        }
    }
}

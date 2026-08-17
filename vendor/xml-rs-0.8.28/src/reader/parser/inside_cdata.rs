use crate::common::is_whitespace_char;
use crate::reader::error::SyntaxError;
use crate::reader::events::XmlEvent;
use crate::reader::lexer::Token;

use super::{PullParser, Result, State};

impl PullParser {
    pub fn inside_cdata(&mut self, t: Token) -> Option<Result> {
        match t {
            Token::CDataEnd => {
                let event = if self.config.c.cdata_to_characters {
                    // start called push_pos, but there will be no event to pop it
                    if self.buf.is_empty() {
                        self.next_pos();
                    }
                    None
                } else {
                    let data = self.take_buf();
                    Some(Ok(XmlEvent::CData(data)))
                };
                self.into_state(State::OutsideTag, event)
            },

            Token::Character(c) if !self.is_valid_xml_char(c) => {
                Some(self.error(SyntaxError::InvalidCharacterEntity(c as u32)))
            },
            Token::Character(c) => {
                if !is_whitespace_char(c) {
                    self.inside_whitespace = false;
                }
                self.buf.push(c);
                None
            },
            _ => {
                debug_assert!(false, "unreachable");
                None
            },
        }
    }
}

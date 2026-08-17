use crate::reader::error::SyntaxError;
use crate::reader::events::XmlEvent;
use crate::reader::lexer::Token;

use super::{PullParser, Result, State};

impl PullParser {
    pub fn inside_comment(&mut self, t: Token) -> Option<Result> {
        match t {
            Token::CommentEnd if self.config.c.ignore_comments => {
                self.into_state_continue(State::OutsideTag)
            }

            Token::CommentEnd => {
                let data = self.take_buf();
                self.into_state_emit(State::OutsideTag, Ok(XmlEvent::Comment(data)))
            },

            Token::Character(c) if !self.is_valid_xml_char(c) => {
                Some(self.error(SyntaxError::InvalidCharacterEntity(c as u32)))
            },

            _ if self.config.c.ignore_comments => None, // Do not modify buffer if ignoring the comment

            _ => {
                if self.buf.len() > self.config.max_data_length {
                    return Some(self.error(SyntaxError::ExceededConfiguredLimit));
                }
                t.push_to_string(&mut self.buf);
                None
            },
        }
    }
}

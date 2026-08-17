use super::{ClosingTagSubstate, PullParser, QualifiedNameTarget, Result, State};
use crate::common::is_whitespace_char;
use crate::namespace;
use crate::reader::error::SyntaxError;
use crate::reader::lexer::Token;

impl PullParser {
    pub fn inside_closing_tag_name(&mut self, t: Token, s: ClosingTagSubstate) -> Option<Result> {
        match s {
            ClosingTagSubstate::CTInsideName => self.read_qualified_name(t, QualifiedNameTarget::ClosingTagNameTarget, |this, token, name| {
                match name.prefix_ref() {
                    Some(prefix) if prefix == namespace::NS_XML_PREFIX ||
                                    prefix == namespace::NS_XMLNS_PREFIX =>
                        Some(this.error(SyntaxError::InvalidNamePrefix(prefix.into()))),
                    _ => {
                        this.data.element_name = Some(name.clone());
                        match token {
                            Token::TagEnd => this.emit_end_element(),
                            Token::Character(c) if is_whitespace_char(c) => this.into_state_continue(State::InsideClosingTag(ClosingTagSubstate::CTAfterName)),
                            _ => Some(this.error(SyntaxError::UnexpectedTokenInClosingTag(token))),
                        }
                    }
                }
            }),
            ClosingTagSubstate::CTAfterName => match t {
                Token::TagEnd => self.emit_end_element(),
                Token::Character(c) if is_whitespace_char(c) => None, //  Skip whitespace
                _ => Some(self.error(SyntaxError::UnexpectedTokenInClosingTag(t))),
            },
        }
    }
}

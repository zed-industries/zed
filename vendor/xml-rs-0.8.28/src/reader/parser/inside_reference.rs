use super::{PullParser, Result, State};
use crate::common::{is_name_char, is_name_start_char, is_whitespace_char};
use crate::reader::error::SyntaxError;
use crate::reader::lexer::Token;
use std::char;

impl PullParser {
    pub fn inside_reference(&mut self, t: Token) -> Option<Result> {
        match t {
            Token::Character(c) if !self.data.ref_data.is_empty() && is_name_char(c) ||
                             self.data.ref_data.is_empty() && (is_name_start_char(c) || c == '#') => {
                self.data.ref_data.push(c);
                None
            },

            Token::ReferenceEnd => {
                let name = self.data.take_ref_data();
                if name.is_empty() {
                    return Some(self.error(SyntaxError::EmptyEntity));
                }

                let c = match &*name {
                    "lt"   => Some('<'),
                    "gt"   => Some('>'),
                    "amp"  => Some('&'),
                    "apos" => Some('\''),
                    "quot" => Some('"'),
                    _ if name.starts_with('#') => match self.numeric_reference_from_str(&name[1..]) {
                        Ok(c) => Some(c),
                        Err(e) => return Some(self.error(e)),
                    },
                    _ => None,
                };
                if let Some(c) = c {
                    self.buf.push(c);
                } else if let Some(v) = self.config.c.extra_entities.get(&name) {
                    self.buf.push_str(v);
                } else if let Some(v) = self.entities.get(&name) {
                    if self.state_after_reference == State::OutsideTag {
                        // an entity can expand to *elements*, so outside of a tag it needs a full reparse
                        if let Err(e) = self.lexer.reparse(v) {
                            return Some(Err(e));
                        }
                    } else {
                        // however, inside attributes it's not allowed to affect attribute quoting,
                        // so it can't be fed to the lexer
                        self.buf.push_str(v);
                    }
                } else {
                    return Some(self.error(SyntaxError::UnexpectedEntity(name.into())));
                }
                let prev_st = self.state_after_reference;
                if prev_st == State::OutsideTag && !is_whitespace_char(self.buf.chars().last().unwrap_or('\0')) {
                    self.inside_whitespace = false;
                }
                self.into_state_continue(prev_st)
            },

            _ => Some(self.error(SyntaxError::UnexpectedTokenInEntity(t))),
        }
    }

    pub(crate) fn numeric_reference_from_str(&self, num_str: &str) -> std::result::Result<char, SyntaxError> {
        let val = if let Some(hex) = num_str.strip_prefix('x') {
            u32::from_str_radix(hex, 16).map_err(move |_| SyntaxError::InvalidNumericEntity(num_str.into()))?
        } else {
            num_str.parse::<u32>().map_err(move |_| SyntaxError::InvalidNumericEntity(num_str.into()))?
        };
        match char::from_u32(val) {
            Some(c) if self.is_valid_xml_char(c) => Ok(c),
            Some(_) if self.config.c.replace_unknown_entity_references => Ok('\u{fffd}'),
            None if self.config.c.replace_unknown_entity_references => Ok('\u{fffd}'),
            _ => Err(SyntaxError::InvalidCharacterEntity(val)),
        }
    }
}

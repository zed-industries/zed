use crate::attribute::OwnedAttribute;
use crate::common::{is_name_start_char, is_whitespace_char};
use crate::namespace;
use crate::reader::error::SyntaxError;

use crate::reader::lexer::Token;

use super::{OpeningTagSubstate, PullParser, QualifiedNameTarget, Result, State};

impl PullParser {
    pub fn inside_opening_tag(&mut self, t: Token, s: OpeningTagSubstate) -> Option<Result> {
        let max_attrs = self.config.max_attributes;
        match s {
            OpeningTagSubstate::InsideName => self.read_qualified_name(t, QualifiedNameTarget::OpeningTagNameTarget, |this, token, name| {
                match name.prefix_ref() {
                    Some(prefix) if prefix == namespace::NS_XML_PREFIX ||
                                    prefix == namespace::NS_XMLNS_PREFIX =>
                        Some(this.error(SyntaxError::InvalidNamePrefix(prefix.into()))),
                    _ => {
                        this.data.element_name = Some(name.clone());
                        match token {
                            Token::TagEnd => this.emit_start_element(false),
                            Token::EmptyTagEnd => this.emit_start_element(true),
                            Token::Character(c) if is_whitespace_char(c) => this.into_state_continue(State::InsideOpeningTag(OpeningTagSubstate::InsideTag)),
                            _ => {
                                debug_assert!(false, "unreachable");
                                None
                            },
                        }
                    }
                }
            }),

            OpeningTagSubstate::InsideTag => match t {
                Token::TagEnd => self.emit_start_element(false),
                Token::EmptyTagEnd => self.emit_start_element(true),
                Token::Character(c) if is_whitespace_char(c) => None, // skip whitespace
                Token::Character(c) if is_name_start_char(c) => {
                    if self.buf.len() > self.config.max_name_length {
                        return Some(self.error(SyntaxError::ExceededConfiguredLimit));
                    }
                    self.buf.push(c);
                    self.into_state_continue(State::InsideOpeningTag(OpeningTagSubstate::InsideAttributeName))
                },
                _ => Some(self.error(SyntaxError::UnexpectedTokenInOpeningTag(t))),
            },

            OpeningTagSubstate::InsideAttributeName => self.read_qualified_name(t, QualifiedNameTarget::AttributeNameTarget, |this, token, name| {
                // check that no attribute with such name is already present
                // if there is one, XML is not well-formed
                if this.data.attributes.contains(&name) {
                    return Some(this.error(SyntaxError::RedefinedAttribute(name.to_string().into())))
                }

                this.data.attr_name = Some(name);
                match token {
                    Token::EqualsSign => this.into_state_continue(State::InsideOpeningTag(OpeningTagSubstate::InsideAttributeValue)),
                    Token::Character(c) if is_whitespace_char(c) => this.into_state_continue(State::InsideOpeningTag(OpeningTagSubstate::AfterAttributeName)),
                    _ => Some(this.error(SyntaxError::UnexpectedTokenInOpeningTag(t))) // likely unreachable
                }
            }),

            OpeningTagSubstate::AfterAttributeName => match t {
                Token::EqualsSign => self.into_state_continue(State::InsideOpeningTag(OpeningTagSubstate::InsideAttributeValue)),
                Token::Character(c) if is_whitespace_char(c) => None,
                _ => Some(self.error(SyntaxError::UnexpectedTokenInOpeningTag(t)))
            },

            OpeningTagSubstate::InsideAttributeValue => self.read_attribute_value(t, |this, value| {
                let name = this.data.take_attr_name()?;  // will always succeed here
                match name.prefix_ref() {
                    // declaring a new prefix; it is sufficient to check prefix only
                    // because "xmlns" prefix is reserved
                    Some(namespace::NS_XMLNS_PREFIX) => {
                        let ln = &*name.local_name;
                        if ln == namespace::NS_XMLNS_PREFIX {
                            Some(this.error(SyntaxError::CannotRedefineXmlnsPrefix))
                        } else if ln == namespace::NS_XML_PREFIX && &*value != namespace::NS_XML_URI {
                            Some(this.error(SyntaxError::CannotRedefineXmlPrefix))
                        } else if value.is_empty() {
                            Some(this.error(SyntaxError::CannotUndefinePrefix(ln.into())))
                        } else {
                            this.nst.put(name.local_name.clone(), value);
                            this.into_state_continue(State::InsideOpeningTag(OpeningTagSubstate::AfterAttributeValue))
                        }
                    },

                    // declaring default namespace
                    None if &*name.local_name == namespace::NS_XMLNS_PREFIX =>
                        match &*value {
                            namespace::NS_XMLNS_PREFIX | namespace::NS_XML_PREFIX | namespace::NS_XML_URI | namespace::NS_XMLNS_URI =>
                                Some(this.error(SyntaxError::InvalidDefaultNamespace(value.into()))),
                            _ => {
                                this.nst.put(namespace::NS_NO_PREFIX, value.clone());
                                this.into_state_continue(State::InsideOpeningTag(OpeningTagSubstate::AfterAttributeValue))
                            }
                        },

                    // regular attribute
                    _ => {
                        if this.data.attributes.len() >= max_attrs {
                            return Some(this.error(SyntaxError::ExceededConfiguredLimit));
                        }
                        this.data.attributes.push(OwnedAttribute { name, value });
                        this.into_state_continue(State::InsideOpeningTag(OpeningTagSubstate::AfterAttributeValue))
                    },
                }
            }),

            OpeningTagSubstate::AfterAttributeValue => match t {
                Token::Character(c) if is_whitespace_char(c) => {
                    self.into_state_continue(State::InsideOpeningTag(OpeningTagSubstate::InsideTag))
                },
                Token::TagEnd => self.emit_start_element(false),
                Token::EmptyTagEnd => self.emit_start_element(true),
                _ => Some(self.error(SyntaxError::UnexpectedTokenInOpeningTag(t))),
            },
        }
    }
}

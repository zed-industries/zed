use crate::common::{is_whitespace_char, XmlVersion};
use crate::reader::error::SyntaxError;
use crate::reader::events::XmlEvent;
use crate::reader::lexer::Token;
use crate::util::Encoding;

use super::{
    DeclarationSubstate, Encountered, PullParser, QualifiedNameTarget, Result, State,
    DEFAULT_VERSION,
};

impl PullParser {
    #[inline(never)]
    fn emit_start_document(&mut self) -> Option<Result> {
        debug_assert!(self.encountered == Encountered::None);
        self.encountered = Encountered::Declaration;

        let version = self.data.version;
        let encoding = self.data.take_encoding();
        let standalone = self.data.standalone;

        if let Some(new_encoding) = encoding.as_deref() {
            let new_encoding = match new_encoding.parse() {
                Ok(e) => e,
                Err(_) if self.config.ignore_invalid_encoding_declarations => Encoding::Latin1,
                Err(_) => return Some(self.error(SyntaxError::UnsupportedEncoding(new_encoding.into()))),
            };
            let current_encoding = self.lexer.encoding();
            if current_encoding != new_encoding {
                let set = match (current_encoding, new_encoding) {
                    (Encoding::Unknown | Encoding::Default, new) if new != Encoding::Utf16 => new,
                    (Encoding::Utf16Be | Encoding::Utf16Le, Encoding::Utf16) => current_encoding,
                    _ if self.config.ignore_invalid_encoding_declarations => current_encoding,
                    _ => return Some(self.error(SyntaxError::ConflictingEncoding(new_encoding, current_encoding))),
                };
                self.lexer.set_encoding(set);
            }
        }

        let current_encoding = self.lexer.encoding();
        self.into_state_emit(State::OutsideTag, Ok(XmlEvent::StartDocument {
            version: version.unwrap_or(DEFAULT_VERSION),
            encoding: encoding.unwrap_or_else(move || current_encoding.to_string()),
            standalone
        }))
    }

    // TODO: remove redundancy via macros or extra methods
    pub fn inside_declaration(&mut self, t: Token, s: DeclarationSubstate) -> Option<Result> {

        match s {
            DeclarationSubstate::BeforeVersion => match t {
                Token::Character('v') => self.into_state_continue(State::InsideDeclaration(DeclarationSubstate::InsideVersion)),
                Token::Character(c) if is_whitespace_char(c) => None,  // continue
                _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
            },

            DeclarationSubstate::InsideVersion => self.read_qualified_name(t, QualifiedNameTarget::AttributeNameTarget, |this, token, name| {
                match &*name.local_name {
                    "ersion" if name.namespace.is_none() =>
                        this.into_state_continue(State::InsideDeclaration(
                            if token == Token::EqualsSign {
                                DeclarationSubstate::InsideVersionValue
                            } else {
                                DeclarationSubstate::AfterVersion
                            }
                        )),
                    _ => Some(this.error(SyntaxError::UnexpectedNameInsideXml(name.to_string().into()))),
                }
            }),

            DeclarationSubstate::AfterVersion => match t {
                Token::EqualsSign => self.into_state_continue(State::InsideDeclaration(DeclarationSubstate::InsideVersionValue)),
                Token::Character(c) if is_whitespace_char(c) => None,
                _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
            },

            DeclarationSubstate::InsideVersionValue => self.read_attribute_value(t, |this, value| {
                this.data.version = match &*value {
                    "1.0" => Some(XmlVersion::Version10),
                    "1.1" => Some(XmlVersion::Version11),
                    _     => None
                };
                if this.data.version.is_some() {
                    this.into_state_continue(State::InsideDeclaration(DeclarationSubstate::AfterVersionValue))
                } else {
                    Some(this.error(SyntaxError::UnexpectedXmlVersion(value.into())))
                }
            }),

            DeclarationSubstate::AfterVersionValue => match t {
                Token::Character(c) if is_whitespace_char(c) => self.into_state_continue(State::InsideDeclaration(DeclarationSubstate::BeforeEncoding)),
                Token::ProcessingInstructionEnd => self.emit_start_document(),
                _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
            },

            DeclarationSubstate::BeforeEncoding => match t {
                Token::Character('e') => self.into_state_continue(State::InsideDeclaration(DeclarationSubstate::InsideEncoding)),
                Token::Character('s') => self.into_state_continue(State::InsideDeclaration(DeclarationSubstate::InsideStandaloneDecl)),
                Token::ProcessingInstructionEnd => self.emit_start_document(),
                Token::Character(c) if is_whitespace_char(c) => None,  // skip whitespace
                _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
            },

            DeclarationSubstate::InsideEncoding => self.read_qualified_name(t, QualifiedNameTarget::AttributeNameTarget, |this, token, name| {
                match &*name.local_name {
                    "ncoding" if name.namespace.is_none() =>
                        this.into_state_continue(State::InsideDeclaration(
                            if token == Token::EqualsSign { DeclarationSubstate::InsideEncodingValue } else { DeclarationSubstate::AfterEncoding }
                        )),
                    _ => Some(this.error(SyntaxError::UnexpectedName(name.to_string().into())))
                }
            }),

            DeclarationSubstate::AfterEncoding => match t {
                Token::EqualsSign => self.into_state_continue(State::InsideDeclaration(DeclarationSubstate::InsideEncodingValue)),
                Token::Character(c) if is_whitespace_char(c) => None,
                _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
            },

            DeclarationSubstate::InsideEncodingValue => self.read_attribute_value(t, |this, value| {
                this.data.encoding = Some(value);
                this.into_state_continue(State::InsideDeclaration(DeclarationSubstate::AfterEncodingValue))
            }),

            DeclarationSubstate::AfterEncodingValue => match t {
                Token::Character(c) if is_whitespace_char(c) => self.into_state_continue(State::InsideDeclaration(DeclarationSubstate::BeforeStandaloneDecl)),
                Token::ProcessingInstructionEnd => self.emit_start_document(),
                _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
            },

            DeclarationSubstate::BeforeStandaloneDecl => match t {
                Token::Character('s') => self.into_state_continue(State::InsideDeclaration(DeclarationSubstate::InsideStandaloneDecl)),
                Token::ProcessingInstructionEnd => self.emit_start_document(),
                Token::Character(c) if is_whitespace_char(c) => None, // skip whitespace
                _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
            },

            DeclarationSubstate::InsideStandaloneDecl => self.read_qualified_name(t, QualifiedNameTarget::AttributeNameTarget, |this, token, name| {
                match &*name.local_name {
                    "tandalone" if name.namespace.is_none() =>
                        this.into_state_continue(State::InsideDeclaration(
                            if token == Token::EqualsSign {
                                DeclarationSubstate::InsideStandaloneDeclValue
                            } else {
                                DeclarationSubstate::AfterStandaloneDecl
                            }
                        )),
                    _ => Some(this.error(SyntaxError::UnexpectedName(name.to_string().into()))),
                }
            }),

            DeclarationSubstate::AfterStandaloneDecl => match t {
                Token::EqualsSign => self.into_state_continue(State::InsideDeclaration(DeclarationSubstate::InsideStandaloneDeclValue)),
                Token::Character(c) if is_whitespace_char(c) => None,
                _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
            },

            DeclarationSubstate::InsideStandaloneDeclValue => self.read_attribute_value(t, |this, value| {
                let standalone = match &*value {
                    "yes" => Some(true),
                    "no"  => Some(false),
                    _     => None
                };
                if standalone.is_some() {
                    this.data.standalone = standalone;
                    this.into_state_continue(State::InsideDeclaration(DeclarationSubstate::AfterStandaloneDeclValue))
                } else {
                    Some(this.error(SyntaxError::InvalidStandaloneDeclaration(value.into())))
                }
            }),

            DeclarationSubstate::AfterStandaloneDeclValue => match t {
                Token::ProcessingInstructionEnd => self.emit_start_document(),
                Token::Character(c) if is_whitespace_char(c) => None, // skip whitespace
                _ => Some(self.error(SyntaxError::UnexpectedToken(t))),
            },
        }
    }
}

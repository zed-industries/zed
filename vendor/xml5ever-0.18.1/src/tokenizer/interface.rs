// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;

use crate::tendril::StrTendril;
use crate::{Attribute, QualName};

pub use self::TagKind::{EmptyTag, EndTag, ShortTag, StartTag};
pub use self::Token::{CharacterTokens, EOFToken, NullCharacterToken, ParseError};
pub use self::Token::{CommentToken, DoctypeToken, PIToken, TagToken};

use super::states;

/// Tag kind denotes which kind of tag did we encounter.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TagKind {
    /// Beginning of a tag (e.g. `<a>`).
    StartTag,
    /// End of a tag (e.g. `</a>`).
    EndTag,
    /// Empty tag (e.g. `<a/>`).
    EmptyTag,
    /// Short tag (e.g. `</>`).
    ShortTag,
}

/// XML 5 Tag Token
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Tag {
    /// Token kind denotes which type of token was encountered.
    /// E.g. if parser parsed `</a>` the token kind would be `EndTag`.
    pub kind: TagKind,
    /// Qualified name of the tag.
    pub name: QualName,
    /// List of attributes attached to this tag.
    /// Only valid in start and empty tag.
    pub attrs: Vec<Attribute>,
}

impl Tag {
    /// Sorts attributes in a tag.
    pub fn equiv_modulo_attr_order(&self, other: &Tag) -> bool {
        if (self.kind != other.kind) || (self.name != other.name) {
            return false;
        }

        let mut self_attrs = self.attrs.clone();
        let mut other_attrs = other.attrs.clone();
        self_attrs.sort();
        other_attrs.sort();

        self_attrs == other_attrs
    }
}

/// A `DOCTYPE` token.
/// Doctype token in XML5 is rather limited for reasons, such as:
/// security and simplicity. XML5 only supports declaring DTD with
/// name, public identifier and system identifier
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Doctype {
    /// Name of DOCTYPE declared
    pub name: Option<StrTendril>,
    /// Public identifier of this DOCTYPE.
    pub public_id: Option<StrTendril>,
    /// System identifier of this DOCTYPE.
    pub system_id: Option<StrTendril>,
}

/// A ProcessingInstruction token.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Pi {
    /// What is the name of processing instruction.
    pub target: StrTendril,

    /// Text of processing instruction.
    pub data: StrTendril,
}

/// Describes tokens encountered during parsing of input.
#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    /// Doctype token
    DoctypeToken(Doctype),
    /// Token tag founds. This token applies to all
    /// possible kinds of tags (like start, end, empty tag, etc.).
    TagToken(Tag),
    /// Processing Instruction token
    PIToken(Pi),
    /// Comment token.
    CommentToken(StrTendril),
    /// Token that represents a series of characters.
    CharacterTokens(StrTendril),
    /// End of File found.
    EOFToken,
    /// NullCharacter encountered.
    NullCharacterToken,
    /// Error happened
    ParseError(Cow<'static, str>),
}

/// Types which can receive tokens from the tokenizer.
pub trait TokenSink {
    /// Process a token.
    fn process_token(&mut self, token: Token);

    /// Signal to the sink that parsing has ended.
    fn end(&mut self) {}

    /// The tokenizer will call this after emitting any start tag.
    /// This allows the tree builder to change the tokenizer's state.
    /// By default no state changes occur.
    fn query_state_change(&mut self) -> Option<states::XmlState> {
        None
    }
}

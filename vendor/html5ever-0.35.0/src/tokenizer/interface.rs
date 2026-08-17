// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::interface::Attribute;
use crate::tendril::StrTendril;
use crate::tokenizer::states;
use crate::LocalName;
use std::borrow::Cow;

pub use self::TagKind::{EndTag, StartTag};
pub use self::Token::{CharacterTokens, CommentToken, DoctypeToken, TagToken};
pub use self::Token::{EOFToken, NullCharacterToken, ParseError};

/// A `DOCTYPE` token.
// FIXME: already exists in Servo DOM
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Doctype {
    pub name: Option<StrTendril>,
    pub public_id: Option<StrTendril>,
    pub system_id: Option<StrTendril>,
    pub force_quirks: bool,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TagKind {
    StartTag,
    EndTag,
}

/// A tag token.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Tag {
    pub kind: TagKind,
    pub name: LocalName,
    pub self_closing: bool,
    pub attrs: Vec<Attribute>,
}

impl Tag {
    /// Are the tags equivalent when we don't care about attribute order?
    /// Also ignores the self-closing flag.
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

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    DoctypeToken(Doctype),
    TagToken(Tag),
    CommentToken(StrTendril),
    CharacterTokens(StrTendril),
    NullCharacterToken,
    EOFToken,
    ParseError(Cow<'static, str>),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum TokenSinkResult<Handle> {
    Continue,
    Script(Handle),
    Plaintext,
    RawData(states::RawKind),
}

/// Types which can receive tokens from the tokenizer.
pub trait TokenSink {
    type Handle;

    /// Process a token.
    fn process_token(&self, token: Token, line_number: u64) -> TokenSinkResult<Self::Handle>;

    // Signal sink that tokenization reached the end.
    fn end(&self) {}

    /// Used in the markup declaration open state. By default, this always
    /// returns false and thus all CDATA sections are tokenized as bogus
    /// comments.
    /// <https://html.spec.whatwg.org/multipage/#markup-declaration-open-state>
    fn adjusted_current_node_present_but_not_in_html_namespace(&self) -> bool {
        false
    }
}

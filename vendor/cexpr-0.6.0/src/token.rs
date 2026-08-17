// (C) Copyright 2016 Jethro G. Beekman
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! Representation of a C token
//!
//! This is designed to map onto a libclang CXToken.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Kind {
    Punctuation,
    Keyword,
    Identifier,
    Literal,
    Comment,
}

/// A single token in a C expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The type of this token.
    pub kind: Kind,
    /// The bytes that make up the token.
    pub raw: Box<[u8]>,
}

impl<'a> From<(Kind, &'a [u8])> for Token {
    fn from((kind, value): (Kind, &'a [u8])) -> Token {
        Token {
            kind,
            raw: value.to_owned().into_boxed_slice(),
        }
    }
}

/// Remove all comment tokens from a vector of tokens
pub fn remove_comments(v: &mut Vec<Token>) -> &mut Vec<Token> {
    v.retain(|t| t.kind != Kind::Comment);
    v
}

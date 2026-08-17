// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use self::Token::*;
pub use self::XmlPhase::*;
pub use self::XmlProcessResult::*;

use crate::tendril::StrTendril;
use crate::tokenizer::{Doctype, Pi, Tag};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum XmlPhase {
    Start,
    Main,
    End,
}

/// A subset/refinement of `tokenizer::XToken`.  Everything else is handled
/// specially at the beginning of `process_token`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    Tag(Tag),
    Doctype(Doctype),
    Comment(StrTendril),
    Characters(StrTendril),
    Pi(Pi),
    NullCharacter,
    Eof,
}

pub enum XmlProcessResult {
    Done,
    Reprocess(XmlPhase, Token),
}

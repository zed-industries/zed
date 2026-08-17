// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Tokenizer states.
//!
//! This is public for use by the tokenizer tests.  Other library
//! users should not have to care about this.

pub use self::AttrValueKind::*;
pub use self::DoctypeIdKind::*;
pub use self::RawKind::*;
pub use self::ScriptEscapeKind::*;
pub use self::State::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
pub enum ScriptEscapeKind {
    Escaped,
    DoubleEscaped,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
pub enum DoctypeIdKind {
    Public,
    System,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
pub enum RawKind {
    Rcdata,
    Rawtext,
    ScriptData,
    ScriptDataEscaped(ScriptEscapeKind),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
pub enum AttrValueKind {
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
pub enum State {
    Data,
    Plaintext,
    TagOpen,
    EndTagOpen,
    TagName,
    RawData(RawKind),
    RawLessThanSign(RawKind),
    RawEndTagOpen(RawKind),
    RawEndTagName(RawKind),
    ScriptDataEscapeStart(ScriptEscapeKind),
    ScriptDataEscapeStartDash,
    ScriptDataEscapedDash(ScriptEscapeKind),
    ScriptDataEscapedDashDash(ScriptEscapeKind),
    ScriptDataDoubleEscapeEnd,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValue(AttrValueKind),
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,
    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThanSign,
    CommentLessThanSignBang,
    CommentLessThanSignBangDash,
    CommentLessThanSignBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,
    Doctype,
    BeforeDoctypeName,
    DoctypeName,
    AfterDoctypeName,
    AfterDoctypeKeyword(DoctypeIdKind),
    BeforeDoctypeIdentifier(DoctypeIdKind),
    DoctypeIdentifierDoubleQuoted(DoctypeIdKind),
    DoctypeIdentifierSingleQuoted(DoctypeIdKind),
    AfterDoctypeIdentifier(DoctypeIdKind),
    BetweenDoctypePublicAndSystemIdentifiers,
    BogusDoctype,
    CdataSection,
    CdataSectionBracket,
    CdataSectionEnd,
}

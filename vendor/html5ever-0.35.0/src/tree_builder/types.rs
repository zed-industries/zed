// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Types used within the tree builder code. Not exported to users.

use crate::tokenizer::states::RawKind;
use crate::tokenizer::Tag;

use crate::tendril::StrTendril;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub(crate) enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoscript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    InSelect,
    InSelectInTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub(crate) enum SplitStatus {
    NotSplit,
    Whitespace,
    NotWhitespace,
}

/// A subset/refinement of `tokenizer::Token`.  Everything else is handled
/// specially at the beginning of `process_token`.
#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum Token {
    Tag(Tag),
    Comment(StrTendril),
    Characters(SplitStatus, StrTendril),
    NullCharacter,
    Eof,
}

pub(crate) enum ProcessResult<Handle> {
    Done,
    DoneAckSelfClosing,
    SplitWhitespace(StrTendril),
    Reprocess(InsertionMode, Token),
    #[allow(dead_code)] // FIXME
    ReprocessForeign(Token),
    Script(Handle),
    ToPlaintext,
    ToRawData(RawKind),
}

pub(crate) enum FormatEntry<Handle> {
    Element(Handle, Tag),
    Marker,
}

pub(crate) enum InsertionPoint<Handle> {
    /// Insert as last child in this parent.
    LastChild(Handle),
    #[allow(dead_code)] // FIXME
    /// Insert before this following sibling.
    BeforeSibling(Handle),
    /// Insertion point is decided based on existence of element's parent node.
    TableFosterParenting {
        element: Handle,
        prev_element: Handle,
    },
}

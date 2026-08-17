//! Common traits and types related to parsing our IR from Clang cursors.
#![deny(clippy::missing_docs_in_private_items)]

use crate::clang;
use crate::ir::context::{BindgenContext, ItemId};

/// Not so much an error in the traditional sense, but a control flow message
/// when walking over Clang's AST with a cursor.
#[derive(Debug)]
pub(crate) enum ParseError {
    /// Recurse down the current AST node's children.
    Recurse,
    /// Continue on to the next sibling AST node, or back up to the parent's
    /// siblings if we've exhausted all of this node's siblings (and so on).
    Continue,
}

/// The result of parsing a Clang AST node.
#[derive(Debug)]
pub(crate) enum ParseResult<T> {
    /// We've already resolved this item before, here is the extant `ItemId` for
    /// it.
    AlreadyResolved(ItemId),

    /// This is a newly parsed item. If the cursor is `Some`, it points to the
    /// AST node where the new `T` was declared.
    New(T, Option<clang::Cursor>),
}

/// An intermediate representation "sub-item" (i.e. one of the types contained
/// inside an `ItemKind` variant) that can be parsed from a Clang cursor.
pub(crate) trait ClangSubItemParser: Sized {
    /// Attempt to parse this type from the given cursor.
    ///
    /// The fact that is a reference guarantees it's held by the context, and
    /// allow returning already existing types.
    fn parse(
        cursor: clang::Cursor,
        context: &mut BindgenContext,
    ) -> Result<ParseResult<Self>, ParseError>;
}

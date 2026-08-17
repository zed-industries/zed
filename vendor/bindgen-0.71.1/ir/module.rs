//! Intermediate representation for modules (AKA C++ namespaces).

use super::context::BindgenContext;
use super::dot::DotAttributes;
use super::item::ItemSet;
use crate::clang;
use crate::parse::{ClangSubItemParser, ParseError, ParseResult};
use crate::parse_one;

use std::io;

/// Whether this module is inline or not.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ModuleKind {
    /// This module is not inline.
    Normal,
    /// This module is inline, as in `inline namespace foo {}`.
    Inline,
}

/// A module, as in, a C++ namespace.
#[derive(Clone, Debug)]
pub(crate) struct Module {
    /// The name of the module, or none if it's anonymous.
    name: Option<String>,
    /// The kind of module this is.
    kind: ModuleKind,
    /// The children of this module, just here for convenience.
    children: ItemSet,
}

impl Module {
    /// Construct a new `Module`.
    pub(crate) fn new(name: Option<String>, kind: ModuleKind) -> Self {
        Module {
            name,
            kind,
            children: ItemSet::new(),
        }
    }

    /// Get this module's name.
    pub(crate) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Get a mutable reference to this module's children.
    pub(crate) fn children_mut(&mut self) -> &mut ItemSet {
        &mut self.children
    }

    /// Get this module's children.
    pub(crate) fn children(&self) -> &ItemSet {
        &self.children
    }

    /// Whether this namespace is inline.
    pub(crate) fn is_inline(&self) -> bool {
        self.kind == ModuleKind::Inline
    }
}

impl DotAttributes for Module {
    fn dot_attributes<W>(
        &self,
        _ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(out, "<tr><td>ModuleKind</td><td>{:?}</td></tr>", self.kind)
    }
}

impl ClangSubItemParser for Module {
    fn parse(
        cursor: clang::Cursor,
        ctx: &mut BindgenContext,
    ) -> Result<ParseResult<Self>, ParseError> {
        use clang_sys::*;
        match cursor.kind() {
            CXCursor_Namespace => {
                let module_id = ctx.module(cursor);
                ctx.with_module(module_id, |ctx| {
                    cursor.visit_sorted(ctx, |ctx, child| {
                        parse_one(ctx, child, Some(module_id.into()));
                    });
                });

                Ok(ParseResult::AlreadyResolved(module_id.into()))
            }
            _ => Err(ParseError::Continue),
        }
    }
}

//! Exception tables: catch handlers on `try_call` instructions.
//!
//! An exception table describes where execution flows after returning
//! from `try_call`. It contains both the "normal" destination -- the
//! block to branch to when the function returns without throwing an
//! exception -- and any "catch" destinations associated with
//! particular exception tags. Each target indicates the arguments to
//! pass to the block that receives control.
//!
//! Like other side-tables (e.g., jump tables), each exception table
//! must be used by only one instruction. Sharing is not permitted
//! because it can complicate transforms (how does one change the
//! table used by only one instruction if others also use it?).
//!
//! In order to allow the `try_call` instruction itself to remain
//! small, the exception table also contains the signature ID of the
//! called function.

use crate::ir::entities::{ExceptionTag, SigRef};
use crate::ir::instructions::ValueListPool;
use crate::ir::{BlockCall, Value};
use alloc::vec::Vec;
use core::fmt::{self, Display, Formatter};
#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// Contents of an exception table.
///
/// An exception table consists of a "no exception" ("normal")
/// destination block-call, and a series of exceptional destination
/// block-calls associated with tags.
///
/// The exceptional tags can also be interspersed with "dynamic
/// context" entries, which result in a particular value being stored
/// in the stack frame and accessible at an offset given in the
/// compiled exception-table metadata. This is needed for some kinds
/// of tag-matching where different dynamic instances of tags may
/// exist (e.g., in the WebAssembly exception-handling proposal).
///
/// The sequence of targets is semantically a list of
/// context-or-tagged-blockcall; e.g., `[context v0, tag1: block1(v1,
/// v2), context v2, tag2: block2(), tag3: block3()]`.
///
/// The "no exception" target can be accessed through the
/// `normal_return` and `normal_return_mut` functions. Exceptional
/// catch clauses may be iterated using the `catches` and
/// `catches_mut` functions.  All targets may be iterated over using
/// the `all_targets` and `all_targets_mut` functions.
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ExceptionTableData {
    /// All BlockCalls packed together. This is necessary because the
    /// rest of the compiler expects to be able to grab a slice of
    /// branch targets for any branch instruction. The last BlockCall
    /// is the normal-return destination, and the rest are referred to
    /// by index by the `items` below.
    targets: Vec<BlockCall>,

    /// Exception-table items.
    ///
    /// This internal representation for items is like
    /// `ExceptionTableItem` except that it has indices that refer to
    /// `targets` above.
    ///
    /// A tag value of `None` indicates a catch-all handler. The
    /// catch-all handler matches only if no other handler matches,
    /// regardless of the order in this vector.
    ///
    /// `tags[i]` corresponds to `targets[i]`. Note that there will be
    /// one more `targets` element than `tags` because the last
    /// element in `targets` is the normal-return path.
    items: Vec<InternalExceptionTableItem>,

    /// The signature of the function whose invocation is associated
    /// with this handler table.
    sig: SigRef,
}

/// A single item in the match-list of an exception table.
#[derive(Clone, Debug)]
pub enum ExceptionTableItem {
    /// A tag match, taking the specified block-call destination if
    /// the tag matches the one in the thrown exception. (The match
    /// predicate is up to the runtime; Cranelift only emits metadata
    /// containing this tag.)
    Tag(ExceptionTag, BlockCall),
    /// A default match, always taking the specified block-call
    /// destination.
    Default(BlockCall),
    /// A dynamic context update, applying to all tags until the next
    /// update. (Cranelift does not interpret this context, but only
    /// provides information to the runtime regarding where to find
    /// it.)
    Context(Value),
}

/// Our internal representation of exception-table items.
///
/// This is a version of `ExceptionTableItem` with block-calls
/// out-of-lined so that we can provide the slice externally. Each
/// block-call is referenced via an index.
#[derive(Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
enum InternalExceptionTableItem {
    Tag(ExceptionTag, u32),
    Default(u32),
    Context(Value),
}

impl ExceptionTableData {
    /// Create new exception-table data.
    ///
    /// This data represents the destinations upon return from
    /// `try_call` or `try_call_indirect` instruction. There are two
    /// possibilities: "normal return" (no exception thrown), or an
    /// exceptional return corresponding to one of the listed
    /// exception tags.
    ///
    /// The given tags are passed through to the metadata provided
    /// alongside the provided function body, and Cranelift itself
    /// does not implement an unwinder; thus, the meaning of the tags
    /// is ultimately up to the embedder of Cranelift. The tags are
    /// wrapped in `Option` to allow encoding a "catch-all" handler.
    ///
    /// The BlockCalls must have signatures that match the targeted
    /// blocks, as usual. These calls are allowed to use
    /// `BlockArg::TryCallRet` in the normal-return case, with types
    /// corresponding to the signature's return values, and
    /// `BlockArg::TryCallExn` in the exceptional-return cases, with
    /// types corresponding to native machine words and an arity
    /// corresponding to the number of payload values that the calling
    /// convention and platform support. (See [`isa::CallConv`] for
    /// more details.)
    pub fn new(
        sig: SigRef,
        normal_return: BlockCall,
        matches: impl IntoIterator<Item = ExceptionTableItem>,
    ) -> Self {
        let mut targets = vec![];
        let mut items = vec![];
        for item in matches {
            let target_idx = u32::try_from(targets.len()).unwrap();
            match item {
                ExceptionTableItem::Tag(tag, target) => {
                    items.push(InternalExceptionTableItem::Tag(tag, target_idx));
                    targets.push(target);
                }
                ExceptionTableItem::Default(target) => {
                    items.push(InternalExceptionTableItem::Default(target_idx));
                    targets.push(target);
                }
                ExceptionTableItem::Context(ctx) => {
                    items.push(InternalExceptionTableItem::Context(ctx));
                }
            }
        }
        targets.push(normal_return);

        ExceptionTableData {
            targets,
            items,
            sig,
        }
    }

    /// Return a value that can display the contents of this exception
    /// table.
    pub fn display<'a>(&'a self, pool: &'a ValueListPool) -> DisplayExceptionTable<'a> {
        DisplayExceptionTable { table: self, pool }
    }

    /// Deep-clone this exception table.
    pub fn deep_clone(&self, pool: &mut ValueListPool) -> Self {
        Self {
            targets: self.targets.iter().map(|b| b.deep_clone(pool)).collect(),
            items: self.items.clone(),
            sig: self.sig,
        }
    }

    /// Get the default target for the non-exceptional return case.
    pub fn normal_return(&self) -> &BlockCall {
        self.targets.last().unwrap()
    }

    /// Get the default target for the non-exceptional return case.
    pub fn normal_return_mut(&mut self) -> &mut BlockCall {
        self.targets.last_mut().unwrap()
    }

    /// Get the exception-catch items: dynamic context updates for
    /// interpreting tags, tag-associated targets, and catch-all
    /// targets.
    pub fn items(&self) -> impl Iterator<Item = ExceptionTableItem> + '_ {
        self.items.iter().map(|item| match item {
            InternalExceptionTableItem::Tag(tag, target_idx) => {
                ExceptionTableItem::Tag(*tag, self.targets[usize::try_from(*target_idx).unwrap()])
            }
            InternalExceptionTableItem::Default(target_idx) => {
                ExceptionTableItem::Default(self.targets[usize::try_from(*target_idx).unwrap()])
            }
            InternalExceptionTableItem::Context(ctx) => ExceptionTableItem::Context(*ctx),
        })
    }

    /// Get all branch targets.
    pub fn all_branches(&self) -> &[BlockCall] {
        &self.targets[..]
    }

    /// Get all branch targets.
    pub fn all_branches_mut(&mut self) -> &mut [BlockCall] {
        &mut self.targets[..]
    }

    /// Get the signature of the function called with this exception
    /// table.
    pub fn signature(&self) -> SigRef {
        self.sig
    }

    /// Get a mutable handle to this exception table's signature.
    pub(crate) fn signature_mut(&mut self) -> &mut SigRef {
        &mut self.sig
    }

    /// Get an iterator over context values.
    pub(crate) fn contexts(&self) -> impl DoubleEndedIterator<Item = Value> {
        self.items.iter().filter_map(|item| match item {
            InternalExceptionTableItem::Context(ctx) => Some(*ctx),
            _ => None,
        })
    }

    /// Get a mutable iterator over context values.
    pub(crate) fn contexts_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Value> {
        self.items.iter_mut().filter_map(|item| match item {
            InternalExceptionTableItem::Context(ctx) => Some(ctx),
            _ => None,
        })
    }

    /// Clears all entries in this exception table, but leaves the function signature.
    pub fn clear(&mut self) {
        self.items.clear();
        self.targets.clear();
    }
}

/// A wrapper for the context required to display a
/// [ExceptionTableData].
pub struct DisplayExceptionTable<'a> {
    table: &'a ExceptionTableData,
    pool: &'a ValueListPool,
}

impl<'a> Display for DisplayExceptionTable<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}, {}, [",
            self.table.sig,
            self.table.normal_return().display(self.pool)
        )?;
        let mut first = true;
        for item in self.table.items() {
            if first {
                write!(fmt, " ")?;
                first = false;
            } else {
                write!(fmt, ", ")?;
            }
            match item {
                ExceptionTableItem::Tag(tag, block_call) => {
                    write!(fmt, "{}: {}", tag, block_call.display(self.pool))?;
                }
                ExceptionTableItem::Default(block_call) => {
                    write!(fmt, "default: {}", block_call.display(self.pool))?;
                }
                ExceptionTableItem::Context(ctx) => {
                    write!(fmt, "context {ctx}")?;
                }
            }
        }
        let space = if first { "" } else { " " };
        write!(fmt, "{space}]")?;
        Ok(())
    }
}

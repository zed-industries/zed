//! Jump table representation.
//!
//! Jump tables are declared in the preamble and assigned an `ir::entities::JumpTable` reference.
//! The actual table of destinations is stored in a `JumpTableData` struct defined in this module.

use crate::ir::BlockCall;
use crate::ir::instructions::ValueListPool;
use alloc::vec::Vec;
use core::fmt::{self, Display, Formatter};
use core::slice::{Iter, IterMut};

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// Contents of a jump table.
///
/// All jump tables use 0-based indexing and are densely populated.
///
/// The default block for the jump table is stored as the first element of the underlying vector.
/// It can be accessed through the `default_block` and `default_block_mut` functions. All blocks
/// may be iterated using the `all_branches` and `all_branches_mut` functions, which will both
/// iterate over the default block first.
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct JumpTableData {
    // Table entries.
    table: Vec<BlockCall>,
}

impl JumpTableData {
    /// Create a new jump table with the provided blocks.
    pub fn new(def: BlockCall, table: &[BlockCall]) -> Self {
        Self {
            table: std::iter::once(def).chain(table.iter().copied()).collect(),
        }
    }

    /// Fetch the default block for this jump table.
    pub fn default_block(&self) -> BlockCall {
        *self.table.first().unwrap()
    }

    /// Mutable access to the default block of this jump table.
    pub fn default_block_mut(&mut self) -> &mut BlockCall {
        self.table.first_mut().unwrap()
    }

    /// The jump table and default block as a single slice. The default block will always be first.
    pub fn all_branches(&self) -> &[BlockCall] {
        self.table.as_slice()
    }

    /// The jump table and default block as a single mutable slice. The default block will always
    /// be first.
    pub fn all_branches_mut(&mut self) -> &mut [BlockCall] {
        self.table.as_mut_slice()
    }

    /// Access the jump table as a slice. This excludes the default block.
    pub fn as_slice(&self) -> &[BlockCall] {
        &self.table.as_slice()[1..]
    }

    /// Access the jump table as a mutable slice. This excludes the default block.
    pub fn as_mut_slice(&mut self) -> &mut [BlockCall] {
        &mut self.table.as_mut_slice()[1..]
    }

    /// Returns an iterator to the jump table, excluding the default block.
    #[deprecated(since = "7.0.0", note = "please use `.as_slice()` instead")]
    pub fn iter(&self) -> Iter<'_, BlockCall> {
        self.as_slice().iter()
    }

    /// Returns an iterator that allows modifying each value, excluding the default block.
    #[deprecated(since = "7.0.0", note = "please use `.as_mut_slice()` instead")]
    pub fn iter_mut(&mut self) -> IterMut<'_, BlockCall> {
        self.as_mut_slice().iter_mut()
    }

    /// Clears all entries in this jump table, except for the default block.
    pub fn clear(&mut self) {
        self.table.drain(1..);
    }

    /// Return a value that can display the contents of this jump table.
    pub fn display<'a>(&'a self, pool: &'a ValueListPool) -> DisplayJumpTable<'a> {
        DisplayJumpTable { jt: self, pool }
    }
}

/// A wrapper for the context required to display a [JumpTableData].
pub struct DisplayJumpTable<'a> {
    jt: &'a JumpTableData,
    pool: &'a ValueListPool,
}

impl<'a> Display for DisplayJumpTable<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}, [", self.jt.default_block().display(self.pool))?;
        if let Some((first, rest)) = self.jt.as_slice().split_first() {
            write!(fmt, "{}", first.display(self.pool))?;
            for block in rest {
                write!(fmt, ", {}", block.display(self.pool))?;
            }
        }
        write!(fmt, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::JumpTableData;
    use crate::entity::EntityRef;
    use crate::ir::instructions::ValueListPool;
    use crate::ir::{Block, BlockArg, BlockCall, Value};
    use alloc::vec::Vec;
    use std::string::ToString;

    #[test]
    fn empty() {
        let mut pool = ValueListPool::default();
        let def = BlockCall::new(Block::new(0), core::iter::empty(), &mut pool);

        let jt = JumpTableData::new(def, &[]);

        assert_eq!(jt.all_branches().get(0), Some(&def));

        assert_eq!(jt.as_slice().get(0), None);
        assert_eq!(jt.as_slice().get(10), None);

        assert_eq!(jt.display(&pool).to_string(), "block0, []");

        assert_eq!(jt.all_branches(), [def]);
        assert_eq!(jt.as_slice(), []);
    }

    #[test]
    fn insert() {
        let mut pool = ValueListPool::default();

        let v0 = Value::new(0);
        let v1 = Value::new(1);

        let e0 = Block::new(0);
        let e1 = Block::new(1);
        let e2 = Block::new(2);

        let def = BlockCall::new(e0, core::iter::empty(), &mut pool);
        let b1 = BlockCall::new(e1, core::iter::once(v0.into()), &mut pool);
        let b2 = BlockCall::new(e2, core::iter::empty(), &mut pool);
        let b3 = BlockCall::new(e1, core::iter::once(v1.into()), &mut pool);

        let jt = JumpTableData::new(def, &[b1, b2, b3]);

        assert_eq!(jt.default_block(), def);
        assert_eq!(
            jt.display(&pool).to_string(),
            "block0, [block1(v0), block2, block1(v1)]"
        );

        assert_eq!(jt.all_branches(), [def, b1, b2, b3]);
        assert_eq!(jt.as_slice(), [b1, b2, b3]);

        assert_eq!(
            jt.as_slice()[0].args(&pool).collect::<Vec<_>>(),
            [BlockArg::Value(v0)]
        );
        assert_eq!(jt.as_slice()[1].args(&pool).collect::<Vec<_>>(), []);
        assert_eq!(
            jt.as_slice()[2].args(&pool).collect::<Vec<_>>(),
            [BlockArg::Value(v1)]
        );
    }
}

//! Traversals over the IR.

use crate::ir;
use alloc::vec::Vec;
use core::fmt::Debug;
use core::hash::Hash;
use cranelift_entity::EntitySet;

/// A low-level DFS traversal event: either entering or exiting the traversal of
/// a block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Event {
    /// Entering traversal of a block.
    ///
    /// Processing a block upon this event corresponds to a pre-order,
    /// depth-first traversal.
    Enter,

    /// Exiting traversal of a block.
    ///
    /// Processing a block upon this event corresponds to a post-order,
    /// depth-first traversal.
    Exit,
}

/// A depth-first traversal.
///
/// This is a fairly low-level traversal type, and is generally intended to be
/// used as a building block for making specific pre-order or post-order
/// traversals for whatever problem is at hand.
///
/// This type may be reused multiple times across different passes or functions
/// and will internally reuse any heap allocations its already made.
///
/// Traversal is not recursive.
#[derive(Debug, Default, Clone)]
pub struct Dfs {
    stack: Vec<(Event, ir::Block)>,
    seen: EntitySet<ir::Block>,
}

impl Dfs {
    /// Construct a new depth-first traversal.
    pub fn new() -> Self {
        Self::default()
    }

    /// Perform a depth-first traversal over the given function.
    ///
    /// Yields pairs of `(Event, ir::Block)`.
    ///
    /// This iterator can be used to perform either pre- or post-order
    /// traversals, or a combination of the two.
    pub fn iter<'a>(&'a mut self, func: &'a ir::Function) -> DfsIter<'a> {
        self.clear();
        if let Some(e) = func.layout.entry_block() {
            self.stack.push((Event::Enter, e));
        }
        DfsIter { dfs: self, func }
    }

    /// Perform a pre-order traversal over the given function.
    ///
    /// Yields `ir::Block` items.
    pub fn pre_order_iter<'a>(&'a mut self, func: &'a ir::Function) -> DfsPreOrderIter<'a> {
        DfsPreOrderIter(self.iter(func))
    }

    /// Perform a post-order traversal over the given function.
    ///
    /// Yields `ir::Block` items.
    pub fn post_order_iter<'a>(&'a mut self, func: &'a ir::Function) -> DfsPostOrderIter<'a> {
        DfsPostOrderIter(self.iter(func))
    }

    /// Clear this DFS, but keep its allocations for future reuse.
    pub fn clear(&mut self) {
        let Dfs { stack, seen } = self;
        stack.clear();
        seen.clear();
    }
}

/// An iterator that yields pairs of `(Event, ir::Block)` items as it performs a
/// depth-first traversal over its associated function.
pub struct DfsIter<'a> {
    dfs: &'a mut Dfs,
    func: &'a ir::Function,
}

impl Iterator for DfsIter<'_> {
    type Item = (Event, ir::Block);

    fn next(&mut self) -> Option<(Event, ir::Block)> {
        loop {
            let (event, block) = self.dfs.stack.pop()?;

            if event == Event::Enter {
                let first_time_seeing = self.dfs.seen.insert(block);
                if !first_time_seeing {
                    continue;
                }

                self.dfs.stack.push((Event::Exit, block));
                self.dfs.stack.extend(
                    self.func
                        .block_successors(block)
                        // Heuristic: chase the children in reverse. This puts
                        // the first successor block first in the postorder, all
                        // other things being equal, which tends to prioritize
                        // loop backedges over out-edges, putting the edge-block
                        // closer to the loop body and minimizing live-ranges in
                        // linear instruction space. This heuristic doesn't have
                        // any effect on the computation of dominators, and is
                        // purely for other consumers of the postorder we cache
                        // here.
                        .rev()
                        // This is purely an optimization to avoid additional
                        // iterations of the loop, and is not required; it's
                        // merely inlining the check from the outer conditional
                        // of this case to avoid the extra loop iteration. This
                        // also avoids potential excess stack growth.
                        .filter(|block| !self.dfs.seen.contains(*block))
                        .map(|block| (Event::Enter, block)),
                );
            }

            return Some((event, block));
        }
    }
}

/// An iterator that yields `ir::Block` items during a depth-first, pre-order
/// traversal over its associated function.
pub struct DfsPreOrderIter<'a>(DfsIter<'a>);

impl Iterator for DfsPreOrderIter<'_> {
    type Item = ir::Block;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next()? {
                (Event::Enter, b) => return Some(b),
                (Event::Exit, _) => continue,
            }
        }
    }
}

/// An iterator that yields `ir::Block` items during a depth-first, post-order
/// traversal over its associated function.
pub struct DfsPostOrderIter<'a>(DfsIter<'a>);

impl Iterator for DfsPostOrderIter<'_> {
    type Item = ir::Block;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next()? {
                (Event::Exit, b) => return Some(b),
                (Event::Enter, _) => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cursor::{Cursor, FuncCursor};
    use crate::ir::{Function, InstBuilder, TrapCode, types::I32};

    #[test]
    fn test_dfs_traversal() {
        let _ = env_logger::try_init();

        let mut func = Function::new();

        let block0 = func.dfg.make_block();
        let v0 = func.dfg.append_block_param(block0, I32);
        let block1 = func.dfg.make_block();
        let block2 = func.dfg.make_block();
        let block3 = func.dfg.make_block();

        let mut cur = FuncCursor::new(&mut func);

        // block0(v0):
        //   br_if v0, block2, trap_block
        cur.insert_block(block0);
        cur.ins().brif(v0, block2, &[], block3, &[]);

        // block3:
        //   trap user0
        cur.insert_block(block3);
        cur.ins().trap(TrapCode::unwrap_user(1));

        // block1:
        //   v1 = iconst.i32 1
        //   v2 = iadd v0, v1
        //   jump block0(v2)
        cur.insert_block(block1);
        let v1 = cur.ins().iconst(I32, 1);
        let v2 = cur.ins().iadd(v0, v1);
        cur.ins().jump(block0, &[v2.into()]);

        // block2:
        //   return v0
        cur.insert_block(block2);
        cur.ins().return_(&[v0]);

        let mut dfs = Dfs::new();

        assert_eq!(
            dfs.iter(&func).collect::<Vec<_>>(),
            vec![
                (Event::Enter, block0),
                (Event::Enter, block2),
                (Event::Exit, block2),
                (Event::Enter, block3),
                (Event::Exit, block3),
                (Event::Exit, block0)
            ],
        );
    }

    #[test]
    fn multiple_successors_to_the_same_block() {
        let _ = env_logger::try_init();

        let mut func = Function::new();

        let block0 = func.dfg.make_block();
        let block1 = func.dfg.make_block();

        let mut cur = FuncCursor::new(&mut func);

        // block0(v0):
        //   v1 = iconst.i32 36
        //   v2 = iconst.i32 42
        //   br_if v0, block1(v1), block1(v2)
        cur.insert_block(block0);
        let v0 = cur.func.dfg.append_block_param(block0, I32);
        let v1 = cur.ins().iconst(ir::types::I32, 36);
        let v2 = cur.ins().iconst(ir::types::I32, 42);
        cur.ins()
            .brif(v0, block1, &[v1.into()], block1, &[v2.into()]);

        // block1(v3: i32):
        //   return v3
        cur.insert_block(block1);
        let v3 = cur.func.dfg.append_block_param(block1, I32);
        cur.ins().return_(&[v3]);

        let mut dfs = Dfs::new();

        // We should only enter `block1` once.
        assert_eq!(
            dfs.iter(&func).collect::<Vec<_>>(),
            vec![
                (Event::Enter, block0),
                (Event::Enter, block1),
                (Event::Exit, block1),
                (Event::Exit, block0),
            ],
        );

        // We should only iterate over `block1` once in a pre-order traversal.
        assert_eq!(
            dfs.pre_order_iter(&func).collect::<Vec<_>>(),
            vec![block0, block1],
        );

        // We should only iterate over `block1` once in a post-order traversal.
        assert_eq!(
            dfs.post_order_iter(&func).collect::<Vec<_>>(),
            vec![block1, block0],
        );
    }
}

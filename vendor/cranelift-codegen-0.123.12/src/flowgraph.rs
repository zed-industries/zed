//! A control flow graph represented as mappings of basic blocks to their predecessors
//! and successors.
//!
//! Successors are represented as basic blocks while predecessors are represented by basic
//! blocks. Basic blocks are denoted by tuples of block and branch/jump instructions. Each
//! predecessor tuple corresponds to the end of a basic block.
//!
//! ```c
//!     Block0:
//!         ...          ; beginning of basic block
//!
//!         ...
//!
//!         brif vx, Block1, Block2 ; end of basic block
//!
//!     Block1:
//!         jump block3
//! ```
//!
//! Here `Block1` and `Block2` would each have a single predecessor denoted as `(Block0, brif)`,
//! while `Block3` would have a single predecessor denoted as `(Block1, jump block3)`.

use crate::bforest;
use crate::entity::SecondaryMap;
use crate::inst_predicates;
use crate::ir::{Block, Function, Inst};
use crate::timing;
use core::mem;

/// A basic block denoted by its enclosing Block and last instruction.
#[derive(Debug, PartialEq, Eq)]
pub struct BlockPredecessor {
    /// Enclosing Block key.
    pub block: Block,
    /// Last instruction in the basic block.
    pub inst: Inst,
}

impl BlockPredecessor {
    /// Convenient method to construct new BlockPredecessor.
    pub fn new(block: Block, inst: Inst) -> Self {
        Self { block, inst }
    }
}

/// A container for the successors and predecessors of some Block.
#[derive(Clone, Default)]
struct CFGNode {
    /// Instructions that can branch or jump to this block.
    ///
    /// This maps branch instruction -> predecessor block which is redundant since the block containing
    /// the branch instruction is available from the `layout.inst_block()` method. We store the
    /// redundant information because:
    ///
    /// 1. Many `pred_iter()` consumers want the block anyway, so it is handily available.
    /// 2. The `invalidate_block_successors()` may be called *after* branches have been removed from
    ///    their block, but we still need to remove them form the old block predecessor map.
    ///
    /// The redundant block stored here is always consistent with the CFG successor lists, even after
    /// the IR has been edited.
    pub predecessors: bforest::Map<Inst, Block>,

    /// Set of blocks that are the targets of branches and jumps in this block.
    /// The set is ordered by block number, indicated by the `()` comparator type.
    pub successors: bforest::Set<Block>,
}

/// The Control Flow Graph maintains a mapping of blocks to their predecessors
/// and successors where predecessors are basic blocks and successors are
/// basic blocks.
pub struct ControlFlowGraph {
    data: SecondaryMap<Block, CFGNode>,
    pred_forest: bforest::MapForest<Inst, Block>,
    succ_forest: bforest::SetForest<Block>,
    valid: bool,
}

impl ControlFlowGraph {
    /// Allocate a new blank control flow graph.
    pub fn new() -> Self {
        Self {
            data: SecondaryMap::new(),
            valid: false,
            pred_forest: bforest::MapForest::new(),
            succ_forest: bforest::SetForest::new(),
        }
    }

    /// Clear all data structures in this control flow graph.
    pub fn clear(&mut self) {
        self.data.clear();
        self.pred_forest.clear();
        self.succ_forest.clear();
        self.valid = false;
    }

    /// Allocate and compute the control flow graph for `func`.
    pub fn with_function(func: &Function) -> Self {
        let mut cfg = Self::new();
        cfg.compute(func);
        cfg
    }

    /// Compute the control flow graph of `func`.
    ///
    /// This will clear and overwrite any information already stored in this data structure.
    pub fn compute(&mut self, func: &Function) {
        let _tt = timing::flowgraph();
        self.clear();
        self.data.resize(func.dfg.num_blocks());

        for block in &func.layout {
            self.compute_block(func, block);
        }

        self.valid = true;
    }

    fn compute_block(&mut self, func: &Function, block: Block) {
        inst_predicates::visit_block_succs(func, block, |inst, dest, _| {
            self.add_edge(block, inst, dest);
        });
    }

    fn invalidate_block_successors(&mut self, block: Block) {
        // Temporarily take ownership because we need mutable access to self.data inside the loop.
        // Unfortunately borrowck cannot see that our mut accesses to predecessors don't alias
        // our iteration over successors.
        let mut successors = mem::replace(&mut self.data[block].successors, Default::default());
        for succ in successors.iter(&self.succ_forest) {
            self.data[succ]
                .predecessors
                .retain(&mut self.pred_forest, |_, &mut e| e != block);
        }
        successors.clear(&mut self.succ_forest);
    }

    /// Recompute the control flow graph of `block`.
    ///
    /// This is for use after modifying instructions within a specific block. It recomputes all edges
    /// from `block` while leaving edges to `block` intact. Its functionality a subset of that of the
    /// more expensive `compute`, and should be used when we know we don't need to recompute the CFG
    /// from scratch, but rather that our changes have been restricted to specific blocks.
    pub fn recompute_block(&mut self, func: &Function, block: Block) {
        debug_assert!(self.is_valid());
        self.invalidate_block_successors(block);
        self.compute_block(func, block);
    }

    fn add_edge(&mut self, from: Block, from_inst: Inst, to: Block) {
        self.data[from]
            .successors
            .insert(to, &mut self.succ_forest, &());
        self.data[to]
            .predecessors
            .insert(from_inst, from, &mut self.pred_forest, &());
    }

    /// Get an iterator over the CFG predecessors to `block`.
    pub fn pred_iter(&self, block: Block) -> PredIter<'_> {
        PredIter(self.data[block].predecessors.iter(&self.pred_forest))
    }

    /// Get an iterator over the CFG successors to `block`.
    pub fn succ_iter(&self, block: Block) -> SuccIter<'_> {
        debug_assert!(self.is_valid());
        self.data[block].successors.iter(&self.succ_forest)
    }

    /// Check if the CFG is in a valid state.
    ///
    /// Note that this doesn't perform any kind of validity checks. It simply checks if the
    /// `compute()` method has been called since the last `clear()`. It does not check that the
    /// CFG is consistent with the function.
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

/// An iterator over block predecessors. The iterator type is `BlockPredecessor`.
///
/// Each predecessor is an instruction that branches to the block.
pub struct PredIter<'a>(bforest::MapIter<'a, Inst, Block>);

impl<'a> Iterator for PredIter<'a> {
    type Item = BlockPredecessor;

    fn next(&mut self) -> Option<BlockPredecessor> {
        self.0.next().map(|(i, e)| BlockPredecessor::new(e, i))
    }
}

/// An iterator over block successors. The iterator type is `Block`.
pub type SuccIter<'a> = bforest::SetIter<'a, Block>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cursor::{Cursor, FuncCursor};
    use crate::ir::{InstBuilder, types};
    use alloc::vec::Vec;

    #[test]
    fn empty() {
        let func = Function::new();
        ControlFlowGraph::with_function(&func);
    }

    #[test]
    fn no_predecessors() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        let block1 = func.dfg.make_block();
        let block2 = func.dfg.make_block();
        func.layout.append_block(block0);
        func.layout.append_block(block1);
        func.layout.append_block(block2);

        let cfg = ControlFlowGraph::with_function(&func);

        let mut fun_blocks = func.layout.blocks();
        for block in func.layout.blocks() {
            assert_eq!(block, fun_blocks.next().unwrap());
            assert_eq!(cfg.pred_iter(block).count(), 0);
            assert_eq!(cfg.succ_iter(block).count(), 0);
        }
    }

    #[test]
    fn branches_and_jumps() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        let cond = func.dfg.append_block_param(block0, types::I32);
        let block1 = func.dfg.make_block();
        let block2 = func.dfg.make_block();

        let br_block0_block2_block1;
        let br_block1_block1_block2;

        {
            let mut cur = FuncCursor::new(&mut func);

            cur.insert_block(block0);
            br_block0_block2_block1 = cur.ins().brif(cond, block2, &[], block1, &[]);

            cur.insert_block(block1);
            br_block1_block1_block2 = cur.ins().brif(cond, block1, &[], block2, &[]);

            cur.insert_block(block2);
        }

        let mut cfg = ControlFlowGraph::with_function(&func);

        {
            let block0_predecessors = cfg.pred_iter(block0).collect::<Vec<_>>();
            let block1_predecessors = cfg.pred_iter(block1).collect::<Vec<_>>();
            let block2_predecessors = cfg.pred_iter(block2).collect::<Vec<_>>();

            let block0_successors = cfg.succ_iter(block0).collect::<Vec<_>>();
            let block1_successors = cfg.succ_iter(block1).collect::<Vec<_>>();
            let block2_successors = cfg.succ_iter(block2).collect::<Vec<_>>();

            assert_eq!(block0_predecessors.len(), 0);
            assert_eq!(block1_predecessors.len(), 2);
            assert_eq!(block2_predecessors.len(), 2);

            assert_eq!(
                block1_predecessors
                    .contains(&BlockPredecessor::new(block0, br_block0_block2_block1)),
                true
            );
            assert_eq!(
                block1_predecessors
                    .contains(&BlockPredecessor::new(block1, br_block1_block1_block2)),
                true
            );
            assert_eq!(
                block2_predecessors
                    .contains(&BlockPredecessor::new(block0, br_block0_block2_block1)),
                true
            );
            assert_eq!(
                block2_predecessors
                    .contains(&BlockPredecessor::new(block1, br_block1_block1_block2)),
                true
            );

            assert_eq!(block0_successors, [block1, block2]);
            assert_eq!(block1_successors, [block1, block2]);
            assert_eq!(block2_successors, []);
        }

        // Add a new block to hold a return instruction
        let ret_block = func.dfg.make_block();

        {
            let mut cur = FuncCursor::new(&mut func);
            cur.insert_block(ret_block);
            cur.ins().return_(&[]);
        }

        // Change some instructions and recompute block0 and ret_block
        func.dfg
            .replace(br_block0_block2_block1)
            .brif(cond, block1, &[], ret_block, &[]);
        cfg.recompute_block(&func, block0);
        cfg.recompute_block(&func, ret_block);
        let br_block0_block1_ret_block = br_block0_block2_block1;

        {
            let block0_predecessors = cfg.pred_iter(block0).collect::<Vec<_>>();
            let block1_predecessors = cfg.pred_iter(block1).collect::<Vec<_>>();
            let block2_predecessors = cfg.pred_iter(block2).collect::<Vec<_>>();

            let block0_successors = cfg.succ_iter(block0);
            let block1_successors = cfg.succ_iter(block1);
            let block2_successors = cfg.succ_iter(block2);

            assert_eq!(block0_predecessors.len(), 0);
            assert_eq!(block1_predecessors.len(), 2);
            assert_eq!(block2_predecessors.len(), 1);

            assert_eq!(
                block1_predecessors
                    .contains(&BlockPredecessor::new(block0, br_block0_block1_ret_block)),
                true
            );
            assert_eq!(
                block1_predecessors
                    .contains(&BlockPredecessor::new(block1, br_block1_block1_block2)),
                true
            );
            assert_eq!(
                block2_predecessors
                    .contains(&BlockPredecessor::new(block0, br_block0_block1_ret_block)),
                false
            );
            assert_eq!(
                block2_predecessors
                    .contains(&BlockPredecessor::new(block1, br_block1_block1_block2)),
                true
            );

            assert_eq!(block0_successors.collect::<Vec<_>>(), [block1, ret_block]);
            assert_eq!(block1_successors.collect::<Vec<_>>(), [block1, block2]);
            assert_eq!(block2_successors.collect::<Vec<_>>(), []);
        }
    }
}

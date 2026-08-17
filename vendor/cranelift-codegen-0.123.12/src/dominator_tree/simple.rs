//! A Dominator Tree represented as mappings of Blocks to their immediate dominator.
//! Computed using Keith D. Cooper's "Simple, Fast Dominator Algorithm."   
//! This version have been used in Cranelift for a very long time
//! and should be quite stable. Used as a baseline i.e. in verification.

use crate::entity::SecondaryMap;
use crate::flowgraph::{BlockPredecessor, ControlFlowGraph};
use crate::ir::{Block, Function, Layout, ProgramPoint};
use crate::packed_option::PackedOption;
use crate::timing;
use crate::traversals::Dfs;
use alloc::vec::Vec;
use core::cmp::Ordering;

/// RPO numbers are not first assigned in a contiguous way but as multiples of STRIDE, to leave
/// room for modifications of the dominator tree.
const STRIDE: u32 = 4;

/// Dominator tree node. We keep one of these per block.
#[derive(Clone, Default)]
struct DomNode {
    /// Number of this node in a reverse post-order traversal of the CFG, starting from 1.
    /// This number is monotonic in the reverse postorder but not contiguous, since we leave
    /// holes for later localized modifications of the dominator tree.
    /// Unreachable nodes get number 0, all others are positive.
    rpo_number: u32,

    /// The immediate dominator of this block.
    ///
    /// This is `None` for unreachable blocks and the entry block which doesn't have an immediate
    /// dominator.
    idom: PackedOption<Block>,
}

/// The dominator tree for a single function.
pub struct SimpleDominatorTree {
    nodes: SecondaryMap<Block, DomNode>,

    /// CFG post-order of all reachable blocks.
    postorder: Vec<Block>,

    /// Scratch traversal state used by `compute_postorder()`.
    dfs: Dfs,

    valid: bool,
}

/// Methods for querying the dominator tree.
impl SimpleDominatorTree {
    /// Is `block` reachable from the entry block?
    pub fn is_reachable(&self, block: Block) -> bool {
        self.nodes[block].rpo_number != 0
    }

    /// Get the CFG post-order of blocks that was used to compute the dominator tree.
    ///
    /// Note that this post-order is not updated automatically when the CFG is modified. It is
    /// computed from scratch and cached by `compute()`.
    pub fn cfg_postorder(&self) -> &[Block] {
        debug_assert!(self.is_valid());
        &self.postorder
    }

    /// Returns the immediate dominator of `block`.
    ///
    /// `block_a` is said to *dominate* `block_b` if all control flow paths from the function
    /// entry to `block_b` must go through `block_a`.
    ///
    /// The *immediate dominator* is the dominator that is closest to `block`. All other dominators
    /// also dominate the immediate dominator.
    ///
    /// This returns `None` if `block` is not reachable from the entry block, or if it is the entry block
    /// which has no dominators.
    pub fn idom(&self, block: Block) -> Option<Block> {
        self.nodes[block].idom.into()
    }

    /// Compare two blocks relative to the reverse post-order.
    pub fn rpo_cmp_block(&self, a: Block, b: Block) -> Ordering {
        self.nodes[a].rpo_number.cmp(&self.nodes[b].rpo_number)
    }

    /// Compare two program points relative to a reverse post-order traversal of the control-flow
    /// graph.
    ///
    /// Return `Ordering::Less` if `a` comes before `b` in the RPO.
    ///
    /// If `a` and `b` belong to the same block, compare their relative position in the block.
    pub fn rpo_cmp<A, B>(&self, a: A, b: B, layout: &Layout) -> Ordering
    where
        A: Into<ProgramPoint>,
        B: Into<ProgramPoint>,
    {
        let a = a.into();
        let b = b.into();
        self.rpo_cmp_block(layout.pp_block(a), layout.pp_block(b))
            .then_with(|| layout.pp_cmp(a, b))
    }

    /// Returns `true` if `a` dominates `b`.
    ///
    /// This means that every control-flow path from the function entry to `b` must go through `a`.
    ///
    /// Dominance is ill defined for unreachable blocks. This function can always determine
    /// dominance for instructions in the same block, but otherwise returns `false` if either block
    /// is unreachable.
    ///
    /// An instruction is considered to dominate itself.
    /// A block is also considered to dominate itself.
    pub fn dominates<A, B>(&self, a: A, b: B, layout: &Layout) -> bool
    where
        A: Into<ProgramPoint>,
        B: Into<ProgramPoint>,
    {
        let a = a.into();
        let b = b.into();
        match a {
            ProgramPoint::Block(block_a) => match b {
                ProgramPoint::Block(block_b) => self.block_dominates(block_a, block_b),
                ProgramPoint::Inst(inst_b) => {
                    let block_b = layout
                        .inst_block(inst_b)
                        .expect("Instruction not in layout.");
                    self.block_dominates(block_a, block_b)
                }
            },
            ProgramPoint::Inst(inst_a) => {
                let block_a: Block = layout
                    .inst_block(inst_a)
                    .expect("Instruction not in layout.");
                match b {
                    ProgramPoint::Block(block_b) => {
                        block_a != block_b && self.block_dominates(block_a, block_b)
                    }
                    ProgramPoint::Inst(inst_b) => {
                        let block_b = layout
                            .inst_block(inst_b)
                            .expect("Instruction not in layout.");
                        if block_a == block_b {
                            layout.pp_cmp(a, b) != Ordering::Greater
                        } else {
                            self.block_dominates(block_a, block_b)
                        }
                    }
                }
            }
        }
    }

    /// Returns `true` if `block_a` dominates `block_b`.
    ///
    /// A block is considered to dominate itself.
    fn block_dominates(&self, block_a: Block, mut block_b: Block) -> bool {
        let rpo_a = self.nodes[block_a].rpo_number;

        // Run a finger up the dominator tree from b until we see a.
        // Do nothing if b is unreachable.
        while rpo_a < self.nodes[block_b].rpo_number {
            let idom = match self.idom(block_b) {
                Some(idom) => idom,
                None => return false, // a is unreachable, so we climbed past the entry
            };
            block_b = idom;
        }

        block_a == block_b
    }

    /// Compute the common dominator of two basic blocks.
    ///
    /// Both basic blocks are assumed to be reachable.
    fn common_dominator(&self, mut a: Block, mut b: Block) -> Block {
        loop {
            match self.rpo_cmp_block(a, b) {
                Ordering::Less => {
                    // `a` comes before `b` in the RPO. Move `b` up.
                    let idom = self.nodes[b].idom.expect("Unreachable basic block?");
                    b = idom;
                }
                Ordering::Greater => {
                    // `b` comes before `a` in the RPO. Move `a` up.
                    let idom = self.nodes[a].idom.expect("Unreachable basic block?");
                    a = idom;
                }
                Ordering::Equal => break,
            }
        }

        debug_assert_eq!(a, b, "Unreachable block passed to common_dominator?");

        a
    }
}

impl SimpleDominatorTree {
    /// Allocate a new blank dominator tree. Use `compute` to compute the dominator tree for a
    /// function.
    pub fn new() -> Self {
        Self {
            nodes: SecondaryMap::new(),
            postorder: Vec::new(),
            dfs: Dfs::new(),
            valid: false,
        }
    }

    /// Allocate and compute a dominator tree.
    pub fn with_function(func: &Function, cfg: &ControlFlowGraph) -> Self {
        let block_capacity = func.layout.block_capacity();
        let mut domtree = Self {
            nodes: SecondaryMap::with_capacity(block_capacity),
            postorder: Vec::with_capacity(block_capacity),
            dfs: Dfs::new(),
            valid: false,
        };
        domtree.compute(func, cfg);
        domtree
    }

    /// Reset and compute a CFG post-order and dominator tree.
    pub fn compute(&mut self, func: &Function, cfg: &ControlFlowGraph) {
        let _tt = timing::domtree();
        debug_assert!(cfg.is_valid());
        self.compute_postorder(func);
        self.compute_domtree(func, cfg);
        self.valid = true;
    }

    /// Clear the data structures used to represent the dominator tree. This will leave the tree in
    /// a state where `is_valid()` returns false.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.postorder.clear();
        self.valid = false;
    }

    /// Check if the dominator tree is in a valid state.
    ///
    /// Note that this doesn't perform any kind of validity checks. It simply checks if the
    /// `compute()` method has been called since the last `clear()`. It does not check that the
    /// dominator tree is consistent with the CFG.
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Reset all internal data structures and compute a post-order of the control flow graph.
    ///
    /// This leaves `rpo_number == 1` for all reachable blocks, 0 for unreachable ones.
    fn compute_postorder(&mut self, func: &Function) {
        self.clear();
        self.nodes.resize(func.dfg.num_blocks());
        self.postorder.extend(self.dfs.post_order_iter(func));
    }

    /// Build a dominator tree from a control flow graph using Keith D. Cooper's
    /// "Simple, Fast Dominator Algorithm."
    fn compute_domtree(&mut self, func: &Function, cfg: &ControlFlowGraph) {
        // During this algorithm, `rpo_number` has the following values:
        //
        // 0: block is not reachable.
        // 1: block is reachable, but has not yet been visited during the first pass. This is set by
        // `compute_postorder`.
        // 2+: block is reachable and has an assigned RPO number.

        // We'll be iterating over a reverse post-order of the CFG, skipping the entry block.
        let (entry_block, postorder) = match self.postorder.as_slice().split_last() {
            Some((&eb, rest)) => (eb, rest),
            None => return,
        };
        debug_assert_eq!(Some(entry_block), func.layout.entry_block());

        // Do a first pass where we assign RPO numbers to all reachable nodes.
        self.nodes[entry_block].rpo_number = 2 * STRIDE;
        for (rpo_idx, &block) in postorder.iter().rev().enumerate() {
            // Update the current node and give it an RPO number.
            // The entry block got 2, the rest start at 3 by multiples of STRIDE to leave
            // room for future dominator tree modifications.
            //
            // Since `compute_idom` will only look at nodes with an assigned RPO number, the
            // function will never see an uninitialized predecessor.
            //
            // Due to the nature of the post-order traversal, every node we visit will have at
            // least one predecessor that has previously been visited during this RPO.
            self.nodes[block] = DomNode {
                idom: self.compute_idom(block, cfg).into(),
                rpo_number: (rpo_idx as u32 + 3) * STRIDE,
            }
        }

        // Now that we have RPO numbers for everything and initial immediate dominator estimates,
        // iterate until convergence.
        //
        // If the function is free of irreducible control flow, this will exit after one iteration.
        let mut changed = true;
        while changed {
            changed = false;
            for &block in postorder.iter().rev() {
                let idom = self.compute_idom(block, cfg).into();
                if self.nodes[block].idom != idom {
                    self.nodes[block].idom = idom;
                    changed = true;
                }
            }
        }
    }

    // Compute the immediate dominator for `block` using the current `idom` states for the reachable
    // nodes.
    fn compute_idom(&self, block: Block, cfg: &ControlFlowGraph) -> Block {
        // Get an iterator with just the reachable, already visited predecessors to `block`.
        // Note that during the first pass, `rpo_number` is 1 for reachable blocks that haven't
        // been visited yet, 0 for unreachable blocks.
        let mut reachable_preds = cfg
            .pred_iter(block)
            .filter(|&BlockPredecessor { block: pred, .. }| self.nodes[pred].rpo_number > 1)
            .map(|pred| pred.block);

        // The RPO must visit at least one predecessor before this node.
        let mut idom = reachable_preds
            .next()
            .expect("block node must have one reachable predecessor");

        for pred in reachable_preds {
            idom = self.common_dominator(idom, pred);
        }

        idom
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cursor::{Cursor, FuncCursor};
    use crate::ir::types::*;
    use crate::ir::{InstBuilder, TrapCode};

    #[test]
    fn empty() {
        let func = Function::new();
        let cfg = ControlFlowGraph::with_function(&func);
        debug_assert!(cfg.is_valid());
        let dtree = SimpleDominatorTree::with_function(&func, &cfg);
        assert_eq!(0, dtree.nodes.keys().count());
        assert_eq!(dtree.cfg_postorder(), &[]);
    }

    #[test]
    fn unreachable_node() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        let v0 = func.dfg.append_block_param(block0, I32);
        let block1 = func.dfg.make_block();
        let block2 = func.dfg.make_block();
        let trap_block = func.dfg.make_block();

        let mut cur = FuncCursor::new(&mut func);

        cur.insert_block(block0);
        cur.ins().brif(v0, block2, &[], trap_block, &[]);

        cur.insert_block(trap_block);
        cur.ins().trap(TrapCode::unwrap_user(1));

        cur.insert_block(block1);
        let v1 = cur.ins().iconst(I32, 1);
        let v2 = cur.ins().iadd(v0, v1);
        cur.ins().jump(block0, &[v2.into()]);

        cur.insert_block(block2);
        cur.ins().return_(&[v0]);

        let cfg = ControlFlowGraph::with_function(cur.func);
        let dt = SimpleDominatorTree::with_function(cur.func, &cfg);

        // Fall-through-first, prune-at-source DFT:
        //
        // block0 {
        //   brif block2 {
        //     trap
        //     block2 {
        //       return
        //     } block2
        // } block0
        assert_eq!(dt.cfg_postorder(), &[block2, trap_block, block0]);

        let v2_def = cur.func.dfg.value_def(v2).unwrap_inst();
        assert!(!dt.dominates(v2_def, block0, &cur.func.layout));
        assert!(!dt.dominates(block0, v2_def, &cur.func.layout));

        assert!(dt.dominates(block0, block0, &cur.func.layout));
        assert!(!dt.dominates(block0, block1, &cur.func.layout));
        assert!(dt.dominates(block0, block2, &cur.func.layout));
        assert!(!dt.dominates(block1, block0, &cur.func.layout));
        assert!(dt.dominates(block1, block1, &cur.func.layout));
        assert!(!dt.dominates(block1, block2, &cur.func.layout));
        assert!(!dt.dominates(block2, block0, &cur.func.layout));
        assert!(!dt.dominates(block2, block1, &cur.func.layout));
        assert!(dt.dominates(block2, block2, &cur.func.layout));
    }

    #[test]
    fn non_zero_entry_block() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        let block1 = func.dfg.make_block();
        let block2 = func.dfg.make_block();
        let block3 = func.dfg.make_block();
        let cond = func.dfg.append_block_param(block3, I32);

        let mut cur = FuncCursor::new(&mut func);

        cur.insert_block(block3);
        let jmp_block3_block1 = cur.ins().jump(block1, &[]);

        cur.insert_block(block1);
        let br_block1_block0_block2 = cur.ins().brif(cond, block0, &[], block2, &[]);

        cur.insert_block(block2);
        cur.ins().jump(block0, &[]);

        cur.insert_block(block0);

        let cfg = ControlFlowGraph::with_function(cur.func);
        let dt = SimpleDominatorTree::with_function(cur.func, &cfg);

        // Fall-through-first, prune-at-source DFT:
        //
        // block3 {
        //   block3:jump block1 {
        //     block1 {
        //       block1:brif block0 {
        //         block1:jump block2 {
        //           block2 {
        //             block2:jump block0 (seen)
        //           } block2
        //         } block1:jump block2
        //         block0 {
        //         } block0
        //       } block1:brif block0
        //     } block1
        //   } block3:jump block1
        // } block3

        assert_eq!(dt.cfg_postorder(), &[block0, block2, block1, block3]);

        assert_eq!(cur.func.layout.entry_block().unwrap(), block3);
        assert_eq!(dt.idom(block3), None);
        assert_eq!(dt.idom(block1).unwrap(), block3);
        assert_eq!(dt.idom(block2).unwrap(), block1);
        assert_eq!(dt.idom(block0).unwrap(), block1);

        assert!(dt.dominates(
            br_block1_block0_block2,
            br_block1_block0_block2,
            &cur.func.layout
        ));
        assert!(!dt.dominates(br_block1_block0_block2, jmp_block3_block1, &cur.func.layout));
        assert!(dt.dominates(jmp_block3_block1, br_block1_block0_block2, &cur.func.layout));

        assert_eq!(
            dt.rpo_cmp(block3, block3, &cur.func.layout),
            Ordering::Equal
        );
        assert_eq!(dt.rpo_cmp(block3, block1, &cur.func.layout), Ordering::Less);
        assert_eq!(
            dt.rpo_cmp(block3, jmp_block3_block1, &cur.func.layout),
            Ordering::Less
        );
        assert_eq!(
            dt.rpo_cmp(jmp_block3_block1, br_block1_block0_block2, &cur.func.layout),
            Ordering::Less
        );
    }

    #[test]
    fn backwards_layout() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        let block1 = func.dfg.make_block();
        let block2 = func.dfg.make_block();

        let mut cur = FuncCursor::new(&mut func);

        cur.insert_block(block0);
        let jmp02 = cur.ins().jump(block2, &[]);

        cur.insert_block(block1);
        let trap = cur.ins().trap(TrapCode::unwrap_user(5));

        cur.insert_block(block2);
        let jmp21 = cur.ins().jump(block1, &[]);

        let cfg = ControlFlowGraph::with_function(cur.func);
        let dt = SimpleDominatorTree::with_function(cur.func, &cfg);

        assert_eq!(cur.func.layout.entry_block(), Some(block0));
        assert_eq!(dt.idom(block0), None);
        assert_eq!(dt.idom(block1), Some(block2));
        assert_eq!(dt.idom(block2), Some(block0));

        assert!(dt.dominates(block0, block0, &cur.func.layout));
        assert!(dt.dominates(block0, jmp02, &cur.func.layout));
        assert!(dt.dominates(block0, block1, &cur.func.layout));
        assert!(dt.dominates(block0, trap, &cur.func.layout));
        assert!(dt.dominates(block0, block2, &cur.func.layout));
        assert!(dt.dominates(block0, jmp21, &cur.func.layout));

        assert!(!dt.dominates(jmp02, block0, &cur.func.layout));
        assert!(dt.dominates(jmp02, jmp02, &cur.func.layout));
        assert!(dt.dominates(jmp02, block1, &cur.func.layout));
        assert!(dt.dominates(jmp02, trap, &cur.func.layout));
        assert!(dt.dominates(jmp02, block2, &cur.func.layout));
        assert!(dt.dominates(jmp02, jmp21, &cur.func.layout));

        assert!(!dt.dominates(block1, block0, &cur.func.layout));
        assert!(!dt.dominates(block1, jmp02, &cur.func.layout));
        assert!(dt.dominates(block1, block1, &cur.func.layout));
        assert!(dt.dominates(block1, trap, &cur.func.layout));
        assert!(!dt.dominates(block1, block2, &cur.func.layout));
        assert!(!dt.dominates(block1, jmp21, &cur.func.layout));

        assert!(!dt.dominates(trap, block0, &cur.func.layout));
        assert!(!dt.dominates(trap, jmp02, &cur.func.layout));
        assert!(!dt.dominates(trap, block1, &cur.func.layout));
        assert!(dt.dominates(trap, trap, &cur.func.layout));
        assert!(!dt.dominates(trap, block2, &cur.func.layout));
        assert!(!dt.dominates(trap, jmp21, &cur.func.layout));

        assert!(!dt.dominates(block2, block0, &cur.func.layout));
        assert!(!dt.dominates(block2, jmp02, &cur.func.layout));
        assert!(dt.dominates(block2, block1, &cur.func.layout));
        assert!(dt.dominates(block2, trap, &cur.func.layout));
        assert!(dt.dominates(block2, block2, &cur.func.layout));
        assert!(dt.dominates(block2, jmp21, &cur.func.layout));

        assert!(!dt.dominates(jmp21, block0, &cur.func.layout));
        assert!(!dt.dominates(jmp21, jmp02, &cur.func.layout));
        assert!(dt.dominates(jmp21, block1, &cur.func.layout));
        assert!(dt.dominates(jmp21, trap, &cur.func.layout));
        assert!(!dt.dominates(jmp21, block2, &cur.func.layout));
        assert!(dt.dominates(jmp21, jmp21, &cur.func.layout));
    }

    #[test]
    fn insts_same_block() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();

        let mut cur = FuncCursor::new(&mut func);

        cur.insert_block(block0);
        let v1 = cur.ins().iconst(I32, 1);
        let v2 = cur.ins().iadd(v1, v1);
        let v3 = cur.ins().iadd(v2, v2);
        cur.ins().return_(&[]);

        let cfg = ControlFlowGraph::with_function(cur.func);
        let dt = SimpleDominatorTree::with_function(cur.func, &cfg);

        let v1_def = cur.func.dfg.value_def(v1).unwrap_inst();
        let v2_def = cur.func.dfg.value_def(v2).unwrap_inst();
        let v3_def = cur.func.dfg.value_def(v3).unwrap_inst();

        assert!(dt.dominates(v1_def, v2_def, &cur.func.layout));
        assert!(dt.dominates(v2_def, v3_def, &cur.func.layout));
        assert!(dt.dominates(v1_def, v3_def, &cur.func.layout));

        assert!(!dt.dominates(v2_def, v1_def, &cur.func.layout));
        assert!(!dt.dominates(v3_def, v2_def, &cur.func.layout));
        assert!(!dt.dominates(v3_def, v1_def, &cur.func.layout));

        assert!(dt.dominates(v2_def, v2_def, &cur.func.layout));
        assert!(dt.dominates(block0, block0, &cur.func.layout));

        assert!(dt.dominates(block0, v1_def, &cur.func.layout));
        assert!(dt.dominates(block0, v2_def, &cur.func.layout));
        assert!(dt.dominates(block0, v3_def, &cur.func.layout));

        assert!(!dt.dominates(v1_def, block0, &cur.func.layout));
        assert!(!dt.dominates(v2_def, block0, &cur.func.layout));
        assert!(!dt.dominates(v3_def, block0, &cur.func.layout));
    }
}

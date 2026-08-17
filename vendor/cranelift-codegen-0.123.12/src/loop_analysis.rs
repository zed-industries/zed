//! A loop analysis represented as mappings of loops to their header Block
//! and parent in the loop tree.

use crate::dominator_tree::DominatorTree;
use crate::entity::SecondaryMap;
use crate::entity::entity_impl;
use crate::entity::{Keys, PrimaryMap};
use crate::flowgraph::ControlFlowGraph;
use crate::ir::{Block, Function, Layout};
use crate::packed_option::PackedOption;
use crate::timing;
use alloc::vec::Vec;
use smallvec::SmallVec;

/// A opaque reference to a code loop.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Loop(u32);
entity_impl!(Loop, "loop");

/// Loop tree information for a single function.
///
/// Loops are referenced by the Loop object, and for each loop you can access its header block,
/// its eventual parent in the loop tree and all the block belonging to the loop.
pub struct LoopAnalysis {
    loops: PrimaryMap<Loop, LoopData>,
    block_loop_map: SecondaryMap<Block, PackedOption<Loop>>,
    valid: bool,
}

struct LoopData {
    header: Block,
    parent: PackedOption<Loop>,
    level: LoopLevel,
}

/// A level in a loop nest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoopLevel(u8);
impl LoopLevel {
    const INVALID: u8 = u8::MAX;

    /// Get the root level (no loop).
    pub fn root() -> Self {
        Self(0)
    }
    /// Get the loop level.
    pub fn level(self) -> usize {
        self.0 as usize
    }
    /// Invalid loop level.
    pub fn invalid() -> Self {
        Self(Self::INVALID)
    }
    /// One loop level deeper.
    pub fn inc(self) -> Self {
        if self.0 == (Self::INVALID - 1) {
            self
        } else {
            Self(self.0 + 1)
        }
    }
    /// A clamped loop level from a larger-width (usize) depth.
    pub fn clamped(level: usize) -> Self {
        Self(
            u8::try_from(std::cmp::min(level, (Self::INVALID as usize) - 1))
                .expect("Clamped value must always convert"),
        )
    }
}

impl std::default::Default for LoopLevel {
    fn default() -> Self {
        LoopLevel::invalid()
    }
}

impl LoopData {
    /// Creates a `LoopData` object with the loop header and its eventual parent in the loop tree.
    pub fn new(header: Block, parent: Option<Loop>) -> Self {
        Self {
            header,
            parent: parent.into(),
            level: LoopLevel::invalid(),
        }
    }
}

/// Methods for querying the loop analysis.
impl LoopAnalysis {
    /// Allocate a new blank loop analysis struct. Use `compute` to compute the loop analysis for
    /// a function.
    pub fn new() -> Self {
        Self {
            valid: false,
            loops: PrimaryMap::new(),
            block_loop_map: SecondaryMap::new(),
        }
    }

    /// Returns all the loops contained in a function.
    pub fn loops(&self) -> Keys<Loop> {
        self.loops.keys()
    }

    /// Returns the header block of a particular loop.
    ///
    /// The characteristic property of a loop header block is that it dominates some of its
    /// predecessors.
    pub fn loop_header(&self, lp: Loop) -> Block {
        self.loops[lp].header
    }

    /// Return the eventual parent of a loop in the loop tree.
    pub fn loop_parent(&self, lp: Loop) -> Option<Loop> {
        self.loops[lp].parent.expand()
    }

    /// Return the innermost loop for a given block.
    pub fn innermost_loop(&self, block: Block) -> Option<Loop> {
        self.block_loop_map[block].expand()
    }

    /// Determine if a Block is a loop header. If so, return the loop.
    pub fn is_loop_header(&self, block: Block) -> Option<Loop> {
        self.innermost_loop(block)
            .filter(|&lp| self.loop_header(lp) == block)
    }

    /// Determine if a Block belongs to a loop by running a finger along the loop tree.
    ///
    /// Returns `true` if `block` is in loop `lp`.
    pub fn is_in_loop(&self, block: Block, lp: Loop) -> bool {
        let block_loop = self.block_loop_map[block];
        match block_loop.expand() {
            None => false,
            Some(block_loop) => self.is_child_loop(block_loop, lp),
        }
    }

    /// Determines if a loop is contained in another loop.
    ///
    /// `is_child_loop(child,parent)` returns `true` if and only if `child` is a child loop of
    /// `parent` (or `child == parent`).
    pub fn is_child_loop(&self, child: Loop, parent: Loop) -> bool {
        let mut finger = Some(child);
        while let Some(finger_loop) = finger {
            if finger_loop == parent {
                return true;
            }
            finger = self.loop_parent(finger_loop);
        }
        false
    }

    /// Returns the loop-nest level of a given block.
    pub fn loop_level(&self, block: Block) -> LoopLevel {
        self.innermost_loop(block)
            .map_or(LoopLevel(0), |lp| self.loops[lp].level)
    }
}

impl LoopAnalysis {
    /// Detects the loops in a function. Needs the control flow graph and the dominator tree.
    pub fn compute(&mut self, func: &Function, cfg: &ControlFlowGraph, domtree: &DominatorTree) {
        let _tt = timing::loop_analysis();
        self.loops.clear();
        self.block_loop_map.clear();
        self.block_loop_map.resize(func.dfg.num_blocks());
        self.find_loop_headers(cfg, domtree, &func.layout);
        self.discover_loop_blocks(cfg, domtree, &func.layout);
        self.assign_loop_levels();
        self.valid = true;
    }

    /// Check if the loop analysis is in a valid state.
    ///
    /// Note that this doesn't perform any kind of validity checks. It simply checks if the
    /// `compute()` method has been called since the last `clear()`. It does not check that the
    /// loop analysis is consistent with the CFG.
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Clear all the data structures contained in the loop analysis. This will leave the
    /// analysis in a similar state to a context returned by `new()` except that allocated
    /// memory be retained.
    pub fn clear(&mut self) {
        self.loops.clear();
        self.block_loop_map.clear();
        self.valid = false;
    }

    // Determines if a block dominates any predecessor
    // and thus is a loop header.
    fn is_block_loop_header(
        block: Block,
        cfg: &ControlFlowGraph,
        domtree: &DominatorTree,
        layout: &Layout,
    ) -> bool {
        // A block is a loop header if it dominates any of its predecessors.
        cfg.pred_iter(block)
            .any(|pred| domtree.dominates(block, pred.inst, layout))
    }

    // Traverses the CFG in reverse postorder and create a loop object for every block having a
    // back edge.
    fn find_loop_headers(
        &mut self,
        cfg: &ControlFlowGraph,
        domtree: &DominatorTree,
        layout: &Layout,
    ) {
        for &block in domtree
            .cfg_rpo()
            .filter(|&&block| Self::is_block_loop_header(block, cfg, domtree, layout))
        {
            // This block is a loop header, so we create its associated loop
            let lp = self.loops.push(LoopData::new(block, None));
            self.block_loop_map[block] = lp.into();
        }
    }

    // Intended to be called after `find_loop_headers`. For each detected loop header,
    // discovers all the block belonging to the loop and its inner loops. After a call to this
    // function, the loop tree is fully constructed.
    fn discover_loop_blocks(
        &mut self,
        cfg: &ControlFlowGraph,
        domtree: &DominatorTree,
        layout: &Layout,
    ) {
        let mut stack: Vec<Block> = Vec::new();
        // We handle each loop header in reverse order, corresponding to a pseudo postorder
        // traversal of the graph.
        for lp in self.loops().rev() {
            // Push all predecessors of this header that it dominates onto the stack.
            stack.extend(
                cfg.pred_iter(self.loops[lp].header)
                    .filter(|pred| {
                        // We follow the back edges
                        domtree.dominates(self.loops[lp].header, pred.inst, layout)
                    })
                    .map(|pred| pred.block),
            );
            while let Some(node) = stack.pop() {
                let continue_dfs: Option<Block>;
                match self.block_loop_map[node].expand() {
                    None => {
                        // The node hasn't been visited yet, we tag it as part of the loop
                        self.block_loop_map[node] = PackedOption::from(lp);
                        continue_dfs = Some(node);
                    }
                    Some(node_loop) => {
                        // We copy the node_loop into a mutable reference passed along the while
                        let mut node_loop = node_loop;
                        // The node is part of a loop, which can be lp or an inner loop
                        let mut node_loop_parent_option = self.loops[node_loop].parent;
                        while let Some(node_loop_parent) = node_loop_parent_option.expand() {
                            if node_loop_parent == lp {
                                // We have encountered lp so we stop (already visited)
                                break;
                            } else {
                                //
                                node_loop = node_loop_parent;
                                // We lookup the parent loop
                                node_loop_parent_option = self.loops[node_loop].parent;
                            }
                        }
                        // Now node_loop_parent is either:
                        // - None and node_loop is an new inner loop of lp
                        // - Some(...) and the initial node_loop was a known inner loop of lp
                        match node_loop_parent_option.expand() {
                            Some(_) => continue_dfs = None,
                            None => {
                                if node_loop != lp {
                                    self.loops[node_loop].parent = lp.into();
                                    continue_dfs = Some(self.loops[node_loop].header)
                                } else {
                                    // If lp is a one-block loop then we make sure we stop
                                    continue_dfs = None
                                }
                            }
                        }
                    }
                }
                // Now we have handled the popped node and need to continue the DFS by adding the
                // predecessors of that node
                if let Some(continue_dfs) = continue_dfs {
                    stack.extend(cfg.pred_iter(continue_dfs).map(|pred| pred.block));
                }
            }
        }
    }

    fn assign_loop_levels(&mut self) {
        let mut stack: SmallVec<[Loop; 8]> = SmallVec::new();
        for lp in self.loops.keys() {
            if self.loops[lp].level == LoopLevel::invalid() {
                stack.push(lp);
                while let Some(&lp) = stack.last() {
                    if let Some(parent) = self.loops[lp].parent.into() {
                        if self.loops[parent].level != LoopLevel::invalid() {
                            self.loops[lp].level = self.loops[parent].level.inc();
                            stack.pop();
                        } else {
                            stack.push(parent);
                        }
                    } else {
                        self.loops[lp].level = LoopLevel::root().inc();
                        stack.pop();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cursor::{Cursor, FuncCursor};
    use crate::dominator_tree::DominatorTree;
    use crate::flowgraph::ControlFlowGraph;
    use crate::ir::{Function, InstBuilder, types};
    use crate::loop_analysis::{Loop, LoopAnalysis};
    use alloc::vec::Vec;

    #[test]
    fn nested_loops_detection() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        let block1 = func.dfg.make_block();
        let block2 = func.dfg.make_block();
        let block3 = func.dfg.make_block();
        let block4 = func.dfg.make_block();
        let cond = func.dfg.append_block_param(block0, types::I32);

        {
            let mut cur = FuncCursor::new(&mut func);

            cur.insert_block(block0);
            cur.ins().jump(block1, &[]);

            cur.insert_block(block1);
            cur.ins().jump(block2, &[]);

            cur.insert_block(block2);
            cur.ins().brif(cond, block1, &[], block3, &[]);

            cur.insert_block(block3);
            cur.ins().brif(cond, block0, &[], block4, &[]);

            cur.insert_block(block4);
            cur.ins().return_(&[]);
        }

        let mut loop_analysis = LoopAnalysis::new();
        let mut cfg = ControlFlowGraph::new();
        let mut domtree = DominatorTree::new();
        cfg.compute(&func);
        domtree.compute(&func, &cfg);
        loop_analysis.compute(&func, &cfg, &domtree);

        let loops = loop_analysis.loops().collect::<Vec<Loop>>();
        assert_eq!(loops.len(), 2);
        assert_eq!(loop_analysis.loop_header(loops[0]), block0);
        assert_eq!(loop_analysis.loop_header(loops[1]), block1);
        assert_eq!(loop_analysis.loop_parent(loops[1]), Some(loops[0]));
        assert_eq!(loop_analysis.loop_parent(loops[0]), None);
        assert_eq!(loop_analysis.is_in_loop(block0, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block0, loops[1]), false);
        assert_eq!(loop_analysis.is_in_loop(block1, loops[1]), true);
        assert_eq!(loop_analysis.is_in_loop(block1, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block2, loops[1]), true);
        assert_eq!(loop_analysis.is_in_loop(block2, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block3, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block0, loops[1]), false);
        assert_eq!(loop_analysis.loop_level(block0).level(), 1);
        assert_eq!(loop_analysis.loop_level(block1).level(), 2);
        assert_eq!(loop_analysis.loop_level(block2).level(), 2);
        assert_eq!(loop_analysis.loop_level(block3).level(), 1);
    }

    #[test]
    fn complex_loop_detection() {
        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        let block1 = func.dfg.make_block();
        let block2 = func.dfg.make_block();
        let block3 = func.dfg.make_block();
        let block4 = func.dfg.make_block();
        let block5 = func.dfg.make_block();
        let block6 = func.dfg.make_block();
        let cond = func.dfg.append_block_param(block0, types::I32);

        {
            let mut cur = FuncCursor::new(&mut func);

            cur.insert_block(block0);
            cur.ins().brif(cond, block1, &[], block3, &[]);

            cur.insert_block(block1);
            cur.ins().jump(block2, &[]);

            cur.insert_block(block2);
            cur.ins().brif(cond, block1, &[], block5, &[]);

            cur.insert_block(block3);
            cur.ins().jump(block4, &[]);

            cur.insert_block(block4);
            cur.ins().brif(cond, block3, &[], block5, &[]);

            cur.insert_block(block5);
            cur.ins().brif(cond, block0, &[], block6, &[]);

            cur.insert_block(block6);
            cur.ins().return_(&[]);
        }

        let mut loop_analysis = LoopAnalysis::new();
        let cfg = ControlFlowGraph::with_function(&func);
        let domtree = DominatorTree::with_function(&func, &cfg);
        loop_analysis.compute(&func, &cfg, &domtree);

        let loops = loop_analysis.loops().collect::<Vec<Loop>>();
        assert_eq!(loops.len(), 3);
        assert_eq!(loop_analysis.loop_header(loops[0]), block0);
        assert_eq!(loop_analysis.loop_header(loops[1]), block3);
        assert_eq!(loop_analysis.loop_header(loops[2]), block1);
        assert_eq!(loop_analysis.loop_parent(loops[1]), Some(loops[0]));
        assert_eq!(loop_analysis.loop_parent(loops[2]), Some(loops[0]));
        assert_eq!(loop_analysis.loop_parent(loops[0]), None);
        assert_eq!(loop_analysis.is_in_loop(block0, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block1, loops[2]), true);
        assert_eq!(loop_analysis.is_in_loop(block2, loops[2]), true);
        assert_eq!(loop_analysis.is_in_loop(block3, loops[1]), true);
        assert_eq!(loop_analysis.is_in_loop(block4, loops[1]), true);
        assert_eq!(loop_analysis.is_in_loop(block5, loops[0]), true);
        assert_eq!(loop_analysis.loop_level(block0).level(), 1);
        assert_eq!(loop_analysis.loop_level(block1).level(), 2);
        assert_eq!(loop_analysis.loop_level(block2).level(), 2);
        assert_eq!(loop_analysis.loop_level(block3).level(), 2);
        assert_eq!(loop_analysis.loop_level(block4).level(), 2);
        assert_eq!(loop_analysis.loop_level(block5).level(), 1);
    }
}

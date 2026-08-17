//! A Dominator Tree represented as mappings of Blocks to their immediate dominator.

use crate::entity::SecondaryMap;
use crate::flowgraph::{BlockPredecessor, ControlFlowGraph};
use crate::ir::{Block, Function, Inst, Layout, ProgramPoint};
use crate::packed_option::PackedOption;
use crate::timing;
use alloc::vec::Vec;
use core::cmp;
use core::cmp::Ordering;
use core::mem;

mod simple;

pub use simple::SimpleDominatorTree;

/// Spanning tree node, used during domtree computation.
#[derive(Clone, Default)]
struct SpanningTreeNode {
    /// This node's block in function CFG.
    block: PackedOption<Block>,
    /// Node's ancestor in the spanning tree.
    /// Gets invalidated during semi-dominator computation.
    ancestor: u32,
    /// The smallest semi value discovered on any semi-dominator path
    /// that went through the node up till the moment.
    /// Gets updated in the course of semi-dominator computation.
    label: u32,
    /// Semidominator value for the node.
    semi: u32,
    /// Immediate dominator value for the node.
    /// Initialized to node's ancestor in the spanning tree.
    idom: u32,
}

/// DFS preorder number for unvisited nodes and the virtual root in the spanning tree.
const NOT_VISITED: u32 = 0;

/// Spanning tree, in CFG preorder.
/// Node 0 is the virtual root and doesn't have a corresponding block.
/// It's not required because function's CFG in Cranelift always have
/// a singular root, but helps to avoid additional checks.
/// Numbering nodes from 0 also follows the convention in
/// `SimpleDominatorTree` and `DominatorTreePreorder`.
#[derive(Clone, Default)]
struct SpanningTree {
    nodes: Vec<SpanningTreeNode>,
}

impl SpanningTree {
    fn new() -> Self {
        // Include the virtual root.
        Self {
            nodes: vec![Default::default()],
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        // Include the virtual root.
        let mut nodes = Vec::with_capacity(capacity + 1);
        nodes.push(Default::default());
        Self { nodes }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn reserve(&mut self, capacity: usize) {
        // Virtual root should be already included.
        self.nodes.reserve(capacity);
    }

    fn clear(&mut self) {
        self.nodes.resize(1, Default::default());
    }

    /// Returns pre_number for the new node.
    fn push(&mut self, ancestor: u32, block: Block) -> u32 {
        // Virtual root should be already included.
        debug_assert!(!self.nodes.is_empty());

        let pre_number = self.nodes.len() as u32;

        self.nodes.push(SpanningTreeNode {
            block: block.into(),
            ancestor,
            label: pre_number,
            semi: pre_number,
            idom: ancestor,
        });

        pre_number
    }
}

impl std::ops::Index<u32> for SpanningTree {
    type Output = SpanningTreeNode;

    fn index(&self, idx: u32) -> &Self::Output {
        &self.nodes[idx as usize]
    }
}

impl std::ops::IndexMut<u32> for SpanningTree {
    fn index_mut(&mut self, idx: u32) -> &mut Self::Output {
        &mut self.nodes[idx as usize]
    }
}

/// Traversal event to compute both preorder spanning tree
/// and postorder block list. Can't use `Dfs` from traversals.rs
/// here because of the need for parent links.
enum TraversalEvent {
    Enter(u32, Block),
    Exit(Block),
}

/// Dominator tree node. We keep one of these per block.
#[derive(Clone, Default)]
struct DominatorTreeNode {
    /// Immediate dominator for the block, `None` for unreachable blocks.
    idom: PackedOption<Block>,
    /// Preorder traversal number, zero for unreachable blocks.
    pre_number: u32,
}

/// The dominator tree for a single function,
/// computed using Semi-NCA algorithm.
pub struct DominatorTree {
    /// DFS spanning tree.
    stree: SpanningTree,
    /// List of CFG blocks in postorder.
    postorder: Vec<Block>,
    /// Dominator tree nodes.
    nodes: SecondaryMap<Block, DominatorTreeNode>,

    /// Stack for building the spanning tree.
    dfs_worklist: Vec<TraversalEvent>,
    /// Stack used for processing semidominator paths
    /// in link-eval procedure.
    eval_worklist: Vec<u32>,

    valid: bool,
}

/// Methods for querying the dominator tree.
impl DominatorTree {
    /// Is `block` reachable from the entry block?
    pub fn is_reachable(&self, block: Block) -> bool {
        self.nodes[block].pre_number != NOT_VISITED
    }

    /// Get the CFG post-order of blocks that was used to compute the dominator tree.
    ///
    /// Note that this post-order is not updated automatically when the CFG is modified. It is
    /// computed from scratch and cached by `compute()`.
    pub fn cfg_postorder(&self) -> &[Block] {
        debug_assert!(self.is_valid());
        &self.postorder
    }

    /// Get an iterator over CFG reverse post-order of blocks used to compute the dominator tree.
    ///
    /// Note that the post-order is not updated automatically when the CFG is modified. It is
    /// computed from scratch and cached by `compute()`.
    pub fn cfg_rpo(&self) -> impl Iterator<Item = &Block> {
        debug_assert!(self.is_valid());
        self.postorder.iter().rev()
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
        let pre_a = self.nodes[block_a].pre_number;

        // Run a finger up the dominator tree from b until we see a.
        // Do nothing if b is unreachable.
        while pre_a < self.nodes[block_b].pre_number {
            let idom = match self.idom(block_b) {
                Some(idom) => idom,
                None => return false, // a is unreachable, so we climbed past the entry
            };
            block_b = idom;
        }

        block_a == block_b
    }
}

impl DominatorTree {
    /// Allocate a new blank dominator tree. Use `compute` to compute the dominator tree for a
    /// function.
    pub fn new() -> Self {
        Self {
            stree: SpanningTree::new(),
            nodes: SecondaryMap::new(),
            postorder: Vec::new(),
            dfs_worklist: Vec::new(),
            eval_worklist: Vec::new(),
            valid: false,
        }
    }

    /// Allocate and compute a dominator tree.
    pub fn with_function(func: &Function, cfg: &ControlFlowGraph) -> Self {
        let block_capacity = func.layout.block_capacity();
        let mut domtree = Self {
            stree: SpanningTree::with_capacity(block_capacity),
            nodes: SecondaryMap::with_capacity(block_capacity),
            postorder: Vec::with_capacity(block_capacity),
            dfs_worklist: Vec::new(),
            eval_worklist: Vec::new(),
            valid: false,
        };
        domtree.compute(func, cfg);
        domtree
    }

    /// Reset and compute a CFG post-order and dominator tree,
    /// using Semi-NCA algorithm, described in the paper:
    ///
    /// Linear-Time Algorithms for Dominators and Related Problems.
    /// Loukas Georgiadis, Princeton University, November 2005.
    ///
    /// The same algorithm is used by Julia, SpiderMonkey and LLVM,
    /// the implementation is heavily inspired by them.
    pub fn compute(&mut self, func: &Function, cfg: &ControlFlowGraph) {
        let _tt = timing::domtree();
        debug_assert!(cfg.is_valid());

        self.clear();
        self.compute_spanning_tree(func);
        self.compute_domtree(cfg);

        self.valid = true;
    }

    /// Clear the data structures used to represent the dominator tree. This will leave the tree in
    /// a state where `is_valid()` returns false.
    pub fn clear(&mut self) {
        self.stree.clear();
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

    /// Reset all internal data structures, build spanning tree
    /// and compute a post-order of the control flow graph.
    fn compute_spanning_tree(&mut self, func: &Function) {
        self.nodes.resize(func.dfg.num_blocks());
        self.stree.reserve(func.dfg.num_blocks());

        if let Some(block) = func.layout.entry_block() {
            self.dfs_worklist.push(TraversalEvent::Enter(0, block));
        }

        loop {
            match self.dfs_worklist.pop() {
                Some(TraversalEvent::Enter(parent, block)) => {
                    let node = &mut self.nodes[block];
                    if node.pre_number != NOT_VISITED {
                        continue;
                    }

                    self.dfs_worklist.push(TraversalEvent::Exit(block));

                    let pre_number = self.stree.push(parent, block);
                    node.pre_number = pre_number;

                    // Use the same traversal heuristics as in traversals.rs.
                    self.dfs_worklist.extend(
                        func.block_successors(block)
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
                            // A simple optimization: push less items to the stack.
                            .filter(|successor| self.nodes[*successor].pre_number == NOT_VISITED)
                            .map(|successor| TraversalEvent::Enter(pre_number, successor)),
                    );
                }
                Some(TraversalEvent::Exit(block)) => self.postorder.push(block),
                None => break,
            }
        }
    }

    /// Eval-link procedure from the paper.
    /// For a predecessor V of node W returns V if V < W, otherwise the minimum of sdom(U),
    /// where U > W and U is on a semi-dominator path for W in CFG.
    /// Use path compression to bring complexity down to O(m*log(n)).
    fn eval(&mut self, v: u32, last_linked: u32) -> u32 {
        if self.stree[v].ancestor < last_linked {
            return self.stree[v].label;
        }

        // Follow semi-dominator path.
        let mut root = v;
        loop {
            self.eval_worklist.push(root);
            root = self.stree[root].ancestor;

            if self.stree[root].ancestor < last_linked {
                break;
            }
        }

        let mut prev = root;
        let root = self.stree[prev].ancestor;

        // Perform path compression. Point all ancestors to the root
        // and propagate minimal sdom(U) value from ancestors to children.
        while let Some(curr) = self.eval_worklist.pop() {
            if self.stree[prev].label < self.stree[curr].label {
                self.stree[curr].label = self.stree[prev].label;
            }

            self.stree[curr].ancestor = root;
            prev = curr;
        }

        self.stree[v].label
    }

    fn compute_domtree(&mut self, cfg: &ControlFlowGraph) {
        // Compute semi-dominators.
        for w in (1..self.stree.len() as u32).rev() {
            let w_node = &mut self.stree[w];
            let block = w_node.block.expect("Virtual root must have been excluded");
            let mut semi = w_node.ancestor;

            let last_linked = w + 1;

            for pred in cfg
                .pred_iter(block)
                .map(|pred: BlockPredecessor| pred.block)
            {
                // Skip unreachable nodes.
                if self.nodes[pred].pre_number == NOT_VISITED {
                    continue;
                }

                let semi_candidate = self.eval(self.nodes[pred].pre_number, last_linked);
                semi = std::cmp::min(semi, semi_candidate);
            }

            let w_node = &mut self.stree[w];
            w_node.label = semi;
            w_node.semi = semi;
        }

        // Compute immediate dominators.
        for v in 1..self.stree.len() as u32 {
            let semi = self.stree[v].semi;
            let block = self.stree[v]
                .block
                .expect("Virtual root must have been excluded");
            let mut idom = self.stree[v].idom;

            while idom > semi {
                idom = self.stree[idom].idom;
            }

            self.stree[v].idom = idom;

            self.nodes[block].idom = self.stree[idom].block;
        }
    }
}

/// Optional pre-order information that can be computed for a dominator tree.
///
/// This data structure is computed from a `DominatorTree` and provides:
///
/// - A forward traversable dominator tree through the `children()` iterator.
/// - An ordering of blocks according to a dominator tree pre-order.
/// - Constant time dominance checks at the block granularity.
///
/// The information in this auxiliary data structure is not easy to update when the control flow
/// graph changes, which is why it is kept separate.
pub struct DominatorTreePreorder {
    nodes: SecondaryMap<Block, ExtraNode>,

    // Scratch memory used by `compute_postorder()`.
    stack: Vec<Block>,
}

#[derive(Default, Clone)]
struct ExtraNode {
    /// First child node in the domtree.
    child: PackedOption<Block>,

    /// Next sibling node in the domtree. This linked list is ordered according to the CFG RPO.
    sibling: PackedOption<Block>,

    /// Sequence number for this node in a pre-order traversal of the dominator tree.
    /// Unreachable blocks have number 0, the entry block is 1.
    pre_number: u32,

    /// Maximum `pre_number` for the sub-tree of the dominator tree that is rooted at this node.
    /// This is always >= `pre_number`.
    pre_max: u32,
}

/// Creating and computing the dominator tree pre-order.
impl DominatorTreePreorder {
    /// Create a new blank `DominatorTreePreorder`.
    pub fn new() -> Self {
        Self {
            nodes: SecondaryMap::new(),
            stack: Vec::new(),
        }
    }

    /// Recompute this data structure to match `domtree`.
    pub fn compute(&mut self, domtree: &DominatorTree) {
        self.nodes.clear();

        // Step 1: Populate the child and sibling links.
        //
        // By following the CFG post-order and pushing to the front of the lists, we make sure that
        // sibling lists are ordered according to the CFG reverse post-order.
        for &block in domtree.cfg_postorder() {
            if let Some(idom) = domtree.idom(block) {
                let sib = mem::replace(&mut self.nodes[idom].child, block.into());
                self.nodes[block].sibling = sib;
            } else {
                // The only block without an immediate dominator is the entry.
                self.stack.push(block);
            }
        }

        // Step 2. Assign pre-order numbers from a DFS of the dominator tree.
        debug_assert!(self.stack.len() <= 1);
        let mut n = 0;
        while let Some(block) = self.stack.pop() {
            n += 1;
            let node = &mut self.nodes[block];
            node.pre_number = n;
            node.pre_max = n;
            if let Some(n) = node.sibling.expand() {
                self.stack.push(n);
            }
            if let Some(n) = node.child.expand() {
                self.stack.push(n);
            }
        }

        // Step 3. Propagate the `pre_max` numbers up the tree.
        // The CFG post-order is topologically ordered w.r.t. dominance so a node comes after all
        // its dominator tree children.
        for &block in domtree.cfg_postorder() {
            if let Some(idom) = domtree.idom(block) {
                let pre_max = cmp::max(self.nodes[block].pre_max, self.nodes[idom].pre_max);
                self.nodes[idom].pre_max = pre_max;
            }
        }
    }
}

/// An iterator that enumerates the direct children of a block in the dominator tree.
pub struct ChildIter<'a> {
    dtpo: &'a DominatorTreePreorder,
    next: PackedOption<Block>,
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Block> {
        let n = self.next.expand();
        if let Some(block) = n {
            self.next = self.dtpo.nodes[block].sibling;
        }
        n
    }
}

/// Query interface for the dominator tree pre-order.
impl DominatorTreePreorder {
    /// Get an iterator over the direct children of `block` in the dominator tree.
    ///
    /// These are the block's whose immediate dominator is an instruction in `block`, ordered according
    /// to the CFG reverse post-order.
    pub fn children(&self, block: Block) -> ChildIter<'_> {
        ChildIter {
            dtpo: self,
            next: self.nodes[block].child,
        }
    }

    /// Fast, constant time dominance check with block granularity.
    ///
    /// This computes the same result as `domtree.dominates(a, b)`, but in guaranteed fast constant
    /// time. This is less general than the `DominatorTree` method because it only works with block
    /// program points.
    ///
    /// A block is considered to dominate itself.
    pub fn dominates(&self, a: Block, b: Block) -> bool {
        let na = &self.nodes[a];
        let nb = &self.nodes[b];
        na.pre_number <= nb.pre_number && na.pre_max >= nb.pre_max
    }

    /// Checks if one instruction dominates another.
    pub fn dominates_inst(&self, a: Inst, b: Inst, layout: &Layout) -> bool {
        match (layout.inst_block(a), layout.inst_block(b)) {
            (Some(block_a), Some(block_b)) => {
                if block_a == block_b {
                    layout.pp_cmp(a, b) != Ordering::Greater
                } else {
                    self.dominates(block_a, block_b)
                }
            }
            _ => false,
        }
    }

    /// Compare two blocks according to the dominator pre-order.
    pub fn pre_cmp_block(&self, a: Block, b: Block) -> Ordering {
        self.nodes[a].pre_number.cmp(&self.nodes[b].pre_number)
    }

    /// Compare two program points according to the dominator tree pre-order.
    ///
    /// This ordering of program points have the property that given a program point, pp, all the
    /// program points dominated by pp follow immediately and contiguously after pp in the order.
    pub fn pre_cmp<A, B>(&self, a: A, b: B, layout: &Layout) -> Ordering
    where
        A: Into<ProgramPoint>,
        B: Into<ProgramPoint>,
    {
        let a = a.into();
        let b = b.into();
        self.pre_cmp_block(layout.pp_block(a), layout.pp_block(b))
            .then_with(|| layout.pp_cmp(a, b))
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
        let dtree = DominatorTree::with_function(&func, &cfg);
        assert_eq!(0, dtree.nodes.keys().count());
        assert_eq!(dtree.cfg_postorder(), &[]);

        let mut dtpo = DominatorTreePreorder::new();
        dtpo.compute(&dtree);
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
        let dt = DominatorTree::with_function(cur.func, &cfg);

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

        let mut dtpo = DominatorTreePreorder::new();
        dtpo.compute(&dt);
        assert!(dtpo.dominates(block0, block0));
        assert!(!dtpo.dominates(block0, block1));
        assert!(dtpo.dominates(block0, block2));
        assert!(!dtpo.dominates(block1, block0));
        assert!(dtpo.dominates(block1, block1));
        assert!(!dtpo.dominates(block1, block2));
        assert!(!dtpo.dominates(block2, block0));
        assert!(!dtpo.dominates(block2, block1));
        assert!(dtpo.dominates(block2, block2));
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
        let dt = DominatorTree::with_function(cur.func, &cfg);

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
        let dt = DominatorTree::with_function(cur.func, &cfg);

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
        let dt = DominatorTree::with_function(cur.func, &cfg);

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

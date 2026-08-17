//! A Constant-Phi-Node removal pass.

use crate::dominator_tree::DominatorTree;
use crate::ir;
use crate::ir::Function;
use crate::ir::{Block, BlockArg, BlockCall, Inst, Value};
use crate::timing;
use bumpalo::Bump;
use cranelift_entity::SecondaryMap;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

// A note on notation.  For the sake of clarity, this file uses the phrase
// "formal parameters" to mean the `Value`s listed in the block head, and
// "actual parameters" to mean the `Value`s passed in a branch or a jump:
//
// block4(v16: i32, v18: i32):            <-- formal parameters
//   ...
//   brif v27, block7(v22, v24), block6   <-- actual parameters

// This transformation pass (conceptually) partitions all values in the
// function into two groups:
//
// * Group A: values defined by block formal parameters, except for the entry block.
//
// * Group B: All other values: that is, values defined by instructions,
//   and the formals of the entry block.
//
// For each value in Group A, it attempts to establish whether it will have
// the value of exactly one member of Group B.  If so, the formal parameter is
// deleted, all corresponding actual parameters (in jumps/branches to the
// defining block) are deleted, and a rename is inserted.
//
// The entry block is special-cased because (1) we don't know what values flow
// to its formals and (2) in any case we can't change its formals.
//
// Work proceeds in three phases.
//
// * Phase 1: examine all instructions.  For each block, make up a useful
//   grab-bag of information, `BlockSummary`, that summarises the block's
//   formals and jump/branch instruction.  This is used by Phases 2 and 3.
//
// * Phase 2: for each value in Group A, try to find a single Group B value
//   that flows to it.  This is done using a classical iterative forward
//   dataflow analysis over a simple constant-propagation style lattice.  It
//   converges quickly in practice -- I have seen at most 4 iterations.  This
//   is relatively cheap because the iteration is done over the
//   `BlockSummary`s, and does not visit each instruction.  The resulting
//   fixed point is stored in a `SolverState`.
//
// * Phase 3: using the `SolverState` and `BlockSummary`, edit the function to
//   remove redundant formals and actuals, and to insert suitable renames.
//
// Note that the effectiveness of the analysis depends on on the fact that
// there are no copy instructions in Cranelift's IR.  If there were, the
// computation of `actual_absval` in Phase 2 would have to be extended to
// chase through such copies.
//
// For large functions, the analysis cost using the new AArch64 backend is about
// 0.6% of the non-optimising compile time, as measured by instruction counts.
// This transformation usually pays for itself several times over, though, by
// reducing the isel/regalloc cost downstream.  Gains of up to 7% have been
// seen for large functions.

/// The `Value`s (Group B) that can flow to a formal parameter (Group A).
#[derive(Clone, Copy, Debug, PartialEq)]
enum AbstractValue {
    /// Two or more values flow to this formal.
    Many,

    /// Exactly one value, as stated, flows to this formal.  The `Value`s that
    /// can appear here are exactly: `Value`s defined by `Inst`s, plus the
    /// `Value`s defined by the formals of the entry block.  Note that this is
    /// exactly the set of `Value`s that are *not* tracked in the solver below
    /// (see `SolverState`).
    One(Value /*Group B*/),

    /// No value flows to this formal.
    None,
}

impl AbstractValue {
    fn join(self, other: AbstractValue) -> AbstractValue {
        match (self, other) {
            // Joining with `None` has no effect
            (AbstractValue::None, p2) => p2,
            (p1, AbstractValue::None) => p1,
            // Joining with `Many` produces `Many`
            (AbstractValue::Many, _p2) => AbstractValue::Many,
            (_p1, AbstractValue::Many) => AbstractValue::Many,
            // The only interesting case
            (AbstractValue::One(v1), AbstractValue::One(v2)) => {
                if v1 == v2 {
                    AbstractValue::One(v1)
                } else {
                    AbstractValue::Many
                }
            }
        }
    }

    fn is_one(self) -> bool {
        matches!(self, AbstractValue::One(_))
    }
}

#[derive(Clone, Copy, Debug)]
struct OutEdge<'a> {
    /// An instruction that transfers control.
    inst: Inst,
    /// The index into branch_destinations for this instruction that corresponds
    /// to this edge.
    branch_index: u32,
    /// The block that control is transferred to.
    block: Block,
    /// The arguments to that block.
    ///
    /// These values can be from both groups A and B.
    args: &'a [BlockArg],
}

impl<'a> OutEdge<'a> {
    /// Construct a new `OutEdge` for the given instruction.
    ///
    /// Returns `None` if this is an edge without any block arguments, which
    /// means we can ignore it for this analysis's purposes.
    #[inline]
    fn new(
        bump: &'a Bump,
        dfg: &ir::DataFlowGraph,
        inst: Inst,
        branch_index: usize,
        block: BlockCall,
    ) -> Option<Self> {
        let inst_var_args = block.args(&dfg.value_lists);

        // Skip edges without params.
        if inst_var_args.len() == 0 {
            return None;
        }

        Some(OutEdge {
            inst,
            branch_index: branch_index as u32,
            block: block.block(&dfg.value_lists),
            args: bump.alloc_slice_fill_iter(
                inst_var_args.map(|arg| arg.map_value(|value| dfg.resolve_aliases(value))),
            ),
        })
    }
}

/// For some block, a useful bundle of info.  The `Block` itself is not stored
/// here since it will be the key in the associated `FxHashMap` -- see
/// `summaries` below.  For the `SmallVec` tuning params: most blocks have
/// few parameters, hence `4`.  And almost all blocks have either one or two
/// successors, hence `2`.
#[derive(Clone, Debug, Default)]
struct BlockSummary<'a> {
    /// Formal parameters for this `Block`.
    ///
    /// These values are from group A.
    formals: &'a [Value],

    /// Each outgoing edge from this block.
    ///
    /// We don't bother to include transfers that pass zero parameters
    /// since that makes more work for the solver for no purpose.
    ///
    /// We optimize for the case where a branch instruction has up to two
    /// outgoing edges, as unconditional jumps and conditional branches are
    /// more prominent than br_table.
    dests: SmallVec<[OutEdge<'a>; 2]>,
}

impl<'a> BlockSummary<'a> {
    /// Construct a new `BlockSummary`, using `values` as its backing storage.
    #[inline]
    fn new(bump: &'a Bump, formals: &[Value]) -> Self {
        Self {
            formals: bump.alloc_slice_copy(formals),
            dests: Default::default(),
        }
    }
}

/// Solver state.  This holds a AbstractValue for each formal parameter, except
/// for those from the entry block.
struct SolverState {
    absvals: FxHashMap<Value /*Group A*/, AbstractValue>,
}

impl SolverState {
    fn new() -> Self {
        Self {
            absvals: FxHashMap::default(),
        }
    }

    fn get(&self, actual: Value) -> AbstractValue {
        *self
            .absvals
            .get(&actual)
            .unwrap_or_else(|| panic!("SolverState::get: formal param {actual:?} is untracked?!"))
    }

    fn maybe_get(&self, actual: Value) -> Option<&AbstractValue> {
        self.absvals.get(&actual)
    }

    fn set(&mut self, actual: Value, lp: AbstractValue) {
        match self.absvals.insert(actual, lp) {
            Some(_old_lp) => {}
            None => panic!("SolverState::set: formal param {actual:?} is untracked?!"),
        }
    }
}

/// Detect phis in `func` that will only ever produce one value, using a
/// classic forward dataflow analysis.  Then remove them.
#[inline(never)]
pub fn do_remove_constant_phis(func: &mut Function, domtree: &mut DominatorTree) {
    let _tt = timing::remove_constant_phis();
    debug_assert!(domtree.is_valid());

    // Phase 1 of 3: for each block, make a summary containing all relevant
    // info.  The solver will iterate over the summaries, rather than having
    // to inspect each instruction in each block.
    let bump =
        Bump::with_capacity(domtree.cfg_postorder().len() * 4 * std::mem::size_of::<Value>());
    let mut summaries =
        SecondaryMap::<Block, BlockSummary>::with_capacity(domtree.cfg_postorder().len());

    for b in domtree.cfg_rpo().copied() {
        let formals = func.dfg.block_params(b);
        let mut summary = BlockSummary::new(&bump, formals);

        for inst in func.layout.block_insts(b) {
            for (ix, dest) in func.dfg.insts[inst]
                .branch_destination(&func.dfg.jump_tables, &func.dfg.exception_tables)
                .iter()
                .enumerate()
            {
                if let Some(edge) = OutEdge::new(&bump, &func.dfg, inst, ix, *dest) {
                    summary.dests.push(edge);
                }
            }
        }

        // Ensure the invariant that all blocks (except for the entry) appear
        // in the summary, *unless* they have neither formals nor any
        // param-carrying branches/jumps.
        if formals.len() > 0 || summary.dests.len() > 0 {
            summaries[b] = summary;
        }
    }

    // Phase 2 of 3: iterate over the summaries in reverse postorder,
    // computing new `AbstractValue`s for each tracked `Value`.  The set of
    // tracked `Value`s is exactly Group A as described above.

    let entry_block = func
        .layout
        .entry_block()
        .expect("remove_constant_phis: entry block unknown");

    // Set up initial solver state
    let mut state = SolverState::new();

    for b in domtree.cfg_rpo().copied() {
        // For each block, get the formals
        if b == entry_block {
            continue;
        }
        let formals = func.dfg.block_params(b);
        for formal in formals {
            let mb_old_absval = state.absvals.insert(*formal, AbstractValue::None);
            assert!(mb_old_absval.is_none());
        }
    }

    // Solve: repeatedly traverse the blocks in reverse postorder, until there
    // are no changes.
    let mut iter_no = 0;
    loop {
        iter_no += 1;
        let mut changed = false;

        for src in domtree.cfg_rpo().copied() {
            let src_summary = &summaries[src];
            for edge in &src_summary.dests {
                assert!(edge.block != entry_block);
                // By contrast, the dst block must have a summary.  Phase 1
                // will have only included an entry in `src_summary.dests` if
                // that branch/jump carried at least one parameter.  So the
                // dst block does take parameters, so it must have a summary.
                let dst_summary = &summaries[edge.block];
                let dst_formals = &dst_summary.formals;
                assert_eq!(edge.args.len(), dst_formals.len());
                for (formal, actual) in dst_formals.iter().zip(edge.args) {
                    // Find the abstract value for `actual`.  If it is a block
                    // formal parameter then the most recent abstract value is
                    // to be found in the solver state.  If not, then it's a
                    // real value defining point (not a phi), in which case
                    // return it itself.
                    let actual_absval = match actual {
                        BlockArg::Value(actual) => match state.maybe_get(*actual) {
                            Some(pt) => *pt,
                            None => AbstractValue::One(*actual),
                        },
                        _ => AbstractValue::Many,
                    };

                    // And `join` the new value with the old.
                    let formal_absval_old = state.get(*formal);
                    let formal_absval_new = formal_absval_old.join(actual_absval);
                    if formal_absval_new != formal_absval_old {
                        changed = true;
                        state.set(*formal, formal_absval_new);
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    let mut n_consts = 0;
    for absval in state.absvals.values() {
        if absval.is_one() {
            n_consts += 1;
        }
    }

    // Phase 3 of 3: edit the function to remove constant formals, using the
    // summaries and the final solver state as a guide.

    // Make up a set of blocks that need editing.
    let mut need_editing = FxHashSet::<Block>::default();
    for (block, summary) in summaries.iter() {
        if block == entry_block {
            continue;
        }
        for formal in summary.formals {
            let formal_absval = state.get(*formal);
            if formal_absval.is_one() {
                need_editing.insert(block);
                break;
            }
        }
    }

    // Firstly, deal with the formals.  For each formal which is redundant,
    // remove it, and also add a reroute from it to the constant value which
    // it we know it to be.
    for b in &need_editing {
        let mut del_these = SmallVec::<[(Value, Value); 32]>::new();
        let formals: &[Value] = func.dfg.block_params(*b);
        for formal in formals {
            // The state must give an absval for `formal`.
            if let AbstractValue::One(replacement_val) = state.get(*formal) {
                del_these.push((*formal, replacement_val));
            }
        }
        // We can delete the formals in any order.  However,
        // `remove_block_param` works by sliding backwards all arguments to
        // the right of the value it is asked to delete.  Hence when removing more
        // than one formal, it is significantly more efficient to ask it to
        // remove the rightmost formal first, and hence this `rev()`.
        for (redundant_formal, replacement_val) in del_these.into_iter().rev() {
            func.dfg.remove_block_param(redundant_formal);
            func.dfg.change_to_alias(redundant_formal, replacement_val);
        }
    }

    // Secondly, visit all branch insns.  If the destination has had its
    // formals changed, change the actuals accordingly.  Don't scan all insns,
    // rather just visit those as listed in the summaries we prepared earlier.
    let mut old_actuals = alloc::vec::Vec::new();
    for summary in summaries.values() {
        for edge in &summary.dests {
            if !need_editing.contains(&edge.block) {
                continue;
            }

            let dfg = &mut func.dfg;
            let dests = dfg.insts[edge.inst]
                .branch_destination_mut(&mut dfg.jump_tables, &mut dfg.exception_tables);
            let block = &mut dests[edge.branch_index as usize];

            old_actuals.extend(block.args(&dfg.value_lists));

            // Check that the numbers of arguments make sense.
            let formals = &summaries[edge.block].formals;
            assert_eq!(formals.len(), old_actuals.len());

            // Filter out redundant block arguments.
            let mut formals = formals.iter();
            old_actuals.retain(|_| {
                let formal_i = formals.next().unwrap();
                !state.get(*formal_i).is_one()
            });

            // Replace the block with a new one that only includes the non-redundant arguments.
            // This leaks the value list from the old block,
            // https://github.com/bytecodealliance/wasmtime/issues/5451 for more information.
            let destination = block.block(&dfg.value_lists);
            *block = BlockCall::new(destination, old_actuals.drain(..), &mut dfg.value_lists);
        }
    }

    log::debug!(
        "do_remove_constant_phis: done, {} iters.   {} formals, of which {} const.",
        iter_no,
        state.absvals.len(),
        n_consts
    );
}

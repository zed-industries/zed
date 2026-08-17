//! Support for egraphs represented in the DataFlowGraph.

use crate::alias_analysis::{AliasAnalysis, LastStores};
use crate::ctxhash::{CtxEq, CtxHash, NullCtx};
use crate::cursor::{Cursor, CursorPosition, FuncCursor};
use crate::dominator_tree::{DominatorTree, DominatorTreePreorder};
use crate::egraph::elaborate::Elaborator;
use crate::inst_predicates::{is_mergeable_for_egraph, is_pure_for_egraph};
use crate::ir::pcc::Fact;
use crate::ir::{
    Block, DataFlowGraph, Function, Inst, InstructionData, Opcode, Type, Value, ValueDef,
    ValueListPool,
};
use crate::loop_analysis::LoopAnalysis;
use crate::opts::IsleContext;
use crate::opts::generated_code::SkeletonInstSimplification;
use crate::scoped_hash_map::{Entry as ScopedEntry, ScopedHashMap};
use crate::settings::Flags;
use crate::take_and_replace::TakeAndReplace;
use crate::trace;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::hash::Hasher;
use cranelift_control::ControlPlane;
use cranelift_entity::SecondaryMap;
use cranelift_entity::packed_option::ReservedValue;
use rustc_hash::FxHashSet;
use smallvec::SmallVec;

mod cost;
mod elaborate;

/// Pass over a Function that does the whole aegraph thing.
///
/// - Removes non-skeleton nodes from the Layout.
/// - Performs a GVN-and-rule-application pass over all Values
///   reachable from the skeleton, potentially creating new Union
///   nodes (i.e., an aegraph) so that some values have multiple
///   representations.
/// - Does "extraction" on the aegraph: selects the best value out of
///   the tree-of-Union nodes for each used value.
/// - Does "scoped elaboration" on the aegraph: chooses one or more
///   locations for pure nodes to become instructions again in the
///   layout, as forced by the skeleton.
///
/// At the beginning and end of this pass, the CLIF should be in a
/// state that passes the verifier and, additionally, has no Union
/// nodes. During the pass, Union nodes may exist, and instructions in
/// the layout may refer to results of instructions that are not
/// placed in the layout.
pub struct EgraphPass<'a> {
    /// The function we're operating on.
    func: &'a mut Function,
    /// Dominator tree for the CFG, used to visit blocks in pre-order
    /// so we see value definitions before their uses, and also used for
    /// O(1) dominance checks.
    domtree: DominatorTreePreorder,
    /// Alias analysis, used during optimization.
    alias_analysis: &'a mut AliasAnalysis<'a>,
    /// Loop analysis results, used for built-in LICM during
    /// elaboration.
    loop_analysis: &'a LoopAnalysis,
    /// Compiler flags.
    flags: &'a Flags,
    /// Chaos-mode control-plane so we can test that we still get
    /// correct results when our heuristics make bad decisions.
    ctrl_plane: &'a mut ControlPlane,
    /// Which Values do we want to rematerialize in each block where
    /// they're used?
    remat_values: FxHashSet<Value>,
    /// Stats collected while we run this pass.
    pub(crate) stats: Stats,
}

/// The maximum number of rewrites we will take from a single call into ISLE.
const MATCHES_LIMIT: usize = 5;

/// The maximum number of enodes in any given eclass.
const ECLASS_ENODE_LIMIT: usize = 5;

/// Context passed through node insertion and optimization.
pub(crate) struct OptimizeCtx<'opt, 'analysis>
where
    'analysis: 'opt,
{
    // Borrowed from EgraphPass:
    pub(crate) func: &'opt mut Function,
    pub(crate) value_to_opt_value: &'opt mut SecondaryMap<Value, Value>,
    available_block: &'opt mut SecondaryMap<Value, Block>,
    eclass_size: &'opt mut SecondaryMap<Value, u8>,
    pub(crate) gvn_map: &'opt mut ScopedHashMap<(Type, InstructionData), Option<Value>>,
    pub(crate) gvn_map_blocks: &'opt Vec<Block>,
    pub(crate) remat_values: &'opt mut FxHashSet<Value>,
    pub(crate) stats: &'opt mut Stats,
    domtree: &'opt DominatorTreePreorder,
    pub(crate) alias_analysis: &'opt mut AliasAnalysis<'analysis>,
    pub(crate) alias_analysis_state: &'opt mut LastStores,
    flags: &'opt Flags,
    ctrl_plane: &'opt mut ControlPlane,
    // Held locally during optimization of one node (recursively):
    pub(crate) rewrite_depth: usize,
    pub(crate) subsume_values: FxHashSet<Value>,
    optimized_values: SmallVec<[Value; MATCHES_LIMIT]>,
    optimized_insts: SmallVec<[SkeletonInstSimplification; MATCHES_LIMIT]>,
}

/// For passing to `insert_pure_enode`. Sometimes the enode already
/// exists as an Inst (from the original CLIF), and sometimes we're in
/// the middle of creating it and want to avoid inserting it if
/// possible until we know we need it.
pub(crate) enum NewOrExistingInst {
    New(InstructionData, Type),
    Existing(Inst),
}

impl NewOrExistingInst {
    fn get_inst_key<'a>(&'a self, dfg: &'a DataFlowGraph) -> (Type, InstructionData) {
        match self {
            NewOrExistingInst::New(data, ty) => (*ty, *data),
            NewOrExistingInst::Existing(inst) => {
                let ty = dfg.ctrl_typevar(*inst);
                (ty, dfg.insts[*inst])
            }
        }
    }
}

impl<'opt, 'analysis> OptimizeCtx<'opt, 'analysis>
where
    'analysis: 'opt,
{
    /// Optimization of a single instruction.
    ///
    /// This does a few things:
    /// - Looks up the instruction in the GVN deduplication map. If we
    ///   already have the same instruction somewhere else, with the
    ///   same args, then we can alias the original instruction's
    ///   results and omit this instruction entirely.
    /// - If the instruction is "new" (not deduplicated), then apply
    ///   optimization rules:
    ///   - All of the mid-end rules written in ISLE.
    ///   - Store-to-load forwarding.
    /// - Update the value-to-opt-value map, and update the eclass
    ///   union-find, if we rewrote the value to different form(s).
    pub(crate) fn insert_pure_enode(&mut self, inst: NewOrExistingInst) -> Value {
        // Create the external context for looking up and updating the
        // GVN map. This is necessary so that instructions themselves
        // do not have to carry all the references or data for a full
        // `Eq` or `Hash` impl.
        let gvn_context = GVNContext {
            value_lists: &self.func.dfg.value_lists,
        };

        self.stats.pure_inst += 1;
        if let NewOrExistingInst::New(..) = inst {
            self.stats.new_inst += 1;
        }

        // Does this instruction already exist? If so, add entries to
        // the value-map to rewrite uses of its results to the results
        // of the original (existing) instruction. If not, optimize
        // the new instruction.
        if let Some(&Some(orig_result)) = self
            .gvn_map
            .get(&gvn_context, &inst.get_inst_key(&self.func.dfg))
        {
            self.stats.pure_inst_deduped += 1;
            if let NewOrExistingInst::Existing(inst) = inst {
                debug_assert_eq!(self.func.dfg.inst_results(inst).len(), 1);
                let result = self.func.dfg.first_result(inst);
                self.value_to_opt_value[result] = orig_result;
                self.available_block[result] = self.available_block[orig_result];
                self.func.dfg.merge_facts(result, orig_result);
            }
            orig_result
        } else {
            // Now actually insert the InstructionData and attach
            // result value (exactly one).
            let (inst, result, ty) = match inst {
                NewOrExistingInst::New(data, typevar) => {
                    self.stats.pure_inst_insert_new += 1;
                    let inst = self.func.dfg.make_inst(data);
                    // TODO: reuse return value?
                    self.func.dfg.make_inst_results(inst, typevar);
                    let result = self.func.dfg.first_result(inst);
                    // New inst. We need to do the analysis of its result.
                    (inst, result, typevar)
                }
                NewOrExistingInst::Existing(inst) => {
                    self.stats.pure_inst_insert_orig += 1;
                    let result = self.func.dfg.first_result(inst);
                    let ty = self.func.dfg.ctrl_typevar(inst);
                    (inst, result, ty)
                }
            };

            self.attach_constant_fact(inst, result, ty);

            self.available_block[result] = self.get_available_block(inst);
            let opt_value = self.optimize_pure_enode(inst);
            log::trace!("optimizing inst {inst} orig result {result} gave {opt_value}");

            let gvn_context = GVNContext {
                value_lists: &self.func.dfg.value_lists,
            };
            // Insert at level implied by args. This enables merging
            // in LICM cases like:
            //
            // while (...) {
            //   if (...) {
            //     let x = loop_invariant_expr;
            //   }
            //   if (...) {
            //     let x = loop_invariant_expr;
            //   }
            // }
            //
            // where the two instances of the expression otherwise
            // wouldn't merge because each would be in a separate
            // subscope of the scoped hashmap during traversal.
            log::trace!(
                "value {} is available at {}",
                opt_value,
                self.available_block[opt_value]
            );
            let depth = self.depth_of_block_in_gvn_map(self.available_block[opt_value]);
            self.gvn_map.insert_with_depth(
                &gvn_context,
                (ty, self.func.dfg.insts[inst]),
                Some(opt_value),
                depth,
            );
            self.value_to_opt_value[result] = opt_value;
            opt_value
        }
    }

    /// Find the block where a pure instruction first becomes available,
    /// defined as the block that is closest to the root where all of
    /// its arguments are available. In the unusual case where a pure
    /// instruction has no arguments (e.g. get_return_address), we can
    /// place it anywhere, so it is available in the entry block.
    ///
    /// This function does not compute available blocks recursively.
    /// All of the instruction's arguments must have had their available
    /// blocks assigned already.
    fn get_available_block(&self, inst: Inst) -> Block {
        // Side-effecting instructions have different rules for where
        // they become available, so this function does not apply.
        debug_assert!(is_pure_for_egraph(self.func, inst));

        // Note that the def-point of all arguments to an instruction
        // in SSA lie on a line of direct ancestors in the domtree, and
        // so do their available-blocks. This means that for any pair of
        // arguments, their available blocks are either the same or one
        // strictly dominates the other. We just need to find any argument
        // whose available block is deepest in the domtree.
        self.func.dfg.insts[inst]
            .arguments(&self.func.dfg.value_lists)
            .iter()
            .map(|&v| {
                let block = self.available_block[v];
                debug_assert!(!block.is_reserved_value());
                block
            })
            .max_by(|&x, &y| {
                if self.domtree.dominates(x, y) {
                    Ordering::Less
                } else {
                    debug_assert!(self.domtree.dominates(y, x));
                    Ordering::Greater
                }
            })
            .unwrap_or(self.func.layout.entry_block().unwrap())
    }

    fn depth_of_block_in_gvn_map(&self, block: Block) -> usize {
        log::trace!(
            "finding depth of available block {} in domtree stack: {:?}",
            block,
            self.gvn_map_blocks
        );
        self.gvn_map_blocks
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, b)| *b == block)
            .unwrap()
            .0
    }

    /// Optimizes an enode by applying any matching mid-end rewrite
    /// rules (or store-to-load forwarding, which is a special case),
    /// unioning together all possible optimized (or rewritten) forms
    /// of this expression into an eclass and returning the `Value`
    /// that represents that eclass.
    fn optimize_pure_enode(&mut self, inst: Inst) -> Value {
        // A pure node always has exactly one result.
        let orig_value = self.func.dfg.first_result(inst);

        let mut guard = TakeAndReplace::new(self, |x| &mut x.optimized_values);
        let (ctx, optimized_values) = guard.get();

        // Limit rewrite depth. When we apply optimization rules, they
        // may create new nodes (values) and those are, recursively,
        // optimized eagerly as soon as they are created. So we may
        // have more than one ISLE invocation on the stack. (This is
        // necessary so that as the toplevel builds the
        // right-hand-side expression bottom-up, it uses the "latest"
        // optimized values for all the constituent parts.) To avoid
        // infinite or problematic recursion, we bound the rewrite
        // depth to a small constant here.
        const REWRITE_LIMIT: usize = 5;
        if ctx.rewrite_depth > REWRITE_LIMIT {
            ctx.stats.rewrite_depth_limit += 1;
            return orig_value;
        }
        ctx.rewrite_depth += 1;
        trace!("Incrementing rewrite depth; now {}", ctx.rewrite_depth);

        // Invoke the ISLE toplevel constructor, getting all new
        // values produced as equivalents to this value.
        trace!("Calling into ISLE with original value {}", orig_value);
        ctx.stats.rewrite_rule_invoked += 1;
        debug_assert!(optimized_values.is_empty());
        crate::opts::generated_code::constructor_simplify(
            &mut IsleContext { ctx },
            orig_value,
            optimized_values,
        );

        ctx.stats.rewrite_rule_results += optimized_values.len() as u64;

        // It's not supposed to matter what order `simplify` returns values in.
        ctx.ctrl_plane.shuffle(optimized_values);

        let num_matches = optimized_values.len();
        if num_matches > MATCHES_LIMIT {
            trace!(
                "Reached maximum matches limit; too many optimized values \
                 ({num_matches} > {MATCHES_LIMIT}); ignoring rest.",
            );
            optimized_values.truncate(MATCHES_LIMIT);
        }

        // Sort and deduplicate optimized values, in case multiple
        // rules produced the same simplification.
        optimized_values.sort_unstable();
        optimized_values.dedup();

        trace!("  -> returned from ISLE: {orig_value} -> {optimized_values:?}");

        // Construct a union-node tree representing the new eclass
        // that results from rewriting. If any returned value was
        // marked "subsume", take only that value. Otherwise,
        // sequentially build the chain over the original value and
        // all returned values.
        let result_value = if let Some(&subsuming_value) = optimized_values
            .iter()
            .find(|&value| ctx.subsume_values.contains(value))
        {
            optimized_values.clear();
            ctx.stats.pure_inst_subsume += 1;
            subsuming_value
        } else {
            let mut union_value = orig_value;
            let mut eclass_size = ctx.eclass_size[orig_value] + 1;
            for optimized_value in optimized_values.drain(..) {
                trace!(
                    "Returned from ISLE for {}, got {:?}",
                    orig_value, optimized_value
                );
                if optimized_value == orig_value {
                    trace!(" -> same as orig value; skipping");
                    ctx.stats.pure_inst_rewrite_to_self += 1;
                    continue;
                }
                let rhs_eclass_size = ctx.eclass_size[optimized_value] + 1;
                if usize::from(eclass_size) + usize::from(rhs_eclass_size) > ECLASS_ENODE_LIMIT {
                    trace!(" -> reached eclass size limit");
                    ctx.stats.eclass_size_limit += 1;
                    break;
                }
                let old_union_value = union_value;
                union_value = ctx.func.dfg.union(old_union_value, optimized_value);
                eclass_size += rhs_eclass_size;
                ctx.eclass_size[union_value] = eclass_size - 1;
                ctx.stats.union += 1;
                trace!(" -> union: now {}", union_value);
                ctx.func.dfg.merge_facts(old_union_value, optimized_value);
                ctx.available_block[union_value] =
                    ctx.merge_availability(old_union_value, optimized_value);
            }
            union_value
        };

        ctx.rewrite_depth -= 1;
        trace!("Decrementing rewrite depth; now {}", ctx.rewrite_depth);
        if ctx.rewrite_depth == 0 {
            ctx.subsume_values.clear();
        }

        debug_assert!(ctx.optimized_values.is_empty());
        result_value
    }

    fn merge_availability(&self, a: Value, b: Value) -> Block {
        let a = self.available_block[a];
        let b = self.available_block[b];
        if self.domtree.dominates(a, b) { a } else { b }
    }

    /// Optimize a "skeleton" instruction.
    ///
    /// Returns an optional command of how to continue processing the optimized
    /// instruction (e.g. removing it or replacing it with a new instruction).
    fn optimize_skeleton_inst(
        &mut self,
        inst: Inst,
        block: Block,
    ) -> Option<SkeletonInstSimplification> {
        self.stats.skeleton_inst += 1;

        // If we have a rewrite rule for this instruction, do that first, so
        // that GVN and alias analysis only see simplified skeleton
        // instructions.
        if let Some(cmd) = self.simplify_skeleton_inst(inst) {
            self.stats.skeleton_inst_simplified += 1;
            return Some(cmd);
        }

        // First, can we try to deduplicate? We need to keep some copy
        // of the instruction around because it's side-effecting, but
        // we may be able to reuse an earlier instance of it.
        if is_mergeable_for_egraph(self.func, inst) {
            let result = self.func.dfg.inst_results(inst).get(0).copied();
            trace!(" -> mergeable side-effecting op {}", inst);

            // Does this instruction already exist? If so, add entries to
            // the value-map to rewrite uses of its results to the results
            // of the original (existing) instruction. If not, optimize
            // the new instruction.
            //
            // Note that the GVN map is scoped, which is important
            // here: because effectful ops are not removed from the
            // skeleton (`Layout`), we need to be mindful of whether
            // our current position is dominated by an instance of the
            // instruction. (See #5796 for details.)
            let ty = self.func.dfg.ctrl_typevar(inst);
            match self
                .gvn_map
                .entry(&NullCtx, (ty, self.func.dfg.insts[inst]))
            {
                ScopedEntry::Occupied(o) => {
                    let orig_result = *o.get();
                    match (result, orig_result) {
                        (Some(result), Some(orig_result)) => {
                            // Hit in GVN map -- reuse value.
                            self.stats.skeleton_inst_gvn += 1;
                            self.value_to_opt_value[result] = orig_result;
                            self.available_block[result] = self.available_block[orig_result];
                            trace!(" -> merges result {} to {}", result, orig_result);
                        }
                        (None, None) => {
                            // Hit in the GVN map, but the instruction doesn't
                            // produce results, only side effects. Nothing else
                            // to do here.
                            self.stats.skeleton_inst_gvn += 1;
                            trace!(" -> merges with dominating instruction");
                        }
                        (_, _) => unreachable!(),
                    }
                    Some(SkeletonInstSimplification::Remove)
                }
                ScopedEntry::Vacant(v) => {
                    // Otherwise, insert it into the value-map.
                    if let Some(result) = result {
                        self.value_to_opt_value[result] = result;
                        self.available_block[result] = block;
                    }
                    v.insert(result);
                    trace!(" -> inserts as new (no GVN)");
                    None
                }
            }
        }
        // Otherwise, if a load or store, process it with the alias
        // analysis to see if we can optimize it (rewrite in terms of
        // an earlier load or stored value).
        else if let Some(new_result) =
            self.alias_analysis
                .process_inst(self.func, self.alias_analysis_state, inst)
        {
            self.stats.alias_analysis_removed += 1;
            let result = self.func.dfg.first_result(inst);
            trace!(
                " -> inst {} has result {} replaced with {}",
                inst, result, new_result
            );
            self.value_to_opt_value[result] = new_result;
            self.available_block[result] = self.available_block[new_result];
            self.func.dfg.merge_facts(result, new_result);
            Some(SkeletonInstSimplification::Remove)
        }
        // Otherwise, generic side-effecting op -- always keep it, and
        // set its results to identity-map to original values.
        else {
            // Set all results to identity-map to themselves
            // in the value-to-opt-value map.
            for &result in self.func.dfg.inst_results(inst) {
                self.value_to_opt_value[result] = result;
                self.available_block[result] = block;
            }
            None
        }
    }

    /// Find the best simplification of the given skeleton instruction, if any,
    /// by consulting our `simplify_skeleton` ISLE rules.
    fn simplify_skeleton_inst(&mut self, inst: Inst) -> Option<SkeletonInstSimplification> {
        // We cannot currently simplify terminators, or simplify into
        // terminators. Anything that could change the control-flow graph is off
        // limits.
        //
        // Consider the following CLIF snippet:
        //
        //     block0(v0: i64):
        //         v1 = iconst.i32 0
        //         trapz v1, user42
        //         v2 = load.i32 v0
        //         brif v1, block1, block2
        //     block1:
        //         return v2
        //     block2:
        //         v3 = iconst.i32 1
        //         v4 = iadd v2, v3
        //         return v4
        //
        // We would ideally like to perform simplifications like replacing the
        // `trapz` with an unconditional `trap` and the conditional `brif`
        // branch with an unconditional `jump`. Note, however, that blocks
        // `block1` and `block2` are dominated by `block0` and therefore can and
        // do use values defined in `block0`. This presents challenges:
        //
        // * If we replace the `brif` with a `jump`, then we've mutated the
        //   control-flow graph and removed that domination property. The uses
        //   of `v2` and `v3` in those blocks become invalid.
        //
        // * Even worse, if we turn the `trapz` into a `trap`, we are
        //   introducing a terminator into the middle of the block, which leaves
        //   us with two choices to fix up the IR so that there aren't any
        //   instructions following the terminator in the block:
        //
        //   1. We can split the unreachable instructions off into a new
        //      block. However, there is no control-flow edge from the current
        //      block to this new block and so, again, the new block isn't
        //      dominated by the current block, and therefore the can't use
        //      values defined in this block or any dominating it. The `load`
        //      instruction uses `v0` but is not dominated by `v0`'s
        //      definition.
        //
        //   2. Alternatively, we can simply delete the trailing instructions,
        //      since they are unreachable. But then not only are the old
        //      instructions' uses no longer dominated by their definitions, but
        //      the definitions do not exist at all anymore!
        //
        // Whatever approach we would take, we would invalidate value uses, and
        // would need to track and fix them up.
        if self.func.dfg.insts[inst].opcode().is_branch() {
            return None;
        }

        let mut guard = TakeAndReplace::new(self, |x| &mut x.optimized_insts);
        let (ctx, optimized_insts) = guard.get();

        crate::opts::generated_code::constructor_simplify_skeleton(
            &mut IsleContext { ctx },
            inst,
            optimized_insts,
        );

        let simplifications_len = optimized_insts.len();
        log::trace!(" -> simplify_skeleton: yielded {simplifications_len} simplification(s)");
        if simplifications_len > MATCHES_LIMIT {
            log::trace!("      too many candidate simplifications; truncating to {MATCHES_LIMIT}");
            optimized_insts.truncate(MATCHES_LIMIT);
        }

        // Find the best simplification, if any, from our candidates.
        //
        // Unlike simplifying pure values, we do not add side-effectful
        // instructions to the egraph, nor do we extract the best version via
        // dynamic programming and considering the costs of operands. Instead,
        // we greedily choose the best simplification. This is because there is
        // an impedance mismatch: the egraph and our pure rewrites are centered
        // around *values*, but we don't represent side-effects with values, we
        // represent them implicitly in their *instructions*.
        //
        // The initial best choice is "no simplification, just use the original
        // instruction" which has the original instruction's cost.
        let mut best = None;
        let mut best_cost = cost::Cost::of_skeleton_op(
            ctx.func.dfg.insts[inst].opcode(),
            ctx.func.dfg.inst_args(inst).len(),
        );
        while let Some(simplification) = optimized_insts.pop() {
            let (new_inst, new_val) = match simplification {
                // We can't do better than completely removing the skeleton
                // instruction, so short-cicuit the loop and eagerly return the
                // `Remove*` simplifications.
                SkeletonInstSimplification::Remove => {
                    log::trace!(" -> simplify_skeleton: remove inst");
                    debug_assert!(ctx.func.dfg.inst_results(inst).is_empty());
                    return Some(simplification);
                }
                SkeletonInstSimplification::RemoveWithVal { val } => {
                    log::trace!(" -> simplify_skeleton: remove inst and use {val} as its result");
                    if cfg!(debug_assertions) {
                        let results = ctx.func.dfg.inst_results(inst);
                        debug_assert_eq!(results.len(), 1);
                        debug_assert_eq!(
                            ctx.func.dfg.value_type(results[0]),
                            ctx.func.dfg.value_type(val),
                        );
                    }
                    return Some(simplification);
                }

                // For instruction replacement simplification, we want to check
                // that the replacements define the same number and types of
                // values as the original instruction, and also determine
                // whether they are actually an improvement over (i.e. have
                // lower cost than) the original instruction.
                SkeletonInstSimplification::Replace { inst } => {
                    log::trace!(
                        " -> simplify_skeleton: replace inst with {inst}: {}",
                        ctx.func.dfg.display_inst(inst)
                    );
                    (inst, None)
                }
                SkeletonInstSimplification::ReplaceWithVal { inst, val } => {
                    log::trace!(
                        " -> simplify_skeleton: replace inst with {val} and {inst}: {}",
                        ctx.func.dfg.display_inst(inst)
                    );
                    (inst, Some(val))
                }
            };

            if cfg!(debug_assertions) {
                let opcode = ctx.func.dfg.insts[inst].opcode();
                debug_assert!(
                    !(opcode.is_terminator() || opcode.is_branch()),
                    "simplifying control-flow instructions and terminators is not yet supported",
                );

                let old_vals = ctx.func.dfg.inst_results(inst);
                let new_vals = if let Some(val) = new_val.as_ref() {
                    std::slice::from_ref(val)
                } else {
                    ctx.func.dfg.inst_results(new_inst)
                };
                debug_assert_eq!(
                    old_vals.len(),
                    new_vals.len(),
                    "skeleton simplification should result in the same number of result values",
                );

                for (old_val, new_val) in old_vals.iter().zip(new_vals) {
                    let old_ty = ctx.func.dfg.value_type(*old_val);
                    let new_ty = ctx.func.dfg.value_type(*new_val);
                    debug_assert_eq!(
                        old_ty, new_ty,
                        "skeleton simplification should result in values of the correct type",
                    );
                }
            }

            // Our best simplification is the one with the least cost. Update
            // `best` if necessary.
            let cost = cost::Cost::of_skeleton_op(
                ctx.func.dfg.insts[new_inst].opcode(),
                ctx.func.dfg.inst_args(new_inst).len(),
            );
            if cost < best_cost {
                best = Some(simplification);
                best_cost = cost;
            }
        }

        // Return the best simplification!
        best
    }

    /// Helper to propagate facts on constant values: if PCC is
    /// enabled, then unconditionally add a fact attesting to the
    /// Value's concrete value.
    fn attach_constant_fact(&mut self, inst: Inst, value: Value, ty: Type) {
        if self.flags.enable_pcc() {
            if let InstructionData::UnaryImm {
                opcode: Opcode::Iconst,
                imm,
            } = self.func.dfg.insts[inst]
            {
                let imm: i64 = imm.into();
                self.func.dfg.facts[value] =
                    Some(Fact::constant(ty.bits().try_into().unwrap(), imm as u64));
            }
        }
    }
}

impl<'a> EgraphPass<'a> {
    /// Create a new EgraphPass.
    pub fn new(
        func: &'a mut Function,
        raw_domtree: &'a DominatorTree,
        loop_analysis: &'a LoopAnalysis,
        alias_analysis: &'a mut AliasAnalysis<'a>,
        flags: &'a Flags,
        ctrl_plane: &'a mut ControlPlane,
    ) -> Self {
        let mut domtree = DominatorTreePreorder::new();
        domtree.compute(raw_domtree);
        Self {
            func,
            domtree,
            loop_analysis,
            alias_analysis,
            flags,
            ctrl_plane,
            stats: Stats::default(),
            remat_values: FxHashSet::default(),
        }
    }

    /// Run the process.
    pub fn run(&mut self) {
        self.remove_pure_and_optimize();

        trace!("egraph built:\n{}\n", self.func.display());
        if cfg!(feature = "trace-log") {
            for (value, def) in self.func.dfg.values_and_defs() {
                trace!(" -> {} = {:?}", value, def);
                match def {
                    ValueDef::Result(i, 0) => {
                        trace!("  -> {} = {:?}", i, self.func.dfg.insts[i]);
                    }
                    _ => {}
                }
            }
        }

        self.elaborate();

        log::trace!("stats: {:#?}", self.stats);
    }

    /// Remove pure nodes from the `Layout` of the function, ensuring
    /// that only the "side-effect skeleton" remains, and also
    /// optimize the pure nodes. This is the first step of
    /// egraph-based processing and turns the pure CFG-based CLIF into
    /// a CFG skeleton with a sea of (optimized) nodes tying it
    /// together.
    ///
    /// As we walk through the code, we eagerly apply optimization
    /// rules; at any given point we have a "latest version" of an
    /// eclass of possible representations for a `Value` in the
    /// original program, which is itself a `Value` at the root of a
    /// union-tree. We keep a map from the original values to these
    /// optimized values. When we encounter any instruction (pure or
    /// side-effecting skeleton) we rewrite its arguments to capture
    /// the "latest" optimized forms of these values. (We need to do
    /// this as part of this pass, and not later using a finished map,
    /// because the eclass can continue to be updated and we need to
    /// only refer to its subset that exists at this stage, to
    /// maintain acyclicity.)
    fn remove_pure_and_optimize(&mut self) {
        let mut cursor = FuncCursor::new(self.func);
        let mut value_to_opt_value: SecondaryMap<Value, Value> =
            SecondaryMap::with_default(Value::reserved_value());

        // Map from instruction to value for hash-consing of pure ops
        // into the egraph. This can be a standard (non-scoped)
        // hashmap because pure ops have no location: they are
        // "outside of" control flow.
        //
        // Note also that we keep the controlling typevar (the `Type`
        // in the tuple below) because it may disambiguate
        // instructions that are identical except for type.
        //
        // We store both skeleton and non-skeleton instructions in the
        // GVN map; for skeleton instructions, we only store those
        // that are idempotent, i.e., still eligible to GVN. Note that
        // some skeleton instructions are idempotent but do not
        // produce a value: e.g., traps on a given condition. To allow
        // for both cases, we store an `Option<Value>` as the value in
        // this map.
        let mut gvn_map: ScopedHashMap<(Type, InstructionData), Option<Value>> =
            ScopedHashMap::with_capacity(cursor.func.dfg.num_values());

        // The block in the domtree preorder traversal at each level
        // of the GVN map.
        let mut gvn_map_blocks: Vec<Block> = vec![];

        // To get the best possible merging and canonicalization, we
        // track where a value is "available" at: this is the
        // domtree-nearest-ancestor join of all args if the value
        // itself is pure, otherwise the block where the value is
        // defined. (And for union nodes, the
        // domtree-highest-ancestor, i.e., the meet or the dual to the
        // above join.)
        let mut available_block: SecondaryMap<Value, Block> =
            SecondaryMap::with_default(Block::reserved_value());

        // To avoid blowing up eclasses too much, we track the size of
        // each eclass reachable by a tree of union nodes from a given
        // value ID, and we avoid union'ing additional values into an
        // eclass when it reaches `ECLASS_ENODE_LIMIT`.
        //
        // For efficiency, this encodes size minus one: so a value of
        // zero (which is cheap to bulk-initialize) means a singleton
        // eclass of size one. This also allows us to avoid explicitly
        // writing the size for any values that are not union nodes.
        let mut eclass_size: SecondaryMap<Value, u8> = SecondaryMap::with_default(0);

        // This is an initial guess at the size we'll need, but we add
        // more values as we build simplified alternative expressions so
        // this is likely to realloc again later.
        available_block.resize(cursor.func.dfg.num_values());

        // In domtree preorder, visit blocks. (TODO: factor out an
        // iterator from this and elaborator.)
        let root = cursor.layout().entry_block().unwrap();
        enum StackEntry {
            Visit(Block),
            Pop,
        }
        let mut block_stack = vec![StackEntry::Visit(root)];
        while let Some(entry) = block_stack.pop() {
            match entry {
                StackEntry::Visit(block) => {
                    // We popped this block; push children
                    // immediately, then process this block.
                    block_stack.push(StackEntry::Pop);
                    block_stack.extend(
                        self.ctrl_plane
                            .shuffled(self.domtree.children(block))
                            .map(StackEntry::Visit),
                    );
                    gvn_map.increment_depth();
                    gvn_map_blocks.push(block);

                    trace!("Processing block {}", block);
                    cursor.set_position(CursorPosition::Before(block));

                    let mut alias_analysis_state = self.alias_analysis.block_starting_state(block);

                    for &param in cursor.func.dfg.block_params(block) {
                        trace!("creating initial singleton eclass for blockparam {}", param);
                        value_to_opt_value[param] = param;
                        available_block[param] = block;
                    }
                    while let Some(inst) = cursor.next_inst() {
                        trace!(
                            "Processing inst {inst}: {}",
                            cursor.func.dfg.display_inst(inst),
                        );

                        // Rewrite args of *all* instructions using the
                        // value-to-opt-value map.
                        cursor.func.dfg.map_inst_values(inst, |arg| {
                            let new_value = value_to_opt_value[arg];
                            trace!("rewriting arg {} of inst {} to {}", arg, inst, new_value);
                            debug_assert_ne!(new_value, Value::reserved_value());
                            new_value
                        });

                        // Build a context for optimization, with borrows of
                        // state. We can't invoke a method on `self` because
                        // we've borrowed `self.func` mutably (as
                        // `cursor.func`) so we pull apart the pieces instead
                        // here.
                        let mut ctx = OptimizeCtx {
                            func: cursor.func,
                            value_to_opt_value: &mut value_to_opt_value,
                            gvn_map: &mut gvn_map,
                            gvn_map_blocks: &mut gvn_map_blocks,
                            available_block: &mut available_block,
                            eclass_size: &mut eclass_size,
                            rewrite_depth: 0,
                            subsume_values: FxHashSet::default(),
                            remat_values: &mut self.remat_values,
                            stats: &mut self.stats,
                            domtree: &self.domtree,
                            alias_analysis: self.alias_analysis,
                            alias_analysis_state: &mut alias_analysis_state,
                            flags: self.flags,
                            ctrl_plane: self.ctrl_plane,
                            optimized_values: Default::default(),
                            optimized_insts: Default::default(),
                        };

                        if is_pure_for_egraph(ctx.func, inst) {
                            // Insert into GVN map and optimize any new nodes
                            // inserted (recursively performing this work for
                            // any nodes the optimization rules produce).
                            let inst = NewOrExistingInst::Existing(inst);
                            ctx.insert_pure_enode(inst);
                            // We've now rewritten all uses, or will when we
                            // see them, and the instruction exists as a pure
                            // enode in the eclass, so we can remove it.
                            cursor.remove_inst_and_step_back();
                        } else {
                            if let Some(cmd) = ctx.optimize_skeleton_inst(inst, block) {
                                Self::execute_skeleton_inst_simplification(
                                    cmd,
                                    &mut cursor,
                                    &mut value_to_opt_value,
                                    inst,
                                );
                            }
                        }
                    }
                }
                StackEntry::Pop => {
                    gvn_map.decrement_depth();
                    gvn_map_blocks.pop();
                }
            }
        }
    }

    /// Execute a simplification of an instruction in the side-effectful
    /// skeleton.
    fn execute_skeleton_inst_simplification(
        simplification: SkeletonInstSimplification,
        cursor: &mut FuncCursor,
        value_to_opt_value: &mut SecondaryMap<Value, Value>,
        old_inst: Inst,
    ) {
        let mut forward_val = |cursor: &mut FuncCursor, old_val, new_val| {
            cursor.func.dfg.change_to_alias(old_val, new_val);
            value_to_opt_value[old_val] = new_val;
        };

        let (new_inst, new_val) = match simplification {
            SkeletonInstSimplification::Remove => {
                cursor.remove_inst_and_step_back();
                return;
            }
            SkeletonInstSimplification::RemoveWithVal { val } => {
                cursor.remove_inst_and_step_back();
                let old_val = cursor.func.dfg.first_result(old_inst);
                cursor.func.dfg.detach_inst_results(old_inst);
                forward_val(cursor, old_val, val);
                return;
            }
            SkeletonInstSimplification::Replace { inst } => (inst, None),
            SkeletonInstSimplification::ReplaceWithVal { inst, val } => (inst, Some(val)),
        };

        // Replace the old instruction with the new one.
        cursor.replace_inst(new_inst);
        debug_assert!(!cursor.func.dfg.insts[new_inst].opcode().is_terminator());

        // Redirect the old instruction's result values to our new instruction's
        // result values.
        let mut i = 0;
        let mut next_new_val = |dfg: &crate::ir::DataFlowGraph| -> Value {
            if let Some(val) = new_val {
                val
            } else {
                let val = dfg.inst_results(new_inst)[i];
                i += 1;
                val
            }
        };
        for i in 0..cursor.func.dfg.inst_results(old_inst).len() {
            let old_val = cursor.func.dfg.inst_results(old_inst)[i];
            let new_val = next_new_val(&cursor.func.dfg);
            forward_val(cursor, old_val, new_val);
        }

        // Back up so that the next iteration of the outer egraph loop will
        // process the new instruction.
        cursor.goto_inst(new_inst);
        cursor.prev_inst();
    }

    /// Scoped elaboration: compute a final ordering of op computation
    /// for each block and update the given Func body. After this
    /// runs, the function body is back into the state where every
    /// Inst with an used result is placed in the layout (possibly
    /// duplicated, if our code-motion logic decides this is the best
    /// option).
    ///
    /// This works in concert with the domtree. We do a preorder
    /// traversal of the domtree, tracking a scoped map from Id to
    /// (new) Value. The map's scopes correspond to levels in the
    /// domtree.
    ///
    /// At each block, we iterate forward over the side-effecting
    /// eclasses, and recursively generate their arg eclasses, then
    /// emit the ops themselves.
    ///
    /// To use an eclass in a given block, we first look it up in the
    /// scoped map, and get the Value if already present. If not, we
    /// need to generate it. We emit the extracted enode for this
    /// eclass after recursively generating its args. Eclasses are
    /// thus computed "as late as possible", but then memoized into
    /// the Id-to-Value map and available to all dominated blocks and
    /// for the rest of this block. (This subsumes GVN.)
    fn elaborate(&mut self) {
        let mut elaborator = Elaborator::new(
            self.func,
            &self.domtree,
            self.loop_analysis,
            &self.remat_values,
            &mut self.stats,
            self.ctrl_plane,
        );
        elaborator.elaborate();

        self.check_post_egraph();
    }

    #[cfg(debug_assertions)]
    fn check_post_egraph(&self) {
        // Verify that no union nodes are reachable from inst args,
        // and that all inst args' defining instructions are in the
        // layout.
        for block in self.func.layout.blocks() {
            for inst in self.func.layout.block_insts(block) {
                self.func
                    .dfg
                    .inst_values(inst)
                    .for_each(|arg| match self.func.dfg.value_def(arg) {
                        ValueDef::Result(i, _) => {
                            debug_assert!(self.func.layout.inst_block(i).is_some());
                        }
                        ValueDef::Union(..) => {
                            panic!("egraph union node {arg} still reachable at {inst}!");
                        }
                        _ => {}
                    })
            }
        }
    }

    #[cfg(not(debug_assertions))]
    fn check_post_egraph(&self) {}
}

/// Implementation of external-context equality and hashing on
/// InstructionData. This allows us to deduplicate instructions given
/// some context that lets us see its value lists, so we don't need to
/// store arguments inline in the `InstuctionData` (or alongside it in
/// some newly-defined key type) in all cases.
struct GVNContext<'a> {
    value_lists: &'a ValueListPool,
}

impl<'a> CtxEq<(Type, InstructionData), (Type, InstructionData)> for GVNContext<'a> {
    fn ctx_eq(
        &self,
        (a_ty, a_inst): &(Type, InstructionData),
        (b_ty, b_inst): &(Type, InstructionData),
    ) -> bool {
        a_ty == b_ty && a_inst.eq(b_inst, self.value_lists)
    }
}

impl<'a> CtxHash<(Type, InstructionData)> for GVNContext<'a> {
    fn ctx_hash<H: Hasher>(&self, state: &mut H, (ty, inst): &(Type, InstructionData)) {
        std::hash::Hash::hash(&ty, state);
        inst.hash(state, self.value_lists);
    }
}

/// Statistics collected during egraph-based processing.
#[derive(Clone, Debug, Default)]
pub(crate) struct Stats {
    pub(crate) pure_inst: u64,
    pub(crate) pure_inst_deduped: u64,
    pub(crate) pure_inst_subsume: u64,
    pub(crate) pure_inst_rewrite_to_self: u64,
    pub(crate) pure_inst_insert_orig: u64,
    pub(crate) pure_inst_insert_new: u64,
    pub(crate) skeleton_inst: u64,
    pub(crate) skeleton_inst_simplified: u64,
    pub(crate) skeleton_inst_gvn: u64,
    pub(crate) alias_analysis_removed: u64,
    pub(crate) new_inst: u64,
    pub(crate) union: u64,
    pub(crate) subsume: u64,
    pub(crate) remat: u64,
    pub(crate) rewrite_rule_invoked: u64,
    pub(crate) rewrite_rule_results: u64,
    pub(crate) rewrite_depth_limit: u64,
    pub(crate) elaborate_visit_node: u64,
    pub(crate) elaborate_memoize_hit: u64,
    pub(crate) elaborate_memoize_miss: u64,
    pub(crate) elaborate_remat: u64,
    pub(crate) elaborate_licm_hoist: u64,
    pub(crate) elaborate_func: u64,
    pub(crate) elaborate_func_pre_insts: u64,
    pub(crate) elaborate_func_post_insts: u64,
    pub(crate) elaborate_best_cost_fixpoint_iters: u64,
    pub(crate) eclass_size_limit: u64,
}

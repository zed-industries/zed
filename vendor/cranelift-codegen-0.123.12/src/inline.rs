//! Function inlining infrastructure.
//!
//! This module provides "inlining as a library" to Cranelift users; it does
//! _not_ provide a complete, off-the-shelf inlining solution. Cranelift's
//! compilation context is per-function and does not encompass the full call
//! graph. It does not know which functions are hot and which are cold, which
//! have been marked the equivalent of `#[inline(never)]`, etc... Only the
//! Cranelift user can understand these aspects of the full compilation
//! pipeline, and these things can be very different between (say) Wasmtime and
//! `cg_clif`. Therefore, this module does not attempt to define hueristics for
//! when inlining a particular call is likely beneficial. This module only
//! provides hooks for the Cranelift user to define whether a given call should
//! be inlined or not, and the mechanics to inline a callee into a particular
//! call site when directed to do so by the Cranelift user.
//!
//! The top-level inlining entry point during Cranelift compilation is
//! [`Context::inline`][crate::Context::inline]. It takes an [`Inline`] trait
//! implementation, which is authored by the Cranelift user and directs
//! Cranelift whether to inline a particular call, and, when inlining, gives
//! Cranelift the body of the callee that is to be inlined.

use crate::cursor::{Cursor as _, FuncCursor};
use crate::ir::{self, ExceptionTableData, ExceptionTableItem, InstBuilder as _};
use crate::result::CodegenResult;
use crate::trace;
use crate::traversals::Dfs;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use cranelift_entity::{SecondaryMap, packed_option::PackedOption};
use smallvec::SmallVec;

type SmallValueVec = SmallVec<[ir::Value; 8]>;
type SmallBlockArgVec = SmallVec<[ir::BlockArg; 8]>;
type SmallBlockCallVec = SmallVec<[ir::BlockCall; 8]>;

/// A command directing Cranelift whether or not to inline a particular call.
pub enum InlineCommand<'a> {
    /// Keep the call as-is, out-of-line, and do not inline the callee.
    KeepCall,
    /// Inline the call, using this function as the body of the callee.
    ///
    /// It is the `Inline` implementor's responsibility to ensure that this
    /// function is the correct callee. Providing the wrong function may result
    /// in panics during compilation or incorrect runtime behavior.
    Inline(Cow<'a, ir::Function>),
}

/// A trait for directing Cranelift whether to inline a particular call or not.
///
/// Used in combination with the [`Context::inline`][crate::Context::inline]
/// method.
pub trait Inline {
    /// A hook invoked for each direct call instruction in a function, whose
    /// result determines whether Cranelift should inline a given call.
    ///
    /// The Cranelift user is responsible for defining their own hueristics and
    /// deciding whether inlining the call is beneficial.
    ///
    /// When returning a function and directing Cranelift to inline its body
    /// into the call site, the `Inline` implementer must ensure the following:
    ///
    /// * The returned function's signature exactly matches the `callee`
    ///   `FuncRef`'s signature.
    ///
    /// * The returned function must be legalized.
    ///
    /// * The returned function must be valid (i.e. it must pass the CLIF
    ///   verifier).
    ///
    /// * The returned function is a correct and valid implementation of the
    ///   `callee` according to your language's semantics.
    ///
    /// Failure to uphold these invariants may result in panics during
    /// compilation or incorrect runtime behavior in the generated code.
    fn inline(
        &mut self,
        caller: &ir::Function,
        call_inst: ir::Inst,
        call_opcode: ir::Opcode,
        callee: ir::FuncRef,
        call_args: &[ir::Value],
    ) -> InlineCommand<'_>;
}

impl<'a, T> Inline for &'a mut T
where
    T: Inline,
{
    fn inline(
        &mut self,
        caller: &ir::Function,
        inst: ir::Inst,
        opcode: ir::Opcode,
        callee: ir::FuncRef,
        args: &[ir::Value],
    ) -> InlineCommand<'_> {
        (*self).inline(caller, inst, opcode, callee, args)
    }
}

/// Walk the given function, invoke the `Inline` implementation for each call
/// instruction, and inline the callee when directed to do so.
///
/// Returns whether any call was inlined.
pub(crate) fn do_inlining(
    func: &mut ir::Function,
    mut inliner: impl Inline,
) -> CodegenResult<bool> {
    trace!("function {} before inlining: {}", func.name, func);

    let mut inlined_any = false;
    let mut allocs = InliningAllocs::default();

    let mut cursor = FuncCursor::new(func);
    while let Some(block) = cursor.next_block() {
        // Always keep track of our previous cursor position. After we inline a
        // call, replacing the current position with an arbitrary sub-CFG, we
        // back up to this previous position. This makes sure our cursor is
        // always at a position that is inserted in the layout and also enables
        // multi-level inlining, if desired by the user, where we consider any
        // newly-inlined call instructions for further inlining.
        let mut prev_pos;

        while let Some(inst) = {
            prev_pos = cursor.position();
            cursor.next_inst()
        } {
            match cursor.func.dfg.insts[inst] {
                ir::InstructionData::Call {
                    opcode: opcode @ ir::Opcode::Call | opcode @ ir::Opcode::ReturnCall,
                    args: _,
                    func_ref,
                } => {
                    let args = cursor.func.dfg.inst_args(inst);
                    match inliner.inline(&cursor.func, inst, opcode, func_ref, args) {
                        InlineCommand::KeepCall => continue,
                        InlineCommand::Inline(callee) => {
                            inline_one(
                                &mut allocs,
                                cursor.func,
                                func_ref,
                                block,
                                inst,
                                opcode,
                                &callee,
                                None,
                            );
                            inlined_any = true;
                            cursor.set_position(prev_pos);
                        }
                    }
                }
                ir::InstructionData::TryCall {
                    opcode: opcode @ ir::Opcode::TryCall,
                    args: _,
                    func_ref,
                    exception,
                } => {
                    let args = cursor.func.dfg.inst_args(inst);
                    match inliner.inline(&cursor.func, inst, opcode, func_ref, args) {
                        InlineCommand::KeepCall => continue,
                        InlineCommand::Inline(callee) => {
                            inline_one(
                                &mut allocs,
                                cursor.func,
                                func_ref,
                                block,
                                inst,
                                opcode,
                                &callee,
                                Some(exception),
                            );
                            inlined_any = true;
                            cursor.set_position(prev_pos);
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    if inlined_any {
        trace!("function {} after inlining: {}", func.name, func);
    } else {
        trace!("function {} did not have any callees inlined", func.name);
    }

    Ok(inlined_any)
}

#[derive(Default)]
struct InliningAllocs {
    /// Map from callee value to inlined caller value.
    values: SecondaryMap<ir::Value, PackedOption<ir::Value>>,

    /// Map from callee constant to inlined caller constant.
    constants: SecondaryMap<ir::Constant, PackedOption<ir::Constant>>,

    /// The set of _caller_ inlined call instructions that need exception table
    /// fixups at the end of inlining.
    ///
    /// This includes all kinds of non-returning calls, not just the literal
    /// `call` instruction: `call_indirect`, `try_call`, `try_call_indirect`,
    /// etc... However, it does not include `return_call` and
    /// `return_call_indirect` instructions because the caller cannot catch
    /// exceptions that those calls throw because the caller is no longer on the
    /// stack as soon as they are executed.
    ///
    /// Note: this is a simple `Vec`, and not an `EntitySet`, because it is very
    /// sparse: most of the caller's instructions are not inlined call
    /// instructions. Additionally, we require deterministic iteration order and
    /// do not require set-membership testing, so a hash set is not a good
    /// choice either.
    calls_needing_exception_table_fixup: Vec<ir::Inst>,
}

impl InliningAllocs {
    fn reset(&mut self, callee: &ir::Function) {
        let InliningAllocs {
            values,
            constants,
            calls_needing_exception_table_fixup,
        } = self;

        values.clear();
        values.resize(callee.dfg.len_values());

        constants.clear();
        constants.resize(callee.dfg.constants.len());

        // Note: We do not reserve capacity for
        // `calls_needing_exception_table_fixup` because it is a sparse set and
        // we don't know how large it needs to be ahead of time.
        calls_needing_exception_table_fixup.clear();
    }

    fn set_inlined_value(
        &mut self,
        callee: &ir::Function,
        callee_val: ir::Value,
        inlined_val: ir::Value,
    ) {
        trace!("  --> callee {callee_val:?} = inlined {inlined_val:?}");
        debug_assert!(self.values[callee_val].is_none());
        let resolved_callee_val = callee.dfg.resolve_aliases(callee_val);
        debug_assert!(self.values[resolved_callee_val].is_none());
        self.values[resolved_callee_val] = Some(inlined_val).into();
    }

    fn get_inlined_value(&self, callee: &ir::Function, callee_val: ir::Value) -> Option<ir::Value> {
        let resolved_callee_val = callee.dfg.resolve_aliases(callee_val);
        self.values[resolved_callee_val].expand()
    }
}

/// Inline one particular function call.
fn inline_one(
    allocs: &mut InliningAllocs,
    func: &mut ir::Function,
    callee_func_ref: ir::FuncRef,
    call_block: ir::Block,
    call_inst: ir::Inst,
    call_opcode: ir::Opcode,
    callee: &ir::Function,
    call_exception_table: Option<ir::ExceptionTable>,
) {
    trace!(
        "Inlining call {call_inst:?}: {}\n\
         with callee = {callee:?}",
        func.dfg.display_inst(call_inst)
    );

    // Type check callee signature.
    let expected_callee_sig = func.dfg.ext_funcs[callee_func_ref].signature;
    let expected_callee_sig = &func.dfg.signatures[expected_callee_sig];
    assert_eq!(expected_callee_sig, &callee.signature);

    allocs.reset(callee);

    // First, append various callee entity arenas to the end of the caller's
    // entity arenas.
    let entity_map = create_entities(allocs, func, callee);

    // Inlined prologue: split the call instruction's block at the point of the
    // call and replace the call with a jump.
    let return_block = split_off_return_block(func, call_inst, call_opcode, callee);
    let call_stack_map = replace_call_with_jump(allocs, func, call_inst, callee, &entity_map);

    // Prepare for translating the actual instructions by inserting the inlined
    // blocks into the caller's layout in the same order that they appear in the
    // callee.
    inline_block_layout(func, call_block, callee, &entity_map);

    // Translate each instruction from the callee into the caller,
    // appending them to their associated block in the caller.
    //
    // Note that we iterate over the callee with a pre-order traversal so that
    // we see value defs before uses.
    for callee_block in Dfs::new().pre_order_iter(callee) {
        let inlined_block = entity_map.inlined_block(callee_block);
        trace!(
            "Processing instructions in callee block {callee_block:?} (inlined block {inlined_block:?}"
        );

        let mut next_callee_inst = callee.layout.first_inst(callee_block);
        while let Some(callee_inst) = next_callee_inst {
            trace!(
                "Processing callee instruction {callee_inst:?}: {}",
                callee.dfg.display_inst(callee_inst)
            );

            assert_ne!(
                callee.dfg.insts[callee_inst].opcode(),
                ir::Opcode::GlobalValue,
                "callee must already be legalized, we shouldn't see any `global_value` \
                 instructions when inlining; found {callee_inst:?}: {}",
                callee.dfg.display_inst(callee_inst)
            );

            // Remap the callee instruction's entities and insert it into the
            // caller's DFG.
            let inlined_inst_data = callee.dfg.insts[callee_inst].map(InliningInstRemapper {
                allocs: &allocs,
                func,
                callee,
                entity_map: &entity_map,
            });
            let inlined_inst = func.dfg.make_inst(inlined_inst_data);
            func.layout.append_inst(inlined_inst, inlined_block);

            let opcode = callee.dfg.insts[callee_inst].opcode();
            if opcode.is_return() {
                // Instructions that return do not define any values, so we
                // don't need to worry about that, but we do need to fix them up
                // so that they return by jumping to our control-flow join
                // block, rather than returning from the caller.
                if let Some(return_block) = return_block {
                    fixup_inst_that_returns(
                        allocs,
                        func,
                        callee,
                        &entity_map,
                        call_opcode,
                        inlined_inst,
                        callee_inst,
                        return_block,
                        call_stack_map.as_ref().map(|es| &**es),
                    );
                } else {
                    // If we are inlining a callee that was invoked via
                    // `return_call`, we leave inlined return instructions
                    // as-is: there is no logical caller frame on the stack to
                    // continue to.
                    debug_assert_eq!(call_opcode, ir::Opcode::ReturnCall);
                }
            } else {
                // Make the instruction's result values.
                let ctrl_typevar = callee.dfg.ctrl_typevar(callee_inst);
                func.dfg.make_inst_results(inlined_inst, ctrl_typevar);

                // Update the value map for this instruction's defs.
                let callee_results = callee.dfg.inst_results(callee_inst);
                let inlined_results = func.dfg.inst_results(inlined_inst);
                debug_assert_eq!(callee_results.len(), inlined_results.len());
                for (callee_val, inlined_val) in callee_results.iter().zip(inlined_results) {
                    allocs.set_inlined_value(callee, *callee_val, *inlined_val);
                }

                if opcode.is_call() {
                    append_stack_map_entries(
                        func,
                        callee,
                        &entity_map,
                        call_stack_map.as_deref(),
                        inlined_inst,
                        callee_inst,
                    );

                    // When we are inlining a `try_call` call site, we need to merge
                    // the call site's exception table into the inlined calls'
                    // exception tables. This can involve rewriting regular `call`s
                    // into `try_call`s, which requires mutating the CFG because
                    // `try_call` is a block terminator. However, we can't mutate
                    // the CFG in the middle of this traversal because we rely on
                    // the existence of a one-to-one mapping between the callee
                    // layout and the inlined layout. Instead, we record the set of
                    // inlined call instructions that will need fixing up, and
                    // perform that possibly-CFG-mutating exception table merging in
                    // a follow up pass, when we no longer rely on that one-to-one
                    // layout mapping.
                    debug_assert_eq!(
                        call_opcode == ir::Opcode::TryCall,
                        call_exception_table.is_some()
                    );
                    if call_opcode == ir::Opcode::TryCall {
                        allocs
                            .calls_needing_exception_table_fixup
                            .push(inlined_inst);
                    }
                }
            }

            trace!(
                "  --> inserted inlined instruction {inlined_inst:?}: {}",
                func.dfg.display_inst(inlined_inst)
            );

            next_callee_inst = callee.layout.next_inst(callee_inst);
        }
    }

    // We copied *all* callee blocks into the caller's layout, but only copied
    // the callee instructions in *reachable* callee blocks into the caller's
    // associated blocks. Therefore, any *unreachable* blocks are empty in the
    // caller, which is invalid CLIF because all blocks must end in a
    // terminator, so do a quick pass over the inlined blocks and remove any
    // empty blocks from the caller's layout.
    for block in entity_map.iter_inlined_blocks(func) {
        if func.layout.first_inst(block).is_none() {
            func.layout.remove_block(block);
        }
    }

    // Final step: fixup the exception tables of any inlined calls when we are
    // inlining a `try_call` site.
    //
    // Subtly, this requires rewriting non-catching `call[_indirect]`
    // instructions into `try_call[_indirect]` instructions so that exceptions
    // that unwound through the original callee frame and were caught by the
    // caller's `try_call` do not unwind past this inlined frame. And turning a
    // `call` into a `try_call` mutates the CFG, breaking our one-to-one mapping
    // between callee blocks and inlined blocks, so we delay these fixups to
    // this final step, when we no longer rely on that mapping.
    debug_assert!(
        allocs.calls_needing_exception_table_fixup.is_empty() || call_exception_table.is_some()
    );
    debug_assert_eq!(
        call_opcode == ir::Opcode::TryCall,
        call_exception_table.is_some()
    );
    if let Some(call_exception_table) = call_exception_table {
        fixup_inlined_call_exception_tables(allocs, func, call_exception_table);
    }
}

/// Append stack map entries from the caller and callee to the given inlined
/// instruction.
fn append_stack_map_entries(
    func: &mut ir::Function,
    callee: &ir::Function,
    entity_map: &EntityMap,
    call_stack_map: Option<&[ir::UserStackMapEntry]>,
    inlined_inst: ir::Inst,
    callee_inst: ir::Inst,
) {
    // Add the caller's stack map to this call. These entries
    // already refer to caller entities and do not need further
    // translation.
    func.dfg.append_user_stack_map_entries(
        inlined_inst,
        call_stack_map
            .iter()
            .flat_map(|entries| entries.iter().cloned()),
    );

    // Append the callee's stack map to this call. These entries
    // refer to callee entities and therefore do require
    // translation into the caller's index space.
    func.dfg.append_user_stack_map_entries(
        inlined_inst,
        callee
            .dfg
            .user_stack_map_entries(callee_inst)
            .iter()
            .flat_map(|entries| entries.iter())
            .map(|entry| ir::UserStackMapEntry {
                ty: entry.ty,
                slot: entity_map.inlined_stack_slot(entry.slot),
                offset: entry.offset,
            }),
    );
}

/// Create or update the exception tables for any inlined call instructions:
/// when inlining at a `try_call` site, we must forward our exceptional edges
/// into each inlined call instruction.
fn fixup_inlined_call_exception_tables(
    allocs: &mut InliningAllocs,
    func: &mut ir::Function,
    call_exception_table: ir::ExceptionTable,
) {
    // Split a block at a `call[_indirect]` instruction, detach the
    // instruction's results, and alias them to the new block's parameters.
    let split_block_for_new_try_call = |func: &mut ir::Function, inst: ir::Inst| -> ir::Block {
        debug_assert!(func.dfg.insts[inst].opcode().is_call());
        debug_assert!(!func.dfg.insts[inst].opcode().is_terminator());

        // Split the block.
        let next_inst = func
            .layout
            .next_inst(inst)
            .expect("inst is not a terminator, should have a successor");
        let new_block = func.dfg.blocks.add();
        func.layout.split_block(new_block, next_inst);

        // `try_call[_indirect]` instructions do not define values themselves;
        // the normal-return block has parameters for the results. So remove
        // this instruction's results, create an associated block parameter for
        // each of them, and alias them to the new block parameter.
        let old_results = SmallValueVec::from_iter(func.dfg.inst_results(inst).iter().copied());
        func.dfg.detach_inst_results(inst);
        for old_result in old_results {
            let ty = func.dfg.value_type(old_result);
            let new_block_param = func.dfg.append_block_param(new_block, ty);
            func.dfg.change_to_alias(old_result, new_block_param);
        }

        new_block
    };

    // Clone the caller's exception table, updating it for use in the current
    // `call[_indirect]` instruction as it becomes a `try_call[_indirect]`.
    let clone_exception_table_for_this_call = |func: &mut ir::Function,
                                               signature: ir::SigRef,
                                               new_block: ir::Block|
     -> ir::ExceptionTable {
        let mut exception = func.stencil.dfg.exception_tables[call_exception_table]
            .deep_clone(&mut func.stencil.dfg.value_lists);

        *exception.signature_mut() = signature;

        let returns_len = func.dfg.signatures[signature].returns.len();
        let returns_len = u32::try_from(returns_len).unwrap();

        *exception.normal_return_mut() = ir::BlockCall::new(
            new_block,
            (0..returns_len).map(|i| ir::BlockArg::TryCallRet(i)),
            &mut func.dfg.value_lists,
        );

        func.dfg.exception_tables.push(exception)
    };

    for inst in allocs.calls_needing_exception_table_fixup.drain(..) {
        debug_assert!(func.dfg.insts[inst].opcode().is_call());
        debug_assert!(!func.dfg.insts[inst].opcode().is_return());
        match func.dfg.insts[inst] {
            //     current_block:
            //         preds...
            //         rets... = call f(args...)
            //         succs...
            //
            // becomes
            //
            //     current_block:
            //         preds...
            //         try_call f(args...), new_block(rets...), [call_exception_table...]
            //     new_block(rets...):
            //         succs...
            ir::InstructionData::Call {
                opcode: ir::Opcode::Call,
                args,
                func_ref,
            } => {
                let new_block = split_block_for_new_try_call(func, inst);
                let signature = func.dfg.ext_funcs[func_ref].signature;
                let exception = clone_exception_table_for_this_call(func, signature, new_block);
                func.dfg.insts[inst] = ir::InstructionData::TryCall {
                    opcode: ir::Opcode::TryCall,
                    args,
                    func_ref,
                    exception,
                };
            }

            //     current_block:
            //         preds...
            //         rets... = call_indirect sig, val(args...)
            //         succs...
            //
            // becomes
            //
            //     current_block:
            //         preds...
            //         try_call_indirect sig, val(args...), new_block(rets...), [call_exception_table...]
            //     new_block(rets...):
            //         succs...
            ir::InstructionData::CallIndirect {
                opcode: ir::Opcode::CallIndirect,
                args,
                sig_ref,
            } => {
                let new_block = split_block_for_new_try_call(func, inst);
                let exception = clone_exception_table_for_this_call(func, sig_ref, new_block);
                func.dfg.insts[inst] = ir::InstructionData::TryCallIndirect {
                    opcode: ir::Opcode::TryCallIndirect,
                    args,
                    exception,
                };
            }

            // For `try_call[_indirect]` instructions, we just need to merge the
            // exception tables.
            ir::InstructionData::TryCall {
                opcode: ir::Opcode::TryCall,
                exception,
                ..
            }
            | ir::InstructionData::TryCallIndirect {
                opcode: ir::Opcode::TryCallIndirect,
                exception,
                ..
            } => {
                // Construct a new exception table that consists of
                // the inlined instruction's exception table match
                // sequence, with the inlining site's exception table
                // appended. This will ensure that the first-match
                // semantics emulates the original behavior of
                // matching in the inner frame first.
                let sig = func.dfg.exception_tables[exception].signature();
                let normal_return = *func.dfg.exception_tables[exception].normal_return();
                let exception_data = ExceptionTableData::new(
                    sig,
                    normal_return,
                    func.dfg.exception_tables[exception]
                        .items()
                        .chain(func.dfg.exception_tables[call_exception_table].items()),
                )
                .deep_clone(&mut func.dfg.value_lists);

                func.dfg.exception_tables[exception] = exception_data;
            }

            otherwise => unreachable!("unknown non-return call instruction: {otherwise:?}"),
        }
    }
}

/// After having created an inlined version of a callee instruction that returns
/// in the caller, we need to fix it up so that it doesn't actually return
/// (since we are already in the caller's frame) and instead just jumps to the
/// control-flow join point.
fn fixup_inst_that_returns(
    allocs: &mut InliningAllocs,
    func: &mut ir::Function,
    callee: &ir::Function,
    entity_map: &EntityMap,
    call_opcode: ir::Opcode,
    inlined_inst: ir::Inst,
    callee_inst: ir::Inst,
    return_block: ir::Block,
    call_stack_map: Option<&[ir::UserStackMapEntry]>,
) {
    debug_assert!(func.dfg.insts[inlined_inst].opcode().is_return());
    match func.dfg.insts[inlined_inst] {
        //     return rets...
        //
        // becomes
        //
        //     jump return_block(rets...)
        ir::InstructionData::MultiAry {
            opcode: ir::Opcode::Return,
            args,
        } => {
            let rets = SmallBlockArgVec::from_iter(
                args.as_slice(&func.dfg.value_lists)
                    .iter()
                    .copied()
                    .map(|v| v.into()),
            );
            func.dfg.replace(inlined_inst).jump(return_block, &rets);
        }

        //     return_call f(args...)
        //
        // becomes
        //
        //     rets... = call f(args...)
        //     jump return_block(rets...)
        ir::InstructionData::Call {
            opcode: ir::Opcode::ReturnCall,
            args,
            func_ref,
        } => {
            func.dfg.insts[inlined_inst] = ir::InstructionData::Call {
                opcode: ir::Opcode::Call,
                args,
                func_ref,
            };
            func.dfg.make_inst_results(inlined_inst, ir::types::INVALID);

            append_stack_map_entries(
                func,
                callee,
                &entity_map,
                call_stack_map,
                inlined_inst,
                callee_inst,
            );

            let rets = SmallBlockArgVec::from_iter(
                func.dfg
                    .inst_results(inlined_inst)
                    .iter()
                    .copied()
                    .map(|v| v.into()),
            );
            let mut cursor = FuncCursor::new(func);
            cursor.goto_after_inst(inlined_inst);
            cursor.ins().jump(return_block, &rets);

            if call_opcode == ir::Opcode::TryCall {
                allocs
                    .calls_needing_exception_table_fixup
                    .push(inlined_inst);
            }
        }

        //     return_call_indirect val(args...)
        //
        // becomes
        //
        //     rets... = call_indirect val(args...)
        //     jump return_block(rets...)
        ir::InstructionData::CallIndirect {
            opcode: ir::Opcode::ReturnCallIndirect,
            args,
            sig_ref,
        } => {
            func.dfg.insts[inlined_inst] = ir::InstructionData::CallIndirect {
                opcode: ir::Opcode::CallIndirect,
                args,
                sig_ref,
            };
            func.dfg.make_inst_results(inlined_inst, ir::types::INVALID);

            append_stack_map_entries(
                func,
                callee,
                &entity_map,
                call_stack_map,
                inlined_inst,
                callee_inst,
            );

            let rets = SmallBlockArgVec::from_iter(
                func.dfg
                    .inst_results(inlined_inst)
                    .iter()
                    .copied()
                    .map(|v| v.into()),
            );
            let mut cursor = FuncCursor::new(func);
            cursor.goto_after_inst(inlined_inst);
            cursor.ins().jump(return_block, &rets);

            if call_opcode == ir::Opcode::TryCall {
                allocs
                    .calls_needing_exception_table_fixup
                    .push(inlined_inst);
            }
        }

        inst_data => unreachable!(
            "should have handled all `is_return() == true` instructions above; \
             got {inst_data:?}"
        ),
    }
}

/// An `InstructionMapper` implementation that remaps a callee instruction's
/// entity references to their new indices in the caller function.
struct InliningInstRemapper<'a> {
    allocs: &'a InliningAllocs,
    func: &'a mut ir::Function,
    callee: &'a ir::Function,
    entity_map: &'a EntityMap,
}

impl<'a> ir::instructions::InstructionMapper for InliningInstRemapper<'a> {
    fn map_value(&mut self, value: ir::Value) -> ir::Value {
        self.allocs.get_inlined_value(self.callee, value).expect(
            "defs come before uses; we should have already inlined all values \
             used by an instruction",
        )
    }

    fn map_value_list(&mut self, value_list: ir::ValueList) -> ir::ValueList {
        let mut inlined_list = ir::ValueList::new();
        for callee_val in value_list.as_slice(&self.callee.dfg.value_lists) {
            let inlined_val = self.map_value(*callee_val);
            inlined_list.push(inlined_val, &mut self.func.dfg.value_lists);
        }
        inlined_list
    }

    fn map_global_value(&mut self, global_value: ir::GlobalValue) -> ir::GlobalValue {
        self.entity_map.inlined_global_value(global_value)
    }

    fn map_jump_table(&mut self, jump_table: ir::JumpTable) -> ir::JumpTable {
        let inlined_default =
            self.map_block_call(self.callee.dfg.jump_tables[jump_table].default_block());
        let inlined_table = self.callee.dfg.jump_tables[jump_table]
            .as_slice()
            .iter()
            .map(|callee_block_call| self.map_block_call(*callee_block_call))
            .collect::<SmallBlockCallVec>();
        self.func
            .dfg
            .jump_tables
            .push(ir::JumpTableData::new(inlined_default, &inlined_table))
    }

    fn map_exception_table(&mut self, exception_table: ir::ExceptionTable) -> ir::ExceptionTable {
        let exception_table = &self.callee.dfg.exception_tables[exception_table];
        let inlined_sig_ref = self.map_sig_ref(exception_table.signature());
        let inlined_normal_return = self.map_block_call(*exception_table.normal_return());
        let inlined_table = exception_table
            .items()
            .map(|item| match item {
                ExceptionTableItem::Tag(tag, block_call) => {
                    ExceptionTableItem::Tag(tag, self.map_block_call(block_call))
                }
                ExceptionTableItem::Default(block_call) => {
                    ExceptionTableItem::Default(self.map_block_call(block_call))
                }
                ExceptionTableItem::Context(value) => {
                    ExceptionTableItem::Context(self.map_value(value))
                }
            })
            .collect::<SmallVec<[_; 8]>>();
        self.func
            .dfg
            .exception_tables
            .push(ir::ExceptionTableData::new(
                inlined_sig_ref,
                inlined_normal_return,
                inlined_table,
            ))
    }

    fn map_block_call(&mut self, block_call: ir::BlockCall) -> ir::BlockCall {
        let callee_block = block_call.block(&self.callee.dfg.value_lists);
        let inlined_block = self.entity_map.inlined_block(callee_block);
        let args = block_call
            .args(&self.callee.dfg.value_lists)
            .map(|arg| match arg {
                ir::BlockArg::Value(value) => self.map_value(value).into(),
                ir::BlockArg::TryCallRet(_) | ir::BlockArg::TryCallExn(_) => arg,
            })
            .collect::<SmallBlockArgVec>();
        ir::BlockCall::new(inlined_block, args, &mut self.func.dfg.value_lists)
    }

    fn map_func_ref(&mut self, func_ref: ir::FuncRef) -> ir::FuncRef {
        self.entity_map.inlined_func_ref(func_ref)
    }

    fn map_sig_ref(&mut self, sig_ref: ir::SigRef) -> ir::SigRef {
        self.entity_map.inlined_sig_ref(sig_ref)
    }

    fn map_stack_slot(&mut self, stack_slot: ir::StackSlot) -> ir::StackSlot {
        self.entity_map.inlined_stack_slot(stack_slot)
    }

    fn map_dynamic_stack_slot(
        &mut self,
        dynamic_stack_slot: ir::DynamicStackSlot,
    ) -> ir::DynamicStackSlot {
        self.entity_map
            .inlined_dynamic_stack_slot(dynamic_stack_slot)
    }

    fn map_constant(&mut self, constant: ir::Constant) -> ir::Constant {
        self.allocs
            .constants
            .get(constant)
            .and_then(|o| o.expand())
            .expect("should have inlined all callee constants")
    }

    fn map_immediate(&mut self, immediate: ir::Immediate) -> ir::Immediate {
        self.entity_map.inlined_immediate(immediate)
    }
}

/// Inline the callee's layout into the caller's layout.
fn inline_block_layout(
    func: &mut ir::Function,
    call_block: ir::Block,
    callee: &ir::Function,
    entity_map: &EntityMap,
) {
    // Iterate over callee blocks in layout order, inserting their associated
    // inlined block into the caller's layout.
    let mut prev_inlined_block = call_block;
    let mut next_callee_block = callee.layout.entry_block();
    while let Some(callee_block) = next_callee_block {
        let inlined_block = entity_map.inlined_block(callee_block);
        func.layout
            .insert_block_after(inlined_block, prev_inlined_block);

        prev_inlined_block = inlined_block;
        next_callee_block = callee.layout.next_block(callee_block);
    }
}

/// Split the call instruction's block just after the call instruction to create
/// the point where control-flow joins after the inlined callee "returns".
///
/// Note that tail calls do not return to the caller and therefore do not have a
/// control-flow join point.
fn split_off_return_block(
    func: &mut ir::Function,
    call_inst: ir::Inst,
    opcode: ir::Opcode,
    callee: &ir::Function,
) -> Option<ir::Block> {
    // When the `call_inst` is not a block terminator, we need to split the
    // block.
    let return_block = func.layout.next_inst(call_inst).map(|next_inst| {
        let return_block = func.dfg.blocks.add();
        func.layout.split_block(return_block, next_inst);

        // Add block parameters for each return value and alias the call
        // instruction's results to them.
        let old_results =
            SmallValueVec::from_iter(func.dfg.inst_results(call_inst).iter().copied());
        debug_assert_eq!(old_results.len(), callee.signature.returns.len());
        func.dfg.detach_inst_results(call_inst);
        for (abi, old_val) in callee.signature.returns.iter().zip(old_results) {
            debug_assert_eq!(abi.value_type, func.dfg.value_type(old_val));
            let ret_param = func.dfg.append_block_param(return_block, abi.value_type);
            func.dfg.change_to_alias(old_val, ret_param);
        }

        return_block
    });

    // When the `call_inst` is a block terminator, then it is either a
    // `return_call` or a `try_call`:
    //
    // * For `return_call`s, we don't have a control-flow join point, because
    //   the caller permanently transfers control to the callee.
    //
    // * For `try_call`s, we probably already have a block for the control-flow
    //   join point, but it isn't guaranteed: the `try_call` might ignore the
    //   call's returns and not forward them to the normal-return block or it
    //   might also pass additional arguments. We can only reuse the existing
    //   normal-return block when the `try_call` forwards exactly our callee's
    //   returns to that block (and therefore that block's parameter types also
    //   exactly match the callee's return types). Otherwise, we must create a new
    //   return block that forwards to the existing normal-return
    //   block. (Elsewhere, at the end of inlining, we will also update any inlined
    //   calls to forward any raised exceptions to the caller's exception table,
    //   as necessary.)
    //
    //   Finally, note that reusing the normal-return's target block is just an
    //   optimization to emit a simpler CFG when we can, and is not
    //   fundamentally required for correctness. We could always insert a
    //   temporary block as our control-flow join point that then forwards to
    //   the normal-return's target block. However, at the time of writing,
    //   Cranelift doesn't currently do any jump-threading or branch
    //   simplification in the mid-end, and removing unnecessary blocks in this
    //   way can help some subsequent mid-end optimizations. If, in the future,
    //   we gain support for jump-threading optimizations in the mid-end, we can
    //   come back and simplify the below code a bit to always generate the
    //   temporary block, and then rely on the subsequent optimizations to clean
    //   everything up.
    debug_assert_eq!(
        return_block.is_none(),
        opcode == ir::Opcode::ReturnCall || opcode == ir::Opcode::TryCall,
    );
    return_block.or_else(|| match func.dfg.insts[call_inst] {
        ir::InstructionData::TryCall {
            opcode: ir::Opcode::TryCall,
            args: _,
            func_ref: _,
            exception,
        } => {
            let normal_return = func.dfg.exception_tables[exception].normal_return();
            let normal_return_block = normal_return.block(&func.dfg.value_lists);

            // Check to see if we can reuse the existing normal-return block.
            {
                let normal_return_args = normal_return.args(&func.dfg.value_lists);
                if normal_return_args.len() == callee.signature.returns.len()
                    && normal_return_args.enumerate().all(|(i, arg)| {
                        let i = u32::try_from(i).unwrap();
                        arg == ir::BlockArg::TryCallRet(i)
                    })
                {
                    return Some(normal_return_block);
                }
            }

            // Okay, we cannot reuse the normal-return block. Create a new block
            // that has the expected block parameter types and have it jump to
            // the normal-return block.
            let return_block = func.dfg.blocks.add();
            func.layout.insert_block(return_block, normal_return_block);

            let return_block_params = callee
                .signature
                .returns
                .iter()
                .map(|abi| func.dfg.append_block_param(return_block, abi.value_type))
                .collect::<SmallValueVec>();

            let normal_return_args = func.dfg.exception_tables[exception]
                .normal_return()
                .args(&func.dfg.value_lists)
                .collect::<SmallBlockArgVec>();
            let jump_args = normal_return_args
                .into_iter()
                .map(|arg| match arg {
                    ir::BlockArg::Value(value) => ir::BlockArg::Value(value),
                    ir::BlockArg::TryCallRet(i) => {
                        let i = usize::try_from(i).unwrap();
                        ir::BlockArg::Value(return_block_params[i])
                    }
                    ir::BlockArg::TryCallExn(_) => {
                        unreachable!("normal-return edges cannot use exceptional results")
                    }
                })
                .collect::<SmallBlockArgVec>();

            let mut cursor = FuncCursor::new(func);
            cursor.goto_first_insertion_point(return_block);
            cursor.ins().jump(normal_return_block, &jump_args);

            Some(return_block)
        }
        _ => None,
    })
}

/// Replace the caller's call instruction with a jump to the caller's inlined
/// copy of the callee's entry block.
///
/// Also associates the callee's parameters with the caller's arguments in our
/// value map.
///
/// Returns the caller's stack map entries, if any.
fn replace_call_with_jump(
    allocs: &mut InliningAllocs,
    func: &mut ir::Function,
    call_inst: ir::Inst,
    callee: &ir::Function,
    entity_map: &EntityMap,
) -> Option<ir::UserStackMapEntryVec> {
    trace!("Replacing `call` with `jump`");
    trace!(
        "  --> call instruction: {call_inst:?}: {}",
        func.dfg.display_inst(call_inst)
    );

    let callee_entry_block = callee
        .layout
        .entry_block()
        .expect("callee function should have an entry block");
    let callee_param_values = callee.dfg.block_params(callee_entry_block);
    let caller_arg_values = SmallValueVec::from_iter(func.dfg.inst_args(call_inst).iter().copied());
    debug_assert_eq!(callee_param_values.len(), caller_arg_values.len());
    debug_assert_eq!(callee_param_values.len(), callee.signature.params.len());
    for (abi, (callee_param_value, caller_arg_value)) in callee
        .signature
        .params
        .iter()
        .zip(callee_param_values.into_iter().zip(caller_arg_values))
    {
        debug_assert_eq!(abi.value_type, callee.dfg.value_type(*callee_param_value));
        debug_assert_eq!(abi.value_type, func.dfg.value_type(caller_arg_value));
        allocs.set_inlined_value(callee, *callee_param_value, caller_arg_value);
    }

    // Replace the caller's call instruction with a jump to the caller's inlined
    // copy of the callee's entry block.
    //
    // Note that the call block dominates the inlined entry block (and also all
    // other inlined blocks) so we can reference the arguments directly, and do
    // not need to add block parameters to the inlined entry block.
    let inlined_entry_block = entity_map.inlined_block(callee_entry_block);
    func.dfg.replace(call_inst).jump(inlined_entry_block, &[]);
    trace!(
        "  --> replaced with jump instruction: {call_inst:?}: {}",
        func.dfg.display_inst(call_inst)
    );

    let stack_map_entries = func.dfg.take_user_stack_map_entries(call_inst);
    stack_map_entries
}

/// Keeps track of mapping callee entities to their associated inlined caller
/// entities.
#[derive(Default)]
struct EntityMap {
    // Rather than doing an implicit, demand-based, DCE'ing translation of
    // entities, which would require maps from each callee entity to its
    // associated caller entity, we copy all entities into the caller, remember
    // each entity's initial offset, and then mapping from the callee to the
    // inlined caller entity is just adding that initial offset to the callee's
    // index. This should be both faster and simpler than the alternative. Most
    // of these sets are relatively small, and they rarely have too much dead
    // code in practice, so this is a good trade off.
    //
    // Note that there are a few kinds of entities that are excluded from the
    // `EntityMap`, and for which we do actually take the demand-based approach:
    // values and value lists being the notable ones.
    block_offset: Option<u32>,
    global_value_offset: Option<u32>,
    sig_ref_offset: Option<u32>,
    func_ref_offset: Option<u32>,
    stack_slot_offset: Option<u32>,
    dynamic_type_offset: Option<u32>,
    dynamic_stack_slot_offset: Option<u32>,
    immediate_offset: Option<u32>,
}

impl EntityMap {
    fn inlined_block(&self, callee_block: ir::Block) -> ir::Block {
        let offset = self
            .block_offset
            .expect("must create inlined `ir::Block`s before calling `EntityMap::inlined_block`");
        ir::Block::from_u32(offset + callee_block.as_u32())
    }

    fn iter_inlined_blocks(&self, func: &ir::Function) -> impl Iterator<Item = ir::Block> + use<> {
        let start = self.block_offset.expect(
            "must create inlined `ir::Block`s before calling `EntityMap::iter_inlined_blocks`",
        );

        let end = func.dfg.blocks.len();
        let end = u32::try_from(end).unwrap();

        (start..end).map(|i| ir::Block::from_u32(i))
    }

    fn inlined_global_value(&self, callee_global_value: ir::GlobalValue) -> ir::GlobalValue {
        let offset = self
            .global_value_offset
            .expect("must create inlined `ir::GlobalValue`s before calling `EntityMap::inlined_global_value`");
        ir::GlobalValue::from_u32(offset + callee_global_value.as_u32())
    }

    fn inlined_sig_ref(&self, callee_sig_ref: ir::SigRef) -> ir::SigRef {
        let offset = self.sig_ref_offset.expect(
            "must create inlined `ir::SigRef`s before calling `EntityMap::inlined_sig_ref`",
        );
        ir::SigRef::from_u32(offset + callee_sig_ref.as_u32())
    }

    fn inlined_func_ref(&self, callee_func_ref: ir::FuncRef) -> ir::FuncRef {
        let offset = self.func_ref_offset.expect(
            "must create inlined `ir::FuncRef`s before calling `EntityMap::inlined_func_ref`",
        );
        ir::FuncRef::from_u32(offset + callee_func_ref.as_u32())
    }

    fn inlined_stack_slot(&self, callee_stack_slot: ir::StackSlot) -> ir::StackSlot {
        let offset = self.stack_slot_offset.expect(
            "must create inlined `ir::StackSlot`s before calling `EntityMap::inlined_stack_slot`",
        );
        ir::StackSlot::from_u32(offset + callee_stack_slot.as_u32())
    }

    fn inlined_dynamic_type(&self, callee_dynamic_type: ir::DynamicType) -> ir::DynamicType {
        let offset = self.dynamic_type_offset.expect(
            "must create inlined `ir::DynamicType`s before calling `EntityMap::inlined_dynamic_type`",
        );
        ir::DynamicType::from_u32(offset + callee_dynamic_type.as_u32())
    }

    fn inlined_dynamic_stack_slot(
        &self,
        callee_dynamic_stack_slot: ir::DynamicStackSlot,
    ) -> ir::DynamicStackSlot {
        let offset = self.dynamic_stack_slot_offset.expect(
            "must create inlined `ir::DynamicStackSlot`s before calling `EntityMap::inlined_dynamic_stack_slot`",
        );
        ir::DynamicStackSlot::from_u32(offset + callee_dynamic_stack_slot.as_u32())
    }

    fn inlined_immediate(&self, callee_immediate: ir::Immediate) -> ir::Immediate {
        let offset = self.immediate_offset.expect(
            "must create inlined `ir::Immediate`s before calling `EntityMap::inlined_immediate`",
        );
        ir::Immediate::from_u32(offset + callee_immediate.as_u32())
    }
}

/// Translate all of the callee's various entities into the caller, producing an
/// `EntityMap` that can be used to translate callee entity references into
/// inlined caller entity references.
fn create_entities(
    allocs: &mut InliningAllocs,
    func: &mut ir::Function,
    callee: &ir::Function,
) -> EntityMap {
    let mut entity_map = EntityMap::default();

    entity_map.block_offset = Some(create_blocks(allocs, func, callee));
    entity_map.global_value_offset = Some(create_global_values(func, callee));
    entity_map.sig_ref_offset = Some(create_sig_refs(func, callee));
    entity_map.func_ref_offset = Some(create_func_refs(func, callee, &entity_map));
    entity_map.stack_slot_offset = Some(create_stack_slots(func, callee));
    entity_map.dynamic_type_offset = Some(create_dynamic_types(func, callee, &entity_map));
    entity_map.dynamic_stack_slot_offset =
        Some(create_dynamic_stack_slots(func, callee, &entity_map));
    entity_map.immediate_offset = Some(create_immediates(func, callee));

    // `ir::ConstantData` is deduplicated, so we cannot use our offset scheme
    // for `ir::Constant`s. Nonetheless, we still insert them into the caller
    // now, at the same time as the rest of our entities.
    create_constants(allocs, func, callee);

    entity_map
}

/// Create inlined blocks in the caller for every block in the callee.
fn create_blocks(
    allocs: &mut InliningAllocs,
    func: &mut ir::Function,
    callee: &ir::Function,
) -> u32 {
    let offset = func.dfg.blocks.len();
    let offset = u32::try_from(offset).unwrap();

    func.dfg.blocks.reserve(callee.dfg.blocks.len());
    for callee_block in callee.dfg.blocks.iter() {
        let caller_block = func.dfg.blocks.add();
        trace!("Callee {callee_block:?} = inlined {caller_block:?}");

        if callee.layout.is_cold(callee_block) {
            func.layout.set_cold(caller_block);
        }

        // Note: the entry block does not need parameters because the only
        // predecessor is the call block and we associate the callee's
        // parameters with the caller's arguments directly.
        if callee.layout.entry_block() != Some(callee_block) {
            for callee_param in callee.dfg.blocks[callee_block].params(&callee.dfg.value_lists) {
                let ty = callee.dfg.value_type(*callee_param);
                let caller_param = func.dfg.append_block_param(caller_block, ty);

                allocs.set_inlined_value(callee, *callee_param, caller_param);
            }
        }
    }

    offset
}

/// Copy and translate global values from the callee into the caller.
fn create_global_values(func: &mut ir::Function, callee: &ir::Function) -> u32 {
    let gv_offset = func.global_values.len();
    let gv_offset = u32::try_from(gv_offset).unwrap();

    func.global_values.reserve(callee.global_values.len());
    for gv in callee.global_values.values() {
        func.global_values.push(match gv {
            // These kinds of global values reference other global values, so we
            // need to fixup that reference.
            ir::GlobalValueData::Load {
                base,
                offset,
                global_type,
                flags,
            } => ir::GlobalValueData::Load {
                base: ir::GlobalValue::from_u32(base.as_u32() + gv_offset),
                offset: *offset,
                global_type: *global_type,
                flags: *flags,
            },
            ir::GlobalValueData::IAddImm {
                base,
                offset,
                global_type,
            } => ir::GlobalValueData::IAddImm {
                base: ir::GlobalValue::from_u32(base.as_u32() + gv_offset),
                offset: *offset,
                global_type: *global_type,
            },

            // These kinds of global values do not reference other global
            // values, so we can just clone them.
            ir::GlobalValueData::VMContext
            | ir::GlobalValueData::Symbol { .. }
            | ir::GlobalValueData::DynScaleTargetConst { .. } => gv.clone(),
        });
    }

    gv_offset
}

/// Copy `ir::SigRef`s from the callee into the caller.
fn create_sig_refs(func: &mut ir::Function, callee: &ir::Function) -> u32 {
    let offset = func.dfg.signatures.len();
    let offset = u32::try_from(offset).unwrap();

    func.dfg.signatures.reserve(callee.dfg.signatures.len());
    for sig in callee.dfg.signatures.values() {
        func.dfg.signatures.push(sig.clone());
    }

    offset
}

/// Translate `ir::FuncRef`s from the callee into the caller.
fn create_func_refs(func: &mut ir::Function, callee: &ir::Function, entity_map: &EntityMap) -> u32 {
    let offset = func.dfg.ext_funcs.len();
    let offset = u32::try_from(offset).unwrap();

    func.dfg.ext_funcs.reserve(callee.dfg.ext_funcs.len());
    for ir::ExtFuncData {
        name,
        signature,
        colocated,
    } in callee.dfg.ext_funcs.values()
    {
        func.dfg.ext_funcs.push(ir::ExtFuncData {
            name: name.clone(),
            signature: entity_map.inlined_sig_ref(*signature),
            colocated: *colocated,
        });
    }

    offset
}

/// Copy stack slots from the callee into the caller.
fn create_stack_slots(func: &mut ir::Function, callee: &ir::Function) -> u32 {
    let offset = func.sized_stack_slots.len();
    let offset = u32::try_from(offset).unwrap();

    func.sized_stack_slots
        .reserve(callee.sized_stack_slots.len());
    for slot in callee.sized_stack_slots.values() {
        func.sized_stack_slots.push(slot.clone());
    }

    offset
}

/// Copy dynamic types from the callee into the caller.
fn create_dynamic_types(
    func: &mut ir::Function,
    callee: &ir::Function,
    entity_map: &EntityMap,
) -> u32 {
    let offset = func.dynamic_stack_slots.len();
    let offset = u32::try_from(offset).unwrap();

    func.dfg
        .dynamic_types
        .reserve(callee.dfg.dynamic_types.len());
    for ir::DynamicTypeData {
        base_vector_ty,
        dynamic_scale,
    } in callee.dfg.dynamic_types.values()
    {
        func.dfg.dynamic_types.push(ir::DynamicTypeData {
            base_vector_ty: *base_vector_ty,
            dynamic_scale: entity_map.inlined_global_value(*dynamic_scale),
        });
    }

    offset
}

/// Copy dynamic stack slots from the callee into the caller.
fn create_dynamic_stack_slots(
    func: &mut ir::Function,
    callee: &ir::Function,
    entity_map: &EntityMap,
) -> u32 {
    let offset = func.dynamic_stack_slots.len();
    let offset = u32::try_from(offset).unwrap();

    func.dynamic_stack_slots
        .reserve(callee.dynamic_stack_slots.len());
    for ir::DynamicStackSlotData { kind, dyn_ty } in callee.dynamic_stack_slots.values() {
        func.dynamic_stack_slots.push(ir::DynamicStackSlotData {
            kind: *kind,
            dyn_ty: entity_map.inlined_dynamic_type(*dyn_ty),
        });
    }

    offset
}

/// Copy immediates from the callee into the caller.
fn create_immediates(func: &mut ir::Function, callee: &ir::Function) -> u32 {
    let offset = func.dfg.immediates.len();
    let offset = u32::try_from(offset).unwrap();

    func.dfg.immediates.reserve(callee.dfg.immediates.len());
    for imm in callee.dfg.immediates.values() {
        func.dfg.immediates.push(imm.clone());
    }

    offset
}

/// Copy constants from the callee into the caller.
fn create_constants(allocs: &mut InliningAllocs, func: &mut ir::Function, callee: &ir::Function) {
    for (callee_constant, data) in callee.dfg.constants.iter() {
        let inlined_constant = func.dfg.constants.insert(data.clone());
        allocs.constants[*callee_constant] = Some(inlined_constant).into();
    }
}

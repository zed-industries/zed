//! Function call emission.  For more details around the ABI and
//! calling convention, see [ABI].
//!
//! This module exposes a single function [`FnCall::emit`], which is responsible
//! of orchestrating the emission of calls. In general such orchestration
//! takes place in 6 steps:
//!
//! 1. [`Callee`] resolution.
//! 2. Mapping of the [`Callee`] to the [`CalleeKind`].
//! 3. Spilling the value stack.
//! 4. Calculate the return area, for 1+ results.
//! 5. Emission.
//! 6. Stack space cleanup.
//!
//! The stack space consumed by the function call is the amount
//! of space used by any memory entries in the value stack present
//! at the callsite (after spilling the value stack), that will be
//! used as arguments for the function call. Any memory values in the
//! value stack that are needed as part of the function
//! arguments will be consumed by the function call (either by
//! assigning those values to a register or by storing those
//! values in a memory location if the callee argument is on
//! the stack).
//! This could also be done when assigning arguments every time a
//! memory entry needs to be assigned to a particular location,
//! but doing so will emit more instructions (e.g. a pop per
//! argument that needs to be assigned); it's more efficient to
//! calculate the space used by those memory values and reclaim it
//! at once when cleaning up the stack after the call has been
//! emitted.
//!
//! The machine stack throughout the function call is as follows:
//! ┌──────────────────────────────────────────────────┐
//! │                                                  │
//! │  Stack space created by any previous spills      │
//! │  from the value stack; and which memory values   │
//! │  are used as function arguments.                 │
//! │                                                  │
//! ├──────────────────────────────────────────────────┤ ---> The Wasm value stack at this point in time would look like:
//! │                                                  │
//! │   Stack space created by spilling locals and     |
//! │   registers at the callsite.                     │
//! │                                                  │
//! ├─────────────────────────────────────────────────┬┤
//! │                                                  │
//! │   Return Area (Multi-value results)              │
//! │                                                  │
//! │                                                  │
//! ├─────────────────────────────────────────────────┬┤ ---> The Wasm value stack at this point in time would look like:
//! │                                                  │      [ Mem(offset) | Mem(offset) | Mem(offset) | Mem(offset) ]
//! │                                                  │      Assuming that the callee takes 4 arguments, we calculate
//! │                                                  │      4 memory values; all of which will be used as arguments to
//! │   Stack space allocated for                      │      the call via `assign_args`, thus the sum of the size of the
//! │   the callee function arguments in the stack;    │      memory they represent is considered to be consumed by the call.
//! │   represented by `arg_stack_space`               │
//! │                                                  │
//! │                                                  │
//! │                                                  │
//! └──────────────────────────────────────────────────┘ ------> Stack pointer when emitting the call

use crate::{
    FuncEnv,
    abi::{ABIOperand, ABISig, RetArea, vmctx},
    codegen::{BuiltinFunction, BuiltinType, Callee, CodeGenContext, CodeGenError, Emission},
    masm::{
        CalleeKind, ContextArgs, IntScratch, MacroAssembler, MemMoveDirection, OperandSize,
        SPOffset, VMContextLoc,
    },
    reg::{Reg, writable},
    stack::Val,
};
use anyhow::{Result, ensure};
use wasmtime_environ::{FuncIndex, PtrSize, VMOffsets};

/// All the information needed to emit a function call.
#[derive(Copy, Clone)]
pub(crate) struct FnCall {}

impl FnCall {
    /// Orchestrates the emission of a function call:
    /// 1. Resolves the [`Callee`] through the given callback.
    /// 2. Lowers the resolved [`Callee`] to a ([`CalleeKind`], [ContextArgs])
    /// 3. Spills the value stack.
    /// 4. Creates the stack space needed for the return area.
    /// 5. Emits the call.
    /// 6. Cleans up the stack space.
    pub fn emit<M: MacroAssembler>(
        env: &mut FuncEnv<M::Ptr>,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
        callee: Callee,
    ) -> Result<()> {
        let (kind, callee_context) = Self::lower(env, context.vmoffsets, &callee, context, masm)?;

        let sig = env.callee_sig::<M::ABI>(&callee)?;
        context.spill(masm)?;
        let ret_area = Self::make_ret_area(&sig, masm)?;
        let arg_stack_space = sig.params_stack_size();
        let reserved_stack = masm.call(arg_stack_space, |masm| {
            Self::assign(sig, &callee_context, ret_area.as_ref(), context, masm)?;
            Ok((kind, sig.call_conv))
        })?;

        Self::cleanup(
            sig,
            &callee_context,
            &kind,
            reserved_stack,
            ret_area,
            masm,
            context,
        )
    }

    /// Calculates the return area for the callee, if any.
    fn make_ret_area<M: MacroAssembler>(
        callee_sig: &ABISig,
        masm: &mut M,
    ) -> Result<Option<RetArea>> {
        if callee_sig.has_stack_results() {
            let base = masm.sp_offset()?.as_u32();
            let end = base + callee_sig.results_stack_size();
            if end > base {
                masm.reserve_stack(end - base)?;
            }
            Ok(Some(RetArea::sp(SPOffset::from_u32(end))))
        } else {
            Ok(None)
        }
    }

    /// Lowers the high-level [`Callee`] to a [`CalleeKind`] and
    /// [ContextArgs] pair which contains all the metadata needed for
    /// emission.
    fn lower<M: MacroAssembler>(
        env: &mut FuncEnv<M::Ptr>,
        vmoffsets: &VMOffsets<u8>,
        callee: &Callee,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
    ) -> Result<(CalleeKind, ContextArgs)> {
        let ptr = vmoffsets.ptr.size();
        match callee {
            Callee::Builtin(b) => Ok(Self::lower_builtin(env, b, None)),
            Callee::BuiltinWithDifferentVmctx(b, offset) => {
                Ok(Self::lower_builtin(env, b, Some(*offset)))
            }
            Callee::FuncRef(_) => {
                Self::lower_funcref(env.callee_sig::<M::ABI>(callee)?, ptr, context, masm)
            }
            Callee::Local(i) => Ok(Self::lower_local(env, *i)),
            Callee::Import(i) => {
                let sig = env.callee_sig::<M::ABI>(callee)?;
                Self::lower_import(*i, sig, context, masm, vmoffsets)
            }
        }
    }

    /// Lowers a builtin function by loading its address to the next available
    /// register.
    fn lower_builtin<P: PtrSize>(
        env: &mut FuncEnv<P>,
        builtin: &BuiltinFunction,
        vmctx_offset: Option<u32>,
    ) -> (CalleeKind, ContextArgs) {
        match builtin.ty() {
            BuiltinType::Builtin(idx) => (
                CalleeKind::direct(env.name_builtin(idx)),
                match vmctx_offset {
                    Some(offset) => ContextArgs::offset_from_pinned_vmctx(offset),
                    None => ContextArgs::pinned_vmctx(),
                },
            ),
        }
    }

    /// Lower  a local function to a [`CalleeKind`] and [ContextArgs] pair.
    fn lower_local<P: PtrSize>(
        env: &mut FuncEnv<P>,
        index: FuncIndex,
    ) -> (CalleeKind, ContextArgs) {
        (
            CalleeKind::direct(env.name_wasm(index)),
            ContextArgs::pinned_callee_and_caller_vmctx(),
        )
    }

    /// Lowers a function import by loading its address to the next available
    /// register.
    fn lower_import<M: MacroAssembler, P: PtrSize>(
        index: FuncIndex,
        sig: &ABISig,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
        vmoffsets: &VMOffsets<P>,
    ) -> Result<(CalleeKind, ContextArgs)> {
        let (callee, callee_vmctx) =
            context.without::<Result<(Reg, Reg)>, M, _>(&sig.regs, masm, |context, masm| {
                Ok((context.any_gpr(masm)?, context.any_gpr(masm)?))
            })??;
        let callee_vmctx_offset = vmoffsets.vmctx_vmfunction_import_vmctx(index);
        let callee_vmctx_addr = masm.address_at_vmctx(callee_vmctx_offset)?;
        masm.load_ptr(callee_vmctx_addr, writable!(callee_vmctx))?;

        let callee_body_offset = vmoffsets.vmctx_vmfunction_import_wasm_call(index);
        let callee_addr = masm.address_at_vmctx(callee_body_offset)?;
        masm.load_ptr(callee_addr, writable!(callee))?;

        Ok((
            CalleeKind::indirect(callee),
            ContextArgs::with_callee_and_pinned_caller(callee_vmctx),
        ))
    }

    /// Lowers a function reference by loading its address into the next
    /// available register.
    fn lower_funcref<M: MacroAssembler>(
        sig: &ABISig,
        ptr: impl PtrSize,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
    ) -> Result<(CalleeKind, ContextArgs)> {
        // Pop the funcref pointer to a register and allocate a register to hold the
        // address of the funcref. Since the callee is not addressed from a global non
        // allocatable register (like the vmctx in the case of an import), we load the
        // funcref to a register ensuring that it doesn't get assigned to a register
        // used in the callee's signature.
        let (funcref_ptr, funcref, callee_vmctx) = context
            .without::<Result<(Reg, Reg, Reg)>, M, _>(&sig.regs, masm, |cx, masm| {
                Ok((
                    cx.pop_to_reg(masm, None)?.into(),
                    cx.any_gpr(masm)?,
                    cx.any_gpr(masm)?,
                ))
            })??;

        // Load the callee VMContext, that will be passed as first argument to
        // the function call.
        masm.load_ptr(
            masm.address_at_reg(funcref_ptr, ptr.vm_func_ref_vmctx().into())?,
            writable!(callee_vmctx),
        )?;

        // Load the function pointer to be called.
        masm.load_ptr(
            masm.address_at_reg(funcref_ptr, ptr.vm_func_ref_wasm_call().into())?,
            writable!(funcref),
        )?;
        context.free_reg(funcref_ptr);

        Ok((
            CalleeKind::indirect(funcref),
            ContextArgs::with_callee_and_pinned_caller(callee_vmctx),
        ))
    }

    /// Materializes any [ContextArgs] as a function argument.
    fn assign_context_args<M: MacroAssembler>(
        sig: &ABISig,
        context: &ContextArgs,
        masm: &mut M,
    ) -> Result<()> {
        ensure!(
            sig.params().len() >= context.len(),
            CodeGenError::vmcontext_arg_expected(),
        );
        for (context_arg, operand) in context
            .as_slice()
            .iter()
            .zip(sig.params_without_retptr().iter().take(context.len()))
        {
            match (context_arg, operand) {
                (VMContextLoc::Pinned, ABIOperand::Reg { ty, reg, .. }) => {
                    masm.mov(writable!(*reg), vmctx!(M).into(), (*ty).try_into()?)?;
                }
                (VMContextLoc::Pinned, ABIOperand::Stack { ty, offset, .. }) => {
                    let addr = masm.address_at_sp(SPOffset::from_u32(*offset))?;
                    masm.store(vmctx!(M).into(), addr, (*ty).try_into()?)?;
                }
                (VMContextLoc::OffsetFromPinned(offset), ABIOperand::Reg { ty, reg, .. }) => {
                    let addr = masm.address_at_vmctx(*offset)?;
                    masm.load(addr, writable!(*reg), (*ty).try_into()?)?;
                }
                (VMContextLoc::OffsetFromPinned(_), ABIOperand::Stack { .. }) => {
                    anyhow::bail!("unimplemented load from vmctx into stack");
                }

                (VMContextLoc::Reg(src), ABIOperand::Reg { ty, reg, .. }) => {
                    masm.mov(writable!(*reg), (*src).into(), (*ty).try_into()?)?;
                }

                (VMContextLoc::Reg(src), ABIOperand::Stack { ty, offset, .. }) => {
                    let addr = masm.address_at_sp(SPOffset::from_u32(*offset))?;
                    masm.store((*src).into(), addr, (*ty).try_into()?)?;
                }
            }
        }
        Ok(())
    }

    /// Assign arguments for the function call.
    fn assign<M: MacroAssembler>(
        sig: &ABISig,
        callee_context: &ContextArgs,
        ret_area: Option<&RetArea>,
        context: &mut CodeGenContext<Emission>,
        masm: &mut M,
    ) -> Result<()> {
        let arg_count = sig.params.len_without_retptr();
        debug_assert!(arg_count >= callee_context.len());
        let stack = &context.stack;
        let stack_values = stack.peekn(arg_count - callee_context.len());

        if callee_context.len() > 0 {
            Self::assign_context_args(&sig, &callee_context, masm)?;
        }

        for (arg, val) in sig
            .params_without_retptr()
            .iter()
            .skip(callee_context.len())
            .zip(stack_values)
        {
            match arg {
                &ABIOperand::Reg { reg, .. } => {
                    context.move_val_to_reg(&val, reg, masm)?;
                }
                &ABIOperand::Stack { ty, offset, .. } => {
                    let addr = masm.address_at_sp(SPOffset::from_u32(offset))?;
                    let size: OperandSize = ty.try_into()?;
                    masm.with_scratch_for(ty, |masm, scratch| {
                        context.move_val_to_reg(val, scratch.inner(), masm)?;
                        masm.store(scratch.inner().into(), addr, size)
                    })?;
                }
            }
        }

        if sig.has_stack_results() {
            let operand = sig.params.unwrap_results_area_operand();
            let base = ret_area.unwrap().unwrap_sp();
            let addr = masm.address_from_sp(base)?;

            match operand {
                &ABIOperand::Reg { ty, reg, .. } => {
                    masm.compute_addr(addr, writable!(reg), ty.try_into()?)?;
                }
                &ABIOperand::Stack { ty, offset, .. } => {
                    let slot = masm.address_at_sp(SPOffset::from_u32(offset))?;
                    // Don't rely on `ABI::scratch_for` as we always use
                    // an int register as the return pointer.
                    masm.with_scratch::<IntScratch, _>(|masm, scratch| {
                        masm.compute_addr(addr, scratch.writable(), ty.try_into()?)?;
                        masm.store(scratch.inner().into(), slot, ty.try_into()?)
                    })?;
                }
            }
        }
        Ok(())
    }

    /// Cleanup stack space, handle multiple results, and free registers after
    /// emitting the call.
    fn cleanup<M: MacroAssembler>(
        sig: &ABISig,
        callee_context: &ContextArgs,
        callee_kind: &CalleeKind,
        reserved_space: u32,
        ret_area: Option<RetArea>,
        masm: &mut M,
        context: &mut CodeGenContext<Emission>,
    ) -> Result<()> {
        // Free any registers holding any function references.
        match callee_kind {
            CalleeKind::Indirect(r) => context.free_reg(*r),
            _ => {}
        }

        // Free any registers used as part of the [ContextArgs].
        for loc in callee_context.as_slice() {
            match loc {
                VMContextLoc::Reg(r) => context.free_reg(*r),
                _ => {}
            }
        }
        // Deallocate the reserved space for stack arguments and for alignment,
        // which was allocated last.
        masm.free_stack(reserved_space)?;

        ensure!(
            sig.params.len_without_retptr() >= callee_context.len(),
            CodeGenError::vmcontext_arg_expected()
        );

        // Drop params from value stack and calculate amount of machine stack
        // space they consumed.
        let mut stack_consumed = 0;
        context.drop_last(
            sig.params.len_without_retptr() - callee_context.len(),
            |_regalloc, v| {
                ensure!(
                    v.is_mem() || v.is_const(),
                    CodeGenError::unexpected_value_in_value_stack()
                );
                if let Val::Memory(mem) = v {
                    stack_consumed += mem.slot.size;
                }
                Ok(())
            },
        )?;

        if let Some(ret_area) = ret_area {
            if stack_consumed > 0 {
                // Perform a memory move, by shuffling the result area to
                // higher addresses. This is needed because the result area
                // is located after any memory addresses located on the stack,
                // and after spilled values consumed by the call.
                let sp = ret_area.unwrap_sp();
                let result_bytes = sig.results_stack_size();
                ensure!(
                    sp.as_u32() >= stack_consumed + result_bytes,
                    CodeGenError::invalid_sp_offset(),
                );
                let dst = SPOffset::from_u32(sp.as_u32() - stack_consumed);
                masm.memmove(sp, dst, result_bytes, MemMoveDirection::LowToHigh)?;
            }
        };

        // Free the bytes consumed by the call.
        masm.free_stack(stack_consumed)?;

        let mut calculated_ret_area = None;

        if let Some(area) = ret_area {
            if stack_consumed > 0 {
                // If there's a return area and stack space was consumed by the
                // call, adjust the return area to be to the current stack
                // pointer offset.
                calculated_ret_area = Some(RetArea::sp(masm.sp_offset()?));
            } else {
                // Else if no stack space was consumed by the call, simply use
                // the previously calculated area.
                ensure!(
                    area.unwrap_sp() == masm.sp_offset()?,
                    CodeGenError::invalid_sp_offset()
                );
                calculated_ret_area = Some(area);
            }
        }

        // In the case of [Callee], there's no need to set the [RetArea] of the
        // signature, as it's only used here to push abi results.
        context.push_abi_results(&sig.results, masm, |_, _, _| calculated_ret_area)?;
        // Reload the [VMContext] pointer into the corresponding pinned
        // register. Winch currently doesn't have any callee-saved registers in
        // the default ABI. So the callee might clobber the designated pinned
        // register.
        context.load_vmctx(masm)
    }
}

//! Lowering rules for X64.

// ISLE integration glue.
pub(super) mod isle;

use crate::ir::pcc::{FactContext, PccResult};
use crate::ir::{
    Endianness, ExternalName, Inst as IRInst, InstructionData, LibCall, Opcode, Type, types,
};
use crate::isa::x64::abi::*;
use crate::isa::x64::inst::args::*;
use crate::isa::x64::inst::*;
use crate::isa::x64::pcc;
use crate::isa::{CallConv, x64::X64Backend};
use crate::machinst::lower::*;
use crate::machinst::*;
use crate::result::CodegenResult;
use crate::settings::Flags;
use std::boxed::Box;
use target_lexicon::Triple;

/// Identifier for a particular input of an instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InsnInput {
    insn: IRInst,
    input: usize,
}

//=============================================================================
// Helpers for instruction lowering.

impl Lower<'_, Inst> {
    #[inline]
    pub fn temp_writable_gpr(&mut self) -> WritableGpr {
        WritableGpr::from_writable_reg(self.alloc_tmp(types::I64).only_reg().unwrap()).unwrap()
    }

    #[inline]
    pub fn temp_writable_xmm(&mut self) -> WritableXmm {
        WritableXmm::from_writable_reg(self.alloc_tmp(types::F64).only_reg().unwrap()).unwrap()
    }
}

fn is_int_or_ref_ty(ty: Type) -> bool {
    match ty {
        types::I8 | types::I16 | types::I32 | types::I64 => true,
        _ => false,
    }
}

/// Returns whether the given specified `input` is a result produced by an instruction with Opcode
/// `op`.
// TODO investigate failures with checking against the result index.
fn matches_input(ctx: &mut Lower<Inst>, input: InsnInput, op: Opcode) -> Option<IRInst> {
    let inputs = ctx.get_input_as_source_or_const(input.insn, input.input);
    inputs.inst.as_inst().and_then(|(src_inst, _)| {
        let data = ctx.data(src_inst);
        if data.opcode() == op {
            return Some(src_inst);
        }
        None
    })
}

/// Put the given input into possibly multiple registers, and mark it as used (side-effect).
fn put_input_in_regs(ctx: &mut Lower<Inst>, spec: InsnInput) -> ValueRegs<Reg> {
    let ty = ctx.input_ty(spec.insn, spec.input);
    let input = ctx.get_input_as_source_or_const(spec.insn, spec.input);

    if let Some(c) = input.constant {
        // Generate constants fresh at each use to minimize long-range register pressure.
        let size = if ty_bits(ty) < 64 {
            OperandSize::Size32
        } else {
            OperandSize::Size64
        };
        assert!(is_int_or_ref_ty(ty)); // Only used for addresses.
        let cst_copy = ctx.alloc_tmp(ty);
        ctx.emit(Inst::imm(size, c, cst_copy.only_reg().unwrap()));
        non_writable_value_regs(cst_copy)
    } else {
        ctx.put_input_in_regs(spec.insn, spec.input)
    }
}

/// Put the given input into a register, and mark it as used (side-effect).
fn put_input_in_reg(ctx: &mut Lower<Inst>, spec: InsnInput) -> Reg {
    put_input_in_regs(ctx, spec)
        .only_reg()
        .expect("Multi-register value not expected")
}

enum MergeableLoadSize {
    /// The load size performed by a sinkable load merging operation is
    /// precisely the size necessary for the type in question.
    Exact,

    /// Narrower-than-32-bit values are handled by ALU insts that are at least
    /// 32 bits wide, which is normally OK as we ignore upper buts; but, if we
    /// generate, e.g., a direct-from-memory 32-bit add for a byte value and
    /// the byte is the last byte in a page, the extra data that we load is
    /// incorrectly accessed. So we only allow loads to merge for
    /// 32-bit-and-above widths.
    Min32,
}

/// Determines whether a load operation (indicated by `src_insn`) can be merged
/// into the current lowering point. If so, returns the address-base source (as
/// an `InsnInput`) and an offset from that address from which to perform the
/// load.
fn is_mergeable_load(
    ctx: &mut Lower<Inst>,
    src_insn: IRInst,
    size: MergeableLoadSize,
) -> Option<(InsnInput, i32)> {
    let insn_data = ctx.data(src_insn);
    let inputs = ctx.num_inputs(src_insn);
    if inputs != 1 {
        return None;
    }

    // If this type is too small to get a merged load, don't merge the load.
    let load_ty = ctx.output_ty(src_insn, 0);
    if ty_bits(load_ty) < 32 {
        match size {
            MergeableLoadSize::Exact => {}
            MergeableLoadSize::Min32 => return None,
        }
    }

    // If the load's flags specify big-endian, we can't merge.
    if let Some(flags) = ctx.memflags(src_insn) {
        if flags.explicit_endianness() == Some(Endianness::Big) {
            return None;
        }
    }

    // Just testing the opcode is enough, because the width will always match if
    // the type does (and the type should match if the CLIF is properly
    // constructed).
    if let &InstructionData::Load {
        opcode: Opcode::Load,
        offset,
        ..
    } = insn_data
    {
        Some((
            InsnInput {
                insn: src_insn,
                input: 0,
            },
            offset.into(),
        ))
    } else {
        None
    }
}

fn input_to_imm(ctx: &mut Lower<Inst>, spec: InsnInput) -> Option<u64> {
    ctx.get_input_as_source_or_const(spec.insn, spec.input)
        .constant
}

fn emit_vm_call(
    ctx: &mut Lower<Inst>,
    flags: &Flags,
    triple: &Triple,
    libcall: LibCall,
    inputs: &[ValueRegs<Reg>],
) -> CodegenResult<InstOutput> {
    let extname = ExternalName::LibCall(libcall);

    // TODO avoid recreating signatures for every single Libcall function.
    let call_conv = CallConv::for_libcall(flags, CallConv::triple_default(triple));
    let sig = libcall.signature(call_conv, types::I64);
    let outputs = ctx.gen_call_output(&sig);

    if !ctx.sigs().have_abi_sig_for_signature(&sig) {
        ctx.sigs_mut()
            .make_abi_sig_from_ir_signature::<X64ABIMachineSpec>(sig.clone(), flags)?;
    }
    let sig = ctx.sigs().abi_sig_for_signature(&sig);

    let uses = ctx.gen_call_args(sig, inputs);
    let defs = ctx.gen_call_rets(sig, &outputs);

    let stack_ret_space = ctx.sigs()[sig].sized_stack_ret_space();
    let stack_arg_space = ctx.sigs()[sig].sized_stack_arg_space();
    ctx.abi_mut()
        .accumulate_outgoing_args_size(stack_ret_space + stack_arg_space);

    if flags.use_colocated_libcalls() {
        let call_info = ctx.gen_call_info(sig, extname, uses, defs, None);
        ctx.emit(Inst::call_known(Box::new(call_info)));
    } else {
        let tmp = ctx.alloc_tmp(types::I64).only_reg().unwrap();
        ctx.emit(Inst::LoadExtName {
            dst: tmp.map(Gpr::unwrap_new),
            name: Box::new(extname),
            offset: 0,
            distance: RelocDistance::Far,
        });
        let call_info = ctx.gen_call_info(sig, RegMem::reg(tmp.to_reg()), uses, defs, None);
        ctx.emit(Inst::call_unknown(Box::new(call_info)));
    }
    Ok(outputs)
}

/// Returns whether the given input is a shift by a constant value less or equal than 3.
/// The goal is to embed it within an address mode.
fn matches_small_constant_shift(ctx: &mut Lower<Inst>, spec: InsnInput) -> Option<(InsnInput, u8)> {
    matches_input(ctx, spec, Opcode::Ishl).and_then(|shift| {
        match input_to_imm(
            ctx,
            InsnInput {
                insn: shift,
                input: 1,
            },
        ) {
            Some(shift_amt) if shift_amt <= 3 => Some((
                InsnInput {
                    insn: shift,
                    input: 0,
                },
                shift_amt as u8,
            )),
            _ => None,
        }
    })
}

/// Lowers an instruction to one of the x86 addressing modes.
///
/// Note: the 32-bit offset in Cranelift has to be sign-extended, which maps x86's behavior.
fn lower_to_amode(ctx: &mut Lower<Inst>, spec: InsnInput, offset: i32) -> Amode {
    let flags = ctx
        .memflags(spec.insn)
        .expect("Instruction with amode should have memflags");

    // We now either have an add that we must materialize, or some other input; as well as the
    // final offset.
    if let Some(add) = matches_input(ctx, spec, Opcode::Iadd) {
        let output_ty = ctx.output_ty(add, 0);
        debug_assert_eq!(
            output_ty,
            types::I64,
            "Address width of 64 expected, got {output_ty}"
        );
        let add_inputs = &[
            InsnInput {
                insn: add,
                input: 0,
            },
            InsnInput {
                insn: add,
                input: 1,
            },
        ];

        // TODO heap_addr legalization generates a uext64 *after* the shift, so these optimizations
        // aren't happening in the wasm case. We could do better, given some range analysis.
        let (base, index, shift) = if let Some((shift_input, shift_amt)) =
            matches_small_constant_shift(ctx, add_inputs[0])
        {
            (
                put_input_in_reg(ctx, add_inputs[1]),
                put_input_in_reg(ctx, shift_input),
                shift_amt,
            )
        } else if let Some((shift_input, shift_amt)) =
            matches_small_constant_shift(ctx, add_inputs[1])
        {
            (
                put_input_in_reg(ctx, add_inputs[0]),
                put_input_in_reg(ctx, shift_input),
                shift_amt,
            )
        } else {
            for input in 0..=1 {
                // Try to pierce through uextend.
                let (inst, inst_input) = if let Some(uextend) =
                    matches_input(ctx, InsnInput { insn: add, input }, Opcode::Uextend)
                {
                    (uextend, 0)
                } else {
                    (add, input)
                };

                // If it's a constant, add it directly!
                if let Some(cst) = ctx.get_input_as_source_or_const(inst, inst_input).constant {
                    let final_offset = (offset as i64).wrapping_add(cst as i64);
                    if let Ok(final_offset) = i32::try_from(final_offset) {
                        let base = put_input_in_reg(ctx, add_inputs[1 - input]);
                        return Amode::imm_reg(final_offset, base).with_flags(flags);
                    }
                }
            }

            (
                put_input_in_reg(ctx, add_inputs[0]),
                put_input_in_reg(ctx, add_inputs[1]),
                0,
            )
        };

        return Amode::imm_reg_reg_shift(
            offset,
            Gpr::unwrap_new(base),
            Gpr::unwrap_new(index),
            shift,
        )
        .with_flags(flags);
    }

    let input = put_input_in_reg(ctx, spec);
    Amode::imm_reg(offset, input).with_flags(flags)
}

//=============================================================================
// Lowering-backend trait implementation.

impl LowerBackend for X64Backend {
    type MInst = Inst;

    fn lower(&self, ctx: &mut Lower<Inst>, ir_inst: IRInst) -> Option<InstOutput> {
        isle::lower(ctx, self, ir_inst)
    }

    fn lower_branch(
        &self,
        ctx: &mut Lower<Inst>,
        ir_inst: IRInst,
        targets: &[MachLabel],
    ) -> Option<()> {
        isle::lower_branch(ctx, self, ir_inst, targets)
    }

    fn maybe_pinned_reg(&self) -> Option<Reg> {
        Some(regs::pinned_reg())
    }

    fn check_fact(
        &self,
        ctx: &FactContext<'_>,
        vcode: &mut VCode<Self::MInst>,
        inst: InsnIndex,
        state: &mut pcc::FactFlowState,
    ) -> PccResult<()> {
        pcc::check(ctx, vcode, inst, state)
    }

    type FactFlowState = pcc::FactFlowState;
}

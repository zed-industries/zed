//! Lowering rules for AArch64.
//!
//! TODO: opportunities for better code generation:
//!
//! - Smarter use of addressing modes. Recognize a+SCALE*b patterns. Recognize
//!   pre/post-index opportunities.
//!
//! - Floating-point immediates (FIMM instruction).

use crate::ir::Inst as IRInst;
use crate::ir::condcodes::{FloatCC, IntCC};
use crate::ir::pcc::{FactContext, PccResult};
use crate::ir::{Opcode, Value};
use crate::isa::aarch64::AArch64Backend;
use crate::isa::aarch64::inst::*;
use crate::isa::aarch64::pcc;
use crate::machinst::lower::*;
use crate::machinst::*;

pub mod isle;

//============================================================================
// Lowering: convert instruction inputs to forms that we can use.

fn get_as_extended_value(ctx: &mut Lower<Inst>, val: Value) -> Option<(Value, ExtendOp)> {
    let inputs = ctx.get_value_as_source_or_const(val);
    let (insn, n) = inputs.inst.as_inst()?;
    if n != 0 {
        return None;
    }
    let op = ctx.data(insn).opcode();
    let out_ty = ctx.output_ty(insn, 0);
    let out_bits = ty_bits(out_ty);

    // Is this a zero-extend or sign-extend and can we handle that with a register-mode operator?
    if op == Opcode::Uextend || op == Opcode::Sextend {
        let sign_extend = op == Opcode::Sextend;
        let inner_ty = ctx.input_ty(insn, 0);
        let inner_bits = ty_bits(inner_ty);
        assert!(inner_bits < out_bits);
        let extendop = match (sign_extend, inner_bits) {
            (true, 8) => ExtendOp::SXTB,
            (false, 8) => ExtendOp::UXTB,
            (true, 16) => ExtendOp::SXTH,
            (false, 16) => ExtendOp::UXTH,
            (true, 32) => ExtendOp::SXTW,
            (false, 32) => ExtendOp::UXTW,
            _ => unreachable!(),
        };
        return Some((ctx.input_as_value(insn, 0), extendop));
    }

    None
}

pub(crate) fn lower_condcode(cc: IntCC) -> Cond {
    match cc {
        IntCC::Equal => Cond::Eq,
        IntCC::NotEqual => Cond::Ne,
        IntCC::SignedGreaterThanOrEqual => Cond::Ge,
        IntCC::SignedGreaterThan => Cond::Gt,
        IntCC::SignedLessThanOrEqual => Cond::Le,
        IntCC::SignedLessThan => Cond::Lt,
        IntCC::UnsignedGreaterThanOrEqual => Cond::Hs,
        IntCC::UnsignedGreaterThan => Cond::Hi,
        IntCC::UnsignedLessThanOrEqual => Cond::Ls,
        IntCC::UnsignedLessThan => Cond::Lo,
    }
}

pub(crate) fn lower_fp_condcode(cc: FloatCC) -> Cond {
    // Refer to `codegen/shared/src/condcodes.rs` and to the `FCMP` AArch64 docs.
    // The FCMP instruction sets:
    //               NZCV
    // - PCSR.NZCV = 0011 on UN (unordered),
    //               0110 on EQ,
    //               1000 on LT,
    //               0010 on GT.
    match cc {
        // EQ | LT | GT. Vc => V clear.
        FloatCC::Ordered => Cond::Vc,
        // UN. Vs => V set.
        FloatCC::Unordered => Cond::Vs,
        // EQ. Eq => Z set.
        FloatCC::Equal => Cond::Eq,
        // UN | LT | GT. Ne => Z clear.
        FloatCC::NotEqual => Cond::Ne,
        // LT | GT.
        FloatCC::OrderedNotEqual => unimplemented!(),
        //  UN | EQ
        FloatCC::UnorderedOrEqual => unimplemented!(),
        // LT. Mi => N set.
        FloatCC::LessThan => Cond::Mi,
        // LT | EQ. Ls => C clear or Z set.
        FloatCC::LessThanOrEqual => Cond::Ls,
        // GT. Gt => Z clear, N = V.
        FloatCC::GreaterThan => Cond::Gt,
        // GT | EQ. Ge => N = V.
        FloatCC::GreaterThanOrEqual => Cond::Ge,
        // UN | LT
        FloatCC::UnorderedOrLessThan => unimplemented!(),
        // UN | LT | EQ
        FloatCC::UnorderedOrLessThanOrEqual => unimplemented!(),
        // UN | GT
        FloatCC::UnorderedOrGreaterThan => unimplemented!(),
        // UN | GT | EQ
        FloatCC::UnorderedOrGreaterThanOrEqual => unimplemented!(),
    }
}

//=============================================================================
// Lowering-backend trait implementation.

impl LowerBackend for AArch64Backend {
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

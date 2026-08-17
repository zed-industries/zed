//! This module defines aarch64-specific machine instruction types.

use crate::binemit::{Addend, CodeOffset, Reloc};
use crate::ir::types::{F16, F32, F64, F128, I8, I8X16, I16, I32, I64, I128};
use crate::ir::{MemFlags, Type, types};
use crate::isa::{CallConv, FunctionAlignment};
use crate::machinst::*;
use crate::{CodegenError, CodegenResult, settings};

use crate::machinst::{PrettyPrint, Reg, RegClass, Writable};

use alloc::vec::Vec;
use core::slice;
use smallvec::{SmallVec, smallvec};
use std::fmt::Write;
use std::string::{String, ToString};

pub(crate) mod regs;
pub(crate) use self::regs::*;
pub mod imms;
pub use self::imms::*;
pub mod args;
pub use self::args::*;
pub mod emit;
pub(crate) use self::emit::*;
use crate::isa::aarch64::abi::AArch64MachineDeps;

pub(crate) mod unwind;

#[cfg(test)]
mod emit_tests;

//=============================================================================
// Instructions (top level): definition

pub use crate::isa::aarch64::lower::isle::generated_code::{
    ALUOp, ALUOp3, AMode, APIKey, AtomicRMWLoopOp, AtomicRMWOp, BitOp, BranchTargetType, FPUOp1,
    FPUOp2, FPUOp3, FpuRoundMode, FpuToIntOp, IntToFpuOp, MInst as Inst, MoveWideOp, VecALUModOp,
    VecALUOp, VecExtendOp, VecLanesOp, VecMisc2, VecPairOp, VecRRLongOp, VecRRNarrowOp,
    VecRRPairLongOp, VecRRRLongModOp, VecRRRLongOp, VecShiftImmModOp, VecShiftImmOp,
};

/// A floating-point unit (FPU) operation with two args, a register and an immediate.
#[derive(Copy, Clone, Debug)]
pub enum FPUOpRI {
    /// Unsigned right shift. Rd = Rn << #imm
    UShr32(FPURightShiftImm),
    /// Unsigned right shift. Rd = Rn << #imm
    UShr64(FPURightShiftImm),
}

/// A floating-point unit (FPU) operation with two args, a register and
/// an immediate that modifies its dest (so takes that input value as a
/// separate virtual register).
#[derive(Copy, Clone, Debug)]
pub enum FPUOpRIMod {
    /// Shift left and insert. Rd |= Rn << #imm
    Sli32(FPULeftShiftImm),
    /// Shift left and insert. Rd |= Rn << #imm
    Sli64(FPULeftShiftImm),
}

impl BitOp {
    /// Get the assembly mnemonic for this opcode.
    pub fn op_str(&self) -> &'static str {
        match self {
            BitOp::RBit => "rbit",
            BitOp::Clz => "clz",
            BitOp::Cls => "cls",
            BitOp::Rev16 => "rev16",
            BitOp::Rev32 => "rev32",
            BitOp::Rev64 => "rev64",
        }
    }
}

/// Additional information for `return_call[_ind]` instructions, left out of
/// line to lower the size of the `Inst` enum.
#[derive(Clone, Debug)]
pub struct ReturnCallInfo<T> {
    /// Where this call is going to
    pub dest: T,
    /// Arguments to the call instruction.
    pub uses: CallArgList,
    /// The size of the new stack frame's stack arguments. This is necessary
    /// for copying the frame over our current frame. It must already be
    /// allocated on the stack.
    pub new_stack_arg_size: u32,
    /// API key to use to restore the return address, if any.
    pub key: Option<APIKey>,
}

fn count_zero_half_words(mut value: u64, num_half_words: u8) -> usize {
    let mut count = 0;
    for _ in 0..num_half_words {
        if value & 0xffff == 0 {
            count += 1;
        }
        value >>= 16;
    }

    count
}

impl Inst {
    /// Create an instruction that loads a constant, using one of several options (MOVZ, MOVN,
    /// logical immediate, or constant pool).
    pub fn load_constant(rd: Writable<Reg>, value: u64) -> SmallVec<[Inst; 4]> {
        // NB: this is duplicated in `lower/isle.rs` and `inst.isle` right now,
        // if modifications are made here before this is deleted after moving to
        // ISLE then those locations should be updated as well.

        if let Some(imm) = MoveWideConst::maybe_from_u64(value) {
            // 16-bit immediate (shifted by 0, 16, 32 or 48 bits) in MOVZ
            smallvec![Inst::MovWide {
                op: MoveWideOp::MovZ,
                rd,
                imm,
                size: OperandSize::Size64
            }]
        } else if let Some(imm) = MoveWideConst::maybe_from_u64(!value) {
            // 16-bit immediate (shifted by 0, 16, 32 or 48 bits) in MOVN
            smallvec![Inst::MovWide {
                op: MoveWideOp::MovN,
                rd,
                imm,
                size: OperandSize::Size64
            }]
        } else if let Some(imml) = ImmLogic::maybe_from_u64(value, I64) {
            // Weird logical-instruction immediate in ORI using zero register
            smallvec![Inst::AluRRImmLogic {
                alu_op: ALUOp::Orr,
                size: OperandSize::Size64,
                rd,
                rn: zero_reg(),
                imml,
            }]
        } else {
            let mut insts = smallvec![];

            // If the top 32 bits are zero, use 32-bit `mov` operations.
            let (num_half_words, size, negated) = if value >> 32 == 0 {
                (2, OperandSize::Size32, (!value << 32) >> 32)
            } else {
                (4, OperandSize::Size64, !value)
            };

            // If the number of 0xffff half words is greater than the number of 0x0000 half words
            // it is more efficient to use `movn` for the first instruction.
            let first_is_inverted = count_zero_half_words(negated, num_half_words)
                > count_zero_half_words(value, num_half_words);

            // Either 0xffff or 0x0000 half words can be skipped, depending on the first
            // instruction used.
            let ignored_halfword = if first_is_inverted { 0xffff } else { 0 };

            let halfwords: SmallVec<[_; 4]> = (0..num_half_words)
                .filter_map(|i| {
                    let imm16 = (value >> (16 * i)) & 0xffff;
                    if imm16 == ignored_halfword {
                        None
                    } else {
                        Some((i, imm16))
                    }
                })
                .collect();

            let mut prev_result = None;
            for (i, imm16) in halfwords {
                let shift = i * 16;

                if let Some(rn) = prev_result {
                    let imm = MoveWideConst::maybe_with_shift(imm16 as u16, shift).unwrap();
                    insts.push(Inst::MovK { rd, rn, imm, size });
                } else {
                    if first_is_inverted {
                        let imm =
                            MoveWideConst::maybe_with_shift(((!imm16) & 0xffff) as u16, shift)
                                .unwrap();
                        insts.push(Inst::MovWide {
                            op: MoveWideOp::MovN,
                            rd,
                            imm,
                            size,
                        });
                    } else {
                        let imm = MoveWideConst::maybe_with_shift(imm16 as u16, shift).unwrap();
                        insts.push(Inst::MovWide {
                            op: MoveWideOp::MovZ,
                            rd,
                            imm,
                            size,
                        });
                    }
                }

                prev_result = Some(rd.to_reg());
            }

            assert!(prev_result.is_some());

            insts
        }
    }

    /// Generic constructor for a load (zero-extending where appropriate).
    pub fn gen_load(into_reg: Writable<Reg>, mem: AMode, ty: Type, flags: MemFlags) -> Inst {
        match ty {
            I8 => Inst::ULoad8 {
                rd: into_reg,
                mem,
                flags,
            },
            I16 => Inst::ULoad16 {
                rd: into_reg,
                mem,
                flags,
            },
            I32 => Inst::ULoad32 {
                rd: into_reg,
                mem,
                flags,
            },
            I64 => Inst::ULoad64 {
                rd: into_reg,
                mem,
                flags,
            },
            _ => {
                if ty.is_vector() || ty.is_float() {
                    let bits = ty_bits(ty);
                    let rd = into_reg;

                    match bits {
                        128 => Inst::FpuLoad128 { rd, mem, flags },
                        64 => Inst::FpuLoad64 { rd, mem, flags },
                        32 => Inst::FpuLoad32 { rd, mem, flags },
                        16 => Inst::FpuLoad16 { rd, mem, flags },
                        _ => unimplemented!("gen_load({})", ty),
                    }
                } else {
                    unimplemented!("gen_load({})", ty);
                }
            }
        }
    }

    /// Generic constructor for a store.
    pub fn gen_store(mem: AMode, from_reg: Reg, ty: Type, flags: MemFlags) -> Inst {
        match ty {
            I8 => Inst::Store8 {
                rd: from_reg,
                mem,
                flags,
            },
            I16 => Inst::Store16 {
                rd: from_reg,
                mem,
                flags,
            },
            I32 => Inst::Store32 {
                rd: from_reg,
                mem,
                flags,
            },
            I64 => Inst::Store64 {
                rd: from_reg,
                mem,
                flags,
            },
            _ => {
                if ty.is_vector() || ty.is_float() {
                    let bits = ty_bits(ty);
                    let rd = from_reg;

                    match bits {
                        128 => Inst::FpuStore128 { rd, mem, flags },
                        64 => Inst::FpuStore64 { rd, mem, flags },
                        32 => Inst::FpuStore32 { rd, mem, flags },
                        16 => Inst::FpuStore16 { rd, mem, flags },
                        _ => unimplemented!("gen_store({})", ty),
                    }
                } else {
                    unimplemented!("gen_store({})", ty);
                }
            }
        }
    }

    /// What type does this load or store instruction access in memory? When
    /// uimm12 encoding is used, the size of this type is the amount that
    /// immediate offsets are scaled by.
    pub fn mem_type(&self) -> Option<Type> {
        match self {
            Inst::ULoad8 { .. } => Some(I8),
            Inst::SLoad8 { .. } => Some(I8),
            Inst::ULoad16 { .. } => Some(I16),
            Inst::SLoad16 { .. } => Some(I16),
            Inst::ULoad32 { .. } => Some(I32),
            Inst::SLoad32 { .. } => Some(I32),
            Inst::ULoad64 { .. } => Some(I64),
            Inst::FpuLoad16 { .. } => Some(F16),
            Inst::FpuLoad32 { .. } => Some(F32),
            Inst::FpuLoad64 { .. } => Some(F64),
            Inst::FpuLoad128 { .. } => Some(I8X16),
            Inst::Store8 { .. } => Some(I8),
            Inst::Store16 { .. } => Some(I16),
            Inst::Store32 { .. } => Some(I32),
            Inst::Store64 { .. } => Some(I64),
            Inst::FpuStore16 { .. } => Some(F16),
            Inst::FpuStore32 { .. } => Some(F32),
            Inst::FpuStore64 { .. } => Some(F64),
            Inst::FpuStore128 { .. } => Some(I8X16),
            _ => None,
        }
    }
}

//=============================================================================
// Instructions: get_regs

fn memarg_operands(memarg: &mut AMode, collector: &mut impl OperandVisitor) {
    match memarg {
        AMode::Unscaled { rn, .. } | AMode::UnsignedOffset { rn, .. } => {
            collector.reg_use(rn);
        }
        AMode::RegReg { rn, rm, .. }
        | AMode::RegScaled { rn, rm, .. }
        | AMode::RegScaledExtended { rn, rm, .. }
        | AMode::RegExtended { rn, rm, .. } => {
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        AMode::Label { .. } => {}
        AMode::SPPreIndexed { .. } | AMode::SPPostIndexed { .. } => {}
        AMode::FPOffset { .. } | AMode::IncomingArg { .. } => {}
        AMode::SPOffset { .. } | AMode::SlotOffset { .. } => {}
        AMode::RegOffset { rn, .. } => {
            collector.reg_use(rn);
        }
        AMode::Const { .. } => {}
    }
}

fn pairmemarg_operands(pairmemarg: &mut PairAMode, collector: &mut impl OperandVisitor) {
    match pairmemarg {
        PairAMode::SignedOffset { reg, .. } => {
            collector.reg_use(reg);
        }
        PairAMode::SPPreIndexed { .. } | PairAMode::SPPostIndexed { .. } => {}
    }
}

fn aarch64_get_operands(inst: &mut Inst, collector: &mut impl OperandVisitor) {
    match inst {
        Inst::AluRRR { rd, rn, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::AluRRRR { rd, rn, rm, ra, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
            collector.reg_use(ra);
        }
        Inst::AluRRImm12 { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::AluRRImmLogic { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::AluRRImmShift { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::AluRRRShift { rd, rn, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::AluRRRExtend { rd, rn, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::BitRR { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::ULoad8 { rd, mem, .. }
        | Inst::SLoad8 { rd, mem, .. }
        | Inst::ULoad16 { rd, mem, .. }
        | Inst::SLoad16 { rd, mem, .. }
        | Inst::ULoad32 { rd, mem, .. }
        | Inst::SLoad32 { rd, mem, .. }
        | Inst::ULoad64 { rd, mem, .. } => {
            collector.reg_def(rd);
            memarg_operands(mem, collector);
        }
        Inst::Store8 { rd, mem, .. }
        | Inst::Store16 { rd, mem, .. }
        | Inst::Store32 { rd, mem, .. }
        | Inst::Store64 { rd, mem, .. } => {
            collector.reg_use(rd);
            memarg_operands(mem, collector);
        }
        Inst::StoreP64 { rt, rt2, mem, .. } => {
            collector.reg_use(rt);
            collector.reg_use(rt2);
            pairmemarg_operands(mem, collector);
        }
        Inst::LoadP64 { rt, rt2, mem, .. } => {
            collector.reg_def(rt);
            collector.reg_def(rt2);
            pairmemarg_operands(mem, collector);
        }
        Inst::Mov { rd, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rm);
        }
        Inst::MovFromPReg { rd, rm } => {
            debug_assert!(rd.to_reg().is_virtual());
            collector.reg_def(rd);
            collector.reg_fixed_nonallocatable(*rm);
        }
        Inst::MovToPReg { rd, rm } => {
            debug_assert!(rm.is_virtual());
            collector.reg_fixed_nonallocatable(*rd);
            collector.reg_use(rm);
        }
        Inst::MovK { rd, rn, .. } => {
            collector.reg_use(rn);
            collector.reg_reuse_def(rd, 0); // `rn` == `rd`.
        }
        Inst::MovWide { rd, .. } => {
            collector.reg_def(rd);
        }
        Inst::CSel { rd, rn, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::CSNeg { rd, rn, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::CSet { rd, .. } | Inst::CSetm { rd, .. } => {
            collector.reg_def(rd);
        }
        Inst::CCmp { rn, rm, .. } => {
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::CCmpImm { rn, .. } => {
            collector.reg_use(rn);
        }
        Inst::AtomicRMWLoop {
            op,
            addr,
            operand,
            oldval,
            scratch1,
            scratch2,
            ..
        } => {
            collector.reg_fixed_use(addr, xreg(25));
            collector.reg_fixed_use(operand, xreg(26));
            collector.reg_fixed_def(oldval, xreg(27));
            collector.reg_fixed_def(scratch1, xreg(24));
            if *op != AtomicRMWLoopOp::Xchg {
                collector.reg_fixed_def(scratch2, xreg(28));
            }
        }
        Inst::AtomicRMW { rs, rt, rn, .. } => {
            collector.reg_use(rs);
            collector.reg_def(rt);
            collector.reg_use(rn);
        }
        Inst::AtomicCAS { rd, rs, rt, rn, .. } => {
            collector.reg_reuse_def(rd, 1); // reuse `rs`.
            collector.reg_use(rs);
            collector.reg_use(rt);
            collector.reg_use(rn);
        }
        Inst::AtomicCASLoop {
            addr,
            expected,
            replacement,
            oldval,
            scratch,
            ..
        } => {
            collector.reg_fixed_use(addr, xreg(25));
            collector.reg_fixed_use(expected, xreg(26));
            collector.reg_fixed_use(replacement, xreg(28));
            collector.reg_fixed_def(oldval, xreg(27));
            collector.reg_fixed_def(scratch, xreg(24));
        }
        Inst::LoadAcquire { rt, rn, .. } => {
            collector.reg_use(rn);
            collector.reg_def(rt);
        }
        Inst::StoreRelease { rt, rn, .. } => {
            collector.reg_use(rn);
            collector.reg_use(rt);
        }
        Inst::Fence {} | Inst::Csdb {} => {}
        Inst::FpuMove32 { rd, rn } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::FpuMove64 { rd, rn } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::FpuMove128 { rd, rn } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::FpuMoveFromVec { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::FpuExtend { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::FpuRR { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::FpuRRR { rd, rn, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::FpuRRI { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::FpuRRIMod { rd, ri, rn, .. } => {
            collector.reg_reuse_def(rd, 1); // reuse `ri`.
            collector.reg_use(ri);
            collector.reg_use(rn);
        }
        Inst::FpuRRRR { rd, rn, rm, ra, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
            collector.reg_use(ra);
        }
        Inst::VecMisc { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }

        Inst::VecLanes { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::VecShiftImm { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::VecShiftImmMod { rd, ri, rn, .. } => {
            collector.reg_reuse_def(rd, 1); // `rd` == `ri`.
            collector.reg_use(ri);
            collector.reg_use(rn);
        }
        Inst::VecExtract { rd, rn, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::VecTbl { rd, rn, rm } => {
            collector.reg_use(rn);
            collector.reg_use(rm);
            collector.reg_def(rd);
        }
        Inst::VecTblExt { rd, ri, rn, rm } => {
            collector.reg_use(rn);
            collector.reg_use(rm);
            collector.reg_reuse_def(rd, 3); // `rd` == `ri`.
            collector.reg_use(ri);
        }

        Inst::VecTbl2 { rd, rn, rn2, rm } => {
            // Constrain to v30 / v31 so that we satisfy the "adjacent
            // registers" constraint without use of pinned vregs in
            // lowering.
            collector.reg_fixed_use(rn, vreg(30));
            collector.reg_fixed_use(rn2, vreg(31));
            collector.reg_use(rm);
            collector.reg_def(rd);
        }
        Inst::VecTbl2Ext {
            rd,
            ri,
            rn,
            rn2,
            rm,
        } => {
            // Constrain to v30 / v31 so that we satisfy the "adjacent
            // registers" constraint without use of pinned vregs in
            // lowering.
            collector.reg_fixed_use(rn, vreg(30));
            collector.reg_fixed_use(rn2, vreg(31));
            collector.reg_use(rm);
            collector.reg_reuse_def(rd, 4); // `rd` == `ri`.
            collector.reg_use(ri);
        }
        Inst::VecLoadReplicate { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::VecCSel { rd, rn, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::FpuCmp { rn, rm, .. } => {
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::FpuLoad16 { rd, mem, .. } => {
            collector.reg_def(rd);
            memarg_operands(mem, collector);
        }
        Inst::FpuLoad32 { rd, mem, .. } => {
            collector.reg_def(rd);
            memarg_operands(mem, collector);
        }
        Inst::FpuLoad64 { rd, mem, .. } => {
            collector.reg_def(rd);
            memarg_operands(mem, collector);
        }
        Inst::FpuLoad128 { rd, mem, .. } => {
            collector.reg_def(rd);
            memarg_operands(mem, collector);
        }
        Inst::FpuStore16 { rd, mem, .. } => {
            collector.reg_use(rd);
            memarg_operands(mem, collector);
        }
        Inst::FpuStore32 { rd, mem, .. } => {
            collector.reg_use(rd);
            memarg_operands(mem, collector);
        }
        Inst::FpuStore64 { rd, mem, .. } => {
            collector.reg_use(rd);
            memarg_operands(mem, collector);
        }
        Inst::FpuStore128 { rd, mem, .. } => {
            collector.reg_use(rd);
            memarg_operands(mem, collector);
        }
        Inst::FpuLoadP64 { rt, rt2, mem, .. } => {
            collector.reg_def(rt);
            collector.reg_def(rt2);
            pairmemarg_operands(mem, collector);
        }
        Inst::FpuStoreP64 { rt, rt2, mem, .. } => {
            collector.reg_use(rt);
            collector.reg_use(rt2);
            pairmemarg_operands(mem, collector);
        }
        Inst::FpuLoadP128 { rt, rt2, mem, .. } => {
            collector.reg_def(rt);
            collector.reg_def(rt2);
            pairmemarg_operands(mem, collector);
        }
        Inst::FpuStoreP128 { rt, rt2, mem, .. } => {
            collector.reg_use(rt);
            collector.reg_use(rt2);
            pairmemarg_operands(mem, collector);
        }
        Inst::FpuToInt { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::IntToFpu { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::FpuCSel16 { rd, rn, rm, .. }
        | Inst::FpuCSel32 { rd, rn, rm, .. }
        | Inst::FpuCSel64 { rd, rn, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::FpuRound { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::MovToFpu { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::FpuMoveFPImm { rd, .. } => {
            collector.reg_def(rd);
        }
        Inst::MovToVec { rd, ri, rn, .. } => {
            collector.reg_reuse_def(rd, 1); // `rd` == `ri`.
            collector.reg_use(ri);
            collector.reg_use(rn);
        }
        Inst::MovFromVec { rd, rn, .. } | Inst::MovFromVecSigned { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::VecDup { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::VecDupFromFpu { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::VecDupFPImm { rd, .. } => {
            collector.reg_def(rd);
        }
        Inst::VecDupImm { rd, .. } => {
            collector.reg_def(rd);
        }
        Inst::VecExtend { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::VecMovElement { rd, ri, rn, .. } => {
            collector.reg_reuse_def(rd, 1); // `rd` == `ri`.
            collector.reg_use(ri);
            collector.reg_use(rn);
        }
        Inst::VecRRLong { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::VecRRNarrowLow { rd, rn, .. } => {
            collector.reg_use(rn);
            collector.reg_def(rd);
        }
        Inst::VecRRNarrowHigh { rd, ri, rn, .. } => {
            collector.reg_use(rn);
            collector.reg_reuse_def(rd, 2); // `rd` == `ri`.
            collector.reg_use(ri);
        }
        Inst::VecRRPair { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::VecRRRLong { rd, rn, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::VecRRRLongMod { rd, ri, rn, rm, .. } => {
            collector.reg_reuse_def(rd, 1); // `rd` == `ri`.
            collector.reg_use(ri);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::VecRRPairLong { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::VecRRR { rd, rn, rm, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::VecRRRMod { rd, ri, rn, rm, .. } | Inst::VecFmlaElem { rd, ri, rn, rm, .. } => {
            collector.reg_reuse_def(rd, 1); // `rd` == `ri`.
            collector.reg_use(ri);
            collector.reg_use(rn);
            collector.reg_use(rm);
        }
        Inst::MovToNZCV { rn } => {
            collector.reg_use(rn);
        }
        Inst::MovFromNZCV { rd } => {
            collector.reg_def(rd);
        }
        Inst::Extend { rd, rn, .. } => {
            collector.reg_def(rd);
            collector.reg_use(rn);
        }
        Inst::Args { args } => {
            for ArgPair { vreg, preg } in args {
                collector.reg_fixed_def(vreg, *preg);
            }
        }
        Inst::Rets { rets } => {
            for RetPair { vreg, preg } in rets {
                collector.reg_fixed_use(vreg, *preg);
            }
        }
        Inst::Ret { .. } | Inst::AuthenticatedRet { .. } => {}
        Inst::Jump { .. } => {}
        Inst::Call { info, .. } => {
            let CallInfo { uses, defs, .. } = &mut **info;
            for CallArgPair { vreg, preg } in uses {
                collector.reg_fixed_use(vreg, *preg);
            }
            for CallRetPair { vreg, location } in defs {
                match location {
                    RetLocation::Reg(preg, ..) => collector.reg_fixed_def(vreg, *preg),
                    RetLocation::Stack(..) => collector.any_def(vreg),
                }
            }
            collector.reg_clobbers(info.clobbers);
            if let Some(try_call_info) = &mut info.try_call_info {
                try_call_info.collect_operands(collector);
            }
        }
        Inst::CallInd { info, .. } => {
            let CallInfo {
                dest, uses, defs, ..
            } = &mut **info;
            collector.reg_use(dest);
            for CallArgPair { vreg, preg } in uses {
                collector.reg_fixed_use(vreg, *preg);
            }
            for CallRetPair { vreg, location } in defs {
                match location {
                    RetLocation::Reg(preg, ..) => collector.reg_fixed_def(vreg, *preg),
                    RetLocation::Stack(..) => collector.any_def(vreg),
                }
            }
            collector.reg_clobbers(info.clobbers);
            if let Some(try_call_info) = &mut info.try_call_info {
                try_call_info.collect_operands(collector);
            }
        }
        Inst::ReturnCall { info } => {
            for CallArgPair { vreg, preg } in &mut info.uses {
                collector.reg_fixed_use(vreg, *preg);
            }
        }
        Inst::ReturnCallInd { info } => {
            // TODO(https://github.com/bytecodealliance/regalloc2/issues/145):
            // This shouldn't be a fixed register constraint, but it's not clear how to pick a
            // register that won't be clobbered by the callee-save restore code emitted with a
            // return_call_indirect.
            collector.reg_fixed_use(&mut info.dest, xreg(1));
            for CallArgPair { vreg, preg } in &mut info.uses {
                collector.reg_fixed_use(vreg, *preg);
            }
        }
        Inst::CondBr { kind, .. } => match kind {
            CondBrKind::Zero(rt, _) | CondBrKind::NotZero(rt, _) => collector.reg_use(rt),
            CondBrKind::Cond(_) => {}
        },
        Inst::TestBitAndBranch { rn, .. } => {
            collector.reg_use(rn);
        }
        Inst::IndirectBr { rn, .. } => {
            collector.reg_use(rn);
        }
        Inst::Nop0 | Inst::Nop4 => {}
        Inst::Brk => {}
        Inst::Udf { .. } => {}
        Inst::TrapIf { kind, .. } => match kind {
            CondBrKind::Zero(rt, _) | CondBrKind::NotZero(rt, _) => collector.reg_use(rt),
            CondBrKind::Cond(_) => {}
        },
        Inst::Adr { rd, .. } | Inst::Adrp { rd, .. } => {
            collector.reg_def(rd);
        }
        Inst::Word4 { .. } | Inst::Word8 { .. } => {}
        Inst::JTSequence {
            ridx, rtmp1, rtmp2, ..
        } => {
            collector.reg_use(ridx);
            collector.reg_early_def(rtmp1);
            collector.reg_early_def(rtmp2);
        }
        Inst::LoadExtName { rd, .. } => {
            collector.reg_def(rd);
        }
        Inst::LoadAddr { rd, mem } => {
            collector.reg_def(rd);
            memarg_operands(mem, collector);
        }
        Inst::Paci { .. } | Inst::Xpaclri => {
            // Neither LR nor SP is an allocatable register, so there is no need
            // to do anything.
        }
        Inst::Bti { .. } => {}

        Inst::ElfTlsGetAddr { rd, tmp, .. } => {
            // TLSDESC has a very neat calling convention. It is required to preserve
            // all registers except x0 and x30. X30 is non allocatable in cranelift since
            // its the link register.
            //
            // Additionally we need a second register as a temporary register for the
            // TLSDESC sequence. This register can be any register other than x0 (and x30).
            collector.reg_fixed_def(rd, regs::xreg(0));
            collector.reg_early_def(tmp);
        }
        Inst::MachOTlsGetAddr { rd, .. } => {
            collector.reg_fixed_def(rd, regs::xreg(0));
            let mut clobbers =
                AArch64MachineDeps::get_regs_clobbered_by_call(CallConv::AppleAarch64, false);
            clobbers.remove(regs::xreg_preg(0));
            collector.reg_clobbers(clobbers);
        }
        Inst::Unwind { .. } => {}
        Inst::EmitIsland { .. } => {}
        Inst::DummyUse { reg } => {
            collector.reg_use(reg);
        }
        Inst::StackProbeLoop { start, end, .. } => {
            collector.reg_early_def(start);
            collector.reg_use(end);
        }
    }
}

//=============================================================================
// Instructions: misc functions and external interface

impl MachInst for Inst {
    type ABIMachineSpec = AArch64MachineDeps;
    type LabelUse = LabelUse;

    // "CLIF" in hex, to make the trap recognizable during
    // debugging.
    const TRAP_OPCODE: &'static [u8] = &0xc11f_u32.to_le_bytes();

    fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        aarch64_get_operands(self, collector);
    }

    fn is_move(&self) -> Option<(Writable<Reg>, Reg)> {
        match self {
            &Inst::Mov {
                size: OperandSize::Size64,
                rd,
                rm,
            } => Some((rd, rm)),
            &Inst::FpuMove64 { rd, rn } => Some((rd, rn)),
            &Inst::FpuMove128 { rd, rn } => Some((rd, rn)),
            _ => None,
        }
    }

    fn is_included_in_clobbers(&self) -> bool {
        let (caller, callee, is_exception) = match self {
            Inst::Args { .. } => return false,
            Inst::Call { info } => (
                info.caller_conv,
                info.callee_conv,
                info.try_call_info.is_some(),
            ),
            Inst::CallInd { info } => (
                info.caller_conv,
                info.callee_conv,
                info.try_call_info.is_some(),
            ),
            _ => return true,
        };

        // We exclude call instructions from the clobber-set when they are calls
        // from caller to callee that both clobber the same register (such as
        // using the same or similar ABIs). Such calls cannot possibly force any
        // new registers to be saved in the prologue, because anything that the
        // callee clobbers, the caller is also allowed to clobber. This both
        // saves work and enables us to more precisely follow the
        // half-caller-save, half-callee-save SysV ABI for some vector
        // registers.
        //
        // See the note in [crate::isa::aarch64::abi::is_caller_save_reg] for
        // more information on this ABI-implementation hack.
        let caller_clobbers = AArch64MachineDeps::get_regs_clobbered_by_call(caller, is_exception);
        let callee_clobbers = AArch64MachineDeps::get_regs_clobbered_by_call(callee, is_exception);

        let mut all_clobbers = caller_clobbers;
        all_clobbers.union_from(callee_clobbers);
        all_clobbers != caller_clobbers
    }

    fn is_trap(&self) -> bool {
        match self {
            Self::Udf { .. } => true,
            _ => false,
        }
    }

    fn is_args(&self) -> bool {
        match self {
            Self::Args { .. } => true,
            _ => false,
        }
    }

    fn is_term(&self) -> MachTerminator {
        match self {
            &Inst::Rets { .. } => MachTerminator::Ret,
            &Inst::ReturnCall { .. } | &Inst::ReturnCallInd { .. } => MachTerminator::RetCall,
            &Inst::Jump { .. } => MachTerminator::Branch,
            &Inst::CondBr { .. } => MachTerminator::Branch,
            &Inst::TestBitAndBranch { .. } => MachTerminator::Branch,
            &Inst::IndirectBr { .. } => MachTerminator::Branch,
            &Inst::JTSequence { .. } => MachTerminator::Branch,
            &Inst::Call { ref info } if info.try_call_info.is_some() => MachTerminator::Branch,
            &Inst::CallInd { ref info } if info.try_call_info.is_some() => MachTerminator::Branch,
            _ => MachTerminator::None,
        }
    }

    fn is_mem_access(&self) -> bool {
        match self {
            &Inst::ULoad8 { .. }
            | &Inst::SLoad8 { .. }
            | &Inst::ULoad16 { .. }
            | &Inst::SLoad16 { .. }
            | &Inst::ULoad32 { .. }
            | &Inst::SLoad32 { .. }
            | &Inst::ULoad64 { .. }
            | &Inst::LoadP64 { .. }
            | &Inst::FpuLoad16 { .. }
            | &Inst::FpuLoad32 { .. }
            | &Inst::FpuLoad64 { .. }
            | &Inst::FpuLoad128 { .. }
            | &Inst::FpuLoadP64 { .. }
            | &Inst::FpuLoadP128 { .. }
            | &Inst::Store8 { .. }
            | &Inst::Store16 { .. }
            | &Inst::Store32 { .. }
            | &Inst::Store64 { .. }
            | &Inst::StoreP64 { .. }
            | &Inst::FpuStore16 { .. }
            | &Inst::FpuStore32 { .. }
            | &Inst::FpuStore64 { .. }
            | &Inst::FpuStore128 { .. } => true,
            // TODO: verify this carefully
            _ => false,
        }
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: Type) -> Inst {
        let bits = ty.bits();

        assert!(bits <= 128);
        assert!(to_reg.to_reg().class() == from_reg.class());
        match from_reg.class() {
            RegClass::Int => Inst::Mov {
                size: OperandSize::Size64,
                rd: to_reg,
                rm: from_reg,
            },
            RegClass::Float => {
                if bits > 64 {
                    Inst::FpuMove128 {
                        rd: to_reg,
                        rn: from_reg,
                    }
                } else {
                    Inst::FpuMove64 {
                        rd: to_reg,
                        rn: from_reg,
                    }
                }
            }
            RegClass::Vector => unreachable!(),
        }
    }

    fn is_safepoint(&self) -> bool {
        match self {
            Inst::Call { .. } | Inst::CallInd { .. } => true,
            _ => false,
        }
    }

    fn gen_dummy_use(reg: Reg) -> Inst {
        Inst::DummyUse { reg }
    }

    fn gen_nop(preferred_size: usize) -> Inst {
        if preferred_size == 0 {
            return Inst::Nop0;
        }
        // We can't give a NOP (or any insn) < 4 bytes.
        assert!(preferred_size >= 4);
        Inst::Nop4
    }

    fn rc_for_type(ty: Type) -> CodegenResult<(&'static [RegClass], &'static [Type])> {
        match ty {
            I8 => Ok((&[RegClass::Int], &[I8])),
            I16 => Ok((&[RegClass::Int], &[I16])),
            I32 => Ok((&[RegClass::Int], &[I32])),
            I64 => Ok((&[RegClass::Int], &[I64])),
            F16 => Ok((&[RegClass::Float], &[F16])),
            F32 => Ok((&[RegClass::Float], &[F32])),
            F64 => Ok((&[RegClass::Float], &[F64])),
            F128 => Ok((&[RegClass::Float], &[F128])),
            I128 => Ok((&[RegClass::Int, RegClass::Int], &[I64, I64])),
            _ if ty.is_vector() && ty.bits() <= 128 => {
                let types = &[types::I8X2, types::I8X4, types::I8X8, types::I8X16];
                Ok((
                    &[RegClass::Float],
                    slice::from_ref(&types[ty.bytes().ilog2() as usize - 1]),
                ))
            }
            _ if ty.is_dynamic_vector() => Ok((&[RegClass::Float], &[I8X16])),
            _ => Err(CodegenError::Unsupported(format!(
                "Unexpected SSA-value type: {ty}"
            ))),
        }
    }

    fn canonical_type_for_rc(rc: RegClass) -> Type {
        match rc {
            RegClass::Float => types::I8X16,
            RegClass::Int => types::I64,
            RegClass::Vector => unreachable!(),
        }
    }

    fn gen_jump(target: MachLabel) -> Inst {
        Inst::Jump {
            dest: BranchTarget::Label(target),
        }
    }

    fn worst_case_size() -> CodeOffset {
        // The maximum size, in bytes, of any `Inst`'s emitted code. We have at least one case of
        // an 8-instruction sequence (saturating int-to-float conversions) with three embedded
        // 64-bit f64 constants.
        //
        // Note that inline jump-tables handle island/pool insertion separately, so we do not need
        // to account for them here (otherwise the worst case would be 2^31 * 4, clearly not
        // feasible for other reasons).
        44
    }

    fn ref_type_regclass(_: &settings::Flags) -> RegClass {
        RegClass::Int
    }

    fn gen_block_start(
        is_indirect_branch_target: bool,
        is_forward_edge_cfi_enabled: bool,
    ) -> Option<Self> {
        if is_indirect_branch_target && is_forward_edge_cfi_enabled {
            Some(Inst::Bti {
                targets: BranchTargetType::J,
            })
        } else {
            None
        }
    }

    fn function_alignment() -> FunctionAlignment {
        // We use 32-byte alignment for performance reasons, but for correctness
        // we would only need 4-byte alignment.
        FunctionAlignment {
            minimum: 4,
            preferred: 32,
        }
    }
}

//=============================================================================
// Pretty-printing of instructions.

fn mem_finalize_for_show(mem: &AMode, access_ty: Type, state: &EmitState) -> (String, String) {
    let (mem_insts, mem) = mem_finalize(None, mem, access_ty, state);
    let mut mem_str = mem_insts
        .into_iter()
        .map(|inst| inst.print_with_state(&mut EmitState::default()))
        .collect::<Vec<_>>()
        .join(" ; ");
    if !mem_str.is_empty() {
        mem_str += " ; ";
    }

    let mem = mem.pretty_print(access_ty.bytes() as u8);
    (mem_str, mem)
}

fn pretty_print_try_call(info: &TryCallInfo) -> String {
    format!(
        "; b {:?}; catch [{}]",
        info.continuation,
        info.pretty_print_dests()
    )
}

impl Inst {
    fn print_with_state(&self, state: &mut EmitState) -> String {
        fn op_name(alu_op: ALUOp) -> &'static str {
            match alu_op {
                ALUOp::Add => "add",
                ALUOp::Sub => "sub",
                ALUOp::Orr => "orr",
                ALUOp::And => "and",
                ALUOp::AndS => "ands",
                ALUOp::Eor => "eor",
                ALUOp::AddS => "adds",
                ALUOp::SubS => "subs",
                ALUOp::SMulH => "smulh",
                ALUOp::UMulH => "umulh",
                ALUOp::SDiv => "sdiv",
                ALUOp::UDiv => "udiv",
                ALUOp::AndNot => "bic",
                ALUOp::OrrNot => "orn",
                ALUOp::EorNot => "eon",
                ALUOp::Extr => "extr",
                ALUOp::Lsr => "lsr",
                ALUOp::Asr => "asr",
                ALUOp::Lsl => "lsl",
                ALUOp::Adc => "adc",
                ALUOp::AdcS => "adcs",
                ALUOp::Sbc => "sbc",
                ALUOp::SbcS => "sbcs",
            }
        }

        match self {
            &Inst::Nop0 => "nop-zero-len".to_string(),
            &Inst::Nop4 => "nop".to_string(),
            &Inst::AluRRR {
                alu_op,
                size,
                rd,
                rn,
                rm,
            } => {
                let op = op_name(alu_op);
                let rd = pretty_print_ireg(rd.to_reg(), size);
                let rn = pretty_print_ireg(rn, size);
                let rm = pretty_print_ireg(rm, size);
                format!("{op} {rd}, {rn}, {rm}")
            }
            &Inst::AluRRRR {
                alu_op,
                size,
                rd,
                rn,
                rm,
                ra,
            } => {
                let (op, da_size) = match alu_op {
                    ALUOp3::MAdd => ("madd", size),
                    ALUOp3::MSub => ("msub", size),
                    ALUOp3::UMAddL => ("umaddl", OperandSize::Size64),
                    ALUOp3::SMAddL => ("smaddl", OperandSize::Size64),
                };
                let rd = pretty_print_ireg(rd.to_reg(), da_size);
                let rn = pretty_print_ireg(rn, size);
                let rm = pretty_print_ireg(rm, size);
                let ra = pretty_print_ireg(ra, da_size);

                format!("{op} {rd}, {rn}, {rm}, {ra}")
            }
            &Inst::AluRRImm12 {
                alu_op,
                size,
                rd,
                rn,
                ref imm12,
            } => {
                let op = op_name(alu_op);
                let rd = pretty_print_ireg(rd.to_reg(), size);
                let rn = pretty_print_ireg(rn, size);

                if imm12.bits == 0 && alu_op == ALUOp::Add && size.is64() {
                    // special-case MOV (used for moving into SP).
                    format!("mov {rd}, {rn}")
                } else {
                    let imm12 = imm12.pretty_print(0);
                    format!("{op} {rd}, {rn}, {imm12}")
                }
            }
            &Inst::AluRRImmLogic {
                alu_op,
                size,
                rd,
                rn,
                ref imml,
            } => {
                let op = op_name(alu_op);
                let rd = pretty_print_ireg(rd.to_reg(), size);
                let rn = pretty_print_ireg(rn, size);
                let imml = imml.pretty_print(0);
                format!("{op} {rd}, {rn}, {imml}")
            }
            &Inst::AluRRImmShift {
                alu_op,
                size,
                rd,
                rn,
                ref immshift,
            } => {
                let op = op_name(alu_op);
                let rd = pretty_print_ireg(rd.to_reg(), size);
                let rn = pretty_print_ireg(rn, size);
                let immshift = immshift.pretty_print(0);
                format!("{op} {rd}, {rn}, {immshift}")
            }
            &Inst::AluRRRShift {
                alu_op,
                size,
                rd,
                rn,
                rm,
                ref shiftop,
            } => {
                let op = op_name(alu_op);
                let rd = pretty_print_ireg(rd.to_reg(), size);
                let rn = pretty_print_ireg(rn, size);
                let rm = pretty_print_ireg(rm, size);
                let shiftop = shiftop.pretty_print(0);
                format!("{op} {rd}, {rn}, {rm}, {shiftop}")
            }
            &Inst::AluRRRExtend {
                alu_op,
                size,
                rd,
                rn,
                rm,
                ref extendop,
            } => {
                let op = op_name(alu_op);
                let rd = pretty_print_ireg(rd.to_reg(), size);
                let rn = pretty_print_ireg(rn, size);
                let rm = pretty_print_ireg(rm, size);
                let extendop = extendop.pretty_print(0);
                format!("{op} {rd}, {rn}, {rm}, {extendop}")
            }
            &Inst::BitRR { op, size, rd, rn } => {
                let op = op.op_str();
                let rd = pretty_print_ireg(rd.to_reg(), size);
                let rn = pretty_print_ireg(rn, size);
                format!("{op} {rd}, {rn}")
            }
            &Inst::ULoad8 { rd, ref mem, .. }
            | &Inst::SLoad8 { rd, ref mem, .. }
            | &Inst::ULoad16 { rd, ref mem, .. }
            | &Inst::SLoad16 { rd, ref mem, .. }
            | &Inst::ULoad32 { rd, ref mem, .. }
            | &Inst::SLoad32 { rd, ref mem, .. }
            | &Inst::ULoad64 { rd, ref mem, .. } => {
                let is_unscaled = match &mem {
                    &AMode::Unscaled { .. } => true,
                    _ => false,
                };
                let (op, size) = match (self, is_unscaled) {
                    (&Inst::ULoad8 { .. }, false) => ("ldrb", OperandSize::Size32),
                    (&Inst::ULoad8 { .. }, true) => ("ldurb", OperandSize::Size32),
                    (&Inst::SLoad8 { .. }, false) => ("ldrsb", OperandSize::Size64),
                    (&Inst::SLoad8 { .. }, true) => ("ldursb", OperandSize::Size64),
                    (&Inst::ULoad16 { .. }, false) => ("ldrh", OperandSize::Size32),
                    (&Inst::ULoad16 { .. }, true) => ("ldurh", OperandSize::Size32),
                    (&Inst::SLoad16 { .. }, false) => ("ldrsh", OperandSize::Size64),
                    (&Inst::SLoad16 { .. }, true) => ("ldursh", OperandSize::Size64),
                    (&Inst::ULoad32 { .. }, false) => ("ldr", OperandSize::Size32),
                    (&Inst::ULoad32 { .. }, true) => ("ldur", OperandSize::Size32),
                    (&Inst::SLoad32 { .. }, false) => ("ldrsw", OperandSize::Size64),
                    (&Inst::SLoad32 { .. }, true) => ("ldursw", OperandSize::Size64),
                    (&Inst::ULoad64 { .. }, false) => ("ldr", OperandSize::Size64),
                    (&Inst::ULoad64 { .. }, true) => ("ldur", OperandSize::Size64),
                    _ => unreachable!(),
                };

                let rd = pretty_print_ireg(rd.to_reg(), size);
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_str, mem) = mem_finalize_for_show(&mem, access_ty, state);

                format!("{mem_str}{op} {rd}, {mem}")
            }
            &Inst::Store8 { rd, ref mem, .. }
            | &Inst::Store16 { rd, ref mem, .. }
            | &Inst::Store32 { rd, ref mem, .. }
            | &Inst::Store64 { rd, ref mem, .. } => {
                let is_unscaled = match &mem {
                    &AMode::Unscaled { .. } => true,
                    _ => false,
                };
                let (op, size) = match (self, is_unscaled) {
                    (&Inst::Store8 { .. }, false) => ("strb", OperandSize::Size32),
                    (&Inst::Store8 { .. }, true) => ("sturb", OperandSize::Size32),
                    (&Inst::Store16 { .. }, false) => ("strh", OperandSize::Size32),
                    (&Inst::Store16 { .. }, true) => ("sturh", OperandSize::Size32),
                    (&Inst::Store32 { .. }, false) => ("str", OperandSize::Size32),
                    (&Inst::Store32 { .. }, true) => ("stur", OperandSize::Size32),
                    (&Inst::Store64 { .. }, false) => ("str", OperandSize::Size64),
                    (&Inst::Store64 { .. }, true) => ("stur", OperandSize::Size64),
                    _ => unreachable!(),
                };

                let rd = pretty_print_ireg(rd, size);
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_str, mem) = mem_finalize_for_show(&mem, access_ty, state);

                format!("{mem_str}{op} {rd}, {mem}")
            }
            &Inst::StoreP64 {
                rt, rt2, ref mem, ..
            } => {
                let rt = pretty_print_ireg(rt, OperandSize::Size64);
                let rt2 = pretty_print_ireg(rt2, OperandSize::Size64);
                let mem = mem.clone();
                let mem = mem.pretty_print_default();
                format!("stp {rt}, {rt2}, {mem}")
            }
            &Inst::LoadP64 {
                rt, rt2, ref mem, ..
            } => {
                let rt = pretty_print_ireg(rt.to_reg(), OperandSize::Size64);
                let rt2 = pretty_print_ireg(rt2.to_reg(), OperandSize::Size64);
                let mem = mem.clone();
                let mem = mem.pretty_print_default();
                format!("ldp {rt}, {rt2}, {mem}")
            }
            &Inst::Mov { size, rd, rm } => {
                let rd = pretty_print_ireg(rd.to_reg(), size);
                let rm = pretty_print_ireg(rm, size);
                format!("mov {rd}, {rm}")
            }
            &Inst::MovFromPReg { rd, rm } => {
                let rd = pretty_print_ireg(rd.to_reg(), OperandSize::Size64);
                let rm = show_ireg_sized(rm.into(), OperandSize::Size64);
                format!("mov {rd}, {rm}")
            }
            &Inst::MovToPReg { rd, rm } => {
                let rd = show_ireg_sized(rd.into(), OperandSize::Size64);
                let rm = pretty_print_ireg(rm, OperandSize::Size64);
                format!("mov {rd}, {rm}")
            }
            &Inst::MovWide {
                op,
                rd,
                ref imm,
                size,
            } => {
                let op_str = match op {
                    MoveWideOp::MovZ => "movz",
                    MoveWideOp::MovN => "movn",
                };
                let rd = pretty_print_ireg(rd.to_reg(), size);
                let imm = imm.pretty_print(0);
                format!("{op_str} {rd}, {imm}")
            }
            &Inst::MovK {
                rd,
                rn,
                ref imm,
                size,
            } => {
                let rn = pretty_print_ireg(rn, size);
                let rd = pretty_print_ireg(rd.to_reg(), size);
                let imm = imm.pretty_print(0);
                format!("movk {rd}, {rn}, {imm}")
            }
            &Inst::CSel { rd, rn, rm, cond } => {
                let rd = pretty_print_ireg(rd.to_reg(), OperandSize::Size64);
                let rn = pretty_print_ireg(rn, OperandSize::Size64);
                let rm = pretty_print_ireg(rm, OperandSize::Size64);
                let cond = cond.pretty_print(0);
                format!("csel {rd}, {rn}, {rm}, {cond}")
            }
            &Inst::CSNeg { rd, rn, rm, cond } => {
                let rd = pretty_print_ireg(rd.to_reg(), OperandSize::Size64);
                let rn = pretty_print_ireg(rn, OperandSize::Size64);
                let rm = pretty_print_ireg(rm, OperandSize::Size64);
                let cond = cond.pretty_print(0);
                format!("csneg {rd}, {rn}, {rm}, {cond}")
            }
            &Inst::CSet { rd, cond } => {
                let rd = pretty_print_ireg(rd.to_reg(), OperandSize::Size64);
                let cond = cond.pretty_print(0);
                format!("cset {rd}, {cond}")
            }
            &Inst::CSetm { rd, cond } => {
                let rd = pretty_print_ireg(rd.to_reg(), OperandSize::Size64);
                let cond = cond.pretty_print(0);
                format!("csetm {rd}, {cond}")
            }
            &Inst::CCmp {
                size,
                rn,
                rm,
                nzcv,
                cond,
            } => {
                let rn = pretty_print_ireg(rn, size);
                let rm = pretty_print_ireg(rm, size);
                let nzcv = nzcv.pretty_print(0);
                let cond = cond.pretty_print(0);
                format!("ccmp {rn}, {rm}, {nzcv}, {cond}")
            }
            &Inst::CCmpImm {
                size,
                rn,
                imm,
                nzcv,
                cond,
            } => {
                let rn = pretty_print_ireg(rn, size);
                let imm = imm.pretty_print(0);
                let nzcv = nzcv.pretty_print(0);
                let cond = cond.pretty_print(0);
                format!("ccmp {rn}, {imm}, {nzcv}, {cond}")
            }
            &Inst::AtomicRMW {
                rs, rt, rn, ty, op, ..
            } => {
                let op = match op {
                    AtomicRMWOp::Add => "ldaddal",
                    AtomicRMWOp::Clr => "ldclral",
                    AtomicRMWOp::Eor => "ldeoral",
                    AtomicRMWOp::Set => "ldsetal",
                    AtomicRMWOp::Smax => "ldsmaxal",
                    AtomicRMWOp::Umax => "ldumaxal",
                    AtomicRMWOp::Smin => "ldsminal",
                    AtomicRMWOp::Umin => "lduminal",
                    AtomicRMWOp::Swp => "swpal",
                };

                let size = OperandSize::from_ty(ty);
                let rs = pretty_print_ireg(rs, size);
                let rt = pretty_print_ireg(rt.to_reg(), size);
                let rn = pretty_print_ireg(rn, OperandSize::Size64);

                let ty_suffix = match ty {
                    I8 => "b",
                    I16 => "h",
                    _ => "",
                };
                format!("{op}{ty_suffix} {rs}, {rt}, [{rn}]")
            }
            &Inst::AtomicRMWLoop {
                ty,
                op,
                addr,
                operand,
                oldval,
                scratch1,
                scratch2,
                ..
            } => {
                let op = match op {
                    AtomicRMWLoopOp::Add => "add",
                    AtomicRMWLoopOp::Sub => "sub",
                    AtomicRMWLoopOp::Eor => "eor",
                    AtomicRMWLoopOp::Orr => "orr",
                    AtomicRMWLoopOp::And => "and",
                    AtomicRMWLoopOp::Nand => "nand",
                    AtomicRMWLoopOp::Smin => "smin",
                    AtomicRMWLoopOp::Smax => "smax",
                    AtomicRMWLoopOp::Umin => "umin",
                    AtomicRMWLoopOp::Umax => "umax",
                    AtomicRMWLoopOp::Xchg => "xchg",
                };
                let addr = pretty_print_ireg(addr, OperandSize::Size64);
                let operand = pretty_print_ireg(operand, OperandSize::Size64);
                let oldval = pretty_print_ireg(oldval.to_reg(), OperandSize::Size64);
                let scratch1 = pretty_print_ireg(scratch1.to_reg(), OperandSize::Size64);
                let scratch2 = pretty_print_ireg(scratch2.to_reg(), OperandSize::Size64);
                format!(
                    "atomic_rmw_loop_{}_{} addr={} operand={} oldval={} scratch1={} scratch2={}",
                    op,
                    ty.bits(),
                    addr,
                    operand,
                    oldval,
                    scratch1,
                    scratch2,
                )
            }
            &Inst::AtomicCAS {
                rd, rs, rt, rn, ty, ..
            } => {
                let op = match ty {
                    I8 => "casalb",
                    I16 => "casalh",
                    I32 | I64 => "casal",
                    _ => panic!("Unsupported type: {ty}"),
                };
                let size = OperandSize::from_ty(ty);
                let rd = pretty_print_ireg(rd.to_reg(), size);
                let rs = pretty_print_ireg(rs, size);
                let rt = pretty_print_ireg(rt, size);
                let rn = pretty_print_ireg(rn, OperandSize::Size64);

                format!("{op} {rd}, {rs}, {rt}, [{rn}]")
            }
            &Inst::AtomicCASLoop {
                ty,
                addr,
                expected,
                replacement,
                oldval,
                scratch,
                ..
            } => {
                let addr = pretty_print_ireg(addr, OperandSize::Size64);
                let expected = pretty_print_ireg(expected, OperandSize::Size64);
                let replacement = pretty_print_ireg(replacement, OperandSize::Size64);
                let oldval = pretty_print_ireg(oldval.to_reg(), OperandSize::Size64);
                let scratch = pretty_print_ireg(scratch.to_reg(), OperandSize::Size64);
                format!(
                    "atomic_cas_loop_{} addr={}, expect={}, replacement={}, oldval={}, scratch={}",
                    ty.bits(),
                    addr,
                    expected,
                    replacement,
                    oldval,
                    scratch,
                )
            }
            &Inst::LoadAcquire {
                access_ty, rt, rn, ..
            } => {
                let (op, ty) = match access_ty {
                    I8 => ("ldarb", I32),
                    I16 => ("ldarh", I32),
                    I32 => ("ldar", I32),
                    I64 => ("ldar", I64),
                    _ => panic!("Unsupported type: {access_ty}"),
                };
                let size = OperandSize::from_ty(ty);
                let rn = pretty_print_ireg(rn, OperandSize::Size64);
                let rt = pretty_print_ireg(rt.to_reg(), size);
                format!("{op} {rt}, [{rn}]")
            }
            &Inst::StoreRelease {
                access_ty, rt, rn, ..
            } => {
                let (op, ty) = match access_ty {
                    I8 => ("stlrb", I32),
                    I16 => ("stlrh", I32),
                    I32 => ("stlr", I32),
                    I64 => ("stlr", I64),
                    _ => panic!("Unsupported type: {access_ty}"),
                };
                let size = OperandSize::from_ty(ty);
                let rn = pretty_print_ireg(rn, OperandSize::Size64);
                let rt = pretty_print_ireg(rt, size);
                format!("{op} {rt}, [{rn}]")
            }
            &Inst::Fence {} => {
                format!("dmb ish")
            }
            &Inst::Csdb {} => {
                format!("csdb")
            }
            &Inst::FpuMove32 { rd, rn } => {
                let rd = pretty_print_vreg_scalar(rd.to_reg(), ScalarSize::Size32);
                let rn = pretty_print_vreg_scalar(rn, ScalarSize::Size32);
                format!("fmov {rd}, {rn}")
            }
            &Inst::FpuMove64 { rd, rn } => {
                let rd = pretty_print_vreg_scalar(rd.to_reg(), ScalarSize::Size64);
                let rn = pretty_print_vreg_scalar(rn, ScalarSize::Size64);
                format!("fmov {rd}, {rn}")
            }
            &Inst::FpuMove128 { rd, rn } => {
                let rd = pretty_print_reg(rd.to_reg());
                let rn = pretty_print_reg(rn);
                format!("mov {rd}.16b, {rn}.16b")
            }
            &Inst::FpuMoveFromVec { rd, rn, idx, size } => {
                let rd = pretty_print_vreg_scalar(rd.to_reg(), size.lane_size());
                let rn = pretty_print_vreg_element(rn, idx as usize, size.lane_size());
                format!("mov {rd}, {rn}")
            }
            &Inst::FpuExtend { rd, rn, size } => {
                let rd = pretty_print_vreg_scalar(rd.to_reg(), size);
                let rn = pretty_print_vreg_scalar(rn, size);
                format!("fmov {rd}, {rn}")
            }
            &Inst::FpuRR {
                fpu_op,
                size,
                rd,
                rn,
            } => {
                let op = match fpu_op {
                    FPUOp1::Abs => "fabs",
                    FPUOp1::Neg => "fneg",
                    FPUOp1::Sqrt => "fsqrt",
                    FPUOp1::Cvt32To64 | FPUOp1::Cvt64To32 => "fcvt",
                };
                let dst_size = match fpu_op {
                    FPUOp1::Cvt32To64 => ScalarSize::Size64,
                    FPUOp1::Cvt64To32 => ScalarSize::Size32,
                    _ => size,
                };
                let rd = pretty_print_vreg_scalar(rd.to_reg(), dst_size);
                let rn = pretty_print_vreg_scalar(rn, size);
                format!("{op} {rd}, {rn}")
            }
            &Inst::FpuRRR {
                fpu_op,
                size,
                rd,
                rn,
                rm,
            } => {
                let op = match fpu_op {
                    FPUOp2::Add => "fadd",
                    FPUOp2::Sub => "fsub",
                    FPUOp2::Mul => "fmul",
                    FPUOp2::Div => "fdiv",
                    FPUOp2::Max => "fmax",
                    FPUOp2::Min => "fmin",
                };
                let rd = pretty_print_vreg_scalar(rd.to_reg(), size);
                let rn = pretty_print_vreg_scalar(rn, size);
                let rm = pretty_print_vreg_scalar(rm, size);
                format!("{op} {rd}, {rn}, {rm}")
            }
            &Inst::FpuRRI { fpu_op, rd, rn } => {
                let (op, imm, vector) = match fpu_op {
                    FPUOpRI::UShr32(imm) => ("ushr", imm.pretty_print(0), true),
                    FPUOpRI::UShr64(imm) => ("ushr", imm.pretty_print(0), false),
                };

                let (rd, rn) = if vector {
                    (
                        pretty_print_vreg_vector(rd.to_reg(), VectorSize::Size32x2),
                        pretty_print_vreg_vector(rn, VectorSize::Size32x2),
                    )
                } else {
                    (
                        pretty_print_vreg_scalar(rd.to_reg(), ScalarSize::Size64),
                        pretty_print_vreg_scalar(rn, ScalarSize::Size64),
                    )
                };
                format!("{op} {rd}, {rn}, {imm}")
            }
            &Inst::FpuRRIMod { fpu_op, rd, ri, rn } => {
                let (op, imm, vector) = match fpu_op {
                    FPUOpRIMod::Sli32(imm) => ("sli", imm.pretty_print(0), true),
                    FPUOpRIMod::Sli64(imm) => ("sli", imm.pretty_print(0), false),
                };

                let (rd, ri, rn) = if vector {
                    (
                        pretty_print_vreg_vector(rd.to_reg(), VectorSize::Size32x2),
                        pretty_print_vreg_vector(ri, VectorSize::Size32x2),
                        pretty_print_vreg_vector(rn, VectorSize::Size32x2),
                    )
                } else {
                    (
                        pretty_print_vreg_scalar(rd.to_reg(), ScalarSize::Size64),
                        pretty_print_vreg_scalar(ri, ScalarSize::Size64),
                        pretty_print_vreg_scalar(rn, ScalarSize::Size64),
                    )
                };
                format!("{op} {rd}, {ri}, {rn}, {imm}")
            }
            &Inst::FpuRRRR {
                fpu_op,
                size,
                rd,
                rn,
                rm,
                ra,
            } => {
                let op = match fpu_op {
                    FPUOp3::MAdd => "fmadd",
                    FPUOp3::MSub => "fmsub",
                    FPUOp3::NMAdd => "fnmadd",
                    FPUOp3::NMSub => "fnmsub",
                };
                let rd = pretty_print_vreg_scalar(rd.to_reg(), size);
                let rn = pretty_print_vreg_scalar(rn, size);
                let rm = pretty_print_vreg_scalar(rm, size);
                let ra = pretty_print_vreg_scalar(ra, size);
                format!("{op} {rd}, {rn}, {rm}, {ra}")
            }
            &Inst::FpuCmp { size, rn, rm } => {
                let rn = pretty_print_vreg_scalar(rn, size);
                let rm = pretty_print_vreg_scalar(rm, size);
                format!("fcmp {rn}, {rm}")
            }
            &Inst::FpuLoad16 { rd, ref mem, .. } => {
                let rd = pretty_print_vreg_scalar(rd.to_reg(), ScalarSize::Size16);
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_str, mem) = mem_finalize_for_show(&mem, access_ty, state);
                format!("{mem_str}ldr {rd}, {mem}")
            }
            &Inst::FpuLoad32 { rd, ref mem, .. } => {
                let rd = pretty_print_vreg_scalar(rd.to_reg(), ScalarSize::Size32);
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_str, mem) = mem_finalize_for_show(&mem, access_ty, state);
                format!("{mem_str}ldr {rd}, {mem}")
            }
            &Inst::FpuLoad64 { rd, ref mem, .. } => {
                let rd = pretty_print_vreg_scalar(rd.to_reg(), ScalarSize::Size64);
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_str, mem) = mem_finalize_for_show(&mem, access_ty, state);
                format!("{mem_str}ldr {rd}, {mem}")
            }
            &Inst::FpuLoad128 { rd, ref mem, .. } => {
                let rd = pretty_print_reg(rd.to_reg());
                let rd = "q".to_string() + &rd[1..];
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_str, mem) = mem_finalize_for_show(&mem, access_ty, state);
                format!("{mem_str}ldr {rd}, {mem}")
            }
            &Inst::FpuStore16 { rd, ref mem, .. } => {
                let rd = pretty_print_vreg_scalar(rd, ScalarSize::Size16);
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_str, mem) = mem_finalize_for_show(&mem, access_ty, state);
                format!("{mem_str}str {rd}, {mem}")
            }
            &Inst::FpuStore32 { rd, ref mem, .. } => {
                let rd = pretty_print_vreg_scalar(rd, ScalarSize::Size32);
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_str, mem) = mem_finalize_for_show(&mem, access_ty, state);
                format!("{mem_str}str {rd}, {mem}")
            }
            &Inst::FpuStore64 { rd, ref mem, .. } => {
                let rd = pretty_print_vreg_scalar(rd, ScalarSize::Size64);
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_str, mem) = mem_finalize_for_show(&mem, access_ty, state);
                format!("{mem_str}str {rd}, {mem}")
            }
            &Inst::FpuStore128 { rd, ref mem, .. } => {
                let rd = pretty_print_reg(rd);
                let rd = "q".to_string() + &rd[1..];
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_str, mem) = mem_finalize_for_show(&mem, access_ty, state);
                format!("{mem_str}str {rd}, {mem}")
            }
            &Inst::FpuLoadP64 {
                rt, rt2, ref mem, ..
            } => {
                let rt = pretty_print_vreg_scalar(rt.to_reg(), ScalarSize::Size64);
                let rt2 = pretty_print_vreg_scalar(rt2.to_reg(), ScalarSize::Size64);
                let mem = mem.clone();
                let mem = mem.pretty_print_default();

                format!("ldp {rt}, {rt2}, {mem}")
            }
            &Inst::FpuStoreP64 {
                rt, rt2, ref mem, ..
            } => {
                let rt = pretty_print_vreg_scalar(rt, ScalarSize::Size64);
                let rt2 = pretty_print_vreg_scalar(rt2, ScalarSize::Size64);
                let mem = mem.clone();
                let mem = mem.pretty_print_default();

                format!("stp {rt}, {rt2}, {mem}")
            }
            &Inst::FpuLoadP128 {
                rt, rt2, ref mem, ..
            } => {
                let rt = pretty_print_vreg_scalar(rt.to_reg(), ScalarSize::Size128);
                let rt2 = pretty_print_vreg_scalar(rt2.to_reg(), ScalarSize::Size128);
                let mem = mem.clone();
                let mem = mem.pretty_print_default();

                format!("ldp {rt}, {rt2}, {mem}")
            }
            &Inst::FpuStoreP128 {
                rt, rt2, ref mem, ..
            } => {
                let rt = pretty_print_vreg_scalar(rt, ScalarSize::Size128);
                let rt2 = pretty_print_vreg_scalar(rt2, ScalarSize::Size128);
                let mem = mem.clone();
                let mem = mem.pretty_print_default();

                format!("stp {rt}, {rt2}, {mem}")
            }
            &Inst::FpuToInt { op, rd, rn } => {
                let (op, sizesrc, sizedest) = match op {
                    FpuToIntOp::F32ToI32 => ("fcvtzs", ScalarSize::Size32, OperandSize::Size32),
                    FpuToIntOp::F32ToU32 => ("fcvtzu", ScalarSize::Size32, OperandSize::Size32),
                    FpuToIntOp::F32ToI64 => ("fcvtzs", ScalarSize::Size32, OperandSize::Size64),
                    FpuToIntOp::F32ToU64 => ("fcvtzu", ScalarSize::Size32, OperandSize::Size64),
                    FpuToIntOp::F64ToI32 => ("fcvtzs", ScalarSize::Size64, OperandSize::Size32),
                    FpuToIntOp::F64ToU32 => ("fcvtzu", ScalarSize::Size64, OperandSize::Size32),
                    FpuToIntOp::F64ToI64 => ("fcvtzs", ScalarSize::Size64, OperandSize::Size64),
                    FpuToIntOp::F64ToU64 => ("fcvtzu", ScalarSize::Size64, OperandSize::Size64),
                };
                let rd = pretty_print_ireg(rd.to_reg(), sizedest);
                let rn = pretty_print_vreg_scalar(rn, sizesrc);
                format!("{op} {rd}, {rn}")
            }
            &Inst::IntToFpu { op, rd, rn } => {
                let (op, sizesrc, sizedest) = match op {
                    IntToFpuOp::I32ToF32 => ("scvtf", OperandSize::Size32, ScalarSize::Size32),
                    IntToFpuOp::U32ToF32 => ("ucvtf", OperandSize::Size32, ScalarSize::Size32),
                    IntToFpuOp::I64ToF32 => ("scvtf", OperandSize::Size64, ScalarSize::Size32),
                    IntToFpuOp::U64ToF32 => ("ucvtf", OperandSize::Size64, ScalarSize::Size32),
                    IntToFpuOp::I32ToF64 => ("scvtf", OperandSize::Size32, ScalarSize::Size64),
                    IntToFpuOp::U32ToF64 => ("ucvtf", OperandSize::Size32, ScalarSize::Size64),
                    IntToFpuOp::I64ToF64 => ("scvtf", OperandSize::Size64, ScalarSize::Size64),
                    IntToFpuOp::U64ToF64 => ("ucvtf", OperandSize::Size64, ScalarSize::Size64),
                };
                let rd = pretty_print_vreg_scalar(rd.to_reg(), sizedest);
                let rn = pretty_print_ireg(rn, sizesrc);
                format!("{op} {rd}, {rn}")
            }
            &Inst::FpuCSel16 { rd, rn, rm, cond } => {
                let rd = pretty_print_vreg_scalar(rd.to_reg(), ScalarSize::Size16);
                let rn = pretty_print_vreg_scalar(rn, ScalarSize::Size16);
                let rm = pretty_print_vreg_scalar(rm, ScalarSize::Size16);
                let cond = cond.pretty_print(0);
                format!("fcsel {rd}, {rn}, {rm}, {cond}")
            }
            &Inst::FpuCSel32 { rd, rn, rm, cond } => {
                let rd = pretty_print_vreg_scalar(rd.to_reg(), ScalarSize::Size32);
                let rn = pretty_print_vreg_scalar(rn, ScalarSize::Size32);
                let rm = pretty_print_vreg_scalar(rm, ScalarSize::Size32);
                let cond = cond.pretty_print(0);
                format!("fcsel {rd}, {rn}, {rm}, {cond}")
            }
            &Inst::FpuCSel64 { rd, rn, rm, cond } => {
                let rd = pretty_print_vreg_scalar(rd.to_reg(), ScalarSize::Size64);
                let rn = pretty_print_vreg_scalar(rn, ScalarSize::Size64);
                let rm = pretty_print_vreg_scalar(rm, ScalarSize::Size64);
                let cond = cond.pretty_print(0);
                format!("fcsel {rd}, {rn}, {rm}, {cond}")
            }
            &Inst::FpuRound { op, rd, rn } => {
                let (inst, size) = match op {
                    FpuRoundMode::Minus32 => ("frintm", ScalarSize::Size32),
                    FpuRoundMode::Minus64 => ("frintm", ScalarSize::Size64),
                    FpuRoundMode::Plus32 => ("frintp", ScalarSize::Size32),
                    FpuRoundMode::Plus64 => ("frintp", ScalarSize::Size64),
                    FpuRoundMode::Zero32 => ("frintz", ScalarSize::Size32),
                    FpuRoundMode::Zero64 => ("frintz", ScalarSize::Size64),
                    FpuRoundMode::Nearest32 => ("frintn", ScalarSize::Size32),
                    FpuRoundMode::Nearest64 => ("frintn", ScalarSize::Size64),
                };
                let rd = pretty_print_vreg_scalar(rd.to_reg(), size);
                let rn = pretty_print_vreg_scalar(rn, size);
                format!("{inst} {rd}, {rn}")
            }
            &Inst::MovToFpu { rd, rn, size } => {
                let operand_size = size.operand_size();
                let rd = pretty_print_vreg_scalar(rd.to_reg(), size);
                let rn = pretty_print_ireg(rn, operand_size);
                format!("fmov {rd}, {rn}")
            }
            &Inst::FpuMoveFPImm { rd, imm, size } => {
                let imm = imm.pretty_print(0);
                let rd = pretty_print_vreg_scalar(rd.to_reg(), size);

                format!("fmov {rd}, {imm}")
            }
            &Inst::MovToVec {
                rd,
                ri,
                rn,
                idx,
                size,
            } => {
                let rd = pretty_print_vreg_element(rd.to_reg(), idx as usize, size.lane_size());
                let ri = pretty_print_vreg_element(ri, idx as usize, size.lane_size());
                let rn = pretty_print_ireg(rn, size.operand_size());
                format!("mov {rd}, {ri}, {rn}")
            }
            &Inst::MovFromVec { rd, rn, idx, size } => {
                let op = match size {
                    ScalarSize::Size8 => "umov",
                    ScalarSize::Size16 => "umov",
                    ScalarSize::Size32 => "mov",
                    ScalarSize::Size64 => "mov",
                    _ => unimplemented!(),
                };
                let rd = pretty_print_ireg(rd.to_reg(), size.operand_size());
                let rn = pretty_print_vreg_element(rn, idx as usize, size);
                format!("{op} {rd}, {rn}")
            }
            &Inst::MovFromVecSigned {
                rd,
                rn,
                idx,
                size,
                scalar_size,
            } => {
                let rd = pretty_print_ireg(rd.to_reg(), scalar_size);
                let rn = pretty_print_vreg_element(rn, idx as usize, size.lane_size());
                format!("smov {rd}, {rn}")
            }
            &Inst::VecDup { rd, rn, size } => {
                let rd = pretty_print_vreg_vector(rd.to_reg(), size);
                let rn = pretty_print_ireg(rn, size.operand_size());
                format!("dup {rd}, {rn}")
            }
            &Inst::VecDupFromFpu { rd, rn, size, lane } => {
                let rd = pretty_print_vreg_vector(rd.to_reg(), size);
                let rn = pretty_print_vreg_element(rn, lane.into(), size.lane_size());
                format!("dup {rd}, {rn}")
            }
            &Inst::VecDupFPImm { rd, imm, size } => {
                let imm = imm.pretty_print(0);
                let rd = pretty_print_vreg_vector(rd.to_reg(), size);

                format!("fmov {rd}, {imm}")
            }
            &Inst::VecDupImm {
                rd,
                imm,
                invert,
                size,
            } => {
                let imm = imm.pretty_print(0);
                let op = if invert { "mvni" } else { "movi" };
                let rd = pretty_print_vreg_vector(rd.to_reg(), size);

                format!("{op} {rd}, {imm}")
            }
            &Inst::VecExtend {
                t,
                rd,
                rn,
                high_half,
                lane_size,
            } => {
                let vec64 = VectorSize::from_lane_size(lane_size.narrow(), false);
                let vec128 = VectorSize::from_lane_size(lane_size.narrow(), true);
                let rd_size = VectorSize::from_lane_size(lane_size, true);
                let (op, rn_size) = match (t, high_half) {
                    (VecExtendOp::Sxtl, false) => ("sxtl", vec64),
                    (VecExtendOp::Sxtl, true) => ("sxtl2", vec128),
                    (VecExtendOp::Uxtl, false) => ("uxtl", vec64),
                    (VecExtendOp::Uxtl, true) => ("uxtl2", vec128),
                };
                let rd = pretty_print_vreg_vector(rd.to_reg(), rd_size);
                let rn = pretty_print_vreg_vector(rn, rn_size);
                format!("{op} {rd}, {rn}")
            }
            &Inst::VecMovElement {
                rd,
                ri,
                rn,
                dest_idx,
                src_idx,
                size,
            } => {
                let rd =
                    pretty_print_vreg_element(rd.to_reg(), dest_idx as usize, size.lane_size());
                let ri = pretty_print_vreg_element(ri, dest_idx as usize, size.lane_size());
                let rn = pretty_print_vreg_element(rn, src_idx as usize, size.lane_size());
                format!("mov {rd}, {ri}, {rn}")
            }
            &Inst::VecRRLong {
                op,
                rd,
                rn,
                high_half,
            } => {
                let (op, rd_size, size, suffix) = match (op, high_half) {
                    (VecRRLongOp::Fcvtl16, false) => {
                        ("fcvtl", VectorSize::Size32x4, VectorSize::Size16x4, "")
                    }
                    (VecRRLongOp::Fcvtl16, true) => {
                        ("fcvtl2", VectorSize::Size32x4, VectorSize::Size16x8, "")
                    }
                    (VecRRLongOp::Fcvtl32, false) => {
                        ("fcvtl", VectorSize::Size64x2, VectorSize::Size32x2, "")
                    }
                    (VecRRLongOp::Fcvtl32, true) => {
                        ("fcvtl2", VectorSize::Size64x2, VectorSize::Size32x4, "")
                    }
                    (VecRRLongOp::Shll8, false) => {
                        ("shll", VectorSize::Size16x8, VectorSize::Size8x8, ", #8")
                    }
                    (VecRRLongOp::Shll8, true) => {
                        ("shll2", VectorSize::Size16x8, VectorSize::Size8x16, ", #8")
                    }
                    (VecRRLongOp::Shll16, false) => {
                        ("shll", VectorSize::Size32x4, VectorSize::Size16x4, ", #16")
                    }
                    (VecRRLongOp::Shll16, true) => {
                        ("shll2", VectorSize::Size32x4, VectorSize::Size16x8, ", #16")
                    }
                    (VecRRLongOp::Shll32, false) => {
                        ("shll", VectorSize::Size64x2, VectorSize::Size32x2, ", #32")
                    }
                    (VecRRLongOp::Shll32, true) => {
                        ("shll2", VectorSize::Size64x2, VectorSize::Size32x4, ", #32")
                    }
                };
                let rd = pretty_print_vreg_vector(rd.to_reg(), rd_size);
                let rn = pretty_print_vreg_vector(rn, size);

                format!("{op} {rd}, {rn}{suffix}")
            }
            &Inst::VecRRNarrowLow {
                op,
                rd,
                rn,
                lane_size,
                ..
            }
            | &Inst::VecRRNarrowHigh {
                op,
                rd,
                rn,
                lane_size,
                ..
            } => {
                let vec64 = VectorSize::from_lane_size(lane_size, false);
                let vec128 = VectorSize::from_lane_size(lane_size, true);
                let rn_size = VectorSize::from_lane_size(lane_size.widen(), true);
                let high_half = match self {
                    &Inst::VecRRNarrowLow { .. } => false,
                    &Inst::VecRRNarrowHigh { .. } => true,
                    _ => unreachable!(),
                };
                let (op, rd_size) = match (op, high_half) {
                    (VecRRNarrowOp::Xtn, false) => ("xtn", vec64),
                    (VecRRNarrowOp::Xtn, true) => ("xtn2", vec128),
                    (VecRRNarrowOp::Sqxtn, false) => ("sqxtn", vec64),
                    (VecRRNarrowOp::Sqxtn, true) => ("sqxtn2", vec128),
                    (VecRRNarrowOp::Sqxtun, false) => ("sqxtun", vec64),
                    (VecRRNarrowOp::Sqxtun, true) => ("sqxtun2", vec128),
                    (VecRRNarrowOp::Uqxtn, false) => ("uqxtn", vec64),
                    (VecRRNarrowOp::Uqxtn, true) => ("uqxtn2", vec128),
                    (VecRRNarrowOp::Fcvtn, false) => ("fcvtn", vec64),
                    (VecRRNarrowOp::Fcvtn, true) => ("fcvtn2", vec128),
                };
                let rn = pretty_print_vreg_vector(rn, rn_size);
                let rd = pretty_print_vreg_vector(rd.to_reg(), rd_size);
                let ri = match self {
                    &Inst::VecRRNarrowLow { .. } => "".to_string(),
                    &Inst::VecRRNarrowHigh { ri, .. } => {
                        format!("{}, ", pretty_print_vreg_vector(ri, rd_size))
                    }
                    _ => unreachable!(),
                };

                format!("{op} {rd}, {ri}{rn}")
            }
            &Inst::VecRRPair { op, rd, rn } => {
                let op = match op {
                    VecPairOp::Addp => "addp",
                };
                let rd = pretty_print_vreg_scalar(rd.to_reg(), ScalarSize::Size64);
                let rn = pretty_print_vreg_vector(rn, VectorSize::Size64x2);

                format!("{op} {rd}, {rn}")
            }
            &Inst::VecRRPairLong { op, rd, rn } => {
                let (op, dest, src) = match op {
                    VecRRPairLongOp::Saddlp8 => {
                        ("saddlp", VectorSize::Size16x8, VectorSize::Size8x16)
                    }
                    VecRRPairLongOp::Saddlp16 => {
                        ("saddlp", VectorSize::Size32x4, VectorSize::Size16x8)
                    }
                    VecRRPairLongOp::Uaddlp8 => {
                        ("uaddlp", VectorSize::Size16x8, VectorSize::Size8x16)
                    }
                    VecRRPairLongOp::Uaddlp16 => {
                        ("uaddlp", VectorSize::Size32x4, VectorSize::Size16x8)
                    }
                };
                let rd = pretty_print_vreg_vector(rd.to_reg(), dest);
                let rn = pretty_print_vreg_vector(rn, src);

                format!("{op} {rd}, {rn}")
            }
            &Inst::VecRRR {
                rd,
                rn,
                rm,
                alu_op,
                size,
            } => {
                let (op, size) = match alu_op {
                    VecALUOp::Sqadd => ("sqadd", size),
                    VecALUOp::Uqadd => ("uqadd", size),
                    VecALUOp::Sqsub => ("sqsub", size),
                    VecALUOp::Uqsub => ("uqsub", size),
                    VecALUOp::Cmeq => ("cmeq", size),
                    VecALUOp::Cmge => ("cmge", size),
                    VecALUOp::Cmgt => ("cmgt", size),
                    VecALUOp::Cmhs => ("cmhs", size),
                    VecALUOp::Cmhi => ("cmhi", size),
                    VecALUOp::Fcmeq => ("fcmeq", size),
                    VecALUOp::Fcmgt => ("fcmgt", size),
                    VecALUOp::Fcmge => ("fcmge", size),
                    VecALUOp::And => ("and", VectorSize::Size8x16),
                    VecALUOp::Bic => ("bic", VectorSize::Size8x16),
                    VecALUOp::Orr => ("orr", VectorSize::Size8x16),
                    VecALUOp::Eor => ("eor", VectorSize::Size8x16),
                    VecALUOp::Umaxp => ("umaxp", size),
                    VecALUOp::Add => ("add", size),
                    VecALUOp::Sub => ("sub", size),
                    VecALUOp::Mul => ("mul", size),
                    VecALUOp::Sshl => ("sshl", size),
                    VecALUOp::Ushl => ("ushl", size),
                    VecALUOp::Umin => ("umin", size),
                    VecALUOp::Smin => ("smin", size),
                    VecALUOp::Umax => ("umax", size),
                    VecALUOp::Smax => ("smax", size),
                    VecALUOp::Urhadd => ("urhadd", size),
                    VecALUOp::Fadd => ("fadd", size),
                    VecALUOp::Fsub => ("fsub", size),
                    VecALUOp::Fdiv => ("fdiv", size),
                    VecALUOp::Fmax => ("fmax", size),
                    VecALUOp::Fmin => ("fmin", size),
                    VecALUOp::Fmul => ("fmul", size),
                    VecALUOp::Addp => ("addp", size),
                    VecALUOp::Zip1 => ("zip1", size),
                    VecALUOp::Zip2 => ("zip2", size),
                    VecALUOp::Sqrdmulh => ("sqrdmulh", size),
                    VecALUOp::Uzp1 => ("uzp1", size),
                    VecALUOp::Uzp2 => ("uzp2", size),
                    VecALUOp::Trn1 => ("trn1", size),
                    VecALUOp::Trn2 => ("trn2", size),
                };
                let rd = pretty_print_vreg_vector(rd.to_reg(), size);
                let rn = pretty_print_vreg_vector(rn, size);
                let rm = pretty_print_vreg_vector(rm, size);
                format!("{op} {rd}, {rn}, {rm}")
            }
            &Inst::VecRRRMod {
                rd,
                ri,
                rn,
                rm,
                alu_op,
                size,
            } => {
                let (op, size) = match alu_op {
                    VecALUModOp::Bsl => ("bsl", VectorSize::Size8x16),
                    VecALUModOp::Fmla => ("fmla", size),
                    VecALUModOp::Fmls => ("fmls", size),
                };
                let rd = pretty_print_vreg_vector(rd.to_reg(), size);
                let ri = pretty_print_vreg_vector(ri, size);
                let rn = pretty_print_vreg_vector(rn, size);
                let rm = pretty_print_vreg_vector(rm, size);
                format!("{op} {rd}, {ri}, {rn}, {rm}")
            }
            &Inst::VecFmlaElem {
                rd,
                ri,
                rn,
                rm,
                alu_op,
                size,
                idx,
            } => {
                let (op, size) = match alu_op {
                    VecALUModOp::Fmla => ("fmla", size),
                    VecALUModOp::Fmls => ("fmls", size),
                    _ => unreachable!(),
                };
                let rd = pretty_print_vreg_vector(rd.to_reg(), size);
                let ri = pretty_print_vreg_vector(ri, size);
                let rn = pretty_print_vreg_vector(rn, size);
                let rm = pretty_print_vreg_element(rm, idx.into(), size.lane_size());
                format!("{op} {rd}, {ri}, {rn}, {rm}")
            }
            &Inst::VecRRRLong {
                rd,
                rn,
                rm,
                alu_op,
                high_half,
            } => {
                let (op, dest_size, src_size) = match (alu_op, high_half) {
                    (VecRRRLongOp::Smull8, false) => {
                        ("smull", VectorSize::Size16x8, VectorSize::Size8x8)
                    }
                    (VecRRRLongOp::Smull8, true) => {
                        ("smull2", VectorSize::Size16x8, VectorSize::Size8x16)
                    }
                    (VecRRRLongOp::Smull16, false) => {
                        ("smull", VectorSize::Size32x4, VectorSize::Size16x4)
                    }
                    (VecRRRLongOp::Smull16, true) => {
                        ("smull2", VectorSize::Size32x4, VectorSize::Size16x8)
                    }
                    (VecRRRLongOp::Smull32, false) => {
                        ("smull", VectorSize::Size64x2, VectorSize::Size32x2)
                    }
                    (VecRRRLongOp::Smull32, true) => {
                        ("smull2", VectorSize::Size64x2, VectorSize::Size32x4)
                    }
                    (VecRRRLongOp::Umull8, false) => {
                        ("umull", VectorSize::Size16x8, VectorSize::Size8x8)
                    }
                    (VecRRRLongOp::Umull8, true) => {
                        ("umull2", VectorSize::Size16x8, VectorSize::Size8x16)
                    }
                    (VecRRRLongOp::Umull16, false) => {
                        ("umull", VectorSize::Size32x4, VectorSize::Size16x4)
                    }
                    (VecRRRLongOp::Umull16, true) => {
                        ("umull2", VectorSize::Size32x4, VectorSize::Size16x8)
                    }
                    (VecRRRLongOp::Umull32, false) => {
                        ("umull", VectorSize::Size64x2, VectorSize::Size32x2)
                    }
                    (VecRRRLongOp::Umull32, true) => {
                        ("umull2", VectorSize::Size64x2, VectorSize::Size32x4)
                    }
                };
                let rd = pretty_print_vreg_vector(rd.to_reg(), dest_size);
                let rn = pretty_print_vreg_vector(rn, src_size);
                let rm = pretty_print_vreg_vector(rm, src_size);
                format!("{op} {rd}, {rn}, {rm}")
            }
            &Inst::VecRRRLongMod {
                rd,
                ri,
                rn,
                rm,
                alu_op,
                high_half,
            } => {
                let (op, dest_size, src_size) = match (alu_op, high_half) {
                    (VecRRRLongModOp::Umlal8, false) => {
                        ("umlal", VectorSize::Size16x8, VectorSize::Size8x8)
                    }
                    (VecRRRLongModOp::Umlal8, true) => {
                        ("umlal2", VectorSize::Size16x8, VectorSize::Size8x16)
                    }
                    (VecRRRLongModOp::Umlal16, false) => {
                        ("umlal", VectorSize::Size32x4, VectorSize::Size16x4)
                    }
                    (VecRRRLongModOp::Umlal16, true) => {
                        ("umlal2", VectorSize::Size32x4, VectorSize::Size16x8)
                    }
                    (VecRRRLongModOp::Umlal32, false) => {
                        ("umlal", VectorSize::Size64x2, VectorSize::Size32x2)
                    }
                    (VecRRRLongModOp::Umlal32, true) => {
                        ("umlal2", VectorSize::Size64x2, VectorSize::Size32x4)
                    }
                };
                let rd = pretty_print_vreg_vector(rd.to_reg(), dest_size);
                let ri = pretty_print_vreg_vector(ri, dest_size);
                let rn = pretty_print_vreg_vector(rn, src_size);
                let rm = pretty_print_vreg_vector(rm, src_size);
                format!("{op} {rd}, {ri}, {rn}, {rm}")
            }
            &Inst::VecMisc { op, rd, rn, size } => {
                let (op, size, suffix) = match op {
                    VecMisc2::Not => (
                        "mvn",
                        if size.is_128bits() {
                            VectorSize::Size8x16
                        } else {
                            VectorSize::Size8x8
                        },
                        "",
                    ),
                    VecMisc2::Neg => ("neg", size, ""),
                    VecMisc2::Abs => ("abs", size, ""),
                    VecMisc2::Fabs => ("fabs", size, ""),
                    VecMisc2::Fneg => ("fneg", size, ""),
                    VecMisc2::Fsqrt => ("fsqrt", size, ""),
                    VecMisc2::Rev16 => ("rev16", size, ""),
                    VecMisc2::Rev32 => ("rev32", size, ""),
                    VecMisc2::Rev64 => ("rev64", size, ""),
                    VecMisc2::Fcvtzs => ("fcvtzs", size, ""),
                    VecMisc2::Fcvtzu => ("fcvtzu", size, ""),
                    VecMisc2::Scvtf => ("scvtf", size, ""),
                    VecMisc2::Ucvtf => ("ucvtf", size, ""),
                    VecMisc2::Frintn => ("frintn", size, ""),
                    VecMisc2::Frintz => ("frintz", size, ""),
                    VecMisc2::Frintm => ("frintm", size, ""),
                    VecMisc2::Frintp => ("frintp", size, ""),
                    VecMisc2::Cnt => ("cnt", size, ""),
                    VecMisc2::Cmeq0 => ("cmeq", size, ", #0"),
                    VecMisc2::Cmge0 => ("cmge", size, ", #0"),
                    VecMisc2::Cmgt0 => ("cmgt", size, ", #0"),
                    VecMisc2::Cmle0 => ("cmle", size, ", #0"),
                    VecMisc2::Cmlt0 => ("cmlt", size, ", #0"),
                    VecMisc2::Fcmeq0 => ("fcmeq", size, ", #0.0"),
                    VecMisc2::Fcmge0 => ("fcmge", size, ", #0.0"),
                    VecMisc2::Fcmgt0 => ("fcmgt", size, ", #0.0"),
                    VecMisc2::Fcmle0 => ("fcmle", size, ", #0.0"),
                    VecMisc2::Fcmlt0 => ("fcmlt", size, ", #0.0"),
                };
                let rd = pretty_print_vreg_vector(rd.to_reg(), size);
                let rn = pretty_print_vreg_vector(rn, size);
                format!("{op} {rd}, {rn}{suffix}")
            }
            &Inst::VecLanes { op, rd, rn, size } => {
                let op = match op {
                    VecLanesOp::Uminv => "uminv",
                    VecLanesOp::Addv => "addv",
                };
                let rd = pretty_print_vreg_scalar(rd.to_reg(), size.lane_size());
                let rn = pretty_print_vreg_vector(rn, size);
                format!("{op} {rd}, {rn}")
            }
            &Inst::VecShiftImm {
                op,
                rd,
                rn,
                size,
                imm,
            } => {
                let op = match op {
                    VecShiftImmOp::Shl => "shl",
                    VecShiftImmOp::Ushr => "ushr",
                    VecShiftImmOp::Sshr => "sshr",
                };
                let rd = pretty_print_vreg_vector(rd.to_reg(), size);
                let rn = pretty_print_vreg_vector(rn, size);
                format!("{op} {rd}, {rn}, #{imm}")
            }
            &Inst::VecShiftImmMod {
                op,
                rd,
                ri,
                rn,
                size,
                imm,
            } => {
                let op = match op {
                    VecShiftImmModOp::Sli => "sli",
                };
                let rd = pretty_print_vreg_vector(rd.to_reg(), size);
                let ri = pretty_print_vreg_vector(ri, size);
                let rn = pretty_print_vreg_vector(rn, size);
                format!("{op} {rd}, {ri}, {rn}, #{imm}")
            }
            &Inst::VecExtract { rd, rn, rm, imm4 } => {
                let rd = pretty_print_vreg_vector(rd.to_reg(), VectorSize::Size8x16);
                let rn = pretty_print_vreg_vector(rn, VectorSize::Size8x16);
                let rm = pretty_print_vreg_vector(rm, VectorSize::Size8x16);
                format!("ext {rd}, {rn}, {rm}, #{imm4}")
            }
            &Inst::VecTbl { rd, rn, rm } => {
                let rn = pretty_print_vreg_vector(rn, VectorSize::Size8x16);
                let rm = pretty_print_vreg_vector(rm, VectorSize::Size8x16);
                let rd = pretty_print_vreg_vector(rd.to_reg(), VectorSize::Size8x16);
                format!("tbl {rd}, {{ {rn} }}, {rm}")
            }
            &Inst::VecTblExt { rd, ri, rn, rm } => {
                let rn = pretty_print_vreg_vector(rn, VectorSize::Size8x16);
                let rm = pretty_print_vreg_vector(rm, VectorSize::Size8x16);
                let rd = pretty_print_vreg_vector(rd.to_reg(), VectorSize::Size8x16);
                let ri = pretty_print_vreg_vector(ri, VectorSize::Size8x16);
                format!("tbx {rd}, {ri}, {{ {rn} }}, {rm}")
            }
            &Inst::VecTbl2 { rd, rn, rn2, rm } => {
                let rn = pretty_print_vreg_vector(rn, VectorSize::Size8x16);
                let rn2 = pretty_print_vreg_vector(rn2, VectorSize::Size8x16);
                let rm = pretty_print_vreg_vector(rm, VectorSize::Size8x16);
                let rd = pretty_print_vreg_vector(rd.to_reg(), VectorSize::Size8x16);
                format!("tbl {rd}, {{ {rn}, {rn2} }}, {rm}")
            }
            &Inst::VecTbl2Ext {
                rd,
                ri,
                rn,
                rn2,
                rm,
            } => {
                let rn = pretty_print_vreg_vector(rn, VectorSize::Size8x16);
                let rn2 = pretty_print_vreg_vector(rn2, VectorSize::Size8x16);
                let rm = pretty_print_vreg_vector(rm, VectorSize::Size8x16);
                let rd = pretty_print_vreg_vector(rd.to_reg(), VectorSize::Size8x16);
                let ri = pretty_print_vreg_vector(ri, VectorSize::Size8x16);
                format!("tbx {rd}, {ri}, {{ {rn}, {rn2} }}, {rm}")
            }
            &Inst::VecLoadReplicate { rd, rn, size, .. } => {
                let rd = pretty_print_vreg_vector(rd.to_reg(), size);
                let rn = pretty_print_reg(rn);

                format!("ld1r {{ {rd} }}, [{rn}]")
            }
            &Inst::VecCSel { rd, rn, rm, cond } => {
                let rd = pretty_print_vreg_vector(rd.to_reg(), VectorSize::Size8x16);
                let rn = pretty_print_vreg_vector(rn, VectorSize::Size8x16);
                let rm = pretty_print_vreg_vector(rm, VectorSize::Size8x16);
                let cond = cond.pretty_print(0);
                format!("vcsel {rd}, {rn}, {rm}, {cond} (if-then-else diamond)")
            }
            &Inst::MovToNZCV { rn } => {
                let rn = pretty_print_reg(rn);
                format!("msr nzcv, {rn}")
            }
            &Inst::MovFromNZCV { rd } => {
                let rd = pretty_print_reg(rd.to_reg());
                format!("mrs {rd}, nzcv")
            }
            &Inst::Extend {
                rd,
                rn,
                signed: false,
                from_bits: 1,
                ..
            } => {
                let rd = pretty_print_ireg(rd.to_reg(), OperandSize::Size32);
                let rn = pretty_print_ireg(rn, OperandSize::Size32);
                format!("and {rd}, {rn}, #1")
            }
            &Inst::Extend {
                rd,
                rn,
                signed: false,
                from_bits: 32,
                to_bits: 64,
            } => {
                // The case of a zero extension from 32 to 64 bits, is implemented
                // with a "mov" to a 32-bit (W-reg) dest, because this zeroes
                // the top 32 bits.
                let rd = pretty_print_ireg(rd.to_reg(), OperandSize::Size32);
                let rn = pretty_print_ireg(rn, OperandSize::Size32);
                format!("mov {rd}, {rn}")
            }
            &Inst::Extend {
                rd,
                rn,
                signed,
                from_bits,
                to_bits,
            } => {
                assert!(from_bits <= to_bits);
                let op = match (signed, from_bits) {
                    (false, 8) => "uxtb",
                    (true, 8) => "sxtb",
                    (false, 16) => "uxth",
                    (true, 16) => "sxth",
                    (true, 32) => "sxtw",
                    (true, _) => "sbfx",
                    (false, _) => "ubfx",
                };
                if op == "sbfx" || op == "ubfx" {
                    let dest_size = OperandSize::from_bits(to_bits);
                    let rd = pretty_print_ireg(rd.to_reg(), dest_size);
                    let rn = pretty_print_ireg(rn, dest_size);
                    format!("{op} {rd}, {rn}, #0, #{from_bits}")
                } else {
                    let dest_size = if signed {
                        OperandSize::from_bits(to_bits)
                    } else {
                        OperandSize::Size32
                    };
                    let rd = pretty_print_ireg(rd.to_reg(), dest_size);
                    let rn = pretty_print_ireg(rn, OperandSize::from_bits(from_bits));
                    format!("{op} {rd}, {rn}")
                }
            }
            &Inst::Call { ref info } => {
                let try_call = info
                    .try_call_info
                    .as_ref()
                    .map(|tci| pretty_print_try_call(tci))
                    .unwrap_or_default();
                format!("bl 0{try_call}")
            }
            &Inst::CallInd { ref info } => {
                let rn = pretty_print_reg(info.dest);
                let try_call = info
                    .try_call_info
                    .as_ref()
                    .map(|tci| pretty_print_try_call(tci))
                    .unwrap_or_default();
                format!("blr {rn}{try_call}")
            }
            &Inst::ReturnCall { ref info } => {
                let mut s = format!(
                    "return_call {:?} new_stack_arg_size:{}",
                    info.dest, info.new_stack_arg_size
                );
                for ret in &info.uses {
                    let preg = pretty_print_reg(ret.preg);
                    let vreg = pretty_print_reg(ret.vreg);
                    write!(&mut s, " {vreg}={preg}").unwrap();
                }
                s
            }
            &Inst::ReturnCallInd { ref info } => {
                let callee = pretty_print_reg(info.dest);
                let mut s = format!(
                    "return_call_ind {callee} new_stack_arg_size:{}",
                    info.new_stack_arg_size
                );
                for ret in &info.uses {
                    let preg = pretty_print_reg(ret.preg);
                    let vreg = pretty_print_reg(ret.vreg);
                    write!(&mut s, " {vreg}={preg}").unwrap();
                }
                s
            }
            &Inst::Args { ref args } => {
                let mut s = "args".to_string();
                for arg in args {
                    let preg = pretty_print_reg(arg.preg);
                    let def = pretty_print_reg(arg.vreg.to_reg());
                    write!(&mut s, " {def}={preg}").unwrap();
                }
                s
            }
            &Inst::Rets { ref rets } => {
                let mut s = "rets".to_string();
                for ret in rets {
                    let preg = pretty_print_reg(ret.preg);
                    let vreg = pretty_print_reg(ret.vreg);
                    write!(&mut s, " {vreg}={preg}").unwrap();
                }
                s
            }
            &Inst::Ret {} => "ret".to_string(),
            &Inst::AuthenticatedRet { key, is_hint } => {
                let key = match key {
                    APIKey::AZ => "az",
                    APIKey::BZ => "bz",
                    APIKey::ASP => "asp",
                    APIKey::BSP => "bsp",
                };
                match is_hint {
                    false => format!("reta{key}"),
                    true => format!("auti{key} ; ret"),
                }
            }
            &Inst::Jump { ref dest } => {
                let dest = dest.pretty_print(0);
                format!("b {dest}")
            }
            &Inst::CondBr {
                ref taken,
                ref not_taken,
                ref kind,
            } => {
                let taken = taken.pretty_print(0);
                let not_taken = not_taken.pretty_print(0);
                match kind {
                    &CondBrKind::Zero(reg, size) => {
                        let reg = pretty_print_reg_sized(reg, size);
                        format!("cbz {reg}, {taken} ; b {not_taken}")
                    }
                    &CondBrKind::NotZero(reg, size) => {
                        let reg = pretty_print_reg_sized(reg, size);
                        format!("cbnz {reg}, {taken} ; b {not_taken}")
                    }
                    &CondBrKind::Cond(c) => {
                        let c = c.pretty_print(0);
                        format!("b.{c} {taken} ; b {not_taken}")
                    }
                }
            }
            &Inst::TestBitAndBranch {
                kind,
                ref taken,
                ref not_taken,
                rn,
                bit,
            } => {
                let cond = match kind {
                    TestBitAndBranchKind::Z => "z",
                    TestBitAndBranchKind::NZ => "nz",
                };
                let taken = taken.pretty_print(0);
                let not_taken = not_taken.pretty_print(0);
                let rn = pretty_print_reg(rn);
                format!("tb{cond} {rn}, #{bit}, {taken} ; b {not_taken}")
            }
            &Inst::IndirectBr { rn, .. } => {
                let rn = pretty_print_reg(rn);
                format!("br {rn}")
            }
            &Inst::Brk => "brk #0xf000".to_string(),
            &Inst::Udf { .. } => "udf #0xc11f".to_string(),
            &Inst::TrapIf {
                ref kind,
                trap_code,
            } => match kind {
                &CondBrKind::Zero(reg, size) => {
                    let reg = pretty_print_reg_sized(reg, size);
                    format!("cbz {reg}, #trap={trap_code}")
                }
                &CondBrKind::NotZero(reg, size) => {
                    let reg = pretty_print_reg_sized(reg, size);
                    format!("cbnz {reg}, #trap={trap_code}")
                }
                &CondBrKind::Cond(c) => {
                    let c = c.pretty_print(0);
                    format!("b.{c} #trap={trap_code}")
                }
            },
            &Inst::Adr { rd, off } => {
                let rd = pretty_print_reg(rd.to_reg());
                format!("adr {rd}, pc+{off}")
            }
            &Inst::Adrp { rd, off } => {
                let rd = pretty_print_reg(rd.to_reg());
                // This instruction addresses 4KiB pages, so multiply it by the page size.
                let byte_offset = off * 4096;
                format!("adrp {rd}, pc+{byte_offset}")
            }
            &Inst::Word4 { data } => format!("data.i32 {data}"),
            &Inst::Word8 { data } => format!("data.i64 {data}"),
            &Inst::JTSequence {
                default,
                ref targets,
                ridx,
                rtmp1,
                rtmp2,
                ..
            } => {
                let ridx = pretty_print_reg(ridx);
                let rtmp1 = pretty_print_reg(rtmp1.to_reg());
                let rtmp2 = pretty_print_reg(rtmp2.to_reg());
                let default_target = BranchTarget::Label(default).pretty_print(0);
                format!(
                    concat!(
                        "b.hs {} ; ",
                        "csel {}, xzr, {}, hs ; ",
                        "csdb ; ",
                        "adr {}, pc+16 ; ",
                        "ldrsw {}, [{}, {}, uxtw #2] ; ",
                        "add {}, {}, {} ; ",
                        "br {} ; ",
                        "jt_entries {:?}"
                    ),
                    default_target,
                    rtmp2,
                    ridx,
                    rtmp1,
                    rtmp2,
                    rtmp1,
                    rtmp2,
                    rtmp1,
                    rtmp1,
                    rtmp2,
                    rtmp1,
                    targets
                )
            }
            &Inst::LoadExtName {
                rd,
                ref name,
                offset,
            } => {
                let rd = pretty_print_reg(rd.to_reg());
                format!("load_ext_name {rd}, {name:?}+{offset}")
            }
            &Inst::LoadAddr { rd, ref mem } => {
                // TODO: we really should find a better way to avoid duplication of
                // this logic between `emit()` and `show_rru()` -- a separate 1-to-N
                // expansion stage (i.e., legalization, but without the slow edit-in-place
                // of the existing legalization framework).
                let mem = mem.clone();
                let (mem_insts, mem) = mem_finalize(None, &mem, I8, state);
                let mut ret = String::new();
                for inst in mem_insts.into_iter() {
                    ret.push_str(&inst.print_with_state(&mut EmitState::default()));
                }
                let (reg, index_reg, offset) = match mem {
                    AMode::RegExtended { rn, rm, extendop } => (rn, Some((rm, extendop)), 0),
                    AMode::Unscaled { rn, simm9 } => (rn, None, simm9.value()),
                    AMode::UnsignedOffset { rn, uimm12 } => (rn, None, uimm12.value() as i32),
                    _ => panic!("Unsupported case for LoadAddr: {mem:?}"),
                };
                let abs_offset = if offset < 0 {
                    -offset as u64
                } else {
                    offset as u64
                };
                let alu_op = if offset < 0 { ALUOp::Sub } else { ALUOp::Add };

                if let Some((idx, extendop)) = index_reg {
                    let add = Inst::AluRRRExtend {
                        alu_op: ALUOp::Add,
                        size: OperandSize::Size64,
                        rd,
                        rn: reg,
                        rm: idx,
                        extendop,
                    };

                    ret.push_str(&add.print_with_state(&mut EmitState::default()));
                } else if offset == 0 {
                    let mov = Inst::gen_move(rd, reg, I64);
                    ret.push_str(&mov.print_with_state(&mut EmitState::default()));
                } else if let Some(imm12) = Imm12::maybe_from_u64(abs_offset) {
                    let add = Inst::AluRRImm12 {
                        alu_op,
                        size: OperandSize::Size64,
                        rd,
                        rn: reg,
                        imm12,
                    };
                    ret.push_str(&add.print_with_state(&mut EmitState::default()));
                } else {
                    let tmp = writable_spilltmp_reg();
                    for inst in Inst::load_constant(tmp, abs_offset).into_iter() {
                        ret.push_str(&inst.print_with_state(&mut EmitState::default()));
                    }
                    let add = Inst::AluRRR {
                        alu_op,
                        size: OperandSize::Size64,
                        rd,
                        rn: reg,
                        rm: tmp.to_reg(),
                    };
                    ret.push_str(&add.print_with_state(&mut EmitState::default()));
                }
                ret
            }
            &Inst::Paci { key } => {
                let key = match key {
                    APIKey::AZ => "az",
                    APIKey::BZ => "bz",
                    APIKey::ASP => "asp",
                    APIKey::BSP => "bsp",
                };

                "paci".to_string() + key
            }
            &Inst::Xpaclri => "xpaclri".to_string(),
            &Inst::Bti { targets } => {
                let targets = match targets {
                    BranchTargetType::None => "",
                    BranchTargetType::C => " c",
                    BranchTargetType::J => " j",
                    BranchTargetType::JC => " jc",
                };

                "bti".to_string() + targets
            }
            &Inst::EmitIsland { needed_space } => format!("emit_island {needed_space}"),

            &Inst::ElfTlsGetAddr {
                ref symbol,
                rd,
                tmp,
            } => {
                let rd = pretty_print_reg(rd.to_reg());
                let tmp = pretty_print_reg(tmp.to_reg());
                format!("elf_tls_get_addr {}, {}, {}", rd, tmp, symbol.display(None))
            }
            &Inst::MachOTlsGetAddr { ref symbol, rd } => {
                let rd = pretty_print_reg(rd.to_reg());
                format!("macho_tls_get_addr {}, {}", rd, symbol.display(None))
            }
            &Inst::Unwind { ref inst } => {
                format!("unwind {inst:?}")
            }
            &Inst::DummyUse { reg } => {
                let reg = pretty_print_reg(reg);
                format!("dummy_use {reg}")
            }
            &Inst::StackProbeLoop { start, end, step } => {
                let start = pretty_print_reg(start.to_reg());
                let end = pretty_print_reg(end);
                let step = step.pretty_print(0);
                format!("stack_probe_loop {start}, {end}, {step}")
            }
        }
    }
}

//=============================================================================
// Label fixups and jump veneers.

/// Different forms of label references for different instruction formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelUse {
    /// 14-bit branch offset (conditional branches). PC-rel, offset is imm <<
    /// 2. Immediate is 14 signed bits, in bits 18:5. Used by tbz and tbnz.
    Branch14,
    /// 19-bit branch offset (conditional branches). PC-rel, offset is imm << 2. Immediate is 19
    /// signed bits, in bits 23:5. Used by cbz, cbnz, b.cond.
    Branch19,
    /// 26-bit branch offset (unconditional branches). PC-rel, offset is imm << 2. Immediate is 26
    /// signed bits, in bits 25:0. Used by b, bl.
    Branch26,
    /// 19-bit offset for LDR (load literal). PC-rel, offset is imm << 2. Immediate is 19 signed bits,
    /// in bits 23:5.
    Ldr19,
    /// 21-bit offset for ADR (get address of label). PC-rel, offset is not shifted. Immediate is
    /// 21 signed bits, with high 19 bits in bits 23:5 and low 2 bits in bits 30:29.
    Adr21,
    /// 32-bit PC relative constant offset (from address of constant itself),
    /// signed. Used in jump tables.
    PCRel32,
}

impl MachInstLabelUse for LabelUse {
    /// Alignment for veneer code. Every AArch64 instruction must be 4-byte-aligned.
    const ALIGN: CodeOffset = 4;

    /// Maximum PC-relative range (positive), inclusive.
    fn max_pos_range(self) -> CodeOffset {
        match self {
            // N-bit immediate, left-shifted by 2, for (N+2) bits of total
            // range. Signed, so +2^(N+1) from zero. Likewise for two other
            // shifted cases below.
            LabelUse::Branch14 => (1 << 15) - 1,
            LabelUse::Branch19 => (1 << 20) - 1,
            LabelUse::Branch26 => (1 << 27) - 1,
            LabelUse::Ldr19 => (1 << 20) - 1,
            // Adr does not shift its immediate, so the 21-bit immediate gives 21 bits of total
            // range.
            LabelUse::Adr21 => (1 << 20) - 1,
            LabelUse::PCRel32 => 0x7fffffff,
        }
    }

    /// Maximum PC-relative range (negative).
    fn max_neg_range(self) -> CodeOffset {
        // All forms are twos-complement signed offsets, so negative limit is one more than
        // positive limit.
        self.max_pos_range() + 1
    }

    /// Size of window into code needed to do the patch.
    fn patch_size(self) -> CodeOffset {
        // Patch is on one instruction only for all of these label reference types.
        4
    }

    /// Perform the patch.
    fn patch(self, buffer: &mut [u8], use_offset: CodeOffset, label_offset: CodeOffset) {
        let pc_rel = (label_offset as i64) - (use_offset as i64);
        debug_assert!(pc_rel <= self.max_pos_range() as i64);
        debug_assert!(pc_rel >= -(self.max_neg_range() as i64));
        let pc_rel = pc_rel as u32;
        let insn_word = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let mask = match self {
            LabelUse::Branch14 => 0x0007ffe0, // bits 18..5 inclusive
            LabelUse::Branch19 => 0x00ffffe0, // bits 23..5 inclusive
            LabelUse::Branch26 => 0x03ffffff, // bits 25..0 inclusive
            LabelUse::Ldr19 => 0x00ffffe0,    // bits 23..5 inclusive
            LabelUse::Adr21 => 0x60ffffe0,    // bits 30..29, 25..5 inclusive
            LabelUse::PCRel32 => 0xffffffff,
        };
        let pc_rel_shifted = match self {
            LabelUse::Adr21 | LabelUse::PCRel32 => pc_rel,
            _ => {
                debug_assert!(pc_rel & 3 == 0);
                pc_rel >> 2
            }
        };
        let pc_rel_inserted = match self {
            LabelUse::Branch14 => (pc_rel_shifted & 0x3fff) << 5,
            LabelUse::Branch19 | LabelUse::Ldr19 => (pc_rel_shifted & 0x7ffff) << 5,
            LabelUse::Branch26 => pc_rel_shifted & 0x3ffffff,
            LabelUse::Adr21 => (pc_rel_shifted & 0x7ffff) << 5 | (pc_rel_shifted & 0x180000) << 10,
            LabelUse::PCRel32 => pc_rel_shifted,
        };
        let is_add = match self {
            LabelUse::PCRel32 => true,
            _ => false,
        };
        let insn_word = if is_add {
            insn_word.wrapping_add(pc_rel_inserted)
        } else {
            (insn_word & !mask) | pc_rel_inserted
        };
        buffer[0..4].clone_from_slice(&u32::to_le_bytes(insn_word));
    }

    /// Is a veneer supported for this label reference type?
    fn supports_veneer(self) -> bool {
        match self {
            LabelUse::Branch14 | LabelUse::Branch19 => true, // veneer is a Branch26
            LabelUse::Branch26 => true,                      // veneer is a PCRel32
            _ => false,
        }
    }

    /// How large is the veneer, if supported?
    fn veneer_size(self) -> CodeOffset {
        match self {
            LabelUse::Branch14 | LabelUse::Branch19 => 4,
            LabelUse::Branch26 => 20,
            _ => unreachable!(),
        }
    }

    fn worst_case_veneer_size() -> CodeOffset {
        20
    }

    /// Generate a veneer into the buffer, given that this veneer is at `veneer_offset`, and return
    /// an offset and label-use for the veneer's use of the original label.
    fn generate_veneer(
        self,
        buffer: &mut [u8],
        veneer_offset: CodeOffset,
    ) -> (CodeOffset, LabelUse) {
        match self {
            LabelUse::Branch14 | LabelUse::Branch19 => {
                // veneer is a Branch26 (unconditional branch). Just encode directly here -- don't
                // bother with constructing an Inst.
                let insn_word = 0b000101 << 26;
                buffer[0..4].clone_from_slice(&u32::to_le_bytes(insn_word));
                (veneer_offset, LabelUse::Branch26)
            }

            // This is promoting a 26-bit call/jump to a 32-bit call/jump to
            // get a further range. This jump translates to a jump to a
            // relative location based on the address of the constant loaded
            // from here.
            //
            // If this path is taken from a call instruction then caller-saved
            // registers are available (minus arguments), so x16/x17 are
            // available. Otherwise for intra-function jumps we also reserve
            // x16/x17 as spill-style registers. In both cases these are
            // available for us to use.
            LabelUse::Branch26 => {
                let tmp1 = regs::spilltmp_reg();
                let tmp1_w = regs::writable_spilltmp_reg();
                let tmp2 = regs::tmp2_reg();
                let tmp2_w = regs::writable_tmp2_reg();
                // ldrsw x16, 16
                let ldr = emit::enc_ldst_imm19(0b1001_1000, 16 / 4, tmp1);
                // adr x17, 12
                let adr = emit::enc_adr(12, tmp2_w);
                // add x16, x16, x17
                let add = emit::enc_arith_rrr(0b10001011_000, 0, tmp1_w, tmp1, tmp2);
                // br x16
                let br = emit::enc_br(tmp1);
                buffer[0..4].clone_from_slice(&u32::to_le_bytes(ldr));
                buffer[4..8].clone_from_slice(&u32::to_le_bytes(adr));
                buffer[8..12].clone_from_slice(&u32::to_le_bytes(add));
                buffer[12..16].clone_from_slice(&u32::to_le_bytes(br));
                // the 4-byte signed immediate we'll load is after these
                // instructions, 16-bytes in.
                (veneer_offset + 16, LabelUse::PCRel32)
            }

            _ => panic!("Unsupported label-reference type for veneer generation!"),
        }
    }

    fn from_reloc(reloc: Reloc, addend: Addend) -> Option<LabelUse> {
        match (reloc, addend) {
            (Reloc::Arm64Call, 0) => Some(LabelUse::Branch26),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inst_size_test() {
        // This test will help with unintentionally growing the size
        // of the Inst enum.
        let expected = if cfg!(target_pointer_width = "32") && !cfg!(target_arch = "arm") {
            28
        } else {
            32
        };
        assert_eq!(expected, std::mem::size_of::<Inst>());
    }
}

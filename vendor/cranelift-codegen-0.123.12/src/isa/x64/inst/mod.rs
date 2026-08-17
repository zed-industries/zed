//! This module defines x86_64-specific machine instruction types.

pub use emit_state::EmitState;

use crate::binemit::{Addend, CodeOffset, Reloc};
use crate::ir::{ExternalName, LibCall, TrapCode, Type, types};
use crate::isa::x64::abi::X64ABIMachineSpec;
use crate::isa::x64::inst::regs::{pretty_print_reg, show_ireg_sized};
use crate::isa::x64::settings as x64_settings;
use crate::isa::{CallConv, FunctionAlignment};
use crate::{CodegenError, CodegenResult, settings};
use crate::{machinst::*, trace};
use alloc::boxed::Box;
use core::slice;
use cranelift_assembler_x64 as asm;
use cranelift_entity::{Signed, Unsigned};
use smallvec::{SmallVec, smallvec};
use std::fmt::{self, Write};
use std::string::{String, ToString};

pub mod args;
mod emit;
mod emit_state;
#[cfg(test)]
mod emit_tests;
pub mod external;
pub mod regs;
mod stack_switch;
pub mod unwind;

use args::*;

//=============================================================================
// Instructions (top level): definition

// `Inst` is defined inside ISLE as `MInst`. We publicly re-export it here.
pub use super::lower::isle::generated_code::AtomicRmwSeqOp;
pub use super::lower::isle::generated_code::MInst as Inst;

/// Out-of-line data for return-calls, to keep the size of `Inst` down.
#[derive(Clone, Debug)]
pub struct ReturnCallInfo<T> {
    /// Where this call is going.
    pub dest: T,

    /// The size of the argument area for this return-call, potentially smaller than that of the
    /// caller, but never larger.
    pub new_stack_arg_size: u32,

    /// The in-register arguments and their constraints.
    pub uses: CallArgList,

    /// A temporary for use when moving the return address.
    pub tmp: WritableGpr,
}

#[test]
#[cfg(target_pointer_width = "64")]
fn inst_size_test() {
    // This test will help with unintentionally growing the size
    // of the Inst enum.
    assert_eq!(48, std::mem::size_of::<Inst>());
}

impl Inst {
    /// Retrieve a list of ISA feature sets in which the instruction is available. An empty list
    /// indicates that the instruction is available in the baseline feature set (i.e. SSE2 and
    /// below); more than one `InstructionSet` in the list indicates that the instruction is present
    /// *any* of the included ISA feature sets.
    fn available_in_any_isa(&self) -> SmallVec<[InstructionSet; 2]> {
        match self {
            // These instructions are part of SSE2, which is a basic requirement in Cranelift, and
            // don't have to be checked.
            Inst::AtomicRmwSeq { .. }
            | Inst::CallKnown { .. }
            | Inst::CallUnknown { .. }
            | Inst::ReturnCallKnown { .. }
            | Inst::ReturnCallUnknown { .. }
            | Inst::CheckedSRemSeq { .. }
            | Inst::CheckedSRemSeq8 { .. }
            | Inst::CvtFloatToSintSeq { .. }
            | Inst::CvtFloatToUintSeq { .. }
            | Inst::CvtUint64ToFloatSeq { .. }
            | Inst::JmpCond { .. }
            | Inst::JmpCondOr { .. }
            | Inst::WinchJmpIf { .. }
            | Inst::JmpKnown { .. }
            | Inst::JmpTableSeq { .. }
            | Inst::LoadExtName { .. }
            | Inst::MovFromPReg { .. }
            | Inst::MovToPReg { .. }
            | Inst::StackProbeLoop { .. }
            | Inst::Args { .. }
            | Inst::Rets { .. }
            | Inst::StackSwitchBasic { .. }
            | Inst::TrapIf { .. }
            | Inst::TrapIfAnd { .. }
            | Inst::TrapIfOr { .. }
            | Inst::XmmCmove { .. }
            | Inst::XmmMinMaxSeq { .. }
            | Inst::XmmUninitializedValue { .. }
            | Inst::GprUninitializedValue { .. }
            | Inst::ElfTlsGetAddr { .. }
            | Inst::MachOTlsGetAddr { .. }
            | Inst::CoffTlsGetAddr { .. }
            | Inst::Unwind { .. }
            | Inst::DummyUse { .. } => smallvec![],

            Inst::Atomic128RmwSeq { .. } | Inst::Atomic128XchgSeq { .. } => {
                smallvec![InstructionSet::CMPXCHG16b]
            }

            Inst::External { inst } => {
                use cranelift_assembler_x64::Feature::*;
                let mut features = smallvec![];
                for f in inst.features() {
                    match f {
                        _64b | compat => {}
                        sse => features.push(InstructionSet::SSE),
                        sse2 => features.push(InstructionSet::SSE2),
                        sse3 => features.push(InstructionSet::SSE3),
                        ssse3 => features.push(InstructionSet::SSSE3),
                        sse41 => features.push(InstructionSet::SSE41),
                        sse42 => features.push(InstructionSet::SSE42),
                        bmi1 => features.push(InstructionSet::BMI1),
                        bmi2 => features.push(InstructionSet::BMI2),
                        lzcnt => features.push(InstructionSet::Lzcnt),
                        popcnt => features.push(InstructionSet::Popcnt),
                        avx => features.push(InstructionSet::AVX),
                        avx2 => features.push(InstructionSet::AVX2),
                        avx512f => features.push(InstructionSet::AVX512F),
                        avx512vl => features.push(InstructionSet::AVX512VL),
                        avx512dq => features.push(InstructionSet::AVX512DQ),
                        avx512bitalg => features.push(InstructionSet::AVX512BITALG),
                        avx512vbmi => features.push(InstructionSet::AVX512VBMI),
                        cmpxchg16b => features.push(InstructionSet::CMPXCHG16b),
                        fma => features.push(InstructionSet::FMA),
                    }
                }
                features
            }
        }
    }
}

// Handy constructors for Insts.

impl Inst {
    pub(crate) fn nop(len: u8) -> Self {
        assert!(len > 0 && len <= 9);
        let inst = match len {
            1 => asm::inst::nop_1b::new().into(),
            2 => asm::inst::nop_2b::new().into(),
            3 => asm::inst::nop_3b::new().into(),
            4 => asm::inst::nop_4b::new().into(),
            5 => asm::inst::nop_5b::new().into(),
            6 => asm::inst::nop_6b::new().into(),
            7 => asm::inst::nop_7b::new().into(),
            8 => asm::inst::nop_8b::new().into(),
            9 => asm::inst::nop_9b::new().into(),
            _ => unreachable!("nop length must be between 1 and 9"),
        };
        Self::External { inst }
    }

    pub(crate) fn addq_mi(dst: Writable<Reg>, simm32: i32) -> Self {
        let inst = if let Ok(simm8) = i8::try_from(simm32) {
            asm::inst::addq_mi_sxb::new(dst, simm8).into()
        } else {
            asm::inst::addq_mi_sxl::new(dst, simm32).into()
        };
        Inst::External { inst }
    }

    pub(crate) fn subq_mi(dst: Writable<Reg>, simm32: i32) -> Self {
        let inst = if let Ok(simm8) = i8::try_from(simm32) {
            asm::inst::subq_mi_sxb::new(dst, simm8).into()
        } else {
            asm::inst::subq_mi_sxl::new(dst, simm32).into()
        };
        Inst::External { inst }
    }

    /// Writes the `simm64` immedaite into `dst`.
    ///
    /// Note that if `dst_size` is less than 64-bits then the upper bits of
    /// `simm64` will be converted to zero.
    pub fn imm(dst_size: OperandSize, simm64: u64, dst: Writable<Reg>) -> Inst {
        debug_assert!(dst_size.is_one_of(&[OperandSize::Size32, OperandSize::Size64]));
        debug_assert!(dst.to_reg().class() == RegClass::Int);
        let dst = WritableGpr::from_writable_reg(dst).unwrap();
        let inst = match dst_size {
            OperandSize::Size64 => match u32::try_from(simm64) {
                // If `simm64` is zero-extended use `movl` which zeros the
                // upper bits.
                Ok(imm32) => asm::inst::movl_oi::new(dst, imm32).into(),
                _ => match i32::try_from(simm64.signed()) {
                    // If `simm64` is sign-extended use `movq` which sign the
                    // upper bits.
                    Ok(simm32) => asm::inst::movq_mi_sxl::new(dst, simm32).into(),
                    // fall back to embedding the entire immediate.
                    _ => asm::inst::movabsq_oi::new(dst, simm64).into(),
                },
            },
            // FIXME: the input to this function is a logical `simm64` stored
            // as `u64`. That means that ideally what we would do here is cast
            // the `simm64` to an `i64`, perform a `i32::try_from()`, then cast
            // that back to `u32`. That would ensure that the immediate loses
            // no meaning and has the same logical value. Currently though
            // Cranelift relies on discarding the upper bits because literals
            // like `0x8000_0000_u64` fail to convert to an `i32`. In theory
            // the input to this function should change to `i64`. In the
            // meantime this is documented as discarding the upper bits,
            // although this is an old function so that's unlikely to help
            // much.
            _ => asm::inst::movl_oi::new(dst, simm64 as u32).into(),
        };
        Inst::External { inst }
    }

    pub(crate) fn movzx_rm_r(ext_mode: ExtMode, src: RegMem, dst: Writable<Reg>) -> Inst {
        src.assert_regclass_is(RegClass::Int);
        debug_assert!(dst.to_reg().class() == RegClass::Int);
        let src = match src {
            RegMem::Reg { reg } => asm::GprMem::Gpr(Gpr::new(reg).unwrap()),
            RegMem::Mem { addr } => asm::GprMem::Mem(addr.into()),
        };
        let inst = match ext_mode {
            ExtMode::BL => asm::inst::movzbl_rm::new(dst, src).into(),
            ExtMode::BQ => asm::inst::movzbq_rm::new(dst, src).into(),
            ExtMode::WL => asm::inst::movzwl_rm::new(dst, src).into(),
            ExtMode::WQ => asm::inst::movzwq_rm::new(dst, src).into(),
            ExtMode::LQ => {
                // This instruction selection may seem strange but is correct in
                // 64-bit mode: section 3.4.1.1 of the Intel manual says that
                // "32-bit operands generate a 32-bit result, zero-extended to a
                // 64-bit result in the destination general-purpose register."
                // This is applicable beyond `mov` but we use this fact to
                // zero-extend `src` into `dst`.
                asm::inst::movl_rm::new(dst, src).into()
            }
        };
        Inst::External { inst }
    }

    pub(crate) fn movsx_rm_r(ext_mode: ExtMode, src: RegMem, dst: Writable<Reg>) -> Inst {
        src.assert_regclass_is(RegClass::Int);
        debug_assert!(dst.to_reg().class() == RegClass::Int);
        let src = match src {
            RegMem::Reg { reg } => asm::GprMem::Gpr(Gpr::new(reg).unwrap()),
            RegMem::Mem { addr } => asm::GprMem::Mem(addr.into()),
        };
        let inst = match ext_mode {
            ExtMode::BL => asm::inst::movsbl_rm::new(dst, src).into(),
            ExtMode::BQ => asm::inst::movsbq_rm::new(dst, src).into(),
            ExtMode::WL => asm::inst::movswl_rm::new(dst, src).into(),
            ExtMode::WQ => asm::inst::movswq_rm::new(dst, src).into(),
            ExtMode::LQ => asm::inst::movslq_rm::new(dst, src).into(),
        };
        Inst::External { inst }
    }

    /// Compares `src1` against `src2`
    pub(crate) fn cmp_mi_sxb(size: OperandSize, src1: Gpr, src2: i8) -> Inst {
        let inst = match size {
            OperandSize::Size8 => asm::inst::cmpb_mi::new(src1, src2.unsigned()).into(),
            OperandSize::Size16 => asm::inst::cmpw_mi_sxb::new(src1, src2).into(),
            OperandSize::Size32 => asm::inst::cmpl_mi_sxb::new(src1, src2).into(),
            OperandSize::Size64 => asm::inst::cmpq_mi_sxb::new(src1, src2).into(),
        };
        Inst::External { inst }
    }

    pub(crate) fn trap_if(cc: CC, trap_code: TrapCode) -> Inst {
        Inst::TrapIf { cc, trap_code }
    }

    pub(crate) fn call_known(info: Box<CallInfo<ExternalName>>) -> Inst {
        Inst::CallKnown { info }
    }

    pub(crate) fn call_unknown(info: Box<CallInfo<RegMem>>) -> Inst {
        info.dest.assert_regclass_is(RegClass::Int);
        Inst::CallUnknown { info }
    }

    pub(crate) fn jmp_known(dst: MachLabel) -> Inst {
        Inst::JmpKnown { dst }
    }

    /// Choose which instruction to use for loading a register value from memory. For loads smaller
    /// than 64 bits, this method expects a way to extend the value (i.e. [ExtKind::SignExtend],
    /// [ExtKind::ZeroExtend]); loads with no extension necessary will ignore this.
    pub(crate) fn load(
        ty: Type,
        from_addr: impl Into<SyntheticAmode>,
        to_reg: Writable<Reg>,
        ext_kind: ExtKind,
    ) -> Inst {
        let rc = to_reg.to_reg().class();
        match rc {
            RegClass::Int => {
                let ext_mode = match ty.bytes() {
                    1 => Some(ExtMode::BQ),
                    2 => Some(ExtMode::WQ),
                    4 => Some(ExtMode::LQ),
                    8 => None,
                    _ => unreachable!("the type should never use a scalar load: {}", ty),
                };
                if let Some(ext_mode) = ext_mode {
                    // Values smaller than 64 bits must be extended in some way.
                    match ext_kind {
                        ExtKind::SignExtend => {
                            Inst::movsx_rm_r(ext_mode, RegMem::mem(from_addr), to_reg)
                        }
                        ExtKind::ZeroExtend => {
                            Inst::movzx_rm_r(ext_mode, RegMem::mem(from_addr), to_reg)
                        }
                        ExtKind::None => {
                            panic!("expected an extension kind for extension mode: {ext_mode:?}")
                        }
                    }
                } else {
                    // 64-bit values can be moved directly.
                    let from_addr = asm::GprMem::from(from_addr.into());
                    Inst::External {
                        inst: asm::inst::movq_rm::new(to_reg, from_addr).into(),
                    }
                }
            }
            RegClass::Float => {
                let to_reg = to_reg.map(|r| Xmm::new(r).unwrap());
                let from_addr = from_addr.into();
                let inst = match ty {
                    types::F16 | types::I8X2 => {
                        panic!("loading a f16 or i8x2 requires multiple instructions")
                    }
                    _ if (ty.is_float() || ty.is_vector()) && ty.bits() == 32 => {
                        asm::inst::movss_a_m::new(to_reg, from_addr).into()
                    }
                    _ if (ty.is_float() || ty.is_vector()) && ty.bits() == 64 => {
                        asm::inst::movsd_a_m::new(to_reg, from_addr).into()
                    }
                    types::F32X4 => asm::inst::movups_a::new(to_reg, from_addr).into(),
                    types::F64X2 => asm::inst::movupd_a::new(to_reg, from_addr).into(),
                    _ if (ty.is_float() || ty.is_vector()) && ty.bits() == 128 => {
                        asm::inst::movdqu_a::new(to_reg, from_addr).into()
                    }
                    _ => unimplemented!("unable to load type: {}", ty),
                };
                Inst::External { inst }
            }
            RegClass::Vector => unreachable!(),
        }
    }

    /// Choose which instruction to use for storing a register value to memory.
    pub(crate) fn store(ty: Type, from_reg: Reg, to_addr: impl Into<SyntheticAmode>) -> Inst {
        let rc = from_reg.class();
        let to_addr = to_addr.into();
        let inst = match rc {
            RegClass::Int => {
                let from_reg = Gpr::unwrap_new(from_reg);
                match ty {
                    types::I8 => asm::inst::movb_mr::new(to_addr, from_reg).into(),
                    types::I16 => asm::inst::movw_mr::new(to_addr, from_reg).into(),
                    types::I32 => asm::inst::movl_mr::new(to_addr, from_reg).into(),
                    types::I64 => asm::inst::movq_mr::new(to_addr, from_reg).into(),
                    _ => unreachable!(),
                }
            }
            RegClass::Float => {
                let from_reg = Xmm::new(from_reg).unwrap();
                match ty {
                    types::F16 | types::I8X2 => {
                        panic!("storing a f16 or i8x2 requires multiple instructions")
                    }
                    _ if (ty.is_float() || ty.is_vector()) && ty.bits() == 32 => {
                        asm::inst::movss_c_m::new(to_addr, from_reg).into()
                    }
                    _ if (ty.is_float() || ty.is_vector()) && ty.bits() == 64 => {
                        asm::inst::movsd_c_m::new(to_addr, from_reg).into()
                    }
                    types::F32X4 => asm::inst::movups_b::new(to_addr, from_reg).into(),
                    types::F64X2 => asm::inst::movupd_b::new(to_addr, from_reg).into(),
                    _ if (ty.is_float() || ty.is_vector()) && ty.bits() == 128 => {
                        asm::inst::movdqu_b::new(to_addr, from_reg).into()
                    }
                    _ => unimplemented!("unable to store type: {}", ty),
                }
            }
            RegClass::Vector => unreachable!(),
        };
        Inst::External { inst }
    }
}

//=============================================================================
// Instructions: printing

impl PrettyPrint for Inst {
    fn pretty_print(&self, _size: u8) -> String {
        fn ljustify(s: String) -> String {
            let w = 7;
            if s.len() >= w {
                s
            } else {
                let need = usize::min(w, w - s.len());
                s + &format!("{nil: <width$}", nil = "", width = need)
            }
        }

        fn ljustify2(s1: String, s2: String) -> String {
            ljustify(s1 + &s2)
        }

        match self {
            Inst::CheckedSRemSeq {
                size,
                divisor,
                dividend_lo,
                dividend_hi,
                dst_quotient,
                dst_remainder,
            } => {
                let divisor = pretty_print_reg(divisor.to_reg(), size.to_bytes());
                let dividend_lo = pretty_print_reg(dividend_lo.to_reg(), size.to_bytes());
                let dividend_hi = pretty_print_reg(dividend_hi.to_reg(), size.to_bytes());
                let dst_quotient =
                    pretty_print_reg(dst_quotient.to_reg().to_reg(), size.to_bytes());
                let dst_remainder =
                    pretty_print_reg(dst_remainder.to_reg().to_reg(), size.to_bytes());
                format!(
                    "checked_srem_seq {dividend_lo}, {dividend_hi}, \
                        {divisor}, {dst_quotient}, {dst_remainder}",
                )
            }

            Inst::CheckedSRemSeq8 {
                divisor,
                dividend,
                dst,
            } => {
                let divisor = pretty_print_reg(divisor.to_reg(), 1);
                let dividend = pretty_print_reg(dividend.to_reg(), 1);
                let dst = pretty_print_reg(dst.to_reg().to_reg(), 1);
                format!("checked_srem_seq {dividend}, {divisor}, {dst}")
            }

            Inst::XmmMinMaxSeq {
                lhs,
                rhs,
                dst,
                is_min,
                size,
            } => {
                let rhs = pretty_print_reg(rhs.to_reg(), 8);
                let lhs = pretty_print_reg(lhs.to_reg(), 8);
                let dst = pretty_print_reg(dst.to_reg().to_reg(), 8);
                let op = ljustify2(
                    if *is_min {
                        "xmm min seq ".to_string()
                    } else {
                        "xmm max seq ".to_string()
                    },
                    format!("f{}", size.to_bits()),
                );
                format!("{op} {lhs}, {rhs}, {dst}")
            }

            Inst::XmmUninitializedValue { dst } => {
                let dst = pretty_print_reg(dst.to_reg().to_reg(), 8);
                let op = ljustify("uninit".into());
                format!("{op} {dst}")
            }

            Inst::GprUninitializedValue { dst } => {
                let dst = pretty_print_reg(dst.to_reg().to_reg(), 8);
                let op = ljustify("uninit".into());
                format!("{op} {dst}")
            }

            Inst::CvtUint64ToFloatSeq {
                src,
                dst,
                dst_size,
                tmp_gpr1,
                tmp_gpr2,
                ..
            } => {
                let src = pretty_print_reg(src.to_reg(), 8);
                let dst = pretty_print_reg(dst.to_reg().to_reg(), dst_size.to_bytes());
                let tmp_gpr1 = pretty_print_reg(tmp_gpr1.to_reg().to_reg(), 8);
                let tmp_gpr2 = pretty_print_reg(tmp_gpr2.to_reg().to_reg(), 8);
                let op = ljustify(format!(
                    "u64_to_{}_seq",
                    if *dst_size == OperandSize::Size64 {
                        "f64"
                    } else {
                        "f32"
                    }
                ));
                format!("{op} {src}, {dst}, {tmp_gpr1}, {tmp_gpr2}")
            }

            Inst::CvtFloatToSintSeq {
                src,
                dst,
                src_size,
                dst_size,
                tmp_xmm,
                tmp_gpr,
                is_saturating,
            } => {
                let src = pretty_print_reg(src.to_reg(), src_size.to_bytes());
                let dst = pretty_print_reg(dst.to_reg().to_reg(), dst_size.to_bytes());
                let tmp_gpr = pretty_print_reg(tmp_gpr.to_reg().to_reg(), 8);
                let tmp_xmm = pretty_print_reg(tmp_xmm.to_reg().to_reg(), 8);
                let op = ljustify(format!(
                    "cvt_float{}_to_sint{}{}_seq",
                    src_size.to_bits(),
                    dst_size.to_bits(),
                    if *is_saturating { "_sat" } else { "" },
                ));
                format!("{op} {src}, {dst}, {tmp_gpr}, {tmp_xmm}")
            }

            Inst::CvtFloatToUintSeq {
                src,
                dst,
                src_size,
                dst_size,
                tmp_gpr,
                tmp_xmm,
                tmp_xmm2,
                is_saturating,
            } => {
                let src = pretty_print_reg(src.to_reg(), src_size.to_bytes());
                let dst = pretty_print_reg(dst.to_reg().to_reg(), dst_size.to_bytes());
                let tmp_gpr = pretty_print_reg(tmp_gpr.to_reg().to_reg(), 8);
                let tmp_xmm = pretty_print_reg(tmp_xmm.to_reg().to_reg(), 8);
                let tmp_xmm2 = pretty_print_reg(tmp_xmm2.to_reg().to_reg(), 8);
                let op = ljustify(format!(
                    "cvt_float{}_to_uint{}{}_seq",
                    src_size.to_bits(),
                    dst_size.to_bits(),
                    if *is_saturating { "_sat" } else { "" },
                ));
                format!("{op} {src}, {dst}, {tmp_gpr}, {tmp_xmm}, {tmp_xmm2}")
            }

            Inst::MovFromPReg { src, dst } => {
                let src: Reg = (*src).into();
                let src = regs::show_ireg_sized(src, 8);
                let dst = pretty_print_reg(dst.to_reg().to_reg(), 8);
                let op = ljustify("movq".to_string());
                format!("{op} {src}, {dst}")
            }

            Inst::MovToPReg { src, dst } => {
                let src = pretty_print_reg(src.to_reg(), 8);
                let dst: Reg = (*dst).into();
                let dst = regs::show_ireg_sized(dst, 8);
                let op = ljustify("movq".to_string());
                format!("{op} {src}, {dst}")
            }

            Inst::XmmCmove {
                ty,
                cc,
                consequent,
                alternative,
                dst,
                ..
            } => {
                let size = u8::try_from(ty.bytes()).unwrap();
                let alternative = pretty_print_reg(alternative.to_reg(), size);
                let dst = pretty_print_reg(dst.to_reg().to_reg(), size);
                let consequent = pretty_print_reg(consequent.to_reg(), size);
                let suffix = match *ty {
                    types::F64 => "sd",
                    types::F32 => "ss",
                    types::F16 => "ss",
                    types::F32X4 => "aps",
                    types::F64X2 => "apd",
                    _ => "dqa",
                };
                let cc = cc.invert();
                format!(
                    "mov{suffix} {alternative}, {dst}; \
                    j{cc} $next; \
                    mov{suffix} {consequent}, {dst}; \
                    $next:"
                )
            }

            Inst::StackProbeLoop {
                tmp,
                frame_size,
                guard_size,
            } => {
                let tmp = pretty_print_reg(tmp.to_reg(), 8);
                let op = ljustify("stack_probe_loop".to_string());
                format!("{op} {tmp}, frame_size={frame_size}, guard_size={guard_size}")
            }

            Inst::CallKnown { info } => {
                let op = ljustify("call".to_string());
                let try_call = info
                    .try_call_info
                    .as_ref()
                    .map(|tci| pretty_print_try_call(tci))
                    .unwrap_or_default();
                format!("{op} {:?}{try_call}", info.dest)
            }

            Inst::CallUnknown { info } => {
                let dest = info.dest.pretty_print(8);
                let op = ljustify("call".to_string());
                let try_call = info
                    .try_call_info
                    .as_ref()
                    .map(|tci| pretty_print_try_call(tci))
                    .unwrap_or_default();
                format!("{op} *{dest}{try_call}")
            }

            Inst::ReturnCallKnown { info } => {
                let ReturnCallInfo {
                    uses,
                    new_stack_arg_size,
                    tmp,
                    dest,
                } = &**info;
                let tmp = pretty_print_reg(tmp.to_reg().to_reg(), 8);
                let mut s = format!("return_call_known {dest:?} ({new_stack_arg_size}) tmp={tmp}");
                for ret in uses {
                    let preg = regs::show_reg(ret.preg);
                    let vreg = pretty_print_reg(ret.vreg, 8);
                    write!(&mut s, " {vreg}={preg}").unwrap();
                }
                s
            }

            Inst::ReturnCallUnknown { info } => {
                let ReturnCallInfo {
                    uses,
                    new_stack_arg_size,
                    tmp,
                    dest,
                } = &**info;
                let callee = pretty_print_reg(*dest, 8);
                let tmp = pretty_print_reg(tmp.to_reg().to_reg(), 8);
                let mut s =
                    format!("return_call_unknown {callee} ({new_stack_arg_size}) tmp={tmp}");
                for ret in uses {
                    let preg = regs::show_reg(ret.preg);
                    let vreg = pretty_print_reg(ret.vreg, 8);
                    write!(&mut s, " {vreg}={preg}").unwrap();
                }
                s
            }

            Inst::Args { args } => {
                let mut s = "args".to_string();
                for arg in args {
                    let preg = regs::show_reg(arg.preg);
                    let def = pretty_print_reg(arg.vreg.to_reg(), 8);
                    write!(&mut s, " {def}={preg}").unwrap();
                }
                s
            }

            Inst::Rets { rets } => {
                let mut s = "rets".to_string();
                for ret in rets {
                    let preg = regs::show_reg(ret.preg);
                    let vreg = pretty_print_reg(ret.vreg, 8);
                    write!(&mut s, " {vreg}={preg}").unwrap();
                }
                s
            }

            Inst::StackSwitchBasic {
                store_context_ptr,
                load_context_ptr,
                in_payload0,
                out_payload0,
            } => {
                let store_context_ptr = pretty_print_reg(**store_context_ptr, 8);
                let load_context_ptr = pretty_print_reg(**load_context_ptr, 8);
                let in_payload0 = pretty_print_reg(**in_payload0, 8);
                let out_payload0 = pretty_print_reg(*out_payload0.to_reg(), 8);
                format!(
                    "{out_payload0} = stack_switch_basic {store_context_ptr}, {load_context_ptr}, {in_payload0}"
                )
            }

            Inst::JmpKnown { dst } => {
                let op = ljustify("jmp".to_string());
                let dst = dst.to_string();
                format!("{op} {dst}")
            }

            Inst::WinchJmpIf { cc, taken } => {
                let taken = taken.to_string();
                let op = ljustify2("j".to_string(), cc.to_string());
                format!("{op} {taken}")
            }

            Inst::JmpCondOr {
                cc1,
                cc2,
                taken,
                not_taken,
            } => {
                let taken = taken.to_string();
                let not_taken = not_taken.to_string();
                let op = ljustify(format!("j{cc1},{cc2}"));
                format!("{op} {taken}; j {not_taken}")
            }

            Inst::JmpCond {
                cc,
                taken,
                not_taken,
            } => {
                let taken = taken.to_string();
                let not_taken = not_taken.to_string();
                let op = ljustify2("j".to_string(), cc.to_string());
                format!("{op} {taken}; j {not_taken}")
            }

            Inst::JmpTableSeq {
                idx, tmp1, tmp2, ..
            } => {
                let idx = pretty_print_reg(*idx, 8);
                let tmp1 = pretty_print_reg(tmp1.to_reg(), 8);
                let tmp2 = pretty_print_reg(tmp2.to_reg(), 8);
                let op = ljustify("br_table".into());
                format!("{op} {idx}, {tmp1}, {tmp2}")
            }

            Inst::TrapIf { cc, trap_code, .. } => {
                format!("j{cc} #trap={trap_code}")
            }

            Inst::TrapIfAnd {
                cc1,
                cc2,
                trap_code,
                ..
            } => {
                let cc1 = cc1.invert();
                let cc2 = cc2.invert();
                format!("trap_if_and {cc1}, {cc2}, {trap_code}")
            }

            Inst::TrapIfOr {
                cc1,
                cc2,
                trap_code,
                ..
            } => {
                let cc2 = cc2.invert();
                format!("trap_if_or {cc1}, {cc2}, {trap_code}")
            }

            Inst::LoadExtName {
                dst, name, offset, ..
            } => {
                let dst = pretty_print_reg(*dst.to_reg(), 8);
                let name = name.display(None);
                let op = ljustify("load_ext_name".into());
                format!("{op} {name}+{offset}, {dst}")
            }

            Inst::AtomicRmwSeq { ty, op, .. } => {
                let ty = ty.bits();
                format!(
                    "atomically {{ {ty}_bits_at_[%r9] {op:?}= %r10; %rax = old_value_at_[%r9]; %r11, %rflags = trash }}"
                )
            }

            Inst::Atomic128RmwSeq {
                op,
                mem,
                operand_low,
                operand_high,
                temp_low,
                temp_high,
                dst_old_low,
                dst_old_high,
            } => {
                let operand_low = pretty_print_reg(**operand_low, 8);
                let operand_high = pretty_print_reg(**operand_high, 8);
                let temp_low = pretty_print_reg(*temp_low.to_reg(), 8);
                let temp_high = pretty_print_reg(*temp_high.to_reg(), 8);
                let dst_old_low = pretty_print_reg(*dst_old_low.to_reg(), 8);
                let dst_old_high = pretty_print_reg(*dst_old_high.to_reg(), 8);
                let mem = mem.pretty_print(16);
                format!(
                    "atomically {{ {dst_old_high}:{dst_old_low} = {mem}; {temp_high}:{temp_low} = {dst_old_high}:{dst_old_low} {op:?} {operand_high}:{operand_low}; {mem} = {temp_high}:{temp_low} }}"
                )
            }

            Inst::Atomic128XchgSeq {
                mem,
                operand_low,
                operand_high,
                dst_old_low,
                dst_old_high,
            } => {
                let operand_low = pretty_print_reg(**operand_low, 8);
                let operand_high = pretty_print_reg(**operand_high, 8);
                let dst_old_low = pretty_print_reg(*dst_old_low.to_reg(), 8);
                let dst_old_high = pretty_print_reg(*dst_old_high.to_reg(), 8);
                let mem = mem.pretty_print(16);
                format!(
                    "atomically {{ {dst_old_high}:{dst_old_low} = {mem}; {mem} = {operand_high}:{operand_low} }}"
                )
            }

            Inst::ElfTlsGetAddr { symbol, dst } => {
                let dst = pretty_print_reg(dst.to_reg().to_reg(), 8);
                format!("{dst} = elf_tls_get_addr {symbol:?}")
            }

            Inst::MachOTlsGetAddr { symbol, dst } => {
                let dst = pretty_print_reg(dst.to_reg().to_reg(), 8);
                format!("{dst} = macho_tls_get_addr {symbol:?}")
            }

            Inst::CoffTlsGetAddr { symbol, dst, tmp } => {
                let dst = pretty_print_reg(dst.to_reg().to_reg(), 8);
                let tmp = tmp.to_reg().to_reg();

                let mut s = format!("{dst} = coff_tls_get_addr {symbol:?}");
                if tmp.is_virtual() {
                    let tmp = show_ireg_sized(tmp, 8);
                    write!(&mut s, ", {tmp}").unwrap();
                };

                s
            }

            Inst::Unwind { inst } => format!("unwind {inst:?}"),

            Inst::DummyUse { reg } => {
                let reg = pretty_print_reg(*reg, 8);
                format!("dummy_use {reg}")
            }

            Inst::External { inst } => {
                format!("{inst}")
            }
        }
    }
}

fn pretty_print_try_call(info: &TryCallInfo) -> String {
    format!(
        "; jmp {:?}; catch [{}]",
        info.continuation,
        info.pretty_print_dests()
    )
}

impl fmt::Debug for Inst {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.pretty_print_inst(&mut Default::default()))
    }
}

fn x64_get_operands(inst: &mut Inst, collector: &mut impl OperandVisitor) {
    // Note: because we need to statically know the indices of each
    // reg in the operands list in order to fetch its allocation
    // later, we put the variable-operand-count bits (the RegMem,
    // RegMemImm, etc args) last. regalloc2 doesn't care what order
    // the operands come in; they can be freely reordered.

    // N.B.: we MUST keep the below in careful sync with (i) emission,
    // in `emit.rs`, and (ii) pretty-printing, in the `pretty_print`
    // method above.
    match inst {
        Inst::CheckedSRemSeq {
            divisor,
            dividend_lo,
            dividend_hi,
            dst_quotient,
            dst_remainder,
            ..
        } => {
            collector.reg_use(divisor);
            collector.reg_fixed_use(dividend_lo, regs::rax());
            collector.reg_fixed_use(dividend_hi, regs::rdx());
            collector.reg_fixed_def(dst_quotient, regs::rax());
            collector.reg_fixed_def(dst_remainder, regs::rdx());
        }
        Inst::CheckedSRemSeq8 {
            divisor,
            dividend,
            dst,
            ..
        } => {
            collector.reg_use(divisor);
            collector.reg_fixed_use(dividend, regs::rax());
            collector.reg_fixed_def(dst, regs::rax());
        }
        Inst::XmmUninitializedValue { dst } => collector.reg_def(dst),
        Inst::GprUninitializedValue { dst } => collector.reg_def(dst),
        Inst::XmmMinMaxSeq { lhs, rhs, dst, .. } => {
            collector.reg_use(rhs);
            collector.reg_use(lhs);
            collector.reg_reuse_def(dst, 0); // Reuse RHS.
        }
        Inst::MovFromPReg { dst, src } => {
            debug_assert!(dst.to_reg().to_reg().is_virtual());
            collector.reg_fixed_nonallocatable(*src);
            collector.reg_def(dst);
        }
        Inst::MovToPReg { dst, src } => {
            debug_assert!(src.to_reg().is_virtual());
            collector.reg_use(src);
            collector.reg_fixed_nonallocatable(*dst);
        }
        Inst::CvtUint64ToFloatSeq {
            src,
            dst,
            tmp_gpr1,
            tmp_gpr2,
            ..
        } => {
            collector.reg_use(src);
            collector.reg_early_def(dst);
            collector.reg_early_def(tmp_gpr1);
            collector.reg_early_def(tmp_gpr2);
        }
        Inst::CvtFloatToSintSeq {
            src,
            dst,
            tmp_xmm,
            tmp_gpr,
            ..
        } => {
            collector.reg_use(src);
            collector.reg_early_def(dst);
            collector.reg_early_def(tmp_gpr);
            collector.reg_early_def(tmp_xmm);
        }
        Inst::CvtFloatToUintSeq {
            src,
            dst,
            tmp_gpr,
            tmp_xmm,
            tmp_xmm2,
            ..
        } => {
            collector.reg_use(src);
            collector.reg_early_def(dst);
            collector.reg_early_def(tmp_gpr);
            collector.reg_early_def(tmp_xmm);
            collector.reg_early_def(tmp_xmm2);
        }

        Inst::XmmCmove {
            consequent,
            alternative,
            dst,
            ..
        } => {
            collector.reg_use(alternative);
            collector.reg_reuse_def(dst, 0);
            collector.reg_use(consequent);
        }
        Inst::StackProbeLoop { tmp, .. } => {
            collector.reg_early_def(tmp);
        }

        Inst::CallKnown { info } => {
            // Probestack is special and is only inserted after
            // regalloc, so we do not need to represent its ABI to the
            // register allocator. Assert that we don't alter that
            // arrangement.
            let CallInfo {
                uses,
                defs,
                clobbers,
                dest,
                try_call_info,
                ..
            } = &mut **info;
            debug_assert_ne!(*dest, ExternalName::LibCall(LibCall::Probestack));
            for CallArgPair { vreg, preg } in uses {
                collector.reg_fixed_use(vreg, *preg);
            }
            for CallRetPair { vreg, location } in defs {
                match location {
                    RetLocation::Reg(preg, ..) => collector.reg_fixed_def(vreg, *preg),
                    RetLocation::Stack(..) => collector.any_def(vreg),
                }
            }
            collector.reg_clobbers(*clobbers);
            if let Some(try_call_info) = try_call_info {
                try_call_info.collect_operands(collector);
            }
        }

        Inst::CallUnknown { info } => {
            let CallInfo {
                uses,
                defs,
                clobbers,
                callee_conv,
                dest,
                try_call_info,
                ..
            } = &mut **info;
            match dest {
                RegMem::Reg { reg } if *callee_conv == CallConv::Winch => {
                    // TODO(https://github.com/bytecodealliance/regalloc2/issues/145):
                    // This shouldn't be a fixed register constraint. r10 is caller-saved, so this
                    // should be safe to use.
                    collector.reg_fixed_use(reg, regs::r10());
                }
                _ => dest.get_operands(collector),
            }
            for CallArgPair { vreg, preg } in uses {
                collector.reg_fixed_use(vreg, *preg);
            }
            for CallRetPair { vreg, location } in defs {
                match location {
                    RetLocation::Reg(preg, ..) => collector.reg_fixed_def(vreg, *preg),
                    RetLocation::Stack(..) => collector.any_def(vreg),
                }
            }
            collector.reg_clobbers(*clobbers);
            if let Some(try_call_info) = try_call_info {
                try_call_info.collect_operands(collector);
            }
        }
        Inst::StackSwitchBasic {
            store_context_ptr,
            load_context_ptr,
            in_payload0,
            out_payload0,
        } => {
            collector.reg_use(load_context_ptr);
            collector.reg_use(store_context_ptr);
            collector.reg_fixed_use(in_payload0, stack_switch::payload_register());
            collector.reg_fixed_def(out_payload0, stack_switch::payload_register());

            let mut clobbers = crate::isa::x64::abi::ALL_CLOBBERS;
            // The return/payload reg must not be included in the clobber set
            clobbers.remove(
                stack_switch::payload_register()
                    .to_real_reg()
                    .unwrap()
                    .into(),
            );
            collector.reg_clobbers(clobbers);
        }

        Inst::ReturnCallKnown { info } => {
            let ReturnCallInfo {
                dest, uses, tmp, ..
            } = &mut **info;
            collector.reg_fixed_def(tmp, regs::r11());
            // Same as in the `Inst::CallKnown` branch.
            debug_assert_ne!(*dest, ExternalName::LibCall(LibCall::Probestack));
            for CallArgPair { vreg, preg } in uses {
                collector.reg_fixed_use(vreg, *preg);
            }
        }

        Inst::ReturnCallUnknown { info } => {
            let ReturnCallInfo {
                dest, uses, tmp, ..
            } = &mut **info;

            // TODO(https://github.com/bytecodealliance/regalloc2/issues/145):
            // This shouldn't be a fixed register constraint, but it's not clear how to
            // pick a register that won't be clobbered by the callee-save restore code
            // emitted with a return_call_indirect. r10 is caller-saved, so this should be
            // safe to use.
            collector.reg_fixed_use(dest, regs::r10());

            collector.reg_fixed_def(tmp, regs::r11());
            for CallArgPair { vreg, preg } in uses {
                collector.reg_fixed_use(vreg, *preg);
            }
        }

        Inst::JmpTableSeq {
            idx, tmp1, tmp2, ..
        } => {
            collector.reg_use(idx);
            collector.reg_early_def(tmp1);
            // In the sequence emitted for this pseudoinstruction in emit.rs,
            // tmp2 is only written after idx is read, so it doesn't need to be
            // an early def.
            collector.reg_def(tmp2);
        }

        Inst::LoadExtName { dst, .. } => {
            collector.reg_def(dst);
        }

        Inst::AtomicRmwSeq {
            operand,
            temp,
            dst_old,
            mem,
            ..
        } => {
            collector.reg_late_use(operand);
            collector.reg_early_def(temp);
            // This `fixed_def` is needed because `CMPXCHG` always uses this
            // register implicitly.
            collector.reg_fixed_def(dst_old, regs::rax());
            mem.get_operands_late(collector)
        }

        Inst::Atomic128RmwSeq {
            operand_low,
            operand_high,
            temp_low,
            temp_high,
            dst_old_low,
            dst_old_high,
            mem,
            ..
        } => {
            // All registers are collected in the `Late` position so that they don't overlap.
            collector.reg_late_use(operand_low);
            collector.reg_late_use(operand_high);
            collector.reg_fixed_def(temp_low, regs::rbx());
            collector.reg_fixed_def(temp_high, regs::rcx());
            collector.reg_fixed_def(dst_old_low, regs::rax());
            collector.reg_fixed_def(dst_old_high, regs::rdx());
            mem.get_operands_late(collector)
        }

        Inst::Atomic128XchgSeq {
            operand_low,
            operand_high,
            dst_old_low,
            dst_old_high,
            mem,
            ..
        } => {
            // All registers are collected in the `Late` position so that they don't overlap.
            collector.reg_fixed_late_use(operand_low, regs::rbx());
            collector.reg_fixed_late_use(operand_high, regs::rcx());
            collector.reg_fixed_def(dst_old_low, regs::rax());
            collector.reg_fixed_def(dst_old_high, regs::rdx());
            mem.get_operands_late(collector)
        }

        Inst::Args { args } => {
            for ArgPair { vreg, preg } in args {
                collector.reg_fixed_def(vreg, *preg);
            }
        }

        Inst::Rets { rets } => {
            // The return value(s) are live-out; we represent this
            // with register uses on the return instruction.
            for RetPair { vreg, preg } in rets {
                collector.reg_fixed_use(vreg, *preg);
            }
        }

        Inst::JmpKnown { .. }
        | Inst::WinchJmpIf { .. }
        | Inst::JmpCond { .. }
        | Inst::JmpCondOr { .. }
        | Inst::TrapIf { .. }
        | Inst::TrapIfAnd { .. }
        | Inst::TrapIfOr { .. } => {
            // No registers are used.
        }

        Inst::ElfTlsGetAddr { dst, .. } | Inst::MachOTlsGetAddr { dst, .. } => {
            collector.reg_fixed_def(dst, regs::rax());
            // All caller-saves are clobbered.
            //
            // We use the SysV calling convention here because the
            // pseudoinstruction (and relocation that it emits) is specific to
            // ELF systems; other x86-64 targets with other conventions (i.e.,
            // Windows) use different TLS strategies.
            let mut clobbers =
                X64ABIMachineSpec::get_regs_clobbered_by_call(CallConv::SystemV, false);
            clobbers.remove(regs::gpr_preg(regs::ENC_RAX));
            collector.reg_clobbers(clobbers);
        }

        Inst::CoffTlsGetAddr { dst, tmp, .. } => {
            // We also use the gs register. But that register is not allocatable by the
            // register allocator, so we don't need to mark it as used here.

            // We use %rax to set the address
            collector.reg_fixed_def(dst, regs::rax());

            // We use %rcx as a temporary variable to load the _tls_index
            collector.reg_fixed_def(tmp, regs::rcx());
        }

        Inst::Unwind { .. } => {}

        Inst::DummyUse { reg } => {
            collector.reg_use(reg);
        }

        Inst::External { inst } => {
            inst.visit(&mut external::RegallocVisitor { collector });
        }
    }
}

//=============================================================================
// Instructions: misc functions and external interface

impl MachInst for Inst {
    type ABIMachineSpec = X64ABIMachineSpec;

    fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        x64_get_operands(self, collector)
    }

    fn is_move(&self) -> Option<(Writable<Reg>, Reg)> {
        use asm::inst::Inst as I;
        match self {
            // Note (carefully!) that a 32-bit mov *isn't* a no-op since it zeroes
            // out the upper 32 bits of the destination.  For example, we could
            // conceivably use `movl %reg, %reg` to zero out the top 32 bits of
            // %reg.
            Self::External {
                inst: I::movq_mr(asm::inst::movq_mr { rm64, r64 }),
            } => match rm64 {
                asm::GprMem::Gpr(reg) => Some((reg.map(|r| r.to_reg()), r64.as_ref().to_reg())),
                asm::GprMem::Mem(_) => None,
            },
            Self::External {
                inst: I::movq_rm(asm::inst::movq_rm { r64, rm64 }),
            } => match rm64 {
                asm::GprMem::Gpr(reg) => Some((r64.as_ref().map(|r| r.to_reg()), reg.to_reg())),
                asm::GprMem::Mem(_) => None,
            },

            // Note that `movss_a_r` and `movsd_a_r` are specifically omitted
            // here because they only overwrite the low bits in the destination
            // register, otherwise preserving the upper bits. That can be used
            // for lane-insertion instructions, for example, meaning it's not
            // classified as a register move.
            //
            // Otherwise though all register-to-register movement instructions
            // which move 128-bits are registered as moves.
            Self::External {
                inst:
                    I::movaps_a(asm::inst::movaps_a { xmm1, xmm_m128 })
                    | I::movups_a(asm::inst::movups_a { xmm1, xmm_m128 })
                    | I::movapd_a(asm::inst::movapd_a { xmm1, xmm_m128 })
                    | I::movupd_a(asm::inst::movupd_a { xmm1, xmm_m128 })
                    | I::movdqa_a(asm::inst::movdqa_a { xmm1, xmm_m128 })
                    | I::movdqu_a(asm::inst::movdqu_a { xmm1, xmm_m128 }),
            } => match xmm_m128 {
                asm::XmmMem::Xmm(xmm2) => Some((xmm1.as_ref().map(|r| r.to_reg()), xmm2.to_reg())),
                asm::XmmMem::Mem(_) => None,
            },
            // In addition to the "A" format of instructions above also
            // recognize the "B" format which while it can be used for stores it
            // can also be used for register moves.
            Self::External {
                inst:
                    I::movaps_b(asm::inst::movaps_b { xmm_m128, xmm1 })
                    | I::movups_b(asm::inst::movups_b { xmm_m128, xmm1 })
                    | I::movapd_b(asm::inst::movapd_b { xmm_m128, xmm1 })
                    | I::movupd_b(asm::inst::movupd_b { xmm_m128, xmm1 })
                    | I::movdqa_b(asm::inst::movdqa_b { xmm_m128, xmm1 })
                    | I::movdqu_b(asm::inst::movdqu_b { xmm_m128, xmm1 }),
            } => match xmm_m128 {
                asm::XmmMem::Xmm(dst) => Some((dst.map(|r| r.to_reg()), xmm1.as_ref().to_reg())),
                asm::XmmMem::Mem(_) => None,
            },
            _ => None,
        }
    }

    fn is_included_in_clobbers(&self) -> bool {
        match self {
            &Inst::Args { .. } => false,
            _ => true,
        }
    }

    fn is_trap(&self) -> bool {
        match self {
            Self::External {
                inst: asm::inst::Inst::ud2_zo(..),
            } => true,
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
            // Interesting cases.
            &Self::Rets { .. } => MachTerminator::Ret,
            &Self::ReturnCallKnown { .. } | &Self::ReturnCallUnknown { .. } => {
                MachTerminator::RetCall
            }
            &Self::JmpKnown { .. } => MachTerminator::Branch,
            &Self::JmpCond { .. } => MachTerminator::Branch,
            &Self::JmpCondOr { .. } => MachTerminator::Branch,
            &Self::JmpTableSeq { .. } => MachTerminator::Branch,
            &Self::CallKnown { ref info } if info.try_call_info.is_some() => MachTerminator::Branch,
            &Self::CallUnknown { ref info } if info.try_call_info.is_some() => {
                MachTerminator::Branch
            }
            // All other cases are boring.
            _ => MachTerminator::None,
        }
    }

    fn is_low_level_branch(&self) -> bool {
        match self {
            &Self::WinchJmpIf { .. } => true,
            _ => false,
        }
    }

    fn is_mem_access(&self) -> bool {
        panic!("TODO FILL ME OUT")
    }

    fn gen_move(dst_reg: Writable<Reg>, src_reg: Reg, ty: Type) -> Inst {
        trace!(
            "Inst::gen_move {:?} -> {:?} (type: {:?})",
            src_reg,
            dst_reg.to_reg(),
            ty
        );
        let rc_dst = dst_reg.to_reg().class();
        let rc_src = src_reg.class();
        // If this isn't true, we have gone way off the rails.
        debug_assert!(rc_dst == rc_src);
        let inst = match rc_dst {
            RegClass::Int => {
                asm::inst::movq_mr::new(dst_reg.map(Gpr::unwrap_new), Gpr::unwrap_new(src_reg))
                    .into()
            }
            RegClass::Float => {
                // The Intel optimization manual, in "3.5.1.13 Zero-Latency MOV Instructions",
                // doesn't include MOVSS/MOVSD as instructions with zero-latency. Use movaps for
                // those, which may write more lanes that we need, but are specified to have
                // zero-latency.
                let dst_reg = dst_reg.map(|r| Xmm::new(r).unwrap());
                let src_reg = Xmm::new(src_reg).unwrap();
                match ty {
                    types::F16 | types::F32 | types::F64 | types::F32X4 => {
                        asm::inst::movaps_a::new(dst_reg, src_reg).into()
                    }
                    types::F64X2 => asm::inst::movapd_a::new(dst_reg, src_reg).into(),
                    _ if (ty.is_float() || ty.is_vector()) && ty.bits() <= 128 => {
                        asm::inst::movdqa_a::new(dst_reg, src_reg).into()
                    }
                    _ => unimplemented!("unable to move type: {}", ty),
                }
            }
            RegClass::Vector => unreachable!(),
        };
        Inst::External { inst }
    }

    fn gen_nop(preferred_size: usize) -> Inst {
        Inst::nop(std::cmp::min(preferred_size, 9) as u8)
    }

    fn rc_for_type(ty: Type) -> CodegenResult<(&'static [RegClass], &'static [Type])> {
        match ty {
            types::I8 => Ok((&[RegClass::Int], &[types::I8])),
            types::I16 => Ok((&[RegClass::Int], &[types::I16])),
            types::I32 => Ok((&[RegClass::Int], &[types::I32])),
            types::I64 => Ok((&[RegClass::Int], &[types::I64])),
            types::F16 => Ok((&[RegClass::Float], &[types::F16])),
            types::F32 => Ok((&[RegClass::Float], &[types::F32])),
            types::F64 => Ok((&[RegClass::Float], &[types::F64])),
            types::F128 => Ok((&[RegClass::Float], &[types::F128])),
            types::I128 => Ok((&[RegClass::Int, RegClass::Int], &[types::I64, types::I64])),
            _ if ty.is_vector() && ty.bits() <= 128 => {
                let types = &[types::I8X2, types::I8X4, types::I8X8, types::I8X16];
                Ok((
                    &[RegClass::Float],
                    slice::from_ref(&types[ty.bytes().ilog2() as usize - 1]),
                ))
            }
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

    fn gen_jump(label: MachLabel) -> Inst {
        Inst::jmp_known(label)
    }

    fn gen_imm_u64(value: u64, dst: Writable<Reg>) -> Option<Self> {
        Some(Inst::imm(OperandSize::Size64, value, dst))
    }

    fn gen_imm_f64(value: f64, tmp: Writable<Reg>, dst: Writable<Reg>) -> SmallVec<[Self; 2]> {
        let imm_to_gpr = Inst::imm(OperandSize::Size64, value.to_bits(), tmp);
        let gpr_to_xmm = Inst::External {
            inst: asm::inst::movq_a::new(dst.map(|r| Xmm::new(r).unwrap()), tmp.to_reg()).into(),
        };
        smallvec![imm_to_gpr, gpr_to_xmm]
    }

    fn gen_dummy_use(reg: Reg) -> Self {
        Inst::DummyUse { reg }
    }

    fn worst_case_size() -> CodeOffset {
        15
    }

    fn ref_type_regclass(_: &settings::Flags) -> RegClass {
        RegClass::Int
    }

    fn is_safepoint(&self) -> bool {
        match self {
            Inst::CallKnown { .. } | Inst::CallUnknown { .. } => true,
            _ => false,
        }
    }

    fn function_alignment() -> FunctionAlignment {
        FunctionAlignment {
            minimum: 1,
            // Change the alignment from 16-bytes to 32-bytes for better performance.
            // fix-8573: https://github.com/bytecodealliance/wasmtime/issues/8573
            preferred: 32,
        }
    }

    type LabelUse = LabelUse;

    const TRAP_OPCODE: &'static [u8] = &[0x0f, 0x0b];
}

/// Constant state used during emissions of a sequence of instructions.
pub struct EmitInfo {
    pub(super) flags: settings::Flags,
    isa_flags: x64_settings::Flags,
}

impl EmitInfo {
    /// Create a constant state for emission of instructions.
    pub fn new(flags: settings::Flags, isa_flags: x64_settings::Flags) -> Self {
        Self { flags, isa_flags }
    }
}

impl MachInstEmit for Inst {
    type State = EmitState;
    type Info = EmitInfo;

    fn emit(&self, sink: &mut MachBuffer<Inst>, info: &Self::Info, state: &mut Self::State) {
        emit::emit(self, sink, info, state);
    }

    fn pretty_print_inst(&self, _: &mut Self::State) -> String {
        PrettyPrint::pretty_print(self, 0)
    }
}

/// A label-use (internal relocation) in generated code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelUse {
    /// A 32-bit offset from location of relocation itself, added to the existing value at that
    /// location. Used for control flow instructions which consider an offset from the start of the
    /// next instruction (so the size of the payload -- 4 bytes -- is subtracted from the payload).
    JmpRel32,

    /// A 32-bit offset from location of relocation itself, added to the existing value at that
    /// location.
    PCRel32,
}

impl MachInstLabelUse for LabelUse {
    const ALIGN: CodeOffset = 1;

    fn max_pos_range(self) -> CodeOffset {
        match self {
            LabelUse::JmpRel32 | LabelUse::PCRel32 => 0x7fff_ffff,
        }
    }

    fn max_neg_range(self) -> CodeOffset {
        match self {
            LabelUse::JmpRel32 | LabelUse::PCRel32 => 0x8000_0000,
        }
    }

    fn patch_size(self) -> CodeOffset {
        match self {
            LabelUse::JmpRel32 | LabelUse::PCRel32 => 4,
        }
    }

    fn patch(self, buffer: &mut [u8], use_offset: CodeOffset, label_offset: CodeOffset) {
        let pc_rel = (label_offset as i64) - (use_offset as i64);
        debug_assert!(pc_rel <= self.max_pos_range() as i64);
        debug_assert!(pc_rel >= -(self.max_neg_range() as i64));
        let pc_rel = pc_rel as u32;
        match self {
            LabelUse::JmpRel32 => {
                let addend = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let value = pc_rel.wrapping_add(addend).wrapping_sub(4);
                buffer.copy_from_slice(&value.to_le_bytes()[..]);
            }
            LabelUse::PCRel32 => {
                let addend = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                let value = pc_rel.wrapping_add(addend);
                buffer.copy_from_slice(&value.to_le_bytes()[..]);
            }
        }
    }

    fn supports_veneer(self) -> bool {
        match self {
            LabelUse::JmpRel32 | LabelUse::PCRel32 => false,
        }
    }

    fn veneer_size(self) -> CodeOffset {
        match self {
            LabelUse::JmpRel32 | LabelUse::PCRel32 => 0,
        }
    }

    fn worst_case_veneer_size() -> CodeOffset {
        0
    }

    fn generate_veneer(self, _: &mut [u8], _: CodeOffset) -> (CodeOffset, LabelUse) {
        match self {
            LabelUse::JmpRel32 | LabelUse::PCRel32 => {
                panic!("Veneer not supported for JumpRel32 label-use.");
            }
        }
    }

    fn from_reloc(reloc: Reloc, addend: Addend) -> Option<Self> {
        match (reloc, addend) {
            (Reloc::X86CallPCRel4, -4) => Some(LabelUse::JmpRel32),
            _ => None,
        }
    }
}

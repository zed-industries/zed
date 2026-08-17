//! Proof-carrying-code validation for x64 VCode.

use crate::ir::pcc::*;
use crate::ir::types::*;
use crate::isa::x64::inst::Inst;
use crate::isa::x64::inst::args::{Amode, Gpr, RegMem, SyntheticAmode, ToWritableReg};
use crate::machinst::pcc::*;
use crate::machinst::{InsnIndex, VCode};
use crate::machinst::{Reg, Writable};
use crate::trace;

fn undefined_result(
    ctx: &FactContext,
    vcode: &mut VCode<Inst>,
    dst: Writable<Gpr>,
    reg_bits: u16,
    result_bits: u16,
) -> PccResult<()> {
    check_output(ctx, vcode, dst.to_writable_reg(), &[], |_vcode| {
        clamp_range(ctx, reg_bits, result_bits, None)
    })
}

fn ensure_no_fact(vcode: &VCode<Inst>, reg: Reg) -> PccResult<()> {
    if vcode.vreg_fact(reg.into()).is_some() {
        Err(PccError::UnsupportedFact)
    } else {
        Ok(())
    }
}

/// Flow-state between facts.
#[derive(Clone, Debug, Default)]
pub(crate) struct FactFlowState {
    cmp_flags: Option<(Fact, Fact)>,
}

pub(crate) fn check(
    ctx: &FactContext,
    vcode: &mut VCode<Inst>,
    inst_idx: InsnIndex,
    state: &mut FactFlowState,
) -> PccResult<()> {
    trace!("Checking facts on inst: {:?}", vcode[inst_idx]);

    // We only persist flag state for one instruction, because we
    // can't exhaustively enumerate all flags-effecting ops; so take
    // the `cmp_state` here and perhaps use it below but don't let it
    // remain.
    let _cmp_flags = state.cmp_flags.take();

    match vcode[inst_idx] {
        Inst::Args { .. } => {
            // Defs on the args have "axiomatic facts": we trust the
            // ABI code to pass through the values unharmed, so the
            // facts given to us in the CLIF should still be true.
            Ok(())
        }

        Inst::CheckedSRemSeq {
            dst_quotient,
            dst_remainder,
            ..
        } => {
            undefined_result(ctx, vcode, dst_quotient, 64, 64)?;
            undefined_result(ctx, vcode, dst_remainder, 64, 64)?;
            Ok(())
        }

        Inst::CheckedSRemSeq8 { dst, .. } => undefined_result(ctx, vcode, dst, 64, 64),

        Inst::MovFromPReg { dst, .. } => undefined_result(ctx, vcode, dst, 64, 64),
        Inst::MovToPReg { .. } => Ok(()),

        Inst::XmmCmove { dst, .. } => ensure_no_fact(vcode, dst.to_writable_reg().to_reg()),

        Inst::StackProbeLoop { tmp, .. } => ensure_no_fact(vcode, tmp.to_reg()),

        Inst::CvtUint64ToFloatSeq {
            dst,
            tmp_gpr1,
            tmp_gpr2,
            ..
        } => {
            ensure_no_fact(vcode, dst.to_writable_reg().to_reg())?;
            ensure_no_fact(vcode, tmp_gpr1.to_writable_reg().to_reg())?;
            ensure_no_fact(vcode, tmp_gpr2.to_writable_reg().to_reg())?;
            Ok(())
        }

        Inst::CvtFloatToSintSeq {
            dst,
            tmp_gpr,
            tmp_xmm,
            ..
        } => {
            undefined_result(ctx, vcode, dst, 64, 64)?;
            ensure_no_fact(vcode, tmp_gpr.to_writable_reg().to_reg())?;
            ensure_no_fact(vcode, tmp_xmm.to_writable_reg().to_reg())?;
            Ok(())
        }

        Inst::CvtFloatToUintSeq {
            dst,
            tmp_gpr,
            tmp_xmm,
            tmp_xmm2,
            ..
        } => {
            undefined_result(ctx, vcode, dst, 64, 64)?;
            ensure_no_fact(vcode, tmp_gpr.to_writable_reg().to_reg())?;
            ensure_no_fact(vcode, tmp_xmm.to_writable_reg().to_reg())?;
            ensure_no_fact(vcode, tmp_xmm2.to_writable_reg().to_reg())?;
            Ok(())
        }

        Inst::XmmMinMaxSeq { dst, .. } => ensure_no_fact(vcode, dst.to_writable_reg().to_reg()),

        Inst::CallKnown { .. }
        | Inst::ReturnCallKnown { .. }
        | Inst::JmpKnown { .. }
        | Inst::WinchJmpIf { .. }
        | Inst::JmpCond { .. }
        | Inst::JmpCondOr { .. }
        | Inst::TrapIf { .. }
        | Inst::TrapIfAnd { .. }
        | Inst::TrapIfOr { .. } => Ok(()),
        Inst::Rets { .. } => Ok(()),

        Inst::ReturnCallUnknown { .. } => Ok(()),

        Inst::CallUnknown { ref info } => match &info.dest {
            RegMem::Mem { addr } => {
                check_load(ctx, None, addr, vcode, I64, 64)?;
                Ok(())
            }
            RegMem::Reg { .. } => Ok(()),
        },

        Inst::JmpTableSeq { tmp1, tmp2, .. } => {
            ensure_no_fact(vcode, tmp1.to_reg())?;
            ensure_no_fact(vcode, tmp2.to_reg())?;
            Ok(())
        }

        Inst::LoadExtName { dst, .. } => {
            ensure_no_fact(vcode, *dst.to_reg())?;
            Ok(())
        }

        Inst::AtomicRmwSeq {
            ref mem,
            temp,
            dst_old,
            ..
        } => {
            ensure_no_fact(vcode, *dst_old.to_reg())?;
            ensure_no_fact(vcode, *temp.to_reg())?;
            check_store(ctx, None, mem, vcode, I64)?;
            Ok(())
        }

        Inst::Atomic128RmwSeq {
            ref mem,
            temp_low,
            temp_high,
            dst_old_low,
            dst_old_high,
            ..
        } => {
            ensure_no_fact(vcode, *dst_old_low.to_reg())?;
            ensure_no_fact(vcode, *dst_old_high.to_reg())?;
            ensure_no_fact(vcode, *temp_low.to_reg())?;
            ensure_no_fact(vcode, *temp_high.to_reg())?;
            check_store(ctx, None, mem, vcode, I128)?;
            Ok(())
        }

        Inst::Atomic128XchgSeq {
            ref mem,
            dst_old_low,
            dst_old_high,
            ..
        } => {
            ensure_no_fact(vcode, *dst_old_low.to_reg())?;
            ensure_no_fact(vcode, *dst_old_high.to_reg())?;
            check_store(ctx, None, mem, vcode, I128)?;
            Ok(())
        }

        Inst::XmmUninitializedValue { dst } => {
            ensure_no_fact(vcode, dst.to_writable_reg().to_reg())
        }

        Inst::GprUninitializedValue { dst } => {
            ensure_no_fact(vcode, dst.to_writable_reg().to_reg())
        }

        Inst::ElfTlsGetAddr { dst, .. } | Inst::MachOTlsGetAddr { dst, .. } => {
            ensure_no_fact(vcode, dst.to_writable_reg().to_reg())
        }
        Inst::CoffTlsGetAddr { dst, tmp, .. } => {
            ensure_no_fact(vcode, dst.to_writable_reg().to_reg())?;
            ensure_no_fact(vcode, tmp.to_writable_reg().to_reg())?;
            Ok(())
        }

        Inst::Unwind { .. } | Inst::DummyUse { .. } => Ok(()),

        Inst::StackSwitchBasic { .. } => Err(PccError::UnimplementedInst),

        Inst::External { .. } => Ok(()), // TODO: unsure what to do about this!
    }
}

fn check_load(
    ctx: &FactContext,
    dst: Option<Writable<Reg>>,
    src: &SyntheticAmode,
    vcode: &VCode<Inst>,
    ty: Type,
    to_bits: u16,
) -> PccResult<Option<Fact>> {
    let result_fact = dst.and_then(|dst| vcode.vreg_fact(dst.to_reg().into()));
    let from_bits = u16::try_from(ty.bits()).unwrap();
    check_mem(
        ctx,
        src,
        vcode,
        ty,
        LoadOrStore::Load {
            result_fact,
            from_bits,
            to_bits,
        },
    )
}

fn check_store(
    ctx: &FactContext,
    data: Option<Reg>,
    dst: &SyntheticAmode,
    vcode: &VCode<Inst>,
    ty: Type,
) -> PccResult<()> {
    let stored_fact = data.and_then(|data| vcode.vreg_fact(data.into()));
    check_mem(ctx, dst, vcode, ty, LoadOrStore::Store { stored_fact }).map(|_| ())
}

fn check_mem<'a>(
    ctx: &FactContext,
    amode: &SyntheticAmode,
    vcode: &VCode<Inst>,
    ty: Type,
    op: LoadOrStore<'a>,
) -> PccResult<Option<Fact>> {
    let addr = match amode {
        SyntheticAmode::Real(amode) if amode.get_flags().checked() => {
            compute_addr(ctx, vcode, amode, 64).ok_or(PccError::MissingFact)?
        }
        _ => return Ok(None),
    };

    match op {
        LoadOrStore::Load {
            result_fact,
            from_bits,
            to_bits,
        } => {
            let loaded_fact = clamp_range(ctx, to_bits, from_bits, ctx.load(&addr, ty)?.cloned())?;
            trace!(
                "loaded_fact = {:?} result_fact = {:?}",
                loaded_fact, result_fact
            );
            if ctx.subsumes_fact_optionals(loaded_fact.as_ref(), result_fact) {
                Ok(loaded_fact.clone())
            } else {
                Err(PccError::UnsupportedFact)
            }
        }
        LoadOrStore::Store { stored_fact } => {
            ctx.store(&addr, ty, stored_fact)?;
            Ok(None)
        }
    }
}

fn compute_addr(ctx: &FactContext, vcode: &VCode<Inst>, amode: &Amode, bits: u16) -> Option<Fact> {
    trace!("compute_addr: {:?}", amode);
    match *amode {
        Amode::ImmReg { simm32, base, .. } => {
            let base = get_fact_or_default(vcode, base, bits);
            trace!("base = {:?}", base);
            let simm32: i64 = simm32.into();
            let simm32: u64 = simm32 as u64;
            let offset = Fact::constant(bits, simm32);
            let sum = ctx.add(&base, &offset, bits)?;
            trace!("sum = {:?}", sum);
            Some(sum)
        }
        Amode::ImmRegRegShift {
            simm32,
            base,
            index,
            shift,
            ..
        } => {
            let base = get_fact_or_default(vcode, base.into(), bits);
            let index = get_fact_or_default(vcode, index.into(), bits);
            trace!("base = {:?} index = {:?}", base, index);
            let shifted = ctx.shl(&index, bits, shift.into())?;
            let sum = ctx.add(&base, &shifted, bits)?;
            let simm32: i64 = simm32.into();
            let simm32: u64 = simm32 as u64;
            let offset = Fact::constant(bits, simm32);
            let sum = ctx.add(&sum, &offset, bits)?;
            trace!("sum = {:?}", sum);
            Some(sum)
        }
        Amode::RipRelative { .. } => None,
    }
}

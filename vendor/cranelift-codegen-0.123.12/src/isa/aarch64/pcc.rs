//! Proof-carrying code checking for AArch64 VCode.

use crate::ir::MemFlags;
use crate::ir::pcc::*;
use crate::ir::types::*;
use crate::isa::aarch64::inst::Inst;
use crate::isa::aarch64::inst::args::{Cond, PairAMode, ShiftOp};
use crate::isa::aarch64::inst::regs::zero_reg;
use crate::isa::aarch64::inst::{ALUOp, MoveWideOp};
use crate::isa::aarch64::inst::{AMode, ExtendOp};
use crate::machinst::Reg;
use crate::machinst::pcc::*;
use crate::machinst::{InsnIndex, VCode};
use crate::trace;

fn extend_fact(ctx: &FactContext, value: &Fact, mode: ExtendOp) -> Option<Fact> {
    match mode {
        ExtendOp::UXTB => ctx.uextend(value, 8, 64),
        ExtendOp::UXTH => ctx.uextend(value, 16, 64),
        ExtendOp::UXTW => ctx.uextend(value, 32, 64),
        ExtendOp::UXTX => Some(value.clone()),
        ExtendOp::SXTB => ctx.sextend(value, 8, 64),
        ExtendOp::SXTH => ctx.sextend(value, 16, 64),
        ExtendOp::SXTW => ctx.sextend(value, 32, 64),
        ExtendOp::SXTX => None,
    }
}

/// Flow-state between facts.
#[derive(Clone, Debug, Default)]
pub struct FactFlowState {
    cmp_flags: Option<(Fact, Fact)>,
}

pub(crate) fn check(
    ctx: &FactContext,
    vcode: &mut VCode<Inst>,
    inst_idx: InsnIndex,
    state: &mut FactFlowState,
) -> PccResult<()> {
    let inst = &vcode[inst_idx];
    trace!("Checking facts on inst: {:?}", inst);

    // We only persist flag state for one instruction, because we
    // can't exhaustively enumerate all flags-effecting ops; so take
    // the `cmp_state` here and perhaps use it below but don't let it
    // remain.
    let cmp_flags = state.cmp_flags.take();
    trace!(" * with cmp_flags = {cmp_flags:?}");

    match *inst {
        Inst::Args { .. } => {
            // Defs on the args have "axiomatic facts": we trust the
            // ABI code to pass through the values unharmed, so the
            // facts given to us in the CLIF should still be true.
            Ok(())
        }
        Inst::ULoad8 { rd, ref mem, flags }
        | Inst::SLoad8 { rd, ref mem, flags }
        | Inst::ULoad16 { rd, ref mem, flags }
        | Inst::SLoad16 { rd, ref mem, flags }
        | Inst::ULoad32 { rd, ref mem, flags }
        | Inst::SLoad32 { rd, ref mem, flags }
        | Inst::ULoad64 { rd, ref mem, flags } => {
            let access_ty = inst.mem_type().unwrap();
            check_load(ctx, Some(rd.to_reg()), flags, mem, vcode, access_ty)
        }
        Inst::FpuLoad16 { ref mem, flags, .. }
        | Inst::FpuLoad32 { ref mem, flags, .. }
        | Inst::FpuLoad64 { ref mem, flags, .. }
        | Inst::FpuLoad128 { ref mem, flags, .. } => {
            let access_ty = inst.mem_type().unwrap();
            check_load(ctx, None, flags, mem, vcode, access_ty)
        }
        Inst::LoadP64 { ref mem, flags, .. } => check_load_pair(ctx, flags, mem, vcode, 16),
        Inst::FpuLoadP64 { ref mem, flags, .. } => check_load_pair(ctx, flags, mem, vcode, 16),
        Inst::FpuLoadP128 { ref mem, flags, .. } => check_load_pair(ctx, flags, mem, vcode, 32),
        Inst::VecLoadReplicate {
            rn, flags, size, ..
        } => check_load_addr(ctx, flags, rn, vcode, size.lane_size().ty()),
        Inst::LoadAcquire {
            access_ty,
            rn,
            flags,
            ..
        } => check_load_addr(ctx, flags, rn, vcode, access_ty),

        Inst::Store8 { rd, ref mem, flags }
        | Inst::Store16 { rd, ref mem, flags }
        | Inst::Store32 { rd, ref mem, flags }
        | Inst::Store64 { rd, ref mem, flags } => {
            let access_ty = inst.mem_type().unwrap();
            check_store(ctx, Some(rd), flags, mem, vcode, access_ty)
        }
        Inst::FpuStore16 { ref mem, flags, .. }
        | Inst::FpuStore32 { ref mem, flags, .. }
        | Inst::FpuStore64 { ref mem, flags, .. }
        | Inst::FpuStore128 { ref mem, flags, .. } => {
            let access_ty = inst.mem_type().unwrap();
            check_store(ctx, None, flags, mem, vcode, access_ty)
        }
        Inst::StoreP64 { ref mem, flags, .. } => check_store_pair(ctx, flags, mem, vcode, 16),
        Inst::FpuStoreP64 { ref mem, flags, .. } => check_store_pair(ctx, flags, mem, vcode, 16),
        Inst::FpuStoreP128 { ref mem, flags, .. } => check_store_pair(ctx, flags, mem, vcode, 32),
        Inst::StoreRelease {
            access_ty,
            rn,
            flags,
            ..
        } => check_store_addr(ctx, flags, rn, vcode, access_ty),

        Inst::AluRRR {
            alu_op: ALUOp::Add | ALUOp::AddS,
            size,
            rd,
            rn,
            rm,
        } => check_binop(ctx, vcode, 64, rd, rn, rm, |rn, rm| {
            clamp_range(
                ctx,
                64,
                size.bits().into(),
                ctx.add(rn, rm, size.bits().into()),
            )
        }),
        Inst::AluRRImm12 {
            alu_op: ALUOp::Add | ALUOp::AddS,
            size,
            rd,
            rn,
            imm12,
        } => check_unop(ctx, vcode, 64, rd, rn, |rn| {
            let imm12: i64 = imm12.value().into();
            clamp_range(
                ctx,
                64,
                size.bits().into(),
                ctx.offset(&rn, size.bits().into(), imm12),
            )
        }),
        Inst::AluRRImm12 {
            alu_op: ALUOp::Sub,
            size,
            rd,
            rn,
            imm12,
        } => check_unop(ctx, vcode, 64, rd, rn, |rn| {
            let imm12: i64 = imm12.value().into();
            clamp_range(
                ctx,
                64,
                size.bits().into(),
                ctx.offset(&rn, size.bits().into(), -imm12),
            )
        }),
        Inst::AluRRR {
            alu_op: ALUOp::Sub,
            size,
            rd,
            rn,
            rm,
        } => check_binop(ctx, vcode, 64, rd, rn, rm, |rn, rm| {
            if let Some(k) = rm.as_const(64) {
                clamp_range(
                    ctx,
                    64,
                    size.bits().into(),
                    ctx.offset(rn, size.bits().into(), -(k as i64)),
                )
            } else {
                clamp_range(ctx, 64, size.bits().into(), None)
            }
        }),
        Inst::AluRRRShift {
            alu_op: ALUOp::Add | ALUOp::AddS,
            size,
            rd,
            rn,
            rm,
            shiftop,
        } if shiftop.op() == ShiftOp::LSL && has_fact(vcode, rn) && has_fact(vcode, rm) => {
            check_binop(ctx, vcode, 64, rd, rn, rm, |rn, rm| {
                let rm_shifted = fail_if_missing(ctx.shl(
                    &rm,
                    size.bits().into(),
                    shiftop.amt().value().into(),
                ))?;
                clamp_range(
                    ctx,
                    64,
                    size.bits().into(),
                    ctx.add(&rn, &rm_shifted, size.bits().into()),
                )
            })
        }
        Inst::AluRRRExtend {
            alu_op: ALUOp::Add | ALUOp::AddS,
            size,
            rd,
            rn,
            rm,
            extendop,
        } if has_fact(vcode, rn) && has_fact(vcode, rm) => {
            check_binop(ctx, vcode, 64, rd, rn, rm, |rn, rm| {
                let rm_extended = fail_if_missing(extend_fact(ctx, rm, extendop))?;
                clamp_range(
                    ctx,
                    64,
                    size.bits().into(),
                    ctx.add(&rn, &rm_extended, size.bits().into()),
                )
            })
        }
        Inst::AluRRImmShift {
            alu_op: ALUOp::Lsl,
            size,
            rd,
            rn,
            immshift,
        } if has_fact(vcode, rn) => check_unop(ctx, vcode, 64, rd, rn, |rn| {
            clamp_range(
                ctx,
                64,
                size.bits().into(),
                ctx.shl(&rn, size.bits().into(), immshift.value().into()),
            )
        }),
        Inst::Extend {
            rd,
            rn,
            signed: false,
            from_bits,
            to_bits,
        } if has_fact(vcode, rn) => check_unop(ctx, vcode, 64, rd, rn, |rn| {
            clamp_range(
                ctx,
                64,
                to_bits.into(),
                ctx.uextend(&rn, from_bits.into(), to_bits.into()),
            )
        }),

        Inst::AluRRR {
            alu_op: ALUOp::SubS,
            size,
            rd,
            rn,
            rm,
        } if rd.to_reg() == zero_reg() => {
            // Compare.
            let rn = get_fact_or_default(vcode, rn, size.bits().into());
            let rm = get_fact_or_default(vcode, rm, size.bits().into());
            state.cmp_flags = Some((rn, rm));
            Ok(())
        }

        Inst::AluRRImmLogic {
            alu_op: ALUOp::Orr,
            size,
            rd,
            rn,
            imml,
        } if rn == zero_reg() => {
            let constant = imml.value();
            check_constant(ctx, vcode, rd, size.bits().into(), constant)
        }

        Inst::AluRRR { rd, size, .. }
        | Inst::AluRRImm12 { rd, size, .. }
        | Inst::AluRRRShift { rd, size, .. }
        | Inst::AluRRRExtend { rd, size, .. }
        | Inst::AluRRImmLogic { rd, size, .. }
        | Inst::AluRRImmShift { rd, size, .. } => check_output(ctx, vcode, rd, &[], |_vcode| {
            clamp_range(ctx, 64, size.bits().into(), None)
        }),

        Inst::Extend {
            rd,
            from_bits,
            to_bits,
            ..
        } => check_output(ctx, vcode, rd, &[], |_vcode| {
            clamp_range(ctx, to_bits.into(), from_bits.into(), None)
        }),

        Inst::MovWide {
            op: MoveWideOp::MovZ,
            imm,
            size: _,
            rd,
        } => {
            let constant = u64::from(imm.bits) << (imm.shift * 16);
            check_constant(ctx, vcode, rd, 64, constant)
        }

        Inst::MovWide {
            op: MoveWideOp::MovN,
            imm,
            size,
            rd,
        } => {
            let constant = !(u64::from(imm.bits) << (imm.shift * 16)) & size.max_value();
            check_constant(ctx, vcode, rd, 64, constant)
        }

        Inst::MovK { rd, rn, imm, .. } => {
            let input = get_fact_or_default(vcode, rn, 64);
            if let Some(input_constant) = input.as_const(64) {
                let mask = 0xffff << (imm.shift * 16);
                let constant = u64::from(imm.bits) << (imm.shift * 16);
                let constant = (input_constant & !mask) | constant;
                check_constant(ctx, vcode, rd, 64, constant)
            } else {
                check_output(ctx, vcode, rd, &[], |_vcode| {
                    Ok(Some(Fact::max_range_for_width(64)))
                })
            }
        }

        Inst::CSel { rd, cond, rn, rm }
            if (cond == Cond::Hs || cond == Cond::Hi) && cmp_flags.is_some() =>
        {
            let (cmp_lhs, cmp_rhs) = cmp_flags.unwrap();
            trace!("CSel: cmp {cond:?} ({cmp_lhs:?}, {cmp_rhs:?})");

            check_output(ctx, vcode, rd, &[], |vcode| {
                // We support transitivity-based reasoning. If the
                // comparison establishes that
                //
                //   (x+K1) <= (y+K2)
                //
                // then on the true-side of the select we can edit the maximum
                // in a DynamicMem or DynamicRange by replacing x's with y's
                // with appropriate offsets -- this widens the range.
                //
                // Likewise, on the false-side of the select we can
                // replace y's with x's -- this also widens the range. On
                // the false side we know the inequality is strict, so we
                // can offset by one.

                // True side: lhs >= rhs (Hs) or lhs > rhs (Hi).
                let rn = get_fact_or_default(vcode, rn, 64);
                let lhs_kind = match cond {
                    Cond::Hs => InequalityKind::Loose,
                    Cond::Hi => InequalityKind::Strict,
                    _ => unreachable!(),
                };
                let rn = ctx.apply_inequality(&rn, &cmp_lhs, &cmp_rhs, lhs_kind);
                // false side: rhs < lhs (Hs) or rhs <= lhs (Hi).
                let rm = get_fact_or_default(vcode, rm, 64);
                let rhs_kind = match cond {
                    Cond::Hs => InequalityKind::Strict,
                    Cond::Hi => InequalityKind::Loose,
                    _ => unreachable!(),
                };
                let rm = ctx.apply_inequality(&rm, &cmp_rhs, &cmp_lhs, rhs_kind);
                let union = ctx.union(&rn, &rm);
                // Union the two facts.
                clamp_range(ctx, 64, 64, union)
            })
        }

        _ if vcode.inst_defines_facts(inst_idx) => Err(PccError::UnsupportedFact),

        _ => Ok(()),
    }
}

fn check_load(
    ctx: &FactContext,
    rd: Option<Reg>,
    flags: MemFlags,
    addr: &AMode,
    vcode: &VCode<Inst>,
    ty: Type,
) -> PccResult<()> {
    let result_fact = rd.and_then(|rd| vcode.vreg_fact(rd.into()));
    let bits = u16::try_from(ty.bits()).unwrap();
    check_addr(
        ctx,
        flags,
        addr,
        vcode,
        ty,
        LoadOrStore::Load {
            result_fact,
            from_bits: bits,
            to_bits: bits,
        },
    )
}

fn check_store(
    ctx: &FactContext,
    rd: Option<Reg>,
    flags: MemFlags,
    addr: &AMode,
    vcode: &VCode<Inst>,
    ty: Type,
) -> PccResult<()> {
    let stored_fact = rd.and_then(|rd| vcode.vreg_fact(rd.into()));
    check_addr(
        ctx,
        flags,
        addr,
        vcode,
        ty,
        LoadOrStore::Store { stored_fact },
    )
}

fn check_addr<'a>(
    ctx: &FactContext,
    flags: MemFlags,
    addr: &AMode,
    vcode: &VCode<Inst>,
    ty: Type,
    op: LoadOrStore<'a>,
) -> PccResult<()> {
    if !flags.checked() {
        return Ok(());
    }

    trace!("check_addr: {:?}", addr);

    let check = |addr: &Fact, ty: Type| -> PccResult<()> {
        match op {
            LoadOrStore::Load {
                result_fact,
                from_bits,
                to_bits,
            } => {
                let loaded_fact =
                    clamp_range(ctx, to_bits, from_bits, ctx.load(addr, ty)?.cloned())?;
                trace!(
                    "checking a load: loaded_fact = {loaded_fact:?} result_fact = {result_fact:?}"
                );
                if ctx.subsumes_fact_optionals(loaded_fact.as_ref(), result_fact) {
                    Ok(())
                } else {
                    Err(PccError::UnsupportedFact)
                }
            }
            LoadOrStore::Store { stored_fact } => ctx.store(addr, ty, stored_fact),
        }
    };

    match addr {
        &AMode::RegReg { rn, rm } => {
            let rn = get_fact_or_default(vcode, rn, 64);
            let rm = get_fact_or_default(vcode, rm, 64);
            let sum = fail_if_missing(ctx.add(&rn, &rm, 64))?;
            trace!("rn = {rn:?} rm = {rm:?} sum = {sum:?}");
            check(&sum, ty)
        }
        &AMode::RegScaled { rn, rm } => {
            let rn = get_fact_or_default(vcode, rn, 64);
            let rm = get_fact_or_default(vcode, rm, 64);
            let rm_scaled = fail_if_missing(ctx.scale(&rm, 64, ty.bytes()))?;
            let sum = fail_if_missing(ctx.add(&rn, &rm_scaled, 64))?;
            check(&sum, ty)
        }
        &AMode::RegScaledExtended { rn, rm, extendop } => {
            let rn = get_fact_or_default(vcode, rn, 64);
            let rm = get_fact_or_default(vcode, rm, 64);
            let rm_extended = fail_if_missing(extend_fact(ctx, &rm, extendop))?;
            let rm_scaled = fail_if_missing(ctx.scale(&rm_extended, 64, ty.bytes()))?;
            let sum = fail_if_missing(ctx.add(&rn, &rm_scaled, 64))?;
            check(&sum, ty)
        }
        &AMode::RegExtended { rn, rm, extendop } => {
            let rn = get_fact_or_default(vcode, rn, 64);
            let rm = get_fact_or_default(vcode, rm, 64);
            let rm_extended = fail_if_missing(extend_fact(ctx, &rm, extendop))?;
            let sum = fail_if_missing(ctx.add(&rn, &rm_extended, 64))?;
            check(&sum, ty)?;
            Ok(())
        }
        &AMode::Unscaled { rn, simm9 } => {
            let rn = get_fact_or_default(vcode, rn, 64);
            let sum = fail_if_missing(ctx.offset(&rn, 64, simm9.value.into()))?;
            check(&sum, ty)
        }
        &AMode::UnsignedOffset { rn, uimm12 } => {
            let rn = get_fact_or_default(vcode, rn, 64);
            // N.B.: the architecture scales the immediate in the
            // encoded instruction by the size of the loaded type, so
            // e.g. an offset field of 4095 can mean a load of offset
            // 32760 (= 4095 * 8) for I64s. The `UImm12Scaled` type
            // stores the *scaled* value, so we don't need to multiply
            // (again) by the type's size here.
            let offset: u64 = uimm12.value().into();
            // This `unwrap()` will always succeed because the value
            // will always be positive and much smaller than
            // `i64::MAX` (see above).
            let sum = fail_if_missing(ctx.offset(&rn, 64, i64::try_from(offset).unwrap()))?;
            check(&sum, ty)
        }
        &AMode::Label { .. } | &AMode::Const { .. } => {
            // Always accept: labels and constants must be within the
            // generated code (else they won't be resolved).
            Ok(())
        }
        &AMode::RegOffset { rn, off, .. } => {
            let rn = get_fact_or_default(vcode, rn, 64);
            let sum = fail_if_missing(ctx.offset(&rn, 64, off))?;
            check(&sum, ty)
        }
        &AMode::SPOffset { .. }
        | &AMode::FPOffset { .. }
        | &AMode::IncomingArg { .. }
        | &AMode::SlotOffset { .. }
        | &AMode::SPPostIndexed { .. }
        | &AMode::SPPreIndexed { .. } => {
            // We trust ABI code (for now!) and no lowering rules
            // lower input value accesses directly to these.
            Ok(())
        }
    }
}

fn check_load_pair(
    _ctx: &FactContext,
    _flags: MemFlags,
    _addr: &PairAMode,
    _vcode: &VCode<Inst>,
    _size: u8,
) -> PccResult<()> {
    Err(PccError::UnimplementedInst)
}

fn check_store_pair(
    _ctx: &FactContext,
    _flags: MemFlags,
    _addr: &PairAMode,
    _vcode: &VCode<Inst>,
    _size: u8,
) -> PccResult<()> {
    Err(PccError::UnimplementedInst)
}

fn check_load_addr(
    ctx: &FactContext,
    flags: MemFlags,
    reg: Reg,
    vcode: &VCode<Inst>,
    ty: Type,
) -> PccResult<()> {
    if !flags.checked() {
        return Ok(());
    }
    let fact = get_fact_or_default(vcode, reg, 64);
    let _output_fact = ctx.load(&fact, ty)?;
    Ok(())
}

fn check_store_addr(
    ctx: &FactContext,
    flags: MemFlags,
    reg: Reg,
    vcode: &VCode<Inst>,
    ty: Type,
) -> PccResult<()> {
    if !flags.checked() {
        return Ok(());
    }
    let fact = get_fact_or_default(vcode, reg, 64);
    let _output_fact = ctx.store(&fact, ty, None)?;
    Ok(())
}

//! AArch64 ISA: binary code emission.

use cranelift_control::ControlPlane;

use crate::ir::{self, types::*};
use crate::isa::aarch64::inst::*;
use crate::trace;

/// Memory addressing mode finalization: convert "special" modes (e.g.,
/// generic arbitrary stack offset) into real addressing modes, possibly by
/// emitting some helper instructions that come immediately before the use
/// of this amode.
pub fn mem_finalize(
    sink: Option<&mut MachBuffer<Inst>>,
    mem: &AMode,
    access_ty: Type,
    state: &EmitState,
) -> (SmallVec<[Inst; 4]>, AMode) {
    match mem {
        &AMode::RegOffset { off, .. }
        | &AMode::SPOffset { off }
        | &AMode::FPOffset { off }
        | &AMode::IncomingArg { off }
        | &AMode::SlotOffset { off } => {
            let basereg = match mem {
                &AMode::RegOffset { rn, .. } => rn,
                &AMode::SPOffset { .. }
                | &AMode::SlotOffset { .. }
                | &AMode::IncomingArg { .. } => stack_reg(),
                &AMode::FPOffset { .. } => fp_reg(),
                _ => unreachable!(),
            };
            let off = match mem {
                &AMode::IncomingArg { .. } => {
                    let frame_layout = state.frame_layout();
                    i64::from(
                        frame_layout.setup_area_size
                            + frame_layout.tail_args_size
                            + frame_layout.clobber_size
                            + frame_layout.fixed_frame_storage_size
                            + frame_layout.outgoing_args_size,
                    ) - off
                }
                &AMode::SlotOffset { .. } => {
                    let adj = i64::from(state.frame_layout().outgoing_args_size);
                    trace!(
                        "mem_finalize: slot offset {} + adj {} -> {}",
                        off,
                        adj,
                        off + adj
                    );
                    off + adj
                }
                _ => off,
            };

            if let Some(simm9) = SImm9::maybe_from_i64(off) {
                let mem = AMode::Unscaled { rn: basereg, simm9 };
                (smallvec![], mem)
            } else if let Some(uimm12) = UImm12Scaled::maybe_from_i64(off, access_ty) {
                let mem = AMode::UnsignedOffset {
                    rn: basereg,
                    uimm12,
                };
                (smallvec![], mem)
            } else {
                let tmp = writable_spilltmp_reg();
                (
                    Inst::load_constant(tmp, off as u64),
                    AMode::RegExtended {
                        rn: basereg,
                        rm: tmp.to_reg(),
                        extendop: ExtendOp::SXTX,
                    },
                )
            }
        }

        AMode::Const { addr } => {
            let sink = match sink {
                Some(sink) => sink,
                None => return (smallvec![], mem.clone()),
            };
            let label = sink.get_label_for_constant(*addr);
            let label = MemLabel::Mach(label);
            (smallvec![], AMode::Label { label })
        }

        _ => (smallvec![], mem.clone()),
    }
}

//=============================================================================
// Instructions and subcomponents: emission

pub(crate) fn machreg_to_gpr(m: Reg) -> u32 {
    assert_eq!(m.class(), RegClass::Int);
    u32::from(m.to_real_reg().unwrap().hw_enc() & 31)
}

pub(crate) fn machreg_to_vec(m: Reg) -> u32 {
    assert_eq!(m.class(), RegClass::Float);
    u32::from(m.to_real_reg().unwrap().hw_enc())
}

fn machreg_to_gpr_or_vec(m: Reg) -> u32 {
    u32::from(m.to_real_reg().unwrap().hw_enc() & 31)
}

/// Encode a 3-register aeithmeric instruction.
pub fn enc_arith_rrr(bits_31_21: u32, bits_15_10: u32, rd: Writable<Reg>, rn: Reg, rm: Reg) -> u32 {
    (bits_31_21 << 21)
        | (bits_15_10 << 10)
        | machreg_to_gpr(rd.to_reg())
        | (machreg_to_gpr(rn) << 5)
        | (machreg_to_gpr(rm) << 16)
}

fn enc_arith_rr_imm12(
    bits_31_24: u32,
    immshift: u32,
    imm12: u32,
    rn: Reg,
    rd: Writable<Reg>,
) -> u32 {
    (bits_31_24 << 24)
        | (immshift << 22)
        | (imm12 << 10)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr(rd.to_reg())
}

fn enc_arith_rr_imml(bits_31_23: u32, imm_bits: u32, rn: Reg, rd: Writable<Reg>) -> u32 {
    (bits_31_23 << 23) | (imm_bits << 10) | (machreg_to_gpr(rn) << 5) | machreg_to_gpr(rd.to_reg())
}

fn enc_arith_rrrr(top11: u32, rm: Reg, bit15: u32, ra: Reg, rn: Reg, rd: Writable<Reg>) -> u32 {
    (top11 << 21)
        | (machreg_to_gpr(rm) << 16)
        | (bit15 << 15)
        | (machreg_to_gpr(ra) << 10)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr(rd.to_reg())
}

fn enc_jump26(op_31_26: u32, off_26_0: u32) -> u32 {
    assert!(off_26_0 < (1 << 26));
    (op_31_26 << 26) | off_26_0
}

fn enc_cmpbr(op_31_24: u32, off_18_0: u32, reg: Reg) -> u32 {
    assert!(off_18_0 < (1 << 19));
    (op_31_24 << 24) | (off_18_0 << 5) | machreg_to_gpr(reg)
}

fn enc_cbr(op_31_24: u32, off_18_0: u32, op_4: u32, cond: u32) -> u32 {
    assert!(off_18_0 < (1 << 19));
    assert!(cond < (1 << 4));
    (op_31_24 << 24) | (off_18_0 << 5) | (op_4 << 4) | cond
}

/// Set the size bit of an instruction.
fn enc_op_size(op: u32, size: OperandSize) -> u32 {
    (op & !(1 << 31)) | (size.sf_bit() << 31)
}

fn enc_conditional_br(taken: BranchTarget, kind: CondBrKind) -> u32 {
    match kind {
        CondBrKind::Zero(reg, size) => enc_op_size(
            enc_cmpbr(0b0_011010_0, taken.as_offset19_or_zero(), reg),
            size,
        ),
        CondBrKind::NotZero(reg, size) => enc_op_size(
            enc_cmpbr(0b0_011010_1, taken.as_offset19_or_zero(), reg),
            size,
        ),
        CondBrKind::Cond(c) => enc_cbr(0b01010100, taken.as_offset19_or_zero(), 0b0, c.bits()),
    }
}

fn enc_test_bit_and_branch(
    kind: TestBitAndBranchKind,
    taken: BranchTarget,
    reg: Reg,
    bit: u8,
) -> u32 {
    assert!(bit < 64);
    let op_31 = u32::from(bit >> 5);
    let op_23_19 = u32::from(bit & 0b11111);
    let op_30_24 = 0b0110110
        | match kind {
            TestBitAndBranchKind::Z => 0,
            TestBitAndBranchKind::NZ => 1,
        };
    (op_31 << 31)
        | (op_30_24 << 24)
        | (op_23_19 << 19)
        | (taken.as_offset14_or_zero() << 5)
        | machreg_to_gpr(reg)
}

/// Encode a move-wide instruction.
pub fn enc_move_wide(
    op: MoveWideOp,
    rd: Writable<Reg>,
    imm: MoveWideConst,
    size: OperandSize,
) -> u32 {
    assert!(imm.shift <= 0b11);
    let op = match op {
        MoveWideOp::MovN => 0b00,
        MoveWideOp::MovZ => 0b10,
    };
    0x12800000
        | size.sf_bit() << 31
        | op << 29
        | u32::from(imm.shift) << 21
        | u32::from(imm.bits) << 5
        | machreg_to_gpr(rd.to_reg())
}

/// Encode a move-keep immediate instruction.
pub fn enc_movk(rd: Writable<Reg>, imm: MoveWideConst, size: OperandSize) -> u32 {
    assert!(imm.shift <= 0b11);
    0x72800000
        | size.sf_bit() << 31
        | u32::from(imm.shift) << 21
        | u32::from(imm.bits) << 5
        | machreg_to_gpr(rd.to_reg())
}

fn enc_ldst_pair(op_31_22: u32, simm7: SImm7Scaled, rn: Reg, rt: Reg, rt2: Reg) -> u32 {
    (op_31_22 << 22)
        | (simm7.bits() << 15)
        | (machreg_to_gpr(rt2) << 10)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr(rt)
}

fn enc_ldst_simm9(op_31_22: u32, simm9: SImm9, op_11_10: u32, rn: Reg, rd: Reg) -> u32 {
    (op_31_22 << 22)
        | (simm9.bits() << 12)
        | (op_11_10 << 10)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr_or_vec(rd)
}

fn enc_ldst_uimm12(op_31_22: u32, uimm12: UImm12Scaled, rn: Reg, rd: Reg) -> u32 {
    (op_31_22 << 22)
        | (0b1 << 24)
        | (uimm12.bits() << 10)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr_or_vec(rd)
}

fn enc_ldst_reg(
    op_31_22: u32,
    rn: Reg,
    rm: Reg,
    s_bit: bool,
    extendop: Option<ExtendOp>,
    rd: Reg,
) -> u32 {
    let s_bit = if s_bit { 1 } else { 0 };
    let extend_bits = match extendop {
        Some(ExtendOp::UXTW) => 0b010,
        Some(ExtendOp::SXTW) => 0b110,
        Some(ExtendOp::SXTX) => 0b111,
        None => 0b011, // LSL
        _ => panic!("bad extend mode for ld/st AMode"),
    };
    (op_31_22 << 22)
        | (1 << 21)
        | (machreg_to_gpr(rm) << 16)
        | (extend_bits << 13)
        | (s_bit << 12)
        | (0b10 << 10)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr_or_vec(rd)
}

pub(crate) fn enc_ldst_imm19(op_31_24: u32, imm19: u32, rd: Reg) -> u32 {
    (op_31_24 << 24) | (imm19 << 5) | machreg_to_gpr_or_vec(rd)
}

fn enc_ldst_vec(q: u32, size: u32, rn: Reg, rt: Writable<Reg>) -> u32 {
    debug_assert_eq!(q & 0b1, q);
    debug_assert_eq!(size & 0b11, size);
    0b0_0_0011010_10_00000_110_0_00_00000_00000
        | q << 30
        | size << 10
        | machreg_to_gpr(rn) << 5
        | machreg_to_vec(rt.to_reg())
}

fn enc_ldst_vec_pair(
    opc: u32,
    amode: u32,
    is_load: bool,
    simm7: SImm7Scaled,
    rn: Reg,
    rt: Reg,
    rt2: Reg,
) -> u32 {
    debug_assert_eq!(opc & 0b11, opc);
    debug_assert_eq!(amode & 0b11, amode);

    0b00_10110_00_0_0000000_00000_00000_00000
        | opc << 30
        | amode << 23
        | (is_load as u32) << 22
        | simm7.bits() << 15
        | machreg_to_vec(rt2) << 10
        | machreg_to_gpr(rn) << 5
        | machreg_to_vec(rt)
}

fn enc_vec_rrr(top11: u32, rm: Reg, bit15_10: u32, rn: Reg, rd: Writable<Reg>) -> u32 {
    (top11 << 21)
        | (machreg_to_vec(rm) << 16)
        | (bit15_10 << 10)
        | (machreg_to_vec(rn) << 5)
        | machreg_to_vec(rd.to_reg())
}

fn enc_vec_rrr_long(
    q: u32,
    u: u32,
    size: u32,
    bit14: u32,
    rm: Reg,
    rn: Reg,
    rd: Writable<Reg>,
) -> u32 {
    debug_assert_eq!(q & 0b1, q);
    debug_assert_eq!(u & 0b1, u);
    debug_assert_eq!(size & 0b11, size);
    debug_assert_eq!(bit14 & 0b1, bit14);

    0b0_0_0_01110_00_1_00000_100000_00000_00000
        | q << 30
        | u << 29
        | size << 22
        | bit14 << 14
        | (machreg_to_vec(rm) << 16)
        | (machreg_to_vec(rn) << 5)
        | machreg_to_vec(rd.to_reg())
}

fn enc_bit_rr(size: u32, opcode2: u32, opcode1: u32, rn: Reg, rd: Writable<Reg>) -> u32 {
    (0b01011010110 << 21)
        | size << 31
        | opcode2 << 16
        | opcode1 << 10
        | machreg_to_gpr(rn) << 5
        | machreg_to_gpr(rd.to_reg())
}

pub(crate) fn enc_br(rn: Reg) -> u32 {
    0b1101011_0000_11111_000000_00000_00000 | (machreg_to_gpr(rn) << 5)
}

pub(crate) fn enc_adr_inst(opcode: u32, off: i32, rd: Writable<Reg>) -> u32 {
    let off = u32::try_from(off).unwrap();
    let immlo = off & 3;
    let immhi = (off >> 2) & ((1 << 19) - 1);
    opcode | (immlo << 29) | (immhi << 5) | machreg_to_gpr(rd.to_reg())
}

pub(crate) fn enc_adr(off: i32, rd: Writable<Reg>) -> u32 {
    let opcode = 0b00010000 << 24;
    enc_adr_inst(opcode, off, rd)
}

pub(crate) fn enc_adrp(off: i32, rd: Writable<Reg>) -> u32 {
    let opcode = 0b10010000 << 24;
    enc_adr_inst(opcode, off, rd)
}

fn enc_csel(rd: Writable<Reg>, rn: Reg, rm: Reg, cond: Cond, op: u32, o2: u32) -> u32 {
    debug_assert_eq!(op & 0b1, op);
    debug_assert_eq!(o2 & 0b1, o2);
    0b100_11010100_00000_0000_00_00000_00000
        | (op << 30)
        | (machreg_to_gpr(rm) << 16)
        | (cond.bits() << 12)
        | (o2 << 10)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr(rd.to_reg())
}

fn enc_fcsel(rd: Writable<Reg>, rn: Reg, rm: Reg, cond: Cond, size: ScalarSize) -> u32 {
    0b000_11110_00_1_00000_0000_11_00000_00000
        | (size.ftype() << 22)
        | (machreg_to_vec(rm) << 16)
        | (machreg_to_vec(rn) << 5)
        | machreg_to_vec(rd.to_reg())
        | (cond.bits() << 12)
}

fn enc_ccmp(size: OperandSize, rn: Reg, rm: Reg, nzcv: NZCV, cond: Cond) -> u32 {
    0b0_1_1_11010010_00000_0000_00_00000_0_0000
        | size.sf_bit() << 31
        | machreg_to_gpr(rm) << 16
        | cond.bits() << 12
        | machreg_to_gpr(rn) << 5
        | nzcv.bits()
}

fn enc_ccmp_imm(size: OperandSize, rn: Reg, imm: UImm5, nzcv: NZCV, cond: Cond) -> u32 {
    0b0_1_1_11010010_00000_0000_10_00000_0_0000
        | size.sf_bit() << 31
        | imm.bits() << 16
        | cond.bits() << 12
        | machreg_to_gpr(rn) << 5
        | nzcv.bits()
}

fn enc_bfm(opc: u8, size: OperandSize, rd: Writable<Reg>, rn: Reg, immr: u8, imms: u8) -> u32 {
    match size {
        OperandSize::Size64 => {
            debug_assert!(immr <= 63);
            debug_assert!(imms <= 63);
        }
        OperandSize::Size32 => {
            debug_assert!(immr <= 31);
            debug_assert!(imms <= 31);
        }
    }
    debug_assert_eq!(opc & 0b11, opc);
    let n_bit = size.sf_bit();
    0b0_00_100110_0_000000_000000_00000_00000
        | size.sf_bit() << 31
        | u32::from(opc) << 29
        | n_bit << 22
        | u32::from(immr) << 16
        | u32::from(imms) << 10
        | machreg_to_gpr(rn) << 5
        | machreg_to_gpr(rd.to_reg())
}

fn enc_vecmov(is_16b: bool, rd: Writable<Reg>, rn: Reg) -> u32 {
    0b00001110_101_00000_00011_1_00000_00000
        | ((is_16b as u32) << 30)
        | machreg_to_vec(rd.to_reg())
        | (machreg_to_vec(rn) << 16)
        | (machreg_to_vec(rn) << 5)
}

fn enc_fpurr(top22: u32, rd: Writable<Reg>, rn: Reg) -> u32 {
    (top22 << 10) | (machreg_to_vec(rn) << 5) | machreg_to_vec(rd.to_reg())
}

fn enc_fpurrr(top22: u32, rd: Writable<Reg>, rn: Reg, rm: Reg) -> u32 {
    (top22 << 10)
        | (machreg_to_vec(rm) << 16)
        | (machreg_to_vec(rn) << 5)
        | machreg_to_vec(rd.to_reg())
}

fn enc_fpurrrr(top17: u32, rd: Writable<Reg>, rn: Reg, rm: Reg, ra: Reg) -> u32 {
    (top17 << 15)
        | (machreg_to_vec(rm) << 16)
        | (machreg_to_vec(ra) << 10)
        | (machreg_to_vec(rn) << 5)
        | machreg_to_vec(rd.to_reg())
}

fn enc_fcmp(size: ScalarSize, rn: Reg, rm: Reg) -> u32 {
    0b000_11110_00_1_00000_00_1000_00000_00000
        | (size.ftype() << 22)
        | (machreg_to_vec(rm) << 16)
        | (machreg_to_vec(rn) << 5)
}

fn enc_fputoint(top16: u32, rd: Writable<Reg>, rn: Reg) -> u32 {
    (top16 << 16) | (machreg_to_vec(rn) << 5) | machreg_to_gpr(rd.to_reg())
}

fn enc_inttofpu(top16: u32, rd: Writable<Reg>, rn: Reg) -> u32 {
    (top16 << 16) | (machreg_to_gpr(rn) << 5) | machreg_to_vec(rd.to_reg())
}

fn enc_fround(top22: u32, rd: Writable<Reg>, rn: Reg) -> u32 {
    (top22 << 10) | (machreg_to_vec(rn) << 5) | machreg_to_vec(rd.to_reg())
}

fn enc_vec_rr_misc(qu: u32, size: u32, bits_12_16: u32, rd: Writable<Reg>, rn: Reg) -> u32 {
    debug_assert_eq!(qu & 0b11, qu);
    debug_assert_eq!(size & 0b11, size);
    debug_assert_eq!(bits_12_16 & 0b11111, bits_12_16);
    let bits = 0b0_00_01110_00_10000_00000_10_00000_00000;
    bits | qu << 29
        | size << 22
        | bits_12_16 << 12
        | machreg_to_vec(rn) << 5
        | machreg_to_vec(rd.to_reg())
}

fn enc_vec_rr_pair(bits_12_16: u32, rd: Writable<Reg>, rn: Reg) -> u32 {
    debug_assert_eq!(bits_12_16 & 0b11111, bits_12_16);

    0b010_11110_11_11000_11011_10_00000_00000
        | bits_12_16 << 12
        | machreg_to_vec(rn) << 5
        | machreg_to_vec(rd.to_reg())
}

fn enc_vec_rr_pair_long(u: u32, enc_size: u32, rd: Writable<Reg>, rn: Reg) -> u32 {
    debug_assert_eq!(u & 0b1, u);
    debug_assert_eq!(enc_size & 0b1, enc_size);

    0b0_1_0_01110_00_10000_00_0_10_10_00000_00000
        | u << 29
        | enc_size << 22
        | machreg_to_vec(rn) << 5
        | machreg_to_vec(rd.to_reg())
}

fn enc_vec_lanes(q: u32, u: u32, size: u32, opcode: u32, rd: Writable<Reg>, rn: Reg) -> u32 {
    debug_assert_eq!(q & 0b1, q);
    debug_assert_eq!(u & 0b1, u);
    debug_assert_eq!(size & 0b11, size);
    debug_assert_eq!(opcode & 0b11111, opcode);
    0b0_0_0_01110_00_11000_0_0000_10_00000_00000
        | q << 30
        | u << 29
        | size << 22
        | opcode << 12
        | machreg_to_vec(rn) << 5
        | machreg_to_vec(rd.to_reg())
}

fn enc_tbl(is_extension: bool, len: u32, rd: Writable<Reg>, rn: Reg, rm: Reg) -> u32 {
    debug_assert_eq!(len & 0b11, len);
    0b0_1_001110_000_00000_0_00_0_00_00000_00000
        | (machreg_to_vec(rm) << 16)
        | len << 13
        | (is_extension as u32) << 12
        | (machreg_to_vec(rn) << 5)
        | machreg_to_vec(rd.to_reg())
}

fn enc_dmb_ish() -> u32 {
    0xD5033BBF
}

fn enc_acq_rel(ty: Type, op: AtomicRMWOp, rs: Reg, rt: Writable<Reg>, rn: Reg) -> u32 {
    assert!(machreg_to_gpr(rt.to_reg()) != 31);
    let sz = match ty {
        I64 => 0b11,
        I32 => 0b10,
        I16 => 0b01,
        I8 => 0b00,
        _ => unreachable!(),
    };
    let bit15 = match op {
        AtomicRMWOp::Swp => 0b1,
        _ => 0b0,
    };
    let op = match op {
        AtomicRMWOp::Add => 0b000,
        AtomicRMWOp::Clr => 0b001,
        AtomicRMWOp::Eor => 0b010,
        AtomicRMWOp::Set => 0b011,
        AtomicRMWOp::Smax => 0b100,
        AtomicRMWOp::Smin => 0b101,
        AtomicRMWOp::Umax => 0b110,
        AtomicRMWOp::Umin => 0b111,
        AtomicRMWOp::Swp => 0b000,
    };
    0b00_111_000_111_00000_0_000_00_00000_00000
        | (sz << 30)
        | (machreg_to_gpr(rs) << 16)
        | bit15 << 15
        | (op << 12)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr(rt.to_reg())
}

fn enc_ldar(ty: Type, rt: Writable<Reg>, rn: Reg) -> u32 {
    let sz = match ty {
        I64 => 0b11,
        I32 => 0b10,
        I16 => 0b01,
        I8 => 0b00,
        _ => unreachable!(),
    };
    0b00_001000_1_1_0_11111_1_11111_00000_00000
        | (sz << 30)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr(rt.to_reg())
}

fn enc_stlr(ty: Type, rt: Reg, rn: Reg) -> u32 {
    let sz = match ty {
        I64 => 0b11,
        I32 => 0b10,
        I16 => 0b01,
        I8 => 0b00,
        _ => unreachable!(),
    };
    0b00_001000_100_11111_1_11111_00000_00000
        | (sz << 30)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr(rt)
}

fn enc_ldaxr(ty: Type, rt: Writable<Reg>, rn: Reg) -> u32 {
    let sz = match ty {
        I64 => 0b11,
        I32 => 0b10,
        I16 => 0b01,
        I8 => 0b00,
        _ => unreachable!(),
    };
    0b00_001000_0_1_0_11111_1_11111_00000_00000
        | (sz << 30)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr(rt.to_reg())
}

fn enc_stlxr(ty: Type, rs: Writable<Reg>, rt: Reg, rn: Reg) -> u32 {
    let sz = match ty {
        I64 => 0b11,
        I32 => 0b10,
        I16 => 0b01,
        I8 => 0b00,
        _ => unreachable!(),
    };
    0b00_001000_000_00000_1_11111_00000_00000
        | (sz << 30)
        | (machreg_to_gpr(rs.to_reg()) << 16)
        | (machreg_to_gpr(rn) << 5)
        | machreg_to_gpr(rt)
}

fn enc_cas(size: u32, rs: Writable<Reg>, rt: Reg, rn: Reg) -> u32 {
    debug_assert_eq!(size & 0b11, size);

    0b00_0010001_1_1_00000_1_11111_00000_00000
        | size << 30
        | machreg_to_gpr(rs.to_reg()) << 16
        | machreg_to_gpr(rn) << 5
        | machreg_to_gpr(rt)
}

fn enc_asimd_mod_imm(rd: Writable<Reg>, q_op: u32, cmode: u32, imm: u8) -> u32 {
    let abc = (imm >> 5) as u32;
    let defgh = (imm & 0b11111) as u32;

    debug_assert_eq!(cmode & 0b1111, cmode);
    debug_assert_eq!(q_op & 0b11, q_op);

    0b0_0_0_0111100000_000_0000_01_00000_00000
        | (q_op << 29)
        | (abc << 16)
        | (cmode << 12)
        | (defgh << 5)
        | machreg_to_vec(rd.to_reg())
}

/// State carried between emissions of a sequence of instructions.
#[derive(Default, Clone, Debug)]
pub struct EmitState {
    /// The user stack map for the upcoming instruction, as provided to
    /// `pre_safepoint()`.
    user_stack_map: Option<ir::UserStackMap>,

    /// Only used during fuzz-testing. Otherwise, it is a zero-sized struct and
    /// optimized away at compiletime. See [cranelift_control].
    ctrl_plane: ControlPlane,

    frame_layout: FrameLayout,
}

impl MachInstEmitState<Inst> for EmitState {
    fn new(abi: &Callee<AArch64MachineDeps>, ctrl_plane: ControlPlane) -> Self {
        EmitState {
            user_stack_map: None,
            ctrl_plane,
            frame_layout: abi.frame_layout().clone(),
        }
    }

    fn pre_safepoint(&mut self, user_stack_map: Option<ir::UserStackMap>) {
        self.user_stack_map = user_stack_map;
    }

    fn ctrl_plane_mut(&mut self) -> &mut ControlPlane {
        &mut self.ctrl_plane
    }

    fn take_ctrl_plane(self) -> ControlPlane {
        self.ctrl_plane
    }

    fn frame_layout(&self) -> &FrameLayout {
        &self.frame_layout
    }
}

impl EmitState {
    fn take_stack_map(&mut self) -> Option<ir::UserStackMap> {
        self.user_stack_map.take()
    }

    fn clear_post_insn(&mut self) {
        self.user_stack_map = None;
    }
}

/// Constant state used during function compilation.
pub struct EmitInfo(settings::Flags);

impl EmitInfo {
    /// Create a constant state for emission of instructions.
    pub fn new(flags: settings::Flags) -> Self {
        Self(flags)
    }
}

impl MachInstEmit for Inst {
    type State = EmitState;
    type Info = EmitInfo;

    fn emit(&self, sink: &mut MachBuffer<Inst>, emit_info: &Self::Info, state: &mut EmitState) {
        // N.B.: we *must* not exceed the "worst-case size" used to compute
        // where to insert islands, except when islands are explicitly triggered
        // (with an `EmitIsland`). We check this in debug builds. This is `mut`
        // to allow disabling the check for `JTSequence`, which is always
        // emitted following an `EmitIsland`.
        let mut start_off = sink.cur_offset();

        match self {
            &Inst::AluRRR {
                alu_op,
                size,
                rd,
                rn,
                rm,
            } => {
                debug_assert!(match alu_op {
                    ALUOp::SMulH | ALUOp::UMulH => size == OperandSize::Size64,
                    _ => true,
                });
                let top11 = match alu_op {
                    ALUOp::Add => 0b00001011_000,
                    ALUOp::Adc => 0b00011010_000,
                    ALUOp::AdcS => 0b00111010_000,
                    ALUOp::Sub => 0b01001011_000,
                    ALUOp::Sbc => 0b01011010_000,
                    ALUOp::SbcS => 0b01111010_000,
                    ALUOp::Orr => 0b00101010_000,
                    ALUOp::And => 0b00001010_000,
                    ALUOp::AndS => 0b01101010_000,
                    ALUOp::Eor => 0b01001010_000,
                    ALUOp::OrrNot => 0b00101010_001,
                    ALUOp::AndNot => 0b00001010_001,
                    ALUOp::EorNot => 0b01001010_001,
                    ALUOp::AddS => 0b00101011_000,
                    ALUOp::SubS => 0b01101011_000,
                    ALUOp::SDiv | ALUOp::UDiv => 0b00011010_110,
                    ALUOp::Extr | ALUOp::Lsr | ALUOp::Asr | ALUOp::Lsl => 0b00011010_110,
                    ALUOp::SMulH => 0b10011011_010,
                    ALUOp::UMulH => 0b10011011_110,
                };

                let top11 = top11 | size.sf_bit() << 10;
                let bit15_10 = match alu_op {
                    ALUOp::SDiv => 0b000011,
                    ALUOp::UDiv => 0b000010,
                    ALUOp::Extr => 0b001011,
                    ALUOp::Lsr => 0b001001,
                    ALUOp::Asr => 0b001010,
                    ALUOp::Lsl => 0b001000,
                    ALUOp::SMulH | ALUOp::UMulH => 0b011111,
                    _ => 0b000000,
                };
                debug_assert_ne!(writable_stack_reg(), rd);
                // The stack pointer is the zero register in this context, so this might be an
                // indication that something is wrong.
                debug_assert_ne!(stack_reg(), rn);
                debug_assert_ne!(stack_reg(), rm);
                sink.put4(enc_arith_rrr(top11, bit15_10, rd, rn, rm));
            }
            &Inst::AluRRRR {
                alu_op,
                size,
                rd,
                rm,
                rn,
                ra,
            } => {
                let (top11, bit15) = match alu_op {
                    ALUOp3::MAdd => (0b0_00_11011_000, 0),
                    ALUOp3::MSub => (0b0_00_11011_000, 1),
                    ALUOp3::UMAddL => {
                        debug_assert!(size == OperandSize::Size32);
                        (0b1_00_11011_1_01, 0)
                    }
                    ALUOp3::SMAddL => {
                        debug_assert!(size == OperandSize::Size32);
                        (0b1_00_11011_0_01, 0)
                    }
                };
                let top11 = top11 | size.sf_bit() << 10;
                sink.put4(enc_arith_rrrr(top11, rm, bit15, ra, rn, rd));
            }
            &Inst::AluRRImm12 {
                alu_op,
                size,
                rd,
                rn,
                ref imm12,
            } => {
                let top8 = match alu_op {
                    ALUOp::Add => 0b000_10001,
                    ALUOp::Sub => 0b010_10001,
                    ALUOp::AddS => 0b001_10001,
                    ALUOp::SubS => 0b011_10001,
                    _ => unimplemented!("{:?}", alu_op),
                };
                let top8 = top8 | size.sf_bit() << 7;
                sink.put4(enc_arith_rr_imm12(
                    top8,
                    imm12.shift_bits(),
                    imm12.imm_bits(),
                    rn,
                    rd,
                ));
            }
            &Inst::AluRRImmLogic {
                alu_op,
                size,
                rd,
                rn,
                ref imml,
            } => {
                let (top9, inv) = match alu_op {
                    ALUOp::Orr => (0b001_100100, false),
                    ALUOp::And => (0b000_100100, false),
                    ALUOp::AndS => (0b011_100100, false),
                    ALUOp::Eor => (0b010_100100, false),
                    ALUOp::OrrNot => (0b001_100100, true),
                    ALUOp::AndNot => (0b000_100100, true),
                    ALUOp::EorNot => (0b010_100100, true),
                    _ => unimplemented!("{:?}", alu_op),
                };
                let top9 = top9 | size.sf_bit() << 8;
                let imml = if inv { imml.invert() } else { *imml };
                sink.put4(enc_arith_rr_imml(top9, imml.enc_bits(), rn, rd));
            }

            &Inst::AluRRImmShift {
                alu_op,
                size,
                rd,
                rn,
                ref immshift,
            } => {
                let amt = immshift.value();
                let (top10, immr, imms) = match alu_op {
                    ALUOp::Extr => (0b0001001110, machreg_to_gpr(rn), u32::from(amt)),
                    ALUOp::Lsr => (0b0101001100, u32::from(amt), 0b011111),
                    ALUOp::Asr => (0b0001001100, u32::from(amt), 0b011111),
                    ALUOp::Lsl => {
                        let bits = if size.is64() { 64 } else { 32 };
                        (
                            0b0101001100,
                            u32::from((bits - amt) % bits),
                            u32::from(bits - 1 - amt),
                        )
                    }
                    _ => unimplemented!("{:?}", alu_op),
                };
                let top10 = top10 | size.sf_bit() << 9 | size.sf_bit();
                let imms = match alu_op {
                    ALUOp::Lsr | ALUOp::Asr => imms | size.sf_bit() << 5,
                    _ => imms,
                };
                sink.put4(
                    (top10 << 22)
                        | (immr << 16)
                        | (imms << 10)
                        | (machreg_to_gpr(rn) << 5)
                        | machreg_to_gpr(rd.to_reg()),
                );
            }

            &Inst::AluRRRShift {
                alu_op,
                size,
                rd,
                rn,
                rm,
                ref shiftop,
            } => {
                let top11: u32 = match alu_op {
                    ALUOp::Add => 0b000_01011000,
                    ALUOp::AddS => 0b001_01011000,
                    ALUOp::Sub => 0b010_01011000,
                    ALUOp::SubS => 0b011_01011000,
                    ALUOp::Orr => 0b001_01010000,
                    ALUOp::And => 0b000_01010000,
                    ALUOp::AndS => 0b011_01010000,
                    ALUOp::Eor => 0b010_01010000,
                    ALUOp::OrrNot => 0b001_01010001,
                    ALUOp::EorNot => 0b010_01010001,
                    ALUOp::AndNot => 0b000_01010001,
                    ALUOp::Extr => 0b000_10011100,
                    _ => unimplemented!("{:?}", alu_op),
                };
                let top11 = top11 | size.sf_bit() << 10;
                let top11 = top11 | (u32::from(shiftop.op().bits()) << 1);
                let bits_15_10 = u32::from(shiftop.amt().value());
                sink.put4(enc_arith_rrr(top11, bits_15_10, rd, rn, rm));
            }

            &Inst::AluRRRExtend {
                alu_op,
                size,
                rd,
                rn,
                rm,
                extendop,
            } => {
                let top11: u32 = match alu_op {
                    ALUOp::Add => 0b00001011001,
                    ALUOp::Sub => 0b01001011001,
                    ALUOp::AddS => 0b00101011001,
                    ALUOp::SubS => 0b01101011001,
                    _ => unimplemented!("{:?}", alu_op),
                };
                let top11 = top11 | size.sf_bit() << 10;
                let bits_15_10 = u32::from(extendop.bits()) << 3;
                sink.put4(enc_arith_rrr(top11, bits_15_10, rd, rn, rm));
            }

            &Inst::BitRR {
                op, size, rd, rn, ..
            } => {
                let (op1, op2) = match op {
                    BitOp::RBit => (0b00000, 0b000000),
                    BitOp::Clz => (0b00000, 0b000100),
                    BitOp::Cls => (0b00000, 0b000101),
                    BitOp::Rev16 => (0b00000, 0b000001),
                    BitOp::Rev32 => (0b00000, 0b000010),
                    BitOp::Rev64 => (0b00000, 0b000011),
                };
                sink.put4(enc_bit_rr(size.sf_bit(), op1, op2, rn, rd))
            }

            &Inst::ULoad8 { rd, ref mem, flags }
            | &Inst::SLoad8 { rd, ref mem, flags }
            | &Inst::ULoad16 { rd, ref mem, flags }
            | &Inst::SLoad16 { rd, ref mem, flags }
            | &Inst::ULoad32 { rd, ref mem, flags }
            | &Inst::SLoad32 { rd, ref mem, flags }
            | &Inst::ULoad64 {
                rd, ref mem, flags, ..
            }
            | &Inst::FpuLoad16 { rd, ref mem, flags }
            | &Inst::FpuLoad32 { rd, ref mem, flags }
            | &Inst::FpuLoad64 { rd, ref mem, flags }
            | &Inst::FpuLoad128 { rd, ref mem, flags } => {
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_insts, mem) = mem_finalize(Some(sink), &mem, access_ty, state);

                for inst in mem_insts.into_iter() {
                    inst.emit(sink, emit_info, state);
                }

                // ldst encoding helpers take Reg, not Writable<Reg>.
                let rd = rd.to_reg();

                // This is the base opcode (top 10 bits) for the "unscaled
                // immediate" form (Unscaled). Other addressing modes will OR in
                // other values for bits 24/25 (bits 1/2 of this constant).
                let op = match self {
                    Inst::ULoad8 { .. } => 0b0011100001,
                    Inst::SLoad8 { .. } => 0b0011100010,
                    Inst::ULoad16 { .. } => 0b0111100001,
                    Inst::SLoad16 { .. } => 0b0111100010,
                    Inst::ULoad32 { .. } => 0b1011100001,
                    Inst::SLoad32 { .. } => 0b1011100010,
                    Inst::ULoad64 { .. } => 0b1111100001,
                    Inst::FpuLoad16 { .. } => 0b0111110001,
                    Inst::FpuLoad32 { .. } => 0b1011110001,
                    Inst::FpuLoad64 { .. } => 0b1111110001,
                    Inst::FpuLoad128 { .. } => 0b0011110011,
                    _ => unreachable!(),
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }

                match &mem {
                    &AMode::Unscaled { rn, simm9 } => {
                        let reg = rn;
                        sink.put4(enc_ldst_simm9(op, simm9, 0b00, reg, rd));
                    }
                    &AMode::UnsignedOffset { rn, uimm12 } => {
                        let reg = rn;
                        sink.put4(enc_ldst_uimm12(op, uimm12, reg, rd));
                    }
                    &AMode::RegReg { rn, rm } => {
                        let r1 = rn;
                        let r2 = rm;
                        sink.put4(enc_ldst_reg(
                            op, r1, r2, /* scaled = */ false, /* extendop = */ None, rd,
                        ));
                    }
                    &AMode::RegScaled { rn, rm } | &AMode::RegScaledExtended { rn, rm, .. } => {
                        let r1 = rn;
                        let r2 = rm;
                        let extendop = match &mem {
                            &AMode::RegScaled { .. } => None,
                            &AMode::RegScaledExtended { extendop, .. } => Some(extendop),
                            _ => unreachable!(),
                        };
                        sink.put4(enc_ldst_reg(
                            op, r1, r2, /* scaled = */ true, extendop, rd,
                        ));
                    }
                    &AMode::RegExtended { rn, rm, extendop } => {
                        let r1 = rn;
                        let r2 = rm;
                        sink.put4(enc_ldst_reg(
                            op,
                            r1,
                            r2,
                            /* scaled = */ false,
                            Some(extendop),
                            rd,
                        ));
                    }
                    &AMode::Label { ref label } => {
                        let offset = match label {
                            // cast i32 to u32 (two's-complement)
                            MemLabel::PCRel(off) => *off as u32,
                            // Emit a relocation into the `MachBuffer`
                            // for the label that's being loaded from and
                            // encode an address of 0 in its place which will
                            // get filled in by relocation resolution later on.
                            MemLabel::Mach(label) => {
                                sink.use_label_at_offset(
                                    sink.cur_offset(),
                                    *label,
                                    LabelUse::Ldr19,
                                );
                                0
                            }
                        } / 4;
                        assert!(offset < (1 << 19));
                        match self {
                            &Inst::ULoad32 { .. } => {
                                sink.put4(enc_ldst_imm19(0b00011000, offset, rd));
                            }
                            &Inst::SLoad32 { .. } => {
                                sink.put4(enc_ldst_imm19(0b10011000, offset, rd));
                            }
                            &Inst::FpuLoad32 { .. } => {
                                sink.put4(enc_ldst_imm19(0b00011100, offset, rd));
                            }
                            &Inst::ULoad64 { .. } => {
                                sink.put4(enc_ldst_imm19(0b01011000, offset, rd));
                            }
                            &Inst::FpuLoad64 { .. } => {
                                sink.put4(enc_ldst_imm19(0b01011100, offset, rd));
                            }
                            &Inst::FpuLoad128 { .. } => {
                                sink.put4(enc_ldst_imm19(0b10011100, offset, rd));
                            }
                            _ => panic!("Unsupported size for LDR from constant pool!"),
                        }
                    }
                    &AMode::SPPreIndexed { simm9 } => {
                        let reg = stack_reg();
                        sink.put4(enc_ldst_simm9(op, simm9, 0b11, reg, rd));
                    }
                    &AMode::SPPostIndexed { simm9 } => {
                        let reg = stack_reg();
                        sink.put4(enc_ldst_simm9(op, simm9, 0b01, reg, rd));
                    }
                    // Eliminated by `mem_finalize()` above.
                    &AMode::SPOffset { .. }
                    | &AMode::FPOffset { .. }
                    | &AMode::IncomingArg { .. }
                    | &AMode::SlotOffset { .. }
                    | &AMode::Const { .. }
                    | &AMode::RegOffset { .. } => {
                        panic!("Should not see {mem:?} here!")
                    }
                }
            }

            &Inst::Store8 { rd, ref mem, flags }
            | &Inst::Store16 { rd, ref mem, flags }
            | &Inst::Store32 { rd, ref mem, flags }
            | &Inst::Store64 { rd, ref mem, flags }
            | &Inst::FpuStore16 { rd, ref mem, flags }
            | &Inst::FpuStore32 { rd, ref mem, flags }
            | &Inst::FpuStore64 { rd, ref mem, flags }
            | &Inst::FpuStore128 { rd, ref mem, flags } => {
                let mem = mem.clone();
                let access_ty = self.mem_type().unwrap();
                let (mem_insts, mem) = mem_finalize(Some(sink), &mem, access_ty, state);

                for inst in mem_insts.into_iter() {
                    inst.emit(sink, emit_info, state);
                }

                let op = match self {
                    Inst::Store8 { .. } => 0b0011100000,
                    Inst::Store16 { .. } => 0b0111100000,
                    Inst::Store32 { .. } => 0b1011100000,
                    Inst::Store64 { .. } => 0b1111100000,
                    Inst::FpuStore16 { .. } => 0b0111110000,
                    Inst::FpuStore32 { .. } => 0b1011110000,
                    Inst::FpuStore64 { .. } => 0b1111110000,
                    Inst::FpuStore128 { .. } => 0b0011110010,
                    _ => unreachable!(),
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual store instruction starts.
                    sink.add_trap(trap_code);
                }

                match &mem {
                    &AMode::Unscaled { rn, simm9 } => {
                        let reg = rn;
                        sink.put4(enc_ldst_simm9(op, simm9, 0b00, reg, rd));
                    }
                    &AMode::UnsignedOffset { rn, uimm12 } => {
                        let reg = rn;
                        sink.put4(enc_ldst_uimm12(op, uimm12, reg, rd));
                    }
                    &AMode::RegReg { rn, rm } => {
                        let r1 = rn;
                        let r2 = rm;
                        sink.put4(enc_ldst_reg(
                            op, r1, r2, /* scaled = */ false, /* extendop = */ None, rd,
                        ));
                    }
                    &AMode::RegScaled { rn, rm } | &AMode::RegScaledExtended { rn, rm, .. } => {
                        let r1 = rn;
                        let r2 = rm;
                        let extendop = match &mem {
                            &AMode::RegScaled { .. } => None,
                            &AMode::RegScaledExtended { extendop, .. } => Some(extendop),
                            _ => unreachable!(),
                        };
                        sink.put4(enc_ldst_reg(
                            op, r1, r2, /* scaled = */ true, extendop, rd,
                        ));
                    }
                    &AMode::RegExtended { rn, rm, extendop } => {
                        let r1 = rn;
                        let r2 = rm;
                        sink.put4(enc_ldst_reg(
                            op,
                            r1,
                            r2,
                            /* scaled = */ false,
                            Some(extendop),
                            rd,
                        ));
                    }
                    &AMode::Label { .. } => {
                        panic!("Store to a MemLabel not implemented!");
                    }
                    &AMode::SPPreIndexed { simm9 } => {
                        let reg = stack_reg();
                        sink.put4(enc_ldst_simm9(op, simm9, 0b11, reg, rd));
                    }
                    &AMode::SPPostIndexed { simm9 } => {
                        let reg = stack_reg();
                        sink.put4(enc_ldst_simm9(op, simm9, 0b01, reg, rd));
                    }
                    // Eliminated by `mem_finalize()` above.
                    &AMode::SPOffset { .. }
                    | &AMode::FPOffset { .. }
                    | &AMode::IncomingArg { .. }
                    | &AMode::SlotOffset { .. }
                    | &AMode::Const { .. }
                    | &AMode::RegOffset { .. } => {
                        panic!("Should not see {mem:?} here!")
                    }
                }
            }

            &Inst::StoreP64 {
                rt,
                rt2,
                ref mem,
                flags,
            } => {
                let mem = mem.clone();
                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual store instruction starts.
                    sink.add_trap(trap_code);
                }
                match &mem {
                    &PairAMode::SignedOffset { reg, simm7 } => {
                        assert_eq!(simm7.scale_ty, I64);
                        sink.put4(enc_ldst_pair(0b1010100100, simm7, reg, rt, rt2));
                    }
                    &PairAMode::SPPreIndexed { simm7 } => {
                        assert_eq!(simm7.scale_ty, I64);
                        let reg = stack_reg();
                        sink.put4(enc_ldst_pair(0b1010100110, simm7, reg, rt, rt2));
                    }
                    &PairAMode::SPPostIndexed { simm7 } => {
                        assert_eq!(simm7.scale_ty, I64);
                        let reg = stack_reg();
                        sink.put4(enc_ldst_pair(0b1010100010, simm7, reg, rt, rt2));
                    }
                }
            }
            &Inst::LoadP64 {
                rt,
                rt2,
                ref mem,
                flags,
            } => {
                let rt = rt.to_reg();
                let rt2 = rt2.to_reg();
                let mem = mem.clone();
                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }

                match &mem {
                    &PairAMode::SignedOffset { reg, simm7 } => {
                        assert_eq!(simm7.scale_ty, I64);
                        sink.put4(enc_ldst_pair(0b1010100101, simm7, reg, rt, rt2));
                    }
                    &PairAMode::SPPreIndexed { simm7 } => {
                        assert_eq!(simm7.scale_ty, I64);
                        let reg = stack_reg();
                        sink.put4(enc_ldst_pair(0b1010100111, simm7, reg, rt, rt2));
                    }
                    &PairAMode::SPPostIndexed { simm7 } => {
                        assert_eq!(simm7.scale_ty, I64);
                        let reg = stack_reg();
                        sink.put4(enc_ldst_pair(0b1010100011, simm7, reg, rt, rt2));
                    }
                }
            }
            &Inst::FpuLoadP64 {
                rt,
                rt2,
                ref mem,
                flags,
            }
            | &Inst::FpuLoadP128 {
                rt,
                rt2,
                ref mem,
                flags,
            } => {
                let rt = rt.to_reg();
                let rt2 = rt2.to_reg();
                let mem = mem.clone();

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }

                let opc = match self {
                    &Inst::FpuLoadP64 { .. } => 0b01,
                    &Inst::FpuLoadP128 { .. } => 0b10,
                    _ => unreachable!(),
                };

                match &mem {
                    &PairAMode::SignedOffset { reg, simm7 } => {
                        assert!(simm7.scale_ty == F64 || simm7.scale_ty == I8X16);
                        sink.put4(enc_ldst_vec_pair(opc, 0b10, true, simm7, reg, rt, rt2));
                    }
                    &PairAMode::SPPreIndexed { simm7 } => {
                        assert!(simm7.scale_ty == F64 || simm7.scale_ty == I8X16);
                        let reg = stack_reg();
                        sink.put4(enc_ldst_vec_pair(opc, 0b11, true, simm7, reg, rt, rt2));
                    }
                    &PairAMode::SPPostIndexed { simm7 } => {
                        assert!(simm7.scale_ty == F64 || simm7.scale_ty == I8X16);
                        let reg = stack_reg();
                        sink.put4(enc_ldst_vec_pair(opc, 0b01, true, simm7, reg, rt, rt2));
                    }
                }
            }
            &Inst::FpuStoreP64 {
                rt,
                rt2,
                ref mem,
                flags,
            }
            | &Inst::FpuStoreP128 {
                rt,
                rt2,
                ref mem,
                flags,
            } => {
                let mem = mem.clone();

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual store instruction starts.
                    sink.add_trap(trap_code);
                }

                let opc = match self {
                    &Inst::FpuStoreP64 { .. } => 0b01,
                    &Inst::FpuStoreP128 { .. } => 0b10,
                    _ => unreachable!(),
                };

                match &mem {
                    &PairAMode::SignedOffset { reg, simm7 } => {
                        assert!(simm7.scale_ty == F64 || simm7.scale_ty == I8X16);
                        sink.put4(enc_ldst_vec_pair(opc, 0b10, false, simm7, reg, rt, rt2));
                    }
                    &PairAMode::SPPreIndexed { simm7 } => {
                        assert!(simm7.scale_ty == F64 || simm7.scale_ty == I8X16);
                        let reg = stack_reg();
                        sink.put4(enc_ldst_vec_pair(opc, 0b11, false, simm7, reg, rt, rt2));
                    }
                    &PairAMode::SPPostIndexed { simm7 } => {
                        assert!(simm7.scale_ty == F64 || simm7.scale_ty == I8X16);
                        let reg = stack_reg();
                        sink.put4(enc_ldst_vec_pair(opc, 0b01, false, simm7, reg, rt, rt2));
                    }
                }
            }
            &Inst::Mov { size, rd, rm } => {
                assert!(rd.to_reg().class() == rm.class());
                assert!(rm.class() == RegClass::Int);

                match size {
                    OperandSize::Size64 => {
                        // MOV to SP is interpreted as MOV to XZR instead. And our codegen
                        // should never MOV to XZR.
                        assert!(rd.to_reg() != stack_reg());

                        if rm == stack_reg() {
                            // We can't use ORR here, so use an `add rd, sp, #0` instead.
                            let imm12 = Imm12::maybe_from_u64(0).unwrap();
                            sink.put4(enc_arith_rr_imm12(
                                0b100_10001,
                                imm12.shift_bits(),
                                imm12.imm_bits(),
                                rm,
                                rd,
                            ));
                        } else {
                            // Encoded as ORR rd, rm, zero.
                            sink.put4(enc_arith_rrr(0b10101010_000, 0b000_000, rd, zero_reg(), rm));
                        }
                    }
                    OperandSize::Size32 => {
                        // MOV to SP is interpreted as MOV to XZR instead. And our codegen
                        // should never MOV to XZR.
                        assert!(machreg_to_gpr(rd.to_reg()) != 31);
                        // Encoded as ORR rd, rm, zero.
                        sink.put4(enc_arith_rrr(0b00101010_000, 0b000_000, rd, zero_reg(), rm));
                    }
                }
            }
            &Inst::MovFromPReg { rd, rm } => {
                let rm: Reg = rm.into();
                debug_assert!(
                    [
                        regs::fp_reg(),
                        regs::stack_reg(),
                        regs::link_reg(),
                        regs::pinned_reg()
                    ]
                    .contains(&rm)
                );
                assert!(rm.class() == RegClass::Int);
                assert!(rd.to_reg().class() == rm.class());
                let size = OperandSize::Size64;
                Inst::Mov { size, rd, rm }.emit(sink, emit_info, state);
            }
            &Inst::MovToPReg { rd, rm } => {
                let rd: Writable<Reg> = Writable::from_reg(rd.into());
                debug_assert!(
                    [
                        regs::fp_reg(),
                        regs::stack_reg(),
                        regs::link_reg(),
                        regs::pinned_reg()
                    ]
                    .contains(&rd.to_reg())
                );
                assert!(rd.to_reg().class() == RegClass::Int);
                assert!(rm.class() == rd.to_reg().class());
                let size = OperandSize::Size64;
                Inst::Mov { size, rd, rm }.emit(sink, emit_info, state);
            }
            &Inst::MovWide { op, rd, imm, size } => {
                sink.put4(enc_move_wide(op, rd, imm, size));
            }
            &Inst::MovK { rd, rn, imm, size } => {
                debug_assert_eq!(rn, rd.to_reg());
                sink.put4(enc_movk(rd, imm, size));
            }
            &Inst::CSel { rd, rn, rm, cond } => {
                sink.put4(enc_csel(rd, rn, rm, cond, 0, 0));
            }
            &Inst::CSNeg { rd, rn, rm, cond } => {
                sink.put4(enc_csel(rd, rn, rm, cond, 1, 1));
            }
            &Inst::CSet { rd, cond } => {
                sink.put4(enc_csel(rd, zero_reg(), zero_reg(), cond.invert(), 0, 1));
            }
            &Inst::CSetm { rd, cond } => {
                sink.put4(enc_csel(rd, zero_reg(), zero_reg(), cond.invert(), 1, 0));
            }
            &Inst::CCmp {
                size,
                rn,
                rm,
                nzcv,
                cond,
            } => {
                sink.put4(enc_ccmp(size, rn, rm, nzcv, cond));
            }
            &Inst::CCmpImm {
                size,
                rn,
                imm,
                nzcv,
                cond,
            } => {
                sink.put4(enc_ccmp_imm(size, rn, imm, nzcv, cond));
            }
            &Inst::AtomicRMW {
                ty,
                op,
                rs,
                rt,
                rn,
                flags,
            } => {
                if let Some(trap_code) = flags.trap_code() {
                    sink.add_trap(trap_code);
                }

                sink.put4(enc_acq_rel(ty, op, rs, rt, rn));
            }
            &Inst::AtomicRMWLoop { ty, op, flags, .. } => {
                /* Emit this:
                     again:
                      ldaxr{,b,h}  x/w27, [x25]
                      // maybe sign extend
                      op          x28, x27, x26 // op is add,sub,and,orr,eor
                      stlxr{,b,h}  w24, x/w28, [x25]
                      cbnz        x24, again

                   Operand conventions:
                      IN:  x25 (addr), x26 (2nd arg for op)
                      OUT: x27 (old value), x24 (trashed), x28 (trashed)

                   It is unfortunate that, per the ARM documentation, x28 cannot be used for
                   both the store-data and success-flag operands of stlxr.  This causes the
                   instruction's behaviour to be "CONSTRAINED UNPREDICTABLE", so we use x24
                   instead for the success-flag.
                */
                // TODO: We should not hardcode registers here, a better idea would be to
                // pass some scratch registers in the AtomicRMWLoop pseudo-instruction, and use those
                let xzr = zero_reg();
                let x24 = xreg(24);
                let x25 = xreg(25);
                let x26 = xreg(26);
                let x27 = xreg(27);
                let x28 = xreg(28);
                let x24wr = writable_xreg(24);
                let x27wr = writable_xreg(27);
                let x28wr = writable_xreg(28);
                let again_label = sink.get_label();

                // again:
                sink.bind_label(again_label, &mut state.ctrl_plane);

                if let Some(trap_code) = flags.trap_code() {
                    sink.add_trap(trap_code);
                }

                sink.put4(enc_ldaxr(ty, x27wr, x25)); // ldaxr x27, [x25]
                let size = OperandSize::from_ty(ty);
                let sign_ext = match op {
                    AtomicRMWLoopOp::Smin | AtomicRMWLoopOp::Smax => match ty {
                        I16 => Some((ExtendOp::SXTH, 16)),
                        I8 => Some((ExtendOp::SXTB, 8)),
                        _ => None,
                    },
                    _ => None,
                };

                // sxt{b|h} the loaded result if necessary.
                if sign_ext.is_some() {
                    let (_, from_bits) = sign_ext.unwrap();
                    Inst::Extend {
                        rd: x27wr,
                        rn: x27,
                        signed: true,
                        from_bits,
                        to_bits: size.bits(),
                    }
                    .emit(sink, emit_info, state);
                }

                match op {
                    AtomicRMWLoopOp::Xchg => {} // do nothing
                    AtomicRMWLoopOp::Nand => {
                        // and x28, x27, x26
                        // mvn x28, x28

                        Inst::AluRRR {
                            alu_op: ALUOp::And,
                            size,
                            rd: x28wr,
                            rn: x27,
                            rm: x26,
                        }
                        .emit(sink, emit_info, state);

                        Inst::AluRRR {
                            alu_op: ALUOp::OrrNot,
                            size,
                            rd: x28wr,
                            rn: xzr,
                            rm: x28,
                        }
                        .emit(sink, emit_info, state);
                    }
                    AtomicRMWLoopOp::Umin
                    | AtomicRMWLoopOp::Umax
                    | AtomicRMWLoopOp::Smin
                    | AtomicRMWLoopOp::Smax => {
                        // cmp x27, x26 {?sxt}
                        // csel.op x28, x27, x26

                        let cond = match op {
                            AtomicRMWLoopOp::Umin => Cond::Lo,
                            AtomicRMWLoopOp::Umax => Cond::Hi,
                            AtomicRMWLoopOp::Smin => Cond::Lt,
                            AtomicRMWLoopOp::Smax => Cond::Gt,
                            _ => unreachable!(),
                        };

                        if sign_ext.is_some() {
                            let (extendop, _) = sign_ext.unwrap();
                            Inst::AluRRRExtend {
                                alu_op: ALUOp::SubS,
                                size,
                                rd: writable_zero_reg(),
                                rn: x27,
                                rm: x26,
                                extendop,
                            }
                            .emit(sink, emit_info, state);
                        } else {
                            Inst::AluRRR {
                                alu_op: ALUOp::SubS,
                                size,
                                rd: writable_zero_reg(),
                                rn: x27,
                                rm: x26,
                            }
                            .emit(sink, emit_info, state);
                        }

                        Inst::CSel {
                            cond,
                            rd: x28wr,
                            rn: x27,
                            rm: x26,
                        }
                        .emit(sink, emit_info, state);
                    }
                    _ => {
                        // add/sub/and/orr/eor x28, x27, x26
                        let alu_op = match op {
                            AtomicRMWLoopOp::Add => ALUOp::Add,
                            AtomicRMWLoopOp::Sub => ALUOp::Sub,
                            AtomicRMWLoopOp::And => ALUOp::And,
                            AtomicRMWLoopOp::Orr => ALUOp::Orr,
                            AtomicRMWLoopOp::Eor => ALUOp::Eor,
                            AtomicRMWLoopOp::Nand
                            | AtomicRMWLoopOp::Umin
                            | AtomicRMWLoopOp::Umax
                            | AtomicRMWLoopOp::Smin
                            | AtomicRMWLoopOp::Smax
                            | AtomicRMWLoopOp::Xchg => unreachable!(),
                        };

                        Inst::AluRRR {
                            alu_op,
                            size,
                            rd: x28wr,
                            rn: x27,
                            rm: x26,
                        }
                        .emit(sink, emit_info, state);
                    }
                }

                if let Some(trap_code) = flags.trap_code() {
                    sink.add_trap(trap_code);
                }
                if op == AtomicRMWLoopOp::Xchg {
                    sink.put4(enc_stlxr(ty, x24wr, x26, x25)); // stlxr w24, x26, [x25]
                } else {
                    sink.put4(enc_stlxr(ty, x24wr, x28, x25)); // stlxr w24, x28, [x25]
                }

                // cbnz w24, again
                // Note, we're actually testing x24, and relying on the default zero-high-half
                // rule in the assignment that `stlxr` does.
                let br_offset = sink.cur_offset();
                sink.put4(enc_conditional_br(
                    BranchTarget::Label(again_label),
                    CondBrKind::NotZero(x24, OperandSize::Size64),
                ));
                sink.use_label_at_offset(br_offset, again_label, LabelUse::Branch19);
            }
            &Inst::AtomicCAS {
                rd,
                rs,
                rt,
                rn,
                ty,
                flags,
            } => {
                debug_assert_eq!(rd.to_reg(), rs);
                let size = match ty {
                    I8 => 0b00,
                    I16 => 0b01,
                    I32 => 0b10,
                    I64 => 0b11,
                    _ => panic!("Unsupported type: {ty}"),
                };

                if let Some(trap_code) = flags.trap_code() {
                    sink.add_trap(trap_code);
                }

                sink.put4(enc_cas(size, rd, rt, rn));
            }
            &Inst::AtomicCASLoop { ty, flags, .. } => {
                /* Emit this:
                    again:
                     ldaxr{,b,h} x/w27, [x25]
                     cmp         x27, x/w26 uxt{b,h}
                     b.ne        out
                     stlxr{,b,h} w24, x/w28, [x25]
                     cbnz        x24, again
                    out:

                  Operand conventions:
                     IN:  x25 (addr), x26 (expected value), x28 (replacement value)
                     OUT: x27 (old value), x24 (trashed)
                */
                let x24 = xreg(24);
                let x25 = xreg(25);
                let x26 = xreg(26);
                let x27 = xreg(27);
                let x28 = xreg(28);
                let xzrwr = writable_zero_reg();
                let x24wr = writable_xreg(24);
                let x27wr = writable_xreg(27);
                let again_label = sink.get_label();
                let out_label = sink.get_label();

                // again:
                sink.bind_label(again_label, &mut state.ctrl_plane);

                if let Some(trap_code) = flags.trap_code() {
                    sink.add_trap(trap_code);
                }

                // ldaxr x27, [x25]
                sink.put4(enc_ldaxr(ty, x27wr, x25));

                // The top 32-bits are zero-extended by the ldaxr so we don't
                // have to use UXTW, just the x-form of the register.
                let (bit21, extend_op) = match ty {
                    I8 => (0b1, 0b000000),
                    I16 => (0b1, 0b001000),
                    _ => (0b0, 0b000000),
                };
                let bits_31_21 = 0b111_01011_000 | bit21;
                // cmp x27, x26 (== subs xzr, x27, x26)
                sink.put4(enc_arith_rrr(bits_31_21, extend_op, xzrwr, x27, x26));

                // b.ne out
                let br_out_offset = sink.cur_offset();
                sink.put4(enc_conditional_br(
                    BranchTarget::Label(out_label),
                    CondBrKind::Cond(Cond::Ne),
                ));
                sink.use_label_at_offset(br_out_offset, out_label, LabelUse::Branch19);

                if let Some(trap_code) = flags.trap_code() {
                    sink.add_trap(trap_code);
                }

                sink.put4(enc_stlxr(ty, x24wr, x28, x25)); // stlxr w24, x28, [x25]

                // cbnz w24, again.
                // Note, we're actually testing x24, and relying on the default zero-high-half
                // rule in the assignment that `stlxr` does.
                let br_again_offset = sink.cur_offset();
                sink.put4(enc_conditional_br(
                    BranchTarget::Label(again_label),
                    CondBrKind::NotZero(x24, OperandSize::Size64),
                ));
                sink.use_label_at_offset(br_again_offset, again_label, LabelUse::Branch19);

                // out:
                sink.bind_label(out_label, &mut state.ctrl_plane);
            }
            &Inst::LoadAcquire {
                access_ty,
                rt,
                rn,
                flags,
            } => {
                if let Some(trap_code) = flags.trap_code() {
                    sink.add_trap(trap_code);
                }

                sink.put4(enc_ldar(access_ty, rt, rn));
            }
            &Inst::StoreRelease {
                access_ty,
                rt,
                rn,
                flags,
            } => {
                if let Some(trap_code) = flags.trap_code() {
                    sink.add_trap(trap_code);
                }

                sink.put4(enc_stlr(access_ty, rt, rn));
            }
            &Inst::Fence {} => {
                sink.put4(enc_dmb_ish()); // dmb ish
            }
            &Inst::Csdb {} => {
                sink.put4(0xd503229f);
            }
            &Inst::FpuMove32 { rd, rn } => {
                sink.put4(enc_fpurr(0b000_11110_00_1_000000_10000, rd, rn));
            }
            &Inst::FpuMove64 { rd, rn } => {
                sink.put4(enc_fpurr(0b000_11110_01_1_000000_10000, rd, rn));
            }
            &Inst::FpuMove128 { rd, rn } => {
                sink.put4(enc_vecmov(/* 16b = */ true, rd, rn));
            }
            &Inst::FpuMoveFromVec { rd, rn, idx, size } => {
                let (imm5, shift, mask) = match size.lane_size() {
                    ScalarSize::Size32 => (0b00100, 3, 0b011),
                    ScalarSize::Size64 => (0b01000, 4, 0b001),
                    _ => unimplemented!(),
                };
                debug_assert_eq!(idx & mask, idx);
                let imm5 = imm5 | ((idx as u32) << shift);
                sink.put4(
                    0b010_11110000_00000_000001_00000_00000
                        | (imm5 << 16)
                        | (machreg_to_vec(rn) << 5)
                        | machreg_to_vec(rd.to_reg()),
                );
            }
            &Inst::FpuExtend { rd, rn, size } => {
                sink.put4(enc_fpurr(
                    0b000_11110_00_1_000000_10000 | (size.ftype() << 12),
                    rd,
                    rn,
                ));
            }
            &Inst::FpuRR {
                fpu_op,
                size,
                rd,
                rn,
            } => {
                let top22 = match fpu_op {
                    FPUOp1::Abs => 0b000_11110_00_1_000001_10000,
                    FPUOp1::Neg => 0b000_11110_00_1_000010_10000,
                    FPUOp1::Sqrt => 0b000_11110_00_1_000011_10000,
                    FPUOp1::Cvt32To64 => {
                        debug_assert_eq!(size, ScalarSize::Size32);
                        0b000_11110_00_1_000101_10000
                    }
                    FPUOp1::Cvt64To32 => {
                        debug_assert_eq!(size, ScalarSize::Size64);
                        0b000_11110_01_1_000100_10000
                    }
                };
                let top22 = top22 | size.ftype() << 12;
                sink.put4(enc_fpurr(top22, rd, rn));
            }
            &Inst::FpuRRR {
                fpu_op,
                size,
                rd,
                rn,
                rm,
            } => {
                let top22 = match fpu_op {
                    FPUOp2::Add => 0b000_11110_00_1_00000_001010,
                    FPUOp2::Sub => 0b000_11110_00_1_00000_001110,
                    FPUOp2::Mul => 0b000_11110_00_1_00000_000010,
                    FPUOp2::Div => 0b000_11110_00_1_00000_000110,
                    FPUOp2::Max => 0b000_11110_00_1_00000_010010,
                    FPUOp2::Min => 0b000_11110_00_1_00000_010110,
                };
                let top22 = top22 | size.ftype() << 12;
                sink.put4(enc_fpurrr(top22, rd, rn, rm));
            }
            &Inst::FpuRRI { fpu_op, rd, rn } => match fpu_op {
                FPUOpRI::UShr32(imm) => {
                    debug_assert_eq!(32, imm.lane_size_in_bits);
                    sink.put4(
                        0b0_0_1_011110_0000000_00_0_0_0_1_00000_00000
                            | imm.enc() << 16
                            | machreg_to_vec(rn) << 5
                            | machreg_to_vec(rd.to_reg()),
                    )
                }
                FPUOpRI::UShr64(imm) => {
                    debug_assert_eq!(64, imm.lane_size_in_bits);
                    sink.put4(
                        0b01_1_111110_0000000_00_0_0_0_1_00000_00000
                            | imm.enc() << 16
                            | machreg_to_vec(rn) << 5
                            | machreg_to_vec(rd.to_reg()),
                    )
                }
            },
            &Inst::FpuRRIMod { fpu_op, rd, ri, rn } => {
                debug_assert_eq!(rd.to_reg(), ri);
                match fpu_op {
                    FPUOpRIMod::Sli64(imm) => {
                        debug_assert_eq!(64, imm.lane_size_in_bits);
                        sink.put4(
                            0b01_1_111110_0000000_010101_00000_00000
                                | imm.enc() << 16
                                | machreg_to_vec(rn) << 5
                                | machreg_to_vec(rd.to_reg()),
                        )
                    }
                    FPUOpRIMod::Sli32(imm) => {
                        debug_assert_eq!(32, imm.lane_size_in_bits);
                        sink.put4(
                            0b0_0_1_011110_0000000_010101_00000_00000
                                | imm.enc() << 16
                                | machreg_to_vec(rn) << 5
                                | machreg_to_vec(rd.to_reg()),
                        )
                    }
                }
            }
            &Inst::FpuRRRR {
                fpu_op,
                size,
                rd,
                rn,
                rm,
                ra,
            } => {
                let top17 = match fpu_op {
                    FPUOp3::MAdd => 0b000_11111_00_0_00000_0,
                    FPUOp3::MSub => 0b000_11111_00_0_00000_1,
                    FPUOp3::NMAdd => 0b000_11111_00_1_00000_0,
                    FPUOp3::NMSub => 0b000_11111_00_1_00000_1,
                };
                let top17 = top17 | size.ftype() << 7;
                sink.put4(enc_fpurrrr(top17, rd, rn, rm, ra));
            }
            &Inst::VecMisc { op, rd, rn, size } => {
                let (q, enc_size) = size.enc_size();
                let (u, bits_12_16, size) = match op {
                    VecMisc2::Not => (0b1, 0b00101, 0b00),
                    VecMisc2::Neg => (0b1, 0b01011, enc_size),
                    VecMisc2::Abs => (0b0, 0b01011, enc_size),
                    VecMisc2::Fabs => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b0, 0b01111, enc_size)
                    }
                    VecMisc2::Fneg => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b1, 0b01111, enc_size)
                    }
                    VecMisc2::Fsqrt => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b1, 0b11111, enc_size)
                    }
                    VecMisc2::Rev16 => {
                        debug_assert_eq!(size, VectorSize::Size8x16);
                        (0b0, 0b00001, enc_size)
                    }
                    VecMisc2::Rev32 => {
                        debug_assert!(size == VectorSize::Size8x16 || size == VectorSize::Size16x8);
                        (0b1, 0b00000, enc_size)
                    }
                    VecMisc2::Rev64 => {
                        debug_assert!(
                            size == VectorSize::Size8x16
                                || size == VectorSize::Size16x8
                                || size == VectorSize::Size32x4
                        );
                        (0b0, 0b00000, enc_size)
                    }
                    VecMisc2::Fcvtzs => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b0, 0b11011, enc_size)
                    }
                    VecMisc2::Fcvtzu => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b1, 0b11011, enc_size)
                    }
                    VecMisc2::Scvtf => {
                        debug_assert!(size == VectorSize::Size32x4 || size == VectorSize::Size64x2);
                        (0b0, 0b11101, enc_size & 0b1)
                    }
                    VecMisc2::Ucvtf => {
                        debug_assert!(size == VectorSize::Size32x4 || size == VectorSize::Size64x2);
                        (0b1, 0b11101, enc_size & 0b1)
                    }
                    VecMisc2::Frintn => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b0, 0b11000, enc_size & 0b01)
                    }
                    VecMisc2::Frintz => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b0, 0b11001, enc_size)
                    }
                    VecMisc2::Frintm => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b0, 0b11001, enc_size & 0b01)
                    }
                    VecMisc2::Frintp => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b0, 0b11000, enc_size)
                    }
                    VecMisc2::Cnt => {
                        debug_assert!(size == VectorSize::Size8x8 || size == VectorSize::Size8x16);
                        (0b0, 0b00101, enc_size)
                    }
                    VecMisc2::Cmeq0 => (0b0, 0b01001, enc_size),
                    VecMisc2::Cmge0 => (0b1, 0b01000, enc_size),
                    VecMisc2::Cmgt0 => (0b0, 0b01000, enc_size),
                    VecMisc2::Cmle0 => (0b1, 0b01001, enc_size),
                    VecMisc2::Cmlt0 => (0b0, 0b01010, enc_size),
                    VecMisc2::Fcmeq0 => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b0, 0b01101, enc_size)
                    }
                    VecMisc2::Fcmge0 => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b1, 0b01100, enc_size)
                    }
                    VecMisc2::Fcmgt0 => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b0, 0b01100, enc_size)
                    }
                    VecMisc2::Fcmle0 => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b1, 0b01101, enc_size)
                    }
                    VecMisc2::Fcmlt0 => {
                        debug_assert!(
                            size == VectorSize::Size32x2
                                || size == VectorSize::Size32x4
                                || size == VectorSize::Size64x2
                        );
                        (0b0, 0b01110, enc_size)
                    }
                };
                sink.put4(enc_vec_rr_misc((q << 1) | u, size, bits_12_16, rd, rn));
            }
            &Inst::VecLanes { op, rd, rn, size } => {
                let (q, size) = match size {
                    VectorSize::Size8x8 => (0b0, 0b00),
                    VectorSize::Size8x16 => (0b1, 0b00),
                    VectorSize::Size16x4 => (0b0, 0b01),
                    VectorSize::Size16x8 => (0b1, 0b01),
                    VectorSize::Size32x4 => (0b1, 0b10),
                    _ => unreachable!(),
                };
                let (u, opcode) = match op {
                    VecLanesOp::Uminv => (0b1, 0b11010),
                    VecLanesOp::Addv => (0b0, 0b11011),
                };
                sink.put4(enc_vec_lanes(q, u, size, opcode, rd, rn));
            }
            &Inst::VecShiftImm {
                op,
                rd,
                rn,
                size,
                imm,
            } => {
                let (is_shr, mut template) = match op {
                    VecShiftImmOp::Ushr => (true, 0b_001_011110_0000_000_000001_00000_00000_u32),
                    VecShiftImmOp::Sshr => (true, 0b_000_011110_0000_000_000001_00000_00000_u32),
                    VecShiftImmOp::Shl => (false, 0b_000_011110_0000_000_010101_00000_00000_u32),
                };
                if size.is_128bits() {
                    template |= 0b1 << 30;
                }
                let imm = imm as u32;
                // Deal with the somewhat strange encoding scheme for, and limits on,
                // the shift amount.
                let immh_immb = match (size.lane_size(), is_shr) {
                    (ScalarSize::Size64, true) if imm >= 1 && imm <= 64 => {
                        0b_1000_000_u32 | (64 - imm)
                    }
                    (ScalarSize::Size32, true) if imm >= 1 && imm <= 32 => {
                        0b_0100_000_u32 | (32 - imm)
                    }
                    (ScalarSize::Size16, true) if imm >= 1 && imm <= 16 => {
                        0b_0010_000_u32 | (16 - imm)
                    }
                    (ScalarSize::Size8, true) if imm >= 1 && imm <= 8 => {
                        0b_0001_000_u32 | (8 - imm)
                    }
                    (ScalarSize::Size64, false) if imm <= 63 => 0b_1000_000_u32 | imm,
                    (ScalarSize::Size32, false) if imm <= 31 => 0b_0100_000_u32 | imm,
                    (ScalarSize::Size16, false) if imm <= 15 => 0b_0010_000_u32 | imm,
                    (ScalarSize::Size8, false) if imm <= 7 => 0b_0001_000_u32 | imm,
                    _ => panic!(
                        "aarch64: Inst::VecShiftImm: emit: invalid op/size/imm {op:?}, {size:?}, {imm:?}"
                    ),
                };
                let rn_enc = machreg_to_vec(rn);
                let rd_enc = machreg_to_vec(rd.to_reg());
                sink.put4(template | (immh_immb << 16) | (rn_enc << 5) | rd_enc);
            }
            &Inst::VecShiftImmMod {
                op,
                rd,
                ri,
                rn,
                size,
                imm,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);
                let (is_shr, mut template) = match op {
                    VecShiftImmModOp::Sli => (false, 0b_001_011110_0000_000_010101_00000_00000_u32),
                };
                if size.is_128bits() {
                    template |= 0b1 << 30;
                }
                let imm = imm as u32;
                // Deal with the somewhat strange encoding scheme for, and limits on,
                // the shift amount.
                let immh_immb = match (size.lane_size(), is_shr) {
                    (ScalarSize::Size64, true) if imm >= 1 && imm <= 64 => {
                        0b_1000_000_u32 | (64 - imm)
                    }
                    (ScalarSize::Size32, true) if imm >= 1 && imm <= 32 => {
                        0b_0100_000_u32 | (32 - imm)
                    }
                    (ScalarSize::Size16, true) if imm >= 1 && imm <= 16 => {
                        0b_0010_000_u32 | (16 - imm)
                    }
                    (ScalarSize::Size8, true) if imm >= 1 && imm <= 8 => {
                        0b_0001_000_u32 | (8 - imm)
                    }
                    (ScalarSize::Size64, false) if imm <= 63 => 0b_1000_000_u32 | imm,
                    (ScalarSize::Size32, false) if imm <= 31 => 0b_0100_000_u32 | imm,
                    (ScalarSize::Size16, false) if imm <= 15 => 0b_0010_000_u32 | imm,
                    (ScalarSize::Size8, false) if imm <= 7 => 0b_0001_000_u32 | imm,
                    _ => panic!(
                        "aarch64: Inst::VecShiftImmMod: emit: invalid op/size/imm {op:?}, {size:?}, {imm:?}"
                    ),
                };
                let rn_enc = machreg_to_vec(rn);
                let rd_enc = machreg_to_vec(rd.to_reg());
                sink.put4(template | (immh_immb << 16) | (rn_enc << 5) | rd_enc);
            }
            &Inst::VecExtract { rd, rn, rm, imm4 } => {
                if imm4 < 16 {
                    let template = 0b_01_101110_000_00000_0_0000_0_00000_00000_u32;
                    let rm_enc = machreg_to_vec(rm);
                    let rn_enc = machreg_to_vec(rn);
                    let rd_enc = machreg_to_vec(rd.to_reg());
                    sink.put4(
                        template | (rm_enc << 16) | ((imm4 as u32) << 11) | (rn_enc << 5) | rd_enc,
                    );
                } else {
                    panic!("aarch64: Inst::VecExtract: emit: invalid extract index {imm4}");
                }
            }
            &Inst::VecTbl { rd, rn, rm } => {
                sink.put4(enc_tbl(/* is_extension = */ false, 0b00, rd, rn, rm));
            }
            &Inst::VecTblExt { rd, ri, rn, rm } => {
                debug_assert_eq!(rd.to_reg(), ri);
                sink.put4(enc_tbl(/* is_extension = */ true, 0b00, rd, rn, rm));
            }
            &Inst::VecTbl2 { rd, rn, rn2, rm } => {
                assert_eq!(machreg_to_vec(rn2), (machreg_to_vec(rn) + 1) % 32);
                sink.put4(enc_tbl(/* is_extension = */ false, 0b01, rd, rn, rm));
            }
            &Inst::VecTbl2Ext {
                rd,
                ri,
                rn,
                rn2,
                rm,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);
                assert_eq!(machreg_to_vec(rn2), (machreg_to_vec(rn) + 1) % 32);
                sink.put4(enc_tbl(/* is_extension = */ true, 0b01, rd, rn, rm));
            }
            &Inst::FpuCmp { size, rn, rm } => {
                sink.put4(enc_fcmp(size, rn, rm));
            }
            &Inst::FpuToInt { op, rd, rn } => {
                let top16 = match op {
                    // FCVTZS (32/32-bit)
                    FpuToIntOp::F32ToI32 => 0b000_11110_00_1_11_000,
                    // FCVTZU (32/32-bit)
                    FpuToIntOp::F32ToU32 => 0b000_11110_00_1_11_001,
                    // FCVTZS (32/64-bit)
                    FpuToIntOp::F32ToI64 => 0b100_11110_00_1_11_000,
                    // FCVTZU (32/64-bit)
                    FpuToIntOp::F32ToU64 => 0b100_11110_00_1_11_001,
                    // FCVTZS (64/32-bit)
                    FpuToIntOp::F64ToI32 => 0b000_11110_01_1_11_000,
                    // FCVTZU (64/32-bit)
                    FpuToIntOp::F64ToU32 => 0b000_11110_01_1_11_001,
                    // FCVTZS (64/64-bit)
                    FpuToIntOp::F64ToI64 => 0b100_11110_01_1_11_000,
                    // FCVTZU (64/64-bit)
                    FpuToIntOp::F64ToU64 => 0b100_11110_01_1_11_001,
                };
                sink.put4(enc_fputoint(top16, rd, rn));
            }
            &Inst::IntToFpu { op, rd, rn } => {
                let top16 = match op {
                    // SCVTF (32/32-bit)
                    IntToFpuOp::I32ToF32 => 0b000_11110_00_1_00_010,
                    // UCVTF (32/32-bit)
                    IntToFpuOp::U32ToF32 => 0b000_11110_00_1_00_011,
                    // SCVTF (64/32-bit)
                    IntToFpuOp::I64ToF32 => 0b100_11110_00_1_00_010,
                    // UCVTF (64/32-bit)
                    IntToFpuOp::U64ToF32 => 0b100_11110_00_1_00_011,
                    // SCVTF (32/64-bit)
                    IntToFpuOp::I32ToF64 => 0b000_11110_01_1_00_010,
                    // UCVTF (32/64-bit)
                    IntToFpuOp::U32ToF64 => 0b000_11110_01_1_00_011,
                    // SCVTF (64/64-bit)
                    IntToFpuOp::I64ToF64 => 0b100_11110_01_1_00_010,
                    // UCVTF (64/64-bit)
                    IntToFpuOp::U64ToF64 => 0b100_11110_01_1_00_011,
                };
                sink.put4(enc_inttofpu(top16, rd, rn));
            }
            &Inst::FpuCSel16 { rd, rn, rm, cond } => {
                sink.put4(enc_fcsel(rd, rn, rm, cond, ScalarSize::Size16));
            }
            &Inst::FpuCSel32 { rd, rn, rm, cond } => {
                sink.put4(enc_fcsel(rd, rn, rm, cond, ScalarSize::Size32));
            }
            &Inst::FpuCSel64 { rd, rn, rm, cond } => {
                sink.put4(enc_fcsel(rd, rn, rm, cond, ScalarSize::Size64));
            }
            &Inst::FpuRound { op, rd, rn } => {
                let top22 = match op {
                    FpuRoundMode::Minus32 => 0b000_11110_00_1_001_010_10000,
                    FpuRoundMode::Minus64 => 0b000_11110_01_1_001_010_10000,
                    FpuRoundMode::Plus32 => 0b000_11110_00_1_001_001_10000,
                    FpuRoundMode::Plus64 => 0b000_11110_01_1_001_001_10000,
                    FpuRoundMode::Zero32 => 0b000_11110_00_1_001_011_10000,
                    FpuRoundMode::Zero64 => 0b000_11110_01_1_001_011_10000,
                    FpuRoundMode::Nearest32 => 0b000_11110_00_1_001_000_10000,
                    FpuRoundMode::Nearest64 => 0b000_11110_01_1_001_000_10000,
                };
                sink.put4(enc_fround(top22, rd, rn));
            }
            &Inst::MovToFpu { rd, rn, size } => {
                let template = match size {
                    ScalarSize::Size16 => 0b000_11110_11_1_00_111_000000_00000_00000,
                    ScalarSize::Size32 => 0b000_11110_00_1_00_111_000000_00000_00000,
                    ScalarSize::Size64 => 0b100_11110_01_1_00_111_000000_00000_00000,
                    _ => unreachable!(),
                };
                sink.put4(template | (machreg_to_gpr(rn) << 5) | machreg_to_vec(rd.to_reg()));
            }
            &Inst::FpuMoveFPImm { rd, imm, size } => {
                sink.put4(
                    0b000_11110_00_1_00_000_000100_00000_00000
                        | size.ftype() << 22
                        | ((imm.enc_bits() as u32) << 13)
                        | machreg_to_vec(rd.to_reg()),
                );
            }
            &Inst::MovToVec {
                rd,
                ri,
                rn,
                idx,
                size,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);
                let (imm5, shift) = match size.lane_size() {
                    ScalarSize::Size8 => (0b00001, 1),
                    ScalarSize::Size16 => (0b00010, 2),
                    ScalarSize::Size32 => (0b00100, 3),
                    ScalarSize::Size64 => (0b01000, 4),
                    _ => unreachable!(),
                };
                debug_assert_eq!(idx & (0b11111 >> shift), idx);
                let imm5 = imm5 | ((idx as u32) << shift);
                sink.put4(
                    0b010_01110000_00000_0_0011_1_00000_00000
                        | (imm5 << 16)
                        | (machreg_to_gpr(rn) << 5)
                        | machreg_to_vec(rd.to_reg()),
                );
            }
            &Inst::MovFromVec { rd, rn, idx, size } => {
                let (q, imm5, shift, mask) = match size {
                    ScalarSize::Size8 => (0b0, 0b00001, 1, 0b1111),
                    ScalarSize::Size16 => (0b0, 0b00010, 2, 0b0111),
                    ScalarSize::Size32 => (0b0, 0b00100, 3, 0b0011),
                    ScalarSize::Size64 => (0b1, 0b01000, 4, 0b0001),
                    _ => panic!("Unexpected scalar FP operand size: {size:?}"),
                };
                debug_assert_eq!(idx & mask, idx);
                let imm5 = imm5 | ((idx as u32) << shift);
                sink.put4(
                    0b000_01110000_00000_0_0111_1_00000_00000
                        | (q << 30)
                        | (imm5 << 16)
                        | (machreg_to_vec(rn) << 5)
                        | machreg_to_gpr(rd.to_reg()),
                );
            }
            &Inst::MovFromVecSigned {
                rd,
                rn,
                idx,
                size,
                scalar_size,
            } => {
                let (imm5, shift, half) = match size {
                    VectorSize::Size8x8 => (0b00001, 1, true),
                    VectorSize::Size8x16 => (0b00001, 1, false),
                    VectorSize::Size16x4 => (0b00010, 2, true),
                    VectorSize::Size16x8 => (0b00010, 2, false),
                    VectorSize::Size32x2 => {
                        debug_assert_ne!(scalar_size, OperandSize::Size32);
                        (0b00100, 3, true)
                    }
                    VectorSize::Size32x4 => {
                        debug_assert_ne!(scalar_size, OperandSize::Size32);
                        (0b00100, 3, false)
                    }
                    _ => panic!("Unexpected vector operand size"),
                };
                debug_assert_eq!(idx & (0b11111 >> (half as u32 + shift)), idx);
                let imm5 = imm5 | ((idx as u32) << shift);
                sink.put4(
                    0b000_01110000_00000_0_0101_1_00000_00000
                        | (scalar_size.is64() as u32) << 30
                        | (imm5 << 16)
                        | (machreg_to_vec(rn) << 5)
                        | machreg_to_gpr(rd.to_reg()),
                );
            }
            &Inst::VecDup { rd, rn, size } => {
                let q = size.is_128bits() as u32;
                let imm5 = match size.lane_size() {
                    ScalarSize::Size8 => 0b00001,
                    ScalarSize::Size16 => 0b00010,
                    ScalarSize::Size32 => 0b00100,
                    ScalarSize::Size64 => 0b01000,
                    _ => unreachable!(),
                };
                sink.put4(
                    0b0_0_0_01110000_00000_000011_00000_00000
                        | (q << 30)
                        | (imm5 << 16)
                        | (machreg_to_gpr(rn) << 5)
                        | machreg_to_vec(rd.to_reg()),
                );
            }
            &Inst::VecDupFromFpu { rd, rn, size, lane } => {
                let q = size.is_128bits() as u32;
                let imm5 = match size.lane_size() {
                    ScalarSize::Size8 => {
                        assert!(lane < 16);
                        0b00001 | (u32::from(lane) << 1)
                    }
                    ScalarSize::Size16 => {
                        assert!(lane < 8);
                        0b00010 | (u32::from(lane) << 2)
                    }
                    ScalarSize::Size32 => {
                        assert!(lane < 4);
                        0b00100 | (u32::from(lane) << 3)
                    }
                    ScalarSize::Size64 => {
                        assert!(lane < 2);
                        0b01000 | (u32::from(lane) << 4)
                    }
                    _ => unimplemented!(),
                };
                sink.put4(
                    0b000_01110000_00000_000001_00000_00000
                        | (q << 30)
                        | (imm5 << 16)
                        | (machreg_to_vec(rn) << 5)
                        | machreg_to_vec(rd.to_reg()),
                );
            }
            &Inst::VecDupFPImm { rd, imm, size } => {
                let imm = imm.enc_bits();
                let op = match size.lane_size() {
                    ScalarSize::Size32 => 0,
                    ScalarSize::Size64 => 1,
                    _ => unimplemented!(),
                };
                let q_op = op | ((size.is_128bits() as u32) << 1);

                sink.put4(enc_asimd_mod_imm(rd, q_op, 0b1111, imm));
            }
            &Inst::VecDupImm {
                rd,
                imm,
                invert,
                size,
            } => {
                let (imm, shift, shift_ones) = imm.value();
                let (op, cmode) = match size.lane_size() {
                    ScalarSize::Size8 => {
                        assert!(!invert);
                        assert_eq!(shift, 0);

                        (0, 0b1110)
                    }
                    ScalarSize::Size16 => {
                        let s = shift & 8;

                        assert!(!shift_ones);
                        assert_eq!(s, shift);

                        (invert as u32, 0b1000 | (s >> 2))
                    }
                    ScalarSize::Size32 => {
                        if shift_ones {
                            assert!(shift == 8 || shift == 16);

                            (invert as u32, 0b1100 | (shift >> 4))
                        } else {
                            let s = shift & 24;

                            assert_eq!(s, shift);

                            (invert as u32, 0b0000 | (s >> 2))
                        }
                    }
                    ScalarSize::Size64 => {
                        assert!(!invert);
                        assert_eq!(shift, 0);

                        (1, 0b1110)
                    }
                    _ => unreachable!(),
                };
                let q_op = op | ((size.is_128bits() as u32) << 1);

                sink.put4(enc_asimd_mod_imm(rd, q_op, cmode, imm));
            }
            &Inst::VecExtend {
                t,
                rd,
                rn,
                high_half,
                lane_size,
            } => {
                let immh = match lane_size {
                    ScalarSize::Size16 => 0b001,
                    ScalarSize::Size32 => 0b010,
                    ScalarSize::Size64 => 0b100,
                    _ => panic!("Unexpected VecExtend to lane size of {lane_size:?}"),
                };
                let u = match t {
                    VecExtendOp::Sxtl => 0b0,
                    VecExtendOp::Uxtl => 0b1,
                };
                sink.put4(
                    0b000_011110_0000_000_101001_00000_00000
                        | ((high_half as u32) << 30)
                        | (u << 29)
                        | (immh << 19)
                        | (machreg_to_vec(rn) << 5)
                        | machreg_to_vec(rd.to_reg()),
                );
            }
            &Inst::VecRRLong {
                op,
                rd,
                rn,
                high_half,
            } => {
                let (u, size, bits_12_16) = match op {
                    VecRRLongOp::Fcvtl16 => (0b0, 0b00, 0b10111),
                    VecRRLongOp::Fcvtl32 => (0b0, 0b01, 0b10111),
                    VecRRLongOp::Shll8 => (0b1, 0b00, 0b10011),
                    VecRRLongOp::Shll16 => (0b1, 0b01, 0b10011),
                    VecRRLongOp::Shll32 => (0b1, 0b10, 0b10011),
                };

                sink.put4(enc_vec_rr_misc(
                    ((high_half as u32) << 1) | u,
                    size,
                    bits_12_16,
                    rd,
                    rn,
                ));
            }
            &Inst::VecRRNarrowLow {
                op,
                rd,
                rn,
                lane_size,
            }
            | &Inst::VecRRNarrowHigh {
                op,
                rd,
                rn,
                lane_size,
                ..
            } => {
                let high_half = match self {
                    &Inst::VecRRNarrowLow { .. } => false,
                    &Inst::VecRRNarrowHigh { .. } => true,
                    _ => unreachable!(),
                };

                let size = match lane_size {
                    ScalarSize::Size8 => 0b00,
                    ScalarSize::Size16 => 0b01,
                    ScalarSize::Size32 => 0b10,
                    _ => panic!("unsupported size: {lane_size:?}"),
                };

                // Floats use a single bit, to encode either half or single.
                let size = match op {
                    VecRRNarrowOp::Fcvtn => size >> 1,
                    _ => size,
                };

                let (u, bits_12_16) = match op {
                    VecRRNarrowOp::Xtn => (0b0, 0b10010),
                    VecRRNarrowOp::Sqxtn => (0b0, 0b10100),
                    VecRRNarrowOp::Sqxtun => (0b1, 0b10010),
                    VecRRNarrowOp::Uqxtn => (0b1, 0b10100),
                    VecRRNarrowOp::Fcvtn => (0b0, 0b10110),
                };

                sink.put4(enc_vec_rr_misc(
                    ((high_half as u32) << 1) | u,
                    size,
                    bits_12_16,
                    rd,
                    rn,
                ));
            }
            &Inst::VecMovElement {
                rd,
                ri,
                rn,
                dest_idx,
                src_idx,
                size,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);
                let (imm5, shift) = match size.lane_size() {
                    ScalarSize::Size8 => (0b00001, 1),
                    ScalarSize::Size16 => (0b00010, 2),
                    ScalarSize::Size32 => (0b00100, 3),
                    ScalarSize::Size64 => (0b01000, 4),
                    _ => unreachable!(),
                };
                let mask = 0b11111 >> shift;
                debug_assert_eq!(dest_idx & mask, dest_idx);
                debug_assert_eq!(src_idx & mask, src_idx);
                let imm4 = (src_idx as u32) << (shift - 1);
                let imm5 = imm5 | ((dest_idx as u32) << shift);
                sink.put4(
                    0b011_01110000_00000_0_0000_1_00000_00000
                        | (imm5 << 16)
                        | (imm4 << 11)
                        | (machreg_to_vec(rn) << 5)
                        | machreg_to_vec(rd.to_reg()),
                );
            }
            &Inst::VecRRPair { op, rd, rn } => {
                let bits_12_16 = match op {
                    VecPairOp::Addp => 0b11011,
                };

                sink.put4(enc_vec_rr_pair(bits_12_16, rd, rn));
            }
            &Inst::VecRRRLong {
                rd,
                rn,
                rm,
                alu_op,
                high_half,
            } => {
                let (u, size, bit14) = match alu_op {
                    VecRRRLongOp::Smull8 => (0b0, 0b00, 0b1),
                    VecRRRLongOp::Smull16 => (0b0, 0b01, 0b1),
                    VecRRRLongOp::Smull32 => (0b0, 0b10, 0b1),
                    VecRRRLongOp::Umull8 => (0b1, 0b00, 0b1),
                    VecRRRLongOp::Umull16 => (0b1, 0b01, 0b1),
                    VecRRRLongOp::Umull32 => (0b1, 0b10, 0b1),
                };
                sink.put4(enc_vec_rrr_long(
                    high_half as u32,
                    u,
                    size,
                    bit14,
                    rm,
                    rn,
                    rd,
                ));
            }
            &Inst::VecRRRLongMod {
                rd,
                ri,
                rn,
                rm,
                alu_op,
                high_half,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);
                let (u, size, bit14) = match alu_op {
                    VecRRRLongModOp::Umlal8 => (0b1, 0b00, 0b0),
                    VecRRRLongModOp::Umlal16 => (0b1, 0b01, 0b0),
                    VecRRRLongModOp::Umlal32 => (0b1, 0b10, 0b0),
                };
                sink.put4(enc_vec_rrr_long(
                    high_half as u32,
                    u,
                    size,
                    bit14,
                    rm,
                    rn,
                    rd,
                ));
            }
            &Inst::VecRRPairLong { op, rd, rn } => {
                let (u, size) = match op {
                    VecRRPairLongOp::Saddlp8 => (0b0, 0b0),
                    VecRRPairLongOp::Uaddlp8 => (0b1, 0b0),
                    VecRRPairLongOp::Saddlp16 => (0b0, 0b1),
                    VecRRPairLongOp::Uaddlp16 => (0b1, 0b1),
                };

                sink.put4(enc_vec_rr_pair_long(u, size, rd, rn));
            }
            &Inst::VecRRR {
                rd,
                rn,
                rm,
                alu_op,
                size,
            } => {
                let (q, enc_size) = size.enc_size();
                let is_float = match alu_op {
                    VecALUOp::Fcmeq
                    | VecALUOp::Fcmgt
                    | VecALUOp::Fcmge
                    | VecALUOp::Fadd
                    | VecALUOp::Fsub
                    | VecALUOp::Fdiv
                    | VecALUOp::Fmax
                    | VecALUOp::Fmin
                    | VecALUOp::Fmul => true,
                    _ => false,
                };

                let (top11, bit15_10) = match alu_op {
                    VecALUOp::Sqadd => (0b000_01110_00_1 | enc_size << 1, 0b000011),
                    VecALUOp::Sqsub => (0b000_01110_00_1 | enc_size << 1, 0b001011),
                    VecALUOp::Uqadd => (0b001_01110_00_1 | enc_size << 1, 0b000011),
                    VecALUOp::Uqsub => (0b001_01110_00_1 | enc_size << 1, 0b001011),
                    VecALUOp::Cmeq => (0b001_01110_00_1 | enc_size << 1, 0b100011),
                    VecALUOp::Cmge => (0b000_01110_00_1 | enc_size << 1, 0b001111),
                    VecALUOp::Cmgt => (0b000_01110_00_1 | enc_size << 1, 0b001101),
                    VecALUOp::Cmhi => (0b001_01110_00_1 | enc_size << 1, 0b001101),
                    VecALUOp::Cmhs => (0b001_01110_00_1 | enc_size << 1, 0b001111),
                    VecALUOp::Fcmeq => (0b000_01110_00_1, 0b111001),
                    VecALUOp::Fcmgt => (0b001_01110_10_1, 0b111001),
                    VecALUOp::Fcmge => (0b001_01110_00_1, 0b111001),
                    // The following logical instructions operate on bytes, so are not encoded differently
                    // for the different vector types.
                    VecALUOp::And => (0b000_01110_00_1, 0b000111),
                    VecALUOp::Bic => (0b000_01110_01_1, 0b000111),
                    VecALUOp::Orr => (0b000_01110_10_1, 0b000111),
                    VecALUOp::Eor => (0b001_01110_00_1, 0b000111),
                    VecALUOp::Umaxp => {
                        debug_assert_ne!(size, VectorSize::Size64x2);

                        (0b001_01110_00_1 | enc_size << 1, 0b101001)
                    }
                    VecALUOp::Add => (0b000_01110_00_1 | enc_size << 1, 0b100001),
                    VecALUOp::Sub => (0b001_01110_00_1 | enc_size << 1, 0b100001),
                    VecALUOp::Mul => {
                        debug_assert_ne!(size, VectorSize::Size64x2);
                        (0b000_01110_00_1 | enc_size << 1, 0b100111)
                    }
                    VecALUOp::Sshl => (0b000_01110_00_1 | enc_size << 1, 0b010001),
                    VecALUOp::Ushl => (0b001_01110_00_1 | enc_size << 1, 0b010001),
                    VecALUOp::Umin => {
                        debug_assert_ne!(size, VectorSize::Size64x2);

                        (0b001_01110_00_1 | enc_size << 1, 0b011011)
                    }
                    VecALUOp::Smin => {
                        debug_assert_ne!(size, VectorSize::Size64x2);

                        (0b000_01110_00_1 | enc_size << 1, 0b011011)
                    }
                    VecALUOp::Umax => {
                        debug_assert_ne!(size, VectorSize::Size64x2);

                        (0b001_01110_00_1 | enc_size << 1, 0b011001)
                    }
                    VecALUOp::Smax => {
                        debug_assert_ne!(size, VectorSize::Size64x2);

                        (0b000_01110_00_1 | enc_size << 1, 0b011001)
                    }
                    VecALUOp::Urhadd => {
                        debug_assert_ne!(size, VectorSize::Size64x2);

                        (0b001_01110_00_1 | enc_size << 1, 0b000101)
                    }
                    VecALUOp::Fadd => (0b000_01110_00_1, 0b110101),
                    VecALUOp::Fsub => (0b000_01110_10_1, 0b110101),
                    VecALUOp::Fdiv => (0b001_01110_00_1, 0b111111),
                    VecALUOp::Fmax => (0b000_01110_00_1, 0b111101),
                    VecALUOp::Fmin => (0b000_01110_10_1, 0b111101),
                    VecALUOp::Fmul => (0b001_01110_00_1, 0b110111),
                    VecALUOp::Addp => (0b000_01110_00_1 | enc_size << 1, 0b101111),
                    VecALUOp::Zip1 => (0b01001110_00_0 | enc_size << 1, 0b001110),
                    VecALUOp::Zip2 => (0b01001110_00_0 | enc_size << 1, 0b011110),
                    VecALUOp::Sqrdmulh => {
                        debug_assert!(
                            size.lane_size() == ScalarSize::Size16
                                || size.lane_size() == ScalarSize::Size32
                        );

                        (0b001_01110_00_1 | enc_size << 1, 0b101101)
                    }
                    VecALUOp::Uzp1 => (0b01001110_00_0 | enc_size << 1, 0b000110),
                    VecALUOp::Uzp2 => (0b01001110_00_0 | enc_size << 1, 0b010110),
                    VecALUOp::Trn1 => (0b01001110_00_0 | enc_size << 1, 0b001010),
                    VecALUOp::Trn2 => (0b01001110_00_0 | enc_size << 1, 0b011010),
                };
                let top11 = if is_float {
                    top11 | size.enc_float_size() << 1
                } else {
                    top11
                };
                sink.put4(enc_vec_rrr(top11 | q << 9, rm, bit15_10, rn, rd));
            }
            &Inst::VecRRRMod {
                rd,
                ri,
                rn,
                rm,
                alu_op,
                size,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);
                let (q, _enc_size) = size.enc_size();

                let (top11, bit15_10) = match alu_op {
                    VecALUModOp::Bsl => (0b001_01110_01_1, 0b000111),
                    VecALUModOp::Fmla => {
                        (0b000_01110_00_1 | (size.enc_float_size() << 1), 0b110011)
                    }
                    VecALUModOp::Fmls => {
                        (0b000_01110_10_1 | (size.enc_float_size() << 1), 0b110011)
                    }
                };
                sink.put4(enc_vec_rrr(top11 | q << 9, rm, bit15_10, rn, rd));
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
                debug_assert_eq!(rd.to_reg(), ri);
                let idx = u32::from(idx);

                let (q, _size) = size.enc_size();
                let o2 = match alu_op {
                    VecALUModOp::Fmla => 0b0,
                    VecALUModOp::Fmls => 0b1,
                    _ => unreachable!(),
                };

                let (h, l) = match size {
                    VectorSize::Size32x4 => {
                        assert!(idx < 4);
                        (idx >> 1, idx & 1)
                    }
                    VectorSize::Size64x2 => {
                        assert!(idx < 2);
                        (idx, 0)
                    }
                    _ => unreachable!(),
                };

                let top11 = 0b000_011111_00 | (q << 9) | (size.enc_float_size() << 1) | l;
                let bit15_10 = 0b000100 | (o2 << 4) | (h << 1);
                sink.put4(enc_vec_rrr(top11, rm, bit15_10, rn, rd));
            }
            &Inst::VecLoadReplicate {
                rd,
                rn,
                size,
                flags,
            } => {
                let (q, size) = size.enc_size();

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }

                sink.put4(enc_ldst_vec(q, size, rn, rd));
            }
            &Inst::VecCSel { rd, rn, rm, cond } => {
                /* Emit this:
                      b.cond  else
                      mov     rd, rm
                      b       out
                     else:
                      mov     rd, rn
                     out:

                   Note, we could do better in the cases where rd == rn or rd == rm.
                */
                let else_label = sink.get_label();
                let out_label = sink.get_label();

                // b.cond else
                let br_else_offset = sink.cur_offset();
                sink.put4(enc_conditional_br(
                    BranchTarget::Label(else_label),
                    CondBrKind::Cond(cond),
                ));
                sink.use_label_at_offset(br_else_offset, else_label, LabelUse::Branch19);

                // mov rd, rm
                sink.put4(enc_vecmov(/* 16b = */ true, rd, rm));

                // b out
                let b_out_offset = sink.cur_offset();
                sink.use_label_at_offset(b_out_offset, out_label, LabelUse::Branch26);
                sink.add_uncond_branch(b_out_offset, b_out_offset + 4, out_label);
                sink.put4(enc_jump26(0b000101, 0 /* will be fixed up later */));

                // else:
                sink.bind_label(else_label, &mut state.ctrl_plane);

                // mov rd, rn
                sink.put4(enc_vecmov(/* 16b = */ true, rd, rn));

                // out:
                sink.bind_label(out_label, &mut state.ctrl_plane);
            }
            &Inst::MovToNZCV { rn } => {
                sink.put4(0xd51b4200 | machreg_to_gpr(rn));
            }
            &Inst::MovFromNZCV { rd } => {
                sink.put4(0xd53b4200 | machreg_to_gpr(rd.to_reg()));
            }
            &Inst::Extend {
                rd,
                rn,
                signed: false,
                from_bits: 1,
                to_bits,
            } => {
                assert!(to_bits <= 64);
                // Reduce zero-extend-from-1-bit to:
                // - and rd, rn, #1
                // Note: This is special cased as UBFX may take more cycles
                // than AND on smaller cores.
                let imml = ImmLogic::maybe_from_u64(1, I32).unwrap();
                Inst::AluRRImmLogic {
                    alu_op: ALUOp::And,
                    size: OperandSize::Size32,
                    rd,
                    rn,
                    imml,
                }
                .emit(sink, emit_info, state);
            }
            &Inst::Extend {
                rd,
                rn,
                signed: false,
                from_bits: 32,
                to_bits: 64,
            } => {
                let mov = Inst::Mov {
                    size: OperandSize::Size32,
                    rd,
                    rm: rn,
                };
                mov.emit(sink, emit_info, state);
            }
            &Inst::Extend {
                rd,
                rn,
                signed,
                from_bits,
                to_bits,
            } => {
                let (opc, size) = if signed {
                    (0b00, OperandSize::from_bits(to_bits))
                } else {
                    (0b10, OperandSize::Size32)
                };
                sink.put4(enc_bfm(opc, size, rd, rn, 0, from_bits - 1));
            }
            &Inst::Jump { ref dest } => {
                let off = sink.cur_offset();
                // Indicate that the jump uses a label, if so, so that a fixup can occur later.
                if let Some(l) = dest.as_label() {
                    sink.use_label_at_offset(off, l, LabelUse::Branch26);
                    sink.add_uncond_branch(off, off + 4, l);
                }
                // Emit the jump itself.
                sink.put4(enc_jump26(0b000101, dest.as_offset26_or_zero()));
            }
            &Inst::Args { .. } | &Inst::Rets { .. } => {
                // Nothing: this is a pseudoinstruction that serves
                // only to constrain registers at a certain point.
            }
            &Inst::Ret {} => {
                sink.put4(0xd65f03c0);
            }
            &Inst::AuthenticatedRet { key, is_hint } => {
                let (op2, is_hint) = match key {
                    APIKey::AZ => (0b100, true),
                    APIKey::ASP => (0b101, is_hint),
                    APIKey::BZ => (0b110, true),
                    APIKey::BSP => (0b111, is_hint),
                };

                if is_hint {
                    sink.put4(key.enc_auti_hint());
                    Inst::Ret {}.emit(sink, emit_info, state);
                } else {
                    sink.put4(0xd65f0bff | (op2 << 9)); // reta{key}
                }
            }
            &Inst::Call { ref info } => {
                let user_stack_map = state.take_stack_map();
                sink.add_reloc(Reloc::Arm64Call, &info.dest, 0);
                sink.put4(enc_jump26(0b100101, 0));
                if let Some(s) = user_stack_map {
                    let offset = sink.cur_offset();
                    sink.push_user_stack_map(state, offset, s);
                }

                if let Some(try_call) = info.try_call_info.as_ref() {
                    sink.add_try_call_site(try_call.exception_handlers(&state.frame_layout));
                } else {
                    sink.add_call_site();
                }

                if info.callee_pop_size > 0 {
                    let callee_pop_size =
                        i32::try_from(info.callee_pop_size).expect("callee popped more than 2GB");
                    for inst in AArch64MachineDeps::gen_sp_reg_adjust(-callee_pop_size) {
                        inst.emit(sink, emit_info, state);
                    }
                }

                // Load any stack-carried return values.
                info.emit_retval_loads::<AArch64MachineDeps, _, _>(
                    state.frame_layout().stackslots_size,
                    |inst| inst.emit(sink, emit_info, state),
                    |needed_space| Some(Inst::EmitIsland { needed_space }),
                );

                // If this is a try-call, jump to the continuation
                // (normal-return) block.
                if let Some(try_call) = info.try_call_info.as_ref() {
                    let jmp = Inst::Jump {
                        dest: BranchTarget::Label(try_call.continuation),
                    };
                    jmp.emit(sink, emit_info, state);
                }

                // We produce an island above if needed, so disable
                // the worst-case-size check in this case.
                start_off = sink.cur_offset();
            }
            &Inst::CallInd { ref info } => {
                let user_stack_map = state.take_stack_map();
                sink.put4(
                    0b1101011_0001_11111_000000_00000_00000 | (machreg_to_gpr(info.dest) << 5),
                );
                if let Some(s) = user_stack_map {
                    let offset = sink.cur_offset();
                    sink.push_user_stack_map(state, offset, s);
                }

                if let Some(try_call) = info.try_call_info.as_ref() {
                    sink.add_try_call_site(try_call.exception_handlers(&state.frame_layout));
                } else {
                    sink.add_call_site();
                }

                if info.callee_pop_size > 0 {
                    let callee_pop_size =
                        i32::try_from(info.callee_pop_size).expect("callee popped more than 2GB");
                    for inst in AArch64MachineDeps::gen_sp_reg_adjust(-callee_pop_size) {
                        inst.emit(sink, emit_info, state);
                    }
                }

                // Load any stack-carried return values.
                info.emit_retval_loads::<AArch64MachineDeps, _, _>(
                    state.frame_layout().stackslots_size,
                    |inst| inst.emit(sink, emit_info, state),
                    |needed_space| Some(Inst::EmitIsland { needed_space }),
                );

                // If this is a try-call, jump to the continuation
                // (normal-return) block.
                if let Some(try_call) = info.try_call_info.as_ref() {
                    let jmp = Inst::Jump {
                        dest: BranchTarget::Label(try_call.continuation),
                    };
                    jmp.emit(sink, emit_info, state);
                }

                // We produce an island above if needed, so disable
                // the worst-case-size check in this case.
                start_off = sink.cur_offset();
            }
            &Inst::ReturnCall { ref info } => {
                emit_return_call_common_sequence(sink, emit_info, state, info);

                // Note: this is not `Inst::Jump { .. }.emit(..)` because we
                // have different metadata in this case: we don't have a label
                // for the target, but rather a function relocation.
                sink.add_reloc(Reloc::Arm64Call, &info.dest, 0);
                sink.put4(enc_jump26(0b000101, 0));
                sink.add_call_site();

                // `emit_return_call_common_sequence` emits an island if
                // necessary, so we can safely disable the worst-case-size check
                // in this case.
                start_off = sink.cur_offset();
            }
            &Inst::ReturnCallInd { ref info } => {
                emit_return_call_common_sequence(sink, emit_info, state, info);

                Inst::IndirectBr {
                    rn: info.dest,
                    targets: vec![],
                }
                .emit(sink, emit_info, state);
                sink.add_call_site();

                // `emit_return_call_common_sequence` emits an island if
                // necessary, so we can safely disable the worst-case-size check
                // in this case.
                start_off = sink.cur_offset();
            }
            &Inst::CondBr {
                taken,
                not_taken,
                kind,
            } => {
                // Conditional part first.
                let cond_off = sink.cur_offset();
                if let Some(l) = taken.as_label() {
                    sink.use_label_at_offset(cond_off, l, LabelUse::Branch19);
                    let inverted = enc_conditional_br(taken, kind.invert()).to_le_bytes();
                    sink.add_cond_branch(cond_off, cond_off + 4, l, &inverted[..]);
                }
                sink.put4(enc_conditional_br(taken, kind));

                // Unconditional part next.
                let uncond_off = sink.cur_offset();
                if let Some(l) = not_taken.as_label() {
                    sink.use_label_at_offset(uncond_off, l, LabelUse::Branch26);
                    sink.add_uncond_branch(uncond_off, uncond_off + 4, l);
                }
                sink.put4(enc_jump26(0b000101, not_taken.as_offset26_or_zero()));
            }
            &Inst::TestBitAndBranch {
                taken,
                not_taken,
                kind,
                rn,
                bit,
            } => {
                // Emit the conditional branch first
                let cond_off = sink.cur_offset();
                if let Some(l) = taken.as_label() {
                    sink.use_label_at_offset(cond_off, l, LabelUse::Branch14);
                    let inverted =
                        enc_test_bit_and_branch(kind.complement(), taken, rn, bit).to_le_bytes();
                    sink.add_cond_branch(cond_off, cond_off + 4, l, &inverted[..]);
                }
                sink.put4(enc_test_bit_and_branch(kind, taken, rn, bit));

                // Unconditional part next.
                let uncond_off = sink.cur_offset();
                if let Some(l) = not_taken.as_label() {
                    sink.use_label_at_offset(uncond_off, l, LabelUse::Branch26);
                    sink.add_uncond_branch(uncond_off, uncond_off + 4, l);
                }
                sink.put4(enc_jump26(0b000101, not_taken.as_offset26_or_zero()));
            }
            &Inst::TrapIf { kind, trap_code } => {
                let label = sink.defer_trap(trap_code);
                // condbr KIND, LABEL
                let off = sink.cur_offset();
                sink.put4(enc_conditional_br(BranchTarget::Label(label), kind));
                sink.use_label_at_offset(off, label, LabelUse::Branch19);
            }
            &Inst::IndirectBr { rn, .. } => {
                sink.put4(enc_br(rn));
            }
            &Inst::Nop0 => {}
            &Inst::Nop4 => {
                sink.put4(0xd503201f);
            }
            &Inst::Brk => {
                sink.put4(0xd43e0000);
            }
            &Inst::Udf { trap_code } => {
                sink.add_trap(trap_code);
                sink.put_data(Inst::TRAP_OPCODE);
            }
            &Inst::Adr { rd, off } => {
                assert!(off > -(1 << 20));
                assert!(off < (1 << 20));
                sink.put4(enc_adr(off, rd));
            }
            &Inst::Adrp { rd, off } => {
                assert!(off > -(1 << 20));
                assert!(off < (1 << 20));
                sink.put4(enc_adrp(off, rd));
            }
            &Inst::Word4 { data } => {
                sink.put4(data);
            }
            &Inst::Word8 { data } => {
                sink.put8(data);
            }
            &Inst::JTSequence {
                ridx,
                rtmp1,
                rtmp2,
                default,
                ref targets,
                ..
            } => {
                // This sequence is *one* instruction in the vcode, and is expanded only here at
                // emission time, because we cannot allow the regalloc to insert spills/reloads in
                // the middle; we depend on hardcoded PC-rel addressing below.

                // Branch to default when condition code from prior comparison indicates.
                let br =
                    enc_conditional_br(BranchTarget::Label(default), CondBrKind::Cond(Cond::Hs));

                // No need to inform the sink's branch folding logic about this branch, because it
                // will not be merged with any other branch, flipped, or elided (it is not preceded
                // or succeeded by any other branch). Just emit it with the label use.
                let default_br_offset = sink.cur_offset();
                sink.use_label_at_offset(default_br_offset, default, LabelUse::Branch19);
                sink.put4(br);

                // Overwrite the index with a zero when the above
                // branch misspeculates (Spectre mitigation). Save the
                // resulting index in rtmp2.
                let inst = Inst::CSel {
                    rd: rtmp2,
                    cond: Cond::Hs,
                    rn: zero_reg(),
                    rm: ridx,
                };
                inst.emit(sink, emit_info, state);
                // Prevent any data value speculation.
                Inst::Csdb.emit(sink, emit_info, state);

                // Load address of jump table
                let inst = Inst::Adr { rd: rtmp1, off: 16 };
                inst.emit(sink, emit_info, state);
                // Load value out of jump table
                let inst = Inst::SLoad32 {
                    rd: rtmp2,
                    mem: AMode::reg_plus_reg_scaled_extended(
                        rtmp1.to_reg(),
                        rtmp2.to_reg(),
                        ExtendOp::UXTW,
                    ),
                    flags: MemFlags::trusted(),
                };
                inst.emit(sink, emit_info, state);
                // Add base of jump table to jump-table-sourced block offset
                let inst = Inst::AluRRR {
                    alu_op: ALUOp::Add,
                    size: OperandSize::Size64,
                    rd: rtmp1,
                    rn: rtmp1.to_reg(),
                    rm: rtmp2.to_reg(),
                };
                inst.emit(sink, emit_info, state);
                // Branch to computed address. (`targets` here is only used for successor queries
                // and is not needed for emission.)
                let inst = Inst::IndirectBr {
                    rn: rtmp1.to_reg(),
                    targets: vec![],
                };
                inst.emit(sink, emit_info, state);
                // Emit jump table (table of 32-bit offsets).
                let jt_off = sink.cur_offset();
                for &target in targets.iter() {
                    let word_off = sink.cur_offset();
                    // off_into_table is an addend here embedded in the label to be later patched
                    // at the end of codegen. The offset is initially relative to this jump table
                    // entry; with the extra addend, it'll be relative to the jump table's start,
                    // after patching.
                    let off_into_table = word_off - jt_off;
                    sink.use_label_at_offset(word_off, target, LabelUse::PCRel32);
                    sink.put4(off_into_table);
                }

                // Lowering produces an EmitIsland before using a JTSequence, so we can safely
                // disable the worst-case-size check in this case.
                start_off = sink.cur_offset();
            }
            &Inst::LoadExtName {
                rd,
                ref name,
                offset,
            } => {
                if emit_info.0.is_pic() {
                    // See this CE Example for the variations of this with and without BTI & PAUTH
                    // https://godbolt.org/z/ncqjbbvvn
                    //
                    // Emit the following code:
                    //   adrp    rd, :got:X
                    //   ldr     rd, [rd, :got_lo12:X]

                    // adrp rd, symbol
                    sink.add_reloc(Reloc::Aarch64AdrGotPage21, &**name, 0);
                    let inst = Inst::Adrp { rd, off: 0 };
                    inst.emit(sink, emit_info, state);

                    // ldr rd, [rd, :got_lo12:X]
                    sink.add_reloc(Reloc::Aarch64Ld64GotLo12Nc, &**name, 0);
                    let inst = Inst::ULoad64 {
                        rd,
                        mem: AMode::reg(rd.to_reg()),
                        flags: MemFlags::trusted(),
                    };
                    inst.emit(sink, emit_info, state);
                } else {
                    // With absolute offsets we set up a load from a preallocated space, and then jump
                    // over it.
                    //
                    // Emit the following code:
                    //   ldr     rd, #8
                    //   b       #0x10
                    //   <8 byte space>

                    let inst = Inst::ULoad64 {
                        rd,
                        mem: AMode::Label {
                            label: MemLabel::PCRel(8),
                        },
                        flags: MemFlags::trusted(),
                    };
                    inst.emit(sink, emit_info, state);
                    let inst = Inst::Jump {
                        dest: BranchTarget::ResolvedOffset(12),
                    };
                    inst.emit(sink, emit_info, state);
                    sink.add_reloc(Reloc::Abs8, &**name, offset);
                    sink.put8(0);
                }
            }
            &Inst::LoadAddr { rd, ref mem } => {
                let mem = mem.clone();
                let (mem_insts, mem) = mem_finalize(Some(sink), &mem, I8, state);
                for inst in mem_insts.into_iter() {
                    inst.emit(sink, emit_info, state);
                }

                let (reg, index_reg, offset) = match mem {
                    AMode::RegExtended { rn, rm, extendop } => {
                        let r = rn;
                        (r, Some((rm, extendop)), 0)
                    }
                    AMode::Unscaled { rn, simm9 } => {
                        let r = rn;
                        (r, None, simm9.value())
                    }
                    AMode::UnsignedOffset { rn, uimm12 } => {
                        let r = rn;
                        (r, None, uimm12.value() as i32)
                    }
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

                    add.emit(sink, emit_info, state);
                } else if offset == 0 {
                    if reg != rd.to_reg() {
                        let mov = Inst::Mov {
                            size: OperandSize::Size64,
                            rd,
                            rm: reg,
                        };

                        mov.emit(sink, emit_info, state);
                    }
                } else if let Some(imm12) = Imm12::maybe_from_u64(abs_offset) {
                    let add = Inst::AluRRImm12 {
                        alu_op,
                        size: OperandSize::Size64,
                        rd,
                        rn: reg,
                        imm12,
                    };
                    add.emit(sink, emit_info, state);
                } else {
                    // Use `tmp2` here: `reg` may be `spilltmp` if the `AMode` on this instruction
                    // was initially an `SPOffset`. Assert that `tmp2` is truly free to use. Note
                    // that no other instructions will be inserted here (we're emitting directly),
                    // and a live range of `tmp2` should not span this instruction, so this use
                    // should otherwise be correct.
                    debug_assert!(rd.to_reg() != tmp2_reg());
                    debug_assert!(reg != tmp2_reg());
                    let tmp = writable_tmp2_reg();
                    for insn in Inst::load_constant(tmp, abs_offset).into_iter() {
                        insn.emit(sink, emit_info, state);
                    }
                    let add = Inst::AluRRR {
                        alu_op,
                        size: OperandSize::Size64,
                        rd,
                        rn: reg,
                        rm: tmp.to_reg(),
                    };
                    add.emit(sink, emit_info, state);
                }
            }
            &Inst::Paci { key } => {
                let (crm, op2) = match key {
                    APIKey::AZ => (0b0011, 0b000),
                    APIKey::ASP => (0b0011, 0b001),
                    APIKey::BZ => (0b0011, 0b010),
                    APIKey::BSP => (0b0011, 0b011),
                };

                sink.put4(0xd503211f | (crm << 8) | (op2 << 5));
            }
            &Inst::Xpaclri => sink.put4(0xd50320ff),
            &Inst::Bti { targets } => {
                let targets = match targets {
                    BranchTargetType::None => 0b00,
                    BranchTargetType::C => 0b01,
                    BranchTargetType::J => 0b10,
                    BranchTargetType::JC => 0b11,
                };

                sink.put4(0xd503241f | targets << 6);
            }
            &Inst::EmitIsland { needed_space } => {
                if sink.island_needed(needed_space + 4) {
                    let jump_around_label = sink.get_label();
                    let jmp = Inst::Jump {
                        dest: BranchTarget::Label(jump_around_label),
                    };
                    jmp.emit(sink, emit_info, state);
                    sink.emit_island(needed_space + 4, &mut state.ctrl_plane);
                    sink.bind_label(jump_around_label, &mut state.ctrl_plane);
                }
            }

            &Inst::ElfTlsGetAddr {
                ref symbol,
                rd,
                tmp,
            } => {
                assert_eq!(xreg(0), rd.to_reg());

                // See the original proposal for TLSDESC.
                // http://www.fsfla.org/~lxoliva/writeups/TLS/paper-lk2006.pdf
                //
                // Implement the TLSDESC instruction sequence:
                //   adrp x0, :tlsdesc:tlsvar
                //   ldr  tmp, [x0, :tlsdesc_lo12:tlsvar]
                //   add  x0, x0, :tlsdesc_lo12:tlsvar
                //   blr  tmp
                //   mrs  tmp, tpidr_el0
                //   add  x0, x0, tmp
                //
                // This is the instruction sequence that GCC emits for ELF GD TLS Relocations in aarch64
                // See: https://gcc.godbolt.org/z/e4j7MdErh

                // adrp x0, :tlsdesc:tlsvar
                sink.add_reloc(Reloc::Aarch64TlsDescAdrPage21, &**symbol, 0);
                Inst::Adrp { rd, off: 0 }.emit(sink, emit_info, state);

                // ldr  tmp, [x0, :tlsdesc_lo12:tlsvar]
                sink.add_reloc(Reloc::Aarch64TlsDescLd64Lo12, &**symbol, 0);
                Inst::ULoad64 {
                    rd: tmp,
                    mem: AMode::reg(rd.to_reg()),
                    flags: MemFlags::trusted(),
                }
                .emit(sink, emit_info, state);

                // add x0, x0, :tlsdesc_lo12:tlsvar
                sink.add_reloc(Reloc::Aarch64TlsDescAddLo12, &**symbol, 0);
                Inst::AluRRImm12 {
                    alu_op: ALUOp::Add,
                    size: OperandSize::Size64,
                    rd,
                    rn: rd.to_reg(),
                    imm12: Imm12::maybe_from_u64(0).unwrap(),
                }
                .emit(sink, emit_info, state);

                // blr tmp
                sink.add_reloc(Reloc::Aarch64TlsDescCall, &**symbol, 0);
                Inst::CallInd {
                    info: crate::isa::Box::new(CallInfo::empty(tmp.to_reg(), CallConv::SystemV)),
                }
                .emit(sink, emit_info, state);

                // mrs tmp, tpidr_el0
                sink.put4(0xd53bd040 | machreg_to_gpr(tmp.to_reg()));

                // add x0, x0, tmp
                Inst::AluRRR {
                    alu_op: ALUOp::Add,
                    size: OperandSize::Size64,
                    rd,
                    rn: rd.to_reg(),
                    rm: tmp.to_reg(),
                }
                .emit(sink, emit_info, state);
            }

            &Inst::MachOTlsGetAddr { ref symbol, rd } => {
                // Each thread local variable gets a descriptor, where the first xword of the descriptor is a pointer
                // to a function that takes the descriptor address in x0, and after the function returns x0
                // contains the address for the thread local variable
                //
                // what we want to emit is basically:
                //
                // adrp x0, <label>@TLVPPAGE  ; Load the address of the page of the thread local variable pointer (TLVP)
                // ldr x0, [x0, <label>@TLVPPAGEOFF] ; Load the descriptor's address into x0
                // ldr x1, [x0] ; Load the function pointer (the first part of the descriptor)
                // blr x1 ; Call the function pointer with the descriptor address in x0
                // ; x0 now contains the TLV address

                assert_eq!(xreg(0), rd.to_reg());
                let rtmp = writable_xreg(1);

                // adrp x0, <label>@TLVPPAGE
                sink.add_reloc(Reloc::MachOAarch64TlsAdrPage21, symbol, 0);
                sink.put4(0x90000000);

                // ldr x0, [x0, <label>@TLVPPAGEOFF]
                sink.add_reloc(Reloc::MachOAarch64TlsAdrPageOff12, symbol, 0);
                sink.put4(0xf9400000);

                // load [x0] into temp register
                Inst::ULoad64 {
                    rd: rtmp,
                    mem: AMode::reg(rd.to_reg()),
                    flags: MemFlags::trusted(),
                }
                .emit(sink, emit_info, state);

                // call function pointer in temp register
                Inst::CallInd {
                    info: crate::isa::Box::new(CallInfo::empty(
                        rtmp.to_reg(),
                        CallConv::AppleAarch64,
                    )),
                }
                .emit(sink, emit_info, state);
            }

            &Inst::Unwind { ref inst } => {
                sink.add_unwind(inst.clone());
            }

            &Inst::DummyUse { .. } => {}

            &Inst::StackProbeLoop { start, end, step } => {
                assert!(emit_info.0.enable_probestack());

                // The loop generated here uses `start` as a counter register to
                // count backwards until negating it exceeds `end`. In other
                // words `start` is an offset from `sp` we're testing where
                // `end` is the max size we need to test. The loop looks like:
                //
                //      loop_start:
                //          sub start, start, #step
                //          stur xzr, [sp, start]
                //          cmn start, end
                //          br.gt loop_start
                //      loop_end:
                //
                // Note that this loop cannot use the spilltmp and tmp2
                // registers as those are currently used as the input to this
                // loop when generating the instruction. This means that some
                // more flavorful address modes and lowerings need to be
                // avoided.
                //
                // Perhaps someone more clever than I can figure out how to use
                // `subs` or the like and skip the `cmn`, but I can't figure it
                // out at this time.

                let loop_start = sink.get_label();
                sink.bind_label(loop_start, &mut state.ctrl_plane);

                Inst::AluRRImm12 {
                    alu_op: ALUOp::Sub,
                    size: OperandSize::Size64,
                    rd: start,
                    rn: start.to_reg(),
                    imm12: step,
                }
                .emit(sink, emit_info, state);
                Inst::Store32 {
                    rd: regs::zero_reg(),
                    mem: AMode::RegReg {
                        rn: regs::stack_reg(),
                        rm: start.to_reg(),
                    },
                    flags: MemFlags::trusted(),
                }
                .emit(sink, emit_info, state);
                Inst::AluRRR {
                    alu_op: ALUOp::AddS,
                    size: OperandSize::Size64,
                    rd: regs::writable_zero_reg(),
                    rn: start.to_reg(),
                    rm: end,
                }
                .emit(sink, emit_info, state);

                let loop_end = sink.get_label();
                Inst::CondBr {
                    taken: BranchTarget::Label(loop_start),
                    not_taken: BranchTarget::Label(loop_end),
                    kind: CondBrKind::Cond(Cond::Gt),
                }
                .emit(sink, emit_info, state);
                sink.bind_label(loop_end, &mut state.ctrl_plane);
            }
        }

        let end_off = sink.cur_offset();
        debug_assert!(
            (end_off - start_off) <= Inst::worst_case_size()
                || matches!(self, Inst::EmitIsland { .. }),
            "Worst case size exceed for {:?}: {}",
            self,
            end_off - start_off
        );

        state.clear_post_insn();
    }

    fn pretty_print_inst(&self, state: &mut Self::State) -> String {
        self.print_with_state(state)
    }
}

fn emit_return_call_common_sequence<T>(
    sink: &mut MachBuffer<Inst>,
    emit_info: &EmitInfo,
    state: &mut EmitState,
    info: &ReturnCallInfo<T>,
) {
    for inst in
        AArch64MachineDeps::gen_clobber_restore(CallConv::Tail, &emit_info.0, state.frame_layout())
    {
        inst.emit(sink, emit_info, state);
    }

    let setup_area_size = state.frame_layout().setup_area_size;
    if setup_area_size > 0 {
        // N.B.: sp is already adjusted to the appropriate place by the
        // clobber-restore code (which also frees the fixed frame). Hence, there
        // is no need for the usual `mov sp, fp` here.

        // `ldp fp, lr, [sp], #16`
        Inst::LoadP64 {
            rt: writable_fp_reg(),
            rt2: writable_link_reg(),
            mem: PairAMode::SPPostIndexed {
                // TODO: we could fold the increment for incoming_args_diff here, as long as that
                // value is less than 502*8, by adding it to `setup_area_size`.
                // https://developer.arm.com/documentation/ddi0596/2020-12/Base-Instructions/LDP--Load-Pair-of-Registers-
                simm7: SImm7Scaled::maybe_from_i64(i64::from(setup_area_size), types::I64).unwrap(),
            },
            flags: MemFlags::trusted(),
        }
        .emit(sink, emit_info, state);
    }

    // Adjust SP to account for the possible over-allocation in the prologue.
    let incoming_args_diff = state.frame_layout().tail_args_size - info.new_stack_arg_size;
    if incoming_args_diff > 0 {
        for inst in
            AArch64MachineDeps::gen_sp_reg_adjust(i32::try_from(incoming_args_diff).unwrap())
        {
            inst.emit(sink, emit_info, state);
        }
    }

    if let Some(key) = info.key {
        sink.put4(key.enc_auti_hint());
    }
}

//! S390x ISA: binary code emission.

use crate::ir::{self, LibCall, MemFlags, TrapCode};
use crate::isa::CallConv;
use crate::isa::s390x::abi::REG_SAVE_AREA_SIZE;
use crate::isa::s390x::inst::*;
use crate::isa::s390x::settings as s390x_settings;
use cranelift_control::ControlPlane;

/// Debug macro for testing that a regpair is valid: that the high register is even, and the low
/// register is one higher than the high register.
macro_rules! debug_assert_valid_regpair {
    ($hi:expr, $lo:expr) => {
        if cfg!(debug_assertions) {
            match ($hi.to_real_reg(), $lo.to_real_reg()) {
                (Some(hi), Some(lo)) => {
                    assert!(
                        hi.hw_enc() % 2 == 0,
                        "High register is not even: {}",
                        show_reg($hi)
                    );
                    assert_eq!(
                        hi.hw_enc() + 1,
                        lo.hw_enc(),
                        "Low register is not valid: {}, {}",
                        show_reg($hi),
                        show_reg($lo)
                    );
                }

                _ => {
                    panic!(
                        "Expected real registers for {} {}",
                        show_reg($hi),
                        show_reg($lo)
                    );
                }
            }
        }
    };
}

macro_rules! debug_assert_valid_fp_regpair {
    ($hi:expr, $lo:expr) => {
        if cfg!(debug_assertions) {
            match ($hi.to_real_reg(), $lo.to_real_reg()) {
                (Some(hi), Some(lo)) => {
                    assert!(
                        hi.hw_enc() & 2 == 0,
                        "High register is not valid: {}",
                        show_reg($hi)
                    );
                    assert_eq!(
                        hi.hw_enc() + 2,
                        lo.hw_enc(),
                        "Low register is not valid: {}, {}",
                        show_reg($hi),
                        show_reg($lo)
                    );
                }

                _ => {
                    panic!(
                        "Expected real registers for {} {}",
                        show_reg($hi),
                        show_reg($lo)
                    );
                }
            }
        }
    };
}

const OPCODE_BRAS: u16 = 0xa75;
const OPCODE_BCR: u16 = 0xa74;
const OPCODE_LDR: u16 = 0x28;
const OPCODE_VLR: u16 = 0xe756;

/// Type(s) of memory instructions available for mem_finalize.
pub struct MemInstType {
    /// True if 12-bit unsigned displacement is supported.
    pub have_d12: bool,
    /// True if 20-bit signed displacement is supported.
    pub have_d20: bool,
    /// True if PC-relative addressing is supported (memory access).
    pub have_pcrel: bool,
    /// True if PC-relative addressing is supported (load address).
    pub have_unaligned_pcrel: bool,
    /// True if an index register is supported.
    pub have_index: bool,
}

/// Memory addressing mode finalization: convert "special" modes (e.g.,
/// generic arbitrary stack offset) into real addressing modes, possibly by
/// emitting some helper instructions that come immediately before the use
/// of this amode.
pub fn mem_finalize(
    mem: &MemArg,
    state: &EmitState,
    mi: MemInstType,
) -> (SmallVec<[Inst; 4]>, MemArg) {
    let mut insts = SmallVec::new();

    // Resolve virtual addressing modes.
    let mem = match mem {
        &MemArg::RegOffset { off, .. }
        | &MemArg::InitialSPOffset { off }
        | &MemArg::IncomingArgOffset { off }
        | &MemArg::OutgoingArgOffset { off }
        | &MemArg::SlotOffset { off }
        | &MemArg::SpillOffset { off } => {
            let base = match mem {
                &MemArg::RegOffset { reg, .. } => reg,
                &MemArg::InitialSPOffset { .. }
                | &MemArg::IncomingArgOffset { .. }
                | &MemArg::OutgoingArgOffset { .. }
                | &MemArg::SlotOffset { .. }
                | &MemArg::SpillOffset { .. } => stack_reg(),
                _ => unreachable!(),
            };
            let adj = match mem {
                &MemArg::IncomingArgOffset { .. } => i64::from(
                    state.incoming_args_size
                        + REG_SAVE_AREA_SIZE
                        + state.frame_layout().clobber_size
                        + state.frame_layout().fixed_frame_storage_size
                        + state.frame_layout().outgoing_args_size
                        + state.nominal_sp_offset,
                ),
                &MemArg::InitialSPOffset { .. } => i64::from(
                    state.frame_layout().clobber_size
                        + state.frame_layout().fixed_frame_storage_size
                        + state.frame_layout().outgoing_args_size
                        + state.nominal_sp_offset,
                ),
                &MemArg::SpillOffset { .. } => i64::from(
                    state.frame_layout().stackslots_size
                        + state.frame_layout().outgoing_args_size
                        + state.nominal_sp_offset,
                ),
                &MemArg::SlotOffset { .. } => {
                    i64::from(state.frame_layout().outgoing_args_size + state.nominal_sp_offset)
                }
                &MemArg::OutgoingArgOffset { .. } => {
                    i64::from(REG_SAVE_AREA_SIZE) - i64::from(state.outgoing_sp_offset)
                }
                _ => 0,
            };
            let off = off + adj;

            if let Some(disp) = UImm12::maybe_from_u64(off as u64) {
                MemArg::BXD12 {
                    base,
                    index: zero_reg(),
                    disp,
                    flags: mem.get_flags(),
                }
            } else if let Some(disp) = SImm20::maybe_from_i64(off) {
                MemArg::BXD20 {
                    base,
                    index: zero_reg(),
                    disp,
                    flags: mem.get_flags(),
                }
            } else {
                let tmp = writable_spilltmp_reg();
                assert!(base != tmp.to_reg());
                if let Ok(imm) = i16::try_from(off) {
                    insts.push(Inst::Mov64SImm16 { rd: tmp, imm });
                } else if let Ok(imm) = i32::try_from(off) {
                    insts.push(Inst::Mov64SImm32 { rd: tmp, imm });
                } else {
                    // The offset must be smaller than the stack frame size,
                    // which the ABI code limits to 128 MB.
                    unreachable!();
                }
                MemArg::reg_plus_reg(base, tmp.to_reg(), mem.get_flags())
            }
        }
        _ => mem.clone(),
    };

    // If this addressing mode cannot be handled by the instruction, use load-address.
    let need_load_address = match &mem {
        &MemArg::Label { .. } | &MemArg::Constant { .. } if !mi.have_pcrel => true,
        &MemArg::Symbol { .. } if !mi.have_pcrel => true,
        &MemArg::Symbol { flags, .. } if !mi.have_unaligned_pcrel && !flags.aligned() => true,
        &MemArg::BXD20 { .. } if !mi.have_d20 => true,
        &MemArg::BXD12 { index, .. } | &MemArg::BXD20 { index, .. } if !mi.have_index => {
            index != zero_reg()
        }
        _ => false,
    };
    let mem = if need_load_address {
        let flags = mem.get_flags();
        let tmp = writable_spilltmp_reg();
        insts.push(Inst::LoadAddr { rd: tmp, mem });
        MemArg::reg(tmp.to_reg(), flags)
    } else {
        mem
    };

    // Convert 12-bit displacement to 20-bit if required.
    let mem = match &mem {
        &MemArg::BXD12 {
            base,
            index,
            disp,
            flags,
        } if !mi.have_d12 => {
            assert!(mi.have_d20);
            MemArg::BXD20 {
                base,
                index,
                disp: SImm20::from_uimm12(disp),
                flags,
            }
        }
        _ => mem,
    };

    (insts, mem)
}

pub fn mem_emit(
    rd: Reg,
    mem: &MemArg,
    opcode_rx: Option<u16>,
    opcode_rxy: Option<u16>,
    opcode_ril: Option<u16>,
    add_trap: bool,
    sink: &mut MachBuffer<Inst>,
    emit_info: &EmitInfo,
    state: &mut EmitState,
) {
    let (mem_insts, mem) = mem_finalize(
        mem,
        state,
        MemInstType {
            have_d12: opcode_rx.is_some(),
            have_d20: opcode_rxy.is_some(),
            have_pcrel: opcode_ril.is_some(),
            have_unaligned_pcrel: opcode_ril.is_some() && !add_trap,
            have_index: true,
        },
    );
    for inst in mem_insts.into_iter() {
        inst.emit(sink, emit_info, state);
    }

    if add_trap {
        if let Some(trap_code) = mem.get_flags().trap_code() {
            sink.add_trap(trap_code);
        }
    }

    match &mem {
        &MemArg::BXD12 {
            base, index, disp, ..
        } => {
            put(
                sink,
                &enc_rx(opcode_rx.unwrap(), rd, base, index, disp.bits()),
            );
        }
        &MemArg::BXD20 {
            base, index, disp, ..
        } => {
            put(
                sink,
                &enc_rxy(opcode_rxy.unwrap(), rd, base, index, disp.bits()),
            );
        }
        &MemArg::Label { target } => {
            sink.use_label_at_offset(sink.cur_offset(), target, LabelUse::BranchRIL);
            put(sink, &enc_ril_b(opcode_ril.unwrap(), rd, 0));
        }
        &MemArg::Constant { constant } => {
            let target = sink.get_label_for_constant(constant);
            sink.use_label_at_offset(sink.cur_offset(), target, LabelUse::BranchRIL);
            put(sink, &enc_ril_b(opcode_ril.unwrap(), rd, 0));
        }
        &MemArg::Symbol {
            ref name, offset, ..
        } => {
            let reloc_offset = sink.cur_offset() + 2;
            sink.add_reloc_at_offset(
                reloc_offset,
                Reloc::S390xPCRel32Dbl,
                &**name,
                (offset + 2).into(),
            );
            put(sink, &enc_ril_b(opcode_ril.unwrap(), rd, 0));
        }
        _ => unreachable!(),
    }
}

pub fn mem_rs_emit(
    rd: Reg,
    rn: Reg,
    mem: &MemArg,
    opcode_rs: Option<u16>,
    opcode_rsy: Option<u16>,
    add_trap: bool,
    sink: &mut MachBuffer<Inst>,
    emit_info: &EmitInfo,
    state: &mut EmitState,
) {
    let (mem_insts, mem) = mem_finalize(
        mem,
        state,
        MemInstType {
            have_d12: opcode_rs.is_some(),
            have_d20: opcode_rsy.is_some(),
            have_pcrel: false,
            have_unaligned_pcrel: false,
            have_index: false,
        },
    );
    for inst in mem_insts.into_iter() {
        inst.emit(sink, emit_info, state);
    }

    if add_trap {
        if let Some(trap_code) = mem.get_flags().trap_code() {
            sink.add_trap(trap_code);
        }
    }

    match &mem {
        &MemArg::BXD12 {
            base, index, disp, ..
        } => {
            assert!(index == zero_reg());
            put(sink, &enc_rs(opcode_rs.unwrap(), rd, rn, base, disp.bits()));
        }
        &MemArg::BXD20 {
            base, index, disp, ..
        } => {
            assert!(index == zero_reg());
            put(
                sink,
                &enc_rsy(opcode_rsy.unwrap(), rd, rn, base, disp.bits()),
            );
        }
        _ => unreachable!(),
    }
}

pub fn mem_imm8_emit(
    imm: u8,
    mem: &MemArg,
    opcode_si: u16,
    opcode_siy: u16,
    add_trap: bool,
    sink: &mut MachBuffer<Inst>,
    emit_info: &EmitInfo,
    state: &mut EmitState,
) {
    let (mem_insts, mem) = mem_finalize(
        mem,
        state,
        MemInstType {
            have_d12: true,
            have_d20: true,
            have_pcrel: false,
            have_unaligned_pcrel: false,
            have_index: false,
        },
    );
    for inst in mem_insts.into_iter() {
        inst.emit(sink, emit_info, state);
    }

    if add_trap {
        if let Some(trap_code) = mem.get_flags().trap_code() {
            sink.add_trap(trap_code);
        }
    }

    match &mem {
        &MemArg::BXD12 {
            base, index, disp, ..
        } => {
            assert!(index == zero_reg());
            put(sink, &enc_si(opcode_si, base, disp.bits(), imm));
        }
        &MemArg::BXD20 {
            base, index, disp, ..
        } => {
            assert!(index == zero_reg());
            put(sink, &enc_siy(opcode_siy, base, disp.bits(), imm));
        }
        _ => unreachable!(),
    }
}

pub fn mem_imm16_emit(
    imm: i16,
    mem: &MemArg,
    opcode_sil: u16,
    add_trap: bool,
    sink: &mut MachBuffer<Inst>,
    emit_info: &EmitInfo,
    state: &mut EmitState,
) {
    let (mem_insts, mem) = mem_finalize(
        mem,
        state,
        MemInstType {
            have_d12: true,
            have_d20: false,
            have_pcrel: false,
            have_unaligned_pcrel: false,
            have_index: false,
        },
    );
    for inst in mem_insts.into_iter() {
        inst.emit(sink, emit_info, state);
    }

    if add_trap {
        if let Some(trap_code) = mem.get_flags().trap_code() {
            sink.add_trap(trap_code);
        }
    }

    match &mem {
        &MemArg::BXD12 {
            base, index, disp, ..
        } => {
            assert!(index == zero_reg());
            put(sink, &enc_sil(opcode_sil, base, disp.bits(), imm));
        }
        _ => unreachable!(),
    }
}

pub fn mem_vrx_emit(
    rd: Reg,
    mem: &MemArg,
    opcode: u16,
    m3: u8,
    add_trap: bool,
    sink: &mut MachBuffer<Inst>,
    emit_info: &EmitInfo,
    state: &mut EmitState,
) {
    let (mem_insts, mem) = mem_finalize(
        mem,
        state,
        MemInstType {
            have_d12: true,
            have_d20: false,
            have_pcrel: false,
            have_unaligned_pcrel: false,
            have_index: true,
        },
    );
    for inst in mem_insts.into_iter() {
        inst.emit(sink, emit_info, state);
    }

    if add_trap {
        if let Some(trap_code) = mem.get_flags().trap_code() {
            sink.add_trap(trap_code);
        }
    }

    match &mem {
        &MemArg::BXD12 {
            base, index, disp, ..
        } => {
            put(sink, &enc_vrx(opcode, rd, base, index, disp.bits(), m3));
        }
        _ => unreachable!(),
    }
}

//=============================================================================
// Instructions and subcomponents: emission

fn machreg_to_gpr(m: Reg) -> u8 {
    assert_eq!(m.class(), RegClass::Int);
    m.to_real_reg().unwrap().hw_enc()
}

fn machreg_to_vr(m: Reg) -> u8 {
    assert_eq!(m.class(), RegClass::Float);
    m.to_real_reg().unwrap().hw_enc()
}

fn machreg_to_fpr(m: Reg) -> u8 {
    assert!(is_fpr(m));
    m.to_real_reg().unwrap().hw_enc()
}

fn machreg_to_gpr_or_fpr(m: Reg) -> u8 {
    let reg = m.to_real_reg().unwrap().hw_enc();
    assert!(reg < 16);
    reg
}

fn rxb(v1: Option<Reg>, v2: Option<Reg>, v3: Option<Reg>, v4: Option<Reg>) -> u8 {
    let mut rxb = 0;

    let is_high_vr = |reg| -> bool {
        if let Some(reg) = reg {
            if !is_fpr(reg) {
                return true;
            }
        }
        false
    };

    if is_high_vr(v1) {
        rxb = rxb | 8;
    }
    if is_high_vr(v2) {
        rxb = rxb | 4;
    }
    if is_high_vr(v3) {
        rxb = rxb | 2;
    }
    if is_high_vr(v4) {
        rxb = rxb | 1;
    }

    rxb
}

/// E-type instructions.
///
///   15
///   opcode
///        0
///
fn enc_e(opcode: u16) -> [u8; 2] {
    let mut enc: [u8; 2] = [0; 2];
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;

    enc[0] = opcode1;
    enc[1] = opcode2;
    enc
}

/// RIa-type instructions.
///
///   31      23 19      15
///   opcode1 r1 opcode2 i2
///        24 20      16  0
///
fn enc_ri_a(opcode: u16, r1: Reg, i2: u16) -> [u8; 4] {
    let mut enc: [u8; 4] = [0; 4];
    let opcode1 = ((opcode >> 4) & 0xff) as u8;
    let opcode2 = (opcode & 0xf) as u8;
    let r1 = machreg_to_gpr(r1) & 0x0f;

    enc[0] = opcode1;
    enc[1] = r1 << 4 | opcode2;
    enc[2..].copy_from_slice(&i2.to_be_bytes());
    enc
}

/// RIb-type instructions.
///
///   31      23 19      15
///   opcode1 r1 opcode2 ri2
///        24 20      16   0
///
fn enc_ri_b(opcode: u16, r1: Reg, ri2: i32) -> [u8; 4] {
    let mut enc: [u8; 4] = [0; 4];
    let opcode1 = ((opcode >> 4) & 0xff) as u8;
    let opcode2 = (opcode & 0xf) as u8;
    let r1 = machreg_to_gpr(r1) & 0x0f;
    let ri2 = ((ri2 >> 1) & 0xffff) as u16;

    enc[0] = opcode1;
    enc[1] = r1 << 4 | opcode2;
    enc[2..].copy_from_slice(&ri2.to_be_bytes());
    enc
}

/// RIc-type instructions.
///
///   31      23 19      15
///   opcode1 m1 opcode2 ri2
///        24 20      16   0
///
fn enc_ri_c(opcode: u16, m1: u8, ri2: i32) -> [u8; 4] {
    let mut enc: [u8; 4] = [0; 4];
    let opcode1 = ((opcode >> 4) & 0xff) as u8;
    let opcode2 = (opcode & 0xf) as u8;
    let m1 = m1 & 0x0f;
    let ri2 = ((ri2 >> 1) & 0xffff) as u16;

    enc[0] = opcode1;
    enc[1] = m1 << 4 | opcode2;
    enc[2..].copy_from_slice(&ri2.to_be_bytes());
    enc
}

/// RIEa-type instructions.
///
///   47      39 35 31 15 11 7
///   opcode1 r1 -- i2 m3 -- opcode2
///        40 36 32 16 12 8       0
///
fn enc_rie_a(opcode: u16, r1: Reg, i2: u16, m3: u8) -> [u8; 6] {
    let mut enc: [u8; 6] = [0; 6];
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr(r1) & 0x0f;
    let m3 = m3 & 0x0f;

    enc[0] = opcode1;
    enc[1] = r1 << 4;
    enc[2..4].copy_from_slice(&i2.to_be_bytes());
    enc[4] = m3 << 4;
    enc[5] = opcode2;
    enc
}

/// RIEd-type instructions.
///
///   47      39 35 31 15 7
///   opcode1 r1 r3 i2 -- opcode2
///        40 36 32 16  8       0
///
fn enc_rie_d(opcode: u16, r1: Reg, r3: Reg, i2: u16) -> [u8; 6] {
    let mut enc: [u8; 6] = [0; 6];
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr(r1) & 0x0f;
    let r3 = machreg_to_gpr(r3) & 0x0f;

    enc[0] = opcode1;
    enc[1] = r1 << 4 | r3;
    enc[2..4].copy_from_slice(&i2.to_be_bytes());
    enc[5] = opcode2;
    enc
}

/// RIEf-type instructions.
///
///   47      39 35 31 23 15 7
///   opcode1 r1 r2 i3 i4 i5 opcode2
///        40 36 32 24 16  8       0
///
fn enc_rie_f(opcode: u16, r1: Reg, r2: Reg, i3: u8, i4: u8, i5: u8) -> [u8; 6] {
    let mut enc: [u8; 6] = [0; 6];
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr(r1) & 0x0f;
    let r2 = machreg_to_gpr(r2) & 0x0f;

    enc[0] = opcode1;
    enc[1] = r1 << 4 | r2;
    enc[2] = i3;
    enc[3] = i4;
    enc[4] = i5;
    enc[5] = opcode2;
    enc
}

/// RIEg-type instructions.
///
///   47      39 35 31 15 7
///   opcode1 r1 m3 i2 -- opcode2
///        40 36 32 16  8       0
///
fn enc_rie_g(opcode: u16, r1: Reg, i2: u16, m3: u8) -> [u8; 6] {
    let mut enc: [u8; 6] = [0; 6];
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr(r1) & 0x0f;
    let m3 = m3 & 0x0f;

    enc[0] = opcode1;
    enc[1] = r1 << 4 | m3;
    enc[2..4].copy_from_slice(&i2.to_be_bytes());
    enc[5] = opcode2;
    enc
}

/// RILa-type instructions.
///
///   47      39 35      31
///   opcode1 r1 opcode2 i2
///        40 36      32  0
///
fn enc_ril_a(opcode: u16, r1: Reg, i2: u32) -> [u8; 6] {
    let mut enc: [u8; 6] = [0; 6];
    let opcode1 = ((opcode >> 4) & 0xff) as u8;
    let opcode2 = (opcode & 0xf) as u8;
    let r1 = machreg_to_gpr(r1) & 0x0f;

    enc[0] = opcode1;
    enc[1] = r1 << 4 | opcode2;
    enc[2..].copy_from_slice(&i2.to_be_bytes());
    enc
}

/// RILb-type instructions.
///
///   47      39 35      31
///   opcode1 r1 opcode2 ri2
///        40 36      32   0
///
fn enc_ril_b(opcode: u16, r1: Reg, ri2: u32) -> [u8; 6] {
    let mut enc: [u8; 6] = [0; 6];
    let opcode1 = ((opcode >> 4) & 0xff) as u8;
    let opcode2 = (opcode & 0xf) as u8;
    let r1 = machreg_to_gpr(r1) & 0x0f;
    let ri2 = ri2 >> 1;

    enc[0] = opcode1;
    enc[1] = r1 << 4 | opcode2;
    enc[2..].copy_from_slice(&ri2.to_be_bytes());
    enc
}

/// RILc-type instructions.
///
///   47      39 35      31
///   opcode1 m1 opcode2 i2
///        40 36      32  0
///
fn enc_ril_c(opcode: u16, m1: u8, ri2: u32) -> [u8; 6] {
    let mut enc: [u8; 6] = [0; 6];
    let opcode1 = ((opcode >> 4) & 0xff) as u8;
    let opcode2 = (opcode & 0xf) as u8;
    let m1 = m1 & 0x0f;
    let ri2 = ri2 >> 1;

    enc[0] = opcode1;
    enc[1] = m1 << 4 | opcode2;
    enc[2..].copy_from_slice(&ri2.to_be_bytes());
    enc
}

/// RR-type instructions.
///
///   15     7  3
///   opcode r1 r2
///        8  4  0
///
fn enc_rr(opcode: u16, r1: Reg, r2: Reg) -> [u8; 2] {
    let mut enc: [u8; 2] = [0; 2];
    let opcode = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr_or_fpr(r1) & 0x0f;
    let r2 = machreg_to_gpr_or_fpr(r2) & 0x0f;

    enc[0] = opcode;
    enc[1] = r1 << 4 | r2;
    enc
}

/// RRD-type instructions.
///
///   31     15 11 7  3
///   opcode r1 -- r3 r2
///       16 12  8 4  0
///
fn enc_rrd(opcode: u16, r1: Reg, r2: Reg, r3: Reg) -> [u8; 4] {
    let mut enc: [u8; 4] = [0; 4];
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let r1 = machreg_to_fpr(r1) & 0x0f;
    let r2 = machreg_to_fpr(r2) & 0x0f;
    let r3 = machreg_to_fpr(r3) & 0x0f;

    enc[0] = opcode1;
    enc[1] = opcode2;
    enc[2] = r1 << 4;
    enc[3] = r3 << 4 | r2;
    enc
}

/// RRE-type instructions.
///
///   31     15 7  3
///   opcode -- r1 r2
///       16  8  4  0
///
fn enc_rre(opcode: u16, r1: Reg, r2: Reg) -> [u8; 4] {
    let mut enc: [u8; 4] = [0; 4];
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr_or_fpr(r1) & 0x0f;
    let r2 = machreg_to_gpr_or_fpr(r2) & 0x0f;

    enc[0] = opcode1;
    enc[1] = opcode2;
    enc[3] = r1 << 4 | r2;
    enc
}

/// RRFa/b-type instructions.
///
///   31     15 11 7  3
///   opcode r3 m4 r1 r2
///       16 12  8  4  0
///
fn enc_rrf_ab(opcode: u16, r1: Reg, r2: Reg, r3: Reg, m4: u8) -> [u8; 4] {
    let mut enc: [u8; 4] = [0; 4];
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr_or_fpr(r1) & 0x0f;
    let r2 = machreg_to_gpr_or_fpr(r2) & 0x0f;
    let r3 = machreg_to_gpr_or_fpr(r3) & 0x0f;
    let m4 = m4 & 0x0f;

    enc[0] = opcode1;
    enc[1] = opcode2;
    enc[2] = r3 << 4 | m4;
    enc[3] = r1 << 4 | r2;
    enc
}

/// RRFc/d/e-type instructions.
///
///   31     15 11 7  3
///   opcode m3 m4 r1 r2
///       16 12  8  4  0
///
fn enc_rrf_cde(opcode: u16, r1: Reg, r2: Reg, m3: u8, m4: u8) -> [u8; 4] {
    let mut enc: [u8; 4] = [0; 4];
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr_or_fpr(r1) & 0x0f;
    let r2 = machreg_to_gpr_or_fpr(r2) & 0x0f;
    let m3 = m3 & 0x0f;
    let m4 = m4 & 0x0f;

    enc[0] = opcode1;
    enc[1] = opcode2;
    enc[2] = m3 << 4 | m4;
    enc[3] = r1 << 4 | r2;
    enc
}

/// RS-type instructions.
///
///   31     23 19 15 11
///   opcode r1 r3 b2 d2
///       24 20 16 12  0
///
fn enc_rs(opcode: u16, r1: Reg, r3: Reg, b2: Reg, d2: u32) -> [u8; 4] {
    let opcode = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr_or_fpr(r1) & 0x0f;
    let r3 = machreg_to_gpr_or_fpr(r3) & 0x0f;
    let b2 = machreg_to_gpr(b2) & 0x0f;
    let d2_lo = (d2 & 0xff) as u8;
    let d2_hi = ((d2 >> 8) & 0x0f) as u8;

    let mut enc: [u8; 4] = [0; 4];
    enc[0] = opcode;
    enc[1] = r1 << 4 | r3;
    enc[2] = b2 << 4 | d2_hi;
    enc[3] = d2_lo;
    enc
}

/// RSY-type instructions.
///
///   47      39 35 31 27  15  7
///   opcode1 r1 r3 b2 dl2 dh2 opcode2
///        40 36 32 28  16   8       0
///
fn enc_rsy(opcode: u16, r1: Reg, r3: Reg, b2: Reg, d2: u32) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr_or_fpr(r1) & 0x0f;
    let r3 = machreg_to_gpr_or_fpr(r3) & 0x0f;
    let b2 = machreg_to_gpr(b2) & 0x0f;
    let dl2_lo = (d2 & 0xff) as u8;
    let dl2_hi = ((d2 >> 8) & 0x0f) as u8;
    let dh2 = ((d2 >> 12) & 0xff) as u8;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = r1 << 4 | r3;
    enc[2] = b2 << 4 | dl2_hi;
    enc[3] = dl2_lo;
    enc[4] = dh2;
    enc[5] = opcode2;
    enc
}

/// RX-type instructions.
///
///   31     23 19 15 11
///   opcode r1 x2 b2 d2
///       24 20 16 12  0
///
fn enc_rx(opcode: u16, r1: Reg, b2: Reg, x2: Reg, d2: u32) -> [u8; 4] {
    let opcode = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr_or_fpr(r1) & 0x0f;
    let b2 = machreg_to_gpr(b2) & 0x0f;
    let x2 = machreg_to_gpr(x2) & 0x0f;
    let d2_lo = (d2 & 0xff) as u8;
    let d2_hi = ((d2 >> 8) & 0x0f) as u8;

    let mut enc: [u8; 4] = [0; 4];
    enc[0] = opcode;
    enc[1] = r1 << 4 | x2;
    enc[2] = b2 << 4 | d2_hi;
    enc[3] = d2_lo;
    enc
}

/// RXY-type instructions.
///
///   47      39 35 31 27  15  7
///   opcode1 r1 x2 b2 dl2 dh2 opcode2
///        40 36 32 28  16   8       0
///
fn enc_rxy(opcode: u16, r1: Reg, b2: Reg, x2: Reg, d2: u32) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let r1 = machreg_to_gpr_or_fpr(r1) & 0x0f;
    let b2 = machreg_to_gpr(b2) & 0x0f;
    let x2 = machreg_to_gpr(x2) & 0x0f;
    let dl2_lo = (d2 & 0xff) as u8;
    let dl2_hi = ((d2 >> 8) & 0x0f) as u8;
    let dh2 = ((d2 >> 12) & 0xff) as u8;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = r1 << 4 | x2;
    enc[2] = b2 << 4 | dl2_hi;
    enc[3] = dl2_lo;
    enc[4] = dh2;
    enc[5] = opcode2;
    enc
}

/// SI-type instructions.
///
///   31     23 15 11
///   opcode i2 b1 d1
///       24 16 12  0
///
fn enc_si(opcode: u16, b1: Reg, d1: u32, i2: u8) -> [u8; 4] {
    let opcode = (opcode & 0xff) as u8;
    let b1 = machreg_to_gpr(b1) & 0x0f;
    let d1_lo = (d1 & 0xff) as u8;
    let d1_hi = ((d1 >> 8) & 0x0f) as u8;

    let mut enc: [u8; 4] = [0; 4];
    enc[0] = opcode;
    enc[1] = i2;
    enc[2] = b1 << 4 | d1_hi;
    enc[3] = d1_lo;
    enc
}

/// SIL-type instructions.
///
///   47     31 27 15
///   opcode b1 d1 i2
///       32 28 16  0
///
fn enc_sil(opcode: u16, b1: Reg, d1: u32, i2: i16) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let b1 = machreg_to_gpr(b1) & 0x0f;
    let d1_lo = (d1 & 0xff) as u8;
    let d1_hi = ((d1 >> 8) & 0x0f) as u8;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = opcode2;
    enc[2] = b1 << 4 | d1_hi;
    enc[3] = d1_lo;
    enc[4..].copy_from_slice(&i2.to_be_bytes());
    enc
}

/// SIY-type instructions.
///
///   47      39 31 27  15  7
///   opcode1 i2 b1 dl1 dh1 opcode2
///        40 32 28  16   8       0
///
fn enc_siy(opcode: u16, b1: Reg, d1: u32, i2: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let b1 = machreg_to_gpr(b1) & 0x0f;
    let dl1_lo = (d1 & 0xff) as u8;
    let dl1_hi = ((d1 >> 8) & 0x0f) as u8;
    let dh1 = ((d1 >> 12) & 0xff) as u8;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = i2;
    enc[2] = b1 << 4 | dl1_hi;
    enc[3] = dl1_lo;
    enc[4] = dh1;
    enc[5] = opcode2;
    enc
}

/// VRIa-type instructions.
///
///   47      39 35 31 15 11  7
///   opcode1 v1 -  i2 m3 rxb opcode2
///        40 36 32 16 12   8       0
///
fn enc_vri_a(opcode: u16, v1: Reg, i2: u16, m3: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(Some(v1), None, None, None);
    let v1 = machreg_to_vr(v1) & 0x0f;
    let m3 = m3 & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = v1 << 4;
    enc[2..4].copy_from_slice(&i2.to_be_bytes());
    enc[4] = m3 << 4 | rxb;
    enc[5] = opcode2;
    enc
}

/// VRIb-type instructions.
///
///   47      39 35 31 23 15 11  7
///   opcode1 v1 -  i2 i3 m4 rxb opcode2
///        40 36 32 24 16 12   8       0
///
fn enc_vri_b(opcode: u16, v1: Reg, i2: u8, i3: u8, m4: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(Some(v1), None, None, None);
    let v1 = machreg_to_vr(v1) & 0x0f;
    let m4 = m4 & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = v1 << 4;
    enc[2] = i2;
    enc[3] = i3;
    enc[4] = m4 << 4 | rxb;
    enc[5] = opcode2;
    enc
}

/// VRIc-type instructions.
///
///   47      39 35 31 15 11  7
///   opcode1 v1 v3 i2 m4 rxb opcode2
///        40 36 32 16 12   8       0
///
fn enc_vri_c(opcode: u16, v1: Reg, i2: u16, v3: Reg, m4: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(Some(v1), Some(v3), None, None);
    let v1 = machreg_to_vr(v1) & 0x0f;
    let v3 = machreg_to_vr(v3) & 0x0f;
    let m4 = m4 & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = v1 << 4 | v3;
    enc[2..4].copy_from_slice(&i2.to_be_bytes());
    enc[4] = m4 << 4 | rxb;
    enc[5] = opcode2;
    enc
}

/// VRRa-type instructions.
///
///   47      39 35 31 23 19 15 11  7
///   opcode1 v1 v2 -  m5 m3 m2 rxb opcode2
///        40 36 32 24 20 16 12   8       0
///
fn enc_vrr_a(opcode: u16, v1: Reg, v2: Reg, m3: u8, m4: u8, m5: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(Some(v1), Some(v2), None, None);
    let v1 = machreg_to_vr(v1) & 0x0f;
    let v2 = machreg_to_vr(v2) & 0x0f;
    let m3 = m3 & 0x0f;
    let m4 = m4 & 0x0f;
    let m5 = m5 & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = v1 << 4 | v2;
    enc[2] = 0;
    enc[3] = m5 << 4 | m4;
    enc[4] = m3 << 4 | rxb;
    enc[5] = opcode2;
    enc
}

/// VRRb-type instructions.
///
///   47      39 35 31 27 23 19 15 11  7
///   opcode1 v1 v2 v3 -  m5 -  m4 rxb opcode2
///        40 36 32 28 24 20 16 12   8       0
///
fn enc_vrr_b(opcode: u16, v1: Reg, v2: Reg, v3: Reg, m4: u8, m5: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(Some(v1), Some(v2), Some(v3), None);
    let v1 = machreg_to_vr(v1) & 0x0f;
    let v2 = machreg_to_vr(v2) & 0x0f;
    let v3 = machreg_to_vr(v3) & 0x0f;
    let m4 = m4 & 0x0f;
    let m5 = m5 & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = v1 << 4 | v2;
    enc[2] = v3 << 4;
    enc[3] = m5 << 4;
    enc[4] = m4 << 4 | rxb;
    enc[5] = opcode2;
    enc
}

/// VRRc-type instructions.
///
///   47      39 35 31 27 23 19 15 11  7
///   opcode1 v1 v2 v3 -  m6 m5 m4 rxb opcode2
///        40 36 32 28 24 20 16 12   8       0
///
fn enc_vrr_c(opcode: u16, v1: Reg, v2: Reg, v3: Reg, m4: u8, m5: u8, m6: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(Some(v1), Some(v2), Some(v3), None);
    let v1 = machreg_to_vr(v1) & 0x0f;
    let v2 = machreg_to_vr(v2) & 0x0f;
    let v3 = machreg_to_vr(v3) & 0x0f;
    let m4 = m4 & 0x0f;
    let m5 = m5 & 0x0f;
    let m6 = m6 & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = v1 << 4 | v2;
    enc[2] = v3 << 4;
    enc[3] = m6 << 4 | m5;
    enc[4] = m4 << 4 | rxb;
    enc[5] = opcode2;
    enc
}

/// VRRe-type instructions.
///
///   47      39 35 31 27 23 19 15 11  7
///   opcode1 v1 v2 v3 m6 -  m5 v4 rxb opcode2
///        40 36 32 28 24 20 16 12   8       0
///
fn enc_vrr_e(opcode: u16, v1: Reg, v2: Reg, v3: Reg, v4: Reg, m5: u8, m6: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(Some(v1), Some(v2), Some(v3), Some(v4));
    let v1 = machreg_to_vr(v1) & 0x0f;
    let v2 = machreg_to_vr(v2) & 0x0f;
    let v3 = machreg_to_vr(v3) & 0x0f;
    let v4 = machreg_to_vr(v4) & 0x0f;
    let m5 = m5 & 0x0f;
    let m6 = m6 & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = v1 << 4 | v2;
    enc[2] = v3 << 4 | m6;
    enc[3] = m5;
    enc[4] = v4 << 4 | rxb;
    enc[5] = opcode2;
    enc
}

/// VRRf-type instructions.
///
///   47      39 35 31 27 11  7
///   opcode1 v1 r2 r3 -  rxb opcode2
///        40 36 32 28 12   8       0
///
fn enc_vrr_f(opcode: u16, v1: Reg, r2: Reg, r3: Reg) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(Some(v1), None, None, None);
    let v1 = machreg_to_vr(v1) & 0x0f;
    let r2 = machreg_to_gpr(r2) & 0x0f;
    let r3 = machreg_to_gpr(r3) & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = v1 << 4 | r2;
    enc[2] = r3 << 4;
    enc[4] = rxb;
    enc[5] = opcode2;
    enc
}

/// VRSa-type instructions.
///
///   47      39 35 31 27 15 11  7
///   opcode1 v1 v3 b2 d2 m4 rxb opcode2
///        40 36 32 28 16 12   8       0
///
fn enc_vrs_a(opcode: u16, v1: Reg, b2: Reg, d2: u32, v3: Reg, m4: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(Some(v1), Some(v3), None, None);
    let v1 = machreg_to_vr(v1) & 0x0f;
    let b2 = machreg_to_gpr(b2) & 0x0f;
    let v3 = machreg_to_vr(v3) & 0x0f;
    let d2_lo = (d2 & 0xff) as u8;
    let d2_hi = ((d2 >> 8) & 0x0f) as u8;
    let m4 = m4 & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = v1 << 4 | v3;
    enc[2] = b2 << 4 | d2_hi;
    enc[3] = d2_lo;
    enc[4] = m4 << 4 | rxb;
    enc[5] = opcode2;
    enc
}

/// VRSb-type instructions.
///
///   47      39 35 31 27 15 11  7
///   opcode1 v1 r3 b2 d2 m4 rxb opcode2
///        40 36 32 28 16 12   8       0
///
fn enc_vrs_b(opcode: u16, v1: Reg, b2: Reg, d2: u32, r3: Reg, m4: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(Some(v1), None, None, None);
    let v1 = machreg_to_vr(v1) & 0x0f;
    let b2 = machreg_to_gpr(b2) & 0x0f;
    let r3 = machreg_to_gpr(r3) & 0x0f;
    let d2_lo = (d2 & 0xff) as u8;
    let d2_hi = ((d2 >> 8) & 0x0f) as u8;
    let m4 = m4 & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = v1 << 4 | r3;
    enc[2] = b2 << 4 | d2_hi;
    enc[3] = d2_lo;
    enc[4] = m4 << 4 | rxb;
    enc[5] = opcode2;
    enc
}

/// VRSc-type instructions.
///
///   47      39 35 31 27 15 11  7
///   opcode1 r1 v3 b2 d2 m4 rxb opcode2
///        40 36 32 28 16 12   8       0
///
fn enc_vrs_c(opcode: u16, r1: Reg, b2: Reg, d2: u32, v3: Reg, m4: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(None, Some(v3), None, None);
    let r1 = machreg_to_gpr(r1) & 0x0f;
    let b2 = machreg_to_gpr(b2) & 0x0f;
    let v3 = machreg_to_vr(v3) & 0x0f;
    let d2_lo = (d2 & 0xff) as u8;
    let d2_hi = ((d2 >> 8) & 0x0f) as u8;
    let m4 = m4 & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = r1 << 4 | v3;
    enc[2] = b2 << 4 | d2_hi;
    enc[3] = d2_lo;
    enc[4] = m4 << 4 | rxb;
    enc[5] = opcode2;
    enc
}

/// VRX-type instructions.
///
///   47      39 35 31 27 15 11  7
///   opcode1 v1 x2 b2 d2 m3 rxb opcode2
///        40 36 32 28 16 12   8       0
///
fn enc_vrx(opcode: u16, v1: Reg, b2: Reg, x2: Reg, d2: u32, m3: u8) -> [u8; 6] {
    let opcode1 = ((opcode >> 8) & 0xff) as u8;
    let opcode2 = (opcode & 0xff) as u8;
    let rxb = rxb(Some(v1), None, None, None);
    let v1 = machreg_to_vr(v1) & 0x0f;
    let b2 = machreg_to_gpr(b2) & 0x0f;
    let x2 = machreg_to_gpr(x2) & 0x0f;
    let d2_lo = (d2 & 0xff) as u8;
    let d2_hi = ((d2 >> 8) & 0x0f) as u8;
    let m3 = m3 & 0x0f;

    let mut enc: [u8; 6] = [0; 6];
    enc[0] = opcode1;
    enc[1] = v1 << 4 | x2;
    enc[2] = b2 << 4 | d2_hi;
    enc[3] = d2_lo;
    enc[4] = m3 << 4 | rxb;
    enc[5] = opcode2;
    enc
}

/// Emit encoding to sink.
fn put(sink: &mut MachBuffer<Inst>, enc: &[u8]) {
    for byte in enc {
        sink.put1(*byte);
    }
}

/// Emit encoding to sink, adding a trap on the last byte.
fn put_with_trap(sink: &mut MachBuffer<Inst>, enc: &[u8], trap_code: TrapCode) {
    let len = enc.len();
    for i in 0..len - 1 {
        sink.put1(enc[i]);
    }
    sink.add_trap(trap_code);
    sink.put1(enc[len - 1]);
}

/// State carried between emissions of a sequence of instructions.
#[derive(Default, Clone, Debug)]
pub struct EmitState {
    /// Offset from the actual SP to the "nominal SP".  The latter is defined
    /// as the value the stack pointer has after the prolog.  This offset is
    /// normally always zero, except during a call sequence using the tail-call
    /// ABI, between the AllocateArgs and the actual call instruction.
    pub(crate) nominal_sp_offset: u32,

    /// Offset from the actual SP to the SP during an outgoing function call.
    /// This is normally always zero, except during processing of the return
    /// argument handling after a call using the tail-call ABI has returned.
    pub(crate) outgoing_sp_offset: u32,

    /// Size of the incoming argument area in the caller's frame.  Always zero
    /// for functions using the tail-call ABI.
    pub(crate) incoming_args_size: u32,

    /// The user stack map for the upcoming instruction, as provided to
    /// `pre_safepoint()`.
    user_stack_map: Option<ir::UserStackMap>,

    /// Only used during fuzz-testing. Otherwise, it is a zero-sized struct and
    /// optimized away at compiletime. See [cranelift_control].
    ctrl_plane: ControlPlane,

    frame_layout: FrameLayout,
}

impl MachInstEmitState<Inst> for EmitState {
    fn new(abi: &Callee<S390xMachineDeps>, ctrl_plane: ControlPlane) -> Self {
        let incoming_args_size = if abi.call_conv() == CallConv::Tail {
            0
        } else {
            abi.frame_layout().incoming_args_size
        };
        EmitState {
            nominal_sp_offset: 0,
            outgoing_sp_offset: 0,
            incoming_args_size,
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
pub struct EmitInfo {
    isa_flags: s390x_settings::Flags,
}

impl EmitInfo {
    pub(crate) fn new(isa_flags: s390x_settings::Flags) -> Self {
        Self { isa_flags }
    }
}

impl MachInstEmit for Inst {
    type State = EmitState;
    type Info = EmitInfo;

    fn emit(&self, sink: &mut MachBuffer<Inst>, emit_info: &Self::Info, state: &mut EmitState) {
        self.emit_with_alloc_consumer(sink, emit_info, state)
    }

    fn pretty_print_inst(&self, state: &mut EmitState) -> String {
        self.print_with_state(state)
    }
}

impl Inst {
    fn emit_with_alloc_consumer(
        &self,
        sink: &mut MachBuffer<Inst>,
        emit_info: &EmitInfo,
        state: &mut EmitState,
    ) {
        // Verify that we can emit this Inst in the current ISA
        let matches_isa_flags = |iset_requirement: &InstructionSet| -> bool {
            match iset_requirement {
                // Baseline ISA is z14
                InstructionSet::Base => true,
                // Miscellaneous-Instruction-Extensions Facility 3 (z15)
                InstructionSet::MIE3 => emit_info.isa_flags.has_mie3(),
                // Vector-Enhancements Facility 2 (z15)
                InstructionSet::VXRS_EXT2 => emit_info.isa_flags.has_vxrs_ext2(),
            }
        };
        let isa_requirements = self.available_in_isa();
        if !matches_isa_flags(&isa_requirements) {
            panic!(
                "Cannot emit inst '{self:?}' for target; failed to match ISA requirements: {isa_requirements:?}"
            )
        }

        match self {
            &Inst::AluRRR { alu_op, rd, rn, rm } => {
                let (opcode, have_rr) = match alu_op {
                    ALUOp::Add32 => (0xb9f8, true),        // ARK
                    ALUOp::Add64 => (0xb9e8, true),        // AGRK
                    ALUOp::AddLogical32 => (0xb9fa, true), // ALRK
                    ALUOp::AddLogical64 => (0xb9ea, true), // ALGRK
                    ALUOp::Sub32 => (0xb9f9, true),        // SRK
                    ALUOp::Sub64 => (0xb9e9, true),        // SGRK
                    ALUOp::SubLogical32 => (0xb9fb, true), // SLRK
                    ALUOp::SubLogical64 => (0xb9eb, true), // SLGRK
                    ALUOp::Mul32 => (0xb9fd, true),        // MSRKC
                    ALUOp::Mul64 => (0xb9ed, true),        // MSGRKC
                    ALUOp::And32 => (0xb9f4, true),        // NRK
                    ALUOp::And64 => (0xb9e4, true),        // NGRK
                    ALUOp::Orr32 => (0xb9f6, true),        // ORK
                    ALUOp::Orr64 => (0xb9e6, true),        // OGRK
                    ALUOp::Xor32 => (0xb9f7, true),        // XRK
                    ALUOp::Xor64 => (0xb9e7, true),        // XGRK
                    ALUOp::NotAnd32 => (0xb974, false),    // NNRK
                    ALUOp::NotAnd64 => (0xb964, false),    // NNGRK
                    ALUOp::NotOrr32 => (0xb976, false),    // NORK
                    ALUOp::NotOrr64 => (0xb966, false),    // NOGRK
                    ALUOp::NotXor32 => (0xb977, false),    // NXRK
                    ALUOp::NotXor64 => (0xb967, false),    // NXGRK
                    ALUOp::AndNot32 => (0xb9f5, false),    // NCRK
                    ALUOp::AndNot64 => (0xb9e5, false),    // NCGRK
                    ALUOp::OrrNot32 => (0xb975, false),    // OCRK
                    ALUOp::OrrNot64 => (0xb965, false),    // OCGRK
                    _ => unreachable!(),
                };
                if have_rr && rd.to_reg() == rn {
                    let inst = Inst::AluRR {
                        alu_op,
                        rd,
                        ri: rn,
                        rm,
                    };
                    inst.emit(sink, emit_info, state);
                } else {
                    put(sink, &enc_rrf_ab(opcode, rd.to_reg(), rn, rm, 0));
                }
            }
            &Inst::AluRRSImm16 {
                alu_op,
                rd,
                rn,
                imm,
            } => {
                if rd.to_reg() == rn {
                    let inst = Inst::AluRSImm16 {
                        alu_op,
                        rd,
                        ri: rn,
                        imm,
                    };
                    inst.emit(sink, emit_info, state);
                } else {
                    let opcode = match alu_op {
                        ALUOp::Add32 => 0xecd8, // AHIK
                        ALUOp::Add64 => 0xecd9, // AGHIK
                        _ => unreachable!(),
                    };
                    put(sink, &enc_rie_d(opcode, rd.to_reg(), rn, imm as u16));
                }
            }
            &Inst::AluRR { alu_op, rd, ri, rm } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let (opcode, is_rre) = match alu_op {
                    ALUOp::Add32 => (0x1a, false),              // AR
                    ALUOp::Add64 => (0xb908, true),             // AGR
                    ALUOp::Add64Ext32 => (0xb918, true),        // AGFR
                    ALUOp::AddLogical32 => (0x1e, false),       // ALR
                    ALUOp::AddLogical64 => (0xb90a, true),      // ALGR
                    ALUOp::AddLogical64Ext32 => (0xb91a, true), // ALGFR
                    ALUOp::Sub32 => (0x1b, false),              // SR
                    ALUOp::Sub64 => (0xb909, true),             // SGR
                    ALUOp::Sub64Ext32 => (0xb919, true),        // SGFR
                    ALUOp::SubLogical32 => (0x1f, false),       // SLR
                    ALUOp::SubLogical64 => (0xb90b, true),      // SLGR
                    ALUOp::SubLogical64Ext32 => (0xb91b, true), // SLGFR
                    ALUOp::Mul32 => (0xb252, true),             // MSR
                    ALUOp::Mul64 => (0xb90c, true),             // MSGR
                    ALUOp::Mul64Ext32 => (0xb91c, true),        // MSGFR
                    ALUOp::And32 => (0x14, false),              // NR
                    ALUOp::And64 => (0xb980, true),             // NGR
                    ALUOp::Orr32 => (0x16, false),              // OR
                    ALUOp::Orr64 => (0xb981, true),             // OGR
                    ALUOp::Xor32 => (0x17, false),              // XR
                    ALUOp::Xor64 => (0xb982, true),             // XGR
                    _ => unreachable!(),
                };
                if is_rre {
                    put(sink, &enc_rre(opcode, rd.to_reg(), rm));
                } else {
                    put(sink, &enc_rr(opcode, rd.to_reg(), rm));
                }
            }
            &Inst::AluRX {
                alu_op,
                rd,
                ri,
                ref mem,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);
                let mem = mem.clone();

                let (opcode_rx, opcode_rxy) = match alu_op {
                    ALUOp::Add32 => (Some(0x5a), Some(0xe35a)),        // A(Y)
                    ALUOp::Add32Ext16 => (Some(0x4a), Some(0xe37a)),   // AH(Y)
                    ALUOp::Add64 => (None, Some(0xe308)),              // AG
                    ALUOp::Add64Ext16 => (None, Some(0xe338)),         // AGH
                    ALUOp::Add64Ext32 => (None, Some(0xe318)),         // AGF
                    ALUOp::AddLogical32 => (Some(0x5e), Some(0xe35e)), // AL(Y)
                    ALUOp::AddLogical64 => (None, Some(0xe30a)),       // ALG
                    ALUOp::AddLogical64Ext32 => (None, Some(0xe31a)),  // ALGF
                    ALUOp::Sub32 => (Some(0x5b), Some(0xe35b)),        // S(Y)
                    ALUOp::Sub32Ext16 => (Some(0x4b), Some(0xe37b)),   // SH(Y)
                    ALUOp::Sub64 => (None, Some(0xe309)),              // SG
                    ALUOp::Sub64Ext16 => (None, Some(0xe339)),         // SGH
                    ALUOp::Sub64Ext32 => (None, Some(0xe319)),         // SGF
                    ALUOp::SubLogical32 => (Some(0x5f), Some(0xe35f)), // SL(Y)
                    ALUOp::SubLogical64 => (None, Some(0xe30b)),       // SLG
                    ALUOp::SubLogical64Ext32 => (None, Some(0xe31b)),  // SLGF
                    ALUOp::Mul32 => (Some(0x71), Some(0xe351)),        // MS(Y)
                    ALUOp::Mul32Ext16 => (Some(0x4c), Some(0xe37c)),   // MH(Y)
                    ALUOp::Mul64 => (None, Some(0xe30c)),              // MSG
                    ALUOp::Mul64Ext16 => (None, Some(0xe33c)),         // MSH
                    ALUOp::Mul64Ext32 => (None, Some(0xe31c)),         // MSGF
                    ALUOp::And32 => (Some(0x54), Some(0xe354)),        // N(Y)
                    ALUOp::And64 => (None, Some(0xe380)),              // NG
                    ALUOp::Orr32 => (Some(0x56), Some(0xe356)),        // O(Y)
                    ALUOp::Orr64 => (None, Some(0xe381)),              // OG
                    ALUOp::Xor32 => (Some(0x57), Some(0xe357)),        // X(Y)
                    ALUOp::Xor64 => (None, Some(0xe382)),              // XG
                    _ => unreachable!(),
                };
                let rd = rd.to_reg();
                mem_emit(
                    rd, &mem, opcode_rx, opcode_rxy, None, true, sink, emit_info, state,
                );
            }
            &Inst::AluRSImm16 {
                alu_op,
                rd,
                ri,
                imm,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = match alu_op {
                    ALUOp::Add32 => 0xa7a, // AHI
                    ALUOp::Add64 => 0xa7b, // AGHI
                    ALUOp::Mul32 => 0xa7c, // MHI
                    ALUOp::Mul64 => 0xa7d, // MGHI
                    _ => unreachable!(),
                };
                put(sink, &enc_ri_a(opcode, rd.to_reg(), imm as u16));
            }
            &Inst::AluRSImm32 {
                alu_op,
                rd,
                ri,
                imm,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = match alu_op {
                    ALUOp::Add32 => 0xc29, // AFI
                    ALUOp::Add64 => 0xc28, // AGFI
                    ALUOp::Mul32 => 0xc21, // MSFI
                    ALUOp::Mul64 => 0xc20, // MSGFI
                    _ => unreachable!(),
                };
                put(sink, &enc_ril_a(opcode, rd.to_reg(), imm as u32));
            }
            &Inst::AluRUImm32 {
                alu_op,
                rd,
                ri,
                imm,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = match alu_op {
                    ALUOp::AddLogical32 => 0xc2b, // ALFI
                    ALUOp::AddLogical64 => 0xc2a, // ALGFI
                    ALUOp::SubLogical32 => 0xc25, // SLFI
                    ALUOp::SubLogical64 => 0xc24, // SLGFI
                    _ => unreachable!(),
                };
                put(sink, &enc_ril_a(opcode, rd.to_reg(), imm));
            }
            &Inst::AluRUImm16Shifted {
                alu_op,
                rd,
                ri,
                imm,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = match (alu_op, imm.shift) {
                    (ALUOp::And32, 0) => 0xa57, // NILL
                    (ALUOp::And32, 1) => 0xa56, // NILH
                    (ALUOp::And64, 0) => 0xa57, // NILL
                    (ALUOp::And64, 1) => 0xa56, // NILH
                    (ALUOp::And64, 2) => 0xa55, // NIHL
                    (ALUOp::And64, 3) => 0xa54, // NIHL
                    (ALUOp::Orr32, 0) => 0xa5b, // OILL
                    (ALUOp::Orr32, 1) => 0xa5a, // OILH
                    (ALUOp::Orr64, 0) => 0xa5b, // OILL
                    (ALUOp::Orr64, 1) => 0xa5a, // OILH
                    (ALUOp::Orr64, 2) => 0xa59, // OIHL
                    (ALUOp::Orr64, 3) => 0xa58, // OIHH
                    _ => unreachable!(),
                };
                put(sink, &enc_ri_a(opcode, rd.to_reg(), imm.bits));
            }
            &Inst::AluRUImm32Shifted {
                alu_op,
                rd,
                ri,
                imm,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = match (alu_op, imm.shift) {
                    (ALUOp::And32, 0) => 0xc0b, // NILF
                    (ALUOp::And64, 0) => 0xc0b, // NILF
                    (ALUOp::And64, 1) => 0xc0a, // NIHF
                    (ALUOp::Orr32, 0) => 0xc0d, // OILF
                    (ALUOp::Orr64, 0) => 0xc0d, // OILF
                    (ALUOp::Orr64, 1) => 0xc0c, // OILF
                    (ALUOp::Xor32, 0) => 0xc07, // XILF
                    (ALUOp::Xor64, 0) => 0xc07, // XILF
                    (ALUOp::Xor64, 1) => 0xc06, // XILH
                    _ => unreachable!(),
                };
                put(sink, &enc_ril_a(opcode, rd.to_reg(), imm.bits));
            }

            &Inst::SMulWide { rd, rn, rm } => {
                let rd1 = rd.hi;
                let rd2 = rd.lo;
                debug_assert_valid_regpair!(rd1.to_reg(), rd2.to_reg());

                let opcode = 0xb9ec; // MGRK
                put(sink, &enc_rrf_ab(opcode, rd1.to_reg(), rn, rm, 0));
            }
            &Inst::UMulWide { rd, ri, rn } => {
                let rd1 = rd.hi;
                let rd2 = rd.lo;
                debug_assert_valid_regpair!(rd1.to_reg(), rd2.to_reg());
                debug_assert_eq!(rd2.to_reg(), ri);

                let opcode = 0xb986; // MLGR
                put(sink, &enc_rre(opcode, rd1.to_reg(), rn));
            }
            &Inst::SDivMod32 { rd, ri, rn } => {
                let rd1 = rd.hi;
                let rd2 = rd.lo;
                debug_assert_valid_regpair!(rd1.to_reg(), rd2.to_reg());
                debug_assert_eq!(rd2.to_reg(), ri);

                let opcode = 0xb91d; // DSGFR
                let trap_code = TrapCode::INTEGER_DIVISION_BY_ZERO;
                put_with_trap(sink, &enc_rre(opcode, rd1.to_reg(), rn), trap_code);
            }
            &Inst::SDivMod64 { rd, ri, rn } => {
                let rd1 = rd.hi;
                let rd2 = rd.lo;
                debug_assert_valid_regpair!(rd1.to_reg(), rd2.to_reg());
                debug_assert_eq!(rd2.to_reg(), ri);

                let opcode = 0xb90d; // DSGR
                let trap_code = TrapCode::INTEGER_DIVISION_BY_ZERO;
                put_with_trap(sink, &enc_rre(opcode, rd1.to_reg(), rn), trap_code);
            }
            &Inst::UDivMod32 { rd, ri, rn } => {
                let rd1 = rd.hi;
                let rd2 = rd.lo;
                debug_assert_valid_regpair!(rd1.to_reg(), rd2.to_reg());
                let ri1 = ri.hi;
                let ri2 = ri.lo;
                debug_assert_eq!(rd1.to_reg(), ri1);
                debug_assert_eq!(rd2.to_reg(), ri2);

                let opcode = 0xb997; // DLR
                let trap_code = TrapCode::INTEGER_DIVISION_BY_ZERO;
                put_with_trap(sink, &enc_rre(opcode, rd1.to_reg(), rn), trap_code);
            }
            &Inst::UDivMod64 { rd, ri, rn } => {
                let rd1 = rd.hi;
                let rd2 = rd.lo;
                debug_assert_valid_regpair!(rd1.to_reg(), rd2.to_reg());
                let ri1 = ri.hi;
                let ri2 = ri.lo;
                debug_assert_eq!(rd1.to_reg(), ri1);
                debug_assert_eq!(rd2.to_reg(), ri2);

                let opcode = 0xb987; // DLGR
                let trap_code = TrapCode::INTEGER_DIVISION_BY_ZERO;
                put_with_trap(sink, &enc_rre(opcode, rd1.to_reg(), rn), trap_code);
            }
            &Inst::Flogr { rd, rn } => {
                let rd1 = rd.hi;
                let rd2 = rd.lo;
                debug_assert_valid_regpair!(rd1.to_reg(), rd2.to_reg());

                let opcode = 0xb983; // FLOGR
                put(sink, &enc_rre(opcode, rd1.to_reg(), rn));
            }

            &Inst::ShiftRR {
                shift_op,
                rd,
                rn,
                shift_imm,
                shift_reg,
            } => {
                let opcode = match shift_op {
                    ShiftOp::RotL32 => 0xeb1d, // RLL
                    ShiftOp::RotL64 => 0xeb1c, // RLLG
                    ShiftOp::LShL32 => 0xebdf, // SLLK  (SLL ?)
                    ShiftOp::LShL64 => 0xeb0d, // SLLG
                    ShiftOp::LShR32 => 0xebde, // SRLK  (SRL ?)
                    ShiftOp::LShR64 => 0xeb0c, // SRLG
                    ShiftOp::AShR32 => 0xebdc, // SRAK  (SRA ?)
                    ShiftOp::AShR64 => 0xeb0a, // SRAG
                };
                put(
                    sink,
                    &enc_rsy(opcode, rd.to_reg(), rn, shift_reg, shift_imm.into()),
                );
            }

            &Inst::RxSBG {
                op,
                rd,
                ri,
                rn,
                start_bit,
                end_bit,
                rotate_amt,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = match op {
                    RxSBGOp::Insert => 0xec59, // RISBGN
                    RxSBGOp::And => 0xec54,    // RNSBG
                    RxSBGOp::Or => 0xec56,     // ROSBG
                    RxSBGOp::Xor => 0xec57,    // RXSBG
                };
                put(
                    sink,
                    &enc_rie_f(
                        opcode,
                        rd.to_reg(),
                        rn,
                        start_bit,
                        end_bit,
                        (rotate_amt as u8) & 63,
                    ),
                );
            }

            &Inst::RxSBGTest {
                op,
                rd,
                rn,
                start_bit,
                end_bit,
                rotate_amt,
            } => {
                let opcode = match op {
                    RxSBGOp::And => 0xec54, // RNSBG
                    RxSBGOp::Or => 0xec56,  // ROSBG
                    RxSBGOp::Xor => 0xec57, // RXSBG
                    _ => unreachable!(),
                };
                put(
                    sink,
                    &enc_rie_f(
                        opcode,
                        rd,
                        rn,
                        start_bit | 0x80,
                        end_bit,
                        (rotate_amt as u8) & 63,
                    ),
                );
            }

            &Inst::UnaryRR { op, rd, rn } => {
                match op {
                    UnaryOp::Abs32 => {
                        let opcode = 0x10; // LPR
                        put(sink, &enc_rr(opcode, rd.to_reg(), rn));
                    }
                    UnaryOp::Abs64 => {
                        let opcode = 0xb900; // LPGR
                        put(sink, &enc_rre(opcode, rd.to_reg(), rn));
                    }
                    UnaryOp::Abs64Ext32 => {
                        let opcode = 0xb910; // LPGFR
                        put(sink, &enc_rre(opcode, rd.to_reg(), rn));
                    }
                    UnaryOp::Neg32 => {
                        let opcode = 0x13; // LCR
                        put(sink, &enc_rr(opcode, rd.to_reg(), rn));
                    }
                    UnaryOp::Neg64 => {
                        let opcode = 0xb903; // LCGR
                        put(sink, &enc_rre(opcode, rd.to_reg(), rn));
                    }
                    UnaryOp::Neg64Ext32 => {
                        let opcode = 0xb913; // LCGFR
                        put(sink, &enc_rre(opcode, rd.to_reg(), rn));
                    }
                    UnaryOp::PopcntByte => {
                        let opcode = 0xb9e1; // POPCNT
                        put(sink, &enc_rrf_cde(opcode, rd.to_reg(), rn, 0, 0));
                    }
                    UnaryOp::PopcntReg => {
                        let opcode = 0xb9e1; // POPCNT
                        put(sink, &enc_rrf_cde(opcode, rd.to_reg(), rn, 8, 0));
                    }
                    UnaryOp::BSwap32 => {
                        let opcode = 0xb91f; // LRVR
                        put(sink, &enc_rre(opcode, rd.to_reg(), rn));
                    }
                    UnaryOp::BSwap64 => {
                        let opcode = 0xb90f; // LRVRG
                        put(sink, &enc_rre(opcode, rd.to_reg(), rn));
                    }
                }
            }

            &Inst::Extend {
                rd,
                rn,
                signed,
                from_bits,
                to_bits,
            } => {
                let opcode = match (signed, from_bits, to_bits) {
                    (_, 1, 32) => 0xb926,      // LBR
                    (_, 1, 64) => 0xb906,      // LGBR
                    (false, 8, 32) => 0xb994,  // LLCR
                    (false, 8, 64) => 0xb984,  // LLGCR
                    (true, 8, 32) => 0xb926,   // LBR
                    (true, 8, 64) => 0xb906,   // LGBR
                    (false, 16, 32) => 0xb995, // LLHR
                    (false, 16, 64) => 0xb985, // LLGHR
                    (true, 16, 32) => 0xb927,  // LHR
                    (true, 16, 64) => 0xb907,  // LGHR
                    (false, 32, 64) => 0xb916, // LLGFR
                    (true, 32, 64) => 0xb914,  // LGFR
                    _ => panic!(
                        "Unsupported extend combination: signed = {signed}, from_bits = {from_bits}, to_bits = {to_bits}"
                    ),
                };
                put(sink, &enc_rre(opcode, rd.to_reg(), rn));
            }

            &Inst::CmpRR { op, rn, rm } => {
                let (opcode, is_rre) = match op {
                    CmpOp::CmpS32 => (0x19, false),       // CR
                    CmpOp::CmpS64 => (0xb920, true),      // CGR
                    CmpOp::CmpS64Ext32 => (0xb930, true), // CGFR
                    CmpOp::CmpL32 => (0x15, false),       // CLR
                    CmpOp::CmpL64 => (0xb921, true),      // CLGR
                    CmpOp::CmpL64Ext32 => (0xb931, true), // CLGFR
                    _ => unreachable!(),
                };
                if is_rre {
                    put(sink, &enc_rre(opcode, rn, rm));
                } else {
                    put(sink, &enc_rr(opcode, rn, rm));
                }
            }
            &Inst::CmpRX { op, rn, ref mem } => {
                let mem = mem.clone();

                let (opcode_rx, opcode_rxy, opcode_ril) = match op {
                    CmpOp::CmpS32 => (Some(0x59), Some(0xe359), Some(0xc6d)), // C(Y), CRL
                    CmpOp::CmpS32Ext16 => (Some(0x49), Some(0xe379), Some(0xc65)), // CH(Y), CHRL
                    CmpOp::CmpS64 => (None, Some(0xe320), Some(0xc68)),       // CG, CGRL
                    CmpOp::CmpS64Ext16 => (None, Some(0xe334), Some(0xc64)),  // CGH, CGHRL
                    CmpOp::CmpS64Ext32 => (None, Some(0xe330), Some(0xc6c)),  // CGF, CGFRL
                    CmpOp::CmpL32 => (Some(0x55), Some(0xe355), Some(0xc6f)), // CL(Y), CLRL
                    CmpOp::CmpL32Ext16 => (None, None, Some(0xc67)),          // CLHRL
                    CmpOp::CmpL64 => (None, Some(0xe321), Some(0xc6a)),       // CLG, CLGRL
                    CmpOp::CmpL64Ext16 => (None, None, Some(0xc66)),          // CLGHRL
                    CmpOp::CmpL64Ext32 => (None, Some(0xe331), Some(0xc6e)),  // CLGF, CLGFRL
                };
                mem_emit(
                    rn, &mem, opcode_rx, opcode_rxy, opcode_ril, true, sink, emit_info, state,
                );
            }
            &Inst::CmpRSImm16 { op, rn, imm } => {
                let opcode = match op {
                    CmpOp::CmpS32 => 0xa7e, // CHI
                    CmpOp::CmpS64 => 0xa7f, // CGHI
                    _ => unreachable!(),
                };
                put(sink, &enc_ri_a(opcode, rn, imm as u16));
            }
            &Inst::CmpRSImm32 { op, rn, imm } => {
                let opcode = match op {
                    CmpOp::CmpS32 => 0xc2d, // CFI
                    CmpOp::CmpS64 => 0xc2c, // CGFI
                    _ => unreachable!(),
                };
                put(sink, &enc_ril_a(opcode, rn, imm as u32));
            }
            &Inst::CmpRUImm32 { op, rn, imm } => {
                let opcode = match op {
                    CmpOp::CmpL32 => 0xc2f, // CLFI
                    CmpOp::CmpL64 => 0xc2e, // CLGFI
                    _ => unreachable!(),
                };
                put(sink, &enc_ril_a(opcode, rn, imm));
            }
            &Inst::CmpTrapRR {
                op,
                rn,
                rm,
                cond,
                trap_code,
            } => {
                let opcode = match op {
                    CmpOp::CmpS32 => 0xb972, // CRT
                    CmpOp::CmpS64 => 0xb960, // CGRT
                    CmpOp::CmpL32 => 0xb973, // CLRT
                    CmpOp::CmpL64 => 0xb961, // CLGRT
                    _ => unreachable!(),
                };
                put_with_trap(
                    sink,
                    &enc_rrf_cde(opcode, rn, rm, cond.bits(), 0),
                    trap_code,
                );
            }
            &Inst::CmpTrapRSImm16 {
                op,
                rn,
                imm,
                cond,
                trap_code,
            } => {
                let opcode = match op {
                    CmpOp::CmpS32 => 0xec72, // CIT
                    CmpOp::CmpS64 => 0xec70, // CGIT
                    _ => unreachable!(),
                };
                put_with_trap(
                    sink,
                    &enc_rie_a(opcode, rn, imm as u16, cond.bits()),
                    trap_code,
                );
            }
            &Inst::CmpTrapRUImm16 {
                op,
                rn,
                imm,
                cond,
                trap_code,
            } => {
                let opcode = match op {
                    CmpOp::CmpL32 => 0xec73, // CLFIT
                    CmpOp::CmpL64 => 0xec71, // CLGIT
                    _ => unreachable!(),
                };
                put_with_trap(sink, &enc_rie_a(opcode, rn, imm, cond.bits()), trap_code);
            }

            &Inst::AtomicRmw {
                alu_op,
                rd,
                rn,
                ref mem,
            } => {
                let mem = mem.clone();

                let opcode = match alu_op {
                    ALUOp::Add32 => 0xebf8,        // LAA
                    ALUOp::Add64 => 0xebe8,        // LAAG
                    ALUOp::AddLogical32 => 0xebfa, // LAAL
                    ALUOp::AddLogical64 => 0xebea, // LAALG
                    ALUOp::And32 => 0xebf4,        // LAN
                    ALUOp::And64 => 0xebe4,        // LANG
                    ALUOp::Orr32 => 0xebf6,        // LAO
                    ALUOp::Orr64 => 0xebe6,        // LAOG
                    ALUOp::Xor32 => 0xebf7,        // LAX
                    ALUOp::Xor64 => 0xebe7,        // LAXG
                    _ => unreachable!(),
                };

                let rd = rd.to_reg();
                mem_rs_emit(
                    rd,
                    rn,
                    &mem,
                    None,
                    Some(opcode),
                    true,
                    sink,
                    emit_info,
                    state,
                );
            }
            &Inst::Loop { ref body, cond } => {
                // This sequence is *one* instruction in the vcode, and is expanded only here at
                // emission time, because it requires branching to internal labels.
                let loop_label = sink.get_label();
                let done_label = sink.get_label();

                // Emit label at the start of the loop.
                sink.bind_label(loop_label, &mut state.ctrl_plane);

                for inst in (&body).into_iter() {
                    match &inst {
                        // Replace a CondBreak with a branch to done_label.
                        &Inst::CondBreak { cond } => {
                            let opcode = 0xc04; // BCRL
                            sink.use_label_at_offset(
                                sink.cur_offset(),
                                done_label,
                                LabelUse::BranchRIL,
                            );
                            put(sink, &enc_ril_c(opcode, cond.bits(), 0));
                        }
                        _ => inst.emit_with_alloc_consumer(sink, emit_info, state),
                    };
                }

                let opcode = 0xc04; // BCRL
                sink.use_label_at_offset(sink.cur_offset(), loop_label, LabelUse::BranchRIL);
                put(sink, &enc_ril_c(opcode, cond.bits(), 0));

                // Emit label at the end of the loop.
                sink.bind_label(done_label, &mut state.ctrl_plane);
            }
            &Inst::CondBreak { .. } => unreachable!(), // Only valid inside a Loop.
            &Inst::AtomicCas32 {
                rd,
                ri,
                rn,
                ref mem,
            }
            | &Inst::AtomicCas64 {
                rd,
                ri,
                rn,
                ref mem,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);
                let mem = mem.clone();

                let (opcode_rs, opcode_rsy) = match self {
                    &Inst::AtomicCas32 { .. } => (Some(0xba), Some(0xeb14)), // CS(Y)
                    &Inst::AtomicCas64 { .. } => (None, Some(0xeb30)),       // CSG
                    _ => unreachable!(),
                };

                let rd = rd.to_reg();
                mem_rs_emit(
                    rd, rn, &mem, opcode_rs, opcode_rsy, true, sink, emit_info, state,
                );
            }
            &Inst::Fence => {
                put(sink, &enc_e(0x07e0));
            }

            &Inst::Load32 { rd, ref mem }
            | &Inst::Load32ZExt8 { rd, ref mem }
            | &Inst::Load32SExt8 { rd, ref mem }
            | &Inst::Load32ZExt16 { rd, ref mem }
            | &Inst::Load32SExt16 { rd, ref mem }
            | &Inst::Load64 { rd, ref mem }
            | &Inst::Load64ZExt8 { rd, ref mem }
            | &Inst::Load64SExt8 { rd, ref mem }
            | &Inst::Load64ZExt16 { rd, ref mem }
            | &Inst::Load64SExt16 { rd, ref mem }
            | &Inst::Load64ZExt32 { rd, ref mem }
            | &Inst::Load64SExt32 { rd, ref mem }
            | &Inst::LoadRev16 { rd, ref mem }
            | &Inst::LoadRev32 { rd, ref mem }
            | &Inst::LoadRev64 { rd, ref mem } => {
                let mem = mem.clone();

                let (opcode_rx, opcode_rxy, opcode_ril) = match self {
                    &Inst::Load32 { .. } => (Some(0x58), Some(0xe358), Some(0xc4d)), // L(Y), LRL
                    &Inst::Load32ZExt8 { .. } => (None, Some(0xe394), None),         // LLC
                    &Inst::Load32SExt8 { .. } => (None, Some(0xe376), None),         // LB
                    &Inst::Load32ZExt16 { .. } => (None, Some(0xe395), Some(0xc42)), // LLH, LLHRL
                    &Inst::Load32SExt16 { .. } => (Some(0x48), Some(0xe378), Some(0xc45)), // LH(Y), LHRL
                    &Inst::Load64 { .. } => (None, Some(0xe304), Some(0xc48)), // LG, LGRL
                    &Inst::Load64ZExt8 { .. } => (None, Some(0xe390), None),   // LLGC
                    &Inst::Load64SExt8 { .. } => (None, Some(0xe377), None),   // LGB
                    &Inst::Load64ZExt16 { .. } => (None, Some(0xe391), Some(0xc46)), // LLGH, LLGHRL
                    &Inst::Load64SExt16 { .. } => (None, Some(0xe315), Some(0xc44)), // LGH, LGHRL
                    &Inst::Load64ZExt32 { .. } => (None, Some(0xe316), Some(0xc4e)), // LLGF, LLGFRL
                    &Inst::Load64SExt32 { .. } => (None, Some(0xe314), Some(0xc4c)), // LGF, LGFRL
                    &Inst::LoadRev16 { .. } => (None, Some(0xe31f), None),     // LRVH
                    &Inst::LoadRev32 { .. } => (None, Some(0xe31e), None),     // LRV
                    &Inst::LoadRev64 { .. } => (None, Some(0xe30f), None),     // LRVG
                    _ => unreachable!(),
                };
                let rd = rd.to_reg();
                mem_emit(
                    rd, &mem, opcode_rx, opcode_rxy, opcode_ril, true, sink, emit_info, state,
                );
            }

            &Inst::Store8 { rd, ref mem }
            | &Inst::Store16 { rd, ref mem }
            | &Inst::Store32 { rd, ref mem }
            | &Inst::Store64 { rd, ref mem }
            | &Inst::StoreRev16 { rd, ref mem }
            | &Inst::StoreRev32 { rd, ref mem }
            | &Inst::StoreRev64 { rd, ref mem } => {
                let mem = mem.clone();

                let (opcode_rx, opcode_rxy, opcode_ril) = match self {
                    &Inst::Store8 { .. } => (Some(0x42), Some(0xe372), None), // STC(Y)
                    &Inst::Store16 { .. } => (Some(0x40), Some(0xe370), Some(0xc47)), // STH(Y), STHRL
                    &Inst::Store32 { .. } => (Some(0x50), Some(0xe350), Some(0xc4f)), // ST(Y), STRL
                    &Inst::Store64 { .. } => (None, Some(0xe324), Some(0xc4b)),       // STG, STGRL
                    &Inst::StoreRev16 { .. } => (None, Some(0xe33f), None),           // STRVH
                    &Inst::StoreRev32 { .. } => (None, Some(0xe33e), None),           // STRV
                    &Inst::StoreRev64 { .. } => (None, Some(0xe32f), None),           // STRVG
                    _ => unreachable!(),
                };
                mem_emit(
                    rd, &mem, opcode_rx, opcode_rxy, opcode_ril, true, sink, emit_info, state,
                );
            }
            &Inst::StoreImm8 { imm, ref mem } => {
                let mem = mem.clone();

                let opcode_si = 0x92; // MVI
                let opcode_siy = 0xeb52; // MVIY
                mem_imm8_emit(
                    imm, &mem, opcode_si, opcode_siy, true, sink, emit_info, state,
                );
            }
            &Inst::StoreImm16 { imm, ref mem }
            | &Inst::StoreImm32SExt16 { imm, ref mem }
            | &Inst::StoreImm64SExt16 { imm, ref mem } => {
                let mem = mem.clone();

                let opcode = match self {
                    &Inst::StoreImm16 { .. } => 0xe544,       // MVHHI
                    &Inst::StoreImm32SExt16 { .. } => 0xe54c, // MVHI
                    &Inst::StoreImm64SExt16 { .. } => 0xe548, // MVGHI
                    _ => unreachable!(),
                };
                mem_imm16_emit(imm, &mem, opcode, true, sink, emit_info, state);
            }

            &Inst::LoadMultiple64 { rt, rt2, ref mem } => {
                let mem = mem.clone();

                let opcode = 0xeb04; // LMG
                let rt = rt.to_reg();
                let rt2 = rt2.to_reg();
                mem_rs_emit(
                    rt,
                    rt2,
                    &mem,
                    None,
                    Some(opcode),
                    true,
                    sink,
                    emit_info,
                    state,
                );
            }
            &Inst::StoreMultiple64 { rt, rt2, ref mem } => {
                let mem = mem.clone();

                let opcode = 0xeb24; // STMG
                mem_rs_emit(
                    rt,
                    rt2,
                    &mem,
                    None,
                    Some(opcode),
                    true,
                    sink,
                    emit_info,
                    state,
                );
            }

            &Inst::LoadAddr { rd, ref mem } => {
                let mem = mem.clone();

                let opcode_rx = Some(0x41); // LA
                let opcode_rxy = Some(0xe371); // LAY
                let opcode_ril = Some(0xc00); // LARL
                let rd = rd.to_reg();
                mem_emit(
                    rd, &mem, opcode_rx, opcode_rxy, opcode_ril, false, sink, emit_info, state,
                );
            }

            &Inst::Mov64 { rd, rm } => {
                let opcode = 0xb904; // LGR
                put(sink, &enc_rre(opcode, rd.to_reg(), rm));
            }
            &Inst::MovPReg { rd, rm } => {
                Inst::Mov64 { rd, rm: rm.into() }.emit(sink, emit_info, state);
            }
            &Inst::Mov32 { rd, rm } => {
                let opcode = 0x18; // LR
                put(sink, &enc_rr(opcode, rd.to_reg(), rm));
            }
            &Inst::Mov32Imm { rd, imm } => {
                let opcode = 0xc09; // IILF
                put(sink, &enc_ril_a(opcode, rd.to_reg(), imm));
            }
            &Inst::Mov32SImm16 { rd, imm } => {
                let opcode = 0xa78; // LHI
                put(sink, &enc_ri_a(opcode, rd.to_reg(), imm as u16));
            }
            &Inst::Mov64SImm16 { rd, imm } => {
                let opcode = 0xa79; // LGHI
                put(sink, &enc_ri_a(opcode, rd.to_reg(), imm as u16));
            }
            &Inst::Mov64SImm32 { rd, imm } => {
                let opcode = 0xc01; // LGFI
                put(sink, &enc_ril_a(opcode, rd.to_reg(), imm as u32));
            }
            &Inst::CMov32 { rd, cond, ri, rm } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = 0xb9f2; // LOCR
                put(sink, &enc_rrf_cde(opcode, rd.to_reg(), rm, cond.bits(), 0));
            }
            &Inst::CMov64 { rd, cond, ri, rm } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = 0xb9e2; // LOCGR
                put(sink, &enc_rrf_cde(opcode, rd.to_reg(), rm, cond.bits(), 0));
            }
            &Inst::CMov32SImm16 { rd, cond, ri, imm } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = 0xec42; // LOCHI
                put(
                    sink,
                    &enc_rie_g(opcode, rd.to_reg(), imm as u16, cond.bits()),
                );
            }
            &Inst::CMov64SImm16 { rd, cond, ri, imm } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = 0xec46; // LOCGHI
                put(
                    sink,
                    &enc_rie_g(opcode, rd.to_reg(), imm as u16, cond.bits()),
                );
            }
            &Inst::Mov64UImm16Shifted { rd, imm } => {
                let opcode = match imm.shift {
                    0 => 0xa5f, // LLILL
                    1 => 0xa5e, // LLILH
                    2 => 0xa5d, // LLIHL
                    3 => 0xa5c, // LLIHH
                    _ => unreachable!(),
                };
                put(sink, &enc_ri_a(opcode, rd.to_reg(), imm.bits));
            }
            &Inst::Mov64UImm32Shifted { rd, imm } => {
                let opcode = match imm.shift {
                    0 => 0xc0f, // LLILF
                    1 => 0xc0e, // LLIHF
                    _ => unreachable!(),
                };
                put(sink, &enc_ril_a(opcode, rd.to_reg(), imm.bits));
            }
            &Inst::Insert64UImm16Shifted { rd, ri, imm } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = match imm.shift {
                    0 => 0xa53, // IILL
                    1 => 0xa52, // IILH
                    2 => 0xa51, // IIHL
                    3 => 0xa50, // IIHH
                    _ => unreachable!(),
                };
                put(sink, &enc_ri_a(opcode, rd.to_reg(), imm.bits));
            }
            &Inst::Insert64UImm32Shifted { rd, ri, imm } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = match imm.shift {
                    0 => 0xc09, // IILF
                    1 => 0xc08, // IIHF
                    _ => unreachable!(),
                };
                put(sink, &enc_ril_a(opcode, rd.to_reg(), imm.bits));
            }
            &Inst::LoadAR { rd, ar } => {
                let opcode = 0xb24f; // EAR
                put(sink, &enc_rre(opcode, rd.to_reg(), gpr(ar)));
            }

            &Inst::InsertAR { rd, ri, ar } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = 0xb24f; // EAR
                put(sink, &enc_rre(opcode, rd.to_reg(), gpr(ar)));
            }
            &Inst::LoadSymbolReloc {
                rd,
                ref symbol_reloc,
            } => {
                let reg = writable_spilltmp_reg().to_reg();
                put(sink, &enc_ri_b(OPCODE_BRAS, reg, 12));
                let (reloc, name, offset) = match &**symbol_reloc {
                    SymbolReloc::Absolute { name, offset } => (Reloc::Abs8, name, *offset),
                    SymbolReloc::TlsGd { name } => (Reloc::S390xTlsGd64, name, 0),
                };
                sink.add_reloc(reloc, name, offset);
                sink.put8(0);
                let inst = Inst::Load64 {
                    rd,
                    mem: MemArg::reg(reg, MemFlags::trusted()),
                };
                inst.emit(sink, emit_info, state);
            }

            &Inst::FpuMove32 { rd, rn } => {
                if is_fpr(rd.to_reg()) && is_fpr(rn) {
                    let opcode = 0x38; // LER
                    put(sink, &enc_rr(opcode, rd.to_reg(), rn));
                } else {
                    put(sink, &enc_vrr_a(OPCODE_VLR, rd.to_reg(), rn, 0, 0, 0));
                }
            }
            &Inst::FpuMove64 { rd, rn } => {
                if is_fpr(rd.to_reg()) && is_fpr(rn) {
                    put(sink, &enc_rr(OPCODE_LDR, rd.to_reg(), rn));
                } else {
                    put(sink, &enc_vrr_a(OPCODE_VLR, rd.to_reg(), rn, 0, 0, 0));
                }
            }
            &Inst::FpuCMov32 { rd, cond, ri, rm } => {
                debug_assert_eq!(rd.to_reg(), ri);

                if is_fpr(rd.to_reg()) && is_fpr(rm) {
                    put(sink, &enc_ri_c(OPCODE_BCR, cond.invert().bits(), 4 + 2));
                    let opcode = 0x38; // LER
                    put(sink, &enc_rr(opcode, rd.to_reg(), rm));
                } else {
                    put(sink, &enc_ri_c(OPCODE_BCR, cond.invert().bits(), 4 + 6));
                    put(sink, &enc_vrr_a(OPCODE_VLR, rd.to_reg(), rm, 0, 0, 0));
                }
            }
            &Inst::FpuCMov64 { rd, cond, ri, rm } => {
                debug_assert_eq!(rd.to_reg(), ri);

                if is_fpr(rd.to_reg()) && is_fpr(rm) {
                    put(sink, &enc_ri_c(OPCODE_BCR, cond.invert().bits(), 4 + 2));
                    put(sink, &enc_rr(OPCODE_LDR, rd.to_reg(), rm));
                } else {
                    put(sink, &enc_ri_c(OPCODE_BCR, cond.invert().bits(), 4 + 6));
                    put(sink, &enc_vrr_a(OPCODE_VLR, rd.to_reg(), rm, 0, 0, 0));
                }
            }
            &Inst::FpuRR { fpu_op, rd, rn } => {
                let (opcode, m3, m4, m5, opcode_fpr) = match fpu_op {
                    FPUOp1::Abs32 => (0xe7cc, 2, 8, 2, Some(0xb300)), // WFPSO, LPEBR
                    FPUOp1::Abs64 => (0xe7cc, 3, 8, 2, Some(0xb310)), // WFPSO, LPDBR
                    FPUOp1::Abs128 => (0xe7cc, 4, 8, 2, None),        // WFPSO
                    FPUOp1::Abs32x4 => (0xe7cc, 2, 0, 2, None),       // VFPSO
                    FPUOp1::Abs64x2 => (0xe7cc, 3, 0, 2, None),       // VFPSO
                    FPUOp1::Neg32 => (0xe7cc, 2, 8, 0, Some(0xb303)), // WFPSO, LCEBR
                    FPUOp1::Neg64 => (0xe7cc, 3, 8, 0, Some(0xb313)), // WFPSO, LCDBR
                    FPUOp1::Neg128 => (0xe7cc, 4, 8, 0, None),        // WFPSO
                    FPUOp1::Neg32x4 => (0xe7cc, 2, 0, 0, None),       // VFPSO
                    FPUOp1::Neg64x2 => (0xe7cc, 3, 0, 0, None),       // VFPSO
                    FPUOp1::NegAbs32 => (0xe7cc, 2, 8, 1, Some(0xb301)), // WFPSO, LNEBR
                    FPUOp1::NegAbs64 => (0xe7cc, 3, 8, 1, Some(0xb311)), // WFPSO, LNDBR
                    FPUOp1::NegAbs128 => (0xe7cc, 4, 8, 1, None),     // WFPSO
                    FPUOp1::NegAbs32x4 => (0xe7cc, 2, 0, 1, None),    // VFPSO
                    FPUOp1::NegAbs64x2 => (0xe7cc, 3, 0, 1, None),    // VFPSO
                    FPUOp1::Sqrt32 => (0xe7ce, 2, 8, 0, Some(0xb314)), // WFSQ, SQEBR
                    FPUOp1::Sqrt64 => (0xe7ce, 3, 8, 0, Some(0xb315)), // WFSQ, SQDBR
                    FPUOp1::Sqrt128 => (0xe7ce, 4, 8, 0, None),       // WFSQ
                    FPUOp1::Sqrt32x4 => (0xe7ce, 2, 0, 0, None),      // VFSQ
                    FPUOp1::Sqrt64x2 => (0xe7ce, 3, 0, 0, None),      // VFSQ
                    FPUOp1::Cvt32To64 => (0xe7c4, 2, 8, 0, Some(0xb304)), // WFLL, LDEBR
                    FPUOp1::Cvt32x4To64x2 => (0xe7c4, 2, 0, 0, None), // VFLL
                    FPUOp1::Cvt64To128 => (0xe7c4, 3, 8, 0, None),    // WFLL
                };
                if m4 == 8 && opcode_fpr.is_some() && is_fpr(rd.to_reg()) && is_fpr(rn) {
                    put(sink, &enc_rre(opcode_fpr.unwrap(), rd.to_reg(), rn));
                } else {
                    put(sink, &enc_vrr_a(opcode, rd.to_reg(), rn, m3, m4, m5));
                }
            }
            &Inst::FpuRRR { fpu_op, rd, rn, rm } => {
                let (opcode, m4, m5, m6, opcode_fpr) = match fpu_op {
                    FPUOp2::Add32 => (0xe7e3, 2, 8, 0, Some(0xb30a)), // WFA, AEBR
                    FPUOp2::Add64 => (0xe7e3, 3, 8, 0, Some(0xb31a)), // WFA, ADBR
                    FPUOp2::Add128 => (0xe7e3, 4, 8, 0, None),        // WFA
                    FPUOp2::Add32x4 => (0xe7e3, 2, 0, 0, None),       // VFA
                    FPUOp2::Add64x2 => (0xe7e3, 3, 0, 0, None),       // VFA
                    FPUOp2::Sub32 => (0xe7e2, 2, 8, 0, Some(0xb30b)), // WFS, SEBR
                    FPUOp2::Sub64 => (0xe7e2, 3, 8, 0, Some(0xb31b)), // WFS, SDBR
                    FPUOp2::Sub128 => (0xe7e2, 4, 8, 0, None),        // WFS
                    FPUOp2::Sub32x4 => (0xe7e2, 2, 0, 0, None),       // VFS
                    FPUOp2::Sub64x2 => (0xe7e2, 3, 0, 0, None),       // VFS
                    FPUOp2::Mul32 => (0xe7e7, 2, 8, 0, Some(0xb317)), // WFM, MEEBR
                    FPUOp2::Mul64 => (0xe7e7, 3, 8, 0, Some(0xb31c)), // WFM, MDBR
                    FPUOp2::Mul128 => (0xe7e7, 4, 8, 0, None),        // WFM
                    FPUOp2::Mul32x4 => (0xe7e7, 2, 0, 0, None),       // VFM
                    FPUOp2::Mul64x2 => (0xe7e7, 3, 0, 0, None),       // VFM
                    FPUOp2::Div32 => (0xe7e5, 2, 8, 0, Some(0xb30d)), // WFD, DEBR
                    FPUOp2::Div64 => (0xe7e5, 3, 8, 0, Some(0xb31d)), // WFD, DDBR
                    FPUOp2::Div128 => (0xe7e5, 4, 8, 0, None),        // WFD
                    FPUOp2::Div32x4 => (0xe7e5, 2, 0, 0, None),       // VFD
                    FPUOp2::Div64x2 => (0xe7e5, 3, 0, 0, None),       // VFD
                    FPUOp2::Max32 => (0xe7ef, 2, 8, 1, None),         // WFMAX
                    FPUOp2::Max64 => (0xe7ef, 3, 8, 1, None),         // WFMAX
                    FPUOp2::Max128 => (0xe7ef, 4, 8, 1, None),        // WFMAX
                    FPUOp2::Max32x4 => (0xe7ef, 2, 0, 1, None),       // VFMAX
                    FPUOp2::Max64x2 => (0xe7ef, 3, 0, 1, None),       // VFMAX
                    FPUOp2::Min32 => (0xe7ee, 2, 8, 1, None),         // WFMIN
                    FPUOp2::Min64 => (0xe7ee, 3, 8, 1, None),         // WFMIN
                    FPUOp2::Min128 => (0xe7ee, 4, 8, 1, None),        // WFMIN
                    FPUOp2::Min32x4 => (0xe7ee, 2, 0, 1, None),       // VFMIN
                    FPUOp2::Min64x2 => (0xe7ee, 3, 0, 1, None),       // VFMIN
                    FPUOp2::MaxPseudo32 => (0xe7ef, 2, 8, 3, None),   // WFMAX
                    FPUOp2::MaxPseudo64 => (0xe7ef, 3, 8, 3, None),   // WFMAX
                    FPUOp2::MaxPseudo128 => (0xe7ef, 4, 8, 3, None),  // WFMAX
                    FPUOp2::MaxPseudo32x4 => (0xe7ef, 2, 0, 3, None), // VFMAX
                    FPUOp2::MaxPseudo64x2 => (0xe7ef, 3, 0, 3, None), // VFMAX
                    FPUOp2::MinPseudo32 => (0xe7ee, 2, 8, 3, None),   // WFMIN
                    FPUOp2::MinPseudo64 => (0xe7ee, 3, 8, 3, None),   // WFMIN
                    FPUOp2::MinPseudo128 => (0xe7ee, 4, 8, 3, None),  // WFMIN
                    FPUOp2::MinPseudo32x4 => (0xe7ee, 2, 0, 3, None), // VFMIN
                    FPUOp2::MinPseudo64x2 => (0xe7ee, 3, 0, 3, None), // VFMIN
                };
                if m5 == 8 && opcode_fpr.is_some() && rd.to_reg() == rn && is_fpr(rn) && is_fpr(rm)
                {
                    put(sink, &enc_rre(opcode_fpr.unwrap(), rd.to_reg(), rm));
                } else {
                    put(sink, &enc_vrr_c(opcode, rd.to_reg(), rn, rm, m4, m5, m6));
                }
            }
            &Inst::FpuRRRR {
                fpu_op,
                rd,
                rn,
                rm,
                ra,
            } => {
                let (opcode, m5, m6, opcode_fpr) = match fpu_op {
                    FPUOp3::MAdd32 => (0xe78f, 8, 2, Some(0xb30e)), // WFMA, MAEBR
                    FPUOp3::MAdd64 => (0xe78f, 8, 3, Some(0xb31e)), // WFMA, MADBR
                    FPUOp3::MAdd128 => (0xe78f, 8, 4, None),        // WFMA
                    FPUOp3::MAdd32x4 => (0xe78f, 0, 2, None),       // VFMA
                    FPUOp3::MAdd64x2 => (0xe78f, 0, 3, None),       // VFMA
                    FPUOp3::MSub32 => (0xe78e, 8, 2, Some(0xb30f)), // WFMS, MSEBR
                    FPUOp3::MSub64 => (0xe78e, 8, 3, Some(0xb31f)), // WFMS, MSDBR
                    FPUOp3::MSub128 => (0xe78e, 8, 4, None),        // WFMS
                    FPUOp3::MSub32x4 => (0xe78e, 0, 2, None),       // VFMS
                    FPUOp3::MSub64x2 => (0xe78e, 0, 3, None),       // VFMS
                };
                if m5 == 8
                    && opcode_fpr.is_some()
                    && rd.to_reg() == ra
                    && is_fpr(rn)
                    && is_fpr(rm)
                    && is_fpr(ra)
                {
                    put(sink, &enc_rrd(opcode_fpr.unwrap(), rd.to_reg(), rm, rn));
                } else {
                    put(sink, &enc_vrr_e(opcode, rd.to_reg(), rn, rm, ra, m5, m6));
                }
            }
            &Inst::FpuRound { op, mode, rd, rn } => {
                let mode = match mode {
                    FpuRoundMode::Current => 0,
                    FpuRoundMode::ToNearest => 1,
                    FpuRoundMode::ShorterPrecision => 3,
                    FpuRoundMode::ToNearestTiesToEven => 4,
                    FpuRoundMode::ToZero => 5,
                    FpuRoundMode::ToPosInfinity => 6,
                    FpuRoundMode::ToNegInfinity => 7,
                };
                let (opcode, m3, m4, opcode_fpr) = match op {
                    FpuRoundOp::Cvt64To32 => (0xe7c5, 3, 8, Some(0xb344)), // WFLR, LEDBR(A)
                    FpuRoundOp::Cvt64x2To32x4 => (0xe7c5, 3, 0, None),     // VFLR
                    FpuRoundOp::Cvt128To64 => (0xe7c5, 4, 8, None),        // WFLR
                    FpuRoundOp::Round32 => (0xe7c7, 2, 8, Some(0xb357)),   // WFI, FIEBR
                    FpuRoundOp::Round64 => (0xe7c7, 3, 8, Some(0xb35f)),   // WFI, FIDBR
                    FpuRoundOp::Round128 => (0xe7c7, 4, 8, None),          // WFI
                    FpuRoundOp::Round32x4 => (0xe7c7, 2, 0, None),         // VFI
                    FpuRoundOp::Round64x2 => (0xe7c7, 3, 0, None),         // VFI
                    FpuRoundOp::ToSInt32 => (0xe7c2, 2, 8, None),          // WCSFP
                    FpuRoundOp::ToSInt64 => (0xe7c2, 3, 8, None),          // WCSFP
                    FpuRoundOp::ToUInt32 => (0xe7c0, 2, 8, None),          // WCLFP
                    FpuRoundOp::ToUInt64 => (0xe7c0, 3, 8, None),          // WCLFP
                    FpuRoundOp::ToSInt32x4 => (0xe7c2, 2, 0, None),        // VCSFP
                    FpuRoundOp::ToSInt64x2 => (0xe7c2, 3, 0, None),        // VCSFP
                    FpuRoundOp::ToUInt32x4 => (0xe7c0, 2, 0, None),        // VCLFP
                    FpuRoundOp::ToUInt64x2 => (0xe7c0, 3, 0, None),        // VCLFP
                    FpuRoundOp::FromSInt32 => (0xe7c3, 2, 8, None),        // WCFPS
                    FpuRoundOp::FromSInt64 => (0xe7c3, 3, 8, None),        // WCFPS
                    FpuRoundOp::FromUInt32 => (0xe7c1, 2, 8, None),        // WCFPL
                    FpuRoundOp::FromUInt64 => (0xe7c1, 3, 8, None),        // WCFPL
                    FpuRoundOp::FromSInt32x4 => (0xe7c3, 2, 0, None),      // VCFPS
                    FpuRoundOp::FromSInt64x2 => (0xe7c3, 3, 0, None),      // VCFPS
                    FpuRoundOp::FromUInt32x4 => (0xe7c1, 2, 0, None),      // VCFPL
                    FpuRoundOp::FromUInt64x2 => (0xe7c1, 3, 0, None),      // VCFPL
                };
                if m4 == 8 && opcode_fpr.is_some() && is_fpr(rd.to_reg()) && is_fpr(rn) {
                    put(
                        sink,
                        &enc_rrf_cde(opcode_fpr.unwrap(), rd.to_reg(), rn, mode, 0),
                    );
                } else {
                    put(sink, &enc_vrr_a(opcode, rd.to_reg(), rn, m3, m4, mode));
                }
            }
            &Inst::FpuConv128FromInt { op, mode, rd, rn } => {
                let rd1 = rd.hi;
                let rd2 = rd.lo;
                debug_assert_valid_fp_regpair!(rd1.to_reg(), rd2.to_reg());

                let mode = match mode {
                    FpuRoundMode::Current => 0,
                    FpuRoundMode::ToNearest => 1,
                    FpuRoundMode::ShorterPrecision => 3,
                    FpuRoundMode::ToNearestTiesToEven => 4,
                    FpuRoundMode::ToZero => 5,
                    FpuRoundMode::ToPosInfinity => 6,
                    FpuRoundMode::ToNegInfinity => 7,
                };
                let opcode = match op {
                    FpuConv128Op::SInt32 => 0xb396, // CXFBRA
                    FpuConv128Op::SInt64 => 0xb3a6, // CXGBRA
                    FpuConv128Op::UInt32 => 0xb392, // CXLFBR
                    FpuConv128Op::UInt64 => 0xb3a2, // CXLGBR
                };
                put(sink, &enc_rrf_cde(opcode, rd1.to_reg(), rn, mode, 0));
            }
            &Inst::FpuConv128ToInt { op, mode, rd, rn } => {
                let rn1 = rn.hi;
                let rn2 = rn.lo;
                debug_assert_valid_fp_regpair!(rn1, rn2);

                let mode = match mode {
                    FpuRoundMode::Current => 0,
                    FpuRoundMode::ToNearest => 1,
                    FpuRoundMode::ShorterPrecision => 3,
                    FpuRoundMode::ToNearestTiesToEven => 4,
                    FpuRoundMode::ToZero => 5,
                    FpuRoundMode::ToPosInfinity => 6,
                    FpuRoundMode::ToNegInfinity => 7,
                };
                let opcode = match op {
                    FpuConv128Op::SInt32 => 0xb39a, // CFXBRA
                    FpuConv128Op::SInt64 => 0xb3aa, // CGXBRA
                    FpuConv128Op::UInt32 => 0xb39e, // CLFXBR
                    FpuConv128Op::UInt64 => 0xb3ae, // CLGXBR
                };
                put(sink, &enc_rrf_cde(opcode, rd.to_reg(), rn1, mode, 0));
            }
            &Inst::FpuCmp32 { rn, rm } => {
                if is_fpr(rn) && is_fpr(rm) {
                    let opcode = 0xb309; // CEBR
                    put(sink, &enc_rre(opcode, rn, rm));
                } else {
                    let opcode = 0xe7cb; // WFC
                    put(sink, &enc_vrr_a(opcode, rn, rm, 2, 0, 0));
                }
            }
            &Inst::FpuCmp64 { rn, rm } => {
                if is_fpr(rn) && is_fpr(rm) {
                    let opcode = 0xb319; // CDBR
                    put(sink, &enc_rre(opcode, rn, rm));
                } else {
                    let opcode = 0xe7cb; // WFC
                    put(sink, &enc_vrr_a(opcode, rn, rm, 3, 0, 0));
                }
            }
            &Inst::FpuCmp128 { rn, rm } => {
                let opcode = 0xe7cb; // WFC
                put(sink, &enc_vrr_a(opcode, rn, rm, 4, 0, 0));
            }

            &Inst::VecRRR { op, rd, rn, rm } => {
                let (opcode, m4) = match op {
                    VecBinaryOp::Add8x16 => (0xe7f3, 0),       // VAB
                    VecBinaryOp::Add16x8 => (0xe7f3, 1),       // VAH
                    VecBinaryOp::Add32x4 => (0xe7f3, 2),       // VAF
                    VecBinaryOp::Add64x2 => (0xe7f3, 3),       // VAG
                    VecBinaryOp::Add128 => (0xe7f3, 4),        // VAQ
                    VecBinaryOp::Sub8x16 => (0xe7f7, 0),       // VSB
                    VecBinaryOp::Sub16x8 => (0xe7f7, 1),       // VSH
                    VecBinaryOp::Sub32x4 => (0xe7f7, 2),       // VSF
                    VecBinaryOp::Sub64x2 => (0xe7f7, 3),       // VSG
                    VecBinaryOp::Sub128 => (0xe7f7, 4),        // VSQ
                    VecBinaryOp::Mul8x16 => (0xe7a2, 0),       // VMLB
                    VecBinaryOp::Mul16x8 => (0xe7a2, 1),       // VMLHW
                    VecBinaryOp::Mul32x4 => (0xe7a2, 2),       // VMLF
                    VecBinaryOp::UMulHi8x16 => (0xe7a1, 0),    // VMLHB
                    VecBinaryOp::UMulHi16x8 => (0xe7a1, 1),    // VMLHH
                    VecBinaryOp::UMulHi32x4 => (0xe7a1, 2),    // VMLHF
                    VecBinaryOp::SMulHi8x16 => (0xe7a3, 0),    // VMHB
                    VecBinaryOp::SMulHi16x8 => (0xe7a3, 1),    // VMHH
                    VecBinaryOp::SMulHi32x4 => (0xe7a3, 2),    // VMHF
                    VecBinaryOp::UMulEven8x16 => (0xe7a4, 0),  // VMLEB
                    VecBinaryOp::UMulEven16x8 => (0xe7a4, 1),  // VMLEH
                    VecBinaryOp::UMulEven32x4 => (0xe7a4, 2),  // VMLEF
                    VecBinaryOp::SMulEven8x16 => (0xe7a6, 0),  // VMEB
                    VecBinaryOp::SMulEven16x8 => (0xe7a6, 1),  // VMEH
                    VecBinaryOp::SMulEven32x4 => (0xe7a6, 2),  // VMEF
                    VecBinaryOp::UMulOdd8x16 => (0xe7a5, 0),   // VMLOB
                    VecBinaryOp::UMulOdd16x8 => (0xe7a5, 1),   // VMLOH
                    VecBinaryOp::UMulOdd32x4 => (0xe7a5, 2),   // VMLOF
                    VecBinaryOp::SMulOdd8x16 => (0xe7a7, 0),   // VMOB
                    VecBinaryOp::SMulOdd16x8 => (0xe7a7, 1),   // VMOH
                    VecBinaryOp::SMulOdd32x4 => (0xe7a7, 2),   // VMOF
                    VecBinaryOp::UMax8x16 => (0xe7fd, 0),      // VMXLB
                    VecBinaryOp::UMax16x8 => (0xe7fd, 1),      // VMXLH
                    VecBinaryOp::UMax32x4 => (0xe7fd, 2),      // VMXLF
                    VecBinaryOp::UMax64x2 => (0xe7fd, 3),      // VMXLG
                    VecBinaryOp::SMax8x16 => (0xe7ff, 0),      // VMXB
                    VecBinaryOp::SMax16x8 => (0xe7ff, 1),      // VMXH
                    VecBinaryOp::SMax32x4 => (0xe7ff, 2),      // VMXF
                    VecBinaryOp::SMax64x2 => (0xe7ff, 3),      // VMXG
                    VecBinaryOp::UMin8x16 => (0xe7fc, 0),      // VMNLB
                    VecBinaryOp::UMin16x8 => (0xe7fc, 1),      // VMNLH
                    VecBinaryOp::UMin32x4 => (0xe7fc, 2),      // VMNLF
                    VecBinaryOp::UMin64x2 => (0xe7fc, 3),      // VMNLG
                    VecBinaryOp::SMin8x16 => (0xe7fe, 0),      // VMNB
                    VecBinaryOp::SMin16x8 => (0xe7fe, 1),      // VMNH
                    VecBinaryOp::SMin32x4 => (0xe7fe, 2),      // VMNF
                    VecBinaryOp::SMin64x2 => (0xe7fe, 3),      // VMNG
                    VecBinaryOp::UAvg8x16 => (0xe7f0, 0),      // VAVGLB
                    VecBinaryOp::UAvg16x8 => (0xe7f0, 1),      // VAVGLH
                    VecBinaryOp::UAvg32x4 => (0xe7f0, 2),      // VAVGLF
                    VecBinaryOp::UAvg64x2 => (0xe7f0, 3),      // VAVGLG
                    VecBinaryOp::SAvg8x16 => (0xe7f2, 0),      // VAVGB
                    VecBinaryOp::SAvg16x8 => (0xe7f2, 1),      // VAVGH
                    VecBinaryOp::SAvg32x4 => (0xe7f2, 2),      // VAVGF
                    VecBinaryOp::SAvg64x2 => (0xe7f2, 3),      // VAVGG
                    VecBinaryOp::And128 => (0xe768, 0),        // VN
                    VecBinaryOp::Orr128 => (0xe76a, 0),        // VO
                    VecBinaryOp::Xor128 => (0xe76d, 0),        // VX
                    VecBinaryOp::NotAnd128 => (0xe76e, 0),     // VNN
                    VecBinaryOp::NotOrr128 => (0xe76b, 0),     // VNO
                    VecBinaryOp::NotXor128 => (0xe76c, 0),     // VNX
                    VecBinaryOp::AndNot128 => (0xe769, 0),     // VNC
                    VecBinaryOp::OrrNot128 => (0xe76f, 0),     // VOC
                    VecBinaryOp::BitPermute128 => (0xe785, 0), // VBPERM
                    VecBinaryOp::LShLByByte128 => (0xe775, 0), // VSLB
                    VecBinaryOp::LShRByByte128 => (0xe77d, 0), // VSRLB
                    VecBinaryOp::AShRByByte128 => (0xe77f, 0), // VSRAB
                    VecBinaryOp::LShLByBit128 => (0xe774, 0),  // VSL
                    VecBinaryOp::LShRByBit128 => (0xe77c, 0),  // VSRL
                    VecBinaryOp::AShRByBit128 => (0xe77e, 0),  // VSRA
                    VecBinaryOp::Pack16x8 => (0xe794, 1),      // VPKH
                    VecBinaryOp::Pack32x4 => (0xe794, 2),      // VPKF
                    VecBinaryOp::Pack64x2 => (0xe794, 3),      // VPKG
                    VecBinaryOp::PackUSat16x8 => (0xe795, 1),  // VPKLSH
                    VecBinaryOp::PackUSat32x4 => (0xe795, 2),  // VPKLSF
                    VecBinaryOp::PackUSat64x2 => (0xe795, 3),  // VPKLSG
                    VecBinaryOp::PackSSat16x8 => (0xe797, 1),  // VPKSH
                    VecBinaryOp::PackSSat32x4 => (0xe797, 2),  // VPKSF
                    VecBinaryOp::PackSSat64x2 => (0xe797, 3),  // VPKSG
                    VecBinaryOp::MergeLow8x16 => (0xe760, 0),  // VMRLB
                    VecBinaryOp::MergeLow16x8 => (0xe760, 1),  // VMRLH
                    VecBinaryOp::MergeLow32x4 => (0xe760, 2),  // VMRLF
                    VecBinaryOp::MergeLow64x2 => (0xe760, 3),  // VMRLG
                    VecBinaryOp::MergeHigh8x16 => (0xe761, 0), // VMRHB
                    VecBinaryOp::MergeHigh16x8 => (0xe761, 1), // VMRHH
                    VecBinaryOp::MergeHigh32x4 => (0xe761, 2), // VMRHF
                    VecBinaryOp::MergeHigh64x2 => (0xe761, 3), // VMRHG
                };

                put(sink, &enc_vrr_c(opcode, rd.to_reg(), rn, rm, m4, 0, 0));
            }
            &Inst::VecRR { op, rd, rn } => {
                let (opcode, m3) = match op {
                    VecUnaryOp::Abs8x16 => (0xe7df, 0),         // VLPB
                    VecUnaryOp::Abs16x8 => (0xe7df, 1),         // VLPH
                    VecUnaryOp::Abs32x4 => (0xe7df, 2),         // VLPF
                    VecUnaryOp::Abs64x2 => (0xe7df, 3),         // VLPG
                    VecUnaryOp::Neg8x16 => (0xe7de, 0),         // VLCB
                    VecUnaryOp::Neg16x8 => (0xe7de, 1),         // VLCH
                    VecUnaryOp::Neg32x4 => (0xe7de, 2),         // VLCF
                    VecUnaryOp::Neg64x2 => (0xe7de, 3),         // VLCG
                    VecUnaryOp::Popcnt8x16 => (0xe750, 0),      // VPOPCTB
                    VecUnaryOp::Popcnt16x8 => (0xe750, 1),      // VPOPCTH
                    VecUnaryOp::Popcnt32x4 => (0xe750, 2),      // VPOPCTF
                    VecUnaryOp::Popcnt64x2 => (0xe750, 3),      // VPOPCTG
                    VecUnaryOp::Clz8x16 => (0xe753, 0),         // VCLZB
                    VecUnaryOp::Clz16x8 => (0xe753, 1),         // VCLZH
                    VecUnaryOp::Clz32x4 => (0xe753, 2),         // VCLZF
                    VecUnaryOp::Clz64x2 => (0xe753, 3),         // VCLZG
                    VecUnaryOp::Ctz8x16 => (0xe752, 0),         // VCTZB
                    VecUnaryOp::Ctz16x8 => (0xe752, 1),         // VCTZH
                    VecUnaryOp::Ctz32x4 => (0xe752, 2),         // VCTZF
                    VecUnaryOp::Ctz64x2 => (0xe752, 3),         // VCTZG
                    VecUnaryOp::UnpackULow8x16 => (0xe7d4, 0),  // VUPLLB
                    VecUnaryOp::UnpackULow16x8 => (0xe7d4, 1),  // VUPLLH
                    VecUnaryOp::UnpackULow32x4 => (0xe7d4, 2),  // VUPLLF
                    VecUnaryOp::UnpackUHigh8x16 => (0xe7d5, 0), // VUPLHB
                    VecUnaryOp::UnpackUHigh16x8 => (0xe7d5, 1), // VUPLHH
                    VecUnaryOp::UnpackUHigh32x4 => (0xe7d5, 2), // VUPLHF
                    VecUnaryOp::UnpackSLow8x16 => (0xe7d6, 0),  // VUPLB
                    VecUnaryOp::UnpackSLow16x8 => (0xe7d6, 1),  // VUPLH
                    VecUnaryOp::UnpackSLow32x4 => (0xe7d6, 2),  // VUPLF
                    VecUnaryOp::UnpackSHigh8x16 => (0xe7d7, 0), // VUPHB
                    VecUnaryOp::UnpackSHigh16x8 => (0xe7d7, 1), // VUPHH
                    VecUnaryOp::UnpackSHigh32x4 => (0xe7d7, 2), // VUPHF
                };

                put(sink, &enc_vrr_a(opcode, rd.to_reg(), rn, m3, 0, 0));
            }
            &Inst::VecShiftRR {
                shift_op,
                rd,
                rn,
                shift_imm,
                shift_reg,
            } => {
                let (opcode, m4) = match shift_op {
                    VecShiftOp::RotL8x16 => (0xe733, 0), // VERLLB
                    VecShiftOp::RotL16x8 => (0xe733, 1), // VERLLH
                    VecShiftOp::RotL32x4 => (0xe733, 2), // VERLLF
                    VecShiftOp::RotL64x2 => (0xe733, 3), // VERLLG
                    VecShiftOp::LShL8x16 => (0xe730, 0), // VESLB
                    VecShiftOp::LShL16x8 => (0xe730, 1), // VESLH
                    VecShiftOp::LShL32x4 => (0xe730, 2), // VESLF
                    VecShiftOp::LShL64x2 => (0xe730, 3), // VESLG
                    VecShiftOp::LShR8x16 => (0xe738, 0), // VESRLB
                    VecShiftOp::LShR16x8 => (0xe738, 1), // VESRLH
                    VecShiftOp::LShR32x4 => (0xe738, 2), // VESRLF
                    VecShiftOp::LShR64x2 => (0xe738, 3), // VESRLG
                    VecShiftOp::AShR8x16 => (0xe73a, 0), // VESRAB
                    VecShiftOp::AShR16x8 => (0xe73a, 1), // VESRAH
                    VecShiftOp::AShR32x4 => (0xe73a, 2), // VESRAF
                    VecShiftOp::AShR64x2 => (0xe73a, 3), // VESRAG
                };
                put(
                    sink,
                    &enc_vrs_a(opcode, rd.to_reg(), shift_reg, shift_imm.into(), rn, m4),
                );
            }
            &Inst::VecSelect { rd, rn, rm, ra } => {
                let opcode = 0xe78d; // VSEL
                put(sink, &enc_vrr_e(opcode, rd.to_reg(), rn, rm, ra, 0, 0));
            }
            &Inst::VecPermute { rd, rn, rm, ra } => {
                let opcode = 0xe78c; // VPERM
                put(sink, &enc_vrr_e(opcode, rd.to_reg(), rn, rm, ra, 0, 0));
            }
            &Inst::VecPermuteDWImm {
                rd,
                rn,
                rm,
                idx1,
                idx2,
            } => {
                let m4 = (idx1 & 1) * 4 + (idx2 & 1);

                let opcode = 0xe784; // VPDI
                put(sink, &enc_vrr_c(opcode, rd.to_reg(), rn, rm, m4, 0, 0));
            }
            &Inst::VecIntCmp { op, rd, rn, rm } | &Inst::VecIntCmpS { op, rd, rn, rm } => {
                let (opcode, m4) = match op {
                    VecIntCmpOp::CmpEq8x16 => (0xe7f8, 0),  // VCEQB
                    VecIntCmpOp::CmpEq16x8 => (0xe7f8, 1),  // VCEQH
                    VecIntCmpOp::CmpEq32x4 => (0xe7f8, 2),  // VCEQF
                    VecIntCmpOp::CmpEq64x2 => (0xe7f8, 3),  // VCEQG
                    VecIntCmpOp::SCmpHi8x16 => (0xe7fb, 0), // VCHB
                    VecIntCmpOp::SCmpHi16x8 => (0xe7fb, 1), // VCHH
                    VecIntCmpOp::SCmpHi32x4 => (0xe7fb, 2), // VCHG
                    VecIntCmpOp::SCmpHi64x2 => (0xe7fb, 3), // VCHG
                    VecIntCmpOp::UCmpHi8x16 => (0xe7f9, 0), // VCHLB
                    VecIntCmpOp::UCmpHi16x8 => (0xe7f9, 1), // VCHLH
                    VecIntCmpOp::UCmpHi32x4 => (0xe7f9, 2), // VCHLG
                    VecIntCmpOp::UCmpHi64x2 => (0xe7f9, 3), // VCHLG
                };
                let m5 = match self {
                    &Inst::VecIntCmp { .. } => 0,
                    &Inst::VecIntCmpS { .. } => 1,
                    _ => unreachable!(),
                };

                put(sink, &enc_vrr_b(opcode, rd.to_reg(), rn, rm, m4, m5));
            }
            &Inst::VecFloatCmp { op, rd, rn, rm } | &Inst::VecFloatCmpS { op, rd, rn, rm } => {
                let (opcode, m4) = match op {
                    VecFloatCmpOp::CmpEq32x4 => (0xe7e8, 2),   // VFCESB
                    VecFloatCmpOp::CmpEq64x2 => (0xe7e8, 3),   // VFCEDB
                    VecFloatCmpOp::CmpHi32x4 => (0xe7eb, 2),   // VFCHSB
                    VecFloatCmpOp::CmpHi64x2 => (0xe7eb, 3),   // VFCHDB
                    VecFloatCmpOp::CmpHiEq32x4 => (0xe7ea, 2), // VFCHESB
                    VecFloatCmpOp::CmpHiEq64x2 => (0xe7ea, 3), // VFCHEDB
                };
                let m6 = match self {
                    &Inst::VecFloatCmp { .. } => 0,
                    &Inst::VecFloatCmpS { .. } => 1,
                    _ => unreachable!(),
                };

                put(sink, &enc_vrr_c(opcode, rd.to_reg(), rn, rm, m4, 0, m6));
            }
            &Inst::VecInt128SCmpHi { tmp, rn, rm } | &Inst::VecInt128UCmpHi { tmp, rn, rm } => {
                // Synthetic instruction to compare 128-bit values.
                // Sets CC 1 if rn > rm, sets a different CC otherwise.

                // Use VECTOR ELEMENT COMPARE to compare the high parts.
                // Swap the inputs to get:
                //    CC 1 if high(rn) > high(rm)
                //    CC 2 if high(rn) < high(rm)
                //    CC 0 if high(rn) == high(rm)
                let (opcode, m3) = match self {
                    &Inst::VecInt128SCmpHi { .. } => (0xe7db, 3), // VECG
                    &Inst::VecInt128UCmpHi { .. } => (0xe7d9, 3), // VECLG
                    _ => unreachable!(),
                };
                put(sink, &enc_vrr_a(opcode, rm, rn, m3, 0, 0));

                // If CC != 0, we'd done, so jump over the next instruction.
                put(sink, &enc_ri_c(OPCODE_BCR, 7, 4 + 6));

                // Otherwise, use VECTOR COMPARE HIGH LOGICAL.
                // Since we already know the high parts are equal, the CC
                // result will only depend on the low parts:
                //     CC 1 if low(rn) > low(rm)
                //     CC 3 if low(rn) <= low(rm)
                let inst = Inst::VecIntCmpS {
                    op: VecIntCmpOp::UCmpHi64x2,
                    // N.B.: This is the first write to tmp, and it happens
                    // after all uses of rn and rm.  If this were to ever
                    // change, tmp would have to become an early-def.
                    rd: tmp,
                    rn,
                    rm,
                };
                inst.emit(sink, emit_info, state);
            }

            &Inst::VecLoad { rd, ref mem }
            | &Inst::VecLoadRev { rd, ref mem }
            | &Inst::VecLoadByte16Rev { rd, ref mem }
            | &Inst::VecLoadByte32Rev { rd, ref mem }
            | &Inst::VecLoadByte64Rev { rd, ref mem }
            | &Inst::VecLoadElt16Rev { rd, ref mem }
            | &Inst::VecLoadElt32Rev { rd, ref mem }
            | &Inst::VecLoadElt64Rev { rd, ref mem } => {
                let mem = mem.clone();

                let (opcode, m3) = match self {
                    &Inst::VecLoad { .. } => (0xe706, 0),          // VL
                    &Inst::VecLoadRev { .. } => (0xe606, 4),       // VLBRQ
                    &Inst::VecLoadByte16Rev { .. } => (0xe606, 1), // VLBRH
                    &Inst::VecLoadByte32Rev { .. } => (0xe606, 2), // VLBRF
                    &Inst::VecLoadByte64Rev { .. } => (0xe606, 3), // VLBRG
                    &Inst::VecLoadElt16Rev { .. } => (0xe607, 1),  // VLERH
                    &Inst::VecLoadElt32Rev { .. } => (0xe607, 2),  // VLERF
                    &Inst::VecLoadElt64Rev { .. } => (0xe607, 3),  // VLERG
                    _ => unreachable!(),
                };
                mem_vrx_emit(rd.to_reg(), &mem, opcode, m3, true, sink, emit_info, state);
            }
            &Inst::VecStore { rd, ref mem }
            | &Inst::VecStoreRev { rd, ref mem }
            | &Inst::VecStoreByte16Rev { rd, ref mem }
            | &Inst::VecStoreByte32Rev { rd, ref mem }
            | &Inst::VecStoreByte64Rev { rd, ref mem }
            | &Inst::VecStoreElt16Rev { rd, ref mem }
            | &Inst::VecStoreElt32Rev { rd, ref mem }
            | &Inst::VecStoreElt64Rev { rd, ref mem } => {
                let mem = mem.clone();

                let (opcode, m3) = match self {
                    &Inst::VecStore { .. } => (0xe70e, 0),          // VST
                    &Inst::VecStoreRev { .. } => (0xe60e, 4),       // VSTBRQ
                    &Inst::VecStoreByte16Rev { .. } => (0xe60e, 1), // VSTBRH
                    &Inst::VecStoreByte32Rev { .. } => (0xe60e, 2), // VSTBRF
                    &Inst::VecStoreByte64Rev { .. } => (0xe60e, 3), // VSTBRG
                    &Inst::VecStoreElt16Rev { .. } => (0xe60f, 1),  // VSTERH
                    &Inst::VecStoreElt32Rev { .. } => (0xe60f, 2),  // VSTERF
                    &Inst::VecStoreElt64Rev { .. } => (0xe60f, 3),  // VSTERG
                    _ => unreachable!(),
                };
                mem_vrx_emit(rd, &mem, opcode, m3, true, sink, emit_info, state);
            }
            &Inst::VecLoadReplicate { size, rd, ref mem }
            | &Inst::VecLoadReplicateRev { size, rd, ref mem } => {
                let mem = mem.clone();

                let (opcode, m3) = match (self, size) {
                    (&Inst::VecLoadReplicate { .. }, 8) => (0xe705, 0), // VLREPB
                    (&Inst::VecLoadReplicate { .. }, 16) => (0xe705, 1), // VLREPH
                    (&Inst::VecLoadReplicate { .. }, 32) => (0xe705, 2), // VLREPF
                    (&Inst::VecLoadReplicate { .. }, 64) => (0xe705, 3), // VLREPG
                    (&Inst::VecLoadReplicateRev { .. }, 16) => (0xe605, 1), // VLREPBRH
                    (&Inst::VecLoadReplicateRev { .. }, 32) => (0xe605, 2), // VLREPBRF
                    (&Inst::VecLoadReplicateRev { .. }, 64) => (0xe605, 3), // VLREPBRG
                    _ => unreachable!(),
                };
                mem_vrx_emit(rd.to_reg(), &mem, opcode, m3, true, sink, emit_info, state);
            }

            &Inst::VecMov { rd, rn } => {
                put(sink, &enc_vrr_a(OPCODE_VLR, rd.to_reg(), rn, 0, 0, 0));
            }
            &Inst::VecCMov { rd, cond, ri, rm } => {
                debug_assert_eq!(rd.to_reg(), ri);

                put(sink, &enc_ri_c(OPCODE_BCR, cond.invert().bits(), 4 + 6));
                put(sink, &enc_vrr_a(OPCODE_VLR, rd.to_reg(), rm, 0, 0, 0));
            }
            &Inst::MovToVec128 { rd, rn, rm } => {
                let opcode = 0xe762; // VLVGP
                put(sink, &enc_vrr_f(opcode, rd.to_reg(), rn, rm));
            }
            &Inst::VecImmByteMask { rd, mask } => {
                let opcode = 0xe744; // VGBM
                put(sink, &enc_vri_a(opcode, rd.to_reg(), mask, 0));
            }
            &Inst::VecImmBitMask {
                size,
                rd,
                start_bit,
                end_bit,
            } => {
                let (opcode, m4) = match size {
                    8 => (0xe746, 0),  // VGMB
                    16 => (0xe746, 1), // VGMH
                    32 => (0xe746, 2), // VGMF
                    64 => (0xe746, 3), // VGMG
                    _ => unreachable!(),
                };
                put(
                    sink,
                    &enc_vri_b(opcode, rd.to_reg(), start_bit, end_bit, m4),
                );
            }
            &Inst::VecImmReplicate { size, rd, imm } => {
                let (opcode, m3) = match size {
                    8 => (0xe745, 0),  // VREPIB
                    16 => (0xe745, 1), // VREPIH
                    32 => (0xe745, 2), // VREPIF
                    64 => (0xe745, 3), // VREPIG
                    _ => unreachable!(),
                };
                put(sink, &enc_vri_a(opcode, rd.to_reg(), imm as u16, m3));
            }
            &Inst::VecLoadLane {
                size,
                rd,
                ri,
                ref mem,
                lane_imm,
            }
            | &Inst::VecLoadLaneRev {
                size,
                rd,
                ri,
                ref mem,
                lane_imm,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);
                let mem = mem.clone();

                let opcode_vrx = match (self, size) {
                    (&Inst::VecLoadLane { .. }, 8) => 0xe700,     // VLEB
                    (&Inst::VecLoadLane { .. }, 16) => 0xe701,    // VLEH
                    (&Inst::VecLoadLane { .. }, 32) => 0xe703,    // VLEF
                    (&Inst::VecLoadLane { .. }, 64) => 0xe702,    // VLEG
                    (&Inst::VecLoadLaneRev { .. }, 16) => 0xe601, // VLEBRH
                    (&Inst::VecLoadLaneRev { .. }, 32) => 0xe603, // VLEBRF
                    (&Inst::VecLoadLaneRev { .. }, 64) => 0xe602, // VLEBRG
                    _ => unreachable!(),
                };

                let rd = rd.to_reg();
                mem_vrx_emit(rd, &mem, opcode_vrx, lane_imm, true, sink, emit_info, state);
            }
            &Inst::VecLoadLaneUndef {
                size,
                rd,
                ref mem,
                lane_imm,
            }
            | &Inst::VecLoadLaneRevUndef {
                size,
                rd,
                ref mem,
                lane_imm,
            } => {
                let mem = mem.clone();

                let (opcode_vrx, opcode_rx, opcode_rxy) = match (self, size) {
                    (&Inst::VecLoadLaneUndef { .. }, 8) => (0xe700, None, None), // VLEB
                    (&Inst::VecLoadLaneUndef { .. }, 16) => (0xe701, None, None), // VLEH
                    (&Inst::VecLoadLaneUndef { .. }, 32) => (0xe703, Some(0x78), Some(0xed64)), // VLEF, LE(Y)
                    (&Inst::VecLoadLaneUndef { .. }, 64) => (0xe702, Some(0x68), Some(0xed65)), // VLEG, LD(Y)
                    (&Inst::VecLoadLaneRevUndef { .. }, 16) => (0xe601, None, None), // VLEBRH
                    (&Inst::VecLoadLaneRevUndef { .. }, 32) => (0xe603, None, None), // VLEBRF
                    (&Inst::VecLoadLaneRevUndef { .. }, 64) => (0xe602, None, None), // VLEBRG
                    _ => unreachable!(),
                };

                let rd = rd.to_reg();
                if lane_imm == 0 && is_fpr(rd) && opcode_rx.is_some() {
                    mem_emit(
                        rd, &mem, opcode_rx, opcode_rxy, None, true, sink, emit_info, state,
                    );
                } else {
                    mem_vrx_emit(rd, &mem, opcode_vrx, lane_imm, true, sink, emit_info, state);
                }
            }
            &Inst::VecStoreLane {
                size,
                rd,
                ref mem,
                lane_imm,
            }
            | &Inst::VecStoreLaneRev {
                size,
                rd,
                ref mem,
                lane_imm,
            } => {
                let mem = mem.clone();

                let (opcode_vrx, opcode_rx, opcode_rxy) = match (self, size) {
                    (&Inst::VecStoreLane { .. }, 8) => (0xe708, None, None), // VSTEB
                    (&Inst::VecStoreLane { .. }, 16) => (0xe709, None, None), // VSTEH
                    (&Inst::VecStoreLane { .. }, 32) => (0xe70b, Some(0x70), Some(0xed66)), // VSTEF, STE(Y)
                    (&Inst::VecStoreLane { .. }, 64) => (0xe70a, Some(0x60), Some(0xed67)), // VSTEG, STD(Y)
                    (&Inst::VecStoreLaneRev { .. }, 16) => (0xe609, None, None), // VSTEBRH
                    (&Inst::VecStoreLaneRev { .. }, 32) => (0xe60b, None, None), // VSTEBRF
                    (&Inst::VecStoreLaneRev { .. }, 64) => (0xe60a, None, None), // VSTEBRG
                    _ => unreachable!(),
                };

                if lane_imm == 0 && is_fpr(rd) && opcode_rx.is_some() {
                    mem_emit(
                        rd, &mem, opcode_rx, opcode_rxy, None, true, sink, emit_info, state,
                    );
                } else {
                    mem_vrx_emit(rd, &mem, opcode_vrx, lane_imm, true, sink, emit_info, state);
                }
            }
            &Inst::VecInsertLane {
                size,
                rd,
                ri,
                rn,
                lane_imm,
                lane_reg,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let (opcode_vrs, m4) = match size {
                    8 => (0xe722, 0),  // VLVGB
                    16 => (0xe722, 1), // VLVGH
                    32 => (0xe722, 2), // VLVGF
                    64 => (0xe722, 3), // VLVGG
                    _ => unreachable!(),
                };
                put(
                    sink,
                    &enc_vrs_b(opcode_vrs, rd.to_reg(), lane_reg, lane_imm.into(), rn, m4),
                );
            }
            &Inst::VecInsertLaneUndef {
                size,
                rd,
                rn,
                lane_imm,
                lane_reg,
            } => {
                let (opcode_vrs, m4, opcode_rre) = match size {
                    8 => (0xe722, 0, None),          // VLVGB
                    16 => (0xe722, 1, None),         // VLVGH
                    32 => (0xe722, 2, None),         // VLVGF
                    64 => (0xe722, 3, Some(0xb3c1)), // VLVGG, LDGR
                    _ => unreachable!(),
                };
                if opcode_rre.is_some()
                    && lane_imm == 0
                    && lane_reg == zero_reg()
                    && is_fpr(rd.to_reg())
                {
                    put(sink, &enc_rre(opcode_rre.unwrap(), rd.to_reg(), rn));
                } else {
                    put(
                        sink,
                        &enc_vrs_b(opcode_vrs, rd.to_reg(), lane_reg, lane_imm.into(), rn, m4),
                    );
                }
            }
            &Inst::VecExtractLane {
                size,
                rd,
                rn,
                lane_imm,
                lane_reg,
            } => {
                let (opcode_vrs, m4, opcode_rre) = match size {
                    8 => (0xe721, 0, None),          // VLGVB
                    16 => (0xe721, 1, None),         // VLGVH
                    32 => (0xe721, 2, None),         // VLGVF
                    64 => (0xe721, 3, Some(0xb3cd)), // VLGVG, LGDR
                    _ => unreachable!(),
                };
                if opcode_rre.is_some() && lane_imm == 0 && lane_reg == zero_reg() && is_fpr(rn) {
                    put(sink, &enc_rre(opcode_rre.unwrap(), rd.to_reg(), rn));
                } else {
                    put(
                        sink,
                        &enc_vrs_c(opcode_vrs, rd.to_reg(), lane_reg, lane_imm.into(), rn, m4),
                    );
                }
            }
            &Inst::VecInsertLaneImm {
                size,
                rd,
                ri,
                imm,
                lane_imm,
            } => {
                debug_assert_eq!(rd.to_reg(), ri);

                let opcode = match size {
                    8 => 0xe740,  // VLEIB
                    16 => 0xe741, // VLEIH
                    32 => 0xe743, // VLEIF
                    64 => 0xe742, // VLEIG
                    _ => unreachable!(),
                };
                put(sink, &enc_vri_a(opcode, rd.to_reg(), imm as u16, lane_imm));
            }
            &Inst::VecInsertLaneImmUndef {
                size,
                rd,
                imm,
                lane_imm,
            } => {
                let opcode = match size {
                    8 => 0xe740,  // VLEIB
                    16 => 0xe741, // VLEIH
                    32 => 0xe743, // VLEIF
                    64 => 0xe742, // VLEIG
                    _ => unreachable!(),
                };
                put(sink, &enc_vri_a(opcode, rd.to_reg(), imm as u16, lane_imm));
            }
            &Inst::VecReplicateLane {
                size,
                rd,
                rn,
                lane_imm,
            } => {
                let (opcode, m4) = match size {
                    8 => (0xe74d, 0),  // VREPB
                    16 => (0xe74d, 1), // VREPH
                    32 => (0xe74d, 2), // VREPF
                    64 => (0xe74d, 3), // VREPG
                    _ => unreachable!(),
                };
                put(
                    sink,
                    &enc_vri_c(opcode, rd.to_reg(), lane_imm.into(), rn, m4),
                );
            }

            &Inst::VecEltRev { lane_count, rd, rn } => {
                assert!(lane_count >= 2 && lane_count <= 16);
                let inst = Inst::VecPermuteDWImm {
                    rd,
                    rn,
                    rm: rn,
                    idx1: 1,
                    idx2: 0,
                };
                inst.emit(sink, emit_info, state);
                if lane_count >= 4 {
                    let inst = Inst::VecShiftRR {
                        shift_op: VecShiftOp::RotL64x2,
                        rd,
                        rn: rd.to_reg(),
                        shift_imm: 32,
                        shift_reg: zero_reg(),
                    };
                    inst.emit(sink, emit_info, state);
                }
                if lane_count >= 8 {
                    let inst = Inst::VecShiftRR {
                        shift_op: VecShiftOp::RotL32x4,
                        rd,
                        rn: rd.to_reg(),
                        shift_imm: 16,
                        shift_reg: zero_reg(),
                    };
                    inst.emit(sink, emit_info, state);
                }
                if lane_count >= 16 {
                    let inst = Inst::VecShiftRR {
                        shift_op: VecShiftOp::RotL16x8,
                        rd,
                        rn: rd.to_reg(),
                        shift_imm: 8,
                        shift_reg: zero_reg(),
                    };
                    inst.emit(sink, emit_info, state);
                }
            }

            &Inst::AllocateArgs { size } => {
                let inst = if let Ok(size) = i16::try_from(size) {
                    Inst::AluRSImm16 {
                        alu_op: ALUOp::Add64,
                        rd: writable_stack_reg(),
                        ri: stack_reg(),
                        imm: -size,
                    }
                } else {
                    Inst::AluRUImm32 {
                        alu_op: ALUOp::SubLogical64,
                        rd: writable_stack_reg(),
                        ri: stack_reg(),
                        imm: size,
                    }
                };
                inst.emit(sink, emit_info, state);
                assert_eq!(state.nominal_sp_offset, 0);
                state.nominal_sp_offset += size;
            }
            &Inst::Call { link, ref info } => {
                let enc: &[u8] = match &info.dest {
                    CallInstDest::Direct { name } => {
                        let offset = sink.cur_offset() + 2;
                        sink.add_reloc_at_offset(offset, Reloc::S390xPLTRel32Dbl, name, 2);
                        let opcode = 0xc05; // BRASL
                        &enc_ril_b(opcode, link.to_reg(), 0)
                    }
                    CallInstDest::Indirect { reg } => {
                        let opcode = 0x0d; // BASR
                        &enc_rr(opcode, link.to_reg(), *reg)
                    }
                };
                if let Some(s) = state.take_stack_map() {
                    let offset = sink.cur_offset() + enc.len() as u32;
                    sink.push_user_stack_map(state, offset, s);
                }
                put(sink, enc);

                if let Some(try_call) = info.try_call_info.as_ref() {
                    sink.add_try_call_site(try_call.exception_handlers(&state.frame_layout));
                } else {
                    sink.add_call_site();
                }

                state.nominal_sp_offset -= info.callee_pop_size;
                assert_eq!(state.nominal_sp_offset, 0);

                state.outgoing_sp_offset = info.callee_pop_size;
                for inst in S390xMachineDeps::gen_retval_loads(info) {
                    inst.emit(sink, emit_info, state);
                }
                state.outgoing_sp_offset = 0;

                // If this is a try-call, jump to the continuation
                // (normal-return) block.
                if let Some(try_call) = info.try_call_info.as_ref() {
                    let jmp = Inst::Jump {
                        dest: try_call.continuation,
                    };
                    jmp.emit(sink, emit_info, state);
                }
            }
            &Inst::ReturnCall { ref info } => {
                let (epilogue_insts, temp_dest) = S390xMachineDeps::gen_tail_epilogue(
                    state.frame_layout(),
                    info.callee_pop_size,
                    &info.dest,
                );
                for inst in epilogue_insts {
                    inst.emit(sink, emit_info, state);
                }

                let enc: &[u8] = match &info.dest {
                    CallInstDest::Direct { name } => {
                        let offset = sink.cur_offset() + 2;
                        sink.add_reloc_at_offset(offset, Reloc::S390xPLTRel32Dbl, name, 2);
                        let opcode = 0xc04; // BCRL
                        &enc_ril_c(opcode, 15, 0)
                    }
                    CallInstDest::Indirect { reg } => {
                        let opcode = 0x07; // BCR
                        &enc_rr(opcode, gpr(15), temp_dest.unwrap_or(*reg))
                    }
                };
                put(sink, enc);
                sink.add_call_site();
            }
            &Inst::ElfTlsGetOffset { ref symbol, .. } => {
                let opcode = 0xc05; // BRASL

                // Add relocation for target function. This has to be done
                // *before* the S390xTlsGdCall, to ensure linker relaxation
                // works correctly.
                let dest = ExternalName::LibCall(LibCall::ElfTlsGetOffset);
                let offset = sink.cur_offset() + 2;
                sink.add_reloc_at_offset(offset, Reloc::S390xPLTRel32Dbl, &dest, 2);
                match &**symbol {
                    SymbolReloc::TlsGd { name } => sink.add_reloc(Reloc::S390xTlsGdCall, name, 0),
                    _ => unreachable!(),
                }

                put(sink, &enc_ril_b(opcode, gpr(14), 0));
                sink.add_call_site();
            }
            &Inst::Args { .. } => {}
            &Inst::Rets { .. } => {}
            &Inst::Ret { link } => {
                let opcode = 0x07; // BCR
                put(sink, &enc_rr(opcode, gpr(15), link));
            }
            &Inst::Jump { dest } => {
                let off = sink.cur_offset();
                // Indicate that the jump uses a label, if so, so that a fixup can occur later.
                sink.use_label_at_offset(off, dest, LabelUse::BranchRIL);
                sink.add_uncond_branch(off, off + 6, dest);
                // Emit the jump itself.
                let opcode = 0xc04; // BCRL
                put(sink, &enc_ril_c(opcode, 15, 0));
            }
            &Inst::IndirectBr { rn, .. } => {
                let opcode = 0x07; // BCR
                put(sink, &enc_rr(opcode, gpr(15), rn));
            }
            &Inst::CondBr {
                taken,
                not_taken,
                cond,
            } => {
                let opcode = 0xc04; // BCRL

                // Conditional part first.
                let cond_off = sink.cur_offset();
                sink.use_label_at_offset(cond_off, taken, LabelUse::BranchRIL);
                let inverted = &enc_ril_c(opcode, cond.invert().bits(), 0);
                sink.add_cond_branch(cond_off, cond_off + 6, taken, inverted);
                put(sink, &enc_ril_c(opcode, cond.bits(), 0));

                // Unconditional part next.
                let uncond_off = sink.cur_offset();
                sink.use_label_at_offset(uncond_off, not_taken, LabelUse::BranchRIL);
                sink.add_uncond_branch(uncond_off, uncond_off + 6, not_taken);
                put(sink, &enc_ril_c(opcode, 15, 0));
            }
            &Inst::Nop0 => {}
            &Inst::Nop2 => {
                put(sink, &enc_e(0x0707));
            }
            &Inst::Debugtrap => {
                put(sink, &enc_e(0x0001));
            }
            &Inst::Trap { trap_code } => {
                put_with_trap(sink, &enc_e(0x0000), trap_code);
            }
            &Inst::TrapIf { cond, trap_code } => {
                // We implement a TrapIf as a conditional branch into the middle
                // of the branch (BRCL) instruction itself - those middle two bytes
                // are zero, which matches the trap instruction itself.
                let opcode = 0xc04; // BCRL
                let enc = &enc_ril_c(opcode, cond.bits(), 2);
                debug_assert!(enc.len() == 6 && enc[2] == 0 && enc[3] == 0);
                // The trap must be placed on the last byte of the embedded trap
                // instruction, so we need to emit the encoding in two parts.
                put_with_trap(sink, &enc[0..4], trap_code);
                put(sink, &enc[4..6]);
            }
            &Inst::JTSequence {
                ridx,
                default,
                default_cond,
                ref targets,
            } => {
                let table_label = sink.get_label();

                // This sequence is *one* instruction in the vcode, and is expanded only here at
                // emission time, because we cannot allow the regalloc to insert spills/reloads in
                // the middle; we depend on hardcoded PC-rel addressing below.

                // Branch to the default target if the given default condition is true.
                let opcode = 0xc04; // BCRL
                sink.use_label_at_offset(sink.cur_offset(), default, LabelUse::BranchRIL);
                put(sink, &enc_ril_c(opcode, default_cond.bits(), 0));

                // Set temp register to address of jump table.
                let rtmp = writable_spilltmp_reg();
                let inst = Inst::LoadAddr {
                    rd: rtmp,
                    mem: MemArg::Label {
                        target: table_label,
                    },
                };
                inst.emit(sink, emit_info, state);

                // Set temp to target address by adding the value of the jump table entry.
                let inst = Inst::AluRX {
                    alu_op: ALUOp::Add64Ext32,
                    rd: rtmp,
                    ri: rtmp.to_reg(),
                    mem: MemArg::reg_plus_reg(rtmp.to_reg(), ridx, MemFlags::trusted()),
                };
                inst.emit(sink, emit_info, state);

                // Branch to computed address. (`targets` here is only used for successor queries
                // and is not needed for emission.)
                let inst = Inst::IndirectBr {
                    rn: rtmp.to_reg(),
                    targets: vec![],
                };
                inst.emit(sink, emit_info, state);

                // Emit jump table (table of 32-bit offsets).
                sink.bind_label(table_label, &mut state.ctrl_plane);
                let jt_off = sink.cur_offset();
                for &target in targets.iter() {
                    let word_off = sink.cur_offset();
                    let off_into_table = word_off - jt_off;
                    sink.use_label_at_offset(word_off, target, LabelUse::PCRel32);
                    sink.put4(off_into_table.swap_bytes());
                }
            }

            Inst::StackProbeLoop {
                probe_count,
                guard_size,
            } => {
                // Emit the loop start label
                let loop_start = sink.get_label();
                sink.bind_label(loop_start, state.ctrl_plane_mut());

                // aghi %r15, -GUARD_SIZE
                let inst = Inst::AluRSImm16 {
                    alu_op: ALUOp::Add64,
                    rd: writable_stack_reg(),
                    ri: stack_reg(),
                    imm: -guard_size,
                };
                inst.emit(sink, emit_info, state);

                // mvi 0(%r15), 0
                let inst = Inst::StoreImm8 {
                    imm: 0,
                    mem: MemArg::reg(stack_reg(), MemFlags::trusted()),
                };
                inst.emit(sink, emit_info, state);

                // brct PROBE_COUNT, LOOP_START
                let opcode = 0xa76; // BRCT
                sink.use_label_at_offset(sink.cur_offset(), loop_start, LabelUse::BranchRI);
                put(sink, &enc_ri_b(opcode, probe_count.to_reg(), 0));
            }

            &Inst::Unwind { ref inst } => {
                sink.add_unwind(inst.clone());
            }

            &Inst::DummyUse { .. } => {}
        }

        state.clear_post_insn();
    }
}

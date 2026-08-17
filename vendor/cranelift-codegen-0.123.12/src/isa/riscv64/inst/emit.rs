//! Riscv64 ISA: binary code emission.

use crate::ir::{self, LibCall, TrapCode};
use crate::isa::riscv64::inst::*;
use crate::isa::riscv64::lower::isle::generated_code::{
    CaOp, CbOp, CiOp, CiwOp, ClOp, CrOp, CsOp, CssOp, CsznOp, FpuOPWidth, ZcbMemOp,
};
use cranelift_control::ControlPlane;

pub struct EmitInfo {
    shared_flag: settings::Flags,
    isa_flags: super::super::riscv_settings::Flags,
}

impl EmitInfo {
    pub(crate) fn new(
        shared_flag: settings::Flags,
        isa_flags: super::super::riscv_settings::Flags,
    ) -> Self {
        Self {
            shared_flag,
            isa_flags,
        }
    }
}

pub(crate) fn reg_to_gpr_num(m: Reg) -> u32 {
    u32::from(m.to_real_reg().unwrap().hw_enc() & 31)
}

pub(crate) fn reg_to_compressed_gpr_num(m: Reg) -> u32 {
    let real_reg = m.to_real_reg().unwrap().hw_enc();
    debug_assert!(real_reg >= 8 && real_reg < 16);
    let compressed_reg = real_reg - 8;
    u32::from(compressed_reg)
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum EmitVState {
    #[default]
    Unknown,
    Known(VState),
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

    /// Vector State
    /// Controls the current state of the vector unit at the emission point.
    vstate: EmitVState,

    frame_layout: FrameLayout,
}

impl EmitState {
    fn take_stack_map(&mut self) -> Option<ir::UserStackMap> {
        self.user_stack_map.take()
    }

    fn clobber_vstate(&mut self) {
        self.vstate = EmitVState::Unknown;
    }
}

impl MachInstEmitState<Inst> for EmitState {
    fn new(
        abi: &Callee<crate::isa::riscv64::abi::Riscv64MachineDeps>,
        ctrl_plane: ControlPlane,
    ) -> Self {
        EmitState {
            user_stack_map: None,
            ctrl_plane,
            vstate: EmitVState::Unknown,
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

    fn on_new_block(&mut self) {
        // Reset the vector state.
        self.clobber_vstate();
    }

    fn frame_layout(&self) -> &FrameLayout {
        &self.frame_layout
    }
}

impl Inst {
    /// Load int mask.
    /// If ty is int then 0xff in rd.
    pub(crate) fn load_int_mask(rd: Writable<Reg>, ty: Type) -> SmallInstVec<Inst> {
        let mut insts = SmallInstVec::new();
        assert!(ty.is_int() && ty.bits() <= 64);
        match ty {
            I64 => {
                insts.push(Inst::load_imm12(rd, Imm12::from_i16(-1)));
            }
            I32 | I16 => {
                insts.push(Inst::load_imm12(rd, Imm12::from_i16(-1)));
                insts.push(Inst::Extend {
                    rd,
                    rn: rd.to_reg(),
                    signed: false,
                    from_bits: ty.bits() as u8,
                    to_bits: 64,
                });
            }
            I8 => {
                insts.push(Inst::load_imm12(rd, Imm12::from_i16(255)));
            }
            _ => unreachable!("ty:{:?}", ty),
        }
        insts
    }
    ///  inverse all bit
    pub(crate) fn construct_bit_not(rd: Writable<Reg>, rs: Reg) -> Inst {
        Inst::AluRRImm12 {
            alu_op: AluOPRRI::Xori,
            rd,
            rs,
            imm12: Imm12::from_i16(-1),
        }
    }

    /// Returns Some(VState) if this instruction is expecting a specific vector state
    /// before emission.
    fn expected_vstate(&self) -> Option<&VState> {
        match self {
            Inst::Nop0
            | Inst::Nop4
            | Inst::BrTable { .. }
            | Inst::Auipc { .. }
            | Inst::Fli { .. }
            | Inst::Lui { .. }
            | Inst::LoadInlineConst { .. }
            | Inst::AluRRR { .. }
            | Inst::FpuRRR { .. }
            | Inst::AluRRImm12 { .. }
            | Inst::CsrReg { .. }
            | Inst::CsrImm { .. }
            | Inst::Load { .. }
            | Inst::Store { .. }
            | Inst::Args { .. }
            | Inst::Rets { .. }
            | Inst::Ret { .. }
            | Inst::Extend { .. }
            | Inst::Call { .. }
            | Inst::CallInd { .. }
            | Inst::ReturnCall { .. }
            | Inst::ReturnCallInd { .. }
            | Inst::Jal { .. }
            | Inst::CondBr { .. }
            | Inst::LoadExtName { .. }
            | Inst::ElfTlsGetAddr { .. }
            | Inst::LoadAddr { .. }
            | Inst::Mov { .. }
            | Inst::MovFromPReg { .. }
            | Inst::Fence { .. }
            | Inst::EBreak
            | Inst::Udf { .. }
            | Inst::FpuRR { .. }
            | Inst::FpuRRRR { .. }
            | Inst::Jalr { .. }
            | Inst::Atomic { .. }
            | Inst::Select { .. }
            | Inst::AtomicCas { .. }
            | Inst::RawData { .. }
            | Inst::AtomicStore { .. }
            | Inst::AtomicLoad { .. }
            | Inst::AtomicRmwLoop { .. }
            | Inst::TrapIf { .. }
            | Inst::Unwind { .. }
            | Inst::DummyUse { .. }
            | Inst::Popcnt { .. }
            | Inst::Cltz { .. }
            | Inst::Brev8 { .. }
            | Inst::StackProbeLoop { .. } => None,

            // VecSetState does not expect any vstate, rather it updates it.
            Inst::VecSetState { .. } => None,

            // `vmv` instructions copy a set of registers and ignore vstate.
            Inst::VecAluRRImm5 { op: VecAluOpRRImm5::VmvrV, .. } => None,

            Inst::VecAluRR { vstate, .. } |
            Inst::VecAluRRR { vstate, .. } |
            Inst::VecAluRRRR { vstate, .. } |
            Inst::VecAluRImm5 { vstate, .. } |
            Inst::VecAluRRImm5 { vstate, .. } |
            Inst::VecAluRRRImm5 { vstate, .. } |
            // TODO: Unit-stride loads and stores only need the AVL to be correct, not
            // the full vtype. A future optimization could be to decouple these two when
            // updating vstate. This would allow us to avoid emitting a VecSetState in
            // some cases.
            Inst::VecLoad { vstate, .. }
            | Inst::VecStore { vstate, .. } => Some(vstate),
            Inst::EmitIsland { .. } => None,
        }
    }
}

impl MachInstEmit for Inst {
    type State = EmitState;
    type Info = EmitInfo;

    fn emit(&self, sink: &mut MachBuffer<Inst>, emit_info: &Self::Info, state: &mut EmitState) {
        // Check if we need to update the vector state before emitting this instruction
        if let Some(expected) = self.expected_vstate() {
            if state.vstate != EmitVState::Known(*expected) {
                // Update the vector state.
                Inst::VecSetState {
                    rd: writable_zero_reg(),
                    vstate: *expected,
                }
                .emit(sink, emit_info, state);
            }
        }

        // N.B.: we *must* not exceed the "worst-case size" used to compute
        // where to insert islands, except when islands are explicitly triggered
        // (with an `EmitIsland`). We check this in debug builds. This is `mut`
        // to allow disabling the check for `JTSequence`, which is always
        // emitted following an `EmitIsland`.
        let mut start_off = sink.cur_offset();

        // First try to emit this as a compressed instruction
        let res = self.try_emit_compressed(sink, emit_info, state, &mut start_off);
        if res.is_none() {
            // If we can't lets emit it as a normal instruction
            self.emit_uncompressed(sink, emit_info, state, &mut start_off);
        }

        // We exclude br_table, call, return_call and try_call from
        // these checks since they emit their own islands, and thus
        // are allowed to exceed the worst case size.
        let emits_own_island = match self {
            Inst::BrTable { .. }
            | Inst::ReturnCall { .. }
            | Inst::ReturnCallInd { .. }
            | Inst::Call { .. }
            | Inst::CallInd { .. }
            | Inst::EmitIsland { .. } => true,
            _ => false,
        };
        if !emits_own_island {
            let end_off = sink.cur_offset();
            assert!(
                (end_off - start_off) <= Inst::worst_case_size(),
                "Inst:{:?} length:{} worst_case_size:{}",
                self,
                end_off - start_off,
                Inst::worst_case_size()
            );
        }
    }

    fn pretty_print_inst(&self, state: &mut Self::State) -> String {
        self.print_with_state(state)
    }
}

impl Inst {
    /// Tries to emit an instruction as compressed, if we can't return false.
    fn try_emit_compressed(
        &self,
        sink: &mut MachBuffer<Inst>,
        emit_info: &EmitInfo,
        state: &mut EmitState,
        start_off: &mut u32,
    ) -> Option<()> {
        let has_m = emit_info.isa_flags.has_m();
        let has_zba = emit_info.isa_flags.has_zba();
        let has_zbb = emit_info.isa_flags.has_zbb();
        let has_zca = emit_info.isa_flags.has_zca();
        let has_zcb = emit_info.isa_flags.has_zcb();
        let has_zcd = emit_info.isa_flags.has_zcd();

        // Currently all compressed extensions (Zcb, Zcd, Zcmp, Zcmt, etc..) require Zca
        // to be enabled, so check it early.
        if !has_zca {
            return None;
        }

        fn reg_is_compressible(r: Reg) -> bool {
            r.to_real_reg()
                .map(|r| r.hw_enc() >= 8 && r.hw_enc() < 16)
                .unwrap_or(false)
        }

        match *self {
            // C.ADD
            Inst::AluRRR {
                alu_op: AluOPRRR::Add,
                rd,
                rs1,
                rs2,
            } if (rd.to_reg() == rs1 || rd.to_reg() == rs2)
                && rs1 != zero_reg()
                && rs2 != zero_reg() =>
            {
                // Technically `c.add rd, rs` expands to `add rd, rd, rs`, but we can
                // also swap rs1 with rs2 and we get an equivalent instruction. i.e we
                // can also compress `add rd, rs, rd` into `c.add rd, rs`.
                let src = if rd.to_reg() == rs1 { rs2 } else { rs1 };

                sink.put2(encode_cr_type(CrOp::CAdd, rd, src));
            }

            // C.MV
            Inst::AluRRImm12 {
                alu_op: AluOPRRI::Addi | AluOPRRI::Ori,
                rd,
                rs,
                imm12,
            } if rd.to_reg() != rs
                && rd.to_reg() != zero_reg()
                && rs != zero_reg()
                && imm12.as_i16() == 0 =>
            {
                sink.put2(encode_cr_type(CrOp::CMv, rd, rs));
            }

            // CA Ops
            Inst::AluRRR {
                alu_op:
                    alu_op @ (AluOPRRR::And
                    | AluOPRRR::Or
                    | AluOPRRR::Xor
                    | AluOPRRR::Addw
                    | AluOPRRR::Mul),
                rd,
                rs1,
                rs2,
            } if (rd.to_reg() == rs1 || rd.to_reg() == rs2)
                && reg_is_compressible(rs1)
                && reg_is_compressible(rs2) =>
            {
                let op = match alu_op {
                    AluOPRRR::And => CaOp::CAnd,
                    AluOPRRR::Or => CaOp::COr,
                    AluOPRRR::Xor => CaOp::CXor,
                    AluOPRRR::Addw => CaOp::CAddw,
                    AluOPRRR::Mul if has_zcb && has_m => CaOp::CMul,
                    _ => return None,
                };
                // The canonical expansion for these instruction has `rd == rs1`, but
                // these are all commutative operations, so we can swap the operands.
                let src = if rd.to_reg() == rs1 { rs2 } else { rs1 };

                sink.put2(encode_ca_type(op, rd, src));
            }

            // The sub instructions are non commutative, so we can't swap the operands.
            Inst::AluRRR {
                alu_op: alu_op @ (AluOPRRR::Sub | AluOPRRR::Subw),
                rd,
                rs1,
                rs2,
            } if rd.to_reg() == rs1 && reg_is_compressible(rs1) && reg_is_compressible(rs2) => {
                let op = match alu_op {
                    AluOPRRR::Sub => CaOp::CSub,
                    AluOPRRR::Subw => CaOp::CSubw,
                    _ => return None,
                };
                sink.put2(encode_ca_type(op, rd, rs2));
            }

            // c.j
            //
            // We don't have a separate JAL as that is only available in RV32C
            Inst::Jal { label } => {
                sink.use_label_at_offset(*start_off, label, LabelUse::RVCJump);
                sink.add_uncond_branch(*start_off, *start_off + 2, label);
                sink.put2(encode_cj_type(CjOp::CJ, Imm12::ZERO));
            }

            // c.jr
            Inst::Jalr { rd, base, offset }
                if rd.to_reg() == zero_reg() && base != zero_reg() && offset.as_i16() == 0 =>
            {
                sink.put2(encode_cr2_type(CrOp::CJr, base));
                state.clobber_vstate();
            }

            // c.jalr
            Inst::Jalr { rd, base, offset }
                if rd.to_reg() == link_reg() && base != zero_reg() && offset.as_i16() == 0 =>
            {
                sink.put2(encode_cr2_type(CrOp::CJalr, base));
                state.clobber_vstate();
            }

            // c.ebreak
            Inst::EBreak => {
                sink.put2(encode_cr_type(
                    CrOp::CEbreak,
                    writable_zero_reg(),
                    zero_reg(),
                ));
            }

            // c.unimp
            Inst::Udf { trap_code } => {
                sink.add_trap(trap_code);
                sink.put2(0x0000);
            }
            // c.addi16sp
            //
            // c.addi16sp shares the opcode with c.lui, but has a destination field of x2.
            // c.addi16sp adds the non-zero sign-extended 6-bit immediate to the value in the stack pointer (sp=x2),
            // where the immediate is scaled to represent multiples of 16 in the range (-512,496). c.addi16sp is used
            // to adjust the stack pointer in procedure prologues and epilogues. It expands into addi x2, x2, nzimm. c.addi16sp
            // is only valid when nzimm≠0; the code point with nzimm=0 is reserved.
            Inst::AluRRImm12 {
                alu_op: AluOPRRI::Addi,
                rd,
                rs,
                imm12,
            } if rd.to_reg() == rs
                && rs == stack_reg()
                && imm12.as_i16() != 0
                && (imm12.as_i16() % 16) == 0
                && Imm6::maybe_from_i16(imm12.as_i16() / 16).is_some() =>
            {
                let imm6 = Imm6::maybe_from_i16(imm12.as_i16() / 16).unwrap();
                sink.put2(encode_c_addi16sp(imm6));
            }

            // c.addi4spn
            //
            // c.addi4spn is a CIW-format instruction that adds a zero-extended non-zero
            // immediate, scaled by 4, to the stack pointer, x2, and writes the result to
            // rd. This instruction is used to generate pointers to stack-allocated variables
            // and expands to addi rd, x2, nzuimm. c.addi4spn is only valid when nzuimm≠0;
            // the code points with nzuimm=0 are reserved.
            Inst::AluRRImm12 {
                alu_op: AluOPRRI::Addi,
                rd,
                rs,
                imm12,
            } if reg_is_compressible(rd.to_reg())
                && rs == stack_reg()
                && imm12.as_i16() != 0
                && (imm12.as_i16() % 4) == 0
                && u8::try_from(imm12.as_i16() / 4).is_ok() =>
            {
                let imm = u8::try_from(imm12.as_i16() / 4).unwrap();
                sink.put2(encode_ciw_type(CiwOp::CAddi4spn, rd, imm));
            }

            // c.li
            Inst::AluRRImm12 {
                alu_op: AluOPRRI::Addi,
                rd,
                rs,
                imm12,
            } if rd.to_reg() != zero_reg() && rs == zero_reg() => {
                let imm6 = Imm6::maybe_from_imm12(imm12)?;
                sink.put2(encode_ci_type(CiOp::CLi, rd, imm6));
            }

            // c.addi
            Inst::AluRRImm12 {
                alu_op: AluOPRRI::Addi,
                rd,
                rs,
                imm12,
            } if rd.to_reg() == rs && rs != zero_reg() && imm12.as_i16() != 0 => {
                let imm6 = Imm6::maybe_from_imm12(imm12)?;
                sink.put2(encode_ci_type(CiOp::CAddi, rd, imm6));
            }

            // c.addiw
            Inst::AluRRImm12 {
                alu_op: AluOPRRI::Addiw,
                rd,
                rs,
                imm12,
            } if rd.to_reg() == rs && rs != zero_reg() => {
                let imm6 = Imm6::maybe_from_imm12(imm12)?;
                sink.put2(encode_ci_type(CiOp::CAddiw, rd, imm6));
            }

            // c.lui
            //
            // c.lui loads the non-zero 6-bit immediate field into bits 17–12
            // of the destination register, clears the bottom 12 bits, and
            // sign-extends bit 17 into all higher bits of the destination.
            Inst::Lui { rd, imm: imm20 }
                if rd.to_reg() != zero_reg()
                    && rd.to_reg() != stack_reg()
                    && imm20.as_i32() != 0 =>
            {
                // Check that the top bits are sign extended
                let imm = imm20.as_i32() << 14 >> 14;
                if imm != imm20.as_i32() {
                    return None;
                }
                let imm6 = Imm6::maybe_from_i32(imm)?;
                sink.put2(encode_ci_type(CiOp::CLui, rd, imm6));
            }

            // c.slli
            Inst::AluRRImm12 {
                alu_op: AluOPRRI::Slli,
                rd,
                rs,
                imm12,
            } if rd.to_reg() == rs && rs != zero_reg() && imm12.as_i16() != 0 => {
                // The shift amount is unsigned, but we encode it as signed.
                let shift = imm12.as_i16() & 0x3f;
                let imm6 = Imm6::maybe_from_i16(shift << 10 >> 10).unwrap();
                sink.put2(encode_ci_type(CiOp::CSlli, rd, imm6));
            }

            // c.srli / c.srai
            Inst::AluRRImm12 {
                alu_op: op @ (AluOPRRI::Srli | AluOPRRI::Srai),
                rd,
                rs,
                imm12,
            } if rd.to_reg() == rs && reg_is_compressible(rs) && imm12.as_i16() != 0 => {
                let op = match op {
                    AluOPRRI::Srli => CbOp::CSrli,
                    AluOPRRI::Srai => CbOp::CSrai,
                    _ => unreachable!(),
                };

                // The shift amount is unsigned, but we encode it as signed.
                let shift = imm12.as_i16() & 0x3f;
                let imm6 = Imm6::maybe_from_i16(shift << 10 >> 10).unwrap();
                sink.put2(encode_cb_type(op, rd, imm6));
            }

            // c.zextb
            //
            // This is an alias for `andi rd, rd, 0xff`
            Inst::AluRRImm12 {
                alu_op: AluOPRRI::Andi,
                rd,
                rs,
                imm12,
            } if has_zcb
                && rd.to_reg() == rs
                && reg_is_compressible(rs)
                && imm12.as_i16() == 0xff =>
            {
                sink.put2(encode_cszn_type(CsznOp::CZextb, rd));
            }

            // c.andi
            Inst::AluRRImm12 {
                alu_op: AluOPRRI::Andi,
                rd,
                rs,
                imm12,
            } if rd.to_reg() == rs && reg_is_compressible(rs) => {
                let imm6 = Imm6::maybe_from_imm12(imm12)?;
                sink.put2(encode_cb_type(CbOp::CAndi, rd, imm6));
            }

            // Stack Based Loads
            Inst::Load {
                rd,
                op: op @ (LoadOP::Lw | LoadOP::Ld | LoadOP::Fld),
                from,
                flags,
            } if from.get_base_register() == Some(stack_reg())
                && (from.get_offset_with_state(state) % op.size()) == 0 =>
            {
                // We encode the offset in multiples of the load size.
                let offset = from.get_offset_with_state(state);
                let imm6 = u8::try_from(offset / op.size())
                    .ok()
                    .and_then(Uimm6::maybe_from_u8)?;

                // Some additional constraints on these instructions.
                //
                // Integer loads are not allowed to target x0, but floating point loads
                // are, since f0 is not a special register.
                //
                // Floating point loads are not included in the base Zca extension
                // but in a separate Zcd extension. Both of these are part of the C Extension.
                let rd_is_zero = rd.to_reg() == zero_reg();
                let op = match op {
                    LoadOP::Lw if !rd_is_zero => CiOp::CLwsp,
                    LoadOP::Ld if !rd_is_zero => CiOp::CLdsp,
                    LoadOP::Fld if has_zcd => CiOp::CFldsp,
                    _ => return None,
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }
                sink.put2(encode_ci_sp_load(op, rd, imm6));
            }

            // Regular Loads
            Inst::Load {
                rd,
                op:
                    op
                    @ (LoadOP::Lw | LoadOP::Ld | LoadOP::Fld | LoadOP::Lbu | LoadOP::Lhu | LoadOP::Lh),
                from,
                flags,
            } if reg_is_compressible(rd.to_reg())
                && from
                    .get_base_register()
                    .map(reg_is_compressible)
                    .unwrap_or(false)
                && (from.get_offset_with_state(state) % op.size()) == 0 =>
            {
                let base = from.get_base_register().unwrap();

                // We encode the offset in multiples of the store size.
                let offset = from.get_offset_with_state(state);
                let offset = u8::try_from(offset / op.size()).ok()?;

                // We mix two different formats here.
                //
                // c.lw / c.ld / c.fld instructions are available in the standard Zca
                // extension using the CL format.
                //
                // c.lbu / c.lhu / c.lh are only available in the Zcb extension and
                // are also encoded differently. Technically they each have a different
                // format, but they are similar enough that we can group them.
                let is_zcb_load = matches!(op, LoadOP::Lbu | LoadOP::Lhu | LoadOP::Lh);
                let encoded = if is_zcb_load {
                    if !has_zcb {
                        return None;
                    }

                    let op = match op {
                        LoadOP::Lbu => ZcbMemOp::CLbu,
                        LoadOP::Lhu => ZcbMemOp::CLhu,
                        LoadOP::Lh => ZcbMemOp::CLh,
                        _ => unreachable!(),
                    };

                    // Byte stores & loads have 2 bits of immediate offset. Halfword stores
                    // and loads only have 1 bit.
                    let imm2 = Uimm2::maybe_from_u8(offset)?;
                    if (offset & !((1 << op.imm_bits()) - 1)) != 0 {
                        return None;
                    }

                    encode_zcbmem_load(op, rd, base, imm2)
                } else {
                    // Floating point loads are not included in the base Zca extension
                    // but in a separate Zcd extension. Both of these are part of the C Extension.
                    let op = match op {
                        LoadOP::Lw => ClOp::CLw,
                        LoadOP::Ld => ClOp::CLd,
                        LoadOP::Fld if has_zcd => ClOp::CFld,
                        _ => return None,
                    };
                    let imm5 = Uimm5::maybe_from_u8(offset)?;

                    encode_cl_type(op, rd, base, imm5)
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }
                sink.put2(encoded);
            }

            // Stack Based Stores
            Inst::Store {
                src,
                op: op @ (StoreOP::Sw | StoreOP::Sd | StoreOP::Fsd),
                to,
                flags,
            } if to.get_base_register() == Some(stack_reg())
                && (to.get_offset_with_state(state) % op.size()) == 0 =>
            {
                // We encode the offset in multiples of the store size.
                let offset = to.get_offset_with_state(state);
                let imm6 = u8::try_from(offset / op.size())
                    .ok()
                    .and_then(Uimm6::maybe_from_u8)?;

                // Floating point stores are not included in the base Zca extension
                // but in a separate Zcd extension. Both of these are part of the C Extension.
                let op = match op {
                    StoreOP::Sw => CssOp::CSwsp,
                    StoreOP::Sd => CssOp::CSdsp,
                    StoreOP::Fsd if has_zcd => CssOp::CFsdsp,
                    _ => return None,
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }
                sink.put2(encode_css_type(op, src, imm6));
            }

            // Regular Stores
            Inst::Store {
                src,
                op: op @ (StoreOP::Sw | StoreOP::Sd | StoreOP::Fsd | StoreOP::Sh | StoreOP::Sb),
                to,
                flags,
            } if reg_is_compressible(src)
                && to
                    .get_base_register()
                    .map(reg_is_compressible)
                    .unwrap_or(false)
                && (to.get_offset_with_state(state) % op.size()) == 0 =>
            {
                let base = to.get_base_register().unwrap();

                // We encode the offset in multiples of the store size.
                let offset = to.get_offset_with_state(state);
                let offset = u8::try_from(offset / op.size()).ok()?;

                // We mix two different formats here.
                //
                // c.sw / c.sd / c.fsd instructions are available in the standard Zca
                // extension using the CL format.
                //
                // c.sb / c.sh are only available in the Zcb extension and are also
                // encoded differently.
                let is_zcb_store = matches!(op, StoreOP::Sh | StoreOP::Sb);
                let encoded = if is_zcb_store {
                    if !has_zcb {
                        return None;
                    }

                    let op = match op {
                        StoreOP::Sh => ZcbMemOp::CSh,
                        StoreOP::Sb => ZcbMemOp::CSb,
                        _ => unreachable!(),
                    };

                    // Byte stores & loads have 2 bits of immediate offset. Halfword stores
                    // and loads only have 1 bit.
                    let imm2 = Uimm2::maybe_from_u8(offset)?;
                    if (offset & !((1 << op.imm_bits()) - 1)) != 0 {
                        return None;
                    }

                    encode_zcbmem_store(op, src, base, imm2)
                } else {
                    // Floating point stores are not included in the base Zca extension
                    // but in a separate Zcd extension. Both of these are part of the C Extension.
                    let op = match op {
                        StoreOP::Sw => CsOp::CSw,
                        StoreOP::Sd => CsOp::CSd,
                        StoreOP::Fsd if has_zcd => CsOp::CFsd,
                        _ => return None,
                    };
                    let imm5 = Uimm5::maybe_from_u8(offset)?;

                    encode_cs_type(op, src, base, imm5)
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }
                sink.put2(encoded);
            }

            // c.not
            //
            // This is an alias for `xori rd, rd, -1`
            Inst::AluRRImm12 {
                alu_op: AluOPRRI::Xori,
                rd,
                rs,
                imm12,
            } if has_zcb
                && rd.to_reg() == rs
                && reg_is_compressible(rs)
                && imm12.as_i16() == -1 =>
            {
                sink.put2(encode_cszn_type(CsznOp::CNot, rd));
            }

            // c.sext.b / c.sext.h / c.zext.h
            //
            // These are all the extend instructions present in `Zcb`, they
            // also require `Zbb` since they aren't available in the base ISA.
            Inst::AluRRImm12 {
                alu_op: alu_op @ (AluOPRRI::Sextb | AluOPRRI::Sexth | AluOPRRI::Zexth),
                rd,
                rs,
                imm12,
            } if has_zcb
                && has_zbb
                && rd.to_reg() == rs
                && reg_is_compressible(rs)
                && imm12.as_i16() == 0 =>
            {
                let op = match alu_op {
                    AluOPRRI::Sextb => CsznOp::CSextb,
                    AluOPRRI::Sexth => CsznOp::CSexth,
                    AluOPRRI::Zexth => CsznOp::CZexth,
                    _ => unreachable!(),
                };
                sink.put2(encode_cszn_type(op, rd));
            }

            // c.zext.w
            //
            // This is an alias for `add.uw rd, rd, zero`
            Inst::AluRRR {
                alu_op: AluOPRRR::Adduw,
                rd,
                rs1,
                rs2,
            } if has_zcb
                && has_zba
                && rd.to_reg() == rs1
                && reg_is_compressible(rs1)
                && rs2 == zero_reg() =>
            {
                sink.put2(encode_cszn_type(CsznOp::CZextw, rd));
            }

            _ => return None,
        }

        return Some(());
    }

    fn emit_uncompressed(
        &self,
        sink: &mut MachBuffer<Inst>,
        emit_info: &EmitInfo,
        state: &mut EmitState,
        start_off: &mut u32,
    ) {
        match self {
            &Inst::Nop0 => {
                // do nothing
            }
            // Addi x0, x0, 0
            &Inst::Nop4 => {
                let x = Inst::AluRRImm12 {
                    alu_op: AluOPRRI::Addi,
                    rd: Writable::from_reg(zero_reg()),
                    rs: zero_reg(),
                    imm12: Imm12::ZERO,
                };
                x.emit(sink, emit_info, state)
            }
            &Inst::RawData { ref data } => {
                // Right now we only put a u32 or u64 in this instruction.
                // It is not very long, no need to check if need `emit_island`.
                // If data is very long , this is a bug because RawData is typically
                // use to load some data and rely on some position in the code stream.
                // and we may exceed `Inst::worst_case_size`.
                // for more information see https://github.com/bytecodealliance/wasmtime/pull/5612.
                sink.put_data(&data[..]);
            }
            &Inst::Lui { rd, ref imm } => {
                let x: u32 = 0b0110111 | reg_to_gpr_num(rd.to_reg()) << 7 | (imm.bits() << 12);
                sink.put4(x);
            }
            &Inst::Fli { rd, width, imm } => {
                sink.put4(encode_fli(width, imm, rd));
            }
            &Inst::LoadInlineConst { rd, ty, imm } => {
                let data = &imm.to_le_bytes()[..ty.bytes() as usize];

                let label_data: MachLabel = sink.get_label();
                let label_end: MachLabel = sink.get_label();

                // Load into rd
                Inst::Load {
                    rd,
                    op: LoadOP::from_type(ty),
                    flags: MemFlags::new(),
                    from: AMode::Label(label_data),
                }
                .emit(sink, emit_info, state);

                // Jump over the inline pool
                Inst::gen_jump(label_end).emit(sink, emit_info, state);

                // Emit the inline data
                sink.bind_label(label_data, &mut state.ctrl_plane);
                Inst::RawData { data: data.into() }.emit(sink, emit_info, state);

                sink.bind_label(label_end, &mut state.ctrl_plane);
            }
            &Inst::FpuRR {
                alu_op,
                width,
                frm,
                rd,
                rs,
            } => {
                if alu_op.is_convert_to_int() {
                    sink.add_trap(TrapCode::BAD_CONVERSION_TO_INTEGER);
                }
                sink.put4(encode_fp_rr(alu_op, width, frm, rd, rs));
            }
            &Inst::FpuRRRR {
                alu_op,
                rd,
                rs1,
                rs2,
                rs3,
                frm,
                width,
            } => {
                sink.put4(encode_fp_rrrr(alu_op, width, frm, rd, rs1, rs2, rs3));
            }
            &Inst::FpuRRR {
                alu_op,
                width,
                frm,
                rd,
                rs1,
                rs2,
            } => {
                sink.put4(encode_fp_rrr(alu_op, width, frm, rd, rs1, rs2));
            }
            &Inst::Unwind { ref inst } => {
                sink.add_unwind(inst.clone());
            }
            &Inst::DummyUse { .. } => {
                // This has already been handled by Inst::allocate.
            }
            &Inst::AluRRR {
                alu_op,
                rd,
                rs1,
                rs2,
            } => {
                let (rs1, rs2) = if alu_op.reverse_rs() {
                    (rs2, rs1)
                } else {
                    (rs1, rs2)
                };

                sink.put4(encode_r_type(
                    alu_op.op_code(),
                    rd,
                    alu_op.funct3(),
                    rs1,
                    rs2,
                    alu_op.funct7(),
                ));
            }
            &Inst::AluRRImm12 {
                alu_op,
                rd,
                rs,
                imm12,
            } => {
                let x = alu_op.op_code()
                    | reg_to_gpr_num(rd.to_reg()) << 7
                    | alu_op.funct3() << 12
                    | reg_to_gpr_num(rs) << 15
                    | alu_op.imm12(imm12) << 20;
                sink.put4(x);
            }
            &Inst::CsrReg { op, rd, rs, csr } => {
                sink.put4(encode_csr_reg(op, rd, rs, csr));
            }
            &Inst::CsrImm { op, rd, csr, imm } => {
                sink.put4(encode_csr_imm(op, rd, csr, imm));
            }
            &Inst::Load {
                rd,
                op: LoadOP::Flh,
                from,
                flags,
            } if !emit_info.isa_flags.has_zfhmin() => {
                // flh unavailable, use an integer load instead
                Inst::Load {
                    rd: writable_spilltmp_reg(),
                    op: LoadOP::Lh,
                    flags,
                    from,
                }
                .emit(sink, emit_info, state);
                // NaN-box the `f16` before loading it into the floating-point
                // register with a 32-bit `fmv`.
                Inst::Lui {
                    rd: writable_spilltmp_reg2(),
                    imm: Imm20::from_i32((0xffff_0000_u32 as i32) >> 12),
                }
                .emit(sink, emit_info, state);
                Inst::AluRRR {
                    alu_op: AluOPRRR::Or,
                    rd: writable_spilltmp_reg(),
                    rs1: spilltmp_reg(),
                    rs2: spilltmp_reg2(),
                }
                .emit(sink, emit_info, state);
                Inst::FpuRR {
                    alu_op: FpuOPRR::FmvFmtX,
                    width: FpuOPWidth::S,
                    frm: FRM::RNE,
                    rd,
                    rs: spilltmp_reg(),
                }
                .emit(sink, emit_info, state);
            }
            &Inst::Load {
                rd,
                op,
                from,
                flags,
            } => {
                let base = from.get_base_register();
                let offset = from.get_offset_with_state(state);
                let offset_imm12 = Imm12::maybe_from_i64(offset);
                let label = from.get_label_with_sink(sink);

                let (addr, imm12) = match (base, offset_imm12, label) {
                    // When loading from a Reg+Offset, if the offset fits into an imm12 we can directly encode it.
                    (Some(base), Some(imm12), None) => (base, imm12),

                    // Otherwise, if the offset does not fit into a imm12, we need to materialize it into a
                    // register and load from that.
                    (Some(_), None, None) => {
                        let tmp = writable_spilltmp_reg();
                        Inst::LoadAddr { rd: tmp, mem: from }.emit(sink, emit_info, state);
                        (tmp.to_reg(), Imm12::ZERO)
                    }

                    // If the AMode contains a label we can emit an internal relocation that gets
                    // resolved with the correct address later.
                    (None, Some(imm), Some(label)) => {
                        debug_assert_eq!(imm.as_i16(), 0);

                        // Get the current PC.
                        sink.use_label_at_offset(sink.cur_offset(), label, LabelUse::PCRelHi20);
                        Inst::Auipc {
                            rd,
                            imm: Imm20::ZERO,
                        }
                        .emit_uncompressed(sink, emit_info, state, start_off);

                        // Emit a relocation for the load. This patches the offset into the instruction.
                        sink.use_label_at_offset(sink.cur_offset(), label, LabelUse::PCRelLo12I);

                        // Imm12 here is meaningless since it's going to get replaced.
                        (rd.to_reg(), Imm12::ZERO)
                    }

                    // These cases are impossible with the current AModes that we have. We either
                    // always have a register, or always have a label. Never both, and never neither.
                    (None, None, None)
                    | (None, Some(_), None)
                    | (Some(_), None, Some(_))
                    | (Some(_), Some(_), Some(_))
                    | (None, None, Some(_)) => {
                        unreachable!("Invalid load address")
                    }
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }

                sink.put4(encode_i_type(op.op_code(), rd, op.funct3(), addr, imm12));
            }
            &Inst::Store {
                op: StoreOP::Fsh,
                src,
                flags,
                to,
            } if !emit_info.isa_flags.has_zfhmin() => {
                // fsh unavailable, use an integer store instead
                Inst::FpuRR {
                    alu_op: FpuOPRR::FmvXFmt,
                    width: FpuOPWidth::S,
                    frm: FRM::RNE,
                    rd: writable_spilltmp_reg(),
                    rs: src,
                }
                .emit(sink, emit_info, state);
                Inst::Store {
                    to,
                    op: StoreOP::Sh,
                    flags,
                    src: spilltmp_reg(),
                }
                .emit(sink, emit_info, state);
            }
            &Inst::Store { op, src, flags, to } => {
                let base = to.get_base_register();
                let offset = to.get_offset_with_state(state);
                let offset_imm12 = Imm12::maybe_from_i64(offset);

                let (addr, imm12) = match (base, offset_imm12) {
                    // If the offset fits into an imm12 we can directly encode it.
                    (Some(base), Some(imm12)) => (base, imm12),
                    // Otherwise load the address it into a reg and load from it.
                    _ => {
                        let tmp = writable_spilltmp_reg();
                        Inst::LoadAddr { rd: tmp, mem: to }.emit(sink, emit_info, state);
                        (tmp.to_reg(), Imm12::ZERO)
                    }
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }

                sink.put4(encode_s_type(op.op_code(), op.funct3(), addr, src, imm12));
            }
            &Inst::Args { .. } | &Inst::Rets { .. } => {
                // Nothing: this is a pseudoinstruction that serves
                // only to constrain registers at a certain point.
            }
            &Inst::Ret {} => {
                // RISC-V does not have a dedicated ret instruction, instead we emit the equivalent
                // `jalr x0, x1, 0` that jumps to the return address.
                Inst::Jalr {
                    rd: writable_zero_reg(),
                    base: link_reg(),
                    offset: Imm12::ZERO,
                }
                .emit(sink, emit_info, state);
            }

            &Inst::Extend {
                rd,
                rn,
                signed,
                from_bits,
                to_bits: _to_bits,
            } => {
                let mut insts = SmallInstVec::new();
                let shift_bits = (64 - from_bits) as i16;
                let is_u8 = || from_bits == 8 && signed == false;
                if is_u8() {
                    // special for u8.
                    insts.push(Inst::AluRRImm12 {
                        alu_op: AluOPRRI::Andi,
                        rd,
                        rs: rn,
                        imm12: Imm12::from_i16(255),
                    });
                } else {
                    insts.push(Inst::AluRRImm12 {
                        alu_op: AluOPRRI::Slli,
                        rd,
                        rs: rn,
                        imm12: Imm12::from_i16(shift_bits),
                    });
                    insts.push(Inst::AluRRImm12 {
                        alu_op: if signed {
                            AluOPRRI::Srai
                        } else {
                            AluOPRRI::Srli
                        },
                        rd,
                        rs: rd.to_reg(),
                        imm12: Imm12::from_i16(shift_bits),
                    });
                }
                insts
                    .into_iter()
                    .for_each(|i| i.emit(sink, emit_info, state));
            }

            &Inst::Call { ref info } => {
                sink.add_reloc(Reloc::RiscvCallPlt, &info.dest, 0);

                Inst::construct_auipc_and_jalr(Some(writable_link_reg()), writable_link_reg(), 0)
                    .into_iter()
                    .for_each(|i| i.emit_uncompressed(sink, emit_info, state, start_off));

                if let Some(s) = state.take_stack_map() {
                    let offset = sink.cur_offset();
                    sink.push_user_stack_map(state, offset, s);
                }

                if let Some(try_call) = info.try_call_info.as_ref() {
                    sink.add_try_call_site(try_call.exception_handlers(&state.frame_layout));
                } else {
                    sink.add_call_site();
                }

                let callee_pop_size = i32::try_from(info.callee_pop_size).unwrap();
                if callee_pop_size > 0 {
                    for inst in Riscv64MachineDeps::gen_sp_reg_adjust(-callee_pop_size) {
                        inst.emit(sink, emit_info, state);
                    }
                }

                // Load any stack-carried return values.
                info.emit_retval_loads::<Riscv64MachineDeps, _, _>(
                    state.frame_layout().stackslots_size,
                    |inst| inst.emit(sink, emit_info, state),
                    |needed_space| Some(Inst::EmitIsland { needed_space }),
                );

                // If this is a try-call, jump to the continuation
                // (normal-return) block.
                if let Some(try_call) = info.try_call_info.as_ref() {
                    let jmp = Inst::Jal {
                        label: try_call.continuation,
                    };
                    jmp.emit(sink, emit_info, state);
                }

                *start_off = sink.cur_offset();
            }
            &Inst::CallInd { ref info } => {
                Inst::Jalr {
                    rd: writable_link_reg(),
                    base: info.dest,
                    offset: Imm12::ZERO,
                }
                .emit(sink, emit_info, state);

                if let Some(s) = state.take_stack_map() {
                    let offset = sink.cur_offset();
                    sink.push_user_stack_map(state, offset, s);
                }

                if let Some(try_call) = info.try_call_info.as_ref() {
                    sink.add_try_call_site(try_call.exception_handlers(&state.frame_layout));
                } else {
                    sink.add_call_site();
                }

                let callee_pop_size = i32::try_from(info.callee_pop_size).unwrap();
                if callee_pop_size > 0 {
                    for inst in Riscv64MachineDeps::gen_sp_reg_adjust(-callee_pop_size) {
                        inst.emit(sink, emit_info, state);
                    }
                }

                // Load any stack-carried return values.
                info.emit_retval_loads::<Riscv64MachineDeps, _, _>(
                    state.frame_layout().stackslots_size,
                    |inst| inst.emit(sink, emit_info, state),
                    |needed_space| Some(Inst::EmitIsland { needed_space }),
                );

                // If this is a try-call, jump to the continuation
                // (normal-return) block.
                if let Some(try_call) = info.try_call_info.as_ref() {
                    let jmp = Inst::Jal {
                        label: try_call.continuation,
                    };
                    jmp.emit(sink, emit_info, state);
                }

                *start_off = sink.cur_offset();
            }

            &Inst::ReturnCall { ref info } => {
                emit_return_call_common_sequence(sink, emit_info, state, info);

                sink.add_call_site();
                sink.add_reloc(Reloc::RiscvCallPlt, &info.dest, 0);
                Inst::construct_auipc_and_jalr(None, writable_spilltmp_reg(), 0)
                    .into_iter()
                    .for_each(|i| i.emit_uncompressed(sink, emit_info, state, start_off));
            }

            &Inst::ReturnCallInd { ref info } => {
                emit_return_call_common_sequence(sink, emit_info, state, &info);

                Inst::Jalr {
                    rd: writable_zero_reg(),
                    base: info.dest,
                    offset: Imm12::ZERO,
                }
                .emit(sink, emit_info, state);
            }
            &Inst::Jal { label } => {
                sink.use_label_at_offset(*start_off, label, LabelUse::Jal20);
                sink.add_uncond_branch(*start_off, *start_off + 4, label);
                sink.put4(0b1101111);
                state.clobber_vstate();
            }
            &Inst::CondBr {
                taken,
                not_taken,
                kind,
            } => {
                match taken {
                    CondBrTarget::Label(label) => {
                        let code = kind.emit();
                        let code_inverse = kind.inverse().emit().to_le_bytes();
                        sink.use_label_at_offset(*start_off, label, LabelUse::B12);
                        sink.add_cond_branch(*start_off, *start_off + 4, label, &code_inverse);
                        sink.put4(code);
                    }
                    CondBrTarget::Fallthrough => panic!("Cannot fallthrough in taken target"),
                }

                match not_taken {
                    CondBrTarget::Label(label) => {
                        Inst::gen_jump(label).emit(sink, emit_info, state)
                    }
                    CondBrTarget::Fallthrough => {}
                };
            }

            &Inst::Mov { rd, rm, ty } => {
                debug_assert_eq!(rd.to_reg().class(), rm.class());
                if rd.to_reg() == rm {
                    return;
                }

                match rm.class() {
                    RegClass::Int => Inst::AluRRImm12 {
                        alu_op: AluOPRRI::Addi,
                        rd,
                        rs: rm,
                        imm12: Imm12::ZERO,
                    },
                    RegClass::Float => Inst::FpuRRR {
                        alu_op: FpuOPRRR::Fsgnj,
                        width: FpuOPWidth::try_from(ty).unwrap(),
                        frm: FRM::RNE,
                        rd,
                        rs1: rm,
                        rs2: rm,
                    },
                    RegClass::Vector => Inst::VecAluRRImm5 {
                        op: VecAluOpRRImm5::VmvrV,
                        vd: rd,
                        vs2: rm,
                        // Imm 0 means copy 1 register.
                        imm: Imm5::maybe_from_i8(0).unwrap(),
                        mask: VecOpMasking::Disabled,
                        // Vstate for this instruction is ignored.
                        vstate: VState::from_type(ty),
                    },
                }
                .emit(sink, emit_info, state);
            }

            &Inst::MovFromPReg { rd, rm } => {
                Inst::gen_move(rd, Reg::from(rm), I64).emit(sink, emit_info, state);
            }

            &Inst::BrTable {
                index,
                tmp1,
                tmp2,
                ref targets,
            } => {
                let ext_index = writable_spilltmp_reg();

                let label_compute_target = sink.get_label();

                // The default target is passed in as the 0th element of `targets`
                // separate it here for clarity.
                let default_target = targets[0];
                let targets = &targets[1..];

                // We are going to potentially emit a large amount of instructions, so ensure that we emit an island
                // now if we need one.
                //
                // The worse case PC calculations are 12 instructions. And each entry in the jump table is 2 instructions.
                // Check if we need to emit a jump table here to support that jump.
                let inst_count = 12 + (targets.len() * 2);
                let distance = (inst_count * Inst::UNCOMPRESSED_INSTRUCTION_SIZE as usize) as u32;
                if sink.island_needed(distance) {
                    let jump_around_label = sink.get_label();
                    Inst::gen_jump(jump_around_label).emit(sink, emit_info, state);
                    sink.emit_island(distance + 4, &mut state.ctrl_plane);
                    sink.bind_label(jump_around_label, &mut state.ctrl_plane);
                }

                // We emit a bounds check on the index, if the index is larger than the number of
                // jump table entries, we jump to the default block.  Otherwise we compute a jump
                // offset by multiplying the index by 8 (the size of each entry) and then jump to
                // that offset. Each jump table entry is a regular auipc+jalr which we emit sequentially.
                //
                // Build the following sequence:
                //
                // extend_index:
                //     zext.w  ext_index, index
                // bounds_check:
                //     li      tmp, n_labels
                //     bltu    ext_index, tmp, compute_target
                // jump_to_default_block:
                //     auipc   pc, 0
                //     jalr    zero, pc, default_block
                // compute_target:
                //     auipc   pc, 0
                //     slli    tmp, ext_index, 3
                //     add     pc, pc, tmp
                //     jalr    zero, pc, 0x10
                // jump_table:
                //     ; This repeats for each entry in the jumptable
                //     auipc   pc, 0
                //     jalr    zero, pc, block_target

                // Extend the index to 64 bits.
                //
                // This prevents us branching on the top 32 bits of the index, which
                // are undefined.
                Inst::Extend {
                    rd: ext_index,
                    rn: index,
                    signed: false,
                    from_bits: 32,
                    to_bits: 64,
                }
                .emit(sink, emit_info, state);

                // Bounds check.
                //
                // Check if the index passed in is larger than the number of jumptable
                // entries that we have. If it is, we fallthrough to a jump into the
                // default block.
                Inst::load_constant_u32(tmp2, targets.len() as u64)
                    .iter()
                    .for_each(|i| i.emit(sink, emit_info, state));
                Inst::CondBr {
                    taken: CondBrTarget::Label(label_compute_target),
                    not_taken: CondBrTarget::Fallthrough,
                    kind: IntegerCompare {
                        kind: IntCC::UnsignedLessThan,
                        rs1: ext_index.to_reg(),
                        rs2: tmp2.to_reg(),
                    },
                }
                .emit(sink, emit_info, state);

                sink.use_label_at_offset(sink.cur_offset(), default_target, LabelUse::PCRel32);
                Inst::construct_auipc_and_jalr(None, tmp2, 0)
                    .iter()
                    .for_each(|i| i.emit_uncompressed(sink, emit_info, state, start_off));

                // Compute the jump table offset.
                // We need to emit a PC relative offset,
                sink.bind_label(label_compute_target, &mut state.ctrl_plane);

                // Get the current PC.
                Inst::Auipc {
                    rd: tmp1,
                    imm: Imm20::ZERO,
                }
                .emit_uncompressed(sink, emit_info, state, start_off);

                // These instructions must be emitted as uncompressed since we
                // are manually computing the offset from the PC.

                // Multiply the index by 8, since that is the size in
                // bytes of each jump table entry
                Inst::AluRRImm12 {
                    alu_op: AluOPRRI::Slli,
                    rd: tmp2,
                    rs: ext_index.to_reg(),
                    imm12: Imm12::from_i16(3),
                }
                .emit_uncompressed(sink, emit_info, state, start_off);

                // Calculate the base of the jump, PC + the offset from above.
                Inst::AluRRR {
                    alu_op: AluOPRRR::Add,
                    rd: tmp1,
                    rs1: tmp1.to_reg(),
                    rs2: tmp2.to_reg(),
                }
                .emit_uncompressed(sink, emit_info, state, start_off);

                // Jump to the middle of the jump table.
                // We add a 16 byte offset here, since we used 4 instructions
                // since the AUIPC that was used to get the PC.
                Inst::Jalr {
                    rd: writable_zero_reg(),
                    base: tmp1.to_reg(),
                    offset: Imm12::from_i16((4 * Inst::UNCOMPRESSED_INSTRUCTION_SIZE) as i16),
                }
                .emit_uncompressed(sink, emit_info, state, start_off);

                // Emit the jump table.
                //
                // Each entry is a auipc + jalr to the target block. We also start with a island
                // if necessary.

                // Emit the jumps back to back
                for target in targets.iter() {
                    sink.use_label_at_offset(sink.cur_offset(), *target, LabelUse::PCRel32);

                    Inst::construct_auipc_and_jalr(None, tmp2, 0)
                        .iter()
                        .for_each(|i| i.emit_uncompressed(sink, emit_info, state, start_off));
                }

                // We've just emitted an island that is safe up to *here*.
                // Mark it as such so that we don't needlessly emit additional islands.
                *start_off = sink.cur_offset();
            }

            &Inst::Atomic {
                op,
                rd,
                addr,
                src,
                amo,
            } => {
                // TODO: get flags from original CLIF atomic instruction
                let flags = MemFlags::new();
                if let Some(trap_code) = flags.trap_code() {
                    sink.add_trap(trap_code);
                }
                let x = op.op_code()
                    | reg_to_gpr_num(rd.to_reg()) << 7
                    | op.funct3() << 12
                    | reg_to_gpr_num(addr) << 15
                    | reg_to_gpr_num(src) << 20
                    | op.funct7(amo) << 25;

                sink.put4(x);
            }
            &Inst::Fence { pred, succ } => {
                let x = 0b0001111
                    | 0b00000 << 7
                    | 0b000 << 12
                    | 0b00000 << 15
                    | (succ as u32) << 20
                    | (pred as u32) << 24;

                sink.put4(x);
            }
            &Inst::Auipc { rd, imm } => {
                sink.put4(enc_auipc(rd, imm));
            }

            &Inst::LoadAddr { rd, mem } => {
                let base = mem.get_base_register();
                let offset = mem.get_offset_with_state(state);
                let offset_imm12 = Imm12::maybe_from_i64(offset);

                match (mem, base, offset_imm12) {
                    (_, Some(rs), Some(imm12)) => {
                        Inst::AluRRImm12 {
                            alu_op: AluOPRRI::Addi,
                            rd,
                            rs,
                            imm12,
                        }
                        .emit(sink, emit_info, state);
                    }
                    (_, Some(rs), None) => {
                        let mut insts = Inst::load_constant_u64(rd, offset as u64);
                        insts.push(Inst::AluRRR {
                            alu_op: AluOPRRR::Add,
                            rd,
                            rs1: rd.to_reg(),
                            rs2: rs,
                        });
                        insts
                            .into_iter()
                            .for_each(|inst| inst.emit(sink, emit_info, state));
                    }
                    (AMode::Const(addr), None, _) => {
                        // Get an address label for the constant and recurse.
                        let label = sink.get_label_for_constant(addr);
                        Inst::LoadAddr {
                            rd,
                            mem: AMode::Label(label),
                        }
                        .emit(sink, emit_info, state);
                    }
                    (AMode::Label(label), None, _) => {
                        // Get the current PC.
                        sink.use_label_at_offset(sink.cur_offset(), label, LabelUse::PCRelHi20);
                        let inst = Inst::Auipc {
                            rd,
                            imm: Imm20::ZERO,
                        };
                        inst.emit_uncompressed(sink, emit_info, state, start_off);

                        // Emit an add to the address with a relocation.
                        // This later gets patched up with the correct offset.
                        sink.use_label_at_offset(sink.cur_offset(), label, LabelUse::PCRelLo12I);
                        Inst::AluRRImm12 {
                            alu_op: AluOPRRI::Addi,
                            rd,
                            rs: rd.to_reg(),
                            imm12: Imm12::ZERO,
                        }
                        .emit_uncompressed(sink, emit_info, state, start_off);
                    }
                    (amode, _, _) => {
                        unimplemented!("LoadAddr: {:?}", amode);
                    }
                }
            }

            &Inst::Select {
                ref dst,
                condition,
                ref x,
                ref y,
            } => {
                // The general form for this select is the following:
                //
                //     mv rd, x
                //     b{cond} rcond, label_end
                //     mv rd, y
                // label_end:
                //     ... etc
                //
                // This is built on the assumption that moves are cheap, but branches and jumps
                // are not. So with this format we always avoid one jump instruction at the expense
                // of an unconditional move.
                //
                // We also perform another optimization here. If the destination register is the same
                // as one of the input registers, we can avoid emitting the first unconditional move
                // and emit just the branch and the second move.
                //
                // To make sure that this happens as often as possible, we also try to invert the
                // condition, so that if either of the input registers are the same as the destination
                // we avoid that move.

                let label_end = sink.get_label();

                let xregs = x.regs();
                let yregs = y.regs();
                let dstregs: Vec<Reg> = dst.regs().into_iter().map(|r| r.to_reg()).collect();
                let condregs = condition.regs();

                // We are going to write to the destination register before evaluating
                // the condition, so we need to make sure that the destination register
                // is not one of the condition registers.
                //
                // This should never happen, since hopefully the regalloc constraints
                // for this register are set up correctly.
                debug_assert_ne!(dstregs, condregs);

                // Check if we can invert the condition and avoid moving the y registers into
                // the destination. This allows us to only emit the branch and one of the moves.
                let (uncond_move, cond_move, condition) = if yregs == dstregs {
                    (yregs, xregs, condition.inverse())
                } else {
                    (xregs, yregs, condition)
                };

                // Unconditionally move one of the values to the destination register.
                //
                // These moves may not end up being emitted if the source and
                // destination registers are the same. That logic is built into
                // the emit function for `Inst::Mov`.
                for i in gen_moves(dst.regs(), uncond_move) {
                    i.emit(sink, emit_info, state);
                }

                // If the condition passes we skip over the conditional move
                Inst::CondBr {
                    taken: CondBrTarget::Label(label_end),
                    not_taken: CondBrTarget::Fallthrough,
                    kind: condition,
                }
                .emit(sink, emit_info, state);

                // Move the conditional value to the destination register.
                for i in gen_moves(dst.regs(), cond_move) {
                    i.emit(sink, emit_info, state);
                }

                sink.bind_label(label_end, &mut state.ctrl_plane);
            }
            &Inst::Jalr { rd, base, offset } => {
                sink.put4(enc_jalr(rd, base, offset));
                state.clobber_vstate();
            }
            &Inst::EBreak => {
                sink.put4(0x00100073);
            }
            &Inst::AtomicCas {
                offset,
                t0,
                dst,
                e,
                addr,
                v,
                ty,
            } => {
                //     # addr holds address of memory location
                //     # e holds expected value
                //     # v holds desired value
                //     # dst holds return value
                // cas:
                //     lr.w dst, (addr)       # Load original value.
                //     bne dst, e, fail       # Doesn’t match, so fail.
                //     sc.w t0, v, (addr)     # Try to update.
                //     bnez t0 , cas          # if store not ok,retry.
                // fail:
                let fail_label = sink.get_label();
                let cas_lebel = sink.get_label();
                sink.bind_label(cas_lebel, &mut state.ctrl_plane);
                Inst::Atomic {
                    op: AtomicOP::load_op(ty),
                    rd: dst,
                    addr,
                    src: zero_reg(),
                    amo: AMO::SeqCst,
                }
                .emit(sink, emit_info, state);
                if ty.bits() < 32 {
                    AtomicOP::extract(dst, offset, dst.to_reg(), ty)
                        .iter()
                        .for_each(|i| i.emit(sink, emit_info, state));
                } else if ty.bits() == 32 {
                    Inst::Extend {
                        rd: dst,
                        rn: dst.to_reg(),
                        signed: false,
                        from_bits: 32,
                        to_bits: 64,
                    }
                    .emit(sink, emit_info, state);
                }
                Inst::CondBr {
                    taken: CondBrTarget::Label(fail_label),
                    not_taken: CondBrTarget::Fallthrough,
                    kind: IntegerCompare {
                        kind: IntCC::NotEqual,
                        rs1: e,
                        rs2: dst.to_reg(),
                    },
                }
                .emit(sink, emit_info, state);
                let store_value = if ty.bits() < 32 {
                    // reload value to t0.
                    Inst::Atomic {
                        op: AtomicOP::load_op(ty),
                        rd: t0,
                        addr,
                        src: zero_reg(),
                        amo: AMO::SeqCst,
                    }
                    .emit(sink, emit_info, state);
                    // set reset part.
                    AtomicOP::merge(t0, writable_spilltmp_reg(), offset, v, ty)
                        .iter()
                        .for_each(|i| i.emit(sink, emit_info, state));
                    t0.to_reg()
                } else {
                    v
                };
                Inst::Atomic {
                    op: AtomicOP::store_op(ty),
                    rd: t0,
                    addr,
                    src: store_value,
                    amo: AMO::SeqCst,
                }
                .emit(sink, emit_info, state);
                // check is our value stored.
                Inst::CondBr {
                    taken: CondBrTarget::Label(cas_lebel),
                    not_taken: CondBrTarget::Fallthrough,
                    kind: IntegerCompare {
                        kind: IntCC::NotEqual,
                        rs1: t0.to_reg(),
                        rs2: zero_reg(),
                    },
                }
                .emit(sink, emit_info, state);
                sink.bind_label(fail_label, &mut state.ctrl_plane);
            }
            &Inst::AtomicRmwLoop {
                offset,
                op,
                dst,
                ty,
                p,
                x,
                t0,
            } => {
                let retry = sink.get_label();
                sink.bind_label(retry, &mut state.ctrl_plane);
                // load old value.
                Inst::Atomic {
                    op: AtomicOP::load_op(ty),
                    rd: dst,
                    addr: p,
                    src: zero_reg(),
                    amo: AMO::SeqCst,
                }
                .emit(sink, emit_info, state);
                //

                let store_value: Reg = match op {
                    crate::ir::AtomicRmwOp::Add
                    | crate::ir::AtomicRmwOp::Sub
                    | crate::ir::AtomicRmwOp::And
                    | crate::ir::AtomicRmwOp::Or
                    | crate::ir::AtomicRmwOp::Xor => {
                        AtomicOP::extract(dst, offset, dst.to_reg(), ty)
                            .iter()
                            .for_each(|i| i.emit(sink, emit_info, state));
                        Inst::AluRRR {
                            alu_op: match op {
                                crate::ir::AtomicRmwOp::Add => AluOPRRR::Add,
                                crate::ir::AtomicRmwOp::Sub => AluOPRRR::Sub,
                                crate::ir::AtomicRmwOp::And => AluOPRRR::And,
                                crate::ir::AtomicRmwOp::Or => AluOPRRR::Or,
                                crate::ir::AtomicRmwOp::Xor => AluOPRRR::Xor,
                                _ => unreachable!(),
                            },
                            rd: t0,
                            rs1: dst.to_reg(),
                            rs2: x,
                        }
                        .emit(sink, emit_info, state);
                        Inst::Atomic {
                            op: AtomicOP::load_op(ty),
                            rd: writable_spilltmp_reg2(),
                            addr: p,
                            src: zero_reg(),
                            amo: AMO::SeqCst,
                        }
                        .emit(sink, emit_info, state);
                        AtomicOP::merge(
                            writable_spilltmp_reg2(),
                            writable_spilltmp_reg(),
                            offset,
                            t0.to_reg(),
                            ty,
                        )
                        .iter()
                        .for_each(|i| i.emit(sink, emit_info, state));
                        spilltmp_reg2()
                    }
                    crate::ir::AtomicRmwOp::Nand => {
                        if ty.bits() < 32 {
                            AtomicOP::extract(dst, offset, dst.to_reg(), ty)
                                .iter()
                                .for_each(|i| i.emit(sink, emit_info, state));
                        }
                        Inst::AluRRR {
                            alu_op: AluOPRRR::And,
                            rd: t0,
                            rs1: x,
                            rs2: dst.to_reg(),
                        }
                        .emit(sink, emit_info, state);
                        Inst::construct_bit_not(t0, t0.to_reg()).emit(sink, emit_info, state);
                        if ty.bits() < 32 {
                            Inst::Atomic {
                                op: AtomicOP::load_op(ty),
                                rd: writable_spilltmp_reg2(),
                                addr: p,
                                src: zero_reg(),
                                amo: AMO::SeqCst,
                            }
                            .emit(sink, emit_info, state);
                            AtomicOP::merge(
                                writable_spilltmp_reg2(),
                                writable_spilltmp_reg(),
                                offset,
                                t0.to_reg(),
                                ty,
                            )
                            .iter()
                            .for_each(|i| i.emit(sink, emit_info, state));
                            spilltmp_reg2()
                        } else {
                            t0.to_reg()
                        }
                    }

                    crate::ir::AtomicRmwOp::Umin
                    | crate::ir::AtomicRmwOp::Umax
                    | crate::ir::AtomicRmwOp::Smin
                    | crate::ir::AtomicRmwOp::Smax => {
                        let label_select_dst = sink.get_label();
                        let label_select_done = sink.get_label();
                        if op == crate::ir::AtomicRmwOp::Umin || op == crate::ir::AtomicRmwOp::Umax
                        {
                            AtomicOP::extract(dst, offset, dst.to_reg(), ty)
                        } else {
                            AtomicOP::extract_sext(dst, offset, dst.to_reg(), ty)
                        }
                        .iter()
                        .for_each(|i| i.emit(sink, emit_info, state));

                        Inst::CondBr {
                            taken: CondBrTarget::Label(label_select_dst),
                            not_taken: CondBrTarget::Fallthrough,
                            kind: IntegerCompare {
                                kind: match op {
                                    crate::ir::AtomicRmwOp::Umin => IntCC::UnsignedLessThan,
                                    crate::ir::AtomicRmwOp::Umax => IntCC::UnsignedGreaterThan,
                                    crate::ir::AtomicRmwOp::Smin => IntCC::SignedLessThan,
                                    crate::ir::AtomicRmwOp::Smax => IntCC::SignedGreaterThan,
                                    _ => unreachable!(),
                                },
                                rs1: dst.to_reg(),
                                rs2: x,
                            },
                        }
                        .emit(sink, emit_info, state);
                        // here we select x.
                        Inst::gen_move(t0, x, I64).emit(sink, emit_info, state);
                        Inst::gen_jump(label_select_done).emit(sink, emit_info, state);
                        sink.bind_label(label_select_dst, &mut state.ctrl_plane);
                        Inst::gen_move(t0, dst.to_reg(), I64).emit(sink, emit_info, state);
                        sink.bind_label(label_select_done, &mut state.ctrl_plane);
                        Inst::Atomic {
                            op: AtomicOP::load_op(ty),
                            rd: writable_spilltmp_reg2(),
                            addr: p,
                            src: zero_reg(),
                            amo: AMO::SeqCst,
                        }
                        .emit(sink, emit_info, state);
                        AtomicOP::merge(
                            writable_spilltmp_reg2(),
                            writable_spilltmp_reg(),
                            offset,
                            t0.to_reg(),
                            ty,
                        )
                        .iter()
                        .for_each(|i| i.emit(sink, emit_info, state));
                        spilltmp_reg2()
                    }
                    crate::ir::AtomicRmwOp::Xchg => {
                        AtomicOP::extract(dst, offset, dst.to_reg(), ty)
                            .iter()
                            .for_each(|i| i.emit(sink, emit_info, state));
                        Inst::Atomic {
                            op: AtomicOP::load_op(ty),
                            rd: writable_spilltmp_reg2(),
                            addr: p,
                            src: zero_reg(),
                            amo: AMO::SeqCst,
                        }
                        .emit(sink, emit_info, state);
                        AtomicOP::merge(
                            writable_spilltmp_reg2(),
                            writable_spilltmp_reg(),
                            offset,
                            x,
                            ty,
                        )
                        .iter()
                        .for_each(|i| i.emit(sink, emit_info, state));
                        spilltmp_reg2()
                    }
                };

                Inst::Atomic {
                    op: AtomicOP::store_op(ty),
                    rd: t0,
                    addr: p,
                    src: store_value,
                    amo: AMO::SeqCst,
                }
                .emit(sink, emit_info, state);

                // if store is not ok,retry.
                Inst::CondBr {
                    taken: CondBrTarget::Label(retry),
                    not_taken: CondBrTarget::Fallthrough,
                    kind: IntegerCompare {
                        kind: IntCC::NotEqual,
                        rs1: t0.to_reg(),
                        rs2: zero_reg(),
                    },
                }
                .emit(sink, emit_info, state);
            }

            &Inst::LoadExtName {
                rd,
                ref name,
                offset,
            } => {
                if emit_info.shared_flag.is_pic() {
                    // Load a PC-relative address into a register.
                    // RISC-V does this slightly differently from other arches. We emit a relocation
                    // with a label, instead of the symbol itself.
                    //
                    // See: https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#pc-relative-symbol-addresses
                    //
                    // Emit the following code:
                    // label:
                    //   auipc rd, 0              # R_RISCV_GOT_HI20 (symbol_name)
                    //   ld    rd, rd, 0          # R_RISCV_PCREL_LO12_I (label)

                    // Create the label that is going to be published to the final binary object.
                    let auipc_label = sink.get_label();
                    sink.bind_label(auipc_label, &mut state.ctrl_plane);

                    // Get the current PC.
                    sink.add_reloc(Reloc::RiscvGotHi20, &**name, 0);
                    Inst::Auipc {
                        rd,
                        imm: Imm20::from_i32(0),
                    }
                    .emit_uncompressed(sink, emit_info, state, start_off);

                    // The `ld` here, points to the `auipc` label instead of directly to the symbol.
                    sink.add_reloc(Reloc::RiscvPCRelLo12I, &auipc_label, 0);
                    Inst::Load {
                        rd,
                        op: LoadOP::Ld,
                        flags: MemFlags::trusted(),
                        from: AMode::RegOffset(rd.to_reg(), 0),
                    }
                    .emit_uncompressed(sink, emit_info, state, start_off);
                } else {
                    // In the non PIC sequence we relocate the absolute address into
                    // a prealocatted space, load it into a register and jump over it.
                    //
                    // Emit the following code:
                    //   ld rd, label_data
                    //   j label_end
                    // label_data:
                    //   <8 byte space>           # ABS8
                    // label_end:

                    let label_data = sink.get_label();
                    let label_end = sink.get_label();

                    // Load the value from a label
                    Inst::Load {
                        rd,
                        op: LoadOP::Ld,
                        flags: MemFlags::trusted(),
                        from: AMode::Label(label_data),
                    }
                    .emit(sink, emit_info, state);

                    // Jump over the data
                    Inst::gen_jump(label_end).emit(sink, emit_info, state);

                    sink.bind_label(label_data, &mut state.ctrl_plane);
                    sink.add_reloc(Reloc::Abs8, name.as_ref(), offset);
                    sink.put8(0);

                    sink.bind_label(label_end, &mut state.ctrl_plane);
                }
            }

            &Inst::ElfTlsGetAddr { rd, ref name } => {
                // RISC-V's TLS GD model is slightly different from other arches.
                //
                // We have a relocation (R_RISCV_TLS_GD_HI20) that loads the high 20 bits
                // of the address relative to the GOT entry. This relocation points to
                // the symbol as usual.
                //
                // However when loading the bottom 12bits of the address, we need to
                // use a label that points to the previous AUIPC instruction.
                //
                // label:
                //    auipc a0,0                    # R_RISCV_TLS_GD_HI20 (symbol)
                //    addi  a0,a0,0                 # R_RISCV_PCREL_LO12_I (label)
                //
                // https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc#global-dynamic

                // Create the label that is going to be published to the final binary object.
                let auipc_label = sink.get_label();
                sink.bind_label(auipc_label, &mut state.ctrl_plane);

                // Get the current PC.
                sink.add_reloc(Reloc::RiscvTlsGdHi20, &**name, 0);
                Inst::Auipc {
                    rd,
                    imm: Imm20::from_i32(0),
                }
                .emit_uncompressed(sink, emit_info, state, start_off);

                // The `addi` here, points to the `auipc` label instead of directly to the symbol.
                sink.add_reloc(Reloc::RiscvPCRelLo12I, &auipc_label, 0);
                Inst::AluRRImm12 {
                    alu_op: AluOPRRI::Addi,
                    rd,
                    rs: rd.to_reg(),
                    imm12: Imm12::from_i16(0),
                }
                .emit_uncompressed(sink, emit_info, state, start_off);

                Inst::Call {
                    info: Box::new(CallInfo::empty(
                        ExternalName::LibCall(LibCall::ElfTlsGetAddr),
                        CallConv::SystemV,
                    )),
                }
                .emit_uncompressed(sink, emit_info, state, start_off);
            }

            &Inst::TrapIf {
                rs1,
                rs2,
                cc,
                trap_code,
            } => {
                let label_end = sink.get_label();
                let cond = IntegerCompare { kind: cc, rs1, rs2 };

                // Jump over the trap if we the condition is false.
                Inst::CondBr {
                    taken: CondBrTarget::Label(label_end),
                    not_taken: CondBrTarget::Fallthrough,
                    kind: cond.inverse(),
                }
                .emit(sink, emit_info, state);
                Inst::Udf { trap_code }.emit(sink, emit_info, state);

                sink.bind_label(label_end, &mut state.ctrl_plane);
            }
            &Inst::Udf { trap_code } => {
                sink.add_trap(trap_code);
                sink.put_data(Inst::TRAP_OPCODE);
            }
            &Inst::AtomicLoad { rd, ty, p } => {
                // emit the fence.
                Inst::Fence {
                    pred: Inst::FENCE_REQ_R | Inst::FENCE_REQ_W,
                    succ: Inst::FENCE_REQ_R | Inst::FENCE_REQ_W,
                }
                .emit(sink, emit_info, state);
                // load.
                Inst::Load {
                    rd,
                    op: LoadOP::from_type(ty),
                    flags: MemFlags::new(),
                    from: AMode::RegOffset(p, 0),
                }
                .emit(sink, emit_info, state);
                Inst::Fence {
                    pred: Inst::FENCE_REQ_R,
                    succ: Inst::FENCE_REQ_R | Inst::FENCE_REQ_W,
                }
                .emit(sink, emit_info, state);
            }
            &Inst::AtomicStore { src, ty, p } => {
                Inst::Fence {
                    pred: Inst::FENCE_REQ_R | Inst::FENCE_REQ_W,
                    succ: Inst::FENCE_REQ_W,
                }
                .emit(sink, emit_info, state);
                Inst::Store {
                    to: AMode::RegOffset(p, 0),
                    op: StoreOP::from_type(ty),
                    flags: MemFlags::new(),
                    src,
                }
                .emit(sink, emit_info, state);
            }

            &Inst::Popcnt {
                sum,
                tmp,
                step,
                rs,
                ty,
            } => {
                // load 0 to sum , init.
                Inst::gen_move(sum, zero_reg(), I64).emit(sink, emit_info, state);
                // load
                Inst::load_imm12(step, Imm12::from_i16(ty.bits() as i16))
                    .emit(sink, emit_info, state);
                //
                Inst::load_imm12(tmp, Imm12::ONE).emit(sink, emit_info, state);
                Inst::AluRRImm12 {
                    alu_op: AluOPRRI::Slli,
                    rd: tmp,
                    rs: tmp.to_reg(),
                    imm12: Imm12::from_i16((ty.bits() - 1) as i16),
                }
                .emit(sink, emit_info, state);
                let label_done = sink.get_label();
                let label_loop = sink.get_label();
                sink.bind_label(label_loop, &mut state.ctrl_plane);
                Inst::CondBr {
                    taken: CondBrTarget::Label(label_done),
                    not_taken: CondBrTarget::Fallthrough,
                    kind: IntegerCompare {
                        kind: IntCC::SignedLessThanOrEqual,
                        rs1: step.to_reg(),
                        rs2: zero_reg(),
                    },
                }
                .emit(sink, emit_info, state);
                // test and add sum.
                {
                    Inst::AluRRR {
                        alu_op: AluOPRRR::And,
                        rd: writable_spilltmp_reg2(),
                        rs1: tmp.to_reg(),
                        rs2: rs,
                    }
                    .emit(sink, emit_info, state);
                    let label_over = sink.get_label();
                    Inst::CondBr {
                        taken: CondBrTarget::Label(label_over),
                        not_taken: CondBrTarget::Fallthrough,
                        kind: IntegerCompare {
                            kind: IntCC::Equal,
                            rs1: zero_reg(),
                            rs2: spilltmp_reg2(),
                        },
                    }
                    .emit(sink, emit_info, state);
                    Inst::AluRRImm12 {
                        alu_op: AluOPRRI::Addi,
                        rd: sum,
                        rs: sum.to_reg(),
                        imm12: Imm12::ONE,
                    }
                    .emit(sink, emit_info, state);
                    sink.bind_label(label_over, &mut state.ctrl_plane);
                }
                // set step and tmp.
                {
                    Inst::AluRRImm12 {
                        alu_op: AluOPRRI::Addi,
                        rd: step,
                        rs: step.to_reg(),
                        imm12: Imm12::from_i16(-1),
                    }
                    .emit(sink, emit_info, state);
                    Inst::AluRRImm12 {
                        alu_op: AluOPRRI::Srli,
                        rd: tmp,
                        rs: tmp.to_reg(),
                        imm12: Imm12::ONE,
                    }
                    .emit(sink, emit_info, state);
                    Inst::gen_jump(label_loop).emit(sink, emit_info, state);
                }
                sink.bind_label(label_done, &mut state.ctrl_plane);
            }
            &Inst::Cltz {
                sum,
                tmp,
                step,
                rs,
                leading,
                ty,
            } => {
                // load 0 to sum , init.
                Inst::gen_move(sum, zero_reg(), I64).emit(sink, emit_info, state);
                // load
                Inst::load_imm12(step, Imm12::from_i16(ty.bits() as i16))
                    .emit(sink, emit_info, state);
                //
                Inst::load_imm12(tmp, Imm12::ONE).emit(sink, emit_info, state);
                if leading {
                    Inst::AluRRImm12 {
                        alu_op: AluOPRRI::Slli,
                        rd: tmp,
                        rs: tmp.to_reg(),
                        imm12: Imm12::from_i16((ty.bits() - 1) as i16),
                    }
                    .emit(sink, emit_info, state);
                }
                let label_done = sink.get_label();
                let label_loop = sink.get_label();
                sink.bind_label(label_loop, &mut state.ctrl_plane);
                Inst::CondBr {
                    taken: CondBrTarget::Label(label_done),
                    not_taken: CondBrTarget::Fallthrough,
                    kind: IntegerCompare {
                        kind: IntCC::SignedLessThanOrEqual,
                        rs1: step.to_reg(),
                        rs2: zero_reg(),
                    },
                }
                .emit(sink, emit_info, state);
                // test and add sum.
                {
                    Inst::AluRRR {
                        alu_op: AluOPRRR::And,
                        rd: writable_spilltmp_reg2(),
                        rs1: tmp.to_reg(),
                        rs2: rs,
                    }
                    .emit(sink, emit_info, state);
                    Inst::CondBr {
                        taken: CondBrTarget::Label(label_done),
                        not_taken: CondBrTarget::Fallthrough,
                        kind: IntegerCompare {
                            kind: IntCC::NotEqual,
                            rs1: zero_reg(),
                            rs2: spilltmp_reg2(),
                        },
                    }
                    .emit(sink, emit_info, state);
                    Inst::AluRRImm12 {
                        alu_op: AluOPRRI::Addi,
                        rd: sum,
                        rs: sum.to_reg(),
                        imm12: Imm12::ONE,
                    }
                    .emit(sink, emit_info, state);
                }
                // set step and tmp.
                {
                    Inst::AluRRImm12 {
                        alu_op: AluOPRRI::Addi,
                        rd: step,
                        rs: step.to_reg(),
                        imm12: Imm12::from_i16(-1),
                    }
                    .emit(sink, emit_info, state);
                    Inst::AluRRImm12 {
                        alu_op: if leading {
                            AluOPRRI::Srli
                        } else {
                            AluOPRRI::Slli
                        },
                        rd: tmp,
                        rs: tmp.to_reg(),
                        imm12: Imm12::ONE,
                    }
                    .emit(sink, emit_info, state);
                    Inst::gen_jump(label_loop).emit(sink, emit_info, state);
                }
                sink.bind_label(label_done, &mut state.ctrl_plane);
            }
            &Inst::Brev8 {
                rs,
                ty,
                step,
                tmp,
                tmp2,
                rd,
            } => {
                Inst::gen_move(rd, zero_reg(), I64).emit(sink, emit_info, state);
                Inst::load_imm12(step, Imm12::from_i16(ty.bits() as i16))
                    .emit(sink, emit_info, state);
                //
                Inst::load_imm12(tmp, Imm12::ONE).emit(sink, emit_info, state);
                Inst::AluRRImm12 {
                    alu_op: AluOPRRI::Slli,
                    rd: tmp,
                    rs: tmp.to_reg(),
                    imm12: Imm12::from_i16((ty.bits() - 1) as i16),
                }
                .emit(sink, emit_info, state);
                Inst::load_imm12(tmp2, Imm12::ONE).emit(sink, emit_info, state);
                Inst::AluRRImm12 {
                    alu_op: AluOPRRI::Slli,
                    rd: tmp2,
                    rs: tmp2.to_reg(),
                    imm12: Imm12::from_i16((ty.bits() - 8) as i16),
                }
                .emit(sink, emit_info, state);

                let label_done = sink.get_label();
                let label_loop = sink.get_label();
                sink.bind_label(label_loop, &mut state.ctrl_plane);
                Inst::CondBr {
                    taken: CondBrTarget::Label(label_done),
                    not_taken: CondBrTarget::Fallthrough,
                    kind: IntegerCompare {
                        kind: IntCC::SignedLessThanOrEqual,
                        rs1: step.to_reg(),
                        rs2: zero_reg(),
                    },
                }
                .emit(sink, emit_info, state);
                // test and set bit.
                {
                    Inst::AluRRR {
                        alu_op: AluOPRRR::And,
                        rd: writable_spilltmp_reg2(),
                        rs1: tmp.to_reg(),
                        rs2: rs,
                    }
                    .emit(sink, emit_info, state);
                    let label_over = sink.get_label();
                    Inst::CondBr {
                        taken: CondBrTarget::Label(label_over),
                        not_taken: CondBrTarget::Fallthrough,
                        kind: IntegerCompare {
                            kind: IntCC::Equal,
                            rs1: zero_reg(),
                            rs2: spilltmp_reg2(),
                        },
                    }
                    .emit(sink, emit_info, state);
                    Inst::AluRRR {
                        alu_op: AluOPRRR::Or,
                        rd,
                        rs1: rd.to_reg(),
                        rs2: tmp2.to_reg(),
                    }
                    .emit(sink, emit_info, state);
                    sink.bind_label(label_over, &mut state.ctrl_plane);
                }
                // set step and tmp.
                {
                    Inst::AluRRImm12 {
                        alu_op: AluOPRRI::Addi,
                        rd: step,
                        rs: step.to_reg(),
                        imm12: Imm12::from_i16(-1),
                    }
                    .emit(sink, emit_info, state);
                    Inst::AluRRImm12 {
                        alu_op: AluOPRRI::Srli,
                        rd: tmp,
                        rs: tmp.to_reg(),
                        imm12: Imm12::ONE,
                    }
                    .emit(sink, emit_info, state);
                    {
                        // reset tmp2
                        // if (step %=8 == 0) then tmp2 = tmp2 >> 15
                        // if (step %=8 != 0) then tmp2 = tmp2 << 1
                        let label_over = sink.get_label();
                        let label_sll_1 = sink.get_label();
                        Inst::load_imm12(writable_spilltmp_reg2(), Imm12::from_i16(8))
                            .emit(sink, emit_info, state);
                        Inst::AluRRR {
                            alu_op: AluOPRRR::Rem,
                            rd: writable_spilltmp_reg2(),
                            rs1: step.to_reg(),
                            rs2: spilltmp_reg2(),
                        }
                        .emit(sink, emit_info, state);
                        Inst::CondBr {
                            taken: CondBrTarget::Label(label_sll_1),
                            not_taken: CondBrTarget::Fallthrough,
                            kind: IntegerCompare {
                                kind: IntCC::NotEqual,
                                rs1: spilltmp_reg2(),
                                rs2: zero_reg(),
                            },
                        }
                        .emit(sink, emit_info, state);
                        Inst::AluRRImm12 {
                            alu_op: AluOPRRI::Srli,
                            rd: tmp2,
                            rs: tmp2.to_reg(),
                            imm12: Imm12::from_i16(15),
                        }
                        .emit(sink, emit_info, state);
                        Inst::gen_jump(label_over).emit(sink, emit_info, state);
                        sink.bind_label(label_sll_1, &mut state.ctrl_plane);
                        Inst::AluRRImm12 {
                            alu_op: AluOPRRI::Slli,
                            rd: tmp2,
                            rs: tmp2.to_reg(),
                            imm12: Imm12::ONE,
                        }
                        .emit(sink, emit_info, state);
                        sink.bind_label(label_over, &mut state.ctrl_plane);
                    }
                    Inst::gen_jump(label_loop).emit(sink, emit_info, state);
                }
                sink.bind_label(label_done, &mut state.ctrl_plane);
            }
            &Inst::StackProbeLoop {
                guard_size,
                probe_count,
                tmp: guard_size_tmp,
            } => {
                let step = writable_spilltmp_reg();
                Inst::load_constant_u64(step, (guard_size as u64) * (probe_count as u64))
                    .iter()
                    .for_each(|i| i.emit(sink, emit_info, state));
                Inst::load_constant_u64(guard_size_tmp, guard_size as u64)
                    .iter()
                    .for_each(|i| i.emit(sink, emit_info, state));

                let loop_start = sink.get_label();
                let label_done = sink.get_label();
                sink.bind_label(loop_start, &mut state.ctrl_plane);
                Inst::CondBr {
                    taken: CondBrTarget::Label(label_done),
                    not_taken: CondBrTarget::Fallthrough,
                    kind: IntegerCompare {
                        kind: IntCC::UnsignedLessThanOrEqual,
                        rs1: step.to_reg(),
                        rs2: guard_size_tmp.to_reg(),
                    },
                }
                .emit(sink, emit_info, state);
                // compute address.
                Inst::AluRRR {
                    alu_op: AluOPRRR::Sub,
                    rd: writable_spilltmp_reg2(),
                    rs1: stack_reg(),
                    rs2: step.to_reg(),
                }
                .emit(sink, emit_info, state);
                Inst::Store {
                    to: AMode::RegOffset(spilltmp_reg2(), 0),
                    op: StoreOP::Sb,
                    flags: MemFlags::new(),
                    src: zero_reg(),
                }
                .emit(sink, emit_info, state);
                // reset step.
                Inst::AluRRR {
                    alu_op: AluOPRRR::Sub,
                    rd: step,
                    rs1: step.to_reg(),
                    rs2: guard_size_tmp.to_reg(),
                }
                .emit(sink, emit_info, state);
                Inst::gen_jump(loop_start).emit(sink, emit_info, state);
                sink.bind_label(label_done, &mut state.ctrl_plane);
            }
            &Inst::VecAluRRRImm5 {
                op,
                vd,
                vd_src,
                imm,
                vs2,
                ref mask,
                ..
            } => {
                debug_assert_eq!(vd.to_reg(), vd_src);

                sink.put4(encode_valu_rrr_imm(op, vd, imm, vs2, *mask));
            }
            &Inst::VecAluRRRR {
                op,
                vd,
                vd_src,
                vs1,
                vs2,
                ref mask,
                ..
            } => {
                debug_assert_eq!(vd.to_reg(), vd_src);

                sink.put4(encode_valu_rrrr(op, vd, vs2, vs1, *mask));
            }
            &Inst::VecAluRRR {
                op,
                vd,
                vs1,
                vs2,
                ref mask,
                ..
            } => {
                sink.put4(encode_valu(op, vd, vs1, vs2, *mask));
            }
            &Inst::VecAluRRImm5 {
                op,
                vd,
                imm,
                vs2,
                ref mask,
                ..
            } => {
                sink.put4(encode_valu_rr_imm(op, vd, imm, vs2, *mask));
            }
            &Inst::VecAluRR {
                op,
                vd,
                vs,
                ref mask,
                ..
            } => {
                sink.put4(encode_valu_rr(op, vd, vs, *mask));
            }
            &Inst::VecAluRImm5 {
                op,
                vd,
                imm,
                ref mask,
                ..
            } => {
                sink.put4(encode_valu_r_imm(op, vd, imm, *mask));
            }
            &Inst::VecSetState { rd, ref vstate } => {
                sink.put4(encode_vcfg_imm(
                    0x57,
                    rd.to_reg(),
                    vstate.avl.unwrap_static(),
                    &vstate.vtype,
                ));

                // Update the current vector emit state.
                state.vstate = EmitVState::Known(*vstate);
            }

            &Inst::VecLoad {
                eew,
                to,
                ref from,
                ref mask,
                flags,
                ..
            } => {
                // Vector Loads don't support immediate offsets, so we need to load it into a register.
                let addr = match from {
                    VecAMode::UnitStride { base } => {
                        let base_reg = base.get_base_register();
                        let offset = base.get_offset_with_state(state);

                        // Reg+0 Offset can be directly encoded
                        if let (Some(base_reg), 0) = (base_reg, offset) {
                            base_reg
                        } else {
                            // Otherwise load the address it into a reg and load from it.
                            let tmp = writable_spilltmp_reg();
                            Inst::LoadAddr {
                                rd: tmp,
                                mem: *base,
                            }
                            .emit(sink, emit_info, state);
                            tmp.to_reg()
                        }
                    }
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }

                sink.put4(encode_vmem_load(
                    0x07,
                    to.to_reg(),
                    eew,
                    addr,
                    from.lumop(),
                    *mask,
                    from.mop(),
                    from.nf(),
                ));
            }

            &Inst::VecStore {
                eew,
                ref to,
                from,
                ref mask,
                flags,
                ..
            } => {
                // Vector Stores don't support immediate offsets, so we need to load it into a register.
                let addr = match to {
                    VecAMode::UnitStride { base } => {
                        let base_reg = base.get_base_register();
                        let offset = base.get_offset_with_state(state);

                        // Reg+0 Offset can be directly encoded
                        if let (Some(base_reg), 0) = (base_reg, offset) {
                            base_reg
                        } else {
                            // Otherwise load the address it into a reg and load from it.
                            let tmp = writable_spilltmp_reg();
                            Inst::LoadAddr {
                                rd: tmp,
                                mem: *base,
                            }
                            .emit(sink, emit_info, state);
                            tmp.to_reg()
                        }
                    }
                };

                if let Some(trap_code) = flags.trap_code() {
                    // Register the offset at which the actual load instruction starts.
                    sink.add_trap(trap_code);
                }

                sink.put4(encode_vmem_store(
                    0x27,
                    from,
                    eew,
                    addr,
                    to.sumop(),
                    *mask,
                    to.mop(),
                    to.nf(),
                ));
            }

            Inst::EmitIsland { needed_space } => {
                if sink.island_needed(*needed_space) {
                    let jump_around_label = sink.get_label();
                    Inst::gen_jump(jump_around_label).emit(sink, emit_info, state);
                    sink.emit_island(needed_space + 4, &mut state.ctrl_plane);
                    sink.bind_label(jump_around_label, &mut state.ctrl_plane);
                }
            }
        }
    }
}

fn emit_return_call_common_sequence<T>(
    sink: &mut MachBuffer<Inst>,
    emit_info: &EmitInfo,
    state: &mut EmitState,
    info: &ReturnCallInfo<T>,
) {
    // The return call sequence can potentially emit a lot of instructions (up to 634 bytes!)
    // So lets emit an island here if we need it.
    //
    // It is difficult to calculate exactly how many instructions are going to be emitted, so
    // we calculate it by emitting it into a disposable buffer, and then checking how many instructions
    // were actually emitted.
    let mut buffer = MachBuffer::new();
    let mut fake_emit_state = state.clone();

    return_call_emit_impl(&mut buffer, emit_info, &mut fake_emit_state, info);

    // Finalize the buffer and get the number of bytes emitted.
    let buffer = buffer.finish(&Default::default(), &mut Default::default());
    let length = buffer.data().len() as u32;

    // And now emit the island inline with this instruction.
    if sink.island_needed(length) {
        let jump_around_label = sink.get_label();
        Inst::gen_jump(jump_around_label).emit(sink, emit_info, state);
        sink.emit_island(length + 4, &mut state.ctrl_plane);
        sink.bind_label(jump_around_label, &mut state.ctrl_plane);
    }

    // Now that we're done, emit the *actual* return sequence.
    return_call_emit_impl(sink, emit_info, state, info);
}

/// This should not be called directly, Instead prefer to call [emit_return_call_common_sequence].
fn return_call_emit_impl<T>(
    sink: &mut MachBuffer<Inst>,
    emit_info: &EmitInfo,
    state: &mut EmitState,
    info: &ReturnCallInfo<T>,
) {
    let sp_to_fp_offset = {
        let frame_layout = state.frame_layout();
        i64::from(
            frame_layout.clobber_size
                + frame_layout.fixed_frame_storage_size
                + frame_layout.outgoing_args_size,
        )
    };

    let mut clobber_offset = sp_to_fp_offset - 8;
    for reg in state.frame_layout().clobbered_callee_saves.clone() {
        let rreg = reg.to_reg();
        let ty = match rreg.class() {
            RegClass::Int => I64,
            RegClass::Float => F64,
            RegClass::Vector => unimplemented!("Vector Clobber Restores"),
        };

        Inst::gen_load(
            reg.map(Reg::from),
            AMode::SPOffset(clobber_offset),
            ty,
            MemFlags::trusted(),
        )
        .emit(sink, emit_info, state);

        clobber_offset -= 8
    }

    // Restore the link register and frame pointer
    let setup_area_size = i64::from(state.frame_layout().setup_area_size);
    if setup_area_size > 0 {
        Inst::gen_load(
            writable_link_reg(),
            AMode::SPOffset(sp_to_fp_offset + 8),
            I64,
            MemFlags::trusted(),
        )
        .emit(sink, emit_info, state);

        Inst::gen_load(
            writable_fp_reg(),
            AMode::SPOffset(sp_to_fp_offset),
            I64,
            MemFlags::trusted(),
        )
        .emit(sink, emit_info, state);
    }

    // If we over-allocated the incoming args area in the prologue, resize down to what the callee
    // is expecting.
    let incoming_args_diff =
        i64::from(state.frame_layout().tail_args_size - info.new_stack_arg_size);

    // Increment SP all at once
    let sp_increment = sp_to_fp_offset + setup_area_size + incoming_args_diff;
    if sp_increment > 0 {
        for inst in Riscv64MachineDeps::gen_sp_reg_adjust(i32::try_from(sp_increment).unwrap()) {
            inst.emit(sink, emit_info, state);
        }
    }
}

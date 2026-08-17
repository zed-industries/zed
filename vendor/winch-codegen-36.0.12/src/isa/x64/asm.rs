//! Assembler library implementation for x64.

use crate::{
    constant_pool::ConstantPool,
    isa::{CallingConvention, reg::Reg},
    masm::{
        DivKind, Extend, ExtendKind, ExtendType, IntCmpKind, MulWideKind, OperandSize, RemKind,
        RoundingMode, ShiftKind, Signed, V128ExtendKind, V128LoadExtendKind, Zero,
    },
    reg::writable,
};
use cranelift_codegen::{
    CallInfo, Final, MachBuffer, MachBufferFinalized, MachInst, MachInstEmit, MachInstEmitState,
    MachLabel, PatchRegion, Writable,
    ir::{ExternalName, MemFlags, SourceLoc, TrapCode, Type, UserExternalNameRef, types},
    isa::{
        unwind::UnwindInst,
        x64::{
            AtomicRmwSeqOp, EmitInfo, EmitState, Inst,
            args::{
                self, Amode, CC, ExtMode, FromWritableReg, Gpr, GprMem, GprMemImm, RegMem,
                RegMemImm, SyntheticAmode, WritableGpr, WritableXmm, Xmm, XmmMem, XmmMemImm,
            },
            external::{PairedGpr, PairedXmm},
            settings as x64_settings,
        },
    },
    settings,
};

use crate::reg::WritableReg;
use cranelift_assembler_x64 as asm;
use wasmtime_environ::Unsigned;

use super::address::Address;
use smallvec::SmallVec;

// Conversions between winch-codegen x64 types and cranelift-codegen x64 types.

impl From<Reg> for RegMemImm {
    fn from(reg: Reg) -> Self {
        RegMemImm::reg(reg.into())
    }
}

impl From<Reg> for RegMem {
    fn from(value: Reg) -> Self {
        RegMem::Reg { reg: value.into() }
    }
}

impl From<Reg> for WritableGpr {
    fn from(reg: Reg) -> Self {
        let writable = Writable::from_reg(reg.into());
        WritableGpr::from_writable_reg(writable).expect("valid writable gpr")
    }
}

impl From<Reg> for WritableXmm {
    fn from(reg: Reg) -> Self {
        let writable = Writable::from_reg(reg.into());
        WritableXmm::from_writable_reg(writable).expect("valid writable xmm")
    }
}

/// Convert a writable GPR register to the read-write pair expected by
/// `cranelift-codegen`.
fn pair_gpr(reg: WritableReg) -> PairedGpr {
    assert!(reg.to_reg().is_int());
    let read = Gpr::unwrap_new(reg.to_reg().into());
    let write = WritableGpr::from_reg(reg.to_reg().into());
    PairedGpr { read, write }
}

impl From<Reg> for asm::Gpr<Gpr> {
    fn from(reg: Reg) -> Self {
        asm::Gpr::new(reg.into())
    }
}

impl From<Reg> for asm::GprMem<Gpr, Gpr> {
    fn from(reg: Reg) -> Self {
        asm::GprMem::Gpr(reg.into())
    }
}

/// Convert a writable XMM register to the read-write pair expected by
/// `cranelift-codegen`.
fn pair_xmm(reg: WritableReg) -> PairedXmm {
    assert!(reg.to_reg().is_float());
    let read = Xmm::unwrap_new(reg.to_reg().into());
    let write = WritableXmm::from_reg(reg.to_reg().into());
    PairedXmm { read, write }
}

impl From<Reg> for asm::Xmm<Xmm> {
    fn from(reg: Reg) -> Self {
        asm::Xmm::new(reg.into())
    }
}

impl From<Reg> for asm::XmmMem<Xmm, Gpr> {
    fn from(reg: Reg) -> Self {
        asm::XmmMem::Xmm(reg.into())
    }
}

impl From<Reg> for Gpr {
    fn from(reg: Reg) -> Self {
        Gpr::unwrap_new(reg.into())
    }
}

impl From<Reg> for GprMem {
    fn from(value: Reg) -> Self {
        GprMem::unwrap_new(value.into())
    }
}

impl From<Reg> for GprMemImm {
    fn from(reg: Reg) -> Self {
        GprMemImm::unwrap_new(reg.into())
    }
}

impl From<Reg> for Xmm {
    fn from(reg: Reg) -> Self {
        Xmm::unwrap_new(reg.into())
    }
}

impl From<Reg> for XmmMem {
    fn from(value: Reg) -> Self {
        XmmMem::unwrap_new(value.into())
    }
}

impl From<Reg> for XmmMemImm {
    fn from(value: Reg) -> Self {
        XmmMemImm::unwrap_new(value.into())
    }
}

impl From<OperandSize> for args::OperandSize {
    fn from(size: OperandSize) -> Self {
        match size {
            OperandSize::S8 => Self::Size8,
            OperandSize::S16 => Self::Size16,
            OperandSize::S32 => Self::Size32,
            OperandSize::S64 => Self::Size64,
            s => panic!("Invalid operand size {s:?}"),
        }
    }
}

impl From<IntCmpKind> for CC {
    fn from(value: IntCmpKind) -> Self {
        match value {
            IntCmpKind::Eq => CC::Z,
            IntCmpKind::Ne => CC::NZ,
            IntCmpKind::LtS => CC::L,
            IntCmpKind::LtU => CC::B,
            IntCmpKind::GtS => CC::NLE,
            IntCmpKind::GtU => CC::NBE,
            IntCmpKind::LeS => CC::LE,
            IntCmpKind::LeU => CC::BE,
            IntCmpKind::GeS => CC::NL,
            IntCmpKind::GeU => CC::NB,
        }
    }
}

impl<T: ExtendType> From<Extend<T>> for ExtMode {
    fn from(value: Extend<T>) -> Self {
        match value {
            Extend::I32Extend8 => ExtMode::BL,
            Extend::I32Extend16 => ExtMode::WL,
            Extend::I64Extend8 => ExtMode::BQ,
            Extend::I64Extend16 => ExtMode::WQ,
            Extend::I64Extend32 => ExtMode::LQ,
            Extend::__Kind(_) => unreachable!(),
        }
    }
}

impl From<ExtendKind> for ExtMode {
    fn from(value: ExtendKind) -> Self {
        match value {
            ExtendKind::Signed(s) => s.into(),
            ExtendKind::Unsigned(u) => u.into(),
        }
    }
}

/// Kinds of extends supported by `vpmov`.
pub(super) enum VpmovKind {
    /// Sign extends 8 lanes of 8-bit integers to 8 lanes of 16-bit integers.
    E8x8S,
    /// Zero extends 8 lanes of 8-bit integers to 8 lanes of 16-bit integers.
    E8x8U,
    /// Sign extends 4 lanes of 16-bit integers to 4 lanes of 32-bit integers.
    E16x4S,
    /// Zero extends 4 lanes of 16-bit integers to 4 lanes of 32-bit integers.
    E16x4U,
    /// Sign extends 2 lanes of 32-bit integers to 2 lanes of 64-bit integers.
    E32x2S,
    /// Zero extends 2 lanes of 32-bit integers to 2 lanes of 64-bit integers.
    E32x2U,
}

impl From<V128LoadExtendKind> for VpmovKind {
    fn from(value: V128LoadExtendKind) -> Self {
        match value {
            V128LoadExtendKind::E8x8S => Self::E8x8S,
            V128LoadExtendKind::E8x8U => Self::E8x8U,
            V128LoadExtendKind::E16x4S => Self::E16x4S,
            V128LoadExtendKind::E16x4U => Self::E16x4U,
            V128LoadExtendKind::E32x2S => Self::E32x2S,
            V128LoadExtendKind::E32x2U => Self::E32x2U,
        }
    }
}

impl From<V128ExtendKind> for VpmovKind {
    fn from(value: V128ExtendKind) -> Self {
        match value {
            V128ExtendKind::LowI8x16S | V128ExtendKind::HighI8x16S => Self::E8x8S,
            V128ExtendKind::LowI8x16U => Self::E8x8U,
            V128ExtendKind::LowI16x8S | V128ExtendKind::HighI16x8S => Self::E16x4S,
            V128ExtendKind::LowI16x8U => Self::E16x4U,
            V128ExtendKind::LowI32x4S | V128ExtendKind::HighI32x4S => Self::E32x2S,
            V128ExtendKind::LowI32x4U => Self::E32x2U,
            _ => unimplemented!(),
        }
    }
}

/// Kinds of comparisons supported by `vcmp`.
pub(super) enum VcmpKind {
    /// Equal comparison.
    Eq,
    /// Not equal comparison.
    Ne,
    /// Less than comparison.
    Lt,
    /// Less than or equal comparison.
    Le,
    /// Unordered comparison. Sets result to all 1s if either source operand is
    /// NaN.
    Unord,
}

/// Kinds of conversions supported by `vcvt`.
pub(super) enum VcvtKind {
    /// Converts 32-bit integers to 32-bit floats.
    I32ToF32,
    /// Converts doubleword integers to double precision floats.
    I32ToF64,
    /// Converts double precision floats to single precision floats.
    F64ToF32,
    // Converts double precision floats to 32-bit integers.
    F64ToI32,
    /// Converts single precision floats to double precision floats.
    F32ToF64,
    /// Converts single precision floats to 32-bit integers.
    F32ToI32,
}

/// Modes supported by `vround`.
pub(crate) enum VroundMode {
    /// Rounds toward nearest (ties to even).
    TowardNearest,
    /// Rounds toward negative infinity.
    TowardNegativeInfinity,
    /// Rounds toward positive infinity.
    TowardPositiveInfinity,
    /// Rounds toward zero.
    TowardZero,
}

/// Low level assembler implementation for x64.
pub(crate) struct Assembler {
    /// The machine instruction buffer.
    buffer: MachBuffer<Inst>,
    /// Constant emission information.
    emit_info: EmitInfo,
    /// Emission state.
    emit_state: EmitState,
    /// x64 flags.
    isa_flags: x64_settings::Flags,
    /// Constant pool.
    pool: ConstantPool,
}

impl Assembler {
    /// Create a new x64 assembler.
    pub fn new(shared_flags: settings::Flags, isa_flags: x64_settings::Flags) -> Self {
        Self {
            buffer: MachBuffer::<Inst>::new(),
            emit_state: Default::default(),
            emit_info: EmitInfo::new(shared_flags, isa_flags.clone()),
            pool: ConstantPool::new(),
            isa_flags,
        }
    }

    /// Get a mutable reference to underlying
    /// machine buffer.
    pub fn buffer_mut(&mut self) -> &mut MachBuffer<Inst> {
        &mut self.buffer
    }

    /// Get a reference to the underlying machine buffer.
    pub fn buffer(&self) -> &MachBuffer<Inst> {
        &self.buffer
    }

    /// Adds a constant to the constant pool and returns its address.
    pub fn add_constant(&mut self, constant: &[u8]) -> Address {
        let handle = self.pool.register(constant, &mut self.buffer);
        Address::constant(handle)
    }

    /// Load a floating point constant, using the constant pool.
    pub fn load_fp_const(&mut self, dst: WritableReg, constant: &[u8], size: OperandSize) {
        let addr = self.add_constant(constant);
        self.xmm_mov_mr(&addr, dst, size, MemFlags::trusted());
    }

    /// Return the emitted code.
    pub fn finalize(mut self, loc: Option<SourceLoc>) -> MachBufferFinalized<Final> {
        let stencil = self
            .buffer
            .finish(&self.pool.constants(), self.emit_state.ctrl_plane_mut());
        stencil.apply_base_srcloc(loc.unwrap_or_default())
    }

    fn emit(&mut self, inst: Inst) {
        inst.emit(&mut self.buffer, &self.emit_info, &mut self.emit_state);
    }

    fn to_synthetic_amode(addr: &Address, memflags: MemFlags) -> SyntheticAmode {
        match *addr {
            Address::Offset { base, offset } => {
                let amode = Amode::imm_reg(offset as i32, base.into()).with_flags(memflags);
                SyntheticAmode::real(amode)
            }
            Address::Const(c) => SyntheticAmode::ConstantOffset(c),
            Address::ImmRegRegShift {
                simm32,
                base,
                index,
                shift,
            } => SyntheticAmode::Real(Amode::ImmRegRegShift {
                simm32,
                base: base.into(),
                index: index.into(),
                shift,
                flags: memflags,
            }),
        }
    }

    /// Emit an unwind instruction.
    pub fn unwind_inst(&mut self, inst: UnwindInst) {
        self.emit(Inst::Unwind { inst })
    }

    /// Push register.
    pub fn push_r(&mut self, reg: Reg) {
        let inst = asm::inst::pushq_o::new(reg).into();
        self.emit(Inst::External { inst });
    }

    /// Pop to register.
    pub fn pop_r(&mut self, dst: WritableReg) {
        let writable: WritableGpr = dst.map(Into::into);
        let inst = asm::inst::popq_o::new(writable).into();
        self.emit(Inst::External { inst });
    }

    /// Return instruction.
    pub fn ret(&mut self) {
        let inst = asm::inst::retq_zo::new().into();
        self.emit(Inst::External { inst });
    }

    /// Register-to-register move.
    pub fn mov_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableGpr = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::movb_mr::new(dst, src).into(),
            OperandSize::S16 => asm::inst::movw_mr::new(dst, src).into(),
            OperandSize::S32 => asm::inst::movl_mr::new(dst, src).into(),
            OperandSize::S64 => asm::inst::movq_mr::new(dst, src).into(),
            _ => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Register-to-memory move.
    pub fn mov_rm(&mut self, src: Reg, addr: &Address, size: OperandSize, flags: MemFlags) {
        assert!(addr.is_offset());
        let dst = Self::to_synthetic_amode(addr, flags);
        let inst = match size {
            OperandSize::S8 => asm::inst::movb_mr::new(dst, src).into(),
            OperandSize::S16 => asm::inst::movw_mr::new(dst, src).into(),
            OperandSize::S32 => asm::inst::movl_mr::new(dst, src).into(),
            OperandSize::S64 => asm::inst::movq_mr::new(dst, src).into(),
            _ => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Immediate-to-memory move.
    pub fn mov_im(&mut self, src: i32, addr: &Address, size: OperandSize, flags: MemFlags) {
        assert!(addr.is_offset());
        let dst = Self::to_synthetic_amode(addr, flags);
        let inst = match size {
            OperandSize::S8 => {
                let src = i8::try_from(src).unwrap();
                asm::inst::movb_mi::new(dst, src.unsigned()).into()
            }
            OperandSize::S16 => {
                let src = i16::try_from(src).unwrap();
                asm::inst::movw_mi::new(dst, src.unsigned()).into()
            }
            OperandSize::S32 => asm::inst::movl_mi::new(dst, src.unsigned()).into(),
            OperandSize::S64 => asm::inst::movq_mi_sxl::new(dst, src).into(),
            _ => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Immediate-to-register move.
    pub fn mov_ir(&mut self, imm: u64, dst: WritableReg, size: OperandSize) {
        self.emit(Inst::imm(size.into(), imm, dst.map(Into::into)));
    }

    /// Zero-extend memory-to-register load.
    pub fn movzx_mr(
        &mut self,
        addr: &Address,
        dst: WritableReg,
        ext: Option<Extend<Zero>>,
        memflags: MemFlags,
    ) {
        let src = Self::to_synthetic_amode(addr, memflags);

        if let Some(ext) = ext {
            let dst = WritableGpr::from_reg(dst.to_reg().into());
            let inst = match ext.into() {
                ExtMode::BL => asm::inst::movzbl_rm::new(dst, src).into(),
                ExtMode::BQ => asm::inst::movzbq_rm::new(dst, src).into(),
                ExtMode::WL => asm::inst::movzwl_rm::new(dst, src).into(),
                ExtMode::WQ => asm::inst::movzwq_rm::new(dst, src).into(),
                ExtMode::LQ => {
                    // This instruction selection may seem strange but is
                    // correct in 64-bit mode: section 3.4.1.1 of the Intel
                    // manual says that "32-bit operands generate a 32-bit
                    // result, zero-extended to a 64-bit result in the
                    // destination general-purpose register." This is applicable
                    // beyond `mov` but we use this fact to zero-extend `src`
                    // into `dst`.
                    asm::inst::movl_rm::new(dst, src).into()
                }
            };
            self.emit(Inst::External { inst });
        } else {
            let dst = WritableGpr::from_reg(dst.to_reg().into());
            let inst = asm::inst::movq_rm::new(dst, src).into();
            self.emit(Inst::External { inst });
        }
    }

    // Sign-extend memory-to-register load.
    pub fn movsx_mr(
        &mut self,
        addr: &Address,
        dst: WritableReg,
        ext: Extend<Signed>,
        memflags: MemFlags,
    ) {
        let src = Self::to_synthetic_amode(addr, memflags);
        let dst = WritableGpr::from_reg(dst.to_reg().into());
        let inst = match ext.into() {
            ExtMode::BL => asm::inst::movsbl_rm::new(dst, src).into(),
            ExtMode::BQ => asm::inst::movsbq_rm::new(dst, src).into(),
            ExtMode::WL => asm::inst::movswl_rm::new(dst, src).into(),
            ExtMode::WQ => asm::inst::movswq_rm::new(dst, src).into(),
            ExtMode::LQ => asm::inst::movslq_rm::new(dst, src).into(),
        };
        self.emit(Inst::External { inst });
    }

    /// Register-to-register move with zero extension.
    pub fn movzx_rr(&mut self, src: Reg, dst: WritableReg, kind: Extend<Zero>) {
        let dst = WritableGpr::from_reg(dst.to_reg().into());
        let inst = match kind.into() {
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
        self.emit(Inst::External { inst });
    }

    /// Register-to-register move with sign extension.
    pub fn movsx_rr(&mut self, src: Reg, dst: WritableReg, kind: Extend<Signed>) {
        let dst = WritableGpr::from_reg(dst.to_reg().into());
        let inst = match kind.into() {
            ExtMode::BL => asm::inst::movsbl_rm::new(dst, src).into(),
            ExtMode::BQ => asm::inst::movsbq_rm::new(dst, src).into(),
            ExtMode::WL => asm::inst::movswl_rm::new(dst, src).into(),
            ExtMode::WQ => asm::inst::movswq_rm::new(dst, src).into(),
            ExtMode::LQ => asm::inst::movslq_rm::new(dst, src).into(),
        };
        self.emit(Inst::External { inst });
    }

    /// Integer register conditional move.
    pub fn cmov(&mut self, src: Reg, dst: WritableReg, cc: IntCmpKind, size: OperandSize) {
        use IntCmpKind::*;
        use OperandSize::*;

        let dst: WritableGpr = dst.map(Into::into);
        let inst = match size {
            S8 | S16 | S32 => match cc {
                Eq => asm::inst::cmovel_rm::new(dst, src).into(),
                Ne => asm::inst::cmovnel_rm::new(dst, src).into(),
                LtS => asm::inst::cmovll_rm::new(dst, src).into(),
                LtU => asm::inst::cmovbl_rm::new(dst, src).into(),
                GtS => asm::inst::cmovgl_rm::new(dst, src).into(),
                GtU => asm::inst::cmoval_rm::new(dst, src).into(),
                LeS => asm::inst::cmovlel_rm::new(dst, src).into(),
                LeU => asm::inst::cmovbel_rm::new(dst, src).into(),
                GeS => asm::inst::cmovgel_rm::new(dst, src).into(),
                GeU => asm::inst::cmovael_rm::new(dst, src).into(),
            },
            S64 => match cc {
                Eq => asm::inst::cmoveq_rm::new(dst, src).into(),
                Ne => asm::inst::cmovneq_rm::new(dst, src).into(),
                LtS => asm::inst::cmovlq_rm::new(dst, src).into(),
                LtU => asm::inst::cmovbq_rm::new(dst, src).into(),
                GtS => asm::inst::cmovgq_rm::new(dst, src).into(),
                GtU => asm::inst::cmovaq_rm::new(dst, src).into(),
                LeS => asm::inst::cmovleq_rm::new(dst, src).into(),
                LeU => asm::inst::cmovbeq_rm::new(dst, src).into(),
                GeS => asm::inst::cmovgeq_rm::new(dst, src).into(),
                GeU => asm::inst::cmovaeq_rm::new(dst, src).into(),
            },
            _ => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Single and double precision floating point
    /// register-to-register move.
    pub fn xmm_mov_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let ty = match size {
            OperandSize::S32 => types::F32,
            OperandSize::S64 => types::F64,
            OperandSize::S128 => types::I32X4,
            OperandSize::S8 | OperandSize::S16 => unreachable!(),
        };
        self.emit(Inst::gen_move(dst.map(|r| r.into()), src.into(), ty));
    }

    /// Single and double precision floating point load.
    pub fn xmm_mov_mr(
        &mut self,
        src: &Address,
        dst: WritableReg,
        size: OperandSize,
        flags: MemFlags,
    ) {
        use OperandSize::*;

        assert!(dst.to_reg().is_float());

        let src = Self::to_synthetic_amode(src, flags);
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            S32 => asm::inst::movss_a_m::new(dst, src).into(),
            S64 => asm::inst::movsd_a_m::new(dst, src).into(),
            S128 => asm::inst::movdqu_a::new(dst, src).into(),
            S8 | S16 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Vector load and extend.
    pub fn xmm_vpmov_mr(
        &mut self,
        src: &Address,
        dst: WritableReg,
        kind: VpmovKind,
        flags: MemFlags,
    ) {
        assert!(dst.to_reg().is_float());
        let src = Self::to_synthetic_amode(src, flags);
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match kind {
            VpmovKind::E8x8S => asm::inst::vpmovsxbw_a::new(dst, src).into(),
            VpmovKind::E8x8U => asm::inst::vpmovzxbw_a::new(dst, src).into(),
            VpmovKind::E16x4S => asm::inst::vpmovsxwd_a::new(dst, src).into(),
            VpmovKind::E16x4U => asm::inst::vpmovzxwd_a::new(dst, src).into(),
            VpmovKind::E32x2S => asm::inst::vpmovsxdq_a::new(dst, src).into(),
            VpmovKind::E32x2U => asm::inst::vpmovzxdq_a::new(dst, src).into(),
        };
        self.emit(Inst::External { inst });
    }

    /// Extends vector of integers in `src` and puts results in `dst`.
    pub fn xmm_vpmov_rr(&mut self, src: Reg, dst: WritableReg, kind: VpmovKind) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match kind {
            VpmovKind::E8x8S => asm::inst::vpmovsxbw_a::new(dst, src).into(),
            VpmovKind::E8x8U => asm::inst::vpmovzxbw_a::new(dst, src).into(),
            VpmovKind::E16x4S => asm::inst::vpmovsxwd_a::new(dst, src).into(),
            VpmovKind::E16x4U => asm::inst::vpmovzxwd_a::new(dst, src).into(),
            VpmovKind::E32x2S => asm::inst::vpmovsxdq_a::new(dst, src).into(),
            VpmovKind::E32x2U => asm::inst::vpmovzxdq_a::new(dst, src).into(),
        };
        self.emit(Inst::External { inst });
    }

    /// Vector load and broadcast.
    pub fn xmm_vpbroadcast_mr(
        &mut self,
        src: &Address,
        dst: WritableReg,
        size: OperandSize,
        flags: MemFlags,
    ) {
        assert!(dst.to_reg().is_float());
        let src = Self::to_synthetic_amode(src, flags);
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpbroadcastb_a::new(dst, src).into(),
            OperandSize::S16 => asm::inst::vpbroadcastw_a::new(dst, src).into(),
            OperandSize::S32 => asm::inst::vpbroadcastd_a::new(dst, src).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Value in `src` is broadcast into lanes of `size` in `dst`.
    pub fn xmm_vpbroadcast_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        assert!(src.is_float() && dst.to_reg().is_float());
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpbroadcastb_a::new(dst, src).into(),
            OperandSize::S16 => asm::inst::vpbroadcastw_a::new(dst, src).into(),
            OperandSize::S32 => asm::inst::vpbroadcastd_a::new(dst, src).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Memory to register shuffle of bytes in vector.
    pub fn xmm_vpshuf_mr(
        &mut self,
        src: &Address,
        dst: WritableReg,
        mask: u8,
        size: OperandSize,
        flags: MemFlags,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let src = Self::to_synthetic_amode(src, flags);
        let inst = match size {
            OperandSize::S32 => asm::inst::vpshufd_a::new(dst, src, mask).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Register to register shuffle of bytes in vector.
    pub fn xmm_vpshuf_rr(&mut self, src: Reg, dst: WritableReg, mask: u8, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());

        let inst = match size {
            OperandSize::S16 => asm::inst::vpshuflw_a::new(dst, src, mask).into(),
            OperandSize::S32 => asm::inst::vpshufd_a::new(dst, src, mask).into(),
            _ => unimplemented!(),
        };

        self.emit(Inst::External { inst });
    }

    /// Single and double precision floating point store.
    pub fn xmm_mov_rm(&mut self, src: Reg, dst: &Address, size: OperandSize, flags: MemFlags) {
        use OperandSize::*;

        assert!(src.is_float());

        let dst = Self::to_synthetic_amode(dst, flags);
        let src: Xmm = src.into();
        let inst = match size {
            S32 => asm::inst::movss_c_m::new(dst, src).into(),
            S64 => asm::inst::movsd_c_m::new(dst, src).into(),
            S128 => asm::inst::movdqu_b::new(dst, src).into(),
            S16 | S8 => unreachable!(),
        };
        self.emit(Inst::External { inst })
    }

    /// Floating point register conditional move.
    pub fn xmm_cmov(&mut self, src: Reg, dst: WritableReg, cc: IntCmpKind, size: OperandSize) {
        let dst: WritableXmm = dst.map(Into::into);
        let ty = match size {
            OperandSize::S32 => types::F32,
            OperandSize::S64 => types::F64,
            // Move the entire 128 bits via movdqa.
            OperandSize::S128 => types::I32X4,
            OperandSize::S8 | OperandSize::S16 => unreachable!(),
        };

        self.emit(Inst::XmmCmove {
            ty,
            cc: cc.into(),
            consequent: Xmm::unwrap_new(src.into()),
            alternative: dst.to_reg(),
            dst,
        })
    }

    /// Subtract register and register
    pub fn sub_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::subb_rm::new(dst, src).into(),
            OperandSize::S16 => asm::inst::subw_rm::new(dst, src).into(),
            OperandSize::S32 => asm::inst::subl_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::subq_rm::new(dst, src).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Subtract immediate register.
    pub fn sub_ir(&mut self, imm: i32, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::subb_mi::new(dst, u8::try_from(imm).unwrap()).into(),
            OperandSize::S16 => asm::inst::subw_mi::new(dst, u16::try_from(imm).unwrap()).into(),
            OperandSize::S32 => asm::inst::subl_mi::new(dst, imm as u32).into(),
            OperandSize::S64 => asm::inst::subq_mi_sxl::new(dst, imm).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// "and" two registers.
    pub fn and_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::andb_rm::new(dst, src).into(),
            OperandSize::S16 => asm::inst::andw_rm::new(dst, src).into(),
            OperandSize::S32 => asm::inst::andl_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::andq_rm::new(dst, src).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn and_ir(&mut self, imm: i32, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::andb_mi::new(dst, u8::try_from(imm).unwrap()).into(),
            OperandSize::S16 => asm::inst::andw_mi::new(dst, u16::try_from(imm).unwrap()).into(),
            OperandSize::S32 => asm::inst::andl_mi::new(dst, imm as u32).into(),
            OperandSize::S64 => asm::inst::andq_mi_sxl::new(dst, imm).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// "and" two float registers.
    pub fn xmm_and_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_xmm(dst);
        let inst = match size {
            OperandSize::S32 => asm::inst::andps_a::new(dst, src).into(),
            OperandSize::S64 => asm::inst::andpd_a::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// "and not" two float registers.
    pub fn xmm_andn_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_xmm(dst);
        let inst = match size {
            OperandSize::S32 => asm::inst::andnps_a::new(dst, src).into(),
            OperandSize::S64 => asm::inst::andnpd_a::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn gpr_to_xmm(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::movd_a::new(dst, src).into(),
            OperandSize::S64 => asm::inst::movq_a::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => unreachable!(),
        };

        self.emit(Inst::External { inst });
    }

    pub fn xmm_to_gpr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableGpr = dst.map(Into::into);
        let src: Xmm = src.into();
        let inst = match size {
            OperandSize::S32 => asm::inst::movd_b::new(dst, src).into(),
            OperandSize::S64 => asm::inst::movq_b::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => unreachable!(),
        };

        self.emit(Inst::External { inst })
    }

    /// Convert float to signed int.
    pub fn cvt_float_to_sint_seq(
        &mut self,
        src: Reg,
        dst: WritableReg,
        tmp_gpr: Reg,
        tmp_xmm: Reg,
        src_size: OperandSize,
        dst_size: OperandSize,
        saturating: bool,
    ) {
        self.emit(Inst::CvtFloatToSintSeq {
            dst_size: dst_size.into(),
            src_size: src_size.into(),
            is_saturating: saturating,
            src: src.into(),
            dst: dst.map(Into::into),
            tmp_gpr: tmp_gpr.into(),
            tmp_xmm: tmp_xmm.into(),
        });
    }

    /// Convert float to unsigned int.
    pub fn cvt_float_to_uint_seq(
        &mut self,
        src: Reg,
        dst: WritableReg,
        tmp_gpr: Reg,
        tmp_xmm: Reg,
        tmp_xmm2: Reg,
        src_size: OperandSize,
        dst_size: OperandSize,
        saturating: bool,
    ) {
        self.emit(Inst::CvtFloatToUintSeq {
            dst_size: dst_size.into(),
            src_size: src_size.into(),
            is_saturating: saturating,
            src: src.into(),
            dst: dst.map(Into::into),
            tmp_gpr: tmp_gpr.into(),
            tmp_xmm: tmp_xmm.into(),
            tmp_xmm2: tmp_xmm2.into(),
        });
    }

    /// Convert signed int to float.
    pub fn cvt_sint_to_float(
        &mut self,
        src: Reg,
        dst: WritableReg,
        src_size: OperandSize,
        dst_size: OperandSize,
    ) {
        use OperandSize::*;
        let dst = pair_xmm(dst);
        let inst = match (src_size, dst_size) {
            (S32, S32) => asm::inst::cvtsi2ssl_a::new(dst, src).into(),
            (S32, S64) => asm::inst::cvtsi2sdl_a::new(dst, src).into(),
            (S64, S32) => asm::inst::cvtsi2ssq_a::new(dst, src).into(),
            (S64, S64) => asm::inst::cvtsi2sdq_a::new(dst, src).into(),
            _ => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Convert unsigned 64-bit int to float.
    pub fn cvt_uint64_to_float_seq(
        &mut self,
        src: Reg,
        dst: WritableReg,
        tmp_gpr1: Reg,
        tmp_gpr2: Reg,
        dst_size: OperandSize,
    ) {
        self.emit(Inst::CvtUint64ToFloatSeq {
            dst_size: dst_size.into(),
            src: src.into(),
            dst: dst.map(Into::into),
            tmp_gpr1: tmp_gpr1.into(),
            tmp_gpr2: tmp_gpr2.into(),
        });
    }

    /// Change precision of float.
    pub fn cvt_float_to_float(
        &mut self,
        src: Reg,
        dst: WritableReg,
        src_size: OperandSize,
        dst_size: OperandSize,
    ) {
        use OperandSize::*;
        let dst = pair_xmm(dst);
        let inst = match (src_size, dst_size) {
            (S32, S64) => asm::inst::cvtss2sd_a::new(dst, src).into(),
            (S64, S32) => asm::inst::cvtsd2ss_a::new(dst, src).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn or_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::orb_rm::new(dst, src).into(),
            OperandSize::S16 => asm::inst::orw_rm::new(dst, src).into(),
            OperandSize::S32 => asm::inst::orl_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::orq_rm::new(dst, src).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn or_ir(&mut self, imm: i32, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::orb_mi::new(dst, u8::try_from(imm).unwrap()).into(),
            OperandSize::S16 => asm::inst::orw_mi::new(dst, u16::try_from(imm).unwrap()).into(),
            OperandSize::S32 => asm::inst::orl_mi::new(dst, imm as u32).into(),
            OperandSize::S64 => asm::inst::orq_mi_sxl::new(dst, imm).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn xmm_or_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_xmm(dst);
        let inst = match size {
            OperandSize::S32 => asm::inst::orps_a::new(dst, src).into(),
            OperandSize::S64 => asm::inst::orpd_a::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Logical exclusive or with registers.
    pub fn xor_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::xorb_rm::new(dst, src).into(),
            OperandSize::S16 => asm::inst::xorw_rm::new(dst, src).into(),
            OperandSize::S32 => asm::inst::xorl_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::xorq_rm::new(dst, src).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn xor_ir(&mut self, imm: i32, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::xorb_mi::new(dst, u8::try_from(imm).unwrap()).into(),
            OperandSize::S16 => asm::inst::xorw_mi::new(dst, u16::try_from(imm).unwrap()).into(),
            OperandSize::S32 => asm::inst::xorl_mi::new(dst, imm as u32).into(),
            OperandSize::S64 => asm::inst::xorq_mi_sxl::new(dst, imm).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Logical exclusive or with float registers.
    pub fn xmm_xor_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_xmm(dst);
        let inst = match size {
            OperandSize::S32 => asm::inst::xorps_a::new(dst, src).into(),
            OperandSize::S64 => asm::inst::xorpd_a::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Shift with register and register.
    pub fn shift_rr(&mut self, src: Reg, dst: WritableReg, kind: ShiftKind, size: OperandSize) {
        let dst = pair_gpr(dst);
        let src: Gpr = src.into();
        let inst = match (kind, size) {
            (ShiftKind::Shl, OperandSize::S32) => asm::inst::shll_mc::new(dst, src).into(),
            (ShiftKind::Shl, OperandSize::S64) => asm::inst::shlq_mc::new(dst, src).into(),
            (ShiftKind::Shl, _) => todo!(),
            (ShiftKind::ShrS, OperandSize::S32) => asm::inst::sarl_mc::new(dst, src).into(),
            (ShiftKind::ShrS, OperandSize::S64) => asm::inst::sarq_mc::new(dst, src).into(),
            (ShiftKind::ShrS, _) => todo!(),
            (ShiftKind::ShrU, OperandSize::S32) => asm::inst::shrl_mc::new(dst, src).into(),
            (ShiftKind::ShrU, OperandSize::S64) => asm::inst::shrq_mc::new(dst, src).into(),
            (ShiftKind::ShrU, _) => todo!(),
            (ShiftKind::Rotl, OperandSize::S32) => asm::inst::roll_mc::new(dst, src).into(),
            (ShiftKind::Rotl, OperandSize::S64) => asm::inst::rolq_mc::new(dst, src).into(),
            (ShiftKind::Rotl, _) => todo!(),
            (ShiftKind::Rotr, OperandSize::S32) => asm::inst::rorl_mc::new(dst, src).into(),
            (ShiftKind::Rotr, OperandSize::S64) => asm::inst::rorq_mc::new(dst, src).into(),
            (ShiftKind::Rotr, _) => todo!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Shift with immediate and register.
    pub fn shift_ir(&mut self, imm: u8, dst: WritableReg, kind: ShiftKind, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match (kind, size) {
            (ShiftKind::Shl, OperandSize::S32) => asm::inst::shll_mi::new(dst, imm).into(),
            (ShiftKind::Shl, OperandSize::S64) => asm::inst::shlq_mi::new(dst, imm).into(),
            (ShiftKind::Shl, _) => todo!(),
            (ShiftKind::ShrS, OperandSize::S32) => asm::inst::sarl_mi::new(dst, imm).into(),
            (ShiftKind::ShrS, OperandSize::S64) => asm::inst::sarq_mi::new(dst, imm).into(),
            (ShiftKind::ShrS, _) => todo!(),
            (ShiftKind::ShrU, OperandSize::S32) => asm::inst::shrl_mi::new(dst, imm).into(),
            (ShiftKind::ShrU, OperandSize::S64) => asm::inst::shrq_mi::new(dst, imm).into(),
            (ShiftKind::ShrU, _) => todo!(),
            (ShiftKind::Rotl, OperandSize::S32) => asm::inst::roll_mi::new(dst, imm).into(),
            (ShiftKind::Rotl, OperandSize::S64) => asm::inst::rolq_mi::new(dst, imm).into(),
            (ShiftKind::Rotl, _) => todo!(),
            (ShiftKind::Rotr, OperandSize::S32) => asm::inst::rorl_mi::new(dst, imm).into(),
            (ShiftKind::Rotr, OperandSize::S64) => asm::inst::rorq_mi::new(dst, imm).into(),
            (ShiftKind::Rotr, _) => todo!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Signed/unsigned division.
    ///
    /// Emits a sequence of instructions to ensure the correctness of
    /// the division invariants.  This function assumes that the
    /// caller has correctly allocated the dividend as `(rdx:rax)` and
    /// accounted for the quotient to be stored in `rax`.
    pub fn div(&mut self, divisor: Reg, dst: (Reg, Reg), kind: DivKind, size: OperandSize) {
        let trap = match kind {
            // Signed division has two trapping conditions, integer overflow and
            // divide-by-zero. Check for divide-by-zero explicitly and let the
            // hardware detect overflow.
            DivKind::Signed => {
                self.cmp_ir(divisor, 0, size);
                self.emit(Inst::TrapIf {
                    cc: CC::Z,
                    trap_code: TrapCode::INTEGER_DIVISION_BY_ZERO,
                });

                // Sign-extend the dividend with tailor-made instructoins for
                // just this operation.
                let ext_dst: WritableGpr = dst.1.into();
                let ext_src: Gpr = dst.0.into();
                let inst = match size {
                    OperandSize::S32 => asm::inst::cltd_zo::new(ext_dst, ext_src).into(),
                    OperandSize::S64 => asm::inst::cqto_zo::new(ext_dst, ext_src).into(),
                    _ => unimplemented!(),
                };
                self.emit(Inst::External { inst });
                TrapCode::INTEGER_OVERFLOW
            }

            // Unsigned division only traps in one case, on divide-by-zero, so
            // defer that to the trap opcode.
            //
            // The divisor_hi reg is initialized with zero through an
            // xor-against-itself op.
            DivKind::Unsigned => {
                self.xor_rr(dst.1, writable!(dst.1), size);
                TrapCode::INTEGER_DIVISION_BY_ZERO
            }
        };
        let dst0 = pair_gpr(writable!(dst.0));
        let dst1 = pair_gpr(writable!(dst.1));
        let inst = match (kind, size) {
            (DivKind::Signed, OperandSize::S32) => {
                asm::inst::idivl_m::new(dst0, dst1, divisor, trap).into()
            }
            (DivKind::Unsigned, OperandSize::S32) => {
                asm::inst::divl_m::new(dst0, dst1, divisor, trap).into()
            }
            (DivKind::Signed, OperandSize::S64) => {
                asm::inst::idivq_m::new(dst0, dst1, divisor, trap).into()
            }
            (DivKind::Unsigned, OperandSize::S64) => {
                asm::inst::divq_m::new(dst0, dst1, divisor, trap).into()
            }
            _ => todo!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Signed/unsigned remainder.
    ///
    /// Emits a sequence of instructions to ensure the correctness of the
    /// division invariants and ultimately calculate the remainder.
    /// This function assumes that the
    /// caller has correctly allocated the dividend as `(rdx:rax)` and
    /// accounted for the remainder to be stored in `rdx`.
    pub fn rem(&mut self, divisor: Reg, dst: (Reg, Reg), kind: RemKind, size: OperandSize) {
        match kind {
            // Signed remainder goes through a pseudo-instruction which has
            // some internal branching. The `dividend_hi`, or `rdx`, is
            // initialized here with a `SignExtendData` instruction.
            RemKind::Signed => {
                let ext_dst: WritableGpr = dst.1.into();

                // Initialize `dividend_hi`, or `rdx`, with a tailor-made
                // instruction for this operation.
                let ext_src: Gpr = dst.0.into();
                let inst = match size {
                    OperandSize::S32 => asm::inst::cltd_zo::new(ext_dst, ext_src).into(),
                    OperandSize::S64 => asm::inst::cqto_zo::new(ext_dst, ext_src).into(),
                    _ => unimplemented!(),
                };
                self.emit(Inst::External { inst });
                self.emit(Inst::CheckedSRemSeq {
                    size: size.into(),
                    divisor: divisor.into(),
                    dividend_lo: dst.0.into(),
                    dividend_hi: dst.1.into(),
                    dst_quotient: dst.0.into(),
                    dst_remainder: dst.1.into(),
                });
            }

            // Unsigned remainder initializes `dividend_hi` with zero and
            // then executes a normal `div` instruction.
            RemKind::Unsigned => {
                self.xor_rr(dst.1, writable!(dst.1), size);
                let dst0 = pair_gpr(writable!(dst.0));
                let dst1 = pair_gpr(writable!(dst.1));
                let trap = TrapCode::INTEGER_DIVISION_BY_ZERO;
                let inst = match size {
                    OperandSize::S32 => asm::inst::divl_m::new(dst0, dst1, divisor, trap).into(),
                    OperandSize::S64 => asm::inst::divq_m::new(dst0, dst1, divisor, trap).into(),
                    _ => todo!(),
                };
                self.emit(Inst::External { inst });
            }
        }
    }

    /// Multiply immediate and register.
    pub fn mul_ir(&mut self, imm: i32, dst: WritableReg, size: OperandSize) {
        use OperandSize::*;
        let src = dst.to_reg();
        let dst: WritableGpr = dst.to_reg().into();
        let inst = match size {
            S16 => asm::inst::imulw_rmi::new(dst, src, u16::try_from(imm).unwrap()).into(),
            S32 => asm::inst::imull_rmi::new(dst, src, imm as u32).into(),
            S64 => asm::inst::imulq_rmi_sxl::new(dst, src, imm).into(),
            S8 | S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Multiply register and register.
    pub fn mul_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        use OperandSize::*;
        let dst = pair_gpr(dst);
        let inst = match size {
            S16 => asm::inst::imulw_rm::new(dst, src).into(),
            S32 => asm::inst::imull_rm::new(dst, src).into(),
            S64 => asm::inst::imulq_rm::new(dst, src).into(),
            S8 | S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Add immediate and register.
    pub fn add_ir(&mut self, imm: i32, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::addb_mi::new(dst, u8::try_from(imm).unwrap()).into(),
            OperandSize::S16 => asm::inst::addw_mi::new(dst, u16::try_from(imm).unwrap()).into(),
            OperandSize::S32 => asm::inst::addl_mi::new(dst, imm as u32).into(),
            OperandSize::S64 => asm::inst::addq_mi_sxl::new(dst, imm).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Add register and register.
    pub fn add_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::addb_rm::new(dst, src).into(),
            OperandSize::S16 => asm::inst::addw_rm::new(dst, src).into(),
            OperandSize::S32 => asm::inst::addl_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::addq_rm::new(dst, src).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn lock_xadd(
        &mut self,
        addr: Address,
        dst: WritableReg,
        size: OperandSize,
        flags: MemFlags,
    ) {
        assert!(addr.is_offset());
        let mem = Self::to_synthetic_amode(&addr, flags);
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::lock_xaddb_mr::new(mem, dst).into(),
            OperandSize::S16 => asm::inst::lock_xaddw_mr::new(mem, dst).into(),
            OperandSize::S32 => asm::inst::lock_xaddl_mr::new(mem, dst).into(),
            OperandSize::S64 => asm::inst::lock_xaddq_mr::new(mem, dst).into(),
            OperandSize::S128 => unimplemented!(),
        };

        self.emit(Inst::External { inst });
    }

    pub fn atomic_rmw_seq(
        &mut self,
        addr: Address,
        operand: Reg,
        dst: WritableReg,
        temp: WritableReg,
        size: OperandSize,
        flags: MemFlags,
        op: AtomicRmwSeqOp,
    ) {
        assert!(addr.is_offset());
        let mem = Self::to_synthetic_amode(&addr, flags);
        self.emit(Inst::AtomicRmwSeq {
            ty: Type::int_with_byte_size(size.bytes() as _).unwrap(),
            mem,
            operand: operand.into(),
            temp: temp.map(Into::into),
            dst_old: dst.map(Into::into),
            op,
        });
    }

    pub fn xchg(&mut self, addr: Address, dst: WritableReg, size: OperandSize, flags: MemFlags) {
        assert!(addr.is_offset());
        let mem = Self::to_synthetic_amode(&addr, flags);
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::xchgb_rm::new(dst, mem).into(),
            OperandSize::S16 => asm::inst::xchgw_rm::new(dst, mem).into(),
            OperandSize::S32 => asm::inst::xchgl_rm::new(dst, mem).into(),
            OperandSize::S64 => asm::inst::xchgq_rm::new(dst, mem).into(),
            OperandSize::S128 => unimplemented!(),
        };

        self.emit(Inst::External { inst });
    }
    pub fn cmpxchg(
        &mut self,
        addr: Address,
        replacement: Reg,
        dst: WritableReg,
        size: OperandSize,
        flags: MemFlags,
    ) {
        assert!(addr.is_offset());
        let mem = Self::to_synthetic_amode(&addr, flags);
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::lock_cmpxchgb_mr::new(mem, replacement, dst).into(),
            OperandSize::S16 => asm::inst::lock_cmpxchgw_mr::new(mem, replacement, dst).into(),
            OperandSize::S32 => asm::inst::lock_cmpxchgl_mr::new(mem, replacement, dst).into(),
            OperandSize::S64 => asm::inst::lock_cmpxchgq_mr::new(mem, replacement, dst).into(),
            OperandSize::S128 => unimplemented!(),
        };

        self.emit(Inst::External { inst });
    }

    pub fn cmp_ir(&mut self, src1: Reg, imm: i32, size: OperandSize) {
        let inst = match size {
            OperandSize::S8 => {
                let imm = i8::try_from(imm).unwrap();
                asm::inst::cmpb_mi::new(src1, imm.unsigned()).into()
            }
            OperandSize::S16 => match i8::try_from(imm) {
                Ok(imm8) => asm::inst::cmpw_mi_sxb::new(src1, imm8).into(),
                Err(_) => {
                    asm::inst::cmpw_mi::new(src1, i16::try_from(imm).unwrap().unsigned()).into()
                }
            },
            OperandSize::S32 => match i8::try_from(imm) {
                Ok(imm8) => asm::inst::cmpl_mi_sxb::new(src1, imm8).into(),
                Err(_) => asm::inst::cmpl_mi::new(src1, imm.unsigned()).into(),
            },
            OperandSize::S64 => match i8::try_from(imm) {
                Ok(imm8) => asm::inst::cmpq_mi_sxb::new(src1, imm8).into(),
                Err(_) => asm::inst::cmpq_mi::new(src1, imm).into(),
            },
            OperandSize::S128 => unimplemented!(),
        };

        self.emit(Inst::External { inst });
    }

    pub fn cmp_rr(&mut self, src1: Reg, src2: Reg, size: OperandSize) {
        let inst = match size {
            OperandSize::S8 => asm::inst::cmpb_rm::new(src1, src2).into(),
            OperandSize::S16 => asm::inst::cmpw_rm::new(src1, src2).into(),
            OperandSize::S32 => asm::inst::cmpl_rm::new(src1, src2).into(),
            OperandSize::S64 => asm::inst::cmpq_rm::new(src1, src2).into(),
            OperandSize::S128 => unimplemented!(),
        };

        self.emit(Inst::External { inst });
    }

    /// Compares values in src1 and src2 and sets ZF, PF, and CF flags in EFLAGS
    /// register.
    pub fn ucomis(&mut self, src1: Reg, src2: Reg, size: OperandSize) {
        let inst = match size {
            OperandSize::S32 => asm::inst::ucomiss_a::new(src1, src2).into(),
            OperandSize::S64 => asm::inst::ucomisd_a::new(src1, src2).into(),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn popcnt(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        assert!(
            self.isa_flags.has_popcnt() && self.isa_flags.has_sse42(),
            "Requires has_popcnt and has_sse42 flags"
        );
        let dst = WritableGpr::from_reg(dst.to_reg().into());
        let inst = match size {
            OperandSize::S16 => asm::inst::popcntw_rm::new(dst, src).into(),
            OperandSize::S32 => asm::inst::popcntl_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::popcntq_rm::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Emit a test instruction with two register operands.
    pub fn test_rr(&mut self, src1: Reg, src2: Reg, size: OperandSize) {
        let inst = match size {
            OperandSize::S8 => asm::inst::testb_mr::new(src1, src2).into(),
            OperandSize::S16 => asm::inst::testw_mr::new(src1, src2).into(),
            OperandSize::S32 => asm::inst::testl_mr::new(src1, src2).into(),
            OperandSize::S64 => asm::inst::testq_mr::new(src1, src2).into(),
            OperandSize::S128 => unimplemented!(),
        };

        self.emit(Inst::External { inst });
    }

    /// Set value in dst to `0` or `1` based on flags in status register and
    /// [`CmpKind`].
    pub fn setcc(&mut self, kind: IntCmpKind, dst: WritableReg) {
        self.setcc_impl(kind.into(), dst);
    }

    /// Set value in dst to `1` if parity flag in status register is set, `0`
    /// otherwise.
    pub fn setp(&mut self, dst: WritableReg) {
        self.setcc_impl(CC::P, dst);
    }

    /// Set value in dst to `1` if parity flag in status register is not set,
    /// `0` otherwise.
    pub fn setnp(&mut self, dst: WritableReg) {
        self.setcc_impl(CC::NP, dst);
    }

    fn setcc_impl(&mut self, cc: CC, dst: WritableReg) {
        // Clear the dst register or bits 1 to 31 may be incorrectly set.
        // Don't use xor since it updates the status register.
        let dst: WritableGpr = dst.map(Into::into);
        let inst = asm::inst::movl_oi::new(dst, 0).into();
        self.emit(Inst::External { inst });

        // Copy correct bit from status register into dst register.
        //
        // Note that some of these mnemonics don't match exactly and that's
        // intentional as there are multiple mnemonics for the same encoding in
        // some cases and the assembler picked ones that match Capstone rather
        // than Cranelift.
        let inst = match cc {
            CC::O => asm::inst::seto_m::new(dst).into(),
            CC::NO => asm::inst::setno_m::new(dst).into(),
            CC::B => asm::inst::setb_m::new(dst).into(),
            CC::NB => asm::inst::setae_m::new(dst).into(), //  nb == ae
            CC::Z => asm::inst::sete_m::new(dst).into(),   //   z ==  e
            CC::NZ => asm::inst::setne_m::new(dst).into(), //  nz == ne
            CC::BE => asm::inst::setbe_m::new(dst).into(),
            CC::NBE => asm::inst::seta_m::new(dst).into(), // nbe ==  a
            CC::S => asm::inst::sets_m::new(dst).into(),
            CC::NS => asm::inst::setns_m::new(dst).into(),
            CC::L => asm::inst::setl_m::new(dst).into(),
            CC::NL => asm::inst::setge_m::new(dst).into(), //  nl == ge
            CC::LE => asm::inst::setle_m::new(dst).into(),
            CC::NLE => asm::inst::setg_m::new(dst).into(), // nle ==  g
            CC::P => asm::inst::setp_m::new(dst).into(),
            CC::NP => asm::inst::setnp_m::new(dst).into(),
        };
        self.emit(Inst::External { inst });
    }

    /// Store the count of leading zeroes in src in dst.
    /// Requires `has_lzcnt` flag.
    pub fn lzcnt(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        assert!(self.isa_flags.has_lzcnt(), "Requires has_lzcnt flag");
        let dst = WritableGpr::from_reg(dst.to_reg().into());
        let inst = match size {
            OperandSize::S16 => asm::inst::lzcntw_rm::new(dst, src).into(),
            OperandSize::S32 => asm::inst::lzcntl_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::lzcntq_rm::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Store the count of trailing zeroes in src in dst.
    /// Requires `has_bmi1` flag.
    pub fn tzcnt(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        assert!(self.isa_flags.has_bmi1(), "Requires has_bmi1 flag");
        let dst = WritableGpr::from_reg(dst.to_reg().into());
        let inst = match size {
            OperandSize::S16 => asm::inst::tzcntw_a::new(dst, src).into(),
            OperandSize::S32 => asm::inst::tzcntl_a::new(dst, src).into(),
            OperandSize::S64 => asm::inst::tzcntq_a::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Stores position of the most significant bit set in src in dst.
    /// Zero flag is set if src is equal to 0.
    pub fn bsr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableGpr = WritableGpr::from_reg(dst.to_reg().into());
        let inst = match size {
            OperandSize::S16 => asm::inst::bsrw_rm::new(dst, src).into(),
            OperandSize::S32 => asm::inst::bsrl_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::bsrq_rm::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs integer negation on `src` and places result in `dst`.
    pub fn neg(&mut self, read: Reg, write: WritableReg, size: OperandSize) {
        let gpr = PairedGpr {
            read: read.into(),
            write: WritableGpr::from_reg(write.to_reg().into()),
        };
        let inst = match size {
            OperandSize::S8 => asm::inst::negb_m::new(gpr).into(),
            OperandSize::S16 => asm::inst::negw_m::new(gpr).into(),
            OperandSize::S32 => asm::inst::negl_m::new(gpr).into(),
            OperandSize::S64 => asm::inst::negq_m::new(gpr).into(),
            OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Stores position of the least significant bit set in src in dst.
    /// Zero flag is set if src is equal to 0.
    pub fn bsf(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableGpr = WritableGpr::from_reg(dst.to_reg().into());
        let inst = match size {
            OperandSize::S16 => asm::inst::bsfw_rm::new(dst, src).into(),
            OperandSize::S32 => asm::inst::bsfl_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::bsfq_rm::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs float addition on src and dst and places result in dst.
    pub fn xmm_add_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_xmm(dst);
        let inst = match size {
            OperandSize::S32 => asm::inst::addss_a::new(dst, src).into(),
            OperandSize::S64 => asm::inst::addsd_a::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs float subtraction on src and dst and places result in dst.
    pub fn xmm_sub_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_xmm(dst);
        let inst = match size {
            OperandSize::S32 => asm::inst::subss_a::new(dst, src).into(),
            OperandSize::S64 => asm::inst::subsd_a::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs float multiplication on src and dst and places result in dst.
    pub fn xmm_mul_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        use OperandSize::*;
        let dst = pair_xmm(dst);
        let inst = match size {
            S32 => asm::inst::mulss_a::new(dst, src).into(),
            S64 => asm::inst::mulsd_a::new(dst, src).into(),
            S8 | S16 | S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs float division on src and dst and places result in dst.
    pub fn xmm_div_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_xmm(dst);
        let inst = match size {
            OperandSize::S32 => asm::inst::divss_a::new(dst, src).into(),
            OperandSize::S64 => asm::inst::divsd_a::new(dst, src).into(),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Minimum for src and dst XMM registers with results put in dst.
    pub fn xmm_min_seq(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        self.emit(Inst::XmmMinMaxSeq {
            size: size.into(),
            is_min: true,
            lhs: src.into(),
            rhs: dst.to_reg().into(),
            dst: dst.map(Into::into),
        });
    }

    /// Maximum for src and dst XMM registers with results put in dst.
    pub fn xmm_max_seq(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        self.emit(Inst::XmmMinMaxSeq {
            size: size.into(),
            is_min: false,
            lhs: src.into(),
            rhs: dst.to_reg().into(),
            dst: dst.map(Into::into),
        });
    }

    /// Perform rounding operation on float register src and place results in
    /// float register dst.
    pub fn xmm_rounds_rr(
        &mut self,
        src: Reg,
        dst: WritableReg,
        mode: RoundingMode,
        size: OperandSize,
    ) {
        let dst = dst.map(|r| r.into());

        let imm: u8 = match mode {
            RoundingMode::Nearest => 0x00,
            RoundingMode::Down => 0x01,
            RoundingMode::Up => 0x02,
            RoundingMode::Zero => 0x03,
        };

        let inst = match size {
            OperandSize::S32 => asm::inst::roundss_rmi::new(dst, src, imm).into(),
            OperandSize::S64 => asm::inst::roundsd_rmi::new(dst, src, imm).into(),
            OperandSize::S8 | OperandSize::S16 | OperandSize::S128 => unreachable!(),
        };

        self.emit(Inst::External { inst });
    }

    pub fn sqrt(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        use OperandSize::*;
        let dst = pair_xmm(dst);
        let inst = match size {
            S32 => asm::inst::sqrtss_a::new(dst, src).into(),
            S64 => asm::inst::sqrtsd_a::new(dst, src).into(),
            S8 | S16 | S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Emit a call to an unknown location through a register.
    pub fn call_with_reg(&mut self, cc: CallingConvention, callee: Reg) {
        self.emit(Inst::CallUnknown {
            info: Box::new(CallInfo::empty(RegMem::reg(callee.into()), cc.into())),
        });
    }

    /// Emit a call to a locally defined function through an index.
    pub fn call_with_name(&mut self, cc: CallingConvention, name: UserExternalNameRef) {
        self.emit(Inst::CallKnown {
            info: Box::new(CallInfo::empty(ExternalName::user(name), cc.into())),
        });
    }

    /// Emits a conditional jump to the given label.
    pub fn jmp_if(&mut self, cc: impl Into<CC>, taken: MachLabel) {
        self.emit(Inst::WinchJmpIf {
            cc: cc.into(),
            taken,
        });
    }

    /// Performs an unconditional jump to the given label.
    pub fn jmp(&mut self, target: MachLabel) {
        self.emit(Inst::JmpKnown { dst: target });
    }

    /// Emits a jump table sequence.
    pub fn jmp_table(
        &mut self,
        targets: SmallVec<[MachLabel; 4]>,
        default: MachLabel,
        index: Reg,
        tmp1: Reg,
        tmp2: Reg,
    ) {
        self.emit(Inst::JmpTableSeq {
            idx: index.into(),
            tmp1: Writable::from_reg(tmp1.into()),
            tmp2: Writable::from_reg(tmp2.into()),
            default_target: default,
            targets: Box::new(targets.to_vec()),
        })
    }

    /// Emit a trap instruction.
    pub fn trap(&mut self, code: TrapCode) {
        let inst = asm::inst::ud2_zo::new(code).into();
        self.emit(Inst::External { inst });
    }

    /// Conditional trap.
    pub fn trapif(&mut self, cc: impl Into<CC>, trap_code: TrapCode) {
        self.emit(Inst::TrapIf {
            cc: cc.into(),
            trap_code,
        });
    }

    /// Load effective address.
    pub fn lea(&mut self, addr: &Address, dst: WritableReg, size: OperandSize) {
        let addr = Self::to_synthetic_amode(addr, MemFlags::trusted());
        let dst: WritableGpr = dst.map(Into::into);
        let inst = match size {
            OperandSize::S16 => asm::inst::leaw_rm::new(dst, addr).into(),
            OperandSize::S32 => asm::inst::leal_rm::new(dst, addr).into(),
            OperandSize::S64 => asm::inst::leaq_rm::new(dst, addr).into(),
            OperandSize::S8 | OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn adc_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::adcb_rm::new(dst, src).into(),
            OperandSize::S16 => asm::inst::adcw_rm::new(dst, src).into(),
            OperandSize::S32 => asm::inst::adcl_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::adcq_rm::new(dst, src).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn sbb_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst = pair_gpr(dst);
        let inst = match size {
            OperandSize::S8 => asm::inst::sbbb_rm::new(dst, src).into(),
            OperandSize::S16 => asm::inst::sbbw_rm::new(dst, src).into(),
            OperandSize::S32 => asm::inst::sbbl_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::sbbq_rm::new(dst, src).into(),
            OperandSize::S128 => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn mul_wide(
        &mut self,
        dst_lo: WritableReg,
        dst_hi: WritableReg,
        lhs: Reg,
        rhs: Reg,
        kind: MulWideKind,
        size: OperandSize,
    ) {
        use MulWideKind::*;
        use OperandSize::*;
        let rax = asm::Fixed(PairedGpr {
            read: lhs.into(),
            write: WritableGpr::from_reg(dst_lo.to_reg().into()),
        });
        let rdx = asm::Fixed(dst_hi.to_reg().into());
        if size == S8 {
            // For `mulb` and `imulb`, both the high and low bits are written to
            // RAX.
            assert_eq!(dst_lo, dst_hi);
        }
        let inst = match (size, kind) {
            (S8, Unsigned) => asm::inst::mulb_m::new(rax, rhs).into(),
            (S8, Signed) => asm::inst::imulb_m::new(rax, rhs).into(),
            (S16, Unsigned) => asm::inst::mulw_m::new(rax, rdx, rhs).into(),
            (S16, Signed) => asm::inst::imulw_m::new(rax, rdx, rhs).into(),
            (S32, Unsigned) => asm::inst::mull_m::new(rax, rdx, rhs).into(),
            (S32, Signed) => asm::inst::imull_m::new(rax, rdx, rhs).into(),
            (S64, Unsigned) => asm::inst::mulq_m::new(rax, rdx, rhs).into(),
            (S64, Signed) => asm::inst::imulq_m::new(rax, rdx, rhs).into(),
            (S128, _) => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Shuffles bytes in `src` according to contents of `mask` and puts
    /// result in `dst`.
    pub fn xmm_vpshufb_rrm(&mut self, dst: WritableReg, src: Reg, mask: &Address) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let mask = Self::to_synthetic_amode(mask, MemFlags::trusted());
        let inst = asm::inst::vpshufb_b::new(dst, src, mask).into();
        self.emit(Inst::External { inst });
    }

    /// Shuffles bytes in `src` according to contents of `mask` and puts
    /// result in `dst`.
    pub fn xmm_vpshufb_rrr(&mut self, dst: WritableReg, src: Reg, mask: Reg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vpshufb_b::new(dst, src, mask).into();
        self.emit(Inst::External { inst });
    }

    /// Add unsigned integers with unsigned saturation.
    ///
    /// Adds the src operands but when an individual byte result is larger than
    /// an unsigned byte integer, 0xFF is written instead.
    pub fn xmm_vpaddus_rrm(
        &mut self,
        dst: WritableReg,
        src1: Reg,
        src2: &Address,
        size: OperandSize,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let src2 = Self::to_synthetic_amode(src2, MemFlags::trusted());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpaddusb_b::new(dst, src1, src2).into(),
            OperandSize::S32 => asm::inst::vpaddusw_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Add unsigned integers with unsigned saturation.
    ///
    /// Adds the src operands but when an individual byte result is larger than
    /// an unsigned byte integer, 0xFF is written instead.
    pub fn xmm_vpaddus_rrr(&mut self, dst: WritableReg, src1: Reg, src2: Reg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpaddusb_b::new(dst, src1, src2).into(),
            OperandSize::S16 => asm::inst::vpaddusw_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Add signed integers.
    pub fn xmm_vpadds_rrr(&mut self, dst: WritableReg, src1: Reg, src2: Reg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpaddsb_b::new(dst, src1, src2).into(),
            OperandSize::S16 => asm::inst::vpaddsw_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn xmm_vpadd_rmr(
        &mut self,
        src1: Reg,
        src2: &Address,
        dst: WritableReg,
        size: OperandSize,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(src2, MemFlags::trusted());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpaddb_b::new(dst, src1, address).into(),
            OperandSize::S16 => asm::inst::vpaddw_b::new(dst, src1, address).into(),
            OperandSize::S32 => asm::inst::vpaddd_b::new(dst, src1, address).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Adds vectors of integers in `src1` and `src2` and puts the results in
    /// `dst`.
    pub fn xmm_vpadd_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpaddb_b::new(dst, src1, src2).into(),
            OperandSize::S16 => asm::inst::vpaddw_b::new(dst, src1, src2).into(),
            OperandSize::S32 => asm::inst::vpaddd_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vpaddq_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn mfence(&mut self) {
        self.emit(Inst::External {
            inst: asm::inst::mfence_zo::new().into(),
        });
    }

    /// Extract a value from `src` into `addr` determined by `lane`.
    pub(crate) fn xmm_vpextr_rm(
        &mut self,
        addr: &Address,
        src: Reg,
        lane: u8,
        size: OperandSize,
        flags: MemFlags,
    ) {
        assert!(addr.is_offset());
        let dst = Self::to_synthetic_amode(addr, flags);
        let inst = match size {
            OperandSize::S8 => asm::inst::vpextrb_a::new(dst, src, lane).into(),
            OperandSize::S16 => asm::inst::vpextrw_b::new(dst, src, lane).into(),
            OperandSize::S32 => asm::inst::vpextrd_a::new(dst, src, lane).into(),
            OperandSize::S64 => asm::inst::vpextrq_a::new(dst, src, lane).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Extract a value from `src` into `dst` (zero extended) determined by `lane`.
    pub fn xmm_vpextr_rr(&mut self, dst: WritableReg, src: Reg, lane: u8, size: OperandSize) {
        let dst: WritableGpr = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpextrb_a::new(dst, src, lane).into(),
            OperandSize::S16 => asm::inst::vpextrw_a::new(dst, src, lane).into(),
            OperandSize::S32 => asm::inst::vpextrd_a::new(dst, src, lane).into(),
            OperandSize::S64 => asm::inst::vpextrq_a::new(dst, src, lane).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Copy value from `src2`, merge into `src1`, and put result in `dst` at
    /// the location specified in `count`.
    pub fn xmm_vpinsr_rrm(
        &mut self,
        dst: WritableReg,
        src1: Reg,
        src2: &Address,
        count: u8,
        size: OperandSize,
    ) {
        let src2 = Self::to_synthetic_amode(src2, MemFlags::trusted());
        let dst: WritableXmm = dst.map(|r| r.into());

        let inst = match size {
            OperandSize::S8 => asm::inst::vpinsrb_b::new(dst, src1, src2, count).into(),
            OperandSize::S16 => asm::inst::vpinsrw_b::new(dst, src1, src2, count).into(),
            OperandSize::S32 => asm::inst::vpinsrd_b::new(dst, src1, src2, count).into(),
            OperandSize::S64 => asm::inst::vpinsrq_b::new(dst, src1, src2, count).into(),
            OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Copy value from `src2`, merge into `src1`, and put result in `dst` at
    /// the location specified in `count`.
    pub fn xmm_vpinsr_rrr(
        &mut self,
        dst: WritableReg,
        src1: Reg,
        src2: Reg,
        count: u8,
        size: OperandSize,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpinsrb_b::new(dst, src1, src2, count).into(),
            OperandSize::S16 => asm::inst::vpinsrw_b::new(dst, src1, src2, count).into(),
            OperandSize::S32 => asm::inst::vpinsrd_b::new(dst, src1, src2, count).into(),
            OperandSize::S64 => asm::inst::vpinsrq_b::new(dst, src1, src2, count).into(),
            OperandSize::S128 => unreachable!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Copy a 32-bit float in `src2`, merge into `src1`, and put result in `dst`.
    pub fn xmm_vinsertps_rrm(&mut self, dst: WritableReg, src1: Reg, address: &Address, imm: u8) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(address, MemFlags::trusted());
        let inst = asm::inst::vinsertps_b::new(dst, src1, address, imm).into();
        self.emit(Inst::External { inst });
    }

    /// Copy a 32-bit float in `src2`, merge into `src1`, and put result in `dst`.
    pub fn xmm_vinsertps_rrr(&mut self, dst: WritableReg, src1: Reg, src2: Reg, imm: u8) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vinsertps_b::new(dst, src1, src2, imm).into();
        self.emit(Inst::External { inst });
    }

    /// Moves lower 64-bit float from `src2` into lower 64-bits of `dst` and the
    /// upper 64-bits in `src1` into the upper 64-bits of `dst`.
    pub fn xmm_vmovsd_rrr(&mut self, dst: WritableReg, src1: Reg, src2: Reg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vmovsd_b::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }

    /// Moves 64-bit float from `src` into lower 64-bits of `dst`.
    /// Zeroes out the upper 64 bits of `dst`.
    pub fn xmm_vmovsd_rm(&mut self, dst: WritableReg, src: &Address) {
        let src = Self::to_synthetic_amode(src, MemFlags::trusted());
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vmovsd_d::new(dst, src).into();
        self.emit(Inst::External { inst });
    }

    /// Moves two 32-bit floats from `src2` to the upper 64-bits of `dst`.
    /// Copies two 32-bit floats from the lower 64-bits of `src1` to lower
    /// 64-bits of `dst`.
    pub fn xmm_vmovlhps_rrm(&mut self, dst: WritableReg, src1: Reg, src2: &Address) {
        let src2 = Self::to_synthetic_amode(src2, MemFlags::trusted());
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vmovhps_b::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }

    /// Moves two 32-bit floats from the lower 64-bits of `src2` to the upper
    /// 64-bits of `dst`. Copies two 32-bit floats from the lower 64-bits of
    /// `src1` to lower 64-bits of `dst`.
    pub fn xmm_vmovlhps_rrr(&mut self, dst: WritableReg, src1: Reg, src2: Reg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vmovlhps_rvm::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }

    /// Move unaligned packed integer values from address `src` to `dst`.
    pub fn xmm_vmovdqu_mr(&mut self, src: &Address, dst: WritableReg, flags: MemFlags) {
        let src = Self::to_synthetic_amode(src, flags);
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vmovdqu_a::new(dst, src).into();
        self.emit(Inst::External { inst });
    }

    /// Move integer from `src` to xmm register `dst` using an AVX instruction.
    pub fn avx_gpr_to_xmm(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vmovd_a::new(dst, src).into(),
            OperandSize::S64 => asm::inst::vmovq_a::new(dst, src).into(),
            _ => unreachable!(),
        };

        self.emit(Inst::External { inst });
    }

    pub fn xmm_vptest(&mut self, src1: Reg, src2: Reg) {
        let inst = asm::inst::vptest_rm::new(src1, src2).into();
        self.emit(Inst::External { inst });
    }

    /// Converts vector of integers into vector of floating values.
    pub fn xmm_vcvt_rr(&mut self, src: Reg, dst: WritableReg, kind: VcvtKind) {
        let dst: WritableXmm = dst.map(|x| x.into());
        let inst = match kind {
            VcvtKind::I32ToF32 => asm::inst::vcvtdq2ps_a::new(dst, src).into(),
            VcvtKind::I32ToF64 => asm::inst::vcvtdq2pd_a::new(dst, src).into(),
            VcvtKind::F64ToF32 => asm::inst::vcvtpd2ps_a::new(dst, src).into(),
            VcvtKind::F64ToI32 => asm::inst::vcvttpd2dq_a::new(dst, src).into(),
            VcvtKind::F32ToF64 => asm::inst::vcvtps2pd_a::new(dst, src).into(),
            VcvtKind::F32ToI32 => asm::inst::vcvttps2dq_a::new(dst, src).into(),
        };
        self.emit(Inst::External { inst });
    }

    /// Subtract floats in vector `src1` to floats in vector `src2`.
    pub fn xmm_vsubp_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vsubps_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vsubpd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Subtract integers in vector `src1` from integers in vector `src2`.
    pub fn xmm_vpsub_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpsubb_b::new(dst, src1, src2).into(),
            OperandSize::S16 => asm::inst::vpsubw_b::new(dst, src1, src2).into(),
            OperandSize::S32 => asm::inst::vpsubd_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vpsubq_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Substract unsigned integers with unsigned saturation.
    pub fn xmm_vpsubus_rrr(&mut self, dst: WritableReg, src1: Reg, src2: Reg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpsubusb_b::new(dst, src1, src2).into(),
            OperandSize::S16 => asm::inst::vpsubusw_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Subtract signed integers with signed saturation.
    pub fn xmm_vpsubs_rrr(&mut self, dst: WritableReg, src1: Reg, src2: Reg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpsubsb_b::new(dst, src1, src2).into(),
            OperandSize::S16 => asm::inst::vpsubsw_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Add floats in vector `src1` to floats in vector `src2`.
    pub fn xmm_vaddp_rrm(
        &mut self,
        src1: Reg,
        src2: &Address,
        dst: WritableReg,
        size: OperandSize,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(src2, MemFlags::trusted());
        let inst = match size {
            OperandSize::S32 => asm::inst::vaddps_b::new(dst, src1, address).into(),
            OperandSize::S64 => asm::inst::vaddpd_b::new(dst, src1, address).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Add floats in vector `src1` to floats in vector `src2`.
    pub fn xmm_vaddp_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vaddps_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vaddpd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Compare vector register `lhs` with a vector of integers in `rhs` for
    /// equality between packed integers and write the resulting vector into
    /// `dst`.
    pub fn xmm_vpcmpeq_rrm(
        &mut self,
        dst: WritableReg,
        lhs: Reg,
        address: &Address,
        size: OperandSize,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(address, MemFlags::trusted());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpcmpeqb_b::new(dst, lhs, address).into(),
            OperandSize::S16 => asm::inst::vpcmpeqw_b::new(dst, lhs, address).into(),
            OperandSize::S32 => asm::inst::vpcmpeqd_b::new(dst, lhs, address).into(),
            OperandSize::S64 => asm::inst::vpcmpeqq_b::new(dst, lhs, address).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Compare vector registers `lhs` and `rhs` for equality between packed
    /// integers and write the resulting vector into `dst`.
    pub fn xmm_vpcmpeq_rrr(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpcmpeqb_b::new(dst, lhs, rhs).into(),
            OperandSize::S16 => asm::inst::vpcmpeqw_b::new(dst, lhs, rhs).into(),
            OperandSize::S32 => asm::inst::vpcmpeqd_b::new(dst, lhs, rhs).into(),
            OperandSize::S64 => asm::inst::vpcmpeqq_b::new(dst, lhs, rhs).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs a greater than comparison with vectors of signed integers in
    /// `lhs` and `rhs` and puts the results in `dst`.
    pub fn xmm_vpcmpgt_rrr(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpcmpgtb_b::new(dst, lhs, rhs).into(),
            OperandSize::S16 => asm::inst::vpcmpgtw_b::new(dst, lhs, rhs).into(),
            OperandSize::S32 => asm::inst::vpcmpgtd_b::new(dst, lhs, rhs).into(),
            OperandSize::S64 => asm::inst::vpcmpgtq_b::new(dst, lhs, rhs).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs a max operation with vectors of signed integers in `lhs` and
    /// `rhs` and puts the results in `dst`.
    pub fn xmm_vpmaxs_rrr(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpmaxsb_b::new(dst, lhs, rhs).into(),
            OperandSize::S16 => asm::inst::vpmaxsw_b::new(dst, lhs, rhs).into(),
            OperandSize::S32 => asm::inst::vpmaxsd_b::new(dst, lhs, rhs).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs a max operation with vectors of unsigned integers in `lhs` and
    /// `rhs` and puts the results in `dst`.
    pub fn xmm_vpmaxu_rrr(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpmaxub_b::new(dst, lhs, rhs).into(),
            OperandSize::S16 => asm::inst::vpmaxuw_b::new(dst, lhs, rhs).into(),
            OperandSize::S32 => asm::inst::vpmaxud_b::new(dst, lhs, rhs).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs a min operation with vectors of signed integers in `lhs` and
    /// `rhs` and puts the results in `dst`.
    pub fn xmm_vpmins_rrr(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpminsb_b::new(dst, lhs, rhs).into(),
            OperandSize::S16 => asm::inst::vpminsw_b::new(dst, lhs, rhs).into(),
            OperandSize::S32 => asm::inst::vpminsd_b::new(dst, lhs, rhs).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs a min operation with vectors of unsigned integers in `lhs` and
    /// `rhs` and puts the results in `dst`.
    pub fn xmm_vpminu_rrr(&mut self, dst: WritableReg, lhs: Reg, rhs: Reg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpminub_b::new(dst, lhs, rhs).into(),
            OperandSize::S16 => asm::inst::vpminuw_b::new(dst, lhs, rhs).into(),
            OperandSize::S32 => asm::inst::vpminud_b::new(dst, lhs, rhs).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs a comparison operation between vectors of floats in `lhs` and
    /// `rhs` and puts the results in `dst`.
    pub fn xmm_vcmpp_rrr(
        &mut self,
        dst: WritableReg,
        lhs: Reg,
        rhs: Reg,
        size: OperandSize,
        kind: VcmpKind,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let imm = match kind {
            VcmpKind::Eq => 0,
            VcmpKind::Lt => 1,
            VcmpKind::Le => 2,
            VcmpKind::Unord => 3,
            VcmpKind::Ne => 4,
        };
        let inst = match size {
            OperandSize::S32 => asm::inst::vcmpps_b::new(dst, lhs, rhs, imm).into(),
            OperandSize::S64 => asm::inst::vcmppd_b::new(dst, lhs, rhs, imm).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs a subtraction on two vectors of floats and puts the results in
    /// `dst`.
    pub fn xmm_vsub_rrm(&mut self, src1: Reg, src2: &Address, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(src2, MemFlags::trusted());
        let inst = match size {
            OperandSize::S64 => asm::inst::vsubpd_b::new(dst, src1, address).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs a subtraction on two vectors of floats and puts the results in
    /// `dst`.
    pub fn xmm_vsub_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vsubps_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vsubpd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Converts a vector of signed integers into a vector of narrower integers
    /// using saturation to handle overflow.
    pub fn xmm_vpackss_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpacksswb_b::new(dst, src1, src2).into(),
            OperandSize::S16 => asm::inst::vpackssdw_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Converts a vector of unsigned integers into a vector of narrower
    /// integers using saturation to handle overflow.
    pub fn xmm_vpackus_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpackuswb_b::new(dst, src1, src2).into(),
            OperandSize::S16 => asm::inst::vpackusdw_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Concatenates `src1` and `src2` and shifts right by `imm` and puts
    /// result in `dst`.
    pub fn xmm_vpalignr_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, imm: u8) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vpalignr_b::new(dst, src1, src2, imm).into();
        self.emit(Inst::External { inst });
    }

    /// Takes the lower lanes of vectors of floats in `src1` and `src2` and
    /// interleaves them in `dst`.
    pub fn xmm_vunpcklp_rrm(
        &mut self,
        src1: Reg,
        src2: &Address,
        dst: WritableReg,
        size: OperandSize,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(src2, MemFlags::trusted());
        let inst = match size {
            OperandSize::S32 => asm::inst::vunpcklps_b::new(dst, src1, address).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Unpacks and interleaves high order data of floats in `src1` and `src2`
    /// and puts the results in `dst`.
    pub fn xmm_vunpckhp_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vunpckhps_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Unpacks and interleaves the lower lanes of vectors of integers in `src1`
    /// and `src2` and puts the results in `dst`.
    pub fn xmm_vpunpckl_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpunpcklbw_b::new(dst, src1, src2).into(),
            OperandSize::S16 => asm::inst::vpunpcklwd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Unpacks and interleaves the higher lanes of vectors of integers in
    /// `src1` and `src2` and puts the results in `dst`.
    pub fn xmm_vpunpckh_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpunpckhbw_b::new(dst, src1, src2).into(),
            OperandSize::S16 => asm::inst::vpunpckhwd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub(crate) fn vpmullq(&mut self, src1: Reg, src2: Reg, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vpmullq_c::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }

    /// Creates a mask made up of the most significant bit of each byte of
    /// `src` and stores the result in `dst`.
    pub fn xmm_vpmovmsk_rr(
        &mut self,
        src: Reg,
        dst: WritableReg,
        src_size: OperandSize,
        dst_size: OperandSize,
    ) {
        assert_eq!(dst_size, OperandSize::S32);
        let dst: WritableGpr = dst.map(|r| r.into());
        let inst = match src_size {
            OperandSize::S8 => asm::inst::vpmovmskb_rm::new(dst, src).into(),
            _ => unimplemented!(),
        };

        self.emit(Inst::External { inst });
    }

    /// Creates a mask made up of the most significant bit of each byte of
    /// in `src` and stores the result in `dst`.
    pub fn xmm_vmovskp_rr(
        &mut self,
        src: Reg,
        dst: WritableReg,
        src_size: OperandSize,
        dst_size: OperandSize,
    ) {
        assert_eq!(dst_size, OperandSize::S32);
        let dst: WritableGpr = dst.map(|r| r.into());
        let inst = match src_size {
            OperandSize::S32 => asm::inst::vmovmskps_rm::new(dst, src).into(),
            OperandSize::S64 => asm::inst::vmovmskpd_rm::new(dst, src).into(),
            _ => unimplemented!(),
        };

        self.emit(Inst::External { inst });
    }

    /// Compute the absolute value of elements in vector `src` and put the
    /// results in `dst`.
    pub fn xmm_vpabs_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpabsb_a::new(dst, src).into(),
            OperandSize::S16 => asm::inst::vpabsw_a::new(dst, src).into(),
            OperandSize::S32 => asm::inst::vpabsd_a::new(dst, src).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Arithmetically (sign preserving) right shift on vector in `src` by
    /// `amount` with result written to `dst`.
    pub fn xmm_vpsra_rrr(&mut self, src: Reg, amount: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S16 => asm::inst::vpsraw_c::new(dst, src, amount).into(),
            OperandSize::S32 => asm::inst::vpsrad_c::new(dst, src, amount).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Arithmetically (sign preserving) right shift on vector in `src` by
    /// `imm` with result written to `dst`.
    pub fn xmm_vpsra_rri(&mut self, src: Reg, dst: WritableReg, imm: u32, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let imm = u8::try_from(imm).expect("immediate must fit in 8 bits");
        let inst = match size {
            OperandSize::S32 => asm::inst::vpsrad_d::new(dst, src, imm).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Shift vector data left by `imm`.
    pub fn xmm_vpsll_rri(&mut self, src: Reg, dst: WritableReg, imm: u32, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let imm = u8::try_from(imm).expect("immediate must fit in 8 bits");
        let inst = match size {
            OperandSize::S32 => asm::inst::vpslld_d::new(dst, src, imm).into(),
            OperandSize::S64 => asm::inst::vpsllq_d::new(dst, src, imm).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Shift vector data left by `amount`.
    pub fn xmm_vpsll_rrr(&mut self, src: Reg, amount: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S16 => asm::inst::vpsllw_c::new(dst, src, amount).into(),
            OperandSize::S32 => asm::inst::vpslld_c::new(dst, src, amount).into(),
            OperandSize::S64 => asm::inst::vpsllq_c::new(dst, src, amount).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Shift vector data right by `imm`.
    pub fn xmm_vpsrl_rri(&mut self, src: Reg, dst: WritableReg, imm: u32, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let imm = u8::try_from(imm).expect("immediate must fit in 8 bits");
        let inst = match size {
            OperandSize::S16 => asm::inst::vpsrlw_d::new(dst, src, imm).into(),
            OperandSize::S32 => asm::inst::vpsrld_d::new(dst, src, imm).into(),
            OperandSize::S64 => asm::inst::vpsrlq_d::new(dst, src, imm).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Shift vector data right by `amount`.
    pub fn xmm_vpsrl_rrr(&mut self, src: Reg, amount: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S16 => asm::inst::vpsrlw_c::new(dst, src, amount).into(),
            OperandSize::S32 => asm::inst::vpsrld_c::new(dst, src, amount).into(),
            OperandSize::S64 => asm::inst::vpsrlq_c::new(dst, src, amount).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Perform an `and` operation on vectors of floats in `src1` and `src2`
    /// and put the results in `dst`.
    pub fn xmm_vandp_rrm(
        &mut self,
        src1: Reg,
        src2: &Address,
        dst: WritableReg,
        size: OperandSize,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(src2, MemFlags::trusted());
        let inst = match size {
            OperandSize::S32 => asm::inst::vandps_b::new(dst, src1, address).into(),
            OperandSize::S64 => asm::inst::vandpd_b::new(dst, src1, address).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Perform an `and` operation on vectors of floats in `src1` and `src2`
    /// and put the results in `dst`.
    pub fn xmm_vandp_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vandps_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vandpd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Performs a bitwise `and` operation on the vectors in `src1` and `src2`
    /// and stores the results in `dst`.
    pub fn xmm_vpand_rrm(&mut self, src1: Reg, src2: &Address, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(&src2, MemFlags::trusted());
        let inst = asm::inst::vpand_b::new(dst, src1, address).into();
        self.emit(Inst::External { inst });
    }

    /// Performs a bitwise `and` operation on the vectors in `src1` and `src2`
    /// and stores the results in `dst`.
    pub fn xmm_vpand_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vpand_b::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }

    /// Perform an `and not` operation on vectors of floats in `src1` and
    /// `src2` and put the results in `dst`.
    pub fn xmm_vandnp_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vandnps_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vandnpd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Perform an `and not` operation on vectors in `src1` and `src2` and put
    /// the results in `dst`.
    pub fn xmm_vpandn_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vpandn_b::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }

    /// Perform an or operation for the vectors of floats in `src1` and `src2`
    /// and put the results in `dst`.
    pub fn xmm_vorp_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vorps_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vorpd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Bitwise OR of `src1` and `src2`.
    pub fn xmm_vpor_rrr(&mut self, dst: WritableReg, src1: Reg, src2: Reg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vpor_b::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }

    /// Bitwise logical xor of vectors of floats in `src1` and `src2` and puts
    /// the results in `dst`.
    pub fn xmm_vxorp_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vxorps_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vxorpd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Perform a logical on vector in `src` and in `address` and put the
    /// results in `dst`.
    pub fn xmm_vpxor_rmr(&mut self, src: Reg, address: &Address, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(address, MemFlags::trusted());
        let inst = asm::inst::vpxor_b::new(dst, src, address).into();
        self.emit(Inst::External { inst });
    }

    /// Perform a logical on vectors in `src1` and `src2` and put the results in
    /// `dst`.
    pub fn xmm_vpxor_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vpxor_b::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }

    /// Perform a max operation across two vectors of floats and put the
    /// results in `dst`.
    pub fn xmm_vmaxp_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vmaxps_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vmaxpd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    // Perform a min operation across two vectors of floats and put the
    // results in `dst`.
    pub fn xmm_vminp_rrm(
        &mut self,
        src1: Reg,
        src2: &Address,
        dst: WritableReg,
        size: OperandSize,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(src2, MemFlags::trusted());
        let inst = match size {
            OperandSize::S32 => asm::inst::vminps_b::new(dst, src1, address).into(),
            OperandSize::S64 => asm::inst::vminpd_b::new(dst, src1, address).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    // Perform a min operation across two vectors of floats and put the
    // results in `dst`.
    pub fn xmm_vminp_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vminps_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vminpd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    // Round a vector of floats.
    pub fn xmm_vroundp_rri(
        &mut self,
        src: Reg,
        dst: WritableReg,
        mode: VroundMode,
        size: OperandSize,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let imm = match mode {
            VroundMode::TowardNearest => 0,
            VroundMode::TowardNegativeInfinity => 1,
            VroundMode::TowardPositiveInfinity => 2,
            VroundMode::TowardZero => 3,
        };

        let inst = match size {
            OperandSize::S32 => asm::inst::vroundps_rmi::new(dst, src, imm).into(),
            OperandSize::S64 => asm::inst::vroundpd_rmi::new(dst, src, imm).into(),
            _ => unimplemented!(),
        };

        self.emit(Inst::External { inst });
    }

    /// Shuffle of vectors of floats.
    pub fn xmm_vshufp_rrri(
        &mut self,
        src1: Reg,
        src2: Reg,
        dst: WritableReg,
        imm: u8,
        size: OperandSize,
    ) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vshufps_b::new(dst, src1, src2, imm).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Each lane in `src1` is multiplied by the corresponding lane in `src2`
    /// producing intermediate 32-bit operands. Each intermediate 32-bit
    /// operand is truncated to 18 most significant bits. Rounding is performed
    /// by adding 1 to the least significant bit of the 18-bit intermediate
    /// result. The 16 bits immediately to the right of the most significant
    /// bit of each 18-bit intermediate result is placed in each lane of `dst`.
    pub fn xmm_vpmulhrs_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S16 => asm::inst::vpmulhrsw_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn xmm_vpmuldq_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vpmuldq_b::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }

    pub fn xmm_vpmuludq_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vpmuludq_b::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }

    pub fn xmm_vpmull_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S16 => asm::inst::vpmullw_b::new(dst, src1, src2).into(),
            OperandSize::S32 => asm::inst::vpmulld_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    pub fn xmm_vmulp_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vmulps_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vmulpd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Perform an average operation for the vector of unsigned integers in
    /// `src1` and `src2` and put the results in `dst`.
    pub fn xmm_vpavg_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S8 => asm::inst::vpavgb_b::new(dst, src1, src2).into(),
            OperandSize::S16 => asm::inst::vpavgw_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Divide the vector of floats in `src1` by the vector of floats in `src2`
    /// and put the results in `dst`.
    pub fn xmm_vdivp_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vdivps_b::new(dst, src1, src2).into(),
            OperandSize::S64 => asm::inst::vdivpd_b::new(dst, src1, src2).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Compute square roots of vector of floats in `src` and put the results
    /// in `dst`.
    pub fn xmm_vsqrtp_rr(&mut self, src: Reg, dst: WritableReg, size: OperandSize) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = match size {
            OperandSize::S32 => asm::inst::vsqrtps_b::new(dst, src).into(),
            OperandSize::S64 => asm::inst::vsqrtpd_b::new(dst, src).into(),
            _ => unimplemented!(),
        };
        self.emit(Inst::External { inst });
    }

    /// Multiply and add packed signed and unsigned bytes.
    pub fn xmm_vpmaddubsw_rmr(&mut self, src: Reg, address: &Address, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(address, MemFlags::trusted());
        let inst = asm::inst::vpmaddubsw_b::new(dst, src, address).into();
        self.emit(Inst::External { inst });
    }

    /// Multiply and add packed signed and unsigned bytes.
    pub fn xmm_vpmaddubsw_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vpmaddubsw_b::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }

    /// Multiple and add packed integers.
    pub fn xmm_vpmaddwd_rmr(&mut self, src: Reg, address: &Address, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let address = Self::to_synthetic_amode(address, MemFlags::trusted());
        let inst = asm::inst::vpmaddwd_b::new(dst, src, address).into();
        self.emit(Inst::External { inst });
    }

    /// Multiple and add packed integers.
    pub fn xmm_vpmaddwd_rrr(&mut self, src1: Reg, src2: Reg, dst: WritableReg) {
        let dst: WritableXmm = dst.map(|r| r.into());
        let inst = asm::inst::vpmaddwd_b::new(dst, src1, src2).into();
        self.emit(Inst::External { inst });
    }
}

/// Captures the region in a MachBuffer where an add-with-immediate instruction would be emitted,
/// but the immediate is not yet known. Currently, this implementation expects a 32-bit immediate,
/// so 8 and 16 bit operand sizes are not supported.
pub(crate) struct PatchableAddToReg {
    /// The region to be patched in the [`MachBuffer`]. It must contain a valid add instruction
    /// sequence, accepting a 32-bit immediate.
    region: PatchRegion,

    /// The offset into the patchable region where the patchable constant begins.
    constant_offset: usize,
}

impl PatchableAddToReg {
    /// Create a new [`PatchableAddToReg`] by capturing a region in the output buffer where the
    /// add-with-immediate occurs. The [`MachBuffer`] will have and add-with-immediate instruction
    /// present in that region, though it will add `0` until the `::finalize` method is called.
    ///
    /// Currently this implementation expects to be able to patch a 32-bit immediate, which means
    /// that 8 and 16-bit addition cannot be supported.
    pub(crate) fn new(reg: Reg, size: OperandSize, asm: &mut Assembler) -> Self {
        let open = asm.buffer_mut().start_patchable();
        let start = asm.buffer().cur_offset();

        // Emit the opcode and register use for the add instruction.
        let reg = pair_gpr(Writable::from_reg(reg));
        let inst = match size {
            OperandSize::S32 => asm::inst::addl_mi::new(reg, 0_u32).into(),
            OperandSize::S64 => asm::inst::addq_mi_sxl::new(reg, 0_i32).into(),
            _ => {
                panic!(
                    "{}-bit addition is not supported, please see the comment on PatchableAddToReg::new",
                    size.num_bits(),
                )
            }
        };
        asm.emit(Inst::External { inst });

        // The offset to the constant is the width of what was just emitted
        // minus 4, the width of the 32-bit immediate.
        let constant_offset = usize::try_from(asm.buffer().cur_offset() - start - 4).unwrap();

        let region = asm.buffer_mut().end_patchable(open);

        Self {
            region,
            constant_offset,
        }
    }

    /// Patch the [`MachBuffer`] with the known constant to be added to the register. The final
    /// value is passed in as an i32, but the instruction encoding is fixed when
    /// [`PatchableAddToReg::new`] is called.
    pub(crate) fn finalize(self, val: i32, buffer: &mut MachBuffer<Inst>) {
        let slice = self.region.patch(buffer);
        debug_assert_eq!(slice.len(), self.constant_offset + 4);
        slice[self.constant_offset..].copy_from_slice(val.to_le_bytes().as_slice());
    }
}

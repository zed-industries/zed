//! This module defines Pulley-specific machine instruction types.

use core::marker::PhantomData;

use crate::binemit::{Addend, CodeOffset, Reloc};
use crate::ir::types::{self, F32, F64, I8, I8X16, I16, I32, I64, I128};
use crate::ir::{self, MemFlags, Type};
use crate::isa::FunctionAlignment;
use crate::isa::pulley_shared::abi::PulleyMachineDeps;
use crate::{CodegenError, CodegenResult, settings};
use crate::{machinst::*, trace};
use alloc::string::{String, ToString};
use regalloc2::RegClass;
use smallvec::SmallVec;

pub mod regs;
pub use self::regs::*;
pub mod args;
pub use self::args::*;
pub mod emit;
pub use self::emit::*;

//=============================================================================
// Instructions (top level): definition

pub use crate::isa::pulley_shared::lower::isle::generated_code::MInst as Inst;
pub use crate::isa::pulley_shared::lower::isle::generated_code::RawInst;

impl From<RawInst> for Inst {
    fn from(raw: RawInst) -> Inst {
        Inst::Raw { raw }
    }
}

use super::PulleyTargetKind;

mod generated {
    use super::*;
    use crate::isa::pulley_shared::lower::isle::generated_code::RawInst;

    include!(concat!(env!("OUT_DIR"), "/pulley_inst_gen.rs"));
}

/// Out-of-line data for return-calls, to keep the size of `Inst` down.
#[derive(Clone, Debug)]
pub struct ReturnCallInfo<T> {
    /// Where this call is going.
    pub dest: T,

    /// The size of the argument area for this return-call, potentially smaller
    /// than that of the caller, but never larger.
    pub new_stack_arg_size: u32,

    /// The in-register arguments and their constraints.
    pub uses: CallArgList,
}

impl Inst {
    /// Generic constructor for a load (zero-extending where appropriate).
    pub fn gen_load(dst: Writable<Reg>, mem: Amode, ty: Type, flags: MemFlags) -> Inst {
        if ty.is_vector() {
            assert_eq!(ty.bytes(), 16);
            Inst::VLoad {
                dst: dst.map(|r| VReg::new(r).unwrap()),
                mem,
                ty,
                flags,
            }
        } else if ty.is_int() {
            assert!(ty.bytes() <= 8);
            Inst::XLoad {
                dst: dst.map(|r| XReg::new(r).unwrap()),
                mem,
                ty,
                flags,
            }
        } else {
            Inst::FLoad {
                dst: dst.map(|r| FReg::new(r).unwrap()),
                mem,
                ty,
                flags,
            }
        }
    }

    /// Generic constructor for a store.
    pub fn gen_store(mem: Amode, from_reg: Reg, ty: Type, flags: MemFlags) -> Inst {
        if ty.is_vector() {
            assert_eq!(ty.bytes(), 16);
            Inst::VStore {
                mem,
                src: VReg::new(from_reg).unwrap(),
                ty,
                flags,
            }
        } else if ty.is_int() {
            assert!(ty.bytes() <= 8);
            Inst::XStore {
                mem,
                src: XReg::new(from_reg).unwrap(),
                ty,
                flags,
            }
        } else {
            Inst::FStore {
                mem,
                src: FReg::new(from_reg).unwrap(),
                ty,
                flags,
            }
        }
    }
}

fn pulley_get_operands(inst: &mut Inst, collector: &mut impl OperandVisitor) {
    match inst {
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

        Inst::DummyUse { reg } => {
            collector.reg_use(reg);
        }

        Inst::Nop => {}

        Inst::TrapIf { cond, code: _ } => {
            cond.get_operands(collector);
        }

        Inst::GetSpecial { dst, reg } => {
            collector.reg_def(dst);
            // Note that this is explicitly ignored as this is only used for
            // special registers that don't participate in register allocation
            // such as the stack pointer, frame pointer, etc.
            assert!(reg.is_special());
        }

        Inst::LoadExtName {
            dst,
            name: _,
            offset: _,
        } => {
            collector.reg_def(dst);
        }

        Inst::Call { info } => {
            let CallInfo {
                uses,
                defs,
                dest,
                try_call_info,
                clobbers,
                ..
            } = &mut **info;

            // Pulley supports having the first few integer arguments in any
            // register, so flag that with `reg_use` here.
            let PulleyCall { args, .. } = dest;
            for arg in args {
                collector.reg_use(arg);
            }

            // Remaining arguments (and return values) are all in fixed
            // registers according to Pulley's ABI, however.
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
        Inst::IndirectCallHost { info } => {
            let CallInfo {
                uses,
                defs,
                try_call_info,
                clobbers,
                ..
            } = &mut **info;
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
        Inst::IndirectCall { info } => {
            collector.reg_use(&mut info.dest);
            let CallInfo {
                uses,
                defs,
                try_call_info,
                clobbers,
                ..
            } = &mut **info;
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
        Inst::ReturnCall { info } => {
            for CallArgPair { vreg, preg } in &mut info.uses {
                collector.reg_fixed_use(vreg, *preg);
            }
        }
        Inst::ReturnIndirectCall { info } => {
            // Use a fixed location of where to store the value to
            // return-call-to. Using a fixed location prevents this register
            // from being allocated to a callee-saved register which will get
            // clobbered during the register restores just before the
            // return-call.
            //
            // Also note that `x15` is specifically the last caller-saved
            // register and, at this time, the only non-argument caller-saved
            // register. This register allocation constraint is why it's not an
            // argument register.
            collector.reg_fixed_use(&mut info.dest, regs::x15());

            for CallArgPair { vreg, preg } in &mut info.uses {
                collector.reg_fixed_use(vreg, *preg);
            }
        }

        Inst::Jump { .. } => {}

        Inst::BrIf {
            cond,
            taken: _,
            not_taken: _,
        } => {
            cond.get_operands(collector);
        }

        Inst::LoadAddr { dst, mem } => {
            collector.reg_def(dst);
            mem.get_operands(collector);
        }

        Inst::XLoad {
            dst,
            mem,
            ty: _,
            flags: _,
        } => {
            collector.reg_def(dst);
            mem.get_operands(collector);
        }

        Inst::XStore {
            mem,
            src,
            ty: _,
            flags: _,
        } => {
            mem.get_operands(collector);
            collector.reg_use(src);
        }

        Inst::FLoad {
            dst,
            mem,
            ty: _,
            flags: _,
        } => {
            collector.reg_def(dst);
            mem.get_operands(collector);
        }

        Inst::FStore {
            mem,
            src,
            ty: _,
            flags: _,
        } => {
            mem.get_operands(collector);
            collector.reg_use(src);
        }

        Inst::VLoad {
            dst,
            mem,
            ty: _,
            flags: _,
        } => {
            collector.reg_def(dst);
            mem.get_operands(collector);
        }

        Inst::VStore {
            mem,
            src,
            ty: _,
            flags: _,
        } => {
            mem.get_operands(collector);
            collector.reg_use(src);
        }

        Inst::BrTable { idx, .. } => {
            collector.reg_use(idx);
        }

        Inst::Raw { raw } => generated::get_operands(raw, collector),

        Inst::EmitIsland { .. } => {}
    }
}

/// A newtype over a Pulley instruction that also carries a phantom type
/// parameter describing whether we are targeting 32- or 64-bit Pulley bytecode.
///
/// Implements `Deref`, `DerefMut`, and `From`/`Into` for `Inst` to allow for
/// seamless conversion between `Inst` and `InstAndKind`.
#[derive(Clone, Debug)]
pub struct InstAndKind<P>
where
    P: PulleyTargetKind,
{
    inst: Inst,
    kind: PhantomData<P>,
}

impl<P> From<Inst> for InstAndKind<P>
where
    P: PulleyTargetKind,
{
    fn from(inst: Inst) -> Self {
        Self {
            inst,
            kind: PhantomData,
        }
    }
}

impl<P> From<RawInst> for InstAndKind<P>
where
    P: PulleyTargetKind,
{
    fn from(inst: RawInst) -> Self {
        Self {
            inst: inst.into(),
            kind: PhantomData,
        }
    }
}

impl<P> From<InstAndKind<P>> for Inst
where
    P: PulleyTargetKind,
{
    fn from(inst: InstAndKind<P>) -> Self {
        inst.inst
    }
}

impl<P> core::ops::Deref for InstAndKind<P>
where
    P: PulleyTargetKind,
{
    type Target = Inst;

    fn deref(&self) -> &Self::Target {
        &self.inst
    }
}

impl<P> core::ops::DerefMut for InstAndKind<P>
where
    P: PulleyTargetKind,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inst
    }
}

impl<P> MachInst for InstAndKind<P>
where
    P: PulleyTargetKind,
{
    type LabelUse = LabelUse;
    type ABIMachineSpec = PulleyMachineDeps<P>;

    const TRAP_OPCODE: &'static [u8] = TRAP_OPCODE;

    fn gen_dummy_use(reg: Reg) -> Self {
        Inst::DummyUse { reg }.into()
    }

    fn canonical_type_for_rc(rc: RegClass) -> Type {
        match rc {
            regalloc2::RegClass::Int => I64,
            regalloc2::RegClass::Float => F64,
            regalloc2::RegClass::Vector => I8X16,
        }
    }

    fn is_safepoint(&self) -> bool {
        match self.inst {
            Inst::Raw {
                raw: RawInst::Trap { .. },
            }
            | Inst::Call { .. }
            | Inst::IndirectCall { .. }
            | Inst::IndirectCallHost { .. } => true,
            _ => false,
        }
    }

    fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        pulley_get_operands(self, collector);
    }

    fn is_move(&self) -> Option<(Writable<Reg>, Reg)> {
        match self.inst {
            Inst::Raw {
                raw: RawInst::Xmov { dst, src },
            } => Some((Writable::from_reg(*dst.to_reg()), *src)),
            _ => None,
        }
    }

    fn is_included_in_clobbers(&self) -> bool {
        !self.is_args()
    }

    fn is_trap(&self) -> bool {
        match self.inst {
            Inst::Raw {
                raw: RawInst::Trap { .. },
            } => true,
            _ => false,
        }
    }

    fn is_args(&self) -> bool {
        match self.inst {
            Inst::Args { .. } => true,
            _ => false,
        }
    }

    fn is_term(&self) -> MachTerminator {
        match &self.inst {
            Inst::Raw {
                raw: RawInst::Ret { .. },
            }
            | Inst::Rets { .. } => MachTerminator::Ret,
            Inst::Jump { .. } => MachTerminator::Branch,
            Inst::BrIf { .. } => MachTerminator::Branch,
            Inst::BrTable { .. } => MachTerminator::Branch,
            Inst::ReturnCall { .. } | Inst::ReturnIndirectCall { .. } => MachTerminator::RetCall,
            Inst::Call { info } if info.try_call_info.is_some() => MachTerminator::Branch,
            Inst::IndirectCall { info } if info.try_call_info.is_some() => MachTerminator::Branch,
            Inst::IndirectCallHost { info } if info.try_call_info.is_some() => {
                MachTerminator::Branch
            }
            _ => MachTerminator::None,
        }
    }

    fn is_mem_access(&self) -> bool {
        todo!()
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: Type) -> Self {
        match ty {
            ir::types::I8 | ir::types::I16 | ir::types::I32 | ir::types::I64 => RawInst::Xmov {
                dst: WritableXReg::try_from(to_reg).unwrap(),
                src: XReg::new(from_reg).unwrap(),
            }
            .into(),
            ir::types::F32 | ir::types::F64 => RawInst::Fmov {
                dst: WritableFReg::try_from(to_reg).unwrap(),
                src: FReg::new(from_reg).unwrap(),
            }
            .into(),
            _ if ty.is_vector() => RawInst::Vmov {
                dst: WritableVReg::try_from(to_reg).unwrap(),
                src: VReg::new(from_reg).unwrap(),
            }
            .into(),
            _ => panic!("don't know how to generate a move for type {ty}"),
        }
    }

    fn gen_nop(_preferred_size: usize) -> Self {
        todo!()
    }

    fn rc_for_type(ty: Type) -> CodegenResult<(&'static [RegClass], &'static [Type])> {
        match ty {
            I8 => Ok((&[RegClass::Int], &[I8])),
            I16 => Ok((&[RegClass::Int], &[I16])),
            I32 => Ok((&[RegClass::Int], &[I32])),
            I64 => Ok((&[RegClass::Int], &[I64])),
            F32 => Ok((&[RegClass::Float], &[F32])),
            F64 => Ok((&[RegClass::Float], &[F64])),
            I128 => Ok((&[RegClass::Int, RegClass::Int], &[I64, I64])),
            _ if ty.is_vector() => {
                debug_assert!(ty.bits() <= 512);

                // Here we only need to return a SIMD type with the same size as `ty`.
                // We use these types for spills and reloads, so prefer types with lanes <= 31
                // since that fits in the immediate field of `vsetivli`.
                const SIMD_TYPES: [[Type; 1]; 6] = [
                    [types::I8X2],
                    [types::I8X4],
                    [types::I8X8],
                    [types::I8X16],
                    [types::I16X16],
                    [types::I32X16],
                ];
                let idx = (ty.bytes().ilog2() - 1) as usize;
                let ty = &SIMD_TYPES[idx][..];

                Ok((&[RegClass::Vector], ty))
            }
            _ => Err(CodegenError::Unsupported(format!(
                "Unexpected SSA-value type: {ty}"
            ))),
        }
    }

    fn gen_jump(label: MachLabel) -> Self {
        Inst::Jump { label }.into()
    }

    fn worst_case_size() -> CodeOffset {
        // `VShuffle { dst, src1, src2, imm }` is 22 bytes:
        // 3-byte opcode
        // dst, src1, src2
        // 16-byte immediate
        22
    }

    fn ref_type_regclass(_settings: &settings::Flags) -> RegClass {
        RegClass::Int
    }

    fn function_alignment() -> FunctionAlignment {
        FunctionAlignment {
            minimum: 1,
            preferred: 1,
        }
    }
}

const TRAP_OPCODE: &'static [u8] = &[
    pulley_interpreter::opcode::Opcode::ExtendedOp as u8,
    ((pulley_interpreter::opcode::ExtendedOpcode::Trap as u16) >> 0) as u8,
    ((pulley_interpreter::opcode::ExtendedOpcode::Trap as u16) >> 8) as u8,
];

#[test]
fn test_trap_encoding() {
    let mut dst = std::vec::Vec::new();
    pulley_interpreter::encode::trap(&mut dst);
    assert_eq!(dst, TRAP_OPCODE);
}

//=============================================================================
// Pretty-printing of instructions.

pub fn reg_name(reg: Reg) -> String {
    match reg.to_real_reg() {
        Some(real) => {
            let n = real.hw_enc();
            match (real.class(), n) {
                (RegClass::Int, 63) => format!("sp"),
                (RegClass::Int, 62) => format!("lr"),
                (RegClass::Int, 61) => format!("fp"),
                (RegClass::Int, 60) => format!("tmp0"),
                (RegClass::Int, 59) => format!("tmp1"),

                (RegClass::Int, _) => format!("x{n}"),
                (RegClass::Float, _) => format!("f{n}"),
                (RegClass::Vector, _) => format!("v{n}"),
            }
        }
        None => {
            format!("{reg:?}")
        }
    }
}

fn pretty_print_try_call(info: &TryCallInfo) -> String {
    format!(
        "; jump {:?}; catch [{}]",
        info.continuation,
        info.pretty_print_dests()
    )
}

impl Inst {
    fn print_with_state<P>(&self, _state: &mut EmitState<P>) -> String
    where
        P: PulleyTargetKind,
    {
        use core::fmt::Write;

        let format_reg = |reg: Reg| -> String { reg_name(reg) };

        match self {
            Inst::Args { args } => {
                let mut s = "args".to_string();
                for arg in args {
                    let preg = format_reg(arg.preg);
                    let def = format_reg(arg.vreg.to_reg());
                    write!(&mut s, " {def}={preg}").unwrap();
                }
                s
            }
            Inst::Rets { rets } => {
                let mut s = "rets".to_string();
                for ret in rets {
                    let preg = format_reg(ret.preg);
                    let vreg = format_reg(ret.vreg);
                    write!(&mut s, " {vreg}={preg}").unwrap();
                }
                s
            }

            Inst::DummyUse { reg } => {
                let reg = format_reg(*reg);
                format!("dummy_use {reg}")
            }

            Inst::TrapIf { cond, code } => {
                format!("trap_{cond} // code = {code:?}")
            }

            Inst::Nop => format!("nop"),

            Inst::GetSpecial { dst, reg } => {
                let dst = format_reg(*dst.to_reg());
                let reg = format_reg(**reg);
                format!("xmov {dst}, {reg}")
            }

            Inst::LoadExtName { dst, name, offset } => {
                let dst = format_reg(*dst.to_reg());
                format!("{dst} = load_ext_name {name:?}, {offset}")
            }

            Inst::Call { info } => {
                let try_call = info
                    .try_call_info
                    .as_ref()
                    .map(|tci| pretty_print_try_call(tci))
                    .unwrap_or_default();
                format!("call {info:?}{try_call}")
            }

            Inst::IndirectCall { info } => {
                let callee = format_reg(*info.dest);
                let try_call = info
                    .try_call_info
                    .as_ref()
                    .map(|tci| pretty_print_try_call(tci))
                    .unwrap_or_default();
                format!("indirect_call {callee}, {info:?}{try_call}")
            }

            Inst::ReturnCall { info } => {
                format!("return_call {info:?}")
            }

            Inst::ReturnIndirectCall { info } => {
                let callee = format_reg(*info.dest);
                format!("return_indirect_call {callee}, {info:?}")
            }

            Inst::IndirectCallHost { info } => {
                let try_call = info
                    .try_call_info
                    .as_ref()
                    .map(|tci| pretty_print_try_call(tci))
                    .unwrap_or_default();
                format!("indirect_call_host {info:?}{try_call}")
            }

            Inst::Jump { label } => format!("jump {}", label.to_string()),

            Inst::BrIf {
                cond,
                taken,
                not_taken,
            } => {
                let taken = taken.to_string();
                let not_taken = not_taken.to_string();
                format!("br_{cond}, {taken}; jump {not_taken}")
            }

            Inst::LoadAddr { dst, mem } => {
                let dst = format_reg(*dst.to_reg());
                let mem = mem.to_string();
                format!("{dst} = load_addr {mem}")
            }

            Inst::XLoad {
                dst,
                mem,
                ty,
                flags,
            } => {
                let dst = format_reg(*dst.to_reg());
                let ty = ty.bits();
                let mem = mem.to_string();
                format!("{dst} = xload{ty} {mem} // flags ={flags}")
            }

            Inst::XStore {
                mem,
                src,
                ty,
                flags,
            } => {
                let ty = ty.bits();
                let mem = mem.to_string();
                let src = format_reg(**src);
                format!("xstore{ty} {mem}, {src} // flags = {flags}")
            }

            Inst::FLoad {
                dst,
                mem,
                ty,
                flags,
            } => {
                let dst = format_reg(*dst.to_reg());
                let ty = ty.bits();
                let mem = mem.to_string();
                format!("{dst} = fload{ty} {mem} // flags ={flags}")
            }

            Inst::FStore {
                mem,
                src,
                ty,
                flags,
            } => {
                let ty = ty.bits();
                let mem = mem.to_string();
                let src = format_reg(**src);
                format!("fstore{ty} {mem}, {src} // flags = {flags}")
            }

            Inst::VLoad {
                dst,
                mem,
                ty,
                flags,
            } => {
                let dst = format_reg(*dst.to_reg());
                let ty = ty.bits();
                let mem = mem.to_string();
                format!("{dst} = vload{ty} {mem} // flags ={flags}")
            }

            Inst::VStore {
                mem,
                src,
                ty,
                flags,
            } => {
                let ty = ty.bits();
                let mem = mem.to_string();
                let src = format_reg(**src);
                format!("vstore{ty} {mem}, {src} // flags = {flags}")
            }

            Inst::BrTable {
                idx,
                default,
                targets,
            } => {
                let idx = format_reg(**idx);
                format!("br_table {idx} {default:?} {targets:?}")
            }
            Inst::Raw { raw } => generated::print(raw),

            Inst::EmitIsland { space_needed } => format!("emit_island {space_needed}"),
        }
    }
}

/// Different forms of label references for different instruction formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelUse {
    /// A PC-relative `jump`/`call`/etc... instruction with an `i32` relative
    /// target. The payload value is an addend that describes the positive
    /// offset from the start of the instruction to the offset being relocated.
    Jump(u32),
}

impl MachInstLabelUse for LabelUse {
    /// Alignment for veneer code. Pulley instructions don't require any
    /// particular alignment.
    const ALIGN: CodeOffset = 1;

    /// Maximum PC-relative range (positive), inclusive.
    fn max_pos_range(self) -> CodeOffset {
        match self {
            Self::Jump(_) => 0x7fff_ffff,
        }
    }

    /// Maximum PC-relative range (negative).
    fn max_neg_range(self) -> CodeOffset {
        match self {
            Self::Jump(_) => 0x8000_0000,
        }
    }

    /// Size of window into code needed to do the patch.
    fn patch_size(self) -> CodeOffset {
        match self {
            Self::Jump(_) => 4,
        }
    }

    /// Perform the patch.
    fn patch(self, buffer: &mut [u8], use_offset: CodeOffset, label_offset: CodeOffset) {
        let use_relative = (label_offset as i64) - (use_offset as i64);
        debug_assert!(use_relative <= self.max_pos_range() as i64);
        debug_assert!(use_relative >= -(self.max_neg_range() as i64));
        let pc_rel = i32::try_from(use_relative).unwrap() as u32;
        match self {
            Self::Jump(addend) => {
                let value = pc_rel.wrapping_add(addend);
                trace!(
                    "patching label use @ {use_offset:#x} to label {label_offset:#x} via \
                     PC-relative offset {pc_rel:#x}"
                );
                buffer.copy_from_slice(&value.to_le_bytes()[..]);
            }
        }
    }

    /// Is a veneer supported for this label reference type?
    fn supports_veneer(self) -> bool {
        match self {
            Self::Jump(_) => false,
        }
    }

    /// How large is the veneer, if supported?
    fn veneer_size(self) -> CodeOffset {
        match self {
            Self::Jump(_) => 0,
        }
    }

    fn worst_case_veneer_size() -> CodeOffset {
        0
    }

    /// Generate a veneer into the buffer, given that this veneer is at `veneer_offset`, and return
    /// an offset and label-use for the veneer's use of the original label.
    fn generate_veneer(
        self,
        _buffer: &mut [u8],
        _veneer_offset: CodeOffset,
    ) -> (CodeOffset, LabelUse) {
        match self {
            Self::Jump(_) => panic!("veneer not supported for {self:?}"),
        }
    }

    fn from_reloc(reloc: Reloc, addend: Addend) -> Option<LabelUse> {
        match reloc {
            Reloc::X86CallPCRel4 if addend < 0 => {
                // We are always relocating some offset that is within an
                // instruction, but pulley adds the offset relative to the PC
                // pointing to the *start* of the instruction. Therefore, adjust
                // back to the beginning of the instruction.
                Some(LabelUse::Jump(i32::try_from(-addend).unwrap() as u32))
            }
            _ => None,
        }
    }
}

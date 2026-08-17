//! X86_64-bit Instruction Set Architecture.

pub use self::inst::{AtomicRmwSeqOp, EmitInfo, EmitState, Inst, args, external};

use super::{OwnedTargetIsa, TargetIsa};
use crate::dominator_tree::DominatorTree;
use crate::ir::{self, Function, Type, types};
#[cfg(feature = "unwind")]
use crate::isa::unwind::systemv;
use crate::isa::x64::settings as x64_settings;
use crate::isa::{Builder as IsaBuilder, FunctionAlignment, IsaFlagsHashKey};
use crate::machinst::{
    CompiledCode, CompiledCodeStencil, MachInst, MachTextSectionBuilder, Reg, SigSet,
    TextSectionBuilder, VCode, compile,
};
use crate::result::CodegenResult;
use crate::settings::{self as shared_settings, Flags};
use crate::{Final, MachBufferFinalized};
use alloc::{boxed::Box, vec::Vec};
use core::fmt;
use cranelift_control::ControlPlane;
use std::string::String;
use target_lexicon::Triple;

mod abi;
mod inst;
mod lower;
mod pcc;
pub mod settings;

pub use inst::unwind::systemv::create_cie;

/// An X64 backend.
pub(crate) struct X64Backend {
    triple: Triple,
    flags: Flags,
    x64_flags: x64_settings::Flags,
}

impl X64Backend {
    /// Create a new X64 backend with the given (shared) flags.
    fn new_with_flags(triple: Triple, flags: Flags, x64_flags: x64_settings::Flags) -> Self {
        Self {
            triple,
            flags,
            x64_flags,
        }
    }

    fn compile_vcode(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<(VCode<inst::Inst>, regalloc2::Output)> {
        // This performs lowering to VCode, register-allocates the code, computes
        // block layout and finalizes branches. The result is ready for binary emission.
        let emit_info = EmitInfo::new(self.flags.clone(), self.x64_flags.clone());
        let sigs = SigSet::new::<abi::X64ABIMachineSpec>(func, &self.flags)?;
        let abi = abi::X64Callee::new(func, self, &self.x64_flags, &sigs)?;
        compile::compile::<Self>(func, domtree, self, abi, emit_info, sigs, ctrl_plane)
    }
}

impl TargetIsa for X64Backend {
    fn compile_function(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        want_disasm: bool,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<CompiledCodeStencil> {
        let (vcode, regalloc_result) = self.compile_vcode(func, domtree, ctrl_plane)?;

        let emit_result = vcode.emit(&regalloc_result, want_disasm, &self.flags, ctrl_plane);
        let frame_size = emit_result.frame_size;
        let value_labels_ranges = emit_result.value_labels_ranges;
        let buffer = emit_result.buffer;
        let sized_stackslot_offsets = emit_result.sized_stackslot_offsets;
        let dynamic_stackslot_offsets = emit_result.dynamic_stackslot_offsets;

        if let Some(disasm) = emit_result.disasm.as_ref() {
            crate::trace!("disassembly:\n{}", disasm);
        }

        Ok(CompiledCodeStencil {
            buffer,
            frame_size,
            vcode: emit_result.disasm,
            value_labels_ranges,
            sized_stackslot_offsets,
            dynamic_stackslot_offsets,
            bb_starts: emit_result.bb_offsets,
            bb_edges: emit_result.bb_edges,
        })
    }

    fn flags(&self) -> &Flags {
        &self.flags
    }

    fn isa_flags(&self) -> Vec<shared_settings::Value> {
        self.x64_flags.iter().collect()
    }

    fn isa_flags_hash_key(&self) -> IsaFlagsHashKey<'_> {
        IsaFlagsHashKey(self.x64_flags.hash_key())
    }

    fn dynamic_vector_bytes(&self, _dyn_ty: Type) -> u32 {
        16
    }

    fn name(&self) -> &'static str {
        "x64"
    }

    fn triple(&self) -> &Triple {
        &self.triple
    }

    #[cfg(feature = "unwind")]
    fn emit_unwind_info(
        &self,
        result: &CompiledCode,
        kind: crate::isa::unwind::UnwindInfoKind,
    ) -> CodegenResult<Option<crate::isa::unwind::UnwindInfo>> {
        emit_unwind_info(&result.buffer, kind)
    }

    #[cfg(feature = "unwind")]
    fn create_systemv_cie(&self) -> Option<gimli::write::CommonInformationEntry> {
        Some(inst::unwind::systemv::create_cie())
    }

    #[cfg(feature = "unwind")]
    fn map_regalloc_reg_to_dwarf(&self, reg: Reg) -> Result<u16, systemv::RegisterMappingError> {
        inst::unwind::systemv::map_reg(reg).map(|reg| reg.0)
    }

    fn text_section_builder(&self, num_funcs: usize) -> Box<dyn TextSectionBuilder> {
        Box::new(MachTextSectionBuilder::<inst::Inst>::new(num_funcs))
    }

    fn function_alignment(&self) -> FunctionAlignment {
        Inst::function_alignment()
    }

    fn page_size_align_log2(&self) -> u8 {
        debug_assert_eq!(1 << 12, 0x1000);
        12
    }

    #[cfg(feature = "disas")]
    fn to_capstone(&self) -> Result<capstone::Capstone, capstone::Error> {
        use capstone::prelude::*;
        Capstone::new()
            .x86()
            .mode(arch::x86::ArchMode::Mode64)
            .syntax(arch::x86::ArchSyntax::Att)
            .detail(true)
            .build()
    }

    fn pretty_print_reg(&self, reg: Reg, size: u8) -> String {
        inst::regs::pretty_print_reg(reg, size)
    }

    fn has_native_fma(&self) -> bool {
        self.x64_flags.use_fma()
    }

    fn has_round(&self) -> bool {
        self.x64_flags.use_sse41()
    }

    fn has_x86_blendv_lowering(&self, ty: Type) -> bool {
        // The `blendvpd`, `blendvps`, and `pblendvb` instructions are all only
        // available from SSE 4.1 and onwards. Otherwise the i16x8 type has no
        // equivalent instruction which only looks at the top bit for a select
        // operation, so that always returns `false`
        self.x64_flags.use_sse41() && ty != types::I16X8
    }

    fn has_x86_pshufb_lowering(&self) -> bool {
        self.x64_flags.use_ssse3()
    }

    fn has_x86_pmulhrsw_lowering(&self) -> bool {
        self.x64_flags.use_ssse3()
    }

    fn has_x86_pmaddubsw_lowering(&self) -> bool {
        self.x64_flags.use_ssse3()
    }

    fn default_argument_extension(&self) -> ir::ArgumentExtension {
        // This is copied/carried over from a historical piece of code in
        // Wasmtime:
        //
        // https://github.com/bytecodealliance/wasmtime/blob/a018a5a9addb77d5998021a0150192aa955c71bf/crates/cranelift/src/lib.rs#L366-L374
        //
        // Whether or not it is still applicable here is unsure, but it's left
        // the same as-is for now to reduce the likelihood of problems arising.
        ir::ArgumentExtension::Uext
    }
}

/// Emit unwind info for an x86 target.
pub fn emit_unwind_info(
    buffer: &MachBufferFinalized<Final>,
    kind: crate::isa::unwind::UnwindInfoKind,
) -> CodegenResult<Option<crate::isa::unwind::UnwindInfo>> {
    use crate::isa::unwind::{UnwindInfo, UnwindInfoKind};
    Ok(match kind {
        UnwindInfoKind::SystemV => {
            let mapper = self::inst::unwind::systemv::RegisterMapper;
            Some(UnwindInfo::SystemV(
                crate::isa::unwind::systemv::create_unwind_info_from_insts(
                    &buffer.unwind_info[..],
                    buffer.data().len(),
                    &mapper,
                )?,
            ))
        }
        UnwindInfoKind::Windows => Some(UnwindInfo::WindowsX64(
            crate::isa::unwind::winx64::create_unwind_info_from_insts::<
                self::inst::unwind::winx64::RegisterMapper,
            >(&buffer.unwind_info[..])?,
        )),
        _ => None,
    })
}

impl fmt::Display for X64Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MachBackend")
            .field("name", &self.name())
            .field("triple", &self.triple())
            .field("flags", &format!("{}", self.flags()))
            .finish()
    }
}

/// Create a new `isa::Builder`.
pub(crate) fn isa_builder(triple: Triple) -> IsaBuilder {
    IsaBuilder {
        triple,
        setup: x64_settings::builder(),
        constructor: isa_constructor,
    }
}

fn isa_constructor(
    triple: Triple,
    shared_flags: Flags,
    builder: &shared_settings::Builder,
) -> CodegenResult<OwnedTargetIsa> {
    let isa_flags = x64_settings::Flags::new(&shared_flags, builder);
    let backend = X64Backend::new_with_flags(triple, shared_flags, isa_flags);
    Ok(backend.wrapped())
}

//! ARM 64-bit Instruction Set Architecture.

use crate::dominator_tree::DominatorTree;
use crate::ir::{self, Function, Type};
use crate::isa::aarch64::settings as aarch64_settings;
#[cfg(feature = "unwind")]
use crate::isa::unwind::systemv;
use crate::isa::{Builder as IsaBuilder, FunctionAlignment, IsaFlagsHashKey, TargetIsa};
use crate::machinst::{
    CompiledCode, CompiledCodeStencil, MachInst, MachTextSectionBuilder, Reg, SigSet,
    TextSectionBuilder, VCode, compile,
};
use crate::result::CodegenResult;
use crate::settings as shared_settings;
use alloc::{boxed::Box, vec::Vec};
use core::fmt;
use cranelift_control::ControlPlane;
use std::string::String;
use target_lexicon::{Aarch64Architecture, Architecture, OperatingSystem, Triple};

// New backend:
mod abi;
pub mod inst;
mod lower;
mod pcc;
pub mod settings;

use self::inst::EmitInfo;

/// An AArch64 backend.
pub struct AArch64Backend {
    triple: Triple,
    flags: shared_settings::Flags,
    isa_flags: aarch64_settings::Flags,
}

impl AArch64Backend {
    /// Create a new AArch64 backend with the given (shared) flags.
    pub fn new_with_flags(
        triple: Triple,
        flags: shared_settings::Flags,
        isa_flags: aarch64_settings::Flags,
    ) -> AArch64Backend {
        AArch64Backend {
            triple,
            flags,
            isa_flags,
        }
    }

    /// This performs lowering to VCode, register-allocates the code, computes block layout and
    /// finalizes branches. The result is ready for binary emission.
    fn compile_vcode(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<(VCode<inst::Inst>, regalloc2::Output)> {
        let emit_info = EmitInfo::new(self.flags.clone());
        let sigs = SigSet::new::<abi::AArch64MachineDeps>(func, &self.flags)?;
        let abi = abi::AArch64Callee::new(func, self, &self.isa_flags, &sigs)?;
        compile::compile::<AArch64Backend>(func, domtree, self, abi, emit_info, sigs, ctrl_plane)
    }
}

impl TargetIsa for AArch64Backend {
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
            log::debug!("disassembly:\n{disasm}");
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

    fn name(&self) -> &'static str {
        "aarch64"
    }

    fn triple(&self) -> &Triple {
        &self.triple
    }

    fn flags(&self) -> &shared_settings::Flags {
        &self.flags
    }

    fn isa_flags(&self) -> Vec<shared_settings::Value> {
        self.isa_flags.iter().collect()
    }

    fn isa_flags_hash_key(&self) -> IsaFlagsHashKey<'_> {
        IsaFlagsHashKey(self.isa_flags.hash_key())
    }

    fn is_branch_protection_enabled(&self) -> bool {
        self.isa_flags.use_bti()
    }

    fn dynamic_vector_bytes(&self, _dyn_ty: Type) -> u32 {
        16
    }

    #[cfg(feature = "unwind")]
    fn emit_unwind_info(
        &self,
        result: &CompiledCode,
        kind: crate::isa::unwind::UnwindInfoKind,
    ) -> CodegenResult<Option<crate::isa::unwind::UnwindInfo>> {
        use crate::isa::unwind::UnwindInfo;
        use crate::isa::unwind::UnwindInfoKind;
        Ok(match kind {
            UnwindInfoKind::SystemV => {
                let mapper = self::inst::unwind::systemv::RegisterMapper;
                Some(UnwindInfo::SystemV(
                    crate::isa::unwind::systemv::create_unwind_info_from_insts(
                        &result.buffer.unwind_info[..],
                        result.buffer.data().len(),
                        &mapper,
                    )?,
                ))
            }
            UnwindInfoKind::Windows => Some(UnwindInfo::WindowsArm64(
                crate::isa::unwind::winarm64::create_unwind_info_from_insts(
                    &result.buffer.unwind_info[..],
                )?,
            )),
            _ => None,
        })
    }

    #[cfg(feature = "unwind")]
    fn create_systemv_cie(&self) -> Option<gimli::write::CommonInformationEntry> {
        let is_apple_os = match self.triple.operating_system {
            OperatingSystem::Darwin(_)
            | OperatingSystem::IOS(_)
            | OperatingSystem::MacOSX { .. }
            | OperatingSystem::TvOS(_) => true,
            _ => false,
        };

        if self.isa_flags.sign_return_address()
            && self.isa_flags.sign_return_address_with_bkey()
            && !is_apple_os
        {
            unimplemented!(
                "Specifying that the B key is used with pointer authentication instructions in the CIE is not implemented."
            );
        }

        Some(inst::unwind::systemv::create_cie())
    }

    fn text_section_builder(&self, num_funcs: usize) -> Box<dyn TextSectionBuilder> {
        Box::new(MachTextSectionBuilder::<inst::Inst>::new(num_funcs))
    }

    #[cfg(feature = "unwind")]
    fn map_regalloc_reg_to_dwarf(&self, reg: Reg) -> Result<u16, systemv::RegisterMappingError> {
        inst::unwind::systemv::map_reg(reg).map(|reg| reg.0)
    }

    fn function_alignment(&self) -> FunctionAlignment {
        inst::Inst::function_alignment()
    }

    fn page_size_align_log2(&self) -> u8 {
        use target_lexicon::*;
        match self.triple().operating_system {
            OperatingSystem::MacOSX { .. }
            | OperatingSystem::Darwin(_)
            | OperatingSystem::IOS(_)
            | OperatingSystem::TvOS(_) => {
                debug_assert_eq!(1 << 14, 0x4000);
                14
            }
            _ => {
                debug_assert_eq!(1 << 16, 0x10000);
                16
            }
        }
    }

    #[cfg(feature = "disas")]
    fn to_capstone(&self) -> Result<capstone::Capstone, capstone::Error> {
        use capstone::prelude::*;
        let mut cs = Capstone::new()
            .arm64()
            .mode(arch::arm64::ArchMode::Arm)
            .detail(true)
            .build()?;
        // AArch64 uses inline constants rather than a separate constant pool right now.
        // Without this option, Capstone will stop disassembling as soon as it sees
        // an inline constant that is not also a valid instruction. With this option,
        // Capstone will print a `.byte` directive with the bytes of the inline constant
        // and continue to the next instruction.
        cs.set_skipdata(true)?;
        Ok(cs)
    }

    fn pretty_print_reg(&self, reg: Reg, _size: u8) -> String {
        inst::regs::pretty_print_reg(reg)
    }

    fn has_native_fma(&self) -> bool {
        true
    }

    fn has_round(&self) -> bool {
        true
    }

    fn has_x86_blendv_lowering(&self, _: Type) -> bool {
        false
    }

    fn has_x86_pshufb_lowering(&self) -> bool {
        false
    }

    fn has_x86_pmulhrsw_lowering(&self) -> bool {
        false
    }

    fn has_x86_pmaddubsw_lowering(&self) -> bool {
        false
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

impl fmt::Display for AArch64Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MachBackend")
            .field("name", &self.name())
            .field("triple", &self.triple())
            .field("flags", &format!("{}", self.flags()))
            .finish()
    }
}

/// Create a new `isa::Builder`.
pub fn isa_builder(triple: Triple) -> IsaBuilder {
    assert!(triple.architecture == Architecture::Aarch64(Aarch64Architecture::Aarch64));
    IsaBuilder {
        triple,
        setup: aarch64_settings::builder(),
        constructor: |triple, shared_flags, builder| {
            let isa_flags = aarch64_settings::Flags::new(&shared_flags, builder);
            let backend = AArch64Backend::new_with_flags(triple, shared_flags, isa_flags);
            Ok(backend.wrapped())
        },
    }
}

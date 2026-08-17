//! risc-v 64-bit Instruction Set Architecture.

use crate::dominator_tree::DominatorTree;
use crate::ir::{Function, Type};
use crate::isa::riscv64::settings as riscv_settings;
use crate::isa::{
    Builder as IsaBuilder, FunctionAlignment, IsaFlagsHashKey, OwnedTargetIsa, TargetIsa,
};
use crate::machinst::{
    CompiledCode, CompiledCodeStencil, MachInst, MachTextSectionBuilder, Reg, SigSet,
    TextSectionBuilder, VCode, compile,
};
use crate::result::CodegenResult;
use crate::settings::{self as shared_settings, Flags};
use crate::{CodegenError, ir};
use alloc::{boxed::Box, vec::Vec};
use core::fmt;
use cranelift_control::ControlPlane;
use std::string::String;
use target_lexicon::{Architecture, Triple};
mod abi;
pub(crate) mod inst;
mod lower;
mod settings;
#[cfg(feature = "unwind")]
use crate::isa::unwind::systemv;

use self::inst::EmitInfo;

/// An riscv64 backend.
pub struct Riscv64Backend {
    triple: Triple,
    flags: shared_settings::Flags,
    isa_flags: riscv_settings::Flags,
}

impl Riscv64Backend {
    /// Create a new riscv64 backend with the given (shared) flags.
    pub fn new_with_flags(
        triple: Triple,
        flags: shared_settings::Flags,
        isa_flags: riscv_settings::Flags,
    ) -> Riscv64Backend {
        Riscv64Backend {
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
        let emit_info = EmitInfo::new(self.flags.clone(), self.isa_flags.clone());
        let sigs = SigSet::new::<abi::Riscv64MachineDeps>(func, &self.flags)?;
        let abi = abi::Riscv64Callee::new(func, self, &self.isa_flags, &sigs)?;
        compile::compile::<Riscv64Backend>(func, domtree, self, abi, emit_info, sigs, ctrl_plane)
    }
}

impl TargetIsa for Riscv64Backend {
    fn compile_function(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        want_disasm: bool,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<CompiledCodeStencil> {
        let (vcode, regalloc_result) = self.compile_vcode(func, domtree, ctrl_plane)?;

        let want_disasm = want_disasm || log::log_enabled!(log::Level::Debug);
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
        "riscv64"
    }
    fn dynamic_vector_bytes(&self, _dynamic_ty: ir::Type) -> u32 {
        16
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
            UnwindInfoKind::Windows => None,
            _ => None,
        })
    }

    #[cfg(feature = "unwind")]
    fn create_systemv_cie(&self) -> Option<gimli::write::CommonInformationEntry> {
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
        debug_assert_eq!(1 << 12, 0x1000);
        12
    }

    #[cfg(feature = "disas")]
    fn to_capstone(&self) -> Result<capstone::Capstone, capstone::Error> {
        use capstone::prelude::*;
        let mut cs_builder = Capstone::new().riscv().mode(arch::riscv::ArchMode::RiscV64);

        // Enable C instruction decoding if we have compressed instructions enabled.
        //
        // We can't enable this unconditionally because it will cause Capstone to
        // emit weird instructions and generally mess up when it encounters unknown
        // instructions, such as any Zba,Zbb,Zbc or Vector instructions.
        //
        // This causes the default disassembly to be quite unreadable, so enable
        // it only when we are actually going to be using them.
        let uses_compressed = self
            .isa_flags()
            .iter()
            .filter(|f| ["has_zca", "has_zcb", "has_zcd"].contains(&f.name))
            .any(|f| f.as_bool().unwrap_or(false));
        if uses_compressed {
            cs_builder = cs_builder.extra_mode([arch::riscv::ArchExtraMode::RiscVC].into_iter());
        }

        let mut cs = cs_builder.build()?;

        // Similar to AArch64, RISC-V uses inline constants rather than a separate
        // constant pool. We want to skip disassembly over inline constants instead
        // of stopping on invalid bytes.
        cs.set_skipdata(true)?;
        Ok(cs)
    }

    fn pretty_print_reg(&self, reg: Reg, _size: u8) -> String {
        // TODO-RISC-V: implement proper register pretty-printing.
        format!("{reg:?}")
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
        // According to https://riscv.org/wp-content/uploads/2024/12/riscv-calling.pdf
        // it says:
        //
        // > In RV64, 32-bit types, such as int, are stored in integer
        // > registers as proper sign extensions of their 32-bit values; that
        // > is, bits 63..31 are all equal. This restriction holds even for
        // > unsigned 32-bit types.
        //
        // leading to `sext` here.
        ir::ArgumentExtension::Sext
    }
}

impl fmt::Display for Riscv64Backend {
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
    match triple.architecture {
        Architecture::Riscv64(..) => {}
        _ => unreachable!(),
    }
    IsaBuilder {
        triple,
        setup: riscv_settings::builder(),
        constructor: isa_constructor,
    }
}

fn isa_constructor(
    triple: Triple,
    shared_flags: Flags,
    builder: &shared_settings::Builder,
) -> CodegenResult<OwnedTargetIsa> {
    let isa_flags = riscv_settings::Flags::new(&shared_flags, builder);

    // The RISC-V backend does not work without at least the G extension enabled.
    // The G extension is simply a combination of the following extensions:
    // - I: Base Integer Instruction Set
    // - M: Integer Multiplication and Division
    // - A: Atomic Instructions
    // - F: Single-Precision Floating-Point
    // - D: Double-Precision Floating-Point
    // - Zicsr: Control and Status Register Instructions
    // - Zifencei: Instruction-Fetch Fence
    //
    // Ensure that those combination of features is enabled.
    if !isa_flags.has_g() {
        return Err(CodegenError::Unsupported(
            "The RISC-V Backend currently requires all the features in the G Extension enabled"
                .into(),
        ));
    }

    let backend = Riscv64Backend::new_with_flags(triple, shared_flags, isa_flags);
    Ok(backend.wrapped())
}

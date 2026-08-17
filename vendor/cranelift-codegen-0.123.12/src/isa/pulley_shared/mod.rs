//! Common support compiling to either 32- or 64-bit Pulley bytecode.

mod abi;
mod inst;
mod lower;
mod settings;

use self::inst::EmitInfo;
use super::{Builder as IsaBuilder, FunctionAlignment};
use crate::{
    MachTextSectionBuilder, TextSectionBuilder,
    dominator_tree::DominatorTree,
    ir,
    isa::{self, IsaFlagsHashKey, OwnedTargetIsa, TargetIsa},
    machinst::{self, CompiledCodeStencil, MachInst, SigSet, VCode},
    result::CodegenResult,
    settings::{self as shared_settings, Flags},
};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::Debug;
use core::marker::PhantomData;
use cranelift_control::ControlPlane;
use std::string::String;
use target_lexicon::{Architecture, Triple};

pub use settings::Flags as PulleyFlags;

/// A trait to abstract over the different kinds of Pulley targets that exist
/// (32- vs 64-bit).
pub trait PulleyTargetKind: 'static + Clone + Debug + Default + Send + Sync {
    // Required types and methods.

    fn pointer_width() -> PointerWidth;

    // Provided methods. Don't overwrite.

    fn name() -> &'static str {
        match Self::pointer_width() {
            PointerWidth::PointerWidth32 => "pulley32",
            PointerWidth::PointerWidth64 => "pulley64",
        }
    }
}

pub enum PointerWidth {
    PointerWidth32,
    PointerWidth64,
}

impl PointerWidth {
    pub fn bits(self) -> u8 {
        match self {
            PointerWidth::PointerWidth32 => 32,
            PointerWidth::PointerWidth64 => 64,
        }
    }

    pub fn bytes(self) -> u8 {
        self.bits() / 8
    }
}

/// A Pulley backend.
pub struct PulleyBackend<P>
where
    P: PulleyTargetKind,
{
    pulley_target: PhantomData<P>,
    triple: Triple,
    flags: Flags,
    isa_flags: PulleyFlags,
}

impl<P> core::fmt::Debug for PulleyBackend<P>
where
    P: PulleyTargetKind,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let PulleyBackend {
            pulley_target: _,
            triple,
            flags: _,
            isa_flags: _,
        } = self;
        f.debug_struct("PulleyBackend")
            .field("triple", triple)
            .finish_non_exhaustive()
    }
}

impl<P> core::fmt::Display for PulleyBackend<P>
where
    P: PulleyTargetKind,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

impl<P> PulleyBackend<P>
where
    P: PulleyTargetKind,
{
    /// Create a new pulley backend with the given (shared) flags.
    pub fn new_with_flags(
        triple: Triple,
        flags: shared_settings::Flags,
        isa_flags: PulleyFlags,
    ) -> Self {
        PulleyBackend {
            pulley_target: PhantomData,
            triple,
            flags,
            isa_flags,
        }
    }

    /// This performs lowering to VCode, register-allocates the code, computes block layout and
    /// finalizes branches. The result is ready for binary emission.
    fn compile_vcode(
        &self,
        func: &ir::Function,
        domtree: &DominatorTree,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<(VCode<inst::InstAndKind<P>>, regalloc2::Output)> {
        let emit_info = EmitInfo::new(
            func.signature.call_conv,
            self.flags.clone(),
            self.isa_flags.clone(),
        );
        let sigs = SigSet::new::<abi::PulleyMachineDeps<P>>(func, &self.flags)?;
        let abi = abi::PulleyCallee::new(func, self, &self.isa_flags, &sigs)?;
        machinst::compile::<Self>(func, domtree, self, abi, emit_info, sigs, ctrl_plane)
    }
}

impl<P> TargetIsa for PulleyBackend<P>
where
    P: PulleyTargetKind,
{
    fn name(&self) -> &'static str {
        P::name()
    }

    fn triple(&self) -> &Triple {
        &self.triple
    }

    fn flags(&self) -> &Flags {
        &self.flags
    }

    fn isa_flags(&self) -> Vec<shared_settings::Value> {
        self.isa_flags.iter().collect()
    }

    fn isa_flags_hash_key(&self) -> IsaFlagsHashKey<'_> {
        IsaFlagsHashKey(self.isa_flags.hash_key())
    }

    fn dynamic_vector_bytes(&self, _dynamic_ty: ir::Type) -> u32 {
        512
    }

    fn page_size_align_log2(&self) -> u8 {
        // Claim 64KiB pages to be conservative.
        16
    }

    fn compile_function(
        &self,
        func: &ir::Function,
        domtree: &DominatorTree,
        want_disasm: bool,
        ctrl_plane: &mut cranelift_control::ControlPlane,
    ) -> CodegenResult<CompiledCodeStencil> {
        let (vcode, regalloc_result) = self.compile_vcode(func, domtree, ctrl_plane)?;

        let want_disasm =
            want_disasm || (cfg!(feature = "trace-log") && log::log_enabled!(log::Level::Debug));
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

    fn emit_unwind_info(
        &self,
        _result: &crate::CompiledCode,
        _kind: super::unwind::UnwindInfoKind,
    ) -> CodegenResult<Option<isa::unwind::UnwindInfo>> {
        // TODO: actually support unwind info?
        Ok(None)
    }

    fn text_section_builder(
        &self,
        num_labeled_funcs: usize,
    ) -> alloc::boxed::Box<dyn TextSectionBuilder> {
        Box::new(MachTextSectionBuilder::<inst::InstAndKind<P>>::new(
            num_labeled_funcs,
        ))
    }

    fn function_alignment(&self) -> FunctionAlignment {
        inst::InstAndKind::<P>::function_alignment()
    }

    fn pretty_print_reg(&self, reg: crate::Reg, _size: u8) -> String {
        format!("{reg:?}")
    }

    fn has_native_fma(&self) -> bool {
        // The pulley interpreter does have fma opcodes.
        true
    }

    fn has_round(&self) -> bool {
        // The pulley interpreter does have rounding opcodes.
        true
    }

    fn has_x86_blendv_lowering(&self, _ty: ir::Type) -> bool {
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
        ir::ArgumentExtension::None
    }
}

/// Create a new Pulley ISA builder.
pub fn isa_builder(triple: Triple) -> IsaBuilder {
    let constructor = match triple.architecture {
        Architecture::Pulley32 | Architecture::Pulley32be => isa_constructor_32,
        Architecture::Pulley64 | Architecture::Pulley64be => isa_constructor_64,
        other => panic!("unexpected architecture {other:?}"),
    };
    IsaBuilder {
        triple,
        setup: self::settings::builder(),
        constructor,
    }
}

fn isa_constructor_32(
    triple: Triple,
    shared_flags: Flags,
    builder: &shared_settings::Builder,
) -> CodegenResult<OwnedTargetIsa> {
    use crate::settings::Configurable;
    let mut builder = builder.clone();
    builder.set("pointer_width", "pointer32").unwrap();
    if triple.endianness().unwrap() == target_lexicon::Endianness::Big {
        builder.enable("big_endian").unwrap();
    }
    let isa_flags = PulleyFlags::new(&shared_flags, &builder);

    let backend =
        PulleyBackend::<super::pulley32::Pulley32>::new_with_flags(triple, shared_flags, isa_flags);
    Ok(backend.wrapped())
}

fn isa_constructor_64(
    triple: Triple,
    shared_flags: Flags,
    builder: &shared_settings::Builder,
) -> CodegenResult<OwnedTargetIsa> {
    use crate::settings::Configurable;
    let mut builder = builder.clone();
    builder.set("pointer_width", "pointer64").unwrap();
    if triple.endianness().unwrap() == target_lexicon::Endianness::Big {
        builder.enable("big_endian").unwrap();
    }
    let isa_flags = PulleyFlags::new(&shared_flags, &builder);

    let backend =
        PulleyBackend::<super::pulley64::Pulley64>::new_with_flags(triple, shared_flags, isa_flags);
    Ok(backend.wrapped())
}

impl PulleyFlags {
    fn endianness(&self) -> ir::Endianness {
        if self.big_endian() {
            ir::Endianness::Big
        } else {
            ir::Endianness::Little
        }
    }
}

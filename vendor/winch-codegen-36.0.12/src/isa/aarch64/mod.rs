use self::regs::{fpr_bit_set, gpr_bit_set};
use crate::{
    BuiltinFunctions,
    abi::{ABI, wasm_sig},
    codegen::{CodeGen, CodeGenContext, FuncEnv, TypeConverter},
    frame::{DefinedLocals, Frame},
    isa::{Builder, TargetIsa},
    masm::MacroAssembler,
    regalloc::RegAlloc,
    stack::Stack,
};
use anyhow::Result;
use cranelift_codegen::settings::{self, Flags};
use cranelift_codegen::{Final, MachBufferFinalized, isa::aarch64::settings as aarch64_settings};
use cranelift_codegen::{MachTextSectionBuilder, TextSectionBuilder};
use masm::MacroAssembler as Aarch64Masm;
use target_lexicon::Triple;
use wasmparser::{FuncValidator, FunctionBody, ValidatorResources};
use wasmtime_cranelift::CompiledFunction;
use wasmtime_environ::{ModuleTranslation, ModuleTypesBuilder, Tunables, VMOffsets, WasmFuncType};

mod abi;
mod address;
mod asm;
mod masm;
mod regs;

/// Create an ISA from the given triple.
pub(crate) fn isa_builder(triple: Triple) -> Builder {
    Builder::new(
        triple,
        aarch64_settings::builder(),
        |triple, shared_flags, settings| {
            let isa_flags = aarch64_settings::Flags::new(&shared_flags, settings);
            let isa = Aarch64::new(triple, shared_flags, isa_flags);
            Ok(Box::new(isa))
        },
    )
}

/// Aarch64 ISA.
pub(crate) struct Aarch64 {
    /// The target triple.
    triple: Triple,
    /// ISA specific flags.
    isa_flags: aarch64_settings::Flags,
    /// Shared flags.
    shared_flags: Flags,
}

impl Aarch64 {
    /// Create an Aarch64 ISA.
    pub fn new(triple: Triple, shared_flags: Flags, isa_flags: aarch64_settings::Flags) -> Self {
        Self {
            isa_flags,
            shared_flags,
            triple,
        }
    }
}

impl TargetIsa for Aarch64 {
    fn name(&self) -> &'static str {
        "aarch64"
    }

    fn triple(&self) -> &Triple {
        &self.triple
    }

    fn flags(&self) -> &settings::Flags {
        &self.shared_flags
    }

    fn isa_flags(&self) -> Vec<settings::Value> {
        self.isa_flags.iter().collect()
    }

    fn is_branch_protection_enabled(&self) -> bool {
        self.isa_flags.use_bti()
    }

    fn compile_function(
        &self,
        sig: &WasmFuncType,
        body: &FunctionBody,
        translation: &ModuleTranslation,
        types: &ModuleTypesBuilder,
        builtins: &mut BuiltinFunctions,
        validator: &mut FuncValidator<ValidatorResources>,
        tunables: &Tunables,
    ) -> Result<CompiledFunction> {
        let pointer_bytes = self.pointer_bytes();
        let vmoffsets = VMOffsets::new(pointer_bytes, &translation.module);
        let mut body = body.get_binary_reader();
        let mut masm = Aarch64Masm::new(pointer_bytes, self.shared_flags.clone())?;
        let stack = Stack::new();
        let abi_sig = wasm_sig::<abi::Aarch64ABI>(sig)?;

        let env = FuncEnv::new(
            &vmoffsets,
            translation,
            types,
            builtins,
            self,
            abi::Aarch64ABI::ptr_type(),
        );
        let type_converter = TypeConverter::new(env.translation, env.types);
        let defined_locals =
            DefinedLocals::new::<abi::Aarch64ABI>(&type_converter, &mut body, validator)?;
        let frame = Frame::new::<abi::Aarch64ABI>(&abi_sig, &defined_locals)?;
        let regalloc = RegAlloc::from(gpr_bit_set(), fpr_bit_set());
        let codegen_context = CodeGenContext::new(regalloc, stack, frame, &vmoffsets);
        let codegen = CodeGen::new(tunables, &mut masm, codegen_context, env, abi_sig);

        let mut body_codegen = codegen.emit_prologue()?;
        body_codegen.emit(body, validator)?;
        let names = body_codegen.env.take_name_map();
        let base = body_codegen.source_location.base;
        Ok(CompiledFunction::new(
            masm.finalize(base)?,
            names,
            self.function_alignment(),
        ))
    }

    fn text_section_builder(&self, num_funcs: usize) -> Box<dyn TextSectionBuilder> {
        Box::new(MachTextSectionBuilder::<
            cranelift_codegen::isa::aarch64::inst::Inst,
        >::new(num_funcs))
    }

    fn function_alignment(&self) -> u32 {
        // See `cranelift_codegen::isa::TargetIsa::function_alignment`.
        32
    }

    fn emit_unwind_info(
        &self,
        _result: &MachBufferFinalized<Final>,
        _kind: cranelift_codegen::isa::unwind::UnwindInfoKind,
    ) -> Result<Option<cranelift_codegen::isa::unwind::UnwindInfo>> {
        // TODO: should fill this in with an actual implementation
        Ok(None)
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
}

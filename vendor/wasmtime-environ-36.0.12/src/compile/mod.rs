//! A `Compilation` contains the compiled function bodies for a WebAssembly
//! module.

use crate::prelude::*;
use crate::{
    BuiltinFunctionIndex, DefinedFuncIndex, FlagValue, FuncIndex, FunctionLoc, ObjectKind,
    PrimaryMap, StaticModuleIndex, TripleExt, WasmError, WasmFuncType,
};
use crate::{Tunables, obj};
use anyhow::Result;
use object::write::{Object, SymbolId};
use object::{Architecture, BinaryFormat, FileFlags};
use std::any::Any;
use std::borrow::Cow;
use std::fmt;
use std::path;
use std::sync::Arc;

mod address_map;
mod module_artifacts;
mod module_environ;
mod module_types;
mod stack_maps;
mod trap_encoding;

pub use self::address_map::*;
pub use self::module_artifacts::*;
pub use self::module_environ::*;
pub use self::module_types::*;
pub use self::stack_maps::*;
pub use self::trap_encoding::*;

/// An error while compiling WebAssembly to machine code.
#[derive(Debug)]
pub enum CompileError {
    /// A wasm translation error occurred.
    Wasm(WasmError),

    /// A compilation error occurred.
    Codegen(String),

    /// A compilation error occurred.
    DebugInfoNotSupported,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::Wasm(_) => write!(f, "WebAssembly translation error"),
            CompileError::Codegen(s) => write!(f, "Compilation error: {s}"),
            CompileError::DebugInfoNotSupported => {
                write!(f, "Debug info is not supported with this configuration")
            }
        }
    }
}

impl From<WasmError> for CompileError {
    fn from(err: WasmError) -> CompileError {
        CompileError::Wasm(err)
    }
}

impl core::error::Error for CompileError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            CompileError::Wasm(e) => Some(e),
            _ => None,
        }
    }
}

/// What relocations can be applied against.
///
/// Each wasm function may refer to various other `RelocationTarget` entries.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RelocationTarget {
    /// This is a reference to another defined wasm function in the same module.
    Wasm(FuncIndex),
    /// This is a reference to a trampoline for a builtin function.
    Builtin(BuiltinFunctionIndex),
    /// A pulley->host call from the interpreter.
    PulleyHostcall(u32),
}

/// Implementation of an incremental compilation's key/value cache store.
///
/// In theory, this could just be Cranelift's `CacheKvStore` trait, but it is not as we want to
/// make sure that wasmtime isn't too tied to Cranelift internals (and as a matter of fact, we
/// can't depend on the Cranelift trait here).
pub trait CacheStore: Send + Sync + std::fmt::Debug {
    /// Try to retrieve an arbitrary cache key entry, and returns a reference to bytes that were
    /// inserted via `Self::insert` before.
    fn get(&self, key: &[u8]) -> Option<Cow<'_, [u8]>>;

    /// Given an arbitrary key and bytes, stores them in the cache.
    ///
    /// Returns false when insertion in the cache failed.
    fn insert(&self, key: &[u8], value: Vec<u8>) -> bool;
}

/// Abstract trait representing the ability to create a `Compiler` below.
///
/// This is used in Wasmtime to separate compiler implementations, currently
/// mostly used to separate Cranelift from Wasmtime itself.
pub trait CompilerBuilder: Send + Sync + fmt::Debug {
    /// Sets the target of compilation to the target specified.
    fn target(&mut self, target: target_lexicon::Triple) -> Result<()>;

    /// Enables clif output in the directory specified.
    fn clif_dir(&mut self, _path: &path::Path) -> Result<()> {
        anyhow::bail!("clif output not supported");
    }

    /// Returns the currently configured target triple that compilation will
    /// produce artifacts for.
    fn triple(&self) -> &target_lexicon::Triple;

    /// Compiler-specific method to configure various settings in the compiler
    /// itself.
    ///
    /// This is expected to be defined per-compiler. Compilers should return
    /// errors for unknown names/values.
    fn set(&mut self, name: &str, val: &str) -> Result<()>;

    /// Compiler-specific method for configuring settings.
    ///
    /// Same as [`CompilerBuilder::set`] except for enabling boolean flags.
    /// Currently cranelift uses this to sometimes enable a family of settings.
    fn enable(&mut self, name: &str) -> Result<()>;

    /// Returns a list of all possible settings that can be configured with
    /// [`CompilerBuilder::set`] and [`CompilerBuilder::enable`].
    fn settings(&self) -> Vec<Setting>;

    /// Enables Cranelift's incremental compilation cache, using the given `CacheStore`
    /// implementation.
    ///
    /// This will return an error if the compiler does not support incremental compilation.
    fn enable_incremental_compilation(&mut self, cache_store: Arc<dyn CacheStore>) -> Result<()>;

    /// Set the tunables for this compiler.
    fn set_tunables(&mut self, tunables: Tunables) -> Result<()>;

    /// Builds a new [`Compiler`] object from this configuration.
    fn build(&self) -> Result<Box<dyn Compiler>>;

    /// Enables or disables wmemcheck during runtime according to the wmemcheck CLI flag.
    fn wmemcheck(&mut self, _enable: bool) {}
}

/// Description of compiler settings returned by [`CompilerBuilder::settings`].
#[derive(Clone, Copy, Debug)]
pub struct Setting {
    /// The name of the setting.
    pub name: &'static str,
    /// The description of the setting.
    pub description: &'static str,
    /// The kind of the setting.
    pub kind: SettingKind,
    /// The supported values of the setting (for enum values).
    pub values: Option<&'static [&'static str]>,
}

/// Different kinds of [`Setting`] values that can be configured in a
/// [`CompilerBuilder`]
#[derive(Clone, Copy, Debug)]
pub enum SettingKind {
    /// The setting is an enumeration, meaning it's one of a set of values.
    Enum,
    /// The setting is a number.
    Num,
    /// The setting is a boolean.
    Bool,
    /// The setting is a preset.
    Preset,
}

/// The result of compiling a single function body.
pub struct CompiledFunctionBody {
    /// The code. This is whatever type the `Compiler` implementation wants it
    /// to be, we just shepherd it around.
    pub code: Box<dyn Any + Send + Sync>,
    /// Whether the compiled function needs a GC heap to run; that is, whether
    /// it reads a struct field, allocates, an array, or etc...
    pub needs_gc_heap: bool,
}

/// An implementation of a compiler which can compile WebAssembly functions to
/// machine code and perform other miscellaneous tasks needed by the JIT runtime.
///
/// The diagram below depicts typical usage of this trait:
///
/// ```ignore
///                     +------+
///                     | Wasm |
///                     +------+
///                        |
///                        |
///           Compiler::compile_function()
///                        |
///                        |
///                        V
///             +----------------------+
///             | CompiledFunctionBody |
///             +----------------------+
///               |                  |
///               |                  |
///               |                When
///               |       Compiler::inlining_compiler()
///               |               is some
///               |                  |
///             When                 |
/// Compiler::inlining_compiler()    |-----------------.
///             is none              |                 |
///               |                  |                 |
///               |           Optionally call          |
///               |        InliningCompiler::inline()  |
///               |                  |                 |
///               |                  |                 |
///               |                  |-----------------'
///               |                  |
///               |                  |
///               |                  V
///               |     InliningCompiler::finish_compiling()
///               |                  |
///               |                  |
///               |------------------'
///               |
///               |
///   Compiler::append_code()
///               |
///               |
///               V
///           +--------+
///           | Object |
///           +--------+
/// ```
pub trait Compiler: Send + Sync {
    /// Get this compiler's inliner.
    ///
    /// Consumers of this trait **must** check for when when this method returns
    /// `Some(_)`, and **must** call `InliningCompiler::finish_compiling` on all
    /// `CompiledFunctionBody`s produced by this compiler in that case before
    /// passing the the compiled functions to `Compiler::append_code`, even if
    /// the consumer does not actually intend to do any inlining. This allows
    /// implementations of the trait to only translate to an internal
    /// representation in `Compiler::compile_*` methods so that they can then
    /// perform inlining afterwards if the consumer desires, and then finally
    /// proceed with compilng that internal representation to native code in
    /// `InliningCompiler::finish_compiling`.
    fn inlining_compiler(&self) -> Option<&dyn InliningCompiler>;

    /// Compiles the function `index` within `translation`.
    ///
    /// The body of the function is available in `data` and configuration
    /// values are also passed in via `tunables`. Type information in
    /// `translation` is all relative to `types`.
    fn compile_function(
        &self,
        translation: &ModuleTranslation<'_>,
        index: DefinedFuncIndex,
        data: FunctionBodyData<'_>,
        types: &ModuleTypesBuilder,
        symbol: &str,
    ) -> Result<CompiledFunctionBody, CompileError>;

    /// Compile a trampoline for an array-call host function caller calling the
    /// `index`th Wasm function.
    ///
    /// The trampoline should save the necessary state to record the
    /// host-to-Wasm transition (e.g. registers used for fast stack walking).
    fn compile_array_to_wasm_trampoline(
        &self,
        translation: &ModuleTranslation<'_>,
        types: &ModuleTypesBuilder,
        index: DefinedFuncIndex,
        symbol: &str,
    ) -> Result<CompiledFunctionBody, CompileError>;

    /// Compile a trampoline for a Wasm caller calling a array callee with the
    /// given signature.
    ///
    /// The trampoline should save the necessary state to record the
    /// Wasm-to-host transition (e.g. registers used for fast stack walking).
    fn compile_wasm_to_array_trampoline(
        &self,
        wasm_func_ty: &WasmFuncType,
        symbol: &str,
    ) -> Result<CompiledFunctionBody, CompileError>;

    /// Creates a trampoline that can be used to call Wasmtime's implementation
    /// of the builtin function specified by `index`.
    ///
    /// The trampoline created can technically have any ABI but currently has
    /// the native ABI. This will then perform all the necessary duties of an
    /// exit trampoline from wasm and then perform the actual dispatch to the
    /// builtin function. Builtin functions in Wasmtime are stored in an array
    /// in all `VMContext` pointers, so the call to the host is an indirect
    /// call.
    fn compile_wasm_to_builtin(
        &self,
        index: BuiltinFunctionIndex,
        symbol: &str,
    ) -> Result<CompiledFunctionBody, CompileError>;

    /// Returns the list of relocations required for a function from one of the
    /// previous `compile_*` functions above.
    fn compiled_function_relocation_targets<'a>(
        &'a self,
        func: &'a dyn Any,
    ) -> Box<dyn Iterator<Item = RelocationTarget> + 'a>;

    /// Appends a list of compiled functions to an in-memory object.
    ///
    /// This function will receive the same `Box<dyn Any>` produced as part of
    /// compilation from functions like `compile_function`,
    /// `compile_host_to_wasm_trampoline`, and other component-related shims.
    /// Internally this will take all of these functions and add information to
    /// the object such as:
    ///
    /// * Compiled code in a `.text` section
    /// * Unwind information in Wasmtime-specific sections
    /// * Relocations, if necessary, for the text section
    ///
    /// Each function is accompanied with its desired symbol name and the return
    /// value of this function is the symbol for each function as well as where
    /// each function was placed within the object.
    ///
    /// The `resolve_reloc` argument is intended to resolving relocations
    /// between function, chiefly resolving intra-module calls within one core
    /// wasm module. The closure here takes two arguments:
    ///
    /// 1. First, the index within `funcs` that is being resolved,
    ///
    /// 2. and next the `RelocationTarget` which is the relocation target to
    ///    resolve.
    ///
    /// The return value is an index within `funcs` that the relocation points
    /// to.
    fn append_code(
        &self,
        obj: &mut Object<'static>,
        funcs: &[(String, Box<dyn Any + Send + Sync>)],
        resolve_reloc: &dyn Fn(usize, RelocationTarget) -> usize,
    ) -> Result<Vec<(SymbolId, FunctionLoc)>>;

    /// Creates a new `Object` file which is used to build the results of a
    /// compilation into.
    ///
    /// The returned object file will have an appropriate
    /// architecture/endianness for `self.triple()`, but at this time it is
    /// always an ELF file, regardless of target platform.
    fn object(&self, kind: ObjectKind) -> Result<Object<'static>> {
        use target_lexicon::Architecture::*;

        let triple = self.triple();
        let (arch, flags) = match triple.architecture {
            X86_32(_) => (Architecture::I386, 0),
            X86_64 => (Architecture::X86_64, 0),
            Arm(_) => (Architecture::Arm, 0),
            Aarch64(_) => (Architecture::Aarch64, 0),
            S390x => (Architecture::S390x, 0),
            Riscv64(_) => (Architecture::Riscv64, 0),
            // XXX: the `object` crate won't successfully build an object
            // with relocations and such if it doesn't know the
            // architecture, so just pretend we are riscv64. Yolo!
            //
            // Also note that we add some flags to `e_flags` in the object file
            // to indicate that it's pulley, not actually riscv64. This is used
            // by `wasmtime objdump` for example.
            Pulley32 | Pulley32be => (Architecture::Riscv64, obj::EF_WASMTIME_PULLEY32),
            Pulley64 | Pulley64be => (Architecture::Riscv64, obj::EF_WASMTIME_PULLEY64),
            architecture => {
                anyhow::bail!("target architecture {:?} is unsupported", architecture,);
            }
        };
        let mut obj = Object::new(
            BinaryFormat::Elf,
            arch,
            match triple.endianness().unwrap() {
                target_lexicon::Endianness::Little => object::Endianness::Little,
                target_lexicon::Endianness::Big => object::Endianness::Big,
            },
        );
        obj.flags = FileFlags::Elf {
            os_abi: obj::ELFOSABI_WASMTIME,
            e_flags: flags
                | match kind {
                    ObjectKind::Module => obj::EF_WASMTIME_MODULE,
                    ObjectKind::Component => obj::EF_WASMTIME_COMPONENT,
                },
            abi_version: 0,
        };
        Ok(obj)
    }

    /// Returns the target triple that this compiler is compiling for.
    fn triple(&self) -> &target_lexicon::Triple;

    /// Returns the alignment necessary to align values to the page size of the
    /// compilation target. Note that this may be an upper-bound where the
    /// alignment is larger than necessary for some platforms since it may
    /// depend on the platform's runtime configuration.
    fn page_size_align(&self) -> u64 {
        // Conservatively assume the max-of-all-supported-hosts for pulley
        // and round up to 64k.
        if self.triple().is_pulley() {
            return 0x10000;
        }

        use target_lexicon::*;
        match (self.triple().operating_system, self.triple().architecture) {
            (
                OperatingSystem::MacOSX { .. }
                | OperatingSystem::Darwin(_)
                | OperatingSystem::IOS(_)
                | OperatingSystem::TvOS(_),
                Architecture::Aarch64(..),
            ) => 0x4000,
            // 64 KB is the maximal page size (i.e. memory translation granule size)
            // supported by the architecture and is used on some platforms.
            (_, Architecture::Aarch64(..)) => 0x10000,
            _ => 0x1000,
        }
    }

    /// Returns a list of configured settings for this compiler.
    fn flags(&self) -> Vec<(&'static str, FlagValue<'static>)>;

    /// Same as [`Compiler::flags`], but ISA-specific (a cranelift-ism)
    fn isa_flags(&self) -> Vec<(&'static str, FlagValue<'static>)>;

    /// Get a flag indicating whether branch protection is enabled.
    fn is_branch_protection_enabled(&self) -> bool;

    /// Returns a suitable compiler usable for component-related compilations.
    ///
    /// Note that the `ComponentCompiler` trait can also be implemented for
    /// `Self` in which case this function would simply return `self`.
    #[cfg(feature = "component-model")]
    fn component_compiler(&self) -> &dyn crate::component::ComponentCompiler;

    /// Appends generated DWARF sections to the `obj` specified.
    ///
    /// The `translations` track all compiled functions and `get_func` can be
    /// used to acquire the metadata for a particular function within a module.
    fn append_dwarf<'a>(
        &self,
        obj: &mut Object<'_>,
        translations: &'a PrimaryMap<StaticModuleIndex, ModuleTranslation<'a>>,
        get_func: &'a dyn Fn(
            StaticModuleIndex,
            DefinedFuncIndex,
        ) -> (SymbolId, &'a (dyn Any + Send + Sync)),
        dwarf_package_bytes: Option<&'a [u8]>,
        tunables: &'a Tunables,
    ) -> Result<()>;

    /// Creates a new System V Common Information Entry for the ISA.
    ///
    /// Returns `None` if the ISA does not support System V unwind information.
    fn create_systemv_cie(&self) -> Option<gimli::write::CommonInformationEntry> {
        // By default, an ISA cannot create a System V CIE.
        None
    }
}

/// An inlining compiler.
pub trait InliningCompiler: Sync + Send {
    /// Enumerate the function calls that the given `func` makes.
    fn calls(&self, func: &CompiledFunctionBody, calls: &mut IndexSet<FuncIndex>) -> Result<()>;

    /// Get the abstract size of the given function, for the purposes of
    /// inlining heuristics.
    fn size(&self, func: &CompiledFunctionBody) -> u32;

    /// Process this function for inlining.
    ///
    /// Implementations should call `get_callee` for each of their direct
    /// function call sites and if `get_callee` returns `Some(_)`, they should
    /// inline the given function body into that call site.
    fn inline<'a>(
        &self,
        func: &mut CompiledFunctionBody,
        get_callee: &'a mut dyn FnMut(FuncIndex) -> Option<&'a CompiledFunctionBody>,
    ) -> Result<()>;

    /// Finish compiling the given function.
    ///
    /// This method **must** be called before passing the
    /// `CompiledFunctionBody`'s contents to `Compiler::append_code`, even if no
    /// inlining was performed.
    fn finish_compiling(
        &self,
        func: &mut CompiledFunctionBody,
        input: Option<wasmparser::FunctionBody<'_>>,
        symbol: &str,
    ) -> Result<()>;
}

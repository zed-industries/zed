//! Definitions of runtime structures and metadata which are serialized into ELF
//! with `bincode` as part of a module's compilation process.

use crate::prelude::*;
use crate::{DefinedFuncIndex, FilePos, FuncIndex, Module, ModuleInternedTypeIndex, PrimaryMap};
use core::fmt;
use core::ops::Range;
use core::str;
use serde_derive::{Deserialize, Serialize};

/// Secondary in-memory results of function compilation.
#[derive(Serialize, Deserialize)]
pub struct CompiledFunctionInfo {
    /// Where this function was found in the original wasm file.
    pub start_srcloc: FilePos,
    /// The [`FunctionLoc`] indicating the location of this function in the text
    /// section of the competition artifact.
    pub wasm_func_loc: FunctionLoc,
    /// A trampoline for array callers (e.g. `Func::new`) calling into this function (if needed).
    pub array_to_wasm_trampoline: Option<FunctionLoc>,
}

/// Description of where a function is located in the text section of a
/// compiled image.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct FunctionLoc {
    /// The byte offset from the start of the text section where this
    /// function starts.
    pub start: u32,
    /// The byte length of this function's function body.
    pub length: u32,
}

/// Secondary in-memory results of module compilation.
///
/// This opaque structure can be optionally passed back to
/// `CompiledModule::from_artifacts` to avoid decoding extra information there.
#[derive(Serialize, Deserialize)]
pub struct CompiledModuleInfo {
    /// Type information about the compiled WebAssembly module.
    pub module: Module,

    /// Metadata about each compiled function.
    pub funcs: PrimaryMap<DefinedFuncIndex, CompiledFunctionInfo>,

    /// Sorted list, by function index, of names we have for this module.
    pub func_names: Vec<FunctionName>,

    /// Metadata about wasm-to-array trampolines. Used when exposing a native
    /// callee (e.g. `Func::wrap`) to a Wasm caller. Sorted by signature index.
    pub wasm_to_array_trampolines: Vec<(ModuleInternedTypeIndex, FunctionLoc)>,

    /// General compilation metadata.
    pub meta: Metadata,
}

/// The name of a function stored in the
/// [`ELF_NAME_DATA`](crate::obj::ELF_NAME_DATA) section.
#[derive(Serialize, Deserialize)]
pub struct FunctionName {
    /// The Wasm function index of this function.
    pub idx: FuncIndex,
    /// The offset of the name in the
    /// [`ELF_NAME_DATA`](crate::obj::ELF_NAME_DATA) section.
    pub offset: u32,
    /// The length of the name in bytes.
    pub len: u32,
}

/// Metadata associated with a compiled ELF artifact.
#[derive(Serialize, Deserialize)]
pub struct Metadata {
    /// Whether or not the original wasm module contained debug information that
    /// we skipped and did not parse.
    pub has_unparsed_debuginfo: bool,

    /// Offset in the original wasm file to the code section.
    pub code_section_offset: u64,

    /// Whether or not custom wasm-specific dwarf sections were inserted into
    /// the ELF image.
    ///
    /// Note that even if this flag is `true` sections may be missing if they
    /// weren't found in the original wasm module itself.
    pub has_wasm_debuginfo: bool,

    /// Dwarf sections and the offsets at which they're stored in the
    /// ELF_WASMTIME_DWARF
    pub dwarf: Vec<(u8, Range<u64>)>,
}

/// Value of a configured setting for a [`Compiler`](crate::Compiler)
#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub enum FlagValue<'a> {
    /// Name of the value that has been configured for this setting.
    Enum(&'a str),
    /// The numerical value of the configured settings.
    Num(u8),
    /// Whether the setting is on or off.
    Bool(bool),
}

impl fmt::Display for FlagValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Enum(v) => v.fmt(f),
            Self::Num(v) => v.fmt(f),
            Self::Bool(v) => v.fmt(f),
        }
    }
}

/// Types of objects that can be created by `Compiler::object`
pub enum ObjectKind {
    /// A core wasm compilation artifact
    Module,
    /// A component compilation artifact
    Component,
}

//! Debug utils for WebAssembly using Cranelift.

// FIXME: this whole crate opts-in to these two noisier-than-default lints, but
// this module has lots of hits on this warning which aren't the easiest to
// resolve. Ideally all warnings would be resolved here though.
#![expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "haven't had a chance to fix these yet"
)]

use crate::CompiledFunctionMetadata;
use core::fmt;
use cranelift_codegen::isa::TargetIsa;
use object::write::SymbolId;
use std::collections::HashMap;
use wasmtime_environ::{
    DefinedFuncIndex, DefinedMemoryIndex, EntityRef, MemoryIndex, ModuleTranslation,
    OwnedMemoryIndex, PrimaryMap, PtrSize, StaticModuleIndex, Tunables, VMOffsets,
};

/// Memory definition offset in the VMContext structure.
#[derive(Debug, Clone)]
pub enum ModuleMemoryOffset {
    /// Not available.
    None,
    /// Offset to the defined memory.
    Defined(u32),
    /// This memory is imported.
    Imported {
        /// Offset, in bytes, to the `*mut VMMemoryDefinition` structure within
        /// `VMContext`.
        offset_to_vm_memory_definition: u32,
        /// Offset, in bytes within `VMMemoryDefinition` where the `base` field
        /// lies.
        offset_to_memory_base: u32,
    },
}

type Reader<'input> = gimli::EndianSlice<'input, gimli::LittleEndian>;

/// "Package structure" to collect together various artifacts/results of a
/// compilation.
///
/// This structure is threaded through a number of top-level functions of DWARF
/// processing within in this submodule to pass along all the bits-and-pieces of
/// the compilation context.
pub struct Compilation<'a> {
    /// All module translations which were present in this compilation.
    ///
    /// This map has one entry for core wasm modules and may have multiple (or
    /// zero) for components.
    translations: &'a PrimaryMap<StaticModuleIndex, ModuleTranslation<'a>>,

    /// Accessor of a particular compiled function for a module.
    ///
    /// This returns the `object`-based-symbol for the function as well as the
    /// `&CompiledFunction`.
    get_func:
        &'a dyn Fn(StaticModuleIndex, DefinedFuncIndex) -> (SymbolId, &'a CompiledFunctionMetadata),

    /// Optionally-specified `*.dwp` file, currently only supported for core
    /// wasm modules.
    dwarf_package_bytes: Option<&'a [u8]>,

    /// Compilation settings used when producing functions.
    tunables: &'a Tunables,

    /// Translation between `SymbolId` and a `usize`-based symbol which gimli
    /// uses.
    symbol_index_to_id: Vec<SymbolId>,
    symbol_id_to_index: HashMap<SymbolId, (usize, StaticModuleIndex, DefinedFuncIndex)>,

    /// The `ModuleMemoryOffset` for each module within `translations`.
    ///
    /// Note that this doesn't support multi-memory at this time.
    module_memory_offsets: PrimaryMap<StaticModuleIndex, ModuleMemoryOffset>,
}

impl<'a> Compilation<'a> {
    pub fn new(
        isa: &dyn TargetIsa,
        translations: &'a PrimaryMap<StaticModuleIndex, ModuleTranslation<'a>>,
        get_func: &'a dyn Fn(
            StaticModuleIndex,
            DefinedFuncIndex,
        ) -> (SymbolId, &'a CompiledFunctionMetadata),
        dwarf_package_bytes: Option<&'a [u8]>,
        tunables: &'a Tunables,
    ) -> Compilation<'a> {
        // Build the `module_memory_offsets` map based on the modules in
        // `translations`.
        let mut module_memory_offsets = PrimaryMap::new();
        for (i, translation) in translations {
            let ofs = VMOffsets::new(
                isa.triple().architecture.pointer_width().unwrap().bytes(),
                &translation.module,
            );

            let memory_offset = if ofs.num_imported_memories > 0 {
                let index = MemoryIndex::new(0);
                ModuleMemoryOffset::Imported {
                    offset_to_vm_memory_definition: ofs.vmctx_vmmemory_import(index)
                        + u32::from(ofs.vmmemory_import_from()),
                    offset_to_memory_base: ofs.ptr.vmmemory_definition_base().into(),
                }
            } else if ofs.num_owned_memories > 0 {
                let index = OwnedMemoryIndex::new(0);
                ModuleMemoryOffset::Defined(ofs.vmctx_vmmemory_definition_base(index))
            } else if ofs.num_defined_memories > 0 {
                let index = DefinedMemoryIndex::new(0);
                ModuleMemoryOffset::Imported {
                    offset_to_vm_memory_definition: ofs.vmctx_vmmemory_pointer(index),
                    offset_to_memory_base: ofs.ptr.vmmemory_definition_base().into(),
                }
            } else {
                ModuleMemoryOffset::None
            };
            let j = module_memory_offsets.push(memory_offset);
            assert_eq!(i, j);
        }

        // Build the `symbol <=> usize` mappings
        let mut symbol_index_to_id = Vec::new();
        let mut symbol_id_to_index = HashMap::new();

        for (module, translation) in translations {
            for func in translation.module.defined_func_indices() {
                let (sym, _func) = get_func(module, func);
                symbol_id_to_index.insert(sym, (symbol_index_to_id.len(), module, func));
                symbol_index_to_id.push(sym);
            }
        }

        Compilation {
            translations,
            get_func,
            dwarf_package_bytes,
            tunables,
            symbol_index_to_id,
            symbol_id_to_index,
            module_memory_offsets,
        }
    }

    /// Returns an iterator over all function indexes present in this
    /// compilation.
    ///
    /// Each function is additionally accompanied with its module index.
    fn indexes(&self) -> impl Iterator<Item = (StaticModuleIndex, DefinedFuncIndex)> + use<'_> {
        self.translations
            .iter()
            .flat_map(|(i, t)| t.module.defined_func_indices().map(move |j| (i, j)))
    }

    /// Returns an iterator of all functions with their module, symbol, and
    /// function metadata that were produced during compilation.
    fn functions(
        &self,
    ) -> impl Iterator<Item = (StaticModuleIndex, usize, &'a CompiledFunctionMetadata)> + '_ {
        self.indexes().map(move |(module, func)| {
            let (sym, func) = self.function(module, func);
            (module, sym, func)
        })
    }

    /// Returns the symbol and metadata associated with a specific function.
    fn function(
        &self,
        module: StaticModuleIndex,
        func: DefinedFuncIndex,
    ) -> (usize, &'a CompiledFunctionMetadata) {
        let (sym, func) = (self.get_func)(module, func);
        (self.symbol_id_to_index[&sym].0, func)
    }

    /// Maps a `usize`-based symbol used by gimli to the object-based
    /// `SymbolId`.
    pub fn symbol_id(&self, sym: usize) -> SymbolId {
        self.symbol_index_to_id[sym]
    }
}

impl<'a> fmt::Debug for Compilation<'a> {
    // Sample output: '[#0: OneModule, #1: TwoModule, #3]'.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut is_first_module = true;
        for (i, translation) in self.translations {
            if !is_first_module {
                write!(f, ", ")?;
            } else {
                is_first_module = false;
            }
            write!(f, "#{}", i.as_u32())?;
            if let Some(name) = translation.debuginfo.name_section.module_name {
                write!(f, ": {name}")?;
            }
        }
        write!(f, "]")
    }
}

pub use write_debuginfo::{DwarfSectionRelocTarget, emit_dwarf};

mod gc;
mod transform;
mod write_debuginfo;

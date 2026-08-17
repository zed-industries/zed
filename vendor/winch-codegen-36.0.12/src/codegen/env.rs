use crate::{
    abi::{ABI, ABISig, wasm_sig},
    codegen::{BlockSig, BuiltinFunction, BuiltinFunctions, OperandSize, control},
    isa::TargetIsa,
};
use anyhow::Result;
use cranelift_codegen::ir::{UserExternalName, UserExternalNameRef};
use std::collections::{
    HashMap,
    hash_map::Entry::{Occupied, Vacant},
};
use std::mem;
use wasmparser::BlockType;
use wasmtime_environ::{
    BuiltinFunctionIndex, FuncIndex, GlobalIndex, IndexType, Memory, MemoryIndex,
    ModuleTranslation, ModuleTypesBuilder, PrimaryMap, PtrSize, Table, TableIndex, TypeConvert,
    TypeIndex, VMOffsets, WasmHeapType, WasmValType,
};

#[derive(Debug, Clone, Copy)]
pub struct GlobalData {
    /// The offset of the global.
    pub offset: u32,
    /// True if the global is imported.
    pub imported: bool,
    /// The WebAssembly type of the global.
    pub ty: WasmValType,
}

/// Table metadata.
#[derive(Debug, Copy, Clone)]
pub struct TableData {
    /// The offset to the base of the table.
    pub offset: u32,
    /// The offset to the current elements field.
    pub current_elems_offset: u32,
    /// If the table is imported, this field contains the offset to locate the
    /// base of the table data.
    pub import_from: Option<u32>,
    /// The size of the table elements.
    pub(crate) element_size: OperandSize,
    /// The size of the current elements field.
    pub(crate) current_elements_size: OperandSize,
    /// The type of this table.
    pub ty: Table,
}

impl TableData {
    pub fn index_type(&self) -> WasmValType {
        match self.ty.idx_type {
            IndexType::I32 => WasmValType::I32,
            IndexType::I64 => WasmValType::I64,
        }
    }
}

/// Heap metadata.
///
/// Heaps represent a WebAssembly linear memory.
#[derive(Debug, Copy, Clone)]
pub struct HeapData {
    /// The offset to the base of the heap.
    /// Relative to the `VMContext` pointer if the WebAssembly memory is locally
    /// defined. Else this is relative to the location of the imported WebAssembly
    /// memory location.
    pub offset: u32,
    /// The offset to the current length field.
    pub current_length_offset: u32,
    /// If the WebAssembly memory is imported or shared, this field contains the offset to locate the
    /// base of the heap.
    pub import_from: Option<u32>,
    /// The memory type this heap is associated with.
    pub memory: Memory,
}

impl HeapData {
    pub fn index_type(&self) -> WasmValType {
        match self.memory.idx_type {
            IndexType::I32 => WasmValType::I32,
            IndexType::I64 => WasmValType::I64,
        }
    }
}

/// A function callee.
/// It categorizes how the callee should be treated
/// when performing the call.
#[derive(Clone)]
pub(crate) enum Callee {
    /// Locally defined function.
    Local(FuncIndex),
    /// Imported function.
    Import(FuncIndex),
    /// Function reference.
    FuncRef(TypeIndex),
    /// A built-in function.
    Builtin(BuiltinFunction),
    /// A built-in function, but the vmctx argument is located at the static
    /// offset provided from the current function's vmctx.
    BuiltinWithDifferentVmctx(BuiltinFunction, u32),
}

/// The function environment.
///
/// Contains all information about the module and runtime that is accessible to
/// to a particular function during code generation.
pub struct FuncEnv<'a, 'translation: 'a, 'data: 'translation, P: PtrSize> {
    /// Offsets to the fields within the `VMContext` ptr.
    pub vmoffsets: &'a VMOffsets<P>,
    /// Metadata about the translation process of a WebAssembly module.
    pub translation: &'translation ModuleTranslation<'data>,
    /// The module's function types.
    pub types: &'translation ModuleTypesBuilder,
    /// The built-in functions available to the JIT code.
    pub builtins: &'translation mut BuiltinFunctions,
    /// Track resolved table information.
    resolved_tables: HashMap<TableIndex, TableData>,
    /// Track resolved heap information.
    resolved_heaps: HashMap<MemoryIndex, HeapData>,
    /// A map from [FunctionIndex] to [ABISig], to keep track of the resolved
    /// function callees.
    resolved_callees: HashMap<FuncIndex, ABISig>,
    /// A map from [TypeIndex] to [ABISig], to keep track of the resolved
    /// indirect function signatures.
    resolved_sigs: HashMap<TypeIndex, ABISig>,
    /// A map from [GlobalIndex] to [GlobalData].
    resolved_globals: HashMap<GlobalIndex, GlobalData>,
    /// Pointer size represented as a WebAssembly type.
    ptr_type: WasmValType,
    /// Whether or not to enable Spectre mitigation on heap bounds checks.
    heap_access_spectre_mitigation: bool,
    /// Whether or not to enable Spectre mitigation on table element accesses.
    table_access_spectre_mitigation: bool,
    /// Size of pages on the compilation target.
    pub page_size_log2: u8,
    name_map: PrimaryMap<UserExternalNameRef, UserExternalName>,
    name_intern: HashMap<UserExternalName, UserExternalNameRef>,
}

pub fn ptr_type_from_ptr_size(size: u8) -> WasmValType {
    (size == 8)
        .then(|| WasmValType::I64)
        .unwrap_or_else(|| unimplemented!("Support for non-64-bit architectures"))
}

impl<'a, 'translation, 'data, P: PtrSize> FuncEnv<'a, 'translation, 'data, P> {
    /// Create a new function environment.
    pub fn new(
        vmoffsets: &'a VMOffsets<P>,
        translation: &'translation ModuleTranslation<'data>,
        types: &'translation ModuleTypesBuilder,
        builtins: &'translation mut BuiltinFunctions,
        isa: &dyn TargetIsa,
        ptr_type: WasmValType,
    ) -> Self {
        Self {
            vmoffsets,
            translation,
            types,
            resolved_tables: HashMap::new(),
            resolved_heaps: HashMap::new(),
            resolved_callees: HashMap::new(),
            resolved_sigs: HashMap::new(),
            resolved_globals: HashMap::new(),
            ptr_type,
            heap_access_spectre_mitigation: isa.flags().enable_heap_access_spectre_mitigation(),
            table_access_spectre_mitigation: isa.flags().enable_table_access_spectre_mitigation(),
            page_size_log2: isa.page_size_align_log2(),
            builtins,
            name_map: Default::default(),
            name_intern: Default::default(),
        }
    }

    /// Derive the [`WasmType`] from the pointer size.
    pub(crate) fn ptr_type(&self) -> WasmValType {
        self.ptr_type
    }

    /// Resolves a [`Callee::FuncRef`] from a type index.
    pub(crate) fn funcref(&mut self, idx: TypeIndex) -> Callee {
        Callee::FuncRef(idx)
    }

    /// Resolves a function [`Callee`] from an index.
    pub(crate) fn callee_from_index(&mut self, idx: FuncIndex) -> Callee {
        let import = self.translation.module.is_imported_function(idx);
        if import {
            Callee::Import(idx)
        } else {
            Callee::Local(idx)
        }
    }

    /// Converts a [wasmparser::BlockType] into a [BlockSig].
    pub(crate) fn resolve_block_sig(&self, ty: BlockType) -> Result<BlockSig> {
        use BlockType::*;
        Ok(match ty {
            Empty => BlockSig::new(control::BlockType::void()),
            Type(ty) => {
                let ty = TypeConverter::new(self.translation, self.types).convert_valtype(ty)?;
                BlockSig::new(control::BlockType::single(ty))
            }
            FuncType(idx) => {
                let sig_index = self.translation.module.types[TypeIndex::from_u32(idx)]
                    .unwrap_module_type_index();
                let sig = self.types[sig_index].unwrap_func();
                BlockSig::new(control::BlockType::func(sig.clone()))
            }
        })
    }

    /// Resolves `GlobalData` of a global at the given index.
    pub fn resolve_global(&mut self, index: GlobalIndex) -> GlobalData {
        let ty = self.translation.module.globals[index].wasm_ty;
        let val = || match self.translation.module.defined_global_index(index) {
            Some(defined_index) => GlobalData {
                offset: self.vmoffsets.vmctx_vmglobal_definition(defined_index),
                imported: false,
                ty,
            },
            None => GlobalData {
                offset: self.vmoffsets.vmctx_vmglobal_import_from(index),
                imported: true,
                ty,
            },
        };

        *self.resolved_globals.entry(index).or_insert_with(val)
    }

    /// Returns the table information for the given table index.
    pub fn resolve_table_data(&mut self, index: TableIndex) -> TableData {
        match self.resolved_tables.entry(index) {
            Occupied(entry) => *entry.get(),
            Vacant(entry) => {
                let (from_offset, base_offset, current_elems_offset) =
                    match self.translation.module.defined_table_index(index) {
                        Some(defined) => (
                            None,
                            self.vmoffsets.vmctx_vmtable_definition_base(defined),
                            self.vmoffsets
                                .vmctx_vmtable_definition_current_elements(defined),
                        ),
                        None => (
                            Some(self.vmoffsets.vmctx_vmtable_from(index)),
                            self.vmoffsets.vmtable_definition_base().into(),
                            self.vmoffsets.vmtable_definition_current_elements().into(),
                        ),
                    };

                *entry.insert(TableData {
                    import_from: from_offset,
                    offset: base_offset,
                    current_elems_offset,
                    element_size: OperandSize::from_bytes(self.vmoffsets.ptr.size()),
                    current_elements_size: OperandSize::from_bytes(
                        self.vmoffsets.size_of_vmtable_definition_current_elements(),
                    ),
                    ty: self.translation.module.tables[index],
                })
            }
        }
    }

    /// Resolve a `HeapData` from a [MemoryIndex].
    pub fn resolve_heap(&mut self, index: MemoryIndex) -> HeapData {
        let mem = self.translation.module.memories[index];
        let is_shared = mem.shared;
        match self.resolved_heaps.entry(index) {
            Occupied(entry) => *entry.get(),
            Vacant(entry) => {
                let (import_from, base_offset, current_length_offset) =
                    match self.translation.module.defined_memory_index(index) {
                        Some(defined) => {
                            if is_shared {
                                (
                                    Some(self.vmoffsets.vmctx_vmmemory_pointer(defined)),
                                    self.vmoffsets.ptr.vmmemory_definition_base().into(),
                                    self.vmoffsets
                                        .ptr
                                        .vmmemory_definition_current_length()
                                        .into(),
                                )
                            } else {
                                let owned = self.translation.module.owned_memory_index(defined);
                                (
                                    None,
                                    self.vmoffsets.vmctx_vmmemory_definition_base(owned),
                                    self.vmoffsets
                                        .vmctx_vmmemory_definition_current_length(owned),
                                )
                            }
                        }
                        None => (
                            Some(self.vmoffsets.vmctx_vmmemory_import_from(index)),
                            self.vmoffsets.ptr.vmmemory_definition_base().into(),
                            self.vmoffsets
                                .ptr
                                .vmmemory_definition_current_length()
                                .into(),
                        ),
                    };

                let memory = &self.translation.module.memories[index];

                *entry.insert(HeapData {
                    offset: base_offset,
                    import_from,
                    current_length_offset,
                    memory: *memory,
                })
            }
        }
    }

    /// Get a [`Table`] from a [`TableIndex`].
    pub fn table(&mut self, index: TableIndex) -> &Table {
        &self.translation.module.tables[index]
    }

    /// Returns true if Spectre mitigations are enabled for heap bounds check.
    pub fn heap_access_spectre_mitigation(&self) -> bool {
        self.heap_access_spectre_mitigation
    }

    /// Returns true if Spectre mitigations are enabled for table element
    /// accesses.
    pub fn table_access_spectre_mitigation(&self) -> bool {
        self.table_access_spectre_mitigation
    }

    pub(crate) fn callee_sig<'b, A>(&'b mut self, callee: &'b Callee) -> Result<&'b ABISig>
    where
        A: ABI,
    {
        match callee {
            Callee::Local(idx) | Callee::Import(idx) => {
                if self.resolved_callees.contains_key(idx) {
                    Ok(self.resolved_callees.get(idx).unwrap())
                } else {
                    let types = self.translation.get_types();
                    let types = types.as_ref();
                    let ty = types[types.core_function_at(idx.as_u32())].unwrap_func();
                    let converter = TypeConverter::new(self.translation, self.types);
                    let ty = converter.convert_func_type(&ty)?;
                    let sig = wasm_sig::<A>(&ty)?;
                    self.resolved_callees.insert(*idx, sig);
                    Ok(self.resolved_callees.get(idx).unwrap())
                }
            }
            Callee::FuncRef(idx) => {
                if self.resolved_sigs.contains_key(idx) {
                    Ok(self.resolved_sigs.get(idx).unwrap())
                } else {
                    let sig_index = self.translation.module.types[*idx].unwrap_module_type_index();
                    let ty = self.types[sig_index].unwrap_func();
                    let sig = wasm_sig::<A>(ty)?;
                    self.resolved_sigs.insert(*idx, sig);
                    Ok(self.resolved_sigs.get(idx).unwrap())
                }
            }
            Callee::Builtin(b) | Callee::BuiltinWithDifferentVmctx(b, _) => Ok(b.sig()),
        }
    }

    /// Creates a name to reference the `builtin` provided.
    pub fn name_builtin(&mut self, builtin: BuiltinFunctionIndex) -> UserExternalNameRef {
        self.intern_name(UserExternalName {
            namespace: wasmtime_cranelift::NS_WASMTIME_BUILTIN,
            index: builtin.index(),
        })
    }

    /// Creates a name to reference the wasm function `index` provided.
    pub fn name_wasm(&mut self, index: FuncIndex) -> UserExternalNameRef {
        self.intern_name(UserExternalName {
            namespace: wasmtime_cranelift::NS_WASM_FUNC,
            index: index.as_u32(),
        })
    }

    /// Interns `name` into a `UserExternalNameRef` and ensures that duplicate
    /// instances of `name` are given a unique name ref index.
    fn intern_name(&mut self, name: UserExternalName) -> UserExternalNameRef {
        *self
            .name_intern
            .entry(name.clone())
            .or_insert_with(|| self.name_map.push(name))
    }

    /// Extracts the name map that was created while translating this function.
    pub fn take_name_map(&mut self) -> PrimaryMap<UserExternalNameRef, UserExternalName> {
        self.name_intern.clear();
        mem::take(&mut self.name_map)
    }
}

/// A wrapper struct over a reference to a [ModuleTranslation] and
/// [ModuleTypesBuilder].
pub(crate) struct TypeConverter<'a, 'data: 'a> {
    translation: &'a ModuleTranslation<'data>,
    types: &'a ModuleTypesBuilder,
}

impl TypeConvert for TypeConverter<'_, '_> {
    fn lookup_heap_type(&self, idx: wasmparser::UnpackedIndex) -> WasmHeapType {
        wasmtime_environ::WasmparserTypeConverter::new(self.types, |idx| {
            self.translation.module.types[idx].unwrap_module_type_index()
        })
        .lookup_heap_type(idx)
    }

    fn lookup_type_index(
        &self,
        index: wasmparser::UnpackedIndex,
    ) -> wasmtime_environ::EngineOrModuleTypeIndex {
        wasmtime_environ::WasmparserTypeConverter::new(self.types, |idx| {
            self.translation.module.types[idx].unwrap_module_type_index()
        })
        .lookup_type_index(index)
    }
}

impl<'a, 'data> TypeConverter<'a, 'data> {
    pub fn new(translation: &'a ModuleTranslation<'data>, types: &'a ModuleTypesBuilder) -> Self {
        Self { translation, types }
    }
}

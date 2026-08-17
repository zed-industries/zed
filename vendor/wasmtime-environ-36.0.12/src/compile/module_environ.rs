use crate::module::{
    FuncRefIndex, Initializer, MemoryInitialization, MemoryInitializer, Module, TableSegment,
    TableSegmentElements,
};
use crate::{
    ConstExpr, ConstOp, DataIndex, DefinedFuncIndex, ElemIndex, EngineOrModuleTypeIndex,
    EntityIndex, EntityType, FuncIndex, GlobalIndex, IndexType, InitMemory, MemoryIndex,
    ModuleInternedTypeIndex, ModuleTypesBuilder, PrimaryMap, SizeOverflow, StaticMemoryInitializer,
    TableIndex, TableInitialValue, Tag, TagIndex, Tunables, TypeConvert, TypeIndex, Unsigned,
    WasmError, WasmHeapTopType, WasmHeapType, WasmResult, WasmValType, WasmparserTypeConverter,
};
use crate::{StaticModuleIndex, prelude::*};
use anyhow::{Result, bail};
use cranelift_entity::SecondaryMap;
use cranelift_entity::packed_option::ReservedValue;
use std::borrow::Cow;
use std::collections::HashMap;
use std::mem;
use std::path::PathBuf;
use std::sync::Arc;
use wasmparser::{
    CustomSectionReader, DataKind, ElementItems, ElementKind, Encoding, ExternalKind,
    FuncToValidate, FunctionBody, KnownCustom, NameSectionReader, Naming, Parser, Payload, TypeRef,
    Validator, ValidatorResources, types::Types,
};

/// Object containing the standalone environment information.
pub struct ModuleEnvironment<'a, 'data> {
    /// The current module being translated
    result: ModuleTranslation<'data>,

    /// Intern'd types for this entire translation, shared by all modules.
    types: &'a mut ModuleTypesBuilder,

    // Various bits and pieces of configuration
    validator: &'a mut Validator,
    tunables: &'a Tunables,
}

/// The result of translating via `ModuleEnvironment`. Function bodies are not
/// yet translated, and data initializers have not yet been copied out of the
/// original buffer.
#[derive(Default)]
pub struct ModuleTranslation<'data> {
    /// Module information.
    pub module: Module,

    /// The input wasm binary.
    ///
    /// This can be useful, for example, when modules are parsed from a
    /// component and the embedder wants access to the raw wasm modules
    /// themselves.
    pub wasm: &'data [u8],

    /// References to the function bodies.
    pub function_body_inputs: PrimaryMap<DefinedFuncIndex, FunctionBodyData<'data>>,

    /// For each imported function, the single statically-known defined function
    /// that satisfies that import, if any. This is used to turn what would
    /// otherwise be indirect calls through the imports table into direct calls,
    /// when possible.
    pub known_imported_functions:
        SecondaryMap<FuncIndex, Option<(StaticModuleIndex, DefinedFuncIndex)>>,

    /// A list of type signatures which are considered exported from this
    /// module, or those that can possibly be called. This list is sorted, and
    /// trampolines for each of these signatures are required.
    pub exported_signatures: Vec<ModuleInternedTypeIndex>,

    /// DWARF debug information, if enabled, parsed from the module.
    pub debuginfo: DebugInfoData<'data>,

    /// Set if debuginfo was found but it was not parsed due to `Tunables`
    /// configuration.
    pub has_unparsed_debuginfo: bool,

    /// List of data segments found in this module which should be concatenated
    /// together for the final compiled artifact.
    ///
    /// These data segments, when concatenated, are indexed by the
    /// `MemoryInitializer` type.
    pub data: Vec<Cow<'data, [u8]>>,

    /// The desired alignment of `data` in the final data section of the object
    /// file that we'll emit.
    ///
    /// Note that this is 1 by default but `MemoryInitialization::Static` might
    /// switch this to a higher alignment to facilitate mmap-ing data from
    /// an object file into a linear memory.
    pub data_align: Option<u64>,

    /// Total size of all data pushed onto `data` so far.
    total_data: u32,

    /// List of passive element segments found in this module which will get
    /// concatenated for the final artifact.
    pub passive_data: Vec<&'data [u8]>,

    /// Total size of all passive data pushed into `passive_data` so far.
    total_passive_data: u32,

    /// When we're parsing the code section this will be incremented so we know
    /// which function is currently being defined.
    code_index: u32,

    /// The type information of the current module made available at the end of the
    /// validation process.
    types: Option<Types>,
}

impl<'data> ModuleTranslation<'data> {
    /// Returns a reference to the type information of the current module.
    pub fn get_types(&self) -> &Types {
        self.types
            .as_ref()
            .expect("module type information to be available")
    }
}

/// Contains function data: byte code and its offset in the module.
pub struct FunctionBodyData<'a> {
    /// The body of the function, containing code and locals.
    pub body: FunctionBody<'a>,
    /// Validator for the function body
    pub validator: FuncToValidate<ValidatorResources>,
}

#[derive(Debug, Default)]
#[expect(missing_docs, reason = "self-describing fields")]
pub struct DebugInfoData<'a> {
    pub dwarf: Dwarf<'a>,
    pub name_section: NameSection<'a>,
    pub wasm_file: WasmFileInfo,
    pub debug_loc: gimli::DebugLoc<Reader<'a>>,
    pub debug_loclists: gimli::DebugLocLists<Reader<'a>>,
    pub debug_ranges: gimli::DebugRanges<Reader<'a>>,
    pub debug_rnglists: gimli::DebugRngLists<Reader<'a>>,
    pub debug_cu_index: gimli::DebugCuIndex<Reader<'a>>,
    pub debug_tu_index: gimli::DebugTuIndex<Reader<'a>>,
}

#[expect(missing_docs, reason = "self-describing")]
pub type Dwarf<'input> = gimli::Dwarf<Reader<'input>>;

type Reader<'input> = gimli::EndianSlice<'input, gimli::LittleEndian>;

#[derive(Debug, Default)]
#[expect(missing_docs, reason = "self-describing fields")]
pub struct NameSection<'a> {
    pub module_name: Option<&'a str>,
    pub func_names: HashMap<FuncIndex, &'a str>,
    pub locals_names: HashMap<FuncIndex, HashMap<u32, &'a str>>,
}

#[derive(Debug, Default)]
#[expect(missing_docs, reason = "self-describing fields")]
pub struct WasmFileInfo {
    pub path: Option<PathBuf>,
    pub code_section_offset: u64,
    pub imported_func_count: u32,
    pub funcs: Vec<FunctionMetadata>,
}

#[derive(Debug)]
#[expect(missing_docs, reason = "self-describing fields")]
pub struct FunctionMetadata {
    pub params: Box<[WasmValType]>,
    pub locals: Box<[(u32, WasmValType)]>,
}

impl<'a, 'data> ModuleEnvironment<'a, 'data> {
    /// Allocates the environment data structures.
    pub fn new(
        tunables: &'a Tunables,
        validator: &'a mut Validator,
        types: &'a mut ModuleTypesBuilder,
    ) -> Self {
        Self {
            result: ModuleTranslation::default(),
            types,
            tunables,
            validator,
        }
    }

    /// Translate a wasm module using this environment.
    ///
    /// This function will translate the `data` provided with `parser`,
    /// validating everything along the way with this environment's validator.
    ///
    /// The result of translation, [`ModuleTranslation`], contains everything
    /// necessary to compile functions afterwards as well as learn type
    /// information about the module at runtime.
    pub fn translate(
        mut self,
        parser: Parser,
        data: &'data [u8],
    ) -> Result<ModuleTranslation<'data>> {
        self.result.wasm = data;

        for payload in parser.parse_all(data) {
            self.translate_payload(payload?)?;
        }

        Ok(self.result)
    }

    fn translate_payload(&mut self, payload: Payload<'data>) -> Result<()> {
        match payload {
            Payload::Version {
                num,
                encoding,
                range,
            } => {
                self.validator.version(num, encoding, &range)?;
                match encoding {
                    Encoding::Module => {}
                    Encoding::Component => {
                        bail!("expected a WebAssembly module but was given a WebAssembly component")
                    }
                }
            }

            Payload::End(offset) => {
                self.result.types = Some(self.validator.end(offset)?);

                // With the `escaped_funcs` set of functions finished
                // we can calculate the set of signatures that are exported as
                // the set of exported functions' signatures.
                self.result.exported_signatures = self
                    .result
                    .module
                    .functions
                    .iter()
                    .filter_map(|(_, func)| {
                        if func.is_escaping() {
                            Some(func.signature.unwrap_module_type_index())
                        } else {
                            None
                        }
                    })
                    .collect();
                self.result.exported_signatures.sort_unstable();
                self.result.exported_signatures.dedup();
            }

            Payload::TypeSection(types) => {
                self.validator.type_section(&types)?;

                let count = self.validator.types(0).unwrap().core_type_count_in_module();
                log::trace!("interning {count} Wasm types");

                let capacity = usize::try_from(count).unwrap();
                self.result.module.types.reserve(capacity);
                self.types.reserve_wasm_signatures(capacity);

                // Iterate over each *rec group* -- not type -- defined in the
                // types section. Rec groups are the unit of canonicalization
                // and therefore the unit at which we need to process at a
                // time. `wasmparser` has already done the hard work of
                // de-duplicating and canonicalizing the rec groups within the
                // module for us, we just need to translate them into our data
                // structures. Note that, if the Wasm defines duplicate rec
                // groups, we need copy the duplicates over (shallowly) as well,
                // so that our types index space doesn't have holes.
                let mut type_index = 0;
                while type_index < count {
                    let validator_types = self.validator.types(0).unwrap();

                    // Get the rec group for the current type index, which is
                    // always the first type defined in a rec group.
                    log::trace!("looking up wasmparser type for index {type_index}");
                    let core_type_id = validator_types.core_type_at_in_module(type_index);
                    log::trace!(
                        "  --> {core_type_id:?} = {:?}",
                        validator_types[core_type_id],
                    );
                    let rec_group_id = validator_types.rec_group_id_of(core_type_id);
                    debug_assert_eq!(
                        validator_types
                            .rec_group_elements(rec_group_id)
                            .position(|id| id == core_type_id),
                        Some(0)
                    );

                    // Intern the rec group and then fill in this module's types
                    // index space.
                    let interned = self.types.intern_rec_group(validator_types, rec_group_id)?;
                    let elems = self.types.rec_group_elements(interned);
                    let len = elems.len();
                    self.result.module.types.reserve(len);
                    for ty in elems {
                        self.result.module.types.push(ty.into());
                    }

                    // Advance `type_index` to the start of the next rec group.
                    type_index += u32::try_from(len).unwrap();
                }
            }

            Payload::ImportSection(imports) => {
                self.validator.import_section(&imports)?;

                let cnt = usize::try_from(imports.count()).unwrap();
                self.result.module.initializers.reserve(cnt);

                for entry in imports {
                    let import = entry?;
                    let ty = match import.ty {
                        TypeRef::Func(index) => {
                            let index = TypeIndex::from_u32(index);
                            let interned_index = self.result.module.types[index];
                            self.result.module.num_imported_funcs += 1;
                            self.result.debuginfo.wasm_file.imported_func_count += 1;
                            EntityType::Function(interned_index)
                        }
                        TypeRef::Memory(ty) => {
                            self.result.module.num_imported_memories += 1;
                            EntityType::Memory(ty.into())
                        }
                        TypeRef::Global(ty) => {
                            self.result.module.num_imported_globals += 1;
                            EntityType::Global(self.convert_global_type(&ty)?)
                        }
                        TypeRef::Table(ty) => {
                            self.result.module.num_imported_tables += 1;
                            EntityType::Table(self.convert_table_type(&ty)?)
                        }
                        TypeRef::Tag(ty) => {
                            let index = TypeIndex::from_u32(ty.func_type_idx);
                            let signature = self.result.module.types[index];
                            let tag = Tag { signature };
                            self.result.module.num_imported_tags += 1;
                            EntityType::Tag(tag)
                        }
                    };
                    self.declare_import(import.module, import.name, ty);
                }
            }

            Payload::FunctionSection(functions) => {
                self.validator.function_section(&functions)?;

                let cnt = usize::try_from(functions.count()).unwrap();
                self.result.module.functions.reserve_exact(cnt);

                for entry in functions {
                    let sigindex = entry?;
                    let ty = TypeIndex::from_u32(sigindex);
                    let interned_index = self.result.module.types[ty];
                    self.result.module.push_function(interned_index);
                }
            }

            Payload::TableSection(tables) => {
                self.validator.table_section(&tables)?;
                let cnt = usize::try_from(tables.count()).unwrap();
                self.result.module.tables.reserve_exact(cnt);

                for entry in tables {
                    let wasmparser::Table { ty, init } = entry?;
                    let table = self.convert_table_type(&ty)?;
                    self.result.module.tables.push(table);
                    let init = match init {
                        wasmparser::TableInit::RefNull => TableInitialValue::Null {
                            precomputed: Vec::new(),
                        },
                        wasmparser::TableInit::Expr(expr) => {
                            let (init, escaped) = ConstExpr::from_wasmparser(expr)?;
                            for f in escaped {
                                self.flag_func_escaped(f);
                            }
                            TableInitialValue::Expr(init)
                        }
                    };
                    self.result
                        .module
                        .table_initialization
                        .initial_values
                        .push(init);
                }
            }

            Payload::MemorySection(memories) => {
                self.validator.memory_section(&memories)?;

                let cnt = usize::try_from(memories.count()).unwrap();
                self.result.module.memories.reserve_exact(cnt);

                for entry in memories {
                    let memory = entry?;
                    self.result.module.memories.push(memory.into());
                }
            }

            Payload::TagSection(tags) => {
                self.validator.tag_section(&tags)?;

                for entry in tags {
                    let sigindex = entry?.func_type_idx;
                    let ty = TypeIndex::from_u32(sigindex);
                    let interned_index = self.result.module.types[ty];
                    self.result.module.push_tag(interned_index);
                }
            }

            Payload::GlobalSection(globals) => {
                self.validator.global_section(&globals)?;

                let cnt = usize::try_from(globals.count()).unwrap();
                self.result.module.globals.reserve_exact(cnt);

                for entry in globals {
                    let wasmparser::Global { ty, init_expr } = entry?;
                    let (initializer, escaped) = ConstExpr::from_wasmparser(init_expr)?;
                    for f in escaped {
                        self.flag_func_escaped(f);
                    }
                    let ty = self.convert_global_type(&ty)?;
                    self.result.module.globals.push(ty);
                    self.result.module.global_initializers.push(initializer);
                }
            }

            Payload::ExportSection(exports) => {
                self.validator.export_section(&exports)?;

                let cnt = usize::try_from(exports.count()).unwrap();
                self.result.module.exports.reserve(cnt);

                for entry in exports {
                    let wasmparser::Export { name, kind, index } = entry?;
                    let entity = match kind {
                        ExternalKind::Func => {
                            let index = FuncIndex::from_u32(index);
                            self.flag_func_escaped(index);
                            EntityIndex::Function(index)
                        }
                        ExternalKind::Table => EntityIndex::Table(TableIndex::from_u32(index)),
                        ExternalKind::Memory => EntityIndex::Memory(MemoryIndex::from_u32(index)),
                        ExternalKind::Global => EntityIndex::Global(GlobalIndex::from_u32(index)),
                        ExternalKind::Tag => EntityIndex::Tag(TagIndex::from_u32(index)),
                    };
                    self.result
                        .module
                        .exports
                        .insert(String::from(name), entity);
                }
            }

            Payload::StartSection { func, range } => {
                self.validator.start_section(func, &range)?;

                let func_index = FuncIndex::from_u32(func);
                self.flag_func_escaped(func_index);
                debug_assert!(self.result.module.start_func.is_none());
                self.result.module.start_func = Some(func_index);
            }

            Payload::ElementSection(elements) => {
                self.validator.element_section(&elements)?;

                for (index, entry) in elements.into_iter().enumerate() {
                    let wasmparser::Element {
                        kind,
                        items,
                        range: _,
                    } = entry?;

                    // Build up a list of `FuncIndex` corresponding to all the
                    // entries listed in this segment. Note that it's not
                    // possible to create anything other than a `ref.null
                    // extern` for externref segments, so those just get
                    // translated to the reserved value of `FuncIndex`.
                    let elements = match items {
                        ElementItems::Functions(funcs) => {
                            let mut elems =
                                Vec::with_capacity(usize::try_from(funcs.count()).unwrap());
                            for func in funcs {
                                let func = FuncIndex::from_u32(func?);
                                self.flag_func_escaped(func);
                                elems.push(func);
                            }
                            TableSegmentElements::Functions(elems.into())
                        }
                        ElementItems::Expressions(_ty, items) => {
                            let mut exprs =
                                Vec::with_capacity(usize::try_from(items.count()).unwrap());
                            for expr in items {
                                let (expr, escaped) = ConstExpr::from_wasmparser(expr?)?;
                                exprs.push(expr);
                                for func in escaped {
                                    self.flag_func_escaped(func);
                                }
                            }
                            TableSegmentElements::Expressions(exprs.into())
                        }
                    };

                    match kind {
                        ElementKind::Active {
                            table_index,
                            offset_expr,
                        } => {
                            let table_index = TableIndex::from_u32(table_index.unwrap_or(0));
                            let (offset, escaped) = ConstExpr::from_wasmparser(offset_expr)?;
                            debug_assert!(escaped.is_empty());

                            self.result
                                .module
                                .table_initialization
                                .segments
                                .push(TableSegment {
                                    table_index,
                                    offset,
                                    elements,
                                });
                        }

                        ElementKind::Passive => {
                            let elem_index = ElemIndex::from_u32(index as u32);
                            let index = self.result.module.passive_elements.len();
                            self.result.module.passive_elements.push(elements);
                            self.result
                                .module
                                .passive_elements_map
                                .insert(elem_index, index);
                        }

                        ElementKind::Declared => {}
                    }
                }
            }

            Payload::CodeSectionStart { count, range, .. } => {
                self.validator.code_section_start(&range)?;
                let cnt = usize::try_from(count).unwrap();
                self.result.function_body_inputs.reserve_exact(cnt);
                self.result.debuginfo.wasm_file.code_section_offset = range.start as u64;
            }

            Payload::CodeSectionEntry(body) => {
                let validator = self.validator.code_section_entry(&body)?;
                let func_index =
                    self.result.code_index + self.result.module.num_imported_funcs as u32;
                let func_index = FuncIndex::from_u32(func_index);

                if self.tunables.generate_native_debuginfo {
                    let sig_index = self.result.module.functions[func_index]
                        .signature
                        .unwrap_module_type_index();
                    let sig = self.types[sig_index].unwrap_func();
                    let mut locals = Vec::new();
                    for pair in body.get_locals_reader()? {
                        let (cnt, ty) = pair?;
                        let ty = self.convert_valtype(ty)?;
                        locals.push((cnt, ty));
                    }
                    self.result
                        .debuginfo
                        .wasm_file
                        .funcs
                        .push(FunctionMetadata {
                            locals: locals.into_boxed_slice(),
                            params: sig.params().into(),
                        });
                }
                self.result
                    .function_body_inputs
                    .push(FunctionBodyData { validator, body });
                self.result.code_index += 1;
            }

            Payload::DataSection(data) => {
                self.validator.data_section(&data)?;

                let initializers = match &mut self.result.module.memory_initialization {
                    MemoryInitialization::Segmented(i) => i,
                    _ => unreachable!(),
                };

                let cnt = usize::try_from(data.count()).unwrap();
                initializers.reserve_exact(cnt);
                self.result.data.reserve_exact(cnt);

                for (index, entry) in data.into_iter().enumerate() {
                    let wasmparser::Data {
                        kind,
                        data,
                        range: _,
                    } = entry?;
                    let mk_range = |total: &mut u32| -> Result<_, WasmError> {
                        let range = u32::try_from(data.len())
                            .ok()
                            .and_then(|size| {
                                let start = *total;
                                let end = start.checked_add(size)?;
                                Some(start..end)
                            })
                            .ok_or_else(|| {
                                WasmError::Unsupported(format!(
                                    "more than 4 gigabytes of data in wasm module",
                                ))
                            })?;
                        *total += range.end - range.start;
                        Ok(range)
                    };
                    match kind {
                        DataKind::Active {
                            memory_index,
                            offset_expr,
                        } => {
                            let range = mk_range(&mut self.result.total_data)?;
                            let memory_index = MemoryIndex::from_u32(memory_index);
                            let (offset, escaped) = ConstExpr::from_wasmparser(offset_expr)?;
                            debug_assert!(escaped.is_empty());

                            initializers.push(MemoryInitializer {
                                memory_index,
                                offset,
                                data: range,
                            });
                            self.result.data.push(data.into());
                        }
                        DataKind::Passive => {
                            let data_index = DataIndex::from_u32(index as u32);
                            let range = mk_range(&mut self.result.total_passive_data)?;
                            self.result.passive_data.push(data);
                            self.result
                                .module
                                .passive_data_map
                                .insert(data_index, range);
                        }
                    }
                }
            }

            Payload::DataCountSection { count, range } => {
                self.validator.data_count_section(count, &range)?;

                // Note: the count passed in here is the *total* segment count
                // There is no way to reserve for just the passive segments as
                // they are discovered when iterating the data section entries
                // Given that the total segment count might be much larger than
                // the passive count, do not reserve anything here.
            }

            Payload::CustomSection(s)
                if s.name() == "webidl-bindings" || s.name() == "wasm-interface-types" =>
            {
                bail!(
                    "\
Support for interface types has temporarily been removed from `wasmtime`.

For more information about this temporary change you can read on the issue online:

    https://github.com/bytecodealliance/wasmtime/issues/1271

and for re-adding support for interface types you can see this issue:

    https://github.com/bytecodealliance/wasmtime/issues/677
"
                )
            }

            Payload::CustomSection(s) => {
                self.register_custom_section(&s);
            }

            // It's expected that validation will probably reject other
            // payloads such as `UnknownSection` or those related to the
            // component model. If, however, something gets past validation then
            // that's a bug in Wasmtime as we forgot to implement something.
            other => {
                self.validator.payload(&other)?;
                panic!("unimplemented section in wasm file {other:?}");
            }
        }
        Ok(())
    }

    fn register_custom_section(&mut self, section: &CustomSectionReader<'data>) {
        match section.as_known() {
            KnownCustom::Name(name) => {
                let result = self.name_section(name);
                if let Err(e) = result {
                    log::warn!("failed to parse name section {e:?}");
                }
            }
            _ => {
                let name = section.name().trim_end_matches(".dwo");
                if name.starts_with(".debug_") {
                    self.dwarf_section(name, section);
                }
            }
        }
    }

    fn dwarf_section(&mut self, name: &str, section: &CustomSectionReader<'data>) {
        if !self.tunables.generate_native_debuginfo && !self.tunables.parse_wasm_debuginfo {
            self.result.has_unparsed_debuginfo = true;
            return;
        }
        let info = &mut self.result.debuginfo;
        let dwarf = &mut info.dwarf;
        let endian = gimli::LittleEndian;
        let data = section.data();
        let slice = gimli::EndianSlice::new(data, endian);

        match name {
            // `gimli::Dwarf` fields.
            ".debug_abbrev" => dwarf.debug_abbrev = gimli::DebugAbbrev::new(data, endian),
            ".debug_addr" => dwarf.debug_addr = gimli::DebugAddr::from(slice),
            ".debug_info" => {
                dwarf.debug_info = gimli::DebugInfo::new(data, endian);
            }
            ".debug_line" => dwarf.debug_line = gimli::DebugLine::new(data, endian),
            ".debug_line_str" => dwarf.debug_line_str = gimli::DebugLineStr::from(slice),
            ".debug_str" => dwarf.debug_str = gimli::DebugStr::new(data, endian),
            ".debug_str_offsets" => dwarf.debug_str_offsets = gimli::DebugStrOffsets::from(slice),
            ".debug_str_sup" => {
                let mut dwarf_sup: Dwarf<'data> = Default::default();
                dwarf_sup.debug_str = gimli::DebugStr::from(slice);
                dwarf.sup = Some(Arc::new(dwarf_sup));
            }
            ".debug_types" => dwarf.debug_types = gimli::DebugTypes::from(slice),

            // Additional fields.
            ".debug_loc" => info.debug_loc = gimli::DebugLoc::from(slice),
            ".debug_loclists" => info.debug_loclists = gimli::DebugLocLists::from(slice),
            ".debug_ranges" => info.debug_ranges = gimli::DebugRanges::new(data, endian),
            ".debug_rnglists" => info.debug_rnglists = gimli::DebugRngLists::new(data, endian),

            // DWARF package fields
            ".debug_cu_index" => info.debug_cu_index = gimli::DebugCuIndex::new(data, endian),
            ".debug_tu_index" => info.debug_tu_index = gimli::DebugTuIndex::new(data, endian),

            // We don't use these at the moment.
            ".debug_aranges" | ".debug_pubnames" | ".debug_pubtypes" => return,
            other => {
                log::warn!("unknown debug section `{other}`");
                return;
            }
        }

        dwarf.ranges = gimli::RangeLists::new(info.debug_ranges, info.debug_rnglists);
        dwarf.locations = gimli::LocationLists::new(info.debug_loc, info.debug_loclists);
    }

    /// Declares a new import with the `module` and `field` names, importing the
    /// `ty` specified.
    ///
    /// Note that this method is somewhat tricky due to the implementation of
    /// the module linking proposal. In the module linking proposal two-level
    /// imports are recast as single-level imports of instances. That recasting
    /// happens here by recording an import of an instance for the first time
    /// we see a two-level import.
    ///
    /// When the module linking proposal is disabled, however, disregard this
    /// logic and instead work directly with two-level imports since no
    /// instances are defined.
    fn declare_import(&mut self, module: &'data str, field: &'data str, ty: EntityType) {
        let index = self.push_type(ty);
        self.result.module.initializers.push(Initializer::Import {
            name: module.to_owned(),
            field: field.to_owned(),
            index,
        });
    }

    fn push_type(&mut self, ty: EntityType) -> EntityIndex {
        match ty {
            EntityType::Function(ty) => EntityIndex::Function({
                let func_index = self
                    .result
                    .module
                    .push_function(ty.unwrap_module_type_index());
                // Imported functions can escape; in fact, they've already done
                // so to get here.
                self.flag_func_escaped(func_index);
                func_index
            }),
            EntityType::Table(ty) => EntityIndex::Table(self.result.module.tables.push(ty)),
            EntityType::Memory(ty) => EntityIndex::Memory(self.result.module.memories.push(ty)),
            EntityType::Global(ty) => EntityIndex::Global(self.result.module.globals.push(ty)),
            EntityType::Tag(ty) => EntityIndex::Tag(self.result.module.tags.push(ty)),
        }
    }

    fn flag_func_escaped(&mut self, func: FuncIndex) {
        let ty = &mut self.result.module.functions[func];
        // If this was already assigned a funcref index no need to re-assign it.
        if ty.is_escaping() {
            return;
        }
        let index = self.result.module.num_escaped_funcs as u32;
        ty.func_ref = FuncRefIndex::from_u32(index);
        self.result.module.num_escaped_funcs += 1;
    }

    /// Parses the Name section of the wasm module.
    fn name_section(&mut self, names: NameSectionReader<'data>) -> WasmResult<()> {
        for subsection in names {
            match subsection? {
                wasmparser::Name::Function(names) => {
                    for name in names {
                        let Naming { index, name } = name?;
                        // Skip this naming if it's naming a function that
                        // doesn't actually exist.
                        if (index as usize) >= self.result.module.functions.len() {
                            continue;
                        }

                        // Store the name unconditionally, regardless of
                        // whether we're parsing debuginfo, since function
                        // names are almost always present in the
                        // final compilation artifact.
                        let index = FuncIndex::from_u32(index);
                        self.result
                            .debuginfo
                            .name_section
                            .func_names
                            .insert(index, name);
                    }
                }
                wasmparser::Name::Module { name, .. } => {
                    self.result.module.name = Some(name.to_string());
                    if self.tunables.generate_native_debuginfo {
                        self.result.debuginfo.name_section.module_name = Some(name);
                    }
                }
                wasmparser::Name::Local(reader) => {
                    if !self.tunables.generate_native_debuginfo {
                        continue;
                    }
                    for f in reader {
                        let f = f?;
                        // Skip this naming if it's naming a function that
                        // doesn't actually exist.
                        if (f.index as usize) >= self.result.module.functions.len() {
                            continue;
                        }
                        for name in f.names {
                            let Naming { index, name } = name?;

                            self.result
                                .debuginfo
                                .name_section
                                .locals_names
                                .entry(FuncIndex::from_u32(f.index))
                                .or_insert(HashMap::new())
                                .insert(index, name);
                        }
                    }
                }
                wasmparser::Name::Label(_)
                | wasmparser::Name::Type(_)
                | wasmparser::Name::Table(_)
                | wasmparser::Name::Global(_)
                | wasmparser::Name::Memory(_)
                | wasmparser::Name::Element(_)
                | wasmparser::Name::Data(_)
                | wasmparser::Name::Tag(_)
                | wasmparser::Name::Field(_)
                | wasmparser::Name::Unknown { .. } => {}
            }
        }
        Ok(())
    }
}

impl TypeConvert for ModuleEnvironment<'_, '_> {
    fn lookup_heap_type(&self, index: wasmparser::UnpackedIndex) -> WasmHeapType {
        WasmparserTypeConverter::new(&self.types, |idx| {
            self.result.module.types[idx].unwrap_module_type_index()
        })
        .lookup_heap_type(index)
    }

    fn lookup_type_index(&self, index: wasmparser::UnpackedIndex) -> EngineOrModuleTypeIndex {
        WasmparserTypeConverter::new(&self.types, |idx| {
            self.result.module.types[idx].unwrap_module_type_index()
        })
        .lookup_type_index(index)
    }
}

impl ModuleTranslation<'_> {
    /// Attempts to convert segmented memory initialization into static
    /// initialization for the module that this translation represents.
    ///
    /// If this module's memory initialization is not compatible with paged
    /// initialization then this won't change anything. Otherwise if it is
    /// compatible then the `memory_initialization` field will be updated.
    ///
    /// Takes a `page_size` argument in order to ensure that all
    /// initialization is page-aligned for mmap-ability, and
    /// `max_image_size_always_allowed` to control how we decide
    /// whether to use static init.
    ///
    /// We will try to avoid generating very sparse images, which are
    /// possible if e.g. a module has an initializer at offset 0 and a
    /// very high offset (say, 1 GiB). To avoid this, we use a dual
    /// condition: we always allow images less than
    /// `max_image_size_always_allowed`, and the embedder of Wasmtime
    /// can set this if desired to ensure that static init should
    /// always be done if the size of the module or its heaps is
    /// otherwise bounded by the system. We also allow images with
    /// static init data bigger than that, but only if it is "dense",
    /// defined as having at least half (50%) of its pages with some
    /// data.
    ///
    /// We could do something slightly better by building a dense part
    /// and keeping a sparse list of outlier/leftover segments (see
    /// issue #3820). This would also allow mostly-static init of
    /// modules that have some dynamically-placed data segments. But,
    /// for now, this is sufficient to allow a system that "knows what
    /// it's doing" to always get static init.
    pub fn try_static_init(&mut self, page_size: u64, max_image_size_always_allowed: u64) {
        // This method only attempts to transform a `Segmented` memory init
        // into a `Static` one, no other state.
        if !self.module.memory_initialization.is_segmented() {
            return;
        }

        // First a dry run of memory initialization is performed. This
        // collects information about the extent of memory initialized for each
        // memory as well as the size of all data segments being copied in.
        struct Memory {
            data_size: u64,
            min_addr: u64,
            max_addr: u64,
            // The `usize` here is a pointer into `self.data` which is the list
            // of data segments corresponding to what was found in the original
            // wasm module.
            segments: Vec<(usize, StaticMemoryInitializer)>,
        }
        let mut info = PrimaryMap::with_capacity(self.module.memories.len());
        for _ in 0..self.module.memories.len() {
            info.push(Memory {
                data_size: 0,
                min_addr: u64::MAX,
                max_addr: 0,
                segments: Vec::new(),
            });
        }

        struct InitMemoryAtCompileTime<'a> {
            module: &'a Module,
            info: &'a mut PrimaryMap<MemoryIndex, Memory>,
            idx: usize,
        }
        impl InitMemory for InitMemoryAtCompileTime<'_> {
            fn memory_size_in_bytes(
                &mut self,
                memory_index: MemoryIndex,
            ) -> Result<u64, SizeOverflow> {
                self.module.memories[memory_index].minimum_byte_size()
            }

            fn eval_offset(&mut self, memory_index: MemoryIndex, expr: &ConstExpr) -> Option<u64> {
                match (expr.ops(), self.module.memories[memory_index].idx_type) {
                    (&[ConstOp::I32Const(offset)], IndexType::I32) => {
                        Some(offset.unsigned().into())
                    }
                    (&[ConstOp::I64Const(offset)], IndexType::I64) => Some(offset.unsigned()),
                    _ => None,
                }
            }

            fn write(&mut self, memory: MemoryIndex, init: &StaticMemoryInitializer) -> bool {
                // Currently `Static` only applies to locally-defined memories,
                // so if a data segment references an imported memory then
                // transitioning to a `Static` memory initializer is not
                // possible.
                if self.module.defined_memory_index(memory).is_none() {
                    return false;
                };
                let info = &mut self.info[memory];
                let data_len = u64::from(init.data.end - init.data.start);
                if data_len > 0 {
                    info.data_size += data_len;
                    info.min_addr = info.min_addr.min(init.offset);
                    info.max_addr = info.max_addr.max(init.offset + data_len);
                    info.segments.push((self.idx, init.clone()));
                }
                self.idx += 1;
                true
            }
        }
        let ok = self
            .module
            .memory_initialization
            .init_memory(&mut InitMemoryAtCompileTime {
                idx: 0,
                module: &self.module,
                info: &mut info,
            });
        if !ok {
            return;
        }

        // Validate that the memory information collected is indeed valid for
        // static memory initialization.
        for (i, info) in info.iter().filter(|(_, info)| info.data_size > 0) {
            let image_size = info.max_addr - info.min_addr;

            // Simplify things for now by bailing out entirely if any memory has
            // a page size smaller than the host's page size. This fixes a case
            // where currently initializers are created in host-page-size units
            // of length which means that a larger-than-the-entire-memory
            // initializer can be created. This can be handled technically but
            // would require some more changes to help fix the assert elsewhere
            // that this protects against.
            if self.module.memories[i].page_size() < page_size {
                return;
            }

            // If the range of memory being initialized is less than twice the
            // total size of the data itself then it's assumed that static
            // initialization is ok. This means we'll at most double memory
            // consumption during the memory image creation process, which is
            // currently assumed to "probably be ok" but this will likely need
            // tweaks over time.
            if image_size < info.data_size.saturating_mul(2) {
                continue;
            }

            // If the memory initialization image is larger than the size of all
            // data, then we still allow memory initialization if the image will
            // be of a relatively modest size, such as 1MB here.
            if image_size < max_image_size_always_allowed {
                continue;
            }

            // At this point memory initialization is concluded to be too
            // expensive to do at compile time so it's entirely deferred to
            // happen at runtime.
            return;
        }

        // Here's where we've now committed to changing to static memory. The
        // memory initialization image is built here from the page data and then
        // it's converted to a single initializer.
        let data = mem::replace(&mut self.data, Vec::new());
        let mut map = PrimaryMap::with_capacity(info.len());
        let mut module_data_size = 0u32;
        for (memory, info) in info.iter() {
            // Create the in-memory `image` which is the initialized contents of
            // this linear memory.
            let extent = if info.segments.len() > 0 {
                (info.max_addr - info.min_addr) as usize
            } else {
                0
            };
            let mut image = Vec::with_capacity(extent);
            for (idx, init) in info.segments.iter() {
                let data = &data[*idx];
                assert_eq!(data.len(), init.data.len());
                let offset = usize::try_from(init.offset - info.min_addr).unwrap();
                if image.len() < offset {
                    image.resize(offset, 0u8);
                    image.extend_from_slice(data);
                } else {
                    image.splice(
                        offset..(offset + data.len()).min(image.len()),
                        data.iter().copied(),
                    );
                }
            }
            assert_eq!(image.len(), extent);
            assert_eq!(image.capacity(), extent);
            let mut offset = if info.segments.len() > 0 {
                info.min_addr
            } else {
                0
            };

            // Chop off trailing zeros from the image as memory is already
            // zero-initialized. Note that `i` is the position of a nonzero
            // entry here, so to not lose it we truncate to `i + 1`.
            if let Some(i) = image.iter().rposition(|i| *i != 0) {
                image.truncate(i + 1);
            }

            // Also chop off leading zeros, if any.
            if let Some(i) = image.iter().position(|i| *i != 0) {
                offset += i as u64;
                image.drain(..i);
            }
            let mut len = u64::try_from(image.len()).unwrap();

            // The goal is to enable mapping this image directly into memory, so
            // the offset into linear memory must be a multiple of the page
            // size. If that's not already the case then the image is padded at
            // the front and back with extra zeros as necessary
            if offset % page_size != 0 {
                let zero_padding = offset % page_size;
                self.data.push(vec![0; zero_padding as usize].into());
                offset -= zero_padding;
                len += zero_padding;
            }
            self.data.push(image.into());
            if len % page_size != 0 {
                let zero_padding = page_size - (len % page_size);
                self.data.push(vec![0; zero_padding as usize].into());
                len += zero_padding;
            }

            // Offset/length should now always be page-aligned.
            assert!(offset % page_size == 0);
            assert!(len % page_size == 0);

            // Create the `StaticMemoryInitializer` which describes this image,
            // only needed if the image is actually present and has a nonzero
            // length. The `offset` has been calculates above, originally
            // sourced from `info.min_addr`. The `data` field is the extent
            // within the final data segment we'll emit to an ELF image, which
            // is the concatenation of `self.data`, so here it's the size of
            // the section-so-far plus the current segment we're appending.
            let len = u32::try_from(len).unwrap();
            let init = if len > 0 {
                Some(StaticMemoryInitializer {
                    offset,
                    data: module_data_size..module_data_size + len,
                })
            } else {
                None
            };
            let idx = map.push(init);
            assert_eq!(idx, memory);
            module_data_size += len;
        }
        self.data_align = Some(page_size);
        self.module.memory_initialization = MemoryInitialization::Static { map };
    }

    /// Attempts to convert the module's table initializers to
    /// FuncTable form where possible. This enables lazy table
    /// initialization later by providing a one-to-one map of initial
    /// table values, without having to parse all segments.
    pub fn try_func_table_init(&mut self) {
        // This should be large enough to support very large Wasm
        // modules with huge funcref tables, but small enough to avoid
        // OOMs or DoS on truly sparse tables.
        const MAX_FUNC_TABLE_SIZE: u64 = 1024 * 1024;

        // First convert any element-initialized tables to images of just that
        // single function if the minimum size of the table allows doing so.
        for ((_, init), (_, table)) in self
            .module
            .table_initialization
            .initial_values
            .iter_mut()
            .zip(
                self.module
                    .tables
                    .iter()
                    .skip(self.module.num_imported_tables),
            )
        {
            let table_size = table.limits.min;
            if table_size > MAX_FUNC_TABLE_SIZE {
                continue;
            }
            if let TableInitialValue::Expr(expr) = init {
                if let [ConstOp::RefFunc(f)] = expr.ops() {
                    *init = TableInitialValue::Null {
                        precomputed: vec![*f; table_size as usize],
                    };
                }
            }
        }

        let mut segments = mem::take(&mut self.module.table_initialization.segments)
            .into_iter()
            .peekable();

        // The goal of this loop is to interpret a table segment and apply it
        // "statically" to a local table. This will iterate over segments and
        // apply them one-by-one to each table.
        //
        // If any segment can't be applied, however, then this loop exits and
        // all remaining segments are placed back into the segment list. This is
        // because segments are supposed to be initialized one-at-a-time which
        // means that intermediate state is visible with respect to traps. If
        // anything isn't statically known to not trap it's pessimistically
        // assumed to trap meaning all further segment initializers must be
        // applied manually at instantiation time.
        while let Some(segment) = segments.peek() {
            let defined_index = match self.module.defined_table_index(segment.table_index) {
                Some(index) => index,
                // Skip imported tables: we can't provide a preconstructed
                // table for them, because their values depend on the
                // imported table overlaid with whatever segments we have.
                None => break,
            };

            // If the base of this segment is dynamic, then we can't
            // include it in the statically-built array of initial
            // contents.
            let offset = match segment.offset.ops() {
                &[ConstOp::I32Const(offset)] => u64::from(offset.unsigned()),
                &[ConstOp::I64Const(offset)] => offset.unsigned(),
                _ => break,
            };

            // Get the end of this segment. If out-of-bounds, or too
            // large for our dense table representation, then skip the
            // segment.
            let top = match offset.checked_add(segment.elements.len()) {
                Some(top) => top,
                None => break,
            };
            let table_size = self.module.tables[segment.table_index].limits.min;
            if top > table_size || top > MAX_FUNC_TABLE_SIZE {
                break;
            }

            match self.module.tables[segment.table_index]
                .ref_type
                .heap_type
                .top()
            {
                WasmHeapTopType::Func => {}
                // If this is not a funcref table, then we can't support a
                // pre-computed table of function indices. Technically this
                // initializer won't trap so we could continue processing
                // segments, but that's left as a future optimization if
                // necessary.
                WasmHeapTopType::Any
                | WasmHeapTopType::Extern
                | WasmHeapTopType::Cont
                | WasmHeapTopType::Exn => break,
            }

            // Function indices can be optimized here, but fully general
            // expressions are deferred to get evaluated at runtime.
            let function_elements = match &segment.elements {
                TableSegmentElements::Functions(indices) => indices,
                TableSegmentElements::Expressions(_) => break,
            };

            let precomputed =
                match &mut self.module.table_initialization.initial_values[defined_index] {
                    TableInitialValue::Null { precomputed } => precomputed,

                    // If this table is still listed as an initial value here
                    // then that means the initial size of the table doesn't
                    // support a precomputed function list, so skip this.
                    // Technically this won't trap so it's possible to process
                    // further initializers, but that's left as a future
                    // optimization.
                    TableInitialValue::Expr(_) => break,
                };

            // At this point we're committing to pre-initializing the table
            // with the `segment` that's being iterated over. This segment is
            // applied to the `precomputed` list for the table by ensuring
            // it's large enough to hold the segment and then copying the
            // segment into the precomputed list.
            if precomputed.len() < top as usize {
                precomputed.resize(top as usize, FuncIndex::reserved_value());
            }
            let dst = &mut precomputed[offset as usize..top as usize];
            dst.copy_from_slice(&function_elements);

            // advance the iterator to see the next segment
            let _ = segments.next();
        }
        self.module.table_initialization.segments = segments.collect();
    }
}

//! Support for parsing and analyzing [dynamic
//! library](https://github.com/WebAssembly/tool-conventions/blob/main/DynamicLinking.md) modules.

use {
    anyhow::{bail, Context, Error, Result},
    std::{
        collections::{BTreeSet, HashMap, HashSet},
        fmt,
    },
    wasmparser::{
        Dylink0Subsection, ExternalKind, FuncType, KnownCustom, MemInfo, Parser, Payload, RefType,
        SymbolFlags, TableType, TypeRef, ValType,
    },
};

/// Represents a core Wasm value type (not including V128 or reference types, which are not yet supported)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

impl TryFrom<ValType> for ValueType {
    type Error = Error;

    fn try_from(value: ValType) -> Result<Self> {
        Ok(match value {
            ValType::I32 => Self::I32,
            ValType::I64 => Self::I64,
            ValType::F32 => Self::F32,
            ValType::F64 => Self::F64,
            _ => bail!("{value:?} not yet supported"),
        })
    }
}

impl From<ValueType> for wasm_encoder::ValType {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::I32 => Self::I32,
            ValueType::I64 => Self::I64,
            ValueType::F32 => Self::F32,
            ValueType::F64 => Self::F64,
        }
    }
}

/// Represents a core Wasm function type
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FunctionType {
    pub parameters: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

impl fmt::Display for FunctionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.parameters, self.results)
    }
}

impl TryFrom<&FuncType> for FunctionType {
    type Error = Error;

    fn try_from(value: &FuncType) -> Result<Self> {
        Ok(Self {
            parameters: value
                .params()
                .iter()
                .map(|&v| ValueType::try_from(v))
                .collect::<Result<_>>()?,
            results: value
                .results()
                .iter()
                .map(|&v| ValueType::try_from(v))
                .collect::<Result<_>>()?,
        })
    }
}

/// Represents a core Wasm global variable type
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GlobalType {
    pub ty: ValueType,
    pub mutable: bool,
    pub shared: bool,
}

impl fmt::Display for GlobalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.mutable {
            write!(f, "mut ")?;
        }
        write!(f, "{:?}", self.ty)
    }
}

/// Represents a core Wasm export or import type
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    Function(FunctionType),
    Global(GlobalType),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Function(ty) => write!(f, "function {ty}"),
            Self::Global(ty) => write!(f, "global {ty}"),
        }
    }
}

impl From<&Type> for wasm_encoder::ExportKind {
    fn from(value: &Type) -> Self {
        match value {
            Type::Function(_) => wasm_encoder::ExportKind::Func,
            Type::Global(_) => wasm_encoder::ExportKind::Global,
        }
    }
}

/// Represents a core Wasm import
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Import<'a> {
    pub module: &'a str,
    pub name: &'a str,
    pub ty: Type,
    pub flags: SymbolFlags,
}

/// Represents a core Wasm export
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExportKey<'a> {
    pub name: &'a str,
    pub ty: Type,
}

impl<'a> fmt::Display for ExportKey<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.ty)
    }
}

/// Represents a core Wasm export, including dylink.0 flags
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Export<'a> {
    pub key: ExportKey<'a>,
    pub flags: SymbolFlags,
}

/// Metadata extracted from a dynamic library module
#[derive(Debug)]
pub struct Metadata<'a> {
    /// The name of the module
    ///
    /// This is currently not part of the file itself and must be provided separately, but the plan is to add
    /// something like a `WASM_DYLINK_SO_NAME` field to the dynamic linking tool convention so we can parse it
    /// along with everything else.
    pub name: &'a str,

    /// Whether this module should be resolvable via `dlopen`
    pub dl_openable: bool,

    /// The `WASM_DYLINK_MEM_INFO` value (or all zeros if not found)
    pub mem_info: MemInfo,

    /// The `WASM_DYLINK_NEEDED` values, if any
    pub needed_libs: Vec<&'a str>,

    /// Whether this module exports `__wasm_apply_data_relocs`
    pub has_data_relocs: bool,

    /// Whether this module exports `__wasm_call_ctors`
    pub has_ctors: bool,

    /// Whether this module exports `_initialize`
    pub has_initialize: bool,

    /// Whether this module exports `_start`
    pub has_wasi_start: bool,

    /// Whether this module exports `__wasm_set_libraries`
    pub has_set_libraries: bool,

    /// Whether this module includes any `component-type*` custom sections which include exports
    pub has_component_exports: bool,

    /// Whether this module imports `__asyncify_state` or `__asyncify_data`, indicating that it is
    /// asyncified with `--pass-arg=asyncify-relocatable` option.
    pub is_asyncified: bool,

    /// The functions imported from the `env` module, if any
    pub env_imports: BTreeSet<(&'a str, (FunctionType, SymbolFlags))>,

    /// The memory addresses imported from `GOT.mem`, if any
    pub memory_address_imports: BTreeSet<&'a str>,

    /// The table addresses imported from `GOT.func`, if any
    pub table_address_imports: BTreeSet<&'a str>,

    /// The symbols exported by this module, if any
    pub exports: BTreeSet<Export<'a>>,

    /// The symbols imported by this module (and not accounted for in the above fields), if any
    pub imports: BTreeSet<Import<'a>>,
}

impl<'a> Metadata<'a> {
    /// Parse the specified module and extract its metadata.
    pub fn try_new(
        name: &'a str,
        dl_openable: bool,
        module: &'a [u8],
        adapter_names: &HashSet<&str>,
    ) -> Result<Self> {
        let bindgen = crate::metadata::decode(module)?.1;
        let has_component_exports = !bindgen.resolve.worlds[bindgen.world].exports.is_empty();

        let mut result = Self {
            name,
            dl_openable,
            mem_info: MemInfo {
                memory_size: 0,
                memory_alignment: 1,
                table_size: 0,
                table_alignment: 1,
            },
            needed_libs: Vec::new(),
            has_data_relocs: false,
            has_ctors: false,
            has_initialize: false,
            has_wasi_start: false,
            has_set_libraries: false,
            has_component_exports,
            is_asyncified: false,
            env_imports: BTreeSet::new(),
            memory_address_imports: BTreeSet::new(),
            table_address_imports: BTreeSet::new(),
            exports: BTreeSet::new(),
            imports: BTreeSet::new(),
        };
        let mut types = Vec::new();
        let mut function_types = Vec::new();
        let mut global_types = Vec::new();
        let mut import_info = HashMap::new();
        let mut export_info = HashMap::new();

        for payload in Parser::new(0).parse_all(module) {
            match payload? {
                Payload::CustomSection(section) => {
                    if let KnownCustom::Dylink0(reader) = section.as_known() {
                        for subsection in reader {
                            match subsection.context("failed to parse `dylink.0` subsection")? {
                                Dylink0Subsection::MemInfo(info) => result.mem_info = info,
                                Dylink0Subsection::Needed(needed) => {
                                    result.needed_libs = needed.clone()
                                }
                                Dylink0Subsection::ExportInfo(info) => {
                                    export_info
                                        .extend(info.iter().map(|info| (info.name, info.flags)));
                                }
                                Dylink0Subsection::ImportInfo(info) => {
                                    import_info.extend(
                                        info.iter()
                                            .map(|info| ((info.module, info.field), info.flags)),
                                    );
                                }
                                Dylink0Subsection::Unknown { ty, .. } => {
                                    bail!("unrecognized `dylink.0` subsection: {ty}")
                                }
                            }
                        }
                    }
                }

                Payload::TypeSection(reader) => {
                    types = reader
                        .into_iter_err_on_gc_types()
                        .collect::<Result<Vec<_>, _>>()?;
                }

                Payload::ImportSection(reader) => {
                    for import in reader {
                        let import = import?;

                        match import.ty {
                            TypeRef::Func(ty) => function_types.push(usize::try_from(ty).unwrap()),
                            TypeRef::Global(ty) => global_types.push(ty),
                            _ => (),
                        }

                        let type_error = || {
                            bail!(
                                "unexpected type for {}:{}: {:?}",
                                import.module,
                                import.name,
                                import.ty
                            )
                        };

                        match (import.module, import.name) {
                            ("env", "memory") => {
                                if !matches!(import.ty, TypeRef::Memory(_)) {
                                    return type_error();
                                }
                            }
                            ("env", "__asyncify_data" | "__asyncify_state") => {
                                result.is_asyncified = true;
                                if !matches!(
                                    import.ty,
                                    TypeRef::Global(wasmparser::GlobalType {
                                        content_type: ValType::I32,
                                        ..
                                    })
                                ) {
                                    return type_error();
                                }
                            }
                            ("env", "__memory_base" | "__table_base" | "__stack_pointer") => {
                                if !matches!(
                                    import.ty,
                                    TypeRef::Global(wasmparser::GlobalType {
                                        content_type: ValType::I32,
                                        ..
                                    })
                                ) {
                                    return type_error();
                                }
                            }
                            ("env", "__indirect_function_table") => {
                                if let TypeRef::Table(TableType {
                                    element_type,
                                    maximum: None,
                                    ..
                                }) = import.ty
                                {
                                    if element_type != RefType::FUNCREF {
                                        return type_error();
                                    }
                                } else {
                                    return type_error();
                                }
                            }
                            ("env", name) => {
                                if let TypeRef::Func(ty) = import.ty {
                                    result.env_imports.insert((
                                        name,
                                        (
                                            FunctionType::try_from(
                                                &types[usize::try_from(ty).unwrap()],
                                            )?,
                                            import_info
                                                .get(&("env", name))
                                                .copied()
                                                .unwrap_or_default(),
                                        ),
                                    ));
                                } else {
                                    return type_error();
                                }
                            }
                            ("GOT.mem", name) => {
                                if let TypeRef::Global(wasmparser::GlobalType {
                                    content_type: ValType::I32,
                                    ..
                                }) = import.ty
                                {
                                    match name {
                                        "__heap_base" | "__heap_end" => (),
                                        _ => {
                                            result.memory_address_imports.insert(name);
                                        }
                                    }
                                } else {
                                    return type_error();
                                }
                            }
                            ("GOT.func", name) => {
                                if let TypeRef::Global(wasmparser::GlobalType {
                                    content_type: ValType::I32,
                                    ..
                                }) = import.ty
                                {
                                    result.table_address_imports.insert(name);
                                } else {
                                    return type_error();
                                }
                            }
                            (module, name) if adapter_names.contains(module) => {
                                let ty = match import.ty {
                                    TypeRef::Global(wasmparser::GlobalType {
                                        content_type,
                                        mutable,
                                        shared,
                                    }) => Type::Global(GlobalType {
                                        ty: content_type.try_into()?,
                                        mutable,
                                        shared,
                                    }),
                                    TypeRef::Func(ty) => Type::Function(FunctionType::try_from(
                                        &types[usize::try_from(ty).unwrap()],
                                    )?),
                                    ty => {
                                        bail!("unsupported import kind for {module}.{name}: {ty:?}",)
                                    }
                                };
                                let flags = import_info
                                    .get(&(module, name))
                                    .copied()
                                    .unwrap_or_default();
                                result.imports.insert(Import {
                                    module,
                                    name,
                                    ty,
                                    flags,
                                });
                            }
                            _ => {
                                if !matches!(import.ty, TypeRef::Func(_) | TypeRef::Global(_)) {
                                    return type_error();
                                }
                            }
                        }
                    }
                }

                Payload::FunctionSection(reader) => {
                    for function in reader {
                        function_types.push(usize::try_from(function?).unwrap());
                    }
                }

                Payload::GlobalSection(reader) => {
                    for global in reader {
                        global_types.push(global?.ty);
                    }
                }

                Payload::ExportSection(reader) => {
                    for export in reader {
                        let export = export?;

                        match export.name {
                            "__wasm_apply_data_relocs" => result.has_data_relocs = true,
                            "__wasm_call_ctors" => result.has_ctors = true,
                            "_initialize" => result.has_initialize = true,
                            "_start" => result.has_wasi_start = true,
                            "__wasm_set_libraries" => result.has_set_libraries = true,
                            _ => {
                                let ty = match export.kind {
                                    ExternalKind::Func => Type::Function(FunctionType::try_from(
                                        &types[function_types
                                            [usize::try_from(export.index).unwrap()]],
                                    )?),
                                    ExternalKind::Global => {
                                        let ty =
                                            global_types[usize::try_from(export.index).unwrap()];
                                        Type::Global(GlobalType {
                                            ty: ValueType::try_from(ty.content_type)?,
                                            mutable: ty.mutable,
                                            shared: ty.shared,
                                        })
                                    }
                                    kind => {
                                        bail!(
                                            "unsupported export kind for {}: {kind:?}",
                                            export.name
                                        )
                                    }
                                };
                                let flags =
                                    export_info.get(&export.name).copied().unwrap_or_default();
                                result.exports.insert(Export {
                                    key: ExportKey {
                                        name: export.name,
                                        ty,
                                    },
                                    flags,
                                });
                            }
                        }
                    }
                }

                _ => {}
            }
        }

        Ok(result)
    }
}

use crate::metadata::{Bindgen, ModuleMetadata};
use anyhow::{bail, Context, Result};
use indexmap::{map::Entry, IndexMap, IndexSet};
use std::mem;
use wasmparser::names::{ComponentName, ComponentNameKind};
use wasmparser::{
    types::Types, Encoding, ExternalKind, FuncType, Parser, Payload, TypeRef, ValType,
    ValidPayload, Validator,
};
use wit_parser::{
    abi::{AbiVariant, WasmSignature, WasmType},
    Function, InterfaceId, PackageName, Resolve, TypeDefKind, TypeId, WorldId, WorldItem, WorldKey,
};

fn is_canonical_function(name: &str) -> bool {
    name.starts_with("cabi_") || name.starts_with("canonical_abi_")
}

fn wasm_sig_to_func_type(signature: WasmSignature) -> FuncType {
    fn from_wasm_type(ty: &WasmType) -> ValType {
        match ty {
            WasmType::I32 => ValType::I32,
            WasmType::I64 => ValType::I64,
            WasmType::F32 => ValType::F32,
            WasmType::F64 => ValType::F64,
            WasmType::Pointer => ValType::I32,
            WasmType::PointerOrI64 => ValType::I64,
            WasmType::Length => ValType::I32,
        }
    }

    FuncType::new(
        signature.params.iter().map(from_wasm_type),
        signature.results.iter().map(from_wasm_type),
    )
}

pub const MAIN_MODULE_IMPORT_NAME: &str = "__main_module__";

/// The module name used when a top-level function in a world is imported into a
/// core wasm module. Note that this is not a valid WIT identifier to avoid
/// clashes with valid WIT interfaces. This is also not empty because LLVM
/// interprets an empty module import string as "not specified" which means it
/// turns into `env`.
pub const BARE_FUNC_MODULE_NAME: &str = "$root";

pub const RESOURCE_DROP: &str = "[resource-drop]";
pub const RESOURCE_REP: &str = "[resource-rep]";
pub const RESOURCE_NEW: &str = "[resource-new]";

pub const POST_RETURN_PREFIX: &str = "cabi_post_";

/// Metadata about a validated module and what was found internally.
///
/// All imports to the module are described by the union of `required_imports`
/// and `adapters_required`.
///
/// This structure is created by the `validate_module` function.
pub struct ValidatedModule<'a> {
    /// The required imports into this module which are to be satisfied by
    /// imported component model instances.
    ///
    /// The key of this map is the name of the interface that the module imports
    /// from and the value is the set of functions required from that interface.
    /// This is used to generate an appropriate instance import in the generated
    /// component which imports only the set of required functions.
    pub required_imports: IndexMap<&'a str, RequiredImports>,

    /// This is the set of imports into the module which were not satisfied by
    /// imported interfaces but are required to be satisfied by adapter modules.
    ///
    /// The key of this map is the name of the adapter that was imported into
    /// the module and the value is a further map from function to function type
    /// as required by this module. This map is used to shrink adapter modules
    /// to the precise size required for this module by ensuring it doesn't
    /// export (and subsequently import) extraneous functions.
    pub adapters_required: IndexMap<&'a str, IndexMap<&'a str, FuncType>>,

    /// Resource-related functions required and imported which work over
    /// exported resources from the final component.
    ///
    /// Note that this is disjoint from `required_imports` which handles
    /// imported resources and this is only for exported resources. Exported
    /// resources still require intrinsics to be imported into the core module
    /// itself.
    pub required_resource_funcs: IndexMap<String, IndexMap<String, ResourceInfo>>,

    /// Whether or not this module exported a linear memory.
    pub has_memory: bool,

    /// Whether or not this module exported a `cabi_realloc` function.
    pub realloc: Option<&'a str>,

    /// Whether or not this module exported a `cabi_realloc_adapter` function.
    pub adapter_realloc: Option<&'a str>,

    /// The original metadata specified for this module.
    pub metadata: &'a ModuleMetadata,

    /// Post-return functions annotated with `cabi_post_*` in their function
    /// name.
    pub post_returns: IndexSet<String>,
}

#[derive(Default)]
pub struct RequiredImports {
    pub funcs: IndexSet<String>,
    pub resources: IndexSet<String>,
}

pub struct ResourceInfo {
    pub drop_import: Option<String>,
    pub new_import: Option<String>,
    pub rep_import: Option<String>,
    pub dtor_export: Option<String>,
    pub id: TypeId,
}

/// This function validates the following:
///
/// * The `bytes` represent a valid core WebAssembly module.
/// * The module's imports are all satisfied by the given `imports` interfaces
///   or the `adapters` set.
/// * The given default and exported interfaces are satisfied by the module's
///   exports.
///
/// The `ValidatedModule` return value contains the metadata which describes the
/// input module on success. This is then further used to generate a component
/// for this module.
pub fn validate_module<'a>(
    bytes: &'a [u8],
    metadata: &'a Bindgen,
    exports: &IndexSet<WorldKey>,
    adapters: &IndexSet<&str>,
) -> Result<ValidatedModule<'a>> {
    let mut validator = Validator::new();
    let mut types = None;
    let mut import_funcs = IndexMap::new();
    let mut export_funcs = IndexMap::new();
    let mut ret = ValidatedModule {
        required_imports: Default::default(),
        adapters_required: Default::default(),
        has_memory: false,
        realloc: None,
        adapter_realloc: None,
        metadata: &metadata.metadata,
        required_resource_funcs: Default::default(),
        post_returns: Default::default(),
    };

    for payload in Parser::new(0).parse_all(bytes) {
        let payload = payload?;
        if let ValidPayload::End(tys) = validator.payload(&payload)? {
            types = Some(tys);
            break;
        }

        match payload {
            Payload::Version { encoding, .. } if encoding != Encoding::Module => {
                bail!("data is not a WebAssembly module");
            }
            Payload::ImportSection(s) => {
                for import in s {
                    let import = import?;
                    match import.ty {
                        TypeRef::Func(ty) => {
                            let map = match import_funcs.entry(import.module) {
                                Entry::Occupied(e) => e.into_mut(),
                                Entry::Vacant(e) => e.insert(IndexMap::new()),
                            };

                            assert!(map.insert(import.name, ty).is_none());
                        }
                        _ => bail!("module is only allowed to import functions"),
                    }
                }
            }
            Payload::ExportSection(s) => {
                for export in s {
                    let export = export?;

                    match export.kind {
                        ExternalKind::Func => {
                            if is_canonical_function(export.name) {
                                // TODO: validate that the cabi_realloc
                                // function is [i32, i32, i32, i32] -> [i32]
                                if export.name == "cabi_realloc"
                                    || export.name == "canonical_abi_realloc"
                                {
                                    ret.realloc = Some(export.name);
                                }
                                if export.name == "cabi_realloc_adapter" {
                                    ret.adapter_realloc = Some(export.name);
                                }
                            }

                            assert!(export_funcs.insert(export.name, export.index).is_none())
                        }
                        ExternalKind::Memory => {
                            if export.name == "memory" {
                                ret.has_memory = true;
                            }
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }

    let types = types.unwrap();
    let world = &metadata.resolve.worlds[metadata.world];
    let mut exported_resource_funcs = Vec::new();

    for (name, funcs) in &import_funcs {
        // An empty module name is indicative of the top-level import namespace,
        // so look for top-level functions here.
        if *name == BARE_FUNC_MODULE_NAME {
            let required =
                validate_imports_top_level(&metadata.resolve, metadata.world, funcs, &types)?;
            let prev = ret.required_imports.insert(BARE_FUNC_MODULE_NAME, required);
            assert!(prev.is_none());
            continue;
        }

        if let Some(interface_name) = name.strip_prefix("[export]") {
            exported_resource_funcs.push((name, interface_name, &import_funcs[name]));
            continue;
        }

        match world.imports.get(&world_key(&metadata.resolve, name)) {
            Some(WorldItem::Interface(interface)) => {
                let required =
                    validate_imported_interface(&metadata.resolve, *interface, name, funcs, &types)
                        .with_context(|| format!("failed to validate import interface `{name}`"))?;
                let prev = ret.required_imports.insert(name, required);
                assert!(prev.is_none());
            }
            None if adapters.contains(name) => {
                let map = ret.adapters_required.entry(name).or_default();
                for (func, ty) in funcs {
                    let ty = types[types.core_type_at(*ty).unwrap_sub()].unwrap_func();
                    map.insert(func, ty.clone());
                }
            }
            Some(WorldItem::Function(_) | WorldItem::Type(_)) => {
                bail!("import `{}` is not an interface", name)
            }
            None => bail!("module requires an import interface named `{name}`"),
        }
    }

    for name in exports {
        validate_exported_item(
            &metadata.resolve,
            &world.exports[name],
            &metadata.resolve.name_world_key(name),
            &export_funcs,
            &types,
            &mut ret.post_returns,
            &mut ret.required_resource_funcs,
        )?;
    }

    for (name, interface_name, funcs) in exported_resource_funcs {
        let world_key = world_key(&metadata.resolve, interface_name);
        match world.exports.get(&world_key) {
            Some(WorldItem::Interface(i)) => {
                validate_exported_interface_resource_imports(
                    &metadata.resolve,
                    *i,
                    name,
                    funcs,
                    &types,
                    &mut ret.required_resource_funcs,
                )?;
            }
            _ => bail!("import from `{name}` does not correspond to exported interface"),
        }
    }

    Ok(ret)
}

fn validate_exported_interface_resource_imports<'a>(
    resolve: &Resolve,
    interface: InterfaceId,
    import_module: &str,
    funcs: &IndexMap<&'a str, u32>,
    types: &Types,
    required_resource_funcs: &mut IndexMap<String, IndexMap<String, ResourceInfo>>,
) -> Result<()> {
    let is_resource = |name: &str| match resolve.interfaces[interface].types.get(name) {
        Some(ty) => matches!(resolve.types[*ty].kind, TypeDefKind::Resource),
        None => false,
    };
    for (func_name, ty) in funcs {
        if valid_exported_resource_func(func_name, *ty, types, is_resource)?.is_none() {
            bail!("import of `{func_name}` is not a valid resource function");
        }
        let info = required_resource_funcs.get_mut(import_module).unwrap();
        if let Some(resource_name) = func_name.strip_prefix(RESOURCE_DROP) {
            info[resource_name].drop_import = Some(func_name.to_string());
            continue;
        }
        if let Some(resource_name) = func_name.strip_prefix(RESOURCE_NEW) {
            info[resource_name].new_import = Some(func_name.to_string());
            continue;
        }
        if let Some(resource_name) = func_name.strip_prefix(RESOURCE_REP) {
            info[resource_name].rep_import = Some(func_name.to_string());
            continue;
        }

        unreachable!();
    }
    Ok(())
}

/// Validation information from an "adapter module" which is distinct from a
/// "main module" validated above.
///
/// This is created by the `validate_adapter_module` function.
pub struct ValidatedAdapter<'a> {
    /// If specified this is the list of required imports from the original set
    /// of possible imports along with the set of functions required from each
    /// imported interface.
    pub required_imports: IndexMap<String, RequiredImports>,

    /// Resource-related functions required and imported which work over
    /// exported resources from the final component.
    ///
    /// Note that this is disjoint from `required_imports` which handles
    /// imported resources and this is only for exported resources. Exported
    /// resources still require intrinsics to be imported into the core module
    /// itself.
    pub required_resource_funcs: IndexMap<String, IndexMap<String, ResourceInfo>>,

    /// This is the module and field name of the memory import, if one is
    /// specified.
    ///
    /// Due to LLVM codegen this is typically `env::memory` as a totally separate
    /// import from the `required_import` above.
    pub needs_memory: Option<(String, String)>,

    /// Set of names required to be exported from the main module which are
    /// imported by this adapter through the `__main_module__` synthetic export.
    /// This is how the WASI adapter imports `_start`, for example.
    pub needs_core_exports: IndexSet<String>,

    /// Name of the exported function to use for the realloc canonical option
    /// for lowering imports.
    pub import_realloc: Option<String>,

    /// Same as `import_realloc`, but for exported interfaces.
    pub export_realloc: Option<String>,

    /// Metadata about the original adapter module.
    pub metadata: &'a ModuleMetadata,

    /// Post-return functions annotated with `cabi_post_*` in their function
    /// name.
    pub post_returns: IndexSet<String>,
}

/// This function will validate the `bytes` provided as a wasm adapter module.
/// Notably this will validate the wasm module itself in addition to ensuring
/// that it has the "shape" of an adapter module. Current constraints are:
///
/// * The adapter module can import only one memory
/// * The adapter module can only import from the name of `interface` specified,
///   and all function imports must match the `required` types which correspond
///   to the lowered types of the functions in `interface`.
///
/// The wasm module passed into this function is the output of the GC pass of an
/// adapter module's original source. This means that the adapter module is
/// already minimized and this is a double-check that the minimization pass
/// didn't accidentally break the wasm module.
///
/// If `is_library` is true, we waive some of the constraints described above,
/// allowing the module to import tables and globals, as well as import
/// functions at the world level, not just at the interface level.
pub fn validate_adapter_module<'a>(
    bytes: &[u8],
    resolve: &'a Resolve,
    world: WorldId,
    metadata: &'a ModuleMetadata,
    required_by_import: Option<&IndexMap<&str, FuncType>>,
    exports: &IndexSet<WorldKey>,
    is_library: bool,
) -> Result<ValidatedAdapter<'a>> {
    let mut validator = Validator::new();
    let mut import_funcs = IndexMap::new();
    let mut export_funcs = IndexMap::new();
    let mut types = None;
    let mut funcs = Vec::new();
    let mut ret = ValidatedAdapter {
        required_imports: Default::default(),
        required_resource_funcs: Default::default(),
        needs_memory: None,
        needs_core_exports: Default::default(),
        import_realloc: None,
        export_realloc: None,
        metadata,
        post_returns: Default::default(),
    };

    let mut cabi_realloc = None;
    for payload in Parser::new(0).parse_all(bytes) {
        let payload = payload?;
        match validator.payload(&payload)? {
            ValidPayload::End(tys) => {
                types = Some(tys);
                break;
            }
            ValidPayload::Func(validator, body) => {
                funcs.push((validator, body));
            }
            _ => {}
        }

        match payload {
            Payload::Version { encoding, .. } if encoding != Encoding::Module => {
                bail!("data is not a WebAssembly module");
            }

            Payload::ImportSection(s) => {
                for import in s {
                    let import = import?;
                    match import.ty {
                        TypeRef::Func(ty) => {
                            let map = match import_funcs.entry(import.module) {
                                Entry::Occupied(e) => e.into_mut(),
                                Entry::Vacant(e) => e.insert(IndexMap::new()),
                            };

                            assert!(map.insert(import.name, ty).is_none());
                        }

                        // A memory is allowed to be imported into the adapter
                        // module so that's skipped here
                        TypeRef::Memory(_) => {
                            ret.needs_memory =
                                Some((import.module.to_string(), import.name.to_string()));
                        }

                        TypeRef::Global(_) | TypeRef::Table(_) if is_library => (),

                        _ => {
                            bail!("adapter module is only allowed to import functions and memories")
                        }
                    }
                }
            }
            Payload::ExportSection(s) => {
                for export in s {
                    let export = export?;

                    match export.kind {
                        ExternalKind::Func => {
                            export_funcs.insert(export.name, export.index);
                            if export.name == "cabi_export_realloc" {
                                ret.export_realloc = Some(export.name.to_string());
                            }
                            if export.name == "cabi_import_realloc" {
                                ret.import_realloc = Some(export.name.to_string());
                            }
                            if export.name == "cabi_realloc" {
                                cabi_realloc = Some(export.name.to_string());
                            }
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }

    if is_library {
        // If we're looking at a library, it may only export the
        // `wit-bindgen`-generated `cabi_realloc` rather than the
        // `cabi_import_realloc` and `cabi_export_realloc` functions, so we'll
        // use whatever's available.
        ret.export_realloc = ret.export_realloc.or_else(|| cabi_realloc.clone());
        ret.import_realloc = ret.import_realloc.or_else(|| cabi_realloc);
    }

    let mut resources = Default::default();
    for (validator, body) in funcs {
        let mut validator = validator.into_validator(resources);
        validator.validate(&body)?;
        resources = validator.into_allocations();
    }

    let types = types.unwrap();
    let mut exported_resource_funcs = Vec::new();
    for (name, funcs) in &import_funcs {
        if *name == MAIN_MODULE_IMPORT_NAME {
            ret.needs_core_exports
                .extend(funcs.iter().map(|(name, _ty)| name.to_string()));
            continue;
        }

        // An empty module name is indicative of the top-level import namespace,
        // so look for top-level functions here.
        if *name == BARE_FUNC_MODULE_NAME {
            let required = validate_imports_top_level(resolve, world, funcs, &types)?;
            ret.required_imports
                .insert(BARE_FUNC_MODULE_NAME.to_string(), required);
            continue;
        }

        if let Some(interface_name) = name.strip_prefix("[export]") {
            exported_resource_funcs.push((name, interface_name, &import_funcs[name]));
            continue;
        }

        match resolve.worlds[world].imports.get(&world_key(resolve, name)) {
            Some(WorldItem::Interface(interface)) => {
                let required =
                    validate_imported_interface(resolve, *interface, name, funcs, &types)
                        .with_context(|| format!("failed to validate import interface `{name}`"))?;
                let prev = ret.required_imports.insert(name.to_string(), required);
                assert!(prev.is_none());
            }
            None | Some(WorldItem::Function(_) | WorldItem::Type(_)) => {
                if !is_library {
                    bail!(
                        "adapter module requires an import interface named `{}`",
                        name
                    )
                }
            }
        }
    }

    if let Some(required) = required_by_import {
        for (name, ty) in required {
            let idx = match export_funcs.get(name) {
                Some(idx) => *idx,
                None => bail!("adapter module did not export `{name}`"),
            };
            let id = types.core_function_at(idx);
            let actual = types[id].unwrap_func();
            validate_func_sig(name, ty, actual)?;
        }
    }

    let world = &resolve.worlds[world];

    for name in exports {
        validate_exported_item(
            resolve,
            &world.exports[name],
            &resolve.name_world_key(name),
            &export_funcs,
            &types,
            &mut ret.post_returns,
            &mut ret.required_resource_funcs,
        )?;
    }

    for (name, interface_name, funcs) in exported_resource_funcs {
        let world_key = world_key(resolve, interface_name);
        match world.exports.get(&world_key) {
            Some(WorldItem::Interface(i)) => {
                validate_exported_interface_resource_imports(
                    resolve,
                    *i,
                    name,
                    funcs,
                    &types,
                    &mut ret.required_resource_funcs,
                )?;
            }
            _ => bail!("import from `{name}` does not correspond to exported interface"),
        }
    }

    Ok(ret)
}

fn world_key(resolve: &Resolve, name: &str) -> WorldKey {
    let kebab_name = ComponentName::new(name, 0);
    let (pkgname, interface) = match kebab_name.as_ref().map(|k| k.kind()) {
        Ok(ComponentNameKind::Interface(name)) => {
            let pkgname = PackageName {
                namespace: name.namespace().to_string(),
                name: name.package().to_string(),
                version: name.version(),
            };
            (pkgname, name.interface().as_str())
        }
        _ => return WorldKey::Name(name.to_string()),
    };
    match resolve
        .package_names
        .get(&pkgname)
        .and_then(|p| resolve.packages[*p].interfaces.get(interface))
    {
        Some(id) => WorldKey::Interface(*id),
        None => WorldKey::Name(name.to_string()),
    }
}

fn validate_imports_top_level(
    resolve: &Resolve,
    world: WorldId,
    funcs: &IndexMap<&str, u32>,
    types: &Types,
) -> Result<RequiredImports> {
    let is_resource = |name: &str| match resolve.worlds[world]
        .imports
        .get(&WorldKey::Name(name.to_string()))
    {
        Some(WorldItem::Type(r)) => {
            matches!(resolve.types[*r].kind, TypeDefKind::Resource)
        }
        _ => false,
    };
    let mut required = RequiredImports::default();
    for (name, ty) in funcs {
        match resolve.worlds[world].imports.get(&world_key(resolve, name)) {
            Some(WorldItem::Function(func)) => {
                let ty = types[types.core_type_at(*ty).unwrap_sub()].unwrap_func();
                validate_func(resolve, ty, func, AbiVariant::GuestImport)?;
            }
            Some(_) => bail!("expected world top-level import `{name}` to be a function"),
            None => match valid_imported_resource_func(name, *ty, types, is_resource)? {
                Some(name) => {
                    required.resources.insert(name.to_string());
                }
                None => bail!("no top-level imported function `{name}` specified"),
            },
        }
        required.funcs.insert(name.to_string());
    }
    Ok(required)
}

fn valid_imported_resource_func<'a>(
    func_name: &'a str,
    ty: u32,
    types: &Types,
    is_resource: impl Fn(&str) -> bool,
) -> Result<Option<&'a str>> {
    if let Some(resource_name) = func_name.strip_prefix(RESOURCE_DROP) {
        if is_resource(resource_name) {
            let ty = types[types.core_type_at(ty).unwrap_sub()].unwrap_func();
            let expected = FuncType::new([ValType::I32], []);
            validate_func_sig(func_name, &expected, ty)?;
            return Ok(Some(resource_name));
        }
    }
    Ok(None)
}

fn valid_exported_resource_func<'a>(
    func_name: &'a str,
    ty: u32,
    types: &Types,
    is_resource: impl Fn(&str) -> bool,
) -> Result<Option<&'a str>> {
    if let Some(name) = valid_imported_resource_func(func_name, ty, types, &is_resource)? {
        return Ok(Some(name));
    }
    if let Some(resource_name) = func_name
        .strip_prefix(RESOURCE_REP)
        .or_else(|| func_name.strip_prefix(RESOURCE_NEW))
    {
        if is_resource(resource_name) {
            let ty = types[types.core_type_at(ty).unwrap_sub()].unwrap_func();
            let expected = FuncType::new([ValType::I32], [ValType::I32]);
            validate_func_sig(func_name, &expected, ty)?;
            return Ok(Some(resource_name));
        }
    }
    Ok(None)
}

fn validate_imported_interface(
    resolve: &Resolve,
    interface: InterfaceId,
    name: &str,
    imports: &IndexMap<&str, u32>,
    types: &Types,
) -> Result<RequiredImports> {
    let mut required = RequiredImports::default();
    let is_resource = |name: &str| {
        let ty = match resolve.interfaces[interface].types.get(name) {
            Some(ty) => *ty,
            None => return false,
        };
        matches!(resolve.types[ty].kind, TypeDefKind::Resource)
    };
    for (func_name, ty) in imports {
        match resolve.interfaces[interface].functions.get(*func_name) {
            Some(f) => {
                let ty = types[types.core_type_at(*ty).unwrap_sub()].unwrap_func();
                validate_func(resolve, ty, f, AbiVariant::GuestImport)?;
            }
            None => match valid_imported_resource_func(func_name, *ty, types, is_resource)? {
                Some(name) => {
                    required.resources.insert(name.to_string());
                }
                None => bail!(
                    "import interface `{name}` is missing function \
                         `{func_name}` that is required by the module",
                ),
            },
        }
        required.funcs.insert(func_name.to_string());
    }

    Ok(required)
}

fn validate_func(
    resolve: &Resolve,
    ty: &wasmparser::FuncType,
    func: &Function,
    abi: AbiVariant,
) -> Result<()> {
    validate_func_sig(
        &func.name,
        &wasm_sig_to_func_type(resolve.wasm_signature(abi, func)),
        ty,
    )
}

fn validate_post_return(
    resolve: &Resolve,
    ty: &wasmparser::FuncType,
    func: &Function,
) -> Result<()> {
    // The expected signature of a post-return function is to take all the
    // parameters that are returned by the guest function and then return no
    // results. Model this by calculating the signature of `func` and then
    // moving its results into the parameters list while emptying out the
    // results.
    let mut sig = resolve.wasm_signature(AbiVariant::GuestExport, func);
    sig.params = mem::take(&mut sig.results);
    validate_func_sig(
        &format!("{} post-return", func.name),
        &wasm_sig_to_func_type(sig),
        ty,
    )
}

fn validate_func_sig(name: &str, expected: &FuncType, ty: &wasmparser::FuncType) -> Result<()> {
    if ty != expected {
        bail!(
            "type mismatch for function `{}`: expected `{:?} -> {:?}` but found `{:?} -> {:?}`",
            name,
            expected.params(),
            expected.results(),
            ty.params(),
            ty.results()
        );
    }

    Ok(())
}

fn validate_exported_item<'a>(
    resolve: &'a Resolve,
    item: &'a WorldItem,
    export_name: &str,
    exports: &IndexMap<&str, u32>,
    types: &Types,
    post_returns: &mut IndexSet<String>,
    required_resource_funcs: &mut IndexMap<String, IndexMap<String, ResourceInfo>>,
) -> Result<()> {
    let mut validate = |func: &Function, name: Option<&str>| {
        let expected_export_name = func.core_export_name(name);
        let func_index = match exports.get(expected_export_name.as_ref()) {
            Some(func_index) => func_index,
            None => bail!(
                "module does not export required function `{}`",
                expected_export_name
            ),
        };
        let id = types.core_function_at(*func_index);
        let ty = types[id].unwrap_func();
        validate_func(resolve, ty, func, AbiVariant::GuestExport)?;

        let post_return = format!("{POST_RETURN_PREFIX}{expected_export_name}");
        if let Some(index) = exports.get(&post_return[..]) {
            let ok = post_returns.insert(post_return);
            assert!(ok);
            let id = types.core_function_at(*index);
            let ty = types[id].unwrap_func();
            validate_post_return(resolve, ty, func)?;
        }
        Ok(())
    };
    match item {
        WorldItem::Function(func) => validate(func, None)?,
        WorldItem::Interface(interface) => {
            let interface = &resolve.interfaces[*interface];
            for (_, f) in interface.functions.iter() {
                validate(f, Some(export_name)).with_context(|| {
                    format!("failed to validate exported interface `{export_name}`")
                })?;
            }
            let mut map = IndexMap::new();
            for (name, id) in interface.types.iter() {
                if !matches!(resolve.types[*id].kind, TypeDefKind::Resource) {
                    continue;
                }
                let mut info = ResourceInfo {
                    id: *id,
                    dtor_export: None,
                    drop_import: None,
                    rep_import: None,
                    new_import: None,
                };
                let dtor = format!("{export_name}#[dtor]{name}");
                if let Some((_, name, func_idx)) = exports.get_full(dtor.as_str()) {
                    let id = types.core_function_at(*func_idx);
                    let ty = types[id].unwrap_func();
                    let expected = FuncType::new([ValType::I32], []);
                    validate_func_sig(name, &expected, ty)?;
                    info.dtor_export = Some(name.to_string());
                }
                let prev = map.insert(name.to_string(), info);
                assert!(prev.is_none());
            }
            let prev = required_resource_funcs.insert(format!("[export]{export_name}"), map);
            assert!(prev.is_none());
        }
        // not required to have anything exported in the core wasm module
        WorldItem::Type(_) => {}
    }

    Ok(())
}

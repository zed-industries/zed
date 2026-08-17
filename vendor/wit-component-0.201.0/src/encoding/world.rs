use super::{Adapter, ComponentEncoder, LibraryInfo, RequiredOptions};
use crate::validation::{
    validate_adapter_module, validate_module, RequiredImports, ValidatedAdapter, ValidatedModule,
    BARE_FUNC_MODULE_NAME, RESOURCE_DROP,
};
use anyhow::{Context, Result};
use indexmap::{IndexMap, IndexSet};
use std::borrow::{Borrow, Cow};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use wasmparser::FuncType;
use wit_parser::{
    abi::{AbiVariant, WasmSignature, WasmType},
    Function, InterfaceId, LiveTypes, Resolve, TypeDefKind, TypeId, TypeOwner, WorldId, WorldItem,
    WorldKey,
};

pub struct WorldAdapter<'a> {
    pub wasm: Cow<'a, [u8]>,
    pub info: ValidatedAdapter<'a>,
    pub library_info: Option<&'a LibraryInfo>,
}

/// Metadata discovered from the state configured in a `ComponentEncoder`.
///
/// This is stored separately from `EncodingState` to be stored as a borrow in
/// `EncodingState` as this information doesn't change throughout the encoding
/// process.
pub struct ComponentWorld<'a> {
    /// Encoder configuration with modules, the document ,etc.
    pub encoder: &'a ComponentEncoder,
    /// Validation information of the input module, or `None` in `--types-only`
    /// mode.
    pub info: ValidatedModule<'a>,
    /// Validation information about adapters populated only for required
    /// adapters. Additionally stores the gc'd wasm for each adapter.
    pub adapters: IndexMap<&'a str, WorldAdapter<'a>>,
    /// Map of all imports and descriptions of what they're importing.
    pub import_map: IndexMap<Option<String>, ImportedInterface>,
    /// Set of all live types which must be exported either because they're
    /// directly used or because they're transitively used.
    pub live_type_imports: IndexMap<InterfaceId, IndexSet<TypeId>>,
    /// For each exported interface in the desired world this map lists
    /// the set of interfaces that it depends on which are also exported.
    ///
    /// This set is used to determine when types are imported/used whether they
    /// come from imports or exports.
    pub exports_used: HashMap<InterfaceId, HashSet<InterfaceId>>,
}

#[derive(Debug)]
pub struct ImportedInterface {
    pub lowerings: IndexMap<String, Lowering>,
    pub interface: Option<InterfaceId>,
}

#[derive(Debug)]
pub enum Lowering {
    Direct,
    Indirect {
        sig: WasmSignature,
        options: RequiredOptions,
    },
    ResourceDrop(TypeId),
}

impl<'a> ComponentWorld<'a> {
    pub fn new(encoder: &'a ComponentEncoder) -> Result<Self> {
        let adapters = encoder
            .adapters
            .keys()
            .map(|s| s.as_str())
            .collect::<IndexSet<_>>();
        let info = validate_module(
            &encoder.module,
            &encoder.metadata,
            &encoder.main_module_exports,
            &adapters,
        )?;

        let mut ret = ComponentWorld {
            encoder,
            info,
            adapters: IndexMap::new(),
            import_map: IndexMap::new(),
            live_type_imports: Default::default(),
            exports_used: HashMap::new(),
        };

        ret.process_adapters()?;
        ret.process_imports()?;
        ret.process_exports_used();
        ret.process_live_type_imports();

        Ok(ret)
    }

    /// Process adapters which are required here. Iterate over all
    /// adapters and figure out what functions are required from the
    /// adapter itself, either because the functions are imported by the
    /// main module or they're part of the adapter's exports.
    fn process_adapters(&mut self) -> Result<()> {
        let resolve = &self.encoder.metadata.resolve;
        let world = self.encoder.metadata.world;
        for (
            name,
            Adapter {
                wasm,
                metadata,
                required_exports,
                library_info,
            },
        ) in self.encoder.adapters.iter()
        {
            let required_by_import = self.info.adapters_required.get(name.as_str());
            let no_required_by_import = || required_by_import.map(|m| m.is_empty()).unwrap_or(true);
            let no_required_exports = || {
                required_exports
                    .iter()
                    .all(|name| match &resolve.worlds[world].exports[name] {
                        WorldItem::Function(_) => false,
                        WorldItem::Interface(id) => resolve.interfaces[*id].functions.is_empty(),
                        WorldItem::Type(_) => true,
                    })
            };
            if no_required_by_import() && no_required_exports() && library_info.is_none() {
                continue;
            }
            let wasm = if library_info.is_some() {
                Cow::Borrowed(wasm as &[u8])
            } else {
                let required = self.required_adapter_exports(
                    resolve,
                    world,
                    required_exports,
                    required_by_import,
                );

                Cow::Owned(
                    crate::gc::run(
                        wasm,
                        &required,
                        if self.encoder.realloc_via_memory_grow {
                            None
                        } else {
                            self.info.realloc
                        },
                    )
                    .context("failed to reduce input adapter module to its minimal size")?,
                )
            };
            let info = validate_adapter_module(
                &wasm,
                resolve,
                world,
                metadata,
                required_by_import,
                required_exports,
                library_info.is_some(),
            )
            .context("failed to validate the imports of the minimized adapter module")?;
            self.adapters.insert(
                name,
                WorldAdapter {
                    info,
                    wasm,
                    library_info: library_info.as_ref(),
                },
            );
        }
        Ok(())
    }

    /// Returns the set of functions required to be exported from an adapter,
    /// either because they're exported from the adapter's world or because
    /// they're required as an import to the main module.
    fn required_adapter_exports<'r>(
        &self,
        resolve: &'r Resolve,
        world: WorldId,
        required_exports: &IndexSet<WorldKey>,
        required_by_import: Option<&IndexMap<&str, FuncType>>,
    ) -> IndexMap<String, (FuncType, Option<&'r Function>)> {
        use wasmparser::ValType;

        let mut required = IndexMap::new();
        if let Some(imports) = required_by_import {
            for (name, ty) in imports {
                required.insert(name.to_string(), (ty.clone(), None));
            }
        }
        let mut add_func = |func: &'r Function, name: Option<&str>| {
            let name = func.core_export_name(name);
            let ty = resolve.wasm_signature(AbiVariant::GuestExport, func);
            let prev = required.insert(
                name.into_owned(),
                (
                    wasmparser::FuncType::new(
                        ty.params.iter().map(to_valty),
                        ty.results.iter().map(to_valty),
                    ),
                    Some(func),
                ),
            );
            assert!(prev.is_none());
        };
        for name in required_exports {
            match &resolve.worlds[world].exports[name] {
                WorldItem::Function(func) => add_func(func, None),
                WorldItem::Interface(id) => {
                    let name = resolve.name_world_key(name);
                    for (_, func) in resolve.interfaces[*id].functions.iter() {
                        add_func(func, Some(&name));
                    }
                }
                WorldItem::Type(_) => {}
            }
        }
        return required;

        fn to_valty(ty: &WasmType) -> ValType {
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
    }

    /// Fills out the `import_map` field of `self` by determining the live
    /// functions from all imports. This additionally classifies imported
    /// functions into direct or indirect lowerings for managing shims.
    fn process_imports(&mut self) -> Result<()> {
        let resolve = &self.encoder.metadata.resolve;
        let world = self.encoder.metadata.world;
        let mut all_required_imports = IndexMap::new();
        for map in self.adapters.values().map(|a| &a.info.required_imports) {
            for (k, v) in map {
                all_required_imports
                    .entry(k.as_str())
                    .or_insert_with(IndexSet::new)
                    .extend(v.funcs.iter().map(|v| v.as_str()));
            }
        }
        for (k, v) in self.info.required_imports.iter() {
            all_required_imports
                .entry(*k)
                .or_insert_with(IndexSet::new)
                .extend(v.funcs.iter().map(|v| v.as_str()));
        }
        for (name, item) in resolve.worlds[world].imports.iter() {
            add_item(
                &mut self.import_map,
                resolve,
                name,
                item,
                &all_required_imports,
            )?;
        }
        return Ok(());

        fn add_item(
            import_map: &mut IndexMap<Option<String>, ImportedInterface>,
            resolve: &Resolve,
            name: &WorldKey,
            item: &WorldItem,
            required: &IndexMap<&str, IndexSet<&str>>,
        ) -> Result<()> {
            let name = resolve.name_world_key(name);
            log::trace!("register import `{name}`");
            let empty = IndexSet::new();
            let import_map_key = match item {
                WorldItem::Function(_) | WorldItem::Type(_) => None,
                WorldItem::Interface(_) => Some(name),
            };
            let interface_id = match item {
                WorldItem::Function(_) | WorldItem::Type(_) => None,
                WorldItem::Interface(id) => Some(*id),
            };
            let required = required
                .get(import_map_key.as_deref().unwrap_or(BARE_FUNC_MODULE_NAME))
                .unwrap_or(&empty);
            let interface = import_map
                .entry(import_map_key)
                .or_insert_with(|| ImportedInterface {
                    interface: interface_id,
                    lowerings: Default::default(),
                });
            assert_eq!(interface.interface, interface_id);
            match item {
                WorldItem::Function(func) => {
                    interface.add_func(required, resolve, func);
                }
                WorldItem::Type(ty) => {
                    interface.add_type(required, resolve, *ty);
                }
                WorldItem::Interface(id) => {
                    for (_name, ty) in resolve.interfaces[*id].types.iter() {
                        interface.add_type(required, resolve, *ty);
                    }
                    for (_name, func) in resolve.interfaces[*id].functions.iter() {
                        interface.add_func(required, resolve, func);
                    }
                }
            }
            Ok(())
        }
    }

    /// Determines the set of live imported types which are required to satisfy
    /// the imports and exports of the lifted core module.
    fn process_live_type_imports(&mut self) {
        let mut live = LiveTypes::default();
        let resolve = &self.encoder.metadata.resolve;
        let world = self.encoder.metadata.world;

        // First use the previously calculated metadata about live imports to
        // determine the set of live types in those imports.
        self.add_live_imports(world, &self.info.required_imports, &mut live);
        for (adapter_name, adapter) in self.adapters.iter() {
            log::trace!("processing adapter `{adapter_name}`");
            self.add_live_imports(world, &adapter.info.required_imports, &mut live);
        }

        // Next any imported types used by an export must also be considered
        // live. This is a little tricky though because interfaces can be both
        // imported and exported, so it's not as simple as registering the
        // entire export's set of types and their transitive references
        // (otherwise if you only export an interface it would consider those
        // types imports live too).
        //
        // Here if the export is an interface the set of live types for that
        // interface is calculated separately. The `exports_used` field
        // previously calculated is then consulted to add any types owned by
        // interfaces not in the `exports_used` set to the live imported types
        // set. This means that only types not defined by referenced exports
        // will get added here.
        for (name, item) in resolve.worlds[world].exports.iter() {
            log::trace!("add live world export `{}`", resolve.name_world_key(name));
            let id = match item {
                WorldItem::Interface(id) => id,
                WorldItem::Function(_) | WorldItem::Type(_) => {
                    live.add_world_item(resolve, item);
                    continue;
                }
            };

            let exports_used = &self.exports_used[id];
            let mut live_from_export = LiveTypes::default();
            live_from_export.add_world_item(resolve, item);
            for ty in live_from_export.iter() {
                let owner = match resolve.types[ty].owner {
                    TypeOwner::Interface(id) => id,
                    _ => continue,
                };
                if owner != *id && !exports_used.contains(&owner) {
                    live.add_type_id(resolve, ty);
                }
            }
        }

        for live in live.iter() {
            let owner = match resolve.types[live].owner {
                TypeOwner::Interface(id) => id,
                _ => continue,
            };
            self.live_type_imports
                .entry(owner)
                .or_insert(Default::default())
                .insert(live);
        }
    }

    fn add_live_imports<S>(
        &self,
        world: WorldId,
        required: &IndexMap<S, RequiredImports>,
        live: &mut LiveTypes,
    ) where
        S: Borrow<str> + Hash + Eq,
    {
        let resolve = &self.encoder.metadata.resolve;
        for (name, item) in resolve.worlds[world].imports.iter() {
            let name = resolve.name_world_key(name);
            match item {
                WorldItem::Function(func) => {
                    let required = match required.get(BARE_FUNC_MODULE_NAME) {
                        Some(set) => set,
                        None => continue,
                    };
                    if !required.funcs.contains(name.as_str()) {
                        continue;
                    }
                    log::trace!("add live function import `{name}`");
                    live.add_func(resolve, func);
                }
                WorldItem::Interface(id) => {
                    let required = match required.get(name.as_str()) {
                        Some(set) => set,
                        None => continue,
                    };
                    log::trace!("add live interface import `{name}`");
                    for (name, func) in resolve.interfaces[*id].functions.iter() {
                        if required.funcs.contains(name.as_str()) {
                            log::trace!("add live func `{name}`");
                            live.add_func(resolve, func);
                        }
                    }
                    for (name, ty) in resolve.interfaces[*id].types.iter() {
                        if required.resources.contains(name.as_str()) {
                            live.add_type_id(resolve, *ty);
                        }
                    }
                }
                WorldItem::Type(id) => live.add_type_id(resolve, *id),
            }
        }
    }

    fn process_exports_used(&mut self) {
        let resolve = &self.encoder.metadata.resolve;
        let world = self.encoder.metadata.world;

        let exports = &resolve.worlds[world].exports;
        for (_name, item) in exports.iter() {
            let id = match item {
                WorldItem::Function(_) => continue,
                WorldItem::Interface(id) => *id,
                WorldItem::Type(_) => unreachable!(),
            };
            let mut set = HashSet::new();

            for other in resolve.interface_direct_deps(id) {
                // If this dependency is not exported, then it'll show up
                // through an import, so we're not interested in it.
                if !exports.contains_key(&WorldKey::Interface(other)) {
                    continue;
                }

                // Otherwise this is a new exported dependency of ours, and
                // additionally this interface inherits all the transitive
                // dependencies too.
                if set.insert(other) {
                    set.extend(self.exports_used[&other].iter().copied());
                }
            }
            let prev = self.exports_used.insert(id, set);
            assert!(prev.is_none());
        }
    }
}

impl ImportedInterface {
    fn add_func(&mut self, required: &IndexSet<&str>, resolve: &Resolve, func: &Function) {
        if !required.contains(func.name.as_str()) {
            return;
        }
        log::trace!("add func {}", func.name);
        let options = RequiredOptions::for_import(resolve, func);
        let lowering = if options.is_empty() {
            Lowering::Direct
        } else {
            let sig = resolve.wasm_signature(AbiVariant::GuestImport, func);
            Lowering::Indirect { sig, options }
        };

        let prev = self.lowerings.insert(func.name.clone(), lowering);
        assert!(prev.is_none());
    }

    fn add_type(&mut self, required: &IndexSet<&str>, resolve: &Resolve, id: TypeId) {
        let ty = &resolve.types[id];
        match &ty.kind {
            TypeDefKind::Resource => {}
            _ => return,
        }
        let name = ty.name.as_deref().expect("resources must be named");

        let mut maybe_add = |name: String, lowering: Lowering| {
            if !required.contains(name.as_str()) {
                return;
            }
            let prev = self.lowerings.insert(name, lowering);
            assert!(prev.is_none());
        };
        maybe_add(format!("{RESOURCE_DROP}{name}"), Lowering::ResourceDrop(id));
    }
}

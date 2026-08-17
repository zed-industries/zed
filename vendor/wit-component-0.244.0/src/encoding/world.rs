use super::{Adapter, ComponentEncoder, LibraryInfo, RequiredOptions};
use crate::validation::{
    Import, ImportMap, PayloadType, ValidatedModule, validate_adapter_module, validate_module,
};
use anyhow::{Context, Result};
use indexmap::{IndexMap, IndexSet};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use wit_parser::{
    Function, InterfaceId, LiveTypes, Resolve, TypeDefKind, TypeId, TypeOwner, WorldId, WorldItem,
    WorldKey,
    abi::{AbiVariant, WasmSignature},
};

pub struct WorldAdapter<'a> {
    pub wasm: Cow<'a, [u8]>,
    pub info: ValidatedModule,
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
    pub info: ValidatedModule,
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
    pub lowerings: IndexMap<(String, AbiVariant), Lowering>,
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
        let info = validate_module(encoder, &encoder.module, encoder.module_import_map.as_ref())
            .context("module was not valid")?;

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
                metadata: _,
                required_exports,
                library_info,
            },
        ) in self.encoder.adapters.iter()
        {
            let required_by_import = self.info.imports.required_from_adapter(name.as_str());
            let no_required_by_import = || required_by_import.is_empty();
            let no_required_exports = || {
                required_exports
                    .iter()
                    .all(|name| match &resolve.worlds[world].exports[name] {
                        WorldItem::Function(_) => false,
                        WorldItem::Interface { id, .. } => {
                            resolve.interfaces[*id].functions.is_empty()
                        }
                        WorldItem::Type(_) => true,
                    })
            };
            if no_required_by_import() && no_required_exports() && library_info.is_none() {
                continue;
            }
            let wasm = if library_info.is_some() {
                Cow::Borrowed(wasm as &[u8])
            } else {
                // Without `library_info` this means that this is an adapter.
                // The goal of the adapter is to provide a suite of symbols that
                // can be imported, but not all symbols may be imported. Here
                // the module is trimmed down to only what's needed by the
                // original main module.
                //
                // The main module requires `required_by_import` above, but
                // adapters may themselves also export WIT items. To handle this
                // the sequence of operations here are:
                //
                // 1. First the adapter is validated as-is. This ensures that
                //    everything looks good before GC.
                // 2. The metadata from step (1) is used to determine the set of
                //    WIT-level exports that are needed. This includes things
                //    like realloc functions and such.
                // 3. The set of WIT-level functions from (2) is unioned with
                //    `required_by_import` to create the set of required exports
                //    of the adapter.
                // 4. This set of exports is used to delete some exports of the
                //    adapter and then perform a GC pass.
                //
                // Finally at the end of all of this the
                // `validate_adapter_module` method is called for a second time
                // on the minimized adapter. This is done because deleting
                // imports may have deleted some imports which means that the
                // final component may not need to import as many interfaces.
                let info = validate_adapter_module(
                    self.encoder,
                    &wasm,
                    &required_by_import,
                    required_exports,
                    library_info.as_ref(),
                )
                .with_context(|| {
                    format!("failed to validate the imports of the adapter module `{name}`")
                })?;
                let mut required = IndexSet::new();
                for (name, _ty) in required_by_import.iter() {
                    required.insert(name.to_string());
                }
                for (name, _export) in info.exports.iter() {
                    required.insert(name.to_string());
                }

                Cow::Owned(
                    crate::gc::run(
                        wasm,
                        &required,
                        if self.encoder.realloc_via_memory_grow {
                            None
                        } else {
                            self.info.exports.realloc_to_import_into_adapter()
                        },
                    )
                    .context("failed to reduce input adapter module to its minimal size")?,
                )
            };
            let info = validate_adapter_module(
                self.encoder,
                &wasm,
                &required_by_import,
                required_exports,
                library_info.as_ref(),
            )
            .with_context(|| {
                format!("failed to validate the imports of the minimized adapter module `{name}`")
            })?;
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

    /// Fills out the `import_map` field of `self` by determining the live
    /// functions from all imports. This additionally classifies imported
    /// functions into direct or indirect lowerings for managing shims.
    fn process_imports(&mut self) -> Result<()> {
        let resolve = &self.encoder.metadata.resolve;
        let world = self.encoder.metadata.world;

        // Inspect all imports of the main module and adapters to find all
        // WIT-looking things and register those as required. This is used to
        // prune out unneeded things in the `add_item` function below.
        let mut required = Required::default();
        for (_, _, import) in self
            .adapters
            .values()
            .flat_map(|a| a.info.imports.imports())
            .chain(self.info.imports.imports())
        {
            match import {
                Import::WorldFunc(_, name, abi) => {
                    required
                        .interface_funcs
                        .entry(None)
                        .or_default()
                        .insert((name, *abi));
                }
                Import::InterfaceFunc(_, id, name, abi) => {
                    required
                        .interface_funcs
                        .entry(Some(*id))
                        .or_default()
                        .insert((name, *abi));
                }
                Import::ImportedResourceDrop(_, _, id) => {
                    required.resource_drops.insert(*id);
                }
                _ => {}
            }
        }
        for (name, item) in resolve.worlds[world].imports.iter() {
            add_item(&mut self.import_map, resolve, name, item, &required)?;
        }
        return Ok(());

        fn add_item(
            import_map: &mut IndexMap<Option<String>, ImportedInterface>,
            resolve: &Resolve,
            name: &WorldKey,
            item: &WorldItem,
            required: &Required<'_>,
        ) -> Result<()> {
            let name = resolve.name_world_key(name);
            log::trace!("register import `{name}`");
            let import_map_key = match item {
                WorldItem::Function(_) | WorldItem::Type(_) => None,
                WorldItem::Interface { .. } => Some(name),
            };
            let interface_id = match item {
                WorldItem::Function(_) | WorldItem::Type(_) => None,
                WorldItem::Interface { id, .. } => Some(*id),
            };
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
                WorldItem::Interface { id, .. } => {
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
        self.add_live_imports(world, &self.info.imports, &mut live);
        for (adapter_name, adapter) in self.adapters.iter() {
            log::trace!("processing adapter `{adapter_name}`");
            self.add_live_imports(world, &adapter.info.imports, &mut live);
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
                WorldItem::Interface { id, .. } => id,
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

    fn add_live_imports(&self, world: WorldId, imports: &ImportMap, live: &mut LiveTypes) {
        let resolve = &self.encoder.metadata.resolve;
        let world = &resolve.worlds[world];

        // FIXME: ideally liveness information here would be plumbed through to
        // encoding but that's not done at this time. Only liveness for each
        // interface is plumbed so top-level world types are unconditionally
        // encoded and therefore unconditionally live here. Once encoding is
        // based on conditionally-live things then this should be removed.
        for (_, item) in world.imports.iter() {
            if let WorldItem::Type(id) = item {
                live.add_type_id(resolve, *id);
            }
        }

        for (_, _, import) in imports.imports() {
            match import {
                // WIT-level function imports need the associated WIT definition.
                Import::WorldFunc(key, _, _) => {
                    live.add_world_item(resolve, &world.imports[key]);
                }
                Import::InterfaceFunc(_, id, name, _) => {
                    live.add_func(resolve, &resolve.interfaces[*id].functions[name]);
                }

                // Resource-related intrinsics will need the resource.
                Import::ImportedResourceDrop(.., ty)
                | Import::ExportedResourceDrop(_, ty)
                | Import::ExportedResourceNew(_, ty)
                | Import::ExportedResourceRep(_, ty) => live.add_type_id(resolve, *ty),

                // Future/Stream related intrinsics need to refer to the type
                // that the intrinsic is operating on.
                Import::StreamNew(info)
                | Import::StreamRead { info, async_: _ }
                | Import::StreamWrite { info, async_: _ }
                | Import::StreamCancelRead { info, async_: _ }
                | Import::StreamCancelWrite { info, async_: _ }
                | Import::StreamDropReadable(info)
                | Import::StreamDropWritable(info)
                | Import::FutureNew(info)
                | Import::FutureRead { info, async_: _ }
                | Import::FutureWrite { info, async_: _ }
                | Import::FutureCancelRead { info, async_: _ }
                | Import::FutureCancelWrite { info, async_: _ }
                | Import::FutureDropReadable(info)
                | Import::FutureDropWritable(info) => {
                    if let PayloadType::Type { id, .. } = info.ty {
                        live.add_type_id(resolve, id);
                    }
                }

                // The `task.return` intrinsic needs to be able to refer to the
                // type that is being returned.
                Import::ExportedTaskReturn(.., ty) => {
                    if let Some(ty) = ty {
                        live.add_type(resolve, ty);
                    }
                }

                // Intrinsics that don't need to refer to WIT types can be
                // skipped here.
                Import::AdapterExport { .. }
                | Import::MainModuleMemory
                | Import::MainModuleExport { .. }
                | Import::Item(_)
                | Import::ContextGet(_)
                | Import::ContextSet(_)
                | Import::BackpressureInc
                | Import::BackpressureDec
                | Import::WaitableSetNew
                | Import::WaitableSetWait { .. }
                | Import::WaitableSetPoll { .. }
                | Import::WaitableSetDrop
                | Import::WaitableJoin
                | Import::ThreadYield { .. }
                | Import::SubtaskDrop
                | Import::SubtaskCancel { .. }
                | Import::ErrorContextNew { .. }
                | Import::ErrorContextDebugMessage { .. }
                | Import::ErrorContextDrop
                | Import::ExportedTaskCancel
                | Import::ThreadIndex
                | Import::ThreadNewIndirect { .. }
                | Import::ThreadSwitchTo { .. }
                | Import::ThreadSuspend { .. }
                | Import::ThreadResumeLater
                | Import::ThreadYieldTo { .. } => {}
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
                WorldItem::Interface { id, .. } => *id,
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

#[derive(Default)]
struct Required<'a> {
    interface_funcs: IndexMap<Option<InterfaceId>, IndexSet<(&'a str, AbiVariant)>>,
    resource_drops: IndexSet<TypeId>,
}

impl ImportedInterface {
    fn add_func(&mut self, required: &Required<'_>, resolve: &Resolve, func: &Function) {
        let mut abis = Vec::with_capacity(2);
        if let Some(set) = required.interface_funcs.get(&self.interface) {
            if set.contains(&(func.name.as_str(), AbiVariant::GuestImport)) {
                abis.push(AbiVariant::GuestImport);
            }
            if set.contains(&(func.name.as_str(), AbiVariant::GuestImportAsync)) {
                abis.push(AbiVariant::GuestImportAsync);
            }
        }
        for abi in abis {
            log::trace!("add func {} {abi:?}", func.name);
            let options = RequiredOptions::for_import(resolve, func, abi);
            let lowering = if options.is_empty() {
                Lowering::Direct
            } else {
                let sig = resolve.wasm_signature(abi, func);
                Lowering::Indirect { sig, options }
            };

            let prev = self.lowerings.insert((func.name.clone(), abi), lowering);
            assert!(prev.is_none());
        }
    }

    fn add_type(&mut self, required: &Required<'_>, resolve: &Resolve, id: TypeId) {
        let ty = &resolve.types[id];
        match &ty.kind {
            TypeDefKind::Resource => {}
            _ => return,
        }
        let name = ty.name.as_deref().expect("resources must be named");

        if required.resource_drops.contains(&id) {
            let name = format!("{name}_drop");
            let prev = self
                .lowerings
                .insert((name, AbiVariant::GuestImport), Lowering::ResourceDrop(id));
            assert!(prev.is_none());
        }
    }
}

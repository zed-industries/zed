//! Support for encoding a core wasm module into a component.
//!
//! This module, at a high level, is tasked with transforming a core wasm
//! module into a component. This will process the imports/exports of the core
//! wasm module and translate between the `wit-parser` AST and the component
//! model binary format, producing a final component which will import
//! `*.wit` defined interfaces and export `*.wit` defined interfaces as well
//! with everything wired up internally according to the canonical ABI and such.
//!
//! This doc block here is not currently 100% complete and doesn't cover the
//! full functionality of this module.
//!
//! # Adapter Modules
//!
//! One feature of this encoding process which is non-obvious is the support for
//! "adapter modules". The general idea here is that historical host API
//! definitions have been around for quite some time, such as
//! `wasi_snapshot_preview1`, but these host API definitions are not compatible
//! with the canonical ABI or component model exactly. These APIs, however, can
//! in most situations be roughly adapted to component-model equivalents. This
//! is where adapter modules come into play, they're converting from some
//! arbitrary API/ABI into a component-model using API.
//!
//! An adapter module is a separately compiled `*.wasm` blob which will export
//! functions matching the desired ABI (e.g. exporting functions matching the
//! `wasi_snapshot_preview1` ABI). The `*.wasm` blob will then import functions
//! in the canonical ABI and internally adapt the exported functions to the
//! imported functions. The encoding support in this module is what wires
//! everything up and makes sure that everything is imported and exported to the
//! right place. Adapter modules currently always use "indirect lowerings"
//! meaning that a shim module is created and provided as the imports to the
//! main core wasm module, and the shim module is "filled in" at a later time
//! during the instantiation process.
//!
//! Adapter modules are not intended to be general purpose and are currently
//! very restrictive, namely:
//!
//! * They must import a linear memory and not define their own linear memory
//!   otherwise. In other words they import memory and cannot use multi-memory.
//! * They cannot define any `elem` or `data` segments since otherwise there's
//!   no knowledge ahead-of-time of where their data or element segments could
//!   go. This means things like no panics, no indirect calls, etc.
//! * If the adapter uses a shadow stack, the global that points to it must be a
//!   mutable `i32` named `__stack_pointer`. This stack is automatically
//!   allocated with an injected `allocate_stack` function that will either use
//!   the main module's `cabi_realloc` export (if present) or `memory.grow`. It
//!   allocates only 64KB of stack space, and there is no protection if that
//!   overflows.
//! * If the adapter has a global, mutable `i32` named `allocation_state`, it
//!   will be used to keep track of stack allocation status and avoid infinite
//!   recursion if the main module's `cabi_realloc` function calls back into the
//!   adapter.  `allocate_stack` will check this global on entry; if it is zero,
//!   it will set it to one, then allocate the stack, and finally set it to two.
//!   If it is non-zero, `allocate_stack` will do nothing and return immediately
//!   (because either the stack has already been allocated or is in the process
//!   of being allocated).  If the adapter does not have an `allocation_state`,
//!   `allocate_stack` will use `memory.grow` to allocate the stack; it will
//!   _not_ use the main module's `cabi_realloc` even if it's available.
//! * If the adapter imports a `cabi_realloc` function, and the main module
//!   exports one, they'll be linked together via an alias. If the adapter
//!   imports such a function but the main module does _not_ export one, we'll
//!   synthesize one based on `memory.grow` (which will trap for any size other
//!   than 64KB). Note that the main module's `cabi_realloc` function may call
//!   back into the adapter before the shadow stack has been allocated. In this
//!   case (when `allocation_state` is zero or one), the adapter should return
//!   whatever dummy value(s) it can immediately without touching the stack.
//!
//! This means that adapter modules are not meant to be written by everyone.
//! It's assumed that these will be relatively few and far between yet still a
//! crucial part of the transition process from to the component model since
//! otherwise there's no way to run a `wasi_snapshot_preview1` module within the
//! component model.

use crate::StringEncoding;
use crate::metadata::{self, Bindgen, ModuleMetadata};
use crate::validation::{
    Export, ExportMap, Import, ImportInstance, ImportMap, PayloadInfo, PayloadType,
};
use anyhow::{Context, Result, anyhow, bail};
use indexmap::{IndexMap, IndexSet};
use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use wasm_encoder::*;
use wasmparser::{Validator, WasmFeatures};
use wit_parser::{
    Function, FunctionKind, InterfaceId, LiveTypes, Resolve, Stability, Type, TypeDefKind, TypeId,
    TypeOwner, WorldItem, WorldKey,
    abi::{AbiVariant, WasmSignature, WasmType},
};

const INDIRECT_TABLE_NAME: &str = "$imports";

mod wit;
pub use wit::{encode, encode_world};

mod types;
use types::{InstanceTypeEncoder, RootTypeEncoder, TypeEncodingMaps, ValtypeEncoder};
mod world;
use world::{ComponentWorld, ImportedInterface, Lowering};

mod dedupe;
pub(crate) use dedupe::ModuleImportMap;
use wasm_metadata::AddMetadataField;

fn to_val_type(ty: &WasmType) -> ValType {
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

bitflags::bitflags! {
    /// Options in the `canon lower` or `canon lift` required for a particular
    /// function.
    #[derive(Copy, Clone, Debug)]
    pub struct RequiredOptions: u8 {
        /// A memory must be specified, typically the "main module"'s memory
        /// export.
        const MEMORY = 1 << 0;
        /// A `realloc` function must be specified, typically named
        /// `cabi_realloc`.
        const REALLOC = 1 << 1;
        /// A string encoding must be specified, which is always utf-8 for now
        /// today.
        const STRING_ENCODING = 1 << 2;
        const ASYNC = 1 << 3;
    }
}

impl RequiredOptions {
    fn for_import(resolve: &Resolve, func: &Function, abi: AbiVariant) -> RequiredOptions {
        let sig = resolve.wasm_signature(abi, func);
        let mut ret = RequiredOptions::empty();
        // Lift the params and lower the results for imports
        ret.add_lift(TypeContents::for_types(
            resolve,
            func.params.iter().map(|(_, t)| t),
        ));
        ret.add_lower(TypeContents::for_types(resolve, &func.result));

        // If anything is indirect then `memory` will be required to read the
        // indirect values.
        if sig.retptr || sig.indirect_params {
            ret |= RequiredOptions::MEMORY;
        }
        if abi == AbiVariant::GuestImportAsync {
            ret |= RequiredOptions::ASYNC;
        }
        ret
    }

    fn for_export(resolve: &Resolve, func: &Function, abi: AbiVariant) -> RequiredOptions {
        let sig = resolve.wasm_signature(abi, func);
        let mut ret = RequiredOptions::empty();
        // Lower the params and lift the results for exports
        ret.add_lower(TypeContents::for_types(
            resolve,
            func.params.iter().map(|(_, t)| t),
        ));
        ret.add_lift(TypeContents::for_types(resolve, &func.result));

        // If anything is indirect then `memory` will be required to read the
        // indirect values, but if the arguments are indirect then `realloc` is
        // additionally required to allocate space for the parameters.
        if sig.retptr || sig.indirect_params {
            ret |= RequiredOptions::MEMORY;
            if sig.indirect_params {
                ret |= RequiredOptions::REALLOC;
            }
        }
        if let AbiVariant::GuestExportAsync | AbiVariant::GuestExportAsyncStackful = abi {
            ret |= RequiredOptions::ASYNC;
            ret |= task_return_options_and_type(resolve, func.result).0;
        }
        ret
    }

    fn add_lower(&mut self, types: TypeContents) {
        // If lists/strings are lowered into wasm then memory is required as
        // usual but `realloc` is also required to allow the external caller to
        // allocate space in the destination for the list/string.
        if types.contains(TypeContents::NEEDS_MEMORY) {
            *self |= RequiredOptions::MEMORY | RequiredOptions::REALLOC;
        }
        if types.contains(TypeContents::STRING) {
            *self |= RequiredOptions::MEMORY
                | RequiredOptions::STRING_ENCODING
                | RequiredOptions::REALLOC;
        }
    }

    fn add_lift(&mut self, types: TypeContents) {
        // Unlike for `lower` when lifting a string/list all that's needed is
        // memory, since the string/list already resides in memory `realloc`
        // isn't needed.
        if types.contains(TypeContents::NEEDS_MEMORY) {
            *self |= RequiredOptions::MEMORY;
        }
        if types.contains(TypeContents::STRING) {
            *self |= RequiredOptions::MEMORY | RequiredOptions::STRING_ENCODING;
        }
    }

    fn into_iter(
        self,
        encoding: StringEncoding,
        memory_index: Option<u32>,
        realloc_index: Option<u32>,
    ) -> Result<impl ExactSizeIterator<Item = CanonicalOption>> {
        #[derive(Default)]
        struct Iter {
            options: [Option<CanonicalOption>; 5],
            current: usize,
            count: usize,
        }

        impl Iter {
            fn push(&mut self, option: CanonicalOption) {
                assert!(self.count < self.options.len());
                self.options[self.count] = Some(option);
                self.count += 1;
            }
        }

        impl Iterator for Iter {
            type Item = CanonicalOption;

            fn next(&mut self) -> Option<Self::Item> {
                if self.current == self.count {
                    return None;
                }
                let option = self.options[self.current];
                self.current += 1;
                option
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.count - self.current, Some(self.count - self.current))
            }
        }

        impl ExactSizeIterator for Iter {}

        let mut iter = Iter::default();

        if self.contains(RequiredOptions::MEMORY) {
            iter.push(CanonicalOption::Memory(memory_index.ok_or_else(|| {
                anyhow!("module does not export a memory named `memory`")
            })?));
        }

        if self.contains(RequiredOptions::REALLOC) {
            iter.push(CanonicalOption::Realloc(realloc_index.ok_or_else(
                || anyhow!("module does not export a function named `cabi_realloc`"),
            )?));
        }

        if self.contains(RequiredOptions::STRING_ENCODING) {
            iter.push(encoding.into());
        }

        if self.contains(RequiredOptions::ASYNC) {
            iter.push(CanonicalOption::Async);
        }

        Ok(iter)
    }
}

bitflags::bitflags! {
    /// Flags about what kinds of types are present within the recursive
    /// structure of a type.
    struct TypeContents: u8 {
        const STRING = 1 << 0;
        const NEEDS_MEMORY = 1 << 1;
    }
}

impl TypeContents {
    fn for_types<'a>(resolve: &Resolve, types: impl IntoIterator<Item = &'a Type>) -> Self {
        let mut cur = TypeContents::empty();
        for ty in types {
            cur |= Self::for_type(resolve, ty);
        }
        cur
    }

    fn for_optional_types<'a>(
        resolve: &Resolve,
        types: impl Iterator<Item = Option<&'a Type>>,
    ) -> Self {
        Self::for_types(resolve, types.flatten())
    }

    fn for_optional_type(resolve: &Resolve, ty: Option<&Type>) -> Self {
        match ty {
            Some(ty) => Self::for_type(resolve, ty),
            None => Self::empty(),
        }
    }

    fn for_type(resolve: &Resolve, ty: &Type) -> Self {
        match ty {
            Type::Id(id) => match &resolve.types[*id].kind {
                TypeDefKind::Handle(h) => match h {
                    wit_parser::Handle::Own(_) => Self::empty(),
                    wit_parser::Handle::Borrow(_) => Self::empty(),
                },
                TypeDefKind::Resource => Self::empty(),
                TypeDefKind::Record(r) => Self::for_types(resolve, r.fields.iter().map(|f| &f.ty)),
                TypeDefKind::Tuple(t) => Self::for_types(resolve, t.types.iter()),
                TypeDefKind::Flags(_) => Self::empty(),
                TypeDefKind::Option(t) => Self::for_type(resolve, t),
                TypeDefKind::Result(r) => {
                    Self::for_optional_type(resolve, r.ok.as_ref())
                        | Self::for_optional_type(resolve, r.err.as_ref())
                }
                TypeDefKind::Variant(v) => {
                    Self::for_optional_types(resolve, v.cases.iter().map(|c| c.ty.as_ref()))
                }
                TypeDefKind::Enum(_) => Self::empty(),
                TypeDefKind::List(t) => Self::for_type(resolve, t) | Self::NEEDS_MEMORY,
                TypeDefKind::Map(k, v) => {
                    Self::for_type(resolve, k) | Self::for_type(resolve, v) | Self::NEEDS_MEMORY
                }
                TypeDefKind::FixedSizeList(t, _elements) => Self::for_type(resolve, t),
                TypeDefKind::Type(t) => Self::for_type(resolve, t),
                TypeDefKind::Future(_) => Self::empty(),
                TypeDefKind::Stream(_) => Self::empty(),
                TypeDefKind::Unknown => unreachable!(),
            },
            Type::String => Self::STRING,
            _ => Self::empty(),
        }
    }
}

/// State relating to encoding a component.
pub struct EncodingState<'a> {
    /// The component being encoded.
    component: ComponentBuilder,
    /// The index into the core module index space for the inner core module.
    ///
    /// If `None`, the core module has not been encoded.
    module_index: Option<u32>,
    /// The index into the core instance index space for the inner core module.
    ///
    /// If `None`, the core module has not been instantiated.
    instance_index: Option<u32>,
    /// The index in the core memory index space for the exported memory.
    ///
    /// If `None`, then the memory has not yet been aliased.
    memory_index: Option<u32>,
    /// The index of the shim instance used for lowering imports into the core instance.
    ///
    /// If `None`, then the shim instance how not yet been encoded.
    shim_instance_index: Option<u32>,
    /// The index of the fixups module to instantiate to fill in the lowered imports.
    ///
    /// If `None`, then a fixup module has not yet been encoded.
    fixups_module_index: Option<u32>,

    /// A map of named adapter modules and the index that the module was defined
    /// at.
    adapter_modules: IndexMap<&'a str, u32>,
    /// A map of adapter module instances and the index of their instance.
    adapter_instances: IndexMap<&'a str, u32>,

    /// Imported instances and what index they were imported as.
    imported_instances: IndexMap<InterfaceId, u32>,
    imported_funcs: IndexMap<String, u32>,
    exported_instances: IndexMap<InterfaceId, u32>,

    /// Maps used when translating types to the component model binary format.
    /// Note that imports and exports are stored in separate maps since they
    /// need fresh hierarchies of types in case the same interface is both
    /// imported and exported.
    import_type_encoding_maps: TypeEncodingMaps<'a>,
    export_type_encoding_maps: TypeEncodingMaps<'a>,

    /// Cache of items that have been aliased from core instances.
    ///
    /// This is a helper to reduce the number of aliases created by ensuring
    /// that repeated requests for the same item return the same index of an
    /// original `core alias` item.
    aliased_core_items: HashMap<(u32, String), u32>,

    /// Metadata about the world inferred from the input to `ComponentEncoder`.
    info: &'a ComponentWorld<'a>,
}

impl<'a> EncodingState<'a> {
    fn encode_core_modules(&mut self) {
        assert!(self.module_index.is_none());
        let idx = self
            .component
            .core_module_raw(Some("main"), &self.info.encoder.module);
        self.module_index = Some(idx);

        for (name, adapter) in self.info.adapters.iter() {
            let debug_name = if adapter.library_info.is_some() {
                name.to_string()
            } else {
                format!("wit-component:adapter:{name}")
            };
            let idx = if self.info.encoder.debug_names {
                let mut add_meta = wasm_metadata::AddMetadata::default();
                add_meta.name = AddMetadataField::Set(debug_name.clone());
                let wasm = add_meta
                    .to_wasm(&adapter.wasm)
                    .expect("core wasm can get name added");
                self.component.core_module_raw(Some(&debug_name), &wasm)
            } else {
                self.component
                    .core_module_raw(Some(&debug_name), &adapter.wasm)
            };
            let prev = self.adapter_modules.insert(name, idx);
            assert!(prev.is_none());
        }
    }

    fn root_import_type_encoder(
        &mut self,
        interface: Option<InterfaceId>,
    ) -> RootTypeEncoder<'_, 'a> {
        RootTypeEncoder {
            state: self,
            interface,
            import_types: true,
        }
    }

    fn root_export_type_encoder(
        &mut self,
        interface: Option<InterfaceId>,
    ) -> RootTypeEncoder<'_, 'a> {
        RootTypeEncoder {
            state: self,
            interface,
            import_types: false,
        }
    }

    fn instance_type_encoder(&mut self, interface: InterfaceId) -> InstanceTypeEncoder<'_, 'a> {
        InstanceTypeEncoder {
            state: self,
            interface,
            type_encoding_maps: Default::default(),
            ty: Default::default(),
        }
    }

    fn encode_imports(&mut self, name_map: &HashMap<String, String>) -> Result<()> {
        let mut has_funcs = false;
        for (name, info) in self.info.import_map.iter() {
            match name {
                Some(name) => {
                    self.encode_interface_import(name_map.get(name).unwrap_or(name), info)?
                }
                None => has_funcs = true,
            }
        }

        let resolve = &self.info.encoder.metadata.resolve;
        let world = &resolve.worlds[self.info.encoder.metadata.world];

        // FIXME: ideally this would use the liveness analysis from
        // world-building to only encode live types, not all type in a world.
        for (_name, item) in world.imports.iter() {
            if let WorldItem::Type(ty) = item {
                self.root_import_type_encoder(None)
                    .encode_valtype(resolve, &Type::Id(*ty))?;
            }
        }

        if has_funcs {
            let info = &self.info.import_map[&None];
            self.encode_root_import_funcs(info)?;
        }
        Ok(())
    }

    fn encode_interface_import(&mut self, name: &str, info: &ImportedInterface) -> Result<()> {
        let resolve = &self.info.encoder.metadata.resolve;
        let interface_id = info.interface.as_ref().unwrap();
        let interface_id = *interface_id;
        let interface = &resolve.interfaces[interface_id];
        log::trace!("encoding imports for `{name}` as {interface_id:?}");
        let mut encoder = self.instance_type_encoder(interface_id);

        // First encode all type information
        if let Some(live) = encoder.state.info.live_type_imports.get(&interface_id) {
            for ty in live {
                log::trace!(
                    "encoding extra type {ty:?} name={:?}",
                    resolve.types[*ty].name
                );
                encoder.encode_valtype(resolve, &Type::Id(*ty))?;
            }
        }

        // Next encode all required functions from this imported interface
        // into the instance type.
        for (_, func) in interface.functions.iter() {
            if !(info
                .lowerings
                .contains_key(&(func.name.clone(), AbiVariant::GuestImport))
                || info
                    .lowerings
                    .contains_key(&(func.name.clone(), AbiVariant::GuestImportAsync)))
            {
                continue;
            }
            log::trace!("encoding function type for `{}`", func.name);
            let idx = encoder.encode_func_type(resolve, func)?;

            encoder.ty.export(&func.name, ComponentTypeRef::Func(idx));
        }

        let ty = encoder.ty;
        // Don't encode empty instance types since they're not
        // meaningful to the runtime of the component anyway.
        if ty.is_empty() {
            return Ok(());
        }
        let instance_type_idx = self
            .component
            .type_instance(Some(&format!("ty-{name}")), &ty);
        let instance_idx = self
            .component
            .import(name, ComponentTypeRef::Instance(instance_type_idx));
        let prev = self.imported_instances.insert(interface_id, instance_idx);
        assert!(prev.is_none());
        Ok(())
    }

    fn encode_root_import_funcs(&mut self, info: &ImportedInterface) -> Result<()> {
        let resolve = &self.info.encoder.metadata.resolve;
        let world = self.info.encoder.metadata.world;
        for (name, item) in resolve.worlds[world].imports.iter() {
            let func = match item {
                WorldItem::Function(f) => f,
                WorldItem::Interface { .. } | WorldItem::Type(_) => continue,
            };
            let name = resolve.name_world_key(name);
            if !(info
                .lowerings
                .contains_key(&(name.clone(), AbiVariant::GuestImport))
                || info
                    .lowerings
                    .contains_key(&(name.clone(), AbiVariant::GuestImportAsync)))
            {
                continue;
            }
            log::trace!("encoding function type for `{}`", func.name);
            let idx = self
                .root_import_type_encoder(None)
                .encode_func_type(resolve, func)?;
            let func_idx = self.component.import(&name, ComponentTypeRef::Func(idx));
            let prev = self.imported_funcs.insert(name, func_idx);
            assert!(prev.is_none());
        }
        Ok(())
    }

    fn alias_imported_type(&mut self, interface: InterfaceId, id: TypeId) -> u32 {
        let ty = &self.info.encoder.metadata.resolve.types[id];
        let name = ty.name.as_ref().expect("type must have a name");
        let instance = self.imported_instances[&interface];
        self.component
            .alias_export(instance, name, ComponentExportKind::Type)
    }

    fn alias_exported_type(&mut self, interface: InterfaceId, id: TypeId) -> u32 {
        let ty = &self.info.encoder.metadata.resolve.types[id];
        let name = ty.name.as_ref().expect("type must have a name");
        let instance = self.exported_instances[&interface];
        self.component
            .alias_export(instance, name, ComponentExportKind::Type)
    }

    fn encode_core_instantiation(&mut self) -> Result<()> {
        // Encode a shim instantiation if needed
        let shims = self.encode_shim_instantiation()?;

        // Next declare any types needed for imported intrinsics. This
        // populates `export_type_map` and will additionally be used for
        // imports to modules instantiated below.
        self.declare_types_for_imported_intrinsics(&shims)?;

        // Next instantiate the main module. This provides the linear memory to
        // use for all future adapters and enables creating indirect lowerings
        // at the end.
        self.instantiate_main_module(&shims)?;

        // Separate the adapters according which should be instantiated before
        // and after indirect lowerings are encoded.
        let (before, after) = self
            .info
            .adapters
            .iter()
            .partition::<Vec<_>, _>(|(_, adapter)| {
                !matches!(
                    adapter.library_info,
                    Some(LibraryInfo {
                        instantiate_after_shims: true,
                        ..
                    })
                )
            });

        for (name, _adapter) in before {
            self.instantiate_adapter_module(&shims, name)?;
        }

        // With all the relevant core wasm instances in play now the original shim
        // module, if present, can be filled in with lowerings/adapters/etc.
        self.encode_indirect_lowerings(&shims)?;

        for (name, _adapter) in after {
            self.instantiate_adapter_module(&shims, name)?;
        }

        self.encode_initialize_with_start()?;

        Ok(())
    }

    fn lookup_resource_index(&mut self, id: TypeId) -> u32 {
        let resolve = &self.info.encoder.metadata.resolve;
        let ty = &resolve.types[id];
        match ty.owner {
            // If this resource is owned by a world then it's a top-level
            // resource which means it must have already been translated so
            // it's available for lookup in `import_type_map`.
            TypeOwner::World(_) => self.import_type_encoding_maps.id_to_index[&id],
            TypeOwner::Interface(i) => {
                let instance = self.imported_instances[&i];
                let name = ty.name.as_ref().expect("resources must be named");
                self.component
                    .alias_export(instance, name, ComponentExportKind::Type)
            }
            TypeOwner::None => panic!("resources must have an owner"),
        }
    }

    fn encode_exports(&mut self, module: CustomModule) -> Result<()> {
        let resolve = &self.info.encoder.metadata.resolve;
        let exports = match module {
            CustomModule::Main => &self.info.encoder.main_module_exports,
            CustomModule::Adapter(name) => &self.info.encoder.adapters[name].required_exports,
        };

        if exports.is_empty() {
            return Ok(());
        }

        let mut interface_func_core_names = IndexMap::new();
        let mut world_func_core_names = IndexMap::new();
        for (core_name, export) in self.info.exports_for(module).iter() {
            match export {
                Export::WorldFunc(_, name, _) => {
                    let prev = world_func_core_names.insert(name, core_name);
                    assert!(prev.is_none());
                }
                Export::InterfaceFunc(_, id, name, _) => {
                    let prev = interface_func_core_names
                        .entry(id)
                        .or_insert(IndexMap::new())
                        .insert(name.as_str(), core_name);
                    assert!(prev.is_none());
                }
                Export::WorldFuncCallback(..)
                | Export::InterfaceFuncCallback(..)
                | Export::WorldFuncPostReturn(..)
                | Export::InterfaceFuncPostReturn(..)
                | Export::ResourceDtor(..)
                | Export::Memory
                | Export::GeneralPurposeRealloc
                | Export::GeneralPurposeExportRealloc
                | Export::GeneralPurposeImportRealloc
                | Export::Initialize
                | Export::ReallocForAdapter
                | Export::IndirectFunctionTable => continue,
            }
        }

        let world = &resolve.worlds[self.info.encoder.metadata.world];

        for export_name in exports {
            let export_string = resolve.name_world_key(export_name);
            match &world.exports[export_name] {
                WorldItem::Function(func) => {
                    let ty = self
                        .root_import_type_encoder(None)
                        .encode_func_type(resolve, func)?;
                    let core_name = world_func_core_names[&func.name];
                    let idx = self.encode_lift(module, &core_name, export_name, func, ty)?;
                    self.component
                        .export(&export_string, ComponentExportKind::Func, idx, None);
                }
                WorldItem::Interface { id, .. } => {
                    let core_names = interface_func_core_names.get(id);
                    self.encode_interface_export(
                        &export_string,
                        module,
                        export_name,
                        *id,
                        core_names,
                    )?;
                }
                WorldItem::Type(_) => unreachable!(),
            }
        }

        Ok(())
    }

    fn encode_interface_export(
        &mut self,
        export_name: &str,
        module: CustomModule<'_>,
        key: &WorldKey,
        export: InterfaceId,
        interface_func_core_names: Option<&IndexMap<&str, &str>>,
    ) -> Result<()> {
        log::trace!("encode interface export `{export_name}`");
        let resolve = &self.info.encoder.metadata.resolve;

        // First execute a `canon lift` for all the functions in this interface
        // from the core wasm export. This requires type information but notably
        // not exported type information since we don't want to export this
        // interface's types from the root of the component. Each lifted
        // function is saved off into an `imports` array to get imported into
        // the nested component synthesized below.
        let mut imports = Vec::new();
        let mut root = self.root_export_type_encoder(Some(export));
        for (_, func) in &resolve.interfaces[export].functions {
            let core_name = interface_func_core_names.unwrap()[func.name.as_str()];
            let ty = root.encode_func_type(resolve, func)?;
            let func_index = root.state.encode_lift(module, &core_name, key, func, ty)?;
            imports.push((
                import_func_name(func),
                ComponentExportKind::Func,
                func_index,
            ));
        }

        // Next a nested component is created which will import the functions
        // above and then reexport them. The purpose of them is to "re-type" the
        // functions through type ascription on each `func` item.
        let mut nested = NestedComponentTypeEncoder {
            component: ComponentBuilder::default(),
            type_encoding_maps: Default::default(),
            export_types: false,
            interface: export,
            state: self,
            imports: IndexMap::new(),
        };

        // Import all transitively-referenced types from other interfaces into
        // this component. This temporarily switches the `interface` listed to
        // the interface of the referred-to-type to generate the import. After
        // this loop `interface` is rewritten to `export`.
        //
        // Each component is a standalone "island" so the necessary type
        // information needs to be rebuilt within this component. This ensures
        // that we're able to build a valid component and additionally connect
        // all the type information to the outer context.
        let mut types_to_import = LiveTypes::default();
        types_to_import.add_interface(resolve, export);
        let exports_used = &nested.state.info.exports_used[&export];
        for ty in types_to_import.iter() {
            if let TypeOwner::Interface(owner) = resolve.types[ty].owner {
                if owner == export {
                    // Here this deals with the current exported interface which
                    // is handled below.
                    continue;
                }

                // Ensure that `self` has encoded this type before. If so this
                // is a noop but otherwise it generates the type here.
                let mut encoder = if exports_used.contains(&owner) {
                    nested.state.root_export_type_encoder(Some(export))
                } else {
                    nested.state.root_import_type_encoder(Some(export))
                };
                encoder.encode_valtype(resolve, &Type::Id(ty))?;

                // Next generate the same type but this time within the
                // component itself. The type generated above (or prior) will be
                // used to satisfy this type import.
                nested.interface = owner;
                nested.encode_valtype(resolve, &Type::Id(ty))?;
            }
        }
        nested.interface = export;

        // Record the map of types imported to their index at where they were
        // imported. This is used after imports are encoded as exported types
        // will refer to these.
        let imported_type_maps = nested.type_encoding_maps.clone();

        // Handle resource types for this instance specially, namely importing
        // them into the nested component. This models how the resource is
        // imported from its definition in the outer component to get reexported
        // internally. This chiefly avoids creating a second resource which is
        // not desired in this situation.
        let mut resources = HashMap::new();
        for (_name, ty) in resolve.interfaces[export].types.iter() {
            if !matches!(resolve.types[*ty].kind, TypeDefKind::Resource) {
                continue;
            }
            let idx = match nested.encode_valtype(resolve, &Type::Id(*ty))? {
                ComponentValType::Type(idx) => idx,
                _ => unreachable!(),
            };
            resources.insert(*ty, idx);
        }

        // Next import each function of this interface. This will end up
        // defining local types as necessary or using the types as imported
        // above.
        for (_, func) in resolve.interfaces[export].functions.iter() {
            let ty = nested.encode_func_type(resolve, func)?;
            nested
                .component
                .import(&import_func_name(func), ComponentTypeRef::Func(ty));
        }

        // Swap the `nested.type_map` which was previously from `TypeId` to
        // `u32` to instead being from `u32` to `TypeId`. This reverse map is
        // then used in conjunction with `self.type_map` to satisfy all type
        // imports of the nested component generated. The type import's index in
        // the inner component is translated to a `TypeId` via `reverse_map`
        // which is then translated back to our own index space via `type_map`.
        let reverse_map = nested
            .type_encoding_maps
            .id_to_index
            .drain()
            .map(|p| (p.1, p.0))
            .collect::<HashMap<_, _>>();
        nested.type_encoding_maps.def_to_index.clear();
        for (name, idx) in nested.imports.drain(..) {
            let id = reverse_map[&idx];
            let owner = match resolve.types[id].owner {
                TypeOwner::Interface(id) => id,
                _ => unreachable!(),
            };
            let idx = if owner == export || exports_used.contains(&owner) {
                log::trace!("consulting exports for {id:?}");
                nested.state.export_type_encoding_maps.id_to_index[&id]
            } else {
                log::trace!("consulting imports for {id:?}");
                nested.state.import_type_encoding_maps.id_to_index[&id]
            };
            imports.push((name, ComponentExportKind::Type, idx))
        }

        // Before encoding exports reset the type map to what all was imported
        // from foreign interfaces. This will enable any encoded types below to
        // refer to imports which, after type substitution, will point to the
        // correct type in the outer component context.
        nested.type_encoding_maps = imported_type_maps;

        // Next the component reexports all of its imports, but notably uses the
        // type ascription feature to change the type of the function. Note that
        // no structural change is happening to the types here but instead types
        // are getting proper names and such now that this nested component is a
        // new type index space. Hence the `export_types = true` flag here which
        // flows through the type encoding and when types are emitted.
        nested.export_types = true;
        nested.type_encoding_maps.func_type_map.clear();

        // To start off all type information is encoded. This will be used by
        // functions below but notably this also has special handling for
        // resources. Resources reexport their imported resource type under
        // the final name which achieves the desired goal of threading through
        // the original resource without creating a new one.
        for (_, id) in resolve.interfaces[export].types.iter() {
            let ty = &resolve.types[*id];
            match ty.kind {
                TypeDefKind::Resource => {
                    let idx = nested.component.export(
                        ty.name.as_ref().expect("resources must be named"),
                        ComponentExportKind::Type,
                        resources[id],
                        None,
                    );
                    nested.type_encoding_maps.id_to_index.insert(*id, idx);
                }
                _ => {
                    nested.encode_valtype(resolve, &Type::Id(*id))?;
                }
            }
        }

        for (i, (_, func)) in resolve.interfaces[export].functions.iter().enumerate() {
            let ty = nested.encode_func_type(resolve, func)?;
            nested.component.export(
                &func.name,
                ComponentExportKind::Func,
                i as u32,
                Some(ComponentTypeRef::Func(ty)),
            );
        }

        // Embed the component within our component and then instantiate it with
        // the lifted functions. That final instance is then exported under the
        // appropriate name as the final typed export of this component.
        let component = nested.component;
        let component_index = self
            .component
            .component(Some(&format!("{export_name}-shim-component")), component);
        let instance_index = self.component.instantiate(
            Some(&format!("{export_name}-shim-instance")),
            component_index,
            imports,
        );
        let idx = self.component.export(
            export_name,
            ComponentExportKind::Instance,
            instance_index,
            None,
        );
        let prev = self.exported_instances.insert(export, idx);
        assert!(prev.is_none());

        // After everything is all said and done remove all the type information
        // about type exports of this interface. Any entries in the map
        // currently were used to create the instance above but aren't the
        // actual copy of the exported type since that comes from the exported
        // instance itself. Entries will be re-inserted into this map as
        // necessary via aliases from the exported instance which is the new
        // source of truth for all these types.
        for (_name, id) in resolve.interfaces[export].types.iter() {
            self.export_type_encoding_maps.id_to_index.remove(id);
            self.export_type_encoding_maps
                .def_to_index
                .remove(&resolve.types[*id].kind);
        }

        return Ok(());

        struct NestedComponentTypeEncoder<'state, 'a> {
            component: ComponentBuilder,
            type_encoding_maps: TypeEncodingMaps<'a>,
            export_types: bool,
            interface: InterfaceId,
            state: &'state mut EncodingState<'a>,
            imports: IndexMap<String, u32>,
        }

        impl<'a> ValtypeEncoder<'a> for NestedComponentTypeEncoder<'_, 'a> {
            fn defined_type(&mut self) -> (u32, ComponentDefinedTypeEncoder<'_>) {
                self.component.type_defined(None)
            }
            fn define_function_type(&mut self) -> (u32, ComponentFuncTypeEncoder<'_>) {
                self.component.type_function(None)
            }
            fn export_type(&mut self, idx: u32, name: &'a str) -> Option<u32> {
                if self.export_types {
                    Some(
                        self.component
                            .export(name, ComponentExportKind::Type, idx, None),
                    )
                } else {
                    let name = self.unique_import_name(name);
                    let ret = self
                        .component
                        .import(&name, ComponentTypeRef::Type(TypeBounds::Eq(idx)));
                    self.imports.insert(name, ret);
                    Some(ret)
                }
            }
            fn export_resource(&mut self, name: &'a str) -> u32 {
                if self.export_types {
                    panic!("resources should already be exported")
                } else {
                    let name = self.unique_import_name(name);
                    let ret = self
                        .component
                        .import(&name, ComponentTypeRef::Type(TypeBounds::SubResource));
                    self.imports.insert(name, ret);
                    ret
                }
            }
            fn import_type(&mut self, _: InterfaceId, _id: TypeId) -> u32 {
                unreachable!()
            }
            fn type_encoding_maps(&mut self) -> &mut TypeEncodingMaps<'a> {
                &mut self.type_encoding_maps
            }
            fn interface(&self) -> Option<InterfaceId> {
                Some(self.interface)
            }
        }

        impl NestedComponentTypeEncoder<'_, '_> {
            fn unique_import_name(&mut self, name: &str) -> String {
                let mut name = format!("import-type-{name}");
                let mut n = 0;
                while self.imports.contains_key(&name) {
                    name = format!("{name}{n}");
                    n += 1;
                }
                name
            }
        }

        fn import_func_name(f: &Function) -> String {
            match f.kind {
                FunctionKind::Freestanding | FunctionKind::AsyncFreestanding => {
                    format!("import-func-{}", f.item_name())
                }

                // transform `[method]foo.bar` into `import-method-foo-bar` to
                // have it be a valid kebab-name which can't conflict with
                // anything else.
                //
                // There's probably a better and more "formal" way to do this
                // but quick-and-dirty string manipulation should work well
                // enough for now hopefully.
                FunctionKind::Method(_)
                | FunctionKind::AsyncMethod(_)
                | FunctionKind::Static(_)
                | FunctionKind::AsyncStatic(_)
                | FunctionKind::Constructor(_) => {
                    format!(
                        "import-{}",
                        f.name.replace('[', "").replace([']', '.', ' '], "-")
                    )
                }
            }
        }
    }

    fn encode_lift(
        &mut self,
        module: CustomModule<'_>,
        core_name: &str,
        key: &WorldKey,
        func: &Function,
        ty: u32,
    ) -> Result<u32> {
        let resolve = &self.info.encoder.metadata.resolve;
        let metadata = self.info.module_metadata_for(module);
        let instance_index = self.instance_for(module);
        let core_func_index =
            self.core_alias_export(Some(core_name), instance_index, core_name, ExportKind::Func);
        let exports = self.info.exports_for(module);

        let options = RequiredOptions::for_export(
            resolve,
            func,
            exports
                .abi(key, func)
                .ok_or_else(|| anyhow!("no ABI found for {}", func.name))?,
        );

        let encoding = metadata
            .export_encodings
            .get(resolve, key, &func.name)
            .unwrap();
        let exports = self.info.exports_for(module);
        let realloc_index = exports
            .export_realloc_for(key, &func.name)
            .map(|name| self.core_alias_export(Some(name), instance_index, name, ExportKind::Func));
        let mut options = options
            .into_iter(encoding, self.memory_index, realloc_index)?
            .collect::<Vec<_>>();

        if let Some(post_return) = exports.post_return(key, func) {
            let post_return = self.core_alias_export(
                Some(post_return),
                instance_index,
                post_return,
                ExportKind::Func,
            );
            options.push(CanonicalOption::PostReturn(post_return));
        }
        if let Some(callback) = exports.callback(key, func) {
            let callback =
                self.core_alias_export(Some(callback), instance_index, callback, ExportKind::Func);
            options.push(CanonicalOption::Callback(callback));
        }
        let func_index = self
            .component
            .lift_func(Some(&func.name), core_func_index, ty, options);
        Ok(func_index)
    }

    fn encode_shim_instantiation(&mut self) -> Result<Shims<'a>> {
        let mut ret = Shims::default();

        ret.append_indirect(self.info, CustomModule::Main)
            .context("failed to register indirect shims for main module")?;

        // For all required adapter modules a shim is created for each required
        // function and additionally a set of shims are created for the
        // interface imported into the shim module itself.
        for (adapter_name, _adapter) in self.info.adapters.iter() {
            ret.append_indirect(self.info, CustomModule::Adapter(adapter_name))
                .with_context(|| {
                    format!("failed to register indirect shims for adapter {adapter_name}")
                })?;
        }

        if ret.shims.is_empty() {
            return Ok(ret);
        }

        assert!(self.shim_instance_index.is_none());
        assert!(self.fixups_module_index.is_none());

        // This function encodes two modules:
        // - A shim module that defines a table and exports functions
        //   that indirectly call through the table.
        // - A fixup module that imports that table and a set of functions
        //   and populates the imported table via active element segments. The
        //   fixup module is used to populate the shim's table once the
        //   imported functions have been lowered.

        let mut types = TypeSection::new();
        let mut tables = TableSection::new();
        let mut functions = FunctionSection::new();
        let mut exports = ExportSection::new();
        let mut code = CodeSection::new();
        let mut sigs = IndexMap::new();
        let mut imports_section = ImportSection::new();
        let mut elements = ElementSection::new();
        let mut func_indexes = Vec::new();
        let mut func_names = NameMap::new();

        for (i, shim) in ret.shims.values().enumerate() {
            let i = i as u32;
            let type_index = *sigs.entry(&shim.sig).or_insert_with(|| {
                let index = types.len();
                types.ty().function(
                    shim.sig.params.iter().map(to_val_type),
                    shim.sig.results.iter().map(to_val_type),
                );
                index
            });

            functions.function(type_index);
            Self::encode_shim_function(type_index, i, &mut code, shim.sig.params.len() as u32);
            exports.export(&shim.name, ExportKind::Func, i);

            imports_section.import("", &shim.name, EntityType::Function(type_index));
            func_indexes.push(i);
            func_names.append(i, &shim.debug_name);
        }
        let mut names = NameSection::new();
        names.module("wit-component:shim");
        names.functions(&func_names);

        let table_type = TableType {
            element_type: RefType::FUNCREF,
            minimum: ret.shims.len() as u64,
            maximum: Some(ret.shims.len() as u64),
            table64: false,
            shared: false,
        };

        tables.table(table_type);

        exports.export(INDIRECT_TABLE_NAME, ExportKind::Table, 0);
        imports_section.import("", INDIRECT_TABLE_NAME, table_type);

        elements.active(
            None,
            &ConstExpr::i32_const(0),
            Elements::Functions(func_indexes.into()),
        );

        let mut shim = Module::new();
        shim.section(&types);
        shim.section(&functions);
        shim.section(&tables);
        shim.section(&exports);
        shim.section(&code);
        shim.section(&RawCustomSection(
            &crate::base_producers().raw_custom_section(),
        ));
        if self.info.encoder.debug_names {
            shim.section(&names);
        }

        let mut fixups = Module::default();
        fixups.section(&types);
        fixups.section(&imports_section);
        fixups.section(&elements);
        fixups.section(&RawCustomSection(
            &crate::base_producers().raw_custom_section(),
        ));

        if self.info.encoder.debug_names {
            let mut names = NameSection::new();
            names.module("wit-component:fixups");
            fixups.section(&names);
        }

        let shim_module_index = self
            .component
            .core_module(Some("wit-component-shim-module"), &shim);
        let fixup_index = self
            .component
            .core_module(Some("wit-component-fixup"), &fixups);
        self.fixups_module_index = Some(fixup_index);
        let shim_instance = self.component.core_instantiate(
            Some("wit-component-shim-instance"),
            shim_module_index,
            [],
        );
        self.shim_instance_index = Some(shim_instance);

        return Ok(ret);
    }

    fn encode_shim_function(
        type_index: u32,
        func_index: u32,
        code: &mut CodeSection,
        param_count: u32,
    ) {
        let mut func = wasm_encoder::Function::new(std::iter::empty());
        for i in 0..param_count {
            func.instructions().local_get(i);
        }
        func.instructions().i32_const(func_index as i32);
        func.instructions().call_indirect(0, type_index);
        func.instructions().end();
        code.function(&func);
    }

    fn encode_indirect_lowerings(&mut self, shims: &Shims<'_>) -> Result<()> {
        if shims.shims.is_empty() {
            return Ok(());
        }

        let shim_instance_index = self
            .shim_instance_index
            .expect("must have an instantiated shim");

        let table_index = self.core_alias_export(
            Some("shim table"),
            shim_instance_index,
            INDIRECT_TABLE_NAME,
            ExportKind::Table,
        );

        let resolve = &self.info.encoder.metadata.resolve;

        let mut exports = Vec::new();
        exports.push((INDIRECT_TABLE_NAME, ExportKind::Table, table_index));

        for shim in shims.shims.values() {
            let core_func_index = match &shim.kind {
                // Indirect lowerings are a `canon lower`'d function with
                // options specified from a previously instantiated instance.
                // This previous instance could either be the main module or an
                // adapter module, which affects the `realloc` option here.
                // Currently only one linear memory is supported so the linear
                // memory always comes from the main module.
                ShimKind::IndirectLowering {
                    interface,
                    index,
                    realloc,
                    encoding,
                } => {
                    let interface = &self.info.import_map[interface];
                    let ((name, _), _) = interface.lowerings.get_index(*index).unwrap();
                    let func_index = match &interface.interface {
                        Some(interface_id) => {
                            let instance_index = self.imported_instances[interface_id];
                            self.component.alias_export(
                                instance_index,
                                name,
                                ComponentExportKind::Func,
                            )
                        }
                        None => self.imported_funcs[name],
                    };

                    let realloc = self
                        .info
                        .exports_for(*realloc)
                        .import_realloc_for(interface.interface, name)
                        .map(|name| {
                            let instance = self.instance_for(*realloc);
                            self.core_alias_export(
                                Some("realloc"),
                                instance,
                                name,
                                ExportKind::Func,
                            )
                        });

                    self.component.lower_func(
                        Some(&shim.debug_name),
                        func_index,
                        shim.options
                            .into_iter(*encoding, self.memory_index, realloc)?,
                    )
                }

                // Adapter shims are defined by an export from an adapter
                // instance, so use the specified name here and the previously
                // created instances to get the core item that represents the
                // shim.
                ShimKind::Adapter { adapter, func } => self.core_alias_export(
                    Some(func),
                    self.adapter_instances[adapter],
                    func,
                    ExportKind::Func,
                ),

                // Resources are required for a module to be instantiated
                // meaning that any destructor for the resource must be called
                // indirectly due to the otherwise circular dependency between
                // the module and the resource itself.
                ShimKind::ResourceDtor { module, export } => self.core_alias_export(
                    Some(export),
                    self.instance_for(*module),
                    export,
                    ExportKind::Func,
                ),

                ShimKind::PayloadFunc {
                    for_module,
                    info,
                    kind,
                } => {
                    let metadata = self.info.module_metadata_for(*for_module);
                    let exports = self.info.exports_for(*for_module);
                    let instance_index = self.instance_for(*for_module);
                    let (encoding, realloc) = match &info.ty {
                        PayloadType::Type { function, .. } => {
                            if info.imported {
                                (
                                    metadata.import_encodings.get(resolve, &info.key, function),
                                    exports.import_realloc_for(info.interface, function),
                                )
                            } else {
                                (
                                    metadata.export_encodings.get(resolve, &info.key, function),
                                    exports.export_realloc_for(&info.key, function),
                                )
                            }
                        }
                        PayloadType::UnitFuture | PayloadType::UnitStream => (None, None),
                    };
                    let encoding = encoding.unwrap_or(StringEncoding::UTF8);
                    let realloc_index = realloc.map(|name| {
                        self.core_alias_export(
                            Some("realloc"),
                            instance_index,
                            name,
                            ExportKind::Func,
                        )
                    });
                    let type_index = self.payload_type_index(info)?;
                    let options =
                        shim.options
                            .into_iter(encoding, self.memory_index, realloc_index)?;

                    match kind {
                        PayloadFuncKind::FutureWrite => {
                            self.component.future_write(type_index, options)
                        }
                        PayloadFuncKind::FutureRead => {
                            self.component.future_read(type_index, options)
                        }
                        PayloadFuncKind::StreamWrite => {
                            self.component.stream_write(type_index, options)
                        }
                        PayloadFuncKind::StreamRead => {
                            self.component.stream_read(type_index, options)
                        }
                    }
                }

                ShimKind::WaitableSetWait { cancellable } => self
                    .component
                    .waitable_set_wait(*cancellable, self.memory_index.unwrap()),
                ShimKind::WaitableSetPoll { cancellable } => self
                    .component
                    .waitable_set_poll(*cancellable, self.memory_index.unwrap()),
                ShimKind::ErrorContextNew { encoding } => self.component.error_context_new(
                    shim.options.into_iter(*encoding, self.memory_index, None)?,
                ),
                ShimKind::ErrorContextDebugMessage {
                    for_module,
                    encoding,
                } => {
                    let instance_index = self.instance_for(*for_module);
                    let realloc = self.info.exports_for(*for_module).import_realloc_fallback();
                    let realloc_index = realloc.map(|r| {
                        self.core_alias_export(Some("realloc"), instance_index, r, ExportKind::Func)
                    });

                    self.component
                        .error_context_debug_message(shim.options.into_iter(
                            *encoding,
                            self.memory_index,
                            realloc_index,
                        )?)
                }
                ShimKind::TaskReturn {
                    interface,
                    func,
                    result,
                    encoding,
                    for_module,
                } => {
                    // See `Import::ExportedTaskReturn` handling for why this
                    // encoder is treated specially.
                    let mut encoder = if interface.is_none() {
                        self.root_import_type_encoder(*interface)
                    } else {
                        self.root_export_type_encoder(*interface)
                    };
                    let result = match result {
                        Some(ty) => Some(encoder.encode_valtype(resolve, ty)?),
                        None => None,
                    };

                    let exports = self.info.exports_for(*for_module);
                    let realloc = exports.import_realloc_for(*interface, func);

                    let instance_index = self.instance_for(*for_module);
                    let realloc_index = realloc.map(|r| {
                        self.core_alias_export(Some("realloc"), instance_index, r, ExportKind::Func)
                    });
                    let options =
                        shim.options
                            .into_iter(*encoding, self.memory_index, realloc_index)?;
                    self.component.task_return(result, options)
                }
                ShimKind::ThreadNewIndirect {
                    for_module,
                    func_ty,
                } => {
                    // Encode the function type for the thread start function so we can reference it in the `canon` call.
                    let (func_ty_idx, f) = self.component.core_type(Some("thread-start"));
                    f.core().func_type(func_ty);

                    // In order for the funcref table referenced by `thread.new-indirect` to be used,
                    // it must have been exported by the module.
                    let exports = self.info.exports_for(*for_module);
                    let instance_index = self.instance_for(*for_module);
                    let table_idx = exports.indirect_function_table().map(|table| {
                        self.core_alias_export(
                            Some("indirect-function-table"),
                            instance_index,
                            table,
                            ExportKind::Table,
                        )
                    }).ok_or_else(|| {
                        anyhow!(
                            "table __indirect_function_table must be an exported funcref table for thread.new-indirect"
                        )
                    })?;

                    self.component.thread_new_indirect(func_ty_idx, table_idx)
                }
            };

            exports.push((shim.name.as_str(), ExportKind::Func, core_func_index));
        }

        let instance_index = self
            .component
            .core_instantiate_exports(Some("fixup-args"), exports);
        self.component.core_instantiate(
            Some("fixup"),
            self.fixups_module_index.expect("must have fixup module"),
            [("", ModuleArg::Instance(instance_index))],
        );
        Ok(())
    }

    /// Encode the specified `stream` or `future` type in the component using
    /// either the `root_import_type_encoder` or the `root_export_type_encoder`
    /// depending on the value of `imported`.
    ///
    /// Note that the payload type `T` of `stream<T>` or `future<T>` may be an
    /// imported or exported type, and that determines the appropriate type
    /// encoder to use.
    fn payload_type_index(&mut self, info: &PayloadInfo) -> Result<u32> {
        let resolve = &self.info.encoder.metadata.resolve;
        // What exactly is selected here as the encoder is a bit unusual here.
        // If the interface is imported, an import encoder is used. An import
        // encoder is also used though if `info` is exported and
        // `info.interface` is `None`, meaning that this is for a function that
        // is in the top-level of a world. At the top level of a world all
        // types are imported.
        //
        // Additionally for the import encoder the interface passed in is
        // `None`, not `info.interface`. Notably this means that references to
        // named types will be aliased from their imported versions, which is
        // what we want here.
        //
        // Finally though exports do use `info.interface`. Honestly I'm not
        // really entirely sure why. Fuzzing is happy though, and truly
        // everything must be ok if the fuzzers are happy, right?
        let mut encoder = if info.imported || info.interface.is_none() {
            self.root_import_type_encoder(None)
        } else {
            self.root_export_type_encoder(info.interface)
        };
        match info.ty {
            PayloadType::Type { id, .. } => match encoder.encode_valtype(resolve, &Type::Id(id))? {
                ComponentValType::Type(index) => Ok(index),
                ComponentValType::Primitive(_) => unreachable!(),
            },
            PayloadType::UnitFuture => Ok(encoder.encode_unit_future()),
            PayloadType::UnitStream => Ok(encoder.encode_unit_stream()),
        }
    }

    /// This is a helper function that will declare any types necessary for
    /// declaring intrinsics that are imported into the module or adapter.
    ///
    /// For example resources must be declared to generate
    /// destructors/constructors/etc. Additionally types must also be declared
    /// for `task.return` with the component model async feature.
    fn declare_types_for_imported_intrinsics(&mut self, shims: &Shims<'_>) -> Result<()> {
        let resolve = &self.info.encoder.metadata.resolve;
        let world = &resolve.worlds[self.info.encoder.metadata.world];

        // Iterate over the main module's exports and the exports of all
        // adapters. Look for exported interfaces.
        let main_module_keys = self.info.encoder.main_module_exports.iter();
        let main_module_keys = main_module_keys.map(|key| (CustomModule::Main, key));
        let adapter_keys = self.info.encoder.adapters.iter().flat_map(|(name, info)| {
            info.required_exports
                .iter()
                .map(move |key| (CustomModule::Adapter(name), key))
        });
        for (for_module, key) in main_module_keys.chain(adapter_keys) {
            let id = match &world.exports[key] {
                WorldItem::Interface { id, .. } => *id,
                WorldItem::Type { .. } => unreachable!(),
                WorldItem::Function(_) => continue,
            };

            for ty in resolve.interfaces[id].types.values() {
                let def = &resolve.types[*ty];
                match &def.kind {
                    // Declare exported resources specially as they generally
                    // need special treatment for later handling exports and
                    // such.
                    TypeDefKind::Resource => {
                        // Load the destructor, previously detected in module
                        // validation, if one is present.
                        let exports = self.info.exports_for(for_module);
                        let dtor = exports.resource_dtor(*ty).map(|name| {
                            let shim = &shims.shims[&ShimKind::ResourceDtor {
                                module: for_module,
                                export: name,
                            }];
                            let index = self.shim_instance_index.unwrap();
                            self.core_alias_export(
                                Some(&shim.debug_name),
                                index,
                                &shim.name,
                                ExportKind::Func,
                            )
                        });

                        // Declare the resource with this destructor and register it in
                        // our internal map. This should be the first and only time this
                        // type is inserted into this map.
                        let resource_idx = self.component.type_resource(
                            Some(def.name.as_ref().unwrap()),
                            ValType::I32,
                            dtor,
                        );
                        let prev = self
                            .export_type_encoding_maps
                            .id_to_index
                            .insert(*ty, resource_idx);
                        assert!(prev.is_none());
                    }
                    _other => {
                        self.root_export_type_encoder(Some(id))
                            .encode_valtype(resolve, &Type::Id(*ty))?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Helper to instantiate the main module and record various results of its
    /// instantiation within `self`.
    fn instantiate_main_module(&mut self, shims: &Shims<'_>) -> Result<()> {
        assert!(self.instance_index.is_none());

        let instance_index = self.instantiate_core_module(shims, CustomModule::Main)?;

        if let Some(memory) = self.info.info.exports.memory() {
            self.memory_index = Some(self.core_alias_export(
                Some("memory"),
                instance_index,
                memory,
                ExportKind::Memory,
            ));
        }

        self.instance_index = Some(instance_index);
        Ok(())
    }

    /// This function will instantiate the specified adapter module, which may
    /// depend on previously-instantiated modules.
    fn instantiate_adapter_module(&mut self, shims: &Shims<'_>, name: &'a str) -> Result<()> {
        let instance = self.instantiate_core_module(shims, CustomModule::Adapter(name))?;
        self.adapter_instances.insert(name, instance);
        Ok(())
    }

    /// Generic helper to instantiate a module.
    ///
    /// The `for_module` provided will have all of its imports satisfied from
    /// either previous instantiations or the `shims` module present. This
    /// iterates over the metadata produced during validation to determine what
    /// hooks up to what import.
    fn instantiate_core_module(
        &mut self,
        shims: &Shims,
        for_module: CustomModule<'_>,
    ) -> Result<u32> {
        let module = self.module_for(for_module);

        let mut args = Vec::new();
        for (core_wasm_name, instance) in self.info.imports_for(for_module).modules() {
            match instance {
                // For import modules that are a "bag of names" iterate over
                // each name and materialize it into this component with the
                // `materialize_import` helper. This is then all bottled up into
                // a bag-of-exports instance which is then used for
                // instantiation.
                ImportInstance::Names(names) => {
                    let mut exports = Vec::new();
                    for (name, import) in names {
                        log::trace!(
                            "attempting to materialize import of `{core_wasm_name}::{name}` for {for_module:?}"
                        );
                        let (kind, index) = self
                            .materialize_import(&shims, for_module, import)
                            .with_context(|| {
                                format!("failed to satisfy import `{core_wasm_name}::{name}`")
                            })?;
                        exports.push((name.as_str(), kind, index));
                    }
                    let index = self
                        .component
                        .core_instantiate_exports(Some(core_wasm_name), exports);
                    args.push((core_wasm_name.as_str(), ModuleArg::Instance(index)));
                }

                // Some imports are entire instances, so use the instance for
                // the module identifier as the import.
                ImportInstance::Whole(which) => {
                    let instance = self.instance_for(which.to_custom_module());
                    args.push((core_wasm_name.as_str(), ModuleArg::Instance(instance)));
                }
            }
        }

        // And with all arguments prepared now, instantiate the module.
        Ok(self
            .component
            .core_instantiate(Some(for_module.debug_name()), module, args))
    }

    /// Helper function to materialize an import into a core module within the
    /// component being built.
    ///
    /// This function is called for individual imports and uses the results of
    /// validation, notably the `Import` type, to determine what WIT-level or
    /// component-level construct is being hooked up.
    fn materialize_import(
        &mut self,
        shims: &Shims<'_>,
        for_module: CustomModule<'_>,
        import: &'a Import,
    ) -> Result<(ExportKind, u32)> {
        let resolve = &self.info.encoder.metadata.resolve;
        match import {
            // Main module dependencies on an adapter in use are done with an
            // indirection here, so load the shim function and use that.
            Import::AdapterExport {
                adapter,
                func,
                ty: _,
            } => {
                assert!(self.info.encoder.adapters.contains_key(adapter));
                Ok(self.materialize_shim_import(shims, &ShimKind::Adapter { adapter, func }))
            }

            // Adapters might use the main module's memory, in which case it
            // should have been previously instantiated.
            Import::MainModuleMemory => {
                let index = self
                    .memory_index
                    .ok_or_else(|| anyhow!("main module cannot import memory"))?;
                Ok((ExportKind::Memory, index))
            }

            // Grab-bag of "this adapter wants this thing from the main module".
            Import::MainModuleExport { name, kind } => {
                let instance = self.instance_index.unwrap();
                let index = self.core_alias_export(Some(name), instance, name, *kind);
                Ok((*kind, index))
            }

            // A similar grab-bag to above but with a slightly different
            // structure. Should probably refactor to make these two the same in
            // the future.
            Import::Item(item) => {
                let instance = self.instance_for(item.which.to_custom_module());
                let index =
                    self.core_alias_export(Some(&item.name), instance, &item.name, item.kind);
                Ok((item.kind, index))
            }

            // Resource intrinsics related to exported resources. Despite being
            // an exported resource the component still provides necessary
            // intrinsics for manipulating resource state. These are all
            // handled here using the resource types created during
            // `declare_types_for_imported_intrinsics` above.
            Import::ExportedResourceDrop(_key, id) => {
                let index = self
                    .component
                    .resource_drop(self.export_type_encoding_maps.id_to_index[id]);
                Ok((ExportKind::Func, index))
            }
            Import::ExportedResourceRep(_key, id) => {
                let index = self
                    .component
                    .resource_rep(self.export_type_encoding_maps.id_to_index[id]);
                Ok((ExportKind::Func, index))
            }
            Import::ExportedResourceNew(_key, id) => {
                let index = self
                    .component
                    .resource_new(self.export_type_encoding_maps.id_to_index[id]);
                Ok((ExportKind::Func, index))
            }

            // And finally here at the end these cases are going to all fall
            // through to the code below. This is where these are connected to a
            // WIT `ImportedInterface` one way or another with the name that was
            // detected during validation.
            Import::ImportedResourceDrop(key, iface, id) => {
                let ty = &resolve.types[*id];
                let name = ty.name.as_ref().unwrap();
                self.materialize_wit_import(
                    shims,
                    for_module,
                    iface.map(|_| resolve.name_world_key(key)),
                    &format!("{name}_drop"),
                    key,
                    AbiVariant::GuestImport,
                )
            }
            Import::ExportedTaskReturn(key, interface, func, result) => {
                let (options, _sig) = task_return_options_and_type(resolve, *result);
                if options.is_empty() {
                    // Note that an "import type encoder" is used here despite
                    // this being for an exported function if the `interface`
                    // is none, meaning that this is for a top-level world
                    // function. In that situation all types that can be
                    // referred to are imported, not exported.
                    let mut encoder = if interface.is_none() {
                        self.root_import_type_encoder(*interface)
                    } else {
                        self.root_export_type_encoder(*interface)
                    };

                    let result = match result {
                        Some(ty) => Some(encoder.encode_valtype(resolve, ty)?),
                        None => None,
                    };
                    let index = self.component.task_return(result, []);
                    Ok((ExportKind::Func, index))
                } else {
                    let metadata = &self.info.module_metadata_for(for_module);
                    let encoding = metadata.export_encodings.get(resolve, key, func).unwrap();
                    Ok(self.materialize_shim_import(
                        shims,
                        &ShimKind::TaskReturn {
                            for_module,
                            interface: *interface,
                            func,
                            result: *result,
                            encoding,
                        },
                    ))
                }
            }
            Import::BackpressureInc => {
                let index = self.component.backpressure_inc();
                Ok((ExportKind::Func, index))
            }
            Import::BackpressureDec => {
                let index = self.component.backpressure_dec();
                Ok((ExportKind::Func, index))
            }
            Import::WaitableSetWait { cancellable } => Ok(self.materialize_shim_import(
                shims,
                &ShimKind::WaitableSetWait {
                    cancellable: *cancellable,
                },
            )),
            Import::WaitableSetPoll { cancellable } => Ok(self.materialize_shim_import(
                shims,
                &ShimKind::WaitableSetPoll {
                    cancellable: *cancellable,
                },
            )),
            Import::ThreadYield { cancellable } => {
                let index = self.component.thread_yield(*cancellable);
                Ok((ExportKind::Func, index))
            }
            Import::SubtaskDrop => {
                let index = self.component.subtask_drop();
                Ok((ExportKind::Func, index))
            }
            Import::SubtaskCancel { async_ } => {
                let index = self.component.subtask_cancel(*async_);
                Ok((ExportKind::Func, index))
            }
            Import::StreamNew(info) => {
                let ty = self.payload_type_index(info)?;
                let index = self.component.stream_new(ty);
                Ok((ExportKind::Func, index))
            }
            Import::StreamRead { info, .. } => Ok(self.materialize_payload_import(
                shims,
                for_module,
                info,
                PayloadFuncKind::StreamRead,
            )),
            Import::StreamWrite { info, .. } => Ok(self.materialize_payload_import(
                shims,
                for_module,
                info,
                PayloadFuncKind::StreamWrite,
            )),
            Import::StreamCancelRead { info, async_ } => {
                let ty = self.payload_type_index(info)?;
                let index = self.component.stream_cancel_read(ty, *async_);
                Ok((ExportKind::Func, index))
            }
            Import::StreamCancelWrite { info, async_ } => {
                let ty = self.payload_type_index(info)?;
                let index = self.component.stream_cancel_write(ty, *async_);
                Ok((ExportKind::Func, index))
            }
            Import::StreamDropReadable(info) => {
                let type_index = self.payload_type_index(info)?;
                let index = self.component.stream_drop_readable(type_index);
                Ok((ExportKind::Func, index))
            }
            Import::StreamDropWritable(info) => {
                let type_index = self.payload_type_index(info)?;
                let index = self.component.stream_drop_writable(type_index);
                Ok((ExportKind::Func, index))
            }
            Import::FutureNew(info) => {
                let ty = self.payload_type_index(info)?;
                let index = self.component.future_new(ty);
                Ok((ExportKind::Func, index))
            }
            Import::FutureRead { info, .. } => Ok(self.materialize_payload_import(
                shims,
                for_module,
                info,
                PayloadFuncKind::FutureRead,
            )),
            Import::FutureWrite { info, .. } => Ok(self.materialize_payload_import(
                shims,
                for_module,
                info,
                PayloadFuncKind::FutureWrite,
            )),
            Import::FutureCancelRead { info, async_ } => {
                let ty = self.payload_type_index(info)?;
                let index = self.component.future_cancel_read(ty, *async_);
                Ok((ExportKind::Func, index))
            }
            Import::FutureCancelWrite { info, async_ } => {
                let ty = self.payload_type_index(info)?;
                let index = self.component.future_cancel_write(ty, *async_);
                Ok((ExportKind::Func, index))
            }
            Import::FutureDropReadable(info) => {
                let type_index = self.payload_type_index(info)?;
                let index = self.component.future_drop_readable(type_index);
                Ok((ExportKind::Func, index))
            }
            Import::FutureDropWritable(info) => {
                let type_index = self.payload_type_index(info)?;
                let index = self.component.future_drop_writable(type_index);
                Ok((ExportKind::Func, index))
            }
            Import::ErrorContextNew { encoding } => Ok(self.materialize_shim_import(
                shims,
                &ShimKind::ErrorContextNew {
                    encoding: *encoding,
                },
            )),
            Import::ErrorContextDebugMessage { encoding } => Ok(self.materialize_shim_import(
                shims,
                &ShimKind::ErrorContextDebugMessage {
                    for_module,
                    encoding: *encoding,
                },
            )),
            Import::ErrorContextDrop => {
                let index = self.component.error_context_drop();
                Ok((ExportKind::Func, index))
            }
            Import::WorldFunc(key, name, abi) => {
                self.materialize_wit_import(shims, for_module, None, name, key, *abi)
            }
            Import::InterfaceFunc(key, _, name, abi) => self.materialize_wit_import(
                shims,
                for_module,
                Some(resolve.name_world_key(key)),
                name,
                key,
                *abi,
            ),

            Import::WaitableSetNew => {
                let index = self.component.waitable_set_new();
                Ok((ExportKind::Func, index))
            }
            Import::WaitableSetDrop => {
                let index = self.component.waitable_set_drop();
                Ok((ExportKind::Func, index))
            }
            Import::WaitableJoin => {
                let index = self.component.waitable_join();
                Ok((ExportKind::Func, index))
            }
            Import::ContextGet(n) => {
                let index = self.component.context_get(*n);
                Ok((ExportKind::Func, index))
            }
            Import::ContextSet(n) => {
                let index = self.component.context_set(*n);
                Ok((ExportKind::Func, index))
            }
            Import::ExportedTaskCancel => {
                let index = self.component.task_cancel();
                Ok((ExportKind::Func, index))
            }
            Import::ThreadIndex => {
                let index = self.component.thread_index();
                Ok((ExportKind::Func, index))
            }
            Import::ThreadNewIndirect => Ok(self.materialize_shim_import(
                shims,
                &ShimKind::ThreadNewIndirect {
                    for_module,
                    // This is fixed for now
                    func_ty: FuncType::new([ValType::I32], []),
                },
            )),
            Import::ThreadSwitchTo { cancellable } => {
                let index = self.component.thread_switch_to(*cancellable);
                Ok((ExportKind::Func, index))
            }
            Import::ThreadSuspend { cancellable } => {
                let index = self.component.thread_suspend(*cancellable);
                Ok((ExportKind::Func, index))
            }
            Import::ThreadResumeLater => {
                let index = self.component.thread_resume_later();
                Ok((ExportKind::Func, index))
            }
            Import::ThreadYieldTo { cancellable } => {
                let index = self.component.thread_yield_to(*cancellable);
                Ok((ExportKind::Func, index))
            }
        }
    }

    /// Helper for `materialize_import` above for materializing functions that
    /// are part of the "shim module" generated.
    fn materialize_shim_import(&mut self, shims: &Shims<'_>, kind: &ShimKind) -> (ExportKind, u32) {
        let index = self.core_alias_export(
            Some(&shims.shims[kind].debug_name),
            self.shim_instance_index
                .expect("shim should be instantiated"),
            &shims.shims[kind].name,
            ExportKind::Func,
        );
        (ExportKind::Func, index)
    }

    /// Helper for `materialize_import` above for generating imports for
    /// future/stream read/write intrinsics.
    fn materialize_payload_import(
        &mut self,
        shims: &Shims<'_>,
        for_module: CustomModule<'_>,
        info: &PayloadInfo,
        kind: PayloadFuncKind,
    ) -> (ExportKind, u32) {
        self.materialize_shim_import(
            shims,
            &ShimKind::PayloadFunc {
                for_module,
                info,
                kind,
            },
        )
    }

    /// Helper for `materialize_import` above which specifically operates on
    /// WIT-level functions identified by `interface_key`, `name`, and `abi`.
    fn materialize_wit_import(
        &mut self,
        shims: &Shims<'_>,
        for_module: CustomModule<'_>,
        interface_key: Option<String>,
        name: &String,
        key: &WorldKey,
        abi: AbiVariant,
    ) -> Result<(ExportKind, u32)> {
        let resolve = &self.info.encoder.metadata.resolve;
        let import = &self.info.import_map[&interface_key];
        let (index, _, lowering) = import.lowerings.get_full(&(name.clone(), abi)).unwrap();
        let metadata = self.info.module_metadata_for(for_module);

        let index = match lowering {
            // All direct lowerings can be `canon lower`'d here immediately
            // and passed as arguments.
            Lowering::Direct => {
                let func_index = match &import.interface {
                    Some(interface) => {
                        let instance_index = self.imported_instances[interface];
                        self.component
                            .alias_export(instance_index, name, ComponentExportKind::Func)
                    }
                    None => self.imported_funcs[name],
                };
                self.component.lower_func(
                    Some(name),
                    func_index,
                    if let AbiVariant::GuestImportAsync = abi {
                        vec![CanonicalOption::Async]
                    } else {
                        Vec::new()
                    },
                )
            }

            // Indirect lowerings come from the shim that was previously
            // created, so the specific export is loaded here and used as an
            // import.
            Lowering::Indirect { .. } => {
                let encoding = metadata.import_encodings.get(resolve, key, name).unwrap();
                return Ok(self.materialize_shim_import(
                    shims,
                    &ShimKind::IndirectLowering {
                        interface: interface_key,
                        index,
                        realloc: for_module,
                        encoding,
                    },
                ));
            }

            // A "resource drop" intrinsic only needs to find the index of the
            // resource type itself and then the intrinsic is declared.
            Lowering::ResourceDrop(id) => {
                let resource_idx = self.lookup_resource_index(*id);
                self.component.resource_drop(resource_idx)
            }
        };
        Ok((ExportKind::Func, index))
    }

    /// Generates component bits that are responsible for executing
    /// `_initialize`, if found, in the original component.
    ///
    /// The `_initialize` function was a part of WASIp1 where it generally is
    /// intended to run after imports and memory and such are all "hooked up"
    /// and performs other various initialization tasks. This is additionally
    /// specified in https://github.com/WebAssembly/component-model/pull/378
    /// to be part of the component model lowerings as well.
    ///
    /// This implements this functionality by encoding a core module that
    /// imports a function and then registers a `start` section with that
    /// imported function. This is all encoded after the
    /// imports/lowerings/tables/etc are all filled in above meaning that this
    /// is the last piece to run. That means that when this is running
    /// everything should be hooked up for all imported functions to work.
    ///
    /// Note that at this time `_initialize` is only detected in the "main
    /// module", not adapters/libraries.
    fn encode_initialize_with_start(&mut self) -> Result<()> {
        let initialize = match self.info.info.exports.initialize() {
            Some(name) => name,
            // If this core module didn't have `_initialize` or similar, then
            // there's nothing to do here.
            None => return Ok(()),
        };
        let initialize_index = self.core_alias_export(
            Some("start"),
            self.instance_index.unwrap(),
            initialize,
            ExportKind::Func,
        );
        let mut shim = Module::default();
        let mut section = TypeSection::new();
        section.ty().function([], []);
        shim.section(&section);
        let mut section = ImportSection::new();
        section.import("", "", EntityType::Function(0));
        shim.section(&section);
        shim.section(&StartSection { function_index: 0 });

        // Declare the core module within the component, create a dummy core
        // instance with one export of our `_initialize` function, and then use
        // that to instantiate the module we emit to run the `start` function in
        // core wasm to run `_initialize`.
        let shim_module_index = self.component.core_module(Some("start-shim-module"), &shim);
        let shim_args_instance_index = self.component.core_instantiate_exports(
            Some("start-shim-args"),
            [("", ExportKind::Func, initialize_index)],
        );
        self.component.core_instantiate(
            Some("start-shim-instance"),
            shim_module_index,
            [("", ModuleArg::Instance(shim_args_instance_index))],
        );
        Ok(())
    }

    /// Convenience function to go from `CustomModule` to the instance index
    /// corresponding to what that points to.
    fn instance_for(&self, module: CustomModule) -> u32 {
        match module {
            CustomModule::Main => self.instance_index.expect("instantiated by now"),
            CustomModule::Adapter(name) => self.adapter_instances[name],
        }
    }

    /// Convenience function to go from `CustomModule` to the module index
    /// corresponding to what that points to.
    fn module_for(&self, module: CustomModule) -> u32 {
        match module {
            CustomModule::Main => self.module_index.unwrap(),
            CustomModule::Adapter(name) => self.adapter_modules[name],
        }
    }

    /// Convenience function which caches aliases created so repeated calls to
    /// this function will all return the same index.
    fn core_alias_export(
        &mut self,
        debug_name: Option<&str>,
        instance: u32,
        name: &str,
        kind: ExportKind,
    ) -> u32 {
        *self
            .aliased_core_items
            .entry((instance, name.to_string()))
            .or_insert_with(|| {
                self.component
                    .core_alias_export(debug_name, instance, name, kind)
            })
    }
}

/// A list of "shims" which start out during the component instantiation process
/// as functions which immediately trap due to a `call_indirect`-to-`null` but
/// will get filled in by the time the component instantiation process
/// completes.
///
/// Shims currently include:
///
/// * "Indirect functions" lowered from imported instances where the lowering
///   requires an item exported from the main module. These are indirect due to
///   the circular dependency between the module needing an import and the
///   import needing the module.
///
/// * Adapter modules which convert from a historical ABI to the component
///   model's ABI (e.g. wasi preview1 to preview2) get a shim since the adapters
///   are currently indicated as always requiring the memory of the main module.
///
/// This structure is created by `encode_shim_instantiation`.
#[derive(Default)]
struct Shims<'a> {
    /// The list of all shims that a module will require.
    shims: IndexMap<ShimKind<'a>, Shim<'a>>,
}

struct Shim<'a> {
    /// Canonical ABI options required by this shim, used during `canon lower`
    /// operations.
    options: RequiredOptions,

    /// The name, in the shim instance, of this shim.
    ///
    /// Currently this is `"0"`, `"1"`, ...
    name: String,

    /// A human-readable debugging name for this shim, used in a core wasm
    /// `name` section.
    debug_name: String,

    /// Precise information about what this shim is a lowering of.
    kind: ShimKind<'a>,

    /// Wasm type of this shim.
    sig: WasmSignature,
}

/// Which variation of `{stream|future}.{read|write}` we're emitting for a
/// `ShimKind::PayloadFunc`.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum PayloadFuncKind {
    FutureWrite,
    FutureRead,
    StreamWrite,
    StreamRead,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum ShimKind<'a> {
    /// This shim is a late indirect lowering of an imported function in a
    /// component which is only possible after prior core wasm modules are
    /// instantiated so their memories and functions are available.
    IndirectLowering {
        /// The name of the interface that's being lowered.
        interface: Option<String>,
        /// The index within the `lowerings` array of the function being lowered.
        index: usize,
        /// Which instance to pull the `realloc` function from, if necessary.
        realloc: CustomModule<'a>,
        /// The string encoding that this lowering is going to use.
        encoding: StringEncoding,
    },
    /// This shim is a core wasm function defined in an adapter module but isn't
    /// available until the adapter module is itself instantiated.
    Adapter {
        /// The name of the adapter module this shim comes from.
        adapter: &'a str,
        /// The name of the export in the adapter module this shim points to.
        func: &'a str,
    },
    /// A shim used as the destructor for a resource which allows defining the
    /// resource before the core module being instantiated.
    ResourceDtor {
        /// Which instance to pull the destructor function from.
        module: CustomModule<'a>,
        /// The exported function name of this destructor in the core module.
        export: &'a str,
    },
    /// A shim used for a `{stream|future}.{read|write}` built-in function,
    /// which must refer to the core module instance's memory from/to which
    /// payload values must be lifted/lowered.
    PayloadFunc {
        /// Which instance to pull the `realloc` function and string encoding
        /// from, if necessary.
        for_module: CustomModule<'a>,
        /// Additional information regarding the function where this `stream` or
        /// `future` type appeared, which we use in combination with
        /// `for_module` to determine which `realloc` and string encoding to
        /// use, as well as which type to specify when emitting the built-in.
        info: &'a PayloadInfo,
        /// Which variation of `{stream|future}.{read|write}` we're emitting.
        kind: PayloadFuncKind,
    },
    /// A shim used for the `waitable-set.wait` built-in function, which must
    /// refer to the core module instance's memory to which results will be
    /// written.
    WaitableSetWait { cancellable: bool },
    /// A shim used for the `waitable-set.poll` built-in function, which must
    /// refer to the core module instance's memory to which results will be
    /// written.
    WaitableSetPoll { cancellable: bool },
    /// Shim for `task.return` to handle a reference to a `memory` which may
    TaskReturn {
        /// The interface (optional) that owns `func` below. If `None` then it's
        /// a world export.
        interface: Option<InterfaceId>,
        /// The function that this `task.return` is returning for, owned
        /// within `interface` above.
        func: &'a str,
        /// The WIT type that `func` returns.
        result: Option<Type>,
        /// Which instance to pull the `realloc` function from, if necessary.
        for_module: CustomModule<'a>,
        /// String encoding to use in the ABI options.
        encoding: StringEncoding,
    },
    /// A shim used for the `error-context.new` built-in function, which must
    /// refer to the core module instance's memory from which the debug message
    /// will be read.
    ErrorContextNew {
        /// String encoding to use when lifting the debug message.
        encoding: StringEncoding,
    },
    /// A shim used for the `error-context.debug-message` built-in function,
    /// which must refer to the core module instance's memory to which results
    /// will be written.
    ErrorContextDebugMessage {
        /// Which instance to pull the `realloc` function from, if necessary.
        for_module: CustomModule<'a>,
        /// The string encoding to use when lowering the debug message.
        encoding: StringEncoding,
    },
    /// A shim used for the `thread.new-indirect` built-in function, which
    /// must refer to the core module instance's indirect function table.
    ThreadNewIndirect {
        /// Which instance to pull the function table from.
        for_module: CustomModule<'a>,
        /// The function type to use when creating the thread.
        func_ty: FuncType,
    },
}

/// Indicator for which module is being used for a lowering or where options
/// like `realloc` are drawn from.
///
/// This is necessary for situations such as an imported function being lowered
/// into the main module and additionally into an adapter module. For example an
/// adapter might adapt from preview1 to preview2 for the standard library of a
/// programming language but the main module's custom application code may also
/// explicitly import from preview2. These two different lowerings of a preview2
/// function are parameterized by this enumeration.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum CustomModule<'a> {
    /// This points to the "main module" which is generally the "output of LLVM"
    /// or what a user wrote.
    Main,
    /// This is selecting an adapter module, identified by name here, where
    /// something is being lowered into.
    Adapter(&'a str),
}

impl<'a> CustomModule<'a> {
    fn debug_name(&self) -> &'a str {
        match self {
            CustomModule::Main => "main",
            CustomModule::Adapter(s) => s,
        }
    }
}

impl<'a> Shims<'a> {
    /// Adds all shims necessary for the instantiation of `for_module`.
    ///
    /// This function will iterate over all the imports required by this module
    /// and for those that require a shim they're registered here.
    fn append_indirect(
        &mut self,
        world: &'a ComponentWorld<'a>,
        for_module: CustomModule<'a>,
    ) -> Result<()> {
        let module_imports = world.imports_for(for_module);
        let module_exports = world.exports_for(for_module);
        let resolve = &world.encoder.metadata.resolve;

        for (module, field, import) in module_imports.imports() {
            match import {
                // These imports don't require shims, they can be satisfied
                // as-needed when required.
                Import::ImportedResourceDrop(..)
                | Import::MainModuleMemory
                | Import::MainModuleExport { .. }
                | Import::Item(_)
                | Import::ExportedResourceDrop(..)
                | Import::ExportedResourceRep(..)
                | Import::ExportedResourceNew(..)
                | Import::ExportedTaskCancel
                | Import::ErrorContextDrop
                | Import::BackpressureInc
                | Import::BackpressureDec
                | Import::ThreadYield { .. }
                | Import::SubtaskDrop
                | Import::SubtaskCancel { .. }
                | Import::FutureNew(..)
                | Import::StreamNew(..)
                | Import::FutureCancelRead { .. }
                | Import::FutureCancelWrite { .. }
                | Import::FutureDropWritable { .. }
                | Import::FutureDropReadable { .. }
                | Import::StreamCancelRead { .. }
                | Import::StreamCancelWrite { .. }
                | Import::StreamDropWritable { .. }
                | Import::StreamDropReadable { .. }
                | Import::WaitableSetNew
                | Import::WaitableSetDrop
                | Import::WaitableJoin
                | Import::ContextGet(_)
                | Import::ContextSet(_)
                | Import::ThreadIndex
                | Import::ThreadSwitchTo { .. }
                | Import::ThreadSuspend { .. }
                | Import::ThreadResumeLater
                | Import::ThreadYieldTo { .. } => {}

                // If `task.return` needs to be indirect then generate a shim
                // for it, otherwise skip the shim and let it get materialized
                // naturally later.
                Import::ExportedTaskReturn(key, interface, func, ty) => {
                    let (options, sig) = task_return_options_and_type(resolve, *ty);
                    if options.is_empty() {
                        continue;
                    }
                    let name = self.shims.len().to_string();
                    let encoding = world
                        .module_metadata_for(for_module)
                        .export_encodings
                        .get(resolve, key, func)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "missing component metadata for export of \
                                `{module}::{field}`"
                            )
                        })?;
                    self.push(Shim {
                        name,
                        debug_name: format!("task-return-{func}"),
                        options,
                        kind: ShimKind::TaskReturn {
                            interface: *interface,
                            func,
                            result: *ty,
                            for_module,
                            encoding,
                        },
                        sig,
                    });
                }

                Import::FutureWrite { async_, info } => {
                    self.append_indirect_payload_push(
                        resolve,
                        for_module,
                        module,
                        *async_,
                        info,
                        PayloadFuncKind::FutureWrite,
                        vec![WasmType::I32; 2],
                        vec![WasmType::I32],
                    );
                }
                Import::FutureRead { async_, info } => {
                    self.append_indirect_payload_push(
                        resolve,
                        for_module,
                        module,
                        *async_,
                        info,
                        PayloadFuncKind::FutureRead,
                        vec![WasmType::I32; 2],
                        vec![WasmType::I32],
                    );
                }
                Import::StreamWrite { async_, info } => {
                    self.append_indirect_payload_push(
                        resolve,
                        for_module,
                        module,
                        *async_,
                        info,
                        PayloadFuncKind::StreamWrite,
                        vec![WasmType::I32; 3],
                        vec![WasmType::I32],
                    );
                }
                Import::StreamRead { async_, info } => {
                    self.append_indirect_payload_push(
                        resolve,
                        for_module,
                        module,
                        *async_,
                        info,
                        PayloadFuncKind::StreamRead,
                        vec![WasmType::I32; 3],
                        vec![WasmType::I32],
                    );
                }

                Import::WaitableSetWait { cancellable } => {
                    let name = self.shims.len().to_string();
                    self.push(Shim {
                        name,
                        debug_name: "waitable-set.wait".to_string(),
                        options: RequiredOptions::empty(),
                        kind: ShimKind::WaitableSetWait {
                            cancellable: *cancellable,
                        },
                        sig: WasmSignature {
                            params: vec![WasmType::I32; 2],
                            results: vec![WasmType::I32],
                            indirect_params: false,
                            retptr: false,
                        },
                    });
                }

                Import::WaitableSetPoll { cancellable } => {
                    let name = self.shims.len().to_string();
                    self.push(Shim {
                        name,
                        debug_name: "waitable-set.poll".to_string(),
                        options: RequiredOptions::empty(),
                        kind: ShimKind::WaitableSetPoll {
                            cancellable: *cancellable,
                        },
                        sig: WasmSignature {
                            params: vec![WasmType::I32; 2],
                            results: vec![WasmType::I32],
                            indirect_params: false,
                            retptr: false,
                        },
                    });
                }

                Import::ErrorContextNew { encoding } => {
                    let name = self.shims.len().to_string();
                    self.push(Shim {
                        name,
                        debug_name: "error-new".to_string(),
                        options: RequiredOptions::MEMORY | RequiredOptions::STRING_ENCODING,
                        kind: ShimKind::ErrorContextNew {
                            encoding: *encoding,
                        },
                        sig: WasmSignature {
                            params: vec![WasmType::I32; 2],
                            results: vec![WasmType::I32],
                            indirect_params: false,
                            retptr: false,
                        },
                    });
                }

                Import::ErrorContextDebugMessage { encoding } => {
                    let name = self.shims.len().to_string();
                    self.push(Shim {
                        name,
                        debug_name: "error-debug-message".to_string(),
                        options: RequiredOptions::MEMORY
                            | RequiredOptions::STRING_ENCODING
                            | RequiredOptions::REALLOC,
                        kind: ShimKind::ErrorContextDebugMessage {
                            for_module,
                            encoding: *encoding,
                        },
                        sig: WasmSignature {
                            params: vec![WasmType::I32; 2],
                            results: vec![],
                            indirect_params: false,
                            retptr: false,
                        },
                    });
                }

                Import::ThreadNewIndirect => {
                    let name = self.shims.len().to_string();
                    self.push(Shim {
                        name,
                        debug_name: "thread.new-indirect".to_string(),
                        options: RequiredOptions::empty(),
                        kind: ShimKind::ThreadNewIndirect {
                            for_module,
                            // This is fixed for now
                            func_ty: FuncType::new([ValType::I32], vec![]),
                        },
                        sig: WasmSignature {
                            params: vec![WasmType::I32; 2],
                            results: vec![WasmType::I32],
                            indirect_params: false,
                            retptr: false,
                        },
                    });
                }

                // Adapter imports into the main module must got through an
                // indirection, so that's registered here.
                Import::AdapterExport { adapter, func, ty } => {
                    let name = self.shims.len().to_string();
                    log::debug!("shim {name} is adapter `{module}::{field}`");
                    self.push(Shim {
                        name,
                        debug_name: format!("adapt-{module}-{field}"),
                        // Pessimistically assume that all adapters require
                        // memory in one form or another. While this isn't
                        // technically true it's true enough for WASI.
                        options: RequiredOptions::MEMORY,
                        kind: ShimKind::Adapter { adapter, func },
                        sig: WasmSignature {
                            params: ty.params().iter().map(to_wasm_type).collect(),
                            results: ty.results().iter().map(to_wasm_type).collect(),
                            indirect_params: false,
                            retptr: false,
                        },
                    });

                    fn to_wasm_type(ty: &wasmparser::ValType) -> WasmType {
                        match ty {
                            wasmparser::ValType::I32 => WasmType::I32,
                            wasmparser::ValType::I64 => WasmType::I64,
                            wasmparser::ValType::F32 => WasmType::F32,
                            wasmparser::ValType::F64 => WasmType::F64,
                            _ => unreachable!(),
                        }
                    }
                }

                // WIT-level functions may require an indirection, so yield some
                // metadata out of this `match` to the loop below to figure that
                // out.
                Import::InterfaceFunc(key, _, name, abi) => {
                    self.append_indirect_wit_func(
                        world,
                        for_module,
                        module,
                        field,
                        key,
                        name,
                        Some(resolve.name_world_key(key)),
                        *abi,
                    )?;
                }
                Import::WorldFunc(key, name, abi) => {
                    self.append_indirect_wit_func(
                        world, for_module, module, field, key, name, None, *abi,
                    )?;
                }
            }
        }

        // In addition to all the shims added for imports above this module also
        // requires shims for resource destructors that it exports. Resource
        // types are declared before the module is instantiated so the actual
        // destructor is registered as a shim (defined here) and it's then
        // filled in with the module's exports later.
        for (export_name, export) in module_exports.iter() {
            let id = match export {
                Export::ResourceDtor(id) => id,
                _ => continue,
            };
            let resource = resolve.types[*id].name.as_ref().unwrap();
            let name = self.shims.len().to_string();
            self.push(Shim {
                name,
                debug_name: format!("dtor-{resource}"),
                options: RequiredOptions::empty(),
                kind: ShimKind::ResourceDtor {
                    module: for_module,
                    export: export_name,
                },
                sig: WasmSignature {
                    params: vec![WasmType::I32],
                    results: Vec::new(),
                    indirect_params: false,
                    retptr: false,
                },
            });
        }

        Ok(())
    }

    /// Helper of `append_indirect` above which pushes information for
    /// futures/streams read/write intrinsics.
    fn append_indirect_payload_push(
        &mut self,
        resolve: &Resolve,
        for_module: CustomModule<'a>,
        module: &str,
        async_: bool,
        info: &'a PayloadInfo,
        kind: PayloadFuncKind,
        params: Vec<WasmType>,
        results: Vec<WasmType>,
    ) {
        let debug_name = format!("{module}-{}", info.name);
        let name = self.shims.len().to_string();

        let payload = info.payload(resolve);
        let (wit_param, wit_result) = match kind {
            PayloadFuncKind::StreamRead | PayloadFuncKind::FutureRead => (None, payload),
            PayloadFuncKind::StreamWrite | PayloadFuncKind::FutureWrite => (payload, None),
        };
        self.push(Shim {
            name,
            debug_name,
            options: RequiredOptions::MEMORY
                | RequiredOptions::for_import(
                    resolve,
                    &Function {
                        name: String::new(),
                        kind: FunctionKind::Freestanding,
                        params: match wit_param {
                            Some(ty) => vec![("a".to_string(), ty)],
                            None => Vec::new(),
                        },
                        result: wit_result,
                        docs: Default::default(),
                        stability: Stability::Unknown,
                    },
                    if async_ {
                        AbiVariant::GuestImportAsync
                    } else {
                        AbiVariant::GuestImport
                    },
                ),
            kind: ShimKind::PayloadFunc {
                for_module,
                info,
                kind,
            },
            sig: WasmSignature {
                params,
                results,
                indirect_params: false,
                retptr: false,
            },
        });
    }

    /// Helper for `append_indirect` above which will conditionally push a shim
    /// for the WIT function specified by `interface_key`, `name`, and `abi`.
    fn append_indirect_wit_func(
        &mut self,
        world: &'a ComponentWorld<'a>,
        for_module: CustomModule<'a>,
        module: &str,
        field: &str,
        key: &WorldKey,
        name: &String,
        interface_key: Option<String>,
        abi: AbiVariant,
    ) -> Result<()> {
        let resolve = &world.encoder.metadata.resolve;
        let metadata = world.module_metadata_for(for_module);
        let interface = &world.import_map[&interface_key];
        let (index, _, lowering) = interface.lowerings.get_full(&(name.clone(), abi)).unwrap();
        let shim_name = self.shims.len().to_string();
        match lowering {
            Lowering::Direct | Lowering::ResourceDrop(_) => {}

            Lowering::Indirect { sig, options } => {
                log::debug!(
                    "shim {shim_name} is import `{module}::{field}` lowering {index} `{name}`",
                );
                let encoding = metadata
                    .import_encodings
                    .get(resolve, key, name)
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "missing component metadata for import of \
                                `{module}::{field}`"
                        )
                    })?;
                self.push(Shim {
                    name: shim_name,
                    debug_name: format!("indirect-{module}-{field}"),
                    options: *options,
                    kind: ShimKind::IndirectLowering {
                        interface: interface_key,
                        index,
                        realloc: for_module,
                        encoding,
                    },
                    sig: sig.clone(),
                });
            }
        }

        Ok(())
    }

    fn push(&mut self, shim: Shim<'a>) {
        // Only one shim per `ShimKind` is retained, so if it's already present
        // don't overwrite it. If it's not present though go ahead and insert
        // it.
        if !self.shims.contains_key(&shim.kind) {
            self.shims.insert(shim.kind.clone(), shim);
        }
    }
}

fn task_return_options_and_type(
    resolve: &Resolve,
    ty: Option<Type>,
) -> (RequiredOptions, WasmSignature) {
    let func_tmp = Function {
        name: String::new(),
        kind: FunctionKind::Freestanding,
        params: match ty {
            Some(ty) => vec![("a".to_string(), ty)],
            None => Vec::new(),
        },
        result: None,
        docs: Default::default(),
        stability: Stability::Unknown,
    };
    let abi = AbiVariant::GuestImport;
    let options = RequiredOptions::for_import(resolve, &func_tmp, abi);
    let sig = resolve.wasm_signature(abi, &func_tmp);
    (options, sig)
}

/// Alias argument to an instantiation
#[derive(Clone, Debug)]
pub struct Item {
    pub alias: String,
    pub kind: ExportKind,
    pub which: MainOrAdapter,
    pub name: String,
}

/// Module argument to an instantiation
#[derive(Debug, PartialEq, Clone)]
pub enum MainOrAdapter {
    Main,
    Adapter(String),
}

impl MainOrAdapter {
    fn to_custom_module(&self) -> CustomModule<'_> {
        match self {
            MainOrAdapter::Main => CustomModule::Main,
            MainOrAdapter::Adapter(s) => CustomModule::Adapter(s),
        }
    }
}

/// Module instantiation argument
#[derive(Clone)]
pub enum Instance {
    /// Module argument
    MainOrAdapter(MainOrAdapter),

    /// Alias argument
    Items(Vec<Item>),
}

/// Provides fine-grained control of how a library module is instantiated
/// relative to other module instances
#[derive(Clone)]
pub struct LibraryInfo {
    /// If true, instantiate any shims prior to this module
    pub instantiate_after_shims: bool,

    /// Instantiation arguments
    pub arguments: Vec<(String, Instance)>,
}

/// Represents an adapter or library to be instantiated as part of the component
pub(super) struct Adapter {
    /// The wasm of the module itself, with `component-type` sections stripped
    wasm: Vec<u8>,

    /// The metadata for the adapter
    metadata: ModuleMetadata,

    /// The set of exports from the final world which are defined by this
    /// adapter or library
    required_exports: IndexSet<WorldKey>,

    /// If present, treat this module as a library rather than a "minimal" adapter
    ///
    /// TODO: We should refactor how various flavors of module are represented
    /// and differentiated to avoid mistaking one for another.
    library_info: Option<LibraryInfo>,
}

/// An encoder of components based on `wit` interface definitions.
#[derive(Default)]
pub struct ComponentEncoder {
    module: Vec<u8>,
    module_import_map: Option<ModuleImportMap>,
    pub(super) metadata: Bindgen,
    validate: bool,
    pub(super) main_module_exports: IndexSet<WorldKey>,
    pub(super) adapters: IndexMap<String, Adapter>,
    import_name_map: HashMap<String, String>,
    realloc_via_memory_grow: bool,
    merge_imports_based_on_semver: Option<bool>,
    pub(super) reject_legacy_names: bool,
    debug_names: bool,
}

impl ComponentEncoder {
    /// Set the core module to encode as a component.
    /// This method will also parse any component type information stored in custom sections
    /// inside the module and add them as the interface, imports, and exports.
    /// It will also add any producers information inside the component type information to the
    /// core module.
    pub fn module(mut self, module: &[u8]) -> Result<Self> {
        let (wasm, metadata) = self.decode(module.as_ref())?;
        let (wasm, module_import_map) = ModuleImportMap::new(wasm)?;
        let exports = self
            .merge_metadata(metadata)
            .context("failed merge WIT metadata for module with previous metadata")?;
        self.main_module_exports.extend(exports);
        self.module = if let Some(producers) = &self.metadata.producers {
            producers.add_to_wasm(&wasm)?
        } else {
            wasm.to_vec()
        };
        self.module_import_map = module_import_map;
        Ok(self)
    }

    fn decode<'a>(&self, wasm: &'a [u8]) -> Result<(Cow<'a, [u8]>, Bindgen)> {
        let (bytes, metadata) = metadata::decode(wasm)?;
        match bytes {
            Some(wasm) => Ok((Cow::Owned(wasm), metadata)),
            None => Ok((Cow::Borrowed(wasm), metadata)),
        }
    }

    fn merge_metadata(&mut self, metadata: Bindgen) -> Result<IndexSet<WorldKey>> {
        self.metadata.merge(metadata)
    }

    /// Sets whether or not the encoder will validate its output.
    pub fn validate(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    /// Sets whether or not to generate debug names in the output component.
    pub fn debug_names(mut self, debug_names: bool) -> Self {
        self.debug_names = debug_names;
        self
    }

    /// Sets whether to merge imports based on semver to the specified value.
    ///
    /// This affects how when to WIT worlds are merged together, for example
    /// from two different libraries, whether their imports are unified when the
    /// semver version ranges for interface allow it.
    ///
    /// This is enabled by default.
    pub fn merge_imports_based_on_semver(mut self, merge: bool) -> Self {
        self.merge_imports_based_on_semver = Some(merge);
        self
    }

    /// Sets whether to reject the historical mangling/name scheme for core wasm
    /// imports/exports as they map to the component model.
    ///
    /// The `wit-component` crate supported a different set of names prior to
    /// WebAssembly/component-model#378 and this can be used to disable this
    /// support.
    ///
    /// This is disabled by default.
    pub fn reject_legacy_names(mut self, reject: bool) -> Self {
        self.reject_legacy_names = reject;
        self
    }

    /// Specifies a new adapter which is used to translate from a historical
    /// wasm ABI to the canonical ABI and the `interface` provided.
    ///
    /// This is primarily used to polyfill, for example,
    /// `wasi_snapshot_preview1` with a component-model using interface. The
    /// `name` provided is the module name of the adapter that is being
    /// polyfilled, for example `"wasi_snapshot_preview1"`.
    ///
    /// The `bytes` provided is a core wasm module which implements the `name`
    /// interface in terms of the `interface` interface. This core wasm module
    /// is severely restricted in its shape, for example it cannot have any data
    /// segments or element segments.
    ///
    /// The `interface` provided is the component-model-using-interface that the
    /// wasm module specified by `bytes` imports. The `bytes` will then import
    /// `interface` and export functions to get imported from the module `name`
    /// in the core wasm that's being wrapped.
    pub fn adapter(self, name: &str, bytes: &[u8]) -> Result<Self> {
        self.library_or_adapter(name, bytes, None)
    }

    /// Specifies a shared-everything library to link into the component.
    ///
    /// Unlike adapters, libraries _may_ have data and/or element segments, but
    /// they must operate on an imported memory and table, respectively.  In
    /// this case, the correct amount of space is presumed to have been
    /// statically allocated in the main module's memory and table at the
    /// offsets which the segments target, e.g. as arranged by
    /// [super::linking::Linker].
    ///
    /// Libraries are treated similarly to adapters, except that they are not
    /// "minified" the way adapters are, and instantiation is controlled
    /// declaratively via the `library_info` parameter.
    pub fn library(self, name: &str, bytes: &[u8], library_info: LibraryInfo) -> Result<Self> {
        self.library_or_adapter(name, bytes, Some(library_info))
    }

    fn library_or_adapter(
        mut self,
        name: &str,
        bytes: &[u8],
        library_info: Option<LibraryInfo>,
    ) -> Result<Self> {
        let (wasm, mut metadata) = self.decode(bytes)?;
        // Merge the adapter's document into our own document to have one large
        // document, and then afterwards merge worlds as well.
        //
        // Note that the `metadata` tracking import/export encodings is removed
        // since this adapter can get different lowerings and is allowed to
        // differ from the main module. This is then tracked within the
        // `Adapter` structure produced below.
        let adapter_metadata = mem::take(&mut metadata.metadata);
        let exports = self.merge_metadata(metadata).with_context(|| {
            format!("failed to merge WIT packages of adapter `{name}` into main packages")
        })?;
        if let Some(library_info) = &library_info {
            // Validate that all referenced modules can be resolved.
            for (_, instance) in &library_info.arguments {
                let resolve = |which: &_| match which {
                    MainOrAdapter::Main => Ok(()),
                    MainOrAdapter::Adapter(name) => {
                        if self.adapters.contains_key(name.as_str()) {
                            Ok(())
                        } else {
                            Err(anyhow!("instance refers to unknown adapter `{name}`"))
                        }
                    }
                };

                match instance {
                    Instance::MainOrAdapter(which) => resolve(which)?,
                    Instance::Items(items) => {
                        for item in items {
                            resolve(&item.which)?;
                        }
                    }
                }
            }
        }
        self.adapters.insert(
            name.to_string(),
            Adapter {
                wasm: wasm.to_vec(),
                metadata: adapter_metadata,
                required_exports: exports,
                library_info,
            },
        );
        Ok(self)
    }

    /// True if the realloc and stack allocation should use memory.grow
    /// The default is to use the main module realloc
    /// Can be useful if cabi_realloc cannot be called before the host
    /// runtime is initialized.
    pub fn realloc_via_memory_grow(mut self, value: bool) -> Self {
        self.realloc_via_memory_grow = value;
        self
    }

    /// The instance import name map to use.
    ///
    /// This is used to rename instance imports in the final component.
    ///
    /// For example, if there is an instance import `foo:bar/baz` and it is
    /// desired that the import actually be an `unlocked-dep` name, then
    /// `foo:bar/baz` can be mapped to `unlocked-dep=<a:b/c@{>=x.y.z}>`.
    ///
    /// Note: the replacement names are not validated during encoding unless
    /// the `validate` option is set to true.
    pub fn import_name_map(mut self, map: HashMap<String, String>) -> Self {
        self.import_name_map = map;
        self
    }

    /// Encode the component and return the bytes.
    pub fn encode(&mut self) -> Result<Vec<u8>> {
        if self.module.is_empty() {
            bail!("a module is required when encoding a component");
        }

        if self.merge_imports_based_on_semver.unwrap_or(true) {
            self.metadata
                .resolve
                .merge_world_imports_based_on_semver(self.metadata.world)?;
        }

        let world = ComponentWorld::new(self).context("failed to decode world from module")?;
        let mut state = EncodingState {
            component: ComponentBuilder::default(),
            module_index: None,
            instance_index: None,
            memory_index: None,
            shim_instance_index: None,
            fixups_module_index: None,
            adapter_modules: IndexMap::new(),
            adapter_instances: IndexMap::new(),
            import_type_encoding_maps: Default::default(),
            export_type_encoding_maps: Default::default(),
            imported_instances: Default::default(),
            imported_funcs: Default::default(),
            exported_instances: Default::default(),
            aliased_core_items: Default::default(),
            info: &world,
        };
        state.encode_imports(&self.import_name_map)?;
        state.encode_core_modules();
        state.encode_core_instantiation()?;
        state.encode_exports(CustomModule::Main)?;
        for name in self.adapters.keys() {
            state.encode_exports(CustomModule::Adapter(name))?;
        }
        state.component.append_names();
        state
            .component
            .raw_custom_section(&crate::base_producers().raw_custom_section());
        let bytes = state.component.finish();

        if self.validate {
            Validator::new_with_features(WasmFeatures::all())
                .validate_all(&bytes)
                .context("failed to validate component output")?;
        }

        Ok(bytes)
    }
}

impl ComponentWorld<'_> {
    /// Convenience function to lookup a module's import map.
    fn imports_for(&self, module: CustomModule) -> &ImportMap {
        match module {
            CustomModule::Main => &self.info.imports,
            CustomModule::Adapter(name) => &self.adapters[name].info.imports,
        }
    }

    /// Convenience function to lookup a module's export map.
    fn exports_for(&self, module: CustomModule) -> &ExportMap {
        match module {
            CustomModule::Main => &self.info.exports,
            CustomModule::Adapter(name) => &self.adapters[name].info.exports,
        }
    }

    /// Convenience function to lookup a module's metadata.
    fn module_metadata_for(&self, module: CustomModule) -> &ModuleMetadata {
        match module {
            CustomModule::Main => &self.encoder.metadata.metadata,
            CustomModule::Adapter(name) => &self.encoder.adapters[name].metadata,
        }
    }
}

#[cfg(all(test, feature = "dummy-module"))]
mod test {
    use super::*;
    use crate::{dummy_module, embed_component_metadata};
    use wit_parser::ManglingAndAbi;

    #[test]
    fn it_renames_imports() {
        let mut resolve = Resolve::new();
        let pkg = resolve
            .push_str(
                "test.wit",
                r#"
package test:wit;

interface i {
    f: func();
}

world test {
    import i;
    import foo: interface {
        f: func();
    }
}
"#,
            )
            .unwrap();
        let world = resolve.select_world(&[pkg], None).unwrap();

        let mut module = dummy_module(&resolve, world, ManglingAndAbi::Standard32);

        embed_component_metadata(&mut module, &resolve, world, StringEncoding::UTF8).unwrap();

        let encoded = ComponentEncoder::default()
            .import_name_map(HashMap::from([
                (
                    "foo".to_string(),
                    "unlocked-dep=<foo:bar/foo@{>=1.0.0 <1.1.0}>".to_string(),
                ),
                (
                    "test:wit/i".to_string(),
                    "locked-dep=<foo:bar/i@1.2.3>".to_string(),
                ),
            ]))
            .module(&module)
            .unwrap()
            .validate(true)
            .encode()
            .unwrap();

        let wat = wasmprinter::print_bytes(encoded).unwrap();
        assert!(wat.contains("unlocked-dep=<foo:bar/foo@{>=1.0.0 <1.1.0}>"));
        assert!(wat.contains("locked-dep=<foo:bar/i@1.2.3>"));
    }
}

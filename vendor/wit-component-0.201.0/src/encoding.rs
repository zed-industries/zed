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

use crate::encoding::world::WorldAdapter;
use crate::metadata::{self, Bindgen, ModuleMetadata};
use crate::validation::{
    ResourceInfo, ValidatedModule, BARE_FUNC_MODULE_NAME, MAIN_MODULE_IMPORT_NAME,
    POST_RETURN_PREFIX,
};
use crate::StringEncoding;
use anyhow::{anyhow, bail, Context, Result};
use indexmap::{IndexMap, IndexSet};
use std::collections::HashMap;
use std::hash::Hash;
use wasm_encoder::*;
use wasmparser::{Validator, WasmFeatures};
use wit_parser::{
    abi::{AbiVariant, WasmSignature, WasmType},
    Function, FunctionKind, InterfaceId, LiveTypes, Resolve, Type, TypeDefKind, TypeId, TypeOwner,
    WorldItem, WorldKey,
};

const INDIRECT_TABLE_NAME: &str = "$imports";

mod wit;
pub use wit::{encode, encode_world};

mod types;
use types::{InstanceTypeEncoder, RootTypeEncoder, ValtypeEncoder};
mod world;
use world::{ComponentWorld, ImportedInterface, Lowering};

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
    }
}

impl RequiredOptions {
    fn for_import(resolve: &Resolve, func: &Function) -> RequiredOptions {
        let sig = resolve.wasm_signature(AbiVariant::GuestImport, func);
        let mut ret = RequiredOptions::empty();
        // Lift the params and lower the results for imports
        ret.add_lift(TypeContents::for_types(
            resolve,
            func.params.iter().map(|(_, t)| t),
        ));
        ret.add_lower(TypeContents::for_types(resolve, func.results.iter_types()));

        // If anything is indirect then `memory` will be required to read the
        // indirect values.
        if sig.retptr || sig.indirect_params {
            ret |= RequiredOptions::MEMORY;
        }
        ret
    }

    fn for_export(resolve: &Resolve, func: &Function) -> RequiredOptions {
        let sig = resolve.wasm_signature(AbiVariant::GuestExport, func);
        let mut ret = RequiredOptions::empty();
        // Lower the params and lift the results for exports
        ret.add_lower(TypeContents::for_types(
            resolve,
            func.params.iter().map(|(_, t)| t),
        ));
        ret.add_lift(TypeContents::for_types(resolve, func.results.iter_types()));

        // If anything is indirect then `memory` will be required to read the
        // indirect values, but if the arguments are indirect then `realloc` is
        // additionally required to allocate space for the parameters.
        if sig.retptr || sig.indirect_params {
            ret |= RequiredOptions::MEMORY;
            if sig.indirect_params {
                ret |= RequiredOptions::REALLOC;
            }
        }
        ret
    }

    fn add_lower(&mut self, types: TypeContents) {
        // If lists/strings are lowered into wasm then memory is required as
        // usual but `realloc` is also required to allow the external caller to
        // allocate space in the destination for the list/string.
        if types.contains(TypeContents::LIST) {
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
        if types.contains(TypeContents::LIST) {
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
            options: [Option<CanonicalOption>; 3],
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

        Ok(iter)
    }
}

bitflags::bitflags! {
    /// Flags about what kinds of types are present within the recursive
    /// structure of a type.
    struct TypeContents: u8 {
        const STRING = 1 << 0;
        const LIST = 1 << 1;
    }
}

impl TypeContents {
    fn for_types<'a>(resolve: &Resolve, types: impl Iterator<Item = &'a Type>) -> Self {
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
                TypeDefKind::List(t) => Self::for_type(resolve, t) | Self::LIST,
                TypeDefKind::Type(t) => Self::for_type(resolve, t),
                TypeDefKind::Future(_) => todo!("encoding for future"),
                TypeDefKind::Stream(_) => todo!("encoding for stream"),
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
    /// The index in the core function index space for the realloc function.
    ///
    /// If `None`, then the realloc function has not yet been aliased.
    realloc_index: Option<u32>,
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
    /// A map of the index of the aliased realloc function for each adapter
    /// module. Note that adapters have two realloc functions, one for imports
    /// and one for exports.
    adapter_import_reallocs: IndexMap<&'a str, Option<u32>>,
    adapter_export_reallocs: IndexMap<&'a str, Option<u32>>,

    /// Imported instances and what index they were imported as.
    imported_instances: IndexMap<InterfaceId, u32>,
    imported_funcs: IndexMap<String, u32>,
    exported_instances: IndexMap<InterfaceId, u32>,

    /// Maps used when translating types to the component model binary format.
    /// Note that imports and exports are stored in separate maps since they
    /// need fresh hierarchies of types in case the same interface is both
    /// imported and exported.
    import_type_map: HashMap<TypeId, u32>,
    import_func_type_map: HashMap<types::FunctionKey<'a>, u32>,
    export_type_map: HashMap<TypeId, u32>,
    export_func_type_map: HashMap<types::FunctionKey<'a>, u32>,

    /// Metadata about the world inferred from the input to `ComponentEncoder`.
    info: &'a ComponentWorld<'a>,
}

impl<'a> EncodingState<'a> {
    fn encode_core_modules(&mut self) {
        assert!(self.module_index.is_none());
        let idx = self.component.core_module_raw(&self.info.encoder.module);
        self.module_index = Some(idx);

        for (name, adapter) in self.info.adapters.iter() {
            let add_meta = wasm_metadata::AddMetadata {
                name: Some(if adapter.library_info.is_some() {
                    name.to_string()
                } else {
                    format!("wit-component:adapter:{name}")
                }),
                ..Default::default()
            };
            let wasm = add_meta
                .to_wasm(&adapter.wasm)
                .expect("core wasm can get name added");
            let idx = self.component.core_module_raw(&wasm);
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
            type_map: Default::default(),
            func_type_map: Default::default(),
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
        log::trace!("encoding imports for `{name}` as {:?}", interface_id);
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
            if !info.lowerings.contains_key(&func.name) {
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
        let instance_type_idx = self.component.type_instance(&ty);
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
                WorldItem::Interface(_) | WorldItem::Type(_) => continue,
            };
            let name = resolve.name_world_key(name);
            if !info.lowerings.contains_key(&name) {
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
        let info = &self.info.info;

        // Encode a shim instantiation if needed
        let shims = self.encode_shim_instantiation()?;

        // For each instance import into the main module create a
        // pseudo-core-wasm-module via a bag-of-exports.
        let mut args = Vec::new();
        for core_wasm_name in info.required_imports.keys() {
            let index = self.import_instance_to_lowered_core_instance(
                CustomModule::Main,
                core_wasm_name,
                &shims,
                info.metadata,
            );
            args.push((*core_wasm_name, ModuleArg::Instance(index)));
        }

        // For each adapter module instance imported into the core wasm module
        // the appropriate shim is packaged up into a bag-of-exports instance.
        // Note that adapter modules currently don't deal with
        // indirect-vs-direct lowerings, everything is indirect.
        for (adapter, funcs) in info.adapters_required.iter() {
            let shim_instance = self
                .shim_instance_index
                .expect("shim should be instantiated");
            let mut exports = Vec::new();

            for (func, _ty) in funcs {
                let index = self.component.core_alias_export(
                    shim_instance,
                    &shims.shim_names[&ShimKind::Adapter { adapter, func }],
                    ExportKind::Func,
                );
                exports.push((*func, ExportKind::Func, index));
            }

            let index = self.component.core_instantiate_exports(exports);
            args.push((*adapter, ModuleArg::Instance(index)));
        }

        self.add_resource_funcs(
            CustomModule::Main,
            &info.required_resource_funcs,
            &shims,
            &mut args,
        );

        // Instantiate the main module now that all of its arguments have been
        // prepared. With this we now have the main linear memory for
        // liftings/lowerings later on as well as the adapter modules, if any,
        // instantiated after the core wasm module.
        self.instantiate_core_module(args, info);

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

        for (name, adapter) in before {
            self.instantiate_adapter_module(&shims, name, adapter);
        }

        // With all the relevant core wasm instances in play now the original shim
        // module, if present, can be filled in with lowerings/adapters/etc.
        self.encode_indirect_lowerings(&shims)?;

        for (name, adapter) in after {
            self.instantiate_adapter_module(&shims, name, adapter);
        }

        Ok(())
    }

    /// Lowers a named imported interface a core wasm instances suitable to
    /// provide as an instantiation argument to another core wasm module.
    ///
    /// * `for_module` the module that this instance is being created for, or
    ///   otherwise which `realloc` option is used for the lowerings.
    /// * `name` - the name of the imported interface that's being lowered.
    /// * `imports` - the list of all imports known for this encoding.
    /// * `shims` - the indirect/adapter shims created prior, if any.
    fn import_instance_to_lowered_core_instance(
        &mut self,
        for_module: CustomModule<'_>,
        core_wasm_name: &str,
        shims: &Shims<'_>,
        metadata: &ModuleMetadata,
    ) -> u32 {
        let interface = if core_wasm_name == BARE_FUNC_MODULE_NAME {
            None
        } else {
            Some(core_wasm_name.to_string())
        };
        let import = &self.info.import_map[&interface];
        let required_imports = match for_module {
            CustomModule::Main => &self.info.info.required_imports[core_wasm_name],
            CustomModule::Adapter(name) => {
                &self.info.adapters[name].info.required_imports[core_wasm_name]
            }
        };
        let mut exports = Vec::with_capacity(import.lowerings.len());

        for (index, (name, lowering)) in import.lowerings.iter().enumerate() {
            if !required_imports.funcs.contains(name.as_str()) {
                continue;
            }
            let index = match lowering {
                // All direct lowerings can be `canon lower`'d here immediately
                // and passed as arguments.
                Lowering::Direct => {
                    let func_index = match &import.interface {
                        Some(interface) => {
                            let instance_index = self.imported_instances[interface];
                            self.component.alias_export(
                                instance_index,
                                name,
                                ComponentExportKind::Func,
                            )
                        }
                        None => self.imported_funcs[name],
                    };
                    self.component.lower_func(func_index, [])
                }

                // Add an entry for all indirect lowerings which come as an
                // export of the shim module.
                Lowering::Indirect { .. } => {
                    let encoding =
                        metadata.import_encodings[&(core_wasm_name.to_string(), name.clone())];
                    self.component.core_alias_export(
                        self.shim_instance_index
                            .expect("shim should be instantiated"),
                        &shims.shim_names[&ShimKind::IndirectLowering {
                            interface: interface.clone(),
                            index,
                            realloc: for_module,
                            encoding,
                        }],
                        ExportKind::Func,
                    )
                }

                Lowering::ResourceDrop(id) => {
                    let resource_idx = self.lookup_resource_index(*id);
                    self.component.resource_drop(resource_idx)
                }
            };
            exports.push((name.as_str(), ExportKind::Func, index));
        }

        self.component.core_instantiate_exports(exports)
    }

    fn lookup_resource_index(&mut self, id: TypeId) -> u32 {
        let resolve = &self.info.encoder.metadata.resolve;
        let ty = &resolve.types[id];
        match ty.owner {
            // If this resource is owned by a world then it's a top-level
            // resource which means it must have already been translated so
            // it's available for lookup in `import_type_map`.
            TypeOwner::World(_) => self.import_type_map[&id],
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
        let world = &resolve.worlds[self.info.encoder.metadata.world];
        for export_name in exports {
            let export_string = resolve.name_world_key(export_name);
            match &world.exports[export_name] {
                WorldItem::Function(func) => {
                    let ty = self
                        .root_import_type_encoder(None)
                        .encode_func_type(resolve, func)?;
                    let core_name = func.core_export_name(None);
                    let idx = self.encode_lift(module, &core_name, func, ty)?;
                    self.component
                        .export(&export_string, ComponentExportKind::Func, idx, None);
                }
                WorldItem::Interface(export) => {
                    self.encode_interface_export(&export_string, module, *export)?;
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
        export: InterfaceId,
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
            let core_name = func.core_export_name(Some(export_name));
            let ty = root.encode_func_type(resolve, func)?;
            let func_index = root.state.encode_lift(module, &core_name, func, ty)?;
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
            type_map: Default::default(),
            func_type_map: Default::default(),
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
        let imported_types = nested.type_map.clone();

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
            .type_map
            .drain()
            .map(|p| (p.1, p.0))
            .collect::<HashMap<_, _>>();
        for (name, idx) in nested.imports.drain(..) {
            let id = reverse_map[&idx];
            let owner = match resolve.types[id].owner {
                TypeOwner::Interface(id) => id,
                _ => unreachable!(),
            };
            let idx = if owner == export || exports_used.contains(&owner) {
                log::trace!("consulting exports for {id:?}");
                nested.state.export_type_map[&id]
            } else {
                log::trace!("consulting imports for {id:?}");
                nested.state.import_type_map[&id]
            };
            imports.push((name, ComponentExportKind::Type, idx))
        }

        // Before encoding exports reset the type map to what all was imported
        // from foreign interfaces. This will enable any encoded types below to
        // refer to imports which, after type substitution, will point to the
        // correct type in the outer component context.
        nested.type_map = imported_types;

        // Next the component reexports all of its imports, but notably uses the
        // type ascription feature to change the type of the function. Note that
        // no structural change is happening to the types here but instead types
        // are getting proper names and such now that this nested component is a
        // new type index space. Hence the `export_types = true` flag here which
        // flows through the type encoding and when types are emitted.
        nested.export_types = true;
        nested.func_type_map.clear();

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
                    nested.type_map.insert(*id, idx);
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
        let component_index = self.component.component(component);
        let instance_index = self.component.instantiate(component_index, imports);
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
            self.export_type_map.remove(id);
        }

        return Ok(());

        struct NestedComponentTypeEncoder<'state, 'a> {
            component: ComponentBuilder,
            type_map: HashMap<TypeId, u32>,
            func_type_map: HashMap<types::FunctionKey<'a>, u32>,
            export_types: bool,
            interface: InterfaceId,
            state: &'state mut EncodingState<'a>,
            imports: IndexMap<String, u32>,
        }

        impl<'a> ValtypeEncoder<'a> for NestedComponentTypeEncoder<'_, 'a> {
            fn defined_type(&mut self) -> (u32, ComponentDefinedTypeEncoder<'_>) {
                self.component.type_defined()
            }
            fn define_function_type(&mut self) -> (u32, ComponentFuncTypeEncoder<'_>) {
                self.component.type_function()
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
            fn type_map(&mut self) -> &mut HashMap<TypeId, u32> {
                &mut self.type_map
            }
            fn func_type_map(&mut self) -> &mut HashMap<types::FunctionKey<'a>, u32> {
                &mut self.func_type_map
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
                FunctionKind::Freestanding => {
                    format!("import-func-{}", f.name)
                }

                // transform `[method]foo.bar` into `import-method-foo-bar` to
                // have it be a valid kebab-name which can't conflict with
                // anything else.
                //
                // There's probably a better and more "formal" way to do this
                // but quick-and-dirty string manipulation should work well
                // enough for now hopefully.
                FunctionKind::Method(_)
                | FunctionKind::Static(_)
                | FunctionKind::Constructor(_) => {
                    format!(
                        "import-{}",
                        f.name.replace('[', "").replace([']', '.'], "-")
                    )
                }
            }
        }
    }

    fn encode_lift(
        &mut self,
        module: CustomModule<'_>,
        core_name: &str,
        func: &Function,
        ty: u32,
    ) -> Result<u32> {
        let resolve = &self.info.encoder.metadata.resolve;
        let metadata = match module {
            CustomModule::Main => &self.info.encoder.metadata.metadata,
            CustomModule::Adapter(name) => &self.info.encoder.adapters[name].metadata,
        };
        let post_returns = match module {
            CustomModule::Main => &self.info.info.post_returns,
            CustomModule::Adapter(name) => &self.info.adapters[name].info.post_returns,
        };
        let instance_index = match module {
            CustomModule::Main => self.instance_index.expect("instantiated by now"),
            CustomModule::Adapter(name) => self.adapter_instances[name],
        };
        let core_func_index =
            self.component
                .core_alias_export(instance_index, core_name, ExportKind::Func);

        let options = RequiredOptions::for_export(resolve, func);

        let encoding = metadata.export_encodings[core_name];
        // TODO: This realloc detection should probably be improved with
        // some sort of scheme to have per-function reallocs like
        // `cabi_realloc_{name}` or something like that.
        let realloc_index = match module {
            CustomModule::Main => self.realloc_index,
            CustomModule::Adapter(name) => self.adapter_export_reallocs[name],
        };
        let mut options = options
            .into_iter(encoding, self.memory_index, realloc_index)?
            .collect::<Vec<_>>();

        let post_return = format!("{POST_RETURN_PREFIX}{core_name}");
        if post_returns.contains(&post_return[..]) {
            let post_return =
                self.component
                    .core_alias_export(instance_index, &post_return, ExportKind::Func);
            options.push(CanonicalOption::PostReturn(post_return));
        }
        let func_index = self.component.lift_func(core_func_index, ty, options);
        Ok(func_index)
    }

    fn encode_shim_instantiation(&mut self) -> Result<Shims<'a>> {
        let mut signatures = Vec::new();
        let mut ret = Shims::default();
        let info = &self.info.info;

        // For all interfaces imported into the main module record all of their
        // indirect lowerings into `Shims`.
        for (core_wasm_name, required) in info.required_imports.iter() {
            let import_name = if *core_wasm_name == BARE_FUNC_MODULE_NAME {
                None
            } else {
                Some(core_wasm_name.to_string())
            };
            let import = &self.info.import_map[&import_name];
            ret.append_indirect(
                core_wasm_name,
                CustomModule::Main,
                import,
                &required.funcs,
                info.metadata,
                &mut signatures,
            )
            .context("failed to register indirect shims for main module")?;
        }

        // For all required adapter modules a shim is created for each required
        // function and additionally a set of shims are created for the
        // interface imported into the shim module itself.
        for (adapter_name, adapter) in self.info.adapters.iter() {
            for (name, required) in adapter.info.required_imports.iter() {
                let import_name = if *name == BARE_FUNC_MODULE_NAME {
                    None
                } else {
                    Some(name.to_string())
                };
                let import = &self.info.import_map[&import_name];
                ret.append_indirect(
                    name,
                    CustomModule::Adapter(adapter_name),
                    import,
                    &required.funcs,
                    adapter.info.metadata,
                    &mut signatures,
                )
                .with_context(|| {
                    format!("failed to register indirect shims for adapter {adapter_name}")
                })?;
            }

            self.encode_resource_dtors(
                CustomModule::Adapter(adapter_name),
                &adapter.info.required_resource_funcs,
                &mut signatures,
                &mut ret,
            );

            let funcs = match self.info.info.adapters_required.get(adapter_name) {
                Some(funcs) => funcs,
                None => continue,
            };
            for (func, ty) in funcs {
                let name = ret.list.len().to_string();
                log::debug!("shim {name} is adapter `{adapter_name}::{func}`");
                signatures.push(WasmSignature {
                    params: ty.params().iter().map(to_wasm_type).collect(),
                    results: ty.results().iter().map(to_wasm_type).collect(),
                    indirect_params: false,
                    retptr: false,
                });
                ret.list.push(Shim {
                    name,
                    debug_name: format!("adapt-{adapter_name}-{func}"),
                    // Pessimistically assume that all adapters require memory
                    // in one form or another. While this isn't technically true
                    // it's true enough for WASI.
                    options: RequiredOptions::MEMORY,
                    kind: ShimKind::Adapter {
                        adapter: adapter_name,
                        func,
                    },
                });
            }
        }

        self.encode_resource_dtors(
            CustomModule::Main,
            &self.info.info.required_resource_funcs,
            &mut signatures,
            &mut ret,
        );

        if ret.list.is_empty() {
            return Ok(ret);
        }

        for shim in ret.list.iter() {
            let prev = ret.shim_names.insert(shim.kind.clone(), shim.name.clone());
            assert!(prev.is_none());
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

        for (i, (sig, shim)) in signatures.iter().zip(&ret.list).enumerate() {
            let i = i as u32;
            let type_index = *sigs.entry(sig).or_insert_with(|| {
                let index = types.len();
                types.function(
                    sig.params.iter().map(to_val_type),
                    sig.results.iter().map(to_val_type),
                );
                index
            });

            functions.function(type_index);
            Self::encode_shim_function(type_index, i, &mut code, sig.params.len() as u32);
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
            minimum: signatures.len() as u32,
            maximum: Some(signatures.len() as u32),
        };

        tables.table(table_type);

        exports.export(INDIRECT_TABLE_NAME, ExportKind::Table, 0);
        imports_section.import("", INDIRECT_TABLE_NAME, table_type);

        elements.active(
            None,
            &ConstExpr::i32_const(0),
            Elements::Functions(&func_indexes),
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
        shim.section(&names);

        let mut fixups = Module::default();
        fixups.section(&types);
        fixups.section(&imports_section);
        fixups.section(&elements);
        fixups.section(&RawCustomSection(
            &crate::base_producers().raw_custom_section(),
        ));

        let mut names = NameSection::new();
        names.module("wit-component:fixups");
        fixups.section(&names);

        let shim_module_index = self.component.core_module(&shim);
        self.fixups_module_index = Some(self.component.core_module(&fixups));
        self.shim_instance_index = Some(self.component.core_instantiate(shim_module_index, []));

        return Ok(ret);

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

    fn encode_shim_function(
        type_index: u32,
        func_index: u32,
        code: &mut CodeSection,
        param_count: u32,
    ) {
        let mut func = wasm_encoder::Function::new(std::iter::empty());
        for i in 0..param_count {
            func.instruction(&Instruction::LocalGet(i));
        }
        func.instruction(&Instruction::I32Const(func_index as i32));
        func.instruction(&Instruction::CallIndirect {
            ty: type_index,
            table: 0,
        });
        func.instruction(&Instruction::End);
        code.function(&func);
    }

    fn encode_indirect_lowerings(&mut self, shims: &Shims<'_>) -> Result<()> {
        if shims.list.is_empty() {
            return Ok(());
        }

        let shim_instance_index = self
            .shim_instance_index
            .expect("must have an instantiated shim");

        let table_index = self.component.core_alias_export(
            shim_instance_index,
            INDIRECT_TABLE_NAME,
            ExportKind::Table,
        );

        let mut exports = Vec::new();
        exports.push((INDIRECT_TABLE_NAME, ExportKind::Table, table_index));

        for shim in shims.list.iter() {
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
                    let (name, _) = interface.lowerings.get_index(*index).unwrap();
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

                    let realloc = match realloc {
                        CustomModule::Main => self.realloc_index,
                        CustomModule::Adapter(name) => self.adapter_import_reallocs[name],
                    };

                    self.component.lower_func(
                        func_index,
                        shim.options
                            .into_iter(*encoding, self.memory_index, realloc)?,
                    )
                }

                // Adapter shims are defined by an export from an adapter
                // instance, so use the specified name here and the previously
                // created instances to get the core item that represents the
                // shim.
                ShimKind::Adapter { adapter, func } => self.component.core_alias_export(
                    self.adapter_instances[adapter],
                    func,
                    ExportKind::Func,
                ),

                // Resources are required for a module to be instantiated
                // meaning that any destructor for the resource must be called
                // indirectly due to the otherwise circular dependency between
                // the module and the resource itself.
                ShimKind::ResourceDtor {
                    module,
                    import,
                    resource,
                } => {
                    let funcs = match module {
                        CustomModule::Main => &self.info.info.required_resource_funcs,
                        CustomModule::Adapter(name) => {
                            &self.info.adapters[name].info.required_resource_funcs
                        }
                    };

                    self.component.core_alias_export(
                        self.instance_index.unwrap(),
                        funcs[*import][*resource].dtor_export.as_deref().unwrap(),
                        ExportKind::Func,
                    )
                }
            };

            exports.push((shim.name.as_str(), ExportKind::Func, core_func_index));
        }

        let instance_index = self.component.core_instantiate_exports(exports);
        self.component.core_instantiate(
            self.fixups_module_index.expect("must have fixup module"),
            [("", ModuleArg::Instance(instance_index))],
        );
        Ok(())
    }

    fn instantiate_core_module<'b, A>(&mut self, args: A, info: &ValidatedModule<'_>)
    where
        A: IntoIterator<Item = (&'b str, ModuleArg)>,
        A::IntoIter: ExactSizeIterator,
    {
        assert!(self.instance_index.is_none());

        let instance_index = self
            .component
            .core_instantiate(self.module_index.expect("core module encoded"), args);

        if info.has_memory {
            self.memory_index = Some(self.component.core_alias_export(
                instance_index,
                "memory",
                ExportKind::Memory,
            ));
        }

        if let Some(name) = &info.realloc {
            self.realloc_index = Some(self.component.core_alias_export(
                instance_index,
                name,
                ExportKind::Func,
            ));
        }

        self.instance_index = Some(instance_index);
    }

    fn encode_resource_dtors<'b>(
        &mut self,
        module: CustomModule<'b>,
        funcs: &'b IndexMap<String, IndexMap<String, ResourceInfo>>,
        signatures: &mut Vec<WasmSignature>,
        shims: &mut Shims<'b>,
    ) {
        // Any resource destructors are encoded through the shim module. The
        // core wasm probably imports resource intrinsics which requires the
        // resource definition, but the resource definition requires
        // the destructor to be available. The shim module breaks this
        // circular dependency.
        for (import, info) in funcs.iter() {
            for (resource, info) in info {
                if info.dtor_export.is_none() {
                    continue;
                }
                signatures.push(WasmSignature {
                    params: vec![WasmType::I32],
                    results: Vec::new(),
                    indirect_params: false,
                    retptr: false,
                });
                let name = shims.list.len().to_string();
                shims.list.push(Shim {
                    name,
                    debug_name: format!("dtor-{import}-{resource}"),
                    options: RequiredOptions::empty(),
                    kind: ShimKind::ResourceDtor {
                        module,
                        import,
                        resource,
                    },
                });
            }
        }
    }

    fn add_resource_funcs<'b>(
        &mut self,
        module: CustomModule<'b>,
        funcs: &'b IndexMap<String, IndexMap<String, ResourceInfo>>,
        shims: &Shims,
        args: &mut Vec<(&'b str, ModuleArg)>,
    ) {
        for (import, info) in funcs {
            let mut exports = Vec::new();
            for (resource, info) in info {
                // Destructors for resources live on the shim module previously
                // created, so if one is specified create the resource with
                // the shim module that currently exists. The shim will get
                // filled in later with the actual destructor after the main
                // module is instantiated.
                let dtor = info.dtor_export.as_deref().map(|_| {
                    self.component.core_alias_export(
                        self.shim_instance_index.unwrap(),
                        &shims.shim_names[&ShimKind::ResourceDtor {
                            module,
                            import,
                            resource,
                        }],
                        ExportKind::Func,
                    )
                });
                let resource_idx = self.component.type_resource(ValType::I32, dtor);
                let prev = self.export_type_map.insert(info.id, resource_idx);
                assert!(prev.is_none());

                if let Some(name) = info.drop_import.as_deref() {
                    let index = self.component.resource_drop(resource_idx);
                    exports.push((name, ExportKind::Func, index));
                }
                if let Some(name) = info.rep_import.as_deref() {
                    let index = self.component.resource_rep(resource_idx);
                    exports.push((name, ExportKind::Func, index));
                }
                if let Some(name) = info.new_import.as_deref() {
                    let index = self.component.resource_new(resource_idx);
                    exports.push((name, ExportKind::Func, index));
                }
            }
            if !exports.is_empty() {
                let index = self.component.core_instantiate_exports(exports);
                args.push((import.as_str(), ModuleArg::Instance(index)));
            }
        }
    }

    /// This function will instantiate the specified adapter module, which may
    /// depend on previously-instantiated modules.
    fn instantiate_adapter_module(
        &mut self,
        shims: &Shims<'_>,
        name: &'a str,
        adapter: &WorldAdapter,
    ) {
        let mut args = Vec::new();

        let mut core_exports = Vec::new();
        for export_name in adapter.info.needs_core_exports.iter() {
            let mut core_export_name = export_name.as_str();
            // provide cabi_realloc_adapter as cabi_realloc to adapters
            // if it exists
            if export_name == "cabi_realloc" {
                if let Some(adapter_realloc) = self.info.info.adapter_realloc {
                    core_export_name = adapter_realloc;
                }
            }
            let index = self.component.core_alias_export(
                self.instance_index
                    .expect("adaptee index set at this point"),
                core_export_name,
                ExportKind::Func,
            );
            core_exports.push((export_name.as_str(), ExportKind::Func, index));
        }
        if !core_exports.is_empty() {
            let instance = self.component.core_instantiate_exports(core_exports);
            args.push((MAIN_MODULE_IMPORT_NAME, ModuleArg::Instance(instance)));
        }
        // The adapter may either be a library or a "minimal" adapter.  If it's
        // the former, we use `LibraryInfo::arguments` to populate inter-module
        // instantiation arguments.
        if let Some(library_info) = adapter.library_info {
            for (import_name, instance) in &library_info.arguments {
                let resolve = |which: &_| match which {
                    MainOrAdapter::Main => self.instance_index.unwrap(),
                    MainOrAdapter::Adapter(adapter_name) => *self
                        .adapter_instances
                        .get(adapter_name.as_str())
                        .unwrap_or_else(|| {
                            panic!(
                                "adapter {name} needs {adapter_name}, \
                                     which has not yet been instantiated"
                            )
                        }),
                };

                args.push((
                    import_name,
                    ModuleArg::Instance(match instance {
                        Instance::MainOrAdapter(which) => resolve(which),
                        Instance::Items(items) => {
                            let exports = items
                                .iter()
                                .map(|item| {
                                    (
                                        item.alias.as_str(),
                                        item.kind,
                                        self.component.core_alias_export(
                                            resolve(&item.which),
                                            &item.name,
                                            item.kind,
                                        ),
                                    )
                                })
                                .collect::<Vec<_>>();
                            self.component.core_instantiate_exports(exports)
                        }
                    }),
                ));
            }
        } else {
            // If the adapter module requires a `memory` import then specify
            // that here. For now assume that the module name of the memory is
            // different from the imported interface. That's true enough for now
            // since it's `env::memory`.
            if let Some((module, name)) = &adapter.info.needs_memory {
                for (import_name, _) in adapter.info.required_imports.iter() {
                    assert!(module != import_name);
                }
                assert!(module != name);
                let memory = self.memory_index.unwrap();
                let instance = self.component.core_instantiate_exports([(
                    name.as_str(),
                    ExportKind::Memory,
                    memory,
                )]);
                args.push((module.as_str(), ModuleArg::Instance(instance)));
            }
        }
        for (import_name, _) in adapter.info.required_imports.iter() {
            let instance = self.import_instance_to_lowered_core_instance(
                CustomModule::Adapter(name),
                import_name,
                shims,
                adapter.info.metadata,
            );
            args.push((import_name, ModuleArg::Instance(instance)));
        }

        self.add_resource_funcs(
            CustomModule::Adapter(name),
            &adapter.info.required_resource_funcs,
            shims,
            &mut args,
        );

        let instance = self
            .component
            .core_instantiate(self.adapter_modules[name], args);
        self.adapter_instances.insert(name, instance);

        let realloc = adapter.info.export_realloc.as_ref().map(|name| {
            self.component
                .core_alias_export(instance, name, ExportKind::Func)
        });
        self.adapter_export_reallocs.insert(name, realloc);
        let realloc = adapter.info.import_realloc.as_ref().map(|name| {
            self.component
                .core_alias_export(instance, name, ExportKind::Func)
        });
        self.adapter_import_reallocs.insert(name, realloc);
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
    list: Vec<Shim<'a>>,

    /// A map from a shim to the name of the shim in the shim instance.
    shim_names: IndexMap<ShimKind<'a>, String>,
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
        /// The import that the resource was defined for.
        import: &'a str,
        /// The name of the resource being destroyed.
        resource: &'a str,
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

impl<'a> Shims<'a> {
    /// Adds all shims necessary for the `import` provided, namely iterating
    /// over its indirect lowerings and appending a shim per lowering.
    fn append_indirect(
        &mut self,
        core_wasm_module: &'a str,
        for_module: CustomModule<'a>,
        import: &ImportedInterface,
        required: &IndexSet<String>,
        metadata: &ModuleMetadata,
        sigs: &mut Vec<WasmSignature>,
    ) -> Result<()> {
        let interface = if core_wasm_module == BARE_FUNC_MODULE_NAME {
            None
        } else {
            Some(core_wasm_module.to_string())
        };
        for (index, (name, lowering)) in import.lowerings.iter().enumerate() {
            if !required.contains(name.as_str()) {
                continue;
            }
            let shim_name = self.list.len().to_string();
            log::debug!(
                "shim {shim_name} is import `{core_wasm_module}` lowering {index} `{name}`",
            );
            match lowering {
                Lowering::Direct | Lowering::ResourceDrop(_) => {}

                Lowering::Indirect { sig, options } => {
                    sigs.push(sig.clone());
                    let encoding = *metadata
                        .import_encodings
                        .get(&(core_wasm_module.to_string(), name.clone()))
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "missing component metadata for import of \
                                `{core_wasm_module}::{name}`"
                            )
                        })?;
                    self.list.push(Shim {
                        name: shim_name,
                        debug_name: format!("indirect-{core_wasm_module}-{name}"),
                        options: *options,
                        kind: ShimKind::IndirectLowering {
                            interface: interface.clone(),
                            index,
                            realloc: for_module,
                            encoding,
                        },
                    });
                }
            }
        }
        Ok(())
    }
}

/// Alias argument to an instantiation
#[derive(Clone)]
pub struct Item {
    pub alias: String,
    pub kind: ExportKind,
    pub which: MainOrAdapter,
    pub name: String,
}

/// Module argument to an instantiation
#[derive(Clone)]
pub enum MainOrAdapter {
    Main,
    Adapter(String),
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
struct Adapter {
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
    metadata: Bindgen,
    validate: bool,
    main_module_exports: IndexSet<WorldKey>,
    adapters: IndexMap<String, Adapter>,
    import_name_map: HashMap<String, String>,
    realloc_via_memory_grow: bool,
}

impl ComponentEncoder {
    /// Set the core module to encode as a component.
    /// This method will also parse any component type information stored in custom sections
    /// inside the module, and add them as the interface, imports, and exports.
    /// It will also add any producers information inside the component type information to the
    /// core module.
    pub fn module(mut self, module: &[u8]) -> Result<Self> {
        let (wasm, metadata) = metadata::decode(module)?;
        let world = self
            .metadata
            .merge(metadata)
            .context("failed merge WIT metadata for module with previous metadata")?;
        self.main_module_exports
            .extend(self.metadata.resolve.worlds[world].exports.keys().cloned());
        self.module = if let Some(producers) = &self.metadata.producers {
            producers.add_to_wasm(&wasm)?
        } else {
            wasm
        };
        Ok(self)
    }

    /// Sets whether or not the encoder will validate its output.
    pub fn validate(mut self, validate: bool) -> Self {
        self.validate = validate;
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
        let (wasm, metadata) = metadata::decode(bytes)?;
        // Merge the adapter's document into our own document to have one large
        // document, and then afterwards merge worlds as well.
        //
        // The first `merge` operation will interleave equivalent packages from
        // each adapter into packages that are stored within our own resolve.
        // The second `merge_worlds` operation will then ensure that both the
        // adapter and the main module have compatible worlds, meaning that they
        // either import the same items or they import disjoint items, for
        // example.
        let world = self
            .metadata
            .resolve
            .merge(metadata.resolve)
            .with_context(|| {
                format!("failed to merge WIT packages of adapter `{name}` into main packages")
            })?
            .worlds[metadata.world.index()];
        self.metadata
            .resolve
            .merge_worlds(world, self.metadata.world)
            .with_context(|| {
                format!("failed to merge WIT world of adapter `{name}` into main package")
            })?;
        let exports = self.metadata.resolve.worlds[world]
            .exports
            .keys()
            .cloned()
            .collect();
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
                wasm,
                metadata: metadata.metadata,
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
    pub fn encode(&self) -> Result<Vec<u8>> {
        if self.module.is_empty() {
            bail!("a module is required when encoding a component");
        }

        let world = ComponentWorld::new(self)?;
        let mut state = EncodingState {
            component: ComponentBuilder::default(),
            module_index: None,
            instance_index: None,
            memory_index: None,
            realloc_index: None,
            shim_instance_index: None,
            fixups_module_index: None,
            adapter_modules: IndexMap::new(),
            adapter_instances: IndexMap::new(),
            adapter_import_reallocs: IndexMap::new(),
            adapter_export_reallocs: IndexMap::new(),
            import_type_map: HashMap::new(),
            import_func_type_map: HashMap::new(),
            export_type_map: HashMap::new(),
            export_func_type_map: HashMap::new(),
            imported_instances: Default::default(),
            imported_funcs: Default::default(),
            exported_instances: Default::default(),
            info: &world,
        };
        state.encode_imports(&self.import_name_map)?;
        state.encode_core_modules();
        state.encode_core_instantiation()?;
        state.encode_exports(CustomModule::Main)?;
        for name in self.adapters.keys() {
            state.encode_exports(CustomModule::Adapter(name))?;
        }
        state
            .component
            .raw_custom_section(&crate::base_producers().raw_custom_section());
        let bytes = state.component.finish();

        if self.validate {
            let mut validator = Validator::new_with_features(WasmFeatures {
                component_model: true,
                ..Default::default()
            });

            validator
                .validate_all(&bytes)
                .context("failed to validate component output")?;
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod test {
    use crate::{dummy_module, embed_component_metadata};

    use super::*;
    use std::path::Path;
    use wit_parser::UnresolvedPackage;

    #[test]
    fn it_renames_imports() {
        let mut resolve = Resolve::new();
        let pkg = resolve
            .push(
                UnresolvedPackage::parse(
                    Path::new("test.wit"),
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
                .unwrap(),
            )
            .unwrap();

        let world = resolve.select_world(pkg, None).unwrap();

        let mut module = dummy_module(&resolve, world);

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

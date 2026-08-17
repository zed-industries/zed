use crate::encoding::{Instance, Item, LibraryInfo, MainOrAdapter, ModuleImportMap};
use crate::{ComponentEncoder, StringEncoding};
use anyhow::{Context, Result, anyhow, bail};
use indexmap::{IndexMap, IndexSet, map::Entry};
use std::fmt;
use std::hash::Hash;
use std::mem;
use wasm_encoder::ExportKind;
use wasmparser::names::{ComponentName, ComponentNameKind};
use wasmparser::{
    Encoding, ExternalKind, FuncType, Parser, Payload, TypeRef, ValType, ValidPayload, Validator,
    WasmFeatures, types::TypesRef,
};
use wit_parser::{
    Function, InterfaceId, PackageName, Resolve, Type, TypeDefKind, TypeId, World, WorldId,
    WorldItem, WorldKey,
    abi::{AbiVariant, WasmSignature, WasmType},
};

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

/// Metadata about a validated module and what was found internally.
///
/// This structure houses information about `imports` and `exports` to the
/// module. Each of these specialized types contains "connection" information
/// between a module's imports/exports and the WIT or component-level constructs
/// they correspond to.

#[derive(Default)]
pub struct ValidatedModule {
    /// Information about a module's imports.
    pub imports: ImportMap,

    /// Information about a module's exports.
    pub exports: ExportMap,
}

impl ValidatedModule {
    fn new(
        encoder: &ComponentEncoder,
        bytes: &[u8],
        exports: &IndexSet<WorldKey>,
        import_map: Option<&ModuleImportMap>,
        info: Option<&LibraryInfo>,
    ) -> Result<ValidatedModule> {
        let mut validator = Validator::new_with_features(WasmFeatures::all());
        let mut ret = ValidatedModule::default();

        for payload in Parser::new(0).parse_all(bytes) {
            let payload = payload?;
            if let ValidPayload::End(_) = validator.payload(&payload)? {
                break;
            }

            let types = validator.types(0).unwrap();

            match payload {
                Payload::Version { encoding, .. } if encoding != Encoding::Module => {
                    bail!("data is not a WebAssembly module");
                }
                Payload::ImportSection(s) => {
                    for import in s.into_imports() {
                        let import = import?;
                        ret.imports.add(import, encoder, import_map, info, types)?;
                    }
                }
                Payload::ExportSection(s) => {
                    for export in s {
                        let export = export?;
                        ret.exports.add(export, encoder, &exports, types)?;
                    }
                }
                _ => continue,
            }
        }

        ret.exports.validate(encoder, exports)?;

        Ok(ret)
    }
}

/// Metadata information about a module's imports.
///
/// This structure maintains the connection between component model "things" and
/// core wasm "things" by ensuring that all imports to the core wasm module are
/// classified by the `Import` enumeration.
#[derive(Default)]
pub struct ImportMap {
    /// The first level of the map here is the module namespace of the import
    /// and the second level of the map is the field namespace. The item is then
    /// how the import is satisfied.
    names: IndexMap<String, ImportInstance>,
}

pub enum ImportInstance {
    /// This import is satisfied by an entire instance of another
    /// adapter/module.
    Whole(MainOrAdapter),

    /// This import is satisfied by filling out each name possibly differently.
    Names(IndexMap<String, Import>),
}

/// Represents metadata about a `stream<T>` or `future<T>` type for a specific
/// payload type `T`.
///
/// Currently, the name mangling scheme we use to represent `stream` and
/// `future` intrinsics as core module function imports refers to a specific
/// `stream` or `future` type by naming an imported or exported component
/// function which has that type as a parameter or return type (where the
/// specific type is referred to using an ordinal numbering scheme).  Not only
/// does this approach unambiguously indicate the type of interest, but it
/// allows us to reuse the `realloc`, string encoding, memory, etc. used by that
/// function when emitting intrinsic declarations.
///
/// TODO: Rather than reusing the same canon opts as the function in which the
/// type appears, consider encoding them in the name mangling stream on an
/// individual basis, similar to how we encode `error-context.*` built-in
/// imports.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct PayloadInfo {
    /// The original, mangled import name used to import this built-in
    /// (currently used only for hashing and debugging).
    pub name: String,
    /// The resolved type id for the `stream` or `future` type of interest.
    ///
    /// If `Unit{Future,Stream}` this means that it's a "unit" payload or has no associated
    /// type being sent.
    pub ty: PayloadType,
    /// The world key representing the import or export context of `function`.
    pub key: WorldKey,
    /// The interface that `function` was imported from or exported in, if any.
    pub interface: Option<InterfaceId>,
    /// Whether `function` is being imported or exported.
    ///
    /// This may affect how we emit the declaration of the built-in, e.g. if the
    /// payload type is an exported resource.
    pub imported: bool,
}

/// The type of future/stream referenced by a `PayloadInfo`
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum PayloadType {
    /// This is a future or stream located in a `Resolve` where `id` points to
    /// either of `TypeDefKind::{Future, Stream}`.
    Type {
        id: TypeId,
        /// The component-level function import or export where the type
        /// appeared as a parameter or result type.
        function: String,
    },
    /// This is a `future` (no type)
    UnitFuture,
    /// This is a `stream` (no type)
    UnitStream,
}

impl PayloadInfo {
    /// Returns the payload type that this future/stream type is using.
    pub fn payload(&self, resolve: &Resolve) -> Option<Type> {
        let id = match self.ty {
            PayloadType::Type { id, .. } => id,
            PayloadType::UnitFuture | PayloadType::UnitStream => return None,
        };
        match resolve.types[id].kind {
            TypeDefKind::Future(payload) | TypeDefKind::Stream(payload) => payload,
            _ => unreachable!(),
        }
    }
}

/// The different kinds of items that a module or an adapter can import.
///
/// This is intended to be an exhaustive definition of what can be imported into
/// core modules within a component that wit-component supports. This doesn't
/// get down to the level of storing any idx numbers; at its most specific, it
/// gives a name.
#[derive(Debug, Clone)]
pub enum Import {
    /// A top-level world function, with the name provided here, is imported
    /// into the module.
    WorldFunc(WorldKey, String, AbiVariant),

    /// An interface's function is imported into the module.
    ///
    /// The `WorldKey` here is the name of the interface in the world in
    /// question. The `InterfaceId` is the interface that was imported from and
    /// `String` is the WIT name of the function.
    InterfaceFunc(WorldKey, InterfaceId, String, AbiVariant),

    /// An imported resource's destructor is imported.
    ///
    /// The key provided indicates whether it's for the top-level types of the
    /// world (`None`) or an interface (`Some` with the name of the interface).
    /// The `TypeId` is what resource is being dropped.
    ImportedResourceDrop(WorldKey, Option<InterfaceId>, TypeId),

    /// A `canon resource.drop` intrinsic for an exported item is being
    /// imported.
    ///
    /// This lists the key of the interface that's exporting the resource plus
    /// the id within that interface.
    ExportedResourceDrop(WorldKey, TypeId),

    /// A `canon resource.new` intrinsic for an exported item is being
    /// imported.
    ///
    /// This lists the key of the interface that's exporting the resource plus
    /// the id within that interface.
    ExportedResourceNew(WorldKey, TypeId),

    /// A `canon resource.rep` intrinsic for an exported item is being
    /// imported.
    ///
    /// This lists the key of the interface that's exporting the resource plus
    /// the id within that interface.
    ExportedResourceRep(WorldKey, TypeId),

    /// An export of an adapter is being imported with the specified type.
    ///
    /// This is used for when the main module imports an adapter function. The
    /// adapter name and function name match the module's own import, and the
    /// type must match that listed here.
    AdapterExport {
        adapter: String,
        func: String,
        ty: FuncType,
    },

    /// An adapter is importing the memory of the main module.
    ///
    /// (should be combined with `MainModuleExport` below one day)
    MainModuleMemory,

    /// An adapter is importing an arbitrary item from the main module.
    MainModuleExport { name: String, kind: ExportKind },

    /// An arbitrary item from either the main module or an adapter is being
    /// imported.
    ///
    /// (should probably subsume `MainModule*` and maybe `AdapterExport` above
    /// one day.
    Item(Item),

    /// A `canon task.return` intrinsic for an exported function.
    ///
    /// This allows an exported function to return a value and then continue
    /// running.
    ///
    /// As of this writing, only async-lifted exports use `task.return`, but the
    /// plan is to also support it for sync-lifted exports in the future as
    /// well.
    ExportedTaskReturn(WorldKey, Option<InterfaceId>, String, Option<Type>),

    /// A `canon task.cancel` intrinsic for an exported function.
    ///
    /// This allows an exported function to acknowledge a `CANCELLED` event.
    ExportedTaskCancel,

    /// The `context.get` intrinsic for the nth slot of storage.
    ContextGet(u32),
    /// The `context.set` intrinsic for the nth slot of storage.
    ContextSet(u32),

    /// A `canon backpressure.inc` intrinsic.
    BackpressureInc,

    /// A `canon backpressure.dec` intrinsic.
    BackpressureDec,

    /// A `waitable-set.new` intrinsic.
    WaitableSetNew,

    /// A `canon waitable-set.wait` intrinsic.
    ///
    /// This allows the guest to wait for any pending calls to async-lowered
    /// imports and/or `stream` and `future` operations to complete without
    /// unwinding the current Wasm stack.
    WaitableSetWait { cancellable: bool },

    /// A `canon waitable.poll` intrinsic.
    ///
    /// This allows the guest to check whether any pending calls to
    /// async-lowered imports and/or `stream` and `future` operations have
    /// completed without unwinding the current Wasm stack and without blocking.
    WaitableSetPoll { cancellable: bool },

    /// A `waitable-set.drop` intrinsic.
    WaitableSetDrop,

    /// A `waitable.join` intrinsic.
    WaitableJoin,

    /// A `canon thread.yield` intrinsic.
    ///
    /// This allows the guest to yield (e.g. during an computationally-intensive
    /// operation) and allow other subtasks to make progress.
    ThreadYield { cancellable: bool },

    /// A `canon subtask.drop` intrinsic.
    ///
    /// This allows the guest to release its handle to a completed subtask.
    SubtaskDrop,

    /// A `canon subtask.cancel` intrinsic.
    ///
    /// This allows the guest to cancel an in-progress subtask.
    SubtaskCancel { async_: bool },

    /// A `canon stream.new` intrinsic.
    ///
    /// This allows the guest to create a new `stream` of the specified type.
    StreamNew(PayloadInfo),

    /// A `canon stream.read` intrinsic.
    ///
    /// This allows the guest to read the next values (if any) from the specified
    /// stream.
    StreamRead { async_: bool, info: PayloadInfo },

    /// A `canon stream.write` intrinsic.
    ///
    /// This allows the guest to write one or more values to the specified
    /// stream.
    StreamWrite { async_: bool, info: PayloadInfo },

    /// A `canon stream.cancel-read` intrinsic.
    ///
    /// This allows the guest to cancel a pending read it initiated earlier (but
    /// which may have already partially or entirely completed).
    StreamCancelRead { info: PayloadInfo, async_: bool },

    /// A `canon stream.cancel-write` intrinsic.
    ///
    /// This allows the guest to cancel a pending write it initiated earlier
    /// (but which may have already partially or entirely completed).
    StreamCancelWrite { info: PayloadInfo, async_: bool },

    /// A `canon stream.drop-readable` intrinsic.
    ///
    /// This allows the guest to drop the readable end of a `stream`.
    StreamDropReadable(PayloadInfo),

    /// A `canon stream.drop-writable` intrinsic.
    ///
    /// This allows the guest to drop the writable end of a `stream`.
    StreamDropWritable(PayloadInfo),

    /// A `canon future.new` intrinsic.
    ///
    /// This allows the guest to create a new `future` of the specified type.
    FutureNew(PayloadInfo),

    /// A `canon future.read` intrinsic.
    ///
    /// This allows the guest to read the value (if any) from the specified
    /// future.
    FutureRead { async_: bool, info: PayloadInfo },

    /// A `canon future.write` intrinsic.
    ///
    /// This allows the guest to write a value to the specified future.
    FutureWrite { async_: bool, info: PayloadInfo },

    /// A `canon future.cancel-read` intrinsic.
    ///
    /// This allows the guest to cancel a pending read it initiated earlier (but
    /// which may have already completed).
    FutureCancelRead { info: PayloadInfo, async_: bool },

    /// A `canon future.cancel-write` intrinsic.
    ///
    /// This allows the guest to cancel a pending write it initiated earlier
    /// (but which may have already completed).
    FutureCancelWrite { info: PayloadInfo, async_: bool },

    /// A `canon future.drop-readable` intrinsic.
    ///
    /// This allows the guest to drop the readable end of a `future`.
    FutureDropReadable(PayloadInfo),

    /// A `canon future.drop-writable` intrinsic.
    ///
    /// This allows the guest to drop the writable end of a `future`.
    FutureDropWritable(PayloadInfo),

    /// A `canon error-context.new` intrinsic.
    ///
    /// This allows the guest to create a new `error-context` instance with a
    /// specified debug message.
    ErrorContextNew { encoding: StringEncoding },

    /// A `canon error-context.debug-message` intrinsic.
    ///
    /// This allows the guest to retrieve the debug message from a
    /// `error-context` instance.  Note that the content of this message might
    /// not be identical to what was passed in to `error-context.new`.
    ErrorContextDebugMessage { encoding: StringEncoding },

    /// A `canon error-context.drop` intrinsic.
    ///
    /// This allows the guest to release its handle to the specified
    /// `error-context` instance.
    ErrorContextDrop,

    /// A `canon thread.index` intrinsic.
    ///
    /// This allows the guest to get the index of the current thread.
    ThreadIndex,

    /// A `canon thread.new-indirect` intrinsic.
    ///
    /// This allows the guest to create a new thread running a specified function.
    ThreadNewIndirect,

    /// A `canon thread.switch-to` intrinsic.
    ///
    /// This allows the guest to switch execution to another thread.
    ThreadSwitchTo { cancellable: bool },

    /// A `canon thread.suspend` intrinsic.
    ///
    /// This allows the guest to suspend the current thread, switching execution to
    /// an unspecified thread.
    ThreadSuspend { cancellable: bool },

    /// A `canon thread.resume-later` intrinsic.
    ///
    /// This allows the guest to mark a suspended thread for later resumption.
    ThreadResumeLater,

    /// A `canon thread.yield-to` intrinsic.
    ///
    /// This allows the guest to suspend, yielding execution to a specified thread.
    ThreadYieldTo { cancellable: bool },
}

impl ImportMap {
    /// Returns the list of items that the adapter named `name` must export.
    pub fn required_from_adapter(&self, name: &str) -> IndexMap<String, FuncType> {
        let names = match self.names.get(name) {
            Some(ImportInstance::Names(names)) => names,
            _ => return IndexMap::new(),
        };
        names
            .iter()
            .map(|(_, import)| match import {
                Import::AdapterExport { ty, func, adapter } => {
                    assert_eq!(adapter, name);
                    (func.clone(), ty.clone())
                }
                _ => unreachable!(),
            })
            .collect()
    }

    /// Returns an iterator over all individual imports registered in this map.
    ///
    /// Note that this doesn't iterate over the "whole instance" imports.
    pub fn imports(&self) -> impl Iterator<Item = (&str, &str, &Import)> + '_ {
        self.names
            .iter()
            .filter_map(|(module, m)| match m {
                ImportInstance::Names(names) => Some((module, names)),
                ImportInstance::Whole(_) => None,
            })
            .flat_map(|(module, m)| {
                m.iter()
                    .map(move |(field, import)| (module.as_str(), field.as_str(), import))
            })
    }

    /// Returns the map for how all imports must be satisfied.
    pub fn modules(&self) -> &IndexMap<String, ImportInstance> {
        &self.names
    }

    /// Classify an import and call `insert_import()` on it. Used during
    /// validation to build up this `ImportMap`.
    fn add(
        &mut self,
        import: wasmparser::Import<'_>,
        encoder: &ComponentEncoder,
        import_map: Option<&ModuleImportMap>,
        library_info: Option<&LibraryInfo>,
        types: TypesRef<'_>,
    ) -> Result<()> {
        if self.classify_import_with_library(import, library_info)? {
            return Ok(());
        }
        let mut import_to_classify = import;
        if let Some(map) = import_map {
            if let Some(original_name) = map.original_name(&import) {
                import_to_classify.name = original_name;
            }
        }
        let item = self
            .classify(import_to_classify, encoder, types)
            .with_context(|| {
                format!(
                    "failed to resolve import `{}::{}`",
                    import.module, import.name,
                )
            })?;
        self.insert_import(import, item)
    }

    /// Determines what kind of thing is being imported: maps it from the
    /// module/name/type triple in the raw wasm module to an enum.
    ///
    /// Handles a few special cases, then delegates to
    /// `classify_component_model_import()`.
    fn classify(
        &self,
        import: wasmparser::Import<'_>,
        encoder: &ComponentEncoder,
        types: TypesRef<'_>,
    ) -> Result<Import> {
        // Special-case the main module's memory imported into adapters which
        // currently with `wasm-ld` is not easily configurable.
        if import.module == "env" && import.name == "memory" {
            return Ok(Import::MainModuleMemory);
        }

        // Special-case imports from the main module into adapters.
        if import.module == "__main_module__" {
            return Ok(Import::MainModuleExport {
                name: import.name.to_string(),
                kind: match import.ty {
                    TypeRef::Func(_) => ExportKind::Func,
                    TypeRef::Table(_) => ExportKind::Table,
                    TypeRef::Memory(_) => ExportKind::Memory,
                    TypeRef::Global(_) => ExportKind::Global,
                    TypeRef::Tag(_) => ExportKind::Tag,
                    TypeRef::FuncExact(_) => bail!("Unexpected func_exact export"),
                },
            });
        }

        let ty_index = match import.ty {
            TypeRef::Func(ty) => ty,
            _ => bail!("module is only allowed to import functions"),
        };
        let ty = types[types.core_type_at_in_module(ty_index)].unwrap_func();

        // Handle main module imports that match known adapters and set it up as
        // an import of an adapter export.
        if encoder.adapters.contains_key(import.module) {
            return Ok(Import::AdapterExport {
                adapter: import.module.to_string(),
                func: import.name.to_string(),
                ty: ty.clone(),
            });
        }

        let (module, names) = match import.module.strip_prefix("cm32p2") {
            Some(suffix) => (suffix, STANDARD),
            None if encoder.reject_legacy_names => (import.module, STANDARD),
            None => (import.module, LEGACY),
        };
        self.classify_component_model_import(module, import.name, encoder, ty, names)
    }

    /// Attempts to classify the import `{module}::{name}` with the rules
    /// specified in WebAssembly/component-model#378
    fn classify_component_model_import(
        &self,
        module: &str,
        name: &str,
        encoder: &ComponentEncoder,
        ty: &FuncType,
        names: &dyn NameMangling,
    ) -> Result<Import> {
        let resolve = &encoder.metadata.resolve;
        let world_id = encoder.metadata.world;
        let world = &resolve.worlds[world_id];

        if module == names.import_root() {
            if names.error_context_drop(name) {
                let expected = FuncType::new([ValType::I32], []);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ErrorContextDrop);
            }

            if names.backpressure_inc(name) {
                let expected = FuncType::new([], []);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::BackpressureInc);
            }

            if names.backpressure_dec(name) {
                let expected = FuncType::new([], []);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::BackpressureDec);
            }

            if names.waitable_set_new(name) {
                let expected = FuncType::new([], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::WaitableSetNew);
            }

            if let Some(info) = names.waitable_set_wait(name) {
                let expected = FuncType::new([ValType::I32; 2], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::WaitableSetWait {
                    cancellable: info.cancellable,
                });
            }

            if let Some(info) = names.waitable_set_poll(name) {
                let expected = FuncType::new([ValType::I32; 2], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::WaitableSetPoll {
                    cancellable: info.cancellable,
                });
            }

            if names.waitable_set_drop(name) {
                let expected = FuncType::new([ValType::I32], []);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::WaitableSetDrop);
            }

            if names.waitable_join(name) {
                let expected = FuncType::new([ValType::I32; 2], []);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::WaitableJoin);
            }

            if let Some(info) = names.thread_yield(name) {
                let expected = FuncType::new([], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ThreadYield {
                    cancellable: info.cancellable,
                });
            }

            if names.subtask_drop(name) {
                let expected = FuncType::new([ValType::I32], []);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::SubtaskDrop);
            }

            if let Some(info) = names.subtask_cancel(name) {
                let expected = FuncType::new([ValType::I32], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::SubtaskCancel {
                    async_: info.async_lowered,
                });
            }

            if let Some(encoding) = names.error_context_new(name) {
                let expected = FuncType::new([ValType::I32; 2], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ErrorContextNew { encoding });
            }

            if let Some(encoding) = names.error_context_debug_message(name) {
                let expected = FuncType::new([ValType::I32; 2], []);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ErrorContextDebugMessage { encoding });
            }

            if let Some(i) = names.context_get(name) {
                let expected = FuncType::new([], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ContextGet(i));
            }
            if let Some(i) = names.context_set(name) {
                let expected = FuncType::new([ValType::I32], []);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ContextSet(i));
            }
            if names.thread_index(name) {
                let expected = FuncType::new([], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ThreadIndex);
            }
            if names.thread_new_indirect(name) {
                let expected = FuncType::new([ValType::I32; 2], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ThreadNewIndirect);
            }
            if let Some(info) = names.thread_switch_to(name) {
                let expected = FuncType::new([ValType::I32], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ThreadSwitchTo {
                    cancellable: info.cancellable,
                });
            }
            if let Some(info) = names.thread_suspend(name) {
                let expected = FuncType::new([], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ThreadSuspend {
                    cancellable: info.cancellable,
                });
            }
            if names.thread_resume_later(name) {
                let expected = FuncType::new([ValType::I32], []);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ThreadResumeLater);
            }
            if let Some(info) = names.thread_yield_to(name) {
                let expected = FuncType::new([ValType::I32], [ValType::I32]);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Import::ThreadYieldTo {
                    cancellable: info.cancellable,
                });
            }

            let (key_name, abi) = names.world_key_name_and_abi(name);
            let key = WorldKey::Name(key_name.to_string());
            if let Some(WorldItem::Function(func)) = world.imports.get(&key) {
                validate_func(resolve, ty, func, abi)?;
                return Ok(Import::WorldFunc(key, func.name.clone(), abi));
            }

            if let Some(import) =
                self.maybe_classify_wit_intrinsic(name, None, encoder, ty, true, names)?
            {
                return Ok(import);
            }

            match world.imports.get(&key) {
                Some(_) => bail!("expected world top-level import `{name}` to be a function"),
                None => bail!("no top-level imported function `{name}` specified"),
            }
        }

        // Check for `[export]$root::[task-return]foo` or similar
        if matches!(
            module.strip_prefix(names.import_exported_intrinsic_prefix()),
            Some(module) if module == names.import_root()
        ) {
            if let Some(import) =
                self.maybe_classify_wit_intrinsic(name, None, encoder, ty, false, names)?
            {
                return Ok(import);
            }
        }

        let interface = match module.strip_prefix(names.import_non_root_prefix()) {
            Some(name) => name,
            None => bail!("unknown or invalid component model import syntax"),
        };

        if let Some(interface) = interface.strip_prefix(names.import_exported_intrinsic_prefix()) {
            let (key, id) = names.module_to_interface(interface, resolve, &world.exports)?;

            if let Some(import) =
                self.maybe_classify_wit_intrinsic(name, Some((key, id)), encoder, ty, false, names)?
            {
                return Ok(import);
            }
            bail!("unknown function `{name}`")
        }

        let (key, id) = names.module_to_interface(interface, resolve, &world.imports)?;
        let interface = &resolve.interfaces[id];
        let (function_name, abi) = names.interface_function_name_and_abi(name);
        if let Some(f) = interface.functions.get(function_name) {
            validate_func(resolve, ty, f, abi).with_context(|| {
                let name = resolve.name_world_key(&key);
                format!("failed to validate import interface `{name}`")
            })?;
            return Ok(Import::InterfaceFunc(key, id, f.name.clone(), abi));
        }

        if let Some(import) =
            self.maybe_classify_wit_intrinsic(name, Some((key, id)), encoder, ty, true, names)?
        {
            return Ok(import);
        }
        bail!(
            "import interface `{module}` is missing function \
             `{name}` that is required by the module",
        )
    }

    /// Attempts to detect and classify `name` as a WIT intrinsic.
    ///
    /// This function is a bit of a sprawling sequence of matches used to
    /// detect whether `name` corresponds to a WIT intrinsic, so specifically
    /// not a WIT function itself. This is only used for functions imported
    /// into a module but the import could be for an imported item in a world
    /// or an exported item.
    ///
    /// ## Parameters
    ///
    /// * `name` - the core module name which is being pattern-matched. This
    ///   should be the "field" of the import. This may include the "[async-lower]"
    ///   or "[cancellable]" prefixes.
    /// * `key_and_id` - this is the inferred "container" for the function
    ///   being described which is inferred from the module portion of the core
    ///   wasm import field. This is `None` for root-level function/type
    ///   imports, such as when referring to `import x: func();`. This is `Some`
    ///   when an interface is used (either `import x: interface { .. }` or a
    ///   standalone `interface`) where the world key is specified for the
    ///   interface in addition to the interface that was identified.
    /// * `encoder` - this is the encoder state that contains
    ///   `Resolve`/metadata information.
    /// * `ty` - the core wasm type of this import.
    /// * `import` - whether or not this core wasm import is operating on a WIT
    ///   level import or export. An example of this being an export is when a
    ///   core module imports a destructor for an exported resource.
    /// * `names` - the name mangling scheme that's configured to be used.
    fn maybe_classify_wit_intrinsic(
        &self,
        name: &str,
        key_and_id: Option<(WorldKey, InterfaceId)>,
        encoder: &ComponentEncoder,
        ty: &FuncType,
        import: bool,
        names: &dyn NameMangling,
    ) -> Result<Option<Import>> {
        let resolve = &encoder.metadata.resolve;
        let world_id = encoder.metadata.world;
        let world = &resolve.worlds[world_id];

        // Separate out `Option<WorldKey>` and `Option<InterfaceId>`. If an
        // interface is NOT specified then the `WorldKey` which is attached to
        // imports is going to be calculated based on the name of the item
        // extracted, such as the resource or function referenced.
        let (key, id) = match key_and_id {
            Some((key, id)) => (Some(key), Some(id)),
            None => (None, None),
        };

        // Tests whether `name` is a resource within `id` (or `world_id`).
        let resource_test = |name: &str| match id {
            Some(id) => resource_test_for_interface(resolve, id)(name),
            None => resource_test_for_world(resolve, world_id)(name),
        };

        // Test whether this is a `resource.drop` intrinsic.
        if let Some(resource) = names.resource_drop_name(name) {
            if let Some(resource_id) = resource_test(resource) {
                let key = key.unwrap_or_else(|| WorldKey::Name(resource.to_string()));
                let expected = FuncType::new([ValType::I32], []);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Some(if import {
                    Import::ImportedResourceDrop(key, id, resource_id)
                } else {
                    Import::ExportedResourceDrop(key, resource_id)
                }));
            }
        }

        // There are some intrinsics which are only applicable to exported
        // functions/resources, so check those use cases here.
        if !import {
            if let Some(name) = names.resource_new_name(name) {
                if let Some(id) = resource_test(name) {
                    let key = key.unwrap_or_else(|| WorldKey::Name(name.to_string()));
                    let expected = FuncType::new([ValType::I32], [ValType::I32]);
                    validate_func_sig(name, &expected, ty)?;
                    return Ok(Some(Import::ExportedResourceNew(key, id)));
                }
            }
            if let Some(name) = names.resource_rep_name(name) {
                if let Some(id) = resource_test(name) {
                    let key = key.unwrap_or_else(|| WorldKey::Name(name.to_string()));
                    let expected = FuncType::new([ValType::I32], [ValType::I32]);
                    validate_func_sig(name, &expected, ty)?;
                    return Ok(Some(Import::ExportedResourceRep(key, id)));
                }
            }
            if let Some(name) = names.task_return_name(name) {
                let func = get_function(resolve, world, name, id, import)?;
                let key = key.unwrap_or_else(|| WorldKey::Name(name.to_string()));
                // TODO: should call `validate_func_sig` but would require
                // calculating the expected signature based of `func.result`.
                return Ok(Some(Import::ExportedTaskReturn(
                    key,
                    id,
                    func.name.clone(),
                    func.result,
                )));
            }
            if names.task_cancel(name) {
                let expected = FuncType::new([], []);
                validate_func_sig(name, &expected, ty)?;
                return Ok(Some(Import::ExportedTaskCancel));
            }
        }

        let lookup_context = PayloadLookupContext {
            resolve,
            world,
            key,
            id,
            import,
        };

        // Test for a number of async-related intrinsics. All intrinsics are
        // prefixed with `[...-N]` where `...` is the name of the intrinsic and
        // the `N` is the indexed future/stream that is being referred to.
        let import = if let Some(info) = names.future_new(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([], [ValType::I64]), ty)?;
            Import::FutureNew(info)
        } else if let Some(info) = names.future_write(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32; 2], [ValType::I32]), ty)?;
            Import::FutureWrite {
                async_: info.async_lowered,
                info: info.inner,
            }
        } else if let Some(info) = names.future_read(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32; 2], [ValType::I32]), ty)?;
            Import::FutureRead {
                async_: info.async_lowered,
                info: info.inner,
            }
        } else if let Some(info) = names.future_cancel_write(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32], [ValType::I32]), ty)?;
            Import::FutureCancelWrite {
                async_: info.async_lowered,
                info: info.inner,
            }
        } else if let Some(info) = names.future_cancel_read(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32], [ValType::I32]), ty)?;
            Import::FutureCancelRead {
                async_: info.async_lowered,
                info: info.inner,
            }
        } else if let Some(info) = names.future_drop_writable(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32], []), ty)?;
            Import::FutureDropWritable(info)
        } else if let Some(info) = names.future_drop_readable(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32], []), ty)?;
            Import::FutureDropReadable(info)
        } else if let Some(info) = names.stream_new(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([], [ValType::I64]), ty)?;
            Import::StreamNew(info)
        } else if let Some(info) = names.stream_write(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32; 3], [ValType::I32]), ty)?;
            Import::StreamWrite {
                async_: info.async_lowered,
                info: info.inner,
            }
        } else if let Some(info) = names.stream_read(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32; 3], [ValType::I32]), ty)?;
            Import::StreamRead {
                async_: info.async_lowered,
                info: info.inner,
            }
        } else if let Some(info) = names.stream_cancel_write(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32], [ValType::I32]), ty)?;
            Import::StreamCancelWrite {
                async_: info.async_lowered,
                info: info.inner,
            }
        } else if let Some(info) = names.stream_cancel_read(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32], [ValType::I32]), ty)?;
            Import::StreamCancelRead {
                async_: info.async_lowered,
                info: info.inner,
            }
        } else if let Some(info) = names.stream_drop_writable(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32], []), ty)?;
            Import::StreamDropWritable(info)
        } else if let Some(info) = names.stream_drop_readable(&lookup_context, name) {
            validate_func_sig(name, &FuncType::new([ValType::I32], []), ty)?;
            Import::StreamDropReadable(info)
        } else {
            return Ok(None);
        };
        Ok(Some(import))
    }

    fn classify_import_with_library(
        &mut self,
        import: wasmparser::Import<'_>,
        library_info: Option<&LibraryInfo>,
    ) -> Result<bool> {
        let info = match library_info {
            Some(info) => info,
            None => return Ok(false),
        };
        let Some((_, instance)) = info
            .arguments
            .iter()
            .find(|(name, _items)| *name == import.module)
        else {
            return Ok(false);
        };
        match instance {
            Instance::MainOrAdapter(module) => match self.names.get(import.module) {
                Some(ImportInstance::Whole(which)) => {
                    if which != module {
                        bail!("different whole modules imported under the same name");
                    }
                }
                Some(ImportInstance::Names(_)) => {
                    bail!("cannot mix individual imports and whole module imports")
                }
                None => {
                    let instance = ImportInstance::Whole(module.clone());
                    self.names.insert(import.module.to_string(), instance);
                }
            },
            Instance::Items(items) => {
                let Some(item) = items.iter().find(|i| i.alias == import.name) else {
                    return Ok(false);
                };
                self.insert_import(import, Import::Item(item.clone()))?;
            }
        }
        Ok(true)
    }

    /// Map an imported item, by module and field name in `self.names`, to the
    /// kind of `Import` it is: for example, a certain-typed function from an
    /// adapter.
    fn insert_import(&mut self, import: wasmparser::Import<'_>, item: Import) -> Result<()> {
        let entry = self
            .names
            .entry(import.module.to_string())
            .or_insert(ImportInstance::Names(IndexMap::default()));
        let names = match entry {
            ImportInstance::Names(names) => names,
            _ => bail!("cannot mix individual imports with module imports"),
        };
        let entry = match names.entry(import.name.to_string()) {
            Entry::Occupied(_) => {
                bail!(
                    "module has duplicate import for `{}::{}`",
                    import.module,
                    import.name
                );
            }
            Entry::Vacant(v) => v,
        };
        log::trace!(
            "classifying import `{}::{} as {item:?}",
            import.module,
            import.name
        );
        entry.insert(item);
        Ok(())
    }
}

/// Dual of `ImportMap` except describes the exports of a module instead of the
/// imports.
#[derive(Default)]
pub struct ExportMap {
    names: IndexMap<String, Export>,
    raw_exports: IndexMap<String, FuncType>,
}

/// All possible (known) exports from a core wasm module that are recognized and
/// handled during the componentization process.
#[derive(Debug)]
pub enum Export {
    /// An export of a top-level function of a world, where the world function
    /// is named here.
    WorldFunc(WorldKey, String, AbiVariant),

    /// A post-return for a top-level function of a world.
    WorldFuncPostReturn(WorldKey),

    /// An export of a function in an interface.
    InterfaceFunc(WorldKey, InterfaceId, String, AbiVariant),

    /// A post-return for the above function.
    InterfaceFuncPostReturn(WorldKey, String),

    /// A destructor for an exported resource.
    ResourceDtor(TypeId),

    /// Memory, typically for an adapter.
    Memory,

    /// `cabi_realloc`
    GeneralPurposeRealloc,

    /// `cabi_export_realloc`
    GeneralPurposeExportRealloc,

    /// `cabi_import_realloc`
    GeneralPurposeImportRealloc,

    /// `_initialize`
    Initialize,

    /// `cabi_realloc_adapter`
    ReallocForAdapter,

    WorldFuncCallback(WorldKey),

    InterfaceFuncCallback(WorldKey, String),

    /// __indirect_function_table, used for `thread.new-indirect`
    IndirectFunctionTable,
}

impl ExportMap {
    fn add(
        &mut self,
        export: wasmparser::Export<'_>,
        encoder: &ComponentEncoder,
        exports: &IndexSet<WorldKey>,
        types: TypesRef<'_>,
    ) -> Result<()> {
        if let Some(item) = self.classify(export, encoder, exports, types)? {
            log::debug!("classifying export `{}` as {item:?}", export.name);
            let prev = self.names.insert(export.name.to_string(), item);
            assert!(prev.is_none());
        }
        Ok(())
    }

    fn classify(
        &mut self,
        export: wasmparser::Export<'_>,
        encoder: &ComponentEncoder,
        exports: &IndexSet<WorldKey>,
        types: TypesRef<'_>,
    ) -> Result<Option<Export>> {
        match export.kind {
            ExternalKind::Func => {
                let ty = types[types.core_function_at(export.index)].unwrap_func();
                self.raw_exports.insert(export.name.to_string(), ty.clone());
            }
            _ => {}
        }

        // Handle a few special-cased names first.
        if export.name == "canonical_abi_realloc" {
            return Ok(Some(Export::GeneralPurposeRealloc));
        } else if export.name == "cabi_import_realloc" {
            return Ok(Some(Export::GeneralPurposeImportRealloc));
        } else if export.name == "cabi_export_realloc" {
            return Ok(Some(Export::GeneralPurposeExportRealloc));
        } else if export.name == "cabi_realloc_adapter" {
            return Ok(Some(Export::ReallocForAdapter));
        }

        let (name, names) = match export.name.strip_prefix("cm32p2") {
            Some(name) => (name, STANDARD),
            None if encoder.reject_legacy_names => return Ok(None),
            None => (export.name, LEGACY),
        };

        if let Some(export) = self
            .classify_component_export(names, name, &export, encoder, exports, types)
            .with_context(|| format!("failed to classify export `{}`", export.name))?
        {
            return Ok(Some(export));
        }
        log::debug!("unknown export `{}`", export.name);
        Ok(None)
    }

    fn classify_component_export(
        &mut self,
        names: &dyn NameMangling,
        name: &str,
        export: &wasmparser::Export<'_>,
        encoder: &ComponentEncoder,
        exports: &IndexSet<WorldKey>,
        types: TypesRef<'_>,
    ) -> Result<Option<Export>> {
        let resolve = &encoder.metadata.resolve;
        let world = encoder.metadata.world;
        match export.kind {
            ExternalKind::Func => {}
            ExternalKind::Memory => {
                if name == names.export_memory() {
                    return Ok(Some(Export::Memory));
                }
                return Ok(None);
            }
            ExternalKind::Table => {
                if Some(name) == names.export_indirect_function_table() {
                    return Ok(Some(Export::IndirectFunctionTable));
                }
                return Ok(None);
            }
            _ => return Ok(None),
        }
        let ty = types[types.core_function_at(export.index)].unwrap_func();

        // Handle a few special-cased names first.
        if name == names.export_realloc() {
            let expected = FuncType::new([ValType::I32; 4], [ValType::I32]);
            validate_func_sig(name, &expected, ty)?;
            return Ok(Some(Export::GeneralPurposeRealloc));
        } else if name == names.export_initialize() {
            let expected = FuncType::new([], []);
            validate_func_sig(name, &expected, ty)?;
            return Ok(Some(Export::Initialize));
        }

        let full_name = name;
        let (abi, name) = if let Some(name) = names.async_lift_name(name) {
            (AbiVariant::GuestExportAsync, name)
        } else if let Some(name) = names.async_lift_stackful_name(name) {
            (AbiVariant::GuestExportAsyncStackful, name)
        } else {
            (AbiVariant::GuestExport, name)
        };

        // Try to match this to a known WIT export that `exports` allows.
        if let Some((key, id, f)) = names.match_wit_export(name, resolve, world, exports) {
            validate_func(resolve, ty, f, abi).with_context(|| {
                let key = resolve.name_world_key(key);
                format!("failed to validate export for `{key}`")
            })?;
            match id {
                Some(id) => {
                    return Ok(Some(Export::InterfaceFunc(
                        key.clone(),
                        id,
                        f.name.clone(),
                        abi,
                    )));
                }
                None => {
                    return Ok(Some(Export::WorldFunc(key.clone(), f.name.clone(), abi)));
                }
            }
        }

        // See if this is a post-return for any known WIT export.
        if let Some(remaining) = names.strip_post_return(name) {
            if let Some((key, id, f)) = names.match_wit_export(remaining, resolve, world, exports) {
                validate_post_return(resolve, ty, f).with_context(|| {
                    let key = resolve.name_world_key(key);
                    format!("failed to validate export for `{key}`")
                })?;
                match id {
                    Some(_id) => {
                        return Ok(Some(Export::InterfaceFuncPostReturn(
                            key.clone(),
                            f.name.clone(),
                        )));
                    }
                    None => {
                        return Ok(Some(Export::WorldFuncPostReturn(key.clone())));
                    }
                }
            }
        }

        if let Some(suffix) = names.async_lift_callback_name(full_name) {
            if let Some((key, id, f)) = names.match_wit_export(suffix, resolve, world, exports) {
                validate_func_sig(
                    full_name,
                    &FuncType::new([ValType::I32; 3], [ValType::I32]),
                    ty,
                )?;
                return Ok(Some(if id.is_some() {
                    Export::InterfaceFuncCallback(key.clone(), f.name.clone())
                } else {
                    Export::WorldFuncCallback(key.clone())
                }));
            }
        }

        // And, finally, see if it matches a known destructor.
        if let Some(dtor) = names.match_wit_resource_dtor(name, resolve, world, exports) {
            let expected = FuncType::new([ValType::I32], []);
            validate_func_sig(full_name, &expected, ty)?;
            return Ok(Some(Export::ResourceDtor(dtor)));
        }

        Ok(None)
    }

    /// Returns the name of the post-return export, if any, for the `key` and
    /// `func` combo.
    pub fn post_return(&self, key: &WorldKey, func: &Function) -> Option<&str> {
        self.find(|m| match m {
            Export::WorldFuncPostReturn(k) => k == key,
            Export::InterfaceFuncPostReturn(k, f) => k == key && func.name == *f,
            _ => false,
        })
    }

    /// Returns the name of the async callback export, if any, for the `key` and
    /// `func` combo.
    pub fn callback(&self, key: &WorldKey, func: &Function) -> Option<&str> {
        self.find(|m| match m {
            Export::WorldFuncCallback(k) => k == key,
            Export::InterfaceFuncCallback(k, f) => k == key && func.name == *f,
            _ => false,
        })
    }

    pub fn abi(&self, key: &WorldKey, func: &Function) -> Option<AbiVariant> {
        self.names.values().find_map(|m| match m {
            Export::WorldFunc(k, f, abi) if k == key && func.name == *f => Some(*abi),
            Export::InterfaceFunc(k, _, f, abi) if k == key && func.name == *f => Some(*abi),
            _ => None,
        })
    }

    /// Returns the realloc that the exported function `interface` and `func`
    /// are using.
    pub fn export_realloc_for(&self, key: &WorldKey, func: &str) -> Option<&str> {
        // TODO: This realloc detection should probably be improved with
        // some sort of scheme to have per-function reallocs like
        // `cabi_realloc_{name}` or something like that.
        let _ = (key, func);

        if let Some(name) = self.find(|m| matches!(m, Export::GeneralPurposeExportRealloc)) {
            return Some(name);
        }
        self.general_purpose_realloc()
    }

    /// Returns the realloc that the imported function `interface` and `func`
    /// are using.
    pub fn import_realloc_for(&self, interface: Option<InterfaceId>, func: &str) -> Option<&str> {
        // TODO: This realloc detection should probably be improved with
        // some sort of scheme to have per-function reallocs like
        // `cabi_realloc_{name}` or something like that.
        let _ = (interface, func);

        self.import_realloc_fallback()
    }

    /// Returns the general-purpose realloc function to use for imports.
    ///
    /// Note that `import_realloc_for` should be used instead where possible.
    pub fn import_realloc_fallback(&self) -> Option<&str> {
        if let Some(name) = self.find(|m| matches!(m, Export::GeneralPurposeImportRealloc)) {
            return Some(name);
        }
        self.general_purpose_realloc()
    }

    /// Returns the realloc that the main module is exporting into the adapter.
    pub fn realloc_to_import_into_adapter(&self) -> Option<&str> {
        if let Some(name) = self.find(|m| matches!(m, Export::ReallocForAdapter)) {
            return Some(name);
        }
        self.general_purpose_realloc()
    }

    fn general_purpose_realloc(&self) -> Option<&str> {
        self.find(|m| matches!(m, Export::GeneralPurposeRealloc))
    }

    /// Returns the memory, if exported, for this module.
    pub fn memory(&self) -> Option<&str> {
        self.find(|m| matches!(m, Export::Memory))
    }

    /// Returns the indirect function table, if exported, for this module.
    pub fn indirect_function_table(&self) -> Option<&str> {
        self.find(|t| matches!(t, Export::IndirectFunctionTable))
    }

    /// Returns the `_initialize` intrinsic, if exported, for this module.
    pub fn initialize(&self) -> Option<&str> {
        self.find(|m| matches!(m, Export::Initialize))
    }

    /// Returns destructor for the exported resource `ty`, if it was listed.
    pub fn resource_dtor(&self, ty: TypeId) -> Option<&str> {
        self.find(|m| match m {
            Export::ResourceDtor(t) => *t == ty,
            _ => false,
        })
    }

    /// NB: this is a linear search and if that's ever a problem this should
    /// build up an inverse map during construction to accelerate it.
    fn find(&self, f: impl Fn(&Export) -> bool) -> Option<&str> {
        let (name, _) = self.names.iter().filter(|(_, m)| f(m)).next()?;
        Some(name)
    }

    /// Iterates over all exports of this module.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Export)> + '_ {
        self.names.iter().map(|(n, e)| (n.as_str(), e))
    }

    fn validate(&self, encoder: &ComponentEncoder, exports: &IndexSet<WorldKey>) -> Result<()> {
        let resolve = &encoder.metadata.resolve;
        let world = encoder.metadata.world;
        // Multi-memory isn't supported because otherwise we don't know what
        // memory to put things in.
        if self
            .names
            .values()
            .filter(|m| matches!(m, Export::Memory))
            .count()
            > 1
        {
            bail!("cannot componentize module that exports multiple memories")
        }

        // Every async-with-callback-lifted export must have a callback.
        for (name, export) in &self.names {
            match export {
                Export::WorldFunc(_, _, AbiVariant::GuestExportAsync) => {
                    if !matches!(
                        self.names.get(&format!("[callback]{name}")),
                        Some(Export::WorldFuncCallback(_))
                    ) {
                        bail!("missing callback for `{name}`");
                    }
                }
                Export::InterfaceFunc(_, _, _, AbiVariant::GuestExportAsync) => {
                    if !matches!(
                        self.names.get(&format!("[callback]{name}")),
                        Some(Export::InterfaceFuncCallback(_, _))
                    ) {
                        bail!("missing callback for `{name}`");
                    }
                }
                _ => {}
            }
        }

        // All of `exports` must be exported and found within this module.
        for export in exports {
            let require_interface_func = |interface: InterfaceId, name: &str| -> Result<()> {
                let result = self.find(|e| match e {
                    Export::InterfaceFunc(_, id, s, _) => interface == *id && name == s,
                    _ => false,
                });
                if result.is_some() {
                    Ok(())
                } else {
                    let export = resolve.name_world_key(export);
                    bail!("failed to find export of interface `{export}` function `{name}`")
                }
            };
            let require_world_func = |name: &str| -> Result<()> {
                let result = self.find(|e| match e {
                    Export::WorldFunc(_, s, _) => name == s,
                    _ => false,
                });
                if result.is_some() {
                    Ok(())
                } else {
                    bail!("failed to find export of function `{name}`")
                }
            };
            match &resolve.worlds[world].exports[export] {
                WorldItem::Interface { id, .. } => {
                    for (name, _) in resolve.interfaces[*id].functions.iter() {
                        require_interface_func(*id, name)?;
                    }
                }
                WorldItem::Function(f) => {
                    require_world_func(&f.name)?;
                }
                WorldItem::Type(_) => unreachable!(),
            }
        }

        Ok(())
    }
}

/// A builtin that may be declared as cancellable.
struct MaybeCancellable<T> {
    #[allow(unused)]
    inner: T,
    cancellable: bool,
}

/// A builtin that may be declared as async-lowered.
struct MaybeAsyncLowered<T> {
    inner: T,
    async_lowered: bool,
}

/// Context passed to `NameMangling` implementations of stream and future functions
/// to help with looking up payload information.
struct PayloadLookupContext<'a> {
    resolve: &'a Resolve,
    world: &'a World,
    id: Option<InterfaceId>,
    import: bool,
    key: Option<WorldKey>,
}

/// Trait dispatch and definition for parsing and interpreting "mangled names"
/// which show up in imports and exports of the component model.
///
/// This trait is used to implement classification of imports and exports in the
/// component model. The methods on `ImportMap` and `ExportMap` will use this to
/// determine what an import is and how it's lifted/lowered in the world being
/// bound.
///
/// This trait has a bit of history behind it as well. Before
/// WebAssembly/component-model#378 there was no standard naming scheme for core
/// wasm imports or exports when componenitizing. This meant that
/// `wit-component` implemented a particular scheme which mostly worked but was
/// mostly along the lines of "this at least works" rather than "someone sat
/// down and designed this". Since then, however, an standard naming scheme has
/// now been specified which was indeed designed.
///
/// This trait serves as the bridge between these two. The historical naming
/// scheme is still supported for now through the `Legacy` implementation below
/// and will be for some time. The transition plan at this time is to support
/// the new scheme, eventually get it supported in bindings generators, and once
/// that's all propagated remove support for the legacy scheme.
trait NameMangling {
    fn import_root(&self) -> &str;
    fn import_non_root_prefix(&self) -> &str;
    fn import_exported_intrinsic_prefix(&self) -> &str;
    fn export_memory(&self) -> &str;
    fn export_initialize(&self) -> &str;
    fn export_realloc(&self) -> &str;
    fn export_indirect_function_table(&self) -> Option<&str>;
    fn resource_drop_name<'a>(&self, name: &'a str) -> Option<&'a str>;
    fn resource_new_name<'a>(&self, name: &'a str) -> Option<&'a str>;
    fn resource_rep_name<'a>(&self, name: &'a str) -> Option<&'a str>;
    fn task_return_name<'a>(&self, name: &'a str) -> Option<&'a str>;
    fn task_cancel(&self, name: &str) -> bool;
    fn backpressure_inc(&self, name: &str) -> bool;
    fn backpressure_dec(&self, name: &str) -> bool;
    fn waitable_set_new(&self, name: &str) -> bool;
    fn waitable_set_wait(&self, name: &str) -> Option<MaybeCancellable<()>>;
    fn waitable_set_poll(&self, name: &str) -> Option<MaybeCancellable<()>>;
    fn waitable_set_drop(&self, name: &str) -> bool;
    fn waitable_join(&self, name: &str) -> bool;
    fn thread_yield(&self, name: &str) -> Option<MaybeCancellable<()>>;
    fn subtask_drop(&self, name: &str) -> bool;
    fn subtask_cancel(&self, name: &str) -> Option<MaybeAsyncLowered<()>>;
    fn async_lift_callback_name<'a>(&self, name: &'a str) -> Option<&'a str>;
    fn async_lift_name<'a>(&self, name: &'a str) -> Option<&'a str>;
    fn async_lift_stackful_name<'a>(&self, name: &'a str) -> Option<&'a str>;
    fn error_context_new(&self, name: &str) -> Option<StringEncoding>;
    fn error_context_debug_message(&self, name: &str) -> Option<StringEncoding>;
    fn error_context_drop(&self, name: &str) -> bool;
    fn context_get(&self, name: &str) -> Option<u32>;
    fn context_set(&self, name: &str) -> Option<u32>;
    fn future_new(&self, lookup_context: &PayloadLookupContext, name: &str) -> Option<PayloadInfo>;
    fn future_write(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>>;
    fn future_read(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>>;
    fn future_cancel_write(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>>;
    fn future_cancel_read(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>>;
    fn future_drop_writable(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<PayloadInfo>;
    fn future_drop_readable(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<PayloadInfo>;
    fn stream_new(&self, lookup_context: &PayloadLookupContext, name: &str) -> Option<PayloadInfo>;
    fn stream_write(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>>;
    fn stream_read(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>>;
    fn stream_cancel_write(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>>;
    fn stream_cancel_read(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>>;
    fn stream_drop_writable(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<PayloadInfo>;
    fn stream_drop_readable(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<PayloadInfo>;
    fn thread_index(&self, name: &str) -> bool;
    fn thread_new_indirect(&self, name: &str) -> bool;
    fn thread_switch_to(&self, name: &str) -> Option<MaybeCancellable<()>>;
    fn thread_suspend(&self, name: &str) -> Option<MaybeCancellable<()>>;
    fn thread_resume_later(&self, name: &str) -> bool;
    fn thread_yield_to(&self, name: &str) -> Option<MaybeCancellable<()>>;
    fn module_to_interface(
        &self,
        module: &str,
        resolve: &Resolve,
        items: &IndexMap<WorldKey, WorldItem>,
    ) -> Result<(WorldKey, InterfaceId)>;
    fn strip_post_return<'a>(&self, name: &'a str) -> Option<&'a str>;
    fn match_wit_export<'a>(
        &self,
        export_name: &str,
        resolve: &'a Resolve,
        world: WorldId,
        exports: &'a IndexSet<WorldKey>,
    ) -> Option<(&'a WorldKey, Option<InterfaceId>, &'a Function)>;
    fn match_wit_resource_dtor<'a>(
        &self,
        export_name: &str,
        resolve: &'a Resolve,
        world: WorldId,
        exports: &'a IndexSet<WorldKey>,
    ) -> Option<TypeId>;
    fn world_key_name_and_abi<'a>(&self, name: &'a str) -> (&'a str, AbiVariant);
    fn interface_function_name_and_abi<'a>(&self, name: &'a str) -> (&'a str, AbiVariant);
}

/// Definition of the "standard" naming scheme which currently starts with
/// "cm32p2". Note that wasm64 is not supported at this time.
struct Standard;

const STANDARD: &'static dyn NameMangling = &Standard;

impl NameMangling for Standard {
    fn import_root(&self) -> &str {
        ""
    }
    fn import_non_root_prefix(&self) -> &str {
        "|"
    }
    fn import_exported_intrinsic_prefix(&self) -> &str {
        "_ex_"
    }
    fn export_memory(&self) -> &str {
        "_memory"
    }
    fn export_initialize(&self) -> &str {
        "_initialize"
    }
    fn export_realloc(&self) -> &str {
        "_realloc"
    }
    fn export_indirect_function_table(&self) -> Option<&str> {
        None
    }
    fn resource_drop_name<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_suffix("_drop")
    }
    fn resource_new_name<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_suffix("_new")
    }
    fn resource_rep_name<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_suffix("_rep")
    }
    fn task_return_name<'a>(&self, _name: &'a str) -> Option<&'a str> {
        None
    }
    fn task_cancel(&self, _name: &str) -> bool {
        false
    }
    fn backpressure_inc(&self, _name: &str) -> bool {
        false
    }
    fn backpressure_dec(&self, _name: &str) -> bool {
        false
    }
    fn waitable_set_new(&self, _name: &str) -> bool {
        false
    }
    fn waitable_set_wait(&self, _name: &str) -> Option<MaybeCancellable<()>> {
        None
    }
    fn waitable_set_poll(&self, _name: &str) -> Option<MaybeCancellable<()>> {
        None
    }
    fn waitable_set_drop(&self, _name: &str) -> bool {
        false
    }
    fn waitable_join(&self, _name: &str) -> bool {
        false
    }
    fn thread_yield(&self, _name: &str) -> Option<MaybeCancellable<()>> {
        None
    }
    fn subtask_drop(&self, _name: &str) -> bool {
        false
    }
    fn subtask_cancel(&self, _name: &str) -> Option<MaybeAsyncLowered<()>> {
        None
    }
    fn async_lift_callback_name<'a>(&self, _name: &'a str) -> Option<&'a str> {
        None
    }
    fn async_lift_name<'a>(&self, _name: &'a str) -> Option<&'a str> {
        None
    }
    fn async_lift_stackful_name<'a>(&self, _name: &'a str) -> Option<&'a str> {
        None
    }
    fn error_context_new(&self, _name: &str) -> Option<StringEncoding> {
        None
    }
    fn error_context_debug_message(&self, _name: &str) -> Option<StringEncoding> {
        None
    }
    fn error_context_drop(&self, _name: &str) -> bool {
        false
    }
    fn context_get(&self, _name: &str) -> Option<u32> {
        None
    }
    fn context_set(&self, _name: &str) -> Option<u32> {
        None
    }
    fn thread_index(&self, _name: &str) -> bool {
        false
    }
    fn thread_new_indirect(&self, _name: &str) -> bool {
        false
    }
    fn thread_switch_to(&self, _name: &str) -> Option<MaybeCancellable<()>> {
        None
    }
    fn thread_suspend(&self, _name: &str) -> Option<MaybeCancellable<()>> {
        None
    }
    fn thread_resume_later(&self, _name: &str) -> bool {
        false
    }
    fn thread_yield_to(&self, _name: &str) -> Option<MaybeCancellable<()>> {
        None
    }
    fn future_new(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<PayloadInfo> {
        None
    }
    fn future_write(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        None
    }
    fn future_read(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        None
    }
    fn future_cancel_write(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        None
    }
    fn future_cancel_read(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        None
    }
    fn future_drop_writable(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<PayloadInfo> {
        None
    }
    fn future_drop_readable(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<PayloadInfo> {
        None
    }
    fn stream_new(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<PayloadInfo> {
        None
    }
    fn stream_write(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        None
    }
    fn stream_read(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        None
    }
    fn stream_cancel_write(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        None
    }
    fn stream_cancel_read(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        None
    }
    fn stream_drop_writable(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<PayloadInfo> {
        None
    }
    fn stream_drop_readable(
        &self,
        _lookup_context: &PayloadLookupContext,
        _name: &str,
    ) -> Option<PayloadInfo> {
        None
    }
    fn module_to_interface(
        &self,
        interface: &str,
        resolve: &Resolve,
        items: &IndexMap<WorldKey, WorldItem>,
    ) -> Result<(WorldKey, InterfaceId)> {
        for (key, item) in items.iter() {
            let id = match key {
                // Bare keys are matched exactly against `interface`
                WorldKey::Name(name) => match item {
                    WorldItem::Interface { id, .. } if name == interface => *id,
                    _ => continue,
                },
                // ID-identified keys are matched with their "canonical name"
                WorldKey::Interface(id) => {
                    if resolve.canonicalized_id_of(*id).as_deref() != Some(interface) {
                        continue;
                    }
                    *id
                }
            };
            return Ok((key.clone(), id));
        }
        bail!("failed to find world item corresponding to interface `{interface}`")
    }
    fn strip_post_return<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_suffix("_post")
    }
    fn match_wit_export<'a>(
        &self,
        export_name: &str,
        resolve: &'a Resolve,
        world: WorldId,
        exports: &'a IndexSet<WorldKey>,
    ) -> Option<(&'a WorldKey, Option<InterfaceId>, &'a Function)> {
        if let Some(world_export_name) = export_name.strip_prefix("||") {
            let key = exports.get(&WorldKey::Name(world_export_name.to_string()))?;
            match &resolve.worlds[world].exports[key] {
                WorldItem::Function(f) => return Some((key, None, f)),
                _ => return None,
            }
        }

        let (key, id, func_name) =
            self.match_wit_interface(export_name, resolve, world, exports)?;
        let func = resolve.interfaces[id].functions.get(func_name)?;
        Some((key, Some(id), func))
    }

    fn match_wit_resource_dtor<'a>(
        &self,
        export_name: &str,
        resolve: &'a Resolve,
        world: WorldId,
        exports: &'a IndexSet<WorldKey>,
    ) -> Option<TypeId> {
        let (_key, id, name) =
            self.match_wit_interface(export_name.strip_suffix("_dtor")?, resolve, world, exports)?;
        let ty = *resolve.interfaces[id].types.get(name)?;
        match resolve.types[ty].kind {
            TypeDefKind::Resource => Some(ty),
            _ => None,
        }
    }

    fn world_key_name_and_abi<'a>(&self, name: &'a str) -> (&'a str, AbiVariant) {
        (name, AbiVariant::GuestImport)
    }
    fn interface_function_name_and_abi<'a>(&self, name: &'a str) -> (&'a str, AbiVariant) {
        (name, AbiVariant::GuestImport)
    }
}

impl Standard {
    fn match_wit_interface<'a, 'b>(
        &self,
        export_name: &'b str,
        resolve: &'a Resolve,
        world: WorldId,
        exports: &'a IndexSet<WorldKey>,
    ) -> Option<(&'a WorldKey, InterfaceId, &'b str)> {
        let world = &resolve.worlds[world];
        let export_name = export_name.strip_prefix("|")?;

        for export in exports {
            let id = match &world.exports[export] {
                WorldItem::Interface { id, .. } => *id,
                WorldItem::Function(_) => continue,
                WorldItem::Type(_) => unreachable!(),
            };
            let remaining = match export {
                WorldKey::Name(name) => export_name.strip_prefix(name),
                WorldKey::Interface(_) => {
                    let prefix = resolve.canonicalized_id_of(id).unwrap();
                    export_name.strip_prefix(&prefix)
                }
            };
            let item_name = match remaining.and_then(|s| s.strip_prefix("|")) {
                Some(name) => name,
                None => continue,
            };
            return Some((export, id, item_name));
        }

        None
    }
}

/// Definition of wit-component's "legacy" naming scheme which predates
/// WebAssembly/component-model#378.
struct Legacy;

const LEGACY: &'static dyn NameMangling = &Legacy;

impl Legacy {
    // Looks for `[$prefix-N]foo` within `name`. If found then `foo` is
    // used to find a function within `id` and `world` above. Once found
    // then `N` is used to index within that function to extract a
    // future/stream type. If that's all found then a `PayloadInfo` is
    // returned to get attached to an intrinsic.
    fn prefixed_payload(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
        prefix: &str,
    ) -> Option<PayloadInfo> {
        // parse the `prefix` into `func_name` and `type_index`, bailing out
        // with `None` if anything doesn't match.
        let (index_or_unit, func_name) = prefixed_intrinsic(name, prefix)?;
        let ty = match index_or_unit {
            "unit" => {
                if name.starts_with("[future") {
                    PayloadType::UnitFuture
                } else if name.starts_with("[stream") {
                    PayloadType::UnitStream
                } else {
                    unreachable!()
                }
            }
            other => {
                // Note that this is parsed as a `u32` to ensure that the
                // integer parsing is the same across platforms regardless of
                // the the width of `usize`.
                let type_index = other.parse::<u32>().ok()? as usize;

                // Double-check that `func_name` is indeed a function name within
                // this interface/world. Then additionally double-check that
                // `type_index` is indeed a valid index for this function's type
                // signature.
                let function = get_function(
                    lookup_context.resolve,
                    lookup_context.world,
                    func_name,
                    lookup_context.id,
                    lookup_context.import,
                )
                .ok()?;
                PayloadType::Type {
                    id: *function
                        .find_futures_and_streams(lookup_context.resolve)
                        .get(type_index)?,
                    function: function.name.clone(),
                }
            }
        };

        // And if all that passes wrap up everything in a `PayloadInfo`.
        Some(PayloadInfo {
            name: name.to_string(),
            ty,
            key: lookup_context
                .key
                .clone()
                .unwrap_or_else(|| WorldKey::Name(name.to_string())),
            interface: lookup_context.id,
            imported: lookup_context.import,
        })
    }

    fn maybe_async_lowered_payload(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
        prefix: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        let (async_lowered, clean_name) = self.strip_async_lowered_prefix(name);
        let payload = self.prefixed_payload(lookup_context, clean_name, prefix)?;
        Some(MaybeAsyncLowered {
            inner: payload,
            async_lowered,
        })
    }

    fn strip_async_lowered_prefix<'a>(&self, name: &'a str) -> (bool, &'a str) {
        name.strip_prefix("[async-lower]")
            .map_or((false, name), |s| (true, s))
    }
    fn match_with_async_lowered_prefix(
        &self,
        name: &str,
        expected: &str,
    ) -> Option<MaybeAsyncLowered<()>> {
        let (async_lowered, clean_name) = self.strip_async_lowered_prefix(name);
        if clean_name == expected {
            Some(MaybeAsyncLowered {
                inner: (),
                async_lowered,
            })
        } else {
            None
        }
    }
    fn strip_cancellable_prefix<'a>(&self, name: &'a str) -> (bool, &'a str) {
        name.strip_prefix("[cancellable]")
            .map_or((false, name), |s| (true, s))
    }
    fn match_with_cancellable_prefix(
        &self,
        name: &str,
        expected: &str,
    ) -> Option<MaybeCancellable<()>> {
        let (cancellable, clean_name) = self.strip_cancellable_prefix(name);
        if clean_name == expected {
            Some(MaybeCancellable {
                inner: (),
                cancellable,
            })
        } else {
            None
        }
    }
}

impl NameMangling for Legacy {
    fn import_root(&self) -> &str {
        "$root"
    }
    fn import_non_root_prefix(&self) -> &str {
        ""
    }
    fn import_exported_intrinsic_prefix(&self) -> &str {
        "[export]"
    }
    fn export_memory(&self) -> &str {
        "memory"
    }
    fn export_initialize(&self) -> &str {
        "_initialize"
    }
    fn export_realloc(&self) -> &str {
        "cabi_realloc"
    }
    fn export_indirect_function_table(&self) -> Option<&str> {
        Some("__indirect_function_table")
    }
    fn resource_drop_name<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_prefix("[resource-drop]")
    }
    fn resource_new_name<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_prefix("[resource-new]")
    }
    fn resource_rep_name<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_prefix("[resource-rep]")
    }
    fn task_return_name<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_prefix("[task-return]")
    }
    fn task_cancel(&self, name: &str) -> bool {
        name == "[task-cancel]"
    }
    fn backpressure_inc(&self, name: &str) -> bool {
        name == "[backpressure-inc]"
    }
    fn backpressure_dec(&self, name: &str) -> bool {
        name == "[backpressure-dec]"
    }
    fn waitable_set_new(&self, name: &str) -> bool {
        name == "[waitable-set-new]"
    }
    fn waitable_set_wait(&self, name: &str) -> Option<MaybeCancellable<()>> {
        self.match_with_cancellable_prefix(name, "[waitable-set-wait]")
    }
    fn waitable_set_poll(&self, name: &str) -> Option<MaybeCancellable<()>> {
        self.match_with_cancellable_prefix(name, "[waitable-set-poll]")
    }
    fn waitable_set_drop(&self, name: &str) -> bool {
        name == "[waitable-set-drop]"
    }
    fn waitable_join(&self, name: &str) -> bool {
        name == "[waitable-join]"
    }
    fn thread_yield(&self, name: &str) -> Option<MaybeCancellable<()>> {
        self.match_with_cancellable_prefix(name, "[thread-yield]")
    }
    fn subtask_drop(&self, name: &str) -> bool {
        name == "[subtask-drop]"
    }
    fn subtask_cancel(&self, name: &str) -> Option<MaybeAsyncLowered<()>> {
        self.match_with_async_lowered_prefix(name, "[subtask-cancel]")
    }
    fn async_lift_callback_name<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_prefix("[callback][async-lift]")
    }
    fn async_lift_name<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_prefix("[async-lift]")
    }
    fn async_lift_stackful_name<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_prefix("[async-lift-stackful]")
    }
    fn error_context_new(&self, name: &str) -> Option<StringEncoding> {
        match name {
            "[error-context-new-utf8]" => Some(StringEncoding::UTF8),
            "[error-context-new-utf16]" => Some(StringEncoding::UTF16),
            "[error-context-new-latin1+utf16]" => Some(StringEncoding::CompactUTF16),
            _ => None,
        }
    }
    fn error_context_debug_message(&self, name: &str) -> Option<StringEncoding> {
        match name {
            "[error-context-debug-message-utf8]" => Some(StringEncoding::UTF8),
            "[error-context-debug-message-utf16]" => Some(StringEncoding::UTF16),
            "[error-context-debug-message-latin1+utf16]" => Some(StringEncoding::CompactUTF16),
            _ => None,
        }
    }
    fn error_context_drop(&self, name: &str) -> bool {
        name == "[error-context-drop]"
    }
    fn context_get(&self, name: &str) -> Option<u32> {
        let (n, rest) = prefixed_integer(name, "[context-get-")?;
        if rest.is_empty() { Some(n) } else { None }
    }
    fn context_set(&self, name: &str) -> Option<u32> {
        let (n, rest) = prefixed_integer(name, "[context-set-")?;
        if rest.is_empty() { Some(n) } else { None }
    }
    fn thread_index(&self, name: &str) -> bool {
        name == "[thread-index]"
    }
    fn thread_new_indirect(&self, name: &str) -> bool {
        // For now, we'll fix the type of the start function and the table to extract it from
        name == "[thread-new-indirect-v0]"
    }
    fn thread_switch_to(&self, name: &str) -> Option<MaybeCancellable<()>> {
        self.match_with_cancellable_prefix(name, "[thread-switch-to]")
    }
    fn thread_suspend(&self, name: &str) -> Option<MaybeCancellable<()>> {
        self.match_with_cancellable_prefix(name, "[thread-suspend]")
    }
    fn thread_resume_later(&self, name: &str) -> bool {
        name == "[thread-resume-later]"
    }
    fn thread_yield_to(&self, name: &str) -> Option<MaybeCancellable<()>> {
        self.match_with_cancellable_prefix(name, "[thread-yield-to]")
    }
    fn future_new(&self, lookup_context: &PayloadLookupContext, name: &str) -> Option<PayloadInfo> {
        self.prefixed_payload(lookup_context, name, "[future-new-")
    }
    fn future_write(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        self.maybe_async_lowered_payload(lookup_context, name, "[future-write-")
    }
    fn future_read(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        self.maybe_async_lowered_payload(lookup_context, name, "[future-read-")
    }
    fn future_cancel_write(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        self.maybe_async_lowered_payload(lookup_context, name, "[future-cancel-write-")
    }
    fn future_cancel_read(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        self.maybe_async_lowered_payload(lookup_context, name, "[future-cancel-read-")
    }
    fn future_drop_writable(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<PayloadInfo> {
        self.prefixed_payload(lookup_context, name, "[future-drop-writable-")
    }
    fn future_drop_readable(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<PayloadInfo> {
        self.prefixed_payload(lookup_context, name, "[future-drop-readable-")
    }
    fn stream_new(&self, lookup_context: &PayloadLookupContext, name: &str) -> Option<PayloadInfo> {
        self.prefixed_payload(lookup_context, name, "[stream-new-")
    }
    fn stream_write(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        self.maybe_async_lowered_payload(lookup_context, name, "[stream-write-")
    }
    fn stream_read(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        self.maybe_async_lowered_payload(lookup_context, name, "[stream-read-")
    }
    fn stream_cancel_write(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        self.maybe_async_lowered_payload(lookup_context, name, "[stream-cancel-write-")
    }
    fn stream_cancel_read(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<MaybeAsyncLowered<PayloadInfo>> {
        self.maybe_async_lowered_payload(lookup_context, name, "[stream-cancel-read-")
    }
    fn stream_drop_writable(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<PayloadInfo> {
        self.prefixed_payload(lookup_context, name, "[stream-drop-writable-")
    }
    fn stream_drop_readable(
        &self,
        lookup_context: &PayloadLookupContext,
        name: &str,
    ) -> Option<PayloadInfo> {
        self.prefixed_payload(lookup_context, name, "[stream-drop-readable-")
    }
    fn module_to_interface(
        &self,
        module: &str,
        resolve: &Resolve,
        items: &IndexMap<WorldKey, WorldItem>,
    ) -> Result<(WorldKey, InterfaceId)> {
        // First see if this is a bare name
        let bare_name = WorldKey::Name(module.to_string());
        if let Some(WorldItem::Interface { id, .. }) = items.get(&bare_name) {
            return Ok((bare_name, *id));
        }

        // ... and if this isn't a bare name then it's time to do some parsing
        // related to interfaces, versions, and such. First up the `module` name
        // is parsed as a normal component name from `wasmparser` to see if it's
        // of the "interface kind". If it's not then that means the above match
        // should have been a hit but it wasn't, so an error is returned.
        let kebab_name = ComponentName::new(module, 0);
        let name = match kebab_name.as_ref().map(|k| k.kind()) {
            Ok(ComponentNameKind::Interface(name)) => name,
            _ => bail!("module requires an import interface named `{module}`"),
        };

        // Prioritize an exact match based on versions, so try that first.
        let pkgname = PackageName {
            namespace: name.namespace().to_string(),
            name: name.package().to_string(),
            version: name.version(),
        };
        if let Some(pkg) = resolve.package_names.get(&pkgname) {
            if let Some(id) = resolve.packages[*pkg]
                .interfaces
                .get(name.interface().as_str())
            {
                let key = WorldKey::Interface(*id);
                if items.contains_key(&key) {
                    return Ok((key, *id));
                }
            }
        }

        // If an exact match wasn't found then instead search for the first
        // match based on versions. This means that a core wasm import for
        // "1.2.3" might end up matching an interface at "1.2.4", for example.
        // (or "1.2.2", depending on what's available).
        for (key, _) in items {
            let id = match key {
                WorldKey::Interface(id) => *id,
                WorldKey::Name(_) => continue,
            };
            // Make sure the interface names match
            let interface = &resolve.interfaces[id];
            if interface.name.as_ref().unwrap() != name.interface().as_str() {
                continue;
            }

            // Make sure the package name (without version) matches
            let pkg = &resolve.packages[interface.package.unwrap()];
            if pkg.name.namespace != pkgname.namespace || pkg.name.name != pkgname.name {
                continue;
            }

            let module_version = match name.version() {
                Some(version) => version,
                None => continue,
            };
            let pkg_version = match &pkg.name.version {
                Some(version) => version,
                None => continue,
            };

            // Test if the two semver versions are compatible
            let module_compat = PackageName::version_compat_track(&module_version);
            let pkg_compat = PackageName::version_compat_track(pkg_version);
            if module_compat == pkg_compat {
                return Ok((key.clone(), id));
            }
        }

        bail!("module requires an import interface named `{module}`")
    }
    fn strip_post_return<'a>(&self, name: &'a str) -> Option<&'a str> {
        name.strip_prefix("cabi_post_")
    }
    fn match_wit_export<'a>(
        &self,
        export_name: &str,
        resolve: &'a Resolve,
        world: WorldId,
        exports: &'a IndexSet<WorldKey>,
    ) -> Option<(&'a WorldKey, Option<InterfaceId>, &'a Function)> {
        let world = &resolve.worlds[world];
        for name in exports {
            match &world.exports[name] {
                WorldItem::Function(f) => {
                    if f.legacy_core_export_name(None) == export_name {
                        return Some((name, None, f));
                    }
                }
                WorldItem::Interface { id, .. } => {
                    let string = resolve.name_world_key(name);
                    for (_, func) in resolve.interfaces[*id].functions.iter() {
                        if func.legacy_core_export_name(Some(&string)) == export_name {
                            return Some((name, Some(*id), func));
                        }
                    }
                }

                WorldItem::Type(_) => unreachable!(),
            }
        }

        None
    }

    fn match_wit_resource_dtor<'a>(
        &self,
        export_name: &str,
        resolve: &'a Resolve,
        world: WorldId,
        exports: &'a IndexSet<WorldKey>,
    ) -> Option<TypeId> {
        let world = &resolve.worlds[world];
        for name in exports {
            let id = match &world.exports[name] {
                WorldItem::Interface { id, .. } => *id,
                WorldItem::Function(_) => continue,
                WorldItem::Type(_) => unreachable!(),
            };
            let name = resolve.name_world_key(name);
            let resource = match export_name
                .strip_prefix(&name)
                .and_then(|s| s.strip_prefix("#[dtor]"))
                .and_then(|r| resolve.interfaces[id].types.get(r))
            {
                Some(id) => *id,
                None => continue,
            };

            match resolve.types[resource].kind {
                TypeDefKind::Resource => {}
                _ => continue,
            }

            return Some(resource);
        }

        None
    }

    fn world_key_name_and_abi<'a>(&self, name: &'a str) -> (&'a str, AbiVariant) {
        let (async_abi, name) = self.strip_async_lowered_prefix(name);
        (
            name,
            if async_abi {
                AbiVariant::GuestImportAsync
            } else {
                AbiVariant::GuestImport
            },
        )
    }
    fn interface_function_name_and_abi<'a>(&self, name: &'a str) -> (&'a str, AbiVariant) {
        let (async_abi, name) = self.strip_async_lowered_prefix(name);
        (
            name,
            if async_abi {
                AbiVariant::GuestImportAsync
            } else {
                AbiVariant::GuestImport
            },
        )
    }
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
pub fn validate_module(
    encoder: &ComponentEncoder,
    bytes: &[u8],
    import_map: Option<&ModuleImportMap>,
) -> Result<ValidatedModule> {
    ValidatedModule::new(
        encoder,
        bytes,
        &encoder.main_module_exports,
        import_map,
        None,
    )
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
pub fn validate_adapter_module(
    encoder: &ComponentEncoder,
    bytes: &[u8],
    required_by_import: &IndexMap<String, FuncType>,
    exports: &IndexSet<WorldKey>,
    library_info: Option<&LibraryInfo>,
) -> Result<ValidatedModule> {
    let ret = ValidatedModule::new(encoder, bytes, exports, None, library_info)?;

    for (name, required_ty) in required_by_import {
        let actual = match ret.exports.raw_exports.get(name) {
            Some(ty) => ty,
            None => return Err(AdapterModuleDidNotExport(name.clone()).into()),
        };
        validate_func_sig(name, required_ty, &actual)?;
    }

    Ok(ret)
}

/// An error that can be returned from adapting a core Wasm module into a
/// component using an adapter module.
///
/// If the core Wasm module contained an import that it requires to be
/// satisfied by the adapter, and the adapter does not contain an export
/// with the same name, an instance of this error is returned.
#[derive(Debug, Clone)]
pub struct AdapterModuleDidNotExport(String);

impl fmt::Display for AdapterModuleDidNotExport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "adapter module did not export `{}`", self.0)
    }
}

impl std::error::Error for AdapterModuleDidNotExport {}

fn resource_test_for_interface<'a>(
    resolve: &'a Resolve,
    id: InterfaceId,
) -> impl Fn(&str) -> Option<TypeId> + 'a {
    let interface = &resolve.interfaces[id];
    move |name: &str| {
        let ty = match interface.types.get(name) {
            Some(ty) => *ty,
            None => return None,
        };
        if matches!(resolve.types[ty].kind, TypeDefKind::Resource) {
            Some(ty)
        } else {
            None
        }
    }
}

fn resource_test_for_world<'a>(
    resolve: &'a Resolve,
    id: WorldId,
) -> impl Fn(&str) -> Option<TypeId> + 'a {
    let world = &resolve.worlds[id];
    move |name: &str| match world.imports.get(&WorldKey::Name(name.to_string()))? {
        WorldItem::Type(r) => {
            if matches!(resolve.types[*r].kind, TypeDefKind::Resource) {
                Some(*r)
            } else {
                None
            }
        }
        _ => None,
    }
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

/// Matches `name` as `[${prefix}S]...`, and if found returns `("S", "...")`
fn prefixed_intrinsic<'a>(name: &'a str, prefix: &str) -> Option<(&'a str, &'a str)> {
    assert!(prefix.starts_with("["));
    assert!(prefix.ends_with("-"));
    let suffix = name.strip_prefix(prefix)?;
    let index = suffix.find(']')?;
    let rest = &suffix[index + 1..];
    Some((&suffix[..index], rest))
}

/// Matches `name` as `[${prefix}N]...`, and if found returns `(N, "...")`
fn prefixed_integer<'a>(name: &'a str, prefix: &str) -> Option<(u32, &'a str)> {
    let (suffix, rest) = prefixed_intrinsic(name, prefix)?;
    let n = suffix.parse().ok()?;
    Some((n, rest))
}

fn get_function<'a>(
    resolve: &'a Resolve,
    world: &'a World,
    name: &str,
    interface: Option<InterfaceId>,
    imported: bool,
) -> Result<&'a Function> {
    let function = if let Some(id) = interface {
        return resolve.interfaces[id]
            .functions
            .get(name)
            .ok_or_else(|| anyhow!("no export `{name}` found"));
    } else if imported {
        world.imports.get(&WorldKey::Name(name.to_string()))
    } else {
        world.exports.get(&WorldKey::Name(name.to_string()))
    };
    let Some(WorldItem::Function(function)) = function else {
        bail!("no export `{name}` found");
    };
    Ok(function)
}

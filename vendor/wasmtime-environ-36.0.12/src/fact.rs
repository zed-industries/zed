//! Wasmtime's Fused Adapter Compiler of Trampolines (FACT)
//!
//! This module contains a compiler which emits trampolines to implement fused
//! adapters for the component model. A fused adapter is when a core wasm
//! function is lifted from one component instance and then lowered into another
//! component instance. This communication between components is well-defined by
//! the spec and ends up creating what's called a "fused adapter".
//!
//! Adapters are currently implemented with WebAssembly modules. This submodule
//! will generate a core wasm binary which contains the adapters specified
//! during compilation. The actual wasm is then later processed by standard
//! paths in Wasmtime to create native machine code and runtime representations
//! of modules.
//!
//! Note that identification of precisely what goes into an adapter module is
//! not handled in this file, instead that's all done in `translate/adapt.rs`.
//! Otherwise this module is only responsible for taking a set of adapters and
//! their imports and then generating a core wasm module to implement all of
//! that.

use crate::component::dfg::CoreDef;
use crate::component::{
    Adapter, AdapterOptions as AdapterOptionsDfg, ComponentTypesBuilder, FlatType, InterfaceType,
    RuntimeComponentInstanceIndex, StringEncoding, Transcode, TypeFuncIndex,
};
use crate::fact::transcode::Transcoder;
use crate::{EntityRef, FuncIndex, GlobalIndex, MemoryIndex, PrimaryMap};
use crate::{ModuleInternedTypeIndex, prelude::*};
use std::borrow::Cow;
use std::collections::HashMap;
use wasm_encoder::*;

mod core_types;
mod signature;
mod trampoline;
mod transcode;
mod traps;

/// Fixed parameter types for the `prepare_call` built-in function.
///
/// Note that `prepare_call` also takes a variable number of parameters in
/// addition to these, determined by the signature of the function for which
/// we're generating an adapter.
pub static PREPARE_CALL_FIXED_PARAMS: &[ValType] = &[
    ValType::FUNCREF, // start
    ValType::FUNCREF, // return
    ValType::I32,     // caller_instance
    ValType::I32,     // callee_instance
    ValType::I32,     // task_return_type
    ValType::I32,     // string_encoding
    ValType::I32,     // result_count_or_max_if_async
];

/// Representation of an adapter module.
pub struct Module<'a> {
    /// Whether or not debug code is inserted into the adapters themselves.
    debug: bool,
    /// Type information from the creator of this `Module`
    types: &'a ComponentTypesBuilder,

    /// Core wasm type section that's incrementally built
    core_types: core_types::CoreTypes,

    /// Core wasm import section which is built as adapters are inserted. Note
    /// that imports here are intern'd to avoid duplicate imports of the same
    /// item.
    core_imports: ImportSection,
    /// Final list of imports that this module ended up using, in the same order
    /// as the imports in the import section.
    imports: Vec<Import>,
    /// Intern'd imports and what index they were assigned. Note that this map
    /// covers all the index spaces for imports, not just one.
    imported: HashMap<CoreDef, usize>,
    /// Intern'd transcoders and what index they were assigned.
    imported_transcoders: HashMap<Transcoder, FuncIndex>,

    /// Cached versions of imported trampolines for working with resources.
    imported_resource_transfer_own: Option<FuncIndex>,
    imported_resource_transfer_borrow: Option<FuncIndex>,
    imported_resource_enter_call: Option<FuncIndex>,
    imported_resource_exit_call: Option<FuncIndex>,

    // Cached versions of imported trampolines for working with the async ABI.
    imported_async_start_calls: HashMap<(Option<FuncIndex>, Option<FuncIndex>), FuncIndex>,

    // Cached versions of imported trampolines for working with `stream`s,
    // `future`s, and `error-context`s.
    imported_future_transfer: Option<FuncIndex>,
    imported_stream_transfer: Option<FuncIndex>,
    imported_error_context_transfer: Option<FuncIndex>,

    // Current status of index spaces from the imports generated so far.
    imported_funcs: PrimaryMap<FuncIndex, Option<CoreDef>>,
    imported_memories: PrimaryMap<MemoryIndex, CoreDef>,
    imported_globals: PrimaryMap<GlobalIndex, CoreDef>,

    funcs: PrimaryMap<FunctionId, Function>,
    helper_funcs: HashMap<Helper, FunctionId>,
    helper_worklist: Vec<(FunctionId, Helper)>,

    exports: Vec<(u32, String)>,
}

struct AdapterData {
    /// Export name of this adapter
    name: String,
    /// Options specified during the `canon lift` operation
    lift: AdapterOptions,
    /// Options specified during the `canon lower` operation
    lower: AdapterOptions,
    /// The core wasm function that this adapter will be calling (the original
    /// function that was `canon lift`'d)
    callee: FuncIndex,
    /// FIXME(#4185) should be plumbed and handled as part of the new reentrance
    /// rules not yet implemented here.
    called_as_export: bool,
}

/// Configuration options which apply at the "global adapter" level.
///
/// These options are typically unique per-adapter and generally aren't needed
/// when translating recursive types within an adapter.
struct AdapterOptions {
    instance: RuntimeComponentInstanceIndex,
    /// The ascribed type of this adapter.
    ty: TypeFuncIndex,
    /// The global that represents the instance flags for where this adapter
    /// came from.
    flags: GlobalIndex,
    /// The configured post-return function, if any.
    post_return: Option<FuncIndex>,
    /// Other, more general, options configured.
    options: Options,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
/// Linear memory.
struct LinearMemoryOptions {
    /// Whether or not the `memory` field, if present, is a 64-bit memory.
    memory64: bool,
    /// An optionally-specified memory where values may travel through for
    /// types like lists.
    memory: Option<MemoryIndex>,
    /// An optionally-specified function to be used to allocate space for
    /// types such as strings as they go into a module.
    realloc: Option<FuncIndex>,
}

impl LinearMemoryOptions {
    fn ptr(&self) -> ValType {
        if self.memory64 {
            ValType::I64
        } else {
            ValType::I32
        }
    }

    fn ptr_size(&self) -> u8 {
        if self.memory64 { 8 } else { 4 }
    }
}

/// The data model for objects passed through an adapter.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum DataModel {
    Gc {},
    LinearMemory(LinearMemoryOptions),
}

impl DataModel {
    #[track_caller]
    fn unwrap_memory(&self) -> &LinearMemoryOptions {
        match self {
            DataModel::Gc {} => panic!("`unwrap_memory` on GC"),
            DataModel::LinearMemory(opts) => opts,
        }
    }
}

/// This type is split out of `AdapterOptions` and is specifically used to
/// deduplicate translation functions within a module. Consequently this has
/// as few fields as possible to minimize the number of functions generated
/// within an adapter module.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Options {
    /// The encoding that strings use from this adapter.
    string_encoding: StringEncoding,
    callback: Option<FuncIndex>,
    async_: bool,
    core_type: ModuleInternedTypeIndex,
    data_model: DataModel,
}

/// Representation of a "helper function" which may be generated as part of
/// generating an adapter trampoline.
///
/// Helper functions are created when inlining the translation for a type in its
/// entirety would make a function excessively large. This is currently done via
/// a simple fuel/cost heuristic based on the type being translated but may get
/// fancier over time.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Helper {
    /// Metadata about the source type of what's being translated.
    src: HelperType,
    /// Metadata about the destination type which is being translated to.
    dst: HelperType,
}

/// Information about a source or destination type in a `Helper` which is
/// generated.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct HelperType {
    /// The concrete type being translated.
    ty: InterfaceType,
    /// The configuration options (memory, etc) for the adapter.
    opts: Options,
    /// Where the type is located (either the stack or in memory)
    loc: HelperLocation,
}

/// Where a `HelperType` is located, dictating the signature of the helper
/// function.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum HelperLocation {
    /// Located on the stack in wasm locals.
    Stack,
    /// Located in linear memory as configured by `opts`.
    Memory,
    /// Located in a GC struct field.
    #[expect(dead_code, reason = "CM+GC is still WIP")]
    StructField,
    /// Located in a GC array element.
    #[expect(dead_code, reason = "CM+GC is still WIP")]
    ArrayElement,
}

impl<'a> Module<'a> {
    /// Creates an empty module.
    pub fn new(types: &'a ComponentTypesBuilder, debug: bool) -> Module<'a> {
        Module {
            debug,
            types,
            core_types: Default::default(),
            core_imports: Default::default(),
            imported: Default::default(),
            imports: Default::default(),
            imported_transcoders: Default::default(),
            imported_funcs: PrimaryMap::new(),
            imported_memories: PrimaryMap::new(),
            imported_globals: PrimaryMap::new(),
            funcs: PrimaryMap::new(),
            helper_funcs: HashMap::new(),
            helper_worklist: Vec::new(),
            imported_resource_transfer_own: None,
            imported_resource_transfer_borrow: None,
            imported_resource_enter_call: None,
            imported_resource_exit_call: None,
            imported_async_start_calls: HashMap::new(),
            imported_future_transfer: None,
            imported_stream_transfer: None,
            imported_error_context_transfer: None,
            exports: Vec::new(),
        }
    }

    /// Registers a new adapter within this adapter module.
    ///
    /// The `name` provided is the export name of the adapter from the final
    /// module, and `adapter` contains all metadata necessary for compilation.
    pub fn adapt(&mut self, name: &str, adapter: &Adapter) {
        // Import any items required by the various canonical options
        // (memories, reallocs, etc)
        let mut lift = self.import_options(adapter.lift_ty, &adapter.lift_options);
        let lower = self.import_options(adapter.lower_ty, &adapter.lower_options);

        // Lowering options are not allowed to specify post-return as per the
        // current canonical abi specification.
        assert!(adapter.lower_options.post_return.is_none());

        // Import the core wasm function which was lifted using its appropriate
        // signature since the exported function this adapter generates will
        // call the lifted function.
        let signature = self.types.signature(&lift);
        let ty = self
            .core_types
            .function(&signature.params, &signature.results);
        let callee = self.import_func("callee", name, ty, adapter.func.clone());

        // Handle post-return specifically here where we have `core_ty` and the
        // results of `core_ty` are the parameters to the post-return function.
        lift.post_return = adapter.lift_options.post_return.as_ref().map(|func| {
            let ty = self.core_types.function(&signature.results, &[]);
            self.import_func("post_return", name, ty, func.clone())
        });

        // This will internally create the adapter as specified and append
        // anything necessary to `self.funcs`.
        trampoline::compile(
            self,
            &AdapterData {
                name: name.to_string(),
                lift,
                lower,
                callee,
                // FIXME(#4185) should be plumbed and handled as part of the new
                // reentrance rules not yet implemented here.
                called_as_export: true,
            },
        );

        while let Some((result, helper)) = self.helper_worklist.pop() {
            trampoline::compile_helper(self, result, helper);
        }
    }

    fn import_options(&mut self, ty: TypeFuncIndex, options: &AdapterOptionsDfg) -> AdapterOptions {
        let AdapterOptionsDfg {
            instance,
            string_encoding,
            post_return: _, // handled above
            callback,
            async_,
            core_type,
            data_model,
        } = options;

        let flags = self.import_global(
            "flags",
            &format!("instance{}", instance.as_u32()),
            GlobalType {
                val_type: ValType::I32,
                mutable: true,
                shared: false,
            },
            CoreDef::InstanceFlags(*instance),
        );

        let data_model = match data_model {
            crate::component::DataModel::Gc {} => DataModel::Gc {},
            crate::component::DataModel::LinearMemory {
                memory,
                memory64,
                realloc,
            } => {
                let memory = memory.as_ref().map(|memory| {
                    self.import_memory(
                        "memory",
                        &format!("m{}", self.imported_memories.len()),
                        MemoryType {
                            minimum: 0,
                            maximum: None,
                            shared: false,
                            memory64: *memory64,
                            page_size_log2: None,
                        },
                        memory.clone().into(),
                    )
                });
                let realloc = realloc.as_ref().map(|func| {
                    let ptr = if *memory64 {
                        ValType::I64
                    } else {
                        ValType::I32
                    };
                    let ty = self.core_types.function(&[ptr, ptr, ptr, ptr], &[ptr]);
                    self.import_func(
                        "realloc",
                        &format!("f{}", self.imported_funcs.len()),
                        ty,
                        func.clone(),
                    )
                });
                DataModel::LinearMemory(LinearMemoryOptions {
                    memory64: *memory64,
                    memory,
                    realloc,
                })
            }
        };

        let callback = callback.as_ref().map(|func| {
            let ty = self
                .core_types
                .function(&[ValType::I32, ValType::I32, ValType::I32], &[ValType::I32]);
            self.import_func(
                "callback",
                &format!("f{}", self.imported_funcs.len()),
                ty,
                func.clone(),
            )
        });

        AdapterOptions {
            instance: *instance,
            ty,
            flags,
            post_return: None,
            options: Options {
                string_encoding: *string_encoding,
                callback,
                async_: *async_,
                core_type: *core_type,
                data_model,
            },
        }
    }

    fn import_func(&mut self, module: &str, name: &str, ty: u32, def: CoreDef) -> FuncIndex {
        self.import(module, name, EntityType::Function(ty), def, |m| {
            &mut m.imported_funcs
        })
    }

    fn import_global(
        &mut self,
        module: &str,
        name: &str,
        ty: GlobalType,
        def: CoreDef,
    ) -> GlobalIndex {
        self.import(module, name, EntityType::Global(ty), def, |m| {
            &mut m.imported_globals
        })
    }

    fn import_memory(
        &mut self,
        module: &str,
        name: &str,
        ty: MemoryType,
        def: CoreDef,
    ) -> MemoryIndex {
        self.import(module, name, EntityType::Memory(ty), def, |m| {
            &mut m.imported_memories
        })
    }

    fn import<K: EntityRef, V: From<CoreDef>>(
        &mut self,
        module: &str,
        name: &str,
        ty: EntityType,
        def: CoreDef,
        map: impl FnOnce(&mut Self) -> &mut PrimaryMap<K, V>,
    ) -> K {
        if let Some(prev) = self.imported.get(&def) {
            return K::new(*prev);
        }
        let idx = map(self).push(def.clone().into());
        self.core_imports.import(module, name, ty);
        self.imported.insert(def.clone(), idx.index());
        self.imports.push(Import::CoreDef(def));
        idx
    }

    fn import_transcoder(&mut self, transcoder: transcode::Transcoder) -> FuncIndex {
        *self
            .imported_transcoders
            .entry(transcoder)
            .or_insert_with(|| {
                // Add the import to the core wasm import section...
                let name = transcoder.name();
                let ty = transcoder.ty(&mut self.core_types);
                self.core_imports.import("transcode", &name, ty);

                // ... and also record the metadata for what this import
                // corresponds to.
                let from = self.imported_memories[transcoder.from_memory].clone();
                let to = self.imported_memories[transcoder.to_memory].clone();
                self.imports.push(Import::Transcode {
                    op: transcoder.op,
                    from,
                    from64: transcoder.from_memory64,
                    to,
                    to64: transcoder.to_memory64,
                });

                self.imported_funcs.push(None)
            })
    }

    fn import_simple(
        &mut self,
        module: &str,
        name: &str,
        params: &[ValType],
        results: &[ValType],
        import: Import,
        get: impl Fn(&mut Self) -> &mut Option<FuncIndex>,
    ) -> FuncIndex {
        self.import_simple_get_and_set(
            module,
            name,
            params,
            results,
            import,
            |me| *get(me),
            |me, v| *get(me) = Some(v),
        )
    }

    fn import_simple_get_and_set(
        &mut self,
        module: &str,
        name: &str,
        params: &[ValType],
        results: &[ValType],
        import: Import,
        get: impl Fn(&mut Self) -> Option<FuncIndex>,
        set: impl Fn(&mut Self, FuncIndex),
    ) -> FuncIndex {
        if let Some(idx) = get(self) {
            return idx;
        }
        let ty = self.core_types.function(params, results);
        let ty = EntityType::Function(ty);
        self.core_imports.import(module, name, ty);

        self.imports.push(import);
        let idx = self.imported_funcs.push(None);
        set(self, idx);
        idx
    }

    /// Import a host built-in function to set up a subtask for a sync-lowered
    /// import call to an async-lifted export.
    ///
    /// Given that the callee may exert backpressure before the host can copy
    /// the parameters, the adapter must use this function to set up the subtask
    /// and stash the parameters as part of that subtask until any backpressure
    /// has cleared.
    fn import_prepare_call(
        &mut self,
        suffix: &str,
        params: &[ValType],
        memory: Option<MemoryIndex>,
    ) -> FuncIndex {
        let ty = self.core_types.function(
            &PREPARE_CALL_FIXED_PARAMS
                .iter()
                .copied()
                .chain(params.iter().copied())
                .collect::<Vec<_>>(),
            &[],
        );
        self.core_imports.import(
            "sync",
            &format!("[prepare-call]{suffix}"),
            EntityType::Function(ty),
        );
        let import = Import::PrepareCall {
            memory: memory.map(|v| self.imported_memories[v].clone()),
        };
        self.imports.push(import);
        self.imported_funcs.push(None)
    }

    /// Import a host built-in function to start a subtask for a sync-lowered
    /// import call to an async-lifted export.
    ///
    /// This call with block until the subtask has produced result(s) via the
    /// `task.return` intrinsic.
    ///
    /// Note that this could potentially be combined with the `sync-prepare`
    /// built-in into a single built-in function that does both jobs.  However,
    /// we've kept them separate to allow a future optimization where the caller
    /// calls the callee directly rather than using `sync-start` to have the host
    /// do it.
    fn import_sync_start_call(
        &mut self,
        suffix: &str,
        callback: Option<FuncIndex>,
        results: &[ValType],
    ) -> FuncIndex {
        let ty = self
            .core_types
            .function(&[ValType::FUNCREF, ValType::I32], results);
        self.core_imports.import(
            "sync",
            &format!("[start-call]{suffix}"),
            EntityType::Function(ty),
        );
        let import = Import::SyncStartCall {
            callback: callback
                .map(|callback| self.imported_funcs.get(callback).unwrap().clone().unwrap()),
        };
        self.imports.push(import);
        self.imported_funcs.push(None)
    }

    /// Import a host built-in function to start a subtask for an async-lowered
    /// import call to an async- or sync-lifted export.
    ///
    /// Note that this could potentially be combined with the `async-prepare`
    /// built-in into a single built-in function that does both jobs.  However,
    /// we've kept them separate to allow a future optimization where the caller
    /// calls the callee directly rather than using `async-start` to have the
    /// host do it.
    fn import_async_start_call(
        &mut self,
        suffix: &str,
        callback: Option<FuncIndex>,
        post_return: Option<FuncIndex>,
    ) -> FuncIndex {
        self.import_simple_get_and_set(
            "async",
            &format!("[start-call]{suffix}"),
            &[ValType::FUNCREF, ValType::I32, ValType::I32, ValType::I32],
            &[ValType::I32],
            Import::AsyncStartCall {
                callback: callback
                    .map(|callback| self.imported_funcs.get(callback).unwrap().clone().unwrap()),
                post_return: post_return.map(|post_return| {
                    self.imported_funcs
                        .get(post_return)
                        .unwrap()
                        .clone()
                        .unwrap()
                }),
            },
            |me| {
                me.imported_async_start_calls
                    .get(&(callback, post_return))
                    .copied()
            },
            |me, v| {
                assert!(
                    me.imported_async_start_calls
                        .insert((callback, post_return), v)
                        .is_none()
                )
            },
        )
    }

    fn import_future_transfer(&mut self) -> FuncIndex {
        self.import_simple(
            "future",
            "transfer",
            &[ValType::I32; 3],
            &[ValType::I32],
            Import::FutureTransfer,
            |me| &mut me.imported_future_transfer,
        )
    }

    fn import_stream_transfer(&mut self) -> FuncIndex {
        self.import_simple(
            "stream",
            "transfer",
            &[ValType::I32; 3],
            &[ValType::I32],
            Import::StreamTransfer,
            |me| &mut me.imported_stream_transfer,
        )
    }

    fn import_error_context_transfer(&mut self) -> FuncIndex {
        self.import_simple(
            "error-context",
            "transfer",
            &[ValType::I32; 3],
            &[ValType::I32],
            Import::ErrorContextTransfer,
            |me| &mut me.imported_error_context_transfer,
        )
    }

    fn import_resource_transfer_own(&mut self) -> FuncIndex {
        self.import_simple(
            "resource",
            "transfer-own",
            &[ValType::I32, ValType::I32, ValType::I32],
            &[ValType::I32],
            Import::ResourceTransferOwn,
            |me| &mut me.imported_resource_transfer_own,
        )
    }

    fn import_resource_transfer_borrow(&mut self) -> FuncIndex {
        self.import_simple(
            "resource",
            "transfer-borrow",
            &[ValType::I32, ValType::I32, ValType::I32],
            &[ValType::I32],
            Import::ResourceTransferBorrow,
            |me| &mut me.imported_resource_transfer_borrow,
        )
    }

    fn import_resource_enter_call(&mut self) -> FuncIndex {
        self.import_simple(
            "resource",
            "enter-call",
            &[],
            &[],
            Import::ResourceEnterCall,
            |me| &mut me.imported_resource_enter_call,
        )
    }

    fn import_resource_exit_call(&mut self) -> FuncIndex {
        self.import_simple(
            "resource",
            "exit-call",
            &[],
            &[],
            Import::ResourceExitCall,
            |me| &mut me.imported_resource_exit_call,
        )
    }

    fn translate_helper(&mut self, helper: Helper) -> FunctionId {
        *self.helper_funcs.entry(helper).or_insert_with(|| {
            // Generate a fresh `Function` with a unique id for what we're about to
            // generate.
            let ty = helper.core_type(self.types, &mut self.core_types);
            let id = self.funcs.push(Function::new(None, ty));
            self.helper_worklist.push((id, helper));
            id
        })
    }

    /// Encodes this module into a WebAssembly binary.
    pub fn encode(&mut self) -> Vec<u8> {
        // Build the function/export sections of the wasm module in a first pass
        // which will assign a final `FuncIndex` to all functions defined in
        // `self.funcs`.
        let mut funcs = FunctionSection::new();
        let mut exports = ExportSection::new();
        let mut id_to_index = PrimaryMap::<FunctionId, FuncIndex>::new();
        for (id, func) in self.funcs.iter() {
            assert!(func.filled_in);
            let idx = FuncIndex::from_u32(self.imported_funcs.next_key().as_u32() + id.as_u32());
            let id2 = id_to_index.push(idx);
            assert_eq!(id2, id);

            funcs.function(func.ty);

            if let Some(name) = &func.export {
                exports.export(name, ExportKind::Func, idx.as_u32());
            }
        }
        for (idx, name) in &self.exports {
            exports.export(name, ExportKind::Func, *idx);
        }

        // With all functions numbered the fragments of the body of each
        // function can be assigned into one final adapter function.
        let mut code = CodeSection::new();
        let mut traps = traps::TrapSection::default();
        for (id, func) in self.funcs.iter() {
            let mut func_traps = Vec::new();
            let mut body = Vec::new();

            // Encode all locals used for this function
            func.locals.len().encode(&mut body);
            for (count, ty) in func.locals.iter() {
                count.encode(&mut body);
                ty.encode(&mut body);
            }

            // Then encode each "chunk" of a body which may have optional traps
            // specified within it. Traps get offset by the current length of
            // the body and otherwise our `Call` instructions are "relocated"
            // here to the final function index.
            for chunk in func.body.iter() {
                match chunk {
                    Body::Raw(code, traps) => {
                        let start = body.len();
                        body.extend_from_slice(code);
                        for (offset, trap) in traps {
                            func_traps.push((start + offset, *trap));
                        }
                    }
                    Body::Call(id) => {
                        Instruction::Call(id_to_index[*id].as_u32()).encode(&mut body);
                    }
                    Body::RefFunc(id) => {
                        Instruction::RefFunc(id_to_index[*id].as_u32()).encode(&mut body);
                    }
                }
            }
            code.raw(&body);
            traps.append(id_to_index[id].as_u32(), func_traps);
        }

        let traps = traps.finish();

        let mut result = wasm_encoder::Module::new();
        result.section(&self.core_types.section);
        result.section(&self.core_imports);
        result.section(&funcs);
        result.section(&exports);
        result.section(&code);
        if self.debug {
            result.section(&CustomSection {
                name: "wasmtime-trampoline-traps".into(),
                data: Cow::Borrowed(&traps),
            });
        }
        result.finish()
    }

    /// Returns the imports that were used, in order, to create this adapter
    /// module.
    pub fn imports(&self) -> &[Import] {
        &self.imports
    }
}

/// Possible imports into an adapter module.
#[derive(Clone)]
pub enum Import {
    /// A definition required in the configuration of an `Adapter`.
    CoreDef(CoreDef),
    /// A transcoding function from the host to convert between string encodings.
    Transcode {
        /// The transcoding operation this performs.
        op: Transcode,
        /// The memory being read
        from: CoreDef,
        /// Whether or not `from` is a 64-bit memory
        from64: bool,
        /// The memory being written
        to: CoreDef,
        /// Whether or not `to` is a 64-bit memory
        to64: bool,
    },
    /// Transfers an owned resource from one table to another.
    ResourceTransferOwn,
    /// Transfers a borrowed resource from one table to another.
    ResourceTransferBorrow,
    /// Sets up entry metadata for a borrow resources when a call starts.
    ResourceEnterCall,
    /// Tears down a previous entry and handles checking borrow-related
    /// metadata.
    ResourceExitCall,
    /// An intrinsic used by FACT-generated modules to begin a call involving
    /// an async-lowered import and/or an async-lifted export.
    PrepareCall {
        /// The memory used to verify that the memory specified for the
        /// `task.return` that is called at runtime (if any) matches the one
        /// specified in the lifted export.
        memory: Option<CoreDef>,
    },
    /// An intrinsic used by FACT-generated modules to complete a call involving
    /// a sync-lowered import and async-lifted export.
    SyncStartCall {
        /// The callee's callback function, if any.
        callback: Option<CoreDef>,
    },
    /// An intrinsic used by FACT-generated modules to complete a call involving
    /// an async-lowered import function.
    AsyncStartCall {
        /// The callee's callback function, if any.
        callback: Option<CoreDef>,

        /// The callee's post-return function, if any.
        post_return: Option<CoreDef>,
    },
    /// An intrinisic used by FACT-generated modules to (partially or entirely) transfer
    /// ownership of a `future`.
    FutureTransfer,
    /// An intrinisic used by FACT-generated modules to (partially or entirely) transfer
    /// ownership of a `stream`.
    StreamTransfer,
    /// An intrinisic used by FACT-generated modules to (partially or entirely) transfer
    /// ownership of an `error-context`.
    ErrorContextTransfer,
}

impl Options {
    fn flat_types<'a>(
        &self,
        ty: &InterfaceType,
        types: &'a ComponentTypesBuilder,
    ) -> Option<&'a [FlatType]> {
        let flat = types.flat_types(ty)?;
        match self.data_model {
            DataModel::Gc {} => todo!("CM+GC"),
            DataModel::LinearMemory(mem_opts) => Some(if mem_opts.memory64 {
                flat.memory64
            } else {
                flat.memory32
            }),
        }
    }
}

/// Temporary index which is not the same as `FuncIndex`.
///
/// This represents the nth generated function in the adapter module where the
/// final index of the function is not known at the time of generation since
/// more imports may be discovered (specifically string transcoders).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct FunctionId(u32);
cranelift_entity::entity_impl!(FunctionId);

/// A generated function to be added to an adapter module.
///
/// At least one function is created per-adapter and depending on the type
/// hierarchy multiple functions may be generated per-adapter.
struct Function {
    /// Whether or not the `body` has been finished.
    ///
    /// Functions are added to a `Module` before they're defined so this is used
    /// to assert that the function was in fact actually filled in by the
    /// time we reach `Module::encode`.
    filled_in: bool,

    /// The type signature that this function has, as an index into the core
    /// wasm type index space of the generated adapter module.
    ty: u32,

    /// The locals that are used by this function, organized by the number of
    /// types of each local.
    locals: Vec<(u32, ValType)>,

    /// If specified, the export name of this function.
    export: Option<String>,

    /// The contents of the function.
    ///
    /// See `Body` for more information, and the `Vec` here represents the
    /// concatenation of all the `Body` fragments.
    body: Vec<Body>,
}

/// Representation of a fragment of the body of a core wasm function generated
/// for adapters.
///
/// This variant comes in one of two flavors:
///
/// 1. First a `Raw` variant is used to contain general instructions for the
///    wasm function. This is populated by `Compiler::instruction` primarily.
///    This also comes with a list of traps. and the byte offset within the
///    first vector of where the trap information applies to.
///
/// 2. A `Call` instruction variant for a `FunctionId` where the final
///    `FuncIndex` isn't known until emission time.
///
/// The purpose of this representation is the `Body::Call` variant. This can't
/// be encoded as an instruction when it's generated due to not knowing the
/// final index of the function being called. During `Module::encode`, however,
/// all indices are known and `Body::Call` is turned into a final
/// `Instruction::Call`.
///
/// One other possible representation in the future would be to encode a `Call`
/// instruction with a 5-byte leb to fill in later, but for now this felt
/// easier to represent. A 5-byte leb may be more efficient at compile-time if
/// necessary, however.
enum Body {
    Raw(Vec<u8>, Vec<(usize, traps::Trap)>),
    Call(FunctionId),
    RefFunc(FunctionId),
}

impl Function {
    fn new(export: Option<String>, ty: u32) -> Function {
        Function {
            filled_in: false,
            ty,
            locals: Vec::new(),
            export,
            body: Vec::new(),
        }
    }
}

impl Helper {
    fn core_type(
        &self,
        types: &ComponentTypesBuilder,
        core_types: &mut core_types::CoreTypes,
    ) -> u32 {
        let mut params = Vec::new();
        let mut results = Vec::new();
        // The source type being translated is always pushed onto the
        // parameters first, either a pointer for memory or its flat
        // representation.
        self.src.push_flat(&mut params, types);

        // The destination type goes into the parameter list if it's from
        // memory or otherwise is the result of the function itself for a
        // stack-based representation.
        match self.dst.loc {
            HelperLocation::Stack => self.dst.push_flat(&mut results, types),
            HelperLocation::Memory => params.push(self.dst.opts.data_model.unwrap_memory().ptr()),
            HelperLocation::StructField | HelperLocation::ArrayElement => todo!("CM+GC"),
        }

        core_types.function(&params, &results)
    }
}

impl HelperType {
    fn push_flat(&self, dst: &mut Vec<ValType>, types: &ComponentTypesBuilder) {
        match self.loc {
            HelperLocation::Stack => {
                for ty in self.opts.flat_types(&self.ty, types).unwrap() {
                    dst.push((*ty).into());
                }
            }
            HelperLocation::Memory => {
                dst.push(self.opts.data_model.unwrap_memory().ptr());
            }
            HelperLocation::StructField | HelperLocation::ArrayElement => todo!("CM+GC"),
        }
    }
}

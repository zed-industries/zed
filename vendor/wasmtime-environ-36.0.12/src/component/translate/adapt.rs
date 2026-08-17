//! Identification and creation of fused adapter modules in Wasmtime.
//!
//! A major piece of the component model is the ability for core wasm modules to
//! talk to each other through the use of lifted and lowered functions. For
//! example one core wasm module can export a function which is lifted. Another
//! component could import that lifted function, lower it, and pass it as the
//! import to another core wasm module. This is what Wasmtime calls "adapter
//! fusion" where two core wasm functions are coming together through the
//! component model.
//!
//! There are a few ingredients during adapter fusion:
//!
//! * A core wasm function which is "lifted".
//! * A "lift type" which is the type that the component model function had in
//!   the original component
//! * A "lower type" which is the type that the component model function has
//!   in the destination component (the one the uses `canon lower`)
//! * Configuration options for both the lift and the lower operations such as
//!   memories, reallocs, etc.
//!
//! With these ingredients combined Wasmtime must produce a function which
//! connects the two components through the options specified. The fused adapter
//! performs tasks such as validation of passed values, copying data between
//! linear memories, etc.
//!
//! Wasmtime's current implementation of fused adapters is designed to reduce
//! complexity elsewhere as much as possible while also being suitable for being
//! used as a polyfill for the component model in JS environments as well. To
//! that end Wasmtime implements a fused adapter with another wasm module that
//! it itself generates on the fly. The usage of WebAssembly for fused adapters
//! has a number of advantages:
//!
//! * There is no need to create a raw Cranelift-based compiler. This is where
//!   majority of "unsafety" lives in Wasmtime so reducing the need to lean on
//!   this or audit another compiler is predicted to weed out a whole class of
//!   bugs in the fused adapter compiler.
//!
//! * As mentioned above generation of WebAssembly modules means that this is
//!   suitable for use in JS environments. For example a hypothetical tool which
//!   polyfills a component onto the web today would need to do something for
//!   adapter modules, and ideally the adapters themselves are speedy. While
//!   this could all be written in JS the adapting process is quite nontrivial
//!   so sharing code with Wasmtime would be ideal.
//!
//! * Using WebAssembly insulates the implementation to bugs to a certain
//!   degree. While logic bugs are still possible it should be much more
//!   difficult to have segfaults or things like that. With adapters exclusively
//!   executing inside a WebAssembly sandbox like everything else the failure
//!   modes to the host at least should be minimized.
//!
//! * Integration into the runtime is relatively simple, the adapter modules are
//!   just another kind of wasm module to instantiate and wire up at runtime.
//!   The goal is that the `GlobalInitializer` list that is processed at runtime
//!   will have all of its `Adapter`-using variants erased by the time it makes
//!   its way all the way up to Wasmtime. This means that the support in
//!   Wasmtime prior to adapter modules is actually the same as the support
//!   after adapter modules are added, keeping the runtime fiddly bits quite
//!   minimal.
//!
//! This isn't to say that this approach isn't without its disadvantages of
//! course. For now though this seems to be a reasonable set of tradeoffs for
//! the development stage of the component model proposal.
//!
//! ## Creating adapter modules
//!
//! With WebAssembly itself being used to implement fused adapters, Wasmtime
//! still has the question of how to organize the adapter functions into actual
//! wasm modules.
//!
//! The first thing you might reach for is to put all the adapters into the same
//! wasm module. This cannot be done, however, because some adapters may depend
//! on other adapters (transitively) to be created. This means that if
//! everything were in the same module there would be no way to instantiate the
//! module. An example of this dependency is an adapter (A) used to create a
//! core wasm instance (M) whose exported memory is then referenced by another
//! adapter (B). In this situation the adapter B cannot be in the same module
//! as adapter A because B needs the memory of M but M is created with A which
//! would otherwise create a circular dependency.
//!
//! The second possibility of organizing adapter modules would be to place each
//! fused adapter into its own module. Each `canon lower` would effectively
//! become a core wasm module instantiation at that point. While this works it's
//! currently believed to be a bit too fine-grained. For example it would mean
//! that importing a dozen lowered functions into a module could possibly result
//! in up to a dozen different adapter modules. While this possibility could
//! work it has been ruled out as "probably too expensive at runtime".
//!
//! Thus the purpose and existence of this module is now evident -- this module
//! exists to identify what exactly goes into which adapter module. This will
//! evaluate the `GlobalInitializer` lists coming out of the `inline` pass and
//! insert `InstantiateModule` entries for where adapter modules should be
//! created.
//!
//! ## Partitioning adapter modules
//!
//! Currently this module does not attempt to be really all that fancy about
//! grouping adapters into adapter modules. The main idea is that most items
//! within an adapter module are likely to be close together since they're
//! theoretically going to be used for an instantiation of a core wasm module
//! just after the fused adapter was declared. With that in mind the current
//! algorithm is a one-pass approach to partitioning everything into adapter
//! modules.
//!
//! Adapters were identified in-order as part of the inlining phase of
//! translation where we're guaranteed that once an adapter is identified
//! it can't depend on anything identified later. The pass implemented here is
//! to visit all transitive dependencies of an adapter. If one of the
//! dependencies of an adapter is an adapter in the current adapter module
//! being built then the current module is finished and a new adapter module is
//! started. This should quickly partition adapters into contiugous chunks of
//! their index space which can be in adapter modules together.
//!
//! There's probably more general algorithms for this but for now this should be
//! fast enough as it's "just" a linear pass. As we get more components over
//! time this may want to be revisited if too many adapter modules are being
//! created.

use crate::EntityType;
use crate::component::translate::*;
use crate::fact;
use std::collections::HashSet;

/// Metadata information about a fused adapter.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Adapter {
    /// The type used when the original core wasm function was lifted.
    ///
    /// Note that this could be different than `lower_ty` (but still matches
    /// according to subtyping rules).
    pub lift_ty: TypeFuncIndex,
    /// Canonical ABI options used when the function was lifted.
    pub lift_options: AdapterOptions,
    /// The type used when the function was lowered back into a core wasm
    /// function.
    ///
    /// Note that this could be different than `lift_ty` (but still matches
    /// according to subtyping rules).
    pub lower_ty: TypeFuncIndex,
    /// Canonical ABI options used when the function was lowered.
    pub lower_options: AdapterOptions,
    /// The original core wasm function which was lifted.
    pub func: dfg::CoreDef,
}

/// The data model for objects that are not unboxed in locals.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum DataModel {
    /// Data is stored in GC objects.
    Gc {},

    /// Data is stored in a linear memory.
    LinearMemory {
        /// An optional memory definition supplied.
        memory: Option<dfg::CoreExport<MemoryIndex>>,
        /// If `memory` is specified, whether it's a 64-bit memory.
        memory64: bool,
        /// An optional definition of `realloc` to used.
        realloc: Option<dfg::CoreDef>,
    },
}

/// Configuration options which can be specified as part of the canonical ABI
/// in the component model.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct AdapterOptions {
    /// The Wasmtime-assigned component instance index where the options were
    /// originally specified.
    pub instance: RuntimeComponentInstanceIndex,
    /// How strings are encoded.
    pub string_encoding: StringEncoding,
    /// The async callback function used by these options, if specified.
    pub callback: Option<dfg::CoreDef>,
    /// An optional definition of a `post-return` to use.
    pub post_return: Option<dfg::CoreDef>,
    /// Whether to use the async ABI for lifting or lowering.
    pub async_: bool,
    /// The core function type that is being lifted from / lowered to.
    pub core_type: ModuleInternedTypeIndex,
    /// The data model used by this adapter: linear memory or GC objects.
    pub data_model: DataModel,
}

impl<'data> Translator<'_, 'data> {
    /// This is the entrypoint of functionality within this module which
    /// performs all the work of identifying adapter usages and organizing
    /// everything into adapter modules.
    ///
    /// This will mutate the provided `component` in-place and fill out the dfg
    /// metadata for adapter modules.
    pub(super) fn partition_adapter_modules(&mut self, component: &mut dfg::ComponentDfg) {
        // Visit each adapter, in order of its original definition, during the
        // partitioning. This allows for the guarantee that dependencies are
        // visited in a topological fashion ideally.
        let mut state = PartitionAdapterModules::default();
        for (id, adapter) in component.adapters.iter() {
            state.adapter(component, id, adapter);
        }
        state.finish_adapter_module();

        // Now that all adapters have been partitioned into modules this loop
        // generates a core wasm module for each adapter module, translates
        // the module using standard core wasm translation, and then fills out
        // the dfg metadata for each adapter.
        for (module_id, adapter_module) in state.adapter_modules.iter() {
            let mut module =
                fact::Module::new(self.types.types(), self.tunables.debug_adapter_modules);
            let mut names = Vec::with_capacity(adapter_module.adapters.len());
            for adapter in adapter_module.adapters.iter() {
                let name = format!("adapter{}", adapter.as_u32());
                module.adapt(&name, &component.adapters[*adapter]);
                names.push(name);
            }
            let wasm = module.encode();
            let imports = module.imports().to_vec();

            // Extend the lifetime of the owned `wasm: Vec<u8>` on the stack to
            // a higher scope defined by our original caller. That allows to
            // transform `wasm` into `&'data [u8]` which is much easier to work
            // with here.
            let wasm = &*self.scope_vec.push(wasm);
            if log::log_enabled!(log::Level::Trace) {
                match wasmprinter::print_bytes(wasm) {
                    Ok(s) => log::trace!("generated adapter module:\n{s}"),
                    Err(e) => log::trace!("failed to print adapter module: {e}"),
                }
            }

            // With the wasm binary this is then pushed through general
            // translation, validation, etc. Note that multi-memory is
            // specifically enabled here since the adapter module is highly
            // likely to use that if anything is actually indirected through
            // memory.
            self.validator.reset();
            let translation = ModuleEnvironment::new(
                self.tunables,
                &mut self.validator,
                self.types.module_types_builder(),
            )
            .translate(Parser::new(0), wasm)
            .expect("invalid adapter module generated");

            // Record, for each adapter in this adapter module, the module that
            // the adapter was placed within as well as the function index of
            // the adapter in the wasm module generated. Note that adapters are
            // paritioned in-order so we're guaranteed to push the adapters
            // in-order here as well. (with an assert to double-check)
            for (adapter, name) in adapter_module.adapters.iter().zip(&names) {
                let index = translation.module.exports[name];
                let i = component.adapter_partitionings.push((module_id, index));
                assert_eq!(i, *adapter);
            }

            // Finally the metadata necessary to instantiate this adapter
            // module is also recorded in the dfg. This metadata will be used
            // to generate `GlobalInitializer` entries during the linearization
            // final phase.
            assert_eq!(imports.len(), translation.module.imports().len());
            let args = imports
                .iter()
                .zip(translation.module.imports())
                .map(|(arg, (_, _, ty))| fact_import_to_core_def(component, arg, ty))
                .collect::<Vec<_>>();
            let static_index = self.static_modules.push(translation);
            let id = component.adapter_modules.push((static_index, args));
            assert_eq!(id, module_id);
        }
    }
}

fn fact_import_to_core_def(
    dfg: &mut dfg::ComponentDfg,
    import: &fact::Import,
    ty: EntityType,
) -> dfg::CoreDef {
    fn unwrap_memory(def: &dfg::CoreDef) -> dfg::CoreExport<MemoryIndex> {
        match def {
            dfg::CoreDef::Export(e) => e.clone().map_index(|i| match i {
                EntityIndex::Memory(i) => i,
                _ => unreachable!(),
            }),
            _ => unreachable!(),
        }
    }

    let mut simple_intrinsic = |trampoline: dfg::Trampoline| {
        let signature = ty.unwrap_func();
        let index = dfg
            .trampolines
            .push((signature.unwrap_module_type_index(), trampoline));
        dfg::CoreDef::Trampoline(index)
    };
    match import {
        fact::Import::CoreDef(def) => def.clone(),
        fact::Import::Transcode {
            op,
            from,
            from64,
            to,
            to64,
        } => {
            let from = dfg.memories.push(unwrap_memory(from));
            let to = dfg.memories.push(unwrap_memory(to));
            let signature = ty.unwrap_func();
            let index = dfg.trampolines.push((
                signature.unwrap_module_type_index(),
                dfg::Trampoline::Transcoder {
                    op: *op,
                    from,
                    from64: *from64,
                    to,
                    to64: *to64,
                },
            ));
            dfg::CoreDef::Trampoline(index)
        }
        fact::Import::ResourceTransferOwn => simple_intrinsic(dfg::Trampoline::ResourceTransferOwn),
        fact::Import::ResourceTransferBorrow => {
            simple_intrinsic(dfg::Trampoline::ResourceTransferBorrow)
        }
        fact::Import::ResourceEnterCall => simple_intrinsic(dfg::Trampoline::ResourceEnterCall),
        fact::Import::ResourceExitCall => simple_intrinsic(dfg::Trampoline::ResourceExitCall),
        fact::Import::PrepareCall { memory } => simple_intrinsic(dfg::Trampoline::PrepareCall {
            memory: memory.as_ref().map(|v| dfg.memories.push(unwrap_memory(v))),
        }),
        fact::Import::SyncStartCall { callback } => {
            simple_intrinsic(dfg::Trampoline::SyncStartCall {
                callback: callback.clone().map(|v| dfg.callbacks.push(v)),
            })
        }
        fact::Import::AsyncStartCall {
            callback,
            post_return,
        } => simple_intrinsic(dfg::Trampoline::AsyncStartCall {
            callback: callback.clone().map(|v| dfg.callbacks.push(v)),
            post_return: post_return.clone().map(|v| dfg.post_returns.push(v)),
        }),
        fact::Import::FutureTransfer => simple_intrinsic(dfg::Trampoline::FutureTransfer),
        fact::Import::StreamTransfer => simple_intrinsic(dfg::Trampoline::StreamTransfer),
        fact::Import::ErrorContextTransfer => {
            simple_intrinsic(dfg::Trampoline::ErrorContextTransfer)
        }
    }
}

#[derive(Default)]
struct PartitionAdapterModules {
    /// The next adapter module that's being created. This may be empty.
    next_module: AdapterModuleInProgress,

    /// The set of items which are known to be defined which the adapter module
    /// in progress is allowed to depend on.
    defined_items: HashSet<Def>,

    /// Finished adapter modules that won't be added to.
    ///
    /// In theory items could be added to preexisting modules here but to keep
    /// this pass linear this is never modified after insertion.
    adapter_modules: PrimaryMap<dfg::AdapterModuleId, AdapterModuleInProgress>,
}

#[derive(Default)]
struct AdapterModuleInProgress {
    /// The adapters which have been placed into this module.
    adapters: Vec<dfg::AdapterId>,
}

/// Items that adapters can depend on.
///
/// Note that this is somewhat of a flat list and is intended to mostly model
/// core wasm instances which are side-effectful unlike other host items like
/// lowerings or always-trapping functions.
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Def {
    Adapter(dfg::AdapterId),
    Instance(dfg::InstanceId),
}

impl PartitionAdapterModules {
    fn adapter(&mut self, dfg: &dfg::ComponentDfg, id: dfg::AdapterId, adapter: &Adapter) {
        // Visit all dependencies of this adapter and if anything depends on
        // the current adapter module in progress then a new adapter module is
        // started.
        self.adapter_options(dfg, &adapter.lift_options);
        self.adapter_options(dfg, &adapter.lower_options);
        self.core_def(dfg, &adapter.func);

        // With all dependencies visited this adapter is added to the next
        // module.
        //
        // This will either get added the preexisting module if this adapter
        // didn't depend on anything in that module itself or it will be added
        // to a fresh module if this adapter depended on something that the
        // current adapter module created.
        log::debug!("adding {id:?} to adapter module");
        self.next_module.adapters.push(id);
    }

    fn adapter_options(&mut self, dfg: &dfg::ComponentDfg, options: &AdapterOptions) {
        if let Some(def) = &options.callback {
            self.core_def(dfg, def);
        }
        if let Some(def) = &options.post_return {
            self.core_def(dfg, def);
        }
        match &options.data_model {
            DataModel::Gc {} => {
                // Nothing to do here yet.
            }
            DataModel::LinearMemory {
                memory,
                memory64: _,
                realloc,
            } => {
                if let Some(memory) = memory {
                    self.core_export(dfg, memory);
                }
                if let Some(def) = realloc {
                    self.core_def(dfg, def);
                }
            }
        }
    }

    fn core_def(&mut self, dfg: &dfg::ComponentDfg, def: &dfg::CoreDef) {
        match def {
            dfg::CoreDef::Export(e) => self.core_export(dfg, e),
            dfg::CoreDef::Adapter(id) => {
                // If this adapter is already defined then we can safely depend
                // on it with no consequences.
                if self.defined_items.contains(&Def::Adapter(*id)) {
                    log::debug!("using existing adapter {id:?} ");
                    return;
                }

                log::debug!("splitting module needing {id:?} ");

                // .. otherwise we found a case of an adapter depending on an
                // adapter-module-in-progress meaning that the current adapter
                // module must be completed and then a new one is started.
                self.finish_adapter_module();
                assert!(self.defined_items.contains(&Def::Adapter(*id)));
            }

            // These items can't transitively depend on an adapter
            dfg::CoreDef::Trampoline(_) | dfg::CoreDef::InstanceFlags(_) => {}
        }
    }

    fn core_export<T>(&mut self, dfg: &dfg::ComponentDfg, export: &dfg::CoreExport<T>) {
        // When an adapter depends on an exported item it actually depends on
        // the instance of that exported item. The caveat here is that the
        // adapter not only depends on that particular instance, but also all
        // prior instances to that instance as well because instance
        // instantiation order is fixed and cannot change.
        //
        // To model this the instance index space is looped over here and while
        // an instance hasn't been visited it's visited. Note that if an
        // instance has already been visited then all prior instances have
        // already been visited so there's no need to continue.
        let mut instance = export.instance;
        while self.defined_items.insert(Def::Instance(instance)) {
            self.instance(dfg, instance);
            if instance.as_u32() == 0 {
                break;
            }
            instance = dfg::InstanceId::from_u32(instance.as_u32() - 1);
        }
    }

    fn instance(&mut self, dfg: &dfg::ComponentDfg, instance: dfg::InstanceId) {
        log::debug!("visiting instance {instance:?}");

        // ... otherwise if this is the first timet he instance has been seen
        // then the instances own arguments are recursively visited to find
        // transitive dependencies on adapters.
        match &dfg.instances[instance] {
            dfg::Instance::Static(_, args) => {
                for arg in args.iter() {
                    self.core_def(dfg, arg);
                }
            }
            dfg::Instance::Import(_, args) => {
                for (_, values) in args {
                    for (_, def) in values {
                        self.core_def(dfg, def);
                    }
                }
            }
        }
    }

    fn finish_adapter_module(&mut self) {
        if self.next_module.adapters.is_empty() {
            return;
        }

        // Reset the state of the current module-in-progress and then flag all
        // pending adapters as now defined since the current module is being
        // committed.
        let module = mem::take(&mut self.next_module);
        for adapter in module.adapters.iter() {
            let inserted = self.defined_items.insert(Def::Adapter(*adapter));
            assert!(inserted);
        }
        let idx = self.adapter_modules.push(module);
        log::debug!("finishing adapter module {idx:?}");
    }
}

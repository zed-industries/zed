//! Implementation of "inlining" a component into a flat list of initializers.
//!
//! After the first phase of compiling a component we're left with a single
//! root `Translation` for the original component along with a "static" list of
//! child components. Each `Translation` has a list of `LocalInitializer` items
//! inside of it which is a primitive representation of how the component
//! should be constructed with effectively one initializer per item in the
//! index space of a component. This "local initializer" list would be
//! relatively inefficient to process at runtime and more importantly doesn't
//! convey enough information to understand what trampolines need to be
//! compiled or what fused adapters need to be generated. This consequently is
//! the motivation for this file.
//!
//! The second phase of compilation, inlining here, will in a sense interpret
//! the initializers, at compile time, into a new list of `GlobalInitializer` entries
//! which are a sort of "global initializer". The generated `GlobalInitializer` is
//! much more specific than the `LocalInitializer` and additionally far fewer
//! `GlobalInitializer` structures are generated (in theory) than there are local
//! initializers.
//!
//! The "inlining" portion of the name of this module indicates how the
//! instantiation of a component is interpreted as calling a function. The
//! function's arguments are the imports provided to the instantiation of a
//! component, and further nested function calls happen on a stack when a
//! nested component is instantiated. The inlining then refers to how this
//! stack of instantiations is flattened to one list of `GlobalInitializer`
//! entries to represent the process of instantiating a component graph,
//! similar to how function inlining removes call instructions and creates one
//! giant function for a call graph. Here there are no inlining heuristics or
//! anything like that, we simply inline everything into the root component's
//! list of initializers.
//!
//! Another primary task this module performs is a form of dataflow analysis
//! to represent items in each index space with their definition rather than
//! references of relative indices. These definitions (all the `*Def` types in
//! this module) are not local to any one nested component and instead
//! represent state available at runtime tracked in the final `Component`
//! produced.
//!
//! With all this pieced together the general idea is relatively
//! straightforward. All of a component's initializers are processed in sequence
//! where instantiating a nested component pushes a "frame" onto a stack to
//! start executing and we resume at the old one when we're done. Items are
//! tracked where they come from and at the end after processing only the
//! side-effectful initializers are emitted to the `GlobalInitializer` list in the
//! final `Component`.

use crate::component::translate::*;
use crate::{EntityType, IndexType};
use std::borrow::Cow;
use wasmparser::component_types::{ComponentAnyTypeId, ComponentCoreModuleTypeId};

pub(super) fn run(
    types: &mut ComponentTypesBuilder,
    result: &Translation<'_>,
    nested_modules: &PrimaryMap<StaticModuleIndex, ModuleTranslation<'_>>,
    nested_components: &PrimaryMap<StaticComponentIndex, Translation<'_>>,
) -> Result<dfg::ComponentDfg> {
    let mut inliner = Inliner {
        nested_modules,
        nested_components,
        result: Default::default(),
        import_path_interner: Default::default(),
        runtime_instances: PrimaryMap::default(),
    };

    let index = RuntimeComponentInstanceIndex::from_u32(0);

    // The initial arguments to the root component are all host imports. This
    // means that they're all using the `ComponentItemDef::Host` variant. Here
    // an `ImportIndex` is allocated for each item and then the argument is
    // recorded.
    //
    // Note that this is represents the abstract state of a host import of an
    // item since we don't know the precise structure of the host import.
    let mut args = HashMap::with_capacity(result.exports.len());
    let mut path = Vec::new();
    types.resources_mut().set_current_instance(index);
    let types_ref = result.types_ref();
    for init in result.initializers.iter() {
        let (name, ty) = match *init {
            LocalInitializer::Import(name, ty) => (name, ty),
            _ => continue,
        };

        // Before `convert_component_entity_type` below all resource types
        // introduced by this import need to be registered and have indexes
        // assigned to them. Any fresh new resource type referred to by imports
        // is a brand new introduction of a resource which needs to have a type
        // allocated to it, so new runtime imports are injected for each
        // resource along with updating the `imported_resources` map.
        let index = inliner.result.import_types.next_key();
        types.resources_mut().register_component_entity_type(
            &types_ref,
            ty,
            &mut path,
            &mut |path| {
                let index = inliner.runtime_import(&ImportPath {
                    index,
                    path: path.iter().copied().map(Into::into).collect(),
                });
                inliner.result.imported_resources.push(index)
            },
        );

        // With resources all taken care of it's now possible to convert this
        // into Wasmtime's type system.
        let ty = types.convert_component_entity_type(types_ref, ty)?;

        // Imports of types that aren't resources are not required to be
        // specified by the host since it's just for type information within
        // the component.
        if let TypeDef::Interface(_) = ty {
            continue;
        }
        let index = inliner.result.import_types.push((name.0.to_string(), ty));
        let path = ImportPath::root(index);
        args.insert(name.0, ComponentItemDef::from_import(path, ty)?);
    }

    // This will run the inliner to completion after being seeded with the
    // initial frame. When the inliner finishes it will return the exports of
    // the root frame which are then used for recording the exports of the
    // component.
    inliner.result.num_runtime_component_instances += 1;
    let frame = InlinerFrame::new(index, result, ComponentClosure::default(), args, None);
    let resources_snapshot = types.resources_mut().clone();
    let mut frames = vec![(frame, resources_snapshot)];
    let exports = inliner.run(types, &mut frames)?;
    assert!(frames.is_empty());

    let mut export_map = Default::default();
    for (name, def) in exports {
        inliner.record_export(name, def, types, &mut export_map)?;
    }
    inliner.result.exports = export_map;
    inliner.result.num_future_tables = types.num_future_tables();
    inliner.result.num_stream_tables = types.num_stream_tables();
    inliner.result.num_error_context_tables = types.num_error_context_tables();

    Ok(inliner.result)
}

struct Inliner<'a> {
    /// The list of static modules that were found during initial translation of
    /// the component.
    ///
    /// This is used during the instantiation of these modules to ahead-of-time
    /// order the arguments precisely according to what the module is defined as
    /// needing which avoids the need to do string lookups or permute arguments
    /// at runtime.
    nested_modules: &'a PrimaryMap<StaticModuleIndex, ModuleTranslation<'a>>,

    /// The list of static components that were found during initial translation of
    /// the component.
    ///
    /// This is used when instantiating nested components to push a new
    /// `InlinerFrame` with the `Translation`s here.
    nested_components: &'a PrimaryMap<StaticComponentIndex, Translation<'a>>,

    /// The final `Component` that is being constructed and returned from this
    /// inliner.
    result: dfg::ComponentDfg,

    // Maps used to "intern" various runtime items to only save them once at
    // runtime instead of multiple times.
    import_path_interner: HashMap<ImportPath<'a>, RuntimeImportIndex>,

    /// Origin information about where each runtime instance came from
    runtime_instances: PrimaryMap<dfg::InstanceId, InstanceModule>,
}

/// A "stack frame" as part of the inlining process, or the progress through
/// instantiating a component.
///
/// All instantiations of a component will create an `InlinerFrame` and are
/// incrementally processed via the `initializers` list here. Note that the
/// inliner frames are stored on the heap to avoid recursion based on user
/// input.
struct InlinerFrame<'a> {
    instance: RuntimeComponentInstanceIndex,

    /// The remaining initializers to process when instantiating this component.
    initializers: std::slice::Iter<'a, LocalInitializer<'a>>,

    /// The component being instantiated.
    translation: &'a Translation<'a>,

    /// The "closure arguments" to this component, or otherwise the maps indexed
    /// by `ModuleUpvarIndex` and `ComponentUpvarIndex`. This is created when
    /// a component is created and stored as part of a component's state during
    /// inlining.
    closure: ComponentClosure<'a>,

    /// The arguments to the creation of this component.
    ///
    /// At the root level these are all imports from the host and between
    /// components this otherwise tracks how all the arguments are defined.
    args: HashMap<&'a str, ComponentItemDef<'a>>,

    // core wasm index spaces
    funcs: PrimaryMap<FuncIndex, (ModuleInternedTypeIndex, dfg::CoreDef)>,
    memories: PrimaryMap<MemoryIndex, dfg::CoreExport<EntityIndex>>,
    tables: PrimaryMap<TableIndex, dfg::CoreExport<EntityIndex>>,
    globals: PrimaryMap<GlobalIndex, dfg::CoreExport<EntityIndex>>,
    tags: PrimaryMap<GlobalIndex, dfg::CoreExport<EntityIndex>>,
    modules: PrimaryMap<ModuleIndex, ModuleDef<'a>>,

    // component model index spaces
    component_funcs: PrimaryMap<ComponentFuncIndex, ComponentFuncDef<'a>>,
    module_instances: PrimaryMap<ModuleInstanceIndex, ModuleInstanceDef<'a>>,
    component_instances: PrimaryMap<ComponentInstanceIndex, ComponentInstanceDef<'a>>,
    components: PrimaryMap<ComponentIndex, ComponentDef<'a>>,

    /// The type of instance produced by completing the instantiation of this
    /// frame.
    ///
    /// This is a wasmparser-relative piece of type information which is used to
    /// register resource types after instantiation has completed.
    ///
    /// This is `Some` for all subcomponents and `None` for the root component.
    instance_ty: Option<ComponentInstanceTypeId>,
}

/// "Closure state" for a component which is resolved from the `ClosedOverVars`
/// state that was calculated during translation.
//
// FIXME: this is cloned quite a lot and given the internal maps if this is a
// perf issue we may want to `Rc` these fields. Note that this is only a perf
// hit at compile-time though which we in general don't pay too much
// attention to.
#[derive(Default, Clone)]
struct ComponentClosure<'a> {
    modules: PrimaryMap<ModuleUpvarIndex, ModuleDef<'a>>,
    components: PrimaryMap<ComponentUpvarIndex, ComponentDef<'a>>,
}

/// Representation of a "path" into an import.
///
/// Imports from the host at this time are one of three things:
///
/// * Functions
/// * Core wasm modules
/// * "Instances" of these three items
///
/// The "base" values are functions and core wasm modules, but the abstraction
/// of an instance allows embedding functions/modules deeply within other
/// instances. This "path" represents optionally walking through a host instance
/// to get to the final desired item. At runtime instances are just maps of
/// values and so this is used to ensure that we primarily only deal with
/// individual functions and modules instead of synthetic instances.
#[derive(Clone, PartialEq, Hash, Eq)]
struct ImportPath<'a> {
    index: ImportIndex,
    path: Vec<Cow<'a, str>>,
}

/// Representation of all items which can be defined within a component.
///
/// This is the "value" of an item defined within a component and is used to
/// represent both imports and exports.
#[derive(Clone)]
enum ComponentItemDef<'a> {
    Component(ComponentDef<'a>),
    Instance(ComponentInstanceDef<'a>),
    Func(ComponentFuncDef<'a>),
    Module(ModuleDef<'a>),
    Type(TypeDef),
}

#[derive(Clone)]
enum ModuleDef<'a> {
    /// A core wasm module statically defined within the original component.
    ///
    /// The `StaticModuleIndex` indexes into the `static_modules` map in the
    /// `Inliner`.
    Static(StaticModuleIndex, ComponentCoreModuleTypeId),

    /// A core wasm module that was imported from the host.
    Import(ImportPath<'a>, TypeModuleIndex),
}

// Note that unlike all other `*Def` types which are not allowed to have local
// indices this type does indeed have local indices. That is represented with
// the lack of a `Clone` here where once this is created it's never moved across
// components because module instances always stick within one component.
enum ModuleInstanceDef<'a> {
    /// A core wasm module instance was created through the instantiation of a
    /// module.
    ///
    /// The `RuntimeInstanceIndex` was the index allocated as this was the
    /// `n`th instantiation and the `ModuleIndex` points into an
    /// `InlinerFrame`'s local index space.
    Instantiated(dfg::InstanceId, ModuleIndex),

    /// A "synthetic" core wasm module which is just a bag of named indices.
    ///
    /// Note that this can really only be used for passing as an argument to
    /// another module's instantiation and is used to rename arguments locally.
    Synthetic(&'a HashMap<&'a str, EntityIndex>),
}

#[derive(Clone)]
enum ComponentFuncDef<'a> {
    /// A host-imported component function.
    Import(ImportPath<'a>),

    /// A core wasm function was lifted into a component function.
    Lifted {
        /// The component function type.
        ty: TypeFuncIndex,
        /// The core Wasm function.
        func: dfg::CoreDef,
        /// Canonical options.
        options: AdapterOptions,
    },
}

#[derive(Clone)]
enum ComponentInstanceDef<'a> {
    /// A host-imported instance.
    ///
    /// This typically means that it's "just" a map of named values. It's not
    /// actually supported to take a `wasmtime::component::Instance` and pass it
    /// to another instance at this time.
    Import(ImportPath<'a>, TypeComponentInstanceIndex),

    /// A concrete map of values.
    ///
    /// This is used for both instantiated components as well as "synthetic"
    /// components. This variant can be used for both because both are
    /// represented by simply a bag of items within the entire component
    /// instantiation process.
    //
    // FIXME: same as the issue on `ComponentClosure` where this is cloned a lot
    // and may need `Rc`.
    Items(
        IndexMap<&'a str, ComponentItemDef<'a>>,
        TypeComponentInstanceIndex,
    ),
}

#[derive(Clone)]
struct ComponentDef<'a> {
    index: StaticComponentIndex,
    closure: ComponentClosure<'a>,
}

impl<'a> Inliner<'a> {
    /// Symbolically instantiates a component using the type information and
    /// `frames` provided.
    ///
    /// The `types` provided is the type information for the entire component
    /// translation process. This is a distinct output artifact separate from
    /// the component metadata.
    ///
    /// The `frames` argument is storage to handle a "call stack" of components
    /// instantiating one another. The youngest frame (last element) of the
    /// frames list is a component that's currently having its initializers
    /// processed. The second element of each frame is a snapshot of the
    /// resource-related information just before the frame was translated. For
    /// more information on this snapshotting see the documentation on
    /// `ResourcesBuilder`.
    fn run(
        &mut self,
        types: &mut ComponentTypesBuilder,
        frames: &mut Vec<(InlinerFrame<'a>, ResourcesBuilder)>,
    ) -> Result<IndexMap<&'a str, ComponentItemDef<'a>>> {
        // This loop represents the execution of the instantiation of a
        // component. This is an iterative process which is finished once all
        // initializers are processed. Currently this is modeled as an infinite
        // loop which drives the top-most iterator of the `frames` stack
        // provided as an argument to this function.
        loop {
            let (frame, _) = frames.last_mut().unwrap();
            types.resources_mut().set_current_instance(frame.instance);
            match frame.initializers.next() {
                // Process the initializer and if it started the instantiation
                // of another component then we push that frame on the stack to
                // continue onwards.
                Some(init) => match self.initializer(frame, types, init)? {
                    Some(new_frame) => {
                        frames.push((new_frame, types.resources_mut().clone()));
                    }
                    None => {}
                },

                // If there are no more initializers for this frame then the
                // component it represents has finished instantiation. The
                // exports of the component are collected and then the entire
                // frame is discarded. The exports are then either pushed in the
                // parent frame, if any, as a new component instance or they're
                // returned from this function for the root set of exports.
                None => {
                    let exports = frame
                        .translation
                        .exports
                        .iter()
                        .map(|(name, item)| Ok((*name, frame.item(*item, types)?)))
                        .collect::<Result<_>>()?;
                    let instance_ty = frame.instance_ty;
                    let (_, snapshot) = frames.pop().unwrap();
                    *types.resources_mut() = snapshot;
                    match frames.last_mut() {
                        Some((parent, _)) => {
                            parent.finish_instantiate(exports, instance_ty.unwrap(), types)?;
                        }
                        None => break Ok(exports),
                    }
                }
            }
        }
    }

    fn initializer(
        &mut self,
        frame: &mut InlinerFrame<'a>,
        types: &mut ComponentTypesBuilder,
        initializer: &'a LocalInitializer,
    ) -> Result<Option<InlinerFrame<'a>>> {
        use LocalInitializer::*;

        match initializer {
            // When a component imports an item the actual definition of the
            // item is looked up here (not at runtime) via its name. The
            // arguments provided in our `InlinerFrame` describe how each
            // argument was defined, so we simply move it from there into the
            // correct index space.
            //
            // Note that for the root component this will add `*::Import` items
            // but for sub-components this will do resolution to connect what
            // was provided as an import at the instantiation-site to what was
            // needed during the component's instantiation.
            Import(name, ty) => {
                let arg = match frame.args.get(name.0) {
                    Some(arg) => arg,

                    // Not all arguments need to be provided for instantiation,
                    // namely the root component in Wasmtime doesn't require
                    // structural type imports to be satisfied. These type
                    // imports are relevant for bindings generators and such but
                    // as a runtime there's not really a definition to fit in.
                    //
                    // If no argument was provided for `name` then it's asserted
                    // that this is a type import and additionally it's not a
                    // resource type import (which indeed must be provided). If
                    // all that passes then this initializer is effectively
                    // skipped.
                    None => {
                        match ty {
                            ComponentEntityType::Type {
                                created: ComponentAnyTypeId::Resource(_),
                                ..
                            } => unreachable!(),
                            ComponentEntityType::Type { .. } => {}
                            _ => unreachable!(),
                        }
                        return Ok(None);
                    }
                };

                // Next resource types need to be handled. For example if a
                // resource is imported into this component then it needs to be
                // assigned a unique table to provide the isolation guarantees
                // of resources (this component's table is shared with no
                // others). Here `register_component_entity_type` will find
                // imported resources and then `lookup_resource` will find the
                // resource within `arg` as necessary to lookup the original
                // true definition of this resource.
                //
                // This is what enables tracking true resource origins
                // throughout component translation while simultaneously also
                // tracking unique tables for each resource in each component.
                let mut path = Vec::new();
                let (resources, types) = types.resources_mut_and_types();
                resources.register_component_entity_type(
                    &frame.translation.types_ref(),
                    *ty,
                    &mut path,
                    &mut |path| arg.lookup_resource(path, types),
                );

                // And now with all the type information out of the way the
                // `arg` definition is moved into its corresponding index space.
                frame.push_item(arg.clone());
            }

            // Lowering a component function to a core wasm function is
            // generally what "triggers compilation". Here various metadata is
            // recorded and then the final component gets an initializer
            // recording the lowering.
            //
            // NB: at this time only lowered imported functions are supported.
            Lower {
                func,
                options,
                lower_ty,
            } => {
                let lower_ty =
                    types.convert_component_func_type(frame.translation.types_ref(), *lower_ty)?;
                let options_lower = self.adapter_options(frame, types, options);
                let lower_core_type = options_lower.core_type;
                let func = match &frame.component_funcs[*func] {
                    // If this component function was originally a host import
                    // then this is a lowered host function which needs a
                    // trampoline to enter WebAssembly. That's recorded here
                    // with all relevant information.
                    ComponentFuncDef::Import(path) => {
                        let import = self.runtime_import(path);
                        let options = self.canonical_options(options_lower);
                        let index = self.result.trampolines.push((
                            lower_core_type,
                            dfg::Trampoline::LowerImport {
                                import,
                                options,
                                lower_ty,
                            },
                        ));
                        dfg::CoreDef::Trampoline(index)
                    }

                    // This case handles when a lifted function is later
                    // lowered, and both the lowering and the lifting are
                    // happening within the same component instance.
                    //
                    // In this situation if the `canon.lower`'d function is
                    // called then it immediately sets `may_enter` to `false`.
                    // When calling the callee, however, that's `canon.lift`
                    // which immediately traps if `may_enter` is `false`. That
                    // means that this pairing of functions creates a function
                    // that always traps.
                    //
                    // When closely reading the spec though the precise trap
                    // that comes out can be somewhat variable. Technically the
                    // function yielded here is one that should validate the
                    // arguments by lifting them, and then trap. This means that
                    // the trap could be different depending on whether all
                    // arguments are valid for now. This was discussed in
                    // WebAssembly/component-model#51 somewhat and the
                    // conclusion was that we can probably get away with "always
                    // trap" here.
                    //
                    // The `CoreDef::AlwaysTrap` variant here is used to
                    // indicate that this function is valid but if something
                    // actually calls it then it just generates a trap
                    // immediately.
                    ComponentFuncDef::Lifted {
                        options: options_lift,
                        ..
                    } if options_lift.instance == options_lower.instance => {
                        let index = self
                            .result
                            .trampolines
                            .push((lower_core_type, dfg::Trampoline::AlwaysTrap));
                        dfg::CoreDef::Trampoline(index)
                    }

                    // Lowering a lifted function where the destination
                    // component is different than the source component means
                    // that a "fused adapter" was just identified.
                    //
                    // Metadata about this fused adapter is recorded in the
                    // `Adapters` output of this compilation pass. Currently the
                    // implementation of fused adapters is to generate a core
                    // wasm module which is instantiated with relevant imports
                    // and the exports are used as the fused adapters. At this
                    // time we don't know when precisely the instance will be
                    // created but we do know that the result of this will be an
                    // export from a previously-created instance.
                    //
                    // To model this the result of this arm is a
                    // `CoreDef::Export`. The actual indices listed within the
                    // export are "fake indices" in the sense of they're not
                    // resolved yet. This resolution will happen at a later
                    // compilation phase. Any usages of the `CoreDef::Export`
                    // here will be detected and rewritten to an actual runtime
                    // instance created.
                    //
                    // The `instance` field of the `CoreExport` has a marker
                    // which indicates that it's a fused adapter. The `item` is
                    // a function where the function index corresponds to the
                    // `adapter_idx` which contains the metadata about this
                    // adapter being created. The metadata is used to learn
                    // about the dependencies and when the adapter module can
                    // be instantiated.
                    ComponentFuncDef::Lifted {
                        ty: lift_ty,
                        func,
                        options: options_lift,
                    } => {
                        let adapter_idx = self.result.adapters.push(Adapter {
                            lift_ty: *lift_ty,
                            lift_options: options_lift.clone(),
                            lower_ty,
                            lower_options: options_lower,
                            func: func.clone(),
                        });
                        dfg::CoreDef::Adapter(adapter_idx)
                    }
                };
                frame.funcs.push((lower_core_type, func));
            }

            // Lifting a core wasm function is relatively easy for now in that
            // some metadata about the lifting is simply recorded. This'll get
            // plumbed through to exports or a fused adapter later on.
            Lift(ty, func, options) => {
                let ty = types.convert_component_func_type(frame.translation.types_ref(), *ty)?;
                let options = self.adapter_options(frame, types, options);
                let func = frame.funcs[*func].1.clone();
                frame
                    .component_funcs
                    .push(ComponentFuncDef::Lifted { ty, func, options });
            }

            // A new resource type is being introduced, so it's recorded as a
            // brand new resource in the final `resources` array. Additionally
            // for now resource introductions are considered side effects to
            // know when to register their destructors so that's recorded as
            // well.
            //
            // Note that this has the effect of when a component is instantiated
            // twice it will produce unique types for the resources from each
            // instantiation. That's the intended runtime semantics and
            // implementation here, however.
            Resource(ty, rep, dtor) => {
                let idx = self.result.resources.push(dfg::Resource {
                    rep: *rep,
                    dtor: dtor.map(|i| frame.funcs[i].1.clone()),
                    instance: frame.instance,
                });
                self.result
                    .side_effects
                    .push(dfg::SideEffect::Resource(idx));

                // Register with type translation that all future references to
                // `ty` will refer to `idx`.
                //
                // Note that this registration information is lost when this
                // component finishes instantiation due to the snapshotting
                // behavior in the frame processing loop above. This is also
                // intended, though, since `ty` can't be referred to outside of
                // this component.
                let idx = self.result.resource_index(idx);
                types.resources_mut().register_resource(ty.resource(), idx);
            }

            // Resource-related intrinsics are generally all the same.
            // Wasmparser type information is converted to wasmtime type
            // information and then new entries for each intrinsic are recorded.
            ResourceNew(id, ty) => {
                let id = types.resource_id(id.resource());
                let index = self
                    .result
                    .trampolines
                    .push((*ty, dfg::Trampoline::ResourceNew(id)));
                frame.funcs.push((*ty, dfg::CoreDef::Trampoline(index)));
            }
            ResourceRep(id, ty) => {
                let id = types.resource_id(id.resource());
                let index = self
                    .result
                    .trampolines
                    .push((*ty, dfg::Trampoline::ResourceRep(id)));
                frame.funcs.push((*ty, dfg::CoreDef::Trampoline(index)));
            }
            ResourceDrop(id, ty) => {
                let id = types.resource_id(id.resource());
                let index = self
                    .result
                    .trampolines
                    .push((*ty, dfg::Trampoline::ResourceDrop(id)));
                frame.funcs.push((*ty, dfg::CoreDef::Trampoline(index)));
            }
            BackpressureSet { func } => {
                let index = self.result.trampolines.push((
                    *func,
                    dfg::Trampoline::BackpressureSet {
                        instance: frame.instance,
                    },
                ));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            TaskReturn { result, options } => {
                let results = result
                    .iter()
                    .map(|ty| types.valtype(frame.translation.types_ref(), ty))
                    .collect::<Result<_>>()?;
                let results = types.new_tuple_type(results);
                let func = options.core_type;
                let options = self.adapter_options(frame, types, options);
                let options = self.canonical_options(options);
                let index = self
                    .result
                    .trampolines
                    .push((func, dfg::Trampoline::TaskReturn { results, options }));
                frame.funcs.push((func, dfg::CoreDef::Trampoline(index)));
            }
            TaskCancel { func } => {
                let index = self.result.trampolines.push((
                    *func,
                    dfg::Trampoline::TaskCancel {
                        instance: frame.instance,
                    },
                ));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            WaitableSetNew { func } => {
                let index = self.result.trampolines.push((
                    *func,
                    dfg::Trampoline::WaitableSetNew {
                        instance: frame.instance,
                    },
                ));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            WaitableSetWait { options } => {
                let func = options.core_type;
                let options = self.adapter_options(frame, types, options);
                let options = self.canonical_options(options);
                let index = self
                    .result
                    .trampolines
                    .push((func, dfg::Trampoline::WaitableSetWait { options }));
                frame.funcs.push((func, dfg::CoreDef::Trampoline(index)));
            }
            WaitableSetPoll { options } => {
                let func = options.core_type;
                let options = self.adapter_options(frame, types, options);
                let options = self.canonical_options(options);
                let index = self
                    .result
                    .trampolines
                    .push((func, dfg::Trampoline::WaitableSetPoll { options }));
                frame.funcs.push((func, dfg::CoreDef::Trampoline(index)));
            }
            WaitableSetDrop { func } => {
                let index = self.result.trampolines.push((
                    *func,
                    dfg::Trampoline::WaitableSetDrop {
                        instance: frame.instance,
                    },
                ));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            WaitableJoin { func } => {
                let index = self.result.trampolines.push((
                    *func,
                    dfg::Trampoline::WaitableJoin {
                        instance: frame.instance,
                    },
                ));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            Yield { func, async_ } => {
                let index = self
                    .result
                    .trampolines
                    .push((*func, dfg::Trampoline::Yield { async_: *async_ }));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            SubtaskDrop { func } => {
                let index = self.result.trampolines.push((
                    *func,
                    dfg::Trampoline::SubtaskDrop {
                        instance: frame.instance,
                    },
                ));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            SubtaskCancel { func, async_ } => {
                let index = self.result.trampolines.push((
                    *func,
                    dfg::Trampoline::SubtaskCancel {
                        instance: frame.instance,
                        async_: *async_,
                    },
                ));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            StreamNew { ty, func } => {
                let InterfaceType::Stream(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let index = self
                    .result
                    .trampolines
                    .push((*func, dfg::Trampoline::StreamNew { ty }));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            StreamRead { ty, options } => {
                let InterfaceType::Stream(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let func = options.core_type;
                let options = self.adapter_options(frame, types, options);
                let options = self.canonical_options(options);
                let index = self
                    .result
                    .trampolines
                    .push((func, dfg::Trampoline::StreamRead { ty, options }));
                frame.funcs.push((func, dfg::CoreDef::Trampoline(index)));
            }
            StreamWrite { ty, options } => {
                let InterfaceType::Stream(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let func = options.core_type;
                let options = self.adapter_options(frame, types, options);
                let options = self.canonical_options(options);
                let index = self
                    .result
                    .trampolines
                    .push((func, dfg::Trampoline::StreamWrite { ty, options }));
                frame.funcs.push((func, dfg::CoreDef::Trampoline(index)));
            }
            StreamCancelRead { ty, func, async_ } => {
                let InterfaceType::Stream(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let index = self.result.trampolines.push((
                    *func,
                    dfg::Trampoline::StreamCancelRead {
                        ty,
                        async_: *async_,
                    },
                ));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            StreamCancelWrite { ty, func, async_ } => {
                let InterfaceType::Stream(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let index = self.result.trampolines.push((
                    *func,
                    dfg::Trampoline::StreamCancelWrite {
                        ty,
                        async_: *async_,
                    },
                ));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            StreamDropReadable { ty, func } => {
                let InterfaceType::Stream(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let index = self
                    .result
                    .trampolines
                    .push((*func, dfg::Trampoline::StreamDropReadable { ty }));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            StreamDropWritable { ty, func } => {
                let InterfaceType::Stream(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let index = self
                    .result
                    .trampolines
                    .push((*func, dfg::Trampoline::StreamDropWritable { ty }));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            FutureNew { ty, func } => {
                let InterfaceType::Future(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let index = self
                    .result
                    .trampolines
                    .push((*func, dfg::Trampoline::FutureNew { ty }));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            FutureRead { ty, options } => {
                let InterfaceType::Future(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let func = options.core_type;
                let options = self.adapter_options(frame, types, options);
                let options = self.canonical_options(options);
                let index = self
                    .result
                    .trampolines
                    .push((func, dfg::Trampoline::FutureRead { ty, options }));
                frame.funcs.push((func, dfg::CoreDef::Trampoline(index)));
            }
            FutureWrite { ty, options } => {
                let InterfaceType::Future(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let func = options.core_type;
                let options = self.adapter_options(frame, types, options);
                let options = self.canonical_options(options);
                let index = self
                    .result
                    .trampolines
                    .push((func, dfg::Trampoline::FutureWrite { ty, options }));
                frame.funcs.push((func, dfg::CoreDef::Trampoline(index)));
            }
            FutureCancelRead { ty, func, async_ } => {
                let InterfaceType::Future(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let index = self.result.trampolines.push((
                    *func,
                    dfg::Trampoline::FutureCancelRead {
                        ty,
                        async_: *async_,
                    },
                ));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            FutureCancelWrite { ty, func, async_ } => {
                let InterfaceType::Future(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let index = self.result.trampolines.push((
                    *func,
                    dfg::Trampoline::FutureCancelWrite {
                        ty,
                        async_: *async_,
                    },
                ));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            FutureDropReadable { ty, func } => {
                let InterfaceType::Future(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let index = self
                    .result
                    .trampolines
                    .push((*func, dfg::Trampoline::FutureDropReadable { ty }));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            FutureDropWritable { ty, func } => {
                let InterfaceType::Future(ty) =
                    types.defined_type(frame.translation.types_ref(), *ty)?
                else {
                    unreachable!()
                };
                let index = self
                    .result
                    .trampolines
                    .push((*func, dfg::Trampoline::FutureDropWritable { ty }));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            ErrorContextNew { options } => {
                let ty = types.error_context_table_type()?;
                let func = options.core_type;
                let options = self.adapter_options(frame, types, options);
                let options = self.canonical_options(options);
                let index = self
                    .result
                    .trampolines
                    .push((func, dfg::Trampoline::ErrorContextNew { ty, options }));
                frame.funcs.push((func, dfg::CoreDef::Trampoline(index)));
            }
            ErrorContextDebugMessage { options } => {
                let ty = types.error_context_table_type()?;
                let func = options.core_type;
                let options = self.adapter_options(frame, types, options);
                let options = self.canonical_options(options);
                let index = self.result.trampolines.push((
                    func,
                    dfg::Trampoline::ErrorContextDebugMessage { ty, options },
                ));
                frame.funcs.push((func, dfg::CoreDef::Trampoline(index)));
            }
            ErrorContextDrop { func } => {
                let ty = types.error_context_table_type()?;
                let index = self
                    .result
                    .trampolines
                    .push((*func, dfg::Trampoline::ErrorContextDrop { ty }));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            ContextGet { func, i } => {
                let index = self
                    .result
                    .trampolines
                    .push((*func, dfg::Trampoline::ContextGet(*i)));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }
            ContextSet { func, i } => {
                let index = self
                    .result
                    .trampolines
                    .push((*func, dfg::Trampoline::ContextSet(*i)));
                frame.funcs.push((*func, dfg::CoreDef::Trampoline(index)));
            }

            ModuleStatic(idx, ty) => {
                frame.modules.push(ModuleDef::Static(*idx, *ty));
            }

            // Instantiation of a module is one of the meatier initializers that
            // we'll generate. The main magic here is that for a statically
            // known module we can order the imports as a list to exactly what
            // the static module needs to be instantiated. For imported modules,
            // however, the runtime string resolution must happen at runtime so
            // that is deferred here by organizing the arguments as a two-layer
            // `IndexMap` of what we're providing.
            //
            // In both cases though a new `RuntimeInstanceIndex` is allocated
            // and an initializer is recorded to indicate that it's being
            // instantiated.
            ModuleInstantiate(module, args) => {
                let (instance_module, init) = match &frame.modules[*module] {
                    ModuleDef::Static(idx, _ty) => {
                        let mut defs = Vec::new();
                        for (module, name, _ty) in self.nested_modules[*idx].module.imports() {
                            let instance = args[module];
                            defs.push(
                                self.core_def_of_module_instance_export(frame, instance, name),
                            );
                        }
                        (
                            InstanceModule::Static(*idx),
                            dfg::Instance::Static(*idx, defs.into()),
                        )
                    }
                    ModuleDef::Import(path, ty) => {
                        let mut defs = IndexMap::new();
                        for ((module, name), _) in types[*ty].imports.iter() {
                            let instance = args[module.as_str()];
                            let def =
                                self.core_def_of_module_instance_export(frame, instance, name);
                            defs.entry(module.to_string())
                                .or_insert(IndexMap::new())
                                .insert(name.to_string(), def);
                        }
                        let index = self.runtime_import(path);
                        (
                            InstanceModule::Import(*ty),
                            dfg::Instance::Import(index, defs),
                        )
                    }
                };

                let instance = self.result.instances.push(init);
                let instance2 = self.runtime_instances.push(instance_module);
                assert_eq!(instance, instance2);

                self.result
                    .side_effects
                    .push(dfg::SideEffect::Instance(instance));

                frame
                    .module_instances
                    .push(ModuleInstanceDef::Instantiated(instance, *module));
            }

            ModuleSynthetic(map) => {
                frame
                    .module_instances
                    .push(ModuleInstanceDef::Synthetic(map));
            }

            // This is one of the stages of the "magic" of implementing outer
            // aliases to components and modules. For more information on this
            // see the documentation on `LexicalScope`. This stage of the
            // implementation of outer aliases is where the `ClosedOverVars` is
            // transformed into a `ComponentClosure` state using the current
            // `InlinerFrame`'s state. This will capture the "runtime" state of
            // outer components and upvars and such naturally as part of the
            // inlining process.
            ComponentStatic(index, vars) => {
                frame.components.push(ComponentDef {
                    index: *index,
                    closure: ComponentClosure {
                        modules: vars
                            .modules
                            .iter()
                            .map(|(_, m)| frame.closed_over_module(m))
                            .collect(),
                        components: vars
                            .components
                            .iter()
                            .map(|(_, m)| frame.closed_over_component(m))
                            .collect(),
                    },
                });
            }

            // Like module instantiation is this is a "meaty" part, and don't be
            // fooled by the relative simplicity of this case. This is
            // implemented primarily by the `Inliner` structure and the design
            // of this entire module, so the "easy" step here is to simply
            // create a new inliner frame and return it to get pushed onto the
            // stack.
            ComponentInstantiate(component, args, ty) => {
                let component: &ComponentDef<'a> = &frame.components[*component];
                let index = RuntimeComponentInstanceIndex::from_u32(
                    self.result.num_runtime_component_instances,
                );
                self.result.num_runtime_component_instances += 1;
                let frame = InlinerFrame::new(
                    index,
                    &self.nested_components[component.index],
                    component.closure.clone(),
                    args.iter()
                        .map(|(name, item)| Ok((*name, frame.item(*item, types)?)))
                        .collect::<Result<_>>()?,
                    Some(*ty),
                );
                return Ok(Some(frame));
            }

            ComponentSynthetic(map, ty) => {
                let items = map
                    .iter()
                    .map(|(name, index)| Ok((*name, frame.item(*index, types)?)))
                    .collect::<Result<_>>()?;
                let types_ref = frame.translation.types_ref();
                let ty = types.convert_instance(types_ref, *ty)?;
                frame
                    .component_instances
                    .push(ComponentInstanceDef::Items(items, ty));
            }

            // Core wasm aliases, this and the cases below, are creating
            // `CoreExport` items primarily to insert into the index space so we
            // can create a unique identifier pointing to each core wasm export
            // with the instance and relevant index/name as necessary.
            AliasExportFunc(instance, name) => {
                let (ty, def) = match &frame.module_instances[*instance] {
                    ModuleInstanceDef::Instantiated(instance, module) => {
                        let (ty, item) = match &frame.modules[*module] {
                            ModuleDef::Static(idx, _ty) => {
                                let entity = self.nested_modules[*idx].module.exports[*name];
                                let ty = match entity {
                                    EntityIndex::Function(f) => {
                                        self.nested_modules[*idx].module.functions[f]
                                            .signature
                                            .unwrap_module_type_index()
                                    }
                                    _ => unreachable!(),
                                };
                                (ty, ExportItem::Index(entity))
                            }
                            ModuleDef::Import(_path, module_ty) => {
                                let module_ty = &types.component_types()[*module_ty];
                                let entity_ty = &module_ty.exports[&**name];
                                let ty = entity_ty.unwrap_func().unwrap_module_type_index();
                                (ty, ExportItem::Name((*name).to_string()))
                            }
                        };
                        let def = dfg::CoreExport {
                            instance: *instance,
                            item,
                        }
                        .into();
                        (ty, def)
                    }
                    ModuleInstanceDef::Synthetic(instance) => match instance[*name] {
                        EntityIndex::Function(i) => frame.funcs[i].clone(),
                        _ => unreachable!(),
                    },
                };
                frame.funcs.push((ty, def));
            }

            AliasExportTable(instance, name) => {
                frame.tables.push(
                    match self.core_def_of_module_instance_export(frame, *instance, *name) {
                        dfg::CoreDef::Export(e) => e,
                        _ => unreachable!(),
                    },
                );
            }

            AliasExportGlobal(instance, name) => {
                frame.globals.push(
                    match self.core_def_of_module_instance_export(frame, *instance, *name) {
                        dfg::CoreDef::Export(e) => e,
                        _ => unreachable!(),
                    },
                );
            }

            AliasExportMemory(instance, name) => {
                frame.memories.push(
                    match self.core_def_of_module_instance_export(frame, *instance, *name) {
                        dfg::CoreDef::Export(e) => e,
                        _ => unreachable!(),
                    },
                );
            }

            AliasExportTag(instance, name) => {
                frame.tags.push(
                    match self.core_def_of_module_instance_export(frame, *instance, *name) {
                        dfg::CoreDef::Export(e) => e,
                        _ => unreachable!(),
                    },
                );
            }

            AliasComponentExport(instance, name) => {
                match &frame.component_instances[*instance] {
                    // Aliasing an export from an imported instance means that
                    // we're extending the `ImportPath` by one name, represented
                    // with the clone + push here. Afterwards an appropriate
                    // item is then pushed in the relevant index space.
                    ComponentInstanceDef::Import(path, ty) => {
                        let path = path.push(*name);
                        let def = ComponentItemDef::from_import(path, types[*ty].exports[*name])?;
                        frame.push_item(def);
                    }

                    // Given a component instance which was either created
                    // through instantiation of a component or through a
                    // synthetic renaming of items we just schlep around the
                    // definitions of various items here.
                    ComponentInstanceDef::Items(map, _) => frame.push_item(map[*name].clone()),
                }
            }

            // For more information on these see `LexicalScope` but otherwise
            // this is just taking a closed over variable and inserting the
            // actual definition into the local index space since this
            // represents an outer alias to a module/component
            AliasModule(idx) => {
                frame.modules.push(frame.closed_over_module(idx));
            }
            AliasComponent(idx) => {
                frame.components.push(frame.closed_over_component(idx));
            }

            Export(item) => match item {
                ComponentItem::Func(i) => {
                    frame
                        .component_funcs
                        .push(frame.component_funcs[*i].clone());
                }
                ComponentItem::Module(i) => {
                    frame.modules.push(frame.modules[*i].clone());
                }
                ComponentItem::Component(i) => {
                    frame.components.push(frame.components[*i].clone());
                }
                ComponentItem::ComponentInstance(i) => {
                    frame
                        .component_instances
                        .push(frame.component_instances[*i].clone());
                }

                // Type index spaces aren't maintained during this inlining pass
                // so ignore this.
                ComponentItem::Type(_) => {}
            },
        }

        Ok(None)
    }

    /// "Commits" a path of an import to an actual index which is something that
    /// will be calculated at runtime.
    ///
    /// Note that the cost of calculating an item for a `RuntimeImportIndex` at
    /// runtime is amortized with an `InstancePre` which represents "all the
    /// runtime imports are lined up" and after that no more name resolution is
    /// necessary.
    fn runtime_import(&mut self, path: &ImportPath<'a>) -> RuntimeImportIndex {
        *self
            .import_path_interner
            .entry(path.clone())
            .or_insert_with(|| {
                self.result.imports.push((
                    path.index,
                    path.path.iter().map(|s| s.to_string()).collect(),
                ))
            })
    }

    /// Returns the `CoreDef`, the canonical definition for a core wasm item,
    /// for the export `name` of `instance` within `frame`.
    fn core_def_of_module_instance_export(
        &self,
        frame: &InlinerFrame<'a>,
        instance: ModuleInstanceIndex,
        name: &'a str,
    ) -> dfg::CoreDef {
        match &frame.module_instances[instance] {
            // Instantiations of a statically known module means that we can
            // refer to the exported item by a precise index, skipping name
            // lookups at runtime.
            //
            // Instantiations of an imported module, however, must do name
            // lookups at runtime since we don't know the structure ahead of
            // time here.
            ModuleInstanceDef::Instantiated(instance, module) => {
                let item = match frame.modules[*module] {
                    ModuleDef::Static(idx, _ty) => {
                        let entity = self.nested_modules[idx].module.exports[name];
                        ExportItem::Index(entity)
                    }
                    ModuleDef::Import(..) => ExportItem::Name(name.to_string()),
                };
                dfg::CoreExport {
                    instance: *instance,
                    item,
                }
                .into()
            }

            // This is a synthetic instance so the canonical definition of the
            // original item is returned.
            ModuleInstanceDef::Synthetic(instance) => match instance[name] {
                EntityIndex::Function(i) => frame.funcs[i].1.clone(),
                EntityIndex::Table(i) => frame.tables[i].clone().into(),
                EntityIndex::Global(i) => frame.globals[i].clone().into(),
                EntityIndex::Memory(i) => frame.memories[i].clone().into(),
                EntityIndex::Tag(_) => todo!(), // FIXME: #10252 support for tags in the component model
            },
        }
    }

    fn memory(
        &mut self,
        frame: &InlinerFrame<'a>,
        types: &ComponentTypesBuilder,
        memory: MemoryIndex,
    ) -> (dfg::CoreExport<MemoryIndex>, bool) {
        let memory = frame.memories[memory].clone().map_index(|i| match i {
            EntityIndex::Memory(i) => i,
            _ => unreachable!(),
        });
        let memory64 = match &self.runtime_instances[memory.instance] {
            InstanceModule::Static(idx) => match &memory.item {
                ExportItem::Index(i) => {
                    let ty = &self.nested_modules[*idx].module.memories[*i];
                    match ty.idx_type {
                        IndexType::I32 => false,
                        IndexType::I64 => true,
                    }
                }
                ExportItem::Name(_) => unreachable!(),
            },
            InstanceModule::Import(ty) => match &memory.item {
                ExportItem::Name(name) => match types[*ty].exports[name] {
                    EntityType::Memory(m) => match m.idx_type {
                        IndexType::I32 => false,
                        IndexType::I64 => true,
                    },
                    _ => unreachable!(),
                },
                ExportItem::Index(_) => unreachable!(),
            },
        };
        (memory, memory64)
    }

    /// Translates a `LocalCanonicalOptions` which indexes into the `frame`
    /// specified into a runtime representation.
    fn adapter_options(
        &mut self,
        frame: &InlinerFrame<'a>,
        types: &ComponentTypesBuilder,
        options: &LocalCanonicalOptions,
    ) -> AdapterOptions {
        let data_model = match options.data_model {
            LocalDataModel::Gc {} => DataModel::Gc {},
            LocalDataModel::LinearMemory { memory, realloc } => {
                let (memory, memory64) = memory
                    .map(|i| {
                        let (memory, memory64) = self.memory(frame, types, i);
                        (Some(memory), memory64)
                    })
                    .unwrap_or((None, false));
                let realloc = realloc.map(|i| frame.funcs[i].1.clone());
                DataModel::LinearMemory {
                    memory,
                    memory64,
                    realloc,
                }
            }
        };
        let callback = options.callback.map(|i| frame.funcs[i].1.clone());
        let post_return = options.post_return.map(|i| frame.funcs[i].1.clone());
        AdapterOptions {
            instance: frame.instance,
            string_encoding: options.string_encoding,
            callback,
            post_return,
            async_: options.async_,
            core_type: options.core_type,
            data_model,
        }
    }

    /// Translatees an `AdapterOptions` into a `CanonicalOptions` where
    /// memories/functions are inserted into the global initializer list for
    /// use at runtime. This is only used for lowered host functions and lifted
    /// functions exported to the host.
    fn canonical_options(&mut self, options: AdapterOptions) -> dfg::OptionsId {
        let data_model = match options.data_model {
            DataModel::Gc {} => dfg::CanonicalOptionsDataModel::Gc {},
            DataModel::LinearMemory {
                memory,
                memory64: _,
                realloc,
            } => dfg::CanonicalOptionsDataModel::LinearMemory {
                memory: memory.map(|export| self.result.memories.push(export)),
                realloc: realloc.map(|def| self.result.reallocs.push(def)),
            },
        };
        let callback = options.callback.map(|def| self.result.callbacks.push(def));
        let post_return = options
            .post_return
            .map(|def| self.result.post_returns.push(def));
        self.result.options.push(dfg::CanonicalOptions {
            instance: options.instance,
            string_encoding: options.string_encoding,
            callback,
            post_return,
            async_: options.async_,
            core_type: options.core_type,
            data_model,
        })
    }

    fn record_export(
        &mut self,
        name: &str,
        def: ComponentItemDef<'a>,
        types: &'a ComponentTypesBuilder,
        map: &mut IndexMap<String, dfg::Export>,
    ) -> Result<()> {
        let export = match def {
            // Exported modules are currently saved in a `PrimaryMap`, at
            // runtime, so an index (`RuntimeModuleIndex`) is assigned here and
            // then an initializer is recorded about where the module comes
            // from.
            ComponentItemDef::Module(module) => match module {
                ModuleDef::Static(index, ty) => dfg::Export::ModuleStatic { ty, index },
                ModuleDef::Import(path, ty) => dfg::Export::ModuleImport {
                    ty,
                    import: self.runtime_import(&path),
                },
            },

            ComponentItemDef::Func(func) => match func {
                // If this is a lifted function from something lowered in this
                // component then the configured options are plumbed through
                // here.
                ComponentFuncDef::Lifted { ty, func, options } => {
                    let options = self.canonical_options(options);
                    dfg::Export::LiftedFunction { ty, func, options }
                }

                // Currently reexported functions from an import are not
                // supported. Being able to actually call these functions is
                // somewhat tricky and needs something like temporary scratch
                // space that isn't implemented.
                ComponentFuncDef::Import(_) => {
                    bail!(
                        "component export `{name}` is a reexport of an imported function which is not implemented"
                    )
                }
            },

            ComponentItemDef::Instance(instance) => {
                let mut exports = IndexMap::new();
                match instance {
                    // If this instance is one that was originally imported by
                    // the component itself then the imports are translated here
                    // by converting to a `ComponentItemDef` and then
                    // recursively recording the export as a reexport.
                    //
                    // Note that for now this would only work with
                    // module-exporting instances.
                    ComponentInstanceDef::Import(path, ty) => {
                        for (name, ty) in types[ty].exports.iter() {
                            let path = path.push(name);
                            let def = ComponentItemDef::from_import(path, *ty)?;
                            self.record_export(name, def, types, &mut exports)?;
                        }
                        dfg::Export::Instance { ty, exports }
                    }

                    // An exported instance which is itself a bag of items is
                    // translated recursively here to our `exports` map which is
                    // the bag of items we're exporting.
                    ComponentInstanceDef::Items(map, ty) => {
                        for (name, def) in map {
                            self.record_export(name, def, types, &mut exports)?;
                        }
                        dfg::Export::Instance { ty, exports }
                    }
                }
            }

            // FIXME(#4283) should make an official decision on whether this is
            // the final treatment of this or not.
            ComponentItemDef::Component(_) => {
                bail!("exporting a component from the root component is not supported")
            }

            ComponentItemDef::Type(def) => dfg::Export::Type(def),
        };

        map.insert(name.to_string(), export);
        Ok(())
    }
}

impl<'a> InlinerFrame<'a> {
    fn new(
        instance: RuntimeComponentInstanceIndex,
        translation: &'a Translation<'a>,
        closure: ComponentClosure<'a>,
        args: HashMap<&'a str, ComponentItemDef<'a>>,
        instance_ty: Option<ComponentInstanceTypeId>,
    ) -> Self {
        // FIXME: should iterate over the initializers of `translation` and
        // calculate the size of each index space to use `with_capacity` for
        // all the maps below. Given that doing such would be wordy and compile
        // time is otherwise not super crucial it's not done at this time.
        InlinerFrame {
            instance,
            translation,
            closure,
            args,
            instance_ty,
            initializers: translation.initializers.iter(),

            funcs: Default::default(),
            memories: Default::default(),
            tables: Default::default(),
            globals: Default::default(),
            tags: Default::default(),

            component_instances: Default::default(),
            component_funcs: Default::default(),
            module_instances: Default::default(),
            components: Default::default(),
            modules: Default::default(),
        }
    }

    fn item(
        &self,
        index: ComponentItem,
        types: &mut ComponentTypesBuilder,
    ) -> Result<ComponentItemDef<'a>> {
        Ok(match index {
            ComponentItem::Func(i) => ComponentItemDef::Func(self.component_funcs[i].clone()),
            ComponentItem::Component(i) => ComponentItemDef::Component(self.components[i].clone()),
            ComponentItem::ComponentInstance(i) => {
                ComponentItemDef::Instance(self.component_instances[i].clone())
            }
            ComponentItem::Module(i) => ComponentItemDef::Module(self.modules[i].clone()),
            ComponentItem::Type(t) => {
                let types_ref = self.translation.types_ref();
                ComponentItemDef::Type(types.convert_type(types_ref, t)?)
            }
        })
    }

    /// Pushes the component `item` definition provided into the appropriate
    /// index space within this component.
    fn push_item(&mut self, item: ComponentItemDef<'a>) {
        match item {
            ComponentItemDef::Func(i) => {
                self.component_funcs.push(i);
            }
            ComponentItemDef::Module(i) => {
                self.modules.push(i);
            }
            ComponentItemDef::Component(i) => {
                self.components.push(i);
            }
            ComponentItemDef::Instance(i) => {
                self.component_instances.push(i);
            }

            // In short, type definitions aren't tracked here.
            //
            // The longer form explanation for this is that structural types
            // like lists and records don't need to be tracked at all and the
            // only significant type which needs tracking is resource types
            // themselves. Resource types, however, are tracked within the
            // `ResourcesBuilder` state rather than an `InlinerFrame` so they're
            // ignored here as well. The general reason for that is that type
            // information is everywhere and this `InlinerFrame` is not
            // everywhere so it seemed like it would make sense to split the
            // two.
            //
            // Note though that this case is actually frequently hit, so it
            // can't be `unreachable!()`. Instead callers are responsible for
            // handling this appropriately with respect to resources.
            ComponentItemDef::Type(_ty) => {}
        }
    }

    fn closed_over_module(&self, index: &ClosedOverModule) -> ModuleDef<'a> {
        match *index {
            ClosedOverModule::Local(i) => self.modules[i].clone(),
            ClosedOverModule::Upvar(i) => self.closure.modules[i].clone(),
        }
    }

    fn closed_over_component(&self, index: &ClosedOverComponent) -> ComponentDef<'a> {
        match *index {
            ClosedOverComponent::Local(i) => self.components[i].clone(),
            ClosedOverComponent::Upvar(i) => self.closure.components[i].clone(),
        }
    }

    /// Completes the instantiation of a subcomponent and records type
    /// information for the instance that was produced.
    ///
    /// This method is invoked when an `InlinerFrame` finishes for a
    /// subcomponent. The `def` provided represents the instance that was
    /// produced from instantiation, and `ty` is the wasmparser-defined type of
    /// the instance produced.
    ///
    /// The purpose of this method is to record type information about resources
    /// in the instance produced. In the component model all instantiations of a
    /// component produce fresh new types for all resources which are unequal to
    /// all prior resources. This means that if wasmparser's `ty` type
    /// information references a unique resource within `def` that has never
    /// been registered before then that means it's a defined resource within
    /// the component that was just instantiated (as opposed to an imported
    /// resource which was reexported).
    ///
    /// Further type translation after this instantiation can refer to these
    /// resource types and a mapping from those types to the wasmtime-internal
    /// types is required, so this method builds up those mappings.
    ///
    /// Essentially what happens here is that the `ty` type is registered and
    /// any new unique resources are registered so new tables can be introduced
    /// along with origin information about the actual underlying resource type
    /// and which component instantiated it.
    fn finish_instantiate(
        &mut self,
        exports: IndexMap<&'a str, ComponentItemDef<'a>>,
        ty: ComponentInstanceTypeId,
        types: &mut ComponentTypesBuilder,
    ) -> Result<()> {
        let types_ref = self.translation.types_ref();
        {
            let (resources, types) = types.resources_mut_and_types();
            let mut path = Vec::new();
            resources.register_component_entity_type(
                &types_ref,
                ComponentEntityType::Instance(ty),
                &mut path,
                &mut |path| match path {
                    [] => unreachable!(),
                    [name, rest @ ..] => exports[name].lookup_resource(rest, types),
                },
            );
        }
        let ty = types.convert_instance(types_ref, ty)?;
        let def = ComponentInstanceDef::Items(exports, ty);
        let arg = ComponentItemDef::Instance(def);
        self.push_item(arg);
        Ok(())
    }
}

impl<'a> ImportPath<'a> {
    fn root(index: ImportIndex) -> ImportPath<'a> {
        ImportPath {
            index,
            path: Vec::new(),
        }
    }

    fn push(&self, s: impl Into<Cow<'a, str>>) -> ImportPath<'a> {
        let mut new = self.clone();
        new.path.push(s.into());
        new
    }
}

impl<'a> ComponentItemDef<'a> {
    fn from_import(path: ImportPath<'a>, ty: TypeDef) -> Result<ComponentItemDef<'a>> {
        let item = match ty {
            TypeDef::Module(ty) => ComponentItemDef::Module(ModuleDef::Import(path, ty)),
            TypeDef::ComponentInstance(ty) => {
                ComponentItemDef::Instance(ComponentInstanceDef::Import(path, ty))
            }
            TypeDef::ComponentFunc(_ty) => ComponentItemDef::Func(ComponentFuncDef::Import(path)),
            // FIXME(#4283) should commit one way or another to how this
            // should be treated.
            TypeDef::Component(_ty) => bail!("root-level component imports are not supported"),
            TypeDef::Interface(_) | TypeDef::Resource(_) => ComponentItemDef::Type(ty),
            TypeDef::CoreFunc(_) => unreachable!(),
        };
        Ok(item)
    }

    /// Walks the `path` within `self` to find a resource at that path.
    ///
    /// This method is used when resources are found within wasmparser's type
    /// information and they need to be correlated with actual concrete
    /// definitions from this inlining pass. The `path` here is a list of
    /// instance export names (or empty) to walk to reach down into the final
    /// definition which should refer to a resource itself.
    fn lookup_resource(&self, path: &[&str], types: &ComponentTypes) -> ResourceIndex {
        let mut cur = self.clone();

        // Each element of `path` represents unwrapping a layer of an instance
        // type, so handle those here by updating `cur` iteratively.
        for element in path.iter().copied() {
            let instance = match cur {
                ComponentItemDef::Instance(def) => def,
                _ => unreachable!(),
            };
            cur = match instance {
                // If this instance is a "bag of things" then this is as easy as
                // looking up the name in the bag of names.
                ComponentInstanceDef::Items(names, _) => names[element].clone(),

                // If, however, this instance is an imported instance then this
                // is a further projection within the import with one more path
                // element. The `types` type information is used to lookup the
                // type of `element` within the instance type, and that's used
                // in conjunction with a one-longer `path` to produce a new item
                // definition.
                ComponentInstanceDef::Import(path, ty) => {
                    ComponentItemDef::from_import(path.push(element), types[ty].exports[element])
                        .unwrap()
                }
            };
        }

        // Once `path` has been iterated over it must be the case that the final
        // item is a resource type, in which case a lookup can be performed.
        match cur {
            ComponentItemDef::Type(TypeDef::Resource(idx)) => types[idx].ty,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
enum InstanceModule {
    Static(StaticModuleIndex),
    Import(TypeModuleIndex),
}

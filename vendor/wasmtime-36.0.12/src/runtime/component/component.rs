use crate::component::InstanceExportLookup;
use crate::component::matching::InstanceType;
use crate::component::types;
use crate::prelude::*;
#[cfg(feature = "std")]
use crate::runtime::vm::open_file_for_mmap;
use crate::runtime::vm::{CompiledModuleId, VMArrayCallFunction, VMFuncRef, VMWasmCallFunction};
use crate::{
    Engine, Module, ResourcesRequired, code::CodeObject, code_memory::CodeMemory,
    type_registry::TypeCollection,
};
use crate::{FuncType, ValType};
use alloc::sync::Arc;
use core::ops::Range;
use core::ptr::NonNull;
#[cfg(feature = "std")]
use std::path::Path;
use wasmtime_environ::TypeTrace;
use wasmtime_environ::component::{
    AllCallFunc, CompiledComponentInfo, ComponentArtifacts, ComponentTypes, CoreDef, Export,
    ExportIndex, GlobalInitializer, InstantiateModule, NameMapNoIntern, OptionsIndex,
    StaticModuleIndex, TrampolineIndex, TypeComponentIndex, TypeFuncIndex, VMComponentOffsets,
};
use wasmtime_environ::{FunctionLoc, HostPtr, ObjectKind, PrimaryMap};

/// A compiled WebAssembly Component.
///
/// This structure represents a compiled component that is ready to be
/// instantiated. This owns a region of virtual memory which contains executable
/// code compiled from a WebAssembly binary originally. This is the analog of
/// [`Module`](crate::Module) in the component embedding API.
///
/// A [`Component`] can be turned into an
/// [`Instance`](crate::component::Instance) through a
/// [`Linker`](crate::component::Linker). [`Component`]s are safe to share
/// across threads. The compilation model of a component is the same as that of
/// [a module](crate::Module) which is to say:
///
/// * Compilation happens synchronously during [`Component::new`].
/// * The result of compilation can be saved into storage with
///   [`Component::serialize`].
/// * A previously compiled artifact can be parsed with
///   [`Component::deserialize`].
/// * No compilation happens at runtime for a component — everything is done
///   by the time [`Component::new`] returns.
///
/// ## Components and `Clone`
///
/// Using `clone` on a `Component` is a cheap operation. It will not create an
/// entirely new component, but rather just a new reference to the existing
/// component. In other words it's a shallow copy, not a deep copy.
///
/// ## Examples
///
/// For example usage see the documentation of [`Module`](crate::Module) as
/// [`Component`] has the same high-level API.
#[derive(Clone)]
pub struct Component {
    inner: Arc<ComponentInner>,
}

struct ComponentInner {
    /// Unique id for this component within this process.
    ///
    /// Note that this is repurposing ids for modules intentionally as there
    /// shouldn't be an issue overlapping them.
    id: CompiledModuleId,

    /// The engine that this component belongs to.
    engine: Engine,

    /// Component type index
    ty: TypeComponentIndex,

    /// Core wasm modules that the component defined internally, indexed by the
    /// compile-time-assigned `ModuleUpvarIndex`.
    static_modules: PrimaryMap<StaticModuleIndex, Module>,

    /// Code-related information such as the compiled artifact, type
    /// information, etc.
    ///
    /// Note that the `Arc` here is used to share this allocation with internal
    /// modules.
    code: Arc<CodeObject>,

    /// Metadata produced during compilation.
    info: CompiledComponentInfo,

    /// A cached handle to the `wasmtime::FuncType` for the canonical ABI's
    /// `realloc`, to avoid the need to look up types in the registry and take
    /// locks when calling `realloc` via `TypedFunc::call_raw`.
    realloc_func_type: Arc<FuncType>,
}

pub(crate) struct AllCallFuncPointers {
    pub wasm_call: NonNull<VMWasmCallFunction>,
    pub array_call: NonNull<VMArrayCallFunction>,
}

impl Component {
    /// Compiles a new WebAssembly component from the in-memory list of bytes
    /// provided.
    ///
    /// The `bytes` provided can either be the binary or text format of a
    /// [WebAssembly component]. Note that the text format requires the `wat`
    /// feature of this crate to be enabled. This API does not support
    /// streaming compilation.
    ///
    /// This function will synchronously validate the entire component,
    /// including all core modules, and then compile all components, modules,
    /// etc., found within the provided bytes.
    ///
    /// [WebAssembly component]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/Binary.md
    ///
    /// # Errors
    ///
    /// This function may fail and return an error. Errors may include
    /// situations such as:
    ///
    /// * The binary provided could not be decoded because it's not a valid
    ///   WebAssembly binary
    /// * The WebAssembly binary may not validate (e.g. contains type errors)
    /// * Implementation-specific limits were exceeded with a valid binary (for
    ///   example too many locals)
    /// * The wasm binary may use features that are not enabled in the
    ///   configuration of `engine`
    /// * If the `wat` feature is enabled and the input is text, then it may be
    ///   rejected if it fails to parse.
    ///
    /// The error returned should contain full information about why compilation
    /// failed.
    ///
    /// # Examples
    ///
    /// The `new` function can be invoked with a in-memory array of bytes:
    ///
    /// ```no_run
    /// # use wasmtime::*;
    /// # use wasmtime::component::Component;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let wasm_bytes: Vec<u8> = Vec::new();
    /// let component = Component::new(&engine, &wasm_bytes)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Or you can also pass in a string to be parsed as the wasm text
    /// format:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # use wasmtime::component::Component;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let component = Component::new(&engine, "(component (core module))")?;
    /// # Ok(())
    /// # }
    #[cfg(any(feature = "cranelift", feature = "winch"))]
    pub fn new(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Component> {
        crate::CodeBuilder::new(engine)
            .wasm_binary_or_text(bytes.as_ref(), None)?
            .compile_component()
    }

    /// Compiles a new WebAssembly component from a wasm file on disk pointed
    /// to by `file`.
    ///
    /// This is a convenience function for reading the contents of `file` on
    /// disk and then calling [`Component::new`].
    #[cfg(all(feature = "std", any(feature = "cranelift", feature = "winch")))]
    pub fn from_file(engine: &Engine, file: impl AsRef<Path>) -> Result<Component> {
        crate::CodeBuilder::new(engine)
            .wasm_binary_or_text_file(file.as_ref())?
            .compile_component()
    }

    /// Compiles a new WebAssembly component from the in-memory wasm image
    /// provided.
    ///
    /// This function is the same as [`Component::new`] except that it does not
    /// accept the text format of WebAssembly. Even if the `wat` feature
    /// is enabled an error will be returned here if `binary` is the text
    /// format.
    ///
    /// For more information on semantics and errors see [`Component::new`].
    #[cfg(any(feature = "cranelift", feature = "winch"))]
    pub fn from_binary(engine: &Engine, binary: &[u8]) -> Result<Component> {
        crate::CodeBuilder::new(engine)
            .wasm_binary(binary, None)?
            .compile_component()
    }

    /// Same as [`Module::deserialize`], but for components.
    ///
    /// Note that the bytes referenced here must contain contents previously
    /// produced by [`Engine::precompile_component`] or
    /// [`Component::serialize`].
    ///
    /// For more information see the [`Module::deserialize`] method.
    ///
    /// # Unsafety
    ///
    /// The unsafety of this method is the same as that of the
    /// [`Module::deserialize`] method.
    ///
    /// [`Module::deserialize`]: crate::Module::deserialize
    pub unsafe fn deserialize(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Component> {
        let code = engine.load_code_bytes(bytes.as_ref(), ObjectKind::Component)?;
        Component::from_parts(engine, code, None)
    }

    /// Same as [`Module::deserialize_raw`], but for components.
    ///
    /// See [`Component::deserialize`] for additional information; this method
    /// works identically except that it will not create a copy of the provided
    /// memory but will use it directly.
    ///
    /// # Unsafety
    ///
    /// All of the safety notes from [`Component::deserialize`] apply here as well
    /// with the additional constraint that the code memory provide by `memory`
    /// lives for as long as the module and is nevery externally modified for
    /// the lifetime of the deserialized module.
    pub unsafe fn deserialize_raw(engine: &Engine, memory: NonNull<[u8]>) -> Result<Component> {
        // SAFETY: the contract required by `load_code_raw` is the same as this
        // function.
        let code = unsafe { engine.load_code_raw(memory, ObjectKind::Component)? };
        Component::from_parts(engine, code, None)
    }

    /// Same as [`Module::deserialize_file`], but for components.
    ///
    /// Note that the file referenced here must contain contents previously
    /// produced by [`Engine::precompile_component`] or
    /// [`Component::serialize`].
    ///
    /// For more information see the [`Module::deserialize_file`] method.
    ///
    /// # Unsafety
    ///
    /// The unsafety of this method is the same as that of the
    /// [`Module::deserialize_file`] method.
    ///
    /// [`Module::deserialize_file`]: crate::Module::deserialize_file
    #[cfg(feature = "std")]
    pub unsafe fn deserialize_file(engine: &Engine, path: impl AsRef<Path>) -> Result<Component> {
        let file = open_file_for_mmap(path.as_ref())?;
        let code = engine
            .load_code_file(file, ObjectKind::Component)
            .with_context(|| format!("failed to load code for: {}", path.as_ref().display()))?;
        Component::from_parts(engine, code, None)
    }

    /// Returns the type of this component as a [`types::Component`].
    ///
    /// This method enables runtime introspection of the type of a component
    /// before instantiation, if necessary.
    ///
    /// ## Component types and Resources
    ///
    /// An important point to note here is that the precise type of imports and
    /// exports of a component change when it is instantiated with respect to
    /// resources. For example a [`Component`] represents an un-instantiated
    /// component meaning that its imported resources are represented as abstract
    /// resource types. These abstract types are not equal to any other
    /// component's types.
    ///
    /// For example:
    ///
    /// ```
    /// # use wasmtime::Engine;
    /// # use wasmtime::component::Component;
    /// # use wasmtime::component::types::ComponentItem;
    /// # fn main() -> wasmtime::Result<()> {
    /// # let engine = Engine::default();
    /// let a = Component::new(&engine, r#"
    ///     (component (import "x" (type (sub resource))))
    /// "#)?;
    /// let b = Component::new(&engine, r#"
    ///     (component (import "x" (type (sub resource))))
    /// "#)?;
    ///
    /// let (_, a_ty) = a.component_type().imports(&engine).next().unwrap();
    /// let (_, b_ty) = b.component_type().imports(&engine).next().unwrap();
    ///
    /// let a_ty = match a_ty {
    ///     ComponentItem::Resource(ty) => ty,
    ///     _ => unreachable!(),
    /// };
    /// let b_ty = match b_ty {
    ///     ComponentItem::Resource(ty) => ty,
    ///     _ => unreachable!(),
    /// };
    /// assert!(a_ty != b_ty);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Additionally, however, these abstract types are "substituted" during
    /// instantiation meaning that a component type will appear to have changed
    /// once it is instantiated.
    ///
    /// ```
    /// # use wasmtime::{Engine, Store};
    /// # use wasmtime::component::{Component, Linker, ResourceType};
    /// # use wasmtime::component::types::ComponentItem;
    /// # fn main() -> wasmtime::Result<()> {
    /// # let engine = Engine::default();
    /// // Here this component imports a resource and then exports it as-is
    /// // which means that the export is equal to the import.
    /// let a = Component::new(&engine, r#"
    ///     (component
    ///         (import "x" (type $x (sub resource)))
    ///         (export "x" (type $x))
    ///     )
    /// "#)?;
    ///
    /// let (_, import) = a.component_type().imports(&engine).next().unwrap();
    /// let (_, export) = a.component_type().exports(&engine).next().unwrap();
    ///
    /// let import = match import {
    ///     ComponentItem::Resource(ty) => ty,
    ///     _ => unreachable!(),
    /// };
    /// let export = match export {
    ///     ComponentItem::Resource(ty) => ty,
    ///     _ => unreachable!(),
    /// };
    /// assert_eq!(import, export);
    ///
    /// // However after instantiation the resource type "changes"
    /// let mut store = Store::new(&engine, ());
    /// let mut linker = Linker::new(&engine);
    /// linker.root().resource("x", ResourceType::host::<()>(), |_, _| Ok(()))?;
    /// let instance = linker.instantiate(&mut store, &a)?;
    /// let instance_ty = instance.get_resource(&mut store, "x").unwrap();
    ///
    /// // Here `instance_ty` is not the same as either `import` or `export`,
    /// // but it is equal to what we provided as an import.
    /// assert!(instance_ty != import);
    /// assert!(instance_ty != export);
    /// assert!(instance_ty == ResourceType::host::<()>());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Finally, each instantiation of an exported resource from a component is
    /// considered "fresh" for all instantiations meaning that different
    /// instantiations will have different exported resource types:
    ///
    /// ```
    /// # use wasmtime::{Engine, Store};
    /// # use wasmtime::component::{Component, Linker};
    /// # fn main() -> wasmtime::Result<()> {
    /// # let engine = Engine::default();
    /// let a = Component::new(&engine, r#"
    ///     (component
    ///         (type $x (resource (rep i32)))
    ///         (export "x" (type $x))
    ///     )
    /// "#)?;
    ///
    /// let mut store = Store::new(&engine, ());
    /// let linker = Linker::new(&engine);
    /// let instance1 = linker.instantiate(&mut store, &a)?;
    /// let instance2 = linker.instantiate(&mut store, &a)?;
    ///
    /// let x1 = instance1.get_resource(&mut store, "x").unwrap();
    /// let x2 = instance2.get_resource(&mut store, "x").unwrap();
    ///
    /// // Despite these two resources being the same export of the same
    /// // component they come from two different instances meaning that their
    /// // types will be unique.
    /// assert!(x1 != x2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn component_type(&self) -> types::Component {
        self.with_uninstantiated_instance_type(|ty| types::Component::from(self.inner.ty, ty))
    }

    fn with_uninstantiated_instance_type<R>(&self, f: impl FnOnce(&InstanceType<'_>) -> R) -> R {
        let resources = Arc::new(PrimaryMap::new());
        f(&InstanceType {
            types: self.types(),
            resources: &resources,
        })
    }

    /// Final assembly step for a component from its in-memory representation.
    ///
    /// If the `artifacts` are specified as `None` here then they will be
    /// deserialized from `code_memory`.
    pub(crate) fn from_parts(
        engine: &Engine,
        code_memory: Arc<CodeMemory>,
        artifacts: Option<ComponentArtifacts>,
    ) -> Result<Component> {
        let ComponentArtifacts {
            ty,
            info,
            mut types,
            mut static_modules,
        } = match artifacts {
            Some(artifacts) => artifacts,
            None => postcard::from_bytes(code_memory.wasmtime_info())?,
        };

        // Validate that the component can be used with the current instance
        // allocator.
        engine.allocator().validate_component(
            &info.component,
            &VMComponentOffsets::new(HostPtr, &info.component),
            &|module_index| &static_modules[module_index].module,
        )?;

        // Create a signature registration with the `Engine` for all trampolines
        // and core wasm types found within this component, both for the
        // component and for all included core wasm modules.
        let signatures = engine.register_and_canonicalize_types(
            types.module_types_mut(),
            static_modules.iter_mut().map(|(_, m)| &mut m.module),
        );
        types.canonicalize_for_runtime_usage(&mut |idx| signatures.shared_type(idx).unwrap());

        // Assemble the `CodeObject` artifact which is shared by all core wasm
        // modules as well as the final component.
        let types = Arc::new(types);
        let code = Arc::new(CodeObject::new(code_memory, signatures, types.into()));

        // Convert all information about static core wasm modules into actual
        // `Module` instances by converting each `CompiledModuleInfo`, the
        // `types` type information, and the code memory to a runtime object.
        let static_modules = static_modules
            .into_iter()
            .map(|(_, info)| Module::from_parts_raw(engine, code.clone(), info, false))
            .collect::<Result<_>>()?;

        let realloc_func_type = Arc::new(FuncType::new(
            engine,
            [ValType::I32, ValType::I32, ValType::I32, ValType::I32],
            [ValType::I32],
        ));

        Ok(Component {
            inner: Arc::new(ComponentInner {
                id: CompiledModuleId::new(),
                engine: engine.clone(),
                ty,
                static_modules,
                code,
                info,
                realloc_func_type,
            }),
        })
    }

    pub(crate) fn ty(&self) -> TypeComponentIndex {
        self.inner.ty
    }

    pub(crate) fn env_component(&self) -> &wasmtime_environ::component::Component {
        &self.inner.info.component
    }

    pub(crate) fn static_module(&self, idx: StaticModuleIndex) -> &Module {
        &self.inner.static_modules[idx]
    }

    #[cfg(feature = "profiling")]
    pub(crate) fn static_modules(&self) -> impl Iterator<Item = &Module> {
        self.inner.static_modules.values()
    }

    #[inline]
    pub(crate) fn types(&self) -> &Arc<ComponentTypes> {
        match self.inner.code.types() {
            crate::code::Types::Component(types) => types,
            // The only creator of a `Component` is itself which uses the other
            // variant, so this shouldn't be possible.
            crate::code::Types::Module(_) => unreachable!(),
        }
    }

    pub(crate) fn signatures(&self) -> &TypeCollection {
        self.inner.code.signatures()
    }

    pub(crate) fn text(&self) -> &[u8] {
        self.inner.code.code_memory().text()
    }

    pub(crate) fn trampoline_ptrs(&self, index: TrampolineIndex) -> AllCallFuncPointers {
        let AllCallFunc {
            wasm_call,
            array_call,
        } = &self.inner.info.trampolines[index];
        AllCallFuncPointers {
            wasm_call: self.func(wasm_call).cast(),
            array_call: self.func(array_call).cast(),
        }
    }

    fn func(&self, loc: &FunctionLoc) -> NonNull<u8> {
        let text = self.text();
        let trampoline = &text[loc.start as usize..][..loc.length as usize];
        NonNull::from(trampoline).cast()
    }

    pub(crate) fn code_object(&self) -> &Arc<CodeObject> {
        &self.inner.code
    }

    /// Same as [`Module::serialize`], except for a component.
    ///
    /// Note that the artifact produced here must be passed to
    /// [`Component::deserialize`] and is not compatible for use with
    /// [`Module`].
    ///
    /// [`Module::serialize`]: crate::Module::serialize
    /// [`Module`]: crate::Module
    pub fn serialize(&self) -> Result<Vec<u8>> {
        Ok(self.code_object().code_memory().mmap().to_vec())
    }

    /// Creates a new `VMFuncRef` with all fields filled out for the destructor
    /// specified.
    ///
    /// The `dtor`'s own `VMFuncRef` won't have `wasm_call` filled out but this
    /// component may have `resource_drop_wasm_to_native_trampoline` filled out
    /// if necessary in which case it's filled in here.
    pub(crate) fn resource_drop_func_ref(&self, dtor: &crate::func::HostFunc) -> VMFuncRef {
        // Host functions never have their `wasm_call` filled in at this time.
        assert!(dtor.func_ref().wasm_call.is_none());

        // Note that if `resource_drop_wasm_to_native_trampoline` is not present
        // then this can't be called by the component, so it's ok to leave it
        // blank.
        let wasm_call = self
            .inner
            .info
            .resource_drop_wasm_to_array_trampoline
            .as_ref()
            .map(|i| self.func(i).cast().into());
        VMFuncRef {
            wasm_call,
            ..*dtor.func_ref()
        }
    }

    /// Returns a summary of the resources required to instantiate this
    /// [`Component`][crate::component::Component].
    ///
    /// Note that when a component imports and instantiates another component or
    /// core module, we cannot determine ahead of time how many resources
    /// instantiating this component will require, and therefore this method
    /// will return `None` in these scenarios.
    ///
    /// Potential uses of the returned information:
    ///
    /// * Determining whether your pooling allocator configuration supports
    ///   instantiating this component.
    ///
    /// * Deciding how many of which `Component` you want to instantiate within
    ///   a fixed amount of resources, e.g. determining whether to create 5
    ///   instances of component X or 10 instances of component Y.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> wasmtime::Result<()> {
    /// use wasmtime::{Config, Engine, component::Component};
    ///
    /// let mut config = Config::new();
    /// config.wasm_multi_memory(true);
    /// config.wasm_component_model(true);
    /// let engine = Engine::new(&config)?;
    ///
    /// let component = Component::new(&engine, &r#"
    ///     (component
    ///         ;; Define a core module that uses two memories.
    ///         (core module $m
    ///             (memory 1)
    ///             (memory 6)
    ///         )
    ///
    ///         ;; Instantiate that core module three times.
    ///         (core instance $i1 (instantiate (module $m)))
    ///         (core instance $i2 (instantiate (module $m)))
    ///         (core instance $i3 (instantiate (module $m)))
    ///     )
    /// "#)?;
    ///
    /// let resources = component.resources_required()
    ///     .expect("this component does not import any core modules or instances");
    ///
    /// // Instantiating the component will require allocating two memories per
    /// // core instance, and there are three instances, so six total memories.
    /// assert_eq!(resources.num_memories, 6);
    /// assert_eq!(resources.max_initial_memory_size, Some(6));
    ///
    /// // The component doesn't need any tables.
    /// assert_eq!(resources.num_tables, 0);
    /// assert_eq!(resources.max_initial_table_size, None);
    /// # Ok(()) }
    /// ```
    pub fn resources_required(&self) -> Option<ResourcesRequired> {
        let mut resources = ResourcesRequired {
            num_memories: 0,
            max_initial_memory_size: None,
            num_tables: 0,
            max_initial_table_size: None,
        };
        for init in &self.env_component().initializers {
            match init {
                GlobalInitializer::InstantiateModule(inst) => match inst {
                    InstantiateModule::Static(index, _) => {
                        let module = self.static_module(*index);
                        resources.add(&module.resources_required());
                    }
                    InstantiateModule::Import(_, _) => {
                        // We can't statically determine the resources required
                        // to instantiate this component.
                        return None;
                    }
                },
                GlobalInitializer::LowerImport { .. }
                | GlobalInitializer::ExtractMemory(_)
                | GlobalInitializer::ExtractTable(_)
                | GlobalInitializer::ExtractRealloc(_)
                | GlobalInitializer::ExtractCallback(_)
                | GlobalInitializer::ExtractPostReturn(_)
                | GlobalInitializer::Resource(_) => {}
            }
        }
        Some(resources)
    }

    /// Returns the range, in the host's address space, that this module's
    /// compiled code resides at.
    ///
    /// For more information see
    /// [`Module::image_range`](crate::Module::image_range).
    pub fn image_range(&self) -> Range<*const u8> {
        self.inner.code.code_memory().mmap().image_range()
    }

    /// Force initialization of copy-on-write images to happen here-and-now
    /// instead of when they're requested during first instantiation.
    ///
    /// When [copy-on-write memory
    /// initialization](crate::Config::memory_init_cow) is enabled then Wasmtime
    /// will lazily create the initialization image for a component. This method
    /// can be used to explicitly dictate when this initialization happens.
    ///
    /// Note that this largely only matters on Linux when memfd is used.
    /// Otherwise the copy-on-write image typically comes from disk and in that
    /// situation the creation of the image is trivial as the image is always
    /// sourced from disk. On Linux, though, when memfd is used a memfd is
    /// created and the initialization image is written to it.
    ///
    /// Also note that this method is not required to be called, it's available
    /// as a performance optimization if required but is otherwise handled
    /// automatically.
    pub fn initialize_copy_on_write_image(&self) -> Result<()> {
        for (_, module) in self.inner.static_modules.iter() {
            module.initialize_copy_on_write_image()?;
        }
        Ok(())
    }

    /// Looks up a specific export of this component by `name` optionally nested
    /// within the `instance` provided.
    ///
    /// See related method [`Self::get_export`] for additional docs and
    /// examples.
    ///
    /// This method is primarily used to acquire a [`ComponentExportIndex`]
    /// which can be used with [`Instance`](crate::component::Instance) when
    /// looking up exports. Export lookup with [`ComponentExportIndex`] can
    /// skip string lookups at runtime and instead use a more efficient
    /// index-based lookup.
    ///
    /// This method only returns the [`ComponentExportIndex`]. If you need the
    /// corresponding [`types::ComponentItem`], use the related function
    /// [`Self::get_export`].
    ///
    ///
    /// [`Instance`](crate::component::Instance) has a corresponding method
    /// [`Instance::get_export_index`](crate::component::Instance::get_export_index).
    pub fn get_export_index(
        &self,
        instance: Option<&ComponentExportIndex>,
        name: &str,
    ) -> Option<ComponentExportIndex> {
        let index = self.lookup_export_index(instance, name)?;
        Some(ComponentExportIndex {
            id: self.inner.id,
            index,
        })
    }

    /// Looks up a specific export of this component by `name` optionally nested
    /// within the `instance` provided.
    ///
    /// This method is primarily used to acquire a [`ComponentExportIndex`]
    /// which can be used with [`Instance`](crate::component::Instance) when
    /// looking up exports. Export lookup with [`ComponentExportIndex`] can
    /// skip string lookups at runtime and instead use a more efficient
    /// index-based lookup.
    ///
    /// This method takes a few arguments:
    ///
    /// * `engine` - the engine that was used to compile this component.
    /// * `instance` - an optional "parent instance" for the export being looked
    ///   up. If this is `None` then the export is looked up on the root of the
    ///   component itself, and otherwise the export is looked up on the
    ///   `instance` specified. Note that `instance` must have come from a
    ///   previous invocation of this method.
    /// * `name` - the name of the export that's being looked up.
    ///
    /// If the export is located then two values are returned: a
    /// [`types::ComponentItem`] which enables introspection about the type of
    /// the export and a [`ComponentExportIndex`]. The index returned notably
    /// implements the [`InstanceExportLookup`] trait which enables using it
    /// with [`Instance::get_func`](crate::component::Instance::get_func) for
    /// example.
    ///
    /// The returned [`types::ComponentItem`] is more expensive to calculate
    /// than the [`ComponentExportIndex`]. If you only consume the
    /// [`ComponentExportIndex`], use the related method
    /// [`Self::get_export_index`] instead.
    ///
    /// [`Instance`](crate::component::Instance) has a corresponding method
    /// [`Instance::get_export`](crate::component::Instance::get_export).
    ///
    /// # Examples
    ///
    /// ```
    /// use wasmtime::{Engine, Store};
    /// use wasmtime::component::{Component, Linker};
    /// use wasmtime::component::types::ComponentItem;
    ///
    /// # fn main() -> wasmtime::Result<()> {
    /// let engine = Engine::default();
    /// let component = Component::new(
    ///     &engine,
    ///     r#"
    ///         (component
    ///             (core module $m
    ///                 (func (export "f"))
    ///             )
    ///             (core instance $i (instantiate $m))
    ///             (func (export "f")
    ///                 (canon lift (core func $i "f")))
    ///         )
    ///     "#,
    /// )?;
    ///
    /// // Perform a lookup of the function "f" before instantiaton.
    /// let (ty, export) = component.get_export(None, "f").unwrap();
    /// assert!(matches!(ty, ComponentItem::ComponentFunc(_)));
    ///
    /// // After instantiation use `export` to lookup the function in question
    /// // which notably does not do a string lookup at runtime.
    /// let mut store = Store::new(&engine, ());
    /// let instance = Linker::new(&engine).instantiate(&mut store, &component)?;
    /// let func = instance.get_typed_func::<(), ()>(&mut store, &export)?;
    /// // ...
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_export(
        &self,
        instance: Option<&ComponentExportIndex>,
        name: &str,
    ) -> Option<(types::ComponentItem, ComponentExportIndex)> {
        let info = self.env_component();
        let index = self.lookup_export_index(instance, name)?;
        let item = self.with_uninstantiated_instance_type(|instance| {
            types::ComponentItem::from_export(
                &self.inner.engine,
                &info.export_items[index],
                instance,
            )
        });
        Some((
            item,
            ComponentExportIndex {
                id: self.inner.id,
                index,
            },
        ))
    }

    pub(crate) fn lookup_export_index(
        &self,
        instance: Option<&ComponentExportIndex>,
        name: &str,
    ) -> Option<ExportIndex> {
        let info = self.env_component();
        let exports = match instance {
            Some(idx) => {
                if idx.id != self.inner.id {
                    return None;
                }
                match &info.export_items[idx.index] {
                    Export::Instance { exports, .. } => exports,
                    _ => return None,
                }
            }
            None => &info.exports,
        };
        exports.get(name, &NameMapNoIntern).copied()
    }

    pub(crate) fn id(&self) -> CompiledModuleId {
        self.inner.id
    }

    /// Returns the [`Engine`] that this [`Component`] was compiled by.
    pub fn engine(&self) -> &Engine {
        &self.inner.engine
    }

    pub(crate) fn realloc_func_ty(&self) -> &Arc<FuncType> {
        &self.inner.realloc_func_type
    }

    /// Returns the `Export::LiftedFunction` metadata associated with `export`.
    ///
    /// # Panics
    ///
    /// Panics if `export` is out of bounds or if it isn't a `LiftedFunction`.
    pub(crate) fn export_lifted_function(
        &self,
        export: ExportIndex,
    ) -> (TypeFuncIndex, &CoreDef, OptionsIndex) {
        let component = self.env_component();
        match &component.export_items[export] {
            Export::LiftedFunction { ty, func, options } => (*ty, func, *options),
            _ => unreachable!(),
        }
    }
}

/// A value which represents a known export of a component.
///
/// This is the return value of [`Component::get_export`] and implements the
/// [`InstanceExportLookup`] trait to work with lookups like
/// [`Instance::get_func`](crate::component::Instance::get_func).
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ComponentExportIndex {
    pub(crate) id: CompiledModuleId,
    pub(crate) index: ExportIndex,
}

impl InstanceExportLookup for ComponentExportIndex {
    fn lookup(&self, component: &Component) -> Option<ExportIndex> {
        if component.inner.id == self.id {
            Some(self.index)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::component::Component;
    use crate::{Config, Engine};
    use wasmtime_environ::MemoryInitialization;

    #[test]
    fn cow_on_by_default() {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config).unwrap();
        let component = Component::new(
            &engine,
            r#"
                (component
                    (core module
                        (memory 1)
                        (data (i32.const 100) "abcd")
                    )
                )
            "#,
        )
        .unwrap();

        for (_, module) in component.inner.static_modules.iter() {
            let init = &module.env_module().memory_initialization;
            assert!(matches!(init, MemoryInitialization::Static { .. }));
        }
    }
}

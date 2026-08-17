#[cfg(feature = "component-model-async")]
use crate::component::concurrent::Accessor;
use crate::component::func::HostFunc;
use crate::component::instance::RuntimeImport;
use crate::component::matching::{InstanceType, TypeChecker};
use crate::component::types;
use crate::component::{
    Component, ComponentNamedList, Instance, InstancePre, Lift, Lower, ResourceType, Val,
};
use crate::hash_map::HashMap;
use crate::prelude::*;
use crate::{AsContextMut, Engine, Module, StoreContextMut};
use alloc::sync::Arc;
use core::marker;
#[cfg(feature = "async")]
use core::{future::Future, pin::Pin};
use wasmtime_environ::PrimaryMap;
use wasmtime_environ::component::{NameMap, NameMapIntern};

/// A type used to instantiate [`Component`]s.
///
/// This type is used to both link components together as well as supply host
/// functionality to components. Values are defined in a [`Linker`] by their
/// import name and then components are instantiated with a [`Linker`] using the
/// names provided for name resolution of the component's imports.
///
/// # Names and Semver
///
/// Names defined in a [`Linker`] correspond to import names in the Component
/// Model. Names in the Component Model are allowed to be semver-qualified, for
/// example:
///
/// * `wasi:cli/stdout@0.2.0`
/// * `wasi:http/types@0.2.0-rc-2023-10-25`
/// * `my:custom/plugin@1.0.0-pre.2`
///
/// These version strings are taken into account when looking up names within a
/// [`Linker`]. You're allowed to define any number of versions within a
/// [`Linker`] still, for example you can define `a:b/c@0.2.0`, `a:b/c@0.2.1`,
/// and `a:b/c@0.3.0` all at the same time.
///
/// Specifically though when names are looked up within a linker, for example
/// during instantiation, semver-compatible names are automatically consulted.
/// This means that if you define `a:b/c@0.2.1` in a [`Linker`] but a component
/// imports `a:b/c@0.2.0` then that import will resolve to the `0.2.1` version.
///
/// This lookup behavior relies on hosts being well-behaved when using Semver,
/// specifically that interfaces once defined are never changed. This reflects
/// how Semver works at the Component Model layer, and it's assumed that if
/// versions are present then hosts are respecting this.
///
/// Note that this behavior goes the other direction, too. If a component
/// imports `a:b/c@0.2.1` and the host has provided `a:b/c@0.2.0` then that
/// will also resolve correctly. This is because if an API was defined at 0.2.0
/// and 0.2.1 then it must be the same API.
///
/// This behavior is intended to make it easier for hosts to upgrade WASI and
/// for guests to upgrade WASI. So long as the actual "meat" of the
/// functionality is defined then it should align correctly and components can
/// be instantiated.
pub struct Linker<T: 'static> {
    engine: Engine,
    strings: Strings,
    map: NameMap<usize, Definition>,
    path: Vec<usize>,
    allow_shadowing: bool,
    _marker: marker::PhantomData<fn() -> T>,
}

impl<T: 'static> Clone for Linker<T> {
    fn clone(&self) -> Linker<T> {
        Linker {
            engine: self.engine.clone(),
            strings: self.strings.clone(),
            map: self.map.clone(),
            path: self.path.clone(),
            allow_shadowing: self.allow_shadowing,
            _marker: self._marker,
        }
    }
}

#[derive(Clone, Default)]
pub struct Strings {
    string2idx: HashMap<Arc<str>, usize>,
    strings: Vec<Arc<str>>,
}

/// Structure representing an "instance" being defined within a linker.
///
/// Instances do not need to be actual [`Instance`]s and instead are defined by
/// a "bag of named items", so each [`LinkerInstance`] can further define items
/// internally.
pub struct LinkerInstance<'a, T: 'static> {
    engine: &'a Engine,
    path: &'a mut Vec<usize>,
    path_len: usize,
    strings: &'a mut Strings,
    map: &'a mut NameMap<usize, Definition>,
    allow_shadowing: bool,
    _marker: marker::PhantomData<fn() -> T>,
}

#[derive(Clone, Debug)]
pub(crate) enum Definition {
    Instance(NameMap<usize, Definition>),
    Func(Arc<HostFunc>),
    Module(Module),
    Resource(ResourceType, Arc<crate::func::HostFunc>),
}

impl<T: 'static> Linker<T> {
    /// Creates a new linker for the [`Engine`] specified with no items defined
    /// within it.
    pub fn new(engine: &Engine) -> Linker<T> {
        Linker {
            engine: engine.clone(),
            strings: Strings::default(),
            map: NameMap::default(),
            allow_shadowing: false,
            path: Vec::new(),
            _marker: marker::PhantomData,
        }
    }

    /// Returns the [`Engine`] this is connected to.
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Configures whether or not name-shadowing is allowed.
    ///
    /// By default name shadowing is not allowed and it's an error to redefine
    /// the same name within a linker.
    pub fn allow_shadowing(&mut self, allow: bool) -> &mut Self {
        self.allow_shadowing = allow;
        self
    }

    /// Returns the "root instance" of this linker, used to define names into
    /// the root namespace.
    pub fn root(&mut self) -> LinkerInstance<'_, T> {
        LinkerInstance {
            engine: &self.engine,
            path: &mut self.path,
            path_len: 0,
            strings: &mut self.strings,
            map: &mut self.map,
            allow_shadowing: self.allow_shadowing,
            _marker: self._marker,
        }
    }

    /// Returns a builder for the named instance specified.
    ///
    /// # Errors
    ///
    /// Returns an error if `name` is already defined within the linker.
    pub fn instance(&mut self, name: &str) -> Result<LinkerInstance<'_, T>> {
        self.root().into_instance(name)
    }

    fn typecheck<'a>(&'a self, component: &'a Component) -> Result<TypeChecker<'a>> {
        let mut cx = TypeChecker {
            engine: &self.engine,
            types: component.types(),
            strings: &self.strings,
            imported_resources: Default::default(),
        };

        // Walk over the component's list of import names and use that to lookup
        // the definition within this linker that it corresponds to. When found
        // perform a typecheck against the component's expected type.
        let env_component = component.env_component();
        for (_idx, (name, ty)) in env_component.import_types.iter() {
            let import = self.map.get(name, &self.strings);
            cx.definition(ty, import)
                .with_context(|| format!("component imports {desc} `{name}`, but a matching implementation was not found in the linker", desc = ty.desc()))?;
        }
        Ok(cx)
    }

    /// Returns the [`types::Component`] corresponding to `component` with resource
    /// types imported by it replaced using imports present in [`Self`].
    pub fn substituted_component_type(&self, component: &Component) -> Result<types::Component> {
        let cx = self.typecheck(&component)?;
        Ok(types::Component::from(
            component.ty(),
            &InstanceType {
                types: cx.types,
                resources: &cx.imported_resources,
            },
        ))
    }

    /// Performs a "pre-instantiation" to resolve the imports of the
    /// [`Component`] specified with the items defined within this linker.
    ///
    /// This method will perform as much work as possible short of actually
    /// instantiating an instance. Internally this will use the names defined
    /// within this linker to satisfy the imports of the [`Component`] provided.
    /// Additionally this will perform type-checks against the component's
    /// imports against all items defined within this linker.
    ///
    /// Note that unlike internally in components where subtyping at the
    /// interface-types layer is supported this is not supported here. Items
    /// defined in this linker must match the component's imports precisely.
    ///
    /// # Errors
    ///
    /// Returns an error if this linker doesn't define a name that the
    /// `component` imports or if a name defined doesn't match the type of the
    /// item imported by the `component` provided.
    pub fn instantiate_pre(&self, component: &Component) -> Result<InstancePre<T>> {
        let cx = self.typecheck(&component)?;

        // A successful typecheck resolves all of the imported resources used by
        // this InstancePre. We keep a clone of this table in the InstancePre
        // so that we can construct an InstanceType for typechecking.
        let imported_resources = cx.imported_resources.clone();

        // Now that all imports are known to be defined and satisfied by this
        // linker a list of "flat" import items (aka no instances) is created
        // using the import map within the component created at
        // component-compile-time.
        let env_component = component.env_component();
        let mut imports = PrimaryMap::with_capacity(env_component.imports.len());
        for (idx, (import, names)) in env_component.imports.iter() {
            let (root, _) = &env_component.import_types[*import];

            // This is the flattening process where we go from a definition
            // optionally through a list of exported names to get to the final
            // item.
            let mut cur = self.map.get(root, &self.strings).unwrap();
            for name in names {
                cur = match cur {
                    Definition::Instance(map) => map.get(&name, &self.strings).unwrap(),
                    _ => unreachable!(),
                };
            }
            let import = match cur {
                Definition::Module(m) => RuntimeImport::Module(m.clone()),
                Definition::Func(f) => RuntimeImport::Func(f.clone()),
                Definition::Resource(t, dtor) => RuntimeImport::Resource {
                    ty: *t,
                    _dtor: dtor.clone(),
                    dtor_funcref: component.resource_drop_func_ref(dtor),
                },

                // This is guaranteed by the compilation process that "leaf"
                // runtime imports are never instances.
                Definition::Instance(_) => unreachable!(),
            };
            let i = imports.push(import);
            assert_eq!(i, idx);
        }
        Ok(unsafe {
            InstancePre::new_unchecked(component.clone(), Arc::new(imports), imported_resources)
        })
    }

    /// Instantiates the [`Component`] provided into the `store` specified.
    ///
    /// This function will use the items defined within this [`Linker`] to
    /// satisfy the imports of the [`Component`] provided as necessary. For more
    /// information about this see [`Linker::instantiate_pre`] as well.
    ///
    /// # Errors
    ///
    /// Returns an error if this [`Linker`] doesn't define an import that
    /// `component` requires or if it is of the wrong type. Additionally this
    /// can return an error if something goes wrong during instantiation such as
    /// a runtime trap or a runtime limit being exceeded.
    pub fn instantiate(
        &self,
        store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        assert!(
            !store.as_context().async_support(),
            "must use async instantiation when async support is enabled"
        );
        self.instantiate_pre(component)?.instantiate(store)
    }

    /// Instantiates the [`Component`] provided into the `store` specified.
    ///
    /// This is exactly like [`Linker::instantiate`] except for async stores.
    ///
    /// # Errors
    ///
    /// Returns an error if this [`Linker`] doesn't define an import that
    /// `component` requires or if it is of the wrong type. Additionally this
    /// can return an error if something goes wrong during instantiation such as
    /// a runtime trap or a runtime limit being exceeded.
    #[cfg(feature = "async")]
    pub async fn instantiate_async(
        &self,
        store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance>
    where
        T: Send,
    {
        assert!(
            store.as_context().async_support(),
            "must use sync instantiation when async support is disabled"
        );
        self.instantiate_pre(component)?
            .instantiate_async(store)
            .await
    }

    /// Implement any imports of the given [`Component`] with a function which traps.
    ///
    /// By default a [`Linker`] will error when unknown imports are encountered when instantiating a [`Component`].
    /// This changes this behavior from an instant error to a trap that will happen if the import is called.
    pub fn define_unknown_imports_as_traps(&mut self, component: &Component) -> Result<()> {
        use wasmtime_environ::component::ComponentTypes;
        use wasmtime_environ::component::TypeDef;
        // Recursively stub out all imports of the component with a function that traps.
        fn stub_item<T>(
            linker: &mut LinkerInstance<T>,
            item_name: &str,
            item_def: &TypeDef,
            parent_instance: Option<&str>,
            types: &ComponentTypes,
        ) -> Result<()> {
            // Skip if the item isn't an instance and has already been defined in the linker.
            if !matches!(item_def, TypeDef::ComponentInstance(_)) && linker.get(item_name).is_some()
            {
                return Ok(());
            }

            match item_def {
                TypeDef::ComponentFunc(_) => {
                    let fully_qualified_name = parent_instance
                        .map(|parent| format!("{parent}#{item_name}"))
                        .unwrap_or_else(|| item_name.to_owned());
                    linker.func_new(&item_name, move |_, _, _| {
                        bail!("unknown import: `{fully_qualified_name}` has not been defined")
                    })?;
                }
                TypeDef::ComponentInstance(i) => {
                    let instance = &types[*i];
                    let mut linker_instance = linker.instance(item_name)?;
                    for (export_name, export) in instance.exports.iter() {
                        stub_item(
                            &mut linker_instance,
                            export_name,
                            export,
                            Some(item_name),
                            types,
                        )?;
                    }
                }
                TypeDef::Resource(_) => {
                    let ty = crate::component::ResourceType::host::<()>();
                    linker.resource(item_name, ty, |_, _| Ok(()))?;
                }
                TypeDef::Component(_) | TypeDef::Module(_) => {
                    bail!("unable to define {} imports as traps", item_def.desc())
                }
                _ => {}
            }
            Ok(())
        }

        for (_, (import_name, import_type)) in &component.env_component().import_types {
            stub_item(
                &mut self.root(),
                import_name,
                import_type,
                None,
                component.types(),
            )?;
        }
        Ok(())
    }
}

impl<T: 'static> LinkerInstance<'_, T> {
    fn as_mut(&mut self) -> LinkerInstance<'_, T> {
        LinkerInstance {
            engine: self.engine,
            path: self.path,
            path_len: self.path_len,
            strings: self.strings,
            map: self.map,
            allow_shadowing: self.allow_shadowing,
            _marker: self._marker,
        }
    }

    /// Defines a new host-provided function into this [`LinkerInstance`].
    ///
    /// This method is used to give host functions to wasm components. The
    /// `func` provided will be callable from linked components with the type
    /// signature dictated by `Params` and `Return`. The `Params` is a tuple of
    /// types that will come from wasm and `Return` is a value coming from the
    /// host going back to wasm.
    ///
    /// Additionally the `func` takes a
    /// [`StoreContextMut`](crate::StoreContextMut) as its first parameter.
    ///
    /// Note that `func` must be an `Fn` and must also be `Send + Sync +
    /// 'static`. Shared state within a func is typically accessed with the `T`
    /// type parameter from [`Store<T>`](crate::Store) which is accessible
    /// through the leading [`StoreContextMut<'_, T>`](crate::StoreContextMut)
    /// argument which can be provided to the `func` given here.
    ///
    /// # Blocking / Async Behavior
    ///
    /// The host function `func` provided here is a blocking function from the
    /// perspective of WebAssembly, regardless of how [`Config::async_support`]
    /// is defined. This function can be used both with sync and async stores
    /// and will always define a blocking function.
    ///
    /// To define a function which is async on the host, but blocking to the
    /// guest, see the [`func_wrap_async`] method.
    ///
    /// [`Config::async_support`]: crate::Config::async_support
    /// [`func_wrap_async`]: LinkerInstance::func_wrap_async
    //
    // TODO: needs more words and examples
    pub fn func_wrap<F, Params, Return>(&mut self, name: &str, func: F) -> Result<()>
    where
        F: Fn(StoreContextMut<T>, Params) -> Result<Return> + Send + Sync + 'static,
        Params: ComponentNamedList + Lift + 'static,
        Return: ComponentNamedList + Lower + 'static,
    {
        self.insert(name, Definition::Func(HostFunc::from_closure(func)))?;
        Ok(())
    }

    /// Defines a new host-provided async function into this [`LinkerInstance`].
    ///
    /// This function is similar to [`Self::func_wrap`] except it takes an async
    /// host function instead of a blocking host function. The `F` function here
    /// is intended to be:
    ///
    /// ```ignore
    /// F: AsyncFn(StoreContextMut<'_, T>, Params) -> Result<Return>
    /// ```
    ///
    /// however the returned future must be `Send` which is not possible to
    /// bound at this time. This will be switched to an async closure once Rust
    /// supports it.
    ///
    /// # Blocking / Async Behavior
    ///
    /// This function can only be called when [`Config::async_support`] is
    /// enabled. The function defined which WebAssembly calls will still appear
    /// as blocking from the perspective of WebAssembly itself. The host,
    /// however, can perform asynchronous operations without blocking the thread
    /// performing a call.
    ///
    /// When [`Config::async_support`] is enabled then all WebAssembly is
    /// invoked on a separate stack within a Wasmtime-managed fiber. This means
    /// that if the future returned by `F` is not immediately ready then the
    /// fiber will be suspended to block WebAssembly but not the host. When the
    /// future becomes ready again the fiber will be resumed to continue
    /// execution within WebAssembly.
    ///
    /// # Panics
    ///
    /// This function panics if [`Config::async_support`] is set to `false`.
    ///
    /// [`Config::async_support`]: crate::Config::async_support
    /// [`func_wrap_async`]: LinkerInstance::func_wrap_async
    #[cfg(feature = "async")]
    pub fn func_wrap_async<Params, Return, F>(&mut self, name: &str, f: F) -> Result<()>
    where
        F: Fn(
                StoreContextMut<'_, T>,
                Params,
            ) -> Box<dyn Future<Output = Result<Return>> + Send + '_>
            + Send
            + Sync
            + 'static,
        Params: ComponentNamedList + Lift + 'static,
        Return: ComponentNamedList + Lower + 'static,
    {
        assert!(
            self.engine.config().async_support,
            "cannot use `func_wrap_async` without enabling async support in the config"
        );
        let ff = move |store: StoreContextMut<'_, T>, params: Params| -> Result<Return> {
            store.block_on(|store| f(store, params).into())?
        };
        self.func_wrap(name, ff)
    }

    /// Defines a new host-provided async function into this [`LinkerInstance`].
    ///
    /// This function defines a host function available to call from
    /// WebAssembly. WebAssembly may additionally make multiple invocations of
    /// this function concurrently all at the same time. This function requires
    /// the [`Config::wasm_component_model_async`] feature to be enabled.
    ///
    /// The function `f` provided will be invoked when called by WebAssembly.
    /// WebAssembly components may then call `f` multiple times while previous
    /// invocations of `f` are already running. Additionally while `f` is
    /// running other host functions may be invoked.
    ///
    /// The `F` function here is intended to be:
    ///
    /// ```ignore
    /// F: AsyncFn(&Accessor<T>, Params) -> Result<Return>
    /// ```
    ///
    /// however the returned future must be `Send` which is not possible to
    /// bound at this time. This will be switched to an async closure once Rust
    /// supports it.
    ///
    /// The closure `f` is provided an [`Accessor`] which can be used to acquire
    /// temporary, blocking, access to a [`StoreContextMut`] (through
    /// [`Access`]). This models how a store is not available to `f` across
    /// `await` points but it is temporarily available while actively being
    /// polled.
    ///
    /// # Blocking / Async Behavior
    ///
    /// This function can only be called when [`Config::async_support`] is
    /// enabled. Unlike [`Self::func_wrap`] and [`Self::func_wrap_async`] this
    /// function is asynchronous even from the perspective of guest WebAssembly.
    /// This means that if `f` is not immediately resolved then the call from
    /// WebAssembly will still return immediately (assuming it was lowered with
    /// `async`). The closure `f` should not block the current thread and should
    /// only perform blocking via `async` meaning that `f` won't block either
    /// WebAssembly nor the host.
    ///
    /// Note that WebAssembly components can lower host functions both with and
    /// without `async`. That means that even if a host function is defined in
    /// the "concurrent" mode here a guest may still lower it synchronously. In
    /// this situation Wasmtime will manage blocking the guest while the closure
    /// `f` provided here completes. If a guest lowers this function with
    /// `async`, though, then no blocking will happen.
    ///
    /// # Panics
    ///
    /// This function panics if [`Config::async_support`] is set to `false`.
    ///
    /// [`Config::async_support`]: crate::Config::async_support
    /// [`Config::wasm_component_model_async`]: crate::Config::wasm_component_model_async
    /// [`func_wrap_async`]: LinkerInstance::func_wrap_async
    #[cfg(feature = "component-model-async")]
    pub fn func_wrap_concurrent<Params, Return, F>(&mut self, name: &str, f: F) -> Result<()>
    where
        T: 'static,
        F: Fn(&Accessor<T>, Params) -> Pin<Box<dyn Future<Output = Result<Return>> + Send + '_>>
            + Send
            + Sync
            + 'static,
        Params: ComponentNamedList + Lift + 'static,
        Return: ComponentNamedList + Lower + 'static,
    {
        assert!(
            self.engine.config().async_support,
            "cannot use `func_wrap_concurrent` without enabling async support in the config"
        );
        self.insert(name, Definition::Func(HostFunc::from_concurrent(f)))?;
        Ok(())
    }

    /// Define a new host-provided function using dynamically typed values.
    ///
    /// The `name` provided is the name of the function to define and the
    /// `func` provided is the host-defined closure to invoke when this
    /// function is called.
    ///
    /// This function is the "dynamic" version of defining a host function as
    /// compared to [`LinkerInstance::func_wrap`]. With
    /// [`LinkerInstance::func_wrap`] a function's type is statically known but
    /// with this method the `func` argument's type isn't known ahead of time.
    /// That means that `func` can be by imported component so long as it's
    /// imported as a matching name.
    ///
    /// Type information will be available at execution time, however. For
    /// example when `func` is invoked the second argument, a `&[Val]` list,
    /// contains [`Val`] entries that say what type they are. Additionally the
    /// third argument, `&mut [Val]`, is the expected number of results. Note
    /// that the expected types of the results cannot be learned during the
    /// execution of `func`. Learning that would require runtime introspection
    /// of a component.
    ///
    /// Return values, stored in the third argument of `&mut [Val]`, are
    /// type-checked at runtime to ensure that they have the appropriate type.
    /// A trap will be raised if they do not have the right type.
    ///
    /// # Examples
    ///
    /// ```
    /// use wasmtime::{Store, Engine};
    /// use wasmtime::component::{Component, Linker, Val};
    ///
    /// # fn main() -> wasmtime::Result<()> {
    /// let engine = Engine::default();
    /// let component = Component::new(
    ///     &engine,
    ///     r#"
    ///         (component
    ///             (import "thunk" (func $thunk))
    ///             (import "is-even" (func $is-even (param "x" u32) (result bool)))
    ///
    ///             (core module $m
    ///                 (import "" "thunk" (func $thunk))
    ///                 (import "" "is-even" (func $is-even (param i32) (result i32)))
    ///
    ///                 (func (export "run")
    ///                     call $thunk
    ///
    ///                     (call $is-even (i32.const 1))
    ///                     if unreachable end
    ///
    ///                     (call $is-even (i32.const 2))
    ///                     i32.eqz
    ///                     if unreachable end
    ///                 )
    ///             )
    ///             (core func $thunk (canon lower (func $thunk)))
    ///             (core func $is-even (canon lower (func $is-even)))
    ///             (core instance $i (instantiate $m
    ///                 (with "" (instance
    ///                     (export "thunk" (func $thunk))
    ///                     (export "is-even" (func $is-even))
    ///                 ))
    ///             ))
    ///
    ///             (func (export "run") (canon lift (core func $i "run")))
    ///         )
    ///     "#,
    /// )?;
    ///
    /// let mut linker = Linker::<()>::new(&engine);
    ///
    /// // Sample function that takes no arguments.
    /// linker.root().func_new("thunk", |_store, params, results| {
    ///     assert!(params.is_empty());
    ///     assert!(results.is_empty());
    ///     println!("Look ma, host hands!");
    ///     Ok(())
    /// })?;
    ///
    /// // This function takes one argument and returns one result.
    /// linker.root().func_new("is-even", |_store, params, results| {
    ///     assert_eq!(params.len(), 1);
    ///     let param = match params[0] {
    ///         Val::U32(n) => n,
    ///         _ => panic!("unexpected type"),
    ///     };
    ///
    ///     assert_eq!(results.len(), 1);
    ///     results[0] = Val::Bool(param % 2 == 0);
    ///     Ok(())
    /// })?;
    ///
    /// let mut store = Store::new(&engine, ());
    /// let instance = linker.instantiate(&mut store, &component)?;
    /// let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;
    /// run.call(&mut store, ())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn func_new(
        &mut self,
        name: &str,
        func: impl Fn(StoreContextMut<'_, T>, &[Val], &mut [Val]) -> Result<()> + Send + Sync + 'static,
    ) -> Result<()> {
        self.insert(name, Definition::Func(HostFunc::new_dynamic(func)))?;
        Ok(())
    }

    /// Define a new host-provided async function using dynamic types.
    ///
    /// As [`Self::func_wrap_async`] is a dual of [`Self::func_wrap`], this
    /// function is the dual of [`Self::func_new`].
    ///
    /// For documentation on blocking behavior see [`Self::func_wrap_async`].
    #[cfg(feature = "async")]
    pub fn func_new_async<F>(&mut self, name: &str, f: F) -> Result<()>
    where
        F: for<'a> Fn(
                StoreContextMut<'a, T>,
                &'a [Val],
                &'a mut [Val],
            ) -> Box<dyn Future<Output = Result<()>> + Send + 'a>
            + Send
            + Sync
            + 'static,
    {
        assert!(
            self.engine.config().async_support,
            "cannot use `func_new_async` without enabling async support in the config"
        );
        let ff = move |store: StoreContextMut<'_, T>, params: &[Val], results: &mut [Val]| {
            store.with_blocking(|store, cx| cx.block_on(Pin::from(f(store, params, results)))?)
        };
        return self.func_new(name, ff);
    }

    /// Define a new host-provided async function using dynamic types.
    ///
    /// As [`Self::func_wrap_concurrent`] is a dual of [`Self::func_wrap`], this
    /// function is the dual of [`Self::func_new`].
    ///
    /// For documentation on async/blocking behavior see
    /// [`Self::func_wrap_concurrent`].
    #[cfg(feature = "component-model-async")]
    pub fn func_new_concurrent<F>(&mut self, name: &str, f: F) -> Result<()>
    where
        T: 'static,
        F: for<'a> Fn(
                &'a Accessor<T>,
                &'a [Val],
                &'a mut [Val],
            ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>
            + Send
            + Sync
            + 'static,
    {
        assert!(
            self.engine.config().async_support,
            "cannot use `func_wrap_concurrent` without enabling async support in the config"
        );
        self.insert(name, Definition::Func(HostFunc::new_dynamic_concurrent(f)))?;
        Ok(())
    }

    /// Defines a [`Module`] within this instance.
    ///
    /// This can be used to provide a core wasm [`Module`] as an import to a
    /// component. The [`Module`] provided is saved within the linker for the
    /// specified `name` in this instance.
    pub fn module(&mut self, name: &str, module: &Module) -> Result<()> {
        self.insert(name, Definition::Module(module.clone()))?;
        Ok(())
    }

    /// Defines a new resource of a given [`ResourceType`] in this linker.
    ///
    /// This function is used to specify resources defined in the host.
    ///
    /// The `name` argument is the name to define the resource within this
    /// linker.
    ///
    /// The `dtor` provided is a destructor that will get invoked when an owned
    /// version of this resource is destroyed from the guest. Note that this
    /// destructor is not called when a host-owned resource is destroyed as it's
    /// assumed the host knows how to handle destroying its own resources.
    ///
    /// The `dtor` closure is provided the store state as the first argument
    /// along with the representation of the resource that was just destroyed.
    ///
    /// [`Resource<U>`]: crate::component::Resource
    ///
    /// # Errors
    ///
    /// The provided `dtor` closure returns an error if something goes wrong
    /// when a guest calls the `dtor` to drop a `Resource<T>` such as
    /// a runtime trap or a runtime limit being exceeded.
    pub fn resource(
        &mut self,
        name: &str,
        ty: ResourceType,
        dtor: impl Fn(StoreContextMut<'_, T>, u32) -> Result<()> + Send + Sync + 'static,
    ) -> Result<()> {
        let dtor = Arc::new(crate::func::HostFunc::wrap_inner(
            &self.engine,
            move |mut cx: crate::Caller<'_, T>, (param,): (u32,)| dtor(cx.as_context_mut(), param),
        ));
        self.insert(name, Definition::Resource(ty, dtor))?;
        Ok(())
    }

    /// Identical to [`Self::resource`], except that it takes an async destructor.
    #[cfg(feature = "async")]
    pub fn resource_async<F>(&mut self, name: &str, ty: ResourceType, dtor: F) -> Result<()>
    where
        F: Fn(StoreContextMut<'_, T>, u32) -> Box<dyn Future<Output = Result<()>> + Send + '_>
            + Send
            + Sync
            + 'static,
    {
        assert!(
            self.engine.config().async_support,
            "cannot use `resource_async` without enabling async support in the config"
        );
        let dtor = Arc::new(crate::func::HostFunc::wrap_inner(
            &self.engine,
            move |mut cx: crate::Caller<'_, T>, (param,): (u32,)| {
                cx.as_context_mut()
                    .block_on(|store| dtor(store, param).into())?
            },
        ));
        self.insert(name, Definition::Resource(ty, dtor))?;
        Ok(())
    }

    /// Identical to [`Self::resource`], except that it takes a concurrent destructor.
    #[cfg(feature = "component-model-async")]
    pub fn resource_concurrent<F>(&mut self, name: &str, ty: ResourceType, dtor: F) -> Result<()>
    where
        T: Send + 'static,
        F: Fn(&Accessor<T>, u32) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>
            + Send
            + Sync
            + 'static,
    {
        assert!(
            self.engine.config().async_support,
            "cannot use `resource_concurrent` without enabling async support in the config"
        );
        // TODO: This isn't really concurrent -- it requires exclusive access to
        // the store for the duration of the call, preventing guest code from
        // running until it completes.  We should make it concurrent and clean
        // up the implementation to avoid using e.g. `Accessor::new` and
        // `tls::set` directly.
        let dtor = Arc::new(dtor);
        let dtor = Arc::new(crate::func::HostFunc::wrap_inner(
            &self.engine,
            move |mut cx: crate::Caller<'_, T>, (param,): (u32,)| {
                let dtor = dtor.clone();
                cx.as_context_mut().block_on(move |mut store| {
                    Box::pin(async move {
                        // NOTE: We currently pass `None` as the `instance`
                        // parameter to `Accessor::new` because we don't have ready
                        // access to it, meaning `dtor` will panic if it tries to
                        // use `Accessor::instance`.  We could plumb that through
                        // from the `wasmtime-cranelift`-generated code, but we plan
                        // to remove `Accessor::instance` once all instances in a
                        // store share the same concurrent state, at which point we
                        // won't need it anyway.
                        let accessor = &Accessor::new(
                            crate::store::StoreToken::new(store.as_context_mut()),
                            None,
                        );
                        let mut future = std::pin::pin!(dtor(accessor, param));
                        std::future::poll_fn(|cx| {
                            crate::component::concurrent::tls::set(store.0.traitobj_mut(), || {
                                future.as_mut().poll(cx)
                            })
                        })
                        .await
                    })
                })?
            },
        ));
        self.insert(name, Definition::Resource(ty, dtor))?;
        Ok(())
    }

    /// Defines a nested instance within this instance.
    ///
    /// This can be used to describe arbitrarily nested levels of instances
    /// within a linker to satisfy nested instance exports of components.
    pub fn instance(&mut self, name: &str) -> Result<LinkerInstance<'_, T>> {
        self.as_mut().into_instance(name)
    }

    /// Same as [`LinkerInstance::instance`] except with different lifetime
    /// parameters.
    pub fn into_instance(mut self, name: &str) -> Result<Self> {
        let name = self.insert(name, Definition::Instance(NameMap::default()))?;
        self.map = match self.map.raw_get_mut(&name) {
            Some(Definition::Instance(map)) => map,
            _ => unreachable!(),
        };
        self.path.truncate(self.path_len);
        self.path.push(name);
        self.path_len += 1;
        Ok(self)
    }

    fn insert(&mut self, name: &str, item: Definition) -> Result<usize> {
        self.map
            .insert(name, self.strings, self.allow_shadowing, item)
    }

    fn get(&self, name: &str) -> Option<&Definition> {
        self.map.get(name, self.strings)
    }
}

impl NameMapIntern for Strings {
    type Key = usize;

    fn intern(&mut self, string: &str) -> usize {
        if let Some(idx) = self.string2idx.get(string) {
            return *idx;
        }
        let string: Arc<str> = string.into();
        let idx = self.strings.len();
        self.strings.push(string.clone());
        self.string2idx.insert(string, idx);
        idx
    }

    fn lookup(&self, string: &str) -> Option<usize> {
        self.string2idx.get(string).cloned()
    }
}

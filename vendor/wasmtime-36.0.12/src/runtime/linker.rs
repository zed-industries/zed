use crate::func::HostFunc;
use crate::hash_map::{Entry, HashMap};
use crate::instance::InstancePre;
use crate::store::StoreOpaque;
use crate::{
    AsContext, AsContextMut, Caller, Engine, Extern, ExternType, Func, FuncType, ImportType,
    Instance, Module, StoreContextMut, Val, ValRaw,
};
use crate::{IntoFunc, prelude::*};
use alloc::sync::Arc;
use core::fmt::{self, Debug};
#[cfg(feature = "async")]
use core::future::Future;
use core::marker;
use log::warn;

/// Structure used to link wasm modules/instances together.
///
/// This structure is used to assist in instantiating a [`Module`]. A [`Linker`]
/// is a way of performing name resolution to make instantiating a module easier
/// than specifying positional imports to [`Instance::new`]. [`Linker`] is a
/// name-based resolver where names are dynamically defined and then used to
/// instantiate a [`Module`].
///
/// An important method is [`Linker::instantiate`] which takes a module to
/// instantiate into the provided store. This method will automatically select
/// all the right imports for the [`Module`] to be instantiated, and will
/// otherwise return an error if an import isn't satisfied.
///
/// ## Name Resolution
///
/// As mentioned previously, `Linker` is a form of name resolver. It will be
/// using the string-based names of imports on a module to attempt to select a
/// matching item to hook up to it. This name resolution has two-levels of
/// namespaces, a module level and a name level. Each item is defined within a
/// module and then has its own name. This basically follows the wasm standard
/// for modularization.
///
/// Names in a `Linker` cannot be defined twice, but allowing duplicates by
/// shadowing the previous definition can be controlled with the
/// [`Linker::allow_shadowing`] method.
///
/// ## Commands and Reactors
///
/// The [`Linker`] type provides conveniences for working with WASI Commands and
/// Reactors through the [`Linker::module`] method. This will automatically
/// handle instantiation and calling `_start` and such as appropriate
/// depending on the inferred type of module.
///
/// ## Type parameter `T`
///
/// It's worth pointing out that the type parameter `T` on [`Linker<T>`] does
/// not represent that `T` is stored within a [`Linker`]. Rather the `T` is used
/// to ensure that linker-defined functions and stores instantiated into all use
/// the same matching `T` as host state.
///
/// ## Multiple `Store`s
///
/// The [`Linker`] type is designed to be compatible, in some scenarios, with
/// instantiation in multiple [`Store`]s. Specifically host-defined functions
/// created in [`Linker`] with [`Linker::func_new`], [`Linker::func_wrap`], and
/// their async versions are compatible to instantiate into any [`Store`]. This
/// enables programs which want to instantiate lots of modules to create one
/// [`Linker`] value at program start up and use that continuously for each
/// [`Store`] created over the lifetime of the program.
///
/// Note that once [`Store`]-owned items, such as [`Global`], are defined within
/// a [`Linker`] then it is no longer compatible with any [`Store`]. At that
/// point only the [`Store`] that owns the [`Global`] can be used to instantiate
/// modules.
///
/// ## Multiple `Engine`s
///
/// The [`Linker`] type is not compatible with usage between multiple [`Engine`]
/// values. An [`Engine`] is provided when a [`Linker`] is created and only
/// stores and items which originate from that [`Engine`] can be used with this
/// [`Linker`]. If more than one [`Engine`] is used with a [`Linker`] then that
/// may cause a panic at runtime, similar to how if a [`Func`] is used with the
/// wrong [`Store`] that can also panic at runtime.
///
/// [`Store`]: crate::Store
/// [`Global`]: crate::Global
pub struct Linker<T> {
    engine: Engine,
    string2idx: HashMap<Arc<str>, usize>,
    strings: Vec<Arc<str>>,
    map: HashMap<ImportKey, Definition>,
    allow_shadowing: bool,
    allow_unknown_exports: bool,
    _marker: marker::PhantomData<fn() -> T>,
}

impl<T> Debug for Linker<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Linker").finish_non_exhaustive()
    }
}

impl<T> Clone for Linker<T> {
    fn clone(&self) -> Linker<T> {
        Linker {
            engine: self.engine.clone(),
            string2idx: self.string2idx.clone(),
            strings: self.strings.clone(),
            map: self.map.clone(),
            allow_shadowing: self.allow_shadowing,
            allow_unknown_exports: self.allow_unknown_exports,
            _marker: self._marker,
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct ImportKey {
    name: usize,
    module: usize,
}

#[derive(Clone)]
pub(crate) enum Definition {
    Extern(Extern, DefinitionType),
    HostFunc(Arc<HostFunc>),
}

/// This is a sort of slimmed down `ExternType` which notably doesn't have a
/// `FuncType`, which is an allocation, and additionally retains the current
/// size of the table/memory.
#[derive(Clone, Debug)]
pub(crate) enum DefinitionType {
    Func(wasmtime_environ::VMSharedTypeIndex),
    Global(wasmtime_environ::Global),
    // Note that tables and memories store not only the original type
    // information but additionally the current size of the table/memory, as
    // this is used during linking since the min size specified in the type may
    // no longer be the current size of the table/memory.
    Table(wasmtime_environ::Table, u64),
    Memory(wasmtime_environ::Memory, u64),
    Tag(wasmtime_environ::Tag),
}

impl<T> Linker<T> {
    /// Creates a new [`Linker`].
    ///
    /// The linker will define functions within the context of the `engine`
    /// provided and can only instantiate modules for a [`Store`][crate::Store]
    /// that is also defined within the same [`Engine`]. Usage of stores with
    /// different [`Engine`]s may cause a panic when used with this [`Linker`].
    pub fn new(engine: &Engine) -> Linker<T> {
        Linker {
            engine: engine.clone(),
            map: HashMap::new(),
            string2idx: HashMap::new(),
            strings: Vec::new(),
            allow_shadowing: false,
            allow_unknown_exports: false,
            _marker: marker::PhantomData,
        }
    }

    /// Returns the [`Engine`] this is connected to.
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Configures whether this [`Linker`] will shadow previous duplicate
    /// definitions of the same signature.
    ///
    /// By default a [`Linker`] will disallow duplicate definitions of the same
    /// signature. This method, however, can be used to instead allow duplicates
    /// and have the latest definition take precedence when linking modules.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let mut linker = Linker::<()>::new(&engine);
    /// linker.func_wrap("", "", || {})?;
    ///
    /// // by default, duplicates are disallowed
    /// assert!(linker.func_wrap("", "", || {}).is_err());
    ///
    /// // but shadowing can be configured to be allowed as well
    /// linker.allow_shadowing(true);
    /// linker.func_wrap("", "", || {})?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn allow_shadowing(&mut self, allow: bool) -> &mut Self {
        self.allow_shadowing = allow;
        self
    }

    /// Configures whether this [`Linker`] will allow unknown exports from
    /// command modules.
    ///
    /// By default a [`Linker`] will error when unknown exports are encountered
    /// in a command module while using [`Linker::module`].
    ///
    /// This method can be used to allow unknown exports from command modules.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let module = Module::new(&engine, "(module)")?;
    /// # let mut store = Store::new(&engine, ());
    /// let mut linker = Linker::new(&engine);
    /// linker.allow_unknown_exports(true);
    /// linker.module(&mut store, "mod", &module)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn allow_unknown_exports(&mut self, allow: bool) -> &mut Self {
        self.allow_unknown_exports = allow;
        self
    }

    /// Implement any imports of the given [`Module`] with a function which traps.
    ///
    /// By default a [`Linker`] will error when unknown imports are encountered
    /// in a command module while using [`Linker::module`].
    ///
    /// This method can be used to allow unknown imports from command modules.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let module = Module::new(&engine, "(module (import \"unknown\" \"import\" (func)))")?;
    /// # let mut store = Store::new(&engine, ());
    /// let mut linker = Linker::new(&engine);
    /// linker.define_unknown_imports_as_traps(&module)?;
    /// linker.instantiate(&mut store, &module)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn define_unknown_imports_as_traps(&mut self, module: &Module) -> anyhow::Result<()>
    where
        T: 'static,
    {
        for import in module.imports() {
            if let Err(import_err) = self._get_by_import(&import) {
                if let ExternType::Func(func_ty) = import_err.ty() {
                    self.func_new(import.module(), import.name(), func_ty, move |_, _, _| {
                        bail!(import_err.clone());
                    })?;
                }
            }
        }
        Ok(())
    }

    /// Implement any function imports of the [`Module`] with a function that
    /// ignores its arguments and returns default values.
    ///
    /// Default values are either zero or null, depending on the value type.
    ///
    /// This method can be used to allow unknown imports from command modules.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let module = Module::new(&engine, "(module (import \"unknown\" \"import\" (func)))")?;
    /// # let mut store = Store::new(&engine, ());
    /// let mut linker = Linker::new(&engine);
    /// linker.define_unknown_imports_as_default_values(&mut store, &module)?;
    /// linker.instantiate(&mut store, &module)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn define_unknown_imports_as_default_values(
        &mut self,
        store: &mut impl AsContextMut<Data = T>,
        module: &Module,
    ) -> anyhow::Result<()>
    where
        T: 'static,
    {
        for import in module.imports() {
            if let Err(import_err) = self._get_by_import(&import) {
                let default_extern =
                    import_err
                        .ty()
                        .default_value(&mut *store)
                        .with_context(|| {
                            anyhow!(
                                "no default value exists for `{}::{}` with type `{:?}`",
                                import.module(),
                                import.name(),
                                import_err.ty(),
                            )
                        })?;
                self.define(
                    store.as_context(),
                    import.module(),
                    import.name(),
                    default_extern,
                )?;
            }
        }
        Ok(())
    }

    /// Defines a new item in this [`Linker`].
    ///
    /// This method will add a new definition, by name, to this instance of
    /// [`Linker`]. The `module` and `name` provided are what to name the
    /// `item`.
    ///
    /// # Errors
    ///
    /// Returns an error if the `module` and `name` already identify an item
    /// of the same type as the `item` provided and if shadowing is disallowed.
    /// For more information see the documentation on [`Linker`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let mut store = Store::new(&engine, ());
    /// let mut linker = Linker::new(&engine);
    /// let ty = GlobalType::new(ValType::I32, Mutability::Const);
    /// let global = Global::new(&mut store, ty, Val::I32(0x1234))?;
    /// linker.define(&store, "host", "offset", global)?;
    ///
    /// let wat = r#"
    ///     (module
    ///         (import "host" "offset" (global i32))
    ///         (memory 1)
    ///         (data (global.get 0) "foo")
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    /// linker.instantiate(&mut store, &module)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn define(
        &mut self,
        store: impl AsContext<Data = T>,
        module: &str,
        name: &str,
        item: impl Into<Extern>,
    ) -> Result<&mut Self>
    where
        T: 'static,
    {
        let store = store.as_context();
        let key = self.import_key(module, Some(name));
        self.insert(key, Definition::new(store.0, item.into()))?;
        Ok(self)
    }

    /// Same as [`Linker::define`], except only the name of the import is
    /// provided, not a module name as well.
    ///
    /// This is only relevant when working with the module linking proposal
    /// where one-level names are allowed (in addition to two-level names).
    /// Otherwise this method need not be used.
    pub fn define_name(
        &mut self,
        store: impl AsContext<Data = T>,
        name: &str,
        item: impl Into<Extern>,
    ) -> Result<&mut Self>
    where
        T: 'static,
    {
        let store = store.as_context();
        let key = self.import_key(name, None);
        self.insert(key, Definition::new(store.0, item.into()))?;
        Ok(self)
    }

    /// Creates a [`Func::new`]-style function named in this linker.
    ///
    /// For more information see [`Linker::func_wrap`].
    ///
    /// # Panics
    ///
    /// Panics if the given function type is not associated with the same engine
    /// as this linker.
    pub fn func_new(
        &mut self,
        module: &str,
        name: &str,
        ty: FuncType,
        func: impl Fn(Caller<'_, T>, &[Val], &mut [Val]) -> Result<()> + Send + Sync + 'static,
    ) -> Result<&mut Self>
    where
        T: 'static,
    {
        assert!(ty.comes_from_same_engine(self.engine()));
        let func = HostFunc::new(&self.engine, ty, func);
        let key = self.import_key(module, Some(name));
        self.insert(key, Definition::HostFunc(Arc::new(func)))?;
        Ok(self)
    }

    /// Creates a [`Func::new_unchecked`]-style function named in this linker.
    ///
    /// For more information see [`Linker::func_wrap`].
    ///
    /// # Panics
    ///
    /// Panics if the given function type is not associated with the same engine
    /// as this linker.
    ///
    /// # Safety
    ///
    /// See [`Func::new_unchecked`] for more safety information.
    pub unsafe fn func_new_unchecked(
        &mut self,
        module: &str,
        name: &str,
        ty: FuncType,
        func: impl Fn(Caller<'_, T>, &mut [ValRaw]) -> Result<()> + Send + Sync + 'static,
    ) -> Result<&mut Self>
    where
        T: 'static,
    {
        assert!(ty.comes_from_same_engine(self.engine()));
        // SAFETY: the contract of this function is the same as `new_unchecked`.
        let func = unsafe { HostFunc::new_unchecked(&self.engine, ty, func) };
        let key = self.import_key(module, Some(name));
        self.insert(key, Definition::HostFunc(Arc::new(func)))?;
        Ok(self)
    }

    /// Creates a [`Func::new_async`]-style function named in this linker.
    ///
    /// For more information see [`Linker::func_wrap`].
    ///
    /// # Panics
    ///
    /// This method panics in the following situations:
    ///
    /// * This linker is not associated with an [async
    ///   config](crate::Config::async_support).
    ///
    /// * If the given function type is not associated with the same engine as
    ///   this linker.
    #[cfg(all(feature = "async", feature = "cranelift"))]
    pub fn func_new_async<F>(
        &mut self,
        module: &str,
        name: &str,
        ty: FuncType,
        func: F,
    ) -> Result<&mut Self>
    where
        F: for<'a> Fn(
                Caller<'a, T>,
                &'a [Val],
                &'a mut [Val],
            ) -> Box<dyn Future<Output = Result<()>> + Send + 'a>
            + Send
            + Sync
            + 'static,
        T: 'static,
    {
        assert!(
            self.engine.config().async_support,
            "cannot use `func_new_async` without enabling async support in the config"
        );
        assert!(ty.comes_from_same_engine(self.engine()));
        self.func_new(module, name, ty, move |caller, params, results| {
            let instance = caller.caller();
            caller.store.with_blocking(|store, cx| {
                let caller = Caller::new(store, instance);
                cx.block_on(core::pin::Pin::from(func(caller, params, results)))
            })?
        })
    }

    /// Define a host function within this linker.
    ///
    /// For information about how the host function operates, see
    /// [`Func::wrap`]. That includes information about translating Rust types
    /// to WebAssembly native types.
    ///
    /// This method creates a host-provided function in this linker under the
    /// provided name. This method is distinct in its capability to create a
    /// [`Store`](crate::Store)-independent function. This means that the
    /// function defined here can be used to instantiate instances in multiple
    /// different stores, or in other words the function can be loaded into
    /// different stores.
    ///
    /// Note that the capability mentioned here applies to all other
    /// host-function-defining-methods on [`Linker`] as well. All of them can be
    /// used to create instances of [`Func`] within multiple stores. In a
    /// multithreaded program, for example, this means that the host functions
    /// could be called concurrently if different stores are executing on
    /// different threads.
    ///
    /// # Errors
    ///
    /// Returns an error if the `module` and `name` already identify an item
    /// of the same type as the `item` provided and if shadowing is disallowed.
    /// For more information see the documentation on [`Linker`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let mut linker = Linker::new(&engine);
    /// linker.func_wrap("host", "double", |x: i32| x * 2)?;
    /// linker.func_wrap("host", "log_i32", |x: i32| println!("{}", x))?;
    /// linker.func_wrap("host", "log_str", |caller: Caller<'_, ()>, ptr: i32, len: i32| {
    ///     // ...
    /// })?;
    ///
    /// let wat = r#"
    ///     (module
    ///         (import "host" "double" (func (param i32) (result i32)))
    ///         (import "host" "log_i32" (func (param i32)))
    ///         (import "host" "log_str" (func (param i32 i32)))
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    ///
    /// // instantiate in multiple different stores
    /// for _ in 0..10 {
    ///     let mut store = Store::new(&engine, ());
    ///     linker.instantiate(&mut store, &module)?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn func_wrap<Params, Args>(
        &mut self,
        module: &str,
        name: &str,
        func: impl IntoFunc<T, Params, Args>,
    ) -> Result<&mut Self>
    where
        T: 'static,
    {
        let func = HostFunc::wrap(&self.engine, func);
        let key = self.import_key(module, Some(name));
        self.insert(key, Definition::HostFunc(Arc::new(func)))?;
        Ok(self)
    }

    /// Asynchronous analog of [`Linker::func_wrap`].
    #[cfg(feature = "async")]
    pub fn func_wrap_async<F, Params: crate::WasmTyList, Args: crate::WasmRet>(
        &mut self,
        module: &str,
        name: &str,
        func: F,
    ) -> Result<&mut Self>
    where
        F: for<'a> Fn(Caller<'a, T>, Params) -> Box<dyn Future<Output = Args> + Send + 'a>
            + Send
            + Sync
            + 'static,
        T: 'static,
    {
        assert!(
            self.engine.config().async_support,
            "cannot use `func_wrap_async` without enabling async support on the config",
        );
        let func =
            HostFunc::wrap_inner(&self.engine, move |caller: Caller<'_, T>, args: Params| {
                let instance = caller.caller();
                let result = caller.store.block_on(|store| {
                    let caller = Caller::new(store, instance);
                    func(caller, args).into()
                });
                match result {
                    Ok(ret) => ret.into_fallible(),
                    Err(e) => Args::fallible_from_error(e),
                }
            });
        let key = self.import_key(module, Some(name));
        self.insert(key, Definition::HostFunc(Arc::new(func)))?;
        Ok(self)
    }

    /// Convenience wrapper to define an entire [`Instance`] in this linker.
    ///
    /// This function is a convenience wrapper around [`Linker::define`] which
    /// will define all exports on `instance` into this linker. The module name
    /// for each export is `module_name`, and the name for each export is the
    /// name in the instance itself.
    ///
    /// Note that when this API is used the [`Linker`] is no longer compatible
    /// with multi-[`Store`][crate::Store] instantiation because the items
    /// defined within this store will belong to the `store` provided, and only
    /// the `store` provided.
    ///
    /// # Errors
    ///
    /// Returns an error if the any item is redefined twice in this linker (for
    /// example the same `module_name` was already defined) and shadowing is
    /// disallowed, or if `instance` comes from a different
    /// [`Store`](crate::Store) than this [`Linker`] originally was created
    /// with.
    ///
    /// # Panics
    ///
    /// Panics if `instance` does not belong to `store`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let mut store = Store::new(&engine, ());
    /// let mut linker = Linker::new(&engine);
    ///
    /// // Instantiate a small instance...
    /// let wat = r#"(module (func (export "run") ))"#;
    /// let module = Module::new(&engine, wat)?;
    /// let instance = linker.instantiate(&mut store, &module)?;
    ///
    /// // ... and inform the linker that the name of this instance is
    /// // `instance1`. This defines the `instance1::run` name for our next
    /// // module to use.
    /// linker.instance(&mut store, "instance1", instance)?;
    ///
    /// let wat = r#"
    ///     (module
    ///         (import "instance1" "run" (func $instance1_run))
    ///         (func (export "run")
    ///             call $instance1_run
    ///         )
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    /// let instance = linker.instantiate(&mut store, &module)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn instance(
        &mut self,
        mut store: impl AsContextMut<Data = T>,
        module_name: &str,
        instance: Instance,
    ) -> Result<&mut Self>
    where
        T: 'static,
    {
        let mut store = store.as_context_mut();
        let exports = instance
            .exports(&mut store)
            .map(|e| {
                (
                    self.import_key(module_name, Some(e.name())),
                    e.into_extern(),
                )
            })
            .collect::<Vec<_>>();
        for (key, export) in exports {
            self.insert(key, Definition::new(store.0, export))?;
        }
        Ok(self)
    }

    /// Define automatic instantiations of a [`Module`] in this linker.
    ///
    /// This automatically handles [Commands and Reactors] instantiation and
    /// initialization.
    ///
    /// Exported functions of a Command module may be called directly, however
    /// instead of having a single instance which is reused for each call,
    /// each call creates a new instance, which lives for the duration of the
    /// call. The imports of the Command are resolved once, and reused for
    /// each instantiation, so all dependencies need to be present at the time
    /// when `Linker::module` is called.
    ///
    /// For Reactors, a single instance is created, and an initialization
    /// function is called, and then its exports may be called.
    ///
    /// Ordinary modules which don't declare themselves to be either Commands
    /// or Reactors are treated as Reactors without any initialization calls.
    ///
    /// [Commands and Reactors]: https://github.com/WebAssembly/WASI/blob/main/legacy/application-abi.md#current-unstable-abi
    ///
    /// # Errors
    ///
    /// Returns an error if the any item is redefined twice in this linker (for
    /// example the same `module_name` was already defined) and shadowing is
    /// disallowed, if `instance` comes from a different
    /// [`Store`](crate::Store) than this [`Linker`] originally was created
    /// with, or if a Reactor initialization function traps.
    ///
    /// # Panics
    ///
    /// Panics if any item used to instantiate the provided [`Module`] is not
    /// owned by `store`, or if the `store` provided comes from a different
    /// [`Engine`] than this [`Linker`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let mut store = Store::new(&engine, ());
    /// let mut linker = Linker::new(&engine);
    ///
    /// // Instantiate a small instance and inform the linker that the name of
    /// // this instance is `instance1`. This defines the `instance1::run` name
    /// // for our next module to use.
    /// let wat = r#"(module (func (export "run") ))"#;
    /// let module = Module::new(&engine, wat)?;
    /// linker.module(&mut store, "instance1", &module)?;
    ///
    /// let wat = r#"
    ///     (module
    ///         (import "instance1" "run" (func $instance1_run))
    ///         (func (export "run")
    ///             call $instance1_run
    ///         )
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    /// let instance = linker.instantiate(&mut store, &module)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For a Command, a new instance is created for each call.
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let mut store = Store::new(&engine, ());
    /// let mut linker = Linker::new(&engine);
    ///
    /// // Create a Command that attempts to count the number of times it is run, but is
    /// // foiled by each call getting a new instance.
    /// let wat = r#"
    ///     (module
    ///         (global $counter (mut i32) (i32.const 0))
    ///         (func (export "_start")
    ///             (global.set $counter (i32.add (global.get $counter) (i32.const 1)))
    ///         )
    ///         (func (export "read_counter") (result i32)
    ///             (global.get $counter)
    ///         )
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    /// linker.module(&mut store, "commander", &module)?;
    /// let run = linker.get_default(&mut store, "")?
    ///     .typed::<(), ()>(&store)?
    ///     .clone();
    /// run.call(&mut store, ())?;
    /// run.call(&mut store, ())?;
    /// run.call(&mut store, ())?;
    ///
    /// let wat = r#"
    ///     (module
    ///         (import "commander" "_start" (func $commander_start))
    ///         (import "commander" "read_counter" (func $commander_read_counter (result i32)))
    ///         (func (export "run") (result i32)
    ///             call $commander_start
    ///             call $commander_start
    ///             call $commander_start
    ///             call $commander_read_counter
    ///         )
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    /// linker.module(&mut store, "", &module)?;
    /// let run = linker.get(&mut store, "", "run").unwrap().into_func().unwrap();
    /// let count = run.typed::<(), i32>(&store)?.call(&mut store, ())?;
    /// assert_eq!(count, 0, "a Command should get a fresh instance on each invocation");
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn module(
        &mut self,
        mut store: impl AsContextMut<Data = T>,
        module_name: &str,
        module: &Module,
    ) -> Result<&mut Self>
    where
        T: 'static,
    {
        // NB: this is intended to function the same as `Linker::module_async`,
        // they should be kept in sync.

        // This assert isn't strictly necessary since it'll bottom out in the
        // `HostFunc::to_func` method anyway. This is placed earlier for this
        // function though to prevent the functions created here from delaying
        // the panic until they're called.
        assert!(
            Engine::same(&self.engine, store.as_context().engine()),
            "different engines for this linker and the store provided"
        );
        match ModuleKind::categorize(module)? {
            ModuleKind::Command => {
                self.command(
                    store,
                    module_name,
                    module,
                    |store, func_ty, export_name, instance_pre| {
                        Func::new(
                            store,
                            func_ty.clone(),
                            move |mut caller, params, results| {
                                // Create a new instance for this command execution.
                                let instance = instance_pre.instantiate(&mut caller)?;

                                // `unwrap()` everything here because we know the instance contains a
                                // function export with the given name and signature because we're
                                // iterating over the module it was instantiated from.
                                instance
                                    .get_export(&mut caller, &export_name)
                                    .unwrap()
                                    .into_func()
                                    .unwrap()
                                    .call(&mut caller, params, results)?;

                                Ok(())
                            },
                        )
                    },
                )
            }
            ModuleKind::Reactor => {
                let instance = self.instantiate(&mut store, &module)?;

                if let Some(export) = instance.get_export(&mut store, "_initialize") {
                    if let Extern::Func(func) = export {
                        func.typed::<(), ()>(&store)
                            .and_then(|f| f.call(&mut store, ()))
                            .context("calling the Reactor initialization function")?;
                    }
                }

                self.instance(store, module_name, instance)
            }
        }
    }

    /// Define automatic instantiations of a [`Module`] in this linker.
    ///
    /// This is the same as [`Linker::module`], except for async `Store`s.
    #[cfg(all(feature = "async", feature = "cranelift"))]
    pub async fn module_async(
        &mut self,
        mut store: impl AsContextMut<Data = T>,
        module_name: &str,
        module: &Module,
    ) -> Result<&mut Self>
    where
        T: Send + 'static,
    {
        // NB: this is intended to function the same as `Linker::module`, they
        // should be kept in sync.
        assert!(
            Engine::same(&self.engine, store.as_context().engine()),
            "different engines for this linker and the store provided"
        );
        match ModuleKind::categorize(module)? {
            ModuleKind::Command => self.command(
                store,
                module_name,
                module,
                |store, func_ty, export_name, instance_pre| {
                    let upvars = Arc::new((instance_pre, export_name));
                    Func::new_async(
                        store,
                        func_ty.clone(),
                        move |mut caller, params, results| {
                            let upvars = upvars.clone();
                            Box::new(async move {
                                let (instance_pre, export_name) = &*upvars;
                                let instance = instance_pre.instantiate_async(&mut caller).await?;

                                instance
                                    .get_export(&mut caller, &export_name)
                                    .unwrap()
                                    .into_func()
                                    .unwrap()
                                    .call_async(&mut caller, params, results)
                                    .await?;
                                Ok(())
                            })
                        },
                    )
                },
            ),
            ModuleKind::Reactor => {
                let instance = self.instantiate_async(&mut store, &module).await?;

                if let Some(export) = instance.get_export(&mut store, "_initialize") {
                    if let Extern::Func(func) = export {
                        let func = func
                            .typed::<(), ()>(&store)
                            .context("loading the Reactor initialization function")?;
                        func.call_async(&mut store, ())
                            .await
                            .context("calling the Reactor initialization function")?;
                    }
                }

                self.instance(store, module_name, instance)
            }
        }
    }

    fn command(
        &mut self,
        mut store: impl AsContextMut<Data = T>,
        module_name: &str,
        module: &Module,
        mk_func: impl Fn(&mut StoreContextMut<T>, &FuncType, String, InstancePre<T>) -> Func,
    ) -> Result<&mut Self>
    where
        T: 'static,
    {
        let mut store = store.as_context_mut();
        for export in module.exports() {
            if let Some(func_ty) = export.ty().func() {
                let instance_pre = self.instantiate_pre(module)?;
                let export_name = export.name().to_owned();
                let func = mk_func(&mut store, func_ty, export_name, instance_pre);
                let key = self.import_key(module_name, Some(export.name()));
                self.insert(key, Definition::new(store.0, func.into()))?;
            } else if export.name() == "memory" && export.ty().memory().is_some() {
                // Allow an exported "memory" memory for now.
            } else if export.name() == "__indirect_function_table" && export.ty().table().is_some()
            {
                // Allow an exported "__indirect_function_table" table for now.
            } else if export.name() == "table" && export.ty().table().is_some() {
                // Allow an exported "table" table for now.
            } else if export.name() == "__data_end" && export.ty().global().is_some() {
                // Allow an exported "__data_end" memory for compatibility with toolchains
                // which use --export-dynamic, which unfortunately doesn't work the way
                // we want it to.
                warn!("command module exporting '__data_end' is deprecated");
            } else if export.name() == "__heap_base" && export.ty().global().is_some() {
                // Allow an exported "__data_end" memory for compatibility with toolchains
                // which use --export-dynamic, which unfortunately doesn't work the way
                // we want it to.
                warn!("command module exporting '__heap_base' is deprecated");
            } else if export.name() == "__dso_handle" && export.ty().global().is_some() {
                // Allow an exported "__dso_handle" memory for compatibility with toolchains
                // which use --export-dynamic, which unfortunately doesn't work the way
                // we want it to.
                warn!("command module exporting '__dso_handle' is deprecated")
            } else if export.name() == "__rtti_base" && export.ty().global().is_some() {
                // Allow an exported "__rtti_base" memory for compatibility with
                // AssemblyScript.
                warn!(
                    "command module exporting '__rtti_base' is deprecated; pass `--runtime half` to the AssemblyScript compiler"
                );
            } else if !self.allow_unknown_exports {
                bail!("command export '{}' is not a function", export.name());
            }
        }

        Ok(self)
    }

    /// Aliases one item's name as another.
    ///
    /// This method will alias an item with the specified `module` and `name`
    /// under a new name of `as_module` and `as_name`.
    ///
    /// # Errors
    ///
    /// Returns an error if any shadowing violations happen while defining new
    /// items, or if the original item wasn't defined.
    pub fn alias(
        &mut self,
        module: &str,
        name: &str,
        as_module: &str,
        as_name: &str,
    ) -> Result<&mut Self> {
        let src = self.import_key(module, Some(name));
        let dst = self.import_key(as_module, Some(as_name));
        match self.map.get(&src).cloned() {
            Some(item) => self.insert(dst, item)?,
            None => bail!("no item named `{}::{}` defined", module, name),
        }
        Ok(self)
    }

    /// Aliases one module's name as another.
    ///
    /// This method will alias all currently defined under `module` to also be
    /// defined under the name `as_module` too.
    ///
    /// # Errors
    ///
    /// Returns an error if any shadowing violations happen while defining new
    /// items.
    pub fn alias_module(&mut self, module: &str, as_module: &str) -> Result<()> {
        let module = self.intern_str(module);
        let as_module = self.intern_str(as_module);
        let items = self
            .map
            .iter()
            .filter(|(key, _def)| key.module == module)
            .map(|(key, def)| (key.name, def.clone()))
            .collect::<Vec<_>>();
        for (name, item) in items {
            self.insert(
                ImportKey {
                    module: as_module,
                    name,
                },
                item,
            )?;
        }
        Ok(())
    }

    fn insert(&mut self, key: ImportKey, item: Definition) -> Result<()> {
        match self.map.entry(key) {
            Entry::Occupied(_) if !self.allow_shadowing => {
                let module = &self.strings[key.module];
                let desc = match self.strings.get(key.name) {
                    Some(name) => format!("{module}::{name}"),
                    None => module.to_string(),
                };
                bail!("import of `{}` defined twice", desc)
            }
            Entry::Occupied(mut o) => {
                o.insert(item);
            }
            Entry::Vacant(v) => {
                v.insert(item);
            }
        }
        Ok(())
    }

    fn import_key(&mut self, module: &str, name: Option<&str>) -> ImportKey {
        ImportKey {
            module: self.intern_str(module),
            name: name
                .map(|name| self.intern_str(name))
                .unwrap_or(usize::max_value()),
        }
    }

    fn intern_str(&mut self, string: &str) -> usize {
        if let Some(idx) = self.string2idx.get(string) {
            return *idx;
        }
        let string: Arc<str> = string.into();
        let idx = self.strings.len();
        self.strings.push(string.clone());
        self.string2idx.insert(string, idx);
        idx
    }

    /// Attempts to instantiate the `module` provided.
    ///
    /// This method will attempt to assemble a list of imports that correspond
    /// to the imports required by the [`Module`] provided. This list
    /// of imports is then passed to [`Instance::new`] to continue the
    /// instantiation process.
    ///
    /// Each import of `module` will be looked up in this [`Linker`] and must
    /// have previously been defined. If it was previously defined with an
    /// incorrect signature or if it was not previously defined then an error
    /// will be returned because the import can not be satisfied.
    ///
    /// Per the WebAssembly spec, instantiation includes running the module's
    /// start function, if it has one (not to be confused with the `_start`
    /// function, which is not run).
    ///
    /// # Errors
    ///
    /// This method can fail because an import may not be found, or because
    /// instantiation itself may fail. For information on instantiation
    /// failures see [`Instance::new`]. If an import is not found, the error
    /// may be downcast to an [`UnknownImportError`].
    ///
    ///
    /// # Panics
    ///
    /// Panics if any item used to instantiate `module` is not owned by
    /// `store`. Additionally this will panic if the [`Engine`] that the `store`
    /// belongs to is different than this [`Linker`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let mut store = Store::new(&engine, ());
    /// let mut linker = Linker::new(&engine);
    /// linker.func_wrap("host", "double", |x: i32| x * 2)?;
    ///
    /// let wat = r#"
    ///     (module
    ///         (import "host" "double" (func (param i32) (result i32)))
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    /// linker.instantiate(&mut store, &module)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn instantiate(
        &self,
        mut store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance>
    where
        T: 'static,
    {
        self._instantiate_pre(module, Some(store.as_context_mut().0))?
            .instantiate(store)
    }

    /// Attempts to instantiate the `module` provided. This is the same as
    /// [`Linker::instantiate`], except for async `Store`s.
    #[cfg(feature = "async")]
    pub async fn instantiate_async(
        &self,
        mut store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance>
    where
        T: Send + 'static,
    {
        self._instantiate_pre(module, Some(store.as_context_mut().0))?
            .instantiate_async(store)
            .await
    }

    /// Performs all checks necessary for instantiating `module` with this
    /// linker, except that instantiation doesn't actually finish.
    ///
    /// This method is used for front-loading type-checking information as well
    /// as collecting the imports to use to instantiate a module with. The
    /// returned [`InstancePre`] represents a ready-to-be-instantiated module,
    /// which can also be instantiated multiple times if desired.
    ///
    /// # Errors
    ///
    /// Returns an error which may be downcast to an [`UnknownImportError`] if
    /// the module has any unresolvable imports.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let mut store = Store::new(&engine, ());
    /// let mut linker = Linker::new(&engine);
    /// linker.func_wrap("host", "double", |x: i32| x * 2)?;
    ///
    /// let wat = r#"
    ///     (module
    ///         (import "host" "double" (func (param i32) (result i32)))
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    /// let instance_pre = linker.instantiate_pre(&module)?;
    ///
    /// // Finish instantiation after the type-checking has all completed...
    /// let instance = instance_pre.instantiate(&mut store)?;
    ///
    /// // ... and we can even continue to keep instantiating if desired!
    /// instance_pre.instantiate(&mut store)?;
    /// instance_pre.instantiate(&mut store)?;
    ///
    /// // Note that functions defined in a linker with `func_wrap` and similar
    /// // constructors are not owned by any particular `Store`, so we can also
    /// // instantiate our `instance_pre` in other stores because no imports
    /// // belong to the original store.
    /// let mut new_store = Store::new(&engine, ());
    /// instance_pre.instantiate(&mut new_store)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn instantiate_pre(&self, module: &Module) -> Result<InstancePre<T>>
    where
        T: 'static,
    {
        self._instantiate_pre(module, None)
    }

    /// This is split out to optionally take a `store` so that when the
    /// `.instantiate` API is used we can get fresh up-to-date type information
    /// for memories and their current size, if necessary.
    ///
    /// Note that providing a `store` here is not required for correctness
    /// per-se. If one is not provided, such as the with the `instantiate_pre`
    /// API, then the type information used for memories and tables will reflect
    /// their size when inserted into the linker rather than their current size.
    /// This isn't expected to be much of a problem though since
    /// per-store-`Linker` types are likely using `.instantiate(..)` and
    /// per-`Engine` linkers don't have memories/tables in them.
    fn _instantiate_pre(
        &self,
        module: &Module,
        store: Option<&StoreOpaque>,
    ) -> Result<InstancePre<T>>
    where
        T: 'static,
    {
        let mut imports = module
            .imports()
            .map(|import| self._get_by_import(&import))
            .collect::<Result<Vec<_>, _>>()?;
        if let Some(store) = store {
            for import in imports.iter_mut() {
                import.update_size(store);
            }
        }
        unsafe { InstancePre::new(module, imports) }
    }

    /// Returns an iterator over all items defined in this `Linker`, in
    /// arbitrary order.
    ///
    /// The iterator returned will yield 3-tuples where the first two elements
    /// are the module name and item name for the external item, and the third
    /// item is the item itself that is defined.
    ///
    /// Note that multiple `Extern` items may be defined for the same
    /// module/name pair.
    ///
    /// # Panics
    ///
    /// This function will panic if the `store` provided does not come from the
    /// same [`Engine`] that this linker was created with.
    pub fn iter<'a: 'p, 'p>(
        &'a self,
        mut store: impl AsContextMut<Data = T> + 'p,
    ) -> impl Iterator<Item = (&'a str, &'a str, Extern)> + 'p
    where
        T: 'static,
    {
        self.map.iter().map(move |(key, item)| {
            let store = store.as_context_mut();
            (
                &*self.strings[key.module],
                &*self.strings[key.name],
                // Should be safe since `T` is connecting the linker and store
                unsafe { item.to_extern(store.0) },
            )
        })
    }

    /// Looks up a previously defined value in this [`Linker`], identified by
    /// the names provided.
    ///
    /// Returns `None` if this name was not previously defined in this
    /// [`Linker`].
    ///
    /// # Panics
    ///
    /// This function will panic if the `store` provided does not come from the
    /// same [`Engine`] that this linker was created with.
    pub fn get(
        &self,
        mut store: impl AsContextMut<Data = T>,
        module: &str,
        name: &str,
    ) -> Option<Extern>
    where
        T: 'static,
    {
        let store = store.as_context_mut().0;
        // Should be safe since `T` is connecting the linker and store
        Some(unsafe { self._get(module, name)?.to_extern(store) })
    }

    fn _get(&self, module: &str, name: &str) -> Option<&Definition> {
        let key = ImportKey {
            module: *self.string2idx.get(module)?,
            name: *self.string2idx.get(name)?,
        };
        self.map.get(&key)
    }

    /// Looks up a value in this `Linker` which matches the `import` type
    /// provided.
    ///
    /// Returns `None` if no match was found.
    ///
    /// # Panics
    ///
    /// This function will panic if the `store` provided does not come from the
    /// same [`Engine`] that this linker was created with.
    pub fn get_by_import(
        &self,
        mut store: impl AsContextMut<Data = T>,
        import: &ImportType,
    ) -> Option<Extern>
    where
        T: 'static,
    {
        let store = store.as_context_mut().0;
        // Should be safe since `T` is connecting the linker and store
        Some(unsafe { self._get_by_import(import).ok()?.to_extern(store) })
    }

    fn _get_by_import(&self, import: &ImportType) -> Result<Definition, UnknownImportError> {
        match self._get(import.module(), import.name()) {
            Some(item) => Ok(item.clone()),
            None => Err(UnknownImportError::new(import)),
        }
    }

    /// Returns the "default export" of a module.
    ///
    /// An export with an empty string is considered to be a "default export".
    /// "_start" is also recognized for compatibility.
    ///
    /// # Panics
    ///
    /// Panics if the default function found is not owned by `store`. This
    /// function will also panic if the `store` provided does not come from the
    /// same [`Engine`] that this linker was created with.
    pub fn get_default(&self, mut store: impl AsContextMut<Data = T>, module: &str) -> Result<Func>
    where
        T: 'static,
    {
        if let Some(external) = self.get(&mut store, module, "") {
            if let Extern::Func(func) = external {
                return Ok(func);
            }
            bail!("default export in '{}' is not a function", module);
        }

        // For compatibility, also recognize "_start".
        if let Some(external) = self.get(&mut store, module, "_start") {
            if let Extern::Func(func) = external {
                return Ok(func);
            }
            bail!("`_start` in '{}' is not a function", module);
        }

        // Otherwise return a no-op function.
        Ok(Func::wrap(store, || {}))
    }
}

impl<T: 'static> Default for Linker<T> {
    fn default() -> Linker<T> {
        Linker::new(&Engine::default())
    }
}

impl Definition {
    fn new(store: &StoreOpaque, item: Extern) -> Definition {
        let ty = DefinitionType::from(store, &item);
        Definition::Extern(item, ty)
    }

    pub(crate) fn ty(&self) -> DefinitionType {
        match self {
            Definition::Extern(_, ty) => ty.clone(),
            Definition::HostFunc(func) => DefinitionType::Func(func.sig_index()),
        }
    }

    /// Inserts this definition into the `store` provided.
    ///
    /// # Safety
    ///
    /// Note the unsafety here is due to calling `HostFunc::to_func`. The
    /// requirement here is that the `T` that was originally used to create the
    /// `HostFunc` matches the `T` on the store.
    pub(crate) unsafe fn to_extern(&self, store: &mut StoreOpaque) -> Extern {
        match self {
            Definition::Extern(e, _) => e.clone(),
            // SAFETY: the contract of this function is the same as what's
            // required of `to_func`, that `T` of the store matches the `T` of
            // this original definition.
            Definition::HostFunc(func) => unsafe { func.to_func(store).into() },
        }
    }

    pub(crate) fn comes_from_same_store(&self, store: &StoreOpaque) -> bool {
        match self {
            Definition::Extern(e, _) => e.comes_from_same_store(store),
            Definition::HostFunc(_func) => true,
        }
    }

    fn update_size(&mut self, store: &StoreOpaque) {
        match self {
            Definition::Extern(Extern::Memory(m), DefinitionType::Memory(_, size)) => {
                *size = m.internal_size(store);
            }
            Definition::Extern(Extern::SharedMemory(m), DefinitionType::Memory(_, size)) => {
                *size = m.size();
            }
            Definition::Extern(Extern::Table(m), DefinitionType::Table(_, size)) => {
                *size = m.internal_size(store);
            }
            _ => {}
        }
    }
}

impl DefinitionType {
    pub(crate) fn from(store: &StoreOpaque, item: &Extern) -> DefinitionType {
        match item {
            Extern::Func(f) => DefinitionType::Func(f.type_index(store)),
            Extern::Table(t) => {
                DefinitionType::Table(*t.wasmtime_ty(store), t.internal_size(store))
            }
            Extern::Global(t) => DefinitionType::Global(*t.wasmtime_ty(store)),
            Extern::Memory(t) => {
                DefinitionType::Memory(*t.wasmtime_ty(store), t.internal_size(store))
            }
            Extern::SharedMemory(t) => DefinitionType::Memory(*t.ty().wasmtime_memory(), t.size()),
            Extern::Tag(t) => DefinitionType::Tag(*t.wasmtime_ty(store)),
        }
    }

    pub(crate) fn desc(&self) -> &'static str {
        match self {
            DefinitionType::Func(_) => "function",
            DefinitionType::Table(..) => "table",
            DefinitionType::Memory(..) => "memory",
            DefinitionType::Global(_) => "global",
            DefinitionType::Tag(_) => "tag",
        }
    }
}

/// Modules can be interpreted either as Commands or Reactors.
enum ModuleKind {
    /// The instance is a Command, meaning an instance is created for each
    /// exported function and lives for the duration of the function call.
    Command,

    /// The instance is a Reactor, meaning one instance is created which
    /// may live across multiple calls.
    Reactor,
}

impl ModuleKind {
    /// Determine whether the given module is a Command or a Reactor.
    fn categorize(module: &Module) -> Result<ModuleKind> {
        let command_start = module.get_export("_start");
        let reactor_start = module.get_export("_initialize");
        match (command_start, reactor_start) {
            (Some(command_start), None) => {
                if let Some(_) = command_start.func() {
                    Ok(ModuleKind::Command)
                } else {
                    bail!("`_start` must be a function")
                }
            }
            (None, Some(reactor_start)) => {
                if let Some(_) = reactor_start.func() {
                    Ok(ModuleKind::Reactor)
                } else {
                    bail!("`_initialize` must be a function")
                }
            }
            (None, None) => {
                // Module declares neither of the recognized functions, so treat
                // it as a reactor with no initialization function.
                Ok(ModuleKind::Reactor)
            }
            (Some(_), Some(_)) => {
                // Module declares itself to be both a Command and a Reactor.
                bail!("Program cannot be both a Command and a Reactor")
            }
        }
    }
}

/// Error for an unresolvable import.
///
/// Returned - wrapped in an [`anyhow::Error`] - by [`Linker::instantiate`] and
/// related methods for modules with unresolvable imports.
#[derive(Clone, Debug)]
pub struct UnknownImportError {
    module: String,
    name: String,
    ty: ExternType,
}

impl UnknownImportError {
    fn new(import: &ImportType) -> Self {
        Self {
            module: import.module().to_string(),
            name: import.name().to_string(),
            ty: import.ty(),
        }
    }

    /// Returns the module name that the unknown import was expected to come from.
    pub fn module(&self) -> &str {
        &self.module
    }

    /// Returns the field name of the module that the unknown import was expected to come from.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the unknown import.
    pub fn ty(&self) -> ExternType {
        self.ty.clone()
    }
}

impl fmt::Display for UnknownImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unknown import: `{}::{}` has not been defined",
            self.module, self.name,
        )
    }
}

impl core::error::Error for UnknownImportError {}

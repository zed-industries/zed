/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `host`.
///
/// This structure is created through [`Host_Pre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`Host_`] as well.
pub struct Host_Pre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: Host_Indices,
}
impl<T: 'static> Clone for Host_Pre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> Host_Pre<_T> {
    /// Creates a new copy of `Host_Pre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = Host_Indices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`Host_`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Host_> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> Host_Pre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Host_> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `host`.
///
/// This is an implementation detail of [`Host_Pre`] and can
/// be constructed if needed as well.
///
/// For more information see [`Host_`] as well.
#[derive(Clone)]
pub struct Host_Indices {}
/// Auto-generated bindings for an instance a component which
/// implements the world `host`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`Host_::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`Host_Pre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`Host_Pre::instantiate`] to
///   create a [`Host_`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`Host_::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct Host_ {}
pub trait Host_ImportsWithStore: wasmtime::component::HasData + Send {
    fn foo<T: 'static>(
        accessor: &wasmtime::component::Accessor<T, Self>,
    ) -> impl ::core::future::Future<Output = ()> + Send;
}
pub trait Host_Imports: Send {}
impl<_T: Host_Imports + ?Sized + Send> Host_Imports for &mut _T {}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl Host_Indices {
        /// Creates a new copy of `Host_Indices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            Ok(Host_Indices {})
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`Host_`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Host_> {
            let _ = &mut store;
            let _instance = instance;
            Ok(Host_ {})
        }
    }
    impl Host_ {
        /// Convenience wrapper around [`Host_Pre::new`] and
        /// [`Host_Pre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Host_> {
            let pre = linker.instantiate_pre(component)?;
            Host_Pre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`Host_Indices::new`] and
        /// [`Host_Indices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Host_> {
            let indices = Host_Indices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`Host_Pre::new`] and
        /// [`Host_Pre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Host_>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            Host_Pre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker_imports<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: Host_ImportsWithStore,
            for<'a> D::Data<'a>: Host_Imports,
            T: 'static + Send,
        {
            let mut linker = linker.root();
            linker
                .func_wrap_concurrent(
                    "foo",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as Host_ImportsWithStore>::foo(accessor).await;
                            Ok(r)
                        })
                    },
                )?;
            Ok(())
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: Host_ImportsWithStore + Send,
            for<'a> D::Data<'a>: Host_Imports + Send,
            T: 'static + Send,
        {
            Self::add_to_linker_imports::<T, D>(linker, host_getter)?;
            Ok(())
        }
    }
};

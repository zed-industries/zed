/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `neptune`.
///
/// This structure is created through [`NeptunePre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`Neptune`] as well.
pub struct NeptunePre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: NeptuneIndices,
}
impl<T: 'static> Clone for NeptunePre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> NeptunePre<_T> {
    /// Creates a new copy of `NeptunePre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = NeptuneIndices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`Neptune`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Neptune> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> NeptunePre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Neptune> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `neptune`.
///
/// This is an implementation detail of [`NeptunePre`] and can
/// be constructed if needed as well.
///
/// For more information see [`Neptune`] as well.
#[derive(Clone)]
pub struct NeptuneIndices {}
/// Auto-generated bindings for an instance a component which
/// implements the world `neptune`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`Neptune::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`NeptunePre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`NeptunePre::instantiate`] to
///   create a [`Neptune`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`Neptune::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct Neptune {}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl NeptuneIndices {
        /// Creates a new copy of `NeptuneIndices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            Ok(NeptuneIndices {})
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`Neptune`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Neptune> {
            let _ = &mut store;
            let _instance = instance;
            Ok(Neptune {})
        }
    }
    impl Neptune {
        /// Convenience wrapper around [`NeptunePre::new`] and
        /// [`NeptunePre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Neptune> {
            let pre = linker.instantiate_pre(component)?;
            NeptunePre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`NeptuneIndices::new`] and
        /// [`NeptuneIndices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Neptune> {
            let indices = NeptuneIndices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`NeptunePre::new`] and
        /// [`NeptunePre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Neptune>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            NeptunePre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: foo::foo::green::HostWithStore + foo::foo::red::HostWithStore + Send,
            for<'a> D::Data<'a>: foo::foo::green::Host + foo::foo::red::Host + Send,
            T: 'static + Send,
        {
            foo::foo::green::add_to_linker::<T, D>(linker, host_getter)?;
            foo::foo::red::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod green {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub type Thing = i32;
            const _: () = {
                assert!(4 == < Thing as wasmtime::component::ComponentType >::SIZE32);
                assert!(4 == < Thing as wasmtime::component::ComponentType >::ALIGN32);
            };
            pub trait HostWithStore: wasmtime::component::HasData {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData,
            {}
            pub trait Host {}
            impl<_T: Host + ?Sized> Host for &mut _T {}
            pub fn add_to_linker<T, D>(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: fn(&mut T) -> D::Data<'_>,
            ) -> wasmtime::Result<()>
            where
                D: HostWithStore,
                for<'a> D::Data<'a>: Host,
                T: 'static,
            {
                let mut inst = linker.instance("foo:foo/green")?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod red {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub type Thing = super::super::super::foo::foo::green::Thing;
            const _: () = {
                assert!(4 == < Thing as wasmtime::component::ComponentType >::SIZE32);
                assert!(4 == < Thing as wasmtime::component::ComponentType >::ALIGN32);
            };
            pub trait HostWithStore: wasmtime::component::HasData + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + Send,
            {}
            pub trait Host: Send {
                fn foo(&mut self) -> impl ::core::future::Future<Output = Thing> + Send;
            }
            impl<_T: Host + ?Sized + Send> Host for &mut _T {
                fn foo(&mut self) -> impl ::core::future::Future<Output = Thing> + Send {
                    async move { Host::foo(*self).await }
                }
            }
            pub fn add_to_linker<T, D>(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: fn(&mut T) -> D::Data<'_>,
            ) -> wasmtime::Result<()>
            where
                D: HostWithStore,
                for<'a> D::Data<'a>: Host,
                T: 'static + Send,
            {
                let mut inst = linker.instance("foo:foo/red")?;
                inst.func_wrap_async(
                    "foo",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::foo(host).await;
                            Ok((r,))
                        })
                    },
                )?;
                Ok(())
            }
        }
    }
}

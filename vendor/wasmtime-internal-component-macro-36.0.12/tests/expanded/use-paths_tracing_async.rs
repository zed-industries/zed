/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `d`.
///
/// This structure is created through [`DPre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`D`] as well.
pub struct DPre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: DIndices,
}
impl<T: 'static> Clone for DPre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> DPre<_T> {
    /// Creates a new copy of `DPre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = DIndices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`D`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<D> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> DPre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<D> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `d`.
///
/// This is an implementation detail of [`DPre`] and can
/// be constructed if needed as well.
///
/// For more information see [`D`] as well.
#[derive(Clone)]
pub struct DIndices {}
/// Auto-generated bindings for an instance a component which
/// implements the world `d`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`D::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`DPre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`DPre::instantiate`] to
///   create a [`D`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`D::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct D {}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl DIndices {
        /// Creates a new copy of `DIndices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            Ok(DIndices {})
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`D`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<D> {
            let _ = &mut store;
            let _instance = instance;
            Ok(D {})
        }
    }
    impl D {
        /// Convenience wrapper around [`DPre::new`] and
        /// [`DPre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<D> {
            let pre = linker.instantiate_pre(component)?;
            DPre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`DIndices::new`] and
        /// [`DIndices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<D> {
            let indices = DIndices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`DPre::new`] and
        /// [`DPre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<D>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            DPre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: foo::foo::a::HostWithStore + foo::foo::b::HostWithStore
                + foo::foo::c::HostWithStore + d::HostWithStore + Send,
            for<'a> D::Data<
                'a,
            >: foo::foo::a::Host + foo::foo::b::Host + foo::foo::c::Host + d::Host
                + Send,
            T: 'static + Send,
        {
            foo::foo::a::add_to_linker::<T, D>(linker, host_getter)?;
            foo::foo::b::add_to_linker::<T, D>(linker, host_getter)?;
            foo::foo::c::add_to_linker::<T, D>(linker, host_getter)?;
            d::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod a {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct Foo {}
            impl core::fmt::Debug for Foo {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Foo").finish()
                }
            }
            const _: () = {
                assert!(0 == < Foo as wasmtime::component::ComponentType >::SIZE32);
                assert!(1 == < Foo as wasmtime::component::ComponentType >::ALIGN32);
            };
            pub trait HostWithStore: wasmtime::component::HasData + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + Send,
            {}
            pub trait Host: Send {
                fn a(&mut self) -> impl ::core::future::Future<Output = Foo> + Send;
            }
            impl<_T: Host + ?Sized + Send> Host for &mut _T {
                fn a(&mut self) -> impl ::core::future::Future<Output = Foo> + Send {
                    async move { Host::a(*self).await }
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
                let mut inst = linker.instance("foo:foo/a")?;
                inst.func_wrap_async(
                    "a",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module = "a",
                            function = "a",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::a(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod b {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub type Foo = super::super::super::foo::foo::a::Foo;
            const _: () = {
                assert!(0 == < Foo as wasmtime::component::ComponentType >::SIZE32);
                assert!(1 == < Foo as wasmtime::component::ComponentType >::ALIGN32);
            };
            pub trait HostWithStore: wasmtime::component::HasData + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + Send,
            {}
            pub trait Host: Send {
                fn a(&mut self) -> impl ::core::future::Future<Output = Foo> + Send;
            }
            impl<_T: Host + ?Sized + Send> Host for &mut _T {
                fn a(&mut self) -> impl ::core::future::Future<Output = Foo> + Send {
                    async move { Host::a(*self).await }
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
                let mut inst = linker.instance("foo:foo/b")?;
                inst.func_wrap_async(
                    "a",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module = "b",
                            function = "a",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::a(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod c {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub type Foo = super::super::super::foo::foo::b::Foo;
            const _: () = {
                assert!(0 == < Foo as wasmtime::component::ComponentType >::SIZE32);
                assert!(1 == < Foo as wasmtime::component::ComponentType >::ALIGN32);
            };
            pub trait HostWithStore: wasmtime::component::HasData + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + Send,
            {}
            pub trait Host: Send {
                fn a(&mut self) -> impl ::core::future::Future<Output = Foo> + Send;
            }
            impl<_T: Host + ?Sized + Send> Host for &mut _T {
                fn a(&mut self) -> impl ::core::future::Future<Output = Foo> + Send {
                    async move { Host::a(*self).await }
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
                let mut inst = linker.instance("foo:foo/c")?;
                inst.func_wrap_async(
                    "a",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module = "c",
                            function = "a",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::a(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                Ok(())
            }
        }
    }
}
#[allow(clippy::all)]
pub mod d {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::{anyhow, Box};
    pub type Foo = super::foo::foo::c::Foo;
    const _: () = {
        assert!(0 == < Foo as wasmtime::component::ComponentType >::SIZE32);
        assert!(1 == < Foo as wasmtime::component::ComponentType >::ALIGN32);
    };
    pub trait HostWithStore: wasmtime::component::HasData + Send {}
    impl<_T: ?Sized> HostWithStore for _T
    where
        _T: wasmtime::component::HasData + Send,
    {}
    pub trait Host: Send {
        fn b(&mut self) -> impl ::core::future::Future<Output = Foo> + Send;
    }
    impl<_T: Host + ?Sized + Send> Host for &mut _T {
        fn b(&mut self) -> impl ::core::future::Future<Output = Foo> + Send {
            async move { Host::b(*self).await }
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
        let mut inst = linker.instance("d")?;
        inst.func_wrap_async(
            "b",
            move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                use tracing::Instrument;
                let span = tracing::span!(
                    tracing::Level::TRACE, "wit-bindgen import", module = "<no module>",
                    function = "b",
                );
                wasmtime::component::__internal::Box::new(
                    async move {
                        tracing::event!(tracing::Level::TRACE, "call");
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::b(host).await;
                        tracing::event!(
                            tracing::Level::TRACE, result = tracing::field::debug(& r),
                            "return"
                        );
                        Ok((r,))
                    }
                        .instrument(span),
                )
            },
        )?;
        Ok(())
    }
}

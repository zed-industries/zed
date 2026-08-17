/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `foo`.
///
/// This structure is created through [`FooPre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`Foo`] as well.
pub struct FooPre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: FooIndices,
}
impl<T: 'static> Clone for FooPre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> FooPre<_T> {
    /// Creates a new copy of `FooPre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = FooIndices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`Foo`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Foo> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> FooPre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Foo> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `foo`.
///
/// This is an implementation detail of [`FooPre`] and can
/// be constructed if needed as well.
///
/// For more information see [`Foo`] as well.
#[derive(Clone)]
pub struct FooIndices {
    interface0: exports::my::dep0_1_0::a::GuestIndices,
    interface1: exports::my::dep0_2_0::a::GuestIndices,
}
/// Auto-generated bindings for an instance a component which
/// implements the world `foo`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`Foo::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`FooPre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`FooPre::instantiate`] to
///   create a [`Foo`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`Foo::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct Foo {
    interface0: exports::my::dep0_1_0::a::Guest,
    interface1: exports::my::dep0_2_0::a::Guest,
}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl FooIndices {
        /// Creates a new copy of `FooIndices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            let interface0 = exports::my::dep0_1_0::a::GuestIndices::new(_instance_pre)?;
            let interface1 = exports::my::dep0_2_0::a::GuestIndices::new(_instance_pre)?;
            Ok(FooIndices {
                interface0,
                interface1,
            })
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`Foo`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Foo> {
            let _ = &mut store;
            let _instance = instance;
            let interface0 = self.interface0.load(&mut store, &_instance)?;
            let interface1 = self.interface1.load(&mut store, &_instance)?;
            Ok(Foo { interface0, interface1 })
        }
    }
    impl Foo {
        /// Convenience wrapper around [`FooPre::new`] and
        /// [`FooPre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Foo> {
            let pre = linker.instantiate_pre(component)?;
            FooPre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`FooIndices::new`] and
        /// [`FooIndices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Foo> {
            let indices = FooIndices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`FooPre::new`] and
        /// [`FooPre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Foo>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            FooPre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: my::dep0_1_0::a::HostWithStore + my::dep0_2_0::a::HostWithStore + Send,
            for<'a> D::Data<'a>: my::dep0_1_0::a::Host + my::dep0_2_0::a::Host + Send,
            T: 'static + Send,
        {
            my::dep0_1_0::a::add_to_linker::<T, D>(linker, host_getter)?;
            my::dep0_2_0::a::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
        pub fn my_dep0_1_0_a(&self) -> &exports::my::dep0_1_0::a::Guest {
            &self.interface0
        }
        pub fn my_dep0_2_0_a(&self) -> &exports::my::dep0_2_0::a::Guest {
            &self.interface1
        }
    }
};
pub mod my {
    pub mod dep0_1_0 {
        #[allow(clippy::all)]
        pub mod a {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub trait HostWithStore: wasmtime::component::HasData + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + Send,
            {}
            pub trait Host: Send {
                fn x(&mut self) -> impl ::core::future::Future<Output = ()> + Send;
            }
            impl<_T: Host + ?Sized + Send> Host for &mut _T {
                fn x(&mut self) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::x(*self).await }
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
                let mut inst = linker.instance("my:dep/a@0.1.0")?;
                inst.func_wrap_async(
                    "x",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module = "a",
                            function = "x",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::x(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok(r)
                            }
                                .instrument(span),
                        )
                    },
                )?;
                Ok(())
            }
        }
    }
    pub mod dep0_2_0 {
        #[allow(clippy::all)]
        pub mod a {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub trait HostWithStore: wasmtime::component::HasData + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + Send,
            {}
            pub trait Host: Send {
                fn x(&mut self) -> impl ::core::future::Future<Output = ()> + Send;
            }
            impl<_T: Host + ?Sized + Send> Host for &mut _T {
                fn x(&mut self) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::x(*self).await }
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
                let mut inst = linker.instance("my:dep/a@0.2.0")?;
                inst.func_wrap_async(
                    "x",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module = "a",
                            function = "x",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::x(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok(r)
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
pub mod exports {
    pub mod my {
        pub mod dep0_1_0 {
            #[allow(clippy::all)]
            pub mod a {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                pub struct Guest {
                    x: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    x: wasmtime::component::ComponentExportIndex,
                }
                impl GuestIndices {
                    /// Constructor for [`GuestIndices`] which takes a
                    /// [`Component`](wasmtime::component::Component) as input and can be executed
                    /// before instantiation.
                    ///
                    /// This constructor can be used to front-load string lookups to find exports
                    /// within a component.
                    pub fn new<_T>(
                        _instance_pre: &wasmtime::component::InstancePre<_T>,
                    ) -> wasmtime::Result<GuestIndices> {
                        let instance = _instance_pre
                            .component()
                            .get_export_index(None, "my:dep/a@0.1.0")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `my:dep/a@0.1.0`"
                                )
                            })?;
                        let mut lookup = move |name| {
                            _instance_pre
                                .component()
                                .get_export_index(Some(&instance), name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `my:dep/a@0.1.0` does \
                    not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let x = lookup("x")?;
                        Ok(GuestIndices { x })
                    }
                    pub fn load(
                        &self,
                        mut store: impl wasmtime::AsContextMut,
                        instance: &wasmtime::component::Instance,
                    ) -> wasmtime::Result<Guest> {
                        let _instance = instance;
                        let _instance_pre = _instance.instance_pre(&store);
                        let _instance_type = _instance_pre.instance_type();
                        let mut store = store.as_context_mut();
                        let _ = &mut store;
                        let x = *_instance
                            .get_typed_func::<(), ()>(&mut store, &self.x)?
                            .func();
                        Ok(Guest { x })
                    }
                }
                impl Guest {
                    pub async fn call_x<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "my:dep/a@0.1.0", function = "x",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.x)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(())
                    }
                }
            }
        }
        pub mod dep0_2_0 {
            #[allow(clippy::all)]
            pub mod a {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                pub struct Guest {
                    x: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    x: wasmtime::component::ComponentExportIndex,
                }
                impl GuestIndices {
                    /// Constructor for [`GuestIndices`] which takes a
                    /// [`Component`](wasmtime::component::Component) as input and can be executed
                    /// before instantiation.
                    ///
                    /// This constructor can be used to front-load string lookups to find exports
                    /// within a component.
                    pub fn new<_T>(
                        _instance_pre: &wasmtime::component::InstancePre<_T>,
                    ) -> wasmtime::Result<GuestIndices> {
                        let instance = _instance_pre
                            .component()
                            .get_export_index(None, "my:dep/a@0.2.0")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `my:dep/a@0.2.0`"
                                )
                            })?;
                        let mut lookup = move |name| {
                            _instance_pre
                                .component()
                                .get_export_index(Some(&instance), name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `my:dep/a@0.2.0` does \
                    not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let x = lookup("x")?;
                        Ok(GuestIndices { x })
                    }
                    pub fn load(
                        &self,
                        mut store: impl wasmtime::AsContextMut,
                        instance: &wasmtime::component::Instance,
                    ) -> wasmtime::Result<Guest> {
                        let _instance = instance;
                        let _instance_pre = _instance.instance_pre(&store);
                        let _instance_type = _instance_pre.instance_type();
                        let mut store = store.as_context_mut();
                        let _ = &mut store;
                        let x = *_instance
                            .get_typed_func::<(), ()>(&mut store, &self.x)?
                            .func();
                        Ok(Guest { x })
                    }
                }
                impl Guest {
                    pub async fn call_x<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "my:dep/a@0.2.0", function = "x",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.x)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(())
                    }
                }
            }
        }
    }
}

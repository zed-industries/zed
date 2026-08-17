/// Link-time configurations.
#[derive(Clone, Debug, Default)]
pub struct LinkOptions {
    experimental_interface: bool,
    experimental_interface_function: bool,
    experimental_interface_resource: bool,
    experimental_interface_resource_method: bool,
    experimental_world: bool,
    experimental_world_function_import: bool,
    experimental_world_interface_import: bool,
    experimental_world_resource: bool,
    experimental_world_resource_method: bool,
}
impl LinkOptions {
    /// Enable members marked as `@unstable(feature = experimental-interface)`
    pub fn experimental_interface(&mut self, enabled: bool) -> &mut Self {
        self.experimental_interface = enabled;
        self
    }
    /// Enable members marked as `@unstable(feature = experimental-interface-function)`
    pub fn experimental_interface_function(&mut self, enabled: bool) -> &mut Self {
        self.experimental_interface_function = enabled;
        self
    }
    /// Enable members marked as `@unstable(feature = experimental-interface-resource)`
    pub fn experimental_interface_resource(&mut self, enabled: bool) -> &mut Self {
        self.experimental_interface_resource = enabled;
        self
    }
    /// Enable members marked as `@unstable(feature = experimental-interface-resource-method)`
    pub fn experimental_interface_resource_method(
        &mut self,
        enabled: bool,
    ) -> &mut Self {
        self.experimental_interface_resource_method = enabled;
        self
    }
    /// Enable members marked as `@unstable(feature = experimental-world)`
    pub fn experimental_world(&mut self, enabled: bool) -> &mut Self {
        self.experimental_world = enabled;
        self
    }
    /// Enable members marked as `@unstable(feature = experimental-world-function-import)`
    pub fn experimental_world_function_import(&mut self, enabled: bool) -> &mut Self {
        self.experimental_world_function_import = enabled;
        self
    }
    /// Enable members marked as `@unstable(feature = experimental-world-interface-import)`
    pub fn experimental_world_interface_import(&mut self, enabled: bool) -> &mut Self {
        self.experimental_world_interface_import = enabled;
        self
    }
    /// Enable members marked as `@unstable(feature = experimental-world-resource)`
    pub fn experimental_world_resource(&mut self, enabled: bool) -> &mut Self {
        self.experimental_world_resource = enabled;
        self
    }
    /// Enable members marked as `@unstable(feature = experimental-world-resource-method)`
    pub fn experimental_world_resource_method(&mut self, enabled: bool) -> &mut Self {
        self.experimental_world_resource_method = enabled;
        self
    }
}
impl core::convert::From<LinkOptions> for foo::foo::the_interface::LinkOptions {
    fn from(src: LinkOptions) -> Self {
        (&src).into()
    }
}
impl core::convert::From<&LinkOptions> for foo::foo::the_interface::LinkOptions {
    fn from(src: &LinkOptions) -> Self {
        let mut dest = Self::default();
        dest.experimental_interface(src.experimental_interface);
        dest.experimental_interface_function(src.experimental_interface_function);
        dest.experimental_interface_resource(src.experimental_interface_resource);
        dest.experimental_interface_resource_method(
            src.experimental_interface_resource_method,
        );
        dest
    }
}
pub enum Baz {}
pub trait HostBazWithStore: wasmtime::component::HasData + Send {}
impl<_T: ?Sized> HostBazWithStore for _T
where
    _T: wasmtime::component::HasData + Send,
{}
pub trait HostBaz: Send {
    fn foo(
        &mut self,
        self_: wasmtime::component::Resource<Baz>,
    ) -> impl ::core::future::Future<Output = ()> + Send;
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<Baz>,
    ) -> impl ::core::future::Future<Output = wasmtime::Result<()>> + Send;
}
impl<_T: HostBaz + ?Sized + Send> HostBaz for &mut _T {
    fn foo(
        &mut self,
        self_: wasmtime::component::Resource<Baz>,
    ) -> impl ::core::future::Future<Output = ()> + Send {
        async move { HostBaz::foo(*self, self_).await }
    }
    async fn drop(
        &mut self,
        rep: wasmtime::component::Resource<Baz>,
    ) -> wasmtime::Result<()> {
        HostBaz::drop(*self, rep).await
    }
}
/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `the-world`.
///
/// This structure is created through [`TheWorldPre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`TheWorld`] as well.
pub struct TheWorldPre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: TheWorldIndices,
}
impl<T: 'static> Clone for TheWorldPre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> TheWorldPre<_T> {
    /// Creates a new copy of `TheWorldPre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = TheWorldIndices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`TheWorld`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<TheWorld> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> TheWorldPre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<TheWorld> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `the-world`.
///
/// This is an implementation detail of [`TheWorldPre`] and can
/// be constructed if needed as well.
///
/// For more information see [`TheWorld`] as well.
#[derive(Clone)]
pub struct TheWorldIndices {}
/// Auto-generated bindings for an instance a component which
/// implements the world `the-world`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`TheWorld::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`TheWorldPre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`TheWorldPre::instantiate`] to
///   create a [`TheWorld`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`TheWorld::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct TheWorld {}
pub trait TheWorldImportsWithStore: wasmtime::component::HasData + HostBazWithStore + Send {}
impl<_T: ?Sized> TheWorldImportsWithStore for _T
where
    _T: wasmtime::component::HasData + HostBazWithStore + Send,
{}
pub trait TheWorldImports: HostBaz + Send {
    fn foo(&mut self) -> impl ::core::future::Future<Output = ()> + Send;
}
impl<_T: TheWorldImports + ?Sized + Send> TheWorldImports for &mut _T {
    fn foo(&mut self) -> impl ::core::future::Future<Output = ()> + Send {
        async move { TheWorldImports::foo(*self).await }
    }
}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl TheWorldIndices {
        /// Creates a new copy of `TheWorldIndices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            Ok(TheWorldIndices {})
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`TheWorld`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<TheWorld> {
            let _ = &mut store;
            let _instance = instance;
            Ok(TheWorld {})
        }
    }
    impl TheWorld {
        /// Convenience wrapper around [`TheWorldPre::new`] and
        /// [`TheWorldPre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<TheWorld> {
            let pre = linker.instantiate_pre(component)?;
            TheWorldPre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`TheWorldIndices::new`] and
        /// [`TheWorldIndices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<TheWorld> {
            let indices = TheWorldIndices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`TheWorldPre::new`] and
        /// [`TheWorldPre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<TheWorld>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            TheWorldPre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker_imports<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            options: &LinkOptions,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: TheWorldImportsWithStore,
            for<'a> D::Data<'a>: TheWorldImports,
            T: 'static + Send,
        {
            let mut linker = linker.root();
            if options.experimental_world {
                if options.experimental_world_resource {
                    linker
                        .resource_async(
                            "baz",
                            wasmtime::component::ResourceType::host::<Baz>(),
                            move |mut store, rep| {
                                wasmtime::component::__internal::Box::new(async move {
                                    HostBaz::drop(
                                            &mut host_getter(store.data_mut()),
                                            wasmtime::component::Resource::new_own(rep),
                                        )
                                        .await
                                })
                            },
                        )?;
                }
                if options.experimental_world_function_import {
                    linker
                        .func_wrap_async(
                            "foo",
                            move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                                use tracing::Instrument;
                                let span = tracing::span!(
                                    tracing::Level::TRACE, "wit-bindgen import", module =
                                    "the-world", function = "foo",
                                );
                                wasmtime::component::__internal::Box::new(
                                    async move {
                                        tracing::event!(tracing::Level::TRACE, "call");
                                        let host = &mut host_getter(caller.data_mut());
                                        let r = TheWorldImports::foo(host).await;
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
                }
                if options.experimental_world_resource_method {
                    linker
                        .func_wrap_async(
                            "[method]baz.foo",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (arg0,): (wasmtime::component::Resource<Baz>,)|
                            {
                                use tracing::Instrument;
                                let span = tracing::span!(
                                    tracing::Level::TRACE, "wit-bindgen import", module =
                                    "the-world", function = "[method]baz.foo",
                                );
                                wasmtime::component::__internal::Box::new(
                                    async move {
                                        tracing::event!(
                                            tracing::Level::TRACE, self_ = tracing::field::debug(&
                                            arg0), "call"
                                        );
                                        let host = &mut host_getter(caller.data_mut());
                                        let r = HostBaz::foo(host, arg0).await;
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
                }
            }
            Ok(())
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            options: &LinkOptions,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: foo::foo::the_interface::HostWithStore + TheWorldImportsWithStore + Send,
            for<'a> D::Data<'a>: foo::foo::the_interface::Host + TheWorldImports + Send,
            T: 'static + Send,
        {
            if options.experimental_world {
                Self::add_to_linker_imports::<T, D>(linker, options, host_getter)?;
                if options.experimental_world_interface_import {
                    foo::foo::the_interface::add_to_linker::<
                        T,
                        D,
                    >(linker, &options.into(), host_getter)?;
                }
            }
            Ok(())
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod the_interface {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            /// Link-time configurations.
            #[derive(Clone, Debug, Default)]
            pub struct LinkOptions {
                experimental_interface: bool,
                experimental_interface_function: bool,
                experimental_interface_resource: bool,
                experimental_interface_resource_method: bool,
            }
            impl LinkOptions {
                /// Enable members marked as `@unstable(feature = experimental-interface)`
                pub fn experimental_interface(&mut self, enabled: bool) -> &mut Self {
                    self.experimental_interface = enabled;
                    self
                }
                /// Enable members marked as `@unstable(feature = experimental-interface-function)`
                pub fn experimental_interface_function(
                    &mut self,
                    enabled: bool,
                ) -> &mut Self {
                    self.experimental_interface_function = enabled;
                    self
                }
                /// Enable members marked as `@unstable(feature = experimental-interface-resource)`
                pub fn experimental_interface_resource(
                    &mut self,
                    enabled: bool,
                ) -> &mut Self {
                    self.experimental_interface_resource = enabled;
                    self
                }
                /// Enable members marked as `@unstable(feature = experimental-interface-resource-method)`
                pub fn experimental_interface_resource_method(
                    &mut self,
                    enabled: bool,
                ) -> &mut Self {
                    self.experimental_interface_resource_method = enabled;
                    self
                }
            }
            pub enum Bar {}
            pub trait HostBarWithStore: wasmtime::component::HasData + Send {}
            impl<_T: ?Sized> HostBarWithStore for _T
            where
                _T: wasmtime::component::HasData + Send,
            {}
            pub trait HostBar: Send {
                fn foo(
                    &mut self,
                    self_: wasmtime::component::Resource<Bar>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Bar>,
                ) -> impl ::core::future::Future<Output = wasmtime::Result<()>> + Send;
            }
            impl<_T: HostBar + ?Sized + Send> HostBar for &mut _T {
                fn foo(
                    &mut self,
                    self_: wasmtime::component::Resource<Bar>,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { HostBar::foo(*self, self_).await }
                }
                async fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Bar>,
                ) -> wasmtime::Result<()> {
                    HostBar::drop(*self, rep).await
                }
            }
            pub trait HostWithStore: wasmtime::component::HasData + HostBarWithStore + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + HostBarWithStore + Send,
            {}
            pub trait Host: HostBar + Send {
                fn foo(&mut self) -> impl ::core::future::Future<Output = ()> + Send;
            }
            impl<_T: Host + ?Sized + Send> Host for &mut _T {
                fn foo(&mut self) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::foo(*self).await }
                }
            }
            pub fn add_to_linker<T, D>(
                linker: &mut wasmtime::component::Linker<T>,
                options: &LinkOptions,
                host_getter: fn(&mut T) -> D::Data<'_>,
            ) -> wasmtime::Result<()>
            where
                D: HostWithStore,
                for<'a> D::Data<'a>: Host,
                T: 'static + Send,
            {
                if options.experimental_interface {
                    let mut inst = linker.instance("foo:foo/the-interface")?;
                    if options.experimental_interface_resource {
                        inst.resource_async(
                            "bar",
                            wasmtime::component::ResourceType::host::<Bar>(),
                            move |mut store, rep| {
                                wasmtime::component::__internal::Box::new(async move {
                                    HostBar::drop(
                                            &mut host_getter(store.data_mut()),
                                            wasmtime::component::Resource::new_own(rep),
                                        )
                                        .await
                                })
                            },
                        )?;
                    }
                    if options.experimental_interface_function {
                        inst.func_wrap_async(
                            "foo",
                            move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                                use tracing::Instrument;
                                let span = tracing::span!(
                                    tracing::Level::TRACE, "wit-bindgen import", module =
                                    "the-interface", function = "foo",
                                );
                                wasmtime::component::__internal::Box::new(
                                    async move {
                                        tracing::event!(tracing::Level::TRACE, "call");
                                        let host = &mut host_getter(caller.data_mut());
                                        let r = Host::foo(host).await;
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
                    }
                    if options.experimental_interface_resource_method {
                        inst.func_wrap_async(
                            "[method]bar.foo",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (arg0,): (wasmtime::component::Resource<Bar>,)|
                            {
                                use tracing::Instrument;
                                let span = tracing::span!(
                                    tracing::Level::TRACE, "wit-bindgen import", module =
                                    "the-interface", function = "[method]bar.foo",
                                );
                                wasmtime::component::__internal::Box::new(
                                    async move {
                                        tracing::event!(
                                            tracing::Level::TRACE, self_ = tracing::field::debug(&
                                            arg0), "call"
                                        );
                                        let host = &mut host_getter(caller.data_mut());
                                        let r = HostBar::foo(host, arg0).await;
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
                    }
                }
                Ok(())
            }
        }
    }
}

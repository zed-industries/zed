/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `http-interface`.
///
/// This structure is created through [`HttpInterfacePre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`HttpInterface`] as well.
pub struct HttpInterfacePre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: HttpInterfaceIndices,
}
impl<T: 'static> Clone for HttpInterfacePre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> HttpInterfacePre<_T> {
    /// Creates a new copy of `HttpInterfacePre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = HttpInterfaceIndices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`HttpInterface`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<HttpInterface> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> HttpInterfacePre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<HttpInterface> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `http-interface`.
///
/// This is an implementation detail of [`HttpInterfacePre`] and can
/// be constructed if needed as well.
///
/// For more information see [`HttpInterface`] as well.
#[derive(Clone)]
pub struct HttpInterfaceIndices {
    interface0: exports::http_handler::GuestIndices,
}
/// Auto-generated bindings for an instance a component which
/// implements the world `http-interface`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`HttpInterface::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`HttpInterfacePre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`HttpInterfacePre::instantiate`] to
///   create a [`HttpInterface`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`HttpInterface::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct HttpInterface {
    interface0: exports::http_handler::Guest,
}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl HttpInterfaceIndices {
        /// Creates a new copy of `HttpInterfaceIndices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            let interface0 = exports::http_handler::GuestIndices::new(_instance_pre)?;
            Ok(HttpInterfaceIndices { interface0 })
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`HttpInterface`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<HttpInterface> {
            let _ = &mut store;
            let _instance = instance;
            let interface0 = self.interface0.load(&mut store, &_instance)?;
            Ok(HttpInterface { interface0 })
        }
    }
    impl HttpInterface {
        /// Convenience wrapper around [`HttpInterfacePre::new`] and
        /// [`HttpInterfacePre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<HttpInterface> {
            let pre = linker.instantiate_pre(component)?;
            HttpInterfacePre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`HttpInterfaceIndices::new`] and
        /// [`HttpInterfaceIndices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<HttpInterface> {
            let indices = HttpInterfaceIndices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`HttpInterfacePre::new`] and
        /// [`HttpInterfacePre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<HttpInterface>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            HttpInterfacePre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: foo::foo::http_types::HostWithStore + http_fetch::HostWithStore + Send,
            for<'a> D::Data<'a>: foo::foo::http_types::Host + http_fetch::Host + Send,
            T: 'static + Send,
        {
            foo::foo::http_types::add_to_linker::<T, D>(linker, host_getter)?;
            http_fetch::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
        pub fn http_handler(&self) -> &exports::http_handler::Guest {
            &self.interface0
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod http_types {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone)]
            pub struct Request {
                #[component(name = "method")]
                pub method: wasmtime::component::__internal::String,
            }
            impl core::fmt::Debug for Request {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Request").field("method", &self.method).finish()
                }
            }
            const _: () = {
                assert!(8 == < Request as wasmtime::component::ComponentType >::SIZE32);
                assert!(4 == < Request as wasmtime::component::ComponentType >::ALIGN32);
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone)]
            pub struct Response {
                #[component(name = "body")]
                pub body: wasmtime::component::__internal::String,
            }
            impl core::fmt::Debug for Response {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Response").field("body", &self.body).finish()
                }
            }
            const _: () = {
                assert!(8 == < Response as wasmtime::component::ComponentType >::SIZE32);
                assert!(
                    4 == < Response as wasmtime::component::ComponentType >::ALIGN32
                );
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
                let mut inst = linker.instance("foo:foo/http-types")?;
                Ok(())
            }
        }
    }
}
#[allow(clippy::all)]
pub mod http_fetch {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::{anyhow, Box};
    pub type Request = super::foo::foo::http_types::Request;
    const _: () = {
        assert!(8 == < Request as wasmtime::component::ComponentType >::SIZE32);
        assert!(4 == < Request as wasmtime::component::ComponentType >::ALIGN32);
    };
    pub type Response = super::foo::foo::http_types::Response;
    const _: () = {
        assert!(8 == < Response as wasmtime::component::ComponentType >::SIZE32);
        assert!(4 == < Response as wasmtime::component::ComponentType >::ALIGN32);
    };
    pub trait HostWithStore: wasmtime::component::HasData + Send {
        fn fetch_request<T: 'static>(
            accessor: &wasmtime::component::Accessor<T, Self>,
            request: Request,
        ) -> impl ::core::future::Future<Output = Response> + Send;
    }
    pub trait Host: Send {}
    impl<_T: Host + ?Sized + Send> Host for &mut _T {}
    pub fn add_to_linker<T, D>(
        linker: &mut wasmtime::component::Linker<T>,
        host_getter: fn(&mut T) -> D::Data<'_>,
    ) -> wasmtime::Result<()>
    where
        D: HostWithStore,
        for<'a> D::Data<'a>: Host,
        T: 'static + Send,
    {
        let mut inst = linker.instance("http-fetch")?;
        inst.func_wrap_concurrent(
            "fetch-request",
            move |caller: &wasmtime::component::Accessor<T>, (arg0,): (Request,)| {
                wasmtime::component::__internal::Box::pin(async move {
                    let accessor = &caller.with_data(host_getter);
                    let r = <D as HostWithStore>::fetch_request(accessor, arg0).await;
                    Ok((r,))
                })
            },
        )?;
        Ok(())
    }
}
pub mod exports {
    #[allow(clippy::all)]
    pub mod http_handler {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::{anyhow, Box};
        pub type Request = super::super::foo::foo::http_types::Request;
        const _: () = {
            assert!(8 == < Request as wasmtime::component::ComponentType >::SIZE32);
            assert!(4 == < Request as wasmtime::component::ComponentType >::ALIGN32);
        };
        pub type Response = super::super::foo::foo::http_types::Response;
        const _: () = {
            assert!(8 == < Response as wasmtime::component::ComponentType >::SIZE32);
            assert!(4 == < Response as wasmtime::component::ComponentType >::ALIGN32);
        };
        pub struct Guest {
            handle_request: wasmtime::component::Func,
        }
        #[derive(Clone)]
        pub struct GuestIndices {
            handle_request: wasmtime::component::ComponentExportIndex,
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
                    .get_export_index(None, "http-handler")
                    .ok_or_else(|| {
                        anyhow::anyhow!("no exported instance named `http-handler`")
                    })?;
                let mut lookup = move |name| {
                    _instance_pre
                        .component()
                        .get_export_index(Some(&instance), name)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "instance export `http-handler` does \
            not have export `{name}`"
                            )
                        })
                };
                let _ = &mut lookup;
                let handle_request = lookup("handle-request")?;
                Ok(GuestIndices { handle_request })
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
                let handle_request = *_instance
                    .get_typed_func::<
                        (&Request,),
                        (Response,),
                    >(&mut store, &self.handle_request)?
                    .func();
                Ok(Guest { handle_request })
            }
        }
        impl Guest {
            pub async fn call_handle_request<_T, _D>(
                &self,
                accessor: &wasmtime::component::Accessor<_T, _D>,
                arg0: Request,
            ) -> wasmtime::Result<Response>
            where
                _T: Send,
                _D: wasmtime::component::HasData,
            {
                let callee = unsafe {
                    wasmtime::component::TypedFunc::<
                        (Request,),
                        (Response,),
                    >::new_unchecked(self.handle_request)
                };
                let (ret0,) = callee.call_concurrent(accessor, (arg0,)).await?;
                Ok(ret0)
            }
        }
    }
}

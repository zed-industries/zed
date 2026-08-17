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
pub struct TheWorldIndices {
    interface0: exports::foo::foo::floats::GuestIndices,
}
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
pub struct TheWorld {
    interface0: exports::foo::foo::floats::Guest,
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
            let interface0 = exports::foo::foo::floats::GuestIndices::new(
                _instance_pre,
            )?;
            Ok(TheWorldIndices { interface0 })
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
            let interface0 = self.interface0.load(&mut store, &_instance)?;
            Ok(TheWorld { interface0 })
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
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: foo::foo::floats::HostWithStore + Send,
            for<'a> D::Data<'a>: foo::foo::floats::Host + Send,
            T: 'static + Send,
        {
            foo::foo::floats::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
        pub fn foo_foo_floats(&self) -> &exports::foo::foo::floats::Guest {
            &self.interface0
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod floats {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub trait HostWithStore: wasmtime::component::HasData + Send {
                fn f32_param<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: f32,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn f64_param<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: f64,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn f32_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = f32> + Send;
                fn f64_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = f64> + Send;
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
                let mut inst = linker.instance("foo:foo/floats")?;
                inst.func_wrap_concurrent(
                    "f32-param",
                    move |caller: &wasmtime::component::Accessor<T>, (arg0,): (f32,)| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::f32_param(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "f64-param",
                    move |caller: &wasmtime::component::Accessor<T>, (arg0,): (f64,)| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::f64_param(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "f32-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::f32_result(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "f64-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::f64_result(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                Ok(())
            }
        }
    }
}
pub mod exports {
    pub mod foo {
        pub mod foo {
            #[allow(clippy::all)]
            pub mod floats {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                pub struct Guest {
                    f32_param: wasmtime::component::Func,
                    f64_param: wasmtime::component::Func,
                    f32_result: wasmtime::component::Func,
                    f64_result: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    f32_param: wasmtime::component::ComponentExportIndex,
                    f64_param: wasmtime::component::ComponentExportIndex,
                    f32_result: wasmtime::component::ComponentExportIndex,
                    f64_result: wasmtime::component::ComponentExportIndex,
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
                            .get_export_index(None, "foo:foo/floats")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/floats`"
                                )
                            })?;
                        let mut lookup = move |name| {
                            _instance_pre
                                .component()
                                .get_export_index(Some(&instance), name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `foo:foo/floats` does \
                not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let f32_param = lookup("f32-param")?;
                        let f64_param = lookup("f64-param")?;
                        let f32_result = lookup("f32-result")?;
                        let f64_result = lookup("f64-result")?;
                        Ok(GuestIndices {
                            f32_param,
                            f64_param,
                            f32_result,
                            f64_result,
                        })
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
                        let f32_param = *_instance
                            .get_typed_func::<(f32,), ()>(&mut store, &self.f32_param)?
                            .func();
                        let f64_param = *_instance
                            .get_typed_func::<(f64,), ()>(&mut store, &self.f64_param)?
                            .func();
                        let f32_result = *_instance
                            .get_typed_func::<(), (f32,)>(&mut store, &self.f32_result)?
                            .func();
                        let f64_result = *_instance
                            .get_typed_func::<(), (f64,)>(&mut store, &self.f64_result)?
                            .func();
                        Ok(Guest {
                            f32_param,
                            f64_param,
                            f32_result,
                            f64_result,
                        })
                    }
                }
                impl Guest {
                    pub async fn call_f32_param<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: f32,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (f32,),
                                (),
                            >::new_unchecked(self.f32_param)
                        };
                        let () = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(())
                    }
                    pub async fn call_f64_param<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: f64,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (f64,),
                                (),
                            >::new_unchecked(self.f64_param)
                        };
                        let () = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(())
                    }
                    pub async fn call_f32_result<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<f32>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (f32,),
                            >::new_unchecked(self.f32_result)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, ()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_f64_result<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<f64>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (f64,),
                            >::new_unchecked(self.f64_result)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, ()).await?;
                        Ok(ret0)
                    }
                }
            }
        }
    }
}

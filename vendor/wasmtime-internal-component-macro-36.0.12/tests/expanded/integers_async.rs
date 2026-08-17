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
    interface0: exports::foo::foo::integers::GuestIndices,
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
    interface0: exports::foo::foo::integers::Guest,
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
            let interface0 = exports::foo::foo::integers::GuestIndices::new(
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
            D: foo::foo::integers::HostWithStore + Send,
            for<'a> D::Data<'a>: foo::foo::integers::Host + Send,
            T: 'static + Send,
        {
            foo::foo::integers::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
        pub fn foo_foo_integers(&self) -> &exports::foo::foo::integers::Guest {
            &self.interface0
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod integers {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub trait HostWithStore: wasmtime::component::HasData + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + Send,
            {}
            pub trait Host: Send {
                fn a1(
                    &mut self,
                    x: u8,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn a2(
                    &mut self,
                    x: i8,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn a3(
                    &mut self,
                    x: u16,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn a4(
                    &mut self,
                    x: i16,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn a5(
                    &mut self,
                    x: u32,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn a6(
                    &mut self,
                    x: i32,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn a7(
                    &mut self,
                    x: u64,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn a8(
                    &mut self,
                    x: i64,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn a9(
                    &mut self,
                    p1: u8,
                    p2: i8,
                    p3: u16,
                    p4: i16,
                    p5: u32,
                    p6: i32,
                    p7: u64,
                    p8: i64,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn r1(&mut self) -> impl ::core::future::Future<Output = u8> + Send;
                fn r2(&mut self) -> impl ::core::future::Future<Output = i8> + Send;
                fn r3(&mut self) -> impl ::core::future::Future<Output = u16> + Send;
                fn r4(&mut self) -> impl ::core::future::Future<Output = i16> + Send;
                fn r5(&mut self) -> impl ::core::future::Future<Output = u32> + Send;
                fn r6(&mut self) -> impl ::core::future::Future<Output = i32> + Send;
                fn r7(&mut self) -> impl ::core::future::Future<Output = u64> + Send;
                fn r8(&mut self) -> impl ::core::future::Future<Output = i64> + Send;
                fn pair_ret(
                    &mut self,
                ) -> impl ::core::future::Future<Output = (i64, u8)> + Send;
            }
            impl<_T: Host + ?Sized + Send> Host for &mut _T {
                fn a1(
                    &mut self,
                    x: u8,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::a1(*self, x).await }
                }
                fn a2(
                    &mut self,
                    x: i8,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::a2(*self, x).await }
                }
                fn a3(
                    &mut self,
                    x: u16,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::a3(*self, x).await }
                }
                fn a4(
                    &mut self,
                    x: i16,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::a4(*self, x).await }
                }
                fn a5(
                    &mut self,
                    x: u32,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::a5(*self, x).await }
                }
                fn a6(
                    &mut self,
                    x: i32,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::a6(*self, x).await }
                }
                fn a7(
                    &mut self,
                    x: u64,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::a7(*self, x).await }
                }
                fn a8(
                    &mut self,
                    x: i64,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::a8(*self, x).await }
                }
                fn a9(
                    &mut self,
                    p1: u8,
                    p2: i8,
                    p3: u16,
                    p4: i16,
                    p5: u32,
                    p6: i32,
                    p7: u64,
                    p8: i64,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::a9(*self, p1, p2, p3, p4, p5, p6, p7, p8).await }
                }
                fn r1(&mut self) -> impl ::core::future::Future<Output = u8> + Send {
                    async move { Host::r1(*self).await }
                }
                fn r2(&mut self) -> impl ::core::future::Future<Output = i8> + Send {
                    async move { Host::r2(*self).await }
                }
                fn r3(&mut self) -> impl ::core::future::Future<Output = u16> + Send {
                    async move { Host::r3(*self).await }
                }
                fn r4(&mut self) -> impl ::core::future::Future<Output = i16> + Send {
                    async move { Host::r4(*self).await }
                }
                fn r5(&mut self) -> impl ::core::future::Future<Output = u32> + Send {
                    async move { Host::r5(*self).await }
                }
                fn r6(&mut self) -> impl ::core::future::Future<Output = i32> + Send {
                    async move { Host::r6(*self).await }
                }
                fn r7(&mut self) -> impl ::core::future::Future<Output = u64> + Send {
                    async move { Host::r7(*self).await }
                }
                fn r8(&mut self) -> impl ::core::future::Future<Output = i64> + Send {
                    async move { Host::r8(*self).await }
                }
                fn pair_ret(
                    &mut self,
                ) -> impl ::core::future::Future<Output = (i64, u8)> + Send {
                    async move { Host::pair_ret(*self).await }
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
                let mut inst = linker.instance("foo:foo/integers")?;
                inst.func_wrap_async(
                    "a1",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (u8,)| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::a1(host, arg0).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "a2",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (i8,)| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::a2(host, arg0).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "a3",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (u16,)| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::a3(host, arg0).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "a4",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (i16,)| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::a4(host, arg0).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "a5",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (u32,)| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::a5(host, arg0).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "a6",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (i32,)| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::a6(host, arg0).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "a7",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (u64,)| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::a7(host, arg0).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "a8",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (i64,)| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::a8(host, arg0).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "a9",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                            arg6,
                            arg7,
                        ): (u8, i8, u16, i16, u32, i32, u64, i64)|
                    {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::a9(
                                    host,
                                    arg0,
                                    arg1,
                                    arg2,
                                    arg3,
                                    arg4,
                                    arg5,
                                    arg6,
                                    arg7,
                                )
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "r1",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::r1(host).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "r2",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::r2(host).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "r3",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::r3(host).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "r4",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::r4(host).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "r5",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::r5(host).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "r6",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::r6(host).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "r7",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::r7(host).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "r8",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::r8(host).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_async(
                    "pair-ret",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::pair_ret(host).await;
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
            pub mod integers {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                pub struct Guest {
                    a1: wasmtime::component::Func,
                    a2: wasmtime::component::Func,
                    a3: wasmtime::component::Func,
                    a4: wasmtime::component::Func,
                    a5: wasmtime::component::Func,
                    a6: wasmtime::component::Func,
                    a7: wasmtime::component::Func,
                    a8: wasmtime::component::Func,
                    a9: wasmtime::component::Func,
                    r1: wasmtime::component::Func,
                    r2: wasmtime::component::Func,
                    r3: wasmtime::component::Func,
                    r4: wasmtime::component::Func,
                    r5: wasmtime::component::Func,
                    r6: wasmtime::component::Func,
                    r7: wasmtime::component::Func,
                    r8: wasmtime::component::Func,
                    pair_ret: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    a1: wasmtime::component::ComponentExportIndex,
                    a2: wasmtime::component::ComponentExportIndex,
                    a3: wasmtime::component::ComponentExportIndex,
                    a4: wasmtime::component::ComponentExportIndex,
                    a5: wasmtime::component::ComponentExportIndex,
                    a6: wasmtime::component::ComponentExportIndex,
                    a7: wasmtime::component::ComponentExportIndex,
                    a8: wasmtime::component::ComponentExportIndex,
                    a9: wasmtime::component::ComponentExportIndex,
                    r1: wasmtime::component::ComponentExportIndex,
                    r2: wasmtime::component::ComponentExportIndex,
                    r3: wasmtime::component::ComponentExportIndex,
                    r4: wasmtime::component::ComponentExportIndex,
                    r5: wasmtime::component::ComponentExportIndex,
                    r6: wasmtime::component::ComponentExportIndex,
                    r7: wasmtime::component::ComponentExportIndex,
                    r8: wasmtime::component::ComponentExportIndex,
                    pair_ret: wasmtime::component::ComponentExportIndex,
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
                            .get_export_index(None, "foo:foo/integers")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/integers`"
                                )
                            })?;
                        let mut lookup = move |name| {
                            _instance_pre
                                .component()
                                .get_export_index(Some(&instance), name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `foo:foo/integers` does \
                                                    not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let a1 = lookup("a1")?;
                        let a2 = lookup("a2")?;
                        let a3 = lookup("a3")?;
                        let a4 = lookup("a4")?;
                        let a5 = lookup("a5")?;
                        let a6 = lookup("a6")?;
                        let a7 = lookup("a7")?;
                        let a8 = lookup("a8")?;
                        let a9 = lookup("a9")?;
                        let r1 = lookup("r1")?;
                        let r2 = lookup("r2")?;
                        let r3 = lookup("r3")?;
                        let r4 = lookup("r4")?;
                        let r5 = lookup("r5")?;
                        let r6 = lookup("r6")?;
                        let r7 = lookup("r7")?;
                        let r8 = lookup("r8")?;
                        let pair_ret = lookup("pair-ret")?;
                        Ok(GuestIndices {
                            a1,
                            a2,
                            a3,
                            a4,
                            a5,
                            a6,
                            a7,
                            a8,
                            a9,
                            r1,
                            r2,
                            r3,
                            r4,
                            r5,
                            r6,
                            r7,
                            r8,
                            pair_ret,
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
                        let a1 = *_instance
                            .get_typed_func::<(u8,), ()>(&mut store, &self.a1)?
                            .func();
                        let a2 = *_instance
                            .get_typed_func::<(i8,), ()>(&mut store, &self.a2)?
                            .func();
                        let a3 = *_instance
                            .get_typed_func::<(u16,), ()>(&mut store, &self.a3)?
                            .func();
                        let a4 = *_instance
                            .get_typed_func::<(i16,), ()>(&mut store, &self.a4)?
                            .func();
                        let a5 = *_instance
                            .get_typed_func::<(u32,), ()>(&mut store, &self.a5)?
                            .func();
                        let a6 = *_instance
                            .get_typed_func::<(i32,), ()>(&mut store, &self.a6)?
                            .func();
                        let a7 = *_instance
                            .get_typed_func::<(u64,), ()>(&mut store, &self.a7)?
                            .func();
                        let a8 = *_instance
                            .get_typed_func::<(i64,), ()>(&mut store, &self.a8)?
                            .func();
                        let a9 = *_instance
                            .get_typed_func::<
                                (u8, i8, u16, i16, u32, i32, u64, i64),
                                (),
                            >(&mut store, &self.a9)?
                            .func();
                        let r1 = *_instance
                            .get_typed_func::<(), (u8,)>(&mut store, &self.r1)?
                            .func();
                        let r2 = *_instance
                            .get_typed_func::<(), (i8,)>(&mut store, &self.r2)?
                            .func();
                        let r3 = *_instance
                            .get_typed_func::<(), (u16,)>(&mut store, &self.r3)?
                            .func();
                        let r4 = *_instance
                            .get_typed_func::<(), (i16,)>(&mut store, &self.r4)?
                            .func();
                        let r5 = *_instance
                            .get_typed_func::<(), (u32,)>(&mut store, &self.r5)?
                            .func();
                        let r6 = *_instance
                            .get_typed_func::<(), (i32,)>(&mut store, &self.r6)?
                            .func();
                        let r7 = *_instance
                            .get_typed_func::<(), (u64,)>(&mut store, &self.r7)?
                            .func();
                        let r8 = *_instance
                            .get_typed_func::<(), (i64,)>(&mut store, &self.r8)?
                            .func();
                        let pair_ret = *_instance
                            .get_typed_func::<
                                (),
                                ((i64, u8),),
                            >(&mut store, &self.pair_ret)?
                            .func();
                        Ok(Guest {
                            a1,
                            a2,
                            a3,
                            a4,
                            a5,
                            a6,
                            a7,
                            a8,
                            a9,
                            r1,
                            r2,
                            r3,
                            r4,
                            r5,
                            r6,
                            r7,
                            r8,
                            pair_ret,
                        })
                    }
                }
                impl Guest {
                    pub async fn call_a1<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: u8,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (u8,),
                                (),
                            >::new_unchecked(self.a1)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(())
                    }
                    pub async fn call_a2<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: i8,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (i8,),
                                (),
                            >::new_unchecked(self.a2)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(())
                    }
                    pub async fn call_a3<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: u16,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (u16,),
                                (),
                            >::new_unchecked(self.a3)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(())
                    }
                    pub async fn call_a4<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: i16,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (i16,),
                                (),
                            >::new_unchecked(self.a4)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(())
                    }
                    pub async fn call_a5<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: u32,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (u32,),
                                (),
                            >::new_unchecked(self.a5)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(())
                    }
                    pub async fn call_a6<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: i32,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (i32,),
                                (),
                            >::new_unchecked(self.a6)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(())
                    }
                    pub async fn call_a7<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: u64,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (u64,),
                                (),
                            >::new_unchecked(self.a7)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(())
                    }
                    pub async fn call_a8<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: i64,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (i64,),
                                (),
                            >::new_unchecked(self.a8)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(())
                    }
                    pub async fn call_a9<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: u8,
                        arg1: i8,
                        arg2: u16,
                        arg3: i16,
                        arg4: u32,
                        arg5: i32,
                        arg6: u64,
                        arg7: i64,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (u8, i8, u16, i16, u32, i32, u64, i64),
                                (),
                            >::new_unchecked(self.a9)
                        };
                        let () = callee
                            .call_async(
                                store.as_context_mut(),
                                (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7),
                            )
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(())
                    }
                    pub async fn call_r1<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<u8>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (u8,),
                            >::new_unchecked(self.r1)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_r2<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<i8>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (i8,),
                            >::new_unchecked(self.r2)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_r3<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<u16>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (u16,),
                            >::new_unchecked(self.r3)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_r4<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<i16>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (i16,),
                            >::new_unchecked(self.r4)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_r5<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<u32>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (u32,),
                            >::new_unchecked(self.r5)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_r6<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<i32>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (i32,),
                            >::new_unchecked(self.r6)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_r7<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<u64>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (u64,),
                            >::new_unchecked(self.r7)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_r8<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<i64>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (i64,),
                            >::new_unchecked(self.r8)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_pair_ret<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<(i64, u8)>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                ((i64, u8),),
                            >::new_unchecked(self.pair_ret)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .await?;
                        callee.post_return_async(store.as_context_mut()).await?;
                        Ok(ret0)
                    }
                }
            }
        }
    }
}

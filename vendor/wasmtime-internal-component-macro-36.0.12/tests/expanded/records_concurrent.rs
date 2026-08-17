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
    interface0: exports::foo::foo::records::GuestIndices,
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
    interface0: exports::foo::foo::records::Guest,
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
            let interface0 = exports::foo::foo::records::GuestIndices::new(
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
            D: foo::foo::records::HostWithStore + Send,
            for<'a> D::Data<'a>: foo::foo::records::Host + Send,
            T: 'static + Send,
        {
            foo::foo::records::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
        pub fn foo_foo_records(&self) -> &exports::foo::foo::records::Guest {
            &self.interface0
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod records {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct Empty {}
            impl core::fmt::Debug for Empty {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Empty").finish()
                }
            }
            const _: () = {
                assert!(0 == < Empty as wasmtime::component::ComponentType >::SIZE32);
                assert!(1 == < Empty as wasmtime::component::ComponentType >::ALIGN32);
            };
            /// A record containing two scalar fields
            /// that both have the same type
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct Scalars {
                /// The first field, named a
                #[component(name = "a")]
                pub a: u32,
                /// The second field, named b
                #[component(name = "b")]
                pub b: u32,
            }
            impl core::fmt::Debug for Scalars {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Scalars")
                        .field("a", &self.a)
                        .field("b", &self.b)
                        .finish()
                }
            }
            const _: () = {
                assert!(8 == < Scalars as wasmtime::component::ComponentType >::SIZE32);
                assert!(4 == < Scalars as wasmtime::component::ComponentType >::ALIGN32);
            };
            /// A record that is really just flags
            /// All of the fields are bool
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct ReallyFlags {
                #[component(name = "a")]
                pub a: bool,
                #[component(name = "b")]
                pub b: bool,
                #[component(name = "c")]
                pub c: bool,
                #[component(name = "d")]
                pub d: bool,
                #[component(name = "e")]
                pub e: bool,
                #[component(name = "f")]
                pub f: bool,
                #[component(name = "g")]
                pub g: bool,
                #[component(name = "h")]
                pub h: bool,
                #[component(name = "i")]
                pub i: bool,
            }
            impl core::fmt::Debug for ReallyFlags {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("ReallyFlags")
                        .field("a", &self.a)
                        .field("b", &self.b)
                        .field("c", &self.c)
                        .field("d", &self.d)
                        .field("e", &self.e)
                        .field("f", &self.f)
                        .field("g", &self.g)
                        .field("h", &self.h)
                        .field("i", &self.i)
                        .finish()
                }
            }
            const _: () = {
                assert!(
                    9 == < ReallyFlags as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    1 == < ReallyFlags as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone)]
            pub struct Aggregates {
                #[component(name = "a")]
                pub a: Scalars,
                #[component(name = "b")]
                pub b: u32,
                #[component(name = "c")]
                pub c: Empty,
                #[component(name = "d")]
                pub d: wasmtime::component::__internal::String,
                #[component(name = "e")]
                pub e: ReallyFlags,
            }
            impl core::fmt::Debug for Aggregates {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Aggregates")
                        .field("a", &self.a)
                        .field("b", &self.b)
                        .field("c", &self.c)
                        .field("d", &self.d)
                        .field("e", &self.e)
                        .finish()
                }
            }
            const _: () = {
                assert!(
                    32 == < Aggregates as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    4 == < Aggregates as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            pub type TupleTypedef = (i32,);
            const _: () = {
                assert!(
                    4 == < TupleTypedef as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    4 == < TupleTypedef as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            pub type IntTypedef = i32;
            const _: () = {
                assert!(
                    4 == < IntTypedef as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    4 == < IntTypedef as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            pub type TupleTypedef2 = (IntTypedef,);
            const _: () = {
                assert!(
                    4 == < TupleTypedef2 as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    4 == < TupleTypedef2 as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            pub trait HostWithStore: wasmtime::component::HasData + Send {
                fn tuple_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: (char, u32),
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn tuple_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = (char, u32)> + Send;
                fn empty_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Empty,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn empty_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = Empty> + Send;
                fn scalar_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Scalars,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn scalar_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = Scalars> + Send;
                fn flags_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: ReallyFlags,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn flags_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ReallyFlags> + Send;
                fn aggregate_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Aggregates,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn aggregate_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = Aggregates> + Send;
                fn typedef_inout<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    e: TupleTypedef2,
                ) -> impl ::core::future::Future<Output = i32> + Send;
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
                let mut inst = linker.instance("foo:foo/records")?;
                inst.func_wrap_concurrent(
                    "tuple-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): ((char, u32),)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::tuple_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "tuple-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::tuple_result(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "empty-arg",
                    move |caller: &wasmtime::component::Accessor<T>, (arg0,): (Empty,)| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::empty_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "empty-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::empty_result(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "scalar-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (Scalars,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::scalar_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "scalar-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::scalar_result(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "flags-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (ReallyFlags,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::flags_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "flags-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::flags_result(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "aggregate-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (Aggregates,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::aggregate_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "aggregate-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::aggregate_result(accessor)
                                .await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "typedef-inout",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (TupleTypedef2,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::typedef_inout(accessor, arg0)
                                .await;
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
            pub mod records {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(record)]
                #[derive(Clone, Copy)]
                pub struct Empty {}
                impl core::fmt::Debug for Empty {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("Empty").finish()
                    }
                }
                const _: () = {
                    assert!(
                        0 == < Empty as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        1 == < Empty as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                /// A record containing two scalar fields
                /// that both have the same type
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(record)]
                #[derive(Clone, Copy)]
                pub struct Scalars {
                    /// The first field, named a
                    #[component(name = "a")]
                    pub a: u32,
                    /// The second field, named b
                    #[component(name = "b")]
                    pub b: u32,
                }
                impl core::fmt::Debug for Scalars {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("Scalars")
                            .field("a", &self.a)
                            .field("b", &self.b)
                            .finish()
                    }
                }
                const _: () = {
                    assert!(
                        8 == < Scalars as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        4 == < Scalars as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                /// A record that is really just flags
                /// All of the fields are bool
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(record)]
                #[derive(Clone, Copy)]
                pub struct ReallyFlags {
                    #[component(name = "a")]
                    pub a: bool,
                    #[component(name = "b")]
                    pub b: bool,
                    #[component(name = "c")]
                    pub c: bool,
                    #[component(name = "d")]
                    pub d: bool,
                    #[component(name = "e")]
                    pub e: bool,
                    #[component(name = "f")]
                    pub f: bool,
                    #[component(name = "g")]
                    pub g: bool,
                    #[component(name = "h")]
                    pub h: bool,
                    #[component(name = "i")]
                    pub i: bool,
                }
                impl core::fmt::Debug for ReallyFlags {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("ReallyFlags")
                            .field("a", &self.a)
                            .field("b", &self.b)
                            .field("c", &self.c)
                            .field("d", &self.d)
                            .field("e", &self.e)
                            .field("f", &self.f)
                            .field("g", &self.g)
                            .field("h", &self.h)
                            .field("i", &self.i)
                            .finish()
                    }
                }
                const _: () = {
                    assert!(
                        9 == < ReallyFlags as wasmtime::component::ComponentType
                        >::SIZE32
                    );
                    assert!(
                        1 == < ReallyFlags as wasmtime::component::ComponentType
                        >::ALIGN32
                    );
                };
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(record)]
                #[derive(Clone)]
                pub struct Aggregates {
                    #[component(name = "a")]
                    pub a: Scalars,
                    #[component(name = "b")]
                    pub b: u32,
                    #[component(name = "c")]
                    pub c: Empty,
                    #[component(name = "d")]
                    pub d: wasmtime::component::__internal::String,
                    #[component(name = "e")]
                    pub e: ReallyFlags,
                }
                impl core::fmt::Debug for Aggregates {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("Aggregates")
                            .field("a", &self.a)
                            .field("b", &self.b)
                            .field("c", &self.c)
                            .field("d", &self.d)
                            .field("e", &self.e)
                            .finish()
                    }
                }
                const _: () = {
                    assert!(
                        32 == < Aggregates as wasmtime::component::ComponentType
                        >::SIZE32
                    );
                    assert!(
                        4 == < Aggregates as wasmtime::component::ComponentType
                        >::ALIGN32
                    );
                };
                pub type TupleTypedef = (i32,);
                const _: () = {
                    assert!(
                        4 == < TupleTypedef as wasmtime::component::ComponentType
                        >::SIZE32
                    );
                    assert!(
                        4 == < TupleTypedef as wasmtime::component::ComponentType
                        >::ALIGN32
                    );
                };
                pub type IntTypedef = i32;
                const _: () = {
                    assert!(
                        4 == < IntTypedef as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        4 == < IntTypedef as wasmtime::component::ComponentType
                        >::ALIGN32
                    );
                };
                pub type TupleTypedef2 = (IntTypedef,);
                const _: () = {
                    assert!(
                        4 == < TupleTypedef2 as wasmtime::component::ComponentType
                        >::SIZE32
                    );
                    assert!(
                        4 == < TupleTypedef2 as wasmtime::component::ComponentType
                        >::ALIGN32
                    );
                };
                pub struct Guest {
                    tuple_arg: wasmtime::component::Func,
                    tuple_result: wasmtime::component::Func,
                    empty_arg: wasmtime::component::Func,
                    empty_result: wasmtime::component::Func,
                    scalar_arg: wasmtime::component::Func,
                    scalar_result: wasmtime::component::Func,
                    flags_arg: wasmtime::component::Func,
                    flags_result: wasmtime::component::Func,
                    aggregate_arg: wasmtime::component::Func,
                    aggregate_result: wasmtime::component::Func,
                    typedef_inout: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    tuple_arg: wasmtime::component::ComponentExportIndex,
                    tuple_result: wasmtime::component::ComponentExportIndex,
                    empty_arg: wasmtime::component::ComponentExportIndex,
                    empty_result: wasmtime::component::ComponentExportIndex,
                    scalar_arg: wasmtime::component::ComponentExportIndex,
                    scalar_result: wasmtime::component::ComponentExportIndex,
                    flags_arg: wasmtime::component::ComponentExportIndex,
                    flags_result: wasmtime::component::ComponentExportIndex,
                    aggregate_arg: wasmtime::component::ComponentExportIndex,
                    aggregate_result: wasmtime::component::ComponentExportIndex,
                    typedef_inout: wasmtime::component::ComponentExportIndex,
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
                            .get_export_index(None, "foo:foo/records")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/records`"
                                )
                            })?;
                        let mut lookup = move |name| {
                            _instance_pre
                                .component()
                                .get_export_index(Some(&instance), name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `foo:foo/records` does \
                not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let tuple_arg = lookup("tuple-arg")?;
                        let tuple_result = lookup("tuple-result")?;
                        let empty_arg = lookup("empty-arg")?;
                        let empty_result = lookup("empty-result")?;
                        let scalar_arg = lookup("scalar-arg")?;
                        let scalar_result = lookup("scalar-result")?;
                        let flags_arg = lookup("flags-arg")?;
                        let flags_result = lookup("flags-result")?;
                        let aggregate_arg = lookup("aggregate-arg")?;
                        let aggregate_result = lookup("aggregate-result")?;
                        let typedef_inout = lookup("typedef-inout")?;
                        Ok(GuestIndices {
                            tuple_arg,
                            tuple_result,
                            empty_arg,
                            empty_result,
                            scalar_arg,
                            scalar_result,
                            flags_arg,
                            flags_result,
                            aggregate_arg,
                            aggregate_result,
                            typedef_inout,
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
                        let tuple_arg = *_instance
                            .get_typed_func::<
                                ((char, u32),),
                                (),
                            >(&mut store, &self.tuple_arg)?
                            .func();
                        let tuple_result = *_instance
                            .get_typed_func::<
                                (),
                                ((char, u32),),
                            >(&mut store, &self.tuple_result)?
                            .func();
                        let empty_arg = *_instance
                            .get_typed_func::<(Empty,), ()>(&mut store, &self.empty_arg)?
                            .func();
                        let empty_result = *_instance
                            .get_typed_func::<
                                (),
                                (Empty,),
                            >(&mut store, &self.empty_result)?
                            .func();
                        let scalar_arg = *_instance
                            .get_typed_func::<
                                (Scalars,),
                                (),
                            >(&mut store, &self.scalar_arg)?
                            .func();
                        let scalar_result = *_instance
                            .get_typed_func::<
                                (),
                                (Scalars,),
                            >(&mut store, &self.scalar_result)?
                            .func();
                        let flags_arg = *_instance
                            .get_typed_func::<
                                (ReallyFlags,),
                                (),
                            >(&mut store, &self.flags_arg)?
                            .func();
                        let flags_result = *_instance
                            .get_typed_func::<
                                (),
                                (ReallyFlags,),
                            >(&mut store, &self.flags_result)?
                            .func();
                        let aggregate_arg = *_instance
                            .get_typed_func::<
                                (&Aggregates,),
                                (),
                            >(&mut store, &self.aggregate_arg)?
                            .func();
                        let aggregate_result = *_instance
                            .get_typed_func::<
                                (),
                                (Aggregates,),
                            >(&mut store, &self.aggregate_result)?
                            .func();
                        let typedef_inout = *_instance
                            .get_typed_func::<
                                (TupleTypedef2,),
                                (i32,),
                            >(&mut store, &self.typedef_inout)?
                            .func();
                        Ok(Guest {
                            tuple_arg,
                            tuple_result,
                            empty_arg,
                            empty_result,
                            scalar_arg,
                            scalar_result,
                            flags_arg,
                            flags_result,
                            aggregate_arg,
                            aggregate_result,
                            typedef_inout,
                        })
                    }
                }
                impl Guest {
                    pub async fn call_tuple_arg<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: (char, u32),
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                ((char, u32),),
                                (),
                            >::new_unchecked(self.tuple_arg)
                        };
                        let () = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(())
                    }
                    pub async fn call_tuple_result<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<(char, u32)>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                ((char, u32),),
                            >::new_unchecked(self.tuple_result)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, ()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_empty_arg<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: Empty,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (Empty,),
                                (),
                            >::new_unchecked(self.empty_arg)
                        };
                        let () = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(())
                    }
                    pub async fn call_empty_result<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<Empty>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (Empty,),
                            >::new_unchecked(self.empty_result)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, ()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_scalar_arg<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: Scalars,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (Scalars,),
                                (),
                            >::new_unchecked(self.scalar_arg)
                        };
                        let () = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(())
                    }
                    pub async fn call_scalar_result<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<Scalars>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (Scalars,),
                            >::new_unchecked(self.scalar_result)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, ()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_flags_arg<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: ReallyFlags,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (ReallyFlags,),
                                (),
                            >::new_unchecked(self.flags_arg)
                        };
                        let () = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(())
                    }
                    pub async fn call_flags_result<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<ReallyFlags>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (ReallyFlags,),
                            >::new_unchecked(self.flags_result)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, ()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_aggregate_arg<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: Aggregates,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (Aggregates,),
                                (),
                            >::new_unchecked(self.aggregate_arg)
                        };
                        let () = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(())
                    }
                    pub async fn call_aggregate_result<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<Aggregates>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (Aggregates,),
                            >::new_unchecked(self.aggregate_result)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, ()).await?;
                        Ok(ret0)
                    }
                    pub async fn call_typedef_inout<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: TupleTypedef2,
                    ) -> wasmtime::Result<i32>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (TupleTypedef2,),
                                (i32,),
                            >::new_unchecked(self.typedef_inout)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(ret0)
                    }
                }
            }
        }
    }
}

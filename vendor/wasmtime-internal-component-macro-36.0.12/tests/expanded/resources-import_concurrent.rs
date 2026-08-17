pub enum WorldResource {}
pub trait HostWorldResourceWithStore: wasmtime::component::HasData + Send {
    fn drop<T: 'static>(
        accessor: &wasmtime::component::Accessor<T, Self>,
        rep: wasmtime::component::Resource<WorldResource>,
    ) -> impl ::core::future::Future<Output = wasmtime::Result<()>> + Send
    where
        Self: Sized;
    fn new<T: 'static>(
        accessor: &wasmtime::component::Accessor<T, Self>,
    ) -> impl ::core::future::Future<
        Output = wasmtime::component::Resource<WorldResource>,
    > + Send;
    fn foo<T: 'static>(
        accessor: &wasmtime::component::Accessor<T, Self>,
        self_: wasmtime::component::Resource<WorldResource>,
    ) -> impl ::core::future::Future<Output = ()> + Send;
    fn static_foo<T: 'static>(
        accessor: &wasmtime::component::Accessor<T, Self>,
    ) -> impl ::core::future::Future<Output = ()> + Send;
}
pub trait HostWorldResource: Send {}
impl<_T: HostWorldResource + ?Sized + Send> HostWorldResource for &mut _T {}
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
    interface1: exports::foo::foo::uses_resource_transitively::GuestIndices,
    some_world_func2: wasmtime::component::ComponentExportIndex,
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
    interface1: exports::foo::foo::uses_resource_transitively::Guest,
    some_world_func2: wasmtime::component::Func,
}
pub trait TheWorldImportsWithStore: wasmtime::component::HasData + HostWorldResourceWithStore + Send {
    fn some_world_func<T: 'static>(
        accessor: &wasmtime::component::Accessor<T, Self>,
    ) -> impl ::core::future::Future<
        Output = wasmtime::component::Resource<WorldResource>,
    > + Send;
}
pub trait TheWorldImports: HostWorldResource + Send {}
impl<_T: TheWorldImports + ?Sized + Send> TheWorldImports for &mut _T {}
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
            let interface1 = exports::foo::foo::uses_resource_transitively::GuestIndices::new(
                _instance_pre,
            )?;
            let some_world_func2 = {
                let (item, index) = _component
                    .get_export(None, "some-world-func2")
                    .ok_or_else(|| {
                        anyhow::anyhow!("no export `some-world-func2` found")
                    })?;
                match item {
                    wasmtime::component::types::ComponentItem::ComponentFunc(func) => {
                        anyhow::Context::context(
                            func
                                .typecheck::<
                                    (),
                                    (wasmtime::component::Resource<WorldResource>,),
                                >(&_instance_type),
                            "type-checking export func `some-world-func2`",
                        )?;
                        index
                    }
                    _ => {
                        Err(
                            anyhow::anyhow!(
                                "export `some-world-func2` is not a function"
                            ),
                        )?
                    }
                }
            };
            Ok(TheWorldIndices {
                interface1,
                some_world_func2,
            })
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
            let interface1 = self.interface1.load(&mut store, &_instance)?;
            let some_world_func2 = *_instance
                .get_typed_func::<
                    (),
                    (wasmtime::component::Resource<WorldResource>,),
                >(&mut store, &self.some_world_func2)?
                .func();
            Ok(TheWorld {
                interface1,
                some_world_func2,
            })
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
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: TheWorldImportsWithStore,
            for<'a> D::Data<'a>: TheWorldImports,
            T: 'static + Send,
        {
            let mut linker = linker.root();
            linker
                .resource_concurrent(
                    "world-resource",
                    wasmtime::component::ResourceType::host::<WorldResource>(),
                    move |caller: &wasmtime::component::Accessor<T>, rep| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            HostWorldResourceWithStore::drop(
                                    accessor,
                                    wasmtime::component::Resource::new_own(rep),
                                )
                                .await
                        })
                    },
                )?;
            linker
                .func_wrap_concurrent(
                    "some-world-func",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as TheWorldImportsWithStore>::some_world_func(
                                    accessor,
                                )
                                .await;
                            Ok((r,))
                        })
                    },
                )?;
            linker
                .func_wrap_concurrent(
                    "[constructor]world-resource",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWorldResourceWithStore>::new(accessor)
                                .await;
                            Ok((r,))
                        })
                    },
                )?;
            linker
                .func_wrap_concurrent(
                    "[method]world-resource.foo",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (wasmtime::component::Resource<WorldResource>,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWorldResourceWithStore>::foo(
                                    accessor,
                                    arg0,
                                )
                                .await;
                            Ok(r)
                        })
                    },
                )?;
            linker
                .func_wrap_concurrent(
                    "[static]world-resource.static-foo",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWorldResourceWithStore>::static_foo(
                                    accessor,
                                )
                                .await;
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
            D: foo::foo::resources::HostWithStore
                + foo::foo::long_use_chain1::HostWithStore
                + foo::foo::long_use_chain2::HostWithStore
                + foo::foo::long_use_chain3::HostWithStore
                + foo::foo::long_use_chain4::HostWithStore
                + foo::foo::transitive_interface_with_resource::HostWithStore
                + TheWorldImportsWithStore + Send,
            for<'a> D::Data<
                'a,
            >: foo::foo::resources::Host + foo::foo::long_use_chain1::Host
                + foo::foo::long_use_chain2::Host + foo::foo::long_use_chain3::Host
                + foo::foo::long_use_chain4::Host
                + foo::foo::transitive_interface_with_resource::Host + TheWorldImports
                + Send,
            T: 'static + Send,
        {
            Self::add_to_linker_imports::<T, D>(linker, host_getter)?;
            foo::foo::resources::add_to_linker::<T, D>(linker, host_getter)?;
            foo::foo::long_use_chain1::add_to_linker::<T, D>(linker, host_getter)?;
            foo::foo::long_use_chain2::add_to_linker::<T, D>(linker, host_getter)?;
            foo::foo::long_use_chain3::add_to_linker::<T, D>(linker, host_getter)?;
            foo::foo::long_use_chain4::add_to_linker::<T, D>(linker, host_getter)?;
            foo::foo::transitive_interface_with_resource::add_to_linker::<
                T,
                D,
            >(linker, host_getter)?;
            Ok(())
        }
        pub async fn call_some_world_func2<_T, _D>(
            &self,
            accessor: &wasmtime::component::Accessor<_T, _D>,
        ) -> wasmtime::Result<wasmtime::component::Resource<WorldResource>>
        where
            _T: Send,
            _D: wasmtime::component::HasData,
        {
            let callee = unsafe {
                wasmtime::component::TypedFunc::<
                    (),
                    (wasmtime::component::Resource<WorldResource>,),
                >::new_unchecked(self.some_world_func2)
            };
            let (ret0,) = callee.call_concurrent(accessor, ()).await?;
            Ok(ret0)
        }
        pub fn foo_foo_uses_resource_transitively(
            &self,
        ) -> &exports::foo::foo::uses_resource_transitively::Guest {
            &self.interface1
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod resources {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub enum Bar {}
            pub trait HostBarWithStore: wasmtime::component::HasData + Send {
                fn drop<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    rep: wasmtime::component::Resource<Bar>,
                ) -> impl ::core::future::Future<Output = wasmtime::Result<()>> + Send
                where
                    Self: Sized;
                fn new<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<
                    Output = wasmtime::component::Resource<Bar>,
                > + Send;
                fn static_a<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = u32> + Send;
                fn method_a<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    self_: wasmtime::component::Resource<Bar>,
                ) -> impl ::core::future::Future<Output = u32> + Send;
            }
            pub trait HostBar: Send {}
            impl<_T: HostBar + ?Sized + Send> HostBar for &mut _T {}
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            pub struct NestedOwn {
                #[component(name = "nested-bar")]
                pub nested_bar: wasmtime::component::Resource<Bar>,
            }
            impl core::fmt::Debug for NestedOwn {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("NestedOwn")
                        .field("nested-bar", &self.nested_bar)
                        .finish()
                }
            }
            const _: () = {
                assert!(
                    4 == < NestedOwn as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    4 == < NestedOwn as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            pub struct NestedBorrow {
                #[component(name = "nested-bar")]
                pub nested_bar: wasmtime::component::Resource<Bar>,
            }
            impl core::fmt::Debug for NestedBorrow {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("NestedBorrow")
                        .field("nested-bar", &self.nested_bar)
                        .finish()
                }
            }
            const _: () = {
                assert!(
                    4 == < NestedBorrow as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    4 == < NestedBorrow as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            pub type SomeHandle = wasmtime::component::Resource<Bar>;
            const _: () = {
                assert!(
                    4 == < SomeHandle as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    4 == < SomeHandle as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            pub trait HostWithStore: wasmtime::component::HasData + HostBarWithStore + Send {
                fn bar_own_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: wasmtime::component::Resource<Bar>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn bar_borrow_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: wasmtime::component::Resource<Bar>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn bar_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<
                    Output = wasmtime::component::Resource<Bar>,
                > + Send;
                fn tuple_own_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: (wasmtime::component::Resource<Bar>, u32),
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn tuple_borrow_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: (wasmtime::component::Resource<Bar>, u32),
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn tuple_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<
                    Output = (wasmtime::component::Resource<Bar>, u32),
                > + Send;
                fn option_own_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Option<wasmtime::component::Resource<Bar>>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn option_borrow_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Option<wasmtime::component::Resource<Bar>>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn option_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<
                    Output = Option<wasmtime::component::Resource<Bar>>,
                > + Send;
                fn result_own_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Result<wasmtime::component::Resource<Bar>, ()>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn result_borrow_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Result<wasmtime::component::Resource<Bar>, ()>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn result_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<
                    Output = Result<wasmtime::component::Resource<Bar>, ()>,
                > + Send;
                fn list_own_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: wasmtime::component::__internal::Vec<
                        wasmtime::component::Resource<Bar>,
                    >,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn list_borrow_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: wasmtime::component::__internal::Vec<
                        wasmtime::component::Resource<Bar>,
                    >,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn list_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<
                    Output = wasmtime::component::__internal::Vec<
                        wasmtime::component::Resource<Bar>,
                    >,
                > + Send;
                fn record_own_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: NestedOwn,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn record_borrow_arg<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: NestedBorrow,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn record_result<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = NestedOwn> + Send;
                fn func_with_handle_typedef<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: SomeHandle,
                ) -> impl ::core::future::Future<Output = ()> + Send;
            }
            pub trait Host: HostBar + Send {}
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
                let mut inst = linker.instance("foo:foo/resources")?;
                inst.resource_concurrent(
                    "bar",
                    wasmtime::component::ResourceType::host::<Bar>(),
                    move |caller: &wasmtime::component::Accessor<T>, rep| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            HostBarWithStore::drop(
                                    accessor,
                                    wasmtime::component::Resource::new_own(rep),
                                )
                                .await
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "[constructor]bar",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostBarWithStore>::new(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "[static]bar.static-a",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostBarWithStore>::static_a(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "[method]bar.method-a",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (wasmtime::component::Resource<Bar>,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostBarWithStore>::method_a(accessor, arg0)
                                .await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "bar-own-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (wasmtime::component::Resource<Bar>,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::bar_own_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "bar-borrow-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (wasmtime::component::Resource<Bar>,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::bar_borrow_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "bar-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::bar_result(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "tuple-own-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): ((wasmtime::component::Resource<Bar>, u32),)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::tuple_own_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "tuple-borrow-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): ((wasmtime::component::Resource<Bar>, u32),)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::tuple_borrow_arg(
                                    accessor,
                                    arg0,
                                )
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
                    "option-own-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (Option<wasmtime::component::Resource<Bar>>,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::option_own_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "option-borrow-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (Option<wasmtime::component::Resource<Bar>>,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::option_borrow_arg(
                                    accessor,
                                    arg0,
                                )
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "option-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::option_result(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "result-own-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (Result<wasmtime::component::Resource<Bar>, ()>,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::result_own_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "result-borrow-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (Result<wasmtime::component::Resource<Bar>, ()>,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::result_borrow_arg(
                                    accessor,
                                    arg0,
                                )
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "result-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::result_result(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "list-own-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (
                            arg0,
                        ): (
                            wasmtime::component::__internal::Vec<
                                wasmtime::component::Resource<Bar>,
                            >,
                        )|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::list_own_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "list-borrow-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (
                            arg0,
                        ): (
                            wasmtime::component::__internal::Vec<
                                wasmtime::component::Resource<Bar>,
                            >,
                        )|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::list_borrow_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "list-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::list_result(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "record-own-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (NestedOwn,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::record_own_arg(accessor, arg0)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "record-borrow-arg",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (NestedBorrow,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::record_borrow_arg(
                                    accessor,
                                    arg0,
                                )
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "record-result",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::record_result(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "func-with-handle-typedef",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (SomeHandle,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::func_with_handle_typedef(
                                    accessor,
                                    arg0,
                                )
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod long_use_chain1 {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub enum A {}
            pub trait HostAWithStore: wasmtime::component::HasData {
                fn drop<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    rep: wasmtime::component::Resource<A>,
                ) -> impl ::core::future::Future<Output = wasmtime::Result<()>> + Send
                where
                    Self: Sized;
            }
            pub trait HostA {}
            impl<_T: HostA + ?Sized + Send> HostA for &mut _T {}
            pub trait HostWithStore: wasmtime::component::HasData + HostAWithStore + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + HostAWithStore + Send,
            {}
            pub trait Host: HostA + Send {}
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
                let mut inst = linker.instance("foo:foo/long-use-chain1")?;
                inst.resource_concurrent(
                    "a",
                    wasmtime::component::ResourceType::host::<A>(),
                    move |caller: &wasmtime::component::Accessor<T>, rep| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            HostAWithStore::drop(
                                    accessor,
                                    wasmtime::component::Resource::new_own(rep),
                                )
                                .await
                        })
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod long_use_chain2 {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub type A = super::super::super::foo::foo::long_use_chain1::A;
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
                let mut inst = linker.instance("foo:foo/long-use-chain2")?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod long_use_chain3 {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub type A = super::super::super::foo::foo::long_use_chain2::A;
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
                let mut inst = linker.instance("foo:foo/long-use-chain3")?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod long_use_chain4 {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub type A = super::super::super::foo::foo::long_use_chain3::A;
            pub trait HostWithStore: wasmtime::component::HasData + Send {
                fn foo<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<
                    Output = wasmtime::component::Resource<A>,
                > + Send;
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
                let mut inst = linker.instance("foo:foo/long-use-chain4")?;
                inst.func_wrap_concurrent(
                    "foo",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::foo(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod transitive_interface_with_resource {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub enum Foo {}
            pub trait HostFooWithStore: wasmtime::component::HasData {
                fn drop<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    rep: wasmtime::component::Resource<Foo>,
                ) -> impl ::core::future::Future<Output = wasmtime::Result<()>> + Send
                where
                    Self: Sized;
            }
            pub trait HostFoo {}
            impl<_T: HostFoo + ?Sized + Send> HostFoo for &mut _T {}
            pub trait HostWithStore: wasmtime::component::HasData + HostFooWithStore + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + HostFooWithStore + Send,
            {}
            pub trait Host: HostFoo + Send {}
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
                let mut inst = linker
                    .instance("foo:foo/transitive-interface-with-resource")?;
                inst.resource_concurrent(
                    "foo",
                    wasmtime::component::ResourceType::host::<Foo>(),
                    move |caller: &wasmtime::component::Accessor<T>, rep| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            HostFooWithStore::drop(
                                    accessor,
                                    wasmtime::component::Resource::new_own(rep),
                                )
                                .await
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
            pub mod uses_resource_transitively {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                pub type Foo = super::super::super::super::foo::foo::transitive_interface_with_resource::Foo;
                pub struct Guest {
                    handle: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    handle: wasmtime::component::ComponentExportIndex,
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
                            .get_export_index(None, "foo:foo/uses-resource-transitively")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/uses-resource-transitively`"
                                )
                            })?;
                        let mut lookup = move |name| {
                            _instance_pre
                                .component()
                                .get_export_index(Some(&instance), name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `foo:foo/uses-resource-transitively` does \
                    not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let handle = lookup("handle")?;
                        Ok(GuestIndices { handle })
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
                        let handle = *_instance
                            .get_typed_func::<
                                (wasmtime::component::Resource<Foo>,),
                                (),
                            >(&mut store, &self.handle)?
                            .func();
                        Ok(Guest { handle })
                    }
                }
                impl Guest {
                    pub async fn call_handle<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: wasmtime::component::Resource<Foo>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::Resource<Foo>,),
                                (),
                            >::new_unchecked(self.handle)
                        };
                        let () = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(())
                    }
                }
            }
        }
    }
}

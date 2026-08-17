pub enum WorldResource {}
pub trait HostWorldResourceWithStore: wasmtime::component::HasData {}
impl<_T: ?Sized> HostWorldResourceWithStore for _T
where
    _T: wasmtime::component::HasData,
{}
pub trait HostWorldResource {
    fn new(&mut self) -> wasmtime::component::Resource<WorldResource>;
    fn foo(&mut self, self_: wasmtime::component::Resource<WorldResource>) -> ();
    fn static_foo(&mut self) -> ();
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<WorldResource>,
    ) -> wasmtime::Result<()>;
}
impl<_T: HostWorldResource + ?Sized> HostWorldResource for &mut _T {
    fn new(&mut self) -> wasmtime::component::Resource<WorldResource> {
        HostWorldResource::new(*self)
    }
    fn foo(&mut self, self_: wasmtime::component::Resource<WorldResource>) -> () {
        HostWorldResource::foo(*self, self_)
    }
    fn static_foo(&mut self) -> () {
        HostWorldResource::static_foo(*self)
    }
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<WorldResource>,
    ) -> wasmtime::Result<()> {
        HostWorldResource::drop(*self, rep)
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
pub trait TheWorldImportsWithStore: wasmtime::component::HasData + HostWorldResourceWithStore {}
impl<_T: ?Sized> TheWorldImportsWithStore for _T
where
    _T: wasmtime::component::HasData + HostWorldResourceWithStore,
{}
pub trait TheWorldImports: HostWorldResource {
    fn some_world_func(&mut self) -> wasmtime::component::Resource<WorldResource>;
}
impl<_T: TheWorldImports + ?Sized> TheWorldImports for &mut _T {
    fn some_world_func(&mut self) -> wasmtime::component::Resource<WorldResource> {
        TheWorldImports::some_world_func(*self)
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
            T: 'static,
        {
            let mut linker = linker.root();
            linker
                .resource(
                    "world-resource",
                    wasmtime::component::ResourceType::host::<WorldResource>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostWorldResource::drop(
                            &mut host_getter(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
            linker
                .func_wrap(
                    "some-world-func",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = TheWorldImports::some_world_func(host);
                        Ok((r,))
                    },
                )?;
            linker
                .func_wrap(
                    "[constructor]world-resource",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostWorldResource::new(host);
                        Ok((r,))
                    },
                )?;
            linker
                .func_wrap(
                    "[method]world-resource.foo",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<WorldResource>,)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostWorldResource::foo(host, arg0);
                        Ok(r)
                    },
                )?;
            linker
                .func_wrap(
                    "[static]world-resource.static-foo",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostWorldResource::static_foo(host);
                        Ok(r)
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
                + TheWorldImportsWithStore,
            for<'a> D::Data<
                'a,
            >: foo::foo::resources::Host + foo::foo::long_use_chain1::Host
                + foo::foo::long_use_chain2::Host + foo::foo::long_use_chain3::Host
                + foo::foo::long_use_chain4::Host
                + foo::foo::transitive_interface_with_resource::Host + TheWorldImports,
            T: 'static,
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
        pub fn call_some_world_func2<S: wasmtime::AsContextMut>(
            &self,
            mut store: S,
        ) -> wasmtime::Result<wasmtime::component::Resource<WorldResource>> {
            let callee = unsafe {
                wasmtime::component::TypedFunc::<
                    (),
                    (wasmtime::component::Resource<WorldResource>,),
                >::new_unchecked(self.some_world_func2)
            };
            let (ret0,) = callee.call(store.as_context_mut(), ())?;
            callee.post_return(store.as_context_mut())?;
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
            pub trait HostBarWithStore: wasmtime::component::HasData {}
            impl<_T: ?Sized> HostBarWithStore for _T
            where
                _T: wasmtime::component::HasData,
            {}
            pub trait HostBar {
                fn new(&mut self) -> wasmtime::component::Resource<Bar>;
                fn static_a(&mut self) -> u32;
                fn method_a(&mut self, self_: wasmtime::component::Resource<Bar>) -> u32;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Bar>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostBar + ?Sized> HostBar for &mut _T {
                fn new(&mut self) -> wasmtime::component::Resource<Bar> {
                    HostBar::new(*self)
                }
                fn static_a(&mut self) -> u32 {
                    HostBar::static_a(*self)
                }
                fn method_a(
                    &mut self,
                    self_: wasmtime::component::Resource<Bar>,
                ) -> u32 {
                    HostBar::method_a(*self, self_)
                }
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Bar>,
                ) -> wasmtime::Result<()> {
                    HostBar::drop(*self, rep)
                }
            }
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
            pub trait HostWithStore: wasmtime::component::HasData + HostBarWithStore {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + HostBarWithStore,
            {}
            pub trait Host: HostBar {
                fn bar_own_arg(&mut self, x: wasmtime::component::Resource<Bar>) -> ();
                fn bar_borrow_arg(
                    &mut self,
                    x: wasmtime::component::Resource<Bar>,
                ) -> ();
                fn bar_result(&mut self) -> wasmtime::component::Resource<Bar>;
                fn tuple_own_arg(
                    &mut self,
                    x: (wasmtime::component::Resource<Bar>, u32),
                ) -> ();
                fn tuple_borrow_arg(
                    &mut self,
                    x: (wasmtime::component::Resource<Bar>, u32),
                ) -> ();
                fn tuple_result(&mut self) -> (wasmtime::component::Resource<Bar>, u32);
                fn option_own_arg(
                    &mut self,
                    x: Option<wasmtime::component::Resource<Bar>>,
                ) -> ();
                fn option_borrow_arg(
                    &mut self,
                    x: Option<wasmtime::component::Resource<Bar>>,
                ) -> ();
                fn option_result(
                    &mut self,
                ) -> Option<wasmtime::component::Resource<Bar>>;
                fn result_own_arg(
                    &mut self,
                    x: Result<wasmtime::component::Resource<Bar>, ()>,
                ) -> ();
                fn result_borrow_arg(
                    &mut self,
                    x: Result<wasmtime::component::Resource<Bar>, ()>,
                ) -> ();
                fn result_result(
                    &mut self,
                ) -> Result<wasmtime::component::Resource<Bar>, ()>;
                fn list_own_arg(
                    &mut self,
                    x: wasmtime::component::__internal::Vec<
                        wasmtime::component::Resource<Bar>,
                    >,
                ) -> ();
                fn list_borrow_arg(
                    &mut self,
                    x: wasmtime::component::__internal::Vec<
                        wasmtime::component::Resource<Bar>,
                    >,
                ) -> ();
                fn list_result(
                    &mut self,
                ) -> wasmtime::component::__internal::Vec<
                    wasmtime::component::Resource<Bar>,
                >;
                fn record_own_arg(&mut self, x: NestedOwn) -> ();
                fn record_borrow_arg(&mut self, x: NestedBorrow) -> ();
                fn record_result(&mut self) -> NestedOwn;
                fn func_with_handle_typedef(&mut self, x: SomeHandle) -> ();
            }
            impl<_T: Host + ?Sized> Host for &mut _T {
                fn bar_own_arg(&mut self, x: wasmtime::component::Resource<Bar>) -> () {
                    Host::bar_own_arg(*self, x)
                }
                fn bar_borrow_arg(
                    &mut self,
                    x: wasmtime::component::Resource<Bar>,
                ) -> () {
                    Host::bar_borrow_arg(*self, x)
                }
                fn bar_result(&mut self) -> wasmtime::component::Resource<Bar> {
                    Host::bar_result(*self)
                }
                fn tuple_own_arg(
                    &mut self,
                    x: (wasmtime::component::Resource<Bar>, u32),
                ) -> () {
                    Host::tuple_own_arg(*self, x)
                }
                fn tuple_borrow_arg(
                    &mut self,
                    x: (wasmtime::component::Resource<Bar>, u32),
                ) -> () {
                    Host::tuple_borrow_arg(*self, x)
                }
                fn tuple_result(&mut self) -> (wasmtime::component::Resource<Bar>, u32) {
                    Host::tuple_result(*self)
                }
                fn option_own_arg(
                    &mut self,
                    x: Option<wasmtime::component::Resource<Bar>>,
                ) -> () {
                    Host::option_own_arg(*self, x)
                }
                fn option_borrow_arg(
                    &mut self,
                    x: Option<wasmtime::component::Resource<Bar>>,
                ) -> () {
                    Host::option_borrow_arg(*self, x)
                }
                fn option_result(
                    &mut self,
                ) -> Option<wasmtime::component::Resource<Bar>> {
                    Host::option_result(*self)
                }
                fn result_own_arg(
                    &mut self,
                    x: Result<wasmtime::component::Resource<Bar>, ()>,
                ) -> () {
                    Host::result_own_arg(*self, x)
                }
                fn result_borrow_arg(
                    &mut self,
                    x: Result<wasmtime::component::Resource<Bar>, ()>,
                ) -> () {
                    Host::result_borrow_arg(*self, x)
                }
                fn result_result(
                    &mut self,
                ) -> Result<wasmtime::component::Resource<Bar>, ()> {
                    Host::result_result(*self)
                }
                fn list_own_arg(
                    &mut self,
                    x: wasmtime::component::__internal::Vec<
                        wasmtime::component::Resource<Bar>,
                    >,
                ) -> () {
                    Host::list_own_arg(*self, x)
                }
                fn list_borrow_arg(
                    &mut self,
                    x: wasmtime::component::__internal::Vec<
                        wasmtime::component::Resource<Bar>,
                    >,
                ) -> () {
                    Host::list_borrow_arg(*self, x)
                }
                fn list_result(
                    &mut self,
                ) -> wasmtime::component::__internal::Vec<
                    wasmtime::component::Resource<Bar>,
                > {
                    Host::list_result(*self)
                }
                fn record_own_arg(&mut self, x: NestedOwn) -> () {
                    Host::record_own_arg(*self, x)
                }
                fn record_borrow_arg(&mut self, x: NestedBorrow) -> () {
                    Host::record_borrow_arg(*self, x)
                }
                fn record_result(&mut self) -> NestedOwn {
                    Host::record_result(*self)
                }
                fn func_with_handle_typedef(&mut self, x: SomeHandle) -> () {
                    Host::func_with_handle_typedef(*self, x)
                }
            }
            pub fn add_to_linker<T, D>(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: fn(&mut T) -> D::Data<'_>,
            ) -> wasmtime::Result<()>
            where
                D: HostWithStore,
                for<'a> D::Data<'a>: Host,
                T: 'static,
            {
                let mut inst = linker.instance("foo:foo/resources")?;
                inst.resource(
                    "bar",
                    wasmtime::component::ResourceType::host::<Bar>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostBar::drop(
                            &mut host_getter(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.func_wrap(
                    "[constructor]bar",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostBar::new(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "[static]bar.static-a",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostBar::static_a(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "[method]bar.method-a",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Bar>,)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostBar::method_a(host, arg0);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "bar-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Bar>,)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::bar_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "bar-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Bar>,)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::bar_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "bar-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::bar_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "tuple-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): ((wasmtime::component::Resource<Bar>, u32),)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::tuple_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "tuple-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): ((wasmtime::component::Resource<Bar>, u32),)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::tuple_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "tuple-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::tuple_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "option-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Option<wasmtime::component::Resource<Bar>>,)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::option_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "option-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Option<wasmtime::component::Resource<Bar>>,)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::option_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "option-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::option_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "result-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Result<wasmtime::component::Resource<Bar>, ()>,)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::result_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "result-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Result<wasmtime::component::Resource<Bar>, ()>,)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::result_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "result-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::result_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "list-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                        ): (
                            wasmtime::component::__internal::Vec<
                                wasmtime::component::Resource<Bar>,
                            >,
                        )|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::list_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "list-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                        ): (
                            wasmtime::component::__internal::Vec<
                                wasmtime::component::Resource<Bar>,
                            >,
                        )|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::list_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "list-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::list_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "record-own-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (NestedOwn,)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::record_own_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "record-borrow-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (NestedBorrow,)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::record_borrow_arg(host, arg0);
                        Ok(r)
                    },
                )?;
                inst.func_wrap(
                    "record-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::record_result(host);
                        Ok((r,))
                    },
                )?;
                inst.func_wrap(
                    "func-with-handle-typedef",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (SomeHandle,)|
                    {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::func_with_handle_typedef(host, arg0);
                        Ok(r)
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
            pub trait HostAWithStore: wasmtime::component::HasData {}
            impl<_T: ?Sized> HostAWithStore for _T
            where
                _T: wasmtime::component::HasData,
            {}
            pub trait HostA {
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<A>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostA + ?Sized> HostA for &mut _T {
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<A>,
                ) -> wasmtime::Result<()> {
                    HostA::drop(*self, rep)
                }
            }
            pub trait HostWithStore: wasmtime::component::HasData + HostAWithStore {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + HostAWithStore,
            {}
            pub trait Host: HostA {}
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
                let mut inst = linker.instance("foo:foo/long-use-chain1")?;
                inst.resource(
                    "a",
                    wasmtime::component::ResourceType::host::<A>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostA::drop(
                            &mut host_getter(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
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
            pub trait HostWithStore: wasmtime::component::HasData {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData,
            {}
            pub trait Host {
                fn foo(&mut self) -> wasmtime::component::Resource<A>;
            }
            impl<_T: Host + ?Sized> Host for &mut _T {
                fn foo(&mut self) -> wasmtime::component::Resource<A> {
                    Host::foo(*self)
                }
            }
            pub fn add_to_linker<T, D>(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: fn(&mut T) -> D::Data<'_>,
            ) -> wasmtime::Result<()>
            where
                D: HostWithStore,
                for<'a> D::Data<'a>: Host,
                T: 'static,
            {
                let mut inst = linker.instance("foo:foo/long-use-chain4")?;
                inst.func_wrap(
                    "foo",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::foo(host);
                        Ok((r,))
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
            pub trait HostFooWithStore: wasmtime::component::HasData {}
            impl<_T: ?Sized> HostFooWithStore for _T
            where
                _T: wasmtime::component::HasData,
            {}
            pub trait HostFoo {
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Foo>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostFoo + ?Sized> HostFoo for &mut _T {
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Foo>,
                ) -> wasmtime::Result<()> {
                    HostFoo::drop(*self, rep)
                }
            }
            pub trait HostWithStore: wasmtime::component::HasData + HostFooWithStore {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + HostFooWithStore,
            {}
            pub trait Host: HostFoo {}
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
                let mut inst = linker
                    .instance("foo:foo/transitive-interface-with-resource")?;
                inst.resource(
                    "foo",
                    wasmtime::component::ResourceType::host::<Foo>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostFoo::drop(
                            &mut host_getter(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
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
                    pub fn call_handle<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::Resource<Foo>,
                    ) -> wasmtime::Result<()> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::Resource<Foo>,),
                                (),
                            >::new_unchecked(self.handle)
                        };
                        let () = callee.call(store.as_context_mut(), (arg0,))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(())
                    }
                }
            }
        }
    }
}

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
    interface0: exports::foo::foo::conventions::GuestIndices,
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
    interface0: exports::foo::foo::conventions::Guest,
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
            let interface0 = exports::foo::foo::conventions::GuestIndices::new(
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
            D: foo::foo::conventions::HostWithStore + Send,
            for<'a> D::Data<'a>: foo::foo::conventions::Host + Send,
            T: 'static + Send,
        {
            foo::foo::conventions::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
        pub fn foo_foo_conventions(&self) -> &exports::foo::foo::conventions::Guest {
            &self.interface0
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod conventions {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct LudicrousSpeed {
                #[component(name = "how-fast-are-you-going")]
                pub how_fast_are_you_going: u32,
                #[component(name = "i-am-going-extremely-slow")]
                pub i_am_going_extremely_slow: u64,
            }
            impl core::fmt::Debug for LudicrousSpeed {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("LudicrousSpeed")
                        .field("how-fast-are-you-going", &self.how_fast_are_you_going)
                        .field(
                            "i-am-going-extremely-slow",
                            &self.i_am_going_extremely_slow,
                        )
                        .finish()
                }
            }
            const _: () = {
                assert!(
                    16 == < LudicrousSpeed as wasmtime::component::ComponentType
                    >::SIZE32
                );
                assert!(
                    8 == < LudicrousSpeed as wasmtime::component::ComponentType
                    >::ALIGN32
                );
            };
            pub trait HostWithStore: wasmtime::component::HasData + Send {
                fn kebab_case<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn foo<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: LudicrousSpeed,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn function_with_dashes<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn function_with_no_weird_characters<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn apple<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn apple_pear<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn apple_pear_grape<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn a0<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                /// Comment out identifiers that collide when mapped to snake_case, for now; see
                ///  https://github.com/WebAssembly/component-model/issues/118
                /// APPLE: func()
                /// APPLE-pear-GRAPE: func()
                /// apple-PEAR-grape: func()
                fn is_xml<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn explicit<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn explicit_kebab<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                /// Identifiers with the same name as keywords are quoted.
                fn bool<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
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
                let mut inst = linker.instance("foo:foo/conventions")?;
                inst.func_wrap_concurrent(
                    "kebab-case",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::kebab_case(accessor).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "foo",
                    move |
                        caller: &wasmtime::component::Accessor<T>,
                        (arg0,): (LudicrousSpeed,)|
                    {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::foo(accessor, arg0).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "function-with-dashes",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::function_with_dashes(accessor)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "function-with-no-weird-characters",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::function_with_no_weird_characters(
                                    accessor,
                                )
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "apple",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::apple(accessor).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "apple-pear",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::apple_pear(accessor).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "apple-pear-grape",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::apple_pear_grape(accessor)
                                .await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "a0",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::a0(accessor).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "is-XML",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::is_xml(accessor).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "explicit",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::explicit(accessor).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "explicit-kebab",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::explicit_kebab(accessor).await;
                            Ok(r)
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "bool",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::bool(accessor).await;
                            Ok(r)
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
            pub mod conventions {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(record)]
                #[derive(Clone, Copy)]
                pub struct LudicrousSpeed {
                    #[component(name = "how-fast-are-you-going")]
                    pub how_fast_are_you_going: u32,
                    #[component(name = "i-am-going-extremely-slow")]
                    pub i_am_going_extremely_slow: u64,
                }
                impl core::fmt::Debug for LudicrousSpeed {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("LudicrousSpeed")
                            .field(
                                "how-fast-are-you-going",
                                &self.how_fast_are_you_going,
                            )
                            .field(
                                "i-am-going-extremely-slow",
                                &self.i_am_going_extremely_slow,
                            )
                            .finish()
                    }
                }
                const _: () = {
                    assert!(
                        16 == < LudicrousSpeed as wasmtime::component::ComponentType
                        >::SIZE32
                    );
                    assert!(
                        8 == < LudicrousSpeed as wasmtime::component::ComponentType
                        >::ALIGN32
                    );
                };
                pub struct Guest {
                    kebab_case: wasmtime::component::Func,
                    foo: wasmtime::component::Func,
                    function_with_dashes: wasmtime::component::Func,
                    function_with_no_weird_characters: wasmtime::component::Func,
                    apple: wasmtime::component::Func,
                    apple_pear: wasmtime::component::Func,
                    apple_pear_grape: wasmtime::component::Func,
                    a0: wasmtime::component::Func,
                    is_xml: wasmtime::component::Func,
                    explicit: wasmtime::component::Func,
                    explicit_kebab: wasmtime::component::Func,
                    bool: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    kebab_case: wasmtime::component::ComponentExportIndex,
                    foo: wasmtime::component::ComponentExportIndex,
                    function_with_dashes: wasmtime::component::ComponentExportIndex,
                    function_with_no_weird_characters: wasmtime::component::ComponentExportIndex,
                    apple: wasmtime::component::ComponentExportIndex,
                    apple_pear: wasmtime::component::ComponentExportIndex,
                    apple_pear_grape: wasmtime::component::ComponentExportIndex,
                    a0: wasmtime::component::ComponentExportIndex,
                    is_xml: wasmtime::component::ComponentExportIndex,
                    explicit: wasmtime::component::ComponentExportIndex,
                    explicit_kebab: wasmtime::component::ComponentExportIndex,
                    bool: wasmtime::component::ComponentExportIndex,
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
                            .get_export_index(None, "foo:foo/conventions")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/conventions`"
                                )
                            })?;
                        let mut lookup = move |name| {
                            _instance_pre
                                .component()
                                .get_export_index(Some(&instance), name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `foo:foo/conventions` does \
                not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let kebab_case = lookup("kebab-case")?;
                        let foo = lookup("foo")?;
                        let function_with_dashes = lookup("function-with-dashes")?;
                        let function_with_no_weird_characters = lookup(
                            "function-with-no-weird-characters",
                        )?;
                        let apple = lookup("apple")?;
                        let apple_pear = lookup("apple-pear")?;
                        let apple_pear_grape = lookup("apple-pear-grape")?;
                        let a0 = lookup("a0")?;
                        let is_xml = lookup("is-XML")?;
                        let explicit = lookup("explicit")?;
                        let explicit_kebab = lookup("explicit-kebab")?;
                        let bool = lookup("bool")?;
                        Ok(GuestIndices {
                            kebab_case,
                            foo,
                            function_with_dashes,
                            function_with_no_weird_characters,
                            apple,
                            apple_pear,
                            apple_pear_grape,
                            a0,
                            is_xml,
                            explicit,
                            explicit_kebab,
                            bool,
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
                        let kebab_case = *_instance
                            .get_typed_func::<(), ()>(&mut store, &self.kebab_case)?
                            .func();
                        let foo = *_instance
                            .get_typed_func::<
                                (LudicrousSpeed,),
                                (),
                            >(&mut store, &self.foo)?
                            .func();
                        let function_with_dashes = *_instance
                            .get_typed_func::<
                                (),
                                (),
                            >(&mut store, &self.function_with_dashes)?
                            .func();
                        let function_with_no_weird_characters = *_instance
                            .get_typed_func::<
                                (),
                                (),
                            >(&mut store, &self.function_with_no_weird_characters)?
                            .func();
                        let apple = *_instance
                            .get_typed_func::<(), ()>(&mut store, &self.apple)?
                            .func();
                        let apple_pear = *_instance
                            .get_typed_func::<(), ()>(&mut store, &self.apple_pear)?
                            .func();
                        let apple_pear_grape = *_instance
                            .get_typed_func::<
                                (),
                                (),
                            >(&mut store, &self.apple_pear_grape)?
                            .func();
                        let a0 = *_instance
                            .get_typed_func::<(), ()>(&mut store, &self.a0)?
                            .func();
                        let is_xml = *_instance
                            .get_typed_func::<(), ()>(&mut store, &self.is_xml)?
                            .func();
                        let explicit = *_instance
                            .get_typed_func::<(), ()>(&mut store, &self.explicit)?
                            .func();
                        let explicit_kebab = *_instance
                            .get_typed_func::<(), ()>(&mut store, &self.explicit_kebab)?
                            .func();
                        let bool = *_instance
                            .get_typed_func::<(), ()>(&mut store, &self.bool)?
                            .func();
                        Ok(Guest {
                            kebab_case,
                            foo,
                            function_with_dashes,
                            function_with_no_weird_characters,
                            apple,
                            apple_pear,
                            apple_pear_grape,
                            a0,
                            is_xml,
                            explicit,
                            explicit_kebab,
                            bool,
                        })
                    }
                }
                impl Guest {
                    pub async fn call_kebab_case<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.kebab_case)
                        };
                        let () = callee.call_concurrent(accessor, ()).await?;
                        Ok(())
                    }
                    pub async fn call_foo<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: LudicrousSpeed,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (LudicrousSpeed,),
                                (),
                            >::new_unchecked(self.foo)
                        };
                        let () = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(())
                    }
                    pub async fn call_function_with_dashes<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.function_with_dashes)
                        };
                        let () = callee.call_concurrent(accessor, ()).await?;
                        Ok(())
                    }
                    pub async fn call_function_with_no_weird_characters<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.function_with_no_weird_characters)
                        };
                        let () = callee.call_concurrent(accessor, ()).await?;
                        Ok(())
                    }
                    pub async fn call_apple<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.apple)
                        };
                        let () = callee.call_concurrent(accessor, ()).await?;
                        Ok(())
                    }
                    pub async fn call_apple_pear<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.apple_pear)
                        };
                        let () = callee.call_concurrent(accessor, ()).await?;
                        Ok(())
                    }
                    pub async fn call_apple_pear_grape<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.apple_pear_grape)
                        };
                        let () = callee.call_concurrent(accessor, ()).await?;
                        Ok(())
                    }
                    pub async fn call_a0<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.a0)
                        };
                        let () = callee.call_concurrent(accessor, ()).await?;
                        Ok(())
                    }
                    /// Comment out identifiers that collide when mapped to snake_case, for now; see
                    ///  https://github.com/WebAssembly/component-model/issues/118
                    /// APPLE: func()
                    /// APPLE-pear-GRAPE: func()
                    /// apple-PEAR-grape: func()
                    pub async fn call_is_xml<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.is_xml)
                        };
                        let () = callee.call_concurrent(accessor, ()).await?;
                        Ok(())
                    }
                    pub async fn call_explicit<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.explicit)
                        };
                        let () = callee.call_concurrent(accessor, ()).await?;
                        Ok(())
                    }
                    pub async fn call_explicit_kebab<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.explicit_kebab)
                        };
                        let () = callee.call_concurrent(accessor, ()).await?;
                        Ok(())
                    }
                    /// Identifiers with the same name as keywords are quoted.
                    pub async fn call_bool<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                    ) -> wasmtime::Result<()>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (),
                            >::new_unchecked(self.bool)
                        };
                        let () = callee.call_concurrent(accessor, ()).await?;
                        Ok(())
                    }
                }
            }
        }
    }
}

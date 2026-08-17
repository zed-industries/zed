/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `the-flags`.
///
/// This structure is created through [`TheFlagsPre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`TheFlags`] as well.
pub struct TheFlagsPre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: TheFlagsIndices,
}
impl<T: 'static> Clone for TheFlagsPre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> TheFlagsPre<_T> {
    /// Creates a new copy of `TheFlagsPre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = TheFlagsIndices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`TheFlags`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<TheFlags> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> TheFlagsPre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<TheFlags> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `the-flags`.
///
/// This is an implementation detail of [`TheFlagsPre`] and can
/// be constructed if needed as well.
///
/// For more information see [`TheFlags`] as well.
#[derive(Clone)]
pub struct TheFlagsIndices {
    interface0: exports::foo::foo::flegs::GuestIndices,
}
/// Auto-generated bindings for an instance a component which
/// implements the world `the-flags`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`TheFlags::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`TheFlagsPre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`TheFlagsPre::instantiate`] to
///   create a [`TheFlags`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`TheFlags::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct TheFlags {
    interface0: exports::foo::foo::flegs::Guest,
}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl TheFlagsIndices {
        /// Creates a new copy of `TheFlagsIndices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            let interface0 = exports::foo::foo::flegs::GuestIndices::new(_instance_pre)?;
            Ok(TheFlagsIndices { interface0 })
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`TheFlags`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<TheFlags> {
            let _ = &mut store;
            let _instance = instance;
            let interface0 = self.interface0.load(&mut store, &_instance)?;
            Ok(TheFlags { interface0 })
        }
    }
    impl TheFlags {
        /// Convenience wrapper around [`TheFlagsPre::new`] and
        /// [`TheFlagsPre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<TheFlags> {
            let pre = linker.instantiate_pre(component)?;
            TheFlagsPre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`TheFlagsIndices::new`] and
        /// [`TheFlagsIndices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<TheFlags> {
            let indices = TheFlagsIndices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`TheFlagsPre::new`] and
        /// [`TheFlagsPre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<TheFlags>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            TheFlagsPre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: foo::foo::flegs::HostWithStore + Send,
            for<'a> D::Data<'a>: foo::foo::flegs::Host + Send,
            T: 'static + Send,
        {
            foo::foo::flegs::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
        pub fn foo_foo_flegs(&self) -> &exports::foo::foo::flegs::Guest {
            &self.interface0
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod flegs {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            wasmtime::component::flags!(Flag1 { #[component(name = "b0")] const B0; });
            const _: () = {
                assert!(1 == < Flag1 as wasmtime::component::ComponentType >::SIZE32);
                assert!(1 == < Flag1 as wasmtime::component::ComponentType >::ALIGN32);
            };
            wasmtime::component::flags!(
                Flag2 { #[component(name = "b0")] const B0; #[component(name = "b1")]
                const B1; }
            );
            const _: () = {
                assert!(1 == < Flag2 as wasmtime::component::ComponentType >::SIZE32);
                assert!(1 == < Flag2 as wasmtime::component::ComponentType >::ALIGN32);
            };
            wasmtime::component::flags!(
                Flag4 { #[component(name = "b0")] const B0; #[component(name = "b1")]
                const B1; #[component(name = "b2")] const B2; #[component(name = "b3")]
                const B3; }
            );
            const _: () = {
                assert!(1 == < Flag4 as wasmtime::component::ComponentType >::SIZE32);
                assert!(1 == < Flag4 as wasmtime::component::ComponentType >::ALIGN32);
            };
            wasmtime::component::flags!(
                Flag8 { #[component(name = "b0")] const B0; #[component(name = "b1")]
                const B1; #[component(name = "b2")] const B2; #[component(name = "b3")]
                const B3; #[component(name = "b4")] const B4; #[component(name = "b5")]
                const B5; #[component(name = "b6")] const B6; #[component(name = "b7")]
                const B7; }
            );
            const _: () = {
                assert!(1 == < Flag8 as wasmtime::component::ComponentType >::SIZE32);
                assert!(1 == < Flag8 as wasmtime::component::ComponentType >::ALIGN32);
            };
            wasmtime::component::flags!(
                Flag16 { #[component(name = "b0")] const B0; #[component(name = "b1")]
                const B1; #[component(name = "b2")] const B2; #[component(name = "b3")]
                const B3; #[component(name = "b4")] const B4; #[component(name = "b5")]
                const B5; #[component(name = "b6")] const B6; #[component(name = "b7")]
                const B7; #[component(name = "b8")] const B8; #[component(name = "b9")]
                const B9; #[component(name = "b10")] const B10; #[component(name =
                "b11")] const B11; #[component(name = "b12")] const B12; #[component(name
                = "b13")] const B13; #[component(name = "b14")] const B14;
                #[component(name = "b15")] const B15; }
            );
            const _: () = {
                assert!(2 == < Flag16 as wasmtime::component::ComponentType >::SIZE32);
                assert!(2 == < Flag16 as wasmtime::component::ComponentType >::ALIGN32);
            };
            wasmtime::component::flags!(
                Flag32 { #[component(name = "b0")] const B0; #[component(name = "b1")]
                const B1; #[component(name = "b2")] const B2; #[component(name = "b3")]
                const B3; #[component(name = "b4")] const B4; #[component(name = "b5")]
                const B5; #[component(name = "b6")] const B6; #[component(name = "b7")]
                const B7; #[component(name = "b8")] const B8; #[component(name = "b9")]
                const B9; #[component(name = "b10")] const B10; #[component(name =
                "b11")] const B11; #[component(name = "b12")] const B12; #[component(name
                = "b13")] const B13; #[component(name = "b14")] const B14;
                #[component(name = "b15")] const B15; #[component(name = "b16")] const
                B16; #[component(name = "b17")] const B17; #[component(name = "b18")]
                const B18; #[component(name = "b19")] const B19; #[component(name =
                "b20")] const B20; #[component(name = "b21")] const B21; #[component(name
                = "b22")] const B22; #[component(name = "b23")] const B23;
                #[component(name = "b24")] const B24; #[component(name = "b25")] const
                B25; #[component(name = "b26")] const B26; #[component(name = "b27")]
                const B27; #[component(name = "b28")] const B28; #[component(name =
                "b29")] const B29; #[component(name = "b30")] const B30; #[component(name
                = "b31")] const B31; }
            );
            const _: () = {
                assert!(4 == < Flag32 as wasmtime::component::ComponentType >::SIZE32);
                assert!(4 == < Flag32 as wasmtime::component::ComponentType >::ALIGN32);
            };
            wasmtime::component::flags!(
                Flag64 { #[component(name = "b0")] const B0; #[component(name = "b1")]
                const B1; #[component(name = "b2")] const B2; #[component(name = "b3")]
                const B3; #[component(name = "b4")] const B4; #[component(name = "b5")]
                const B5; #[component(name = "b6")] const B6; #[component(name = "b7")]
                const B7; #[component(name = "b8")] const B8; #[component(name = "b9")]
                const B9; #[component(name = "b10")] const B10; #[component(name =
                "b11")] const B11; #[component(name = "b12")] const B12; #[component(name
                = "b13")] const B13; #[component(name = "b14")] const B14;
                #[component(name = "b15")] const B15; #[component(name = "b16")] const
                B16; #[component(name = "b17")] const B17; #[component(name = "b18")]
                const B18; #[component(name = "b19")] const B19; #[component(name =
                "b20")] const B20; #[component(name = "b21")] const B21; #[component(name
                = "b22")] const B22; #[component(name = "b23")] const B23;
                #[component(name = "b24")] const B24; #[component(name = "b25")] const
                B25; #[component(name = "b26")] const B26; #[component(name = "b27")]
                const B27; #[component(name = "b28")] const B28; #[component(name =
                "b29")] const B29; #[component(name = "b30")] const B30; #[component(name
                = "b31")] const B31; #[component(name = "b32")] const B32;
                #[component(name = "b33")] const B33; #[component(name = "b34")] const
                B34; #[component(name = "b35")] const B35; #[component(name = "b36")]
                const B36; #[component(name = "b37")] const B37; #[component(name =
                "b38")] const B38; #[component(name = "b39")] const B39; #[component(name
                = "b40")] const B40; #[component(name = "b41")] const B41;
                #[component(name = "b42")] const B42; #[component(name = "b43")] const
                B43; #[component(name = "b44")] const B44; #[component(name = "b45")]
                const B45; #[component(name = "b46")] const B46; #[component(name =
                "b47")] const B47; #[component(name = "b48")] const B48; #[component(name
                = "b49")] const B49; #[component(name = "b50")] const B50;
                #[component(name = "b51")] const B51; #[component(name = "b52")] const
                B52; #[component(name = "b53")] const B53; #[component(name = "b54")]
                const B54; #[component(name = "b55")] const B55; #[component(name =
                "b56")] const B56; #[component(name = "b57")] const B57; #[component(name
                = "b58")] const B58; #[component(name = "b59")] const B59;
                #[component(name = "b60")] const B60; #[component(name = "b61")] const
                B61; #[component(name = "b62")] const B62; #[component(name = "b63")]
                const B63; }
            );
            const _: () = {
                assert!(8 == < Flag64 as wasmtime::component::ComponentType >::SIZE32);
                assert!(4 == < Flag64 as wasmtime::component::ComponentType >::ALIGN32);
            };
            pub trait HostWithStore: wasmtime::component::HasData + Send {
                fn roundtrip_flag1<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Flag1,
                ) -> impl ::core::future::Future<Output = Flag1> + Send;
                fn roundtrip_flag2<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Flag2,
                ) -> impl ::core::future::Future<Output = Flag2> + Send;
                fn roundtrip_flag4<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Flag4,
                ) -> impl ::core::future::Future<Output = Flag4> + Send;
                fn roundtrip_flag8<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Flag8,
                ) -> impl ::core::future::Future<Output = Flag8> + Send;
                fn roundtrip_flag16<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Flag16,
                ) -> impl ::core::future::Future<Output = Flag16> + Send;
                fn roundtrip_flag32<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Flag32,
                ) -> impl ::core::future::Future<Output = Flag32> + Send;
                fn roundtrip_flag64<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    x: Flag64,
                ) -> impl ::core::future::Future<Output = Flag64> + Send;
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
                let mut inst = linker.instance("foo:foo/flegs")?;
                inst.func_wrap_concurrent(
                    "roundtrip-flag1",
                    move |caller: &wasmtime::component::Accessor<T>, (arg0,): (Flag1,)| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::roundtrip_flag1(accessor, arg0)
                                .await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "roundtrip-flag2",
                    move |caller: &wasmtime::component::Accessor<T>, (arg0,): (Flag2,)| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::roundtrip_flag2(accessor, arg0)
                                .await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "roundtrip-flag4",
                    move |caller: &wasmtime::component::Accessor<T>, (arg0,): (Flag4,)| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::roundtrip_flag4(accessor, arg0)
                                .await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "roundtrip-flag8",
                    move |caller: &wasmtime::component::Accessor<T>, (arg0,): (Flag8,)| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::roundtrip_flag8(accessor, arg0)
                                .await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "roundtrip-flag16",
                    move |caller: &wasmtime::component::Accessor<T>, (arg0,): (Flag16,)| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::roundtrip_flag16(
                                    accessor,
                                    arg0,
                                )
                                .await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "roundtrip-flag32",
                    move |caller: &wasmtime::component::Accessor<T>, (arg0,): (Flag32,)| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::roundtrip_flag32(
                                    accessor,
                                    arg0,
                                )
                                .await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "roundtrip-flag64",
                    move |caller: &wasmtime::component::Accessor<T>, (arg0,): (Flag64,)| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::roundtrip_flag64(
                                    accessor,
                                    arg0,
                                )
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
            pub mod flegs {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                wasmtime::component::flags!(
                    Flag1 { #[component(name = "b0")] const B0; }
                );
                const _: () = {
                    assert!(
                        1 == < Flag1 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        1 == < Flag1 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                wasmtime::component::flags!(
                    Flag2 { #[component(name = "b0")] const B0; #[component(name = "b1")]
                    const B1; }
                );
                const _: () = {
                    assert!(
                        1 == < Flag2 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        1 == < Flag2 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                wasmtime::component::flags!(
                    Flag4 { #[component(name = "b0")] const B0; #[component(name = "b1")]
                    const B1; #[component(name = "b2")] const B2; #[component(name =
                    "b3")] const B3; }
                );
                const _: () = {
                    assert!(
                        1 == < Flag4 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        1 == < Flag4 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                wasmtime::component::flags!(
                    Flag8 { #[component(name = "b0")] const B0; #[component(name = "b1")]
                    const B1; #[component(name = "b2")] const B2; #[component(name =
                    "b3")] const B3; #[component(name = "b4")] const B4; #[component(name
                    = "b5")] const B5; #[component(name = "b6")] const B6;
                    #[component(name = "b7")] const B7; }
                );
                const _: () = {
                    assert!(
                        1 == < Flag8 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        1 == < Flag8 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                wasmtime::component::flags!(
                    Flag16 { #[component(name = "b0")] const B0; #[component(name =
                    "b1")] const B1; #[component(name = "b2")] const B2; #[component(name
                    = "b3")] const B3; #[component(name = "b4")] const B4;
                    #[component(name = "b5")] const B5; #[component(name = "b6")] const
                    B6; #[component(name = "b7")] const B7; #[component(name = "b8")]
                    const B8; #[component(name = "b9")] const B9; #[component(name =
                    "b10")] const B10; #[component(name = "b11")] const B11;
                    #[component(name = "b12")] const B12; #[component(name = "b13")]
                    const B13; #[component(name = "b14")] const B14; #[component(name =
                    "b15")] const B15; }
                );
                const _: () = {
                    assert!(
                        2 == < Flag16 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        2 == < Flag16 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                wasmtime::component::flags!(
                    Flag32 { #[component(name = "b0")] const B0; #[component(name =
                    "b1")] const B1; #[component(name = "b2")] const B2; #[component(name
                    = "b3")] const B3; #[component(name = "b4")] const B4;
                    #[component(name = "b5")] const B5; #[component(name = "b6")] const
                    B6; #[component(name = "b7")] const B7; #[component(name = "b8")]
                    const B8; #[component(name = "b9")] const B9; #[component(name =
                    "b10")] const B10; #[component(name = "b11")] const B11;
                    #[component(name = "b12")] const B12; #[component(name = "b13")]
                    const B13; #[component(name = "b14")] const B14; #[component(name =
                    "b15")] const B15; #[component(name = "b16")] const B16;
                    #[component(name = "b17")] const B17; #[component(name = "b18")]
                    const B18; #[component(name = "b19")] const B19; #[component(name =
                    "b20")] const B20; #[component(name = "b21")] const B21;
                    #[component(name = "b22")] const B22; #[component(name = "b23")]
                    const B23; #[component(name = "b24")] const B24; #[component(name =
                    "b25")] const B25; #[component(name = "b26")] const B26;
                    #[component(name = "b27")] const B27; #[component(name = "b28")]
                    const B28; #[component(name = "b29")] const B29; #[component(name =
                    "b30")] const B30; #[component(name = "b31")] const B31; }
                );
                const _: () = {
                    assert!(
                        4 == < Flag32 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        4 == < Flag32 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                wasmtime::component::flags!(
                    Flag64 { #[component(name = "b0")] const B0; #[component(name =
                    "b1")] const B1; #[component(name = "b2")] const B2; #[component(name
                    = "b3")] const B3; #[component(name = "b4")] const B4;
                    #[component(name = "b5")] const B5; #[component(name = "b6")] const
                    B6; #[component(name = "b7")] const B7; #[component(name = "b8")]
                    const B8; #[component(name = "b9")] const B9; #[component(name =
                    "b10")] const B10; #[component(name = "b11")] const B11;
                    #[component(name = "b12")] const B12; #[component(name = "b13")]
                    const B13; #[component(name = "b14")] const B14; #[component(name =
                    "b15")] const B15; #[component(name = "b16")] const B16;
                    #[component(name = "b17")] const B17; #[component(name = "b18")]
                    const B18; #[component(name = "b19")] const B19; #[component(name =
                    "b20")] const B20; #[component(name = "b21")] const B21;
                    #[component(name = "b22")] const B22; #[component(name = "b23")]
                    const B23; #[component(name = "b24")] const B24; #[component(name =
                    "b25")] const B25; #[component(name = "b26")] const B26;
                    #[component(name = "b27")] const B27; #[component(name = "b28")]
                    const B28; #[component(name = "b29")] const B29; #[component(name =
                    "b30")] const B30; #[component(name = "b31")] const B31;
                    #[component(name = "b32")] const B32; #[component(name = "b33")]
                    const B33; #[component(name = "b34")] const B34; #[component(name =
                    "b35")] const B35; #[component(name = "b36")] const B36;
                    #[component(name = "b37")] const B37; #[component(name = "b38")]
                    const B38; #[component(name = "b39")] const B39; #[component(name =
                    "b40")] const B40; #[component(name = "b41")] const B41;
                    #[component(name = "b42")] const B42; #[component(name = "b43")]
                    const B43; #[component(name = "b44")] const B44; #[component(name =
                    "b45")] const B45; #[component(name = "b46")] const B46;
                    #[component(name = "b47")] const B47; #[component(name = "b48")]
                    const B48; #[component(name = "b49")] const B49; #[component(name =
                    "b50")] const B50; #[component(name = "b51")] const B51;
                    #[component(name = "b52")] const B52; #[component(name = "b53")]
                    const B53; #[component(name = "b54")] const B54; #[component(name =
                    "b55")] const B55; #[component(name = "b56")] const B56;
                    #[component(name = "b57")] const B57; #[component(name = "b58")]
                    const B58; #[component(name = "b59")] const B59; #[component(name =
                    "b60")] const B60; #[component(name = "b61")] const B61;
                    #[component(name = "b62")] const B62; #[component(name = "b63")]
                    const B63; }
                );
                const _: () = {
                    assert!(
                        8 == < Flag64 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        4 == < Flag64 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                pub struct Guest {
                    roundtrip_flag1: wasmtime::component::Func,
                    roundtrip_flag2: wasmtime::component::Func,
                    roundtrip_flag4: wasmtime::component::Func,
                    roundtrip_flag8: wasmtime::component::Func,
                    roundtrip_flag16: wasmtime::component::Func,
                    roundtrip_flag32: wasmtime::component::Func,
                    roundtrip_flag64: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    roundtrip_flag1: wasmtime::component::ComponentExportIndex,
                    roundtrip_flag2: wasmtime::component::ComponentExportIndex,
                    roundtrip_flag4: wasmtime::component::ComponentExportIndex,
                    roundtrip_flag8: wasmtime::component::ComponentExportIndex,
                    roundtrip_flag16: wasmtime::component::ComponentExportIndex,
                    roundtrip_flag32: wasmtime::component::ComponentExportIndex,
                    roundtrip_flag64: wasmtime::component::ComponentExportIndex,
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
                            .get_export_index(None, "foo:foo/flegs")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/flegs`"
                                )
                            })?;
                        let mut lookup = move |name| {
                            _instance_pre
                                .component()
                                .get_export_index(Some(&instance), name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `foo:foo/flegs` does \
                not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let roundtrip_flag1 = lookup("roundtrip-flag1")?;
                        let roundtrip_flag2 = lookup("roundtrip-flag2")?;
                        let roundtrip_flag4 = lookup("roundtrip-flag4")?;
                        let roundtrip_flag8 = lookup("roundtrip-flag8")?;
                        let roundtrip_flag16 = lookup("roundtrip-flag16")?;
                        let roundtrip_flag32 = lookup("roundtrip-flag32")?;
                        let roundtrip_flag64 = lookup("roundtrip-flag64")?;
                        Ok(GuestIndices {
                            roundtrip_flag1,
                            roundtrip_flag2,
                            roundtrip_flag4,
                            roundtrip_flag8,
                            roundtrip_flag16,
                            roundtrip_flag32,
                            roundtrip_flag64,
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
                        let roundtrip_flag1 = *_instance
                            .get_typed_func::<
                                (Flag1,),
                                (Flag1,),
                            >(&mut store, &self.roundtrip_flag1)?
                            .func();
                        let roundtrip_flag2 = *_instance
                            .get_typed_func::<
                                (Flag2,),
                                (Flag2,),
                            >(&mut store, &self.roundtrip_flag2)?
                            .func();
                        let roundtrip_flag4 = *_instance
                            .get_typed_func::<
                                (Flag4,),
                                (Flag4,),
                            >(&mut store, &self.roundtrip_flag4)?
                            .func();
                        let roundtrip_flag8 = *_instance
                            .get_typed_func::<
                                (Flag8,),
                                (Flag8,),
                            >(&mut store, &self.roundtrip_flag8)?
                            .func();
                        let roundtrip_flag16 = *_instance
                            .get_typed_func::<
                                (Flag16,),
                                (Flag16,),
                            >(&mut store, &self.roundtrip_flag16)?
                            .func();
                        let roundtrip_flag32 = *_instance
                            .get_typed_func::<
                                (Flag32,),
                                (Flag32,),
                            >(&mut store, &self.roundtrip_flag32)?
                            .func();
                        let roundtrip_flag64 = *_instance
                            .get_typed_func::<
                                (Flag64,),
                                (Flag64,),
                            >(&mut store, &self.roundtrip_flag64)?
                            .func();
                        Ok(Guest {
                            roundtrip_flag1,
                            roundtrip_flag2,
                            roundtrip_flag4,
                            roundtrip_flag8,
                            roundtrip_flag16,
                            roundtrip_flag32,
                            roundtrip_flag64,
                        })
                    }
                }
                impl Guest {
                    pub async fn call_roundtrip_flag1<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: Flag1,
                    ) -> wasmtime::Result<Flag1>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (Flag1,),
                                (Flag1,),
                            >::new_unchecked(self.roundtrip_flag1)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(ret0)
                    }
                    pub async fn call_roundtrip_flag2<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: Flag2,
                    ) -> wasmtime::Result<Flag2>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (Flag2,),
                                (Flag2,),
                            >::new_unchecked(self.roundtrip_flag2)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(ret0)
                    }
                    pub async fn call_roundtrip_flag4<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: Flag4,
                    ) -> wasmtime::Result<Flag4>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (Flag4,),
                                (Flag4,),
                            >::new_unchecked(self.roundtrip_flag4)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(ret0)
                    }
                    pub async fn call_roundtrip_flag8<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: Flag8,
                    ) -> wasmtime::Result<Flag8>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (Flag8,),
                                (Flag8,),
                            >::new_unchecked(self.roundtrip_flag8)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(ret0)
                    }
                    pub async fn call_roundtrip_flag16<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: Flag16,
                    ) -> wasmtime::Result<Flag16>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (Flag16,),
                                (Flag16,),
                            >::new_unchecked(self.roundtrip_flag16)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(ret0)
                    }
                    pub async fn call_roundtrip_flag32<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: Flag32,
                    ) -> wasmtime::Result<Flag32>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (Flag32,),
                                (Flag32,),
                            >::new_unchecked(self.roundtrip_flag32)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(ret0)
                    }
                    pub async fn call_roundtrip_flag64<_T, _D>(
                        &self,
                        accessor: &wasmtime::component::Accessor<_T, _D>,
                        arg0: Flag64,
                    ) -> wasmtime::Result<Flag64>
                    where
                        _T: Send,
                        _D: wasmtime::component::HasData,
                    {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (Flag64,),
                                (Flag64,),
                            >::new_unchecked(self.roundtrip_flag64)
                        };
                        let (ret0,) = callee.call_concurrent(accessor, (arg0,)).await?;
                        Ok(ret0)
                    }
                }
            }
        }
    }
}

/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `wasi`.
///
/// This structure is created through [`WasiPre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`Wasi`] as well.
pub struct WasiPre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: WasiIndices,
}
impl<T: 'static> Clone for WasiPre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> WasiPre<_T> {
    /// Creates a new copy of `WasiPre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = WasiIndices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`Wasi`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Wasi> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> WasiPre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Wasi> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `wasi`.
///
/// This is an implementation detail of [`WasiPre`] and can
/// be constructed if needed as well.
///
/// For more information see [`Wasi`] as well.
#[derive(Clone)]
pub struct WasiIndices {}
/// Auto-generated bindings for an instance a component which
/// implements the world `wasi`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`Wasi::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`WasiPre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`WasiPre::instantiate`] to
///   create a [`Wasi`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`Wasi::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct Wasi {}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl WasiIndices {
        /// Creates a new copy of `WasiIndices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            Ok(WasiIndices {})
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`Wasi`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Wasi> {
            let _ = &mut store;
            let _instance = instance;
            Ok(Wasi {})
        }
    }
    impl Wasi {
        /// Convenience wrapper around [`WasiPre::new`] and
        /// [`WasiPre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Wasi> {
            let pre = linker.instantiate_pre(component)?;
            WasiPre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`WasiIndices::new`] and
        /// [`WasiIndices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Wasi> {
            let indices = WasiIndices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`WasiPre::new`] and
        /// [`WasiPre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Wasi>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            WasiPre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: foo::foo::wasi_filesystem::HostWithStore
                + foo::foo::wall_clock::HostWithStore + Send,
            for<'a> D::Data<
                'a,
            >: foo::foo::wasi_filesystem::Host + foo::foo::wall_clock::Host + Send,
            T: 'static + Send,
        {
            foo::foo::wasi_filesystem::add_to_linker::<T, D>(linker, host_getter)?;
            foo::foo::wall_clock::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod wasi_filesystem {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct DescriptorStat {}
            impl core::fmt::Debug for DescriptorStat {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("DescriptorStat").finish()
                }
            }
            const _: () = {
                assert!(
                    0 == < DescriptorStat as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    1 == < DescriptorStat as wasmtime::component::ComponentType
                    >::ALIGN32
                );
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(enum)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            #[repr(u8)]
            pub enum Errno {
                #[component(name = "e")]
                E,
            }
            impl Errno {
                pub fn name(&self) -> &'static str {
                    match self {
                        Errno::E => "e",
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        Errno::E => "",
                    }
                }
            }
            impl core::fmt::Debug for Errno {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Errno")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl core::fmt::Display for Errno {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{} (error {})", self.name(), * self as i32)
                }
            }
            impl core::error::Error for Errno {}
            const _: () = {
                assert!(1 == < Errno as wasmtime::component::ComponentType >::SIZE32);
                assert!(1 == < Errno as wasmtime::component::ComponentType >::ALIGN32);
            };
            pub trait HostWithStore: wasmtime::component::HasData + Send {
                fn create_directory_at<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<Output = Result<(), Errno>> + Send;
                fn stat<T: 'static>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> impl ::core::future::Future<
                    Output = Result<DescriptorStat, Errno>,
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
                let mut inst = linker.instance("foo:foo/wasi-filesystem")?;
                inst.func_wrap_concurrent(
                    "create-directory-at",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::create_directory_at(accessor)
                                .await;
                            Ok((r,))
                        })
                    },
                )?;
                inst.func_wrap_concurrent(
                    "stat",
                    move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                        wasmtime::component::__internal::Box::pin(async move {
                            let accessor = &caller.with_data(host_getter);
                            let r = <D as HostWithStore>::stat(accessor).await;
                            Ok((r,))
                        })
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod wall_clock {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct WallClock {}
            impl core::fmt::Debug for WallClock {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("WallClock").finish()
                }
            }
            const _: () = {
                assert!(
                    0 == < WallClock as wasmtime::component::ComponentType >::SIZE32
                );
                assert!(
                    1 == < WallClock as wasmtime::component::ComponentType >::ALIGN32
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
                let mut inst = linker.instance("foo:foo/wall-clock")?;
                Ok(())
            }
        }
    }
}

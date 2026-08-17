/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `imports`.
///
/// This structure is created through [`ImportsPre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`Imports`] as well.
pub struct ImportsPre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: ImportsIndices,
}
impl<T: 'static> Clone for ImportsPre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> ImportsPre<_T> {
    /// Creates a new copy of `ImportsPre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = ImportsIndices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`Imports`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Imports> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> ImportsPre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Imports> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `imports`.
///
/// This is an implementation detail of [`ImportsPre`] and can
/// be constructed if needed as well.
///
/// For more information see [`Imports`] as well.
#[derive(Clone)]
pub struct ImportsIndices {}
/// Auto-generated bindings for an instance a component which
/// implements the world `imports`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`Imports::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`ImportsPre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`ImportsPre::instantiate`] to
///   create a [`Imports`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`Imports::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct Imports {}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl ImportsIndices {
        /// Creates a new copy of `ImportsIndices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            Ok(ImportsIndices {})
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`Imports`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Imports> {
            let _ = &mut store;
            let _instance = instance;
            Ok(Imports {})
        }
    }
    impl Imports {
        /// Convenience wrapper around [`ImportsPre::new`] and
        /// [`ImportsPre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Imports> {
            let pre = linker.instantiate_pre(component)?;
            ImportsPre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`ImportsIndices::new`] and
        /// [`ImportsIndices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Imports> {
            let indices = ImportsIndices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`ImportsPre::new`] and
        /// [`ImportsPre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Imports>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            ImportsPre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: a::b::interface_with_live_type::HostWithStore
                + a::b::interface_with_dead_type::HostWithStore + Send,
            for<'a> D::Data<
                'a,
            >: a::b::interface_with_live_type::Host
                + a::b::interface_with_dead_type::Host + Send,
            T: 'static + Send,
        {
            a::b::interface_with_live_type::add_to_linker::<T, D>(linker, host_getter)?;
            a::b::interface_with_dead_type::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
    }
};
pub mod a {
    pub mod b {
        #[allow(clippy::all)]
        pub mod interface_with_live_type {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct LiveType {
                #[component(name = "a")]
                pub a: u32,
            }
            impl core::fmt::Debug for LiveType {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("LiveType").field("a", &self.a).finish()
                }
            }
            const _: () = {
                assert!(4 == < LiveType as wasmtime::component::ComponentType >::SIZE32);
                assert!(
                    4 == < LiveType as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            pub trait HostWithStore: wasmtime::component::HasData + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + Send,
            {}
            pub trait Host: Send {
                fn f(&mut self) -> impl ::core::future::Future<Output = LiveType> + Send;
            }
            impl<_T: Host + ?Sized + Send> Host for &mut _T {
                fn f(
                    &mut self,
                ) -> impl ::core::future::Future<Output = LiveType> + Send {
                    async move { Host::f(*self).await }
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
                let mut inst = linker.instance("a:b/interface-with-live-type")?;
                inst.func_wrap_async(
                    "f",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        wasmtime::component::__internal::Box::new(async move {
                            let host = &mut host_getter(caller.data_mut());
                            let r = Host::f(host).await;
                            Ok((r,))
                        })
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod interface_with_dead_type {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub type LiveType = super::super::super::a::b::interface_with_live_type::LiveType;
            const _: () = {
                assert!(4 == < LiveType as wasmtime::component::ComponentType >::SIZE32);
                assert!(
                    4 == < LiveType as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone, Copy)]
            pub struct DeadType {
                #[component(name = "a")]
                pub a: u32,
            }
            impl core::fmt::Debug for DeadType {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("DeadType").field("a", &self.a).finish()
                }
            }
            const _: () = {
                assert!(4 == < DeadType as wasmtime::component::ComponentType >::SIZE32);
                assert!(
                    4 == < DeadType as wasmtime::component::ComponentType >::ALIGN32
                );
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(variant)]
            #[derive(Clone, Copy)]
            pub enum V {
                #[component(name = "a")]
                A(LiveType),
                #[component(name = "b")]
                B(DeadType),
            }
            impl core::fmt::Debug for V {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        V::A(e) => f.debug_tuple("V::A").field(e).finish(),
                        V::B(e) => f.debug_tuple("V::B").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(8 == < V as wasmtime::component::ComponentType >::SIZE32);
                assert!(4 == < V as wasmtime::component::ComponentType >::ALIGN32);
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
                let mut inst = linker.instance("a:b/interface-with-dead-type")?;
                Ok(())
            }
        }
    }
}

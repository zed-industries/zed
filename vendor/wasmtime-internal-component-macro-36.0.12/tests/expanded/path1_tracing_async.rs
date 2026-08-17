/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `path1`.
///
/// This structure is created through [`Path1Pre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`Path1`] as well.
pub struct Path1Pre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: Path1Indices,
}
impl<T: 'static> Clone for Path1Pre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> Path1Pre<_T> {
    /// Creates a new copy of `Path1Pre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = Path1Indices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`Path1`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Path1> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> Path1Pre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Path1> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `path1`.
///
/// This is an implementation detail of [`Path1Pre`] and can
/// be constructed if needed as well.
///
/// For more information see [`Path1`] as well.
#[derive(Clone)]
pub struct Path1Indices {}
/// Auto-generated bindings for an instance a component which
/// implements the world `path1`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`Path1::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`Path1Pre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`Path1Pre::instantiate`] to
///   create a [`Path1`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`Path1::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct Path1 {}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl Path1Indices {
        /// Creates a new copy of `Path1Indices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            Ok(Path1Indices {})
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`Path1`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Path1> {
            let _ = &mut store;
            let _instance = instance;
            Ok(Path1 {})
        }
    }
    impl Path1 {
        /// Convenience wrapper around [`Path1Pre::new`] and
        /// [`Path1Pre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Path1> {
            let pre = linker.instantiate_pre(component)?;
            Path1Pre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`Path1Indices::new`] and
        /// [`Path1Indices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Path1> {
            let indices = Path1Indices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`Path1Pre::new`] and
        /// [`Path1Pre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Path1>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            Path1Pre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: paths::path1::test::HostWithStore,
            for<'a> D::Data<'a>: paths::path1::test::Host,
            T: 'static,
        {
            paths::path1::test::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
    }
};
pub mod paths {
    pub mod path1 {
        #[allow(clippy::all)]
        pub mod test {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
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
                let mut inst = linker.instance("paths:path1/test")?;
                Ok(())
            }
        }
    }
}

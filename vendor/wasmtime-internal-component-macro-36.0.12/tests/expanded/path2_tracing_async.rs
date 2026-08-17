/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `path2`.
///
/// This structure is created through [`Path2Pre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`Path2`] as well.
pub struct Path2Pre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: Path2Indices,
}
impl<T: 'static> Clone for Path2Pre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> Path2Pre<_T> {
    /// Creates a new copy of `Path2Pre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = Path2Indices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`Path2`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Path2> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> Path2Pre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Path2> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `path2`.
///
/// This is an implementation detail of [`Path2Pre`] and can
/// be constructed if needed as well.
///
/// For more information see [`Path2`] as well.
#[derive(Clone)]
pub struct Path2Indices {}
/// Auto-generated bindings for an instance a component which
/// implements the world `path2`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`Path2::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`Path2Pre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`Path2Pre::instantiate`] to
///   create a [`Path2`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`Path2::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct Path2 {}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl Path2Indices {
        /// Creates a new copy of `Path2Indices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            Ok(Path2Indices {})
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`Path2`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Path2> {
            let _ = &mut store;
            let _instance = instance;
            Ok(Path2 {})
        }
    }
    impl Path2 {
        /// Convenience wrapper around [`Path2Pre::new`] and
        /// [`Path2Pre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Path2> {
            let pre = linker.instantiate_pre(component)?;
            Path2Pre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`Path2Indices::new`] and
        /// [`Path2Indices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Path2> {
            let indices = Path2Indices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`Path2Pre::new`] and
        /// [`Path2Pre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Path2>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            Path2Pre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: paths::path2::test::HostWithStore,
            for<'a> D::Data<'a>: paths::path2::test::Host,
            T: 'static,
        {
            paths::path2::test::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
    }
};
pub mod paths {
    pub mod path2 {
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
                let mut inst = linker.instance("paths:path2/test")?;
                Ok(())
            }
        }
    }
}

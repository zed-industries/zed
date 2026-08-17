/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `example`.
///
/// This structure is created through [`ExamplePre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`Example`] as well.
pub struct ExamplePre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: ExampleIndices,
}
impl<T: 'static> Clone for ExamplePre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> ExamplePre<_T> {
    /// Creates a new copy of `ExamplePre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = ExampleIndices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`Example`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Example> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> ExamplePre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<Example> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `example`.
///
/// This is an implementation detail of [`ExamplePre`] and can
/// be constructed if needed as well.
///
/// For more information see [`Example`] as well.
#[derive(Clone)]
pub struct ExampleIndices {
    interface0: exports::same::name::this_name_is_duplicated::GuestIndices,
}
/// Auto-generated bindings for an instance a component which
/// implements the world `example`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`Example::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`ExamplePre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`ExamplePre::instantiate`] to
///   create a [`Example`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`Example::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct Example {
    interface0: exports::same::name::this_name_is_duplicated::Guest,
}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl ExampleIndices {
        /// Creates a new copy of `ExampleIndices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            let interface0 = exports::same::name::this_name_is_duplicated::GuestIndices::new(
                _instance_pre,
            )?;
            Ok(ExampleIndices { interface0 })
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`Example`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Example> {
            let _ = &mut store;
            let _instance = instance;
            let interface0 = self.interface0.load(&mut store, &_instance)?;
            Ok(Example { interface0 })
        }
    }
    impl Example {
        /// Convenience wrapper around [`ExamplePre::new`] and
        /// [`ExamplePre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Example> {
            let pre = linker.instantiate_pre(component)?;
            ExamplePre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`ExampleIndices::new`] and
        /// [`ExampleIndices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Example> {
            let indices = ExampleIndices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`ExamplePre::new`] and
        /// [`ExamplePre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<Example>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            ExamplePre::new(pre)?.instantiate_async(store).await
        }
        pub fn same_name_this_name_is_duplicated(
            &self,
        ) -> &exports::same::name::this_name_is_duplicated::Guest {
            &self.interface0
        }
    }
};
pub mod exports {
    pub mod same {
        pub mod name {
            #[allow(clippy::all)]
            pub mod this_name_is_duplicated {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                pub type ThisNameIsDuplicated = wasmtime::component::ResourceAny;
                pub struct GuestThisNameIsDuplicated<'a> {
                    funcs: &'a Guest,
                }
                pub struct Guest {}
                #[derive(Clone)]
                pub struct GuestIndices {}
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
                            .get_export_index(None, "same:name/this-name-is-duplicated")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `same:name/this-name-is-duplicated`"
                                )
                            })?;
                        let mut lookup = move |name| {
                            _instance_pre
                                .component()
                                .get_export_index(Some(&instance), name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `same:name/this-name-is-duplicated` does \
                not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        Ok(GuestIndices {})
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
                        Ok(Guest {})
                    }
                }
                impl Guest {}
            }
        }
    }
}

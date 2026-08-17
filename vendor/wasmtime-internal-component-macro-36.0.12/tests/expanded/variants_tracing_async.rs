/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `my-world`.
///
/// This structure is created through [`MyWorldPre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`MyWorld`] as well.
pub struct MyWorldPre<T: 'static> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: MyWorldIndices,
}
impl<T: 'static> Clone for MyWorldPre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T: 'static> MyWorldPre<_T> {
    /// Creates a new copy of `MyWorldPre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = MyWorldIndices::new(&instance_pre)?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`MyWorld`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub fn instantiate(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<MyWorld> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate(&mut store)?;
        self.indices.load(&mut store, &instance)
    }
}
impl<_T: Send + 'static> MyWorldPre<_T> {
    /// Same as [`Self::instantiate`], except with `async`.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<MyWorld> {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `my-world`.
///
/// This is an implementation detail of [`MyWorldPre`] and can
/// be constructed if needed as well.
///
/// For more information see [`MyWorld`] as well.
#[derive(Clone)]
pub struct MyWorldIndices {
    interface0: exports::foo::foo::variants::GuestIndices,
}
/// Auto-generated bindings for an instance a component which
/// implements the world `my-world`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`MyWorld::instantiate`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`MyWorldPre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`MyWorldPre::instantiate`] to
///   create a [`MyWorld`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`MyWorld::new`].
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct MyWorld {
    interface0: exports::foo::foo::variants::Guest,
}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl MyWorldIndices {
        /// Creates a new copy of `MyWorldIndices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new<_T>(
            _instance_pre: &wasmtime::component::InstancePre<_T>,
        ) -> wasmtime::Result<Self> {
            let _component = _instance_pre.component();
            let _instance_type = _instance_pre.instance_type();
            let interface0 = exports::foo::foo::variants::GuestIndices::new(
                _instance_pre,
            )?;
            Ok(MyWorldIndices { interface0 })
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`MyWorld`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<MyWorld> {
            let _ = &mut store;
            let _instance = instance;
            let interface0 = self.interface0.load(&mut store, &_instance)?;
            Ok(MyWorld { interface0 })
        }
    }
    impl MyWorld {
        /// Convenience wrapper around [`MyWorldPre::new`] and
        /// [`MyWorldPre::instantiate`].
        pub fn instantiate<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<MyWorld> {
            let pre = linker.instantiate_pre(component)?;
            MyWorldPre::new(pre)?.instantiate(store)
        }
        /// Convenience wrapper around [`MyWorldIndices::new`] and
        /// [`MyWorldIndices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<MyWorld> {
            let indices = MyWorldIndices::new(&instance.instance_pre(&store))?;
            indices.load(&mut store, instance)
        }
        /// Convenience wrapper around [`MyWorldPre::new`] and
        /// [`MyWorldPre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<MyWorld>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            MyWorldPre::new(pre)?.instantiate_async(store).await
        }
        pub fn add_to_linker<T, D>(
            linker: &mut wasmtime::component::Linker<T>,
            host_getter: fn(&mut T) -> D::Data<'_>,
        ) -> wasmtime::Result<()>
        where
            D: foo::foo::variants::HostWithStore + Send,
            for<'a> D::Data<'a>: foo::foo::variants::Host + Send,
            T: 'static + Send,
        {
            foo::foo::variants::add_to_linker::<T, D>(linker, host_getter)?;
            Ok(())
        }
        pub fn foo_foo_variants(&self) -> &exports::foo::foo::variants::Guest {
            &self.interface0
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod variants {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(enum)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            #[repr(u8)]
            pub enum E1 {
                #[component(name = "a")]
                A,
            }
            impl core::fmt::Debug for E1 {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        E1::A => f.debug_tuple("E1::A").finish(),
                    }
                }
            }
            const _: () = {
                assert!(1 == < E1 as wasmtime::component::ComponentType >::SIZE32);
                assert!(1 == < E1 as wasmtime::component::ComponentType >::ALIGN32);
            };
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
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(variant)]
            #[derive(Clone)]
            pub enum V1 {
                #[component(name = "a")]
                A,
                #[component(name = "c")]
                C(E1),
                #[component(name = "d")]
                D(wasmtime::component::__internal::String),
                #[component(name = "e")]
                E(Empty),
                #[component(name = "f")]
                F,
                #[component(name = "g")]
                G(u32),
            }
            impl core::fmt::Debug for V1 {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        V1::A => f.debug_tuple("V1::A").finish(),
                        V1::C(e) => f.debug_tuple("V1::C").field(e).finish(),
                        V1::D(e) => f.debug_tuple("V1::D").field(e).finish(),
                        V1::E(e) => f.debug_tuple("V1::E").field(e).finish(),
                        V1::F => f.debug_tuple("V1::F").finish(),
                        V1::G(e) => f.debug_tuple("V1::G").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(12 == < V1 as wasmtime::component::ComponentType >::SIZE32);
                assert!(4 == < V1 as wasmtime::component::ComponentType >::ALIGN32);
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(variant)]
            #[derive(Clone, Copy)]
            pub enum Casts1 {
                #[component(name = "a")]
                A(i32),
                #[component(name = "b")]
                B(f32),
            }
            impl core::fmt::Debug for Casts1 {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Casts1::A(e) => f.debug_tuple("Casts1::A").field(e).finish(),
                        Casts1::B(e) => f.debug_tuple("Casts1::B").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(8 == < Casts1 as wasmtime::component::ComponentType >::SIZE32);
                assert!(4 == < Casts1 as wasmtime::component::ComponentType >::ALIGN32);
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(variant)]
            #[derive(Clone, Copy)]
            pub enum Casts2 {
                #[component(name = "a")]
                A(f64),
                #[component(name = "b")]
                B(f32),
            }
            impl core::fmt::Debug for Casts2 {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Casts2::A(e) => f.debug_tuple("Casts2::A").field(e).finish(),
                        Casts2::B(e) => f.debug_tuple("Casts2::B").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(16 == < Casts2 as wasmtime::component::ComponentType >::SIZE32);
                assert!(8 == < Casts2 as wasmtime::component::ComponentType >::ALIGN32);
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(variant)]
            #[derive(Clone, Copy)]
            pub enum Casts3 {
                #[component(name = "a")]
                A(f64),
                #[component(name = "b")]
                B(u64),
            }
            impl core::fmt::Debug for Casts3 {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Casts3::A(e) => f.debug_tuple("Casts3::A").field(e).finish(),
                        Casts3::B(e) => f.debug_tuple("Casts3::B").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(16 == < Casts3 as wasmtime::component::ComponentType >::SIZE32);
                assert!(8 == < Casts3 as wasmtime::component::ComponentType >::ALIGN32);
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(variant)]
            #[derive(Clone, Copy)]
            pub enum Casts4 {
                #[component(name = "a")]
                A(u32),
                #[component(name = "b")]
                B(i64),
            }
            impl core::fmt::Debug for Casts4 {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Casts4::A(e) => f.debug_tuple("Casts4::A").field(e).finish(),
                        Casts4::B(e) => f.debug_tuple("Casts4::B").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(16 == < Casts4 as wasmtime::component::ComponentType >::SIZE32);
                assert!(8 == < Casts4 as wasmtime::component::ComponentType >::ALIGN32);
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(variant)]
            #[derive(Clone, Copy)]
            pub enum Casts5 {
                #[component(name = "a")]
                A(f32),
                #[component(name = "b")]
                B(i64),
            }
            impl core::fmt::Debug for Casts5 {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Casts5::A(e) => f.debug_tuple("Casts5::A").field(e).finish(),
                        Casts5::B(e) => f.debug_tuple("Casts5::B").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(16 == < Casts5 as wasmtime::component::ComponentType >::SIZE32);
                assert!(8 == < Casts5 as wasmtime::component::ComponentType >::ALIGN32);
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(variant)]
            #[derive(Clone, Copy)]
            pub enum Casts6 {
                #[component(name = "a")]
                A((f32, u32)),
                #[component(name = "b")]
                B((u32, u32)),
            }
            impl core::fmt::Debug for Casts6 {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Casts6::A(e) => f.debug_tuple("Casts6::A").field(e).finish(),
                        Casts6::B(e) => f.debug_tuple("Casts6::B").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(12 == < Casts6 as wasmtime::component::ComponentType >::SIZE32);
                assert!(4 == < Casts6 as wasmtime::component::ComponentType >::ALIGN32);
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(enum)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            #[repr(u8)]
            pub enum MyErrno {
                #[component(name = "bad1")]
                Bad1,
                #[component(name = "bad2")]
                Bad2,
            }
            impl MyErrno {
                pub fn name(&self) -> &'static str {
                    match self {
                        MyErrno::Bad1 => "bad1",
                        MyErrno::Bad2 => "bad2",
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        MyErrno::Bad1 => "",
                        MyErrno::Bad2 => "",
                    }
                }
            }
            impl core::fmt::Debug for MyErrno {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("MyErrno")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl core::fmt::Display for MyErrno {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{} (error {})", self.name(), * self as i32)
                }
            }
            impl core::error::Error for MyErrno {}
            const _: () = {
                assert!(1 == < MyErrno as wasmtime::component::ComponentType >::SIZE32);
                assert!(1 == < MyErrno as wasmtime::component::ComponentType >::ALIGN32);
            };
            #[derive(wasmtime::component::ComponentType)]
            #[derive(wasmtime::component::Lift)]
            #[derive(wasmtime::component::Lower)]
            #[component(record)]
            #[derive(Clone)]
            pub struct IsClone {
                #[component(name = "v1")]
                pub v1: V1,
            }
            impl core::fmt::Debug for IsClone {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("IsClone").field("v1", &self.v1).finish()
                }
            }
            const _: () = {
                assert!(12 == < IsClone as wasmtime::component::ComponentType >::SIZE32);
                assert!(4 == < IsClone as wasmtime::component::ComponentType >::ALIGN32);
            };
            pub trait HostWithStore: wasmtime::component::HasData + Send {}
            impl<_T: ?Sized> HostWithStore for _T
            where
                _T: wasmtime::component::HasData + Send,
            {}
            pub trait Host: Send {
                fn e1_arg(
                    &mut self,
                    x: E1,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn e1_result(
                    &mut self,
                ) -> impl ::core::future::Future<Output = E1> + Send;
                fn v1_arg(
                    &mut self,
                    x: V1,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn v1_result(
                    &mut self,
                ) -> impl ::core::future::Future<Output = V1> + Send;
                fn bool_arg(
                    &mut self,
                    x: bool,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn bool_result(
                    &mut self,
                ) -> impl ::core::future::Future<Output = bool> + Send;
                fn option_arg(
                    &mut self,
                    a: Option<bool>,
                    b: Option<()>,
                    c: Option<u32>,
                    d: Option<E1>,
                    e: Option<f32>,
                    g: Option<Option<bool>>,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn option_result(
                    &mut self,
                ) -> impl ::core::future::Future<
                    Output = (
                        Option<bool>,
                        Option<()>,
                        Option<u32>,
                        Option<E1>,
                        Option<f32>,
                        Option<Option<bool>>,
                    ),
                > + Send;
                fn casts(
                    &mut self,
                    a: Casts1,
                    b: Casts2,
                    c: Casts3,
                    d: Casts4,
                    e: Casts5,
                    f: Casts6,
                ) -> impl ::core::future::Future<
                    Output = (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6),
                > + Send;
                fn result_arg(
                    &mut self,
                    a: Result<(), ()>,
                    b: Result<(), E1>,
                    c: Result<E1, ()>,
                    d: Result<(), ()>,
                    e: Result<u32, V1>,
                    f: Result<
                        wasmtime::component::__internal::String,
                        wasmtime::component::__internal::Vec<u8>,
                    >,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn result_result(
                    &mut self,
                ) -> impl ::core::future::Future<
                    Output = (
                        Result<(), ()>,
                        Result<(), E1>,
                        Result<E1, ()>,
                        Result<(), ()>,
                        Result<u32, V1>,
                        Result<
                            wasmtime::component::__internal::String,
                            wasmtime::component::__internal::Vec<u8>,
                        >,
                    ),
                > + Send;
                fn return_result_sugar(
                    &mut self,
                ) -> impl ::core::future::Future<Output = Result<i32, MyErrno>> + Send;
                fn return_result_sugar2(
                    &mut self,
                ) -> impl ::core::future::Future<Output = Result<(), MyErrno>> + Send;
                fn return_result_sugar3(
                    &mut self,
                ) -> impl ::core::future::Future<
                    Output = Result<MyErrno, MyErrno>,
                > + Send;
                fn return_result_sugar4(
                    &mut self,
                ) -> impl ::core::future::Future<
                    Output = Result<(i32, u32), MyErrno>,
                > + Send;
                fn return_option_sugar(
                    &mut self,
                ) -> impl ::core::future::Future<Output = Option<i32>> + Send;
                fn return_option_sugar2(
                    &mut self,
                ) -> impl ::core::future::Future<Output = Option<MyErrno>> + Send;
                fn result_simple(
                    &mut self,
                ) -> impl ::core::future::Future<Output = Result<u32, i32>> + Send;
                fn is_clone_arg(
                    &mut self,
                    a: IsClone,
                ) -> impl ::core::future::Future<Output = ()> + Send;
                fn is_clone_return(
                    &mut self,
                ) -> impl ::core::future::Future<Output = IsClone> + Send;
            }
            impl<_T: Host + ?Sized + Send> Host for &mut _T {
                fn e1_arg(
                    &mut self,
                    x: E1,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::e1_arg(*self, x).await }
                }
                fn e1_result(
                    &mut self,
                ) -> impl ::core::future::Future<Output = E1> + Send {
                    async move { Host::e1_result(*self).await }
                }
                fn v1_arg(
                    &mut self,
                    x: V1,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::v1_arg(*self, x).await }
                }
                fn v1_result(
                    &mut self,
                ) -> impl ::core::future::Future<Output = V1> + Send {
                    async move { Host::v1_result(*self).await }
                }
                fn bool_arg(
                    &mut self,
                    x: bool,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::bool_arg(*self, x).await }
                }
                fn bool_result(
                    &mut self,
                ) -> impl ::core::future::Future<Output = bool> + Send {
                    async move { Host::bool_result(*self).await }
                }
                fn option_arg(
                    &mut self,
                    a: Option<bool>,
                    b: Option<()>,
                    c: Option<u32>,
                    d: Option<E1>,
                    e: Option<f32>,
                    g: Option<Option<bool>>,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::option_arg(*self, a, b, c, d, e, g).await }
                }
                fn option_result(
                    &mut self,
                ) -> impl ::core::future::Future<
                    Output = (
                        Option<bool>,
                        Option<()>,
                        Option<u32>,
                        Option<E1>,
                        Option<f32>,
                        Option<Option<bool>>,
                    ),
                > + Send {
                    async move { Host::option_result(*self).await }
                }
                fn casts(
                    &mut self,
                    a: Casts1,
                    b: Casts2,
                    c: Casts3,
                    d: Casts4,
                    e: Casts5,
                    f: Casts6,
                ) -> impl ::core::future::Future<
                    Output = (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6),
                > + Send {
                    async move { Host::casts(*self, a, b, c, d, e, f).await }
                }
                fn result_arg(
                    &mut self,
                    a: Result<(), ()>,
                    b: Result<(), E1>,
                    c: Result<E1, ()>,
                    d: Result<(), ()>,
                    e: Result<u32, V1>,
                    f: Result<
                        wasmtime::component::__internal::String,
                        wasmtime::component::__internal::Vec<u8>,
                    >,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::result_arg(*self, a, b, c, d, e, f).await }
                }
                fn result_result(
                    &mut self,
                ) -> impl ::core::future::Future<
                    Output = (
                        Result<(), ()>,
                        Result<(), E1>,
                        Result<E1, ()>,
                        Result<(), ()>,
                        Result<u32, V1>,
                        Result<
                            wasmtime::component::__internal::String,
                            wasmtime::component::__internal::Vec<u8>,
                        >,
                    ),
                > + Send {
                    async move { Host::result_result(*self).await }
                }
                fn return_result_sugar(
                    &mut self,
                ) -> impl ::core::future::Future<Output = Result<i32, MyErrno>> + Send {
                    async move { Host::return_result_sugar(*self).await }
                }
                fn return_result_sugar2(
                    &mut self,
                ) -> impl ::core::future::Future<Output = Result<(), MyErrno>> + Send {
                    async move { Host::return_result_sugar2(*self).await }
                }
                fn return_result_sugar3(
                    &mut self,
                ) -> impl ::core::future::Future<
                    Output = Result<MyErrno, MyErrno>,
                > + Send {
                    async move { Host::return_result_sugar3(*self).await }
                }
                fn return_result_sugar4(
                    &mut self,
                ) -> impl ::core::future::Future<
                    Output = Result<(i32, u32), MyErrno>,
                > + Send {
                    async move { Host::return_result_sugar4(*self).await }
                }
                fn return_option_sugar(
                    &mut self,
                ) -> impl ::core::future::Future<Output = Option<i32>> + Send {
                    async move { Host::return_option_sugar(*self).await }
                }
                fn return_option_sugar2(
                    &mut self,
                ) -> impl ::core::future::Future<Output = Option<MyErrno>> + Send {
                    async move { Host::return_option_sugar2(*self).await }
                }
                fn result_simple(
                    &mut self,
                ) -> impl ::core::future::Future<Output = Result<u32, i32>> + Send {
                    async move { Host::result_simple(*self).await }
                }
                fn is_clone_arg(
                    &mut self,
                    a: IsClone,
                ) -> impl ::core::future::Future<Output = ()> + Send {
                    async move { Host::is_clone_arg(*self, a).await }
                }
                fn is_clone_return(
                    &mut self,
                ) -> impl ::core::future::Future<Output = IsClone> + Send {
                    async move { Host::is_clone_return(*self).await }
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
                let mut inst = linker.instance("foo:foo/variants")?;
                inst.func_wrap_async(
                    "e1-arg",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (E1,)| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "e1-arg",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(
                                    tracing::Level::TRACE, x = tracing::field::debug(& arg0),
                                    "call"
                                );
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::e1_arg(host, arg0).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok(r)
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "e1-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "e1-result",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::e1_result(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "v1-arg",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (V1,)| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "v1-arg",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(
                                    tracing::Level::TRACE, x = tracing::field::debug(& arg0),
                                    "call"
                                );
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::v1_arg(host, arg0).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok(r)
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "v1-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "v1-result",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::v1_result(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "bool-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (bool,)|
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "bool-arg",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(
                                    tracing::Level::TRACE, x = tracing::field::debug(& arg0),
                                    "call"
                                );
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::bool_arg(host, arg0).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok(r)
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "bool-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "bool-result",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::bool_result(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "option-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                        ): (
                            Option<bool>,
                            Option<()>,
                            Option<u32>,
                            Option<E1>,
                            Option<f32>,
                            Option<Option<bool>>,
                        )|
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "option-arg",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(
                                    tracing::Level::TRACE, a = tracing::field::debug(& arg0), b
                                    = tracing::field::debug(& arg1), c = tracing::field::debug(&
                                    arg2), d = tracing::field::debug(& arg3), e =
                                    tracing::field::debug(& arg4), g = tracing::field::debug(&
                                    arg5), "call"
                                );
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::option_arg(
                                        host,
                                        arg0,
                                        arg1,
                                        arg2,
                                        arg3,
                                        arg4,
                                        arg5,
                                    )
                                    .await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok(r)
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "option-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "option-result",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::option_result(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "casts",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                        ): (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6)|
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "casts",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(
                                    tracing::Level::TRACE, a = tracing::field::debug(& arg0), b
                                    = tracing::field::debug(& arg1), c = tracing::field::debug(&
                                    arg2), d = tracing::field::debug(& arg3), e =
                                    tracing::field::debug(& arg4), f = tracing::field::debug(&
                                    arg5), "call"
                                );
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::casts(
                                        host,
                                        arg0,
                                        arg1,
                                        arg2,
                                        arg3,
                                        arg4,
                                        arg5,
                                    )
                                    .await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "result-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                            arg2,
                            arg3,
                            arg4,
                            arg5,
                        ): (
                            Result<(), ()>,
                            Result<(), E1>,
                            Result<E1, ()>,
                            Result<(), ()>,
                            Result<u32, V1>,
                            Result<
                                wasmtime::component::__internal::String,
                                wasmtime::component::__internal::Vec<u8>,
                            >,
                        )|
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "result-arg",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(
                                    tracing::Level::TRACE, a = tracing::field::debug(& arg0), b
                                    = tracing::field::debug(& arg1), c = tracing::field::debug(&
                                    arg2), d = tracing::field::debug(& arg3), e =
                                    tracing::field::debug(& arg4), f =
                                    tracing::field::debug("..."), "call"
                                );
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::result_arg(
                                        host,
                                        arg0,
                                        arg1,
                                        arg2,
                                        arg3,
                                        arg4,
                                        arg5,
                                    )
                                    .await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok(r)
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "result-result",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "result-result",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::result_result(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result =
                                    tracing::field::debug("..."), "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "return-result-sugar",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "return-result-sugar",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::return_result_sugar(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "return-result-sugar2",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "return-result-sugar2",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::return_result_sugar2(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "return-result-sugar3",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "return-result-sugar3",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::return_result_sugar3(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "return-result-sugar4",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "return-result-sugar4",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::return_result_sugar4(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "return-option-sugar",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "return-option-sugar",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::return_option_sugar(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "return-option-sugar2",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "return-option-sugar2",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::return_option_sugar2(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "result-simple",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "result-simple",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::result_simple(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "is-clone-arg",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (IsClone,)|
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "is-clone-arg",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(
                                    tracing::Level::TRACE, a = tracing::field::debug(& arg0),
                                    "call"
                                );
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::is_clone_arg(host, arg0).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok(r)
                            }
                                .instrument(span),
                        )
                    },
                )?;
                inst.func_wrap_async(
                    "is-clone-return",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen import", module =
                            "variants", function = "is-clone-return",
                        );
                        wasmtime::component::__internal::Box::new(
                            async move {
                                tracing::event!(tracing::Level::TRACE, "call");
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::is_clone_return(host).await;
                                tracing::event!(
                                    tracing::Level::TRACE, result = tracing::field::debug(& r),
                                    "return"
                                );
                                Ok((r,))
                            }
                                .instrument(span),
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
            pub mod variants {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(enum)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                #[repr(u8)]
                pub enum E1 {
                    #[component(name = "a")]
                    A,
                }
                impl core::fmt::Debug for E1 {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            E1::A => f.debug_tuple("E1::A").finish(),
                        }
                    }
                }
                const _: () = {
                    assert!(1 == < E1 as wasmtime::component::ComponentType >::SIZE32);
                    assert!(1 == < E1 as wasmtime::component::ComponentType >::ALIGN32);
                };
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
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(variant)]
                #[derive(Clone)]
                pub enum V1 {
                    #[component(name = "a")]
                    A,
                    #[component(name = "c")]
                    C(E1),
                    #[component(name = "d")]
                    D(wasmtime::component::__internal::String),
                    #[component(name = "e")]
                    E(Empty),
                    #[component(name = "f")]
                    F,
                    #[component(name = "g")]
                    G(u32),
                }
                impl core::fmt::Debug for V1 {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            V1::A => f.debug_tuple("V1::A").finish(),
                            V1::C(e) => f.debug_tuple("V1::C").field(e).finish(),
                            V1::D(e) => f.debug_tuple("V1::D").field(e).finish(),
                            V1::E(e) => f.debug_tuple("V1::E").field(e).finish(),
                            V1::F => f.debug_tuple("V1::F").finish(),
                            V1::G(e) => f.debug_tuple("V1::G").field(e).finish(),
                        }
                    }
                }
                const _: () = {
                    assert!(12 == < V1 as wasmtime::component::ComponentType >::SIZE32);
                    assert!(4 == < V1 as wasmtime::component::ComponentType >::ALIGN32);
                };
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(variant)]
                #[derive(Clone, Copy)]
                pub enum Casts1 {
                    #[component(name = "a")]
                    A(i32),
                    #[component(name = "b")]
                    B(f32),
                }
                impl core::fmt::Debug for Casts1 {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            Casts1::A(e) => f.debug_tuple("Casts1::A").field(e).finish(),
                            Casts1::B(e) => f.debug_tuple("Casts1::B").field(e).finish(),
                        }
                    }
                }
                const _: () = {
                    assert!(
                        8 == < Casts1 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        4 == < Casts1 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(variant)]
                #[derive(Clone, Copy)]
                pub enum Casts2 {
                    #[component(name = "a")]
                    A(f64),
                    #[component(name = "b")]
                    B(f32),
                }
                impl core::fmt::Debug for Casts2 {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            Casts2::A(e) => f.debug_tuple("Casts2::A").field(e).finish(),
                            Casts2::B(e) => f.debug_tuple("Casts2::B").field(e).finish(),
                        }
                    }
                }
                const _: () = {
                    assert!(
                        16 == < Casts2 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        8 == < Casts2 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(variant)]
                #[derive(Clone, Copy)]
                pub enum Casts3 {
                    #[component(name = "a")]
                    A(f64),
                    #[component(name = "b")]
                    B(u64),
                }
                impl core::fmt::Debug for Casts3 {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            Casts3::A(e) => f.debug_tuple("Casts3::A").field(e).finish(),
                            Casts3::B(e) => f.debug_tuple("Casts3::B").field(e).finish(),
                        }
                    }
                }
                const _: () = {
                    assert!(
                        16 == < Casts3 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        8 == < Casts3 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(variant)]
                #[derive(Clone, Copy)]
                pub enum Casts4 {
                    #[component(name = "a")]
                    A(u32),
                    #[component(name = "b")]
                    B(i64),
                }
                impl core::fmt::Debug for Casts4 {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            Casts4::A(e) => f.debug_tuple("Casts4::A").field(e).finish(),
                            Casts4::B(e) => f.debug_tuple("Casts4::B").field(e).finish(),
                        }
                    }
                }
                const _: () = {
                    assert!(
                        16 == < Casts4 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        8 == < Casts4 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(variant)]
                #[derive(Clone, Copy)]
                pub enum Casts5 {
                    #[component(name = "a")]
                    A(f32),
                    #[component(name = "b")]
                    B(i64),
                }
                impl core::fmt::Debug for Casts5 {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            Casts5::A(e) => f.debug_tuple("Casts5::A").field(e).finish(),
                            Casts5::B(e) => f.debug_tuple("Casts5::B").field(e).finish(),
                        }
                    }
                }
                const _: () = {
                    assert!(
                        16 == < Casts5 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        8 == < Casts5 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(variant)]
                #[derive(Clone, Copy)]
                pub enum Casts6 {
                    #[component(name = "a")]
                    A((f32, u32)),
                    #[component(name = "b")]
                    B((u32, u32)),
                }
                impl core::fmt::Debug for Casts6 {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            Casts6::A(e) => f.debug_tuple("Casts6::A").field(e).finish(),
                            Casts6::B(e) => f.debug_tuple("Casts6::B").field(e).finish(),
                        }
                    }
                }
                const _: () = {
                    assert!(
                        12 == < Casts6 as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        4 == < Casts6 as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(enum)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                #[repr(u8)]
                pub enum MyErrno {
                    #[component(name = "bad1")]
                    Bad1,
                    #[component(name = "bad2")]
                    Bad2,
                }
                impl MyErrno {
                    pub fn name(&self) -> &'static str {
                        match self {
                            MyErrno::Bad1 => "bad1",
                            MyErrno::Bad2 => "bad2",
                        }
                    }
                    pub fn message(&self) -> &'static str {
                        match self {
                            MyErrno::Bad1 => "",
                            MyErrno::Bad2 => "",
                        }
                    }
                }
                impl core::fmt::Debug for MyErrno {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("MyErrno")
                            .field("code", &(*self as i32))
                            .field("name", &self.name())
                            .field("message", &self.message())
                            .finish()
                    }
                }
                impl core::fmt::Display for MyErrno {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(f, "{} (error {})", self.name(), * self as i32)
                    }
                }
                impl core::error::Error for MyErrno {}
                const _: () = {
                    assert!(
                        1 == < MyErrno as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        1 == < MyErrno as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                #[derive(wasmtime::component::ComponentType)]
                #[derive(wasmtime::component::Lift)]
                #[derive(wasmtime::component::Lower)]
                #[component(record)]
                #[derive(Clone)]
                pub struct IsClone {
                    #[component(name = "v1")]
                    pub v1: V1,
                }
                impl core::fmt::Debug for IsClone {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        f.debug_struct("IsClone").field("v1", &self.v1).finish()
                    }
                }
                const _: () = {
                    assert!(
                        12 == < IsClone as wasmtime::component::ComponentType >::SIZE32
                    );
                    assert!(
                        4 == < IsClone as wasmtime::component::ComponentType >::ALIGN32
                    );
                };
                pub struct Guest {
                    e1_arg: wasmtime::component::Func,
                    e1_result: wasmtime::component::Func,
                    v1_arg: wasmtime::component::Func,
                    v1_result: wasmtime::component::Func,
                    bool_arg: wasmtime::component::Func,
                    bool_result: wasmtime::component::Func,
                    option_arg: wasmtime::component::Func,
                    option_result: wasmtime::component::Func,
                    casts: wasmtime::component::Func,
                    result_arg: wasmtime::component::Func,
                    result_result: wasmtime::component::Func,
                    return_result_sugar: wasmtime::component::Func,
                    return_result_sugar2: wasmtime::component::Func,
                    return_result_sugar3: wasmtime::component::Func,
                    return_result_sugar4: wasmtime::component::Func,
                    return_option_sugar: wasmtime::component::Func,
                    return_option_sugar2: wasmtime::component::Func,
                    result_simple: wasmtime::component::Func,
                    is_clone_arg: wasmtime::component::Func,
                    is_clone_return: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    e1_arg: wasmtime::component::ComponentExportIndex,
                    e1_result: wasmtime::component::ComponentExportIndex,
                    v1_arg: wasmtime::component::ComponentExportIndex,
                    v1_result: wasmtime::component::ComponentExportIndex,
                    bool_arg: wasmtime::component::ComponentExportIndex,
                    bool_result: wasmtime::component::ComponentExportIndex,
                    option_arg: wasmtime::component::ComponentExportIndex,
                    option_result: wasmtime::component::ComponentExportIndex,
                    casts: wasmtime::component::ComponentExportIndex,
                    result_arg: wasmtime::component::ComponentExportIndex,
                    result_result: wasmtime::component::ComponentExportIndex,
                    return_result_sugar: wasmtime::component::ComponentExportIndex,
                    return_result_sugar2: wasmtime::component::ComponentExportIndex,
                    return_result_sugar3: wasmtime::component::ComponentExportIndex,
                    return_result_sugar4: wasmtime::component::ComponentExportIndex,
                    return_option_sugar: wasmtime::component::ComponentExportIndex,
                    return_option_sugar2: wasmtime::component::ComponentExportIndex,
                    result_simple: wasmtime::component::ComponentExportIndex,
                    is_clone_arg: wasmtime::component::ComponentExportIndex,
                    is_clone_return: wasmtime::component::ComponentExportIndex,
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
                            .get_export_index(None, "foo:foo/variants")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/variants`"
                                )
                            })?;
                        let mut lookup = move |name| {
                            _instance_pre
                                .component()
                                .get_export_index(Some(&instance), name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `foo:foo/variants` does \
                                                            not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let e1_arg = lookup("e1-arg")?;
                        let e1_result = lookup("e1-result")?;
                        let v1_arg = lookup("v1-arg")?;
                        let v1_result = lookup("v1-result")?;
                        let bool_arg = lookup("bool-arg")?;
                        let bool_result = lookup("bool-result")?;
                        let option_arg = lookup("option-arg")?;
                        let option_result = lookup("option-result")?;
                        let casts = lookup("casts")?;
                        let result_arg = lookup("result-arg")?;
                        let result_result = lookup("result-result")?;
                        let return_result_sugar = lookup("return-result-sugar")?;
                        let return_result_sugar2 = lookup("return-result-sugar2")?;
                        let return_result_sugar3 = lookup("return-result-sugar3")?;
                        let return_result_sugar4 = lookup("return-result-sugar4")?;
                        let return_option_sugar = lookup("return-option-sugar")?;
                        let return_option_sugar2 = lookup("return-option-sugar2")?;
                        let result_simple = lookup("result-simple")?;
                        let is_clone_arg = lookup("is-clone-arg")?;
                        let is_clone_return = lookup("is-clone-return")?;
                        Ok(GuestIndices {
                            e1_arg,
                            e1_result,
                            v1_arg,
                            v1_result,
                            bool_arg,
                            bool_result,
                            option_arg,
                            option_result,
                            casts,
                            result_arg,
                            result_result,
                            return_result_sugar,
                            return_result_sugar2,
                            return_result_sugar3,
                            return_result_sugar4,
                            return_option_sugar,
                            return_option_sugar2,
                            result_simple,
                            is_clone_arg,
                            is_clone_return,
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
                        let e1_arg = *_instance
                            .get_typed_func::<(E1,), ()>(&mut store, &self.e1_arg)?
                            .func();
                        let e1_result = *_instance
                            .get_typed_func::<(), (E1,)>(&mut store, &self.e1_result)?
                            .func();
                        let v1_arg = *_instance
                            .get_typed_func::<(&V1,), ()>(&mut store, &self.v1_arg)?
                            .func();
                        let v1_result = *_instance
                            .get_typed_func::<(), (V1,)>(&mut store, &self.v1_result)?
                            .func();
                        let bool_arg = *_instance
                            .get_typed_func::<(bool,), ()>(&mut store, &self.bool_arg)?
                            .func();
                        let bool_result = *_instance
                            .get_typed_func::<
                                (),
                                (bool,),
                            >(&mut store, &self.bool_result)?
                            .func();
                        let option_arg = *_instance
                            .get_typed_func::<
                                (
                                    Option<bool>,
                                    Option<()>,
                                    Option<u32>,
                                    Option<E1>,
                                    Option<f32>,
                                    Option<Option<bool>>,
                                ),
                                (),
                            >(&mut store, &self.option_arg)?
                            .func();
                        let option_result = *_instance
                            .get_typed_func::<
                                (),
                                (
                                    (
                                        Option<bool>,
                                        Option<()>,
                                        Option<u32>,
                                        Option<E1>,
                                        Option<f32>,
                                        Option<Option<bool>>,
                                    ),
                                ),
                            >(&mut store, &self.option_result)?
                            .func();
                        let casts = *_instance
                            .get_typed_func::<
                                (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6),
                                ((Casts1, Casts2, Casts3, Casts4, Casts5, Casts6),),
                            >(&mut store, &self.casts)?
                            .func();
                        let result_arg = *_instance
                            .get_typed_func::<
                                (
                                    Result<(), ()>,
                                    Result<(), E1>,
                                    Result<E1, ()>,
                                    Result<(), ()>,
                                    Result<u32, &V1>,
                                    Result<&str, &[u8]>,
                                ),
                                (),
                            >(&mut store, &self.result_arg)?
                            .func();
                        let result_result = *_instance
                            .get_typed_func::<
                                (),
                                (
                                    (
                                        Result<(), ()>,
                                        Result<(), E1>,
                                        Result<E1, ()>,
                                        Result<(), ()>,
                                        Result<u32, V1>,
                                        Result<
                                            wasmtime::component::__internal::String,
                                            wasmtime::component::__internal::Vec<u8>,
                                        >,
                                    ),
                                ),
                            >(&mut store, &self.result_result)?
                            .func();
                        let return_result_sugar = *_instance
                            .get_typed_func::<
                                (),
                                (Result<i32, MyErrno>,),
                            >(&mut store, &self.return_result_sugar)?
                            .func();
                        let return_result_sugar2 = *_instance
                            .get_typed_func::<
                                (),
                                (Result<(), MyErrno>,),
                            >(&mut store, &self.return_result_sugar2)?
                            .func();
                        let return_result_sugar3 = *_instance
                            .get_typed_func::<
                                (),
                                (Result<MyErrno, MyErrno>,),
                            >(&mut store, &self.return_result_sugar3)?
                            .func();
                        let return_result_sugar4 = *_instance
                            .get_typed_func::<
                                (),
                                (Result<(i32, u32), MyErrno>,),
                            >(&mut store, &self.return_result_sugar4)?
                            .func();
                        let return_option_sugar = *_instance
                            .get_typed_func::<
                                (),
                                (Option<i32>,),
                            >(&mut store, &self.return_option_sugar)?
                            .func();
                        let return_option_sugar2 = *_instance
                            .get_typed_func::<
                                (),
                                (Option<MyErrno>,),
                            >(&mut store, &self.return_option_sugar2)?
                            .func();
                        let result_simple = *_instance
                            .get_typed_func::<
                                (),
                                (Result<u32, i32>,),
                            >(&mut store, &self.result_simple)?
                            .func();
                        let is_clone_arg = *_instance
                            .get_typed_func::<
                                (&IsClone,),
                                (),
                            >(&mut store, &self.is_clone_arg)?
                            .func();
                        let is_clone_return = *_instance
                            .get_typed_func::<
                                (),
                                (IsClone,),
                            >(&mut store, &self.is_clone_return)?
                            .func();
                        Ok(Guest {
                            e1_arg,
                            e1_result,
                            v1_arg,
                            v1_result,
                            bool_arg,
                            bool_result,
                            option_arg,
                            option_result,
                            casts,
                            result_arg,
                            result_result,
                            return_result_sugar,
                            return_result_sugar2,
                            return_result_sugar3,
                            return_result_sugar4,
                            return_option_sugar,
                            return_option_sugar2,
                            result_simple,
                            is_clone_arg,
                            is_clone_return,
                        })
                    }
                }
                impl Guest {
                    pub async fn call_e1_arg<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: E1,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "e1-arg",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (E1,),
                                (),
                            >::new_unchecked(self.e1_arg)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(())
                    }
                    pub async fn call_e1_result<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<E1>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "e1-result",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (E1,),
                            >::new_unchecked(self.e1_result)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_v1_arg<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: &V1,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "v1-arg",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (&V1,),
                                (),
                            >::new_unchecked(self.v1_arg)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(())
                    }
                    pub async fn call_v1_result<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<V1>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "v1-result",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (V1,),
                            >::new_unchecked(self.v1_result)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_bool_arg<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: bool,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "bool-arg",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (bool,),
                                (),
                            >::new_unchecked(self.bool_arg)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(())
                    }
                    pub async fn call_bool_result<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<bool>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "bool-result",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (bool,),
                            >::new_unchecked(self.bool_result)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_option_arg<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: Option<bool>,
                        arg1: Option<()>,
                        arg2: Option<u32>,
                        arg3: Option<E1>,
                        arg4: Option<f32>,
                        arg5: Option<Option<bool>>,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "option-arg",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (
                                    Option<bool>,
                                    Option<()>,
                                    Option<u32>,
                                    Option<E1>,
                                    Option<f32>,
                                    Option<Option<bool>>,
                                ),
                                (),
                            >::new_unchecked(self.option_arg)
                        };
                        let () = callee
                            .call_async(
                                store.as_context_mut(),
                                (arg0, arg1, arg2, arg3, arg4, arg5),
                            )
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(())
                    }
                    pub async fn call_option_result<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<
                        (
                            Option<bool>,
                            Option<()>,
                            Option<u32>,
                            Option<E1>,
                            Option<f32>,
                            Option<Option<bool>>,
                        ),
                    >
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "option-result",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (
                                    (
                                        Option<bool>,
                                        Option<()>,
                                        Option<u32>,
                                        Option<E1>,
                                        Option<f32>,
                                        Option<Option<bool>>,
                                    ),
                                ),
                            >::new_unchecked(self.option_result)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_casts<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: Casts1,
                        arg1: Casts2,
                        arg2: Casts3,
                        arg3: Casts4,
                        arg4: Casts5,
                        arg5: Casts6,
                    ) -> wasmtime::Result<
                        (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6),
                    >
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "casts",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6),
                                ((Casts1, Casts2, Casts3, Casts4, Casts5, Casts6),),
                            >::new_unchecked(self.casts)
                        };
                        let (ret0,) = callee
                            .call_async(
                                store.as_context_mut(),
                                (arg0, arg1, arg2, arg3, arg4, arg5),
                            )
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_result_arg<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: Result<(), ()>,
                        arg1: Result<(), E1>,
                        arg2: Result<E1, ()>,
                        arg3: Result<(), ()>,
                        arg4: Result<u32, &V1>,
                        arg5: Result<&str, &[u8]>,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "result-arg",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (
                                    Result<(), ()>,
                                    Result<(), E1>,
                                    Result<E1, ()>,
                                    Result<(), ()>,
                                    Result<u32, &V1>,
                                    Result<&str, &[u8]>,
                                ),
                                (),
                            >::new_unchecked(self.result_arg)
                        };
                        let () = callee
                            .call_async(
                                store.as_context_mut(),
                                (arg0, arg1, arg2, arg3, arg4, arg5),
                            )
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(())
                    }
                    pub async fn call_result_result<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<
                        (
                            Result<(), ()>,
                            Result<(), E1>,
                            Result<E1, ()>,
                            Result<(), ()>,
                            Result<u32, V1>,
                            Result<
                                wasmtime::component::__internal::String,
                                wasmtime::component::__internal::Vec<u8>,
                            >,
                        ),
                    >
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "result-result",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (
                                    (
                                        Result<(), ()>,
                                        Result<(), E1>,
                                        Result<E1, ()>,
                                        Result<(), ()>,
                                        Result<u32, V1>,
                                        Result<
                                            wasmtime::component::__internal::String,
                                            wasmtime::component::__internal::Vec<u8>,
                                        >,
                                    ),
                                ),
                            >::new_unchecked(self.result_result)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_return_result_sugar<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<Result<i32, MyErrno>>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "return-result-sugar",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (Result<i32, MyErrno>,),
                            >::new_unchecked(self.return_result_sugar)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_return_result_sugar2<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<Result<(), MyErrno>>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "return-result-sugar2",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (Result<(), MyErrno>,),
                            >::new_unchecked(self.return_result_sugar2)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_return_result_sugar3<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<Result<MyErrno, MyErrno>>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "return-result-sugar3",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (Result<MyErrno, MyErrno>,),
                            >::new_unchecked(self.return_result_sugar3)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_return_result_sugar4<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<Result<(i32, u32), MyErrno>>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "return-result-sugar4",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (Result<(i32, u32), MyErrno>,),
                            >::new_unchecked(self.return_result_sugar4)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_return_option_sugar<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<Option<i32>>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "return-option-sugar",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (Option<i32>,),
                            >::new_unchecked(self.return_option_sugar)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_return_option_sugar2<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<Option<MyErrno>>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "return-option-sugar2",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (Option<MyErrno>,),
                            >::new_unchecked(self.return_option_sugar2)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_result_simple<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<Result<u32, i32>>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "result-simple",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (Result<u32, i32>,),
                            >::new_unchecked(self.result_simple)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_is_clone_arg<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: &IsClone,
                    ) -> wasmtime::Result<()>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "is-clone-arg",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (&IsClone,),
                                (),
                            >::new_unchecked(self.is_clone_arg)
                        };
                        let () = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(())
                    }
                    pub async fn call_is_clone_return<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<IsClone>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/variants", function = "is-clone-return",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (IsClone,),
                            >::new_unchecked(self.is_clone_return)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                }
            }
        }
    }
}

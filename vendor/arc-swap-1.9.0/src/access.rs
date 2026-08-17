#![deny(unsafe_code)]

//! Abstracting over accessing parts of stored value.
//!
//! Sometimes, there's a big globalish data structure (like a configuration for the whole program).
//! Then there are parts of the program that need access to up-to-date version of their *part* of
//! the configuration, but for reasons of code separation and reusability, it is not desirable to
//! pass the whole configuration to each of the parts.
//!
//! This module provides means to grant the parts access to the relevant subsets of such global
//! data structure while masking the fact it is part of the bigger whole from the component.
//!
//! Note that the [`cache`][crate::cache] module has its own [`Access`][crate::cache::Access] trait
//! that serves a similar purpose, but with cached access. The signatures are different, therefore
//! an incompatible trait.
//!
//! # The general idea
//!
//! Each part of the code accepts generic [`Access<T>`][Access] for the `T` of its interest. This
//! provides means to load current version of the structure behind the scenes and get only the
//! relevant part, without knowing what the big structure is.
//!
//! For technical reasons, the [`Access`] trait is not object safe. If type erasure is desired, it
//! is possible use the [`DynAccess`] instead, which is object safe, but slightly slower.
//!
//! For some cases, it is possible to use [`ArcSwapAny::map`]. If that is not flexible enough, the
//! [`Map`] type can be created directly.
//!
//! Note that the [`Access`] trait is also implemented for [`ArcSwapAny`] itself. Additionally,
//! there's the [`Constant`] helper type, which is useful mostly for testing (it doesn't allow
//! reloading).
//!
//! # Performance
//!
//! In general, these utilities use [`ArcSwapAny::load`] internally and then apply the provided
//! transformation. This has several consequences:
//!
//! * Limitations of the [`load`][ArcSwapAny::load] apply ‒ including the recommendation to not
//!   hold the returned guard object for too long, but long enough to get consistency.
//! * The transformation should be cheap ‒ optimally just borrowing into the structure.
//!
//! # Examples
//!
//! ```rust
//! use std::sync::Arc;
//! use std::thread::{self, JoinHandle};
//! use std::time::Duration;
//!
//! use arc_swap::ArcSwap;
//! use arc_swap::access::{Access, Constant, Map};
//!
//! fn work_with_usize<A: Access<usize> + Send + 'static>(a: A) -> JoinHandle<()> {
//!     thread::spawn(move || {
//!         let mut value = 0;
//!         while value != 42 {
//!             let guard = a.load();
//!             value = *guard;
//!             println!("{}", value);
//!             // Not strictly necessary, but dropping the guard can free some resources, like
//!             // slots for tracking what values are still in use. We do it before the sleeping,
//!             // not at the end of the scope.
//!             drop(guard);
//!             thread::sleep(Duration::from_millis(50));
//!         }
//!     })
//! }
//!
//! // Passing the whole thing directly
//! // (If we kept another Arc to it, we could change the value behind the scenes)
//! work_with_usize(Arc::new(ArcSwap::from_pointee(42))).join().unwrap();
//!
//! // Passing a subset of a structure
//! struct Cfg {
//!     value: usize,
//! }
//!
//! let cfg = Arc::new(ArcSwap::from_pointee(Cfg { value: 0 }));
//! let thread = work_with_usize(Map::new(Arc::clone(&cfg), |cfg: &Cfg| &cfg.value));
//! cfg.store(Arc::new(Cfg { value: 42 }));
//! thread.join().unwrap();
//!
//! // Passing a constant that can't change. Useful mostly for testing purposes.
//! work_with_usize(Constant(42)).join().unwrap();
//! ```
use core::marker::PhantomData;
use core::ops::Deref;

use crate::imports::{Arc, Box, Rc};

use super::ref_cnt::RefCnt;
use super::strategy::Strategy;
use super::{ArcSwapAny, Guard};

/// Abstracts over ways code can get access to a value of type `T`.
///
/// This is the trait that parts of code will use when accessing a subpart of the big data
/// structure. See the [module documentation](index.html) for details.
pub trait Access<T> {
    /// A guard object containing the value and keeping it alive.
    ///
    /// For technical reasons, the library doesn't allow direct access into the stored value. A
    /// temporary guard object must be loaded, that keeps the actual value alive for the time of
    /// use.
    type Guard: Deref<Target = T>;

    /// The loading method.
    ///
    /// This returns the guard that holds the actual value. Should be called anew each time a fresh
    /// value is needed.
    fn load(&self) -> Self::Guard;
}

impl<T, A: Access<T> + ?Sized, P: Deref<Target = A>> Access<T> for P {
    type Guard = A::Guard;
    fn load(&self) -> Self::Guard {
        self.deref().load()
    }
}

impl<T> Access<T> for dyn DynAccess<T> + '_ {
    type Guard = DynGuard<T>;

    fn load(&self) -> Self::Guard {
        self.load()
    }
}

impl<T> Access<T> for dyn DynAccess<T> + '_ + Send {
    type Guard = DynGuard<T>;

    fn load(&self) -> Self::Guard {
        self.load()
    }
}

impl<T> Access<T> for dyn DynAccess<T> + '_ + Sync + Send {
    type Guard = DynGuard<T>;

    fn load(&self) -> Self::Guard {
        self.load()
    }
}

impl<T: RefCnt, S: Strategy<T>> Access<T> for ArcSwapAny<T, S> {
    type Guard = Guard<T, S>;

    fn load(&self) -> Self::Guard {
        self.load()
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub struct DirectDeref<T: RefCnt, S: Strategy<T>>(Guard<T, S>);

impl<T, S: Strategy<Arc<T>>> Deref for DirectDeref<Arc<T>, S> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.deref().deref()
    }
}

impl<T, S: Strategy<Arc<T>>> Access<T> for ArcSwapAny<Arc<T>, S> {
    type Guard = DirectDeref<Arc<T>, S>;
    fn load(&self) -> Self::Guard {
        DirectDeref(self.load())
    }
}

impl<T, S: Strategy<Rc<T>>> Deref for DirectDeref<Rc<T>, S> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.deref().deref()
    }
}

impl<T, S: Strategy<Rc<T>>> Access<T> for ArcSwapAny<Rc<T>, S> {
    type Guard = DirectDeref<Rc<T>, S>;
    fn load(&self) -> Self::Guard {
        DirectDeref(self.load())
    }
}

#[doc(hidden)]
pub struct DynGuard<T: ?Sized>(Box<dyn Deref<Target = T>>);

impl<T: ?Sized> Deref for DynGuard<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

/// An object-safe version of the [`Access`] trait.
///
/// This can be used instead of the [`Access`] trait in case a type erasure is desired. This has
/// the effect of performance hit (due to boxing of the result and due to dynamic dispatch), but
/// makes certain code simpler and possibly makes the executable smaller.
///
/// This is automatically implemented for everything that implements [`Access`].
///
/// # Examples
///
/// ```rust
/// use arc_swap::access::{Constant, DynAccess};
///
/// fn do_something(value: Box<dyn DynAccess<usize> + Send>) {
///     let v = value.load();
///     println!("{}", *v);
/// }
///
/// do_something(Box::new(Constant(42)));
/// ```
pub trait DynAccess<T> {
    /// The equivalent of [`Access::load`].
    fn load(&self) -> DynGuard<T>;
}

impl<T, A> DynAccess<T> for A
where
    A: Access<T>,
    A::Guard: 'static,
{
    fn load(&self) -> DynGuard<T> {
        DynGuard(Box::new(Access::load(self)))
    }
}

/// [DynAccess] to [Access] wrapper.
///
/// In previous versions, `Box<dyn DynAccess>` didn't implement [Access], to use inside [Map] one
/// could use this wrapper. Since then, a way was found to solve it. In most cases, this wrapper is
/// no longer necessary.
///
/// This is left in place for two reasons:
/// * Backwards compatibility.
/// * Corner-cases not covered by the found solution. For example, trait inheritance in the form of
///   `Box<dyn SomeTrait>` where `SomeTrait: Access` doesn't work out of the box and still needs
///   this wrapper.
///
/// # Examples
///
/// The example is for the simple case (which is no longer needed, but may help as an inspiration).
///
/// ```rust
/// use std::sync::Arc;
///
/// use arc_swap::ArcSwap;
/// use arc_swap::access::{AccessConvert, DynAccess, Map};
///
/// struct Inner {
///     val: usize,
/// }
///
/// struct Middle {
///     inner: Inner,
/// }
///
/// struct Outer {
///     middle: Middle,
/// }
///
/// let outer = Arc::new(ArcSwap::from_pointee(Outer {
///     middle: Middle {
///         inner: Inner {
///             val: 42,
///         }
///     }
/// }));
///
/// let middle: Arc<dyn DynAccess<Middle>> =
///     Arc::new(Map::new(outer, |outer: &Outer| &outer.middle));
/// let inner: Arc<dyn DynAccess<Inner>> =
///     Arc::new(Map::new(AccessConvert(middle), |middle: &Middle| &middle.inner));
/// let guard = inner.load();
/// assert_eq!(42, guard.val);
/// ```
pub struct AccessConvert<D>(pub D);

impl<T, D> Access<T> for AccessConvert<D>
where
    D: Deref,
    D::Target: DynAccess<T>,
{
    type Guard = DynGuard<T>;

    fn load(&self) -> Self::Guard {
        self.0.load()
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug)]
pub struct MapGuard<G, F, T, R> {
    guard: G,
    projection: F,
    _t: PhantomData<fn(&T) -> &R>,
}

impl<G, F, T, R> Deref for MapGuard<G, F, T, R>
where
    G: Deref<Target = T>,
    F: Fn(&T) -> &R,
{
    type Target = R;
    fn deref(&self) -> &R {
        (self.projection)(&self.guard)
    }
}

/// An adaptor to provide access to a part of larger structure.
///
/// This is the *active* part of this module. Use the [module documentation](index.html) for the
/// details.
#[derive(Copy, Clone, Debug)]
pub struct Map<A, T, F> {
    access: A,
    projection: F,
    _t: PhantomData<fn() -> T>,
}

impl<A, T, F> Map<A, T, F> {
    /// Creates a new instance.
    ///
    /// # Parameters
    ///
    /// * `access`: Access to the bigger structure. This is usually something like `Arc<ArcSwap>`
    ///   or `&ArcSwap`. It is technically possible to use any other [`Access`] here, though, for
    ///   example to sub-delegate into even smaller structure from a [`Map`] (or generic
    ///   [`Access`]).
    /// * `projection`: A function (or closure) responsible to providing a reference into the
    ///   bigger bigger structure, selecting just subset of it. In general, it is expected to be
    ///   *cheap* (like only taking reference).
    pub fn new<R>(access: A, projection: F) -> Self
    where
        F: Fn(&T) -> &R + Clone,
    {
        Map {
            access,
            projection,
            _t: PhantomData,
        }
    }
}

impl<A, F, T, R> Access<R> for Map<A, T, F>
where
    A: Access<T>,
    F: Fn(&T) -> &R + Clone,
{
    type Guard = MapGuard<A::Guard, F, T, R>;
    fn load(&self) -> Self::Guard {
        let guard = self.access.load();
        MapGuard {
            guard,
            projection: self.projection.clone(),
            _t: PhantomData,
        }
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConstantDeref<T>(T);

impl<T> Deref for ConstantDeref<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

/// Access to an constant.
///
/// This wraps a constant value to provide [`Access`] to it. It is constant in the sense that,
/// unlike [`ArcSwapAny`] and [`Map`], the loaded value will always stay the same (there's no
/// remote `store`).
///
/// The purpose is mostly testing and plugging a parameter that works generically from code that
/// doesn't need the updating functionality.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Constant<T>(pub T);

impl<T: Clone> Access<T> for Constant<T> {
    type Guard = ConstantDeref<T>;
    fn load(&self) -> Self::Guard {
        ConstantDeref(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ArcSwap, ArcSwapOption};

    use super::*;

    fn check_static_dispatch_direct<A: Access<usize>>(a: A) {
        assert_eq!(42, *a.load());
    }

    fn check_static_dispatch<A: Access<Arc<usize>>>(a: A) {
        assert_eq!(42, **a.load());
    }

    /// Tests dispatching statically from arc-swap works
    #[test]
    fn static_dispatch() {
        let a = ArcSwap::from_pointee(42);
        check_static_dispatch_direct(&a);
        check_static_dispatch(&a);
        check_static_dispatch(a);
    }

    fn check_dyn_dispatch_direct(a: &dyn DynAccess<usize>) {
        assert_eq!(42, *a.load());
    }

    fn check_dyn_dispatch(a: &dyn DynAccess<Arc<usize>>) {
        assert_eq!(42, **a.load());
    }

    /// Tests we can also do a dynamic dispatch of the companion trait
    #[test]
    fn dyn_dispatch() {
        let a = ArcSwap::from_pointee(42);
        check_dyn_dispatch_direct(&a);
        check_dyn_dispatch(&a);
    }

    fn check_transition<A>(a: A)
    where
        A: Access<usize>,
        A::Guard: 'static,
    {
        check_dyn_dispatch_direct(&a)
    }

    /// Tests we can easily transition from the static dispatch trait to the dynamic one
    #[test]
    fn transition() {
        let a = ArcSwap::from_pointee(42);
        check_transition(&a);
        check_transition(a);
    }

    /// Test we can dispatch from Arc<ArcSwap<_>> or similar.
    #[test]
    fn indirect() {
        let a = Arc::new(ArcSwap::from_pointee(42));
        check_static_dispatch(&a);
        check_dyn_dispatch(&a);
    }

    struct Cfg {
        value: usize,
    }

    #[test]
    fn map() {
        let a = ArcSwap::from_pointee(Cfg { value: 42 });
        let map = a.map(|a: &Cfg| &a.value);
        check_static_dispatch_direct(&map);
        check_dyn_dispatch_direct(&map);
    }

    #[test]
    fn map_option_some() {
        let a = ArcSwapOption::from_pointee(Cfg { value: 42 });
        let map = a.map(|a: &Option<Arc<Cfg>>| a.as_ref().map(|c| &c.value).unwrap());
        check_static_dispatch_direct(&map);
        check_dyn_dispatch_direct(&map);
    }

    #[test]
    fn map_option_none() {
        let a = ArcSwapOption::empty();
        let map = a.map(|a: &Option<Arc<Cfg>>| a.as_ref().map(|c| &c.value).unwrap_or(&42));
        check_static_dispatch_direct(&map);
        check_dyn_dispatch_direct(&map);
    }

    #[test]
    fn constant() {
        let c = Constant(42);
        check_static_dispatch_direct(c);
        check_dyn_dispatch_direct(&c);
        check_static_dispatch_direct(c);
    }

    #[test]
    fn map_reload() {
        let a = ArcSwap::from_pointee(Cfg { value: 0 });
        let map = a.map(|cfg: &Cfg| &cfg.value);
        assert_eq!(0, *Access::load(&map));
        a.store(Arc::new(Cfg { value: 42 }));
        assert_eq!(42, *Access::load(&map));
    }

    // Compile tests for dynamic access
    fn _expect_access<T>(_: impl Access<T>) {}

    fn _dyn_access<T>(x: Box<dyn DynAccess<T> + '_>) {
        _expect_access(x)
    }

    fn _dyn_access_send<T>(x: Box<dyn DynAccess<T> + '_ + Send>) {
        _expect_access(x)
    }

    fn _dyn_access_send_sync<T>(x: Box<dyn DynAccess<T> + '_ + Send + Sync>) {
        _expect_access(x)
    }

    #[test]
    #[allow(clippy::arc_with_non_send_sync)] // Whatever, it's tests...
    fn double_dyn_access_complex() {
        struct Inner {
            val: usize,
        }

        struct Middle {
            inner: Inner,
        }

        struct Outer {
            middle: Middle,
        }

        let outer = Arc::new(ArcSwap::from_pointee(Outer {
            middle: Middle {
                inner: Inner { val: 42 },
            },
        }));

        let middle: Arc<dyn DynAccess<Middle>> =
            Arc::new(Map::new(outer, |outer: &Outer| &outer.middle));
        let inner: Arc<dyn DynAccess<Inner>> =
            Arc::new(Map::new(middle, |middle: &Middle| &middle.inner));
        // Damn. We have the DynAccess wrapper in scope and need to disambiguate the inner.load()
        let guard = Access::load(&inner);
        assert_eq!(42, guard.val);
    }
}

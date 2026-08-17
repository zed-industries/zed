#![deny(unsafe_code)]

//! Caching handle into the [ArcSwapAny].
//!
//! The [Cache] keeps a copy of the internal [Arc] for faster access.
//!
//! [Arc]: std::sync::Arc

use core::ops::Deref;
use core::sync::atomic::Ordering;

use super::ref_cnt::RefCnt;
use super::strategy::Strategy;
use super::ArcSwapAny;

/// Generalization of caches providing access to `T`.
///
/// This abstracts over all kinds of caches that can provide a cheap access to values of type `T`.
/// This is useful in cases where some code doesn't care if the `T` is the whole structure or just
/// a part of it.
///
/// See the example at [`Cache::map`].
pub trait Access<T> {
    /// Loads the value from cache.
    ///
    /// This revalidates the value in the cache, then provides the access to the cached value.
    fn load(&mut self) -> &T;
}

/// Caching handle for [`ArcSwapAny`].
///
/// Instead of loading the [`Arc`][Arc] on every request from the shared storage, this keeps
/// another copy inside itself. Upon request it only cheaply revalidates it is up to
/// date. If it is, access is significantly faster. If it is stale, the [load_full] is done and the
/// cache value is replaced. Under a read-heavy loads, the measured speedup are 10-25 times,
/// depending on the architecture.
///
/// There are, however, downsides:
///
/// * The handle needs to be kept around by the caller (usually, one per thread). This is fine if
///   there's one global `ArcSwapAny`, but starts being tricky with eg. data structures build from
///   them.
/// * As it keeps a copy of the [Arc] inside the cache, the old value may be kept alive for longer
///   period of time ‒ it is replaced by the new value on [load][Cache::load]. You may not want to
///   use this if dropping the old value in timely manner is important (possibly because of
///   releasing large amount of RAM or because of closing file handles).
///
/// # Examples
///
/// ```rust
/// # fn do_something<V>(_v: V) { }
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicBool, Ordering};
///
/// use arc_swap::{ArcSwap, Cache};
///
/// let shared = Arc::new(ArcSwap::from_pointee(42));
/// # let mut threads = Vec::new();
/// let terminate = Arc::new(AtomicBool::new(false));
/// // Start 10 worker threads...
/// for _ in 0..10 {
///     let mut cache = Cache::new(Arc::clone(&shared));
///     let terminate = Arc::clone(&terminate);
///     # let thread =
///     std::thread::spawn(move || {
///         // Keep loading it like mad..
///         while !terminate.load(Ordering::Relaxed) {
///             let value = cache.load();
///             do_something(value);
///         }
///     });
///     # threads.push(thread);
/// }
/// shared.store(Arc::new(12));
/// # terminate.store(true, Ordering::Relaxed);
/// # for thread in threads { thread.join().unwrap() }
/// ```
///
/// Another one with using a thread local storage and explicit types:
///
/// ```rust
/// # use std::sync::Arc;
/// # use std::ops::Deref;
/// # use std::cell::RefCell;
/// #
/// # use arc_swap::ArcSwap;
/// # use arc_swap::cache::Cache;
/// # use once_cell::sync::Lazy;
/// #
/// # #[derive(Debug, Default)]
/// # struct Config;
/// #
/// static CURRENT_CONFIG: Lazy<ArcSwap<Config>> = Lazy::new(|| ArcSwap::from_pointee(Config::default()));
///
/// thread_local! {
///     static CACHE: RefCell<Cache<&'static ArcSwap<Config>, Arc<Config>>> = RefCell::new(Cache::from(CURRENT_CONFIG.deref()));
/// }
///
/// CACHE.with(|c| {
///     // * RefCell needed, because load on cache is `&mut`.
///     // * You want to operate inside the `with` ‒ cloning the Arc is comparably expensive as
///     //   ArcSwap::load itself and whatever you'd save by the cache would be lost on that.
///     println!("{:?}", c.borrow_mut().load());
/// });
/// ```
///
/// [Arc]: std::sync::Arc
/// [load_full]: ArcSwapAny::load_full
#[derive(Clone, Debug)]
pub struct Cache<A, T> {
    arc_swap: A,
    cached: T,
}

impl<A, T, S> Cache<A, T>
where
    A: Deref<Target = ArcSwapAny<T, S>>,
    T: RefCnt,
    S: Strategy<T>,
{
    /// Creates a new caching handle.
    ///
    /// The parameter is something dereferencing into an [`ArcSwapAny`] (eg. either to [`ArcSwap`]
    /// or [`ArcSwapOption`]). That can be [`ArcSwapAny`] itself, but that's not very useful. But
    /// it also can be a reference to it or `Arc`, which makes it possible to share the
    /// [`ArcSwapAny`] with multiple caches or access it in non-cached way too.
    ///
    /// [`ArcSwapOption`]: crate::ArcSwapOption
    /// [`ArcSwap`]: crate::ArcSwap
    pub fn new(arc_swap: A) -> Self {
        let cached = arc_swap.load_full();
        Self { arc_swap, cached }
    }

    /// Gives access to the (possibly shared) cached [`ArcSwapAny`].
    pub fn arc_swap(&self) -> &A::Target {
        &self.arc_swap
    }

    /// Loads the currently held value.
    ///
    /// This first checks if the cached value is up to date. This check is very cheap.
    ///
    /// If it is up to date, the cached value is simply returned without additional costs. If it is
    /// outdated, a load is done on the underlying shared storage. The newly loaded value is then
    /// stored in the cache and returned.
    #[inline]
    pub fn load(&mut self) -> &T {
        self.revalidate();
        self.load_no_revalidate()
    }

    #[inline]
    fn load_no_revalidate(&self) -> &T {
        &self.cached
    }

    #[inline]
    fn revalidate(&mut self) {
        let cached_ptr = RefCnt::as_ptr(&self.cached);
        // Node: Relaxed here is fine. We do not synchronize any data through this, we already have
        // it synchronized in self.cache. We just want to check if it changed, if it did, the
        // load_full will be responsible for any synchronization needed.
        let shared_ptr = self.arc_swap.ptr.load(Ordering::Relaxed);
        if cached_ptr != shared_ptr {
            self.cached = self.arc_swap.load_full();
        }
    }

    /// Turns this cache into a cache with a projection inside the cached value.
    ///
    /// You'd use this in case when some part of code needs access to fresh values of `U`, however
    /// a bigger structure containing `U` is provided by this cache. The possibility of giving the
    /// whole structure to the part of the code falls short in terms of reusability (the part of
    /// the code could be used within multiple contexts, each with a bigger different structure
    /// containing `U`) and code separation (the code shouldn't needs to know about the big
    /// structure).
    ///
    /// # Warning
    ///
    /// As the provided `f` is called inside every [`load`][Access::load], this one should be
    /// cheap. Most often it is expected to be just a closure taking reference of some inner field.
    ///
    /// For the same reasons, it should not have side effects and should never panic (these will
    /// not break Rust's safety rules, but might produce behaviour you don't expect).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arc_swap::ArcSwap;
    /// use arc_swap::cache::{Access, Cache};
    ///
    /// struct InnerCfg {
    ///     answer: usize,
    /// }
    ///
    /// struct FullCfg {
    ///     inner: InnerCfg,
    /// }
    ///
    /// fn use_inner<A: Access<InnerCfg>>(cache: &mut A) {
    ///     let value = cache.load();
    ///     println!("The answer is: {}", value.answer);
    /// }
    ///
    /// let full_cfg = ArcSwap::from_pointee(FullCfg {
    ///     inner: InnerCfg {
    ///         answer: 42,
    ///     }
    /// });
    /// let cache = Cache::new(&full_cfg);
    /// use_inner(&mut cache.map(|full| &full.inner));
    ///
    /// let inner_cfg = ArcSwap::from_pointee(InnerCfg { answer: 24 });
    /// let mut inner_cache = Cache::new(&inner_cfg);
    /// use_inner(&mut inner_cache);
    /// ```
    pub fn map<F, U>(self, f: F) -> MapCache<A, T, F>
    where
        F: FnMut(&T) -> &U,
    {
        MapCache {
            inner: self,
            projection: f,
        }
    }
}

impl<A, T, S> Access<T::Target> for Cache<A, T>
where
    A: Deref<Target = ArcSwapAny<T, S>>,
    T: Deref<Target = <T as RefCnt>::Base> + RefCnt,
    S: Strategy<T>,
{
    fn load(&mut self) -> &T::Target {
        self.load().deref()
    }
}

impl<A, T, S> From<A> for Cache<A, T>
where
    A: Deref<Target = ArcSwapAny<T, S>>,
    T: RefCnt,
    S: Strategy<T>,
{
    fn from(arc_swap: A) -> Self {
        Self::new(arc_swap)
    }
}

/// An implementation of a cache with a projection into the accessed value.
///
/// This is the implementation structure for [`Cache::map`]. It can't be created directly and it
/// should be used through the [`Access`] trait.
#[derive(Clone, Debug)]
pub struct MapCache<A, T, F> {
    inner: Cache<A, T>,
    projection: F,
}

impl<A, T, S, F, U> Access<U> for MapCache<A, T, F>
where
    A: Deref<Target = ArcSwapAny<T, S>>,
    T: RefCnt,
    S: Strategy<T>,
    F: FnMut(&T) -> &U,
{
    fn load(&mut self) -> &U {
        (self.projection)(self.inner.load())
    }
}

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;

    use super::*;
    use crate::{ArcSwap, ArcSwapOption};

    #[test]
    fn cached_value() {
        let a = ArcSwap::from_pointee(42);
        let mut c1 = Cache::new(&a);
        let mut c2 = Cache::new(&a);

        assert_eq!(42, **c1.load());
        assert_eq!(42, **c2.load());

        a.store(Arc::new(43));
        assert_eq!(42, **c1.load_no_revalidate());
        assert_eq!(43, **c1.load());
    }

    #[test]
    fn cached_through_arc() {
        let a = Arc::new(ArcSwap::from_pointee(42));
        let mut c = Cache::new(Arc::clone(&a));
        assert_eq!(42, **c.load());
        a.store(Arc::new(0));
        drop(a); // A is just one handle, the ArcSwap is kept alive by the cache.
    }

    #[test]
    fn cache_option() {
        let a = ArcSwapOption::from_pointee(42);
        let mut c = Cache::new(&a);

        assert_eq!(42, **c.load().as_ref().unwrap());
        a.store(None);
        assert!(c.load().is_none());
    }

    struct Inner {
        answer: usize,
    }

    struct Outer {
        inner: Inner,
    }

    #[test]
    fn map_cache() {
        let a = ArcSwap::from_pointee(Outer {
            inner: Inner { answer: 42 },
        });

        let mut cache = Cache::new(&a);
        let mut inner = cache.clone().map(|outer| &outer.inner);
        let mut answer = cache.clone().map(|outer| &outer.inner.answer);

        assert_eq!(42, cache.load().inner.answer);
        assert_eq!(42, inner.load().answer);
        assert_eq!(42, *answer.load());

        a.store(Arc::new(Outer {
            inner: Inner { answer: 24 },
        }));

        assert_eq!(24, cache.load().inner.answer);
        assert_eq!(24, inner.load().answer);
        assert_eq!(24, *answer.load());
    }
}

use core::ffi::c_void;
#[cfg(not(all(debug_assertions, not(feature = "unstable-autoreleasesafe"))))]
use core::marker::PhantomData;
#[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
use std::{cell::RefCell, thread_local, vec::Vec};

use crate::ffi;

/// The actual pool object.
///
/// It is drained when dropped.
///
/// This is not [`Send`], since `objc_autoreleasePoolPop` must be called on
/// the same thread as `objc_autoreleasePoolPush`.
///
/// And this is not [`Sync`], since that would make `AutoreleasePool` `Send`.
#[derive(Debug)]
struct Pool {
    /// This is an opaque handle, and is not guaranteed to be neither a valid
    /// nor an aligned pointer.
    context: *mut c_void,
}

impl Pool {
    /// Construct a new autorelease pool.
    ///
    ///
    /// # Safety
    ///
    /// The caller must ensure that when handing out `AutoreleasePool<'p>` to
    /// functions that this is the innermost pool.
    ///
    /// Additionally, the pools must be dropped in the same order they were
    /// created.
    #[inline]
    unsafe fn new() -> Self {
        let context = unsafe { ffi::objc_autoreleasePoolPush() };
        #[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
        POOLS.with(|c| c.borrow_mut().push(context));
        Self { context }
    }

    /// Drains the autoreleasepool.
    ///
    /// The [clang documentation] says that `@autoreleasepool` blocks are not
    /// drained when exceptions occur because:
    ///
    /// > Not draining the pool during an unwind is apparently required by the
    /// > Objective-C exceptions implementation.
    ///
    /// We _would_ really like to do this anyway whenever possible, since the
    /// unwind is probably caused by Rust, and forgetting to pop the pool will
    /// likely leak memory.
    ///
    /// The above statement was true in the past, but since [revision `551.1`]
    /// of objc4 (ships with MacOS 10.9) the exception is now retained when
    /// `@throw` is encountered (on __OBJC2__, so e.g. not on macOS 32bit).
    ///
    /// So in the future, once we drop support for older versions, we should
    /// move this to `Drop`.
    ///
    /// [clang documentation]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#autoreleasepool
    /// [revision `551.1`]: https://github.com/apple-oss-distributions/objc4/blob/objc4-551.1/runtime/objc-exception.mm#L516
    #[inline]
    unsafe fn drain(self) {
        unsafe { ffi::objc_autoreleasePoolPop(self.context) }
    }
}

impl Drop for Pool {
    #[inline]
    fn drop(&mut self) {
        #[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
        POOLS.with(|c| {
            assert_eq!(
                c.borrow_mut().pop(),
                Some(self.context),
                "popped pool that was not the innermost pool"
            );
        });
    }
}

/// An Objective-C autorelease pool.
///
/// Autorelease pools are a way to store objects in a certain thread-local
/// scope, such that they are only released at the end of said scope.
///
/// See [`autoreleasepool`] and [`autoreleasepool_leaking`] for how to create
/// this.
///
/// This is not [`Send`] nor [`Sync`], since you can only autorelease a
/// reference to a pool on the current thread.
///
///
/// # Example
///
/// Use the pool as a bound on a function, and release an object to that pool.
///
/// ```
/// use objc2::rc::{autoreleasepool, AutoreleasePool};
/// use objc2::runtime::NSObject;
/// use objc2::msg_send;
///
/// fn needs_lifetime_from_pool<'p>(pool: AutoreleasePool<'p>) -> &'p NSObject {
///     let obj = NSObject::new();
///     // Do action that returns an autoreleased object
///     let description: *mut NSObject = unsafe { msg_send![&obj, description] };
///     // Bound the lifetime of the reference to the pool
///     unsafe { pool.ptr_as_ref(description) }
/// }
///
/// autoreleasepool(|pool| {
///     let obj = needs_lifetime_from_pool(pool);
///     println!("{obj:?}");
/// });
/// ```
#[derive(Debug, Copy, Clone)]
pub struct AutoreleasePool<'pool> {
    /// A reference to the pool.
    ///
    /// The lifetime is covariant, since shortening the lifetime is not a
    /// problem (the lifetime talks about the pool, and not any data inside
    /// the pool).
    ///
    /// To somewhat prove this, consider the following example using
    /// `typed-arena` to partially implement the autorelease pool:
    ///
    /// ```ignore
    /// struct Pool(typed_arena::Arena<String>);
    ///
    /// pub struct AutoreleasePool<'pool>(&'pool Pool);
    ///
    /// impl<'pool> AutoreleasePool<'pool> {
    ///     pub fn autorelease(self, s: String) -> &'pool str {
    ///         &*self.0.0.alloc(s)
    ///     }
    ///
    ///     pub fn autorelease_mut(self, s: String) -> &'pool mut str {
    ///         &mut *self.0.0.alloc(s)
    ///     }
    /// }
    ///
    /// pub fn autoreleasepool<F, R>(f: F) -> R
    /// where
    ///     F: for<'pool> FnOnce(AutoreleasePool<'pool>) -> R
    /// {
    ///     let pool = Pool(Default::default());
    ///     f(AutoreleasePool(&pool))
    /// }
    /// ```
    ///
    /// Hence assuming `typed-arena` is sound, having covariance here should
    /// also be sound.
    #[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
    inner: Option<&'pool Pool>,
    /// We use `PhantomData` here to make `AutoreleasePool` a ZST.
    #[cfg(not(all(debug_assertions, not(feature = "unstable-autoreleasesafe"))))]
    inner: PhantomData<&'pool Pool>,
}

#[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
thread_local! {
    /// We track the thread's pools to verify that object lifetimes are only
    /// taken from the innermost pool.
    static POOLS: RefCell<Vec<*mut c_void>> = const { RefCell::new(Vec::new()) };
}

impl<'pool> AutoreleasePool<'pool> {
    fn new(_inner: Option<&'pool Pool>) -> Self {
        Self {
            #[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
            inner: _inner,
            #[cfg(not(all(debug_assertions, not(feature = "unstable-autoreleasesafe"))))]
            inner: PhantomData,
        }
    }

    /// This will be removed in a future version.
    #[inline]
    #[doc(hidden)]
    pub fn __verify_is_inner(self) {
        #[cfg(all(debug_assertions, not(feature = "unstable-autoreleasesafe")))]
        if let Some(pool) = &self.inner {
            POOLS.with(|c| {
                assert_eq!(
                    c.borrow().last(),
                    Some(&pool.context),
                    "tried to use lifetime from pool that was not innermost"
                );
            });
        }
    }

    /// Returns a shared reference to the given autoreleased pointer object.
    ///
    /// This is the preferred way to make references from autoreleased
    /// objects, since it binds the lifetime of the reference to the pool, and
    /// does some extra checks when debug assertions are enabled.
    ///
    /// For the mutable counterpart see [`ptr_as_mut`](#method.ptr_as_mut).
    ///
    ///
    /// # Safety
    ///
    /// This is equivalent to `&*ptr`, and shares the unsafety of that, except
    /// the lifetime is bound to the pool instead of being unbounded.
    #[inline]
    pub unsafe fn ptr_as_ref<T: ?Sized>(self, ptr: *const T) -> &'pool T {
        self.__verify_is_inner();
        // SAFETY: Checked by the caller
        unsafe { ptr.as_ref().unwrap_unchecked() }
    }

    /// Returns a unique reference to the given autoreleased pointer object.
    ///
    /// This is the preferred way to make mutable references from autoreleased
    /// objects, since it binds the lifetime of the reference to the pool, and
    /// does some extra checks when debug assertions are enabled.
    ///
    /// For the shared counterpart see [`ptr_as_ref`](#method.ptr_as_ref).
    ///
    ///
    /// # Safety
    ///
    /// This is equivalent to `&mut *ptr`, and shares the unsafety of that,
    /// except the lifetime is bound to the pool instead of being unbounded.
    #[inline]
    pub unsafe fn ptr_as_mut<T: ?Sized>(self, ptr: *mut T) -> &'pool mut T {
        self.__verify_is_inner();
        // SAFETY: Checked by the caller
        unsafe { ptr.as_mut().unwrap_unchecked() }
    }
}

/// We use a macro here so that the documentation is included whether the
/// feature is enabled or not.
#[cfg(not(feature = "unstable-autoreleasesafe"))]
macro_rules! auto_trait {
    {$(#[$fn_meta:meta])* $v:vis unsafe trait AutoreleaseSafe {}} => {
        $(#[$fn_meta])*
        $v unsafe trait AutoreleaseSafe {}
    }
}

#[cfg(feature = "unstable-autoreleasesafe")]
macro_rules! auto_trait {
    {$(#[$fn_meta:meta])* $v:vis unsafe trait AutoreleaseSafe {}} => {
        $(#[$fn_meta])*
        $v unsafe auto trait AutoreleaseSafe {}
    }
}

auto_trait! {
    /// Marks types that are safe to pass across the closure in an
    /// [`autoreleasepool`].
    ///
    /// With the `"unstable-autoreleasesafe"` feature enabled, this is an auto
    /// trait that is implemented for all types except [`AutoreleasePool`].
    ///
    /// Otherwise it is a dummy trait that is implemented for all types; the
    /// safety invariants are checked with debug assertions instead.
    ///
    /// You should not normally need to implement this trait yourself.
    ///
    ///
    /// # Safety
    ///
    /// Must not be implemented for types that interract with the autorelease
    /// pool. So if you reimplement the [`AutoreleasePool`] struct or
    /// likewise, this should be negatively implemented for that.
    ///
    /// This can be accomplished with an `PhantomData<AutoreleasePool<'_>>` if
    /// the `"unstable-autoreleasesafe"` feature is enabled.
    ///
    ///
    /// # Examples
    ///
    /// Most types are [`AutoreleaseSafe`].
    ///
    /// ```
    /// use objc2::rc::{AutoreleasePool, AutoreleaseSafe};
    /// fn requires_autoreleasesafe<T: AutoreleaseSafe>() {}
    /// requires_autoreleasesafe::<()>();
    /// requires_autoreleasesafe::<Box<Vec<i32>>>();
    /// requires_autoreleasesafe::<fn(AutoreleasePool<'_>)>();
    /// ```
    ///
    /// But [`AutoreleasePool`] isn't (if the `"unstable-autoreleasesafe"`
    /// feature is enabled).
    ///
    #[cfg_attr(feature = "unstable-autoreleasesafe", doc = "```compile_fail")]
    #[cfg_attr(not(feature = "unstable-autoreleasesafe"), doc = "```")]
    /// use objc2::rc::AutoreleasePool;
    /// # use objc2::rc::AutoreleaseSafe;
    /// # fn requires_autoreleasesafe<T: AutoreleaseSafe>() {}
    /// requires_autoreleasesafe::<AutoreleasePool<'static>>();
    /// ```
    ///
    /// This also means that trait objects aren't (since they may contain an
    /// [`AutoreleasePool`] internally):
    ///
    #[cfg_attr(feature = "unstable-autoreleasesafe", doc = "```compile_fail")]
    #[cfg_attr(not(feature = "unstable-autoreleasesafe"), doc = "```")]
    /// # use objc2::rc::AutoreleaseSafe;
    /// # fn requires_autoreleasesafe<T: AutoreleaseSafe>() {}
    /// requires_autoreleasesafe::<&dyn std::io::Write>();
    /// ```
    pub unsafe trait AutoreleaseSafe {}
}

#[cfg(not(feature = "unstable-autoreleasesafe"))]
unsafe impl<T: ?Sized> AutoreleaseSafe for T {}

#[cfg(feature = "unstable-autoreleasesafe")]
impl !AutoreleaseSafe for Pool {}
#[cfg(feature = "unstable-autoreleasesafe")]
impl !AutoreleaseSafe for AutoreleasePool<'_> {}

/// Execute `f` in the context of a new autorelease pool. The pool is drained
/// after the execution of `f` completes.
///
/// This corresponds to `@autoreleasepool` blocks in Objective-C and Swift.
///
/// The pool is passed as a parameter to the closure to give you a lifetime
/// parameter that autoreleased objects can refer to.
///
/// Note that this is mostly useful for preventing leaks (as any Objective-C
/// method may autorelease internally - see also [`autoreleasepool_leaking`]).
/// If implementing an interface to an object, you should try to return
/// retained pointers with [`msg_send_id!`] wherever you can instead, since
/// it is usually more efficient, and having to use this function can be quite
/// cumbersome for users.
///
/// [`msg_send_id!`]: crate::msg_send_id
///
///
/// # Restrictions
///
/// The given parameter must not be used in an inner `autoreleasepool` - doing
/// so will panic with debug assertions enabled, and be a compile error in a
/// future release.
///
/// Note that this means that **this function is currently unsound**, since it
/// doesn't disallow wrong usage in all cases. Enabling the assertions in
/// release mode would be prohibitively expensive though, so this is the
/// least-bad solution.
///
/// You can try to compile your crate with the `"unstable-autoreleasesafe"`
/// crate feature enabled on nightly Rust - if your crate compiles with that,
/// its autoreleasepool usage is guaranteed to be correct.
///
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use objc2::rc::{autoreleasepool, Retained};
/// use objc2::runtime::NSObject;
///
/// autoreleasepool(|pool| {
///     // Create `obj` and autorelease it to the pool
///     let obj = Retained::autorelease(NSObject::new(), pool);
///     // We now have a reference that we can freely use
///     println!("{obj:?}");
///     // `obj` is deallocated when the pool ends
/// });
/// // And is no longer usable outside the closure
/// ```
///
/// Fails to compile because `obj` does not live long enough for us to take it
/// out of the pool:
///
/// ```compile_fail
/// use objc2::rc::{autoreleasepool, Retained};
/// use objc2::runtime::NSObject;
///
/// let obj = autoreleasepool(|pool| {
///     Retained::autorelease(NSObject::new(), pool)
/// });
/// ```
///
/// Fails to compile with the `"unstable-autoreleasesafe"` feature enabled, or
/// panics with debug assertions enabled, because we tried to pass an outer
/// pool to an inner pool:
///
#[cfg_attr(feature = "unstable-autoreleasesafe", doc = "```compile_fail")]
#[cfg_attr(not(feature = "unstable-autoreleasesafe"), doc = "```should_panic")]
/// use objc2::rc::{autoreleasepool, Retained};
/// use objc2::runtime::NSObject;
///
/// autoreleasepool(|outer_pool| {
///     let obj = autoreleasepool(|inner_pool| {
///         Retained::autorelease(NSObject::new(), outer_pool)
///     });
///     // `obj` could wrongly be used here because its lifetime was
///     // assigned to the outer pool, even though it was released by the
///     // inner pool already.
/// });
/// #
/// # panic!("Does not panic in release mode, so for testing we make it!");
/// ```
///
/// It is impossible to extend the lifetime of the pool.
///
/// ```compile_fail
/// use std::cell::RefCell;
/// use objc2::rc::{autoreleasepool, AutoreleasePool};
///
/// thread_local! {
///     static POOL: RefCell<Option<&'static AutoreleasePool<'static>>> = RefCell::new(None);
/// }
///
/// autoreleasepool(|pool| {
///     POOL.with(|p| {
///         *p.borrow_mut() = Some(Box::leak(Box::new(pool)))
///     });
/// });
/// ```
#[doc(alias = "@autoreleasepool")]
#[doc(alias = "objc_autoreleasePoolPush")]
#[doc(alias = "objc_autoreleasePoolPop")]
#[inline]
pub fn autoreleasepool<T, F>(f: F) -> T
where
    for<'pool> F: AutoreleaseSafe + FnOnce(AutoreleasePool<'pool>) -> T,
{
    // SAFETY:
    // - The `AutoreleaseSafe` bound on the closure ensures that no pool from
    //   a different "level" can be passed down through and used in this one.
    // - The pools are guaranteed to be dropped in the reverse order they were
    //   created (since you can't possibly "interleave" closures).
    //
    //   This would not work if we e.g. allowed users to create pools on the
    //   stack, since they could then safely control the drop order.
    let pool = unsafe { Pool::new() };
    let res = f(AutoreleasePool::new(Some(&pool)));
    unsafe { pool.drain() };
    res
}

/// Execute `f` in the context of a "fake" autorelease pool.
///
/// This is useful to create a context in which to use autoreleased objects,
/// without the overhead of actually creating and draining the pool.
///
/// Any function boundary in Objective-C is an implicit autorelease pool, so
/// there you'd do `id obj2 = [obj autorelease]` and be done with it - but we
/// do this using a closure instead because we need some way to bind the
/// lifetime of any objects released to the pool.
///
///
/// # Examples
///
/// Autorelease an object to an outer pool, from inside an inner, "fake" pool.
///
/// ```
/// use objc2::rc::{autoreleasepool, autoreleasepool_leaking, Retained};
/// use objc2::runtime::NSObject;
///
/// autoreleasepool(|outer_pool| {
///     let obj = autoreleasepool_leaking(|inner_pool| {
///         Retained::autorelease(NSObject::new(), outer_pool)
///     });
///     // `obj` is still usable here, since the leaking pool doesn't actually
///     // do anything.
///     println!("{obj:?}");
/// });
///
/// // But it is not usable here, since the outer pool has been closed
/// ```
///
/// Like [`autoreleasepool`], you can't extend the lifetime of an object to
/// outside the closure.
///
/// ```compile_fail
/// use objc2::rc::{autoreleasepool_leaking, Retained};
/// use objc2::runtime::NSObject;
///
/// let obj = autoreleasepool_leaking(|pool| {
///     Retained::autorelease(NSObject::new(), pool)
/// });
/// ```
///
/// While you can pass an outer pool into this, you still can't pass the pool
/// from this into [`autoreleasepool`]:
///
#[cfg_attr(feature = "unstable-autoreleasesafe", doc = "```compile_fail")]
#[cfg_attr(not(feature = "unstable-autoreleasesafe"), doc = "```should_panic")]
/// use objc2::rc::{autoreleasepool, autoreleasepool_leaking, Retained};
/// use objc2::runtime::NSObject;
///
/// autoreleasepool_leaking(|outer_pool| {
///     let obj = autoreleasepool(|inner_pool| {
///         Retained::autorelease(NSObject::new(), outer_pool)
///     });
/// });
/// #
/// # panic!("Does not panic in release mode, so for testing we make it!");
/// ```
#[inline]
pub fn autoreleasepool_leaking<T, F>(f: F) -> T
where
    for<'pool> F: FnOnce(AutoreleasePool<'pool>) -> T,
{
    // SAFETY: This is effectively what most Objective-C code does; they
    // assume that there's an autorelease pool _somewhere_ in the call stack
    // above it, and then use their autoreleased objects for a duration that
    // is guaranteed to be shorter than that.
    //
    // The `AutoreleaseSafe` bound is not required, since we don't actually do
    // anything inside this; hence if the user know they have the _actual_
    // innermost pool, they may still safely use it to extend the lifetime
    // beyond this closure.
    f(AutoreleasePool::new(None))
}

#[cfg(test)]
mod tests {
    use core::mem;
    use core::panic::{RefUnwindSafe, UnwindSafe};

    use static_assertions::{assert_impl_all, assert_not_impl_any};

    use super::{AutoreleasePool, AutoreleaseSafe};
    use crate::runtime::AnyObject;

    #[test]
    fn auto_traits() {
        assert_impl_all!(AutoreleasePool<'static>: Unpin, UnwindSafe, RefUnwindSafe);
        assert_not_impl_any!(AutoreleasePool<'static>: Send, Sync);

        assert_impl_all!(usize: AutoreleaseSafe);
        assert_impl_all!(*mut AnyObject: AutoreleaseSafe);
        assert_impl_all!(&mut AnyObject: AutoreleaseSafe);
        #[cfg(feature = "unstable-autoreleasesafe")]
        assert_not_impl_any!(AutoreleasePool<'static>: AutoreleaseSafe);
    }

    #[allow(unused)]
    fn assert_covariant1<'a>(pool: AutoreleasePool<'static>) -> AutoreleasePool<'a> {
        pool
    }

    #[allow(unused)]
    fn assert_covariant2<'long: 'short, 'short>(
        pool: AutoreleasePool<'long>,
    ) -> AutoreleasePool<'short> {
        pool
    }

    #[allow(unused)]
    fn assert_object_safe(_: &dyn AutoreleaseSafe) {}

    #[cfg_attr(
        not(feature = "unstable-autoreleasesafe"),
        ignore = "only stably ZST when `unstable-autoreleasesafe` is enabled"
    )]
    #[test]
    fn assert_zst() {
        assert_eq!(mem::size_of::<AutoreleasePool<'static>>(), 0);
    }
}

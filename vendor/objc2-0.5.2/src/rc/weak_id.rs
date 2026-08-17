use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::fmt;
use core::marker::PhantomData;
use core::ptr;
use std::panic::{RefUnwindSafe, UnwindSafe};

use super::Retained;
use crate::mutability::{IsIdCloneable, IsRetainable};
use crate::{ffi, Message};

/// A weak pointer to an Objective-C reference counted object.
///
/// The object is allowed to be deallocated while the weak pointer is alive,
/// though the backing allocation for the object can only be released once all
/// weak pointers are gone.
///
/// Useful for breaking reference cycles and safely checking whether an
/// object has been deallocated.
///
///
/// # Comparison to `std` types
///
/// This is the Objective-C equivalent of [`sync::Weak`] from the standard
/// library, and hence is only usable on types where `Retained<T>` acts like
/// [`sync::Arc`], a.k.a. on non-mutable types.
///
/// [`sync::Weak`]: std::sync::Weak
/// [`sync::Arc`]: std::sync::Arc
#[repr(transparent)] // This is not a public guarantee
#[doc(alias = "WeakId")]
pub struct Weak<T: ?Sized> {
    /// We give the runtime the address to this box, so that it can modify it
    /// even if the `Weak` is moved.
    ///
    /// Loading may modify the pointer through a shared reference, so we use
    /// an UnsafeCell to get a *mut without self being mutable.
    ///
    /// Remember that any thread may actually modify the inner value
    /// concurrently, but as long as we only use it through the `objc_XXXWeak`
    /// methods, all access is behind a lock.
    ///
    /// TODO: Verify the need for UnsafeCell?
    /// TODO: Investigate if we can avoid some allocations using `Pin`.
    inner: Box<UnsafeCell<*mut ffi::objc_object>>,
    /// Weak inherits variance, dropck and various marker traits from
    /// `Retained<T>`.
    item: PhantomData<Retained<T>>,
}

/// Soft-deprecated type-alias to [`Weak`].
pub type WeakId<T> = Weak<T>;

impl<T: Message> Weak<T> {
    /// Construct a new weak pointer that references the given object.
    #[doc(alias = "objc_initWeak")]
    #[inline]
    pub fn new(obj: &T) -> Self
    where
        T: IsRetainable,
    {
        // SAFETY: `obj` is retainable
        unsafe { Self::new_inner(obj) }
    }

    /// Construct a new weak pointer that references the given [`Retained`].
    ///
    /// Soft-deprecated alias of [`Weak::from_retained`].
    #[doc(alias = "objc_initWeak")]
    #[inline]
    pub fn from_id(obj: &Retained<T>) -> Self
    where
        T: IsIdCloneable,
    {
        Self::from_retained(obj)
    }

    /// Construct a new weak pointer that references the given [`Retained`].
    ///
    /// You should prefer [`Weak::new`] whenever the object is retainable.
    #[doc(alias = "objc_initWeak")]
    #[inline]
    pub fn from_retained(obj: &Retained<T>) -> Self
    where
        T: IsIdCloneable,
    {
        // SAFETY: `obj` is cloneable, and is known to have come from `Retained`.
        unsafe { Self::new_inner(Retained::as_ptr(obj)) }
    }

    /// Raw constructor.
    ///
    ///
    /// # Safety
    ///
    /// The object must be valid or null.
    unsafe fn new_inner(obj: *const T) -> Self {
        let inner = Box::new(UnsafeCell::new(ptr::null_mut()));
        // SAFETY: `ptr` will never move, and the caller verifies `obj`
        let _ = unsafe { ffi::objc_initWeak(inner.get(), (obj as *mut T).cast()) };
        Self {
            inner,
            item: PhantomData,
        }
    }

    /// Load the object into an [`Retained`] if it still exists.
    ///
    /// Returns [`None`] if the object has been deallocated, or the `Weak`
    /// was created with [`Default::default`].
    #[doc(alias = "retain")]
    #[doc(alias = "objc_loadWeak")]
    #[doc(alias = "objc_loadWeakRetained")]
    #[inline]
    pub fn load(&self) -> Option<Retained<T>> {
        let ptr = self.inner.get();
        let obj = unsafe { ffi::objc_loadWeakRetained(ptr) }.cast();
        // SAFETY: The object has +1 retain count
        unsafe { Retained::from_raw(obj) }
    }

    // TODO: Add `autorelease(&self, pool) -> Option<&T>` using `objc_loadWeak`?
}

impl<T: ?Sized> Drop for Weak<T> {
    /// Destroys the weak pointer.
    #[doc(alias = "objc_destroyWeak")]
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::objc_destroyWeak(self.inner.get()) }
    }
}

// TODO: Add ?Sized
impl<T: Message + IsRetainable> Clone for Weak<T> {
    /// Make a clone of the weak pointer that points to the same object.
    #[doc(alias = "objc_copyWeak")]
    fn clone(&self) -> Self {
        let ptr = Box::new(UnsafeCell::new(ptr::null_mut()));
        unsafe { ffi::objc_copyWeak(ptr.get(), self.inner.get()) };
        Self {
            inner: ptr,
            item: PhantomData,
        }
    }
}

// TODO: Add ?Sized
impl<T: Message + IsRetainable> Default for Weak<T> {
    /// Constructs a new weak pointer that doesn't reference any object.
    ///
    /// Calling [`Self::load`] on the return value always gives [`None`].
    #[inline]
    fn default() -> Self {
        // SAFETY: The pointer is null
        unsafe { Self::new_inner(ptr::null()) }
    }
}

impl<T: ?Sized> fmt::Debug for Weak<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Note: We intentionally don't try to debug-print the value, since
        // that could lead to cycles. See:
        // https://github.com/rust-lang/rust/pull/90291
        write!(f, "(Weak)")
    }
}

// Same as `std::sync::Weak<T>`.
unsafe impl<T: ?Sized + Sync + Send + IsIdCloneable> Sync for Weak<T> {}

// Same as `std::sync::Weak<T>`.
unsafe impl<T: ?Sized + Sync + Send + IsIdCloneable> Send for Weak<T> {}

// Same as `std::sync::Weak<T>`.
impl<T: ?Sized> Unpin for Weak<T> {}

// Same as `std::sync::Weak<T>`.
impl<T: ?Sized + RefUnwindSafe + IsIdCloneable> RefUnwindSafe for Weak<T> {}

// Same as `std::sync::Weak<T>`.
impl<T: ?Sized + RefUnwindSafe + IsIdCloneable> UnwindSafe for Weak<T> {}

impl<T: Message + IsRetainable> From<&T> for Weak<T> {
    #[inline]
    fn from(obj: &T) -> Self {
        Weak::new(obj)
    }
}

impl<T: Message + IsIdCloneable> From<&Retained<T>> for Weak<T> {
    #[inline]
    fn from(obj: &Retained<T>) -> Self {
        Weak::from_retained(obj)
    }
}

impl<T: Message + IsIdCloneable> From<Retained<T>> for Weak<T> {
    #[inline]
    fn from(obj: Retained<T>) -> Self {
        Weak::from_retained(&obj)
    }
}

#[cfg(test)]
mod tests {
    use core::mem;

    use super::*;
    use crate::rc::{RcTestObject, ThreadTestData};
    use crate::runtime::NSObject;

    #[test]
    fn test_weak() {
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let weak = Weak::from(&obj);
        expected.assert_current();

        let strong = weak.load().unwrap();
        expected.try_retain += 1;
        expected.assert_current();
        assert!(ptr::eq(&*strong, &*obj));

        drop(obj);
        drop(strong);
        expected.release += 2;
        expected.drop += 1;
        expected.assert_current();

        if cfg!(not(feature = "gnustep-1-7")) {
            // This loads the object on GNUStep for some reason??
            assert!(weak.load().is_none());
            expected.assert_current();
        }

        drop(weak);
        expected.assert_current();
    }

    #[test]
    fn test_weak_clone() {
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let weak = Weak::from(&obj);
        expected.assert_current();

        let weak2 = weak.clone();
        if cfg!(target_vendor = "apple") {
            expected.try_retain += 1;
            expected.release += 1;
        }
        expected.assert_current();

        let strong = weak.load().unwrap();
        expected.try_retain += 1;
        expected.assert_current();
        assert!(ptr::eq(&*strong, &*obj));

        let strong2 = weak2.load().unwrap();
        expected.try_retain += 1;
        expected.assert_current();
        assert!(ptr::eq(&*strong, &*strong2));

        drop(weak);
        drop(weak2);
        expected.assert_current();
    }

    #[test]
    fn test_weak_default() {
        let weak: Weak<RcTestObject> = Weak::default();
        assert!(weak.load().is_none());
        drop(weak);
    }

    #[repr(C)]
    struct MyObject<'a> {
        inner: NSObject,
        p: PhantomData<&'a str>,
    }

    /// Test that `Weak<T>` is covariant over `T`.
    #[allow(unused)]
    fn assert_variance<'a, 'b>(obj: &'a Weak<MyObject<'static>>) -> &'a Weak<MyObject<'b>> {
        obj
    }

    #[test]
    fn test_size_of() {
        assert_eq!(
            mem::size_of::<Option<Weak<NSObject>>>(),
            mem::size_of::<*const ()>()
        );
    }
}

use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;
use core::{fmt, ptr};

use crate::__macro_helpers::declared_ivars::initialize_ivars;
use crate::runtime::{objc_release_fast, AnyObject};
use crate::{DeclaredClass, Message};

/// An Objective-C object that has been allocated, but not initialized.
///
/// Objective-C splits the allocation and initialization steps up into two, so
/// we need to track it in the type-system whether something has been
/// intialized or not.
///
/// Note that allocation in Objective-C can fail, e.g. in Out Of Memory
/// situations! This is handled by `objc2` automatically, but if you really
/// need to, you can check for this explicitly by inspecting the pointer
/// returned from [`as_ptr`].
///
/// Note also that this represents that the _current_ class's instance
/// variables are not yet initialized; but subclass instance variables may
/// have been so.
///
/// See [Apple's documentation on Object Allocation][object-allocation] for a
/// few more details.
///
/// [`as_ptr`]: Self::as_ptr
/// [object-allocation]: https://developer.apple.com/library/archive/documentation/General/Conceptual/CocoaEncyclopedia/ObjectAllocation/ObjectAllocation.html
///
///
/// # Memory layout
///
/// This is guaranteed to have the same size and alignment as a pointer to the
/// object, `*const T`. The pointer may be NULL.
#[repr(transparent)]
#[derive(Debug)]
pub struct Allocated<T: ?Sized> {
    /// The yet-to-be initialized object.
    ///
    /// We don't use `Retained` here, since that has different auto-trait
    /// impls, and requires in its safety contract that the object is
    /// initialized (which makes it difficult to ensure correctness if such
    /// things are split across different files). Additionally, we want to
    /// have fine control over NULL-ness.
    ///
    /// Covariance is correct, same as `Retained`.
    ptr: *const T, // Intentially not `NonNull`!
    /// Necessary for dropck, as with `Retained`.
    p: PhantomData<T>,
    /// Necessary for restricting auto traits.
    ///
    /// We _could_ probably implement auto traits `Send` and `Sync` here, but to be
    /// safe, we won't for now.
    p_auto_traits: PhantomData<AnyObject>,
}

// Explicitly don't implement `Deref`, `Message` nor `RefEncode`.

impl<T: ?Sized + Message> Allocated<T> {
    /// # Safety
    ///
    /// The caller must ensure the pointer is NULL, or that the given object
    /// has +1 retain count, and that the object behind the pointer has been
    /// allocated (but not yet initialized).
    #[inline]
    pub(crate) unsafe fn new(ptr: *mut T) -> Self {
        Self {
            ptr,
            p: PhantomData,
            p_auto_traits: PhantomData,
        }
    }

    /// Returns a raw pointer to the object.
    ///
    /// The pointer is valid for at least as long as the `Allocated` is held.
    ///
    /// See [`Allocated::as_mut_ptr`] for the mutable equivalent.
    ///
    /// This is an associated method, and must be called as
    /// `Allocated::as_ptr(obj)`.
    #[inline]
    pub fn as_ptr(this: &Self) -> *const T {
        this.ptr
    }

    /// Returns a raw mutable pointer to the object.
    ///
    /// The pointer is valid for at least as long as the `Allocated` is held.
    ///
    /// See [`Allocated::as_ptr`] for the immutable equivalent.
    ///
    /// This is an associated method, and must be called as
    /// `Allocated::as_mut_ptr(obj)`.
    ///
    ///
    /// # Note about mutable references
    ///
    /// In general, you're not allowed to create a mutable reference from
    /// `Allocated`, unless you're defining the object and know that to be
    /// safe.
    ///
    /// For example, `+[NSMutableString alloc]` is allowed to return a
    /// non-unique object as an optimization, and then only figure out
    /// afterwards whether it needs to allocate, or if it can store an
    /// `NSString` internally.
    ///
    /// Similarly, while e.g. `+[NSData alloc]` may return a unique object,
    /// calling `-[NSData init]` afterwards could return a shared empty
    /// `NSData` instance.
    #[inline]
    #[allow(unknown_lints)] // New lint below
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn as_mut_ptr(this: &mut Self) -> *mut T {
        // Note: Not bound by `T: IsMutable`, since mutable pointers _can_ be
        // safe for non-mutable classes, especially right when they're being
        // allocated / initialized.
        this.ptr as *mut T
    }

    #[inline]
    pub(crate) fn into_ptr(this: Self) -> *mut T {
        let this = ManuallyDrop::new(this);
        this.ptr as *mut T
    }

    /// Initialize the instance variables for this object.
    ///
    /// This consumes the allocated instance, and returns the now partially
    /// initialized instance instead, which can be further used in
    /// [`msg_send_id!`] `super` calls.
    ///
    /// This works very similarly to [Swift's two-phase initialization
    /// scheme][two-phase-init], see that for details.
    ///
    /// [`msg_send_id!`]: crate::msg_send_id
    /// [two-phase-init]: https://docs.swift.org/swift-book/documentation/the-swift-programming-language/initialization/#Two-Phase-Initialization
    ///
    ///
    /// # Panics
    ///
    /// If debug assertions are enabled, this function will panic if the
    /// allocated instance is `NULL`, which usually only happens in Out of
    /// Memory situations.
    ///
    /// If debug assertions are disabled, this will return a `NULL` instance
    /// and the ivars will be dropped. The NULL instance cannot cause
    /// unsoundness and will likely lead to an initialization failure later on
    /// instead, but not panicking here is done as a code-size optimization.
    //
    // Note: This is intentionally _not_ an associated method, even though
    // `Allocated` will become `MethodReceiver` in the future.
    #[inline]
    #[track_caller]
    pub fn set_ivars(self, ivars: T::Ivars) -> PartialInit<T>
    where
        T: DeclaredClass + Sized,
    {
        if let Some(ptr) = NonNull::new(ManuallyDrop::new(self).ptr as *mut T) {
            // SAFETY: The pointer came from `self`, so it is valid.
            unsafe { initialize_ivars::<T>(ptr, ivars) };

            // SAFETY:
            // - The pointer came from a `ManuallyDrop<Allocated<T>>`, which means
            //   that we've now transfered ownership over +1 retain count.
            // - The instance variables for this class have been intialized above.
            unsafe { PartialInit::new(ptr.as_ptr()) }
        } else if cfg!(debug_assertions) {
            panic!("tried to initialize instance variables on a NULL allocated object")
        } else {
            // Explicitly drop the ivars in this branch
            drop(ivars);

            // Create a new NULL PartialInit, which will likely be checked for
            // NULL-ness later on, after initialization of it has failed.
            //
            // SAFETY: The pointer is NULL.
            unsafe { PartialInit::new(ptr::null_mut()) }
        }
    }
}

impl<T: ?Sized> Drop for Allocated<T> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: Allocated objects can always safely be released, since
        // destructors are written to take into account that the object may
        // not have been initialized.
        //
        // This is also safe in the case where the object is NULL,
        // since `objc_release` allows NULL pointers.
        //
        // Rest is same as `Retained`'s `Drop`.
        unsafe { objc_release_fast(self.ptr as *mut _) };
    }
}

impl<T: ?Sized> fmt::Pointer for Allocated<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

/// An Objective-C object that has been allocated and initialized in the
/// current class, but not yet initialized in the superclass.
///
/// This is returned by [`Allocated::set_ivars`], and is intended to be used
/// further in [`msg_send_id!`] `super` calls.
///
/// [`msg_send_id!`]: crate::msg_send_id
///
///
/// # Memory layout
///
/// The memory layout of this struct is NOT currently guaranteed, as we may
/// want to be able to move a drop flag to the stack in the future.
//
// Internally, this is very similar to `Allocated`, except that we have
// different guarantees on the validity of the object.
#[repr(transparent)]
#[derive(Debug)]
pub struct PartialInit<T: ?Sized> {
    /// The partially initialized object.
    ///
    /// Variance is same as `Retained`.
    ptr: *const T, // Intentionally not NonNull<T>
    /// Necessary for dropck, as with `Retained`.
    p: PhantomData<T>,
    /// Restrict auto traits, same as `Allocated<T>`.
    p_auto_traits: PhantomData<AnyObject>,
}

impl<T: ?Sized + Message> PartialInit<T> {
    /// # Safety
    ///
    /// The caller must ensure the pointer is NULL, or that the given object
    /// is allocated, has +1 retain count, and that the class' instance
    /// variables have been initialized.
    #[inline]
    pub(crate) unsafe fn new(ptr: *mut T) -> Self {
        Self {
            ptr,
            p: PhantomData,
            p_auto_traits: PhantomData,
        }
    }

    /// Returns a raw pointer to the object.
    ///
    /// The pointer is valid for at least as long as the `PartialInit` is
    /// held.
    ///
    /// See [`PartialInit::as_mut_ptr`] for the mutable equivalent.
    ///
    /// This is an associated method, and must be called as
    /// `PartialInit::as_ptr(obj)`.
    #[inline]
    pub fn as_ptr(this: &Self) -> *const T {
        this.ptr
    }

    /// Returns a raw mutable pointer to the object.
    ///
    /// The pointer is valid for at least as long as the `PartialInit` is
    /// held.
    ///
    /// See [`PartialInit::as_ptr`] for the immutable equivalent.
    ///
    /// This is an associated method, and must be called as
    /// `PartialInit::as_mut_ptr(obj)`.
    #[inline]
    #[allow(unknown_lints)] // New lint below
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn as_mut_ptr(this: &mut Self) -> *mut T {
        this.ptr as *mut T
    }

    #[inline]
    pub(crate) fn into_ptr(this: Self) -> *mut T {
        let this = ManuallyDrop::new(this);
        this.ptr as *mut T
    }
}

impl<T: ?Sized> Drop for PartialInit<T> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: Partially initialized objects can always safely be
        // released, since destructors are written to take into account that
        // the object may not have been fully initialized.
        //
        // This is also safe in the case where the object is NULL,
        // since `objc_release` allows NULL pointers.
        //
        // Rest is same as `Retained`.
        unsafe { objc_release_fast(self.ptr as *mut _) };
    }
}

impl<T: ?Sized> fmt::Pointer for PartialInit<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

#[cfg(test)]
mod tests {
    use core::panic::{RefUnwindSafe, UnwindSafe};

    use static_assertions::assert_not_impl_any;

    use super::*;
    use crate::rc::RcTestObject;
    use crate::runtime::NSObject;

    #[test]
    fn auto_traits() {
        assert_not_impl_any!(Allocated<()>: Send, Sync, UnwindSafe, RefUnwindSafe, Unpin);
        assert_not_impl_any!(PartialInit<()>: Send, Sync, UnwindSafe, RefUnwindSafe, Unpin);
    }

    #[repr(C)]
    struct MyObject<'a> {
        inner: NSObject,
        p: PhantomData<&'a str>,
    }

    /// Test that `Allocated<T>` is covariant over `T`.
    #[allow(unused)]
    fn assert_allocated_variance<'b>(obj: Allocated<MyObject<'static>>) -> Allocated<MyObject<'b>> {
        obj
    }

    /// Test that `PartialInit<T>` is covariant over `T`.
    #[allow(unused)]
    fn assert_partialinit_variance<'b>(
        obj: PartialInit<MyObject<'static>>,
    ) -> PartialInit<MyObject<'b>> {
        obj
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic = "tried to initialize instance variables on a NULL allocated object"
    )]
    fn test_set_ivars_null() {
        // SAFETY: The pointer is NULL
        let obj: Allocated<RcTestObject> = unsafe { Allocated::new(ptr::null_mut()) };
        let _ = obj.set_ivars(());
    }
}

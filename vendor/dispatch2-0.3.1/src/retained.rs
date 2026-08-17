use core::fmt;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::NonNull;

use crate::generated::{dispatch_release, dispatch_retain};
use crate::DispatchObject;

// Symlinked to `objc2/src/rc/retained_forwarding_impls.rs`, Cargo will make
// a copy when publishing.
mod forwarding_impls;
// Allow the `use super::Retained;` in `forwarding_impls` to work.
use DispatchRetained as Retained;

/// A reference counted pointer type for Dispatch objects.
///
/// [`DispatchRetained`] strongly references or "retains" the given object
/// `T`, and decrements the retain count or "releases" it again when dropped,
/// thereby ensuring it will be deallocated at the right time.
///
/// The type `T` inside `DispatchRetained<T>` can be anything that implements
/// [`DispatchObject`], i.e. any Dispatch object.
///
///
/// # Comparison to other types
///
/// `DispatchRetained<T>` is equivalent to [`objc2::rc::Retained`], and can be
/// converted to/from that when the `"objc2"` feature is enabled. Note though
/// that this type uses the underlying Dispatch primitives `dispatch_retain` /
/// `dispatch_release` instead of `objc_retain` / `objc_release` for
/// performance, and to avoid depending on the Objective-C runtime if not
/// needed.
///
/// You can also view `DispatchRetained<T>` as the Dispatch equivalent of
/// [`std::sync::Arc`], that is, it is a thread-safe reference-counting smart
/// pointer that allows cloning by bumping the reference count.
///
/// Unlike `Arc`, objects can be retained directly from a `&T` using
/// [`DispatchObject::retain`] (for `Arc` you need `&Arc<T>`).
///
/// Weak references are not supported, for that you need to convert to
/// `objc2::rc::Retained`.
///
#[cfg_attr(
    not(feature = "objc2"),
    doc = "[`objc2::rc::Retained`]: #objc2-not-available"
)]
#[cfg_attr(not(feature = "std"), doc = "[`std::sync::Arc`]: #std-not-enabled")]
///
///
/// # Forwarding implementations
///
/// Since `DispatchRetained<T>` is a smart pointer, it [`Deref`]s to `T`.
///
/// It also forwards the implementation of a bunch of standard library traits
/// such as [`PartialEq`], [`AsRef`], and so on, so that it becomes possible
/// to use e.g. `DispatchRetained<DispatchQueue>` as-if it you had a
/// `&DispatchQueue`. Note that having a `DispatchQueue` directly is not
/// possible since dispatch objects cannot live on the stack, but instead must
/// reside on the heap, and as such must be accessed behind a pointer or a
/// reference.
///
///
/// # Memory layout
///
/// This is guaranteed to have the same size and alignment as a pointer to the
/// object, i.e. same as `*const T` or `dispatch_object_t`.
///
/// Additionally, it participates in the null-pointer optimization, that is,
/// `Option<DispatchRetained<T>>` is guaranteed to have the same size as
/// `DispatchRetained<T>`.
#[repr(transparent)]
#[doc(alias = "Retained")]
#[doc(alias = "objc2::rc::Retained")]
// TODO: Add `ptr::Thin` bound on `T` to allow for only extern types
pub struct DispatchRetained<T: ?Sized> {
    /// A pointer to the contained object. The pointer is always retained.
    ///
    /// It is important that this is `NonNull`, since we want to dereference
    /// it later, and be able to use the null-pointer optimization.
    ptr: NonNull<T>,
    /// Necessary for dropck even though we never actually run T's destructor,
    /// because it might have a `dealloc` that assumes that contained
    /// references outlive the type.
    ///
    /// See <https://doc.rust-lang.org/nightly/nomicon/phantom-data.html>
    item: PhantomData<T>,
    /// Marks the type as !UnwindSafe. Later on we'll re-enable this.
    ///
    /// See <https://github.com/rust-lang/rust/issues/93367> for why this is
    /// required.
    notunwindsafe: PhantomData<&'static mut ()>,
}

// Same as `objc::rc::Retained`, `#[may_dangle]` does not apply here.
impl<T: ?Sized> Drop for DispatchRetained<T> {
    /// Releases the contained object.
    #[doc(alias = "dispatch_release")]
    #[doc(alias = "release")]
    #[inline]
    fn drop(&mut self) {
        // SAFETY: The `ptr` is guaranteed to be valid and have at least one
        // retain count.
        unsafe { dispatch_release(self.ptr.cast()) };
    }
}

impl<T: ?Sized + DispatchObject> DispatchRetained<T> {
    /// Construct a `DispatchRetained` from a pointer that already has +1
    /// retain count.
    ///
    /// This is useful when you have been given a pointer to an object from
    /// some API that returns a retained pointer, and expects you to release
    /// it. That is, an API that follows [the create rule].
    ///
    /// [the create rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-103029
    ///
    ///
    /// # Safety
    ///
    /// The pointer must be a valid and live object according to Dispatch, and
    /// it must be of type `T`.
    ///
    /// Additionally, you must ensure the given object pointer has +1 retain
    /// count.
    #[inline]
    pub unsafe fn from_raw(ptr: NonNull<T>) -> Self {
        Self {
            ptr,
            item: PhantomData,
            notunwindsafe: PhantomData,
        }
    }

    /// Retain the pointer and construct a [`DispatchRetained`] from it.
    ///
    /// This is useful when you have been given a pointer to an object from
    /// some API that follows [the get rule], and you would like to keep it
    /// around for longer than the current memory context.
    ///
    /// [the get rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-SW1
    ///
    ///
    /// # Safety
    ///
    /// The pointer must be a valid and live object according to Dispatch, and
    /// it must be of type `T`.
    #[doc(alias = "dispatch_retain")]
    #[inline]
    pub unsafe fn retain(ptr: NonNull<T>) -> Self {
        // SAFETY: The caller upholds that the pointer is valid.
        unsafe { dispatch_retain(ptr.cast()) };

        // SAFETY: We just retained the object, so it has +1 retain count.
        // Validity of the pointer is upheld by the caller.
        unsafe { Self::from_raw(ptr) }
    }

    /// Consumes the `DispatchRetained`, returning a raw pointer with +1
    /// retain count.
    ///
    /// After calling this function, the caller is responsible for the memory
    /// previously managed by the `DispatchRetained`.
    ///
    /// This is effectively the opposite of [`DispatchRetained::from_raw`].
    ///
    /// This is an associated method, and must be called as
    /// `DispatchRetained::into_raw(obj)`.
    #[inline]
    pub fn into_raw(this: Self) -> NonNull<T> {
        ManuallyDrop::new(this).ptr
    }

    /// Returns a raw pointer to the object.
    ///
    /// The pointer is valid for at least as long as the `DispatchRetained` is
    /// held.
    ///
    /// This is an associated method, and must be called as
    /// `DispatchRetained::as_ptr(&obj)`.
    #[inline]
    pub fn as_ptr(this: &Self) -> NonNull<T> {
        this.ptr
    }

    /// Unchecked conversion to another Dispatch object.
    ///
    /// This is equivalent to a `cast` between two pointers.
    ///
    /// This is an associated method, and must be called as
    /// `DispatchRetained::cast_unchecked(obj)`.
    ///
    ///
    /// # Safety
    ///
    /// You must ensure that the object can be reinterpreted as the given
    /// type.
    ///
    /// If `T` is not `'static`, you must ensure that `U` ensures that the
    /// data contained by `T` is kept alive for as long as `U` lives.
    ///
    /// Additionally, you must ensure that any safety invariants that the new
    /// type has are upheld.
    #[inline]
    // TODO: Add ?Sized bound
    pub unsafe fn cast_unchecked<U: DispatchObject>(this: Self) -> DispatchRetained<U> {
        // SAFETY: The object is forgotten, so we have +1 retain count.
        //
        // Caller verifies that the object is of the correct type.
        unsafe { DispatchRetained::from_raw(Self::into_raw(this).cast()) }
    }
}

impl<T: ?Sized + DispatchObject> Clone for DispatchRetained<T> {
    /// Retain the object, increasing its reference count.
    ///
    /// This calls [`DispatchObject::retain`] internally.
    #[doc(alias = "dispatch_retain")]
    #[doc(alias = "retain")]
    #[inline]
    fn clone(&self) -> Self {
        self.retain()
    }
}

impl<T: ?Sized> Deref for DispatchRetained<T> {
    type Target = T;

    /// Obtain a reference to the object.
    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: The pointer's validity is verified when the type is
        // created.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> fmt::Pointer for DispatchRetained<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr.as_ptr(), f)
    }
}

// Same as what's implemented for `objc2::rc::Retained`.
impl<T: ?Sized + AsRef<U>, U: DispatchObject> From<&T> for DispatchRetained<U> {
    /// Cast the object to a superclass, and retain it.
    #[inline]
    fn from(obj: &T) -> Self {
        obj.as_ref().retain()
    }
}

#[cfg(feature = "objc2")]
impl<T: ?Sized + DispatchObject + objc2::Message> From<objc2::rc::Retained<T>>
    for DispatchRetained<T>
{
    /// Convert a [`objc2::rc::Retained`] into a [`DispatchRetained`].
    ///
    /// This only works if the type is a Dispatch object (implements the
    /// [`DispatchObject`] trait).
    ///
    /// This conversion is cost-free.
    #[inline]
    fn from(obj: objc2::rc::Retained<T>) -> Self {
        let ptr = objc2::rc::Retained::into_raw(obj);
        let ptr = NonNull::new(ptr).unwrap();
        // SAFETY: `T` is bound by `DispatchObject`, so we know that the type
        // is a Dispatch object, and hence we know that it will respond to
        // `dispatch_retain`/`dispatch_release`.
        //
        // Additionally, the pointer is valid and has +1 retain count, since
        // we're passing it from `Retained::into_raw`.
        unsafe { Self::from_raw(ptr) }
    }
}

#[cfg(feature = "objc2")]
impl<T: ?Sized + DispatchObject + objc2::Message> From<DispatchRetained<T>>
    for objc2::rc::Retained<T>
{
    /// Convert a [`DispatchRetained`] into a [`objc2::rc::Retained`].
    ///
    /// This conversion is cost-free, since Dispatch objects are fully
    /// interoperable with Objective-C retain/release message sending.
    #[inline]
    fn from(obj: DispatchRetained<T>) -> Self {
        let ptr = DispatchRetained::into_raw(obj);
        // SAFETY: `T` is bound by `Message`, so we know that the type is an
        // Objective-C object, and hence we know that it will respond to
        // `objc_retain`, `objc_release` etc.
        //
        // Additionally, the pointer is valid and has +1 retain count, since
        // we're passing it from `DispatchRetained::into_raw`.
        unsafe { Self::from_raw(ptr.as_ptr()) }.unwrap()
    }
}

/// `DispatchRetained<T>` is `Send` if `T` is `Send + Sync`.
//
// SAFETY: dispatch_retain/dispatch_release is thread safe, rest is the same
// as `std::sync::Arc` and `objc2::rc::Retained`.
unsafe impl<T: ?Sized + Sync + Send> Send for DispatchRetained<T> {}

/// `DispatchRetained<T>` is `Sync` if `T` is `Send + Sync`.
//
// SAFETY: dispatch_retain/dispatch_release is thread safe, rest is the same
// as `std::sync::Arc` and `objc2::rc::Retained`.
unsafe impl<T: ?Sized + Sync + Send> Sync for DispatchRetained<T> {}

// Same as `std::sync::Arc` and `objc2::rc::Retained`.
impl<T: ?Sized> Unpin for DispatchRetained<T> {}

// Same as `std::sync::Arc` and `objc2::rc::Retained`.
impl<T: ?Sized + RefUnwindSafe> RefUnwindSafe for DispatchRetained<T> {}

// Same as `std::sync::Arc` and `objc2::rc::Retained`.
impl<T: ?Sized + RefUnwindSafe> UnwindSafe for DispatchRetained<T> {}

use core::ffi::c_void;
use core::fmt;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::NonNull;

use crate::{CFType, CFTypeID, ConcreteType, Type};

// Symlinked to `objc2/src/rc/retained_forwarding_impls.rs`, Cargo will make
// a copy when publishing.
mod forwarding_impls;
// Allow the `use super::Retained;` in `forwarding_impls` to work.
use CFRetained as Retained;

/// A reference counted pointer type for CoreFoundation types.
///
/// [`CFRetained`] strongly references or "retains" the given object `T`, and
/// decrements the retain count or "releases" it again when dropped, thereby
/// ensuring it will be deallocated at the right time.
///
/// The type `T` inside `CFRetained<T>` can be anything that implements
/// [`Type`], i.e. any CoreFoundation-like type.
///
///
/// # Comparison to other types
///
/// `CFRetained<T>` is equivalent to [`objc2::rc::Retained`], and can be
/// converted to/from that when the `"objc2"` feature is enabled. Note though
/// that this uses the underlying CoreFoundation primitives `CFRetain` /
/// `CFRelease` / `CFAutorelease` instead of `objc_retain` / `objc_release` /
/// `objc_autorelease`, to avoid depending on the Objective-C runtime if not
/// needed.
///
/// You can also view `CFRetained<T>` as the CoreFoundation equivalent of
/// [`std::sync::Arc`], that is, it is a thread-safe reference-counting smart
/// pointer that allows cloning by bumping the reference count.
///
/// Unlike `Arc`, objects can be retained directly from a `&T` using
/// [`Type::retain`] (for `Arc` you need `&Arc<T>`).
///
/// Weak references are not supported though without the Objective-C runtime.
///
#[cfg_attr(
    not(feature = "objc2"),
    doc = "[`objc2::rc::Retained`]: #objc2-not-available"
)]
///
///
/// # Forwarding implementations
///
/// Since `CFRetained<T>` is a smart pointer, it [`Deref`]s to `T`.
///
/// It also forwards the implementation of a bunch of standard library traits
/// such as [`PartialEq`], [`AsRef`], and so on, so that it becomes possible
/// to use e.g. `CFRetained<CFString>` as-if it was `CFString`. Note that
/// having a `CFString` directly is not possible since CoreFoundation objects
/// cannot live on the stack, but instead must reside on the heap, and as such
/// must be accessed behind a pointer or a reference (i.e. `&CFString`).
///
///
/// # Memory layout
///
/// This is guaranteed to have the same size and alignment as a pointer to the
/// object, `*const T`.
///
/// Additionally, it participates in the null-pointer optimization, that is,
/// `Option<CFRetained<T>>` is guaranteed to have the same size as
/// `CFRetained<T>`.
#[repr(transparent)]
#[doc(alias = "id")]
#[doc(alias = "Retained")]
#[doc(alias = "objc2::rc::Retained")]
#[cfg_attr(
    feature = "unstable-coerce-pointee",
    derive(std::marker::CoercePointee)
)]
// TODO: Add `ptr::Thin` bound on `T` to allow for only extern types
pub struct CFRetained<T: ?Sized> {
    /// A pointer to the contained type. The pointer is always retained.
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

impl<T: ?Sized + Type> CFRetained<T> {
    /// Construct a `CFRetained` from a pointer that already has +1 retain
    /// count.
    ///
    /// This is useful when you have been given a pointer to a type from some
    /// API that [returns a retained pointer][c-retain] (i.e. follows
    /// [the Create rule]).
    ///
    /// [c-retain]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#auditing-of-c-retainable-pointer-interfaces
    /// [the Create rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-103029
    ///
    ///
    /// # Safety
    ///
    /// You must uphold the same requirements as described in
    /// [`CFRetained::retain`].
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

    /// Consumes the `CFRetained`, returning a raw pointer with +1 retain
    /// count.
    ///
    /// After calling this function, the caller is responsible for the memory
    /// previously managed by the `CFRetained`.
    ///
    /// This is effectively the opposite of [`CFRetained::from_raw`], see that
    /// for more details on when this function is useful.
    ///
    /// This is an associated method, and must be called as
    /// `CFRetained::into_raw(obj)`.
    #[inline]
    pub fn into_raw(this: Self) -> NonNull<T> {
        ManuallyDrop::new(this).ptr
    }

    /// Returns a raw pointer to the type.
    ///
    /// The pointer is valid for at least as long as the `CFRetained` is held.
    ///
    /// This is an associated method, and must be called as
    /// `CFRetained::as_ptr(&obj)`.
    #[inline]
    pub fn as_ptr(this: &Self) -> NonNull<T> {
        this.ptr
    }

    /// Unchecked conversion to another CoreFoundation type.
    ///
    /// This is equivalent to an `unsafe` `cast` between two pointers, see
    /// [`CFRetained::downcast`] for a safe alternative.
    ///
    /// This is an associated method, and must be called as
    /// `CFRetained::cast_unchecked(obj)`.
    ///
    ///
    /// # Safety
    ///
    /// You must ensure that the type can be reinterpreted as the given type.
    ///
    /// If `T` is not `'static`, you must ensure that `U` ensures that the
    /// data contained by `T` is kept alive for as long as `U` lives.
    ///
    /// Additionally, you must ensure that any safety invariants that the new
    /// type has are upheld.
    #[inline]
    // TODO: Add ?Sized bound
    pub unsafe fn cast_unchecked<U: Type>(this: Self) -> CFRetained<U> {
        let ptr = ManuallyDrop::new(this).ptr.cast();
        // SAFETY: The type is forgotten, so we have +1 retain count.
        //
        // Caller verifies that the returned type is of the correct type.
        unsafe { CFRetained::from_raw(ptr) }
    }
}

// TODO: Add ?Sized bound
impl<T: Type> CFRetained<T> {
    /// Attempt to downcast the type to that of type `U`.
    ///
    /// This is the owned variant. Use [`CFType::downcast_ref`] if you want to
    /// convert a reference type. See also [`ConcreteType`] for more details
    /// on which types support being converted to.
    ///
    /// See [`CFType::downcast_ref`] for more details.
    ///
    /// [`CFType::downcast_ref`]: crate::CFType::downcast_ref
    ///
    /// # Errors
    ///
    /// If casting failed, this will return the original back as the [`Err`]
    /// variant. If you do not care about this, and just want an [`Option`],
    /// use `.downcast().ok()`.
    //
    // NOTE: This is _not_ an associated method, since we want it to be easy
    // to call, and it does not conflict with `CFType::downcast_ref`.
    #[doc(alias = "CFGetTypeID")]
    pub fn downcast<U: ConcreteType>(self) -> Result<CFRetained<U>, Self>
    where
        T: 'static,
    {
        extern "C-unwind" {
            // `*const c_void` and `Option<&CFType>` are ABI compatible.
            #[allow(clashing_extern_declarations)]
            fn CFGetTypeID(cf: *const c_void) -> CFTypeID;
        }

        let ptr: *const c_void = self.ptr.as_ptr().cast();

        // SAFETY: The pointer is valid.
        if unsafe { CFGetTypeID(ptr) } == U::type_id() {
            // SAFETY: Just checked that the object is a class of type `U`,
            // and `T` is `'static`. Additionally, `ConcreteType::type_id` is
            // guaranteed to uniquely identify the class (including ruling out
            // mutable subclasses), so we know for _sure_ that the class is
            // actually of that type here.
            Ok(unsafe { Self::cast_unchecked::<U>(self) })
        } else {
            Err(self)
        }
    }

    /// Retain the pointer and construct a [`CFRetained`] from it.
    ///
    /// This is useful when you have been given a pointer to a type from some
    /// API that [returns a non-retained reference][c-retain] (i.e. follows
    /// [the Get rule]).
    ///
    /// See also [`Type::retain`] for a safe alternative when you already have
    /// a reference to the type.
    ///
    /// [c-retain]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#auditing-of-c-retainable-pointer-interfaces
    /// [the Get rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-SW1
    ///
    ///
    /// # Safety
    ///
    /// The pointer must be valid as a reference (aligned, dereferenceable and
    /// initialized, see the [`std::ptr`] module for more information).
    ///
    /// You must ensure that if `T` is non-`'static` (i.e. has a lifetime
    /// parameter), that any data that `T` may reference lives for at least as
    /// long as the return value.
    ///
    /// [`std::ptr`]: core::ptr
    #[doc(alias = "CFRetain")]
    #[inline]
    pub unsafe fn retain(ptr: NonNull<T>) -> Self {
        extern "C-unwind" {
            fn CFRetain(cf: *mut c_void) -> *mut c_void;
        }

        // SAFETY: The caller upholds that the pointer is valid.
        //
        // Note that `CFRetain` will abort if given a NULL pointer, but we
        // avoid that here by ensuring that the pointer is non-NULL.
        let res: *mut T = unsafe { CFRetain(ptr.as_ptr().cast()) }.cast();

        // SAFETY: The pointer returned from `CFRetain` is the same as the one
        // given, and we gave it a non-NULL pointer.
        let res = unsafe { NonNull::new_unchecked(res) };

        // SAFETY: We just retained the type, so it has +1 retain count.
        //
        // The pointer returned from `CFRetain` is the same as the one given,
        // and the validity of it is upheld by the caller.
        unsafe { Self::from_raw(res) }
    }

    /// Autoreleases the `CFRetained`, returning a pointer.
    ///
    /// The object is not immediately released, but will be when the innermost
    /// / current autorelease pool is drained.
    ///
    /// This is an associated method, and must be called as
    /// `CFRetained::autorelease_ptr(obj)`.
    ///
    /// # Safety
    ///
    /// This method is safe to call, but the returned pointer is only
    /// guaranteed to be valid until the innermost autorelease pool is
    /// drained.
    #[doc(alias = "CFAutorelease")]
    #[must_use = "if you don't intend to use the object any more, drop it as usual instead"]
    #[inline]
    pub fn autorelease_ptr(this: Self) -> *mut T {
        extern "C-unwind" {
            fn CFAutorelease(cf: *mut c_void) -> *mut c_void;
        }

        let ptr = Self::into_raw(this);
        // SAFETY: The `ptr` is valid and has +1 retain count.
        unsafe { CFAutorelease(ptr.as_ptr().cast()) }.cast()
    }
}

// TODO: Add ?Sized bound
impl<T: Type> Clone for CFRetained<T> {
    /// Retain the type, increasing its reference count.
    ///
    /// This calls [`Type::retain`] internally.
    #[doc(alias = "CFRetain")]
    #[doc(alias = "retain")]
    #[inline]
    fn clone(&self) -> Self {
        self.retain()
    }
}

// Same as `objc::rc::Retained`, `#[may_dangle]` does not apply here.
impl<T: ?Sized> Drop for CFRetained<T> {
    /// Releases the contained type.
    #[doc(alias = "CFRelease")]
    #[doc(alias = "release")]
    #[inline]
    fn drop(&mut self) {
        extern "C-unwind" {
            fn CFRelease(cf: *mut c_void);
        }

        // SAFETY: The `ptr` is guaranteed to be valid, and have at least one
        // retain count.
        //
        // Note that `CFRelease` will abort if given a NULL pointer, but we
        // avoid that here by ensuring that the pointer is non-NULL.
        unsafe { CFRelease(self.ptr.as_ptr().cast()) };
    }
}

impl<T: ?Sized> Deref for CFRetained<T> {
    type Target = T;

    /// Obtain a reference to the type.
    // Box doesn't inline, but that's because it's a compiler built-in
    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: The pointer's validity is verified when the type is
        // created.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> fmt::Pointer for CFRetained<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr.as_ptr(), f)
    }
}

// Same as what's implemented for `objc2::rc::Retained`.
impl<T: ?Sized + AsRef<U>, U: Type> From<&T> for CFRetained<U> {
    /// Cast the type to a superclass or `CFType`, and retain it.
    #[inline]
    fn from(obj: &T) -> Self {
        obj.as_ref().retain()
    }
}

// Use `ConcreteType` to avoid the reflexive impl (as CFType does not implement that).
impl<T: ?Sized + ConcreteType + 'static> From<CFRetained<T>> for CFRetained<CFType> {
    /// Convert to [`CFType`].
    #[inline]
    fn from(obj: CFRetained<T>) -> Self {
        // SAFETY: All `'static` types can be converted to `CFType` without
        // loss of information.
        unsafe { CFRetained::cast_unchecked(obj) }
    }
}

#[cfg(feature = "objc2")]
impl<T: ?Sized + Type + objc2::Message> From<objc2::rc::Retained<T>> for CFRetained<T> {
    /// Convert a [`objc2::rc::Retained`] into a [`CFRetained`].
    ///
    /// This only works if the type is a CoreFoundation type (implements the
    /// [`Type`] trait).
    ///
    /// This conversion is cost-free.
    #[inline]
    fn from(obj: objc2::rc::Retained<T>) -> Self {
        let ptr = objc2::rc::Retained::into_raw(obj);
        let ptr = NonNull::new(ptr).unwrap();
        // SAFETY: `T` is bound by `Type`, so we know that the type is a
        // CoreFoundation-like type, and hence we know that it will respond to
        // `CFRetain`/`CFRelease`.
        //
        // Additionally, the pointer is valid and has +1 retain count, since
        // we're passing it from `Retained::into_raw`.
        unsafe { Self::from_raw(ptr) }
    }
}

#[cfg(feature = "objc2")]
impl<T: ?Sized + objc2::Message> From<CFRetained<T>> for objc2::rc::Retained<T> {
    /// Convert a [`CFRetained`] into a [`objc2::rc::Retained`].
    ///
    /// This conversion is cost-free, since CoreFoundation types are fully
    /// interoperable with Objective-C retain/release message sending.
    #[inline]
    fn from(obj: CFRetained<T>) -> Self {
        let ptr = ManuallyDrop::new(obj).ptr;
        // SAFETY: `T` is bound by `Message`, so we know that the type is an
        // Objective-C object, and hence we know that it will respond to
        // `objc_retain`, `objc_release` etc.
        //
        // Additionally, the pointer is valid and has +1 retain count, since
        // we're passing it from `CFRetained::into_raw`.
        unsafe { Self::from_raw(ptr.as_ptr()) }.unwrap()
    }
}

/// `CFRetained<T>` is `Send` if `T` is `Send + Sync`.
//
// SAFETY: CFRetain/CFRelease is thread safe, rest is the same as
// `std::sync::Arc` and `objc2::rc::Retained`.
unsafe impl<T: ?Sized + Sync + Send> Send for CFRetained<T> {}

/// `CFRetained<T>` is `Sync` if `T` is `Send + Sync`.
//
// SAFETY: CFRetain/CFRelease is thread safe, rest is the same as
// `std::sync::Arc` and `objc2::rc::Retained`.
unsafe impl<T: ?Sized + Sync + Send> Sync for CFRetained<T> {}

// Same as `std::sync::Arc` and `objc2::rc::Retained`.
impl<T: ?Sized> Unpin for CFRetained<T> {}

// Same as `std::sync::Arc` and `objc2::rc::Retained`.
impl<T: ?Sized + RefUnwindSafe> RefUnwindSafe for CFRetained<T> {}

// Same as `std::sync::Arc` and `objc2::rc::Retained`.
impl<T: ?Sized + RefUnwindSafe> UnwindSafe for CFRetained<T> {}

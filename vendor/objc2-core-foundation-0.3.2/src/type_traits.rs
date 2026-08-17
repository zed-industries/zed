use core::ptr::NonNull;

use crate::{CFRetained, CFType, CFTypeID};

/// A CoreFoundation-like type.
///
/// This trait is implemented for all CoreFoundation-like types (i.e. both
/// types in this crate like `CFString`, but also types from other frameworks
/// like `CGColor` from CoreGraphics).
///
/// This trait allows the type to be used in [`CFRetained`].
///
/// You should not need to implement this yourself.
///
///
/// # Safety
///
/// - The type must be a CoreFoundation-like type.
/// - The type must be safe to retain/release using `CFRetain` and
///   `CFRelease`.
pub unsafe trait Type {
    /// Increment the reference count of the receiver.
    ///
    /// This extends the duration in which the receiver is alive by detaching
    /// it from the lifetime information carried by the reference.
    ///
    /// This is similar to using [`Clone` on `CFRetained<Self>`][clone-id],
    /// with the addition that it can be used on a plain reference.
    ///
    /// [clone-id]: crate::CFRetained#impl-Clone-for-CFRetained<T>
    #[inline]
    #[doc(alias = "CFRetain")]
    fn retain(&self) -> CFRetained<Self>
    where
        Self: Sized,
    {
        let ptr = NonNull::from(self);

        // SAFETY: The pointer is valid, since it came from a Rust reference.
        unsafe { CFRetained::retain(ptr) }
    }

    /// Helper for easier transition from the `core-foundation` crate.
    #[deprecated = "this is redundant"]
    #[inline]
    #[allow(non_snake_case)]
    fn as_concrete_TypeRef(&self) -> &Self {
        self
    }

    /// Helper for easier transition from the `core-foundation` crate.
    ///
    /// # Safety
    ///
    /// Same as [`CFRetained::retain`].
    #[deprecated = "use CFRetained::retain"]
    #[inline]
    unsafe fn wrap_under_get_rule(ptr: *const Self) -> CFRetained<Self>
    where
        Self: Sized,
    {
        let ptr = NonNull::new(ptr.cast_mut()).expect("attempted to create a NULL object");
        // SAFETY: Upheld by caller.
        unsafe { CFRetained::retain(ptr) }
    }

    /// Helper for easier transition from the `core-foundation` crate.
    #[deprecated = "this is redundant (CF types deref to CFType)"]
    #[inline]
    #[allow(non_snake_case)]
    fn as_CFTypeRef(&self) -> &CFType
    where
        Self: AsRef<CFType>,
    {
        self.as_ref()
    }

    /// Helper for easier transition from the `core-foundation` crate.
    ///
    /// # Safety
    ///
    /// Same as [`CFRetained::from_raw`].
    #[deprecated = "use CFRetained::from_raw"]
    #[inline]
    unsafe fn wrap_under_create_rule(ptr: *const Self) -> CFRetained<Self>
    where
        Self: Sized,
    {
        let ptr = NonNull::new(ptr.cast_mut()).expect("attempted to create a NULL object");
        // SAFETY: Upheld by caller.
        unsafe { CFRetained::from_raw(ptr) }
    }
}

/// A concrete CoreFoundation type.
///
/// This trait is implemented for CoreFoundation types which have a
/// `CFTypeID`, which should be most types except for mutable variants, types
/// with generics, as well as the root `CFType`.
///
///
/// # Mutable types
///
/// Some types have immutable and mutable variants, the prime example being
/// `CFString` and `CFMutableString`. Internally, these are very complex class
/// clusters, but in the simplest case they're sometimes the same type, the
/// only difference being that the mutable variant has a boolean flag set.
/// This means that they also share the same type ID, and thus we cannot
/// (stably) differentiate between them at runtime.
///
/// Therefore, this trait is only implemented for the immutable variant, to
/// prevent it from accidentally being misused (it is unclear whether it would
/// be unsound or not). If you're looking to convert to a mutable type, you'll
/// have to either construct a new one with APIs like
/// `CFStringCreateMutableCopy`, or use an unchecked cast.
///
///
/// # Generic types
///
/// Only the "base" type / the type without generics implements this.
///
/// So for example, `CFArray` implements this trait, but `CFArray<CFString>`
/// does not (as we cannot query the type of the inner element without
/// actually inspecting the element, and since `CFArray`s aren't required to
/// hold CF types).
///
///
/// # Safety
///
/// - The type must not be mutable.
/// - The type must not be generic (e.g. this can only be implemented for the
///   base `CFArray`, not for all `CFArray<T>`).
/// - The [`type_id`][Self::type_id] must be implemented correctly, and must
///   uniquely identify the type.
pub unsafe trait ConcreteType: Type {
    /// Get the unique `CFTypeID` identifier for the type.
    ///
    /// For example, this corresponds to `CFStringGetTypeID` for `CFString`
    /// and `CGColorGetTypeID` for `CGColor`.
    #[doc(alias = "GetTypeID")]
    fn type_id() -> CFTypeID;
}

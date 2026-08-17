//! Internal crate used by foreign-types

#![no_std]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/foreign-types-shared/0.3")]

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::mem;

/// An opaque type used to define `ForeignTypeRef` types.
///
/// A type implementing `ForeignTypeRef` should simply be a newtype wrapper around this type.
pub struct Opaque(PhantomData<UnsafeCell<*mut ()>>);

/// A type implemented by wrappers over foreign types.
///
/// # Safety
///
/// Implementations of `ForeignType` must guarantee the following:
/// - `Self::from_ptr(x).as_ptr() == x`
/// - `Self::from_ptr(x).into_ptr(x) == x`
/// - `Self::from_ptr(x).deref().as_ptr(x) == x`
/// - `Self::from_ptr(x).deref_mut().as_ptr(x) == x`
/// - `Self::from_ptr(x).as_ref().as_ptr(x) == x`
/// - `Self::from_ptr(x).as_mut().as_ptr(x) == x`
pub unsafe trait ForeignType: Sized {
    /// The raw C type.
    type CType;

    /// The type representing a reference to this type.
    type Ref: ForeignTypeRef<CType = Self::CType>;

    /// Constructs an instance of this type from its raw type.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid, owned instance of the native type.
    unsafe fn from_ptr(ptr: *mut Self::CType) -> Self;

    /// Returns a raw pointer to the wrapped value.
    fn as_ptr(&self) -> *mut Self::CType;

    /// Consumes the wrapper and returns the raw pointer.
    #[inline]
    fn into_ptr(self) -> *mut Self::CType {
        let ptr = self.as_ptr();
        mem::forget(self);
        ptr
    }
}

/// A trait implemented by types which reference borrowed foreign types.
///
/// # Safety
///
/// Implementations of `ForeignTypeRef` must guarantee the following:
///
/// - `Self::from_ptr(x).as_ptr() == x`
/// - `Self::from_mut_ptr(x).as_ptr() == x`
pub unsafe trait ForeignTypeRef: Sized {
    /// The raw C type.
    type CType;

    /// Constructs a shared instance of this type from its raw type.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid, immutable, instance of the type for the `'a` lifetime.
    #[inline]
    unsafe fn from_ptr<'a>(ptr: *mut Self::CType) -> &'a Self {
        debug_assert!(!ptr.is_null());
        &*(ptr as *mut _)
    }

    /// Constructs a mutable reference of this type from its raw type.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid, unique, instance of the type for the `'a` lifetime.
    #[inline]
    unsafe fn from_ptr_mut<'a>(ptr: *mut Self::CType) -> &'a mut Self {
        debug_assert!(!ptr.is_null());
        &mut *(ptr as *mut _)
    }

    /// Returns a raw pointer to the wrapped value.
    #[inline]
    fn as_ptr(&self) -> *mut Self::CType {
        self as *const _ as *mut _
    }
}

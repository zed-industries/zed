// The header `CoreFoundation/CFBase.h` contains:
//
// #if defined(__WIN64__) && !defined(__LLP64__)
// #define __LLP64__ 1
// #endif
//
// #if __LLP64__
// typedef unsigned long long CFTypeID;
// typedef unsigned long long CFOptionFlags;
// typedef unsigned long long CFHashCode;
// typedef signed long long CFIndex;
// #else
// typedef unsigned long CFTypeID;
// typedef unsigned long CFOptionFlags;
// typedef unsigned long CFHashCode;
// typedef signed long CFIndex;
// #endif
//
// Looking at the corresponding Rust definitions for longs:
// <https://doc.rust-lang.org/1.83.0/src/core/ffi/mod.rs.html#168-179>
// cfg_if! {
//     if #[cfg(all(target_pointer_width = "64", not(windows)))] {
//         pub type c_long = i64;
//         pub type c_ulong = u64;
//     } else {
//         // The minimal size of `long` in the C standard is 32 bits
//         pub type c_long = i32;
//         pub type c_ulong = u32;
//     }
// }
// <https://doc.rust-lang.org/1.83.0/src/core/ffi/mod.rs.html#65-66>
// pub type c_longlong = i64;
// pub type c_ulonglong = u64;
//
// It becomes easy to convince ourselves that combined, these amount to making
// these types be 32-bit on systems with 32-bit pointers and 64-bit on systems
// with 64-bit pointers.
//
// That means we can use `isize`/`usize`, which is more ergonomic.

use core::cell::UnsafeCell;
use core::cmp::Ordering;
use core::convert::AsRef;
use core::fmt;
use core::hash;
use core::marker::{PhantomData, PhantomPinned};

use crate::{
    CFComparisonResult, CFEqual, CFGetRetainCount, CFGetTypeID, CFHash, CFRange, ConcreteType, Type,
};

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cftypeid?language=objc)
pub type CFTypeID = usize;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfoptionflags?language=objc)
pub type CFOptionFlags = usize;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfhashcode?language=objc)
pub type CFHashCode = usize;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfindex?language=objc)
pub type CFIndex = isize;

// Manually define CFType

/// An instance of a Core Foundation type.
///
/// This is meant to be used behind a reference. In the future, this will be
/// defined as an [`extern type`][RFC-1861].
///
/// All Core Foundation types [`Deref`](std::ops::Deref) to this type (it can
/// be considered the "root" type).
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cftype?language=objc).
///
/// [RFC-1861]: https://rust-lang.github.io/rfcs/1861-extern-types.html
#[repr(C)]
pub struct CFType {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

impl CFType {
    /// Attempt to downcast the type to that of type `T`.
    ///
    /// This is the reference-variant. Use [`CFRetained::downcast`] if you
    /// want to convert a retained type. See also [`ConcreteType`] for more
    /// details on which types support being converted to.
    ///
    /// [`CFRetained::downcast`]: crate::CFRetained::downcast
    //
    // Not #[inline], we call two functions here.
    #[doc(alias = "CFGetTypeID")]
    pub fn downcast_ref<T: ConcreteType>(&self) -> Option<&T> {
        if CFGetTypeID(Some(self)) == T::type_id() {
            let ptr: *const Self = self;
            let ptr: *const T = ptr.cast();
            // SAFETY: Just checked that the object is a class of type `T`.
            // Additionally, `ConcreteType::type_id` is guaranteed to uniquely
            // identify the class (including ruling out mutable subclasses),
            // so we know for _sure_ that the class is actually of that type
            // here.
            let this: &T = unsafe { &*ptr };
            Some(this)
        } else {
            None
        }
    }

    /// Get the reference count of the object.
    ///
    /// This function may be useful for debugging. You normally do not use
    /// this function otherwise.
    ///
    /// Beware that some things (like `CFNumber`s, small `CFString`s etc.) may
    /// not have a normal retain count for optimization purposes, and can
    /// return `usize::MAX` in that case.
    #[doc(alias = "CFGetRetainCount")]
    pub fn retain_count(&self) -> usize {
        // Cast is fine, if the reference count is `-1` we want to return
        // `usize::MAX` as a sentinel instead.
        CFGetRetainCount(Some(self)) as _
    }
}

// Reflexive AsRef impl.
impl AsRef<Self> for CFType {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

// SAFETY: CFType represents a CoreFoundation-like type (even though it isn't
// a real type itself).
unsafe impl Type for CFType {}

impl fmt::Debug for CFType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "CFString")]
        {
            let desc = crate::CFCopyDescription(Some(self)).expect("must have description");
            write!(f, "{desc}")
        }
        #[cfg(not(feature = "CFString"))]
        {
            f.debug_struct("<CoreFoundation type (enable CFString feature for more info)>")
                .finish_non_exhaustive()
        }
    }
}

// Equality in CF has approximately the same semantics as Rust equality.
//
// From the docs:
// > Equality is something specific to each Core Foundation opaque type. For
// > example, two CFNumber objects are equal if the numeric values they
// > represent are equal. Two CFString objects are equal if they represent
// > identical sequences of characters, regardless of encoding.
impl PartialEq for CFType {
    #[inline]
    #[doc(alias = "CFEqual")]
    fn eq(&self, other: &Self) -> bool {
        CFEqual(Some(self), Some(other))
    }
}

// Similar to NSObject, most types' equality is reflexive.
impl Eq for CFType {}

// From the documentation for CFHash:
// > Two objects that are equal (as determined by the `CFEqual` function) have
// > the same hashing value. However, the converse is not true: two objects
// > with the same hashing value might not be equal. That is, hashing values
// > are not necessarily unique.
//
// I.e. the same semantics as Rust's `Hash`.
impl hash::Hash for CFType {
    #[doc(alias = "CFHash")]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        CFHash(Some(self)).hash(state);
    }
}

// SAFETY: CFType is defined as the following in the header:
// typedef const CF_BRIDGED_TYPE(id) void * CFTypeRef;
#[cfg(feature = "objc2")]
unsafe impl objc2::encode::RefEncode for CFType {
    const ENCODING_REF: objc2::encode::Encoding =
        objc2::encode::Encoding::Pointer(&objc2::encode::Encoding::Void);
}

// SAFETY: CF types are message-able in the Objective-C runtime.
#[cfg(feature = "objc2")]
unsafe impl objc2::Message for CFType {}

#[cfg(feature = "objc2")]
impl AsRef<objc2::runtime::AnyObject> for CFType {
    fn as_ref(&self) -> &objc2::runtime::AnyObject {
        // SAFETY: CFType is valid to re-interpret as AnyObject.
        unsafe { core::mem::transmute(self) }
    }
}

#[cfg(feature = "objc2")]
impl core::borrow::Borrow<objc2::runtime::AnyObject> for CFType {
    fn borrow(&self) -> &objc2::runtime::AnyObject {
        <Self as AsRef<objc2::runtime::AnyObject>>::as_ref(self)
    }
}

// NOTE: impl AsRef<CFType> for AnyObject would probably not be valid, since
// not all Objective-C objects can be used as CoreFoundation objects (?)

impl Default for CFComparisonResult {
    #[inline]
    fn default() -> Self {
        Self::CompareEqualTo
    }
}

impl From<Ordering> for CFComparisonResult {
    #[inline]
    fn from(order: Ordering) -> Self {
        match order {
            Ordering::Less => Self::CompareLessThan,
            Ordering::Equal => Self::CompareEqualTo,
            Ordering::Greater => Self::CompareGreaterThan,
        }
    }
}

impl From<CFComparisonResult> for Ordering {
    #[inline]
    fn from(comparison_result: CFComparisonResult) -> Self {
        match comparison_result.0 {
            ..=-1 => Self::Less,  // ..=CFComparisonResult::CompareLessThan
            0 => Self::Equal,     // CFComparisonResult::CompareEqualTo
            1.. => Self::Greater, // CFComparisonResult::CompareGreaterThan..
            #[allow(unreachable_patterns)] // MSRV between 1.73 and 1.76
            _ => Self::Equal,
        }
    }
}

impl CFRange {
    /// Create a new [`CFRange`].
    #[doc(alias = "CFRangeMake")]
    pub fn new(location: CFIndex, length: CFIndex) -> Self {
        Self { location, length }
    }
}

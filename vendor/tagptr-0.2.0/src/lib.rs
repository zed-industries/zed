//! Strongly typed pointers with reserved space for storing additional bit
//! patterns within the same memory word.
//!
//! # Motivation
//!
//! In low-level concurrent programming (synchronization primitives,
//! lock-free data structures, etc) it is often required to store additional
//! state information (tags) alongside pointers to objects in memory, since
//! most atomic CPU instructions operate on pointer-wide memory words.
//! The marked pointer types provided by this crate encapsulate the logic and
//! pointer arithmetic for composing (creating), decomposing and mutating
//! such pointers and tag values.
//!
//! # Tag Bits and Type Alignment
//!
//! The possible space for storing tag bits in a pointer is determined by the
//! alignment of the pointed-to type, as long as the pointer is well-aligned
//! (e.g., not packed).
//! For instance, pointers to types with an alignment of 2 (2^1) bytes (e.g.,
//! `u16`) never use the first of their lower bits (i.e., it is always zero),
//! pointers to types with an alignment of 8 (2^3) bytes such as `u64` never
//! use their 3 lowest bits and so on.
//! Great care must be taken at all times to avoid over- or underflows in the
//! usually highly restricted range of valid tags for common tag sizes when
//! doing arithmetic operations.
//! Any operations resulting in tag values outside of their valid range will
//! invariably corrupt the bits representing the pointer and hence invoke
//! undefined behavior when dereferencing these pointers.
//!
//! Constructing a type such as `TagPtr<u64, 4>` is hence usually a user error,
//! since a pointer to a `u64` has only 3 unused bits.
//! The resulting type would consider the first actual bit of the pointer to be
//! part of its tag and return a potentially corrupted pointer in methods such
//! as [`decompose`][TagPtr::decompose].
//! The [`has_sufficient_alignment`] and [`assert_alignment`] functions can be
//! used to explicitly check for or assert this property.
//! There is, however, one exception where using an otherwise ill-formed tag
//! pointer type is valid:
//! After composing a well-formed tag pointer instance (e.g., `TagPtr<u64, 3>`)
//! it is valid to [`cast`][TagPtr::cast] it to a type with a smaller alignment
//! and the same number of tag bits such as `TagPtr<(), 3>` for the purpose of
//! type-erasure.
//!
//! # Example
//!
//! Storing a boolean status flag alongside the pointer to a mutable `u64`:
//!
//! ```
//! type TagPtr = tagptr::TagPtr<u64, 3>;
//!
//! let mut val = 0xCAFE;
//! let is_ok = true;
//!
//! let ptr = TagPtr::compose(&mut val, is_ok as usize);
//! let (reference, tag) = unsafe { ptr.decompose_mut() };
//! assert_eq!(reference, Some(&mut 0xCAFE));
//! assert_eq!(tag == 1, true);
//! ```

#![no_std]

#[cfg(test)]
extern crate std;

#[macro_use]
mod macros;

mod imp {
    mod atomic;
    mod non_null;
    mod ptr;
}

use core::{marker::PhantomData, mem, ptr::NonNull, sync::atomic::AtomicUsize};

// *************************************************************************************************
// AtomicTagPtr (impl in "imp/atomic.rs")
// *************************************************************************************************

/// A raw pointer type which can be safely shared between threads and which can
/// use up to `N` of its lower bits to store additional information (the *tag*).
///
/// This type has the same in-memory representation as a `*mut T`.
/// It is mostly identical to [`AtomicPtr`][atomic], except that all of its
/// methods take or return a [`TagPtr`] instead of `*mut T`.
/// See the [crate][crate] level documentation for restrictions on the value of
/// `N`.
///
/// [atomic]: core::sync::atomic::AtomicPtr
#[repr(transparent)]
pub struct AtomicTagPtr<T, const N: usize> {
    inner: AtomicUsize,
    _marker: PhantomData<*mut T>,
}

// *************************************************************************************************
// TagPtr (impl in "imp/ptr.rs")
// *************************************************************************************************

/// A raw, unsafe pointer type like `*mut T` which can use up to `N` of its
/// lower bits to store additional information (the *tag*).
///
/// This type has the same in-memory representation as a `*mut T`.
/// See the [crate][crate] level documentation for restrictions on the value of
/// `N`.
#[repr(transparent)]
pub struct TagPtr<T, const N: usize> {
    inner: *mut T,
    _marker: PhantomData<()>, // the "fake" marker allows to use the same macro for all pointers
}

// *************************************************************************************************
// TagNonNull (impl in "imp/non_null.rs")
// *************************************************************************************************

/// A non-nullable tagged raw pointer type similar to [`NonNull`] which can use
/// up to `N` of its lower bits to store additional information (the *tag*).
///
/// This type has the same in-memory representation as a `NonNull<T>`.
/// See the [crate][crate] level documentation for restrictions on the value of
/// `N`.
///
/// # Invariants
///
/// This type imposes stricter construction requirements than a regular
/// [`NonNull`], since it requires the pointer to be non-null even after its `N`
/// tag bits are stripped off as well.
/// For instance, the value `0x1` can be used to construct a valid (but not
/// dereferencable) [`NonNull`] since it is not zero, but it can not be used to
/// construct e.g. a valid `TagNonNull<u64, 1>`, since its only non-zero bit
/// would be considered to represent the tag and the value of the pointer would
/// be 0.
/// For valid, well-aligned pointers, this is usually not a concern.
#[repr(transparent)]
pub struct TagNonNull<T, const N: usize> {
    inner: NonNull<T>,
    _marker: PhantomData<()>,
}

// *************************************************************************************************
// Null
// *************************************************************************************************

/// A type representing a `null` pointer with potential tag bits.
///
/// The contained `usize` is the value of the pointer's tag.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Null(pub usize);

/********** impl inherent *************************************************************************/

impl Null {
    /// Returns the tag value.
    #[inline]
    pub fn tag(self) -> usize {
        self.0
    }
}

/********** public functions **********************************************************************/

/// Returns `true` if the alignment of `T` is large enough so a pointer to an
/// instance may store the given number of `tag_bits`.
#[inline]
pub const fn has_sufficient_alignment<T>(tag_bits: usize) -> bool {
    lower_bits::<T>() >= tag_bits
}

/// Asserts that the alignment of `U` is large enough so a pointer to an
/// instance may store `N` tag bits.
///
/// # Panics
///
/// This function panics if the alignment of `U` is insufficient for storing
/// `N` tag bits.
#[inline]
pub fn assert_alignment<T, const N: usize>() {
    assert!(
        has_sufficient_alignment::<T>(N),
        "the respective type has insufficient alignment for storing N tag bits"
    );
}

/********** helper functions **********************************************************************/

/// Composes the given `ptr` with `tag` and returns the composed marked pointer
/// as a raw `*mut T`.
///
/// # Panics
///
/// Panics in *debug builds only* if `ptr` is not well aligned, i.e., if it
/// contains any bits in its lower bits reserved for the tag value.
#[inline(always)]
fn compose<T, const N: usize>(ptr: *mut T, tag: usize) -> *mut T {
    debug_assert_eq!(ptr as usize & mark_mask(N), 0, "tag bits in raw pointer must be zeroed");
    ((ptr as usize) | (mark_mask(N) & tag)) as *mut _
}

/// Decomposes the integer representation of a `ptr` for a given number
/// of `tag_bits` into only a raw pointer stripped of its tag.
#[inline(always)]
const fn decompose_ptr<T>(ptr: usize, tag_bits: usize) -> *mut T {
    (ptr & !mark_mask(tag_bits)) as *mut _
}

/// Decomposes the integer representation of a `ptr` for a given number
/// of `tag_bits` into only a separated tag value.
#[inline(always)]
const fn decompose_tag<T>(ptr: usize, tag_bits: usize) -> usize {
    ptr & mark_mask(tag_bits)
}

/// Returns the (alignment-dependent) number of unused lower bits in a pointer
/// to type `T`.
#[inline(always)]
const fn lower_bits<T>() -> usize {
    mem::align_of::<T>().trailing_zeros() as usize
}

/// Returns the bit-mask for the lower bits containing the tag value.
#[inline(always)]
const fn mark_mask(tag_bits: usize) -> usize {
    (1 << tag_bits) - 1
}

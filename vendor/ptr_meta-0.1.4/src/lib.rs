//! A radioactive stabilization of the [`ptr_meta` RFC][rfc].
//!
//! [rfc]: https://rust-lang.github.io/rfcs/2580-ptr-meta.html
//!
//! ## Usage
//!
//! ### Sized types
//!
//! Sized types already have `Pointee` implemented for them, so most of the time you won't have to worry
//! about them. However, trying to derive `Pointee` for a struct that may or may not have a DST as its
//! last field will cause an implementation conflict with the automatic sized implementation.
//!
//! ### `slice`s and `str`s
//!
//! These core types have implementations built in.
//!
//! ### Structs with a DST as its last field
//!
//! You can derive `Pointee` for last-field DSTs:
//!
//! ```
//! use ptr_meta::Pointee;
//!
//! #[derive(Pointee)]
//! struct Block<H, T> {
//!     header: H,
//!     elements: [T],
//! }
//! ```
//!
//! ### Trait objects
//!
//! You can generate a `Pointee` for trait objects:
//!
//! ```
//! use ptr_meta::pointee;
//!
//! // Generates Pointee for dyn Stringy
//! #[pointee]
//! trait Stringy {
//!     fn as_string(&self) -> String;
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod impls;

use core::{
    alloc::Layout,
    cmp,
    fmt,
    hash,
    marker::PhantomData,
    ptr,
};

pub use ptr_meta_derive::{pointee, Pointee};

/// Provides the pointer metadata type of any pointed-to type.
///
/// # Pointer metadata
///
/// Raw pointer types and reference types in Rust can be thought of as made of two parts:
/// a data pointer that contains the memory address of the value, and some metadata.
///
/// For statically-sized types (that implement the `Sized` traits)
/// as well as for `extern` types,
/// pointers are said to be “thin”: metadata is zero-sized and its type is `()`.
///
/// Pointers to [dynamically-sized types][dst] are said to be “wide” or “fat”,
/// they have non-zero-sized metadata:
///
/// * For structs whose last field is a DST, metadata is the metadata for the last field
/// * For the `str` type, metadata is the length in bytes as `usize`
/// * For slice types like `[T]`, metadata is the length in items as `usize`
/// * For trait objects like `dyn SomeTrait`, metadata is [`DynMetadata<Self>`][DynMetadata]
///   (e.g. `DynMetadata<dyn SomeTrait>`)
///
/// In the future, the Rust language may gain new kinds of types
/// that have different pointer metadata.
///
/// [dst]: https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts
///
///
/// # The `Pointee` trait
///
/// The point of this trait is its `Metadata` associated type,
/// which is `()` or `usize` or `DynMetadata<_>` as described above.
/// It is automatically implemented for every type.
/// It can be assumed to be implemented in a generic context, even without a corresponding bound.
///
///
/// # Usage
///
/// Raw pointers can be decomposed into the data address and metadata components
/// with their [`to_raw_parts`] method.
///
/// Alternatively, metadata alone can be extracted with the [`metadata`] function.
/// A reference can be passed to [`metadata`] and implicitly coerced.
///
/// A (possibly-wide) pointer can be put back together from its address and metadata
/// with [`from_raw_parts`] or [`from_raw_parts_mut`].
///
/// [`to_raw_parts`]: PtrExt::to_raw_parts
pub trait Pointee {
    /// The type for metadata in pointers and references to `Self`.
    type Metadata: Copy + Send + Sync + Ord + hash::Hash + Unpin;
}

impl<T> Pointee for T {
    type Metadata = ();
}

impl<T> Pointee for [T] {
    type Metadata = usize;
}

impl Pointee for str {
    type Metadata = usize;
}

#[cfg(feature = "std")]
impl Pointee for ::std::ffi::CStr {
    type Metadata = usize;
}

#[cfg(feature = "std")]
impl Pointee for ::std::ffi::OsStr {
    type Metadata = usize;
}

#[repr(C)]
pub(crate) union PtrRepr<T: Pointee + ?Sized> {
    pub(crate) const_ptr: *const T,
    pub(crate) mut_ptr: *mut T,
    pub(crate) components: PtrComponents<T>,
}

#[repr(C)]
pub(crate) struct PtrComponents<T: Pointee + ?Sized> {
    pub(crate) data_address: *const (),
    pub(crate) metadata: <T as Pointee>::Metadata,
}

impl<T: Pointee + ?Sized> Clone for PtrComponents<T> {
    fn clone(&self) -> Self {
        Self {
            data_address: self.data_address.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl<T: Pointee + ?Sized> Copy for PtrComponents<T> {}

/// Extract the metadata component of a pointer.
///
/// Values of type `*mut T`, `&T`, or `&mut T` can be passed directly to this function
/// as they implicitly coerce to `*const T`.
///
/// # Example
///
/// ```
/// use ptr_meta::metadata;
///
/// assert_eq!(metadata("foo"), 3_usize);
/// ```
pub fn metadata<T: Pointee + ?Sized>(ptr: *const T) -> <T as Pointee>::Metadata {
    unsafe { PtrRepr { const_ptr: ptr }.components.metadata }
}

/// Forms a (possibly-wide) raw pointer from a data address and metadata.
///
/// This function is safe but the returned pointer is not necessarily safe to dereference.
/// For slices, see the documentation of [`slice::from_raw_parts`] for safety requirements.
/// For trait objects, the metadata must come from a pointer to the same underlying ereased type.
///
/// [`slice::from_raw_parts`]: core::slice::from_raw_parts
pub fn from_raw_parts<T: Pointee + ?Sized>(data_address: *const (), metadata: <T as Pointee>::Metadata) -> *const T {
    unsafe { PtrRepr { components: PtrComponents { data_address, metadata } }.const_ptr }
}

/// Performs the same functionality as [`from_raw_parts`], except that a
/// raw `*mut` pointer is returned, as opposed to a raw `*const` pointer.
///
/// See the documentation of [`from_raw_parts`] for more details.
pub fn from_raw_parts_mut<T: Pointee + ?Sized>(data_address: *mut (), metadata: <T as Pointee>::Metadata) -> *mut T {
    unsafe { PtrRepr { components: PtrComponents { data_address, metadata } }.mut_ptr }
}

/// Extension methods for [`NonNull`](core::ptr::NonNull).
pub trait NonNullExt<T: Pointee + ?Sized> {
    /// The type's raw pointer (`NonNull<()>`).
    type Raw;

    /// Creates a new non-null pointer from its raw parts.
    fn from_raw_parts(raw: Self::Raw, meta: <T as Pointee>::Metadata) -> Self;
    /// Converts a non-null pointer to its raw parts.
    fn to_raw_parts(self) -> (Self::Raw, <T as Pointee>::Metadata);
}

impl<T: Pointee + ?Sized> NonNullExt<T> for ptr::NonNull<T> {
    type Raw = ptr::NonNull<()>;

    fn from_raw_parts(raw: Self::Raw, meta: <T as Pointee>::Metadata) -> Self {
        unsafe { Self::new_unchecked(from_raw_parts_mut(raw.as_ptr(), meta)) }
    }

    fn to_raw_parts(self) -> (Self::Raw, <T as Pointee>::Metadata) {
        let (raw, meta) = PtrExt::to_raw_parts(self.as_ptr());
        unsafe { (ptr::NonNull::new_unchecked(raw), meta) }
    }
}

/// Extension methods for pointers.
pub trait PtrExt<T: Pointee + ?Sized> {
    /// The type's raw pointer (`*const ()` or `*mut ()`).
    type Raw;

    /// Decompose a (possibly wide) pointer into its address and metadata
    /// components.
    ///
    /// The pointer can be later reconstructed with [`from_raw_parts`].
    fn to_raw_parts(self) -> (Self::Raw, <T as Pointee>::Metadata);
}

impl<T: Pointee + ?Sized> PtrExt<T> for *const T {
    type Raw = *const ();

    fn to_raw_parts(self) -> (Self::Raw, <T as Pointee>::Metadata) {
        unsafe { (&self as *const Self).cast::<(Self::Raw, <T as Pointee>::Metadata)>().read() }
    }
}

impl<T: Pointee + ?Sized> PtrExt<T> for *mut T {
    type Raw = *mut ();

    fn to_raw_parts(self) -> (Self::Raw, <T as Pointee>::Metadata) {
        unsafe { (&self as *const Self).cast::<(Self::Raw, <T as Pointee>::Metadata)>().read() }
    }
}

/// The metadata for a `Dyn = dyn SomeTrait` trait object type.
///
/// It is a pointer to a vtable (virtual call table)
/// that represents all the necessary information
/// to manipulate the concrete type stored inside a trait object.
/// The vtable notably it contains:
///
/// * type size
/// * type alignment
/// * a pointer to the type’s `drop_in_place` impl (may be a no-op for plain-old-data)
/// * pointers to all the methods for the type’s implementation of the trait
///
/// Note that the first three are special because they’re necessary to allocate, drop,
/// and deallocate any trait object.
///
/// It is possible to name this struct with a type parameter that is not a `dyn` trait object
/// (for example `DynMetadata<u64>`) but not to obtain a meaningful value of that struct.
#[repr(transparent)]
pub struct DynMetadata<Dyn: ?Sized> {
    vtable_ptr: &'static VTable,
    phantom: PhantomData<Dyn>,
}

#[repr(C)]
struct VTable {
    drop_in_place: fn(*mut ()),
    size_of: usize,
    align_of: usize,
}

impl<Dyn: ?Sized> DynMetadata<Dyn> {
    /// Returns the size of the type associated with this vtable.
    pub fn size_of(self) -> usize {
        self.vtable_ptr.size_of
    }

    /// Returns the alignment of the type associated with this vtable.
    pub fn align_of(self) -> usize {
        self.vtable_ptr.align_of
    }

    /// Returns the size and alignment together as a `Layout`.
    pub fn layout(self) -> Layout {
        unsafe { Layout::from_size_align_unchecked(self.size_of(), self.align_of()) }
    }
}

unsafe impl<Dyn: ?Sized> Send for DynMetadata<Dyn> {}
unsafe impl<Dyn: ?Sized> Sync for DynMetadata<Dyn> {}
impl<Dyn: ?Sized> fmt::Debug for DynMetadata<Dyn> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DynMetadata").field(&(self.vtable_ptr as *const VTable)).finish()
    }
}
impl<Dyn: ?Sized> Unpin for DynMetadata<Dyn> {}
impl<Dyn: ?Sized> Copy for DynMetadata<Dyn> {}
impl<Dyn: ?Sized> Clone for DynMetadata<Dyn> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<Dyn: ?Sized> cmp::Eq for DynMetadata<Dyn> {}
impl<Dyn: ?Sized> cmp::PartialEq for DynMetadata<Dyn> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.vtable_ptr, other.vtable_ptr)
    }
}
impl<Dyn: ?Sized> cmp::Ord for DynMetadata<Dyn> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.vtable_ptr as *const VTable).cmp(&(other.vtable_ptr as *const VTable))
    }
}
impl<Dyn: ?Sized> cmp::PartialOrd for DynMetadata<Dyn> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<Dyn: ?Sized> hash::Hash for DynMetadata<Dyn> {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        ptr::hash(self.vtable_ptr, hasher)
    }
}

#[cfg(test)]
mod tests {
    use crate as ptr_meta;
    use super::{from_raw_parts, pointee, Pointee, PtrExt};

    fn test_pointee<T: Pointee + ?Sized>(value: &T) {
        let ptr = value as *const T;
        let (raw, meta) = PtrExt::to_raw_parts(ptr);
        let re_ptr = from_raw_parts::<T>(raw, meta);
        assert_eq!(ptr, re_ptr);
    }

    #[test]
    fn sized_types() {
        test_pointee(&());
        test_pointee(&42);
        test_pointee(&true);
        test_pointee(&[1, 2, 3, 4]);

        struct TestUnit;

        test_pointee(&TestUnit);

        #[allow(dead_code)]
        struct TestStruct {
            a: (),
            b: i32,
            c: bool,
        }

        test_pointee(&TestStruct { a: (), b: 42, c: true });

        struct TestTuple((), i32, bool);

        test_pointee(&TestTuple((), 42, true));

        struct TestGeneric<T>(T);

        test_pointee(&TestGeneric(42));
    }

    #[test]
    fn unsized_types() {
        test_pointee("hello world");
        test_pointee(&[1, 2, 3, 4] as &[i32]);
    }

    #[test]
    fn trait_objects() {
        #[pointee]
        trait TestTrait {
            fn foo(&self);
        }

        struct A;

        impl TestTrait for A {
            fn foo(&self) {}
        }

        let trait_object = &A as &dyn TestTrait;

        test_pointee(trait_object);

        let (_, meta) = PtrExt::to_raw_parts(trait_object as *const dyn TestTrait);

        assert_eq!(meta.size_of(), 0);
        assert_eq!(meta.align_of(), 1);

        struct B(i32);

        impl TestTrait for B {
            fn foo(&self) {}
        }

        let b = B(42);
        let trait_object = &b as &dyn TestTrait;

        test_pointee(trait_object);

        let (_, meta) = PtrExt::to_raw_parts(trait_object as *const dyn TestTrait);

        assert_eq!(meta.size_of(), 4);
        assert_eq!(meta.align_of(), 4);
    }

    #[test]
    fn last_field_dst() {
        #[allow(dead_code)]
        #[derive(Pointee)]
        struct Test<H, T> {
            head: H,
            tail: [T],
        }

        #[allow(dead_code)]
        #[derive(Pointee)]
        struct TestDyn {
            tail: dyn core::any::Any,
        }

        #[pointee]
        trait TestTrait {}

        #[allow(dead_code)]
        #[derive(Pointee)]
        struct TestCustomDyn {
            tail: dyn TestTrait,
        }
    }
}

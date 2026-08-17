// Needed otherwise the rkyv macros generate code incompatible with rust-2024
#![cfg_attr(feature = "rkyv-serialize", allow(unsafe_op_in_unsafe_fn))]

use std::fmt::{self, Debug, Formatter};
// use std::hash::{Hash, Hasher};
use std::ops::Mul;

#[cfg(feature = "serde-serialize-no-std")]
use serde::de::{Error, SeqAccess, Visitor};
#[cfg(feature = "serde-serialize-no-std")]
use serde::ser::SerializeTuple;
#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "serde-serialize-no-std")]
use std::marker::PhantomData;

#[cfg(feature = "rkyv-serialize")]
use rkyv::bytecheck;

use crate::Storage;
use crate::base::Scalar;
use crate::base::allocator::Allocator;
use crate::base::default_allocator::DefaultAllocator;
use crate::base::dimension::{Const, ToTypenum};
use crate::base::storage::{IsContiguous, Owned, RawStorage, RawStorageMut, ReshapableStorage};
use std::mem;

/*
 *
 * Static RawStorage.
 *
 */
/// An array-based statically sized matrix data storage.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "rkyv-serialize-no-std",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
    archive(
        as = "ArrayStorage<T::Archived, R, C>",
        bound(archive = "
        T: rkyv::Archive,
        [[T; R]; C]: rkyv::Archive<Archived = [[T::Archived; R]; C]>
    ")
    )
)]
#[cfg_attr(feature = "rkyv-serialize", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ArrayStorage<T, const R: usize, const C: usize>(pub [[T; R]; C]);

impl<T, const R: usize, const C: usize> ArrayStorage<T, R, C> {
    /// Converts this array storage to a slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: this is OK because ArrayStorage is contiguous.
        unsafe { self.as_slice_unchecked() }
    }

    /// Converts this array storage to a mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: this is OK because ArrayStorage is contiguous.
        unsafe { self.as_mut_slice_unchecked() }
    }
}

// TODO: remove this once the stdlib implements Default for arrays.
impl<T: Default, const R: usize, const C: usize> Default for ArrayStorage<T, R, C>
where
    [[T; R]; C]: Default,
{
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Debug, const R: usize, const C: usize> Debug for ArrayStorage<T, R, C> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

unsafe impl<T, const R: usize, const C: usize> RawStorage<T, Const<R>, Const<C>>
    for ArrayStorage<T, R, C>
{
    type RStride = Const<1>;
    type CStride = Const<R>;

    #[inline]
    fn ptr(&self) -> *const T {
        self.0.as_ptr() as *const T
    }

    #[inline]
    fn shape(&self) -> (Const<R>, Const<C>) {
        (Const, Const)
    }

    #[inline]
    fn strides(&self) -> (Self::RStride, Self::CStride) {
        (Const, Const)
    }

    #[inline]
    fn is_contiguous(&self) -> bool {
        true
    }

    #[inline]
    unsafe fn as_slice_unchecked(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr(), R * C) }
    }
}

unsafe impl<T: Scalar, const R: usize, const C: usize> Storage<T, Const<R>, Const<C>>
    for ArrayStorage<T, R, C>
where
    DefaultAllocator: Allocator<Const<R>, Const<C>, Buffer<T> = Self>,
{
    #[inline]
    fn into_owned(self) -> Owned<T, Const<R>, Const<C>>
    where
        DefaultAllocator: Allocator<Const<R>, Const<C>>,
    {
        self
    }

    #[inline]
    fn clone_owned(&self) -> Owned<T, Const<R>, Const<C>>
    where
        DefaultAllocator: Allocator<Const<R>, Const<C>>,
    {
        self.clone()
    }

    #[inline]
    fn forget_elements(self) {
        // No additional cleanup required.
        std::mem::forget(self);
    }
}

unsafe impl<T, const R: usize, const C: usize> RawStorageMut<T, Const<R>, Const<C>>
    for ArrayStorage<T, R, C>
{
    #[inline]
    fn ptr_mut(&mut self) -> *mut T {
        self.0.as_mut_ptr() as *mut T
    }

    #[inline]
    unsafe fn as_mut_slice_unchecked(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr_mut(), R * C) }
    }
}

unsafe impl<T, const R: usize, const C: usize> IsContiguous for ArrayStorage<T, R, C> {}

impl<T, const R1: usize, const C1: usize, const R2: usize, const C2: usize>
    ReshapableStorage<T, Const<R1>, Const<C1>, Const<R2>, Const<C2>> for ArrayStorage<T, R1, C1>
where
    T: Scalar,
    Const<R1>: ToTypenum,
    Const<C1>: ToTypenum,
    Const<R2>: ToTypenum,
    Const<C2>: ToTypenum,
    <Const<R1> as ToTypenum>::Typenum: Mul<<Const<C1> as ToTypenum>::Typenum>,
    <Const<R2> as ToTypenum>::Typenum: Mul<
            <Const<C2> as ToTypenum>::Typenum,
            Output = typenum::Prod<
                <Const<R1> as ToTypenum>::Typenum,
                <Const<C1> as ToTypenum>::Typenum,
            >,
        >,
{
    type Output = ArrayStorage<T, R2, C2>;

    fn reshape_generic(self, _: Const<R2>, _: Const<C2>) -> Self::Output {
        unsafe {
            let data: [[T; R2]; C2] = mem::transmute_copy(&self.0);
            mem::forget(self.0);
            ArrayStorage(data)
        }
    }
}

/*
 *
 * Serialization.
 *
 */
// XXX: open an issue for serde so that it allows the serialization/deserialization of all arrays?
#[cfg(feature = "serde-serialize-no-std")]
impl<T, const R: usize, const C: usize> Serialize for ArrayStorage<T, R, C>
where
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_tuple(R * C)?;

        for e in self.as_slice().iter() {
            serializer.serialize_element(e)?;
        }

        serializer.end()
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<'a, T, const R: usize, const C: usize> Deserialize<'a> for ArrayStorage<T, R, C>
where
    T: Scalar + Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_tuple(R * C, ArrayStorageVisitor::new())
    }
}

#[cfg(feature = "serde-serialize-no-std")]
/// A visitor that produces a matrix array.
struct ArrayStorageVisitor<T, const R: usize, const C: usize> {
    marker: PhantomData<T>,
}

#[cfg(feature = "serde-serialize-no-std")]
impl<T, const R: usize, const C: usize> ArrayStorageVisitor<T, R, C>
where
    T: Scalar,
{
    /// Construct a new sequence visitor.
    pub fn new() -> Self {
        ArrayStorageVisitor {
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<'a, T, const R: usize, const C: usize> Visitor<'a> for ArrayStorageVisitor<T, R, C>
where
    T: Scalar + Deserialize<'a>,
{
    type Value = ArrayStorage<T, R, C>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("a matrix array")
    }

    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<ArrayStorage<T, R, C>, V::Error>
    where
        V: SeqAccess<'a>,
    {
        let mut out: ArrayStorage<core::mem::MaybeUninit<T>, R, C> =
            <DefaultAllocator as Allocator<_, _>>::allocate_uninit(Const::<R>, Const::<C>);
        let mut curr = 0;

        while let Some(value) = visitor.next_element()? {
            *out.as_mut_slice()
                .get_mut(curr)
                .ok_or_else(|| V::Error::invalid_length(curr, &self))? =
                core::mem::MaybeUninit::new(value);
            curr += 1;
        }

        if curr == R * C {
            // Safety: all the elements have been initialized.
            unsafe { Ok(<DefaultAllocator as Allocator<Const<R>, Const<C>>>::assume_init(out)) }
        } else {
            for i in 0..curr {
                // Safety:
                // - We couldn’t initialize the whole storage. Drop the ones we initialized.
                unsafe { std::ptr::drop_in_place(out.as_mut_slice()[i].as_mut_ptr()) };
            }

            Err(V::Error::invalid_length(curr, &self))
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Scalar + Copy + bytemuck::Zeroable, const R: usize, const C: usize>
    bytemuck::Zeroable for ArrayStorage<T, R, C>
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Scalar + Copy + bytemuck::Pod, const R: usize, const C: usize> bytemuck::Pod
    for ArrayStorage<T, R, C>
{
}

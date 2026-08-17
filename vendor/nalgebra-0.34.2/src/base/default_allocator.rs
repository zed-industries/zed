//! The default matrix data storage allocator.
//!
//! This will use stack-allocated buffers for matrices with dimensions known at compile-time, and
//! heap-allocated buffers for matrices with at least one dimension unknown at compile-time.

use std::cmp;
use std::ptr;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

use super::Const;
use crate::base::Scalar;
use crate::base::allocator::{Allocator, Reallocator};
use crate::base::array_storage::ArrayStorage;
use crate::base::dimension::Dim;
#[cfg(any(feature = "alloc", feature = "std"))]
use crate::base::dimension::{DimName, Dyn};
use crate::base::storage::{RawStorage, RawStorageMut, Storage};
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::base::vec_storage::VecStorage;
#[cfg(any(feature = "std", feature = "alloc"))]
use std::mem::ManuallyDrop;
use std::mem::MaybeUninit;

/*
 *
 * Allocator.
 *
 */
/// An allocator based on [`ArrayStorage`] and [`VecStorage`] for statically-sized and dynamically-sized
/// matrices respectively.
#[derive(Copy, Clone, Debug)]
pub struct DefaultAllocator;

// Static - Static
impl<const R: usize, const C: usize> Allocator<Const<R>, Const<C>> for DefaultAllocator {
    type Buffer<T: Scalar> = ArrayStorage<T, R, C>;
    type BufferUninit<T: Scalar> = ArrayStorage<MaybeUninit<T>, R, C>;

    #[inline(always)]
    fn allocate_uninit<T: Scalar>(_: Const<R>, _: Const<C>) -> ArrayStorage<MaybeUninit<T>, R, C> {
        // SAFETY: An uninitialized `[MaybeUninit<_>; _]` is valid.
        let array: [[MaybeUninit<T>; R]; C] = unsafe { MaybeUninit::uninit().assume_init() };
        ArrayStorage(array)
    }

    #[inline(always)]
    unsafe fn assume_init<T: Scalar>(
        uninit: ArrayStorage<MaybeUninit<T>, R, C>,
    ) -> ArrayStorage<T, R, C> {
        unsafe {
            // Safety:
            // * The caller guarantees that all elements of the array are initialized
            // * `MaybeUninit<T>` and T are guaranteed to have the same layout
            // * `MaybeUninit` does not drop, so there are no double-frees
            // And thus the conversion is safe
            ArrayStorage((&uninit as *const _ as *const [_; C]).read())
        }
    }

    #[inline]
    fn allocate_from_iterator<T: Scalar, I: IntoIterator<Item = T>>(
        nrows: Const<R>,
        ncols: Const<C>,
        iter: I,
    ) -> Self::Buffer<T> {
        let mut res = Self::allocate_uninit(nrows, ncols);
        let mut count = 0;

        // Safety: conversion to a slice is OK because the Buffer is known to be contiguous.
        let res_slice = unsafe { res.as_mut_slice_unchecked() };
        for (res, e) in res_slice.iter_mut().zip(iter.into_iter()) {
            *res = MaybeUninit::new(e);
            count += 1;
        }

        assert!(
            count == nrows.value() * ncols.value(),
            "Matrix init. from iterator: iterator not long enough."
        );

        // Safety: the assertion above made sure that the iterator
        //         yielded enough elements to initialize our matrix.
        unsafe { <Self as Allocator<Const<R>, Const<C>>>::assume_init(res) }
    }
}

// Dyn - Static
// Dyn - Dyn
#[cfg(any(feature = "std", feature = "alloc"))]
impl<C: Dim> Allocator<Dyn, C> for DefaultAllocator {
    type Buffer<T: Scalar> = VecStorage<T, Dyn, C>;
    type BufferUninit<T: Scalar> = VecStorage<MaybeUninit<T>, Dyn, C>;

    #[inline]
    fn allocate_uninit<T: Scalar>(nrows: Dyn, ncols: C) -> VecStorage<MaybeUninit<T>, Dyn, C> {
        let mut data = Vec::new();
        let length = nrows.value() * ncols.value();
        data.reserve_exact(length);
        data.resize_with(length, MaybeUninit::uninit);
        VecStorage::new(nrows, ncols, data)
    }

    #[inline]
    unsafe fn assume_init<T: Scalar>(
        uninit: VecStorage<MaybeUninit<T>, Dyn, C>,
    ) -> VecStorage<T, Dyn, C> {
        unsafe {
            // Avoids a double-drop.
            let (nrows, ncols) = uninit.shape();
            let vec: Vec<_> = uninit.into();
            let mut md = ManuallyDrop::new(vec);

            // Safety:
            // - MaybeUninit<T> has the same alignment and layout as T.
            // - The length and capacity come from a valid vector.
            let new_data = Vec::from_raw_parts(md.as_mut_ptr() as *mut _, md.len(), md.capacity());

            VecStorage::new(nrows, ncols, new_data)
        }
    }

    #[inline]
    fn allocate_from_iterator<T: Scalar, I: IntoIterator<Item = T>>(
        nrows: Dyn,
        ncols: C,
        iter: I,
    ) -> Self::Buffer<T> {
        let it = iter.into_iter();
        let res: Vec<T> = it.collect();
        assert!(
            res.len() == nrows.value() * ncols.value(),
            "Allocation from iterator error: the iterator did not yield the correct number of elements."
        );

        VecStorage::new(nrows, ncols, res)
    }
}

// Static - Dyn
#[cfg(any(feature = "std", feature = "alloc"))]
impl<R: DimName> Allocator<R, Dyn> for DefaultAllocator {
    type Buffer<T: Scalar> = VecStorage<T, R, Dyn>;
    type BufferUninit<T: Scalar> = VecStorage<MaybeUninit<T>, R, Dyn>;

    #[inline]
    fn allocate_uninit<T: Scalar>(nrows: R, ncols: Dyn) -> VecStorage<MaybeUninit<T>, R, Dyn> {
        let mut data = Vec::new();
        let length = nrows.value() * ncols.value();
        data.reserve_exact(length);
        data.resize_with(length, MaybeUninit::uninit);

        VecStorage::new(nrows, ncols, data)
    }

    #[inline]
    unsafe fn assume_init<T: Scalar>(
        uninit: VecStorage<MaybeUninit<T>, R, Dyn>,
    ) -> VecStorage<T, R, Dyn> {
        unsafe {
            // Avoids a double-drop.
            let (nrows, ncols) = uninit.shape();
            let vec: Vec<_> = uninit.into();
            let mut md = ManuallyDrop::new(vec);

            // Safety:
            // - MaybeUninit<T> has the same alignment and layout as T.
            // - The length and capacity come from a valid vector.
            let new_data = Vec::from_raw_parts(md.as_mut_ptr() as *mut _, md.len(), md.capacity());

            VecStorage::new(nrows, ncols, new_data)
        }
    }

    #[inline]
    fn allocate_from_iterator<T: Scalar, I: IntoIterator<Item = T>>(
        nrows: R,
        ncols: Dyn,
        iter: I,
    ) -> Self::Buffer<T> {
        let it = iter.into_iter();
        let res: Vec<T> = it.collect();
        assert!(
            res.len() == nrows.value() * ncols.value(),
            "Allocation from iterator error: the iterator did not yield the correct number of elements."
        );

        VecStorage::new(nrows, ncols, res)
    }
}

/*
 *
 * Reallocator.
 *
 */
// Anything -> Static × Static
impl<T: Scalar, RFrom, CFrom, const RTO: usize, const CTO: usize>
    Reallocator<T, RFrom, CFrom, Const<RTO>, Const<CTO>> for DefaultAllocator
where
    RFrom: Dim,
    CFrom: Dim,
    Self: Allocator<RFrom, CFrom>,
{
    #[inline]
    unsafe fn reallocate_copy(
        rto: Const<RTO>,
        cto: Const<CTO>,
        buf: <Self as Allocator<RFrom, CFrom>>::Buffer<T>,
    ) -> ArrayStorage<MaybeUninit<T>, RTO, CTO> {
        unsafe {
            let mut res = <Self as Allocator<Const<RTO>, Const<CTO>>>::allocate_uninit(rto, cto);

            let (rfrom, cfrom) = buf.shape();

            let len_from = rfrom.value() * cfrom.value();
            let len_to = rto.value() * cto.value();
            let len_copied = cmp::min(len_from, len_to);
            ptr::copy_nonoverlapping(buf.ptr(), res.ptr_mut() as *mut T, len_copied);

            // Safety:
            // - We don’t care about dropping elements because the caller is responsible for dropping things.
            // - We forget `buf` so that we don’t drop the other elements, but ensure the buffer itself is cleaned up.
            buf.forget_elements();

            res
        }
    }
}

// Static × Static -> Dyn × Any
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar, CTo, const RFROM: usize, const CFROM: usize>
    Reallocator<T, Const<RFROM>, Const<CFROM>, Dyn, CTo> for DefaultAllocator
where
    CTo: Dim,
{
    #[inline]
    unsafe fn reallocate_copy(
        rto: Dyn,
        cto: CTo,
        buf: ArrayStorage<T, RFROM, CFROM>,
    ) -> VecStorage<MaybeUninit<T>, Dyn, CTo> {
        unsafe {
            let mut res = <Self as Allocator<Dyn, CTo>>::allocate_uninit(rto, cto);

            let (rfrom, cfrom) = buf.shape();

            let len_from = rfrom.value() * cfrom.value();
            let len_to = rto.value() * cto.value();
            let len_copied = cmp::min(len_from, len_to);
            ptr::copy_nonoverlapping(buf.ptr(), res.ptr_mut() as *mut T, len_copied);

            // Safety:
            // - We don’t care about dropping elements because the caller is responsible for dropping things.
            // - We forget `buf` so that we don’t drop the other elements, but ensure the buffer itself is cleaned up.
            buf.forget_elements();

            res
        }
    }
}

// Static × Static -> Static × Dyn
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar, RTo, const RFROM: usize, const CFROM: usize>
    Reallocator<T, Const<RFROM>, Const<CFROM>, RTo, Dyn> for DefaultAllocator
where
    RTo: DimName,
{
    #[inline]
    unsafe fn reallocate_copy(
        rto: RTo,
        cto: Dyn,
        buf: ArrayStorage<T, RFROM, CFROM>,
    ) -> VecStorage<MaybeUninit<T>, RTo, Dyn> {
        unsafe {
            let mut res = <Self as Allocator<RTo, Dyn>>::allocate_uninit(rto, cto);

            let (rfrom, cfrom) = buf.shape();

            let len_from = rfrom.value() * cfrom.value();
            let len_to = rto.value() * cto.value();
            let len_copied = cmp::min(len_from, len_to);
            ptr::copy_nonoverlapping(buf.ptr(), res.ptr_mut() as *mut T, len_copied);

            // Safety:
            // - We don’t care about dropping elements because the caller is responsible for dropping things.
            // - We forget `buf` so that we don’t drop the other elements, but ensure the buffer itself is cleaned up.
            buf.forget_elements();

            res
        }
    }
}

// All conversion from a dynamic buffer to a dynamic buffer.
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar, CFrom: Dim, CTo: Dim> Reallocator<T, Dyn, CFrom, Dyn, CTo> for DefaultAllocator {
    #[inline]
    unsafe fn reallocate_copy(
        rto: Dyn,
        cto: CTo,
        buf: VecStorage<T, Dyn, CFrom>,
    ) -> VecStorage<MaybeUninit<T>, Dyn, CTo> {
        unsafe {
            let new_buf = buf.resize(rto.value() * cto.value());
            VecStorage::new(rto, cto, new_buf)
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar, CFrom: Dim, RTo: DimName> Reallocator<T, Dyn, CFrom, RTo, Dyn>
    for DefaultAllocator
{
    #[inline]
    unsafe fn reallocate_copy(
        rto: RTo,
        cto: Dyn,
        buf: VecStorage<T, Dyn, CFrom>,
    ) -> VecStorage<MaybeUninit<T>, RTo, Dyn> {
        unsafe {
            let new_buf = buf.resize(rto.value() * cto.value());
            VecStorage::new(rto, cto, new_buf)
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar, RFrom: DimName, CTo: Dim> Reallocator<T, RFrom, Dyn, Dyn, CTo>
    for DefaultAllocator
{
    #[inline]
    unsafe fn reallocate_copy(
        rto: Dyn,
        cto: CTo,
        buf: VecStorage<T, RFrom, Dyn>,
    ) -> VecStorage<MaybeUninit<T>, Dyn, CTo> {
        unsafe {
            let new_buf = buf.resize(rto.value() * cto.value());
            VecStorage::new(rto, cto, new_buf)
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar, RFrom: DimName, RTo: DimName> Reallocator<T, RFrom, Dyn, RTo, Dyn>
    for DefaultAllocator
{
    #[inline]
    unsafe fn reallocate_copy(
        rto: RTo,
        cto: Dyn,
        buf: VecStorage<T, RFrom, Dyn>,
    ) -> VecStorage<MaybeUninit<T>, RTo, Dyn> {
        unsafe {
            let new_buf = buf.resize(rto.value() * cto.value());
            VecStorage::new(rto, cto, new_buf)
        }
    }
}

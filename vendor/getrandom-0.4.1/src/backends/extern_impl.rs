//! An implementation which calls out to an externally defined function.
use crate::Error;
use core::mem::MaybeUninit;

/// Declares this function as an external implementation of [`fill_uninit`](crate::fill_uninit).
#[eii(fill_uninit)]
pub(crate) fn fill_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error>;

/// Declares this function as an external implementation of [`u32`](crate::u32).
#[eii(u32)]
pub(crate) fn inner_u32() -> Result<u32, crate::Error> {
    crate::util::inner_u32()
}

/// Declares this function as an external implementation of [`u64`](crate::u64).
#[eii(u64)]
pub(crate) fn inner_u64() -> Result<u64, crate::Error> {
    crate::util::inner_u64()
}

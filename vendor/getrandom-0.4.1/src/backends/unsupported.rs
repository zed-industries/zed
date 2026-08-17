//! Implementation that errors at runtime.
use crate::Error;
use core::mem::MaybeUninit;

pub use crate::util::{inner_u32, inner_u64};

pub fn fill_inner(_dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    Err(Error::UNSUPPORTED)
}

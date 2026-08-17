//! Archived bitwise containers.

use crate::{vec::ArchivedVec, Archived};
use bitvec::{
    order::{BitOrder, Lsb0},
    slice::BitSlice,
    store::BitStore,
    view::BitView,
};
use core::{marker::PhantomData, ops::Deref};

/// An archived `BitVec`.
// We also have to store the bit length in the archived `BitVec`.
// This is because when calling `as_raw_slice` we will get unwanted bits if the `BitVec` bit length is not a multiple of the bit size of T.
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "strict", repr(C))]
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ArchivedBitVec<T = Archived<usize>, O = Lsb0> {
    pub(crate) inner: ArchivedVec<T>,
    pub(crate) bit_len: Archived<usize>,
    pub(crate) _or: PhantomData<O>,
}

impl<T: BitStore, O: BitOrder> Deref for ArchivedBitVec<T, O> {
    type Target = BitSlice<T, O>;

    fn deref(&self) -> &Self::Target {
        &self.inner.view_bits::<O>()[..self.bit_len as usize]
    }
}

//! Miscellaneous helpers for machine backends.

use crate::ir::Type;
use std::ops::{Add, BitAnd, Not, Sub};

/// Returns the size (in bits) of a given type.
pub fn ty_bits(ty: Type) -> usize {
    ty.bits() as usize
}

/// Align a size up to a power-of-two alignment.
pub(crate) fn align_to<N>(x: N, alignment: N) -> N
where
    N: Not<Output = N>
        + BitAnd<N, Output = N>
        + Add<N, Output = N>
        + Sub<N, Output = N>
        + From<u8>
        + Copy,
{
    let alignment_mask = alignment - 1.into();
    (x + alignment_mask) & !alignment_mask
}

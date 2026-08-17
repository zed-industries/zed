//! Channel orders for packed Luma types.

use crate::{cast::ComponentOrder, luma};

/// Luma+Alpha color packed in LA order.
///
/// See [Packed](crate::cast::Packed) for more details.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct La;

impl<S, T> ComponentOrder<luma::Lumaa<S, T>, [T; 2]> for La {
    #[inline]
    fn pack(color: luma::Lumaa<S, T>) -> [T; 2] {
        color.into()
    }

    #[inline]
    fn unpack(packed: [T; 2]) -> luma::Lumaa<S, T> {
        packed.into()
    }
}

/// Luma+Alpha color packed in AL order.
///
/// See [Packed](crate::cast::Packed) for more details.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Al;

impl<S, T> ComponentOrder<luma::Lumaa<S, T>, [T; 2]> for Al {
    #[inline]
    fn pack(color: luma::Lumaa<S, T>) -> [T; 2] {
        let [luma, alpha]: [T; 2] = color.into();
        [alpha, luma]
    }

    #[inline]
    fn unpack(packed: [T; 2]) -> luma::Lumaa<S, T> {
        let [alpha, luma] = packed;
        luma::Lumaa::new(luma, alpha)
    }
}

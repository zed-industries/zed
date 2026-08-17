#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, rc::Rc};

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
use alloc::sync::Arc;

use crate::varint::varint_max;
use core::{
    marker::PhantomData,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::{Range, RangeFrom, RangeInclusive, RangeTo},
    time::Duration,
};

/// This trait is used to enforce the maximum size required to
/// store the serialization of a given type.
pub trait MaxSize {
    /// The maximum possible size that the serialization of this
    /// type can have, in bytes.
    const POSTCARD_MAX_SIZE: usize;
}

impl MaxSize for bool {
    const POSTCARD_MAX_SIZE: usize = 1;
}

impl MaxSize for i8 {
    const POSTCARD_MAX_SIZE: usize = 1;
}

impl MaxSize for i16 {
    const POSTCARD_MAX_SIZE: usize = varint_max::<Self>();
}

impl MaxSize for i32 {
    const POSTCARD_MAX_SIZE: usize = varint_max::<Self>();
}

impl MaxSize for i64 {
    const POSTCARD_MAX_SIZE: usize = varint_max::<Self>();
}

impl MaxSize for i128 {
    const POSTCARD_MAX_SIZE: usize = varint_max::<Self>();
}

impl MaxSize for isize {
    const POSTCARD_MAX_SIZE: usize = varint_max::<Self>();
}

impl MaxSize for u8 {
    const POSTCARD_MAX_SIZE: usize = 1;
}

impl MaxSize for u16 {
    const POSTCARD_MAX_SIZE: usize = varint_max::<Self>();
}

impl MaxSize for u32 {
    const POSTCARD_MAX_SIZE: usize = varint_max::<Self>();
}

impl MaxSize for u64 {
    const POSTCARD_MAX_SIZE: usize = varint_max::<Self>();
}

impl MaxSize for u128 {
    const POSTCARD_MAX_SIZE: usize = varint_max::<Self>();
}

impl MaxSize for usize {
    const POSTCARD_MAX_SIZE: usize = varint_max::<Self>();
}

impl MaxSize for f32 {
    const POSTCARD_MAX_SIZE: usize = 4;
}

impl MaxSize for f64 {
    const POSTCARD_MAX_SIZE: usize = 8;
}

impl MaxSize for char {
    const POSTCARD_MAX_SIZE: usize = 5;
}

impl<T: MaxSize> MaxSize for Option<T> {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE + 1;
}

impl<T: MaxSize, E: MaxSize> MaxSize for Result<T, E> {
    const POSTCARD_MAX_SIZE: usize = max(T::POSTCARD_MAX_SIZE, E::POSTCARD_MAX_SIZE) + 1;
}

impl MaxSize for () {
    const POSTCARD_MAX_SIZE: usize = 0;
}

impl<T: MaxSize, const N: usize> MaxSize for [T; N] {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE * N;
}

impl<T: MaxSize> MaxSize for &'_ T {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE;
}

impl<T: MaxSize> MaxSize for &'_ mut T {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroI8 {
    const POSTCARD_MAX_SIZE: usize = i8::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroI16 {
    const POSTCARD_MAX_SIZE: usize = i16::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroI32 {
    const POSTCARD_MAX_SIZE: usize = i32::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroI64 {
    const POSTCARD_MAX_SIZE: usize = i64::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroI128 {
    const POSTCARD_MAX_SIZE: usize = i128::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroIsize {
    const POSTCARD_MAX_SIZE: usize = isize::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroU8 {
    const POSTCARD_MAX_SIZE: usize = u8::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroU16 {
    const POSTCARD_MAX_SIZE: usize = u16::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroU32 {
    const POSTCARD_MAX_SIZE: usize = u32::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroU64 {
    const POSTCARD_MAX_SIZE: usize = u64::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroU128 {
    const POSTCARD_MAX_SIZE: usize = u128::POSTCARD_MAX_SIZE;
}

impl MaxSize for NonZeroUsize {
    const POSTCARD_MAX_SIZE: usize = usize::POSTCARD_MAX_SIZE;
}

impl MaxSize for Duration {
    const POSTCARD_MAX_SIZE: usize = u64::POSTCARD_MAX_SIZE + u32::POSTCARD_MAX_SIZE;
}

impl<T> MaxSize for PhantomData<T> {
    const POSTCARD_MAX_SIZE: usize = 0;
}

impl<A: MaxSize> MaxSize for (A,) {
    const POSTCARD_MAX_SIZE: usize = A::POSTCARD_MAX_SIZE;
}

impl<A: MaxSize, B: MaxSize> MaxSize for (A, B) {
    const POSTCARD_MAX_SIZE: usize = A::POSTCARD_MAX_SIZE + B::POSTCARD_MAX_SIZE;
}

impl<A: MaxSize, B: MaxSize, C: MaxSize> MaxSize for (A, B, C) {
    const POSTCARD_MAX_SIZE: usize =
        A::POSTCARD_MAX_SIZE + B::POSTCARD_MAX_SIZE + C::POSTCARD_MAX_SIZE;
}

impl<A: MaxSize, B: MaxSize, C: MaxSize, D: MaxSize> MaxSize for (A, B, C, D) {
    const POSTCARD_MAX_SIZE: usize =
        A::POSTCARD_MAX_SIZE + B::POSTCARD_MAX_SIZE + C::POSTCARD_MAX_SIZE + D::POSTCARD_MAX_SIZE;
}

impl<A: MaxSize, B: MaxSize, C: MaxSize, D: MaxSize, E: MaxSize> MaxSize for (A, B, C, D, E) {
    const POSTCARD_MAX_SIZE: usize = A::POSTCARD_MAX_SIZE
        + B::POSTCARD_MAX_SIZE
        + C::POSTCARD_MAX_SIZE
        + D::POSTCARD_MAX_SIZE
        + E::POSTCARD_MAX_SIZE;
}

impl<A: MaxSize, B: MaxSize, C: MaxSize, D: MaxSize, E: MaxSize, F: MaxSize> MaxSize
    for (A, B, C, D, E, F)
{
    const POSTCARD_MAX_SIZE: usize = A::POSTCARD_MAX_SIZE
        + B::POSTCARD_MAX_SIZE
        + C::POSTCARD_MAX_SIZE
        + D::POSTCARD_MAX_SIZE
        + E::POSTCARD_MAX_SIZE
        + F::POSTCARD_MAX_SIZE;
}

impl<T: MaxSize> MaxSize for Range<T> {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE * 2;
}

impl<T: MaxSize> MaxSize for RangeInclusive<T> {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE * 2;
}

impl<T: MaxSize> MaxSize for RangeFrom<T> {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE;
}

impl<T: MaxSize> MaxSize for RangeTo<T> {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE;
}

impl<T: MaxSize> MaxSize for core::num::Wrapping<T> {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE;
}

#[cfg(all(feature = "core-num-saturating", feature = "experimental-derive"))]
#[cfg_attr(docsrs, doc(cfg(feature = "core-num-saturating")))]
impl<T: MaxSize> MaxSize for core::num::Saturating<T> {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE;
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: MaxSize> MaxSize for Box<T> {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE;
}

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "alloc", target_has_atomic = "ptr"))))]
impl<T: MaxSize> MaxSize for Arc<T> {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE;
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: MaxSize> MaxSize for Rc<T> {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE;
}

#[cfg(feature = "heapless")]
#[cfg_attr(docsrs, doc(cfg(feature = "heapless")))]
impl<T: MaxSize, const N: usize> MaxSize for heapless::Vec<T, N> {
    const POSTCARD_MAX_SIZE: usize = <[T; N]>::POSTCARD_MAX_SIZE + varint_size(N);
}

#[cfg(feature = "heapless")]
#[cfg_attr(docsrs, doc(cfg(feature = "heapless")))]
impl<const N: usize> MaxSize for heapless::String<N> {
    const POSTCARD_MAX_SIZE: usize = <[u8; N]>::POSTCARD_MAX_SIZE + varint_size(N);
}

#[cfg(all(feature = "nalgebra-v0_33", feature = "experimental-derive"))]
#[cfg_attr(docsrs, doc(cfg(feature = "nalgebra-v0_33")))]
impl<T, const R: usize, const C: usize> MaxSize
    for nalgebra_v0_33::Matrix<
        T,
        nalgebra_v0_33::Const<R>,
        nalgebra_v0_33::Const<C>,
        nalgebra_v0_33::ArrayStorage<T, R, C>,
    >
where
    T: MaxSize + nalgebra_v0_33::Scalar,
{
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE * R * C;
}

#[cfg(all(feature = "nalgebra-v0_33", feature = "experimental-derive"))]
#[cfg_attr(docsrs, doc(cfg(feature = "nalgebra-v0_33")))]
impl<T: MaxSize> MaxSize for nalgebra_v0_33::Unit<T> {
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE;
}

#[cfg(all(feature = "nalgebra-v0_33", feature = "experimental-derive"))]
#[cfg_attr(docsrs, doc(cfg(feature = "nalgebra-v0_33")))]
impl<T: MaxSize + nalgebra_v0_33::Scalar> MaxSize for nalgebra_v0_33::Quaternion<T> {
    const POSTCARD_MAX_SIZE: usize = nalgebra_v0_33::Vector4::<T>::POSTCARD_MAX_SIZE;
}

#[cfg(feature = "heapless")]
const fn varint_size(max_n: usize) -> usize {
    const BITS_PER_BYTE: usize = 8;
    const BITS_PER_VARINT_BYTE: usize = 7;

    if max_n == 0 {
        return 1;
    }

    // How many data bits do we need for `max_n`.
    let bits = core::mem::size_of::<usize>() * BITS_PER_BYTE - max_n.leading_zeros() as usize;

    // We add (BITS_PER_BYTE - 1), to ensure any integer divisions
    // with a remainder will always add exactly one full byte, but
    // an evenly divided number of bits will be the same
    let roundup_bits = bits + (BITS_PER_VARINT_BYTE - 1);

    // Apply division, using normal "round down" integer division
    roundup_bits / BITS_PER_VARINT_BYTE
}

const fn max(lhs: usize, rhs: usize) -> usize {
    if lhs > rhs {
        lhs
    } else {
        rhs
    }
}

#[cfg(any(feature = "alloc", feature = "use-std"))]
#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use alloc::rc::Rc;

    #[cfg(target_has_atomic = "ptr")]
    use alloc::sync::Arc;

    #[test]
    fn box_max_size() {
        assert_eq!(Box::<u8>::POSTCARD_MAX_SIZE, 1);
        assert_eq!(Box::<u32>::POSTCARD_MAX_SIZE, 5);
        assert_eq!(Box::<(u128, [u8; 8])>::POSTCARD_MAX_SIZE, 27);
    }

    #[test]
    #[cfg(target_has_atomic = "ptr")]
    fn arc_max_size() {
        assert_eq!(Arc::<u8>::POSTCARD_MAX_SIZE, 1);
        assert_eq!(Arc::<u32>::POSTCARD_MAX_SIZE, 5);
        assert_eq!(Arc::<(u128, [u8; 8])>::POSTCARD_MAX_SIZE, 27);
    }

    #[test]
    fn rc_max_size() {
        assert_eq!(Rc::<u8>::POSTCARD_MAX_SIZE, 1);
        assert_eq!(Rc::<u32>::POSTCARD_MAX_SIZE, 5);
        assert_eq!(Rc::<(u128, [u8; 8])>::POSTCARD_MAX_SIZE, 27);
    }
}

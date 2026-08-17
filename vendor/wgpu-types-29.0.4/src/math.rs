//! Utility math functions.

use core::ops::{Add, Rem, Sub};

///
/// Aligns a `value` to an `alignment`.
///
/// Returns the first number greater than or equal to `value` that is also a
/// multiple of `alignment`. If `value` is already a multiple of `alignment`,
/// `value` will be returned.
///
/// # Examples
///
/// ```
/// # use wgpu_types::math::align_to;
/// assert_eq!(align_to(253, 16), 256);
/// assert_eq!(align_to(256, 16), 256);
/// assert_eq!(align_to(0, 16), 0);
/// ```
///
pub fn align_to<T>(value: T, alignment: T) -> T
where
    T: Add<Output = T> + Copy + Default + PartialEq<T> + Rem<Output = T> + Sub<Output = T>,
{
    let remainder = value % alignment;
    if remainder == T::default() {
        value
    } else {
        value + alignment - remainder
    }
}

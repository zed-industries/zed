use core::mem;

/// Find last set bit
/// fls(0) == 0, fls(u32::MAX) == 32
pub fn fls<T: num_traits::PrimInt>(v: T) -> usize {
    mem::size_of::<T>() * 8 - v.leading_zeros() as usize
}

pub fn ilog2<T: num_traits::PrimInt>(v: T) -> usize {
    fls(v) - 1
}

/// Divide two integers, and ceil the result.
pub fn idiv_ceil<T: num_traits::PrimInt>(a: T, b: T) -> T {
    if a % b != T::zero() {
        a / b + T::one()
    } else {
        a / b
    }
}

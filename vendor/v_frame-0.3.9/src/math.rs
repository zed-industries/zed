// Copyright (c) 2017-2020, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use num_traits::PrimInt;
use std::mem::size_of;

pub trait Fixed {
    fn floor_log2(&self, n: usize) -> usize;
    fn ceil_log2(&self, n: usize) -> usize;
    fn align_power_of_two(&self, n: usize) -> usize;
    fn align_power_of_two_and_shift(&self, n: usize) -> usize;
}

impl Fixed for usize {
    #[inline]
    fn floor_log2(&self, n: usize) -> usize {
        self & !((1 << n) - 1)
    }
    #[inline]
    fn ceil_log2(&self, n: usize) -> usize {
        (self + (1 << n) - 1).floor_log2(n)
    }
    #[inline]
    fn align_power_of_two(&self, n: usize) -> usize {
        self.ceil_log2(n)
    }
    #[inline]
    fn align_power_of_two_and_shift(&self, n: usize) -> usize {
        (self + (1 << n) - 1) >> n
    }
}

pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

pub trait ILog: PrimInt {
    // Integer binary logarithm of an integer value.
    // Returns floor(log2(self)) + 1, or 0 if self == 0.
    // This is the number of bits that would be required to represent self in two's
    //  complement notation with all of the leading zeros stripped.
    // TODO: Mark const once trait functions can be constant
    fn ilog(self) -> usize {
        size_of::<Self>() * 8 - self.leading_zeros() as usize
    }
}

impl<T> ILog for T where T: PrimInt {}

#[inline(always)]
pub fn msb(x: i32) -> i32 {
    debug_assert!(x > 0);
    31 ^ (x.leading_zeros() as i32)
}

#[inline(always)]
pub const fn round_shift(value: i32, bit: usize) -> i32 {
    (value + (1 << bit >> 1)) >> bit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_log2() {
        assert_eq!(123usize.floor_log2(4), 112);
        assert_eq!(16usize.floor_log2(4), 16);
        assert_eq!(0usize.floor_log2(4), 0);
    }

    #[test]
    fn test_ceil_log2() {
        assert_eq!(123usize.ceil_log2(4), 128);
        assert_eq!(16usize.ceil_log2(4), 16);
        assert_eq!(0usize.ceil_log2(4), 0);
    }

    #[test]
    fn test_align_power_of_two() {
        assert_eq!(123usize.align_power_of_two(4), 128);
        assert_eq!(16usize.align_power_of_two(4), 16);
        assert_eq!(0usize.align_power_of_two(4), 0);
    }

    #[test]
    fn test_align_power_of_two_and_shift() {
        assert_eq!(123usize.align_power_of_two_and_shift(4), 8);
        assert_eq!(16usize.align_power_of_two_and_shift(4), 1);
        assert_eq!(0usize.align_power_of_two_and_shift(4), 0);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-1, 0, 10), 0);
        assert_eq!(clamp(11, 0, 10), 10);
    }

    #[test]
    fn test_ilog() {
        assert_eq!(ILog::ilog(0i32), 0);
        assert_eq!(ILog::ilog(1i32), 1);
        assert_eq!(ILog::ilog(2i32), 2);
        assert_eq!(ILog::ilog(3i32), 2);
        assert_eq!(ILog::ilog(4i32), 3);
    }

    #[test]
    fn test_msb() {
        assert_eq!(msb(1), 0);
        assert_eq!(msb(2), 1);
        assert_eq!(msb(3), 1);
        assert_eq!(msb(4), 2);
    }

    #[test]
    fn test_round_shift() {
        assert_eq!(round_shift(7, 2), 2);
        assert_eq!(round_shift(8, 2), 2);
        assert_eq!(round_shift(9, 2), 2);
        assert_eq!(round_shift(10, 2), 3);
    }
}

mod permutation;
mod sim_hash;
mod sim_hash_index;
mod sim_hasher;

pub use self::sim_hash::SimHash;
pub use self::sim_hash_index::SimHashIndex;
pub use self::sim_hasher::SimSipHasher128;
pub use self::sim_hasher::SimSipHasher64;

use core::{fmt, mem};

use num_traits::{One, Zero};
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not, Shl, ShlAssign,
    Shr, ShrAssign,
};

pub trait SimHashBits:
    Sized
    + Clone
    + Copy
    + Zero
    + One
    + Debug
    + PartialOrd
    + PartialEq
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + BitOrAssign
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + Hash
    + Eq
{
    fn count_ones(self) -> usize;

    fn to_u32_high_bits(self) -> u32;

    fn to_u64_high_bits(self) -> u64;

    fn hamming_distance(&self, rhs: &Self) -> usize;

    fn hamming_angle(&self, rhs: &Self) -> f64 {
        self.hamming_distance(rhs) as f64 * (PI /  Self::bit_length() as f64)
    }

    fn bit_length() -> usize;
}

macro_rules! prim_int_impl {
    ($T:ty, $S:ty, $U:ty) => {
        impl SimHashBits for $T {
            #[inline]
            fn count_ones(self) -> usize {
                <$T>::count_ones(self) as usize
            }

            #[inline]
            fn to_u32_high_bits(self) -> u32 {
                (self >> ((mem::size_of::<$T>() * 8) - 32)) as u32
            }

            #[inline]
            fn to_u64_high_bits(self) -> u64 {
                (self >> ((mem::size_of::<$T>() * 8) - 64)) as u64
            }

            #[inline]
            fn hamming_distance(&self, rhs: &Self) -> usize {
                (self ^ rhs).count_ones() as usize
            }

            #[inline]
            fn bit_length() -> usize {
                mem::size_of::<$T>() * 8
            }
        }
    };
}

prim_int_impl!(u64, i64, u64);
prim_int_impl!(u128, i128, u128);

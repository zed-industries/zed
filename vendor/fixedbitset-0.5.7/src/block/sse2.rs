#![allow(clippy::undocumented_unsafe_blocks)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Block(pub(super) __m128i);

impl Block {
    #[inline]
    pub fn is_empty(self) -> bool {
        #[cfg(not(target_feature = "sse4.1"))]
        {
            self == Self::NONE
        }
        #[cfg(target_feature = "sse4.1")]
        {
            unsafe { _mm_test_all_zeros(self.0, self.0) == 1 }
        }
    }

    #[inline]
    pub fn andnot(self, other: Self) -> Self {
        Self(unsafe { _mm_andnot_si128(other.0, self.0) })
    }
}

impl Not for Block {
    type Output = Block;
    #[inline]
    fn not(self) -> Self::Output {
        unsafe { Self(_mm_xor_si128(self.0, Self::ALL.0)) }
    }
}

impl BitAnd for Block {
    type Output = Block;
    #[inline]
    fn bitand(self, other: Self) -> Self::Output {
        unsafe { Self(_mm_and_si128(self.0, other.0)) }
    }
}

impl BitAndAssign for Block {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        unsafe {
            self.0 = _mm_and_si128(self.0, other.0);
        }
    }
}

impl BitOr for Block {
    type Output = Block;
    #[inline]
    fn bitor(self, other: Self) -> Self::Output {
        unsafe { Self(_mm_or_si128(self.0, other.0)) }
    }
}

impl BitOrAssign for Block {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        unsafe {
            self.0 = _mm_or_si128(self.0, other.0);
        }
    }
}

impl BitXor for Block {
    type Output = Block;
    #[inline]
    fn bitxor(self, other: Self) -> Self::Output {
        unsafe { Self(_mm_xor_si128(self.0, other.0)) }
    }
}

impl BitXorAssign for Block {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        unsafe { self.0 = _mm_xor_si128(self.0, other.0) }
    }
}

impl PartialEq for Block {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            #[cfg(not(target_feature = "sse4.1"))]
            {
                _mm_movemask_epi8(_mm_cmpeq_epi8(self.0, other.0)) == 0xffff
            }
            #[cfg(target_feature = "sse4.1")]
            {
                let neq = _mm_xor_si128(self.0, other.0);
                _mm_test_all_zeros(neq, neq) == 1
            }
        }
    }
}

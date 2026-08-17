use core::{
    arch::wasm32::*,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Block(pub(super) v128);

impl Block {
    #[inline]
    pub fn is_empty(self) -> bool {
        !v128_any_true(self.0)
    }

    #[inline]
    pub fn andnot(self, other: Self) -> Self {
        Self(v128_andnot(self.0, other.0))
    }
}

impl Not for Block {
    type Output = Block;
    #[inline]
    fn not(self) -> Self::Output {
        Self(v128_xor(self.0, Self::ALL.0))
    }
}

impl BitAnd for Block {
    type Output = Block;
    #[inline]
    fn bitand(self, other: Self) -> Self::Output {
        Self(v128_and(self.0, other.0))
    }
}

impl BitAndAssign for Block {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 = v128_and(self.0, other.0);
    }
}

impl BitOr for Block {
    type Output = Block;
    #[inline]
    fn bitor(self, other: Self) -> Self::Output {
        Self(v128_or(self.0, other.0))
    }
}

impl BitOrAssign for Block {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 = v128_or(self.0, other.0);
    }
}

impl BitXor for Block {
    type Output = Block;
    #[inline]
    fn bitxor(self, other: Self) -> Self::Output {
        Self(v128_xor(self.0, other.0))
    }
}

impl BitXorAssign for Block {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        self.0 = v128_xor(self.0, other.0)
    }
}

impl PartialEq for Block {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        !v128_any_true(v128_xor(self.0, other.0))
    }
}

use subtle::Choice;

use crate::Word;

/// A boolean value returned by constant-time `const fn`s.
// TODO: should be replaced by `subtle::Choice` or `CtOption`
// when `subtle` starts supporting const fns.
#[derive(Debug, Copy, Clone)]
pub struct CtChoice(Word);

impl CtChoice {
    /// The falsy value.
    pub const FALSE: Self = Self(0);

    /// The truthy value.
    pub const TRUE: Self = Self(Word::MAX);

    /// Returns the truthy value if `value == Word::MAX`, and the falsy value if `value == 0`.
    /// Panics for other values.
    pub(crate) const fn from_mask(value: Word) -> Self {
        debug_assert!(value == Self::FALSE.0 || value == Self::TRUE.0);
        Self(value)
    }

    /// Returns the truthy value if `value == 1`, and the falsy value if `value == 0`.
    /// Panics for other values.
    pub(crate) const fn from_lsb(value: Word) -> Self {
        debug_assert!(value == 0 || value == 1);
        Self(value.wrapping_neg())
    }

    /// Returns the truthy value if `value != 0`, and the falsy value otherwise.
    pub(crate) const fn from_usize_being_nonzero(value: usize) -> Self {
        const HI_BIT: u32 = usize::BITS - 1;
        Self::from_lsb(((value | value.wrapping_neg()) >> HI_BIT) as Word)
    }

    /// Returns the truthy value if `x == y`, and the falsy value otherwise.
    pub(crate) const fn from_usize_equality(x: usize, y: usize) -> Self {
        Self::from_usize_being_nonzero(x.wrapping_sub(y)).not()
    }

    /// Returns the truthy value if `x < y`, and the falsy value otherwise.
    pub(crate) const fn from_usize_lt(x: usize, y: usize) -> Self {
        let bit = (((!x) & y) | (((!x) | y) & (x.wrapping_sub(y)))) >> (usize::BITS - 1);
        Self::from_lsb(bit as Word)
    }

    pub(crate) const fn not(&self) -> Self {
        Self(!self.0)
    }

    pub(crate) const fn or(&self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub(crate) const fn and(&self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Return `b` if `self` is truthy, otherwise return `a`.
    pub(crate) const fn select(&self, a: Word, b: Word) -> Word {
        a ^ (self.0 & (a ^ b))
    }

    /// Return `x` if `self` is truthy, otherwise return 0.
    pub(crate) const fn if_true(&self, x: Word) -> Word {
        x & self.0
    }

    pub(crate) const fn is_true_vartime(&self) -> bool {
        self.0 == CtChoice::TRUE.0
    }

    pub(crate) const fn to_u8(self) -> u8 {
        (self.0 as u8) & 1
    }
}

impl From<CtChoice> for Choice {
    fn from(choice: CtChoice) -> Self {
        Choice::from(choice.to_u8())
    }
}

impl From<CtChoice> for bool {
    fn from(choice: CtChoice) -> Self {
        choice.is_true_vartime()
    }
}

#[cfg(test)]
mod tests {
    use super::CtChoice;
    use crate::Word;

    #[test]
    fn select() {
        let a: Word = 1;
        let b: Word = 2;
        assert_eq!(CtChoice::TRUE.select(a, b), b);
        assert_eq!(CtChoice::FALSE.select(a, b), a);
    }
}

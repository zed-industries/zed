use crate::{CtChoice, Limb, Uint, Word};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Returns `true` if the bit at position `index` is set, `false` otherwise.
    ///
    /// # Remarks
    /// This operation is variable time with respect to `index` only.
    #[inline(always)]
    pub const fn bit_vartime(&self, index: usize) -> bool {
        if index >= Self::BITS {
            false
        } else {
            (self.limbs[index / Limb::BITS].0 >> (index % Limb::BITS)) & 1 == 1
        }
    }

    /// Calculate the number of bits needed to represent this number.
    pub const fn bits_vartime(&self) -> usize {
        let mut i = LIMBS - 1;
        while i > 0 && self.limbs[i].0 == 0 {
            i -= 1;
        }

        let limb = self.limbs[i];
        Limb::BITS * (i + 1) - limb.leading_zeros()
    }

    /// Calculate the number of leading zeros in the binary representation of this number.
    pub const fn leading_zeros(&self) -> usize {
        let limbs = self.as_limbs();

        let mut count: Word = 0;
        let mut i = LIMBS;
        let mut nonzero_limb_not_encountered = CtChoice::TRUE;
        while i > 0 {
            i -= 1;
            let l = limbs[i];
            let z = l.leading_zeros() as Word;
            count += nonzero_limb_not_encountered.if_true(z);
            nonzero_limb_not_encountered =
                nonzero_limb_not_encountered.and(l.ct_is_nonzero().not());
        }

        count as usize
    }

    /// Calculate the number of leading zeros in the binary representation of this number,
    /// variable time in `self`.
    pub const fn leading_zeros_vartime(&self) -> usize {
        let limbs = self.as_limbs();

        let mut count = 0;
        let mut i = LIMBS;
        while i > 0 {
            i -= 1;
            let l = limbs[i];
            let z = l.leading_zeros();
            count += z;
            if z != Limb::BITS {
                break;
            }
        }

        count
    }

    /// Calculate the number of trailing zeros in the binary representation of this number.
    pub const fn trailing_zeros(&self) -> usize {
        let limbs = self.as_limbs();

        let mut count: Word = 0;
        let mut i = 0;
        let mut nonzero_limb_not_encountered = CtChoice::TRUE;
        while i < LIMBS {
            let l = limbs[i];
            let z = l.trailing_zeros() as Word;
            count += nonzero_limb_not_encountered.if_true(z);
            nonzero_limb_not_encountered =
                nonzero_limb_not_encountered.and(l.ct_is_nonzero().not());
            i += 1;
        }

        count as usize
    }

    /// Calculate the number of trailing zeros in the binary representation of this number,
    /// variable time in `self`.
    pub const fn trailing_zeros_vartime(&self) -> usize {
        let limbs = self.as_limbs();

        let mut count = 0;
        let mut i = 0;
        while i < LIMBS {
            let l = limbs[i];
            let z = l.trailing_zeros();
            count += z;
            if z != Limb::BITS {
                break;
            }
            i += 1;
        }

        count
    }

    /// Calculate the number of trailing ones in the binary representation of this number.
    pub const fn trailing_ones(&self) -> usize {
        let limbs = self.as_limbs();

        let mut count: Word = 0;
        let mut i = 0;
        let mut nonmax_limb_not_encountered = CtChoice::TRUE;
        while i < LIMBS {
            let l = limbs[i];
            let z = l.trailing_ones() as Word;
            count += nonmax_limb_not_encountered.if_true(z);
            nonmax_limb_not_encountered =
                nonmax_limb_not_encountered.and(Limb::ct_eq(l, Limb::MAX));
            i += 1;
        }

        count as usize
    }

    /// Calculate the number of trailing ones in the binary representation of this number,
    /// variable time in `self`.
    pub const fn trailing_ones_vartime(&self) -> usize {
        let limbs = self.as_limbs();

        let mut count = 0;
        let mut i = 0;
        while i < LIMBS {
            let l = limbs[i];
            let z = l.trailing_ones();
            count += z;
            if z != Limb::BITS {
                break;
            }
            i += 1;
        }

        count
    }

    /// Calculate the number of bits needed to represent this number.
    pub const fn bits(&self) -> usize {
        Self::BITS - self.leading_zeros()
    }

    /// Get the value of the bit at position `index`, as a truthy or falsy `CtChoice`.
    /// Returns the falsy value for indices out of range.
    pub const fn bit(&self, index: usize) -> CtChoice {
        let limb_num = index / Limb::BITS;
        let index_in_limb = index % Limb::BITS;
        let index_mask = 1 << index_in_limb;

        let limbs = self.as_words();

        let mut result: Word = 0;
        let mut i = 0;
        while i < LIMBS {
            let bit = limbs[i] & index_mask;
            let is_right_limb = CtChoice::from_usize_equality(i, limb_num);
            result |= is_right_limb.if_true(bit);
            i += 1;
        }

        CtChoice::from_lsb(result >> index_in_limb)
    }

    /// Sets the bit at `index` to 0 or 1 depending on the value of `bit_value`.
    pub(crate) const fn set_bit(self, index: usize, bit_value: CtChoice) -> Self {
        let mut result = self;
        let limb_num = index / Limb::BITS;
        let index_in_limb = index % Limb::BITS;
        let index_mask = 1 << index_in_limb;

        let mut i = 0;
        while i < LIMBS {
            let is_right_limb = CtChoice::from_usize_equality(i, limb_num);
            let old_limb = result.limbs[i].0;
            let new_limb = bit_value.select(old_limb & !index_mask, old_limb | index_mask);
            result.limbs[i] = Limb(is_right_limb.select(old_limb, new_limb));
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{CtChoice, U256};

    fn uint_with_bits_at(positions: &[usize]) -> U256 {
        let mut result = U256::ZERO;
        for pos in positions {
            result |= U256::ONE << *pos;
        }
        result
    }

    #[test]
    fn bit_vartime() {
        let u = uint_with_bits_at(&[16, 48, 112, 127, 255]);
        assert!(!u.bit_vartime(0));
        assert!(!u.bit_vartime(1));
        assert!(u.bit_vartime(16));
        assert!(u.bit_vartime(127));
        assert!(u.bit_vartime(255));
        assert!(!u.bit_vartime(256));
        assert!(!u.bit_vartime(260));
    }

    #[test]
    fn bit() {
        let u = uint_with_bits_at(&[16, 48, 112, 127, 255]);
        assert!(!u.bit(0).is_true_vartime());
        assert!(!u.bit(1).is_true_vartime());
        assert!(u.bit(16).is_true_vartime());
        assert!(u.bit(127).is_true_vartime());
        assert!(u.bit(255).is_true_vartime());
        assert!(!u.bit(256).is_true_vartime());
        assert!(!u.bit(260).is_true_vartime());
    }

    #[test]
    fn leading_zeros() {
        let u = uint_with_bits_at(&[256 - 16, 256 - 79, 256 - 207]);
        assert_eq!(u.leading_zeros(), 15);

        let u = uint_with_bits_at(&[256 - 79, 256 - 207]);
        assert_eq!(u.leading_zeros(), 78);

        let u = uint_with_bits_at(&[256 - 207]);
        assert_eq!(u.leading_zeros(), 206);

        let u = uint_with_bits_at(&[256 - 1, 256 - 75, 256 - 150]);
        assert_eq!(u.leading_zeros(), 0);

        let u = U256::ZERO;
        assert_eq!(u.leading_zeros(), 256);
    }

    #[test]
    fn leading_zeros_vartime() {
        let u = uint_with_bits_at(&[256 - 16, 256 - 79, 256 - 207]);
        assert_eq!(u.leading_zeros_vartime(), 15);

        let u = uint_with_bits_at(&[256 - 79, 256 - 207]);
        assert_eq!(u.leading_zeros_vartime(), 78);

        let u = uint_with_bits_at(&[256 - 207]);
        assert_eq!(u.leading_zeros_vartime(), 206);

        let u = uint_with_bits_at(&[256 - 1, 256 - 75, 256 - 150]);
        assert_eq!(u.leading_zeros_vartime(), 0);

        let u = U256::ZERO;
        assert_eq!(u.leading_zeros_vartime(), 256);
    }

    #[test]
    fn trailing_zeros() {
        let u = uint_with_bits_at(&[16, 79, 150]);
        assert_eq!(u.trailing_zeros(), 16);

        let u = uint_with_bits_at(&[79, 150]);
        assert_eq!(u.trailing_zeros(), 79);

        let u = uint_with_bits_at(&[150, 207]);
        assert_eq!(u.trailing_zeros(), 150);

        let u = uint_with_bits_at(&[0, 150, 207]);
        assert_eq!(u.trailing_zeros(), 0);

        let u = U256::ZERO;
        assert_eq!(u.trailing_zeros(), 256);
    }

    #[test]
    fn trailing_zeros_vartime() {
        let u = uint_with_bits_at(&[16, 79, 150]);
        assert_eq!(u.trailing_zeros_vartime(), 16);

        let u = uint_with_bits_at(&[79, 150]);
        assert_eq!(u.trailing_zeros_vartime(), 79);

        let u = uint_with_bits_at(&[150, 207]);
        assert_eq!(u.trailing_zeros_vartime(), 150);

        let u = uint_with_bits_at(&[0, 150, 207]);
        assert_eq!(u.trailing_zeros_vartime(), 0);

        let u = U256::ZERO;
        assert_eq!(u.trailing_zeros_vartime(), 256);
    }

    #[test]
    fn trailing_ones() {
        let u = !uint_with_bits_at(&[16, 79, 150]);
        assert_eq!(u.trailing_ones(), 16);

        let u = !uint_with_bits_at(&[79, 150]);
        assert_eq!(u.trailing_ones(), 79);

        let u = !uint_with_bits_at(&[150, 207]);
        assert_eq!(u.trailing_ones(), 150);

        let u = !uint_with_bits_at(&[0, 150, 207]);
        assert_eq!(u.trailing_ones(), 0);

        let u = U256::MAX;
        assert_eq!(u.trailing_ones(), 256);
    }

    #[test]
    fn trailing_ones_vartime() {
        let u = !uint_with_bits_at(&[16, 79, 150]);
        assert_eq!(u.trailing_ones_vartime(), 16);

        let u = !uint_with_bits_at(&[79, 150]);
        assert_eq!(u.trailing_ones_vartime(), 79);

        let u = !uint_with_bits_at(&[150, 207]);
        assert_eq!(u.trailing_ones_vartime(), 150);

        let u = !uint_with_bits_at(&[0, 150, 207]);
        assert_eq!(u.trailing_ones_vartime(), 0);

        let u = U256::MAX;
        assert_eq!(u.trailing_ones_vartime(), 256);
    }

    #[test]
    fn set_bit() {
        let u = uint_with_bits_at(&[16, 79, 150]);
        assert_eq!(
            u.set_bit(127, CtChoice::TRUE),
            uint_with_bits_at(&[16, 79, 127, 150])
        );

        let u = uint_with_bits_at(&[16, 79, 150]);
        assert_eq!(
            u.set_bit(150, CtChoice::TRUE),
            uint_with_bits_at(&[16, 79, 150])
        );

        let u = uint_with_bits_at(&[16, 79, 150]);
        assert_eq!(
            u.set_bit(127, CtChoice::FALSE),
            uint_with_bits_at(&[16, 79, 150])
        );

        let u = uint_with_bits_at(&[16, 79, 150]);
        assert_eq!(
            u.set_bit(150, CtChoice::FALSE),
            uint_with_bits_at(&[16, 79])
        );
    }
}

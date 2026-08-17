use crate::{Limb, UInt, Word};

impl<const LIMBS: usize> UInt<LIMBS> {
    /// Get the value of the bit at position `index`, as a 0- or 1-valued Word.
    /// Returns 0 for indices out of range.
    #[inline(always)]
    pub const fn bit_vartime(self, index: usize) -> Word {
        if index >= LIMBS * Limb::BIT_SIZE {
            0
        } else {
            (self.limbs[index / Limb::BIT_SIZE].0 >> (index % Limb::BIT_SIZE)) & 1
        }
    }

    /// Calculate the number of bits needed to represent this number.
    #[deprecated(note = "please use `bits_vartime` instead")]
    #[inline(always)]
    pub const fn bits(self) -> usize {
        self.bits_vartime()
    }

    /// Calculate the number of bits needed to represent this number.
    #[allow(trivial_numeric_casts)]
    pub const fn bits_vartime(self) -> usize {
        let mut i = LIMBS - 1;
        while i > 0 && self.limbs[i].0 == 0 {
            i -= 1;
        }

        let limb = self.limbs[i].0;
        let bits = (Limb::BIT_SIZE * (i + 1)) as Word - limb.leading_zeros() as Word;

        Limb::ct_select(
            Limb(bits),
            Limb::ZERO,
            !self.limbs[0].is_nonzero() & !Limb(i as Word).is_nonzero(),
        )
        .0 as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    #[test]
    fn bit_vartime_ok() {
        let u = U128::from_be_hex("f0010000000000000001000000010000");
        assert_eq!(u.bit_vartime(0), 0);
        assert_eq!(u.bit_vartime(1), 0);
        assert_eq!(u.bit_vartime(16), 1);
        assert_eq!(u.bit_vartime(127), 1);
        assert_eq!(u.bit_vartime(130), 0);
    }
}

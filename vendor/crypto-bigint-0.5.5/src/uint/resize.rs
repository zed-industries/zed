use super::Uint;

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Construct a `Uint<T>` from the unsigned integer value,
    /// truncating the upper bits if the value is too large to be
    /// represented.
    #[inline(always)]
    pub const fn resize<const T: usize>(&self) -> Uint<T> {
        let mut res = Uint::ZERO;
        let mut i = 0;
        let dim = if T < LIMBS { T } else { LIMBS };
        while i < dim {
            res.limbs[i] = self.limbs[i];
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::{U128, U64};

    #[test]
    fn resize_larger() {
        let u = U64::from_be_hex("AAAAAAAABBBBBBBB");
        let u2: U128 = u.resize();
        assert_eq!(u2, U128::from_be_hex("0000000000000000AAAAAAAABBBBBBBB"));
    }

    #[test]
    fn resize_smaller() {
        let u = U128::from_be_hex("AAAAAAAABBBBBBBBCCCCCCCCDDDDDDDD");
        let u2: U64 = u.resize();
        assert_eq!(u2, U64::from_be_hex("CCCCCCCCDDDDDDDD"));
    }
}

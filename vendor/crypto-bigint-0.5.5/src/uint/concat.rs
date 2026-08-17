use crate::{Concat, ConcatMixed, Limb, Uint};

impl<T> Concat for T
where
    T: ConcatMixed<T>,
{
    type Output = Self::MixedOutput;
}

/// Concatenate the two values, with `lo` as least significant and `hi`
/// as the most significant.
#[inline]
pub(crate) const fn concat_mixed<const L: usize, const H: usize, const O: usize>(
    lo: &Uint<L>,
    hi: &Uint<H>,
) -> Uint<O> {
    let top = L + H;
    let top = if top < O { top } else { O };
    let mut limbs = [Limb::ZERO; O];
    let mut i = 0;

    while i < top {
        if i < L {
            limbs[i] = lo.limbs[i];
        } else {
            limbs[i] = hi.limbs[i - L];
        }
        i += 1;
    }

    Uint { limbs }
}

#[cfg(test)]
mod tests {
    use crate::{ConcatMixed, U128, U192, U64};

    #[test]
    fn concat() {
        let hi = U64::from_u64(0x0011223344556677);
        let lo = U64::from_u64(0x8899aabbccddeeff);
        assert_eq!(
            hi.concat(&lo),
            U128::from_be_hex("00112233445566778899aabbccddeeff")
        );
    }

    #[test]
    fn concat_mixed() {
        let a = U64::from_u64(0x0011223344556677);
        let b = U128::from_u128(0x8899aabbccddeeff_8899aabbccddeeff);
        assert_eq!(
            a.concat_mixed(&b),
            U192::from_be_hex("00112233445566778899aabbccddeeff8899aabbccddeeff")
        );
        assert_eq!(
            b.concat_mixed(&a),
            U192::from_be_hex("8899aabbccddeeff8899aabbccddeeff0011223344556677")
        );
    }

    #[test]
    fn convert() {
        let res: U128 = U64::ONE.mul_wide(&U64::ONE).into();
        assert_eq!(res, U128::ONE);

        let res: U128 = U64::ONE.square_wide().into();
        assert_eq!(res, U128::ONE);
    }
}

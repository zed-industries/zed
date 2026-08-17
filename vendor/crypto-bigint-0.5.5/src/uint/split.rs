use crate::{Limb, Uint};

/// Split this number in half, returning its high and low components
/// respectively.
#[inline]
pub(crate) const fn split_mixed<const L: usize, const H: usize, const O: usize>(
    n: &Uint<O>,
) -> (Uint<H>, Uint<L>) {
    let top = L + H;
    let top = if top < O { top } else { O };
    let mut lo = [Limb::ZERO; L];
    let mut hi = [Limb::ZERO; H];
    let mut i = 0;

    while i < top {
        if i < L {
            lo[i] = n.limbs[i];
        } else {
            hi[i - L] = n.limbs[i];
        }
        i += 1;
    }

    (Uint { limbs: hi }, Uint { limbs: lo })
}

#[cfg(test)]
mod tests {
    use crate::{U128, U64};

    #[test]
    fn split() {
        let (hi, lo) = U128::from_be_hex("00112233445566778899aabbccddeeff").split();
        assert_eq!(hi, U64::from_u64(0x0011223344556677));
        assert_eq!(lo, U64::from_u64(0x8899aabbccddeeff));
    }
}

use crate::{Limb, Uint, WideWord, Word};

/// Returns `(hi, lo)` such that `hi * R + lo = x * y + z + w`.
#[inline(always)]
const fn muladdcarry(x: Word, y: Word, z: Word, w: Word) -> (Word, Word) {
    let res = (x as WideWord)
        .wrapping_mul(y as WideWord)
        .wrapping_add(z as WideWord)
        .wrapping_add(w as WideWord);
    ((res >> Word::BITS) as Word, res as Word)
}

/// Algorithm 14.32 in Handbook of Applied Cryptography <https://cacr.uwaterloo.ca/hac/about/chap14.pdf>
pub const fn montgomery_reduction<const LIMBS: usize>(
    lower_upper: &(Uint<LIMBS>, Uint<LIMBS>),
    modulus: &Uint<LIMBS>,
    mod_neg_inv: Limb,
) -> Uint<LIMBS> {
    let (mut lower, mut upper) = *lower_upper;

    let mut meta_carry = Limb(0);
    let mut new_sum;

    let mut i = 0;
    while i < LIMBS {
        let u = lower.limbs[i].0.wrapping_mul(mod_neg_inv.0);

        let (mut carry, _) = muladdcarry(u, modulus.limbs[0].0, lower.limbs[i].0, 0);
        let mut new_limb;

        let mut j = 1;
        while j < (LIMBS - i) {
            (carry, new_limb) = muladdcarry(u, modulus.limbs[j].0, lower.limbs[i + j].0, carry);
            lower.limbs[i + j] = Limb(new_limb);
            j += 1;
        }
        while j < LIMBS {
            (carry, new_limb) =
                muladdcarry(u, modulus.limbs[j].0, upper.limbs[i + j - LIMBS].0, carry);
            upper.limbs[i + j - LIMBS] = Limb(new_limb);
            j += 1;
        }

        (new_sum, meta_carry) = upper.limbs[i].adc(Limb(carry), meta_carry);
        upper.limbs[i] = new_sum;

        i += 1;
    }

    // Division is simply taking the upper half of the limbs
    // Final reduction (at this point, the value is at most 2 * modulus,
    // so `meta_carry` is either 0 or 1)

    upper.sub_mod_with_carry(meta_carry, modulus, modulus)
}

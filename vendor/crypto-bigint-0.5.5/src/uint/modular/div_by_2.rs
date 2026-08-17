use crate::Uint;

pub(crate) fn div_by_2<const LIMBS: usize>(a: &Uint<LIMBS>, modulus: &Uint<LIMBS>) -> Uint<LIMBS> {
    // We are looking for such `x` that `x * 2 = y mod modulus`,
    // where the given `a = M(y)` is the Montgomery representation of some `y`.
    // This means that in Montgomery representation it would still apply:
    // `M(x) + M(x) = a mod modulus`.
    // So we can just forget about Montgomery representation, and return whatever is
    // `a` divided by 2, and this will be the Montgomery representation of `x`.
    // (Which means that this function works regardless of whether `a`
    // is in Montgomery representation or not, but the algorithm below
    // does need `modulus` to be odd)

    // Two possibilities:
    // - if `a` is even, we can just divide by 2;
    // - if `a` is odd, we divide `(a + modulus)` by 2.
    // To stay within the modulus we open the parentheses turning it into `a / 2 + modulus / 2 + 1`
    // ("+1" because both `a` and `modulus` are odd, we lose 0.5 in each integer division).
    // This will not overflow, so we can just use wrapping operations.

    let (half, is_odd) = a.shr_1();
    let half_modulus = modulus.shr_vartime(1);

    let if_even = half;
    let if_odd = half
        .wrapping_add(&half_modulus)
        .wrapping_add(&Uint::<LIMBS>::ONE);

    Uint::<LIMBS>::ct_select(&if_even, &if_odd, is_odd)
}

//! [`UInt`] subtraction modulus operations.

use crate::{Limb, SubMod, UInt};

impl<const LIMBS: usize> UInt<LIMBS> {
    /// Computes `self - rhs mod p` in constant time.
    ///
    /// Assumes `self - rhs` as unbounded signed integer is in `[-p, p)`.
    pub const fn sub_mod(&self, rhs: &UInt<LIMBS>, p: &UInt<LIMBS>) -> UInt<LIMBS> {
        let (mut out, borrow) = self.sbb(rhs, Limb::ZERO);

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the modulus.
        let mut carry = Limb::ZERO;
        let mut i = 0;

        while i < LIMBS {
            let (l, c) = out.limbs[i].adc(p.limbs[i].bitand(borrow), carry);
            out.limbs[i] = l;
            carry = c;
            i += 1;
        }

        out
    }

    /// Computes `self - rhs mod p` in constant time for the special modulus
    /// `p = MAX+1-c` where `c` is small enough to fit in a single [`Limb`].
    ///
    /// Assumes `self - rhs` as unbounded signed integer is in `[-p, p)`.
    pub const fn sub_mod_special(&self, rhs: &Self, c: Limb) -> Self {
        let (out, borrow) = self.sbb(rhs, Limb::ZERO);

        // If underflow occurred, then we need to subtract `c` to account for
        // the underflow. This cannot underflow due to the assumption
        // `self - rhs >= -p`.
        let l = borrow.0 & c.0;
        let (out, _) = out.sbb(&UInt::from_word(l), Limb::ZERO);
        out
    }
}

impl<const LIMBS: usize> SubMod for UInt<LIMBS> {
    type Output = Self;

    fn sub_mod(&self, rhs: &Self, p: &Self) -> Self {
        debug_assert!(self < p);
        debug_assert!(rhs < p);
        self.sub_mod(rhs, p)
    }
}

#[cfg(all(test, feature = "rand"))]
mod tests {
    use crate::{Limb, NonZero, Random, RandomMod, UInt};
    use rand_core::SeedableRng;

    macro_rules! test_sub_mod {
        ($size:expr, $test_name:ident) => {
            #[test]
            fn $test_name() {
                let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(1);
                let moduli = [
                    NonZero::<UInt<$size>>::random(&mut rng),
                    NonZero::<UInt<$size>>::random(&mut rng),
                ];

                for p in &moduli {
                    let base_cases = [
                        (1u64, 0u64, 1u64.into()),
                        (0, 1, p.wrapping_sub(&1u64.into())),
                        (0, 0, 0u64.into()),
                    ];
                    for (a, b, c) in &base_cases {
                        let a: UInt<$size> = (*a).into();
                        let b: UInt<$size> = (*b).into();

                        let x = a.sub_mod(&b, p);
                        assert_eq!(*c, x, "{} - {} mod {} = {} != {}", a, b, p, x, c);
                    }

                    if $size > 1 {
                        for _i in 0..100 {
                            let a: UInt<$size> = Limb::random(&mut rng).into();
                            let b: UInt<$size> = Limb::random(&mut rng).into();
                            let (a, b) = if a < b { (b, a) } else { (a, b) };

                            let c = a.sub_mod(&b, p);
                            assert!(c < **p, "not reduced");
                            assert_eq!(c, a.wrapping_sub(&b), "result incorrect");
                        }
                    }

                    for _i in 0..100 {
                        let a = UInt::<$size>::random_mod(&mut rng, p);
                        let b = UInt::<$size>::random_mod(&mut rng, p);

                        let c = a.sub_mod(&b, p);
                        assert!(c < **p, "not reduced: {} >= {} ", c, p);

                        let x = a.wrapping_sub(&b);
                        if a >= b && x < **p {
                            assert_eq!(c, x, "incorrect result");
                        }
                    }
                }
            }
        };
    }

    macro_rules! test_sub_mod_special {
        ($size:expr, $test_name:ident) => {
            #[test]
            fn $test_name() {
                let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(1);
                let moduli = [
                    NonZero::<Limb>::random(&mut rng),
                    NonZero::<Limb>::random(&mut rng),
                ];

                for special in &moduli {
                    let p = &NonZero::new(UInt::ZERO.wrapping_sub(&UInt::from_word(special.0)))
                        .unwrap();

                    let minus_one = p.wrapping_sub(&UInt::ONE);

                    let base_cases = [
                        (UInt::ZERO, UInt::ZERO, UInt::ZERO),
                        (UInt::ONE, UInt::ZERO, UInt::ONE),
                        (UInt::ZERO, UInt::ONE, minus_one),
                        (minus_one, minus_one, UInt::ZERO),
                        (UInt::ZERO, minus_one, UInt::ONE),
                    ];
                    for (a, b, c) in &base_cases {
                        let x = a.sub_mod_special(&b, *special.as_ref());
                        assert_eq!(*c, x, "{} - {} mod {} = {} != {}", a, b, p, x, c);
                    }

                    for _i in 0..100 {
                        let a = UInt::<$size>::random_mod(&mut rng, p);
                        let b = UInt::<$size>::random_mod(&mut rng, p);

                        let c = a.sub_mod_special(&b, *special.as_ref());
                        assert!(c < **p, "not reduced: {} >= {} ", c, p);

                        let expected = a.sub_mod(&b, p);
                        assert_eq!(c, expected, "incorrect result");
                    }
                }
            }
        };
    }

    // Test requires 1-limb is capable of representing a 64-bit integer
    #[cfg(target_pointer_width = "64")]
    test_sub_mod!(1, sub1);

    test_sub_mod!(2, sub2);
    test_sub_mod!(3, sub3);
    test_sub_mod!(4, sub4);
    test_sub_mod!(5, sub5);
    test_sub_mod!(6, sub6);
    test_sub_mod!(7, sub7);
    test_sub_mod!(8, sub8);
    test_sub_mod!(9, sub9);
    test_sub_mod!(10, sub10);
    test_sub_mod!(11, sub11);
    test_sub_mod!(12, sub12);

    test_sub_mod_special!(1, sub_mod_special_1);
    test_sub_mod_special!(2, sub_mod_special_2);
    test_sub_mod_special!(3, sub_mod_special_3);
    test_sub_mod_special!(4, sub_mod_special_4);
    test_sub_mod_special!(5, sub_mod_special_5);
    test_sub_mod_special!(6, sub_mod_special_6);
    test_sub_mod_special!(7, sub_mod_special_7);
    test_sub_mod_special!(8, sub_mod_special_8);
    test_sub_mod_special!(9, sub_mod_special_9);
    test_sub_mod_special!(10, sub_mod_special_10);
    test_sub_mod_special!(11, sub_mod_special_11);
    test_sub_mod_special!(12, sub_mod_special_12);
}

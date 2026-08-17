use crate::big_digit::{BigDigit, DoubleBigDigit, BITS};
use crate::bigint::Sign::*;
use crate::bigint::{BigInt, ToBigInt};
use crate::biguint::{BigUint, IntDigits};
use crate::integer::Integer;
use alloc::borrow::Cow;
use core::ops::Neg;
use num_traits::{One, Signed, Zero};

/// XGCD sets z to the greatest common divisor of a and b and returns z.
/// If extended is true, XGCD returns their value such that z = a*x + b*y.
///
/// Allow the inputs a and b to be zero or negative to GCD
/// with the following definitions.
///
/// If x or y are not nil, GCD sets their value such that z = a*x + b*y.
/// Regardless of the signs of a and b, z is always >= 0.
/// If a == b == 0, GCD sets z = x = y = 0.
/// If a == 0 and b != 0, GCD sets z = |b|, x = 0, y = sign(b) * 1.
/// If a != 0 and b == 0, GCD sets z = |a|, x = sign(a) * 1, y = 0.
pub fn xgcd(
    a_in: &BigInt,
    b_in: &BigInt,
    extended: bool,
) -> (BigInt, Option<BigInt>, Option<BigInt>) {
    //If a == b == 0, GCD sets z = x = y = 0.
    if a_in.is_zero() && b_in.is_zero() {
        if extended {
            return (0.into(), Some(0.into()), Some(0.into()));
        } else {
            return (0.into(), None, None);
        }
    }

    //If a == 0 and b != 0, GCD sets z = |b|, x = 0, y = sign(b) * 1.
    // if a_in.is_zero() && !b_in.is_zero() {
    if a_in.is_zero() {
        if extended {
            let mut y = BigInt::one();
            if b_in.sign == Minus {
                y.sign = Minus;
            }

            return (b_in.abs(), Some(0.into()), Some(y));
        } else {
            return (b_in.abs(), None, None);
        }
    }

    //If a != 0 and b == 0, GCD sets z = |a|, x = sign(a) * 1, y = 0.
    //if !a_in.is_zero() && b_in.is_zero() {
    if b_in.is_zero() {
        if extended {
            let mut x = BigInt::one();
            if a_in.sign == Minus {
                x.sign = Minus;
            }

            return (a_in.abs(), Some(x), Some(0.into()));
        } else {
            return (a_in.abs(), None, None);
        }
    }
    lehmer_gcd(a_in, b_in, extended)
}

/// lehmerGCD sets z to the greatest common divisor of a and b,
/// which both must be != 0, and returns z.
/// If x or y are not nil, their values are set such that z = a*x + b*y.
/// See Knuth, The Art of Computer Programming, Vol. 2, Section 4.5.2, Algorithm L.
/// This implementation uses the improved condition by Collins requiring only one
/// quotient and avoiding the possibility of single Word overflow.
/// See Jebelean, "Improving the multiprecision Euclidean algorithm",
/// Design and Implementation of Symbolic Computation Systems, pp 45-58.
/// The cosequences are updated according to Algorithm 10.45 from
/// Cohen et al. "Handbook of Elliptic and Hyperelliptic Curve Cryptography" pp 192.
fn lehmer_gcd(
    a_in: &BigInt,
    b_in: &BigInt,
    extended: bool,
) -> (BigInt, Option<BigInt>, Option<BigInt>) {
    let mut a = a_in.clone();
    let mut b = b_in.clone();

    //essential absolute value on both a & b
    a.sign = Plus;
    b.sign = Plus;

    // `ua` (`ub`) tracks how many times input `a_in` has beeen accumulated into `a` (`b`).
    let mut ua = if extended { Some(1.into()) } else { None };
    let mut ub = if extended { Some(0.into()) } else { None };

    // temp variables for multiprecision update
    let mut q: BigInt = 0.into();
    let mut r: BigInt = 0.into();
    let mut s: BigInt = 0.into();
    let mut t: BigInt = 0.into();

    // Ensure that a >= b
    if a < b {
        core::mem::swap(&mut a, &mut b);
        core::mem::swap(&mut ua, &mut ub);
    }

    // loop invariant A >= B
    while b.len() > 1 {
        // Attempt to calculate in single-precision using leading words of a and b.
        let (u0, u1, v0, v1, even) = lehmer_simulate(&a, &b);

        // multiprecision step
        if v0 != 0 {
            // Simulate the effect of the single-precision steps using cosequences.
            // a = u0 * a + v0 * b
            // b = u1 * a + v1 * b
            lehmer_update(
                &mut a, &mut b, &mut q, &mut r, &mut s, &mut t, u0, u1, v0, v1, even,
            );

            if extended {
                // ua = u0 * ua + v0 * ub
                // ub = u1 * ua + v1 * ub
                lehmer_update(
                    ua.as_mut().unwrap(),
                    ub.as_mut().unwrap(),
                    &mut q,
                    &mut r,
                    &mut s,
                    &mut t,
                    u0,
                    u1,
                    v0,
                    v1,
                    even,
                );
            }
        } else {
            // Single-digit calculations failed to simulate any quotients.
            euclid_udpate(
                &mut a, &mut b, &mut ua, &mut ub, &mut q, &mut r, &mut s, &mut t, extended,
            );
        }
    }

    if b.len() > 0 {
        // base case if b is a single digit
        if a.len() > 1 {
            // a is longer than a single word, so one update is needed
            euclid_udpate(
                &mut a, &mut b, &mut ua, &mut ub, &mut q, &mut r, &mut s, &mut t, extended,
            );
        }

        if b.len() > 0 {
            // a and b are both single word
            let mut a_word = a.digits()[0];
            let mut b_word = b.digits()[0];

            if extended {
                let mut ua_word: BigDigit = 1;
                let mut ub_word: BigDigit = 0;
                let mut va: BigDigit = 0;
                let mut vb: BigDigit = 1;
                let mut even = true;

                while b_word != 0 {
                    let q = a_word / b_word;
                    let r = a_word % b_word;
                    a_word = b_word;
                    b_word = r;

                    let k = ua_word.wrapping_add(q.wrapping_mul(ub_word));
                    ua_word = ub_word;
                    ub_word = k;

                    let k = va.wrapping_add(q.wrapping_mul(vb));
                    va = vb;
                    vb = k;
                    even = !even;
                }

                t.data.set_digit(ua_word);
                s.data.set_digit(va);
                t.sign = if even { Plus } else { Minus };
                s.sign = if even { Minus } else { Plus };

                if let Some(ua) = ua.as_mut() {
                    t *= &*ua;
                    s *= ub.unwrap();

                    *ua = &t + &s;
                }
            } else {
                while b_word != 0 {
                    let quotient = a_word % b_word;
                    a_word = b_word;
                    b_word = quotient;
                }
            }
            a.digits_mut()[0] = a_word;
        }
    }

    a.normalize();

    //Sign fixing
    let mut neg_a: bool = false;
    if a_in.sign == Minus {
        neg_a = true;
    }

    let y = if let Some(ref mut ua) = ua {
        // y = (z - a * x) / b

        //a_in*x
        let mut tmp = a_in * &*ua;

        if neg_a {
            tmp.sign = tmp.sign.neg();
            ua.sign = ua.sign.neg();
        }

        //z - (a_in * x)
        tmp = &a - &tmp;
        tmp = &tmp / b_in;

        Some(tmp)
    } else {
        None
    };

    a.sign = Plus;

    (a, ua, y)
}

/// Uses the lehemer algorithm.
/// Based on https://github.com/golang/go/blob/master/src/math/big/int.go#L612
/// If `extended` is set, the Bezout coefficients are calculated, otherwise they are `None`.
pub fn extended_gcd(
    a_in: Cow<BigUint>,
    b_in: Cow<BigUint>,
    extended: bool,
) -> (BigInt, Option<BigInt>, Option<BigInt>) {
    if a_in.is_zero() && b_in.is_zero() {
        if extended {
            return (b_in.to_bigint().unwrap(), Some(0.into()), Some(0.into()));
        } else {
            return (b_in.to_bigint().unwrap(), None, None);
        }
    }

    if a_in.is_zero() {
        if extended {
            return (b_in.to_bigint().unwrap(), Some(0.into()), Some(1.into()));
        } else {
            return (b_in.to_bigint().unwrap(), None, None);
        }
    }

    if b_in.is_zero() {
        if extended {
            return (a_in.to_bigint().unwrap(), Some(1.into()), Some(0.into()));
        } else {
            return (a_in.to_bigint().unwrap(), None, None);
        }
    }

    let a_in = a_in.to_bigint().unwrap();
    let b_in = b_in.to_bigint().unwrap();

    let mut a = a_in.clone();
    let mut b = b_in.clone();

    // `ua` (`ub`) tracks how many times input `a_in` has beeen accumulated into `a` (`b`).
    let mut ua = if extended { Some(1.into()) } else { None };
    let mut ub = if extended { Some(0.into()) } else { None };

    // Ensure that a >= b
    if a < b {
        core::mem::swap(&mut a, &mut b);
        core::mem::swap(&mut ua, &mut ub);
    }

    let mut q: BigInt = 0.into();
    let mut r: BigInt = 0.into();
    let mut s: BigInt = 0.into();
    let mut t: BigInt = 0.into();

    while b.len() > 1 {
        // Attempt to calculate in single-precision using leading words of a and b.
        let (u0, u1, v0, v1, even) = lehmer_simulate(&a, &b);

        // multiprecision step
        if v0 != 0 {
            // Simulate the effect of the single-precision steps using cosequences.
            // a = u0 * a + v0 * b
            // b = u1 * a + v1 * b
            lehmer_update(
                &mut a, &mut b, &mut q, &mut r, &mut s, &mut t, u0, u1, v0, v1, even,
            );

            if extended {
                // ua = u0 * ua + v0 * ub
                // ub = u1 * ua + v1 * ub
                lehmer_update(
                    ua.as_mut().unwrap(),
                    ub.as_mut().unwrap(),
                    &mut q,
                    &mut r,
                    &mut s,
                    &mut t,
                    u0,
                    u1,
                    v0,
                    v1,
                    even,
                );
            }
        } else {
            // Single-digit calculations failed to simulate any quotients.
            euclid_udpate(
                &mut a, &mut b, &mut ua, &mut ub, &mut q, &mut r, &mut s, &mut t, extended,
            );
        }
    }

    if b.len() > 0 {
        // base case if b is a single digit
        if a.len() > 1 {
            // a is longer than a single word, so one update is needed
            euclid_udpate(
                &mut a, &mut b, &mut ua, &mut ub, &mut q, &mut r, &mut s, &mut t, extended,
            );
        }

        if b.len() > 0 {
            // a and b are both single word
            let mut a_word = a.digits()[0];
            let mut b_word = b.digits()[0];

            if extended {
                let mut ua_word: BigDigit = 1;
                let mut ub_word: BigDigit = 0;
                let mut va: BigDigit = 0;
                let mut vb: BigDigit = 1;
                let mut even = true;

                while b_word != 0 {
                    let q = a_word / b_word;
                    let r = a_word % b_word;
                    a_word = b_word;
                    b_word = r;

                    let k = ua_word.wrapping_add(q.wrapping_mul(ub_word));
                    ua_word = ub_word;
                    ub_word = k;

                    let k = va.wrapping_add(q.wrapping_mul(vb));
                    va = vb;
                    vb = k;
                    even = !even;
                }

                t.data.set_digit(ua_word);
                s.data.set_digit(va);
                t.sign = if even { Plus } else { Minus };
                s.sign = if even { Minus } else { Plus };

                if let Some(ua) = ua.as_mut() {
                    t *= &*ua;
                    s *= ub.unwrap();

                    *ua = &t + &s;
                }
            } else {
                while b_word != 0 {
                    let quotient = a_word % b_word;
                    a_word = b_word;
                    b_word = quotient;
                }
            }
            a.digits_mut()[0] = a_word;
        }
    }

    a.normalize();

    let y = if let Some(ref ua) = ua {
        // y = (z - a * x) / b
        Some((&a - (&a_in * ua)) / &b_in)
    } else {
        None
    };

    (a, ua, y)
}

/// Attempts to simulate several Euclidean update steps using leading digits of `a` and `b`.
/// It returns `u0`, `u1`, `v0`, `v1` such that `a` and `b` can be updated as:
///     a = u0 * a + v0 * b
///     b = u1 * a + v1 * b
///
/// Requirements: `a >= b` and `b.len() > 2`.
/// Since we are calculating with full words to avoid overflow, `even` (the returned bool)
/// is used to track the sign of cosequences.
/// For even iterations: `u0, v1 >= 0 && u1, v0 <= 0`
/// For odd iterations: `u0, v1 <= && u1, v0 >= 0`
#[inline]
fn lehmer_simulate(a: &BigInt, b: &BigInt) -> (BigDigit, BigDigit, BigDigit, BigDigit, bool) {
    // m >= 2
    let m = b.len();
    // n >= m >= 2
    let n = a.len();

    // println!("a len is {:?}", a.len());
    // println!("b len is {:?}", b.len());

    // debug_assert!(m >= 2);
    // debug_assert!(n >= m);

    // extract the top word of bits from a and b
    let h = a.digits()[n - 1].leading_zeros();

    let mut a1: BigDigit = a.digits()[n - 1] << h
        | ((a.digits()[n - 2] as DoubleBigDigit) >> (BITS as u32 - h)) as BigDigit;

    // b may have implicit zero words in the high bits if the lengths differ
    let mut a2: BigDigit = if n == m {
        b.digits()[n - 1] << h
            | ((b.digits()[n - 2] as DoubleBigDigit) >> (BITS as u32 - h)) as BigDigit
    } else if n == m + 1 {
        ((b.digits()[n - 2] as DoubleBigDigit) >> (BITS as u32 - h)) as BigDigit
    } else {
        0
    };

    // odd, even tracking
    let mut even = false;

    let mut u0 = 0;
    let mut u1 = 1;
    let mut u2 = 0;

    let mut v0 = 0;
    let mut v1 = 0;
    let mut v2 = 1;

    // Calculate the quotient and cosequences using Collins' stoppting condition.
    while a2 >= v2 && a1.wrapping_sub(a2) >= v1 + v2 {
        let q = a1 / a2;
        let r = a1 % a2;

        a1 = a2;
        a2 = r;

        let k = u1 + q * u2;
        u0 = u1;
        u1 = u2;
        u2 = k;

        let k = v1 + q * v2;
        v0 = v1;
        v1 = v2;
        v2 = k;

        even = !even;
    }

    (u0, u1, v0, v1, even)
}

fn lehmer_update(
    a: &mut BigInt,
    b: &mut BigInt,
    q: &mut BigInt,
    r: &mut BigInt,
    s: &mut BigInt,
    t: &mut BigInt,
    u0: BigDigit,
    u1: BigDigit,
    v0: BigDigit,
    v1: BigDigit,
    even: bool,
) {
    t.data.set_digit(u0);
    s.data.set_digit(v0);
    if even {
        t.sign = Plus;
        s.sign = Minus
    } else {
        t.sign = Minus;
        s.sign = Plus;
    }

    *t *= &*a;
    *s *= &*b;

    r.data.set_digit(u1);
    q.data.set_digit(v1);
    if even {
        q.sign = Plus;
        r.sign = Minus
    } else {
        q.sign = Minus;
        r.sign = Plus;
    }

    *r *= &*a;
    *q *= &*b;

    *a = t + s;
    *b = r + q;
}

fn euclid_udpate(
    a: &mut BigInt,
    b: &mut BigInt,
    ua: &mut Option<BigInt>,
    ub: &mut Option<BigInt>,
    q: &mut BigInt,
    r: &mut BigInt,
    s: &mut BigInt,
    t: &mut BigInt,
    extended: bool,
) {
    let (q_new, r_new) = a.div_rem(b);
    *q = q_new;
    *r = r_new;

    core::mem::swap(a, b);
    core::mem::swap(b, r);

    if extended {
        // ua, ub = ub, ua - q * ub
        if let Some(ub) = ub.as_mut() {
            if let Some(ua) = ua.as_mut() {
                *t = ub.clone();
                *s = &*ub * &*q;
                *ub = &*ua - &*s;
                *ua = t.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    use num_traits::FromPrimitive;

    #[cfg(feature = "rand")]
    use crate::bigrand::RandBigInt;
    #[cfg(feature = "rand")]
    use num_traits::{One, Zero};
    #[cfg(feature = "rand")]
    use rand::SeedableRng;
    #[cfg(feature = "rand")]
    use rand_xorshift::XorShiftRng;

    #[cfg(feature = "rand")]
    fn extended_gcd_euclid(a: Cow<BigUint>, b: Cow<BigUint>) -> (BigInt, BigInt, BigInt) {
        // use crate::bigint::ToBigInt;

        if a.is_zero() && b.is_zero() {
            return (0.into(), 0.into(), 0.into());
        }

        let (mut s, mut old_s) = (BigInt::zero(), BigInt::one());
        let (mut t, mut old_t) = (BigInt::one(), BigInt::zero());
        let (mut r, mut old_r) = (b.to_bigint().unwrap(), a.to_bigint().unwrap());

        while !r.is_zero() {
            let quotient = &old_r / &r;
            old_r = old_r - &quotient * &r;
            core::mem::swap(&mut old_r, &mut r);
            old_s = old_s - &quotient * &s;
            core::mem::swap(&mut old_s, &mut s);
            old_t = old_t - quotient * &t;
            core::mem::swap(&mut old_t, &mut t);
        }

        (old_r, old_s, old_t)
    }

    #[test]
    #[cfg(feature = "rand")]
    fn test_extended_gcd_assumptions() {
        let mut rng = XorShiftRng::from_seed([1u8; 16]);

        for i in 1usize..100 {
            for j in &[1usize, 64, 128] {
                //println!("round {} - {}", i, j);
                let a = rng.gen_biguint(i * j);
                let b = rng.gen_biguint(i * j);
                let (q, s_k, t_k) = extended_gcd(Cow::Borrowed(&a), Cow::Borrowed(&b), true);

                let lhs = BigInt::from_biguint(Plus, a) * &s_k.unwrap();
                let rhs = BigInt::from_biguint(Plus, b) * &t_k.unwrap();

                assert_eq!(q.clone(), &lhs + &rhs, "{} = {} + {}", q, lhs, rhs);
            }
        }
    }

    #[test]
    fn test_extended_gcd_example() {
        // simple example for wikipedia
        let a = BigUint::from_u32(240).unwrap();
        let b = BigUint::from_u32(46).unwrap();
        let (q, s_k, t_k) = extended_gcd(Cow::Owned(a), Cow::Owned(b), true);

        assert_eq!(q, BigInt::from_i32(2).unwrap());
        assert_eq!(s_k.unwrap(), BigInt::from_i32(-9).unwrap());
        assert_eq!(t_k.unwrap(), BigInt::from_i32(47).unwrap());
    }

    #[test]
    fn test_extended_gcd_example_not_extended() {
        // simple example for wikipedia
        let a = BigUint::from_u32(240).unwrap();
        let b = BigUint::from_u32(46).unwrap();
        let (q, s_k, t_k) = extended_gcd(Cow::Owned(a), Cow::Owned(b), false);

        assert_eq!(q, BigInt::from_i32(2).unwrap());
        assert_eq!(s_k, None);
        assert_eq!(t_k, None);
    }

    #[test]
    fn test_extended_gcd_example_wolfram() {
        // https://www.wolframalpha.com/input/?i=ExtendedGCD%5B-565721958+,+4486780496%5D
        // https://github.com/Chia-Network/oldvdf-competition/blob/master/tests/test_classgroup.py#L109

        let a = BigInt::from_str("-565721958").unwrap();
        let b = BigInt::from_str("4486780496").unwrap();

        let (q, _s_k, _t_k) = xgcd(&a, &b, true);

        assert_eq!(q, BigInt::from(2));
        assert_eq!(_s_k, Some(BigInt::from(-1090996795)));
        assert_eq!(_t_k, Some(BigInt::from(-137559848)));
    }

    #[test]
    fn test_golang_bignum_negative() {
        // a <= 0 || b <= 0
        //d, x, y, a, b string
        let gcd_test_cases = [
            ["0", "0", "0", "0", "0"],
            ["7", "0", "1", "0", "7"],
            ["7", "0", "-1", "0", "-7"],
            ["11", "1", "0", "11", "0"],
            ["7", "-1", "-2", "-77", "35"],
            ["935", "-3", "8", "64515", "24310"],
            ["935", "-3", "-8", "64515", "-24310"],
            ["935", "3", "-8", "-64515", "-24310"],
            ["1", "-9", "47", "120", "23"],
            ["7", "1", "-2", "77", "35"],
            ["935", "-3", "8", "64515", "24310"],
            [
                "935000000000000000",
                "-3",
                "8",
                "64515000000000000000",
                "24310000000000000000",
            ],
            [
                "1",
                "-221",
                "22059940471369027483332068679400581064239780177629666810348940098015901108344",
                "98920366548084643601728869055592650835572950932266967461790948584315647051443",
                "991",
            ],
        ];

        for t in 0..gcd_test_cases.len() {
            //d, x, y, a, b string
            let d_case = BigInt::from_str(gcd_test_cases[t][0]).unwrap();
            let x_case = BigInt::from_str(gcd_test_cases[t][1]).unwrap();
            let y_case = BigInt::from_str(gcd_test_cases[t][2]).unwrap();
            let a_case = BigInt::from_str(gcd_test_cases[t][3]).unwrap();
            let b_case = BigInt::from_str(gcd_test_cases[t][4]).unwrap();

            // println!("round is {:?}", t);
            // println!("a len is {:?}", a_case.len());
            // println!("b len is {:?}", b_case.len());
            // println!("a is {:?}", &a_case);
            // println!("b is {:?}", &b_case);

            //testGcd(d, nil, nil, a, b)
            //testGcd(d, x, y, a, b)
            let (_d, _x, _y) = xgcd(&a_case, &b_case, false);

            assert_eq!(_d, d_case);
            assert_eq!(_x, None);
            assert_eq!(_y, None);

            let (_d, _x, _y) = xgcd(&a_case, &b_case, true);

            assert_eq!(_d, d_case);
            assert_eq!(_x.unwrap(), x_case);
            assert_eq!(_y.unwrap(), y_case);
        }
    }

    #[test]
    #[cfg(feature = "rand")]
    fn test_gcd_lehmer_euclid_extended() {
        let mut rng = XorShiftRng::from_seed([1u8; 16]);

        for i in 1usize..80 {
            for j in &[1usize, 16, 24, 64, 128] {
                //println!("round {} - {}", i, j);
                let a = rng.gen_biguint(i * j);
                let b = rng.gen_biguint(i * j);
                let (q, s_k, t_k) = extended_gcd(Cow::Borrowed(&a), Cow::Borrowed(&b), true);

                let expected = extended_gcd_euclid(Cow::Borrowed(&a), Cow::Borrowed(&b));
                assert_eq!(q, expected.0);
                assert_eq!(s_k.unwrap(), expected.1);
                assert_eq!(t_k.unwrap(), expected.2);
            }
        }
    }

    #[test]
    #[cfg(feature = "rand")]
    fn test_gcd_lehmer_euclid_not_extended() {
        let mut rng = XorShiftRng::from_seed([1u8; 16]);

        for i in 1usize..80 {
            for j in &[1usize, 16, 24, 64, 128] {
                //println!("round {} - {}", i, j);
                let a = rng.gen_biguint(i * j);
                let b = rng.gen_biguint(i * j);
                let (q, s_k, t_k) = extended_gcd(Cow::Borrowed(&a), Cow::Borrowed(&b), false);

                let expected = extended_gcd_euclid(Cow::Borrowed(&a), Cow::Borrowed(&b));
                assert_eq!(
                    q, expected.0,
                    "gcd({}, {}) = {} != {}",
                    &a, &b, &q, expected.0
                );
                assert_eq!(s_k, None);
                assert_eq!(t_k, None);
            }
        }
    }
}

//! Implementation of comparison operations
//!
//! Comparisons between decimals and decimal refs
//! are not directly supported as we lose some type
//! inference features at the savings of a single
//! '&' character.
//!
//! &BigDecimal and BigDecimalRef are comparable.
//!

use crate::*;

use stdlib::cmp::Ordering;
use stdlib::iter;

impl PartialEq for BigDecimal
{
    fn eq(&self, rhs: &BigDecimal) -> bool {
        self.to_ref() == rhs.to_ref()
    }
}

impl<'rhs, T> PartialEq<T> for BigDecimalRef<'_>
where
    T: Into<BigDecimalRef<'rhs>> + Copy
{
    fn eq(&self, rhs: &T) -> bool {
        let rhs: BigDecimalRef<'rhs> = (*rhs).into();
        check_equality_bigdecimal_ref(*self, rhs)
    }
}

fn check_equality_bigdecimal_ref(lhs: BigDecimalRef, rhs: BigDecimalRef) -> bool {
    match (lhs.sign(), rhs.sign()) {
        // both zero
        (Sign::NoSign, Sign::NoSign) => return true,
        // signs are different
        (a, b) if a != b => return false,
        // signs are same, do nothing
        _ => {}
    }

    let unscaled_int;
    let scaled_int;
    let trailing_zero_count;
    match arithmetic::checked_diff(lhs.scale, rhs.scale) {
        (Ordering::Equal, _) => {
            return lhs.digits == rhs.digits;
        }
        (Ordering::Greater, Some(scale_diff)) => {
            unscaled_int = lhs.digits;
            scaled_int = rhs.digits;
            trailing_zero_count = scale_diff;
        }
        (Ordering::Less, Some(scale_diff)) => {
            unscaled_int = rhs.digits;
            scaled_int = lhs.digits;
            trailing_zero_count = scale_diff;
        }
        _ => {
            // all other cases imply overflow in difference of scale,
            // numbers must not be equal
            return false;
        }
    }

    debug_assert_ne!(trailing_zero_count, 0);

    // test if unscaled_int is guaranteed to be less than
    // scaled_int*10^trailing_zero_count based on highest bit
    if highest_bit_lessthan_scaled(unscaled_int, scaled_int, trailing_zero_count) {
        return false;
    }

    // try compare without allocating
    if trailing_zero_count < 20 {
        let pow = ten_to_the_u64(trailing_zero_count as u8);

        let mut a_digits = unscaled_int.iter_u32_digits();
        let mut b_digits = scaled_int.iter_u32_digits();

        let mut carry = 0;
        loop {
            match (a_digits.next(), b_digits.next()) {
                (Some(next_a), Some(next_b)) => {
                    let wide_b = match (next_b as u64).checked_mul(pow) {
                        Some(tmp) => tmp + carry,
                        None => break,
                    };

                    let true_b = wide_b as u32;

                    if next_a != true_b {
                        return false;
                    }

                    carry = wide_b >> 32;
                }
                (None, Some(_)) => {
                    return false;
                }
                (Some(a_digit), None) => {
                    if a_digit != (carry as u32) {
                        return false
                    }
                    carry = 0;
                }
                (None, None) => {
                    return carry == 0;
                }
            }
        }

        // we broke out of loop due to overflow - compare via allocation
        let scaled_int = scaled_int * pow;
        return &scaled_int == unscaled_int;
    }

    let trailing_zero_count = trailing_zero_count.to_usize().unwrap();
    let unscaled_digits = unscaled_int.to_radix_le(10);

    if trailing_zero_count > unscaled_digits.len() {
        return false;
    }

    // split into digits below the other value, and digits overlapping
    let (low_digits, overlap_digits) = unscaled_digits.split_at(trailing_zero_count);

    // if any of the low digits are zero, they are not equal
    if low_digits.iter().any(|&d| d != 0) {
        return false;
    }

    let scaled_digits = scaled_int.to_radix_le(10);

    // different lengths with trailing zeros
    if overlap_digits.len() != scaled_digits.len() {
        return false;
    }

    // return true if all digits are the same
    overlap_digits.iter().zip(scaled_digits.iter()).all(|(digit_a, digit_b)| digit_a == digit_b)
}


impl PartialOrd for BigDecimal {
    #[inline]
    fn partial_cmp(&self, other: &BigDecimal) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for BigDecimalRef<'_> {
    fn partial_cmp(&self, other: &BigDecimalRef<'_>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for BigDecimal {
    #[inline]
    fn cmp(&self, other: &BigDecimal) -> Ordering {
        self.to_ref().cmp(&other.to_ref())
    }
}

impl Ord for BigDecimalRef<'_> {
    /// Complete ordering implementation for BigDecimal
    ///
    /// # Example
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// let a = bigdecimal::BigDecimal::from_str("-1").unwrap();
    /// let b = bigdecimal::BigDecimal::from_str("1").unwrap();
    /// assert!(a < b);
    /// assert!(b > a);
    /// let c = bigdecimal::BigDecimal::from_str("1").unwrap();
    /// assert!(b >= c);
    /// assert!(c >= b);
    /// let d = bigdecimal::BigDecimal::from_str("10.0").unwrap();
    /// assert!(d > c);
    /// let e = bigdecimal::BigDecimal::from_str(".5").unwrap();
    /// assert!(e < c);
    /// ```
    #[inline]
    fn cmp(&self, other: &BigDecimalRef) -> Ordering {
        use Ordering::*;

        let scmp = self.sign().cmp(&other.sign());
        if scmp != Ordering::Equal {
            return scmp;
        }

        if self.sign() == Sign::NoSign {
            return Ordering::Equal;
        }

        let result = match arithmetic::checked_diff(self.scale, other.scale) {
            (Greater, Some(scale_diff)) | (Equal, Some(scale_diff)) => {
                compare_scaled_biguints(self.digits, other.digits, scale_diff)
            }
            (Less, Some(scale_diff)) => {
                compare_scaled_biguints(other.digits, self.digits, scale_diff).reverse()
            }
            (res, None) => {
                // The difference in scale does not fit in a u64,
                // we can safely assume the value of digits do not matter
                // (unless we have a 2^64 (i.e. ~16 exabyte) long number

                // larger scale means smaller number, reverse this ordering
                res.reverse()
            }
        };

        if other.sign == Sign::Minus {
            result.reverse()
        } else {
            result
        }
    }
}


/// compare scaled uints: a <=> b * 10^{scale_diff}
///
fn compare_scaled_biguints(a: &BigUint, b: &BigUint, scale_diff: u64) -> Ordering {
    use Ordering::*;

    if scale_diff == 0 {
        return a.cmp(b);
    }

    // check if highest bit of a is less than b * 10^scale_diff
    if highest_bit_lessthan_scaled(a, b, scale_diff) {
        return Ordering::Less;
    }

    // if biguints fit it u64 or u128, compare using those (avoiding allocations)
    if let Some(result) = compare_scalar_biguints(a, b, scale_diff) {
        return result;
    }

    let a_digit_count = count_decimal_digits_uint(a);
    let b_digit_count = count_decimal_digits_uint(b);

    let digit_count_cmp = a_digit_count.cmp(&(b_digit_count + scale_diff));
    if digit_count_cmp != Equal {
        return digit_count_cmp;
    }

    let a_digits = a.to_radix_le(10);
    let b_digits = b.to_radix_le(10);

    debug_assert_eq!(a_digits.len(), a_digit_count as usize);
    debug_assert_eq!(b_digits.len(), b_digit_count as usize);

    let mut a_it = a_digits.iter().rev();
    let mut b_it = b_digits.iter().rev();

    loop {
        match (a_it.next(), b_it.next()) {
            (Some(ai), Some(bi)) => {
                match ai.cmp(bi) {
                    Equal => continue,
                    result => return result,
                }
            }
            (Some(&ai), None) => {
                if ai == 0 && a_it.all(Zero::is_zero) {
                    return Equal;
                } else {
                    return Greater;
                }
            }
            (None, Some(&bi)) => {
                if bi == 0 && b_it.all(Zero::is_zero) {
                    return Equal;
                } else {
                    return Less;
                }
            }
            (None, None) => {
                return Equal;
            }
        }
    }
}

/// Try fitting biguints into primitive integers, using those for ordering if possible
fn compare_scalar_biguints(a: &BigUint, b: &BigUint, scale_diff: u64) -> Option<Ordering> {
    let scale_diff = scale_diff.to_usize()?;

    // try u64, then u128
    compare_scaled_uints::<u64>(a, b, scale_diff)
    .or_else(|| compare_scaled_uints::<u128>(a, b, scale_diff))
}

/// Implementation comparing biguints cast to generic type
fn compare_scaled_uints<'a, T>(a: &'a BigUint, b: &'a BigUint, scale_diff: usize) -> Option<Ordering>
where
    T: num_traits::PrimInt + TryFrom<&'a BigUint>
{
    let ten = T::from(10).unwrap();

    let a = T::try_from(a).ok();
    let b = T::try_from(b).ok().and_then(
                |b| num_traits::checked_pow(ten, scale_diff).and_then(
                    |p| b.checked_mul(&p)));

    match (a, b) {
        (Some(a), Some(scaled_b)) => Some(a.cmp(&scaled_b)),
        // if scaled_b doesn't fit in size T, while 'a' does, then a is certainly less
        (Some(_), None) => Some(Ordering::Less),
        // if a doesn't fit in size T, while 'scaled_b' does, then a is certainly greater
        (None, Some(_)) => Some(Ordering::Greater),
        // neither fits, cannot determine relative size
        (None, None) => None,
    }
}

/// Return highest_bit(a) < highest_bit(b * 10^{scale})
///
/// Used for optimization when comparing scaled integers
///
/// ```math
/// a < b * 10^{scale}
/// log(a) < log(b) + scale * log(10)
/// ```
///
fn highest_bit_lessthan_scaled(a: &BigUint, b: &BigUint, scale: u64) -> bool {
    let a_bits = a.bits();
    let b_bits = b.bits();
    if a_bits < b_bits {
        return true;
    }
    let log_scale = LOG2_10 * scale as f64;
    match b_bits.checked_add(log_scale as u64) {
        Some(scaled_b_bit) => a_bits < scaled_b_bit,
        None => true, // overflowing u64 means we are definitely bigger
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod compare_scaled_biguints {
        use super::*;

        macro_rules! impl_test {
            ($name:ident: $a:literal > $b:literal e $e:literal) => {
                impl_test!($name: $a Greater $b e $e);
            };
            ($name:ident: $a:literal < $b:literal e $e:literal) => {
                impl_test!($name: $a Less $b e $e);
            };
            ($name:ident: $a:literal = $b:literal e $e:literal) => {
                impl_test!($name: $a Equal $b e $e);
            };
            ($name:ident: $a:literal $op:ident $b:literal e $e:literal) => {
                #[test]
                fn $name() {
                    let a: BigUint = $a.parse().unwrap();
                    let b: BigUint = $b.parse().unwrap();

                    let result = compare_scaled_biguints(&a, &b, $e);
                    assert_eq!(result, Ordering::$op);
                }
            };
        }

        impl_test!(case_500_51e1: "500" < "51" e 1);
        impl_test!(case_500_44e1: "500" > "44" e 1);
        impl_test!(case_5000_50e2: "5000" = "50" e 2);
        impl_test!(case_1234e9_12345e9: "1234000000000" < "12345" e 9);
        impl_test!(case_1116xx459_759xx717e2: "1116386634271380982470843247639640260491505327092723527088459" < "759522625769651746138617259189939751893902453291243506584717" e 2);
    }

    /// Test that large-magnitidue exponentials will not crash
    #[test]
    fn test_cmp_on_exp_boundaries() {
        let a = BigDecimal::new(1.into(), i64::MAX);
        let z = BigDecimal::new(1.into(), i64::MIN);
        assert_ne!(a, z);
        assert_ne!(z, a);

        assert!(a < z);

        assert_eq!(a, a);
        assert_eq!(z, z);
    }

    mod ord {
        use super::*;

        macro_rules! impl_test {
            ($name:ident: $a:literal < $b:literal) => {
                #[test]
                fn $name() {
                    let a: BigDecimal = $a.parse().unwrap();
                    let b: BigDecimal = $b.parse().unwrap();

                    assert!(&a < &b);
                    assert!(&b > &a);
                    assert_ne!(a, b);
                }
            };
        }

        impl_test!(case_diff_signs: "-1" < "1");
        impl_test!(case_n1_0: "-1" < "0");
        impl_test!(case_0_1: "0" < "1");
        impl_test!(case_1d2345_1d2346: "1.2345" < "1.2346");
        impl_test!(case_compare_extreme: "1e-9223372036854775807" < "1");
        impl_test!(case_compare_extremes: "1e-9223372036854775807" < "1e9223372036854775807");
        impl_test!(case_small_difference: "472697816888807260.1604" < "472697816888807260.16040000000000000000001");
        impl_test!(case_very_small_diff: "-1.0000000000000000000000000000000000000000000000000001" < "-1");

        impl_test!(case_1_2p128: "1" < "340282366920938463463374607431768211455");
        impl_test!(case_1_1e39: "1000000000000000000000000000000000000000" < "1e41");

        impl_test!(case_1d414xxx573: "1.414213562373095048801688724209698078569671875376948073176679730000000000000000000000000000000000000" < "1.41421356237309504880168872420969807856967187537694807317667974000000000");
        impl_test!(case_11d414xxx573: "1.414213562373095048801688724209698078569671875376948073176679730000000000000000000000000000000000000" < "11.41421356237309504880168872420969807856967187537694807317667974000000000");
    }

    mod eq {
        use super::*;

        macro_rules! impl_test {
            ($name:ident: $a:literal = $b:literal) => {
                #[test]
                fn $name() {
                    let a: BigDecimal = $a.parse().unwrap();
                    let b: BigDecimal = $b.parse().unwrap();

                    assert_eq!(&a, &b);
                    assert_eq!(a, b);
                }
            };
        }

        impl_test!(case_zero: "0" = "0.00");
        impl_test!(case_1_1d00: "1" = "1.00");
        impl_test!(case_n1_n1000en3: "-1" = "-1000e-3");
        impl_test!(case_0d000034500_345en7: "0.000034500" = "345e-7");
    }

    #[test]
    fn test_borrow_neg_cmp() {
        let x: BigDecimal = "1514932018891593.916341142773".parse().unwrap();
        let y: BigDecimal = "1514932018891593916341142773e-12".parse().unwrap();

        assert_eq!(x, y);

        let x_ref = x.to_ref();
        assert_eq!(x_ref, &y);
        assert_ne!(x_ref.neg(), x_ref);
        assert_eq!(x_ref.neg().neg(), x_ref);
    }

    #[cfg(property_tests)]
    mod prop {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #![proptest_config(ProptestConfig { cases: 5000, ..Default::default() })]

            #[test]
            fn cmp_matches_f64(
                f in proptest::num::f64::NORMAL | proptest::num::f64::SUBNORMAL | proptest::num::f64::ZERO,
                g in proptest::num::f64::NORMAL | proptest::num::f64::SUBNORMAL | proptest::num::f64::ZERO
            ) {
                let a: BigDecimal = BigDecimal::from_f64(f).unwrap();
                let b: BigDecimal = BigDecimal::from_f64(g).unwrap();

                let expected = PartialOrd::partial_cmp(&f, &g).unwrap();
                let value = a.cmp(&b);

                prop_assert_eq!(expected, value)
            }
        }
    }
}

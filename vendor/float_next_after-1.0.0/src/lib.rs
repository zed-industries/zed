#![no_std]

/// Returns the next representable float value in the direction of y
///
/// This function is strict and will step to the very next representable floating point,
/// even if that value is subnormal.
///
/// Base assumptions:
///
/// self == y -> return y
///
/// self >= positive infinity -> return positive infinity
///
/// self <= negative infinity -> return negative infinity
///
/// self == NaN -> return NaN
///
/// self == -0.0 and y == 0.0 -> return positive 0.0
///
/// self == -0.0 and y == positive infinity -> 5e-324
///
/// # Examples
///
/// ```
/// use float_next_after::NextAfter;
///
/// // Large numbers
/// let big_num = 16237485966.00000437586943_f64;
/// let next = big_num.next_after(std::f64::INFINITY);
/// assert_eq!(next, 16237485966.000006_f64);
///
/// // Expected handling of 1.0
/// let one = 1_f64;
/// let next = one.next_after(std::f64::INFINITY);
/// assert_eq!(next, 1_f64 + std::f64::EPSILON);
///
/// // Tiny (negative) numbers
/// let zero = 0_f32;
/// let next = zero.next_after(std::f32::NEG_INFINITY);
/// assert_eq!(next, -0.000000000000000000000000000000000000000000001_f32);
///
/// // Equal source/dest (even -0 == 0)
/// let zero = 0_f64;
/// let next = zero.next_after(-0_f64);
/// assert_eq!(next, -0_f64);
/// ```
///
/// # Safety
///
/// This trait uses the ToBits and FromBits functions from f32 and f64.
/// Those both use unsafe { mem::transmute(self) } / unsafe { mem::transmute(v) }
/// to convert a f32/f64 to u32/u64.  The docs for those functions claim they are safe
/// and that "the safety issues with sNaN were overblown!"
///
pub trait NextAfter {
    #[must_use]
    fn next_after(self, y: Self) -> Self;
}

macro_rules! impl_next_after {
    ($float_type:ty, $module:ident, $minimum_non_zero:literal) => {
        use super::NextAfter;

        impl NextAfter for $float_type {
            fn next_after(self, y: Self) -> Self {
                // Check first if the input matches a fixed set of values
                // that receive a pre-calculated response in line with the
                // stated rules.
                if let Some(out) = short_circuit_operands(self, y) {
                    return out;
                }

                // To get the next float, use trick fXX -> uXX +- 1 -> fXX
                // increment or decrement depending on same sign.
                let return_value = if (y > self) == (self > 0.0) {
                    <$float_type>::from_bits(self.to_bits() + 1)
                } else {
                    <$float_type>::from_bits(self.to_bits() - 1)
                };

                // Note that following the IEEE standard both 0.0 and -0.0 are equal in rust
                if return_value != 0.0 {
                    return return_value;
                }

                // At this point the return number must be zero
                // so make sure it returns with the input's original sign.
                copy_sign(return_value, self)
            }
        }

        #[inline]
        fn short_circuit_operands(x: $float_type, y: $float_type) -> Option<$float_type> {
            // If x and y are equal (also -0.0 == 0.0 in the IEEE standard and rust),
            // there is nothing further to do
            if y == x {
                // The policy is to return y in cases of equality,
                // this only matters when x = 0.0 and y = -0.0 (or the reverse)
                return Some(y);
            }

            // If x or y is NaN return NaN
            if x.is_nan() || y.is_nan() {
                return Some(core::$module::NAN);
            }

            if x.is_infinite() {
                return Some(x);
            }

            // If x is (+/-)0 and y is not 0 (see first condition),
            // return the hard coded value closest to zero and use
            // positive/negative sign to move it in the proper direction
            if x == 0.0 {
                return Some(copy_sign($minimum_non_zero, y));
            }

            None
        }

        #[inline]
        fn copy_sign(x: $float_type, y: $float_type) -> $float_type {
            if x.is_sign_positive() == y.is_sign_positive() {
                x
            } else {
                -x
            }
        }
    };
}

macro_rules! tests {
    ($float_type:ty, $module:ident, $smallest_pos:expr, $largest_neg:expr, $next_before_one:expr, $sequential_large_numbers:expr) => {
        #[cfg(test)]
        mod tests {
            use super::{copy_sign, NextAfter};

            const POS_INFINITY: $float_type = core::$module::INFINITY;
            const NEG_INFINITY: $float_type = core::$module::NEG_INFINITY;
            const POS_ZERO: $float_type = 0.0;
            const NEG_ZERO: $float_type = -0.0;

            // Note: Not the same as fXX::MIN_POSITIVE, because that is only the min *normal* number.
            const SMALLEST_POS: $float_type = $smallest_pos;
            const LARGEST_NEG: $float_type = $largest_neg;
            const LARGEST_POS: $float_type = core::$module::MAX;
            const SMALLEST_NEG: $float_type = core::$module::MIN;

            const POS_ONE: $float_type = 1.0;
            const NEG_ONE: $float_type = -1.0;
            const NEXT_LARGER_THAN_ONE: $float_type = 1.0 + core::$module::EPSILON;
            const NEXT_SMALLER_THAN_ONE: $float_type = $next_before_one;

            const SEQUENTIAL_LARGE_NUMBERS: ($float_type, $float_type) = $sequential_large_numbers;

            const NAN: $float_type = core::$module::NAN;

            fn is_positive_zero(x: $float_type) -> bool {
                x.to_bits() == POS_ZERO.to_bits()
            }

            fn is_negative_zero(x: $float_type) -> bool {
                x.to_bits() == NEG_ZERO.to_bits()
            }

            #[test]
            fn test_copy_sign() {
                assert_eq!(copy_sign(POS_ONE, POS_ZERO), POS_ONE);
                assert_eq!(copy_sign(NEG_ONE, POS_ZERO), POS_ONE);
                assert_eq!(copy_sign(POS_ONE, NEG_ZERO), NEG_ONE);
                assert_eq!(copy_sign(NEG_ONE, NEG_ZERO), NEG_ONE);
            }

            #[test]
            fn verify_constants() {
                assert_ne!(POS_ZERO.to_bits(), NEG_ZERO.to_bits());
                assert!(SMALLEST_POS > POS_ZERO);
                assert!(LARGEST_NEG < NEG_ZERO);
                assert!(!SMALLEST_POS.is_normal());
                assert!(!LARGEST_NEG.is_normal());
            }

            #[test]
            fn next_larger_than_0() {
                assert_eq!(POS_ZERO.next_after(POS_INFINITY), SMALLEST_POS);
                assert_eq!(NEG_ZERO.next_after(POS_INFINITY), SMALLEST_POS);
            }

            #[test]
            fn next_smaller_than_0() {
                assert_eq!(POS_ZERO.next_after(NEG_INFINITY), LARGEST_NEG);
                assert_eq!(NEG_ZERO.next_after(NEG_INFINITY), LARGEST_NEG);
            }

            #[test]
            fn step_towards_zero() {
                // For steps towards zero, the sign of the zero reflects the direction
                // from where zero was approached.
                assert!(is_positive_zero(SMALLEST_POS.next_after(POS_ZERO)));
                assert!(is_positive_zero(SMALLEST_POS.next_after(NEG_ZERO)));
                assert!(is_positive_zero(SMALLEST_POS.next_after(NEG_INFINITY)));
                assert!(is_negative_zero(LARGEST_NEG.next_after(NEG_ZERO)));
                assert!(is_negative_zero(LARGEST_NEG.next_after(POS_ZERO)));
                assert!(is_negative_zero(LARGEST_NEG.next_after(POS_INFINITY)));
            }

            #[test]
            fn special_case_signed_zeros() {
                // For a non-zero dest, stepping away from either POS_ZERO or NEG_ZERO
                // has a non-zero result. Only if the destination itself points to the
                // "other zero", the next_after call performs a zero sign switch.
                assert!(is_negative_zero(POS_ZERO.next_after(NEG_ZERO)));
                assert!(is_positive_zero(NEG_ZERO.next_after(POS_ZERO)));
            }

            #[test]
            fn nextafter_around_one() {
                assert_eq!(POS_ONE.next_after(POS_INFINITY), NEXT_LARGER_THAN_ONE);
                assert_eq!(POS_ONE.next_after(NEG_INFINITY), NEXT_SMALLER_THAN_ONE);
                assert_eq!(NEG_ONE.next_after(NEG_INFINITY), -NEXT_LARGER_THAN_ONE);
                assert_eq!(NEG_ONE.next_after(POS_INFINITY), -NEXT_SMALLER_THAN_ONE);
            }

            #[test]
            fn nextafter_for_big_positive_number() {
                let (lo, hi) = SEQUENTIAL_LARGE_NUMBERS;
                assert_eq!(lo.next_after(POS_INFINITY), hi);
                assert_eq!(hi.next_after(NEG_INFINITY), lo);
                assert_eq!(lo.next_after(hi), hi);
                assert_eq!(hi.next_after(lo), lo);
            }

            #[test]
            fn nextafter_for_small_negative_number() {
                let (lo, hi) = SEQUENTIAL_LARGE_NUMBERS;
                let (lo, hi) = (-lo, -hi);
                assert_eq!(lo.next_after(NEG_INFINITY), hi);
                assert_eq!(hi.next_after(POS_INFINITY), lo);
                assert_eq!(lo.next_after(hi), hi);
                assert_eq!(hi.next_after(lo), lo);
            }

            #[test]
            fn step_to_largest_is_possible() {
                let smaller = LARGEST_POS.next_after(NEG_INFINITY);
                assert_eq!(smaller.next_after(POS_INFINITY), LARGEST_POS);
                let smaller = SMALLEST_NEG.next_after(POS_INFINITY);
                assert_eq!(smaller.next_after(NEG_INFINITY), SMALLEST_NEG);
            }

            #[test]
            fn jump_to_infinity() {
                // Incrementing the max representable number has to go to infinity.
                assert_eq!(LARGEST_POS.next_after(POS_INFINITY), POS_INFINITY);
                assert_eq!(SMALLEST_NEG.next_after(NEG_INFINITY), NEG_INFINITY);
            }

            #[test]
            fn stays_at_infinity() {
                // Once infinity is reached, there is not going back to normal numbers
                assert_eq!(POS_INFINITY.next_after(NEG_INFINITY), POS_INFINITY);
                assert_eq!(NEG_INFINITY.next_after(POS_INFINITY), NEG_INFINITY);
            }

            #[test]
            fn returns_nan_for_any_nan_involved() {
                assert!(NAN.next_after(POS_ONE).is_nan());
                assert!(POS_ONE.next_after(NAN).is_nan());
                assert!(NAN.next_after(NAN).is_nan());
            }

            #[test]
            fn returns_identity_for_equal_dest() {
                let values = [
                    POS_ZERO,
                    NEG_ZERO,
                    POS_ONE,
                    NEG_ONE,
                    SEQUENTIAL_LARGE_NUMBERS.0,
                    SEQUENTIAL_LARGE_NUMBERS.1,
                    POS_INFINITY,
                    NEG_INFINITY,
                    SMALLEST_POS,
                    LARGEST_NEG,
                    LARGEST_POS,
                    SMALLEST_NEG,
                ];
                for x in values.iter() {
                    assert_eq!(x.next_after(*x), *x);
                }
            }

            #[test]
            fn can_successfully_roundtrip() {
                let values = [
                    POS_ONE,
                    NEG_ONE,
                    SEQUENTIAL_LARGE_NUMBERS.0,
                    SEQUENTIAL_LARGE_NUMBERS.1,
                    SMALLEST_POS,
                    LARGEST_NEG,
                ];
                for orig in values.to_vec() {
                    assert_eq!(orig.next_after(POS_INFINITY).next_after(NEG_INFINITY), orig);
                    assert_eq!(orig.next_after(NEG_INFINITY).next_after(POS_INFINITY), orig);

                    let upper = orig.next_after(POS_INFINITY);
                    let lower = orig.next_after(NEG_INFINITY);

                    assert_eq!(orig.next_after(upper).next_after(lower), orig);
                    assert_eq!(orig.next_after(lower).next_after(upper), orig);
                }
            }
        }
    };
}

mod f64 {
    impl_next_after!(f64, f64, 5e-324);

    tests!(
        f64,
        f64,
        5e-324,
        -5e-324,
        0.999_999_999_999_999_9,
        (16_237_485_966.000_004, 16_237_485_966.000_006)
    );
}

mod f32 {
    impl_next_after!(f32, f32, 1e-45);

    tests!(
        f32,
        f32,
        1e-45,
        -1e-45,
        0.999_999_94,
        (1.230_000_1e34, 1.230_000_3e34)
    );
}

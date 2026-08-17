use crate::constants::{MAX_SCALE_U32, POWERS_10, U32_MASK};

/// Rescales the given decimal to new scale.
/// e.g. with 1.23 and new scale 3 rescale the value to 1.230
#[inline]
pub(crate) fn rescale_internal(value: &mut [u32; 3], value_scale: &mut u32, new_scale: u32) {
    rescale::<true>(value, value_scale, new_scale);
}

#[inline(always)]
fn rescale<const ROUND: bool>(value: &mut [u32; 3], value_scale: &mut u32, new_scale: u32) {
    if *value_scale == new_scale {
        // Nothing to do
        return;
    }

    if is_all_zero(value) {
        *value_scale = new_scale.min(MAX_SCALE_U32);
        return;
    }

    if *value_scale > new_scale {
        let mut diff = value_scale.wrapping_sub(new_scale);
        // Scaling further isn't possible since we got an overflow
        // In this case we need to reduce the accuracy of the "side to keep"

        // Now do the necessary rounding
        let mut remainder = 0;
        while let Some(diff_minus_one) = diff.checked_sub(1) {
            if is_all_zero(value) {
                *value_scale = new_scale;
                return;
            }

            diff = diff_minus_one;

            // Any remainder is discarded if diff > 0 still (i.e. lost precision)
            remainder = div_by_u32(value, 10);
        }
        if ROUND && remainder >= 5 {
            for part in value.iter_mut() {
                let digit = u64::from(*part) + 1u64;
                remainder = if digit > U32_MASK { 1 } else { 0 };
                *part = (digit & U32_MASK) as u32;
                if remainder == 0 {
                    break;
                }
            }
        }
        *value_scale = new_scale;
    } else {
        let mut diff = new_scale.wrapping_sub(*value_scale);
        let mut working = [value[0], value[1], value[2]];
        while let Some(diff_minus_one) = diff.checked_sub(1) {
            if mul_by_10(&mut working) == 0 {
                value.copy_from_slice(&working);
                diff = diff_minus_one;
            } else {
                break;
            }
        }
        *value_scale = new_scale.wrapping_sub(diff);
    }
}

#[inline]
pub(crate) fn truncate_internal(value: &mut [u32; 3], value_scale: &mut u32, desired_scale: u32) {
    rescale::<false>(value, value_scale, desired_scale);
}

#[cfg(feature = "legacy-ops")]
pub(crate) fn add_by_internal(value: &mut [u32], by: &[u32]) -> u32 {
    let mut carry: u64 = 0;
    let vl = value.len();
    let bl = by.len();
    if vl >= bl {
        let mut sum: u64;
        for i in 0..bl {
            sum = u64::from(value[i]) + u64::from(by[i]) + carry;
            value[i] = (sum & U32_MASK) as u32;
            carry = sum >> 32;
        }
        if vl > bl && carry > 0 {
            for i in value.iter_mut().skip(bl) {
                sum = u64::from(*i) + carry;
                *i = (sum & U32_MASK) as u32;
                carry = sum >> 32;
                if carry == 0 {
                    break;
                }
            }
        }
    } else if vl + 1 == bl {
        // Overflow, by default, is anything in the high portion of by
        let mut sum: u64;
        for i in 0..vl {
            sum = u64::from(value[i]) + u64::from(by[i]) + carry;
            value[i] = (sum & U32_MASK) as u32;
            carry = sum >> 32;
        }
        if by[vl] > 0 {
            carry += u64::from(by[vl]);
        }
    } else {
        panic!("Internal error: add using incompatible length arrays. {} <- {}", vl, bl);
    }
    carry as u32
}

pub(crate) fn add_by_internal_flattened(value: &mut [u32; 3], by: u32) -> u32 {
    manage_add_by_internal(by, value)
}

#[inline]
pub(crate) fn add_one_internal(value: &mut [u32; 3]) -> u32 {
    manage_add_by_internal(1, value)
}

// `u64 as u32` are safe because of widening and 32bits shifts
#[inline]
pub(crate) fn manage_add_by_internal<const N: usize>(initial_carry: u32, value: &mut [u32; N]) -> u32 {
    let mut carry = u64::from(initial_carry);
    let mut iter = 0..value.len();
    let mut sum = 0;

    let mut sum_fn = |local_carry: &mut u64, idx| {
        sum = u64::from(value[idx]).wrapping_add(*local_carry);
        value[idx] = (sum & U32_MASK) as u32;
        *local_carry = sum.wrapping_shr(32);
    };

    if let Some(idx) = iter.next() {
        sum_fn(&mut carry, idx);
    }

    for idx in iter {
        if carry > 0 {
            sum_fn(&mut carry, idx);
        }
    }

    carry as u32
}

pub(crate) fn sub_by_internal(value: &mut [u32], by: &[u32]) -> u32 {
    // The way this works is similar to long subtraction
    // Let's assume we're working with bytes for simplicity in an example:
    //   257 - 8 = 249
    //   0000_0001 0000_0001 - 0000_0000 0000_1000 = 0000_0000 1111_1001
    // We start by doing the first byte...
    //   Overflow = 0
    //   Left = 0000_0001 (1)
    //   Right = 0000_1000 (8)
    // Firstly, we make sure the left and right are scaled up to twice the size
    //   Left = 0000_0000 0000_0001
    //   Right = 0000_0000 0000_1000
    // We then subtract right from left
    //   Result = Left - Right = 1111_1111 1111_1001
    // We subtract the overflow, which in this case is 0.
    // Because left < right (1 < 8) we invert the high part.
    //   Lo = 1111_1001
    //   Hi = 1111_1111 -> 0000_0001
    // Lo is the field, hi is the overflow.
    // We do the same for the second byte...
    //   Overflow = 1
    //   Left = 0000_0001
    //   Right = 0000_0000
    //   Result = Left - Right = 0000_0000 0000_0001
    // We subtract the overflow...
    //   Result = 0000_0000 0000_0001 - 1 = 0
    // And we invert the high, just because (invert 0 = 0).
    // So our result is:
    //   0000_0000 1111_1001
    let mut overflow = 0;
    let vl = value.len();
    let bl = by.len();
    for i in 0..vl {
        if i >= bl {
            break;
        }
        let (lo, hi) = sub_part(value[i], by[i], overflow);
        value[i] = lo;
        overflow = hi;
    }
    overflow
}

fn sub_part(left: u32, right: u32, overflow: u32) -> (u32, u32) {
    let part = 0x1_0000_0000u64 + u64::from(left) - (u64::from(right) + u64::from(overflow));
    let lo = part as u32;
    let hi = 1 - ((part >> 32) as u32);
    (lo, hi)
}

// Returns overflow
#[inline]
pub(crate) fn mul_by_10(bits: &mut [u32; 3]) -> u32 {
    let mut overflow = 0u64;
    for b in bits.iter_mut() {
        let result = u64::from(*b) * 10u64 + overflow;
        let hi = (result >> 32) & U32_MASK;
        let lo = (result & U32_MASK) as u32;
        *b = lo;
        overflow = hi;
    }

    overflow as u32
}

// Returns overflow
pub(crate) fn mul_by_u32(bits: &mut [u32], m: u32) -> u32 {
    let mut overflow = 0;
    for b in bits.iter_mut() {
        let (lo, hi) = mul_part(*b, m, overflow);
        *b = lo;
        overflow = hi;
    }
    overflow
}

pub(crate) fn mul_part(left: u32, right: u32, high: u32) -> (u32, u32) {
    let result = u64::from(left) * u64::from(right) + u64::from(high);
    let hi = ((result >> 32) & U32_MASK) as u32;
    let lo = (result & U32_MASK) as u32;
    (lo, hi)
}

// Returns remainder
pub(crate) fn div_by_u32<const N: usize>(bits: &mut [u32; N], divisor: u32) -> u32 {
    if divisor == 0 {
        // Divide by zero
        panic!("Internal error: divide by zero");
    } else if divisor == 1 {
        // dividend remains unchanged
        0
    } else {
        let mut remainder = 0u32;
        let divisor = u64::from(divisor);
        for part in bits.iter_mut().rev() {
            let temp = (u64::from(remainder) << 32) + u64::from(*part);
            remainder = (temp % divisor) as u32;
            *part = (temp / divisor) as u32;
        }

        remainder
    }
}

// This function should be used with caution. It unwraps the standard divide loop - it is intended
// for small inputs (<10) and is optimized to be left as unchecked.
pub(crate) fn div_by_power<const POWER: usize>(bits: &mut [u32; 3]) -> u32 {
    let mut remainder = 0u32;
    let divisor = POWERS_10[POWER] as u64;
    let temp = ((remainder as u64) << 32) + (bits[2] as u64);
    remainder = (temp % divisor) as u32;
    bits[2] = (temp / divisor) as u32;
    let temp = ((remainder as u64) << 32) + (bits[1] as u64);
    remainder = (temp % divisor) as u32;
    bits[1] = (temp / divisor) as u32;
    let temp = ((remainder as u64) << 32) + (bits[0] as u64);
    remainder = (temp % divisor) as u32;
    bits[0] = (temp / divisor) as u32;
    remainder
}

#[inline]
pub(crate) fn shl1_internal(bits: &mut [u32], carry: u32) -> u32 {
    let mut carry = carry;
    for part in bits.iter_mut() {
        let b = *part >> 31;
        *part = (*part << 1) | carry;
        carry = b;
    }
    carry
}

#[inline]
pub(crate) fn cmp_internal(left: &[u32; 3], right: &[u32; 3]) -> core::cmp::Ordering {
    let left_hi: u32 = left[2];
    let right_hi: u32 = right[2];
    let left_lo: u64 = (u64::from(left[1]) << 32) | u64::from(left[0]);
    let right_lo: u64 = (u64::from(right[1]) << 32) | u64::from(right[0]);
    if left_hi < right_hi || (left_hi <= right_hi && left_lo < right_lo) {
        core::cmp::Ordering::Less
    } else if left_hi == right_hi && left_lo == right_lo {
        core::cmp::Ordering::Equal
    } else {
        core::cmp::Ordering::Greater
    }
}

#[inline]
pub(crate) fn is_all_zero<const N: usize>(bits: &[u32; N]) -> bool {
    bits.iter().all(|b| *b == 0)
}

#[cfg(test)]
mod test {
    // Tests on private methods.
    //
    // All public tests should go under `tests/`.

    use super::*;
    use crate::prelude::*;

    fn to_mantissa_array_with_scale(value: &str) -> ([u32; 3], u32) {
        let v = Decimal::from_str(value).unwrap();
        (v.mantissa_array3(), v.scale())
    }

    #[test]
    fn it_can_rescale_internal() {
        let tests = &[
            ("1", 0, "1", 0),
            ("1", 1, "1.0", 1),
            ("1", 5, "1.00000", 5),
            ("1", 10, "1.0000000000", 10),
            ("1", 20, "1.00000000000000000000", 20),
            (
                "0.6386554621848739495798319328",
                27,
                "0.638655462184873949579831933",
                27,
            ),
            (
                "843.65000000", // Scale 8
                25,
                "843.6500000000000000000000000",
                25,
            ),
            (
                "843.65000000", // Scale 8
                30,
                "843.6500000000000000000000000",
                25, // Only fits 25
            ),
            ("0", 130, "0.000000000000000000000000000000", 28),
        ];

        for &(value_raw, new_scale, expected_value, expected_scale) in tests {
            let (expected_value, _) = to_mantissa_array_with_scale(expected_value);
            let (mut value, mut value_scale) = to_mantissa_array_with_scale(value_raw);
            rescale_internal(&mut value, &mut value_scale, new_scale);
            assert_eq!(value, expected_value);
            assert_eq!(
                value_scale, expected_scale,
                "value: {value_raw}, requested scale: {new_scale}"
            );
        }
    }

    #[test]
    fn test_shl1_internal() {
        struct TestCase {
            // One thing to be cautious of is that the structure of a number here for shifting left is
            // the reverse of how you may conceive this mentally. i.e. a[2] contains the higher order
            // bits: a[2] a[1] a[0]
            given: [u32; 3],
            given_carry: u32,
            expected: [u32; 3],
            expected_carry: u32,
        }
        let tests = [
            TestCase {
                given: [1, 0, 0],
                given_carry: 0,
                expected: [2, 0, 0],
                expected_carry: 0,
            },
            TestCase {
                given: [1, 0, 2147483648],
                given_carry: 1,
                expected: [3, 0, 0],
                expected_carry: 1,
            },
        ];
        for case in &tests {
            let mut test = [case.given[0], case.given[1], case.given[2]];
            let carry = shl1_internal(&mut test, case.given_carry);
            assert_eq!(
                test, case.expected,
                "Bits: {:?} << 1 | {}",
                case.given, case.given_carry
            );
            assert_eq!(
                carry, case.expected_carry,
                "Carry: {:?} << 1 | {}",
                case.given, case.given_carry
            )
        }
    }
}

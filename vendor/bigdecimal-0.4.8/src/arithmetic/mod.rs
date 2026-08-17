//! arithmetic routines

use crate::*;
use num_traits::CheckedSub;

pub(crate) mod addition;
pub(crate) mod sqrt;
pub(crate) mod cbrt;
pub(crate) mod inverse;

/// Return 10^pow
///
/// Try to calculate this with fewest number of allocations
///
pub(crate) fn ten_to_the(pow: u64) -> BigInt {
    ten_to_the_uint(pow).into()
}

/// Return 10^{pow} as u64
pub(crate) fn ten_to_the_u64(pow: u8) -> u64 {
    debug_assert!(pow < 20);
    10u64.pow(pow as u32)
}

/// Return 10^pow
pub(crate) fn ten_to_the_uint(pow: u64) -> BigUint {
    if pow < 20 {
        return BigUint::from(10u64.pow(pow as u32));
    }

    // linear case of 10^pow = 10^(19 * count + rem)
    if pow < 590 {
        let ten_to_nineteen = 10u64.pow(19);

        // count factors of 19
        let (count, rem) = pow.div_rem(&19);

        let mut res = BigUint::from(ten_to_nineteen);
        for _ in 1..count {
            res *= ten_to_nineteen;
        }
        if rem != 0 {
            res *= 10u64.pow(rem as u32);
        }

        return res;
    }

    // use recursive algorithm where linear case might be too slow
    let (quotient, rem) = pow.div_rem(&16);
    let x = ten_to_the_uint(quotient);

    let x2 = &x * &x;
    let x4 = &x2 * &x2;
    let x8 = &x4 * &x4;
    let res = &x8 * &x8;

    if rem == 0 {
        res
    } else {
        res * 10u64.pow(rem as u32)
    }
}

pub(crate) fn multiply_by_ten_to_the_uint<T, P>(n: &mut T, pow: P)
    where T: MulAssign<u64> + MulAssign<BigUint>,
          P: ToPrimitive
{
    let pow = pow.to_u64().expect("exponent overflow error");
    if pow < 20 {
        *n *= 10u64.pow(pow as u32);
    } else {
        *n *= ten_to_the_uint(pow);
    }

}

/// Return number of decimal digits in integer
pub(crate) fn count_decimal_digits(int: &BigInt) -> u64 {
    count_decimal_digits_uint(int.magnitude())
}

/// Return number of decimal digits in unsigned integer
pub(crate) fn count_decimal_digits_uint(uint: &BigUint) -> u64 {
    if uint.is_zero() {
        return 1;
    }
    let mut digits = (uint.bits() as f64 / LOG2_10) as u64;
    // guess number of digits based on number of bits in UInt
    let mut num = ten_to_the_uint(digits);
    while *uint >= num {
        num *= 10u8;
        digits += 1;
    }
    digits
}


/// Return difference of two numbers, returning diff as u64
pub(crate) fn diff<T>(a: T, b: T) -> (Ordering, u64)
where
    T: ToPrimitive + CheckedSub + stdlib::cmp::Ord
{
    use stdlib::cmp::Ordering::*;

    let (ord, diff) = checked_diff(a, b);

    (ord, diff.expect("subtraction overflow"))
}

/// Return difference of two numbers. If num doesn't fit in u64, return None
pub(crate) fn checked_diff<T>(a: T, b: T) -> (Ordering, Option<u64>)
where
    T: ToPrimitive + CheckedSub + stdlib::cmp::Ord
{
    use stdlib::cmp::Ordering::*;

    let _try_subtracting = |x:T, y:T| x.checked_sub(&y).and_then(|diff| diff.to_u64());

    match a.cmp(&b) {
        Less => (Less, _try_subtracting(b, a)),
        Greater => (Greater, _try_subtracting(a, b)),
        Equal => (Equal, Some(0)),
    }
}

/// Return difference of two numbers, returning diff as usize
#[allow(dead_code)]
pub(crate) fn diff_usize(a: usize, b: usize) -> (Ordering, usize) {
    use stdlib::cmp::Ordering::*;

    match a.cmp(&b) {
        Less => (Less, b - a),
        Greater => (Greater, a - b),
        Equal => (Equal, 0),
    }
}

/// Return absolute difference between two numbers
#[cfg(rustc_1_60)]
#[allow(clippy::incompatible_msrv)]
#[allow(dead_code)]
pub(crate) fn abs_diff(x: i64, y: i64) -> u64 {
    x.abs_diff(y)
}

#[cfg(not(rustc_1_60))]
#[allow(dead_code)]
pub(crate) fn abs_diff(x: i64, y: i64) -> u64 {
    (x as i128 - y as i128).to_u64().unwrap_or(0)
}


/// Add carry to given number, returning trimmed value and storing overflow back in carry
///
pub(crate) fn add_carry(n: u8, carry: &mut u8) -> u8 {
    let s = n + *carry;
    if s < 10 {
        *carry = 0;
        s
    } else {
        debug_assert!(s < 20);
        *carry = 1;
        s - 10
    }
}


/// If n is greater than 10, split and store overflow in carry
///
/// No action if n is less than 10.
///
/// Carry is not allowed to be 1 if n is two digits
///
pub(crate) fn store_carry(n: u8, carry: &mut u8) -> u8 {
    if n < 10 {
        n
    } else {
        debug_assert!(n < 20);
        debug_assert_eq!(carry, &0);
        *carry = 1;
        n - 10
    }
}


/// Extend destination vector with values in D, adding carry while carry is not zero
///
/// If carry overflows, it is NOT pushed into the destination vector.
///
pub(crate) fn extend_adding_with_carry<D: Iterator<Item=u8>>(
    dest: &mut Vec<u8>,
    mut digits: D,
    carry: &mut u8,
) {
    while *carry != 0 {
        match digits.next() {
            Some(d) => {
                dest.push(add_carry(d, carry))
            }
            None => {
                return;
            }
        }
    }
    dest.extend(digits);
}

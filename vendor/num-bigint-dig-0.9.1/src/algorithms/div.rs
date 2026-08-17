use core::cmp::Ordering;
use num_traits::{One, Zero};
use smallvec::SmallVec;

use crate::algorithms::{add2, cmp_slice, sub2};
use crate::big_digit::{self, BigDigit, DoubleBigDigit};
use crate::BigUint;

pub fn div_rem_digit(mut a: BigUint, b: BigDigit) -> (BigUint, BigDigit) {
    let mut rem = 0;

    for d in a.data.iter_mut().rev() {
        let (q, r) = div_wide(rem, *d, b);
        *d = q;
        rem = r;
    }

    (a.normalized(), rem)
}

/// Divide a two digit numerator by a one digit divisor, returns quotient and remainder:
///
/// Note: the caller must ensure that both the quotient and remainder will fit into a single digit.
/// This is _not_ true for an arbitrary numerator/denominator.
///
/// (This function also matches what the x86 divide instruction does).
#[inline]
pub fn div_wide(hi: BigDigit, lo: BigDigit, divisor: BigDigit) -> (BigDigit, BigDigit) {
    debug_assert!(hi < divisor);

    let lhs = big_digit::to_doublebigdigit(hi, lo);
    let rhs = divisor as DoubleBigDigit;
    ((lhs / rhs) as BigDigit, (lhs % rhs) as BigDigit)
}

pub fn div_rem(u: &BigUint, d: &BigUint) -> (BigUint, BigUint) {
    if d.is_zero() {
        panic!()
    }
    if u.is_zero() {
        return (Zero::zero(), Zero::zero());
    }
    if d.data.len() == 1 {
        if d.data[0] == 1 {
            return (u.clone(), Zero::zero());
        }

        let (div, rem) = div_rem_digit(u.clone(), d.data[0]);
        return (div, rem.into());
    }

    // Required or the q_len calculation below can underflow:
    match u.cmp(d) {
        Ordering::Less => return (Zero::zero(), u.clone()),
        Ordering::Equal => return (One::one(), Zero::zero()),
        Ordering::Greater => {} // Do nothing
    }

    // This algorithm is from Knuth, TAOCP vol 2 section 4.3, algorithm D:
    //
    // First, normalize the arguments so the highest bit in the highest digit of the divisor is
    // set: the main loop uses the highest digit of the divisor for generating guesses, so we
    // want it to be the largest number we can efficiently divide by.
    //
    let shift = d.data.last().unwrap().leading_zeros() as usize;
    let mut a = u << shift;
    let b = d << shift;

    // The algorithm works by incrementally calculating "guesses", q0, for part of the
    // remainder. Once we have any number q0 such that q0 * b <= a, we can set
    //
    //     q += q0
    //     a -= q0 * b
    //
    // and then iterate until a < b. Then, (q, a) will be our desired quotient and remainder.
    //
    // q0, our guess, is calculated by dividing the last few digits of a by the last digit of b
    // - this should give us a guess that is "close" to the actual quotient, but is possibly
    // greater than the actual quotient. If q0 * b > a, we simply use iterated subtraction
    // until we have a guess such that q0 * b <= a.
    //

    let bn = *b.data.last().unwrap();
    let q_len = a.data.len() - b.data.len() + 1;
    let mut q = BigUint {
        data: smallvec![0; q_len],
    };

    // We reuse the same temporary to avoid hitting the allocator in our inner loop - this is
    // sized to hold a0 (in the common case; if a particular digit of the quotient is zero a0
    // can be bigger).
    //
    let mut tmp = BigUint {
        data: SmallVec::with_capacity(2),
    };

    for j in (0..q_len).rev() {
        /*
         * When calculating our next guess q0, we don't need to consider the digits below j
         * + b.data.len() - 1: we're guessing digit j of the quotient (i.e. q0 << j) from
         * digit bn of the divisor (i.e. bn << (b.data.len() - 1) - so the product of those
         * two numbers will be zero in all digits up to (j + b.data.len() - 1).
         */
        let offset = j + b.data.len() - 1;
        if offset >= a.data.len() {
            continue;
        }

        /* just avoiding a heap allocation: */
        let mut a0 = tmp;
        a0.data.truncate(0);
        a0.data.extend(a.data[offset..].iter().cloned());

        /*
         * q0 << j * big_digit::BITS is our actual quotient estimate - we do the shifts
         * implicitly at the end, when adding and subtracting to a and q. Not only do we
         * save the cost of the shifts, the rest of the arithmetic gets to work with
         * smaller numbers.
         */
        let (mut q0, _) = div_rem_digit(a0, bn);
        let mut prod = &b * &q0;

        while cmp_slice(&prod.data[..], &a.data[j..]) == Ordering::Greater {
            let one: BigUint = One::one();
            q0 = q0 - one;
            prod = prod - &b;
        }

        add2(&mut q.data[j..], &q0.data[..]);
        sub2(&mut a.data[j..], &prod.data[..]);
        a.normalize();

        tmp = q0;
    }

    debug_assert!(a < b);

    (q.normalized(), a >> shift)
}

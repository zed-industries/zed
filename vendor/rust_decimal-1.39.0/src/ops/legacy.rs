use crate::{
    constants::{POWERS_10, U32_MASK},
    decimal::{CalculationResult, Decimal},
    ops::array::{
        add_by_internal, cmp_internal, div_by_u32, is_all_zero, mul_by_u32, mul_part, rescale_internal, shl1_internal,
    },
};

use core::cmp::Ordering;
use num_traits::Zero;

pub(crate) fn add_impl(d1: &Decimal, d2: &Decimal) -> CalculationResult {
    // Convert to the same scale
    let mut my = d1.mantissa_array3();
    let mut my_scale = d1.scale();
    let mut ot = d2.mantissa_array3();
    let mut other_scale = d2.scale();
    rescale_to_maximum_scale(&mut my, &mut my_scale, &mut ot, &mut other_scale);
    let mut final_scale = my_scale.max(other_scale);

    // Add the items together
    let my_negative = d1.is_sign_negative();
    let other_negative = d2.is_sign_negative();
    let mut negative = false;
    let carry;
    if !(my_negative ^ other_negative) {
        negative = my_negative;
        carry = add_by_internal3(&mut my, &ot);
    } else {
        let cmp = cmp_internal(&my, &ot);
        // -x + y
        // if x > y then it's negative (i.e. -2 + 1)
        match cmp {
            Ordering::Less => {
                negative = other_negative;
                sub_by_internal3(&mut ot, &my);
                my[0] = ot[0];
                my[1] = ot[1];
                my[2] = ot[2];
            }
            Ordering::Greater => {
                negative = my_negative;
                sub_by_internal3(&mut my, &ot);
            }
            Ordering::Equal => {
                // -2 + 2
                my[0] = 0;
                my[1] = 0;
                my[2] = 0;
            }
        }
        carry = 0;
    }

    // If we have a carry we underflowed.
    // We need to lose some significant digits (if possible)
    if carry > 0 {
        if final_scale == 0 {
            return CalculationResult::Overflow;
        }

        // Copy it over to a temp array for modification
        let mut temp = [my[0], my[1], my[2], carry];
        while final_scale > 0 && temp[3] != 0 {
            div_by_u32(&mut temp, 10);
            final_scale -= 1;
        }

        // If we still have a carry bit then we overflowed
        if temp[3] > 0 {
            return CalculationResult::Overflow;
        }

        // Copy it back - we're done
        my[0] = temp[0];
        my[1] = temp[1];
        my[2] = temp[2];
    }

    CalculationResult::Ok(Decimal::from_parts(my[0], my[1], my[2], negative, final_scale))
}

pub(crate) fn sub_impl(d1: &Decimal, d2: &Decimal) -> CalculationResult {
    add_impl(d1, &(-*d2))
}

pub(crate) fn div_impl(d1: &Decimal, d2: &Decimal) -> CalculationResult {
    if d2.is_zero() {
        return CalculationResult::DivByZero;
    }
    if d1.is_zero() {
        return CalculationResult::Ok(Decimal::zero());
    }

    let dividend = d1.mantissa_array3();
    let divisor = d2.mantissa_array3();
    let mut quotient = [0u32, 0u32, 0u32];
    let mut quotient_scale: i32 = d1.scale() as i32 - d2.scale() as i32;

    // We supply an extra overflow word for each of the dividend and the remainder
    let mut working_quotient = [dividend[0], dividend[1], dividend[2], 0u32];
    let mut working_remainder = [0u32, 0u32, 0u32, 0u32];
    let mut working_scale = quotient_scale;
    let mut remainder_scale = quotient_scale;
    let mut underflow;

    loop {
        div_internal(&mut working_quotient, &mut working_remainder, &divisor);
        underflow = add_with_scale_internal(
            &mut quotient,
            &mut quotient_scale,
            &mut working_quotient,
            &mut working_scale,
        );

        // Multiply the remainder by 10
        let mut overflow = 0;
        for part in working_remainder.iter_mut() {
            let (lo, hi) = mul_part(*part, 10, overflow);
            *part = lo;
            overflow = hi;
        }
        // Copy temp remainder into the temp quotient section
        working_quotient.copy_from_slice(&working_remainder);

        remainder_scale += 1;
        working_scale = remainder_scale;

        if underflow || is_all_zero(&working_remainder) {
            break;
        }
    }

    // If we have a really big number try to adjust the scale to 0
    while quotient_scale < 0 {
        copy_array_diff_lengths(&mut working_quotient, &quotient);
        working_quotient[3] = 0;
        working_remainder.iter_mut().for_each(|x| *x = 0);

        // Mul 10
        let mut overflow = 0;
        for part in &mut working_quotient {
            let (lo, hi) = mul_part(*part, 10, overflow);
            *part = lo;
            overflow = hi;
        }
        for part in &mut working_remainder {
            let (lo, hi) = mul_part(*part, 10, overflow);
            *part = lo;
            overflow = hi;
        }
        if working_quotient[3] == 0 && is_all_zero(&working_remainder) {
            quotient_scale += 1;
            quotient[0] = working_quotient[0];
            quotient[1] = working_quotient[1];
            quotient[2] = working_quotient[2];
        } else {
            // Overflow
            return CalculationResult::Overflow;
        }
    }

    if quotient_scale > 255 {
        quotient[0] = 0;
        quotient[1] = 0;
        quotient[2] = 0;
        quotient_scale = 0;
    }

    let mut quotient_negative = d1.is_sign_negative() ^ d2.is_sign_negative();

    // Check for underflow
    let mut final_scale: u32 = quotient_scale as u32;
    if final_scale > Decimal::MAX_SCALE {
        let mut remainder = 0;

        // Division underflowed. We must remove some significant digits over using
        //  an invalid scale.
        while final_scale > Decimal::MAX_SCALE && !is_all_zero(&quotient) {
            remainder = div_by_u32(&mut quotient, 10);
            final_scale -= 1;
        }
        if final_scale > Decimal::MAX_SCALE {
            // Result underflowed so set to zero
            final_scale = 0;
            quotient_negative = false;
        } else if remainder >= 5 {
            for part in &mut quotient {
                if remainder == 0 {
                    break;
                }
                let digit: u64 = u64::from(*part) + 1;
                remainder = if digit > 0xFFFF_FFFF { 1 } else { 0 };
                *part = (digit & 0xFFFF_FFFF) as u32;
            }
        }
    }

    CalculationResult::Ok(Decimal::from_parts(
        quotient[0],
        quotient[1],
        quotient[2],
        quotient_negative,
        final_scale,
    ))
}

pub(crate) fn mul_impl(d1: &Decimal, d2: &Decimal) -> CalculationResult {
    // Early exit if either is zero
    if d1.is_zero() || d2.is_zero() {
        return CalculationResult::Ok(Decimal::zero());
    }

    // We are only resulting in a negative if we have mismatched signs
    let negative = d1.is_sign_negative() ^ d2.is_sign_negative();

    // We get the scale of the result by adding the operands. This may be too big, however
    //  we'll correct later
    let mut final_scale = d1.scale() + d2.scale();

    // First of all, if ONLY the lo parts of both numbers is filled
    // then we can simply do a standard 64 bit calculation. It's a minor
    // optimization however prevents the need for long form multiplication
    let my = d1.mantissa_array3();
    let ot = d2.mantissa_array3();
    if my[1] == 0 && my[2] == 0 && ot[1] == 0 && ot[2] == 0 {
        // Simply multiplication
        let mut u64_result = u64_to_array(u64::from(my[0]) * u64::from(ot[0]));

        // If we're above max precision then this is a very small number
        if final_scale > Decimal::MAX_SCALE {
            final_scale -= Decimal::MAX_SCALE;

            // If the number is above 19 then this will equate to zero.
            // This is because the max value in 64 bits is 1.84E19
            if final_scale > 19 {
                return CalculationResult::Ok(Decimal::zero());
            }

            let mut rem_lo = 0;
            let mut power;
            if final_scale > 9 {
                // Since 10^10 doesn't fit into u32, we divide by 10^10/4
                // and multiply the next divisor by 4.
                rem_lo = div_by_u32(&mut u64_result, 2_500_000_000);
                power = POWERS_10[final_scale as usize - 10] << 2;
            } else {
                power = POWERS_10[final_scale as usize];
            }

            // Divide fits in 32 bits
            let rem_hi = div_by_u32(&mut u64_result, power);

            // Round the result. Since the divisor is a power of 10
            // we check to see if the remainder is >= 1/2 divisor
            power >>= 1;
            if rem_hi >= power && (rem_hi > power || (rem_lo | (u64_result[0] & 0x1)) != 0) {
                u64_result[0] += 1;
            }

            final_scale = Decimal::MAX_SCALE;
        }
        return CalculationResult::Ok(Decimal::from_parts(
            u64_result[0],
            u64_result[1],
            0,
            negative,
            final_scale,
        ));
    }

    // We're using some of the high bits, so we essentially perform
    // long form multiplication. We compute the 9 partial products
    // into a 192 bit result array.
    //
    //                     [my-h][my-m][my-l]
    //                  x  [ot-h][ot-m][ot-l]
    // --------------------------------------
    // 1.                        [r-hi][r-lo] my-l * ot-l [0, 0]
    // 2.                  [r-hi][r-lo]       my-l * ot-m [0, 1]
    // 3.                  [r-hi][r-lo]       my-m * ot-l [1, 0]
    // 4.            [r-hi][r-lo]             my-m * ot-m [1, 1]
    // 5.            [r-hi][r-lo]             my-l * ot-h [0, 2]
    // 6.            [r-hi][r-lo]             my-h * ot-l [2, 0]
    // 7.      [r-hi][r-lo]                   my-m * ot-h [1, 2]
    // 8.      [r-hi][r-lo]                   my-h * ot-m [2, 1]
    // 9.[r-hi][r-lo]                         my-h * ot-h [2, 2]
    let mut product = [0u32, 0u32, 0u32, 0u32, 0u32, 0u32];

    // We can perform a minor short circuit here. If the
    // high portions are both 0 then we can skip portions 5-9
    let to = if my[2] == 0 && ot[2] == 0 { 2 } else { 3 };

    for (my_index, my_item) in my.iter().enumerate().take(to) {
        for (ot_index, ot_item) in ot.iter().enumerate().take(to) {
            let (mut rlo, mut rhi) = mul_part(*my_item, *ot_item, 0);

            // Get the index for the lo portion of the product
            for prod in product.iter_mut().skip(my_index + ot_index) {
                let (res, overflow) = add_part(rlo, *prod);
                *prod = res;

                // If we have something in rhi from before then promote that
                if rhi > 0 {
                    // If we overflowed in the last add, add that with rhi
                    if overflow > 0 {
                        let (nlo, nhi) = add_part(rhi, overflow);
                        rlo = nlo;
                        rhi = nhi;
                    } else {
                        rlo = rhi;
                        rhi = 0;
                    }
                } else if overflow > 0 {
                    rlo = overflow;
                    rhi = 0;
                } else {
                    break;
                }

                // If nothing to do next round then break out
                if rlo == 0 {
                    break;
                }
            }
        }
    }

    // If our result has used up the high portion of the product
    // then we either have an overflow or an underflow situation
    // Overflow will occur if we can't scale it back, whereas underflow
    // with kick in rounding
    let mut remainder = 0;
    while final_scale > 0 && (product[3] != 0 || product[4] != 0 || product[5] != 0) {
        remainder = div_by_u32(&mut product, 10u32);
        final_scale -= 1;
    }

    // Round up the carry if we need to
    if remainder >= 5 {
        for part in product.iter_mut() {
            if remainder == 0 {
                break;
            }
            let digit: u64 = u64::from(*part) + 1;
            remainder = if digit > 0xFFFF_FFFF { 1 } else { 0 };
            *part = (digit & 0xFFFF_FFFF) as u32;
        }
    }

    // If we're still above max precision then we'll try again to
    // reduce precision - we may be dealing with a limit of "0"
    if final_scale > Decimal::MAX_SCALE {
        // We're in an underflow situation
        // The easiest way to remove precision is to divide off the result
        while final_scale > Decimal::MAX_SCALE && !is_all_zero(&product) {
            div_by_u32(&mut product, 10);
            final_scale -= 1;
        }
        // If we're still at limit then we can't represent any
        // significant decimal digits and will return an integer only
        // Can also be invoked while representing 0.
        if final_scale > Decimal::MAX_SCALE {
            final_scale = 0;
        }
    } else if !(product[3] == 0 && product[4] == 0 && product[5] == 0) {
        // We're in an overflow situation - we're within our precision bounds
        // but still have bits in overflow
        return CalculationResult::Overflow;
    }

    CalculationResult::Ok(Decimal::from_parts(
        product[0],
        product[1],
        product[2],
        negative,
        final_scale,
    ))
}

pub(crate) fn rem_impl(d1: &Decimal, d2: &Decimal) -> CalculationResult {
    if d2.is_zero() {
        return CalculationResult::DivByZero;
    }
    if d1.is_zero() {
        return CalculationResult::Ok(Decimal::zero());
    }

    // Rescale so comparable
    let initial_scale = d1.scale();
    let mut quotient = d1.mantissa_array3();
    let mut quotient_scale = initial_scale;
    let mut divisor = d2.mantissa_array3();
    let mut divisor_scale = d2.scale();
    rescale_to_maximum_scale(&mut quotient, &mut quotient_scale, &mut divisor, &mut divisor_scale);

    // Working is the remainder + the quotient
    // We use an aligned array since we'll be using it a lot.
    let mut working_quotient = [quotient[0], quotient[1], quotient[2], 0u32];
    let mut working_remainder = [0u32, 0u32, 0u32, 0u32];
    div_internal(&mut working_quotient, &mut working_remainder, &divisor);

    // Round if necessary. This is for semantic correctness, but could feasibly be removed for
    // performance improvements.
    if quotient_scale > initial_scale {
        let mut working = [
            working_remainder[0],
            working_remainder[1],
            working_remainder[2],
            working_remainder[3],
        ];
        while quotient_scale > initial_scale {
            if div_by_u32(&mut working, 10) > 0 {
                break;
            }
            quotient_scale -= 1;
            working_remainder.copy_from_slice(&working);
        }
    }

    CalculationResult::Ok(Decimal::from_parts(
        working_remainder[0],
        working_remainder[1],
        working_remainder[2],
        d1.is_sign_negative(),
        quotient_scale,
    ))
}

pub(crate) fn cmp_impl(d1: &Decimal, d2: &Decimal) -> Ordering {
    // Quick exit if major differences
    if d1.is_zero() && d2.is_zero() {
        return Ordering::Equal;
    }
    let self_negative = d1.is_sign_negative();
    let other_negative = d2.is_sign_negative();
    if self_negative && !other_negative {
        return Ordering::Less;
    } else if !self_negative && other_negative {
        return Ordering::Greater;
    }

    // If we have 1.23 and 1.2345 then we have
    //  123 scale 2 and 12345 scale 4
    //  We need to convert the first to
    //  12300 scale 4 so we can compare equally
    let left: &Decimal;
    let right: &Decimal;
    if self_negative && other_negative {
        // Both are negative, so reverse cmp
        left = d2;
        right = d1;
    } else {
        left = d1;
        right = d2;
    }
    let mut left_scale = left.scale();
    let mut right_scale = right.scale();
    let mut left_raw = left.mantissa_array3();
    let mut right_raw = right.mantissa_array3();

    if left_scale == right_scale {
        // Fast path for same scale
        if left_raw[2] != right_raw[2] {
            return left_raw[2].cmp(&right_raw[2]);
        }
        if left_raw[1] != right_raw[1] {
            return left_raw[1].cmp(&right_raw[1]);
        }
        return left_raw[0].cmp(&right_raw[0]);
    }

    // Rescale and compare
    rescale_to_maximum_scale(&mut left_raw, &mut left_scale, &mut right_raw, &mut right_scale);
    cmp_internal(&left_raw, &right_raw)
}

#[inline]
fn add_part(left: u32, right: u32) -> (u32, u32) {
    let added = u64::from(left) + u64::from(right);
    ((added & U32_MASK) as u32, ((added >> 32) & U32_MASK) as u32)
}

#[inline(always)]
fn sub_by_internal3(value: &mut [u32; 3], by: &[u32; 3]) {
    let mut overflow = 0;
    let vl = value.len();
    for i in 0..vl {
        let part = (0x1_0000_0000u64 + u64::from(value[i])) - (u64::from(by[i]) + overflow);
        value[i] = part as u32;
        overflow = 1 - (part >> 32);
    }
}

fn div_internal(quotient: &mut [u32; 4], remainder: &mut [u32; 4], divisor: &[u32; 3]) {
    // There are a couple of ways to do division on binary numbers:
    //   1. Using long division
    //   2. Using the complement method
    // ref: http://paulmason.me/dividing-binary-numbers-part-2/
    // The complement method basically keeps trying to subtract the
    // divisor until it can't anymore and placing the rest in remainder.
    let mut complement = [
        divisor[0] ^ 0xFFFF_FFFF,
        divisor[1] ^ 0xFFFF_FFFF,
        divisor[2] ^ 0xFFFF_FFFF,
        0xFFFF_FFFF,
    ];

    // Add one onto the complement
    add_one_internal4(&mut complement);

    // Make sure the remainder is 0
    remainder.iter_mut().for_each(|x| *x = 0);

    // If we have nothing in our hi+ block then shift over till we do
    let mut blocks_to_process = 0;
    while blocks_to_process < 4 && quotient[3] == 0 {
        // memcpy would be useful here
        quotient[3] = quotient[2];
        quotient[2] = quotient[1];
        quotient[1] = quotient[0];
        quotient[0] = 0;

        // Increment the counter
        blocks_to_process += 1;
    }

    // Let's try and do the addition...
    let mut block = blocks_to_process << 5;
    let mut working = [0u32, 0u32, 0u32, 0u32];
    while block < 128 {
        // << 1 for quotient AND remainder. Moving the carry from the quotient to the bottom of the
        // remainder.
        let carry = shl1_internal(quotient, 0);
        shl1_internal(remainder, carry);

        // Copy the remainder of working into sub
        working.copy_from_slice(remainder);

        // Add the remainder with the complement
        add_by_internal(&mut working, &complement);

        // Check for the significant bit - move over to the quotient
        // as necessary
        if (working[3] & 0x8000_0000) == 0 {
            remainder.copy_from_slice(&working);
            quotient[0] |= 1;
        }

        // Increment our pointer
        block += 1;
    }
}

#[inline]
fn copy_array_diff_lengths(into: &mut [u32], from: &[u32]) {
    for i in 0..into.len() {
        if i >= from.len() {
            break;
        }
        into[i] = from[i];
    }
}

#[inline]
fn add_one_internal4(value: &mut [u32; 4]) -> u32 {
    let mut carry: u64 = 1; // Start with one, since adding one
    let mut sum: u64;
    for i in value.iter_mut() {
        sum = (*i as u64) + carry;
        *i = (sum & U32_MASK) as u32;
        carry = sum >> 32;
    }

    carry as u32
}

#[inline]
fn add_by_internal3(value: &mut [u32; 3], by: &[u32; 3]) -> u32 {
    let mut carry: u32 = 0;
    let bl = by.len();
    for i in 0..bl {
        let res1 = value[i].overflowing_add(by[i]);
        let res2 = res1.0.overflowing_add(carry);
        value[i] = res2.0;
        carry = (res1.1 | res2.1) as u32;
    }
    carry
}

#[inline]
const fn u64_to_array(value: u64) -> [u32; 2] {
    [(value & U32_MASK) as u32, ((value >> 32) & U32_MASK) as u32]
}

fn add_with_scale_internal(
    quotient: &mut [u32; 3],
    quotient_scale: &mut i32,
    working_quotient: &mut [u32; 4],
    working_scale: &mut i32,
) -> bool {
    // Add quotient and the working (i.e. quotient = quotient + working)
    if is_all_zero(quotient) {
        // Quotient is zero so we can just copy the working quotient in directly
        // First, make sure they are both 96 bit.
        while working_quotient[3] != 0 {
            div_by_u32(working_quotient, 10);
            *working_scale -= 1;
        }
        copy_array_diff_lengths(quotient, working_quotient);
        *quotient_scale = *working_scale;
        return false;
    }

    if is_all_zero(working_quotient) {
        return false;
    }

    // We have ensured that our working is not zero so we should do the addition

    // If our two quotients are different then
    // try to scale down the one with the bigger scale
    let mut temp3 = [0u32, 0u32, 0u32];
    let mut temp4 = [0u32, 0u32, 0u32, 0u32];
    if *quotient_scale != *working_scale {
        // TODO: Remove necessity for temp (without performance impact)
        fn div_by_10<const N: usize>(target: &mut [u32], temp: &mut [u32; N], scale: &mut i32, target_scale: i32) {
            // Copy to the temp array
            temp.copy_from_slice(target);
            // divide by 10 until target scale is reached
            while *scale > target_scale {
                let remainder = div_by_u32(temp, 10);
                if remainder == 0 {
                    *scale -= 1;
                    target.copy_from_slice(temp);
                } else {
                    break;
                }
            }
        }

        if *quotient_scale < *working_scale {
            div_by_10(working_quotient, &mut temp4, working_scale, *quotient_scale);
        } else {
            div_by_10(quotient, &mut temp3, quotient_scale, *working_scale);
        }
    }

    // If our two quotients are still different then
    // try to scale up the smaller scale
    if *quotient_scale != *working_scale {
        // TODO: Remove necessity for temp (without performance impact)
        fn mul_by_10(target: &mut [u32], temp: &mut [u32], scale: &mut i32, target_scale: i32) {
            temp.copy_from_slice(target);
            let mut overflow = 0;
            // Multiply by 10 until target scale reached or overflow
            while *scale < target_scale && overflow == 0 {
                overflow = mul_by_u32(temp, 10);
                if overflow == 0 {
                    // Still no overflow
                    *scale += 1;
                    target.copy_from_slice(temp);
                }
            }
        }

        if *quotient_scale > *working_scale {
            mul_by_10(working_quotient, &mut temp4, working_scale, *quotient_scale);
        } else {
            mul_by_10(quotient, &mut temp3, quotient_scale, *working_scale);
        }
    }

    // If our two quotients are still different then
    // try to scale down the one with the bigger scale
    // (ultimately losing significant digits)
    if *quotient_scale != *working_scale {
        // TODO: Remove necessity for temp (without performance impact)
        fn div_by_10_lossy<const N: usize>(
            target: &mut [u32],
            temp: &mut [u32; N],
            scale: &mut i32,
            target_scale: i32,
        ) {
            temp.copy_from_slice(target);
            // divide by 10 until target scale is reached
            while *scale > target_scale {
                div_by_u32(temp, 10);
                *scale -= 1;
                target.copy_from_slice(temp);
            }
        }
        if *quotient_scale < *working_scale {
            div_by_10_lossy(working_quotient, &mut temp4, working_scale, *quotient_scale);
        } else {
            div_by_10_lossy(quotient, &mut temp3, quotient_scale, *working_scale);
        }
    }

    // If quotient or working are zero we have an underflow condition
    if is_all_zero(quotient) || is_all_zero(working_quotient) {
        // Underflow
        return true;
    } else {
        // Both numbers have the same scale and can be added.
        // We just need to know whether we can fit them in
        let mut underflow = false;
        let mut temp = [0u32, 0u32, 0u32];
        while !underflow {
            temp.copy_from_slice(quotient);

            // Add the working quotient
            let overflow = add_by_internal(&mut temp, working_quotient);
            if overflow == 0 {
                // addition was successful
                quotient.copy_from_slice(&temp);
                break;
            } else {
                // addition overflowed - remove significant digits and try again
                div_by_u32(quotient, 10);
                *quotient_scale -= 1;
                div_by_u32(working_quotient, 10);
                *working_scale -= 1;
                // Check for underflow
                underflow = is_all_zero(quotient) || is_all_zero(working_quotient);
            }
        }
        if underflow {
            return true;
        }
    }
    false
}

/// Rescales the given decimals to equivalent scales.
/// It will firstly try to scale both the left and the right side to
/// the maximum scale of left/right. If it is unable to do that it
/// will try to reduce the accuracy of the other argument.
/// e.g. with 1.23 and 2.345 it'll rescale the first arg to 1.230
#[inline(always)]
fn rescale_to_maximum_scale(left: &mut [u32; 3], left_scale: &mut u32, right: &mut [u32; 3], right_scale: &mut u32) {
    if left_scale == right_scale {
        // Nothing to do
        return;
    }

    if is_all_zero(left) {
        *left_scale = *right_scale;
        return;
    } else if is_all_zero(right) {
        *right_scale = *left_scale;
        return;
    }

    if left_scale > right_scale {
        rescale_internal(right, right_scale, *left_scale);
        if right_scale != left_scale {
            rescale_internal(left, left_scale, *right_scale);
        }
    } else {
        rescale_internal(left, left_scale, *right_scale);
        if right_scale != left_scale {
            rescale_internal(right, right_scale, *left_scale);
        }
    }
}

#[cfg(test)]
mod test {
    // Tests on private methods.
    //
    // All public tests should go under `tests/`.

    use super::*;
    use crate::prelude::*;

    #[test]
    fn it_can_rescale_to_maximum_scale() {
        fn extract(value: &str) -> ([u32; 3], u32) {
            let v = Decimal::from_str(value).unwrap();
            (v.mantissa_array3(), v.scale())
        }

        let tests = &[
            ("1", "1", "1", "1"),
            ("1", "1.0", "1.0", "1.0"),
            ("1", "1.00000", "1.00000", "1.00000"),
            ("1", "1.0000000000", "1.0000000000", "1.0000000000"),
            (
                "1",
                "1.00000000000000000000",
                "1.00000000000000000000",
                "1.00000000000000000000",
            ),
            ("1.1", "1.1", "1.1", "1.1"),
            ("1.1", "1.10000", "1.10000", "1.10000"),
            ("1.1", "1.1000000000", "1.1000000000", "1.1000000000"),
            (
                "1.1",
                "1.10000000000000000000",
                "1.10000000000000000000",
                "1.10000000000000000000",
            ),
            (
                "0.6386554621848739495798319328",
                "11.815126050420168067226890757",
                "0.638655462184873949579831933",
                "11.815126050420168067226890757",
            ),
            (
                "0.0872727272727272727272727272", // Scale 28
                "843.65000000",                   // Scale 8
                "0.0872727272727272727272727",    // 25
                "843.6500000000000000000000000",  // 25
            ),
        ];

        for &(left_raw, right_raw, expected_left, expected_right) in tests {
            // Left = the value to rescale
            // Right = the new scale we're scaling to
            // Expected = the expected left value after rescale
            let (expected_left, expected_lscale) = extract(expected_left);
            let (expected_right, expected_rscale) = extract(expected_right);

            let (mut left, mut left_scale) = extract(left_raw);
            let (mut right, mut right_scale) = extract(right_raw);
            rescale_to_maximum_scale(&mut left, &mut left_scale, &mut right, &mut right_scale);
            assert_eq!(left, expected_left);
            assert_eq!(left_scale, expected_lscale);
            assert_eq!(right, expected_right);
            assert_eq!(right_scale, expected_rscale);

            // Also test the transitive case
            let (mut left, mut left_scale) = extract(left_raw);
            let (mut right, mut right_scale) = extract(right_raw);
            rescale_to_maximum_scale(&mut right, &mut right_scale, &mut left, &mut left_scale);
            assert_eq!(left, expected_left);
            assert_eq!(left_scale, expected_lscale);
            assert_eq!(right, expected_right);
            assert_eq!(right_scale, expected_rscale);
        }
    }
}

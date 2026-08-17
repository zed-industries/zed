use crate::constants::{MAX_I32_SCALE, POWERS_10, SCALE_MASK, SCALE_SHIFT, SIGN_MASK, U32_MASK, U32_MAX};
use crate::decimal::{CalculationResult, Decimal};
use crate::ops::common::{Buf24, Dec64};

pub(crate) fn add_impl(d1: &Decimal, d2: &Decimal) -> CalculationResult {
    add_sub_internal(d1, d2, false)
}

pub(crate) fn sub_impl(d1: &Decimal, d2: &Decimal) -> CalculationResult {
    add_sub_internal(d1, d2, true)
}

#[inline]
fn add_sub_internal(d1: &Decimal, d2: &Decimal, subtract: bool) -> CalculationResult {
    if d1.is_zero() {
        // 0 - x or 0 + x
        let mut result = *d2;
        if subtract && !d2.is_zero() {
            result.set_sign_negative(d2.is_sign_positive());
        }
        return CalculationResult::Ok(result);
    }
    if d2.is_zero() {
        // x - 0 or x + 0
        return CalculationResult::Ok(*d1);
    }

    // Work out whether we need to rescale and/or if it's a subtract still given the signs of the
    // numbers.
    let flags = d1.flags() ^ d2.flags();
    let subtract = subtract ^ ((flags & SIGN_MASK) != 0);
    let rescale = (flags & SCALE_MASK) > 0;

    // We optimize towards using 32 bit logic as much as possible. It's noticeably faster at
    // scale, even on 64 bit machines
    if d1.mid() | d1.hi() == 0 && d2.mid() | d2.hi() == 0 {
        // We'll try to rescale, however we may end up with 64 bit (or more) numbers
        // If we do, we'll choose a different flow than fast_add
        if rescale {
            // This is less optimized if we scale to a 64 bit integer. We can add some further logic
            // here later on.
            let rescale_factor = ((d2.flags() & SCALE_MASK) as i32 - (d1.flags() & SCALE_MASK) as i32) >> SCALE_SHIFT;
            if rescale_factor < 0 {
                // We try to rescale the rhs
                if let Some(rescaled) = rescale32(d2.lo(), -rescale_factor) {
                    return fast_add(d1.lo(), rescaled, d1.flags(), subtract);
                }
            } else {
                // We try to rescale the lhs
                if let Some(rescaled) = rescale32(d1.lo(), rescale_factor) {
                    return fast_add(
                        rescaled,
                        d2.lo(),
                        (d2.flags() & SCALE_MASK) | (d1.flags() & SIGN_MASK),
                        subtract,
                    );
                }
            }
        } else {
            return fast_add(d1.lo(), d2.lo(), d1.flags(), subtract);
        }
    }

    // Continue on with the slower 64 bit method
    let d1 = Dec64::new(d1);
    let d2 = Dec64::new(d2);

    // If we're not the same scale then make sure we're there first before starting addition
    if rescale {
        let rescale_factor = d2.scale as i32 - d1.scale as i32;
        if rescale_factor < 0 {
            let negative = subtract ^ d1.negative;
            let scale = d1.scale;
            unaligned_add(d2, d1, negative, scale, -rescale_factor, subtract)
        } else {
            let negative = d1.negative;
            let scale = d2.scale;
            unaligned_add(d1, d2, negative, scale, rescale_factor, subtract)
        }
    } else {
        let neg = d1.negative;
        let scale = d1.scale;
        aligned_add(d1, d2, neg, scale, subtract)
    }
}

#[inline(always)]
fn rescale32(num: u32, rescale_factor: i32) -> Option<u32> {
    if rescale_factor > MAX_I32_SCALE {
        return None;
    }
    num.checked_mul(POWERS_10[rescale_factor as usize])
}

fn fast_add(lo1: u32, lo2: u32, flags: u32, subtract: bool) -> CalculationResult {
    if subtract {
        // Sub can't overflow because we're ensuring the bigger number always subtracts the smaller number
        if lo1 < lo2 {
            return CalculationResult::Ok(Decimal::from_parts_raw(lo2 - lo1, 0, 0, flags ^ SIGN_MASK));
        }
        return CalculationResult::Ok(Decimal::from_parts_raw(lo1 - lo2, 0, 0, flags));
    }
    // Add can overflow however, so we check for that explicitly
    let lo = lo1.wrapping_add(lo2);
    let mid = if lo < lo1 { 1 } else { 0 };
    CalculationResult::Ok(Decimal::from_parts_raw(lo, mid, 0, flags))
}

fn aligned_add(lhs: Dec64, rhs: Dec64, negative: bool, scale: u32, subtract: bool) -> CalculationResult {
    if subtract {
        // Signs differ, so subtract
        let mut result = Dec64 {
            negative,
            scale,
            low64: lhs.low64.wrapping_sub(rhs.low64),
            hi: lhs.hi.wrapping_sub(rhs.hi),
        };

        // Check for carry
        if result.low64 > lhs.low64 {
            result.hi = result.hi.wrapping_sub(1);
            if result.hi >= lhs.hi {
                flip_sign(&mut result);
            }
        } else if result.hi > lhs.hi {
            flip_sign(&mut result);
        }
        CalculationResult::Ok(result.to_decimal())
    } else {
        // Signs are the same, so add
        let mut result = Dec64 {
            negative,
            scale,
            low64: lhs.low64.wrapping_add(rhs.low64),
            hi: lhs.hi.wrapping_add(rhs.hi),
        };

        // Check for carry
        if result.low64 < lhs.low64 {
            result.hi = result.hi.wrapping_add(1);
            if result.hi <= lhs.hi {
                if result.scale == 0 {
                    return CalculationResult::Overflow;
                }
                reduce_scale(&mut result);
            }
        } else if result.hi < lhs.hi {
            if result.scale == 0 {
                return CalculationResult::Overflow;
            }
            reduce_scale(&mut result);
        }
        CalculationResult::Ok(result.to_decimal())
    }
}

fn flip_sign(result: &mut Dec64) {
    // Bitwise not the high portion
    result.hi = !result.hi;
    let low64 = ((result.low64 as i64).wrapping_neg()) as u64;
    if low64 == 0 {
        result.hi += 1;
    }
    result.low64 = low64;
    result.negative = !result.negative;
}

fn reduce_scale(result: &mut Dec64) {
    let mut low64 = result.low64;
    let mut hi = result.hi;

    let mut num = (hi as u64) + (1u64 << 32);
    hi = (num / 10u64) as u32;
    num = ((num - (hi as u64) * 10u64) << 32) + (low64 >> 32);
    let mut div = (num / 10) as u32;
    num = ((num - (div as u64) * 10u64) << 32) + (low64 & U32_MASK);
    low64 = (div as u64) << 32;
    div = (num / 10u64) as u32;
    low64 = low64.wrapping_add(div as u64);
    let remainder = (num as u32).wrapping_sub(div.wrapping_mul(10));

    // Finally, round. This is optimizing slightly toward non-rounded numbers
    if remainder >= 5 && (remainder > 5 || (low64 & 1) > 0) {
        low64 = low64.wrapping_add(1);
        if low64 == 0 {
            hi += 1;
        }
    }

    result.low64 = low64;
    result.hi = hi;
    result.scale -= 1;
}

// Assumption going into this function is that the LHS is the larger number and will "absorb" the
// smaller number.
fn unaligned_add(
    lhs: Dec64,
    rhs: Dec64,
    negative: bool,
    scale: u32,
    rescale_factor: i32,
    subtract: bool,
) -> CalculationResult {
    let mut lhs = lhs;
    let mut low64 = lhs.low64;
    let mut high = lhs.hi;
    let mut rescale_factor = rescale_factor;

    // First off, we see if we can get away with scaling small amounts (or none at all)
    if high == 0 {
        if low64 <= U32_MAX {
            // We know it's not zero, so we start scaling.
            // Start with reducing the scale down for the low portion
            while low64 <= U32_MAX {
                if rescale_factor <= MAX_I32_SCALE {
                    low64 *= POWERS_10[rescale_factor as usize] as u64;
                    lhs.low64 = low64;
                    return aligned_add(lhs, rhs, negative, scale, subtract);
                }
                rescale_factor -= MAX_I32_SCALE;
                low64 *= POWERS_10[9] as u64;
            }
        }

        // Reduce the scale for the high portion
        while high == 0 {
            let power = if rescale_factor <= MAX_I32_SCALE {
                POWERS_10[rescale_factor as usize] as u64
            } else {
                POWERS_10[9] as u64
            };

            let tmp_low = (low64 & U32_MASK) * power;
            let tmp_hi = (low64 >> 32) * power + (tmp_low >> 32);
            low64 = (tmp_low & U32_MASK) + (tmp_hi << 32);
            high = (tmp_hi >> 32) as u32;
            rescale_factor -= MAX_I32_SCALE;
            if rescale_factor <= 0 {
                lhs.low64 = low64;
                lhs.hi = high;
                return aligned_add(lhs, rhs, negative, scale, subtract);
            }
        }
    }

    // See if we can get away with keeping it in the 96 bits. Otherwise, we need a buffer
    let mut tmp64: u64;
    loop {
        let power = if rescale_factor <= MAX_I32_SCALE {
            POWERS_10[rescale_factor as usize] as u64
        } else {
            POWERS_10[9] as u64
        };

        let tmp_low = (low64 & U32_MASK) * power;
        tmp64 = (low64 >> 32) * power + (tmp_low >> 32);
        low64 = (tmp_low & U32_MASK) + (tmp64 << 32);
        tmp64 >>= 32;
        tmp64 += (high as u64) * power;

        rescale_factor -= MAX_I32_SCALE;

        if tmp64 > U32_MAX || scale > Decimal::MAX_SCALE {
            break;
        } else {
            high = tmp64 as u32;
            if rescale_factor <= 0 {
                lhs.low64 = low64;
                lhs.hi = high;
                return aligned_add(lhs, rhs, negative, scale, subtract);
            }
        }
    }

    let mut buffer = Buf24::zero();
    buffer.set_low64(low64);
    buffer.set_mid64(tmp64);

    let mut upper_word = buffer.upper_word();
    while rescale_factor > 0 {
        let power = if rescale_factor <= MAX_I32_SCALE {
            POWERS_10[rescale_factor as usize] as u64
        } else {
            POWERS_10[9] as u64
        };
        tmp64 = 0;
        for (index, part) in buffer.data.iter_mut().enumerate() {
            tmp64 = tmp64.wrapping_add((*part as u64) * power);
            *part = tmp64 as u32;
            tmp64 >>= 32;
            if index + 1 > upper_word {
                break;
            }
        }

        if tmp64 & U32_MASK > 0 {
            // Extend the result
            upper_word += 1;
            buffer.data[upper_word] = tmp64 as u32;
        }

        rescale_factor -= MAX_I32_SCALE;
    }

    // Do the add
    tmp64 = buffer.low64();
    low64 = rhs.low64;
    let tmp_hi = buffer.data[2];
    high = rhs.hi;

    if subtract {
        low64 = tmp64.wrapping_sub(low64);
        high = tmp_hi.wrapping_sub(high);

        // Check for carry
        let carry = if low64 > tmp64 {
            high = high.wrapping_sub(1);
            high >= tmp_hi
        } else {
            high > tmp_hi
        };

        if carry {
            for part in buffer.data.iter_mut().skip(3) {
                *part = part.wrapping_sub(1);
                if *part > 0 {
                    break;
                }
            }

            if buffer.data[upper_word] == 0 && upper_word < 3 {
                return CalculationResult::Ok(Decimal::from_parts(
                    low64 as u32,
                    (low64 >> 32) as u32,
                    high,
                    negative,
                    scale,
                ));
            }
        }
    } else {
        low64 = low64.wrapping_add(tmp64);
        high = high.wrapping_add(tmp_hi);

        // Check for carry
        let carry = if low64 < tmp64 {
            high = high.wrapping_add(1);
            high <= tmp_hi
        } else {
            high < tmp_hi
        };

        if carry {
            for (index, part) in buffer.data.iter_mut().enumerate().skip(3) {
                if upper_word < index {
                    *part = 1;
                    upper_word = index;
                    break;
                }
                *part = part.wrapping_add(1);
                if *part > 0 {
                    break;
                }
            }
        }
    }

    buffer.set_low64(low64);
    buffer.data[2] = high;
    if let Some(scale) = buffer.rescale(upper_word, scale) {
        CalculationResult::Ok(Decimal::from_parts(
            buffer.data[0],
            buffer.data[1],
            buffer.data[2],
            negative,
            scale,
        ))
    } else {
        CalculationResult::Overflow
    }
}

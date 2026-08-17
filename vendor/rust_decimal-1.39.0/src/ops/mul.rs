use crate::constants::{BIG_POWERS_10, MAX_I64_SCALE, U32_MAX};
use crate::decimal::{CalculationResult, Decimal};
use crate::ops::common::Buf24;

pub(crate) fn mul_impl(d1: &Decimal, d2: &Decimal) -> CalculationResult {
    if d1.is_zero() || d2.is_zero() {
        // We should think about this - does zero need to maintain precision? This treats it like
        // an absolute which I think is ok, especially since we have is_zero() functions etc.
        return CalculationResult::Ok(Decimal::ZERO);
    }

    let mut scale = d1.scale() + d2.scale();
    let negative = d1.is_sign_negative() ^ d2.is_sign_negative();
    let mut product = Buf24::zero();

    // See if we can optimize this calculation depending on whether the hi bits are set
    if d1.hi() | d1.mid() == 0 {
        if d2.hi() | d2.mid() == 0 {
            // We're multiplying two 32 bit integers, so we can take some liberties to optimize this.
            let mut low64 = d1.lo() as u64 * d2.lo() as u64;
            if scale > Decimal::MAX_SCALE {
                // We've exceeded maximum scale so we need to start reducing the precision (aka
                // rounding) until we have something that fits.
                // If we're too big then we effectively round to zero.
                if scale > Decimal::MAX_SCALE + MAX_I64_SCALE {
                    return CalculationResult::Ok(Decimal::ZERO);
                }

                scale -= Decimal::MAX_SCALE + 1;
                let mut power = BIG_POWERS_10[scale as usize];

                let tmp = low64 / power;
                let remainder = low64 - tmp * power;
                low64 = tmp;

                // Round the result. Since the divisor was a power of 10, it's always even.
                power >>= 1;
                if remainder >= power && (remainder > power || (low64 as u32 & 1) > 0) {
                    low64 += 1;
                }

                scale = Decimal::MAX_SCALE;
            }

            // Early exit
            return CalculationResult::Ok(Decimal::from_parts(
                low64 as u32,
                (low64 >> 32) as u32,
                0,
                negative,
                scale,
            ));
        }

        // We know that the left hand side is just 32 bits but the right hand side is either
        // 64 or 96 bits.
        mul_by_32bit_lhs(d1.lo() as u64, d2, &mut product);
    } else if d2.mid() | d2.hi() == 0 {
        // We know that the right hand side is just 32 bits.
        mul_by_32bit_lhs(d2.lo() as u64, d1, &mut product);
    } else {
        // We know we're not dealing with simple 32 bit operands on either side.
        // We compute and accumulate the 9 partial products using long multiplication

        // 1: ll * rl
        let mut tmp = d1.lo() as u64 * d2.lo() as u64;
        product.data[0] = tmp as u32;

        // 2: ll * rm
        let mut tmp2 = (d1.lo() as u64 * d2.mid() as u64).wrapping_add(tmp >> 32);

        // 3: lm * rl
        tmp = d1.mid() as u64 * d2.lo() as u64;
        tmp = tmp.wrapping_add(tmp2);
        product.data[1] = tmp as u32;

        // Detect if carry happened from the wrapping add
        if tmp < tmp2 {
            tmp2 = (tmp >> 32) | (1u64 << 32);
        } else {
            tmp2 = tmp >> 32;
        }

        // 4: lm * rm
        tmp = (d1.mid() as u64 * d2.mid() as u64) + tmp2;

        // If the high bit isn't set then we can stop here. Otherwise, we need to continue calculating
        // using the high bits.
        if (d1.hi() | d2.hi()) > 0 {
            // 5. ll * rh
            tmp2 = d1.lo() as u64 * d2.hi() as u64;
            tmp = tmp.wrapping_add(tmp2);
            // Detect if we carried
            let mut tmp3 = if tmp < tmp2 { 1 } else { 0 };

            // 6. lh * rl
            tmp2 = d1.hi() as u64 * d2.lo() as u64;
            tmp = tmp.wrapping_add(tmp2);
            product.data[2] = tmp as u32;
            // Detect if we carried
            if tmp < tmp2 {
                tmp3 += 1;
            }
            tmp2 = (tmp3 << 32) | (tmp >> 32);

            // 7. lm * rh
            tmp = d1.mid() as u64 * d2.hi() as u64;
            tmp = tmp.wrapping_add(tmp2);
            // Check for carry
            tmp3 = if tmp < tmp2 { 1 } else { 0 };

            // 8. lh * rm
            tmp2 = d1.hi() as u64 * d2.mid() as u64;
            tmp = tmp.wrapping_add(tmp2);
            product.data[3] = tmp as u32;
            // Check for carry
            if tmp < tmp2 {
                tmp3 += 1;
            }
            tmp = (tmp3 << 32) | (tmp >> 32);

            // 9. lh * rh
            product.set_high64(d1.hi() as u64 * d2.hi() as u64 + tmp);
        } else {
            product.set_mid64(tmp);
        }
    }

    // We may want to "rescale". This is the case if the mantissa is > 96 bits or if the scale
    // exceeds the maximum precision.
    let upper_word = product.upper_word();
    if upper_word > 2 || scale > Decimal::MAX_SCALE {
        scale = if let Some(new_scale) = product.rescale(upper_word, scale) {
            new_scale
        } else {
            return CalculationResult::Overflow;
        }
    }

    CalculationResult::Ok(Decimal::from_parts(
        product.data[0],
        product.data[1],
        product.data[2],
        negative,
        scale,
    ))
}

#[inline(always)]
fn mul_by_32bit_lhs(d1: u64, d2: &Decimal, product: &mut Buf24) {
    let mut tmp = d1 * d2.lo() as u64;
    product.data[0] = tmp as u32;
    tmp = (d1 * d2.mid() as u64).wrapping_add(tmp >> 32);
    product.data[1] = tmp as u32;
    tmp >>= 32;

    // If we're multiplying by a 96 bit integer then continue the calculation
    if d2.hi() > 0 {
        tmp = tmp.wrapping_add(d1 * d2.hi() as u64);
        if tmp > U32_MAX {
            product.set_mid64(tmp);
        } else {
            product.data[2] = tmp as u32;
        }
    } else {
        product.data[2] = tmp as u32;
    }
}

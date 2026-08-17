use crate::constants::{MAX_SCALE_I32, POWERS_10};
use crate::decimal::{CalculationResult, Decimal};
use crate::ops::common::{Buf12, Buf16, Dec64};

use core::cmp::Ordering;
use core::ops::BitXor;

impl Buf12 {
    // Returns true if successful, else false for an overflow
    fn add32(&mut self, value: u32) -> Result<(), DivError> {
        let value = value as u64;
        let new = self.low64().wrapping_add(value);
        self.set_low64(new);
        if new < value {
            self.data[2] = self.data[2].wrapping_add(1);
            if self.data[2] == 0 {
                return Err(DivError::Overflow);
            }
        }
        Ok(())
    }

    // Divide a Decimal union by a 32 bit divisor.
    // Self is overwritten with the quotient.
    // Return value is a 32 bit remainder.
    fn div32(&mut self, divisor: u32) -> u32 {
        let divisor64 = divisor as u64;
        // See if we can get by using a simple u64 division
        if self.data[2] != 0 {
            let mut temp = self.high64();
            let q64 = temp / divisor64;
            self.set_high64(q64);

            // Calculate the "remainder"
            temp = ((temp - q64 * divisor64) << 32) | (self.data[0] as u64);
            if temp == 0 {
                return 0;
            }
            let q32 = (temp / divisor64) as u32;
            self.data[0] = q32;
            (temp as u32).wrapping_sub(q32.wrapping_mul(divisor))
        } else {
            // Super easy divisor
            let low64 = self.low64();
            if low64 == 0 {
                // Nothing to do
                return 0;
            }
            // Do the calc
            let quotient = low64 / divisor64;
            self.set_low64(quotient);
            // Remainder is the leftover that wasn't used
            (low64.wrapping_sub(quotient.wrapping_mul(divisor64))) as u32
        }
    }

    // Divide the number by a power constant
    // Returns true if division was successful
    fn div32_const(&mut self, pow: u32) -> bool {
        let pow64 = pow as u64;
        let high64 = self.high64();
        let lo = self.data[0] as u64;
        let div64: u64 = high64 / pow64;
        let div = ((((high64 - div64 * pow64) << 32) + lo) / pow64) as u32;
        if self.data[0] == div.wrapping_mul(pow) {
            self.set_high64(div64);
            self.data[0] = div;
            true
        } else {
            false
        }
    }
}

impl Buf16 {
    // Does a partial divide with a 64 bit divisor. The divisor in this case must be 64 bits
    // otherwise various assumptions fail (e.g. 32 bit quotient).
    // To assist, the upper 64 bits must be greater than the divisor for this to succeed.
    // Consequently, it will return the quotient as a 32 bit number and overwrite self with the
    // 64 bit remainder.
    pub(super) fn partial_divide_64(&mut self, divisor: u64) -> u32 {
        // We make this assertion here, however below we pivot based on the data
        debug_assert!(divisor > self.mid64());

        // If we have an empty high bit, then divisor must be greater than the dividend due to
        // the assumption that the divisor REQUIRES 64 bits.
        if self.data[2] == 0 {
            let low64 = self.low64();
            if low64 < divisor {
                // We can't divide at at all so result is 0. The dividend remains untouched since
                // the full amount is the remainder.
                return 0;
            }

            let quotient = low64 / divisor;
            self.set_low64(low64 - (quotient * divisor));
            return quotient as u32;
        }

        // Do a simple check to see if the hi portion of the dividend is greater than the hi
        // portion of the divisor.
        let divisor_hi32 = (divisor >> 32) as u32;
        if self.data[2] >= divisor_hi32 {
            // We know that the divisor goes into this at MOST u32::max times.
            // So we kick things off, with that assumption
            let mut low64 = self.low64();
            low64 = low64.wrapping_sub(divisor << 32).wrapping_add(divisor);
            let mut quotient = u32::MAX;

            // If we went negative then keep adding it back in
            loop {
                if low64 < divisor {
                    break;
                }
                quotient = quotient.wrapping_sub(1);
                low64 = low64.wrapping_add(divisor);
            }
            self.set_low64(low64);
            return quotient;
        }

        let mid64 = self.mid64();
        let divisor_hi32_64 = divisor_hi32 as u64;
        if mid64 < divisor_hi32_64 {
            // similar situation as above where we've got nothing left to divide
            return 0;
        }

        let mut quotient = mid64 / divisor_hi32_64;
        let mut remainder = self.data[0] as u64 | ((mid64 - quotient * divisor_hi32_64) << 32);

        // Do quotient * lo divisor
        let product = quotient * (divisor & 0xFFFF_FFFF);
        remainder = remainder.wrapping_sub(product);

        // Check if we've gone negative. If so, add it back
        if remainder > product.bitxor(u64::MAX) {
            loop {
                quotient = quotient.wrapping_sub(1);
                remainder = remainder.wrapping_add(divisor);
                if remainder < divisor {
                    break;
                }
            }
        }

        self.set_low64(remainder);
        quotient as u32
    }

    // Does a partial divide with a 96 bit divisor. The divisor in this case must require 96 bits
    // otherwise various assumptions fail (e.g. 32 bit quotient).
    pub(super) fn partial_divide_96(&mut self, divisor: &Buf12) -> u32 {
        let dividend = self.high64();
        let divisor_hi = divisor.data[2];
        if dividend < divisor_hi as u64 {
            // Dividend is too small - entire number is remainder
            return 0;
        }

        let mut quo = (dividend / divisor_hi as u64) as u32;
        let mut remainder = (dividend as u32).wrapping_sub(quo.wrapping_mul(divisor_hi));

        // Compute full remainder
        let mut prod1 = quo as u64 * divisor.data[0] as u64;
        let mut prod2 = quo as u64 * divisor.data[1] as u64;
        prod2 += prod1 >> 32;
        prod1 = (prod1 & 0xFFFF_FFFF) | (prod2 << 32);
        prod2 >>= 32;

        let mut num = self.low64();
        num = num.wrapping_sub(prod1);
        remainder = remainder.wrapping_sub(prod2 as u32);

        // If there are carries make sure they are propagated
        if num > prod1.bitxor(u64::MAX) {
            remainder = remainder.wrapping_sub(1);
            if remainder < (prod2 as u32).bitxor(u32::MAX) {
                self.set_low64(num);
                self.data[2] = remainder;
                return quo;
            }
        } else if remainder <= (prod2 as u32).bitxor(u32::MAX) {
            self.set_low64(num);
            self.data[2] = remainder;
            return quo;
        }

        // Remainder went negative, add divisor back until it's positive
        prod1 = divisor.low64();
        loop {
            quo = quo.wrapping_sub(1);
            num = num.wrapping_add(prod1);
            remainder = remainder.wrapping_add(divisor_hi);

            if num < prod1 {
                // Detected carry.
                let tmp = remainder;
                remainder = remainder.wrapping_add(1);
                if tmp < divisor_hi {
                    break;
                }
            }
            if remainder < divisor_hi {
                break; // detected carry
            }
        }

        self.set_low64(num);
        self.data[2] = remainder;
        quo
    }
}

enum DivError {
    Overflow,
}

pub(crate) fn div_impl(dividend: &Decimal, divisor: &Decimal) -> CalculationResult {
    if divisor.is_zero() {
        return CalculationResult::DivByZero;
    }
    if dividend.is_zero() {
        return CalculationResult::Ok(Decimal::ZERO);
    }
    let dividend = Dec64::new(dividend);
    let divisor = Dec64::new(divisor);

    // Pre calculate the scale and the sign
    let mut scale = (dividend.scale as i32) - (divisor.scale as i32);
    let sign_negative = dividend.negative ^ divisor.negative;

    // Set up some variables for modification throughout
    let mut require_unscale = false;
    let mut quotient = Buf12::from_dec64(&dividend);
    let divisor = Buf12::from_dec64(&divisor);

    // Branch depending on the complexity of the divisor
    if divisor.data[2] | divisor.data[1] == 0 {
        // We have a simple(r) divisor (32 bit)
        let divisor32 = divisor.data[0];

        // Remainder can only be 32 bits since the divisor is 32 bits.
        let mut remainder = quotient.div32(divisor32);
        let mut power_scale = 0;

        // Figure out how to apply the remainder (i.e. we may have performed something like 10/3 or 8/5)
        loop {
            // Remainder is 0 so we have a simple situation
            if remainder == 0 {
                // If the scale is positive then we're actually done
                if scale >= 0 {
                    break;
                }
                power_scale = 9usize.min((-scale) as usize);
            } else {
                // We may need to normalize later, so set the flag appropriately
                require_unscale = true;

                // We have a remainder so we effectively want to try to adjust the quotient and add
                // the remainder into the quotient. We do this below, however first of all we want
                // to try to avoid overflowing so we do that check first.
                let will_overflow = if scale == MAX_SCALE_I32 {
                    true
                } else {
                    // Figure out how much we can scale by
                    if let Some(s) = quotient.find_scale(scale) {
                        power_scale = s;
                    } else {
                        return CalculationResult::Overflow;
                    }
                    // If it comes back as 0 (i.e. 10^0 = 1) then we're going to overflow since
                    // we're doing nothing.
                    power_scale == 0
                };
                if will_overflow {
                    // No more scaling can be done, but remainder is non-zero so we round if necessary.
                    let tmp = remainder << 1;
                    let round = if tmp < remainder {
                        // We round if we wrapped around
                        true
                    } else if tmp >= divisor32 {
                        // If we're greater than the divisor (i.e. underflow)
                        // or if there is a lo bit set, we round
                        tmp > divisor32 || (quotient.data[0] & 0x1) > 0
                    } else {
                        false
                    };

                    // If we need to round, try to do so.
                    if round {
                        if let Ok(new_scale) = round_up(&mut quotient, scale) {
                            scale = new_scale;
                        } else {
                            // Overflowed
                            return CalculationResult::Overflow;
                        }
                    }
                    break;
                }
            }

            // Do some scaling
            let power = POWERS_10[power_scale];
            scale += power_scale as i32;
            // Increase the quotient by the power that was looked up
            let overflow = increase_scale(&mut quotient, power as u64);
            if overflow > 0 {
                return CalculationResult::Overflow;
            }

            let remainder_scaled = (remainder as u64) * (power as u64);
            let remainder_quotient = (remainder_scaled / (divisor32 as u64)) as u32;
            remainder = (remainder_scaled - remainder_quotient as u64 * divisor32 as u64) as u32;
            if let Err(DivError::Overflow) = quotient.add32(remainder_quotient) {
                if let Ok(adj) = unscale_from_overflow(&mut quotient, scale, remainder != 0) {
                    scale = adj;
                } else {
                    // Still overflowing
                    return CalculationResult::Overflow;
                }
                break;
            }
        }
    } else {
        // We have a divisor greater than 32 bits. Both of these share some quick calculation wins
        // so we'll do those before branching into separate logic.
        // The win we can do is shifting the bits to the left as much as possible. We do this to both
        // the dividend and the divisor to ensure the quotient is not changed.
        // As a simple contrived example: if we have 4 / 2 then we could bit shift all the way to the
        // left meaning that the lo portion would have nothing inside of it. Of course, shifting these
        // left one has the same result (8/4) etc.
        // The advantage is that we may be able to write off lower portions of the number making things
        // easier.
        let mut power_scale = if divisor.data[2] == 0 {
            divisor.data[1].leading_zeros()
        } else {
            divisor.data[2].leading_zeros()
        } as usize;
        let mut remainder = Buf16::zero();
        remainder.set_low64(quotient.low64() << power_scale);
        let tmp_high = ((quotient.data[1] as u64) + ((quotient.data[2] as u64) << 32)) >> (32 - power_scale);
        remainder.set_high64(tmp_high);

        // Work out the divisor after it's shifted
        let divisor64 = divisor.low64() << power_scale;
        // Check if the divisor is 64 bit or the full 96 bits
        if divisor.data[2] == 0 {
            // It's 64 bits
            quotient.data[2] = 0;

            // Calc mid/lo by shifting accordingly
            let rem_lo = remainder.data[0];
            remainder.data[0] = remainder.data[1];
            remainder.data[1] = remainder.data[2];
            remainder.data[2] = remainder.data[3];
            quotient.data[1] = remainder.partial_divide_64(divisor64);

            remainder.data[2] = remainder.data[1];
            remainder.data[1] = remainder.data[0];
            remainder.data[0] = rem_lo;
            quotient.data[0] = remainder.partial_divide_64(divisor64);

            loop {
                let rem_low64 = remainder.low64();
                if rem_low64 == 0 {
                    // If the scale is positive then we're actually done
                    if scale >= 0 {
                        break;
                    }
                    power_scale = 9usize.min((-scale) as usize);
                } else {
                    // We may need to normalize later, so set the flag appropriately
                    require_unscale = true;

                    // We have a remainder so we effectively want to try to adjust the quotient and add
                    // the remainder into the quotient. We do this below, however first of all we want
                    // to try to avoid overflowing so we do that check first.
                    let will_overflow = if scale == MAX_SCALE_I32 {
                        true
                    } else {
                        // Figure out how much we can scale by
                        if let Some(s) = quotient.find_scale(scale) {
                            power_scale = s;
                        } else {
                            return CalculationResult::Overflow;
                        }
                        // If it comes back as 0 (i.e. 10^0 = 1) then we're going to overflow since
                        // we're doing nothing.
                        power_scale == 0
                    };
                    if will_overflow {
                        // No more scaling can be done, but remainder is non-zero so we round if necessary.
                        let mut tmp = remainder.low64();
                        let round = if (tmp as i64) < 0 {
                            // We round if we wrapped around
                            true
                        } else {
                            tmp <<= 1;
                            if tmp > divisor64 {
                                true
                            } else {
                                tmp == divisor64 && quotient.data[0] & 0x1 != 0
                            }
                        };

                        // If we need to round, try to do so.
                        if round {
                            if let Ok(new_scale) = round_up(&mut quotient, scale) {
                                scale = new_scale;
                            } else {
                                // Overflowed
                                return CalculationResult::Overflow;
                            }
                        }
                        break;
                    }
                }

                // Do some scaling
                let power = POWERS_10[power_scale];
                scale += power_scale as i32;

                // Increase the quotient by the power that was looked up
                let overflow = increase_scale(&mut quotient, power as u64);
                if overflow > 0 {
                    return CalculationResult::Overflow;
                }
                increase_scale64(&mut remainder, power as u64);

                let tmp = remainder.partial_divide_64(divisor64);
                if let Err(DivError::Overflow) = quotient.add32(tmp) {
                    if let Ok(adj) = unscale_from_overflow(&mut quotient, scale, remainder.low64() != 0) {
                        scale = adj;
                    } else {
                        // Still overflowing
                        return CalculationResult::Overflow;
                    }
                    break;
                }
            }
        } else {
            // It's 96 bits
            // Start by finishing the shift left
            let divisor_mid = divisor.data[1];
            let divisor_hi = divisor.data[2];
            let mut divisor = divisor;
            divisor.set_low64(divisor64);
            divisor.data[2] = ((divisor_mid as u64 + ((divisor_hi as u64) << 32)) >> (32 - power_scale)) as u32;

            let quo = remainder.partial_divide_96(&divisor);
            quotient.set_low64(quo as u64);
            quotient.data[2] = 0;

            loop {
                let mut rem_low64 = remainder.low64();
                if rem_low64 == 0 && remainder.data[2] == 0 {
                    // If the scale is positive then we're actually done
                    if scale >= 0 {
                        break;
                    }
                    power_scale = 9usize.min((-scale) as usize);
                } else {
                    // We may need to normalize later, so set the flag appropriately
                    require_unscale = true;

                    // We have a remainder so we effectively want to try to adjust the quotient and add
                    // the remainder into the quotient. We do this below, however first of all we want
                    // to try to avoid overflowing so we do that check first.
                    let will_overflow = if scale == MAX_SCALE_I32 {
                        true
                    } else {
                        // Figure out how much we can scale by
                        if let Some(s) = quotient.find_scale(scale) {
                            power_scale = s;
                        } else {
                            return CalculationResult::Overflow;
                        }
                        // If it comes back as 0 (i.e. 10^0 = 1) then we're going to overflow since
                        // we're doing nothing.
                        power_scale == 0
                    };
                    if will_overflow {
                        // No more scaling can be done, but remainder is non-zero so we round if necessary.
                        let round = if (remainder.data[2] as i32) < 0 {
                            // We round if we wrapped around
                            true
                        } else {
                            let tmp = remainder.data[1] >> 31;
                            rem_low64 <<= 1;
                            remainder.set_low64(rem_low64);
                            remainder.data[2] = (&remainder.data[2] << 1) + tmp;

                            match remainder.data[2].cmp(&divisor.data[2]) {
                                Ordering::Less => false,
                                Ordering::Equal => {
                                    let divisor_low64 = divisor.low64();
                                    if rem_low64 > divisor_low64 {
                                        true
                                    } else {
                                        rem_low64 == divisor_low64 && (quotient.data[0] & 1) != 0
                                    }
                                }
                                Ordering::Greater => true,
                            }
                        };

                        // If we need to round, try to do so.
                        if round {
                            if let Ok(new_scale) = round_up(&mut quotient, scale) {
                                scale = new_scale;
                            } else {
                                // Overflowed
                                return CalculationResult::Overflow;
                            }
                        }
                        break;
                    }
                }

                // Do some scaling
                let power = POWERS_10[power_scale];
                scale += power_scale as i32;

                // Increase the quotient by the power that was looked up
                let overflow = increase_scale(&mut quotient, power as u64);
                if overflow > 0 {
                    return CalculationResult::Overflow;
                }
                let mut tmp_remainder = Buf12 {
                    data: [remainder.data[0], remainder.data[1], remainder.data[2]],
                };
                let overflow = increase_scale(&mut tmp_remainder, power as u64);
                remainder.data[0] = tmp_remainder.data[0];
                remainder.data[1] = tmp_remainder.data[1];
                remainder.data[2] = tmp_remainder.data[2];
                remainder.data[3] = overflow;

                let tmp = remainder.partial_divide_96(&divisor);
                if let Err(DivError::Overflow) = quotient.add32(tmp) {
                    if let Ok(adj) =
                        unscale_from_overflow(&mut quotient, scale, (remainder.low64() | remainder.high64()) != 0)
                    {
                        scale = adj;
                    } else {
                        // Still overflowing
                        return CalculationResult::Overflow;
                    }
                    break;
                }
            }
        }
    }
    if require_unscale {
        scale = unscale(&mut quotient, scale);
    }
    CalculationResult::Ok(Decimal::from_parts(
        quotient.data[0],
        quotient.data[1],
        quotient.data[2],
        sign_negative,
        scale as u32,
    ))
}

// Multiply num by power (multiple of 10). Power must be 32 bits.
// Returns the overflow, if any
fn increase_scale(num: &mut Buf12, power: u64) -> u32 {
    let mut tmp = (num.data[0] as u64) * power;
    num.data[0] = tmp as u32;
    tmp >>= 32;
    tmp += (num.data[1] as u64) * power;
    num.data[1] = tmp as u32;
    tmp >>= 32;
    tmp += (num.data[2] as u64) * power;
    num.data[2] = tmp as u32;
    (tmp >> 32) as u32
}

// Multiply num by power (multiple of 10). Power must be 32 bits.
fn increase_scale64(num: &mut Buf16, power: u64) {
    let mut tmp = (num.data[0] as u64) * power;
    num.data[0] = tmp as u32;
    tmp >>= 32;
    tmp += (num.data[1] as u64) * power;
    num.set_mid64(tmp)
}

// Adjust the number to deal with an overflow. This function follows being scaled up (i.e. multiplied
// by 10, so this effectively tries to reverse that by dividing by 10 then feeding in the high bit
// to undo the overflow and rounding instead.
// Returns the updated scale.
fn unscale_from_overflow(num: &mut Buf12, scale: i32, sticky: bool) -> Result<i32, DivError> {
    let scale = scale - 1;
    if scale < 0 {
        return Err(DivError::Overflow);
    }

    // This function is called when the hi portion has "overflowed" upon adding one and has wrapped
    // back around to 0. Consequently, we need to "feed" that back in, but also rescaling down
    // to reverse out the overflow.
    const HIGH_BIT: u64 = 0x1_0000_0000;
    num.data[2] = (HIGH_BIT / 10) as u32;

    // Calc the mid
    let mut tmp = ((HIGH_BIT % 10) << 32) + (num.data[1] as u64);
    let mut val = (tmp / 10) as u32;
    num.data[1] = val;

    // Calc the lo using a similar method
    tmp = ((tmp - (val as u64) * 10) << 32) + (num.data[0] as u64);
    val = (tmp / 10) as u32;
    num.data[0] = val;

    // Work out the remainder, and round if we have one (since it doesn't fit)
    let remainder = (tmp - (val as u64) * 10) as u32;
    if remainder > 5 || (remainder == 5 && (sticky || num.data[0] & 0x1 > 0)) {
        let _ = num.add32(1);
    }
    Ok(scale)
}

#[inline]
fn round_up(num: &mut Buf12, scale: i32) -> Result<i32, DivError> {
    let low64 = num.low64().wrapping_add(1);
    num.set_low64(low64);
    if low64 != 0 {
        return Ok(scale);
    }
    let hi = num.data[2].wrapping_add(1);
    num.data[2] = hi;
    if hi != 0 {
        return Ok(scale);
    }
    unscale_from_overflow(num, scale, true)
}

fn unscale(num: &mut Buf12, scale: i32) -> i32 {
    // Since 10 = 2 * 5, there must be a factor of 2 for every power of 10 we can extract.
    // We use this as a quick test on whether to try a given power.
    let mut scale = scale;
    while num.data[0] == 0 && scale >= 8 && num.div32_const(100000000) {
        scale -= 8;
    }

    if (num.data[0] & 0xF) == 0 && scale >= 4 && num.div32_const(10000) {
        scale -= 4;
    }

    if (num.data[0] & 0x3) == 0 && scale >= 2 && num.div32_const(100) {
        scale -= 2;
    }

    if (num.data[0] & 0x1) == 0 && scale >= 1 && num.div32_const(10) {
        scale -= 1;
    }
    scale
}

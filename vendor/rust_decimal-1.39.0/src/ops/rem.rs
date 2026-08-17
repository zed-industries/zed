use crate::constants::{MAX_I32_SCALE, MAX_SCALE_I32, POWERS_10};
use crate::decimal::{CalculationResult, Decimal};
use crate::ops::common::{Buf12, Buf16, Buf24, Dec64};

pub(crate) fn rem_impl(d1: &Decimal, d2: &Decimal) -> CalculationResult {
    if d2.is_zero() {
        return CalculationResult::DivByZero;
    }
    if d1.is_zero() {
        return CalculationResult::Ok(Decimal::ZERO);
    }

    // We handle the structs a bit different here. Firstly, we ignore both the sign/scale of d2.
    // This is because during a remainder operation we do not care about the sign of the divisor
    // and only concern ourselves with that of the dividend.
    let mut d1 = Dec64::new(d1);
    let d2_scale = d2.scale();
    let mut d2 = Buf12::from_decimal(d2);

    let cmp = crate::ops::cmp::cmp_internal(
        &d1,
        &Dec64 {
            negative: d1.negative,
            scale: d2_scale,
            hi: d2.hi(),
            low64: d2.low64(),
        },
    );
    match cmp {
        core::cmp::Ordering::Equal => {
            // Same numbers meaning that remainder is zero
            return CalculationResult::Ok(Decimal::ZERO);
        }
        core::cmp::Ordering::Less => {
            // d1 < d2, e.g. 1/2. This means that the result is the value of d1
            return CalculationResult::Ok(d1.to_decimal());
        }
        core::cmp::Ordering::Greater => {}
    }

    // At this point we know that the dividend > divisor and that they are both non-zero.
    let mut scale = d1.scale as i32 - d2_scale as i32;
    if scale > 0 {
        // Scale up the divisor
        loop {
            let power = if scale >= MAX_I32_SCALE {
                POWERS_10[9]
            } else {
                POWERS_10[scale as usize]
            } as u64;

            let mut tmp = d2.lo() as u64 * power;
            d2.set_lo(tmp as u32);
            tmp >>= 32;
            tmp = tmp.wrapping_add((d2.mid() as u64 + ((d2.hi() as u64) << 32)) * power);
            d2.set_mid(tmp as u32);
            d2.set_hi((tmp >> 32) as u32);

            // Keep scaling if there is more to go
            scale -= MAX_I32_SCALE;
            if scale <= 0 {
                break;
            }
        }
        scale = 0;
    }

    loop {
        // If the dividend is smaller than the divisor then try to scale that up first
        if scale < 0 {
            let mut quotient = Buf12 {
                data: [d1.lo(), d1.mid(), d1.hi],
            };
            loop {
                // Figure out how much we can scale by
                let power_scale;
                if let Some(u) = quotient.find_scale(MAX_SCALE_I32 + scale) {
                    if u >= POWERS_10.len() {
                        power_scale = 9;
                    } else {
                        power_scale = u;
                    }
                } else {
                    return CalculationResult::Overflow;
                };
                if power_scale == 0 {
                    break;
                }
                let power = POWERS_10[power_scale] as u64;
                scale += power_scale as i32;

                let mut tmp = quotient.data[0] as u64 * power;
                quotient.data[0] = tmp as u32;
                tmp >>= 32;
                quotient.set_high64(tmp.wrapping_add(quotient.high64().wrapping_mul(power)));
                if power_scale != 9 {
                    break;
                }
                if scale >= 0 {
                    break;
                }
            }
            d1.low64 = quotient.low64();
            d1.hi = quotient.data[2];
            d1.scale = d2_scale;
        }

        // if the high portion is empty then return the modulus of the bottom portion
        if d1.hi == 0 {
            d1.low64 %= d2.low64();
            return CalculationResult::Ok(d1.to_decimal());
        } else if (d2.mid() | d2.hi()) == 0 {
            let mut tmp = d1.high64();
            tmp = ((tmp % d2.lo() as u64) << 32) | (d1.lo() as u64);
            d1.low64 = tmp % d2.lo() as u64;
            d1.hi = 0;
        } else {
            // Divisor is > 32 bits
            return rem_full(&d1, &d2, scale);
        }

        if scale >= 0 {
            break;
        }
    }

    CalculationResult::Ok(d1.to_decimal())
}

fn rem_full(d1: &Dec64, d2: &Buf12, scale: i32) -> CalculationResult {
    let mut scale = scale;

    // First normalize the divisor
    let shift = if d2.hi() == 0 {
        d2.mid().leading_zeros()
    } else {
        d2.hi().leading_zeros()
    };

    let mut buffer = Buf24::zero();
    let mut overflow = 0u32;
    buffer.set_low64(d1.low64 << shift);
    buffer.set_mid64(((d1.mid() as u64).wrapping_add((d1.hi as u64) << 32)) >> (32 - shift));
    let mut upper = 3; // We start at 3 due to bit shifting

    while scale < 0 {
        let power = if -scale >= MAX_I32_SCALE {
            POWERS_10[9]
        } else {
            POWERS_10[-scale as usize]
        } as u64;
        let mut tmp64 = buffer.data[0] as u64 * power;
        buffer.data[0] = tmp64 as u32;

        for (index, part) in buffer.data.iter_mut().enumerate().skip(1) {
            if index > upper {
                break;
            }
            tmp64 >>= 32;
            tmp64 = tmp64.wrapping_add((*part as u64).wrapping_mul(power));
            *part = tmp64 as u32;
        }
        // If we have overflow then also process that
        if upper == 6 {
            tmp64 >>= 32;
            tmp64 = tmp64.wrapping_add((overflow as u64).wrapping_mul(power));
            overflow = tmp64 as u32;
        }

        // Make sure the high bit is not set
        if tmp64 > 0x7FFF_FFFF {
            upper += 1;
            if upper > 5 {
                overflow = (tmp64 >> 32) as u32;
            } else {
                buffer.data[upper] = (tmp64 >> 32) as u32;
            }
        }
        scale += MAX_I32_SCALE;
    }

    // TODO: Optimize slice logic

    let mut tmp = Buf16::zero();
    let divisor = d2.low64() << shift;
    if d2.hi() == 0 {
        // Do some division
        if upper == 6 {
            upper -= 1;

            tmp.data = [buffer.data[4], buffer.data[5], overflow, 0];
            tmp.partial_divide_64(divisor);
            buffer.data[4] = tmp.data[0];
            buffer.data[5] = tmp.data[1];
        }
        if upper == 5 {
            upper -= 1;
            tmp.data = [buffer.data[3], buffer.data[4], buffer.data[5], 0];
            tmp.partial_divide_64(divisor);
            buffer.data[3] = tmp.data[0];
            buffer.data[4] = tmp.data[1];
            buffer.data[5] = tmp.data[2];
        }
        if upper == 4 {
            tmp.data = [buffer.data[2], buffer.data[3], buffer.data[4], 0];
            tmp.partial_divide_64(divisor);
            buffer.data[2] = tmp.data[0];
            buffer.data[3] = tmp.data[1];
            buffer.data[4] = tmp.data[2];
        }

        tmp.data = [buffer.data[1], buffer.data[2], buffer.data[3], 0];
        tmp.partial_divide_64(divisor);
        buffer.data[1] = tmp.data[0];
        buffer.data[2] = tmp.data[1];
        buffer.data[3] = tmp.data[2];

        tmp.data = [buffer.data[0], buffer.data[1], buffer.data[2], 0];
        tmp.partial_divide_64(divisor);
        buffer.data[0] = tmp.data[0];
        buffer.data[1] = tmp.data[1];
        buffer.data[2] = tmp.data[2];

        let low64 = buffer.low64() >> shift;
        CalculationResult::Ok(Decimal::from_parts(
            low64 as u32,
            (low64 >> 32) as u32,
            0,
            d1.negative,
            d1.scale,
        ))
    } else {
        let divisor_low64 = divisor;
        let divisor = Buf12 {
            data: [
                divisor_low64 as u32,
                (divisor_low64 >> 32) as u32,
                (((d2.mid() as u64) + ((d2.hi() as u64) << 32)) >> (32 - shift)) as u32,
            ],
        };

        // Do some division
        if upper == 6 {
            upper -= 1;
            tmp.data = [buffer.data[3], buffer.data[4], buffer.data[5], overflow];
            tmp.partial_divide_96(&divisor);
            buffer.data[3] = tmp.data[0];
            buffer.data[4] = tmp.data[1];
            buffer.data[5] = tmp.data[2];
        }
        if upper == 5 {
            upper -= 1;
            tmp.data = [buffer.data[2], buffer.data[3], buffer.data[4], buffer.data[5]];
            tmp.partial_divide_96(&divisor);
            buffer.data[2] = tmp.data[0];
            buffer.data[3] = tmp.data[1];
            buffer.data[4] = tmp.data[2];
            buffer.data[5] = tmp.data[3];
        }
        if upper == 4 {
            tmp.data = [buffer.data[1], buffer.data[2], buffer.data[3], buffer.data[4]];
            tmp.partial_divide_96(&divisor);
            buffer.data[1] = tmp.data[0];
            buffer.data[2] = tmp.data[1];
            buffer.data[3] = tmp.data[2];
            buffer.data[4] = tmp.data[3];
        }

        tmp.data = [buffer.data[0], buffer.data[1], buffer.data[2], buffer.data[3]];
        tmp.partial_divide_96(&divisor);
        buffer.data[0] = tmp.data[0];
        buffer.data[1] = tmp.data[1];
        buffer.data[2] = tmp.data[2];
        buffer.data[3] = tmp.data[3];

        let low64 = (buffer.low64() >> shift) + ((buffer.data[2] as u64) << (32 - shift) << 32);
        CalculationResult::Ok(Decimal::from_parts(
            low64 as u32,
            (low64 >> 32) as u32,
            buffer.data[2] >> shift,
            d1.negative,
            d1.scale,
        ))
    }
}

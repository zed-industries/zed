use crate::constants::{MAX_I32_SCALE, MAX_SCALE_I32, POWERS_10};
use crate::Decimal;

#[derive(Debug)]
pub struct Buf12 {
    pub data: [u32; 3],
}

impl Buf12 {
    pub(super) const fn from_dec64(value: &Dec64) -> Self {
        Buf12 {
            data: [value.low64 as u32, (value.low64 >> 32) as u32, value.hi],
        }
    }

    pub(super) const fn from_decimal(value: &Decimal) -> Self {
        Buf12 {
            data: value.mantissa_array3(),
        }
    }

    #[inline(always)]
    pub const fn lo(&self) -> u32 {
        self.data[0]
    }
    #[inline(always)]
    pub const fn mid(&self) -> u32 {
        self.data[1]
    }
    #[inline(always)]
    pub const fn hi(&self) -> u32 {
        self.data[2]
    }
    #[inline(always)]
    pub fn set_lo(&mut self, value: u32) {
        self.data[0] = value;
    }
    #[inline(always)]
    pub fn set_mid(&mut self, value: u32) {
        self.data[1] = value;
    }
    #[inline(always)]
    pub fn set_hi(&mut self, value: u32) {
        self.data[2] = value;
    }

    #[inline(always)]
    pub const fn low64(&self) -> u64 {
        ((self.data[1] as u64) << 32) | (self.data[0] as u64)
    }

    #[inline(always)]
    pub fn set_low64(&mut self, value: u64) {
        self.data[1] = (value >> 32) as u32;
        self.data[0] = value as u32;
    }

    #[inline(always)]
    pub const fn high64(&self) -> u64 {
        ((self.data[2] as u64) << 32) | (self.data[1] as u64)
    }

    #[inline(always)]
    pub fn set_high64(&mut self, value: u64) {
        self.data[2] = (value >> 32) as u32;
        self.data[1] = value as u32;
    }

    // Determine the maximum value of x that ensures that the quotient when scaled up by 10^x
    // still fits in 96 bits. Ultimately, we want to make scale positive - if we can't then
    // we're going to overflow. Because x is ultimately used to lookup inside the POWERS array, it
    // must be a valid value 0 <= x <= 9
    pub fn find_scale(&self, scale: i32) -> Option<usize> {
        const OVERFLOW_MAX_9_HI: u32 = 4;
        const OVERFLOW_MAX_8_HI: u32 = 42;
        const OVERFLOW_MAX_7_HI: u32 = 429;
        const OVERFLOW_MAX_6_HI: u32 = 4294;
        const OVERFLOW_MAX_5_HI: u32 = 42949;
        const OVERFLOW_MAX_4_HI: u32 = 429496;
        const OVERFLOW_MAX_3_HI: u32 = 4294967;
        const OVERFLOW_MAX_2_HI: u32 = 42949672;
        const OVERFLOW_MAX_1_HI: u32 = 429496729;
        const OVERFLOW_MAX_9_LOW64: u64 = 5441186219426131129;

        let hi = self.data[2];
        let low64 = self.low64();
        let mut x = 0usize;

        // Quick check to stop us from trying to scale any more.
        //
        if hi > OVERFLOW_MAX_1_HI {
            // If it's less than 0, which it probably is - overflow. We can't do anything.
            if scale < 0 {
                return None;
            }
            return Some(x);
        }

        if scale > MAX_SCALE_I32 - 9 {
            // We can't scale by 10^9 without exceeding the max scale factor.
            // Instead, we'll try to scale by the most that we can and see if that works.
            // This is safe to do due to the check above. e.g. scale > 19 in the above, so it will
            // evaluate to 9 or less below.
            x = (MAX_SCALE_I32 - scale) as usize;
            if hi < POWER_OVERFLOW_VALUES[x - 1].data[2] {
                if x as i32 + scale < 0 {
                    // We still overflow
                    return None;
                }
                return Some(x);
            }
        } else if hi < OVERFLOW_MAX_9_HI || hi == OVERFLOW_MAX_9_HI && low64 <= OVERFLOW_MAX_9_LOW64 {
            return Some(9);
        }

        // Do a binary search to find a power to scale by that is less than 9
        x = if hi > OVERFLOW_MAX_5_HI {
            if hi > OVERFLOW_MAX_3_HI {
                if hi > OVERFLOW_MAX_2_HI {
                    1
                } else {
                    2
                }
            } else if hi > OVERFLOW_MAX_4_HI {
                3
            } else {
                4
            }
        } else if hi > OVERFLOW_MAX_7_HI {
            if hi > OVERFLOW_MAX_6_HI {
                5
            } else {
                6
            }
        } else if hi > OVERFLOW_MAX_8_HI {
            7
        } else {
            8
        };

        // Double check what we've found won't overflow. Otherwise, we go one below.
        if hi == POWER_OVERFLOW_VALUES[x - 1].data[2] && low64 > POWER_OVERFLOW_VALUES[x - 1].low64() {
            x -= 1;
        }

        // Confirm we've actually resolved things
        if x as i32 + scale < 0 {
            None
        } else {
            Some(x)
        }
    }
}

// This is a table of the largest values that will not overflow when multiplied
// by a given power as represented by the index.
static POWER_OVERFLOW_VALUES: [Buf12; 8] = [
    Buf12 {
        data: [2576980377, 2576980377, 429496729],
    },
    Buf12 {
        data: [687194767, 4123168604, 42949672],
    },
    Buf12 {
        data: [2645699854, 1271310319, 4294967],
    },
    Buf12 {
        data: [694066715, 3133608139, 429496],
    },
    Buf12 {
        data: [2216890319, 2890341191, 42949],
    },
    Buf12 {
        data: [2369172679, 4154504685, 4294],
    },
    Buf12 {
        data: [4102387834, 2133437386, 429],
    },
    Buf12 {
        data: [410238783, 4078814305, 42],
    },
];

pub(super) struct Dec64 {
    pub negative: bool,
    pub scale: u32,
    pub hi: u32,
    pub low64: u64,
}

impl Dec64 {
    pub(super) const fn new(d: &Decimal) -> Dec64 {
        let m = d.mantissa_array3();
        if m[1] == 0 {
            Dec64 {
                negative: d.is_sign_negative(),
                scale: d.scale(),
                hi: m[2],
                low64: m[0] as u64,
            }
        } else {
            Dec64 {
                negative: d.is_sign_negative(),
                scale: d.scale(),
                hi: m[2],
                low64: ((m[1] as u64) << 32) | (m[0] as u64),
            }
        }
    }

    #[inline(always)]
    pub(super) const fn lo(&self) -> u32 {
        self.low64 as u32
    }
    #[inline(always)]
    pub(super) const fn mid(&self) -> u32 {
        (self.low64 >> 32) as u32
    }

    #[inline(always)]
    pub(super) const fn high64(&self) -> u64 {
        (self.low64 >> 32) | ((self.hi as u64) << 32)
    }

    pub(super) const fn to_decimal(&self) -> Decimal {
        Decimal::from_parts(
            self.low64 as u32,
            (self.low64 >> 32) as u32,
            self.hi,
            self.negative,
            self.scale,
        )
    }
}

pub struct Buf16 {
    pub data: [u32; 4],
}

impl Buf16 {
    pub const fn zero() -> Self {
        Buf16 { data: [0, 0, 0, 0] }
    }

    pub const fn low64(&self) -> u64 {
        ((self.data[1] as u64) << 32) | (self.data[0] as u64)
    }

    pub fn set_low64(&mut self, value: u64) {
        self.data[1] = (value >> 32) as u32;
        self.data[0] = value as u32;
    }

    pub const fn mid64(&self) -> u64 {
        ((self.data[2] as u64) << 32) | (self.data[1] as u64)
    }

    pub fn set_mid64(&mut self, value: u64) {
        self.data[2] = (value >> 32) as u32;
        self.data[1] = value as u32;
    }

    pub const fn high64(&self) -> u64 {
        ((self.data[3] as u64) << 32) | (self.data[2] as u64)
    }

    pub fn set_high64(&mut self, value: u64) {
        self.data[3] = (value >> 32) as u32;
        self.data[2] = value as u32;
    }
}

#[derive(Debug)]
pub struct Buf24 {
    pub data: [u32; 6],
}

impl Buf24 {
    pub const fn zero() -> Self {
        Buf24 {
            data: [0, 0, 0, 0, 0, 0],
        }
    }

    pub const fn low64(&self) -> u64 {
        ((self.data[1] as u64) << 32) | (self.data[0] as u64)
    }

    pub fn set_low64(&mut self, value: u64) {
        self.data[1] = (value >> 32) as u32;
        self.data[0] = value as u32;
    }

    #[allow(dead_code)]
    pub const fn mid64(&self) -> u64 {
        ((self.data[3] as u64) << 32) | (self.data[2] as u64)
    }

    pub fn set_mid64(&mut self, value: u64) {
        self.data[3] = (value >> 32) as u32;
        self.data[2] = value as u32;
    }

    #[allow(dead_code)]
    pub const fn high64(&self) -> u64 {
        ((self.data[5] as u64) << 32) | (self.data[4] as u64)
    }

    pub fn set_high64(&mut self, value: u64) {
        self.data[5] = (value >> 32) as u32;
        self.data[4] = value as u32;
    }

    pub const fn upper_word(&self) -> usize {
        if self.data[5] > 0 {
            return 5;
        }
        if self.data[4] > 0 {
            return 4;
        }
        if self.data[3] > 0 {
            return 3;
        }
        if self.data[2] > 0 {
            return 2;
        }
        if self.data[1] > 0 {
            return 1;
        }
        0
    }

    // Attempt to rescale the number into 96 bits. If successful, the scale is returned wrapped
    // in an Option. If it failed due to overflow, we return None.
    // * `upper` - Index of last non-zero value in self.
    // * `scale` - Current scale factor for this value.
    pub fn rescale(&mut self, upper: usize, scale: u32) -> Option<u32> {
        let mut scale = scale as i32;
        let mut upper = upper;

        // Determine a rescale target to start with
        let mut rescale_target = 0i32;
        if upper > 2 {
            rescale_target = upper as i32 * 32 - 64 - 1;
            rescale_target -= self.data[upper].leading_zeros() as i32;
            rescale_target = ((rescale_target * 77) >> 8) + 1;
            if rescale_target > scale {
                return None;
            }
        }

        // Make sure we scale enough to bring it into a valid range
        if rescale_target < scale - MAX_SCALE_I32 {
            rescale_target = scale - MAX_SCALE_I32;
        }

        if rescale_target > 0 {
            // We're going to keep reducing by powers of 10. So, start by reducing the scale by
            // that amount.
            scale -= rescale_target;
            let mut sticky = 0;
            let mut remainder = 0;
            loop {
                sticky |= remainder;
                let mut power = if rescale_target > 8 {
                    POWERS_10[9]
                } else {
                    POWERS_10[rescale_target as usize]
                };

                let high = self.data[upper];
                let high_quotient = high / power;
                remainder = high - high_quotient * power;

                for item in self.data.iter_mut().rev().skip(6 - upper) {
                    let num = (*item as u64).wrapping_add((remainder as u64) << 32);
                    *item = (num / power as u64) as u32;
                    remainder = (num as u32).wrapping_sub(item.wrapping_mul(power));
                }

                self.data[upper] = high_quotient;

                // If the high quotient was zero then decrease the upper bound
                if high_quotient == 0 && upper > 0 {
                    upper -= 1;
                }
                if rescale_target > MAX_I32_SCALE {
                    // Scale some more
                    rescale_target -= MAX_I32_SCALE;
                    continue;
                }

                // If we fit into 96 bits then we've scaled enough. Otherwise, scale once more.
                if upper > 2 {
                    if scale == 0 {
                        return None;
                    }
                    // Equivalent to scaling down by 10
                    rescale_target = 1;
                    scale -= 1;
                    continue;
                }

                // Round the final result.
                power >>= 1;
                let carried = if power <= remainder {
                    // If we're less than half then we're fine. Otherwise, we round if odd or if the
                    // sticky bit is set.
                    if power < remainder || ((self.data[0] & 1) | sticky) != 0 {
                        // Round up
                        self.data[0] = self.data[0].wrapping_add(1);
                        // Check if we carried
                        self.data[0] == 0
                    } else {
                        false
                    }
                } else {
                    false
                };

                // If we carried then propagate through the portions
                if carried {
                    let mut pos = 0;
                    for (index, value) in self.data.iter_mut().enumerate().skip(1) {
                        pos = index;
                        *value = value.wrapping_add(1);
                        if *value != 0 {
                            break;
                        }
                    }

                    // If we ended up rounding over the 96 bits then we'll try to rescale down (again)
                    if pos > 2 {
                        // Nothing to scale down from will cause overflow
                        if scale == 0 {
                            return None;
                        }

                        // Loop back around using scale of 10.
                        // Reset the sticky bit and remainder before looping.
                        upper = pos;
                        sticky = 0;
                        remainder = 0;
                        rescale_target = 1;
                        scale -= 1;
                        continue;
                    }
                }
                break;
            }
        }

        Some(scale as u32)
    }
}

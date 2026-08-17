use crate::constants::{MAX_I32_SCALE, POWERS_10, U32_MASK, U32_MAX};
use crate::decimal::Decimal;
use crate::ops::common::Dec64;

use core::cmp::Ordering;

pub(crate) fn cmp_impl(d1: &Decimal, d2: &Decimal) -> Ordering {
    if d2.is_zero() {
        return if d1.is_zero() {
            return Ordering::Equal;
        } else if d1.is_sign_negative() {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }
    if d1.is_zero() {
        return if d2.is_sign_negative() {
            Ordering::Greater
        } else {
            Ordering::Less
        };
    }
    // If the sign is different, then it's an easy answer
    if d1.is_sign_negative() != d2.is_sign_negative() {
        return if d1.is_sign_negative() {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }

    // Otherwise, do a deep comparison
    let d1 = Dec64::new(d1);
    let d2 = Dec64::new(d2);
    // We know both signs are the same here so flip it here.
    // Negative is handled differently. i.e. 0.5 > 0.01 however -0.5 < -0.01
    if d1.negative {
        cmp_internal(&d2, &d1)
    } else {
        cmp_internal(&d1, &d2)
    }
}

pub(in crate::ops) fn cmp_internal(d1: &Dec64, d2: &Dec64) -> Ordering {
    // This function ignores sign
    let mut d1_low = d1.low64;
    let mut d1_high = d1.hi;
    let mut d2_low = d2.low64;
    let mut d2_high = d2.hi;

    // If the scale factors aren't equal then
    if d1.scale != d2.scale {
        let mut diff = d2.scale as i32 - d1.scale as i32;
        if diff < 0 {
            diff = -diff;
            if !rescale(&mut d2_low, &mut d2_high, diff as u32) {
                return Ordering::Less;
            }
        } else if !rescale(&mut d1_low, &mut d1_high, diff as u32) {
            return Ordering::Greater;
        }
    }

    // They're the same scale, do a standard bitwise comparison
    let hi_order = d1_high.cmp(&d2_high);
    if hi_order != Ordering::Equal {
        return hi_order;
    }
    d1_low.cmp(&d2_low)
}

fn rescale(low64: &mut u64, high: &mut u32, diff: u32) -> bool {
    let mut diff = diff as i32;
    // We need to modify d1 by 10^diff to get it to the same scale as d2
    loop {
        let power = if diff >= MAX_I32_SCALE {
            POWERS_10[9]
        } else {
            POWERS_10[diff as usize]
        } as u64;
        let tmp_lo_32 = (*low64 & U32_MASK) * power;
        let mut tmp = (*low64 >> 32) * power + (tmp_lo_32 >> 32);
        *low64 = (tmp_lo_32 & U32_MASK) + (tmp << 32);
        tmp >>= 32;
        tmp = tmp.wrapping_add((*high as u64) * power);
        // Indicates > 96 bits
        if tmp > U32_MAX {
            return false;
        }
        *high = tmp as u32;

        // Keep scaling if there is more to go
        diff -= MAX_I32_SCALE;
        if diff <= 0 {
            break;
        }
    }

    true
}

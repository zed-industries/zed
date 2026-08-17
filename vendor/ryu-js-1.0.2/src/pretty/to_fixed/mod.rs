use crate::{
    d2s::{DOUBLE_BIAS, DOUBLE_EXPONENT_BITS, DOUBLE_MANTISSA_BITS},
    digit_table::DIGIT_TABLE,
    pretty::{
        format64,
        to_fixed::d2fixed_full_table::{
            ADDITIONAL_BITS_2, MIN_BLOCK_2, POW10_OFFSET, POW10_OFFSET_2, POW10_SPLIT,
            POW10_SPLIT_2,
        },
    },
};
#[cfg(feature = "no-panic")]
use no_panic::no_panic;

mod d2fixed_full_table;

/// Max bytes/characters required for `toFixed` representation of a [`f64`] value:
///
/// - 1 byte for sign (-)
/// - `22` bytes for whole part:
///   Because we have a check for if `>= 1e21` (1 byte extra, just in case)
/// - `1` byte for dot (`.`)
/// - `108` (`9 * 12`) bytes for fraction part:
///   We write digits in blocks, which consist of `9` digits.
///
/// Total: `1 + 22 + 1 + 108 = 132`
pub const MAX_BUFFER_SIZE: usize = 132;

pub struct Cursor {
    buffer: *mut u8,
    len: isize,
    index: isize,
}

impl Cursor {
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new(buffer: *mut u8, len: usize) -> Self {
        debug_assert!(!buffer.is_null());
        Self {
            buffer,
            len: len as isize,
            index: 0,
        }
    }

    /// Append one byte to buffer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that there is enough space for the given byte.
    #[cfg_attr(feature = "no-panic", no_panic)]
    unsafe fn append_byte(&mut self, c: u8) {
        debug_assert!(self.index < self.len);

        *self.buffer.offset(self.index) = c;
        self.index += 1;
    }

    /// Append the byte `count` times into the buffer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that there is enough space for the given bytes.
    #[cfg_attr(feature = "no-panic", no_panic)]
    unsafe fn append_bytes(&mut self, c: u8, count: usize) {
        debug_assert!(self.index + count as isize <= self.len);

        self.buffer.offset(self.index).write_bytes(c, count);
        self.index += count as isize;
    }

    /// Gets the current [`Cursor`] index.
    ///
    /// The `index` is also the amount of bytes that have been written into the buffer.
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn index(&self) -> usize {
        self.index as usize
    }

    /// Convert `digits` to decimal and write the last 9 decimal digits to result.
    /// If `digits` contains additional digits, then those are silently ignored.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the buffer has enough space for `9` bytes.
    #[cfg_attr(feature = "no-panic", no_panic)]
    unsafe fn append_nine_digits(&mut self, mut digits: u32) {
        let count = 9;

        debug_assert!(self.index + count <= self.len);

        if digits == 0 {
            self.append_bytes(b'0', 9);
            return;
        }

        let result = self.buffer.offset(self.index);

        for i in [0, 4] {
            let c = digits % 10000;
            digits /= 10000;
            let c0 = (c % 100) << 1;
            let c1 = (c / 100) << 1;

            // memcpy(result + 7 - i, DIGIT_TABLE + c0, 2);
            // memcpy(result + 5 - i, DIGIT_TABLE + c1, 2);
            result
                .offset(7 - i as isize)
                .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c0 as isize), 2);
            result
                .offset(5 - i as isize)
                .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c1 as isize), 2);
        }
        *(result.offset(0)) = b'0' + digits as u8;

        self.index += count;
    }

    /// Convert `digits` to a sequence of decimal digits. Append the digits to the result.
    ///
    /// # Safety
    ///
    /// The caller has to guarantee that:
    ///
    /// - 10^(olength-1) <= digits < 10^olength
    ///   e.g., by passing `olength` as `decimalLength9(digits)`.
    ///
    /// - That the buffer has enough space for the decimal length of the given integer.
    #[cfg_attr(feature = "no-panic", no_panic)]
    unsafe fn append_n_digits(&mut self, mut digits: u32) {
        let olength = decimal_length9(digits);

        debug_assert!(self.index + olength as isize <= self.len);

        let result = self.buffer.offset(self.index);

        let mut i = 0;
        while digits >= 10000 {
            let c = digits % 10000;

            digits /= 10000;
            let c0 = (c % 100) << 1;
            let c1 = (c / 100) << 1;

            // memcpy(result + olength - i - 2, DIGIT_TABLE + c0, 2);
            // memcpy(result + olength - i - 4, DIGIT_TABLE + c1, 2);
            result
                .offset(olength as isize - i as isize - 2)
                .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c0 as isize), 2);
            result
                .offset(olength as isize - i as isize - 4)
                .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c1 as isize), 2);

            i += 4;
        }
        if digits >= 100 {
            let c = (digits % 100) << 1;
            digits /= 100;

            // memcpy(result + olength - i - 2, DIGIT_TABLE + c, 2);
            result
                .offset(olength as isize - i as isize - 2)
                .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c as isize), 2);

            i += 2;
        }
        if digits >= 10 {
            let c = digits << 1;

            // memcpy(result + olength - i - 2, DIGIT_TABLE + c, 2);
            result
                .offset(olength as isize - i as isize - 2)
                .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c as isize), 2);
        } else {
            *result = b'0' + digits as u8;
        }

        self.index += olength as isize;
    }

    /// Convert `digits` to decimal and write the last `count` decimal digits to result.
    /// If `digits` contains additional digits, then those are silently ignored.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the buffer has enough space for the given `count`.
    #[cfg_attr(feature = "no-panic", no_panic)]
    unsafe fn append_c_digits(&mut self, count: u32, mut digits: u32) {
        debug_assert!(self.index + count as isize <= self.len);

        let result = self.buffer.offset(self.index);

        // Copy pairs of digits from DIGIT_TABLE.
        let mut i: u32 = 0;
        //   for (; i < count - 1; i += 2) {
        while i < count - 1 {
            let c: u32 = (digits % 100) << 1;
            digits /= 100;

            // memcpy(result + count - i - 2, DIGIT_TABLE + c, 2);
            result
                .offset((count - i - 2) as isize)
                .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c as isize), 2);

            i += 2;
        }
        // Generate the last digit if count is odd.
        if i < count {
            let c = b'0' + (digits % 10) as u8;

            // result[count - i - 1] = c;
            *result.offset((count - i - 1) as isize) = c;
        }

        self.index += count as isize;
    }

    /// Get the byte at the given index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is within `[0, len)`.
    #[cfg_attr(feature = "no-panic", no_panic)]
    unsafe fn get(&mut self, i: isize) -> u8 {
        debug_assert!((0..self.len).contains(&i));

        *self.buffer.offset(i)
    }

    /// Set the byte at the given index with the value.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is within `[0, len)`.
    #[cfg_attr(feature = "no-panic", no_panic)]
    unsafe fn set(&mut self, i: isize, c: u8) {
        debug_assert!((0..self.len).contains(&i));

        *self.buffer.offset(i) = c;
    }
}

/// Because of the abs(f) >= 1e21 check, falls back to ToString ([`format64`]).
///
/// See tests.
const MAX_EXPONENT: u32 = 0b100_0100_0100; // 1029
const MIN_EXPONENT: u16 = 0b010_1001_0011;

/// `e2 = exponent - bias - |mantissa|`
const MAX_E2: i32 = MAX_EXPONENT as i32 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32;
const MIN_E2: i32 = MIN_EXPONENT as i32 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32;

const MAX_POW10_SPLIT_2_INX: i32 = -MIN_E2 / 16;

const POW10_ADDITIONAL_BITS: u32 = 120;

/// Returns `floor(log_10(2^e))` requires `0 <= e <= 1650`.
#[cfg_attr(feature = "no-panic", no_panic)]
fn log10_pow2(e: i32) -> u32 {
    // The first value this approximation fails for is 2^1651 which is just greater than 10^297.
    debug_assert!((0..=1650).contains(&e));

    ((e as u32) * 78913) >> 18
}

/// Get index from `e2` value.
///
/// Range `[0, 2]` inclusive.
#[cfg_attr(feature = "no-panic", no_panic)]
fn index_for_exponent(e: u32) -> u32 {
    debug_assert!((0..=MAX_E2 as u32).contains(&e));

    let result = (e + 15) / 16;

    debug_assert!((0..=2).contains(&result));

    result
}

#[cfg_attr(feature = "no-panic", no_panic)]
fn pow10_bits_for_index(idx: u32) -> u32 {
    16 * idx + POW10_ADDITIONAL_BITS
}

/// Get the length from the index.
///
/// Range `[2, 3]` inclusive.
///
// TODO: Because the ranges are so small we could have tables, too speed up execution.
#[cfg_attr(feature = "no-panic", no_panic)]
fn length_for_index(idx: u32) -> u32 {
    // +1 for ceil, +16 for mantissa, +8 to round up when dividing by 9
    (log10_pow2(16 * idx as i32) + 1 + 16 + 8) / 9
}

#[cfg_attr(feature = "no-panic", no_panic)]
fn umul256(a: u128, b_hi: u64, b_lo: u64) -> (u128, u128) {
    let a_lo = a as u64;
    let a_hi = (a >> 64) as u64;

    let b00 = (a_lo as u128) * (b_lo as u128);
    let b01 = (a_lo as u128) * (b_hi as u128);
    let b10 = (a_hi as u128) * (b_lo as u128);
    let b11 = (a_hi as u128) * (b_hi as u128);

    let b00_lo = b00 as u64;
    let b00_hi = (b00 >> 64) as u64;

    let mid1 = b10 + b00_hi as u128;
    let mid1_lo = (mid1) as u64;
    let mid1_hi = (mid1 >> 64) as u64;

    let mid2 = b01 + mid1_lo as u128;
    let mid2_lo = (mid2) as u64;
    let mid2_hi = (mid2 >> 64) as u64;

    let p_hi = b11 + mid1_hi as u128 + mid2_hi as u128;
    let p_lo = ((mid2_lo as u128) << 64) | b00_lo as u128;

    (p_hi, p_lo)
}

// Returns the high 128 bits of the 256-bit product of a and b.
#[cfg_attr(feature = "no-panic", no_panic)]
fn umul256_hi(a: u128, b_hi: u64, b_lo: u64) -> u128 {
    // Reuse the umul256 implementation.
    // Optimizers will likely eliminate the instructions used to compute the
    // low part of the product.
    let (hi, _lo) = umul256(a, b_hi, b_lo);
    hi
}

// Unfortunately, gcc/clang do not automatically turn a 128-bit integer division
// into a multiplication, so we have to do it manually.
#[cfg_attr(feature = "no-panic", no_panic)]
fn uint128_mod1e9(v: u128) -> u32 {
    // After multiplying, we're going to shift right by 29, then truncate to uint32_t.
    // This means that we need only 29 + 32 = 61 bits, so we can truncate to uint64_t before shifting.
    let multiplied = umul256_hi(v, 0x89705F4136B4A597, 0x31680A88F8953031) as u64;

    // For uint32_t truncation, see the mod1e9() comment in d2s_intrinsics.rs
    let shifted = (multiplied >> 29) as u32;

    (v as u32).wrapping_sub(1000000000u32.wrapping_mul(shifted))
}

// Best case: use 128-bit type.
#[cfg_attr(feature = "no-panic", no_panic)]
fn mul_shift_mod1e9(m: u64, mul: &[u64; 3], j: i32) -> u32 {
    let b0 = m as u128 * mul[0] as u128; // 0
    let b1 = m as u128 * mul[1] as u128; // 64
    let b2 = m as u128 * mul[2] as u128; // 128

    debug_assert!((128..=180).contains(&j));

    let mid = b1 + ((b0 >> 64) as u64) as u128; // 64
    let s1 = b2 + ((mid >> 64) as u64) as u128; // 128
    uint128_mod1e9(s1 >> (j - 128))
}

// Returns the number of decimal digits in v, which must not contain more than 9 digits.
#[cfg_attr(feature = "no-panic", no_panic)]
fn decimal_length9(v: u32) -> u32 {
    // Function precondition: v is not a 10-digit number.
    // (f2s: 9 digits are sufficient for round-tripping.)
    // (d2fixed: We print 9-digit blocks.)
    debug_assert!(v < 1000000000);

    if v >= 100000000 {
        9
    } else if v >= 10000000 {
        8
    } else if v >= 1000000 {
        7
    } else if v >= 100000 {
        6
    } else if v >= 10000 {
        5
    } else if v >= 1000 {
        4
    } else if v >= 100 {
        3
    } else if v >= 10 {
        2
    } else {
        1
    }
}

/// Print [`f64`] to the given buffer using fixed notation,
/// as defined in the ECMAScript `Number.prototype.toFixed()` method
/// and return number of bytes written.
///
/// At most 132 bytes will be written.
///
/// ## Special cases
///
/// This function **does not** check for NaN or infinity. If the input
/// number is not a finite float, the printed representation will be some
/// correctly formatted but unspecified numerical value.
///
/// Please check [`is_finite`] yourself before calling this function, or
/// check [`is_nan`] and [`is_infinite`] and handle those cases yourself.
///
/// [`is_finite`]: https://doc.rust-lang.org/std/primitive.f64.html#method.is_finite
/// [`is_nan`]: https://doc.rust-lang.org/std/primitive.f64.html#method.is_nan
/// [`is_infinite`]: https://doc.rust-lang.org/std/primitive.f64.html#method.is_infinite
///
/// ## Safety
///
/// The `result` pointer argument must point to sufficiently many writable bytes
/// to hold Ryū's representation of `f`.
///
/// ## Example
///
/// ```
/// use std::{mem::MaybeUninit, slice, str};
///
/// let f = 1.235f64;
///
/// unsafe {
///     let mut buffer = [MaybeUninit::<u8>::uninit(); 132];
///     let len = ryu_js::raw::format64_to_fixed(f, 2, buffer.as_mut_ptr() as *mut u8);
///     let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
///     let print = str::from_utf8_unchecked(slice);
///     assert_eq!(print, "1.24");
/// }
/// ```
#[must_use]
#[cfg_attr(feature = "no-panic", no_panic)]
pub unsafe fn format64_to_fixed(f: f64, fraction_digits: u8, result: *mut u8) -> usize {
    // SKIPPED: 1. Let x be ? thisNumberValue(this value).
    // SKIPPED: 2. Let f be ? ToIntegerOrInfinity(fractionDigits).
    // SKIPPED: 3. Assert: If fractionDigits is undefined, then f is 0.
    // SKIPPED: 4. If f is not finite, throw a RangeError exception.
    // 5. If f < 0 or f > 100, throw a RangeError exception.
    debug_assert!((0..=100).contains(&fraction_digits));

    // 10. If x ≥ 10^21, then
    let f_abs = if f < 0.0 { -f } else { f };
    if f_abs >= 1e21 {
        // a. Let m be ! ToString(𝔽(x)).
        return format64(f, result);
    }

    let mut result = Cursor::new(result, MAX_BUFFER_SIZE);

    let bits = f.to_bits();
    let sign = ((bits >> (DOUBLE_MANTISSA_BITS + DOUBLE_EXPONENT_BITS)) & 1) != 0;
    let ieee_mantissa = bits & ((1u64 << DOUBLE_MANTISSA_BITS) - 1);
    let ieee_exponent =
        (bits >> DOUBLE_MANTISSA_BITS) as u32 & ((1u32 << DOUBLE_EXPONENT_BITS) - 1);

    // Special case when it's 0 or -0 it's the same.
    //
    // Return and append '.' and '0's is needed.
    //
    // See: https://tc39.es/ecma262/#%E2%84%9D
    if ieee_exponent == 0 && ieee_mantissa == 0 {
        result.append_byte(b'0');
        if fraction_digits == 0 {
            return result.index();
        }
        result.append_byte(b'.');
        result.append_bytes(b'0', fraction_digits as usize);
        return result.index();
    }

    debug_assert!((0..=MAX_EXPONENT).contains(&ieee_exponent));

    if sign {
        result.append_byte(b'-');
    }

    let (e2, m2) = if ieee_exponent == 0 {
        (1 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32, ieee_mantissa)
    } else {
        (
            ieee_exponent as i32 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32,
            (1 << DOUBLE_MANTISSA_BITS) | ieee_mantissa,
        )
    };

    debug_assert!((..=MAX_E2).contains(&e2));

    let mut nonzero = false;

    // Write the whole part (integral part) of the floating point.
    //
    // xxxxxxx.1234567 (write xs)
    if e2 >= -(DOUBLE_MANTISSA_BITS as i32) {
        // 0 <= idx <= 2
        let idx = if e2 < 0 {
            0
        } else {
            index_for_exponent(e2 as u32)
        };
        let p10bits = pow10_bits_for_index(idx);
        let len = length_for_index(idx) as i32;

        for i in (0..len).rev() {
            let j = p10bits as i32 - e2;
            // SAFETY: 0 <= idx <= 2, putting idx inside the index bounds of `POW10_OFFSET`.
            let split_idx = *POW10_OFFSET.get_unchecked(idx as usize) as usize;

            // SAFETY: The max value inside `POW10_OFFSET` is 5, and the max value of `i` is 2,
            // putting `split_idx + i` inside the index bounds of `POW10_SPLIT`.
            let mul = POW10_SPLIT.get_unchecked(split_idx + i as usize);

            // Temporary: j is usually around 128, and by shifting a bit, we push it to 128 or above, which is
            // a slightly faster code path in mulShift_mod1e9. Instead, we can just increase the multipliers.
            let digits = mul_shift_mod1e9(m2 << 8, mul, j + 8);
            if nonzero {
                result.append_nine_digits(digits);
            } else if digits != 0 {
                result.append_n_digits(digits);
                nonzero = true;
            }
        }
    }

    // If the whole part is zero (nothing was writen), write a zero.
    if !nonzero {
        result.append_byte(b'0');
    }

    // If fraction_digits is not zero, then write the dot.
    if fraction_digits != 0 {
        result.append_byte(b'.');
    }

    // Check if it has fractional part.
    if e2 >= 0 {
        result.append_bytes(b'0', fraction_digits as usize);
        return result.index();
    }

    // Write fractional part.
    //
    // 1234567.yyyyyyy (write ys)

    let fraction_digits = fraction_digits as u32;

    let idx = (-e2 / 16).min(MAX_POW10_SPLIT_2_INX) as usize;

    let min_block = MIN_BLOCK_2[idx];

    // fraction_digits is defined to be [0, 100] inclusive.
    //
    // Therefore blocks can be [1, 12] inclusive.
    let blocks: u32 = fraction_digits / 9 + 1;
    if blocks <= min_block as u32 {
        result.append_bytes(b'0', fraction_digits as usize);
        return result.index();
    }

    debug_assert!(idx <= 25);

    let mut round_up = false;

    for i in 0..blocks {
        let p: isize = POW10_OFFSET_2[idx] as isize + i as isize - min_block as isize;
        debug_assert!(p >= 0);
        let p = p as usize;

        // SAFETY: `idx` <= 26 per the min operation above. If `idx == 26` (which is the last index
        // of `POW10_OFFSET_2`), blocks <= min_block will always be true, since `1 <= blocks <= 12`
        // and `MIN_BLOCK_2[26]` = 12. Hence, for that value of `idx` this won't be executed.
        // Finally, for `idx <= 25` it is always true that `idx + 1 <= 26`, making this access always
        // in bounds for `POW10_OFFSET_2`.
        if p >= *POW10_OFFSET_2.get_unchecked(idx + 1) as usize {
            // If the remaining digits are all 0, then we might as well use memset.
            // No rounding required in this case.
            let fill = fraction_digits as usize - 9 * i as usize;
            // memset(result + index, '0', fill);
            result.append_bytes(b'0', fill);
            break;
        }

        debug_assert!(p <= 480);

        // Temporary: j is usually around 128, and by shifting a bit, we push it to 128 or above, which is
        // a slightly faster code path in mulShift_mod1e9. Instead, we can just increase the multipliers.
        let j: isize = ADDITIONAL_BITS_2 as isize + (-(e2 as isize) - 16 * idx as isize);

        // SAFETY: Since `idx <= 25`, the maximum value of `POW10_OFFSET_2[idx]` must be `480` for
        // `idx == 25`.
        // However, this also means that `min_block == 11` for that value of `idx`.
        // Hence, `POW10_OFFSET_2[25] - MIN_BLOCK_2[25] == 469`, and for that value of `blocks`,
        // `0 <= 1 <= 10`.
        //
        // This shows that the maximum value of `p` is `480`, which is exactly the biggest valid
        // index for `POW10_SPLIT_2`.
        let mut digits: u32 =
            mul_shift_mod1e9(m2 << 8, POW10_SPLIT_2.get_unchecked(p), j as i32 + 8);

        if i < blocks - 1 {
            result.append_nine_digits(digits);
        } else {
            let maximum: u32 = fraction_digits - 9 * i;
            let mut last_digit: u32 = 0;
            for _k in 0..(9 - maximum) {
                last_digit = digits % 10;
                digits /= 10;
            }

            // If last digit is 5 or above, round up.
            round_up = last_digit >= 5;

            if maximum != 0 {
                result.append_c_digits(maximum, digits);
            }
            break;
        }
    }

    // Roundup if needed.
    if round_up {
        let mut round_index = result.index;
        let mut dot_index = 0; // '.' can't be located at index 0
        loop {
            round_index -= 1;

            let c = result.get(round_index);
            if round_index == -1 || c == b'-' {
                result.set(round_index + 1, b'1');
                if dot_index > 0 {
                    result.set(dot_index, b'0');
                    result.set(dot_index + 1, b'.');
                }
                result.append_byte(b'0');
                break;
            }
            if c == b'.' {
                dot_index = round_index;
                continue;
            } else if c == b'9' {
                result.set(round_index, b'0');
                continue;
            }

            result.set(round_index, c + 1);
            break;
        }
    }

    result.index()
}

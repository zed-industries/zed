use crate::pretty::to_fixed::MAX_BUFFER_SIZE;

use crate::raw;
use core::mem::MaybeUninit;
use core::{slice, str};
#[cfg(feature = "no-panic")]
use no_panic::no_panic;

const NAN: &str = "NaN";
const INFINITY: &str = "Infinity";
const NEG_INFINITY: &str = "-Infinity";

const BUFFER_SIZE: usize = MAX_BUFFER_SIZE;

/// Safe API for formatting floating point numbers to text.
///
/// ## Example
///
/// ```
/// let mut buffer = ryu_js::Buffer::new();
/// let printed = buffer.format_finite(1.234);
/// assert_eq!(printed, "1.234");
/// ```
#[derive(Copy)]
pub struct Buffer {
    bytes: [MaybeUninit<u8>; BUFFER_SIZE],
}

impl Buffer {
    /// This is a cheap operation; you don't need to worry about reusing buffers
    /// for efficiency.
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new() -> Self {
        let bytes = [MaybeUninit::<u8>::uninit(); BUFFER_SIZE];

        Buffer { bytes }
    }

    /// Print a floating point number into this buffer and return a reference to
    /// its string representation within the buffer.
    ///
    /// # Special cases
    ///
    /// This function formats NaN as the string "NaN", positive infinity as
    /// "Infinity", and negative infinity as "-Infinity" to match the [ECMAScript specification][spec].
    ///
    /// If your input is known to be finite, you may get better performance by
    /// calling the `format_finite` method instead of `format` to avoid the
    /// checks for special cases.
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-numeric-types-number-tostring
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format<F: Float>(&mut self, f: F) -> &str {
        if f.is_nonfinite() {
            f.format_nonfinite()
        } else {
            self.format_finite(f)
        }
    }

    /// Print a floating point number into this buffer and return a reference to
    /// its string representation within the buffer.
    ///
    /// # Special cases
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
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format_finite<F: Float>(&mut self, f: F) -> &str {
        unsafe {
            let n = f.write_to_ryu_buffer(self.bytes.as_mut_ptr().cast::<u8>());
            debug_assert!(n <= self.bytes.len());
            let slice = slice::from_raw_parts(self.bytes.as_ptr().cast::<u8>(), n);
            str::from_utf8_unchecked(slice)
        }
    }

    /// Print a floating point number into this buffer using the `Number.prototype.toFixed()` notation
    /// and return a reference to its string representation within the buffer.
    ///
    /// The `fraction_digits` argument must be between `[0, 100]` inclusive,
    /// If a values value that is greater than the max is passed in will be clamped to max.
    ///
    /// # Special cases
    ///
    /// This function formats NaN as the string "NaN", positive infinity as
    /// "Infinity", and negative infinity as "-Infinity" to match the [ECMAScript specification][spec].
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-numeric-types-number-tofixed
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format_to_fixed<F: FloatToFixed>(&mut self, f: F, fraction_digits: u8) -> &str {
        let fraction_digits = fraction_digits.min(100);

        if f.is_nonfinite() {
            return f.format_nonfinite();
        }

        unsafe {
            let n = f.write_to_ryu_buffer_to_fixed(
                fraction_digits,
                self.bytes.as_mut_ptr().cast::<u8>(),
            );
            debug_assert!(n <= self.bytes.len());
            let slice = slice::from_raw_parts(self.bytes.as_ptr().cast::<u8>(), n);
            str::from_utf8_unchecked(slice)
        }
    }
}

impl Clone for Buffer {
    #[inline]
    #[allow(clippy::non_canonical_clone_impl)] // false positive https://github.com/rust-lang/rust-clippy/issues/11072
    fn clone(&self) -> Self {
        Buffer::new()
    }
}

impl Default for Buffer {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn default() -> Self {
        Buffer::new()
    }
}

/// A floating point number, [`f32`] or [`f64`], that can be written into a
/// [`ryu_js::Buffer`][Buffer].
///
/// This trait is sealed and cannot be implemented for types outside of the
/// `ryu-js` crate.
pub trait Float: Sealed {}
impl Float for f32 {}
impl Float for f64 {}

/// A floating point number that can be written into a
/// [`ryu_js::Buffer`][Buffer] using the fixed notation as defined in the
/// [`Number.prototype.toFixed( fractionDigits )`][spec] ECMAScript specification.
///
/// This trait is sealed and cannot be implemented for types outside of the
/// `ryu-js` crate.
///
/// [spec]: https://tc39.es/ecma262/#sec-number.prototype.tofixed
pub trait FloatToFixed: Sealed {}
impl FloatToFixed for f64 {}

pub trait Sealed: Copy {
    fn is_nonfinite(self) -> bool;
    fn format_nonfinite(self) -> &'static str;
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize;
    unsafe fn write_to_ryu_buffer_to_fixed(self, fraction_digits: u8, result: *mut u8) -> usize;
}

impl Sealed for f32 {
    #[inline]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u32 = 0x7f800000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }

    #[cold]
    #[cfg_attr(feature = "no-panic", inline)]
    fn format_nonfinite(self) -> &'static str {
        const MANTISSA_MASK: u32 = 0x007fffff;
        const SIGN_MASK: u32 = 0x80000000;
        let bits = self.to_bits();
        if bits & MANTISSA_MASK != 0 {
            NAN
        } else if bits & SIGN_MASK != 0 {
            NEG_INFINITY
        } else {
            INFINITY
        }
    }

    #[inline]
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        raw::format32(self, result)
    }

    #[inline]
    unsafe fn write_to_ryu_buffer_to_fixed(self, _fraction_digits: u8, _result: *mut u8) -> usize {
        panic!("toFixed for f32 type is not implemented yet!")
    }
}

impl Sealed for f64 {
    #[inline]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u64 = 0x7ff0000000000000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }

    #[cold]
    #[cfg_attr(feature = "no-panic", inline)]
    fn format_nonfinite(self) -> &'static str {
        const MANTISSA_MASK: u64 = 0x000fffffffffffff;
        const SIGN_MASK: u64 = 0x8000000000000000;
        let bits = self.to_bits();
        if bits & MANTISSA_MASK != 0 {
            NAN
        } else if bits & SIGN_MASK != 0 {
            NEG_INFINITY
        } else {
            INFINITY
        }
    }

    #[inline]
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        raw::format64(self, result)
    }

    #[inline]
    unsafe fn write_to_ryu_buffer_to_fixed(self, fraction_digits: u8, result: *mut u8) -> usize {
        raw::format64_to_fixed(self, fraction_digits, result)
    }
}

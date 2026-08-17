//! [![github]](https://github.com/dtolnay/dtoa)&ensp;[![crates-io]](https://crates.io/crates/dtoa)&ensp;[![docs-rs]](https://docs.rs/dtoa)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! This crate provides fast conversion of floating point primitives to decimal
//! strings. The implementation is a straightforward Rust port of [Milo Yip]'s
//! C++ implementation [dtoa.h]. The original C++ code of each function is
//! included in comments.
//!
//! See also [`itoa`] for printing integer primitives.
//!
//! [Milo Yip]: https://github.com/miloyip
//! [dtoa.h]: https://github.com/miloyip/rapidjson/blob/master/include/rapidjson/internal/dtoa.h
//! [`itoa`]: https://github.com/dtolnay/itoa
//!
//! # Example
//!
//! ```
//! fn main() {
//!     let mut buffer = dtoa::Buffer::new();
//!     let printed = buffer.format(2.71828f64);
//!     assert_eq!(printed, "2.71828");
//! }
//! ```
//!
//! ## Performance (lower is better)
//!
//! ![performance](https://raw.githubusercontent.com/dtolnay/dtoa/master/performance.png)

#![doc(html_root_url = "https://docs.rs/dtoa/1.0.10")]
#![no_std]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::doc_markdown,
    clippy::expl_impl_clone_on_copy,
    clippy::if_not_else,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::range_plus_one,
    clippy::semicolon_if_nothing_returned, // https://github.com/rust-lang/rust-clippy/issues/7768
    clippy::shadow_unrelated,
    clippy::suspicious_else_formatting,
    clippy::transmute_float_to_int,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix
)]

#[macro_use]
mod diyfp;
#[macro_use]
mod dtoa;

use core::mem::{self, MaybeUninit};
use core::slice;
use core::str;
#[cfg(feature = "no-panic")]
use no_panic::no_panic;

const NAN: &str = "NaN";
const INFINITY: &str = "inf";
const NEG_INFINITY: &str = "-inf";

/// A correctly sized stack allocation for the formatted float to be written
/// into.
///
/// # Example
///
/// ```
/// let mut buffer = dtoa::Buffer::new();
/// let printed = buffer.format_finite(2.71828);
/// assert_eq!(printed, "2.71828");
/// ```
pub struct Buffer {
    bytes: [MaybeUninit<u8>; 25],
}

impl Default for Buffer {
    #[inline]
    fn default() -> Buffer {
        Buffer::new()
    }
}

impl Copy for Buffer {}

impl Clone for Buffer {
    #[inline]
    #[allow(clippy::non_canonical_clone_impl)] // false positive https://github.com/rust-lang/rust-clippy/issues/11072
    fn clone(&self) -> Self {
        Buffer::new()
    }
}

impl Buffer {
    /// This is a cheap operation; you don't need to worry about reusing buffers
    /// for efficiency.
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new() -> Buffer {
        let bytes = [MaybeUninit::<u8>::uninit(); 25];
        Buffer { bytes }
    }

    /// Print a floating point number into this buffer and return a reference to
    /// its string representation within the buffer.
    ///
    /// # Special cases
    ///
    /// This function formats NaN as the string "NaN", positive infinity as
    /// "inf", and negative infinity as "-inf" to match std::fmt.
    ///
    /// If your input is known to be finite, you may get better performance by
    /// calling the `format_finite` method instead of `format` to avoid the
    /// checks for special cases.
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format<F: Float>(&mut self, value: F) -> &str {
        if value.is_nonfinite() {
            value.format_nonfinite()
        } else {
            self.format_finite(value)
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
    /// [`is_finite`]: f64::is_finite
    /// [`is_nan`]: f64::is_nan
    /// [`is_infinite`]: f64::is_infinite
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format_finite<F: Float>(&mut self, value: F) -> &str {
        value.write(self)
    }
}

/// A floating point number that can be written into a [`dtoa::Buffer`][Buffer].
///
/// This trait is sealed and cannot be implemented for types outside of dtoa.
pub trait Float: private::Sealed {}

impl Float for f32 {}
impl Float for f64 {}

// Seal to prevent downstream implementations of Float trait.
mod private {
    pub trait Sealed: Copy {
        fn is_nonfinite(self) -> bool;
        fn format_nonfinite(self) -> &'static str;
        fn write(self, buf: &mut crate::Buffer) -> &str;
    }
}

impl private::Sealed for f32 {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u32 = 0x7f800000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }

    #[cold]
    #[cfg_attr(feature = "no-panic", no_panic)]
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
    fn write(self, buf: &mut Buffer) -> &str {
        dtoa! {
            floating_type: f32,
            significand_type: u32,
            exponent_type: i32,

            diy_significand_size: 32,
            significand_size: 23,
            exponent_bias: 0x7F,
            mask_type: u32,
            exponent_mask: 0x7F800000,
            significand_mask: 0x007FFFFF,
            hidden_bit: 0x00800000,
            cached_powers_f: CACHED_POWERS_F_32,
            cached_powers_e: CACHED_POWERS_E_32,
            min_power: (-36),
        };
        unsafe { dtoa(buf, self) }
    }
}

impl private::Sealed for f64 {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn is_nonfinite(self) -> bool {
        const EXP_MASK: u64 = 0x7ff0000000000000;
        let bits = self.to_bits();
        bits & EXP_MASK == EXP_MASK
    }

    #[cold]
    #[cfg_attr(feature = "no-panic", no_panic)]
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
    fn write(self, buf: &mut Buffer) -> &str {
        dtoa! {
            floating_type: f64,
            significand_type: u64,
            exponent_type: isize,

            diy_significand_size: 64,
            significand_size: 52,
            exponent_bias: 0x3FF,
            mask_type: u64,
            exponent_mask: 0x7FF0000000000000,
            significand_mask: 0x000FFFFFFFFFFFFF,
            hidden_bit: 0x0010000000000000,
            cached_powers_f: CACHED_POWERS_F_64,
            cached_powers_e: CACHED_POWERS_E_64,
            min_power: (-348),
        };
        unsafe { dtoa(buf, self) }
    }
}

////////////////////////////////////////////////////////////////////////////////

const MAX_DECIMAL_PLACES: isize = 324;

static DEC_DIGITS_LUT: [u8; 200] = *b"\
    0001020304050607080910111213141516171819\
    2021222324252627282930313233343536373839\
    4041424344454647484950515253545556575859\
    6061626364656667686970717273747576777879\
    8081828384858687888990919293949596979899";

// 10^-36, 10^-28, ..., 10^52
#[rustfmt::skip]
static CACHED_POWERS_F_32: [u32; 12] = [
    0xaa242499, 0xfd87b5f3, 0xbce50865, 0x8cbccc09,
    0xd1b71759, 0x9c400000, 0xe8d4a510, 0xad78ebc6,
    0x813f3979, 0xc097ce7c, 0x8f7e32ce, 0xd5d238a5,
];

#[rustfmt::skip]
static CACHED_POWERS_E_32: [i16; 12] = [
    -151, -125, -98, -71, -45, -18, 8, 35, 62, 88, 115, 141,
];

// 10^-348, 10^-340, ..., 10^340
#[rustfmt::skip]
static CACHED_POWERS_F_64: [u64; 87] = [
    0xfa8fd5a0081c0288, 0xbaaee17fa23ebf76,
    0x8b16fb203055ac76, 0xcf42894a5dce35ea,
    0x9a6bb0aa55653b2d, 0xe61acf033d1a45df,
    0xab70fe17c79ac6ca, 0xff77b1fcbebcdc4f,
    0xbe5691ef416bd60c, 0x8dd01fad907ffc3c,
    0xd3515c2831559a83, 0x9d71ac8fada6c9b5,
    0xea9c227723ee8bcb, 0xaecc49914078536d,
    0x823c12795db6ce57, 0xc21094364dfb5637,
    0x9096ea6f3848984f, 0xd77485cb25823ac7,
    0xa086cfcd97bf97f4, 0xef340a98172aace5,
    0xb23867fb2a35b28e, 0x84c8d4dfd2c63f3b,
    0xc5dd44271ad3cdba, 0x936b9fcebb25c996,
    0xdbac6c247d62a584, 0xa3ab66580d5fdaf6,
    0xf3e2f893dec3f126, 0xb5b5ada8aaff80b8,
    0x87625f056c7c4a8b, 0xc9bcff6034c13053,
    0x964e858c91ba2655, 0xdff9772470297ebd,
    0xa6dfbd9fb8e5b88f, 0xf8a95fcf88747d94,
    0xb94470938fa89bcf, 0x8a08f0f8bf0f156b,
    0xcdb02555653131b6, 0x993fe2c6d07b7fac,
    0xe45c10c42a2b3b06, 0xaa242499697392d3,
    0xfd87b5f28300ca0e, 0xbce5086492111aeb,
    0x8cbccc096f5088cc, 0xd1b71758e219652c,
    0x9c40000000000000, 0xe8d4a51000000000,
    0xad78ebc5ac620000, 0x813f3978f8940984,
    0xc097ce7bc90715b3, 0x8f7e32ce7bea5c70,
    0xd5d238a4abe98068, 0x9f4f2726179a2245,
    0xed63a231d4c4fb27, 0xb0de65388cc8ada8,
    0x83c7088e1aab65db, 0xc45d1df942711d9a,
    0x924d692ca61be758, 0xda01ee641a708dea,
    0xa26da3999aef774a, 0xf209787bb47d6b85,
    0xb454e4a179dd1877, 0x865b86925b9bc5c2,
    0xc83553c5c8965d3d, 0x952ab45cfa97a0b3,
    0xde469fbd99a05fe3, 0xa59bc234db398c25,
    0xf6c69a72a3989f5c, 0xb7dcbf5354e9bece,
    0x88fcf317f22241e2, 0xcc20ce9bd35c78a5,
    0x98165af37b2153df, 0xe2a0b5dc971f303a,
    0xa8d9d1535ce3b396, 0xfb9b7cd9a4a7443c,
    0xbb764c4ca7a44410, 0x8bab8eefb6409c1a,
    0xd01fef10a657842c, 0x9b10a4e5e9913129,
    0xe7109bfba19c0c9d, 0xac2820d9623bf429,
    0x80444b5e7aa7cf85, 0xbf21e44003acdd2d,
    0x8e679c2f5e44ff8f, 0xd433179d9c8cb841,
    0x9e19db92b4e31ba9, 0xeb96bf6ebadf77d9,
    0xaf87023b9bf0ee6b,
];

#[rustfmt::skip]
static CACHED_POWERS_E_64: [i16; 87] = [
    -1220, -1193, -1166, -1140, -1113, -1087, -1060, -1034, -1007,  -980,
    -954,   -927,  -901,  -874,  -847,  -821,  -794,  -768,  -741,  -715,
    -688,   -661,  -635,  -608,  -582,  -555,  -529,  -502,  -475,  -449,
    -422,   -396,  -369,  -343,  -316,  -289,  -263,  -236,  -210,  -183,
    -157,   -130,  -103,   -77,   -50,   -24,     3,    30,    56,    83,
     109,    136,   162,   189,   216,   242,   269,   295,   322,   348,
     375,    402,   428,   455,   481,   508,   534,   561,   588,   614,
     641,    667,   694,   720,   747,   774,   800,   827,   853,   880,
     907,    933,   960,   986,  1013,  1039,  1066,
];

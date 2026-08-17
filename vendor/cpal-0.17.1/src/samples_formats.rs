//! Audio sample format types and conversions.
//!
//! # Byte Order
//!
//! All multi-byte sample formats use the native endianness of the target platform.
//! CPAL handles any necessary conversions when interfacing with hardware that uses
//! a different byte order.

use std::{fmt::Display, mem};
#[cfg(all(
    target_arch = "wasm32",
    any(target_os = "emscripten", feature = "wasm-bindgen")
))]
use wasm_bindgen::prelude::*;

pub use dasp_sample::{FromSample, Sample};

/// 24-bit signed integer sample type.
///
/// Represents 24-bit audio with range `-(1 << 23)..=((1 << 23) - 1)`.
///
/// **Note:** While representing 24-bit audio, this format uses 4 bytes (i32) of storage
/// with the most significant byte unused. Use [`SampleFormat::bits_per_sample`] to get
/// the actual bit depth (24) vs [`SampleFormat::sample_size`] for storage size (4 bytes).
pub use dasp_sample::I24;

/// 24-bit unsigned integer sample type.
///
/// Represents 24-bit audio with range `0..=((1 << 24) - 1)`, with origin at `1 << 23 == 8388608`.
///
/// **Note:** While representing 24-bit audio, this format uses 4 bytes (u32) of storage
/// with the most significant byte unused. Use [`SampleFormat::bits_per_sample`] to get
/// the actual bit depth (24) vs [`SampleFormat::sample_size`] for storage size (4 bytes).
pub use dasp_sample::U24;

// I48 and U48 are not currently supported by cpal but available in dasp_sample:
// pub use dasp_sample::{I48, U48};

/// Format that each sample has. Usually, this corresponds to the sampling
/// depth of the audio source. For example, 16 bit quantized samples can be
/// encoded in `i16` or `u16`. Note that the quantized sampling depth is not
/// directly visible for formats where [`is_float`] is true.
///
/// Also note that the backend must support the encoding of the quantized
/// samples in the given format, as there is no generic transformation from one
/// format into the other done inside the frontend-library code. You can query
/// the supported formats by using [`supported_input_configs`].
///
/// A good rule of thumb is to use [`SampleFormat::I16`] as this covers typical
/// music (WAV, MP3) as well as typical audio input devices on most platforms,
///
/// [`is_float`]: SampleFormat::is_float
/// [`supported_input_configs`]: crate::traits::DeviceTrait::supported_input_configs
#[cfg_attr(
    all(
        target_arch = "wasm32",
        any(target_os = "emscripten", feature = "wasm-bindgen")
    ),
    wasm_bindgen
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum SampleFormat {
    /// `i8` with a valid range of `i8::MIN..=i8::MAX` with `0` being the origin.
    I8,

    /// `i16` with a valid range of `i16::MIN..=i16::MAX` with `0` being the origin.
    I16,

    /// `I24` with a valid range of `-(1 << 23)..=((1 << 23) - 1)` with `0` being the origin.
    ///
    /// This format uses 4 bytes of storage but only 24 bits are significant.
    I24,

    /// `i32` with a valid range of `i32::MIN..=i32::MAX` with `0` being the origin.
    I32,

    // /// `I48` with a valid range of '-(1 << 47)..(1 << 47)' with `0` being the origin
    // I48,
    /// `i64` with a valid range of `i64::MIN..=i64::MAX` with `0` being the origin.
    I64,

    /// `u8` with a valid range of `u8::MIN..=u8::MAX` with `1 << 7 == 128` being the origin.
    U8,

    /// `u16` with a valid range of `u16::MIN..=u16::MAX` with `1 << 15 == 32768` being the origin.
    U16,

    /// `U24` with a valid range of `0..=((1 << 24) - 1)` with `1 << 23 == 8388608` being the origin.
    ///
    /// This format uses 4 bytes of storage but only 24 bits are significant.
    U24,

    /// `u32` with a valid range of `u32::MIN..=u32::MAX` with `1 << 31` being the origin.
    U32,

    /// `U48` with a valid range of '0..(1 << 48)' with `1 << 47` being the origin
    // U48,

    /// `u64` with a valid range of `u64::MIN..=u64::MAX` with `1 << 63` being the origin.
    U64,

    /// `f32` with a valid range of `-1.0..=1.0` with `0.0` being the origin.
    F32,

    /// `f64` with a valid range of `-1.0..=1.0` with `0.0` being the origin.
    F64,
}

impl SampleFormat {
    /// Returns the size in bytes of a sample of this format. This corresponds to
    /// the internal size of the rust primitives that are used to represent this
    /// sample format (e.g., i24 has size of i32).
    #[inline]
    #[must_use]
    pub fn sample_size(&self) -> usize {
        match *self {
            SampleFormat::I8 => mem::size_of::<i8>(),
            SampleFormat::U8 => mem::size_of::<u8>(),
            SampleFormat::I16 => mem::size_of::<i16>(),
            SampleFormat::U16 => mem::size_of::<u16>(),
            SampleFormat::I24 => mem::size_of::<i32>(),
            SampleFormat::U24 => mem::size_of::<i32>(),
            SampleFormat::I32 => mem::size_of::<i32>(),
            SampleFormat::U32 => mem::size_of::<u32>(),
            // SampleFormat::I48 => mem::size_of::<i64>(),
            // SampleFormat::U48 => mem::size_of::<i64>(),
            SampleFormat::I64 => mem::size_of::<i64>(),
            SampleFormat::U64 => mem::size_of::<u64>(),
            SampleFormat::F32 => mem::size_of::<f32>(),
            SampleFormat::F64 => mem::size_of::<f64>(),
        }
    }

    /// Returns the number of bits of a sample of this format. Note that this is
    /// not necessarily the same as the size of the primitive used to represent
    /// this sample format (e.g., I24 has size of i32 but 24 bits per sample).
    #[inline]
    #[must_use]
    pub fn bits_per_sample(&self) -> u32 {
        match *self {
            SampleFormat::I8 => i8::BITS,
            SampleFormat::U8 => u8::BITS,
            SampleFormat::I16 => i16::BITS,
            SampleFormat::U16 => u16::BITS,
            SampleFormat::I24 => 24,
            SampleFormat::U24 => 24,
            SampleFormat::I32 => i32::BITS,
            SampleFormat::U32 => u32::BITS,
            // SampleFormat::I48 => 48,
            // SampleFormat::U48 => 48,
            SampleFormat::I64 => i64::BITS,
            SampleFormat::U64 => u64::BITS,
            SampleFormat::F32 => 32,
            SampleFormat::F64 => 64,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_int(&self) -> bool {
        matches!(
            *self,
            SampleFormat::I8
                | SampleFormat::I16
                | SampleFormat::I24
                | SampleFormat::I32
                // | SampleFormat::I48
                | SampleFormat::I64
        )
    }

    #[inline]
    #[must_use]
    pub fn is_uint(&self) -> bool {
        matches!(
            *self,
            SampleFormat::U8
                | SampleFormat::U16
                | SampleFormat::U24
                | SampleFormat::U32
                // | SampleFormat::U48
                | SampleFormat::U64
        )
    }

    #[inline]
    #[must_use]
    pub fn is_float(&self) -> bool {
        matches!(*self, SampleFormat::F32 | SampleFormat::F64)
    }
}

impl Display for SampleFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SampleFormat::I8 => "i8",
            SampleFormat::I16 => "i16",
            SampleFormat::I24 => "i24",
            SampleFormat::I32 => "i32",
            // SampleFormat::I48 => "i48",
            SampleFormat::I64 => "i64",
            SampleFormat::U8 => "u8",
            SampleFormat::U16 => "u16",
            SampleFormat::U24 => "u24",
            SampleFormat::U32 => "u32",
            // SampleFormat::U48 => "u48",
            SampleFormat::U64 => "u64",
            SampleFormat::F32 => "f32",
            SampleFormat::F64 => "f64",
        }
        .fmt(f)
    }
}

/// A [`Sample`] type with a known corresponding [`SampleFormat`].
///
/// This trait is automatically implemented for all primitive sample types and provides
/// a way to determine the [`SampleFormat`] at compile time.
///
/// # Example
///
/// ```
/// use cpal::SizedSample;
///
/// assert_eq!(i16::FORMAT, cpal::SampleFormat::I16);
/// assert_eq!(f32::FORMAT, cpal::SampleFormat::F32);
/// ```
pub trait SizedSample: Sample {
    /// The corresponding [`SampleFormat`] for this sample type.
    const FORMAT: SampleFormat;
}

impl SizedSample for i8 {
    const FORMAT: SampleFormat = SampleFormat::I8;
}

impl SizedSample for i16 {
    const FORMAT: SampleFormat = SampleFormat::I16;
}

impl SizedSample for I24 {
    const FORMAT: SampleFormat = SampleFormat::I24;
}

impl SizedSample for i32 {
    const FORMAT: SampleFormat = SampleFormat::I32;
}

// impl SizedSample for I48 {
//     const FORMAT: SampleFormat = SampleFormat::I48;
// }

impl SizedSample for i64 {
    const FORMAT: SampleFormat = SampleFormat::I64;
}

impl SizedSample for u8 {
    const FORMAT: SampleFormat = SampleFormat::U8;
}

impl SizedSample for u16 {
    const FORMAT: SampleFormat = SampleFormat::U16;
}

impl SizedSample for U24 {
    const FORMAT: SampleFormat = SampleFormat::U24;
}

impl SizedSample for u32 {
    const FORMAT: SampleFormat = SampleFormat::U32;
}

// impl SizedSample for U48 {
//     const FORMAT: SampleFormat = SampleFormat::U48;
// }

impl SizedSample for u64 {
    const FORMAT: SampleFormat = SampleFormat::U64;
}

impl SizedSample for f32 {
    const FORMAT: SampleFormat = SampleFormat::F32;
}

impl SizedSample for f64 {
    const FORMAT: SampleFormat = SampleFormat::F64;
}

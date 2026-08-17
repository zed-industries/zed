//! Extract pixel samples from a block of pixel bytes.

use crate::prelude::*;
use half::prelude::HalfFloatSliceExt;


/// A single red, green, blue, or alpha value.
#[derive(Copy, Clone, Debug)]
pub enum Sample {

    /// A 16-bit float sample.
    F16(f16),

    /// A 32-bit float sample.
    F32(f32),

    /// An unsigned integer sample.
    U32(u32)
}

impl Sample {

    /// Create a sample containing a 32-bit float.
    pub fn f32(f32: f32) -> Self { Sample::F32(f32) }

    /// Create a sample containing a 16-bit float.
    pub fn f16(f16: f16) -> Self { Sample::F16(f16) }

    /// Create a sample containing a 32-bit integer.
    pub fn u32(u32: u32) -> Self { Sample::U32(u32) }

    /// Convert the sample to an f16 value. This has lower precision than f32.
    /// Note: An f32 can only represent integers up to `1024` as precise as a u32 could.
    #[inline]
    pub fn to_f16(self) -> f16 {
        match self {
            Sample::F16(sample) => sample,
            Sample::F32(sample) => f16::from_f32(sample),
            Sample::U32(sample) => f16::from_f32(sample as f32),
        }
    }

    /// Convert the sample to an f32 value.
    /// Note: An f32 can only represent integers up to `8388608` as precise as a u32 could.
    #[inline]
    pub fn to_f32(self) -> f32 {
        match self {
            Sample::F32(sample) => sample,
            Sample::F16(sample) => sample.to_f32(),
            Sample::U32(sample) => sample as f32,
        }
    }

    /// Convert the sample to a u32. Rounds floats to integers the same way that `3.1 as u32` does.
    #[inline]
    pub fn to_u32(self) -> u32 {
        match self {
            Sample::F16(sample) => sample.to_f32() as u32,
            Sample::F32(sample) => sample as u32,
            Sample::U32(sample) => sample,
        }
    }

    /// Is this value not a number?
    #[inline]
    pub fn is_nan(self) -> bool {
        match self {
            Sample::F16(value) => value.is_nan(),
            Sample::F32(value) => value.is_nan(),
            Sample::U32(_) => false,
        }
    }

    /// Is this value zero or negative zero?
    #[inline]
    pub fn is_zero(&self) -> bool {
        match *self {
            Sample::F16(value) => value == f16::ZERO || value == f16::NEG_ZERO,
            Sample::F32(value) => value == 0.0,
            Sample::U32(value) => value == 0,
        }
    }
}

impl PartialEq for Sample {
    fn eq(&self, other: &Self) -> bool {
        match *self {
            Sample::F16(num) => num == other.to_f16(),
            Sample::F32(num) => num == other.to_f32(),
            Sample::U32(num) => num == other.to_u32(),
        }
    }
}

// this is not recommended because it may hide whether a color is transparent or opaque and might be undesired for depth channels
impl Default for Sample {
    fn default() -> Self { Sample::F32(0.0) }
}

impl From<f16> for Sample { #[inline] fn from(f: f16) -> Self { Sample::F16(f) } }
impl From<f32> for Sample { #[inline] fn from(f: f32) -> Self { Sample::F32(f) } }
impl From<u32> for Sample { #[inline] fn from(f: u32) -> Self { Sample::U32(f) } }

impl<T> From<Option<T>> for Sample where T: Into<Sample> + Default {
    #[inline] fn from(num: Option<T>) -> Self { num.unwrap_or_default().into() }
}


impl From<Sample> for f16 { #[inline] fn from(s: Sample) -> Self { s.to_f16() } }
impl From<Sample> for f32 { #[inline] fn from(s: Sample) -> Self { s.to_f32() } }
impl From<Sample> for u32 { #[inline] fn from(s: Sample) -> Self { s.to_u32() } }


/// Create an arbitrary sample type from one of the defined sample types.
/// Should be compiled to a no-op where the file contains the predicted sample type.
/// The slice functions should be optimized into a `memcpy` where there is no conversion needed.
pub trait FromNativeSample: Sized + Copy + Default + 'static {

    /// Create this sample from a f16, trying to represent the same numerical value
    fn from_f16(value: f16) -> Self;

    /// Create this sample from a f32, trying to represent the same numerical value
    fn from_f32(value: f32) -> Self;

    /// Create this sample from a u32, trying to represent the same numerical value
    fn from_u32(value: u32) -> Self;

    /// Convert all values from the slice into this type.
    /// This function exists to allow the compiler to perform a vectorization optimization.
    /// Note that this default implementation will **not** be vectorized by the compiler automatically.
    /// For maximum performance you will need to override this function and implement it via
    /// an explicit batched conversion such as [`convert_to_f32_slice`](https://docs.rs/half/2.3.1/half/slice/trait.HalfFloatSliceExt.html#tymethod.convert_to_f32_slice)
    #[inline]
    fn from_f16s(from: &[f16], to: &mut [Self]) {
        assert_eq!(from.len(), to.len(), "slices must have the same length");
        for (from, to) in from.iter().zip(to.iter_mut()) {
            *to = Self::from_f16(*from);
        }
    }

    /// Convert all values from the slice into this type.
    /// This function exists to allow the compiler to perform a vectorization optimization.
    /// Note that this default implementation will be vectorized by the compiler automatically.
    #[inline]
    fn from_f32s(from: &[f32], to: &mut [Self]) {
        assert_eq!(from.len(), to.len(), "slices must have the same length");
        for (from, to) in from.iter().zip(to.iter_mut()) {
            *to = Self::from_f32(*from);
        }
    }

    /// Convert all values from the slice into this type.
    /// This function exists to allow the compiler to perform a vectorization optimization.
    /// Note that this default implementation will be vectorized by the compiler automatically,
    /// provided that the CPU supports the necessary conversion instructions.
    /// For example, x86_64 lacks the instructions to convert `u32` to floats,
    /// so this will inevitably be slow on x86_64.
    #[inline]
    fn from_u32s(from: &[u32], to: &mut [Self]) {
        assert_eq!(from.len(), to.len(), "slices must have the same length");
        for (from, to) in from.iter().zip(to.iter_mut()) {
            *to = Self::from_u32(*from);
        }
    }
}

// TODO haven't i implemented this exact behaviour already somewhere else in this library...??
impl FromNativeSample for f32 {
    #[inline] fn from_f16(value: f16) -> Self { value.to_f32() }
    #[inline] fn from_f32(value: f32) -> Self { value }
    #[inline] fn from_u32(value: u32) -> Self { value as f32 }

    // f16 is a custom type
    // so the compiler can not automatically vectorize the conversion
    // that's why we need to specialize this function
    #[inline]
    fn from_f16s(from: &[f16], to: &mut [Self]) {
        from.convert_to_f32_slice(to);
    }
}

impl FromNativeSample for u32 {
    #[inline] fn from_f16(value: f16) -> Self { value.to_f32() as u32 }
    #[inline] fn from_f32(value: f32) -> Self { value as u32 }
    #[inline] fn from_u32(value: u32) -> Self { value }
}

impl FromNativeSample for f16 {
    #[inline] fn from_f16(value: f16) -> Self { value }
    #[inline] fn from_f32(value: f32) -> Self { f16::from_f32(value) }
    #[inline] fn from_u32(value: u32) -> Self { f16::from_f32(value as f32) }

    // f16 is a custom type
    // so the compiler can not automatically vectorize the conversion
    // that's why we need to specialize this function
    #[inline]
    fn from_f32s(from: &[f32], to: &mut [Self]) {
        to.convert_from_f32_slice(from)
    }
}

impl FromNativeSample for Sample {
    #[inline] fn from_f16(value: f16) -> Self { Self::from(value) }
    #[inline] fn from_f32(value: f32) -> Self { Self::from(value) }
    #[inline] fn from_u32(value: u32) -> Self { Self::from(value) }
}


/// Convert any type into one of the supported sample types.
/// Should be compiled to a no-op where the file contains the predicted sample type
pub trait IntoNativeSample: Copy + Default + Sync + 'static {

    /// Convert this sample to an f16, trying to represent the same numerical value.
    fn to_f16(&self) -> f16;

    /// Convert this sample to an f32, trying to represent the same numerical value.
    fn to_f32(&self) -> f32;

    /// Convert this sample to an u16, trying to represent the same numerical value.
    fn to_u32(&self) -> u32;
}

impl IntoNativeSample for f16 {
    fn to_f16(&self) -> f16 { f16::from_f16(*self) }
    fn to_f32(&self) -> f32 { f32::from_f16(*self) }
    fn to_u32(&self) -> u32 { u32::from_f16(*self) }
}

impl IntoNativeSample for f32 {
    fn to_f16(&self) -> f16 { f16::from_f32(*self) }
    fn to_f32(&self) -> f32 { f32::from_f32(*self) }
    fn to_u32(&self) -> u32 { u32::from_f32(*self) }
}

impl IntoNativeSample for u32 {
    fn to_f16(&self) -> f16 { f16::from_u32(*self) }
    fn to_f32(&self) -> f32 { f32::from_u32(*self) }
    fn to_u32(&self) -> u32 { u32::from_u32(*self) }
}

impl IntoNativeSample for Sample {
    fn to_f16(&self) -> f16 { Sample::to_f16(*self) }
    fn to_f32(&self) -> f32 { Sample::to_f32(*self) }
    fn to_u32(&self) -> u32 { Sample::to_u32(*self) }
}




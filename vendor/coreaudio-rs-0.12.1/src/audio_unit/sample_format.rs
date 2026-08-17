use super::audio_format::{self, LinearPcmFlags};

/// Dynamic representation of audio data sample format.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    /// 32-bit float.
    F32,
    /// 32-bit signed integer.
    I32,
    /// 24-bit signed integer. Can be packed or not depending on the flags.
    I24,
    /// 16-bit signed integer.
    I16,
    /// 8-bit signed integer.
    I8,
}

impl SampleFormat {
    /// Check if the format flags are appropriate for the given format.
    pub fn does_match_flags(&self, flags: audio_format::LinearPcmFlags) -> bool {
        let is_float = flags.contains(LinearPcmFlags::IS_FLOAT);
        let is_signed_integer = flags.contains(LinearPcmFlags::IS_SIGNED_INTEGER);
        let is_packed = flags.contains(LinearPcmFlags::IS_PACKED);
        match *self {
            SampleFormat::F32 => is_float && !is_signed_integer && is_packed,
            SampleFormat::I32 | SampleFormat::I16 | SampleFormat::I8 => {
                is_signed_integer && !is_float && is_packed
            }
            SampleFormat::I24 => is_signed_integer && !is_float,
        }
    }

    /// Convert format flags and bits_per_sample to a SampleFormat.
    pub fn from_flags_and_bits_per_sample(
        flags: audio_format::LinearPcmFlags,
        bits_per_sample: u32,
    ) -> Option<Self> {
        let packed = flags.contains(LinearPcmFlags::IS_PACKED);
        let sample_format = if flags.contains(LinearPcmFlags::IS_FLOAT) {
            match (bits_per_sample, packed) {
                (32, true) => SampleFormat::F32,
                _ => return None,
            }
        } else if flags.contains(LinearPcmFlags::IS_SIGNED_INTEGER) {
            match (bits_per_sample, packed) {
                (8, true) => SampleFormat::I8,
                (16, true) => SampleFormat::I16,
                (24, _) => SampleFormat::I24,
                (32, true) => SampleFormat::I32,
                _ => return None,
            }
        } else {
            // TODO: Check whether or not we need to consider other formats, like unsigned ints.
            return None;
        };
        Some(sample_format)
    }

    /// Return the size of one sample in bytes, assuming that the format is packed.
    pub fn size_in_bytes(&self) -> usize {
        use std::mem::size_of;
        match *self {
            SampleFormat::F32 => size_of::<f32>(),
            SampleFormat::I32 => size_of::<i32>(),
            SampleFormat::I24 => 3 * size_of::<u8>(),
            SampleFormat::I16 => size_of::<i16>(),
            SampleFormat::I8 => size_of::<i8>(),
        }
    }

    /// Return the number of valid bits for one sample.
    pub fn size_in_bits(&self) -> u32 {
        match *self {
            SampleFormat::F32 => 32,
            SampleFormat::I32 => 32,
            SampleFormat::I24 => 24,
            SampleFormat::I16 => 16,
            SampleFormat::I8 => 8,
        }
    }
}

/// Audio data sample types.
pub trait Sample {
    /// Dynamic representation of audio data sample format.
    fn sample_format() -> SampleFormat;
}

/// Simplified implementation of the `Sample` trait for sample types.
/// This is only implemented for the sample types that map directly to a numeric type.
macro_rules! impl_sample {
    ($($T:ident $format:ident),* $(,)*) => {
        $(
            impl Sample for $T {
                fn sample_format() -> SampleFormat {
                    SampleFormat::$format
                }
            }
        )*
    }
}

impl_sample!(f32 F32, i32 I32, i16 I16, i8 I8);

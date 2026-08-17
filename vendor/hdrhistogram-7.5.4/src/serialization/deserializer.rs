use super::{V2_COMPRESSED_COOKIE, V2_COOKIE};
use crate::{Counter, Histogram, RestatState};
use byteorder::{BigEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use num_traits::ToPrimitive;
use std::io::{self, Cursor, Read};
use std::marker::PhantomData;
use std::{self, error, fmt};

/// Errors that can happen during deserialization.
#[derive(Debug)]
pub enum DeserializeError {
    /// An i/o operation failed.
    IoError(io::Error),
    /// The cookie (first 4 bytes) did not match that for any supported format.
    InvalidCookie,
    /// The histogram uses features that this implementation doesn't support (yet), so it cannot
    /// be deserialized correctly.
    UnsupportedFeature,
    /// A count exceeded what can be represented in the chosen counter type.
    UnsuitableCounterType,
    /// The histogram instance could not be created because the serialized parameters were invalid
    /// (e.g. lowest value, highest value, etc.)
    InvalidParameters,
    /// The current system's pointer width cannot represent the encoded histogram.
    UsizeTypeTooSmall,
    /// The encoded array is longer than it should be for the histogram's value range.
    EncodedArrayTooLong,
}

impl std::convert::From<std::io::Error> for DeserializeError {
    fn from(e: std::io::Error) -> Self {
        DeserializeError::IoError(e)
    }
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeserializeError::IoError(e) => write!(f, "An i/o operation failed: {}", e),
            DeserializeError::InvalidCookie => write!(
                f,
                "The cookie (first 4 bytes) did not match that for any supported format"
            ),
            DeserializeError::UnsupportedFeature => write!(
                f,
                "The histogram uses features that this implementation doesn't support"
            ),
            DeserializeError::UnsuitableCounterType => write!(
                f,
                "A count exceeded what can be represented in the chosen counter type"
            ),
            DeserializeError::InvalidParameters => write!(
                f,
                "The serialized parameters were invalid(e.g. lowest value, highest value, etc)"
            ),
            DeserializeError::UsizeTypeTooSmall => write!(
                f,
                "The current system's pointer width cannot represent the encoded histogram"
            ),
            DeserializeError::EncodedArrayTooLong => write!(
                f,
                "The encoded array is longer than it should be for the histogram's value range"
            ),
        }
    }
}

impl error::Error for DeserializeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DeserializeError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

/// Deserializer for all supported formats.
///
/// Since the serialization formats all include some magic bytes that allow reliable identification
/// of the different formats, only one Deserializer implementation is needed.
pub struct Deserializer {
    payload_buf: Vec<u8>,
}

impl Default for Deserializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Deserializer {
    /// Create a new deserializer.
    pub fn new() -> Deserializer {
        Deserializer {
            payload_buf: Vec::new(),
        }
    }

    /// Deserialize an encoded histogram from the provided reader.
    ///
    /// Note that `&[u8]` and `Cursor` are convenient implementations of `Read` if you have some
    /// bytes already in slice or `Vec` form.
    pub fn deserialize<T: Counter, R: Read>(
        &mut self,
        reader: &mut R,
    ) -> Result<Histogram<T>, DeserializeError> {
        let cookie = reader.read_u32::<BigEndian>()?;

        match cookie {
            V2_COOKIE => self.deser_v2(reader),
            V2_COMPRESSED_COOKIE => self.deser_v2_compressed(reader),
            _ => Err(DeserializeError::InvalidCookie),
        }
    }

    fn deser_v2_compressed<T: Counter, R: Read>(
        &mut self,
        reader: &mut R,
    ) -> Result<Histogram<T>, DeserializeError> {
        let payload_len = reader
            .read_u32::<BigEndian>()?
            .to_usize()
            .ok_or(DeserializeError::UsizeTypeTooSmall)?;

        // TODO reuse deflate buf, or switch to lower-level flate2::Decompress
        let mut deflate_reader = ZlibDecoder::new(reader.take(payload_len as u64));
        let inner_cookie = deflate_reader.read_u32::<BigEndian>()?;
        if inner_cookie != V2_COOKIE {
            return Err(DeserializeError::InvalidCookie);
        }

        self.deser_v2(&mut deflate_reader)
    }

    #[allow(clippy::float_cmp)]
    fn deser_v2<T: Counter, R: Read>(
        &mut self,
        reader: &mut R,
    ) -> Result<Histogram<T>, DeserializeError> {
        let payload_len = reader
            .read_u32::<BigEndian>()?
            .to_usize()
            .ok_or(DeserializeError::UsizeTypeTooSmall)?;
        let normalizing_offset = reader.read_u32::<BigEndian>()?;
        if normalizing_offset != 0 {
            return Err(DeserializeError::UnsupportedFeature);
        }
        let num_digits = reader
            .read_u32::<BigEndian>()?
            .to_u8()
            .ok_or(DeserializeError::InvalidParameters)?;
        let low = reader.read_u64::<BigEndian>()?;
        let high = reader.read_u64::<BigEndian>()?;
        let int_double_ratio = reader.read_f64::<BigEndian>()?;
        if int_double_ratio != 1.0 {
            return Err(DeserializeError::UnsupportedFeature);
        }

        let mut h = Histogram::new_with_bounds(low, high, num_digits)
            .map_err(|_| DeserializeError::InvalidParameters)?;

        if payload_len > self.payload_buf.len() {
            self.payload_buf.resize(payload_len, 0);
        }

        let mut payload_slice = &mut self.payload_buf[0..payload_len];
        reader.read_exact(&mut payload_slice)?;

        let mut payload_index: usize = 0;
        let mut restat_state = RestatState::new();
        let mut decode_state = DecodeLoopState::new();

        while payload_index < payload_len.saturating_sub(9) {
            // Read with fast loop until we are within 9 of the end. Fast loop can't handle EOF,
            // so bail to slow version for the last few bytes.

            // payload_index math is safe because payload_len is a usize
            let (zz_num, bytes_read) =
                varint_read_slice(&payload_slice[payload_index..(payload_index + 9)]);
            payload_index += bytes_read;

            let count_or_zeros = zig_zag_decode(zz_num);

            decode_state.on_decoded_num(count_or_zeros, &mut restat_state, &mut h)?;
        }

        // Now read the leftovers
        let leftover_slice = &payload_slice[payload_index..];
        let mut cursor = Cursor::new(&leftover_slice);
        while cursor.position() < leftover_slice.len() as u64 {
            let count_or_zeros = zig_zag_decode(varint_read(&mut cursor)?);

            decode_state.on_decoded_num(count_or_zeros, &mut restat_state, &mut h)?;
        }

        restat_state.update_histogram(&mut h);

        Ok(h)
    }
}

// Only public for testing.
/// Read from a slice that must be 9 bytes long or longer. Returns the decoded number and how many
/// bytes were consumed.
#[inline]
pub fn varint_read_slice(slice: &[u8]) -> (u64, usize) {
    let mut b = slice[0];

    // take low 7 bits
    let mut value: u64 = low_7_bits(b);
    if !is_high_bit_set(b) {
        return (value, 1);
    }
    // high bit set, keep reading
    b = slice[1];
    value |= low_7_bits(b) << 7;
    if !is_high_bit_set(b) {
        return (value, 2);
    }
    b = slice[2];
    value |= low_7_bits(b) << (7 * 2);
    if !is_high_bit_set(b) {
        return (value, 3);
    }
    b = slice[3];
    value |= low_7_bits(b) << (7 * 3);
    if !is_high_bit_set(b) {
        return (value, 4);
    }
    b = slice[4];
    value |= low_7_bits(b) << (7 * 4);
    if !is_high_bit_set(b) {
        return (value, 5);
    }
    b = slice[5];
    value |= low_7_bits(b) << (7 * 5);
    if !is_high_bit_set(b) {
        return (value, 6);
    }
    b = slice[6];
    value |= low_7_bits(b) << (7 * 6);
    if !is_high_bit_set(b) {
        return (value, 7);
    }
    b = slice[7];
    value |= low_7_bits(b) << (7 * 7);
    if !is_high_bit_set(b) {
        return (value, 8);
    }

    b = slice[8];
    // special case: use last byte as is
    value |= u64::from(b) << (7 * 8);

    (value, 9)
}

// Only public for testing.
/// Read a LEB128-64b9B from the buffer
pub fn varint_read<R: Read>(reader: &mut R) -> io::Result<u64> {
    let mut b = reader.read_u8()?;

    // take low 7 bits
    let mut value: u64 = low_7_bits(b);

    if is_high_bit_set(b) {
        // high bit set, keep reading
        b = reader.read_u8()?;
        value |= low_7_bits(b) << 7;
        if is_high_bit_set(b) {
            b = reader.read_u8()?;
            value |= low_7_bits(b) << (7 * 2);
            if is_high_bit_set(b) {
                b = reader.read_u8()?;
                value |= low_7_bits(b) << (7 * 3);
                if is_high_bit_set(b) {
                    b = reader.read_u8()?;
                    value |= low_7_bits(b) << (7 * 4);
                    if is_high_bit_set(b) {
                        b = reader.read_u8()?;
                        value |= low_7_bits(b) << (7 * 5);
                        if is_high_bit_set(b) {
                            b = reader.read_u8()?;
                            value |= low_7_bits(b) << (7 * 6);
                            if is_high_bit_set(b) {
                                b = reader.read_u8()?;
                                value |= low_7_bits(b) << (7 * 7);
                                if is_high_bit_set(b) {
                                    b = reader.read_u8()?;
                                    // special case: use last byte as is
                                    value |= u64::from(b) << (7 * 8);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(value)
}

/// truncate byte to low 7 bits, cast to u64
#[inline]
fn low_7_bits(b: u8) -> u64 {
    u64::from(b & 0x7F)
}

#[inline]
fn is_high_bit_set(b: u8) -> bool {
    (b & 0x80) != 0
}

// Only public for testing.
#[inline]
pub fn zig_zag_decode(encoded: u64) -> i64 {
    ((encoded >> 1) as i64) ^ -((encoded & 1) as i64)
}

/// We need to perform the same logic in two different decode loops while carrying over a modicum
/// of state.
struct DecodeLoopState<T: Counter> {
    dest_index: usize,
    phantom: PhantomData<T>,
}

impl<T: Counter> DecodeLoopState<T> {
    fn new() -> DecodeLoopState<T> {
        DecodeLoopState {
            dest_index: 0,
            phantom: PhantomData,
        }
    }

    #[inline]
    fn on_decoded_num(
        &mut self,
        count_or_zeros: i64,
        restat_state: &mut RestatState<T>,
        h: &mut Histogram<T>,
    ) -> Result<(), DeserializeError> {
        if count_or_zeros < 0 {
            // For a valid histogram, negation won't overflow because you can't have anywhere close
            // to even 2^32 array length
            let zero_count = (-count_or_zeros)
                .to_usize()
                .ok_or(DeserializeError::UsizeTypeTooSmall)?;
            // skip the zeros
            self.dest_index = self
                .dest_index
                .checked_add(zero_count)
                .ok_or(DeserializeError::UsizeTypeTooSmall)?;
        } else {
            let count: T =
                T::from_i64(count_or_zeros).ok_or(DeserializeError::UnsuitableCounterType)?;

            if count > T::zero() {
                h.set_count_at_index(self.dest_index, count)
                    .map_err(|_| DeserializeError::EncodedArrayTooLong)?;

                restat_state.on_nonzero_count(self.dest_index, count);
            }

            self.dest_index = self
                .dest_index
                .checked_add(1)
                .ok_or(DeserializeError::UsizeTypeTooSmall)?;
        }

        Ok(())
    }
}

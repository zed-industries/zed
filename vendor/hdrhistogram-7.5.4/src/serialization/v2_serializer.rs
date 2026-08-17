use super::{Serializer, V2_COOKIE, V2_HEADER_SIZE};
use crate::{Counter, Histogram};
use byteorder::{BigEndian, WriteBytesExt};
use std::io::{self, Write};
use std::{error, fmt};

/// Errors that occur during serialization.
#[derive(Debug)]
pub enum V2SerializeError {
    /// A count above i64::max_value() cannot be zig-zag encoded, and therefore cannot be
    /// serialized.
    CountNotSerializable,
    /// Internal calculations cannot be represented in `usize`. Use smaller histograms or beefier
    /// hardware.
    UsizeTypeTooSmall,
    /// An i/o operation failed.
    IoError(io::Error),
}

impl std::convert::From<std::io::Error> for V2SerializeError {
    fn from(e: std::io::Error) -> Self {
        V2SerializeError::IoError(e)
    }
}

impl fmt::Display for V2SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            V2SerializeError::CountNotSerializable => write!(
                f,
                "A count above i64::max_value() cannot be zig-zag encoded"
            ),
            V2SerializeError::UsizeTypeTooSmall => {
                write!(f, "Internal calculations cannot be represented in `usize`")
            }
            V2SerializeError::IoError(e) => write!(f, "An i/o operation failed: {}", e),
        }
    }
}

impl error::Error for V2SerializeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            V2SerializeError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

/// Serializer for the V2 binary format.
pub struct V2Serializer {
    buf: Vec<u8>,
}

impl Default for V2Serializer {
    fn default() -> Self {
        Self::new()
    }
}
impl V2Serializer {
    /// Create a new serializer.
    pub fn new() -> V2Serializer {
        V2Serializer { buf: Vec::new() }
    }
}

impl Serializer for V2Serializer {
    type SerializeError = V2SerializeError;

    fn serialize<T: Counter, W: Write>(
        &mut self,
        h: &Histogram<T>,
        writer: &mut W,
    ) -> Result<usize, V2SerializeError> {
        // TODO benchmark encoding directly into target Vec

        self.buf.clear();
        let max_size = max_encoded_size(h).ok_or(V2SerializeError::UsizeTypeTooSmall)?;
        self.buf.reserve(max_size);

        self.buf.write_u32::<BigEndian>(V2_COOKIE)?;
        // placeholder for length
        self.buf.write_u32::<BigEndian>(0)?;
        // normalizing index offset
        self.buf.write_u32::<BigEndian>(0)?;
        self.buf
            .write_u32::<BigEndian>(u32::from(h.significant_value_digits))?;
        self.buf
            .write_u64::<BigEndian>(h.lowest_discernible_value)?;
        self.buf.write_u64::<BigEndian>(h.highest_trackable_value)?;
        // int to double conversion
        self.buf.write_f64::<BigEndian>(1.0)?;

        debug_assert_eq!(V2_HEADER_SIZE, self.buf.len());

        unsafe {
            // want to treat the rest of the vec as a slice, and we've already reserved this
            // space, so this way we don't have to resize() on a lot of dummy bytes.
            self.buf.set_len(max_size);
        }

        let counts_len = encode_counts(h, &mut self.buf[V2_HEADER_SIZE..])?;
        // addition should be safe as max_size is already a usize
        let total_len = V2_HEADER_SIZE + counts_len;

        // TODO benchmark fastest buffer management scheme
        // counts is always under 2^24
        (&mut self.buf[4..8]).write_u32::<BigEndian>(counts_len as u32)?;

        writer
            .write_all(&self.buf[0..(total_len)])
            .map(|_| total_len)
            .map_err(V2SerializeError::IoError)
    }
}

fn max_encoded_size<T: Counter>(h: &Histogram<T>) -> Option<usize> {
    h.index_for(h.max())
        .and_then(|i| counts_array_max_encoded_size(i + 1))
        .and_then(|x| x.checked_add(V2_HEADER_SIZE))
}

// Only public for testing.
pub fn counts_array_max_encoded_size(length: usize) -> Option<usize> {
    // LEB128-64b9B uses at most 9 bytes
    // Won't overflow (except sometimes on 16 bit systems) because largest possible counts
    // len is 47 buckets, each with 2^17 half count, for a total of 6e6. This product will
    // therefore be about 5e7 (50 million) at most.
    length.checked_mul(9)
}

// Only public for testing.
/// Encode counts array into slice.
/// The slice must be at least 9 * the number of counts that will be encoded.
pub fn encode_counts<T: Counter>(
    h: &Histogram<T>,
    buf: &mut [u8],
) -> Result<usize, V2SerializeError> {
    let index_limit = h
        .index_for(h.max())
        .expect("Index for max value must exist");
    let mut index = 0;
    let mut bytes_written = 0;

    assert!(index_limit <= h.counts.len());

    while index <= index_limit {
        // index is inside h.counts because of the assert above
        let count = unsafe { *(h.counts.get_unchecked(index)) };
        index += 1;

        // Non-negative values are counts for the respective value, negative values are skipping
        // that many (absolute value) zero-count values.

        let mut zero_count = 0;
        if count == T::zero() {
            zero_count = 1;

            // index is inside h.counts because of the assert above
            while (index <= index_limit)
                && (unsafe { *(h.counts.get_unchecked(index)) } == T::zero())
            {
                zero_count += 1;
                index += 1;
            }
        }

        let count_or_zeros: i64 = if zero_count > 1 {
            // zero count can be at most the entire counts array, which is at most 2^24, so will
            // fit.
            -zero_count
        } else {
            // TODO while writing tests that serialize random counts, this was annoying.
            // Don't want to silently cap them at i64::max_value() for users that, say, aren't
            // serializing. Don't want to silently eat counts beyond i63 max when serializing.
            // Perhaps we should provide some sort of pluggability here -- choose whether you want
            // to truncate counts to i63 max, or report errors if you need maximum fidelity?
            count
                .to_i64()
                .ok_or(V2SerializeError::CountNotSerializable)?
        };

        let zz = zig_zag_encode(count_or_zeros);

        // this can't be longer than the length of `buf`, so this won't overflow `usize`
        bytes_written += varint_write(zz, &mut buf[bytes_written..]);
    }

    Ok(bytes_written)
}

// Only public for testing.
/// Write a number as a LEB128-64b9B little endian base 128 varint to buf. This is not
/// quite the same as Protobuf's LEB128 as it encodes 64 bit values in a max of 9 bytes, not 10.
/// The first 8 7-bit chunks are encoded normally (up through the first 7 bytes of input). The last
/// byte is added to the buf as-is. This limits the input to 8 bytes, but that's all we need.
/// Returns the number of bytes written (in [1, 9]).
#[inline]
pub fn varint_write(input: u64, buf: &mut [u8]) -> usize {
    // The loop is unrolled because the special case is awkward to express in a loop, and it
    // probably makes the branch predictor happier to do it this way.
    // This way about twice as fast as the other "obvious" approach: a sequence of `if`s to detect
    // size directly with each branch encoding that number completely and returning.

    if shift_by_7s(input, 1) == 0 {
        buf[0] = input as u8;
        return 1;
    }
    // set high bit because more bytes are coming, then next 7 bits of value.
    buf[0] = 0x80 | ((input & 0x7F) as u8);
    if shift_by_7s(input, 2) == 0 {
        // All zero above bottom 2 chunks, this is the last byte, so no high bit
        buf[1] = shift_by_7s(input, 1) as u8;
        return 2;
    }
    buf[1] = nth_7b_chunk_with_high_bit(input, 1);
    if shift_by_7s(input, 3) == 0 {
        buf[2] = shift_by_7s(input, 2) as u8;
        return 3;
    }
    buf[2] = nth_7b_chunk_with_high_bit(input, 2);
    if shift_by_7s(input, 4) == 0 {
        buf[3] = shift_by_7s(input, 3) as u8;
        return 4;
    }
    buf[3] = nth_7b_chunk_with_high_bit(input, 3);
    if shift_by_7s(input, 5) == 0 {
        buf[4] = shift_by_7s(input, 4) as u8;
        return 5;
    }
    buf[4] = nth_7b_chunk_with_high_bit(input, 4);
    if shift_by_7s(input, 6) == 0 {
        buf[5] = shift_by_7s(input, 5) as u8;
        return 6;
    }
    buf[5] = nth_7b_chunk_with_high_bit(input, 5);
    if shift_by_7s(input, 7) == 0 {
        buf[6] = shift_by_7s(input, 6) as u8;
        return 7;
    }
    buf[6] = nth_7b_chunk_with_high_bit(input, 6);
    if shift_by_7s(input, 8) == 0 {
        buf[7] = shift_by_7s(input, 7) as u8;
        return 8;
    }
    buf[7] = nth_7b_chunk_with_high_bit(input, 7);
    // special case: write last whole byte as is
    buf[8] = (input >> 56) as u8;
    9
}

/// input: a u64
/// n: >0, how many 7-bit shifts to do
/// Returns the input shifted over by `n` groups of 7 bits.
#[inline]
fn shift_by_7s(input: u64, n: u8) -> u64 {
    input >> (7 * n)
}

/// input: a u64
/// n: >0, how many 7-bit shifts to do
/// Returns the n'th chunk (starting from least significant) of 7 bits as a byte.
/// The high bit in the byte will be set (not one of the 7 bits that map to input bits).
#[inline]
fn nth_7b_chunk_with_high_bit(input: u64, n: u8) -> u8 {
    (shift_by_7s(input, n) as u8) | 0x80
}

// Only public for testing.
/// Map signed numbers to unsigned: 0 to 0, -1 to 1, 1 to 2, -2 to 3, etc
#[inline]
pub fn zig_zag_encode(num: i64) -> u64 {
    // If num < 0, num >> 63 is all 1 and vice versa.
    ((num << 1) ^ (num >> 63)) as u64
}

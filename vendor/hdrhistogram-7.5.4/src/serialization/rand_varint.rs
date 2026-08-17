//! This is used in tests (both unit tests and integration tests) to provide useful distributions
//! of random numbers.

use rand::distributions::uniform::Uniform;
use rand::distributions::Distribution;
use rand::Rng;

/// Smallest number in our varint encoding that takes the given number of bytes
pub fn smallest_number_in_n_byte_varint(byte_length: usize) -> u64 {
    assert!(byte_length <= 9 && byte_length >= 1);

    match byte_length {
        1 => 0,
        // one greater than the largest of the previous length
        _ => largest_number_in_n_byte_varint(byte_length - 1) + 1,
    }
}

/// Largest number in our varint encoding that takes the given number of bytes
pub fn largest_number_in_n_byte_varint(byte_length: usize) -> u64 {
    assert!(byte_length <= 9 && byte_length >= 1);

    match byte_length {
        9 => u64::max_value(),
        _ => largest_number_in_7_bit_chunk(byte_length - 1),
    }
}

/// The largest in the set of numbers that have at least 1 bit set in the n'th chunk of 7 bits.
fn largest_number_in_7_bit_chunk(chunk_index: usize) -> u64 {
    // Our 9-byte varints do different encoding in the last byte, so we don't handle them here
    assert!(chunk_index <= 7);

    // 1 in every bit below the lowest bit in this chunk
    let lower_bits = match chunk_index {
        0 => 0,
        _ => largest_number_in_7_bit_chunk(chunk_index - 1),
    };

    // 1 in every bit in this chunk
    let this_chunk = 0x7F_u64 << (chunk_index * 7);

    lower_bits | this_chunk
}

// Evenly distributed random numbers end up biased heavily towards longer encoded byte lengths:
// there are a lot more large numbers than there are small (duh), but for exercising serialization
// code paths, we'd like many at all byte lengths. This is also arguably more representative of
// real data. This should emit values whose varint lengths are uniformly distributed across the
// whole length range (1 to 9).
pub struct RandomVarintEncodedLengthIter<R: Rng> {
    ranges: [Uniform<u64>; 9],
    range_for_picking_range: Uniform<usize>,
    rng: R,
}

impl<R: Rng> RandomVarintEncodedLengthIter<R> {
    pub fn new(rng: R) -> RandomVarintEncodedLengthIter<R> {
        RandomVarintEncodedLengthIter {
            ranges: [
                Uniform::new(
                    smallest_number_in_n_byte_varint(1),
                    largest_number_in_n_byte_varint(1) + 1,
                ),
                Uniform::new(
                    smallest_number_in_n_byte_varint(2),
                    largest_number_in_n_byte_varint(2) + 1,
                ),
                Uniform::new(
                    smallest_number_in_n_byte_varint(3),
                    largest_number_in_n_byte_varint(3) + 1,
                ),
                Uniform::new(
                    smallest_number_in_n_byte_varint(4),
                    largest_number_in_n_byte_varint(4) + 1,
                ),
                Uniform::new(
                    smallest_number_in_n_byte_varint(5),
                    largest_number_in_n_byte_varint(5) + 1,
                ),
                Uniform::new(
                    smallest_number_in_n_byte_varint(6),
                    largest_number_in_n_byte_varint(6) + 1,
                ),
                Uniform::new(
                    smallest_number_in_n_byte_varint(7),
                    largest_number_in_n_byte_varint(7) + 1,
                ),
                Uniform::new(
                    smallest_number_in_n_byte_varint(8),
                    largest_number_in_n_byte_varint(8) + 1,
                ),
                Uniform::new(
                    smallest_number_in_n_byte_varint(9),
                    largest_number_in_n_byte_varint(9),
                ),
            ],
            range_for_picking_range: Uniform::new(0, 9),
            rng,
        }
    }
}

impl<R: Rng> Iterator for RandomVarintEncodedLengthIter<R> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // pick the range we'll use
        let value_range = self.ranges[self.range_for_picking_range.sample(&mut self.rng)];

        Some(value_range.sample(&mut self.rng))
    }
}

#[test]
fn largest_number_in_7_bit_chunk_correct() {
    // 8 chunks (indices 0-7) of 7 bits gets you to 56 bits. Last byte in varint is handled
    // differently, so we don't test that here.
    for i in 0..8 {
        let largest = largest_number_in_7_bit_chunk(i);
        assert_eq!((i as u32 + 1) * 7, largest.count_ones());

        assert_eq!(64 - ((i as u32) + 1) * 7, largest.leading_zeros());
        // any larger and it will be in the next chunk
        assert_eq!(largest.leading_zeros() - 1, (largest + 1).leading_zeros());
    }
}

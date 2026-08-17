/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::{Alphanumeric, DistString};

/// Generates a random string of a given length
fn random_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

const INPUT_SIZES: [usize; 4] = [1, 10, 1_000, 100_000];

fn bench_encodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Encode");

    for length in INPUT_SIZES {
        let input = &random_string(length);

        group.bench_with_input(
            BenchmarkId::new("handrolled_base64", length),
            input,
            |b, i| b.iter(|| handrolled_base64::encode(i)),
        );
        group.bench_with_input(BenchmarkId::new("base64_simd", length), input, |b, i| {
            b.iter(|| aws_smithy_types::base64::encode(i))
        });
    }
    group.finish()
}

fn bench_decodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Decode");

    for length in INPUT_SIZES {
        let string = &random_string(length);
        let encoded = &aws_smithy_types::base64::encode(string);

        group.bench_with_input(
            BenchmarkId::new("handrolled_base64", length),
            encoded,
            |b, i| b.iter(|| handrolled_base64::decode(i).unwrap()),
        );
        group.bench_with_input(BenchmarkId::new("base64_simd", length), encoded, |b, i| {
            b.iter(|| aws_smithy_types::base64::decode(i).unwrap())
        });
    }
    group.finish()
}

fn bench_encoded_lengths(c: &mut Criterion) {
    let mut group = c.benchmark_group("Decoded length");

    for length in INPUT_SIZES {
        group.bench_with_input(
            BenchmarkId::new("handrolled_base64", length),
            &length,
            |b, &i| b.iter(|| handrolled_base64::encoded_length(i as u64)),
        );
        group.bench_with_input(BenchmarkId::new("base64_simd", length), &length, |b, &i| {
            b.iter(|| aws_smithy_types::base64::encoded_length(i))
        });
    }
    group.finish()
}

criterion_group!(benches, bench_encodes, bench_decodes, bench_encoded_lengths);
criterion_main!(benches);

mod handrolled_base64 {
    /*
     * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
     * SPDX-License-Identifier: Apache-2.0
     */

    //! A correct, small, but not especially fast base64 implementation

    use std::error::Error;
    use std::fmt;

    const BASE64_ENCODE_TABLE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    const BASE64_DECODE_TABLE: &[Option<u8>; 256] = &decode_table();

    const PADDING_SENTINEL: u8 = 0xFF;

    const fn encode_table_index_of(i: usize) -> Option<u8> {
        let mut index = 0;
        // inline const index-of implementation
        while index < BASE64_ENCODE_TABLE.len() {
            if BASE64_ENCODE_TABLE[index] as usize == i {
                return Some(index as u8);
            }
            index += 1;
        }
        None
    }

    /// Build a decode table mapping `char as u8` to base64 bit sequences
    const fn decode_table() -> [Option<u8>; 256] {
        let mut output = [None; 256];
        let mut i = 0;
        while i < 256 {
            if i == 61 {
                output[i] = Some(PADDING_SENTINEL);
            } else {
                output[i] = encode_table_index_of(i);
            }
            i += 1;
        }
        output
    }

    /// Encode `input` into base64 using the standard base64 alphabet
    pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
        encode_inner(input.as_ref())
    }

    /// encode_inner defined to reduce monomorphisation cost
    fn encode_inner(inp: &[u8]) -> String {
        // Base 64 encodes groups of 6 bits into characters—this means that each
        // 3 byte group (24 bits) is encoded into 4 base64 characters.
        let char_ct = inp.len().div_ceil(3) * 4;
        let mut output = String::with_capacity(char_ct);
        for chunk in inp.chunks(3) {
            let mut block: i32 = 0;
            // Write the chunks into the beginning of a 32 bit int
            for (idx, chunk) in chunk.iter().enumerate() {
                block |= (*chunk as i32) << ((3 - idx) * 8);
            }
            let num_sextets = (chunk.len() * 8).div_ceil(6);
            for idx in 0..num_sextets {
                let slice = block >> (26 - (6 * idx));
                let idx = (slice as u8) & 0b0011_1111;
                output.push(BASE64_ENCODE_TABLE[idx as usize] as char);
            }
            for _ in 0..(4 - num_sextets) {
                output.push('=');
            }
        }
        // be sure we calculated the size right
        debug_assert_eq!(output.capacity(), char_ct);
        output
    }

    /// Decode `input` from base64 using the standard base64 alphabet
    ///
    /// If input is not a valid base64 encoded string, this function will return `DecodeError`.
    pub fn decode<T: AsRef<str>>(input: T) -> Result<Vec<u8>, DecodeError> {
        decode_inner(input.as_ref())
    }

    /// Failure to decode a base64 value.
    #[allow(clippy::enum_variant_names)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum DecodeError {
        /// Encountered an invalid byte.
        InvalidByte,
        /// Encountered an invalid base64 padding value.
        InvalidPadding,
        /// Input wasn't long enough to be a valid base64 value.
        InvalidLength,
    }

    impl Error for DecodeError {}

    impl fmt::Display for DecodeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use DecodeError::*;
            match self {
                InvalidByte => write!(f, "invalid byte"),
                InvalidPadding => write!(f, "invalid padding"),
                InvalidLength => write!(f, "invalid length"),
            }
        }
    }

    fn decode_inner(inp: &str) -> Result<Vec<u8>, DecodeError> {
        // one base64 character is only 6 bits so it can't produce valid data.
        if inp.len() == 1 {
            return Err(DecodeError::InvalidLength);
        }

        // when there's padding, we might slightly over allocate but it significantly simplifies
        // the code to just ignore it.
        let mut ret = Vec::with_capacity(inp.len().div_ceil(4) * 3);

        // 4 base-64 characters = 3 bytes
        // 1. Break the input into 4 character segments
        // 2. Write those segments into an i32
        // 3. Read u8s back out of the i32
        let chunks = inp.as_bytes().chunks(4);
        let mut padding = 0;
        for chunk in chunks {
            // padding should only be set on the last input
            if padding != 0 {
                return Err(DecodeError::InvalidPadding);
            }
            let mut block = 0_i32;
            for (idx, chunk) in chunk.iter().enumerate() {
                let bits = BASE64_DECODE_TABLE[*chunk as usize].ok_or(DecodeError::InvalidByte)?;
                if bits == 0xFF {
                    padding += 1;
                } else if padding > 0 {
                    // Once you've started padding, you can't stop.
                    return Err(DecodeError::InvalidPadding);
                }
                block |= (bits as i32) << (18 - (idx * 6));
            }
            // if we got a short slice, its because of implied padding
            let missing_chars = 4 - chunk.len();
            for i in (padding + missing_chars..3).rev() {
                let byte = ((block >> (i * 8)) & 0xFF) as u8;
                ret.push(byte)
            }
        }

        // The code is much simpler if we _slightly_ over allocate in certain cases
        debug_assert!(ret.capacity() - ret.len() < 4);
        Ok(ret)
    }

    /// Given the length of some data in bytes, return how many bytes it would take to base64 encode
    /// that data.
    pub fn encoded_length(length: u64) -> u64 {
        length.div_ceil(3) * 4
    }
}

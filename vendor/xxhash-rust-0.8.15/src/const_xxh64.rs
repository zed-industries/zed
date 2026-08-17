//!Const 64 bit version of xxhash algorithm

use core::mem;

use crate::xxh64_common::*;

#[inline(always)]
const fn read_u32(input: &[u8], cursor: usize) -> u32 {
    input[cursor] as u32 | (input[cursor + 1] as u32) << 8 | (input[cursor + 2] as u32) << 16 | (input[cursor + 3] as u32) << 24
}

#[inline(always)]
const fn read_u64(input: &[u8], cursor: usize) -> u64 {
    input[cursor] as u64
        | (input[cursor + 1] as u64) << 8
        | (input[cursor + 2] as u64) << 16
        | (input[cursor + 3] as u64) << 24
        | (input[cursor + 4] as u64) << 32
        | (input[cursor + 5] as u64) << 40
        | (input[cursor + 6] as u64) << 48
        | (input[cursor + 7] as u64) << 56
}

const fn finalize(mut input: u64, data: &[u8], mut cursor: usize) -> u64 {
    let mut len = data.len() - cursor;

    while len >= 8 {
        input ^= round(0, read_u64(data, cursor));
        cursor += mem::size_of::<u64>();
        len -= mem::size_of::<u64>();
        input = input.rotate_left(27).wrapping_mul(PRIME_1).wrapping_add(PRIME_4)
    }

    if len >= 4 {
        input ^= (read_u32(data, cursor) as u64).wrapping_mul(PRIME_1);
        cursor += mem::size_of::<u32>();
        len -= mem::size_of::<u32>();
        input = input.rotate_left(23).wrapping_mul(PRIME_2).wrapping_add(PRIME_3);
    }

    while len > 0 {
        input ^= (data[cursor] as u64).wrapping_mul(PRIME_5);
        cursor += mem::size_of::<u8>();
        len -= mem::size_of::<u8>();
        input = input.rotate_left(11).wrapping_mul(PRIME_1);
    }

    avalanche(input)
}

///Returns hash for the provided input.
pub const fn xxh64(input: &[u8], seed: u64) -> u64 {
    let input_len = input.len() as u64;
    let mut cursor = 0;
    let mut result;

    if input.len() >= CHUNK_SIZE {
        let mut v1 = seed.wrapping_add(PRIME_1).wrapping_add(PRIME_2);
        let mut v2 = seed.wrapping_add(PRIME_2);
        let mut v3 = seed;
        let mut v4 = seed.wrapping_sub(PRIME_1);

        loop {
            v1 = round(v1, read_u64(input, cursor));
            cursor += mem::size_of::<u64>();
            v2 = round(v2, read_u64(input, cursor));
            cursor += mem::size_of::<u64>();
            v3 = round(v3, read_u64(input, cursor));
            cursor += mem::size_of::<u64>();
            v4 = round(v4, read_u64(input, cursor));
            cursor += mem::size_of::<u64>();

            if (input.len() - cursor) < CHUNK_SIZE {
                break;
            }
        }

        result = v1.rotate_left(1).wrapping_add(v2.rotate_left(7))
                                  .wrapping_add(v3.rotate_left(12))
                                  .wrapping_add(v4.rotate_left(18));

        result = merge_round(result, v1);
        result = merge_round(result, v2);
        result = merge_round(result, v3);
        result = merge_round(result, v4);
    } else {
        result = seed.wrapping_add(PRIME_5)
    }

    result = result.wrapping_add(input_len);

    finalize(result, input, cursor)
}

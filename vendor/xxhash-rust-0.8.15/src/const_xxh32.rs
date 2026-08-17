//!Const eval friendly xxh32 implementation.

use core::mem;

use crate::xxh32_common::*;

#[inline(always)]
const fn read_u32(input: &[u8], cursor: usize) -> u32 {
    input[cursor] as u32 | (input[cursor + 1] as u32) << 8 | (input[cursor + 2] as u32) << 16 | (input[cursor + 3] as u32) << 24
}

const fn finalize(mut input: u32, data: &[u8], mut cursor: usize) -> u32 {
    let mut len = data.len() - cursor;

    while len >= 4 {
        input = input.wrapping_add(
            read_u32(data, cursor).wrapping_mul(PRIME_3)
        );
        cursor += mem::size_of::<u32>();
        len -= mem::size_of::<u32>();
        input = input.rotate_left(17).wrapping_mul(PRIME_4);
    }

    while len > 0 {
        input = input.wrapping_add((data[cursor] as u32).wrapping_mul(PRIME_5));
        cursor += mem::size_of::<u8>();
        len -= mem::size_of::<u8>();
        input = input.rotate_left(11).wrapping_mul(PRIME_1);
    }

    avalanche(input)
}

///Const variant of xxh32 hashing
pub const fn xxh32(input: &[u8], seed: u32) -> u32 {
    let mut result = input.len() as u32;
    let mut cursor = 0;

    if input.len() >= CHUNK_SIZE {
        let mut v1 = seed.wrapping_add(PRIME_1).wrapping_add(PRIME_2);
        let mut v2 = seed.wrapping_add(PRIME_2);
        let mut v3 = seed;
        let mut v4 = seed.wrapping_sub(PRIME_1);

        loop {
            v1 = round(v1, read_u32(input, cursor));
            cursor += mem::size_of::<u32>();
            v2 = round(v2, read_u32(input, cursor));
            cursor += mem::size_of::<u32>();
            v3 = round(v3, read_u32(input, cursor));
            cursor += mem::size_of::<u32>();
            v4 = round(v4, read_u32(input, cursor));
            cursor += mem::size_of::<u32>();

            if (input.len() - cursor) < CHUNK_SIZE {
                break;
            }
        }

        result = result.wrapping_add(
            v1.rotate_left(1).wrapping_add(
                v2.rotate_left(7).wrapping_add(
                    v3.rotate_left(12).wrapping_add(
                        v4.rotate_left(18)
                    )
                )
            )
        );
    } else {
        result = result.wrapping_add(seed.wrapping_add(PRIME_5));
    }

    finalize(result, input, cursor)
}

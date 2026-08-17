#![allow(unused)]

use core::mem;

pub const CHUNK_SIZE: usize = mem::size_of::<u32>() * 4;
pub const PRIME_1: u32 = 0x9E3779B1;
pub const PRIME_2: u32 = 0x85EBCA77;
pub const PRIME_3: u32 = 0xC2B2AE3D;
pub const PRIME_4: u32 = 0x27D4EB2F;
pub const PRIME_5: u32 = 0x165667B1;

#[inline]
pub const fn round(acc: u32, input: u32) -> u32 {
    acc.wrapping_add(input.wrapping_mul(PRIME_2))
       .rotate_left(13)
       .wrapping_mul(PRIME_1)
}

#[inline]
pub const fn avalanche(mut input: u32) -> u32 {
    input ^= input >> 15;
    input = input.wrapping_mul(PRIME_2);
    input ^= input >> 13;
    input = input.wrapping_mul(PRIME_3);
    input ^= input >> 16;
    input
}

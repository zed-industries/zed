// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cmp;

use crate::io::Monitor;

fn transform(state: &mut [u32; 4], buf: &[u8]) {
    // Assert to hopefully force the compiler to elide bounds checks on buf.
    assert!(buf.len() == 64);

    let mut input = [0u32; 16];

    // Collect 4 bytes from an input buffer and store as a u32 in the output buffer. Note: input
    // bytes are considered little-endian for MD5.
    macro_rules! collect {
        ($output:ident, $input:ident, $idx:expr) => {
            $output[$idx] = u32::from_le_bytes([
                $input[$idx * 4 + 0],
                $input[$idx * 4 + 1],
                $input[$idx * 4 + 2],
                $input[$idx * 4 + 3],
            ]);
        };
    }

    collect!(input, buf, 0);
    collect!(input, buf, 1);
    collect!(input, buf, 2);
    collect!(input, buf, 3);
    collect!(input, buf, 4);
    collect!(input, buf, 5);
    collect!(input, buf, 6);
    collect!(input, buf, 7);
    collect!(input, buf, 8);
    collect!(input, buf, 9);
    collect!(input, buf, 10);
    collect!(input, buf, 11);
    collect!(input, buf, 12);
    collect!(input, buf, 13);
    collect!(input, buf, 14);
    collect!(input, buf, 15);

    // The transformation for a single step of a round: A = B + ROTL32(F + A + K[i] + M[g], S).
    macro_rules! round_step {
        ($a:ident, $b:ident, $f:expr, $m:expr, $s:expr, $k:expr) => {
            $a = $f.wrapping_add($a).wrapping_add($k).wrapping_add($m);
            $a = $b.wrapping_add($a.rotate_left($s));
        };
    }

    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];

    // Round 1: F(B, C, D) = D xor (B and (C xor D))
    {
        macro_rules! T {
            ($a:ident, $b:ident, $c:ident, $d:ident, $m:expr, $s:expr, $k:expr) => {
                round_step!($a, $b, $d ^ ($b & ($c ^ $d)), $m, $s, $k);
            };
        }

        T!(a, b, c, d, input[0], 7, 0xd76aa478);
        T!(d, a, b, c, input[1], 12, 0xe8c7b756);
        T!(c, d, a, b, input[2], 17, 0x242070db);
        T!(b, c, d, a, input[3], 22, 0xc1bdceee);
        T!(a, b, c, d, input[4], 7, 0xf57c0faf);
        T!(d, a, b, c, input[5], 12, 0x4787c62a);
        T!(c, d, a, b, input[6], 17, 0xa8304613);
        T!(b, c, d, a, input[7], 22, 0xfd469501);
        T!(a, b, c, d, input[8], 7, 0x698098d8);
        T!(d, a, b, c, input[9], 12, 0x8b44f7af);
        T!(c, d, a, b, input[10], 17, 0xffff5bb1);
        T!(b, c, d, a, input[11], 22, 0x895cd7be);
        T!(a, b, c, d, input[12], 7, 0x6b901122);
        T!(d, a, b, c, input[13], 12, 0xfd987193);
        T!(c, d, a, b, input[14], 17, 0xa679438e);
        T!(b, c, d, a, input[15], 22, 0x49b40821);
    }

    // Round 2: G(B, C, D) = C xor (D and (B xor C))
    {
        macro_rules! T {
            ($a:ident, $b:ident, $c:ident, $d:ident, $m:expr, $s:expr, $k:expr) => {
                round_step!($a, $b, $c ^ ($d & ($b ^ $c)), $m, $s, $k);
            };
        }

        T!(a, b, c, d, input[1], 5, 0xf61e2562);
        T!(d, a, b, c, input[6], 9, 0xc040b340);
        T!(c, d, a, b, input[11], 14, 0x265e5a51);
        T!(b, c, d, a, input[0], 20, 0xe9b6c7aa);
        T!(a, b, c, d, input[5], 5, 0xd62f105d);
        T!(d, a, b, c, input[10], 9, 0x02441453);
        T!(c, d, a, b, input[15], 14, 0xd8a1e681);
        T!(b, c, d, a, input[4], 20, 0xe7d3fbc8);
        T!(a, b, c, d, input[9], 5, 0x21e1cde6);
        T!(d, a, b, c, input[14], 9, 0xc33707d6);
        T!(c, d, a, b, input[3], 14, 0xf4d50d87);
        T!(b, c, d, a, input[8], 20, 0x455a14ed);
        T!(a, b, c, d, input[13], 5, 0xa9e3e905);
        T!(d, a, b, c, input[2], 9, 0xfcefa3f8);
        T!(c, d, a, b, input[7], 14, 0x676f02d9);
        T!(b, c, d, a, input[12], 20, 0x8d2a4c8a);
    }

    // Round 3: H(B, C, D) = B xor C xor D
    {
        macro_rules! T {
            ($a:ident, $b:ident, $c:ident, $d:ident, $m:expr, $s:expr, $k:expr) => {
                round_step!($a, $b, $b ^ $c ^ $d, $m, $s, $k);
            };
        }

        T!(a, b, c, d, input[5], 4, 0xfffa3942);
        T!(d, a, b, c, input[8], 11, 0x8771f681);
        T!(c, d, a, b, input[11], 16, 0x6d9d6122);
        T!(b, c, d, a, input[14], 23, 0xfde5380c);
        T!(a, b, c, d, input[1], 4, 0xa4beea44);
        T!(d, a, b, c, input[4], 11, 0x4bdecfa9);
        T!(c, d, a, b, input[7], 16, 0xf6bb4b60);
        T!(b, c, d, a, input[10], 23, 0xbebfbc70);
        T!(a, b, c, d, input[13], 4, 0x289b7ec6);
        T!(d, a, b, c, input[0], 11, 0xeaa127fa);
        T!(c, d, a, b, input[3], 16, 0xd4ef3085);
        T!(b, c, d, a, input[6], 23, 0x04881d05);
        T!(a, b, c, d, input[9], 4, 0xd9d4d039);
        T!(d, a, b, c, input[12], 11, 0xe6db99e5);
        T!(c, d, a, b, input[15], 16, 0x1fa27cf8);
        T!(b, c, d, a, input[2], 23, 0xc4ac5665);
    }

    // Round 4: I(B,C,D) = C xor (B or (not D))
    {
        macro_rules! T {
            ($a:ident, $b:ident, $c:ident, $d:ident, $m:expr, $s:expr, $k:expr) => {
                round_step!($a, $b, $c ^ ($b | !$d), $m, $s, $k);
            };
        }

        T!(a, b, c, d, input[0], 6, 0xf4292244);
        T!(d, a, b, c, input[7], 10, 0x432aff97);
        T!(c, d, a, b, input[14], 15, 0xab9423a7);
        T!(b, c, d, a, input[5], 21, 0xfc93a039);
        T!(a, b, c, d, input[12], 6, 0x655b59c3);
        T!(d, a, b, c, input[3], 10, 0x8f0ccc92);
        T!(c, d, a, b, input[10], 15, 0xffeff47d);
        T!(b, c, d, a, input[1], 21, 0x85845dd1);
        T!(a, b, c, d, input[8], 6, 0x6fa87e4f);
        T!(d, a, b, c, input[15], 10, 0xfe2ce6e0);
        T!(c, d, a, b, input[6], 15, 0xa3014314);
        T!(b, c, d, a, input[13], 21, 0x4e0811a1);
        T!(a, b, c, d, input[4], 6, 0xf7537e82);
        T!(d, a, b, c, input[11], 10, 0xbd3af235);
        T!(c, d, a, b, input[2], 15, 0x2ad7d2bb);
        T!(b, c, d, a, input[9], 21, 0xeb86d391);
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
}

/// `Md5` implements the MD5 hashing algorithm.
pub struct Md5 {
    state: [u32; 4],
    block: [u8; Md5::BLOCK_LEN],
    len: u64,
}

impl Default for Md5 {
    fn default() -> Self {
        Md5 {
            state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            block: [0; Md5::BLOCK_LEN],
            len: 0,
        }
    }
}

impl Md5 {
    const BLOCK_LEN: usize = 64;
    const BLOCK_LEN_MASK: u64 = 0x3f;

    /// Finalizes and returns the computed MD5 hash.
    pub fn md5(&self) -> [u8; 16] {
        // Finalize locally.
        let mut block = [0; Md5::BLOCK_LEN];
        let mut state = self.state;

        // The block length is the amount of data buffered for the current block.
        let block_len = (self.len & Md5::BLOCK_LEN_MASK) as usize;

        // The block length should *always* be less than the MD5 block length if the process_*
        // functions transform the block when it's full.
        assert!(block_len < Md5::BLOCK_LEN);

        // Copy the buffered block data locally for finalization.
        block[..block_len].copy_from_slice(&self.block[..block_len]);

        // Append the message terminator to the block.
        block[block_len] = 0x80;

        // If the message length can not be appended to the block, transform the block, and start
        // a new block.
        if Md5::BLOCK_LEN - block_len - 1 < 8 {
            transform(&mut state, &block);
            block = [0; Md5::BLOCK_LEN];
        }

        // The final 8 bytes of the final block contain the message length in bits mod 2^64.
        block[Md5::BLOCK_LEN - 8..Md5::BLOCK_LEN].copy_from_slice(&(self.len << 3).to_le_bytes());
        transform(&mut state, &block);

        // The message digest is in big-endian.
        let mut hash = [0; 16];
        hash[0..4].copy_from_slice(&state[0].to_le_bytes());
        hash[4..8].copy_from_slice(&state[1].to_le_bytes());
        hash[8..12].copy_from_slice(&state[2].to_le_bytes());
        hash[12..16].copy_from_slice(&state[3].to_le_bytes());
        hash
    }
}

impl Monitor for Md5 {
    #[inline(always)]
    fn process_byte(&mut self, byte: u8) {
        self.block[(self.len & Md5::BLOCK_LEN_MASK) as usize] = byte;
        self.len += 1;

        // Atleast 1 bytes has been written (see above) and the length is a multiple of the MD5
        // block length, therefore the current block is full. Perform a MD5 transformation on the
        // current block.
        if self.len & Md5::BLOCK_LEN_MASK == 0 {
            transform(&mut self.state, &self.block);
        }
    }

    #[inline(always)]
    fn process_buf_bytes(&mut self, buf: &[u8]) {
        let mut rem = buf;

        while !rem.is_empty() {
            let block_len = (self.len & Md5::BLOCK_LEN_MASK) as usize;

            let copy_len = cmp::min(rem.len(), Md5::BLOCK_LEN - block_len);

            self.len += copy_len as u64;

            // If the copy length is a whole block then perform the transformation directly from the
            // source buffer.
            if copy_len == Md5::BLOCK_LEN {
                transform(&mut self.state, &rem[..copy_len]);
            }
            else {
                // If the copy length is less than a whole block, buffer it into the current block.
                self.block[block_len..block_len + copy_len].copy_from_slice(&rem[..copy_len]);

                if self.len & Md5::BLOCK_LEN_MASK == 0 {
                    transform(&mut self.state, &self.block);
                }
            }

            rem = &rem[copy_len..];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Md5;
    use super::Monitor;

    #[test]
    fn verify_md5() {
        const STRINGS: [&[u8]; 8] = [
            b"",
            b"a",
            b"abc",
            b"The quick brown fox jumps over the lazy dog",
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!",
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!?",
            b".s)cyIl?XKs}wDnLEUeZj'72=A/0!w;B[e*QUh)0{&XcGvf'xMx5Chhx_'ahg{GP|_R(0=Xe`lXQN_@MK9::",
        ];

        #[rustfmt::skip]
        const HASHES: [[u8; 16]; 8] = [
            [
                0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04,
                0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e,
            ],
            [
                0x0c, 0xc1, 0x75, 0xb9, 0xc0, 0xf1, 0xb6, 0xa8,
                0x31, 0xc3, 0x99, 0xe2, 0x69, 0x77, 0x26, 0x61,
            ],
            [
                0x90, 0x01, 0x50, 0x98, 0x3c, 0xd2, 0x4f, 0xb0,
                0xd6, 0x96, 0x3f, 0x7d, 0x28, 0xe1, 0x7f, 0x72,
            ],
            [
                0x9e, 0x10, 0x7d, 0x9d, 0x37, 0x2b, 0xb6, 0x82,
                0x6b, 0xd8, 0x1d, 0x35, 0x42, 0xa4, 0x19, 0xd6,
            ],
            [
                0xd1, 0x74, 0xab, 0x98, 0xd2, 0x77, 0xd9, 0xf5,
                0xa5, 0x61, 0x1c, 0x2c, 0x9f, 0x41, 0x9d, 0x9f,
            ],
            [
                0x64, 0x1b, 0xa6, 0x02, 0x88, 0xc1, 0x7a, 0x2d,
                0xa5, 0x09, 0x00, 0x77, 0xeb, 0x89, 0x58, 0xad,
            ],
            [
                0x0a, 0x71, 0xdb, 0x4d, 0xf3, 0x50, 0x92, 0x73,
                0x62, 0x42, 0x3a, 0x42, 0xdc, 0xf8, 0x14, 0x57,
            ],
            [
                0x0b, 0x76, 0x74, 0x7e, 0xfd, 0xcd, 0xb9, 0x33,
                0x67, 0xfe, 0x2d, 0xa3, 0x21, 0x1b, 0x5d, 0x41,
            ],
        ];

        // As a buffer.
        for (string, hash) in STRINGS.iter().zip(&HASHES) {
            let mut md5: Md5 = Default::default();

            md5.process_buf_bytes(string);

            assert_eq!(*hash, md5.md5());
        }

        // As partial buffers.
        for (string, hash) in STRINGS.iter().zip(&HASHES) {
            let mut md5: Md5 = Default::default();

            for bytes in string.chunks(21) {
                md5.process_buf_bytes(bytes);
            }

            assert_eq!(*hash, md5.md5());
        }

        // Byte-by-byte
        for (string, hash) in STRINGS.iter().zip(&HASHES) {
            let mut md5: Md5 = Default::default();

            for byte in string.iter() {
                md5.process_byte(*byte);
            }

            assert_eq!(*hash, md5.md5());
        }
    }
}

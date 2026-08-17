//! This module provides a function to combine CRCs of two sequences of bytes.
//!
//! It is based on the work of Mark Adler and is designed to be used with
//! different CRC algorithms.
/*
  Derived from this excellent answer by Mark Adler on StackOverflow:
  https://stackoverflow.com/questions/29915764/generic-crc-8-16-32-64-combine-implementation/29928573#29928573
*/

/* crccomb.c -- generalized combination of CRCs
 * Copyright (C) 2015 Mark Adler
 * Version 1.1  29 Apr 2015  Mark Adler
 */

/*
 This software is provided 'as-is', without any express or implied
 warranty.  In no event will the author be held liable for any damages
 arising from the use of this software.

 Permission is granted to anyone to use this software for any purpose,
 including commercial applications, and to alter it and redistribute it
 freely, subject to the following restrictions:

 1. The origin of this software must not be misrepresented; you must not
    claim that you wrote the original software. If you use this software
    in a product, an acknowledgment in the product documentation would be
    appreciated but is not required.
 2. Altered source versions must be plainly marked as such, and must not be
    misrepresented as being the original software.
 3. This notice may not be removed or altered from any source distribution.

 Mark Adler
 madler@alumni.caltech.edu
*/

/*
  zlib provides a fast operation to combine the CRCs of two sequences of bytes
  into a single CRC, which is the CRC of the two sequences concatenated.  That
  operation requires only the two CRC's and the length of the second sequence.
  The routine in zlib only works on the particular CRC-32 used by zlib.  The
  code provided here generalizes that operation to apply to a wide range of
  CRCs.  The CRC is specified in a series of #defines, based on the
  parameterization found in Ross William's excellent CRC tutorial here:

     http://www.ross.net/crc/download/crc_v3.txt

  A comprehensive catalogue of known CRCs, their parameters, check values, and
  references can be found here:

     http://reveng.sourceforge.net/crc-catalogue/all.htm
*/

use crate::CrcParams;

/* Multiply the GF(2) vector vec by the GF(2) matrix mat, returning the
resulting vector.  The vector is stored as bits in a crc_t.  The matrix is
similarly stored with each column as a crc_t, where the number of columns is
at least enough to cover the position of the most significant 1 bit in the
vector (so a dimension parameter is not needed). */
fn gf2_matrix_times(mat: &[u64; 64], mut vec: u64) -> u64 {
    let mut sum = 0;
    let mut idx = 0;
    while vec > 0 {
        if vec & 1 == 1 {
            sum ^= mat[idx];
        }
        vec >>= 1;
        idx += 1;
    }

    sum
}

/* Multiply the matrix mat by itself, returning the result in square.  WIDTH is
the dimension of the matrices, i.e., the number of bits in each crc_t
(rows), and the number of crc_t's (columns). */
fn gf2_matrix_square(square: &mut [u64; 64], mat: &[u64; 64]) {
    for n in 0..64 {
        square[n] = gf2_matrix_times(mat, mat[n]);
    }
}

/* Combine the CRCs of two successive sequences, where crc1 is the CRC of the
first sequence of bytes, crc2 is the CRC of the immediately following
sequence of bytes, and len2 is the length of the second sequence.  The CRC
of the combined sequence is returned. */
pub fn checksums(mut crc1: u64, crc2: u64, mut len2: u64, params: &CrcParams) -> u64 {
    let mut col: u64;
    let mut even = [0u64; 64]; /* even-power-of-two zeros operator */
    let mut odd = [0u64; 64]; /* odd-power-of-two zeros operator */

    /* exclusive-or the result with len2 zeros applied to the CRC of an empty
    sequence */
    crc1 ^= params.init_algorithm ^ params.xorout;

    /* construct the operator for one zero bit and put in odd[] */
    if params.refin && params.refout {
        // use the reflected POLY
        odd[0] = reflect_poly(params.poly, params.width as u32);
        col = 1;
        for n in 1..params.width {
            odd[n as usize] = col;
            col <<= 1;
        }
    } else if !params.refin && !params.refout {
        col = 2;
        for n in 0..params.width - 1 {
            odd[n as usize] = col;
            col <<= 1;
        }
        // Put poly at the last valid index (width-1)
        odd[(params.width - 1) as usize] = params.poly;
    } else {
        panic!("Unsupported CRC configuration");
    }

    /* put operator for two zero bits in even */
    gf2_matrix_square(&mut even, &odd);

    /* put operator for four zero bits in odd */
    gf2_matrix_square(&mut odd, &even);

    /* apply len2 zeros to crc1 (first square will put the operator for one
    zero byte, eight zero bits, in even) */
    loop {
        /* apply zeros operator for this bit of len2 */
        gf2_matrix_square(&mut even, &odd);
        if len2 & 1 == 1 {
            crc1 = gf2_matrix_times(&even, crc1);
        }
        len2 >>= 1;

        /* if no more bits set, then done */
        if len2 == 0 {
            break;
        }

        /* another iteration of the loop with odd and even swapped */
        gf2_matrix_square(&mut odd, &even);
        if len2 & 1 == 1 {
            crc1 = gf2_matrix_times(&odd, crc1);
        }
        len2 >>= 1;

        /* if no more bits set, then done */
        if len2 == 0 {
            break;
        }
    }

    /* return combined crc */
    crc1 ^= crc2;

    crc1
}

fn reflect_poly(poly: u64, width: u32) -> u64 {
    assert!(width <= 64, "Width must be <= 64 bits");

    // First reverse all bits
    let reversed = bit_reverse(poly);

    // Shift right to get the significant bits in the correct position
    // For a 32-bit poly, we need to shift right by (64 - 32) = 32 bits
    let shifted = reversed >> (64 - width);

    // Create mask for the target width
    let mask = if width == 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    };

    // Apply mask to ensure we only keep the bits we want
    shifted & mask
}

fn bit_reverse(mut forward: u64) -> u64 {
    let mut reversed = 0;

    for _ in 0..64 {
        reversed <<= 1;
        reversed |= forward & 1;
        forward >>= 1;
    }

    reversed
}

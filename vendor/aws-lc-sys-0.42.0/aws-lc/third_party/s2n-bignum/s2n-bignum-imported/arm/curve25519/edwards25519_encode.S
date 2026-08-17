// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Encode edwards25519 point into compressed form as 256-bit number
// Input p[8]; output z[32] (bytes)
//
//    extern void edwards25519_encode(uint8_t z[static 32],
//                                    const uint64_t p[static 8]);
//
// This assumes that the input buffer p points to a pair of 256-bit
// numbers x (at p) and y (at p+4) representing a point (x,y) on the
// edwards25519 curve. It is assumed that both x and y are < p_25519
// but there is no checking of this, nor of the fact that (x,y) is
// in fact on the curve.
//
// The output in z is a little-endian array of bytes corresponding to
// the standard compressed encoding of a point as 2^255 * x_0 + y
// where x_0 is the least significant bit of x.
// See "https://datatracker.ietf.org/doc/html/rfc8032#section-5.1.2"
// In this implementation, y is simply truncated to 255 bits, but if
// it is reduced mod p_25519 as expected this does not affect values.
//
// Standard ARM ABI: X0 = z, X1 = p
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(edwards25519_encode)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(edwards25519_encode)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(edwards25519_encode)
        .text
        .balign 4

#define z x0
#define p x1

#define y0 x2
#define y1 x3
#define y2 x4
#define y3 x5
#define y0short w2
#define y1short w3
#define y2short w4
#define y3short w5
#define xb x6

S2N_BN_SYMBOL(edwards25519_encode):
        CFI_START

// Load lowest word of x coordinate in xb and full y as [y3;y2;y1;y0].

        ldr     xb, [p]
        ldp     y0, y1, [p, #32]
        ldp     y2, y3, [p, #48]

// Compute the encoded form, making the LSB of x the MSB of the encoding

        and     y3, y3, #0x7FFFFFFFFFFFFFFF
        orr     y3, y3, xb, lsl #63

// Write back in a byte-oriented fashion to be independent of endianness

        strb    y0short, [z]
        lsr     y0, y0, #8
        strb    y0short, [z, #1]
        lsr     y0, y0, #8
        strb    y0short, [z, #2]
        lsr     y0, y0, #8
        strb    y0short, [z, #3]
        lsr     y0, y0, #8
        strb    y0short, [z, #4]
        lsr     y0, y0, #8
        strb    y0short, [z, #5]
        lsr     y0, y0, #8
        strb    y0short, [z, #6]
        lsr     y0, y0, #8
        strb    y0short, [z, #7]

        strb    y1short, [z, #8]
        lsr     y1, y1, #8
        strb    y1short, [z, #9]
        lsr     y1, y1, #8
        strb    y1short, [z, #10]
        lsr     y1, y1, #8
        strb    y1short, [z, #11]
        lsr     y1, y1, #8
        strb    y1short, [z, #12]
        lsr     y1, y1, #8
        strb    y1short, [z, #13]
        lsr     y1, y1, #8
        strb    y1short, [z, #14]
        lsr     y1, y1, #8
        strb    y1short, [z, #15]

        strb    y2short, [z, #16]
        lsr     y2, y2, #8
        strb    y2short, [z, #17]
        lsr     y2, y2, #8
        strb    y2short, [z, #18]
        lsr     y2, y2, #8
        strb    y2short, [z, #19]
        lsr     y2, y2, #8
        strb    y2short, [z, #20]
        lsr     y2, y2, #8
        strb    y2short, [z, #21]
        lsr     y2, y2, #8
        strb    y2short, [z, #22]
        lsr     y2, y2, #8
        strb    y2short, [z, #23]

        strb    y3short, [z, #24]
        lsr     y3, y3, #8
        strb    y3short, [z, #25]
        lsr     y3, y3, #8
        strb    y3short, [z, #26]
        lsr     y3, y3, #8
        strb    y3short, [z, #27]
        lsr     y3, y3, #8
        strb    y3short, [z, #28]
        lsr     y3, y3, #8
        strb    y3short, [z, #29]
        lsr     y3, y3, #8
        strb    y3short, [z, #30]
        lsr     y3, y3, #8
        strb    y3short, [z, #31]

// Return

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(edwards25519_encode)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

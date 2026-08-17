// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo basepoint order, z := x mod n_25519
// Input x[4]; output z[4]
//
//    extern void bignum_mod_n25519_4(uint64_t z[static 4],
//                                    const uint64_t x[static 4]);
//
// Reduction is modulo the order of the curve25519/edwards25519 basepoint.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_n25519_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_n25519_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_n25519_4)
        .text
        .balign 4

#define z x0
#define x x1

#define n0 x2
#define n1 x3

#define d0 x4
#define d1 x5
#define d2 x6
#define d3 x7

#define q x8

#define m0 x9
#define m1 x10
#define m2 x11

// Loading large constants

#define movbig(nn,n3,n2,n1,n0)                                      \
        movz    nn, n0 __LF                                            \
        movk    nn, n1, lsl #16 __LF                                   \
        movk    nn, n2, lsl #32 __LF                                   \
        movk    nn, n3, lsl #48

S2N_BN_SYMBOL(bignum_mod_n25519_4):
        CFI_START

// Load the complicated three words of n_25519.
// The others are n2 = 0 and n3 = 0x1000000000000000, which
// are handled a bit differently

        movbig( n0, #0x5812, #0x631a, #0x5cf5, #0xd3ed)
        movbig( n1, #0x14de, #0xf9de, #0xa2f7, #0x9cd6)

// Load the input number

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]

// Get the quotient estimate q = floor(x/2^252).
// Also delete it from d3, in effect doing x' = x - q * 2^252

        lsr     q, d3, #60
        and     d3, d3, #0x0FFFFFFFFFFFFFFF

// Multiply [m2;m1;m0] = q * [n1;n0]

        mul     m0, n0, q
        mul     m1, n1, q
        umulh   m2, n0, q
        adds    m1, m1, m2
        umulh   m2, n1, q
        adc     m2, m2, xzr

// Subtract [d3;d2;d1;d0] = x' - q * [n1;n0] = x - q * n_25519

        subs    d0, d0, m0
        sbcs    d1, d1, m1
        sbcs    d2, d2, m2
        sbcs    d3, d3, xzr

// If this borrows (CF = 0 because of inversion), add back n_25519.
// The masked n3 digit exploits the fact that bit 60 of n0 is set.

        csel    n0, n0, xzr, cc
        csel    n1, n1, xzr, cc

        adds    d0, d0, n0
        adcs    d1, d1, n1
        and     n0, n0, #0x1000000000000000
        adcs    d2, d2, xzr
        adc     d3, d3, n0

// Store the end result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_n25519_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

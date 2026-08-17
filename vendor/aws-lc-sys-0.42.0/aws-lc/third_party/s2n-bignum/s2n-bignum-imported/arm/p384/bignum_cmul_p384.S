// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_384, z := (c * x) mod p_384, assuming
// x reduced
// Inputs c, x[6]; output z[6]
//
//    extern void bignum_cmul_p384(uint64_t z[static 6], uint64_t c,
//                                 const uint64_t x[static 6]);
//
// Standard ARM ABI: X0 = z, X1 = c, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p384)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p384_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p384_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p384_alt)
        .text
        .balign 4

#define z x0
#define c x1
#define x x2

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define d4 x6
#define d5 x7
#define a0 x8
#define a1 x9
#define a2 x10
#define a3 x11
#define a4 x12
#define a5 x13

// Some shared here

#define h x1
#define h1 x12
#define hn x13
#define m x8
#define l x9


S2N_BN_SYMBOL(bignum_cmul_p384):

S2N_BN_SYMBOL(bignum_cmul_p384_alt):
        CFI_START

// First do the multiply, straightforwardly, getting [h; d5; ...; d0]

        ldp     a0, a1, [x]
        ldp     a2, a3, [x, #16]
        ldp     a4, a5, [x, #32]
        mul     d0, c, a0
        mul     d1, c, a1
        mul     d2, c, a2
        mul     d3, c, a3
        mul     d4, c, a4
        mul     d5, c, a5
        umulh   a0, c, a0
        umulh   a1, c, a1
        umulh   a2, c, a2
        umulh   a3, c, a3
        umulh   a4, c, a4
        umulh   h, c, a5
        adds    d1, d1, a0
        adcs    d2, d2, a1
        adcs    d3, d3, a2
        adcs    d4, d4, a3
        adcs    d5, d5, a4
        adc     h, h, xzr

// Let h be the top word of this intermediate product and l the low 6 words.
// By the range hypothesis on the input, we know h1 = h + 1 does not wrap
// And then -p_384 <= z - h1 * p_384 < p_384, so we just need to subtract
// h1 * p_384 and then correct if that is negative by adding p_384.
//
// Write p_384 = 2^384 - r where r = 2^128 + 2^96 - 2^32 + 1
//
// We want z - (h + 1) * (2^384 - r)
//       = (2^384 * h + l) - (h + 1) * (2^384 - r)
//       = (l + (h + 1) * r) - 2^384.
//
// Thus we can do the computation in 6 words of l + (h + 1) * r, and if it
// does *not* carry we need to add p_384. We can rewrite this as the following,
// using ~h = 2^64 - (h + 1) and absorbing the 2^64 in the higher term
// using h instead of h + 1.
//
//         l + (h + 1) * r
//       = l + 2^128 * (h + 1) + 2^96 * (h + 1) - 2^32 * (h + 1) + (h + 1)
//       = l + 2^128 * (h + 1) + 2^96 * h + 2^32 * ~h + (h + 1)

        add     h1, h, #1
        orn     hn, xzr, h
        lsl     a0, hn, #32
        extr    a1, h, hn, #32
        lsr     a2, h, #32

        adds    a0, a0, h1
        adcs    a1, a1, xzr
        adcs    a2, a2, h1
        adc     a3, xzr, xzr

        adds    d0, d0, a0
        adcs    d1, d1, a1
        adcs    d2, d2, a2
        adcs    d3, d3, a3
        adcs    d4, d4, xzr
        adcs    d5, d5, xzr

// Catch the carry and do a masked addition of p_384

        csetm   m, cc

        mov     l, #0x00000000ffffffff
        and     l, l, m
        adds    d0, d0, l
        eor     l, l, m
        adcs    d1, d1, l
        mov     l, #0xfffffffffffffffe
        and     l, l, m
        adcs    d2, d2, l
        adcs    d3, d3, m
        adcs    d4, d4, m
        adc     d5, d5, m

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

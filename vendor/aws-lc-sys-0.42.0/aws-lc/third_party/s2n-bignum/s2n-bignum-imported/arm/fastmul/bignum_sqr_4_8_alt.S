// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[4]; output z[8]
//
//    extern void bignum_sqr_4_8_alt(uint64_t z[static 8],
//                                   const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_4_8_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_4_8_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_4_8_alt)
        .text
        .balign 4

#define z x0
#define x x1

#define a0 x2
#define a1 x3
#define a2 x4
#define a3 x5

#define l x6
#define h x7

#define u0 x8
#define u1 x9
#define u2 x10
#define u3 x11
#define u4 x12
#define u5 x13
#define u6 x14

// This one is the same as h, which is safe with this computation sequence

#define u7 h

S2N_BN_SYMBOL(bignum_sqr_4_8_alt):
        CFI_START

// Load all the elements, set up an initial window [u6;...u1] = [23;03;01]
// and chain in the addition of 02 + 12 + 13 (no carry-out is possible).
// This gives all the "heterogeneous" terms of the squaring ready to double

        ldp     a0, a1, [x]

        mul     u1, a0, a1
        umulh   u2, a0, a1

        ldp     a2, a3, [x, #16]

        mul     u3, a0, a3
        umulh   u4, a0, a3

        mul     l, a0, a2
        umulh   h, a0, a2
        adds    u2, u2, l

        adcs    u3, u3, h
        mul     l, a1, a2
        umulh   h, a1, a2
        adc     h, h, xzr
        adds    u3, u3, l

        mul     u5, a2, a3
        umulh   u6, a2, a3

        adcs    u4, u4, h
        mul     l, a1, a3
        umulh   h, a1, a3
        adc     h, h, xzr
        adds    u4, u4, l

        adcs    u5, u5, h
        adc     u6, u6, xzr

// Now just double it; this simple approach seems to work better than extr

        adds    u1, u1, u1
        adcs    u2, u2, u2
        adcs    u3, u3, u3
        adcs    u4, u4, u4
        adcs    u5, u5, u5
        adcs    u6, u6, u6
        cset    u7, cs

// Add the homogeneous terms 00 + 11 + 22 + 33

        umulh   l, a0, a0
        mul     u0, a0, a0
        adds    u1, u1, l

        mul     l, a1, a1
        adcs    u2, u2, l
        umulh   l, a1, a1
        adcs    u3, u3, l

        mul     l, a2, a2
        adcs    u4, u4, l
        umulh   l, a2, a2
        adcs    u5, u5, l

        mul     l, a3, a3
        adcs    u6, u6, l
        umulh   l, a3, a3
        adc     u7, u7, l

// Store back final result

        stp     u0, u1, [z]
        stp     u2, u3, [z, #16]
        stp     u4, u5, [z, #32]
        stp     u6, u7, [z, #48]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_4_8_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square modulo p_256k1, z := (x^2) mod p_256k1
// Input x[4]; output z[4]
//
//    extern void bignum_sqr_p256k1(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_p256k1)
        .text
        .balign 4

#define z x0
#define x x1

// Variables

#define u0 x2
#define u1 x3
#define u2 x4
#define u3 x5
#define u4 x6
#define u5 x7
#define u6 x8
#define u7 x9

#define a0 x10
#define a1 x11
#define a2 x12
#define b0 x13
#define b1 x14
#define b3 x15
#define c x16
#define d x17

// Some additional aliases

#define l u4
#define h u5
#define b2 u6
#define q u4
#define a3 u7

S2N_BN_SYMBOL(bignum_sqr_p256k1):
        CFI_START

// First just a near-clone of bignum_sqr_4_8 to get the square, using
// different registers to collect full product without writeback.

        ldp     x10, x11, [x1]
        ldp     x12, x13, [x1, #16]
        umull   x2, w10, w10
        lsr     x14, x10, #32
        umull   x3, w14, w14
        umull   x14, w10, w14
        adds    x2, x2, x14, lsl #33
        lsr     x14, x14, #31
        adc     x3, x3, x14
        umull   x4, w11, w11
        lsr     x14, x11, #32
        umull   x5, w14, w14
        umull   x14, w11, w14
        mul     x15, x10, x11
        umulh   x16, x10, x11
        adds    x4, x4, x14, lsl #33
        lsr     x14, x14, #31
        adc     x5, x5, x14
        adds    x15, x15, x15
        adcs    x16, x16, x16
        adc     x5, x5, xzr
        adds    x3, x3, x15
        adcs    x4, x4, x16
        adc     x5, x5, xzr
        umull   x6, w12, w12
        lsr     x14, x12, #32
        umull   x7, w14, w14
        umull   x14, w12, w14
        adds    x6, x6, x14, lsl #33
        lsr     x14, x14, #31
        adc     x7, x7, x14
        umull   x8, w13, w13
        lsr     x14, x13, #32
        umull   x9, w14, w14
        umull   x14, w13, w14
        mul     x15, x12, x13
        umulh   x16, x12, x13
        adds    x8, x8, x14, lsl #33
        lsr     x14, x14, #31
        adc     x9, x9, x14
        adds    x15, x15, x15
        adcs    x16, x16, x16
        adc     x9, x9, xzr
        adds    x7, x7, x15
        adcs    x8, x8, x16
        adc     x9, x9, xzr
        subs    x10, x10, x12
        sbcs    x11, x11, x13
        csetm   x16, cc
        eor     x10, x10, x16
        subs    x10, x10, x16
        eor     x11, x11, x16
        sbc     x11, x11, x16
        adds    x6, x6, x4
        adcs    x7, x7, x5
        adcs    x8, x8, xzr
        adc     x9, x9, xzr
        umull   x12, w10, w10
        lsr     x5, x10, #32
        umull   x13, w5, w5
        umull   x5, w10, w5
        adds    x12, x12, x5, lsl #33
        lsr     x5, x5, #31
        adc     x13, x13, x5
        umull   x15, w11, w11
        lsr     x5, x11, #32
        umull   x14, w5, w5
        umull   x5, w11, w5
        mul     x4, x10, x11
        umulh   x16, x10, x11
        adds    x15, x15, x5, lsl #33
        lsr     x5, x5, #31
        adc     x14, x14, x5
        adds    x4, x4, x4
        adcs    x16, x16, x16
        adc     x14, x14, xzr
        adds    x13, x13, x4
        adcs    x15, x15, x16
        adc     x14, x14, xzr
        adds    x4, x2, x6
        adcs    x5, x3, x7
        adcs    x6, x6, x8
        adcs    x7, x7, x9
        csetm   x16, cc
        subs    x4, x4, x12
        sbcs    x5, x5, x13
        sbcs    x6, x6, x15
        sbcs    x7, x7, x14
        adcs    x8, x8, x16
        adc     x9, x9, x16

// Now we have the full 8-digit product 2^256 * h + l where
// h = [u7,u6,u5,u4] and l = [u3,u2,u1,u0]
// and this is == 4294968273 * h + l (mod p_256k1)
// Some of the word products are done straightforwardly using mul + umulh
// while others are broken down in a more complicated way as
// (2^32 + 977) * (2^32 * h + l) = 2^64 * h + 2^32 * (d * h + l) + d * l

        mov     d, #977
        orr     c, d, #0x100000000

        mul     a0, c, u4
        umulh   b0, c, u4

        and     l, u5, #0xFFFFFFFF
        lsr     h, u5, #32
        mul     a1, d, l
        madd    l, d, h, l
        adds    a1, a1, l, lsl #32
        lsr     l, l, #32
        adc     b1, h, l

        mul     a2, c, u6
        umulh   b2, c, u6

        and     l, u7, #0xFFFFFFFF
        lsr     h, u7, #32
        mul     a3, d, l
        madd    l, d, h, l
        adds    a3, a3, l, lsl #32
        lsr     l, l, #32
        adc     b3, h, l

        adds    u0, u0, a0
        adcs    u1, u1, a1
        adcs    u2, u2, a2
        adcs    u3, u3, a3
        cset    u4, cs

        adds    u1, u1, b0
        adcs    u2, u2, b1
        adcs    u3, u3, b2
        adc     u4, u4, b3

// Now we have reduced to 5 digits, 2^256 * h + l = [u4,u3,u2,u1,u0]
// Use q = h + 1 as the initial quotient estimate, either right or 1 too big.
// Since q <= 2^33 we do 4294968273 * q = (q<<32) + 977 * q to avoid umulh

        add     q, u4, #1
        mul     a0, d, q
        lsr     a1, q, #32
        adds    a0, a0, q, lsl #32
        adc     a1, xzr, a1
        adds    u0, u0, a0
        adcs    u1, u1, a1
        adcs    u2, u2, xzr
        adcs    u3, u3, xzr

// Now the effective answer is 2^256 * (CF - 1) + [u3,u2,u1,u0]
// So we correct if CF = 0 by subtracting 4294968273, i.e. by
// adding p_256k1 to the "full" answer

        csel    c, c, xzr, cc
        subs    u0, u0, c
        sbcs    u1, u1, xzr
        sbcs    u2, u2, xzr
        sbc     u3, u3, xzr

// Write back

        stp     u0, u1, [x0]
        stp     u2, u3, [x0, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

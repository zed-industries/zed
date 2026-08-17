// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square modulo p_25519, z := (x^2) mod p_25519
// Input x[4]; output z[4]
//
//    extern void bignum_sqr_p25519_alt(uint64_t z[static 4],
//                                      const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_p25519_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_p25519_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_p25519_alt)
        .text
        .balign 4

#define z x0
#define x x1

#define a0 x2
#define a1 x3
#define a2 x4
#define a3 x5

#define h x6
#define l x7

#define u0 x8
#define u1 x9
#define u2 x10
#define u3 x11
#define u4 x12
#define u5 x13
#define u6 x14

// Just aliases

#define q a0
#define c a1
#define t a2
#define u7 h

S2N_BN_SYMBOL(bignum_sqr_p25519_alt):
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

// Now we have the full 8-digit product 2^256 * h + l where
// h = [u7,u6,u5,u4] and l = [u3,u2,u1,u0]
// and this is == 38 * h + l (mod p_25519)

        mov     c, #38

        mul     l, c, u4
        umulh   t, c, u4
        adds    u0, u0, l

        mul     l, c, u5
        umulh   u5, c, u5
        adcs    u1, u1, l

        mul     l, c, u6
        umulh   u6, c, u6
        adcs    u2, u2, l

        mul     l, c, u7
        umulh   u7, c, u7
        adcs    u3, u3, l
        cset    u4, cs

// Compute the top part deferring the [u5,t] addition till the following
// carry chain. This is enough to get a good quotient estimate and saves
// a couple of instructions.

        adds    u3, u3, u6
        adc     u4, u4, u7

// Now we have reduced to 5 digits, 2^255 * H + L = [u4,u3,u2,u1,u0]
// Use q = H + 1 as the initial quotient estimate, either right or 1 too big.

        adds    xzr, u3, u3
        orr     u3, u3, #0x8000000000000000
        adc     q, u4, u4
        mov     c, #19
        madd    l, c, q, c
        adds    u0, u0, l
        adcs    u1, u1, t
        adcs    u2, u2, u5
        adcs    u3, u3, xzr

// Now the effective answer is 2^256 * (CF - 1) + [u3,u2,u1,u0]
// So we correct if CF = 0 by subtracting 19, either way masking to
// 255 bits, i.e. by effectively adding p_25519 to the "full" answer

        csel    c, c, xzr, cc
        subs    u0, u0, c
        sbcs    u1, u1, xzr
        sbcs    u2, u2, xzr
        sbc     u3, u3, xzr
        bic     u3, u3, #0x8000000000000000

// Write back and return

        stp     u0, u1, [x0]
        stp     u2, u3, [x0, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_p25519_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

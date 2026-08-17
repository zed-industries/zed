// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply modulo p_256k1, z := (x * y) mod p_256k1
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_mul_p256k1_alt(uint64_t z[static 4],
//                                      const uint64_t x[static 4],
//                                      const uint64_t y[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_p256k1_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_p256k1_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_p256k1_alt)
        .text
        .balign 4

#define z x0
#define x x1
#define y x2

#define a0 x3
#define a1 x4
#define a2 x5
#define a3 x6
#define b0 x7
#define b1 x8
#define b2 x9
#define b3 x10

#define l x11

#define u0 x12
#define u1 x13
#define u2 x14
#define u3 x15
#define u4 x16

// These alias to the input arguments when no longer needed

#define u5 a0
#define u6 a1
#define u7 a2

#define c b0
#define q b1
#define h b2

S2N_BN_SYMBOL(bignum_mul_p256k1_alt):
        CFI_START

// Load operands and set up row 0 = [u4;...;u0] = a0 * [b3;...;b0]

        ldp     a0, a1, [x]
        ldp     b0, b1, [y]

        mul     u0, a0, b0
        umulh   u1, a0, b0
        mul     l, a0, b1
        umulh   u2, a0, b1
        adds    u1, u1, l

        ldp     b2, b3, [y, #16]

        mul     l, a0, b2
        umulh   u3, a0, b2
        adcs    u2, u2, l

        mul     l, a0, b3
        umulh   u4, a0, b3
        adcs    u3, u3, l
        adc     u4, u4, xzr

        ldp     a2, a3, [x, #16]

// Row 1 = [u5;...;u0] = [a1;a0] * [b3;...;b0]

        mul     l, a1, b0
        adds    u1, u1, l
        mul     l, a1, b1
        adcs    u2, u2, l
        mul     l, a1, b2
        adcs    u3, u3, l
        mul     l, a1, b3
        adcs    u4, u4, l
        umulh   u5, a1, b3
        adc     u5, u5, xzr

        umulh   l, a1, b0
        adds    u2, u2, l
        umulh   l, a1, b1
        adcs    u3, u3, l
        umulh   l, a1, b2
        adcs    u4, u4, l
        adc     u5, u5, xzr

// Row 2 = [u6;...;u0] = [a2;a1;a0] * [b3;...;b0]

        mul     l, a2, b0
        adds    u2, u2, l
        mul     l, a2, b1
        adcs    u3, u3, l
        mul     l, a2, b2
        adcs    u4, u4, l
        mul     l, a2, b3
        adcs    u5, u5, l
        umulh   u6, a2, b3
        adc     u6, u6, xzr

        umulh   l, a2, b0
        adds    u3, u3, l
        umulh   l, a2, b1
        adcs    u4, u4, l
        umulh   l, a2, b2
        adcs    u5, u5, l
        adc     u6, u6, xzr

// Row 3 = [u7;...;u0] = [a3;...a0] * [b3;...;b0]

        mul     l, a3, b0
        adds    u3, u3, l
        mul     l, a3, b1
        adcs    u4, u4, l
        mul     l, a3, b2
        adcs    u5, u5, l
        mul     l, a3, b3
        adcs    u6, u6, l
        umulh   u7, a3, b3
        adc     u7, u7, xzr

        umulh   l, a3, b0
        adds    u4, u4, l
        umulh   l, a3, b1
        adcs    u5, u5, l
        umulh   l, a3, b2
        adcs    u6, u6, l
        adc     u7, u7, xzr

// Now we have the full 8-digit product 2^256 * h + l where
// h = [u7,u6,u5,u4] and l = [u3,u2,u1,u0]
// and this is == 4294968273 * h + l (mod p_256k1)

        mov     c, #977
        orr     c, c, #0x100000000

        mul     l, c, u4
        umulh   h, c, u4
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

        adds    u1, u1, h
        adcs    u2, u2, u5
        adcs    u3, u3, u6
        adc     u4, u4, u7

// Now we have reduced to 5 digits, 2^256 * h + l = [u4,u3,u2,u1,u0]
// Use q = h + 1 as the initial quotient estimate, either right or 1 too big.

        add     q, u4, #1
        mul     l, c, q
        umulh   h, c, q
        adds    u0, u0, l
        adcs    u1, u1, h
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

// Write back and return

        stp     u0, u1, [x0]
        stp     u2, u3, [x0, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_p256k1_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[4], y[4]; output z[8]
//
//    extern void bignum_mul_4_8_alt(uint64_t z[static 8], const uint64_t x[static 4],
//                                   const uint64_t y[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_4_8_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_4_8_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_4_8_alt)
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

#define t x11

#define u0 x12
#define u1 x13
#define u2 x14
#define u3 x15
#define u4 x16

// These alias to the input arguments when no longer needed

#define u5 a0
#define u6 a1
#define u7 a2

S2N_BN_SYMBOL(bignum_mul_4_8_alt):
        CFI_START

// Load operands and set up row 0 = [u4;...;u0] = a0 * [b3;...;b0]

        ldp     a0, a1, [x]
        ldp     b0, b1, [y]

        mul     u0, a0, b0
        umulh   u1, a0, b0
        mul     t, a0, b1
        umulh   u2, a0, b1
        adds    u1, u1, t

        ldp     b2, b3, [y, #16]

        mul     t, a0, b2
        umulh   u3, a0, b2
        adcs    u2, u2, t

        mul     t, a0, b3
        umulh   u4, a0, b3
        adcs    u3, u3, t
        adc     u4, u4, xzr

        ldp     a2, a3, [x, #16]

// Row 1 = [u5;...;u0] = [a1;a0] * [b3;...;b0]

        mul     t, a1, b0
        adds    u1, u1, t
        mul     t, a1, b1
        adcs    u2, u2, t
        mul     t, a1, b2
        adcs    u3, u3, t
        mul     t, a1, b3
        adcs    u4, u4, t
        umulh   u5, a1, b3
        adc     u5, u5, xzr

        umulh   t, a1, b0
        adds    u2, u2, t
        umulh   t, a1, b1
        adcs    u3, u3, t
        umulh   t, a1, b2
        adcs    u4, u4, t
        adc     u5, u5, xzr

// Row 2 = [u6;...;u0] = [a2;a1;a0] * [b3;...;b0]

        mul     t, a2, b0
        adds    u2, u2, t
        mul     t, a2, b1
        adcs    u3, u3, t
        mul     t, a2, b2
        adcs    u4, u4, t
        mul     t, a2, b3
        adcs    u5, u5, t
        umulh   u6, a2, b3
        adc     u6, u6, xzr

        umulh   t, a2, b0
        adds    u3, u3, t
        umulh   t, a2, b1
        adcs    u4, u4, t
        umulh   t, a2, b2
        adcs    u5, u5, t
        adc     u6, u6, xzr

// Row 3 = [u7;...;u0] = [a3;...a0] * [b3;...;b0]

        mul     t, a3, b0
        adds    u3, u3, t
        mul     t, a3, b1
        adcs    u4, u4, t
        mul     t, a3, b2
        adcs    u5, u5, t
        mul     t, a3, b3
        adcs    u6, u6, t
        umulh   u7, a3, b3
        adc     u7, u7, xzr

        umulh   t, a3, b0
        adds    u4, u4, t
        umulh   t, a3, b1
        adcs    u5, u5, t
        umulh   t, a3, b2
        adcs    u6, u6, t
        adc     u7, u7, xzr

// Store back final result [a3;...a0] * [b3;...;b0] = a * b

        stp     u0, u1, [z]
        stp     u2, u3, [z, #16]
        stp     u4, u5, [z, #32]
        stp     u6, u7, [z, #48]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_4_8_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[6], y[6]; output z[12]
//
//    extern void bignum_mul_6_12_alt(uint64_t z[static 12],
//                                    const uint64_t x[static 6],
//                                    const uint64_t y[static 6]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_6_12_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_6_12_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_6_12_alt)
        .text
        .balign 4

#define z x0
#define x x1
#define y x2

// These are repeated mod 2 as we load pairs of inputs

#define a0 x3
#define a1 x4
#define a2 x3
#define a3 x4
#define a4 x3
#define a5 x4

#define b0 x5
#define b1 x6
#define b2 x7
#define b3 x8
#define b4 x9
#define b5 x10

#define t x11

// These repeat mod 8 as we write back

#define u0 x12
#define u1 x13
#define u2 x14
#define u3 x15
#define u4 x16
#define u5 x17
#define u6 x19
#define u7 x20
#define u8 x12
#define u9 x13
#define u10 x14
#define u11 x15

S2N_BN_SYMBOL(bignum_mul_6_12_alt):
        CFI_START

// Save more registers

        CFI_PUSH2(x19,x20)

// Load operands and set up row 0 = [u6;...;u0] = a0 * [b5;...;b0]

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

        ldp     b4, b5, [y, #32]

        mul     t, a0, b4
        umulh   u5, a0, b4
        adcs    u4, u4, t

        mul     t, a0, b5
        umulh   u6, a0, b5
        adcs    u5, u5, t

        adc     u6, u6, xzr

// Row 1 = [u7;...;u0] = [a1;a0] * [b5;...;b0]

        mul     t, a1, b0
        adds    u1, u1, t
        mul     t, a1, b1
        adcs    u2, u2, t
        mul     t, a1, b2
        adcs    u3, u3, t
        mul     t, a1, b3
        adcs    u4, u4, t
        mul     t, a1, b4
        adcs    u5, u5, t
        mul     t, a1, b5
        adcs    u6, u6, t
        cset    u7, cs

        umulh   t, a1, b0
        adds    u2, u2, t
        umulh   t, a1, b1
        adcs    u3, u3, t
        umulh   t, a1, b2
        adcs    u4, u4, t
        umulh   t, a1, b3
        adcs    u5, u5, t
        umulh   t, a1, b4
        adcs    u6, u6, t
        umulh   t, a1, b5
        adc     u7, u7, t

        stp     u0, u1, [z]

// Row 2 = [u8;...;u0] = [a2;a1;a0] * [b5;...;b0]

        ldp     a2, a3, [x, #16]

        mul     t, a2, b0
        adds    u2, u2, t
        mul     t, a2, b1
        adcs    u3, u3, t
        mul     t, a2, b2
        adcs    u4, u4, t
        mul     t, a2, b3
        adcs    u5, u5, t
        mul     t, a2, b4
        adcs    u6, u6, t
        mul     t, a2, b5
        adcs    u7, u7, t
        cset    u8, cs

        umulh   t, a2, b0
        adds    u3, u3, t
        umulh   t, a2, b1
        adcs    u4, u4, t
        umulh   t, a2, b2
        adcs    u5, u5, t
        umulh   t, a2, b3
        adcs    u6, u6, t
        umulh   t, a2, b4
        adcs    u7, u7, t
        umulh   t, a2, b5
        adc     u8, u8, t

// Row 3 = [u9;...;u0] = [a3;a2;a1;a0] * [b5;...;b0]

        mul     t, a3, b0
        adds    u3, u3, t
        mul     t, a3, b1
        adcs    u4, u4, t
        mul     t, a3, b2
        adcs    u5, u5, t
        mul     t, a3, b3
        adcs    u6, u6, t
        mul     t, a3, b4
        adcs    u7, u7, t
        mul     t, a3, b5
        adcs    u8, u8, t
        cset    u9, cs

        umulh   t, a3, b0
        adds    u4, u4, t
        umulh   t, a3, b1
        adcs    u5, u5, t
        umulh   t, a3, b2
        adcs    u6, u6, t
        umulh   t, a3, b3
        adcs    u7, u7, t
        umulh   t, a3, b4
        adcs    u8, u8, t
        umulh   t, a3, b5
        adc     u9, u9, t

        stp     u2, u3, [z, #16]

// Row 4 = [u10;...;u0] = [a4;a3;a2;a1;a0] * [b5;...;b0]

        ldp     a4, a5, [x, #32]

        mul     t, a4, b0
        adds    u4, u4, t
        mul     t, a4, b1
        adcs    u5, u5, t
        mul     t, a4, b2
        adcs    u6, u6, t
        mul     t, a4, b3
        adcs    u7, u7, t
        mul     t, a4, b4
        adcs    u8, u8, t
        mul     t, a4, b5
        adcs    u9, u9, t
        cset    u10, cs

        umulh   t, a4, b0
        adds    u5, u5, t
        umulh   t, a4, b1
        adcs    u6, u6, t
        umulh   t, a4, b2
        adcs    u7, u7, t
        umulh   t, a4, b3
        adcs    u8, u8, t
        umulh   t, a4, b4
        adcs    u9, u9, t
        umulh   t, a4, b5
        adc     u10, u10, t

// Row 5 = [u11;...;u0] = [a5;a4;a3;a2;a1;a0] * [b5;...;b0]

        mul     t, a5, b0
        adds    u5, u5, t
        mul     t, a5, b1
        adcs    u6, u6, t
        mul     t, a5, b2
        adcs    u7, u7, t
        mul     t, a5, b3
        adcs    u8, u8, t
        mul     t, a5, b4
        adcs    u9, u9, t
        mul     t, a5, b5
        adcs    u10, u10, t
        cset    u11, cs

        umulh   t, a5, b0
        adds    u6, u6, t
        umulh   t, a5, b1
        adcs    u7, u7, t
        umulh   t, a5, b2
        adcs    u8, u8, t
        umulh   t, a5, b3
        adcs    u9, u9, t
        umulh   t, a5, b4
        adcs    u10, u10, t
        umulh   t, a5, b5
        adc     u11, u11, t

        stp     u4, u5, [z, #32]

// Store back remaining digits of final result

        stp     u6, u7, [z, #48]
        stp     u8, u9, [z, #64]
        stp     u10, u11, [z, #80]

// Restore registers and return

        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_6_12_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[6]; output z[12]
//
//    extern void bignum_sqr_6_12_alt(uint64_t z[static 12],
//                                    const uint64_t x[static 6]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_6_12_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_6_12_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_6_12_alt)
        .text
        .balign 4

#define z x0
#define x x1

#define a0 x2
#define a1 x3
#define a2 x4
#define a3 x5
#define a4 x6
#define a5 x7

#define l x8

#define u0 x2 // The same as a0, which is safe
#define u1 x9
#define u2 x10
#define u3 x11
#define u4 x12
#define u5 x13
#define u6 x14
#define u7 x15
#define u8 x16
#define u9 x17
#define u10 x19
#define u11 x20

S2N_BN_SYMBOL(bignum_sqr_6_12_alt):
        CFI_START

// It's convenient to have two more registers to play with

        CFI_PUSH2(x19,x20)

// Load all the elements as [a5;a4;a3;a2;a1;a0], set up an initial
// window [u8;u7; u6;u5; u4;u3; u2;u1] = [34;05;03;01], and then
// chain in the addition of 02 + 12 + 13 + 14 + 15 to that window
// (no carry-out possible since we add it to the top of a product).

        ldp     a0, a1, [x]

        mul     u1, a0, a1
        umulh   u2, a0, a1

        ldp     a2, a3, [x, #16]

        mul     l, a0, a2
        adds    u2, u2, l

        mul     u3, a0, a3
        mul     l, a1, a2
        adcs    u3, u3, l

        umulh   u4, a0, a3
        mul     l, a1, a3
        adcs    u4, u4, l

        ldp     a4, a5, [x, #32]

        mul     u5, a0, a5
        mul     l, a1, a4
        adcs    u5, u5, l

        umulh   u6, a0, a5
        mul     l, a1, a5
        adcs    u6, u6, l

        mul     u7, a3, a4
        adcs    u7, u7, xzr

        umulh   u8, a3, a4
        adc     u8, u8, xzr

        umulh   l, a0, a2
        adds    u3, u3, l
        umulh   l, a1, a2
        adcs    u4, u4, l
        umulh   l, a1, a3
        adcs    u5, u5, l
        umulh   l, a1, a4
        adcs    u6, u6, l
        umulh   l, a1, a5
        adcs    u7, u7, l
        adc     u8, u8, xzr

// Now chain in the 04 + 23 + 24 + 25 + 35 + 45 terms

        mul     l, a0, a4
        adds    u4, u4, l
        mul     l, a2, a3
        adcs    u5, u5, l
        mul     l, a2, a4
        adcs    u6, u6, l
        mul     l, a2, a5
        adcs    u7, u7, l
        mul     l, a3, a5
        adcs    u8, u8, l
        mul     u9, a4, a5
        adcs    u9, u9, xzr
        umulh   u10, a4, a5
        adc     u10, u10, xzr

        umulh   l, a0, a4
        adds    u5, u5, l
        umulh   l, a2, a3
        adcs    u6, u6, l
        umulh   l, a2, a4
        adcs    u7, u7, l
        umulh   l, a2, a5
        adcs    u8, u8, l
        umulh   l, a3, a5
        adcs    u9, u9, l
        adc     u10, u10, xzr

// Double that, with h holding the top carry

        adds    u1, u1, u1
        adcs    u2, u2, u2
        adcs    u3, u3, u3
        adcs    u4, u4, u4
        adcs    u5, u5, u5
        adcs    u6, u6, u6
        adcs    u7, u7, u7
        adcs    u8, u8, u8
        adcs    u9, u9, u9
        adcs    u10, u10, u10
        cset    u11, cs

// Add the homogeneous terms 00 + 11 + 22 + 33 + 44 + 55

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
        adcs    u7, u7, l

        mul     l, a4, a4
        adcs    u8, u8, l
        umulh   l, a4, a4
        adcs    u9, u9, l

        mul     l, a5, a5
        adcs    u10, u10, l
        umulh   l, a5, a5
        adc     u11, u11, l

// Store back final result

        stp     u0, u1, [z]
        stp     u2, u3, [z, #16]
        stp     u4, u5, [z, #32]
        stp     u6, u7, [z, #48]
        stp     u8, u9, [z, #64]
        stp     u10, u11, [z, #80]

// Restore registers and return

        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_6_12_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

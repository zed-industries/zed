// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^576) mod p_521
// Input x[9]; output z[9]
//
//    extern void bignum_montsqr_p521_alt(uint64_t z[static 9],
//                                        const uint64_t x[static 9]);
//
// Does z := (x^2 / 2^576) mod p_521, assuming x < p_521. This means the
// Montgomery base is the "native size" 2^{9*64} = 2^576; since p_521 is
// a Mersenne prime the basic modular squaring bignum_sqr_p521 can be
// considered a Montgomery operation to base 2^521.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p521_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_p521_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p521_alt)
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
#define a6 x8
#define a7 x9
#define a8 x1 // Overwrites input argument at last load

#define l x10

#define u0 x11
#define u1 x12
#define u2 x13
#define u3 x14
#define u4 x15
#define u5 x16
#define u6 x17
#define u7 x19
#define u8 x20
#define u9 x21
#define u10 x22
#define u11 x23
#define u12 x24
#define u13 x25
#define u14 x26
#define u15 x27
#define u16 x29

S2N_BN_SYMBOL(bignum_montsqr_p521_alt):
        CFI_START

// It's convenient to have more registers to play with

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_PUSH2(x27,x29)

// Load low 8 elements as [a7;a6;a5;a4;a3;a2;a1;a0], set up an initial
// window [u8;u7;u6;u5;u4;u3;u2;u1] =  10 + 20 + 30 + 40 + 50 + 60 + 70

        ldp     a0, a1, [x]

        mul     u1, a0, a1
        umulh   u2, a0, a1

        ldp     a2, a3, [x, #16]

        mul     l, a0, a2
        umulh   u3, a0, a2
        adds    u2, u2, l

        ldp     a4, a5, [x, #32]

        mul     l, a0, a3
        umulh   u4, a0, a3
        adcs    u3, u3, l

        ldp     a6, a7, [x, #48]

        mul     l, a0, a4
        umulh   u5, a0, a4
        adcs    u4, u4, l

        mul     l, a0, a5
        umulh   u6, a0, a5
        adcs    u5, u5, l

        mul     l, a0, a6
        umulh   u7, a0, a6
        adcs    u6, u6, l

        mul     l, a0, a7
        umulh   u8, a0, a7
        adcs    u7, u7, l

        adc     u8, u8, xzr

// Add in the next diagonal = 21 + 31 + 41 + 51 + 61 + 71 + 54

        mul     l, a1, a2
        adds    u3, u3, l
        mul     l, a1, a3
        adcs    u4, u4, l
        mul     l, a1, a4
        adcs    u5, u5, l
        mul     l, a1, a5
        adcs    u6, u6, l
        mul     l, a1, a6
        adcs    u7, u7, l
        mul     l, a1, a7
        adcs    u8, u8, l
        cset    u9, cs

        umulh   l, a1, a2
        adds    u4, u4, l
        umulh   l, a1, a3
        adcs    u5, u5, l
        umulh   l, a1, a4
        adcs    u6, u6, l
        umulh   l, a1, a5
        adcs    u7, u7, l
        umulh   l, a1, a6
        adcs    u8, u8, l
        umulh   l, a1, a7
        adc     u9, u9, l
        mul     l, a4, a5
        umulh   u10, a4, a5
        adds    u9, u9, l
        adc     u10, u10, xzr

// And the next one = 32 + 42 + 52 + 62 + 72 + 64 + 65

        mul     l, a2, a3
        adds    u5, u5, l
        mul     l, a2, a4
        adcs    u6, u6, l
        mul     l, a2, a5
        adcs    u7, u7, l
        mul     l, a2, a6
        adcs    u8, u8, l
        mul     l, a2, a7
        adcs    u9, u9, l
        mul     l, a4, a6
        adcs    u10, u10, l
        cset    u11, cs

        umulh   l, a2, a3
        adds    u6, u6, l
        umulh   l, a2, a4
        adcs    u7, u7, l
        umulh   l, a2, a5
        adcs    u8, u8, l
        umulh   l, a2, a6
        adcs    u9, u9, l
        umulh   l, a2, a7
        adcs    u10, u10, l
        umulh   l, a4, a6
        adc     u11, u11, l
        mul     l, a5, a6
        umulh   u12, a5, a6
        adds    u11, u11, l
        adc     u12, u12, xzr

// And the final one = 43 + 53 + 63 + 73 + 74 + 75 + 76

        mul     l, a3, a4
        adds    u7, u7, l
        mul     l, a3, a5
        adcs    u8, u8, l
        mul     l, a3, a6
        adcs    u9, u9, l
        mul     l, a3, a7
        adcs    u10, u10, l
        mul     l, a4, a7
        adcs    u11, u11, l
        mul     l, a5, a7
        adcs    u12, u12, l
        cset    u13, cs

        umulh   l, a3, a4
        adds    u8, u8, l
        umulh   l, a3, a5
        adcs    u9, u9, l
        umulh   l, a3, a6
        adcs    u10, u10, l
        umulh   l, a3, a7
        adcs    u11, u11, l
        umulh   l, a4, a7
        adcs    u12, u12, l
        umulh   l, a5, a7
        adc     u13, u13, l
        mul     l, a6, a7
        umulh   u14, a6, a7
        adds    u13, u13, l
        adc     u14, u14, xzr

// Double that, with u15 holding the top carry

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
        adcs    u11, u11, u11
        adcs    u12, u12, u12
        adcs    u13, u13, u13
        adcs    u14, u14, u14
        cset    u15, cs

// Add the homogeneous terms 00 + 11 + 22 + 33 + 44 + 55 + 66 + 77

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
        adcs    u11, u11, l

        mul     l, a6, a6
        adcs    u12, u12, l
        umulh   l, a6, a6
        adcs    u13, u13, l

        mul     l, a7, a7
        adcs    u14, u14, l
        umulh   l, a7, a7
        adc     u15, u15, l

// Now load in the top digit a8, and also set up its double and square

        ldr     a8, [x, #64]
        mul     u16, a8, a8
        add     a8, a8, a8

// Add a8 * [a7;...;a0] into the top of the buffer

        mul     l, a8, a0
        adds    u8, u8, l
        mul     l, a8, a1
        adcs    u9, u9, l
        mul     l, a8, a2
        adcs    u10, u10, l
        mul     l, a8, a3
        adcs    u11, u11, l
        mul     l, a8, a4
        adcs    u12, u12, l
        mul     l, a8, a5
        adcs    u13, u13, l
        mul     l, a8, a6
        adcs    u14, u14, l
        mul     l, a8, a7
        adcs    u15, u15, l
        adc     u16, u16, xzr

        umulh   l, a8, a0
        adds    u9, u9, l
        umulh   l, a8, a1
        adcs    u10, u10, l
        umulh   l, a8, a2
        adcs    u11, u11, l
        umulh   l, a8, a3
        adcs    u12, u12, l
        umulh   l, a8, a4
        adcs    u13, u13, l
        umulh   l, a8, a5
        adcs    u14, u14, l
        umulh   l, a8, a6
        adcs    u15, u15, l
        umulh   l, a8, a7
        adc     u16, u16, l

// Now we have the full product, which we consider as
// 2^521 * h + l. Form h + l + 1

        subs    xzr, xzr, xzr
        extr    l, u9, u8, #9
        adcs    u0, u0, l
        extr    l, u10, u9, #9
        adcs    u1, u1, l
        extr    l, u11, u10, #9
        adcs    u2, u2, l
        extr    l, u12, u11, #9
        adcs    u3, u3, l
        extr    l, u13, u12, #9
        adcs    u4, u4, l
        extr    l, u14, u13, #9
        adcs    u5, u5, l
        extr    l, u15, u14, #9
        adcs    u6, u6, l
        extr    l, u16, u15, #9
        adcs    u7, u7, l
        orr     u8, u8, #~0x1FF
        lsr     l, u16, #9
        adcs    u8, u8, l

// Now CF is set if h + l + 1 >= 2^521, which means it's already
// the answer, while if ~CF the answer is h + l so we should subtract
// 1 (all considered in 521 bits). Hence subtract ~CF and mask.

        sbcs    u0, u0, xzr
        sbcs    u1, u1, xzr
        sbcs    u2, u2, xzr
        sbcs    u3, u3, xzr
        sbcs    u4, u4, xzr
        sbcs    u5, u5, xzr
        sbcs    u6, u6, xzr
        sbcs    u7, u7, xzr
        sbc     u8, u8, xzr
        and     u8, u8, #0x1FF

// So far, this has been the same as a pure modular squaring
// Now finally the Montgomery ingredient, which is just a 521-bit
// rotation by 9*64 - 521 = 55 bits right.

        lsl     l, u0, #9
        extr    u0, u1, u0, #55
        extr    u1, u2, u1, #55
        extr    u2, u3, u2, #55
        extr    u3, u4, u3, #55
        orr     u8, u8, l
        extr    u4, u5, u4, #55
        extr    u5, u6, u5, #55
        extr    u6, u7, u6, #55
        extr    u7, u8, u7, #55
        lsr     u8, u8, #55

// Store back digits of final result

        stp     u0, u1, [z]
        stp     u2, u3, [z, #16]
        stp     u4, u5, [z, #32]
        stp     u6, u7, [z, #48]
        str     u8, [z, #64]

// Restore registers and return

        CFI_POP2(x27,x29)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_p521_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^576) mod p_521
// Inputs x[9], y[9]; output z[9]
//
//    extern void bignum_montmul_p521_alt(uint64_t z[static 9],
//                                        const uint64_t x[static 9],
//                                        const uint64_t y[static 9]);
//
// Does z := (x * y / 2^576) mod p_521, assuming x < p_521, y < p_521. This
// means the Montgomery base is the "native size" 2^{9*64} = 2^576; since
// p_521 is a Mersenne prime the basic modular multiplication bignum_mul_p521
// can be considered a Montgomery operation to base 2^521.
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p521_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_p521_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p521_alt)
        .text
        .balign 4

#define z x0
#define x x1
#define y x2

// These are repeated mod 2 as we load paris of inputs

#define a0 x3
#define a1 x4
#define a2 x3
#define a3 x4
#define a4 x3
#define a5 x4
#define a6 x3
#define a7 x4
#define a8 x3

#define b0 x5
#define b1 x6
#define b2 x7
#define b3 x8
#define b4 x9
#define b5 x10
#define b6 x11
#define b7 x12
#define b8 x13

#define t x14

// These repeat mod 11 as we stash some intermediate results in the
// output buffer.

#define u0 x15
#define u1 x16
#define u2 x17
#define u3 x19
#define u4 x20
#define u5 x21
#define u6 x22
#define u7 x23
#define u8 x24
#define u9 x25
#define u10 x26
#define u11 x15
#define u12 x16
#define u13 x17
#define u14 x19
#define u15 x20
#define u16 x21

S2N_BN_SYMBOL(bignum_montmul_p521_alt):
        CFI_START

// Save more registers and make space for the temporary buffer

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x26)
        CFI_DEC_SP(64)

// Load operands and set up row 0 = [u9;...;u0] = a0 * [b8;...;b0]

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

        ldp     b6, b7, [y, #48]

        mul     t, a0, b6
        umulh   u7, a0, b6
        adcs    u6, u6, t

        ldr     b8, [y, #64]

        mul     t, a0, b7
        umulh   u8, a0, b7
        adcs    u7, u7, t

        mul     t, a0, b8
        umulh   u9, a0, b8
        adcs    u8, u8, t

        adc     u9, u9, xzr

// Row 1 = [u10;...;u0] = [a1;a0] * [b8;...;b0]

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
        mul     t, a1, b6
        adcs    u7, u7, t
        mul     t, a1, b7
        adcs    u8, u8, t
        mul     t, a1, b8
        adcs    u9, u9, t
        cset    u10, cs

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
        adcs    u7, u7, t
        umulh   t, a1, b6
        adcs    u8, u8, t
        umulh   t, a1, b7
        adcs    u9, u9, t
        umulh   t, a1, b8
        adc     u10, u10, t

        stp     u0, u1, [sp]

// Row 2 = [u11;...;u0] = [a2;a1;a0] * [b8;...;b0]

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
        mul     t, a2, b6
        adcs    u8, u8, t
        mul     t, a2, b7
        adcs    u9, u9, t
        mul     t, a2, b8
        adcs    u10, u10, t
        cset    u11, cs

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
        adcs    u8, u8, t
        umulh   t, a2, b6
        adcs    u9, u9, t
        umulh   t, a2, b7
        adcs    u10, u10, t
        umulh   t, a2, b8
        adc     u11, u11, t

// Row 3 = [u12;...;u0] = [a3;a2;a1;a0] * [b8;...;b0]

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
        mul     t, a3, b6
        adcs    u9, u9, t
        mul     t, a3, b7
        adcs    u10, u10, t
        mul     t, a3, b8
        adcs    u11, u11, t
        cset    u12, cs

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
        adcs    u9, u9, t
        umulh   t, a3, b6
        adcs    u10, u10, t
        umulh   t, a3, b7
        adcs    u11, u11, t
        umulh   t, a3, b8
        adc     u12, u12, t

        stp     u2, u3, [sp, #16]

// Row 4 = [u13;...;u0] = [a4;a3;a2;a1;a0] * [b8;...;b0]

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
        mul     t, a4, b6
        adcs    u10, u10, t
        mul     t, a4, b7
        adcs    u11, u11, t
        mul     t, a4, b8
        adcs    u12, u12, t
        cset    u13, cs

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
        adcs    u10, u10, t
        umulh   t, a4, b6
        adcs    u11, u11, t
        umulh   t, a4, b7
        adcs    u12, u12, t
        umulh   t, a4, b8
        adc     u13, u13, t

// Row 5 = [u14;...;u0] = [a5;a4;a3;a2;a1;a0] * [b8;...;b0]

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
        mul     t, a5, b6
        adcs    u11, u11, t
        mul     t, a5, b7
        adcs    u12, u12, t
        mul     t, a5, b8
        adcs    u13, u13, t
        cset    u14, cs

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
        adcs    u11, u11, t
        umulh   t, a5, b6
        adcs    u12, u12, t
        umulh   t, a5, b7
        adcs    u13, u13, t
        umulh   t, a5, b8
        adc     u14, u14, t

        stp     u4, u5, [sp, #32]

// Row 6 = [u15;...;u0] = [a6;a5;a4;a3;a2;a1;a0] * [b8;...;b0]

        ldp     a6, a7, [x, #48]

        mul     t, a6, b0
        adds    u6, u6, t
        mul     t, a6, b1
        adcs    u7, u7, t
        mul     t, a6, b2
        adcs    u8, u8, t
        mul     t, a6, b3
        adcs    u9, u9, t
        mul     t, a6, b4
        adcs    u10, u10, t
        mul     t, a6, b5
        adcs    u11, u11, t
        mul     t, a6, b6
        adcs    u12, u12, t
        mul     t, a6, b7
        adcs    u13, u13, t
        mul     t, a6, b8
        adcs    u14, u14, t
        cset    u15, cs

        umulh   t, a6, b0
        adds    u7, u7, t
        umulh   t, a6, b1
        adcs    u8, u8, t
        umulh   t, a6, b2
        adcs    u9, u9, t
        umulh   t, a6, b3
        adcs    u10, u10, t
        umulh   t, a6, b4
        adcs    u11, u11, t
        umulh   t, a6, b5
        adcs    u12, u12, t
        umulh   t, a6, b6
        adcs    u13, u13, t
        umulh   t, a6, b7
        adcs    u14, u14, t
        umulh   t, a6, b8
        adc     u15, u15, t

// Row 7 = [u16;...;u0] = [a7;a6;a5;a4;a3;a2;a1;a0] * [b8;...;b0]

        mul     t, a7, b0
        adds    u7, u7, t
        mul     t, a7, b1
        adcs    u8, u8, t
        mul     t, a7, b2
        adcs    u9, u9, t
        mul     t, a7, b3
        adcs    u10, u10, t
        mul     t, a7, b4
        adcs    u11, u11, t
        mul     t, a7, b5
        adcs    u12, u12, t
        mul     t, a7, b6
        adcs    u13, u13, t
        mul     t, a7, b7
        adcs    u14, u14, t
        mul     t, a7, b8
        adcs    u15, u15, t
        cset    u16, cs

        umulh   t, a7, b0
        adds    u8, u8, t
        umulh   t, a7, b1
        adcs    u9, u9, t
        umulh   t, a7, b2
        adcs    u10, u10, t
        umulh   t, a7, b3
        adcs    u11, u11, t
        umulh   t, a7, b4
        adcs    u12, u12, t
        umulh   t, a7, b5
        adcs    u13, u13, t
        umulh   t, a7, b6
        adcs    u14, u14, t
        umulh   t, a7, b7
        adcs    u15, u15, t
        umulh   t, a7, b8
        adc     u16, u16, t

        stp     u6, u7, [sp, #48]

// Row 8 = [u16;...;u0] = [a8;a7;a6;a5;a4;a3;a2;a1;a0] * [b8;...;b0]

        ldr     a8, [x, #64]

        mul     t, a8, b0
        adds    u8, u8, t
        mul     t, a8, b1
        adcs    u9, u9, t
        mul     t, a8, b2
        adcs    u10, u10, t
        mul     t, a8, b3
        adcs    u11, u11, t
        mul     t, a8, b4
        adcs    u12, u12, t
        mul     t, a8, b5
        adcs    u13, u13, t
        mul     t, a8, b6
        adcs    u14, u14, t
        mul     t, a8, b7
        adcs    u15, u15, t
        mul     t, a8, b8
        adc     u16, u16, t

        umulh   t, a8, b0
        adds    u9, u9, t
        umulh   t, a8, b1
        adcs    u10, u10, t
        umulh   t, a8, b2
        adcs    u11, u11, t
        umulh   t, a8, b3
        adcs    u12, u12, t
        umulh   t, a8, b4
        adcs    u13, u13, t
        umulh   t, a8, b5
        adcs    u14, u14, t
        umulh   t, a8, b6
        adcs    u15, u15, t
        umulh   t, a8, b7
        adc     u16, u16, t

// Now we have the full product, which we consider as
// 2^521 * h + l. Form h + l + 1

        subs    xzr, xzr, xzr
        ldp     b0, b1, [sp]
        extr    t, u9, u8, #9
        adcs    b0, b0, t
        extr    t, u10, u9, #9
        adcs    b1, b1, t
        ldp     b2, b3, [sp, #16]
        extr    t, u11, u10, #9
        adcs    b2, b2, t
        extr    t, u12, u11, #9
        adcs    b3, b3, t
        ldp     b4, b5, [sp, #32]
        extr    t, u13, u12, #9
        adcs    b4, b4, t
        extr    t, u14, u13, #9
        adcs    b5, b5, t
        ldp     b6, b7, [sp, #48]
        extr    t, u15, u14, #9
        adcs    b6, b6, t
        extr    t, u16, u15, #9
        adcs    b7, b7, t
        orr     b8, u8, #~0x1FF
        lsr     t, u16, #9
        adcs    b8, b8, t

// Now CF is set if h + l + 1 >= 2^521, which means it's already
// the answer, while if ~CF the answer is h + l so we should subtract
// 1 (all considered in 521 bits). Hence subtract ~CF and mask.

        sbcs    b0, b0, xzr
        sbcs    b1, b1, xzr
        sbcs    b2, b2, xzr
        sbcs    b3, b3, xzr
        sbcs    b4, b4, xzr
        sbcs    b5, b5, xzr
        sbcs    b6, b6, xzr
        sbcs    b7, b7, xzr
        sbc     b8, b8, xzr
        and     b8, b8, #0x1FF

// So far, this has been the same as a pure modular multiplication.
// Now finally the Montgomery ingredient, which is just a 521-bit
// rotation by 9*64 - 521 = 55 bits right.

        lsl     t, b0, #9
        extr    b0, b1, b0, #55
        extr    b1, b2, b1, #55
        extr    b2, b3, b2, #55
        extr    b3, b4, b3, #55
        orr     b8, b8, t
        extr    b4, b5, b4, #55
        extr    b5, b6, b5, #55
        extr    b6, b7, b6, #55
        extr    b7, b8, b7, #55
        lsr     b8, b8, #55

// Store back digits of final result

        stp     b0, b1, [z]
        stp     b2, b3, [z, #16]
        stp     b4, b5, [z, #32]
        stp     b6, b7, [z, #48]
        str     b8, [z, #64]

// Restore registers

        CFI_INC_SP(64)
        CFI_POP2(x25,x26)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_p521_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

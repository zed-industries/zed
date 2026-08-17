// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^384) mod p_384
// Inputs x[6], y[6]; output z[6]
//
//    extern void bignum_montmul_p384_alt(uint64_t z[static 6],
//                                        const uint64_t x[static 6],
//                                        const uint64_t y[static 6]);
//
// Does z := (2^{-384} * x * y) mod p_384, assuming that the inputs x and y
// satisfy x * y <= 2^384 * p_384 (in particular this is true if we are in
// the "usual" case x < p_384 and y < p_384).
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p384_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_p384_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p384_alt)
        .text
        .balign 4

// ---------------------------------------------------------------------------
// Core one-step "short" Montgomery reduction macro. Takes input in
// [d5;d4;d3;d2;d1;d0] and returns result in [d6;d5;d4;d3;d2;d1],
// adding to the existing contents of [d5;d4;d3;d2;d1]. It is fine
// for d6 to be the same register as d0.
//
// We want to add (2^384 - 2^128 - 2^96 + 2^32 - 1) * w
// where w = [d0 + (d0<<32)] mod 2^64
// ---------------------------------------------------------------------------

#define montreds(d6,d5,d4,d3,d2,d1,d0, t3,t2,t1)                            \
/* Our correction multiplier is w = [d0 + (d0<<32)] mod 2^64            */  \
/* Store it in d6 to make the 2^384 * w contribution already            */  \
        lsl     t1, d0, #32 __LF                                       \
        add     d6, t1, d0 __LF                                        \
/* Now let [t3;t2;t1;-] = (2^384 - p_384) * w                    */         \
/* We know the lowest word will cancel d0 so we don't need it    */         \
        mov     t1, #0xffffffff00000001 __LF                           \
        umulh   t1, t1, d6 __LF                                        \
        mov     t2, #0x00000000ffffffff __LF                           \
        mul     t3, t2, d6 __LF                                        \
        umulh   t2, t2, d6 __LF                                        \
        adds    t1, t1, t3 __LF                                        \
        adcs    t2, t2, d6 __LF                                        \
        adc     t3, xzr, xzr __LF                                      \
/* Now add it, by subtracting from 2^384 * w + x */                         \
        subs    d1, d1, t1 __LF                                        \
        sbcs    d2, d2, t2 __LF                                        \
        sbcs    d3, d3, t3 __LF                                        \
        sbcs    d4, d4, xzr __LF                                       \
        sbcs    d5, d5, xzr __LF                                       \
        sbc     d6, d6, xzr


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

#define l x11

#define u0 x12
#define u1 x13
#define u2 x14
#define u3 x15
#define u4 x16
#define u5 x17
#define u6 x19
#define u7 x20
#define u8 x21
#define u9 x22
#define u10 x2 // same as y
#define u11 x1 // same as x
#define h b5 // same as b5

S2N_BN_SYMBOL(bignum_montmul_p384_alt):
        CFI_START

// Save more registers

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)

// Load operands and set up row 0 = [u6;...;u0] = a0 * [b5;...;b0]

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

        ldp     b4, b5, [y, #32]

        mul     l, a0, b4
        umulh   u5, a0, b4
        adcs    u4, u4, l

        mul     l, a0, b5
        umulh   u6, a0, b5
        adcs    u5, u5, l

        adc     u6, u6, xzr

// Row 1 = [u7;...;u0] = [a1;a0] * [b5;...;b0]

        mul     l, a1, b0
        adds    u1, u1, l
        mul     l, a1, b1
        adcs    u2, u2, l
        mul     l, a1, b2
        adcs    u3, u3, l
        mul     l, a1, b3
        adcs    u4, u4, l
        mul     l, a1, b4
        adcs    u5, u5, l
        mul     l, a1, b5
        adcs    u6, u6, l
        cset    u7, cs

        umulh   l, a1, b0
        adds    u2, u2, l
        umulh   l, a1, b1
        adcs    u3, u3, l
        umulh   l, a1, b2
        adcs    u4, u4, l
        umulh   l, a1, b3
        adcs    u5, u5, l
        umulh   l, a1, b4
        adcs    u6, u6, l
        umulh   l, a1, b5
        adc     u7, u7, l

// Row 2 = [u8;...;u0] = [a2;a1;a0] * [b5;...;b0]

        ldp     a2, a3, [x, #16]

        mul     l, a2, b0
        adds    u2, u2, l
        mul     l, a2, b1
        adcs    u3, u3, l
        mul     l, a2, b2
        adcs    u4, u4, l
        mul     l, a2, b3
        adcs    u5, u5, l
        mul     l, a2, b4
        adcs    u6, u6, l
        mul     l, a2, b5
        adcs    u7, u7, l
        cset    u8, cs

        umulh   l, a2, b0
        adds    u3, u3, l
        umulh   l, a2, b1
        adcs    u4, u4, l
        umulh   l, a2, b2
        adcs    u5, u5, l
        umulh   l, a2, b3
        adcs    u6, u6, l
        umulh   l, a2, b4
        adcs    u7, u7, l
        umulh   l, a2, b5
        adc     u8, u8, l

// Row 3 = [u9;...;u0] = [a3;a2;a1;a0] * [b5;...;b0]

        mul     l, a3, b0
        adds    u3, u3, l
        mul     l, a3, b1
        adcs    u4, u4, l
        mul     l, a3, b2
        adcs    u5, u5, l
        mul     l, a3, b3
        adcs    u6, u6, l
        mul     l, a3, b4
        adcs    u7, u7, l
        mul     l, a3, b5
        adcs    u8, u8, l
        cset    u9, cs

        umulh   l, a3, b0
        adds    u4, u4, l
        umulh   l, a3, b1
        adcs    u5, u5, l
        umulh   l, a3, b2
        adcs    u6, u6, l
        umulh   l, a3, b3
        adcs    u7, u7, l
        umulh   l, a3, b4
        adcs    u8, u8, l
        umulh   l, a3, b5
        adc     u9, u9, l

// Row 4 = [u10;...;u0] = [a4;a3;a2;a1;a0] * [b5;...;b0]

        ldp     a4, a5, [x, #32]

        mul     l, a4, b0
        adds    u4, u4, l
        mul     l, a4, b1
        adcs    u5, u5, l
        mul     l, a4, b2
        adcs    u6, u6, l
        mul     l, a4, b3
        adcs    u7, u7, l
        mul     l, a4, b4
        adcs    u8, u8, l
        mul     l, a4, b5
        adcs    u9, u9, l
        cset    u10, cs

        umulh   l, a4, b0
        adds    u5, u5, l
        umulh   l, a4, b1
        adcs    u6, u6, l
        umulh   l, a4, b2
        adcs    u7, u7, l
        umulh   l, a4, b3
        adcs    u8, u8, l
        umulh   l, a4, b4
        adcs    u9, u9, l
        umulh   l, a4, b5
        adc     u10, u10, l

// Row 5 = [u11;...;u0] = [a5;a4;a3;a2;a1;a0] * [b5;...;b0]

        mul     l, a5, b0
        adds    u5, u5, l
        mul     l, a5, b1
        adcs    u6, u6, l
        mul     l, a5, b2
        adcs    u7, u7, l
        mul     l, a5, b3
        adcs    u8, u8, l
        mul     l, a5, b4
        adcs    u9, u9, l
        mul     l, a5, b5
        adcs    u10, u10, l
        cset    u11, cs

        umulh   l, a5, b0
        adds    u6, u6, l
        umulh   l, a5, b1
        adcs    u7, u7, l
        umulh   l, a5, b2
        adcs    u8, u8, l
        umulh   l, a5, b3
        adcs    u9, u9, l
        umulh   l, a5, b4
        adcs    u10, u10, l
        umulh   l, a5, b5
        adc     u11, u11, l

// Montgomery rotate the low half

        montreds(u0,u5,u4,u3,u2,u1,u0, b0,b1,b2)
        montreds(u1,u0,u5,u4,u3,u2,u1, b0,b1,b2)
        montreds(u2,u1,u0,u5,u4,u3,u2, b0,b1,b2)
        montreds(u3,u2,u1,u0,u5,u4,u3, b0,b1,b2)
        montreds(u4,u3,u2,u1,u0,u5,u4, b0,b1,b2)
        montreds(u5,u4,u3,u2,u1,u0,u5, b0,b1,b2)

// Add up the high and low parts as [h; u5;u4;u3;u2;u1;u0] = z

        adds    u0, u0, u6
        adcs    u1, u1, u7
        adcs    u2, u2, u8
        adcs    u3, u3, u9
        adcs    u4, u4, u10
        adcs    u5, u5, u11
        adc     h, xzr, xzr

// Now add [h; u11;u10;u9;u8;u7;u6] = z + (2^384 - p_384)

        mov     l, #0xffffffff00000001
        adds    u6, u0, l
        mov     l, #0x00000000ffffffff
        adcs    u7, u1, l
        mov     l, #0x0000000000000001
        adcs    u8, u2, l
        adcs    u9, u3, xzr
        adcs    u10, u4, xzr
        adcs    u11, u5, xzr
        adcs    h, h, xzr

// Now z >= p_384 iff h is nonzero, so select accordingly

        csel    u0, u0, u6, eq
        csel    u1, u1, u7, eq
        csel    u2, u2, u8, eq
        csel    u3, u3, u9, eq
        csel    u4, u4, u10, eq
        csel    u5, u5, u11, eq

// Store back final result

        stp     u0, u1, [z]
        stp     u2, u3, [z, #16]
        stp     u4, u5, [z, #32]

// Restore registers

        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_p384_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

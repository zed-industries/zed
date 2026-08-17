// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^256) mod p_sm2
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_montmul_sm2_alt(uint64_t z[static 4],
//                                       const uint64_t x[static 4],
//                                       const uint64_t y[static 4]);
//
// Does z := (2^{-256} * x * y) mod p_sm2, assuming that the inputs x and y
// satisfy x * y <= 2^256 * p_sm2 (in particular this is true if we are in
// the "usual" case x < p_sm2 and y < p_sm2).
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_sm2_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_sm2_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_sm2_alt)
        .text
        .balign 4

// ---------------------------------------------------------------------------
// Core one-step "short" Montgomery reduction macro. Takes input in
// [d3;d2;d1;d0] and returns result in [d4;d3;d2;d1], adding to the
// existing contents of [d3;d2;d1], and using t0, t1, t2 and t3 as
// temporaries. It is fine for d4 to be the same register as d0,
// and it often is.
// ---------------------------------------------------------------------------

#define montreds(d4,d3,d2,d1,d0, t3,t2,t1,t0)                               \
/* First let [t3;t2] = 2^32 * d0 and [t1;t0] = (2^32-1) * d0             */ \
        lsl     t2, d0, #32 __LF                                               \
        lsr     t3, d0, #32 __LF                                               \
        subs    t0, t2, d0 __LF                                                \
        sbc     t1, t3, xzr __LF                                               \
/* Now [d4;d3;d2;d1] := [d0;d3;d2;d1] - [t3;t2;t1;t0]                    */ \
        subs    d1, d1, t0 __LF                                                \
        sbcs    d2, d2, t1 __LF                                                \
        sbcs    d3, d3, t2 __LF                                                \
        sbc     d4, d0, t3

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
#define h a3

S2N_BN_SYMBOL(bignum_montmul_sm2_alt):
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

// Perform 4 Montgomery steps to rotate the lower half

        montreds(u0,u3,u2,u1,u0, h,l,b0,b1)
        montreds(u1,u0,u3,u2,u1, h,l,b0,b1)
        montreds(u2,u1,u0,u3,u2, h,l,b0,b1)
        montreds(u3,u2,u1,u0,u3, h,l,b0,b1)

// Add high and low parts, catching carry in b1

        adds    u0, u0, u4
        adcs    u1, u1, u5
        adcs    u2, u2, u6
        adcs    u3, u3, u7
        cset    b1, cs

// Set [h;-1;l;-1] = p_sm2 and form [u7,u6,u5,u4] = [b1;u3;u2;u1;u0] - p_sm2

        mov     l, #0xffffffff00000000
        mov     h, #0xfffffffeffffffff

        subs    u4, u0, #-1
        sbcs    u5, u1, l
        adcs    u6, u2, xzr
        sbcs    u7, u3, h
        sbcs    xzr, b1, xzr

// Now CF is clear if the comparison carried so the original was fine
// Otherwise take the form with p_sm2 subtracted.

        csel    u0, u0, u4, cc
        csel    u1, u1, u5, cc
        csel    u2, u2, u6, cc
        csel    u3, u3, u7, cc

// Store back final result

        stp     u0, u1, [z]
        stp     u2, u3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_sm2_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

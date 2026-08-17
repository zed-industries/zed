// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^256) mod p_sm2
// Input x[4]; output z[4]
//
//    extern void bignum_montsqr_sm2_alt(uint64_t z[static 4],
//                                       const uint64_t x[static 4]);
//
// Does z := (x^2 / 2^256) mod p_sm2, assuming x^2 <= 2^256 * p_sm2, which is
// guaranteed in particular if x < p_sm2 initially (the "intended" case).
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_sm2_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_sm2_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_sm2_alt)
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

#define a0 x2
#define a1 x3
#define a2 x4
#define a3 x5

#define l x6
#define h x7

#define u0 x8
#define u1 x9
#define u2 x10
#define u3 x11
#define u4 x12
#define u5 x13
#define u6 x14

// This one is the same as h, which is safe with this computation sequence

#define u7 h

S2N_BN_SYMBOL(bignum_montsqr_sm2_alt):
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

// Squaring complete. Perform 4 Montgomery steps to rotate the lower half

        montreds(u0,u3,u2,u1,u0, a3,a2,a1,a0)
        montreds(u1,u0,u3,u2,u1, a3,a2,a1,a0)
        montreds(u2,u1,u0,u3,u2, a3,a2,a1,a0)
        montreds(u3,u2,u1,u0,u3, a3,a2,a1,a0)

// Add high and low parts, catching carry in a0

        adds    u0, u0, u4
        adcs    u1, u1, u5
        adcs    u2, u2, u6
        adcs    u3, u3, u7
        cset    a0, cs

// Set [a3;-1;a1;-1] = p_sm2 and form [u7,u6,u5,u4] = [a0;u3;u2;u1;u0] - p_sm2

        mov     a1, #0xffffffff00000000
        mov     a3, #0xfffffffeffffffff

        subs    u4, u0, #-1
        sbcs    u5, u1, a1
        adcs    u6, u2, xzr
        sbcs    u7, u3, a3
        sbcs    xzr, a0, xzr

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

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_sm2_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^256) mod p_sm2
// Input x[4]; output z[4]
//
//    extern void bignum_montsqr_sm2(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// Does z := (x^2 / 2^256) mod p_sm2, assuming x^2 <= 2^256 * p_sm2, which is
// guaranteed in particular if x < p_sm2 initially (the "intended" case).
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_sm2)
        .text
        .balign 4

// ---------------------------------------------------------------------------
// Macro returning (c,h,l) = 3-word 1s complement (x - y) * (w - z)
// c,h,l,t should all be different
// t,h should not overlap w,z
// ---------------------------------------------------------------------------

#define muldiffn(c,h,l, t, x,y, w,z)    \
        subs    t, x, y __LF               \
        cneg    t, t, cc __LF              \
        csetm   c, cc __LF                 \
        subs    h, w, z __LF               \
        cneg    h, h, cc __LF              \
        mul     l, t, h __LF               \
        umulh   h, t, h __LF               \
        cinv    c, c, cc __LF              \
        eor     l, l, c __LF               \
        eor     h, h, c

// ---------------------------------------------------------------------------
// Core one-step "end" Montgomery reduction macro. Takes input in
// [d5;d4;d3;d2;d1;d0] and returns result in [d5;d4;d3;d2;d1], adding to
// the existing [d4;d3;d2;d1], re-using d0 as a temporary internally as well
// as t1, t2, t3, and initializing d5 from zero (hence "end").
// ---------------------------------------------------------------------------

#define montrede(d5, d4,d3,d2,d1,d0, t3,t2,t1,t0)                           \
/* First let [t3;t2] = 2^32 * d0 and [t1;t0] = (2^32-1) * d0             */ \
        lsl     t2, d0, #32 __LF                                               \
        lsr     t3, d0, #32 __LF                                               \
        subs    t0, t2, d0 __LF                                                \
        sbc     t1, t3, xzr __LF                                               \
/* Now [d4;d3;d2;d1] := [d0;d3;d2;d1] - [t3;t2;t1;t0]                    */ \
        subs    d1, d1, t0 __LF                                                \
        sbcs    d2, d2, t1 __LF                                                \
        sbcs    d3, d3, t2 __LF                                                \
        sbc     t0, d0, t3 __LF                                                \
        adds    d4, d4, t0 __LF                                                \
        adc     d5, xzr, xzr

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

#define a0 x2
#define a1 x3
#define a2 x4
#define a3 x5

#define c0 x6
#define c1 x7
#define c2 x8
#define c3 x9
#define c4 x10
#define d1 x11
#define d2 x12
#define d3 x13
#define d4 x14

#define s0 x15
#define s1 x16
#define s2 x17
#define s3 x1

#define a0short w2
#define a1short w3
#define d1short w11

S2N_BN_SYMBOL(bignum_montsqr_sm2):
        CFI_START

// Load in all words of the input

        ldp     a0, a1, [x1]
        ldp     a2, a3, [x1, #16]

// Square the low half, getting a result in [s3;s2;s1;s0]
// This uses 32x32->64 multiplications to reduce the number of UMULHs

        umull   s0, a0short, a0short
        lsr     d1, a0, #32
        umull   s1, d1short, d1short
        umull   d1, a0short, d1short
        adds    s0, s0, d1, lsl #33
        lsr     d1, d1, #31
        adc     s1, s1, d1
        umull   s2, a1short, a1short
        lsr     d1, a1, #32
        umull   s3, d1short, d1short
        umull   d1, a1short, d1short
        mul     d2, a0, a1
        umulh   d3, a0, a1
        adds    s2, s2, d1, lsl #33
        lsr     d1, d1, #31
        adc     s3, s3, d1
        adds    d2, d2, d2
        adcs    d3, d3, d3
        adc     s3, s3, xzr
        adds    s1, s1, d2
        adcs    s2, s2, d3
        adc     s3, s3, xzr

// Perform two "short" Montgomery steps on the low square
// This shifts it to an offset compatible with middle product

        montreds(s0,s3,s2,s1,s0, d1,d2,d3,d4)

        montreds(s1,s0,s3,s2,s1, d1,d2,d3,d4)

// Compute cross-product with ADK 2x2->4 multiplier as [c3;c2;c1;c0]

        mul     c0, a0, a2
        mul     d4, a1, a3
        umulh   c2, a0, a2
        muldiffn(d3,d2,d1, c4, a0,a1, a3,a2)

        adds    c1, c0, c2
        adc     c2, c2, xzr

        umulh   c3, a1, a3

        adds    c1, c1, d4
        adcs    c2, c2, c3
        adc     c3, c3, xzr
        adds    c2, c2, d4
        adc     c3, c3, xzr

        adds    xzr, d3, #1
        adcs    c1, c1, d1
        adcs    c2, c2, d2
        adc     c3, c3, d3

// Double it and add the Montgomerified low square

        adds    c0, c0, c0
        adcs    c1, c1, c1
        adcs    c2, c2, c2
        adcs    c3, c3, c3
        adc     c4, xzr, xzr

        adds    c0, c0, s2
        adcs    c1, c1, s3
        adcs    c2, c2, s0
        adcs    c3, c3, s1
        adc     c4, c4, xzr

// Montgomery-reduce the combined low and middle term another twice

        montrede(c0,c4,c3,c2,c1,c0, d1,d2,d3,d4)

        montrede(c1,c0,c4,c3,c2,c1, d1,d2,d3,d4)

// Our sum so far is in [c1,c0,c4,c3,c2]; choose more intuitive names

#define r0 x8
#define r1 x9
#define r2 x10
#define r3 x6
#define c x7

// Remind ourselves what else we can't destroy

#define a2 x4
#define a3 x5

// So we can have these as temps

#define t1 x11
#define t2 x12
#define t3 x13

// Add in the pure squares 22 + 33

        mul     t1, a2, a2
        adds    r0, r0, t1
        mul     t2, a3, a3
        umulh   t1, a2, a2
        adcs    r1, r1, t1
        adcs    r2, r2, t2
        umulh   t2, a3, a3
        adcs    r3, r3, t2
        adc     c, c, xzr

// Construct the 23 term, double and add it in

        mul     t1, a2, a3
        umulh   t2, a2, a3
        adds    t1, t1, t1
        adcs    t2, t2, t2
        adc     t3, xzr, xzr

        adds    r1, r1, t1
        adcs    r2, r2, t2
        adcs    r3, r3, t3
        adcs    c, c, xzr

// We know, writing B = 2^{4*64} that the full implicit result is
// B^2 c <= z + (B - 1) * p < B * p + (B - 1) * p < 2 * B * p,
// so the top half is certainly < 2 * p. If c = 1 already, we know
// subtracting p will give the reduced modulus. But now we do a
// subtraction-comparison to catch cases where the residue is >= p.
// The constants are such that [t3;0;t1;-1] = p_256.

#define t0      x5

// Set CF (because of inversion) iff (0,p_256) <= (c,r3,r2,r1,r0)

        mov     t1, #0xffffffff00000000
        subs    t0, r0, #-1
        sbcs    t1, r1, t1
        mov     t3, #0xfffffffeffffffff
        adcs    t2, r2, xzr
        sbcs    t3, r3, t3
        sbcs    xzr, c, xzr

// Select final output accordingly

        csel    r0, t0, r0, cs
        csel    r1, t1, r1, cs
        csel    r2, t2, r2, cs
        csel    r3, t3, r3, cs

// Store things back in place

        stp     r0, r1, [x0]
        stp     r2, r3, [x0, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

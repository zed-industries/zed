// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^256) mod p_sm2
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_montmul_sm2(uint64_t z[static 4], const uint64_t x[static 4],
//                                   const uint64_t y[static 4]);
//
// Does z := (2^{-256} * x * y) mod p_sm2, assuming that the inputs x and y
// satisfy x * y <= 2^256 * p_sm2 (in particular this is true if we are in
// the "usual" case x < p_sm2 and y < p_sm2).
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_sm2)
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

#define a0 x3
#define a1 x4
#define a2 x5
#define a3 x6
#define b0 x7
#define b1 x8
#define b2 x9
#define b3 x10

#define s0 x11
#define s1 x12
#define s2 x13
#define s3 x14
#define t0 x15
#define t1 x16
#define t2 x17
#define t3 x1
#define s4 x2

S2N_BN_SYMBOL(bignum_montmul_sm2):
        CFI_START

// Load in all words of both inputs

        ldp     a0, a1, [x1]
        ldp     a2, a3, [x1, #16]
        ldp     b0, b1, [x2]
        ldp     b2, b3, [x2, #16]

// Multiply low halves with a 2x2->4 ADK multiplier as L = [s3;s2;s1;s0]

        mul     s0, a0, b0
        mul     s2, a1, b1
        umulh   s1, a0, b0
        adds    t1, s0, s2
        umulh   s3, a1, b1
        adcs    t2, s1, s3
        adcs    s3, s3, xzr
        adds    s1, s1, t1
        adcs    s2, s2, t2
        adcs    s3, s3, xzr
        muldiffn(t3,t2,t1, t0, a0,a1, b1,b0)
        adds    xzr, t3, #1
        adcs    s1, s1, t1
        adcs    s2, s2, t2
        adc     s3, s3, t3

// Perform two "short" Montgomery steps on the low product to
// get a modified low result L' = [s1;s0;s3;s2]
// This shifts it to an offset compatible with middle terms
// Stash the result L' temporarily in the output buffer to avoid
// using additional registers.

        montreds(s0,s3,s2,s1,s0, t0,t1,t2,t3)
        montreds(s1,s0,s3,s2,s1, t0,t1,t2,t3)

        stp     s2, s3, [x0]
        stp     s0, s1, [x0, #16]

// Multiply high halves with a 2x2->4 ADK multiplier as H = [s3;s2;s1;s0]

        mul     s0, a2, b2
        mul     s2, a3, b3
        umulh   s1, a2, b2
        adds    t1, s0, s2
        umulh   s3, a3, b3
        adcs    t2, s1, s3
        adcs    s3, s3, xzr
        adds    s1, s1, t1
        adcs    s2, s2, t2
        adcs    s3, s3, xzr
        muldiffn(t3,t2,t1, t0, a2,a3, b3,b2)
        adds    xzr, t3, #1
        adcs    s1, s1, t1
        adcs    s2, s2, t2
        adc     s3, s3, t3

// Compute sign-magnitude a2,[a1,a0] = x_hi - x_lo

        subs    a0, a2, a0
        sbcs    a1, a3, a1
        sbc     a2, xzr, xzr
        adds    xzr, a2, #1
        eor     a0, a0, a2
        adcs    a0, a0, xzr
        eor     a1, a1, a2
        adcs    a1, a1, xzr

// Compute sign-magnitude b2,[b1,b0] = y_lo - y_hi

        subs    b0, b0, b2
        sbcs    b1, b1, b3
        sbc     b2, xzr, xzr
        adds    xzr, b2, #1
        eor     b0, b0, b2
        adcs    b0, b0, xzr
        eor     b1, b1, b2
        adcs    b1, b1, xzr

// Save the correct sign for the sub-product in b3

        eor     b3, a2, b2

// Add the high H to the modified low term L' as H + L' = [s4;b2;a2;t3;t0]

        ldp     t0, t3, [x0]
        adds    t0, s0, t0
        adcs    t3, s1, t3
        ldp     a2, b2, [x0, #16]
        adcs    a2, s2, a2
        adcs    b2, s3, b2
        adc     s4, xzr, xzr

// Multiply with yet a third 2x2->4 ADK multiplier for complex mid-term M

        mul     s0, a0, b0
        mul     s2, a1, b1
        umulh   s1, a0, b0
        adds    t1, s0, s2
        umulh   s3, a1, b1
        adcs    t2, s1, s3
        adcs    s3, s3, xzr
        adds    s1, s1, t1
        adcs    s2, s2, t2
        adcs    s3, s3, xzr
        muldiffn(a1,t2,t1, a0, a0,a1, b1,b0)
        adds    xzr, a1, #1
        adcs    s1, s1, t1
        adcs    s2, s2, t2
        adc     s3, s3, a1

// Set up a sign-modified version of the mid-product in a long accumulator
// as [b3;a1;a0;s3;s2;s1;s0], adding in the H + L' term once with
// zero offset as this signed value is created

        adds    xzr, b3, #1
        eor     s0, s0, b3
        adcs    s0, s0, t0
        eor     s1, s1, b3
        adcs    s1, s1, t3
        eor     s2, s2, b3
        adcs    s2, s2, a2
        eor     s3, s3, b3
        adcs    s3, s3, b2
        adcs    a0, s4, b3
        adcs    a1, b3, xzr
        adc     b3, b3, xzr

// Add in the stashed H + L' term an offset of 2 words as well

        adds    s2, s2, t0
        adcs    s3, s3, t3
        adcs    a0, a0, a2
        adcs    a1, a1, b2
        adc     b3, b3, s4

// Do two more Montgomery steps on the composed term
// Net pre-reduct is in [b3;a1;a0;s3;s2]

        montreds(s0,s3,s2,s1,s0, t0,t1,t2,t3)
        montreds(s1,s0,s3,s2,s1, t0,t1,t2,t3)

        adds    a0, a0, s0
        adcs    a1, a1, s1
        adc     b3, b3, xzr

// Because of the way we added L' in two places, we can overspill by
// more than usual in Montgomery, with the result being only known to
// be < 3 * p_sm2, not the usual < 2 * p_sm2. So now we do a more
// elaborate final correction in the style of bignum_cmul_sm2, though
// we can use much simpler quotient estimation logic (q = h + 1) and
// slightly more direct accumulation of p_sm2 * q.

#define d0 s2
#define d1 s3
#define d2 a0
#define d3 a1
#define h b3

#define q s4
#define c b0

        add     q, h, #1
        lsl     t0, q, #32
        sub     t1, t0, q
        adds    d0, d0, q
        adcs    d1, d1, t1
        adcs    d2, d2, xzr
        adcs    d3, d3, t0
        csetm   c, cc
        adds    d0, d0, c
        and     t1, c, #0xffffffff00000000
        adcs    d1, d1, t1
        adcs    d2, d2, c
        and     t0, c, #0xfffffffeffffffff
        adc     d3, d3, t0

// Finally store the result

        stp     d0, d1, [x0]
        stp     d2, d3, [x0, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

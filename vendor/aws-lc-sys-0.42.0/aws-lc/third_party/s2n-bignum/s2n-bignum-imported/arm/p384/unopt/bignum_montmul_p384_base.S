// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^384) mod p_384
// Inputs x[6], y[6]; output z[6]
//
//    extern void bignum_montmul_p384_base
//     (uint64_t z[static 6], uint64_t x[static 6], uint64_t y[static 6]);
//
// Does z := (2^{-384} * x * y) mod p_384, assuming that the inputs x and y
// satisfy x * y <= 2^384 * p_384 (in particular this is true if we are in
// the "usual" case x < p_384 and y < p_384).
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p384_base)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p384_base)
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
// [d5;d4;d3;d2;d1;d0] and returns result in [d6;d5;d4;d3;d2;d1],
// adding to the existing contents of [d5;d4;d3;d2;d1]. It is fine
// for d6 to be the same register as d0.
//
// We want to add (2^384 - 2^128 - 2^96 + 2^32 - 1) * w
// where w = [d0 + (d0<<32)] mod 2^64
// ---------------------------------------------------------------------------

#define montreds(d6,d5,d4,d3,d2,d1,d0, t3,t2,t1)                            \
/* Our correction multiplier is w = [d0 + (d0<<32)] mod 2^64            */  \
/* Recycle d0 (which we know gets implicitly cancelled) to store it     */  \
        lsl     t1, d0, #32 __LF                                       \
        add     d0, t1, d0 __LF                                        \
/* Now let [t2;t1] = 2^64 * w - w + w_hi where w_hi = floor(w/2^32)     */  \
/* We need to subtract 2^32 * this, and we can ignore its lower 32      */  \
/* bits since by design it will cancel anyway; we only need the w_hi    */  \
/* part to get the carry propagation going.                             */  \
        lsr     t1, d0, #32 __LF                                       \
        subs    t1, t1, d0 __LF                                        \
        sbc     t2, d0, xzr __LF                                       \
/* Now select in t1 the field to subtract from d1                       */  \
        extr    t1, t2, t1, #32 __LF                                   \
/* And now get the terms to subtract from d2 and d3                     */  \
        lsr     t2, t2, #32 __LF                                       \
        adds    t2, t2, d0 __LF                                        \
        adc     t3, xzr, xzr __LF                                      \
/* Do the subtraction of that portion                                   */  \
        subs    d1, d1, t1 __LF                                        \
        sbcs    d2, d2, t2 __LF                                        \
        sbcs    d3, d3, t3 __LF                                        \
        sbcs    d4, d4, xzr __LF                                       \
        sbcs    d5, d5, xzr __LF                                       \
/* Now effectively add 2^384 * w by taking d0 as the input for last sbc */  \
        sbc     d6, d0, xzr

#define a0 x3
#define a1 x4
#define a2 x5
#define a3 x6
#define a4 x7
#define a5 x8
#define b0 x9
#define b1 x10
#define b2 x11
#define b3 x12
#define b4 x13
#define b5 x14

#define s0 x15
#define s1 x16
#define s2 x17
#define s3 x19
#define s4 x20
#define s5 x1
#define s6 x2

#define t1 x21
#define t2 x22
#define t3 x23
#define t4 x24

S2N_BN_SYMBOL(bignum_montmul_p384_base):

// Save some registers

        stp     x19, x20, [sp, -16]!
        stp     x21, x22, [sp, -16]!
        stp     x23, x24, [sp, -16]!

// Load in all words of both inputs

        ldp     a0, a1, [x1]
        ldp     a2, a3, [x1, #16]
        ldp     a4, a5, [x1, #32]
        ldp     b0, b1, [x2]
        ldp     b2, b3, [x2, #16]
        ldp     b4, b5, [x2, #32]

// Multiply low halves with a 3x3->6 ADK multiplier as [s5;s4;s3;s2;s1;s0]

        mul     s0, a0, b0
        mul     t1, a1, b1
        mul     t2, a2, b2
        umulh   t3, a0, b0
        umulh   t4, a1, b1
        umulh   s5, a2, b2

        adds    t3, t3, t1
        adcs    t4, t4, t2
        adc     s5, s5, xzr

        adds    s1, t3, s0
        adcs    s2, t4, t3
        adcs    s3, s5, t4
        adc     s4, s5, xzr

        adds    s2, s2, s0
        adcs    s3, s3, t3
        adcs    s4, s4, t4
        adc     s5, s5, xzr

        muldiffn(t3,t2,t1, t4, a0,a1, b1,b0)
        adds    xzr, t3, #1
        adcs    s1, s1, t1
        adcs    s2, s2, t2
        adcs    s3, s3, t3
        adcs    s4, s4, t3
        adc     s5, s5, t3

        muldiffn(t3,t2,t1, t4, a0,a2, b2,b0)
        adds    xzr, t3, #1
        adcs    s2, s2, t1
        adcs    s3, s3, t2
        adcs    s4, s4, t3
        adc     s5, s5, t3

        muldiffn(t3,t2,t1, t4, a1,a2, b2,b1)
        adds    xzr, t3, #1
        adcs    s3, s3, t1
        adcs    s4, s4, t2
        adc     s5, s5, t3

// Perform three "short" Montgomery steps on the low product
// This shifts it to an offset compatible with middle terms
// Stash the result temporarily in the output buffer
// We could keep this in registers by directly adding to it in the next
// ADK block, but if anything that seems to be slightly slower

        montreds(s0,s5,s4,s3,s2,s1,s0, t1,t2,t3)

        montreds(s1,s0,s5,s4,s3,s2,s1, t1,t2,t3)

        montreds(s2,s1,s0,s5,s4,s3,s2, t1,t2,t3)

        stp     s3, s4, [x0]
        stp     s5, s0, [x0, #16]
        stp     s1, s2, [x0, #32]

// Multiply high halves with a 3x3->6 ADK multiplier as [s5;s4;s3;s2;s1;s0]

        mul     s0, a3, b3
        mul     t1, a4, b4
        mul     t2, a5, b5
        umulh   t3, a3, b3
        umulh   t4, a4, b4
        umulh   s5, a5, b5

        adds    t3, t3, t1
        adcs    t4, t4, t2
        adc     s5, s5, xzr

        adds    s1, t3, s0
        adcs    s2, t4, t3
        adcs    s3, s5, t4
        adc     s4, s5, xzr

        adds    s2, s2, s0
        adcs    s3, s3, t3
        adcs    s4, s4, t4
        adc     s5, s5, xzr

        muldiffn(t3,t2,t1, t4, a3,a4, b4,b3)
        adds    xzr, t3, #1
        adcs    s1, s1, t1
        adcs    s2, s2, t2
        adcs    s3, s3, t3
        adcs    s4, s4, t3
        adc     s5, s5, t3

        muldiffn(t3,t2,t1, t4, a3,a5, b5,b3)
        adds    xzr, t3, #1
        adcs    s2, s2, t1
        adcs    s3, s3, t2
        adcs    s4, s4, t3
        adc     s5, s5, t3

        muldiffn(t3,t2,t1, t4, a4,a5, b5,b4)
        adds    xzr, t3, #1
        adcs    s3, s3, t1
        adcs    s4, s4, t2
        adc     s5, s5, t3

// Compute sign-magnitude a0,[a5,a4,a3] = x_hi - x_lo

        subs    a3, a3, a0
        sbcs    a4, a4, a1
        sbcs    a5, a5, a2
        sbc     a0, xzr, xzr
        adds    xzr, a0, #1
        eor     a3, a3, a0
        adcs    a3, a3, xzr
        eor     a4, a4, a0
        adcs    a4, a4, xzr
        eor     a5, a5, a0
        adc     a5, a5, xzr

// Compute sign-magnitude b5,[b2,b1,b0] = y_lo - y_hi

        subs    b0, b0, b3
        sbcs    b1, b1, b4
        sbcs    b2, b2, b5
        sbc     b5, xzr, xzr

        adds    xzr, b5, #1
        eor     b0, b0, b5
        adcs    b0, b0, xzr
        eor     b1, b1, b5
        adcs    b1, b1, xzr
        eor     b2, b2, b5
        adc     b2, b2, xzr

// Save the correct sign for the sub-product in b5

        eor     b5, a0, b5

// Add the high H to the modified low term L' and re-stash 6 words,
// keeping top word in s6

        ldp     t1, t2, [x0]
        adds    s0, s0, t1
        adcs    s1, s1, t2
        ldp     t1, t2, [x0, #16]
        adcs    s2, s2, t1
        adcs    s3, s3, t2
        ldp     t1, t2, [x0, #32]
        adcs    s4, s4, t1
        adcs    s5, s5, t2
        adc     s6, xzr, xzr
        stp     s0, s1, [x0]
        stp     s2, s3, [x0, #16]
        stp     s4, s5, [x0, #32]

// Multiply with yet a third 3x3 ADK for the complex mid-term

        mul     s0, a3, b0
        mul     t1, a4, b1
        mul     t2, a5, b2
        umulh   t3, a3, b0
        umulh   t4, a4, b1
        umulh   s5, a5, b2

        adds    t3, t3, t1
        adcs    t4, t4, t2
        adc     s5, s5, xzr

        adds    s1, t3, s0
        adcs    s2, t4, t3
        adcs    s3, s5, t4
        adc     s4, s5, xzr

        adds    s2, s2, s0
        adcs    s3, s3, t3
        adcs    s4, s4, t4
        adc     s5, s5, xzr

        muldiffn(t3,t2,t1, t4, a3,a4, b1,b0)
        adds    xzr, t3, #1
        adcs    s1, s1, t1
        adcs    s2, s2, t2
        adcs    s3, s3, t3
        adcs    s4, s4, t3
        adc     s5, s5, t3

        muldiffn(t3,t2,t1, t4, a3,a5, b2,b0)
        adds    xzr, t3, #1
        adcs    s2, s2, t1
        adcs    s3, s3, t2
        adcs    s4, s4, t3
        adc     s5, s5, t3

        muldiffn(t3,t2,t1, t4, a4,a5, b2,b1)
        adds    xzr, t3, #1
        adcs    s3, s3, t1
        adcs    s4, s4, t2
        adc     s5, s5, t3

// Unstash the H + L' sum to add in twice

        ldp     a0, a1, [x0]
        ldp     a2, a3, [x0, #16]
        ldp     a4, a5, [x0, #32]

// Set up a sign-modified version of the mid-product in a long accumulator
// as [b3;b2;b1;b0;s5;s4;s3;s2;s1;s0], adding in the H + L' term once with
// zero offset as this signed value is created

        adds    xzr, b5, #1
        eor     s0, s0, b5
        adcs    s0, s0, a0
        eor     s1, s1, b5
        adcs    s1, s1, a1
        eor     s2, s2, b5
        adcs    s2, s2, a2
        eor     s3, s3, b5
        adcs    s3, s3, a3
        eor     s4, s4, b5
        adcs    s4, s4, a4
        eor     s5, s5, b5
        adcs    s5, s5, a5
        adcs    b0, b5, s6
        adcs    b1, b5, xzr
        adcs    b2, b5, xzr
        adc     b3, b5, xzr

// Add in the stashed H + L' term an offset of 3 words as well

        adds    s3, s3, a0
        adcs    s4, s4, a1
        adcs    s5, s5, a2
        adcs    b0, b0, a3
        adcs    b1, b1, a4
        adcs    b2, b2, a5
        adc     b3, b3, s6

// Do three more Montgomery steps on the composed term

        montreds(s0,s5,s4,s3,s2,s1,s0, t1,t2,t3)
        montreds(s1,s0,s5,s4,s3,s2,s1, t1,t2,t3)
        montreds(s2,s1,s0,s5,s4,s3,s2, t1,t2,t3)

        adds    b0, b0, s0
        adcs    b1, b1, s1
        adcs    b2, b2, s2
        adc     b3, b3, xzr

// Because of the way we added L' in two places, we can overspill by
// more than usual in Montgomery, with the result being only known to
// be < 3 * p_384, not the usual < 2 * p_384. So now we do a more
// elaborate final correction in the style of bignum_cmul_p384, just
// a little bit simpler because we know q is small.

        add     t2, b3, #1
        lsl     t1, t2, #32
        subs    t4, t2, t1
        sbc     t1, t1, xzr

        adds    s3, s3, t4
        adcs    s4, s4, t1
        adcs    s5, s5, t2
        adcs    b0, b0, xzr
        adcs    b1, b1, xzr
        adcs    b2, b2, xzr

        csetm   t2, cc

        mov     t3, #0x00000000ffffffff
        and     t3, t3, t2
        adds    s3, s3, t3
        eor     t3, t3, t2
        adcs    s4, s4, t3
        mov     t3, #0xfffffffffffffffe
        and     t3, t3, t2
        adcs    s5, s5, t3
        adcs    b0, b0, t2
        adcs    b1, b1, t2
        adc     b2, b2, t2

// Write back the result

        stp     s3, s4, [x0]
        stp     s5, b0, [x0, #16]
        stp     b1, b2, [x0, #32]

// Restore registers and return

        ldp     x23, x24, [sp], #16
        ldp     x21, x22, [sp], #16
        ldp     x19, x20, [sp], #16

        ret

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

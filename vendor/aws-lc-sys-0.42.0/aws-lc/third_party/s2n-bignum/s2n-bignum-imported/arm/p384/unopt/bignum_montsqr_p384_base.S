// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^384) mod p_384
// Input x[6]; output z[6]
//
//    extern void bignum_montsqr_p384_base
//     (uint64_t z[static 6], uint64_t x[static 6]);
//
// Does z := (x^2 / 2^384) mod p_384, assuming x^2 <= 2^384 * p_384, which is
// guaranteed in particular if x < p_384 initially (the "intended" case).
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p384_base)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p384_base)
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

#define a0 x2
#define a1 x3
#define a2 x4
#define a3 x5
#define a4 x6
#define a5 x7

#define c0 x8
#define c1 x9
#define c2 x10
#define c3 x11
#define c4 x12
#define c5 x13
#define d1 x14
#define d2 x15
#define d3 x16
#define d4 x17

S2N_BN_SYMBOL(bignum_montsqr_p384_base):

// Load in all words of the input

        ldp     a0, a1, [x1]
        ldp     a2, a3, [x1, #16]
        ldp     a4, a5, [x1, #32]

// Square the low half getting a result in [c5;c4;c3;c2;c1;c0]

        mul     d1, a0, a1
        mul     d2, a0, a2
        mul     d3, a1, a2
        mul     c0, a0, a0
        mul     c2, a1, a1
        mul     c4, a2, a2

        umulh   d4, a0, a1
        adds    d2, d2, d4
        umulh   d4, a0, a2
        adcs    d3, d3, d4
        umulh   d4, a1, a2
        adcs    d4, d4, xzr

        umulh   c1, a0, a0
        umulh   c3, a1, a1
        umulh   c5, a2, a2

        adds    d1, d1, d1
        adcs    d2, d2, d2
        adcs    d3, d3, d3
        adcs    d4, d4, d4
        adc     c5, c5, xzr

        adds    c1, c1, d1
        adcs    c2, c2, d2
        adcs    c3, c3, d3
        adcs    c4, c4, d4
        adc     c5, c5, xzr

// Perform three "short" Montgomery steps on the low square
// This shifts it to an offset compatible with middle product
// Stash the result temporarily in the output buffer (to avoid more registers)

        montreds(c0,c5,c4,c3,c2,c1,c0, d1,d2,d3)

        montreds(c1,c0,c5,c4,c3,c2,c1, d1,d2,d3)

        montreds(c2,c1,c0,c5,c4,c3,c2, d1,d2,d3)

        stp     c3, c4, [x0]
        stp     c5, c0, [x0, #16]
        stp     c1, c2, [x0, #32]

// Compute product of the cross-term with ADK 3x3->6 multiplier

#define a0 x2
#define a1 x3
#define a2 x4
#define a3 x5
#define a4 x6
#define a5 x7
#define s0 x8
#define s1 x9
#define s2 x10
#define s3 x11
#define s4 x12
#define s5 x13

#define l1 x14
#define l2 x15
#define h0 x16
#define h1 x17
#define h2 x1

#define s6 h1
#define c  l1
#define h  l2
#define l  h0
#define t  h1

        mul     s0, a0, a3
        mul     l1, a1, a4
        mul     l2, a2, a5
        umulh   h0, a0, a3
        umulh   h1, a1, a4
        umulh   h2, a2, a5

        adds    h0, h0, l1
        adcs    h1, h1, l2
        adc     h2, h2, xzr

        adds    s1, h0, s0
        adcs    s2, h1, h0
        adcs    s3, h2, h1
        adc     s4, h2, xzr

        adds    s2, s2, s0
        adcs    s3, s3, h0
        adcs    s4, s4, h1
        adc     s5, h2, xzr

        muldiffn(c,h,l, t, a0,a1, a4,a3)
        adds    xzr, c, #1
        adcs    s1, s1, l
        adcs    s2, s2, h
        adcs    s3, s3, c
        adcs    s4, s4, c
        adc     s5, s5, c

        muldiffn(c,h,l, t, a0,a2, a5,a3)
        adds    xzr, c, #1
        adcs    s2, s2, l
        adcs    s3, s3, h
        adcs    s4, s4, c
        adc     s5, s5, c

        muldiffn(c,h,l, t, a1,a2, a5,a4)
        adds    xzr, c, #1
        adcs    s3, s3, l
        adcs    s4, s4, h
        adc     s5, s5, c

// Double it and add the stashed Montgomerified low square

        adds    s0, s0, s0
        adcs    s1, s1, s1
        adcs    s2, s2, s2
        adcs    s3, s3, s3
        adcs    s4, s4, s4
        adcs    s5, s5, s5
        adc     s6, xzr, xzr

        ldp     a0, a1, [x0]
        adds    s0, s0, a0
        adcs    s1, s1, a1
        ldp     a0, a1, [x0, #16]
        adcs    s2, s2, a0
        adcs    s3, s3, a1
        ldp     a0, a1, [x0, #32]
        adcs    s4, s4, a0
        adcs    s5, s5, a1
        adc     s6, s6, xzr

// Montgomery-reduce the combined low and middle term another thrice

        montreds(s0,s5,s4,s3,s2,s1,s0, a0,a1,a2)

        montreds(s1,s0,s5,s4,s3,s2,s1, a0,a1,a2)

        montreds(s2,s1,s0,s5,s4,s3,s2, a0,a1,a2)

        adds    s6, s6, s0
        adcs    s0, s1, xzr
        adcs    s1, s2, xzr
        adcs    s2, xzr, xzr

// Our sum so far is in [s2;s1;s0;s6;s5;s4;s3]
// Choose more intuitive names

#define r0 x11
#define r1 x12
#define r2 x13
#define r3 x17
#define r4 x8
#define r5 x9
#define r6 x10

// Remind ourselves what else we can't destroy

#define a3 x5
#define a4 x6
#define a5 x7

// So we can have these as temps

#define t1 x1
#define t2 x14
#define t3 x15
#define t4 x16

// Add in all the pure squares 33 + 44 + 55

        mul     t1, a3, a3
        adds    r0, r0, t1
        mul     t2, a4, a4
        mul     t3, a5, a5
        umulh   t1, a3, a3
        adcs    r1, r1, t1
        umulh   t1, a4, a4
        adcs    r2, r2, t2
        adcs    r3, r3, t1
        umulh   t1, a5, a5
        adcs    r4, r4, t3
        adcs    r5, r5, t1
        adc     r6, r6, xzr

// Now compose the 34 + 35 + 45 terms, which need doubling

        mul     t1, a3, a4
        mul     t2, a3, a5
        mul     t3, a4, a5
        umulh   t4, a3, a4
        adds    t2, t2, t4
        umulh   t4, a3, a5
        adcs    t3, t3, t4
        umulh   t4, a4, a5
        adc     t4, t4, xzr

// Double and add. Recycle one of the no-longer-needed inputs as a temp

#define t5 x5

        adds    t1, t1, t1
        adcs    t2, t2, t2
        adcs    t3, t3, t3
        adcs    t4, t4, t4
        adc     t5, xzr, xzr

        adds    r1, r1, t1
        adcs    r2, r2, t2
        adcs    r3, r3, t3
        adcs    r4, r4, t4
        adcs    r5, r5, t5
        adc     r6, r6, xzr

// We know, writing B = 2^{6*64} that the full implicit result is
// B^2 c <= z + (B - 1) * p < B * p + (B - 1) * p < 2 * B * p,
// so the top half is certainly < 2 * p. If c = 1 already, we know
// subtracting p will give the reduced modulus. But now we do a
// comparison to catch cases where the residue is >= p.
// First set [0;0;0;t3;t2;t1] = 2^384 - p_384

        mov     t1, #0xffffffff00000001
        mov     t2, #0x00000000ffffffff
        mov     t3, #0x0000000000000001

// Let dd = [] be the 6-word intermediate result.
// Set CF if the addition dd + (2^384 - p_384) >= 2^384, hence iff dd >= p_384.

        adds    xzr, r0, t1
        adcs    xzr, r1, t2
        adcs    xzr, r2, t3
        adcs    xzr, r3, xzr
        adcs    xzr, r4, xzr
        adcs    xzr, r5, xzr

// Now just add this new carry into the existing r6. It's easy to see they
// can't both be 1 by our range assumptions, so this gives us a {0,1} flag

        adc     r6, r6, xzr

// Now convert it into a bitmask

        sub     r6, xzr, r6

// Masked addition of 2^384 - p_384, hence subtraction of p_384

        and     t1, t1, r6
        adds    r0, r0, t1
        and     t2, t2, r6
        adcs    r1, r1, t2
        and     t3, t3, r6
        adcs    r2, r2, t3
        adcs    r3, r3, xzr
        adcs    r4, r4, xzr
        adc     r5, r5, xzr

// Store it back

        stp     r0, r1, [x0]
        stp     r2, r3, [x0, #16]
        stp     r4, r5, [x0, #32]

        ret

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

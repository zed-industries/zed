// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[6]; output z[12]
//
//    extern void bignum_sqr_6_12(uint64_t z[static 12], const uint64_t x[static 6]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_6_12)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_6_12)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_6_12)
        .text
        .balign 4

// ---------------------------------------------------------------------------
// Macro returning (c,h,l) = 3-word 1s complement (x - y) * (w - z)
// c,h,l,t should all be different
// t,h should not overlap w,z
// ---------------------------------------------------------------------------

.macro muldiffn c,h,l, t, x,y, w,z
        subs    \t, \x, \y
        cneg    \t, \t, cc
        csetm   \c, cc
        subs    \h, \w, \z
        cneg    \h, \h, cc
        mul     \l, \t, \h
        umulh   \h, \t, \h
        cinv    \c, \c, cc
        eor     \l, \l, \c
        eor     \h, \h, \c
.endm

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

S2N_BN_SYMBOL(bignum_sqr_6_12):
        CFI_START

// Load in all words of the input

        ldp     a0, a1, [x1]
        ldp     a2, a3, [x1, #16]
        ldp     a4, a5, [x1, #32]

// Square the low half

        mul     d1, a0, a1
        mul     d2, a0, a2
        mul     d3, a1, a2
        mul     c0, a0, a0
        str     c0, [x0]
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
        str     c1, [x0,#8]
        adcs    c2, c2, d2
        str     c2, [x0,#16]
        adcs    c3, c3, d3
        str     c3, [x0,#24]
        adcs    c4, c4, d4
        str     c4, [x0,#32]
        adc     c5, c5, xzr
        str     c5, [x0,#40]

// Square the high half

        mul     d1, a3, a4
        mul     d2, a3, a5
        mul     d3, a4, a5
        mul     c0, a3, a3
        str     c0, [x0,#48]
        mul     c2, a4, a4
        mul     c4, a5, a5

        umulh   d4, a3, a4
        adds    d2, d2, d4
        umulh   d4, a3, a5
        adcs    d3, d3, d4
        umulh   d4, a4, a5
        adcs    d4, d4, xzr

        umulh   c1, a3, a3
        umulh   c3, a4, a4
        umulh   c5, a5, a5

        adds    d1, d1, d1
        adcs    d2, d2, d2
        adcs    d3, d3, d3
        adcs    d4, d4, d4
        adc     c5, c5, xzr

        adds    c1, c1, d1
        str     c1, [x0,#56]
        adcs    c2, c2, d2
        str     c2, [x0,#64]
        adcs    c3, c3, d3
        str     c3, [x0,#72]
        adcs    c4, c4, d4
        str     c4, [x0,#80]
        adc     c5, c5, xzr
        str     c5, [x0,#88]

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
#define h2 x13

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

        muldiffn c,h,l, t, a0,a1, a4,a3
        adds    xzr, c, #1
        adcs    s1, s1, l
        adcs    s2, s2, h
        adcs    s3, s3, c
        adcs    s4, s4, c
        adc     s5, s5, c

        muldiffn c,h,l, t, a0,a2, a5,a3
        adds    xzr, c, #1
        adcs    s2, s2, l
        adcs    s3, s3, h
        adcs    s4, s4, c
        adc     s5, s5, c

        muldiffn c,h,l, t, a1,a2, a5,a4
        adds    xzr, c, #1
        adcs    s3, s3, l
        adcs    s4, s4, h
        adc     s5, s5, c

// Double it, catching the carry

        adds    s0, s0, s0
        adcs    s1, s1, s1
        adcs    s2, s2, s2
        adcs    s3, s3, s3
        adcs    s4, s4, s4
        adcs    s5, s5, s5
        adc     s6, xzr, xzr

// Finally, add it into the term

        ldr     a0, [x0, #24]
        adds    a0, a0, s0
        str     a0, [x0, #24]

        ldr     a0, [x0, #32]
        adcs    a0, a0, s1
        str     a0, [x0, #32]

        ldr     a0, [x0, #40]
        adcs    a0, a0, s2
        str     a0, [x0, #40]

        ldr     a0, [x0, #48]
        adcs    a0, a0, s3
        str     a0, [x0, #48]

        ldr     a0, [x0, #56]
        adcs    a0, a0, s4
        str     a0, [x0, #56]

        ldr     a0, [x0, #64]
        adcs    a0, a0, s5
        str     a0, [x0, #64]

        ldr     a0, [x0, #72]
        adcs    a0, a0, s6
        str     a0, [x0, #72]

        ldr     a0, [x0, #80]
        adcs    a0, a0, xzr
        str     a0, [x0, #80]

        ldr     a0, [x0, #88]
        adc     a0, a0, xzr
        str     a0, [x0, #88]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_6_12)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

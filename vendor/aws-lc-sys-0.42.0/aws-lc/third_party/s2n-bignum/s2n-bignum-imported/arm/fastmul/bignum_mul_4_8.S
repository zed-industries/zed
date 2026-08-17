// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[4], y[4]; output z[8]
//
//    extern void bignum_mul_4_8(uint64_t z[static 8], const uint64_t x[static 4],
//                               const uint64_t y[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_4_8)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_4_8)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_4_8)
        .text
        .balign 4

#define z x0
#define x x1
#define y x2

#define a0 x3
#define a0short w3
#define a1 x4
#define b0 x5
#define b0short w5
#define b1 x6

#define u0 x7
#define u1 x8
#define u2 x9
#define u3 x10
#define u4 x11
#define u5 x12
#define u6 x13
#define u7 x14

#define t  x15

#define sgn x16
#define ysgn x17

// These are aliases to registers used elsewhere including input pointers.
// By the time they are used this does not conflict with other uses.

#define m0 y
#define m1 ysgn
#define m2 t
#define m3 x
#define u u2

S2N_BN_SYMBOL(bignum_mul_4_8):
        CFI_START

// Multiply the low halves using Karatsuba 2x2->4 to get [u3,u2,u1,u0]
// The zeroth multiplication (only) is done via 32-bit breakdowns

        ldp     a0, a1, [x]
        ldp     b0, b1, [y]

        umull   u0, a0short, b0short
        lsr     x17, a0, #32
        umull   x15, w17, b0short
        lsr     x16, b0, #32
        umull   u1, w16, w17
        umull   x16, a0short, w16
        adds    u0, u0, x15, lsl #32
        lsr     x15, x15, #32
        adc     u1, u1, x15
        adds    u0, u0, x16, lsl #32
        lsr     x16, x16, #32
        adc     u1, u1, x16

        mul     u2, a1, b1
        umulh   u3, a1, b1

        subs    a1, a1, a0
        cneg    a1, a1, cc
        csetm   sgn, cc

        adds    u2, u2, u1
        adc     u3, u3, xzr

        subs    a0, b0, b1
        cneg    a0, a0, cc
        cinv    sgn, sgn, cc

        mul     t, a1, a0
        umulh   a0, a1, a0

        adds    u1, u0, u2
        adcs    u2, u2, u3
        adc     u3, u3, xzr

        adds    xzr, sgn, #1
        eor     t, t, sgn
        adcs    u1, t, u1
        eor     a0, a0, sgn
        adcs    u2, a0, u2
        adc     u3, u3, sgn

// Multiply the high halves using Karatsuba 2x2->4 to get [u7,u6,u5,u4]
// Again, the zeroth multiplication (only) is done via 32-bit breakdowns

        ldp     a0, a1, [x, #16]
        ldp     b0, b1, [y, #16]

        umull   u4, a0short, b0short
        lsr     x17, a0, #32
        umull   x15, w17, b0short
        lsr     x16, b0, #32
        umull   u5, w16, w17
        umull   x16, a0short, w16
        adds    u4, u4, x15, lsl #32
        lsr     x15, x15, #32
        adc     u5, u5, x15
        adds    u4, u4, x16, lsl #32
        lsr     x16, x16, #32
        adc     u5, u5, x16

        mul     u6, a1, b1
        umulh   u7, a1, b1

        subs    a1, a1, a0
        cneg    a1, a1, cc
        csetm   sgn, cc

        adds    u6, u6, u5
        adc     u7, u7, xzr

        subs    a0, b0, b1
        cneg    a0, a0, cc
        cinv    sgn, sgn, cc

        mul     t, a1, a0
        umulh   a0, a1, a0

        adds    u5, u4, u6
        adcs    u6, u6, u7
        adc     u7, u7, xzr

        adds    xzr, sgn, #1
        eor     t, t, sgn
        adcs    u5, t, u5
        eor     a0, a0, sgn
        adcs    u6, a0, u6
        adc     u7, u7, sgn

// Compute  sgn,[a1,a0] = x_hi - x_lo
// and     ysgn,[b1,b0] = y_lo - y_hi
// sign-magnitude differences

        ldp     a0, a1, [x, #16]
        ldp     t, sgn, [x]
        subs    a0, a0, t
        sbcs    a1, a1, sgn
        csetm   sgn, cc

        ldp     t, ysgn, [y]
        subs    b0, t, b0
        sbcs    b1, ysgn, b1
        csetm   ysgn, cc

        eor     a0, a0, sgn
        subs    a0, a0, sgn
        eor     a1, a1, sgn
        sbc     a1, a1, sgn

        eor     b0, b0, ysgn
        subs    b0, b0, ysgn
        eor     b1, b1, ysgn
        sbc     b1, b1, ysgn

// Save the correct sign for the sub-product

        eor     sgn, ysgn, sgn

// Add H' = H + L_top, still in [u7,u6,u5,u4]

        adds    u4, u4, u2
        adcs    u5, u5, u3
        adcs    u6, u6, xzr
        adc     u7, u7, xzr

// Now compute the mid-product as [m3,m2,m1,m0]

        mul     m0, a0, b0
        umulh   m1, a0, b0
        mul     m2, a1, b1
        umulh   m3, a1, b1

        subs    a1, a1, a0
        cneg    a1, a1, cc
        csetm   u, cc

        adds    m2, m2, m1
        adc     m3, m3, xzr

        subs    b1, b0, b1
        cneg    b1, b1, cc
        cinv    u, u, cc

        mul     b0, a1, b1
        umulh   b1, a1, b1

        adds    m1, m0, m2
        adcs    m2, m2, m3
        adc     m3, m3, xzr

        adds    xzr, u, #1
        eor     b0, b0, u
        adcs    m1, b0, m1
        eor     b1, b1, u
        adcs    m2, b1, m2
        adc     m3, m3, u

// Accumulate the positive mid-terms as [u7,u6,u5,u4,u3,u2]

        adds    u2, u4, u0
        adcs    u3, u5, u1
        adcs    u4, u6, u4
        adcs    u5, u7, u5
        adcs    u6, u6, xzr
        adc     u7, u7, xzr

// Add in the sign-adjusted complex term

        adds    xzr, sgn, #1
        eor     m0, m0, sgn
        adcs    u2, m0, u2
        eor     m1, m1, sgn
        adcs    u3, m1, u3
        eor     m2, m2, sgn
        adcs    u4, m2, u4
        eor     m3, m3, sgn
        adcs    u5, m3, u5
        adcs    u6, u6, sgn
        adc     u7, u7, sgn

// Store back the result

        stp     u0, u1, [z]
        stp     u2, u3, [z, #16]
        stp     u4, u5, [z, #32]
        stp     u6, u7, [z, #48]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_4_8)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

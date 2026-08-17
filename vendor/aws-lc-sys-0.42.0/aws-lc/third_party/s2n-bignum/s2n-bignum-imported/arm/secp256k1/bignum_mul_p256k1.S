// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply modulo p_256k1, z := (x * y) mod p_256k1
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_mul_p256k1(uint64_t z[static 4], const uint64_t x[static 4],
//                                  const uint64_t y[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_p256k1)
        .text
        .balign 4

#define z x0
#define x x1
#define y x2

#define a0 x3
#define a1 x4
#define b0 x5
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

// For the reduction stages, again aliasing other things

#define c x1
#define h x2
#define l x15
#define d x16
#define q x17
#define a2 x11
#define a3 x12
#define b2 x13
#define b3 x14

S2N_BN_SYMBOL(bignum_mul_p256k1):
        CFI_START

// Multiply the low halves using Karatsuba 2x2->4 to get [u3,u2,u1,u0]

        ldp     a0, a1, [x]
        ldp     b0, b1, [y]

        mul     u0, a0, b0
        umulh   u1, a0, b0
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

        ldp     a0, a1, [x, #16]
        ldp     b0, b1, [y, #16]

        mul     u4, a0, b0
        umulh   u5, a0, b0
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

// Now we have the full 8-digit product 2^256 * h + l where
// h = [u7,u6,u5,u4] and l = [u3,u2,u1,u0]
// and this is == 4294968273 * h + l (mod p_256k1)
// Some of the word products are done straightforwardly using mul + umulh
// while others are broken down in a more complicated way as
// (2^32 + 977) * (2^32 * h + l) = 2^64 * h + 2^32 * (d * h + l) + d * l

        mov     d, #977
        orr     c, d, #0x100000000

        mul     a0, c, u4
        umulh   b0, c, u4

        and     l, u5, #0xFFFFFFFF
        lsr     h, u5, #32
        mul     a1, d, l
        madd    l, d, h, l
        adds    a1, a1, l, lsl #32
        lsr     l, l, #32
        adc     b1, h, l

        mul     a2, c, u6
        umulh   b2, c, u6

        and     l, u7, #0xFFFFFFFF
        lsr     h, u7, #32
        mul     a3, d, l
        madd    l, d, h, l
        adds    a3, a3, l, lsl #32
        lsr     l, l, #32
        adc     b3, h, l

        adds    u0, u0, a0
        adcs    u1, u1, a1
        adcs    u2, u2, a2
        adcs    u3, u3, a3
        cset    u4, cs

        adds    u1, u1, b0
        adcs    u2, u2, b1
        adcs    u3, u3, b2
        adc     u4, u4, b3

// Now we have reduced to 5 digits, 2^256 * h + l = [u4,u3,u2,u1,u0]
// Use q = h + 1 as the initial quotient estimate, either right or 1 too big.
// Since q <= 2^33 we do 4294968273 * q = (q<<32) + 977 * q to avoid umulh

        add     q, u4, #1
        mul     a0, d, q
        lsr     a1, q, #32
        adds    a0, a0, q, lsl #32
        adc     a1, xzr, a1
        adds    u0, u0, a0
        adcs    u1, u1, a1
        adcs    u2, u2, xzr
        adcs    u3, u3, xzr

// Now the effective answer is 2^256 * (CF - 1) + [u3,u2,u1,u0]
// So we correct if CF = 0 by subtracting 4294968273, i.e. by
// adding p_256k1 to the "full" answer

        csel    c, c, xzr, cc
        subs    u0, u0, c
        sbcs    u1, u1, xzr
        sbcs    u2, u2, xzr
        sbc     u3, u3, xzr

// Write back and return

        stp     u0, u1, [x0]
        stp     u2, u3, [x0, #16]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

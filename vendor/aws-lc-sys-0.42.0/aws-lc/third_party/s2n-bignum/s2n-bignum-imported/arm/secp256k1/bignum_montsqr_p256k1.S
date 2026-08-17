// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^256) mod p_256k1
// Input x[4]; output z[4]
//
//    extern void bignum_montsqr_p256k1(uint64_t z[static 4],
//                                      const uint64_t x[static 4]);
//
// Does z := (x^2 / 2^256) mod p_256k1, assuming x^2 <= 2^256 * p_256k1, which
// is guaranteed in particular if x < p_256k1 initially (the "intended" case).
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p256k1)
        .text
        .balign 4

#define z x0
#define x x1

// Variables

#define u0 x2
#define u1 x3
#define u2 x4
#define u3 x5
#define u4 x6
#define u5 x7
#define u6 x8
#define u7 x9

#define w x10
#define c x11
#define t x12
#define uu x13

S2N_BN_SYMBOL(bignum_montsqr_p256k1):
        CFI_START

// First just a near-clone of bignum_sqr_4_8 to get the square, using
// different registers to collect full product without writeback.

        ldp     u4, u5, [x]
        ldp     x10, x11, [x, #16]
        mul     u2, u4, x10
        mul     u7, u5, x11
        umulh   x12, u4, x10
        subs    x13, u4, u5
        cneg    x13, x13, cc
        csetm   u1, cc
        subs    u0, x11, x10
        cneg    u0, u0, cc
        mul     u6, x13, u0
        umulh   u0, x13, u0
        cinv    u1, u1, cc
        eor     u6, u6, u1
        eor     u0, u0, u1
        adds    u3, u2, x12
        adc     x12, x12, xzr
        umulh   x13, u5, x11
        adds    u3, u3, u7
        adcs    x12, x12, x13
        adc     x13, x13, xzr
        adds    x12, x12, u7
        adc     x13, x13, xzr
        cmn     u1, #0x1
        adcs    u3, u3, u6
        adcs    x12, x12, u0
        adc     x13, x13, u1
        adds    u2, u2, u2
        adcs    u3, u3, u3
        adcs    x12, x12, x12
        adcs    x13, x13, x13
        adc     x14, xzr, xzr
        mul     u0, u4, u4
        mul     u6, u5, u5
        mul     x15, u4, u5
        umulh   u1, u4, u4
        umulh   u7, u5, u5
        umulh   x16, u4, u5
        adds    u1, u1, x15
        adcs    u6, u6, x16
        adc     u7, u7, xzr
        adds    u1, u1, x15
        adcs    u6, u6, x16
        adc     u7, u7, xzr
        adds    u2, u2, u6
        adcs    u3, u3, u7
        adcs    x12, x12, xzr
        adcs    x13, x13, xzr
        adc     x14, x14, xzr
        mul     u4, x10, x10
        mul     u6, x11, x11
        mul     x15, x10, x11
        umulh   u5, x10, x10
        umulh   u7, x11, x11
        umulh   x16, x10, x11
        adds    u5, u5, x15
        adcs    u6, u6, x16
        adc     u7, u7, xzr
        adds    u5, u5, x15
        adcs    u6, u6, x16
        adc     u7, u7, xzr
        adds    u4, u4, x12
        adcs    u5, u5, x13
        adcs    u6, u6, x14
        adc     u7, u7, xzr

// Now we have the full 8-digit product 2^256 * h + l where
// h = [u7,u6,u5,u4] and l = [u3,u2,u1,u0]. Set up constants
// c = 4294968273 so that p_256k1 = 2^256 - c, and w the negated
// multiplicative inverse so that p_256k1 * w == -1 (mod 2^64).

        movz    w, #0x3531
        movk    w, #0xd225, lsl #16
        movk    w, #0x091d, lsl #32
        movk    w, #0xd838, lsl #48
        mov     c, #977
        orr     c, c, #0x100000000

// Do 4 iterations of Montgomery reduction, rotating [u3;u2;u1;u0]

        mul     u0, w, u0
        umulh   t, u0, c
        subs    u1, u1, t

        mul     u1, w, u1
        umulh   t, u1, c
        sbcs    u2, u2, t

        mul     u2, w, u2
        umulh   t, u2, c
        sbcs    u3, u3, t

        mul     u3, w, u3
        umulh   t, u3, c
        sbcs    u0, u0, t

        sbcs    u1, u1, xzr
        sbcs    u2, u2, xzr
        sbc     u3, u3, xzr

// Add the high part and the Montgomery reduced low part

        adds    u0, u0, u4
        adcs    u1, u1, u5
        adcs    u2, u2, u6
        and     uu, u1, u2
        adcs    u3, u3, u7
        and     uu, uu, u3
        cset    t, cs

// Decide whether z >= p_256k1 <=> z + 4294968273 >= 2^256

        adds    xzr, u0, c
        adcs    xzr, uu, xzr
        adcs    t, t, xzr

// Now t <> 0 <=> z >= p_256k1, so mask the constant c accordingly

        csel    c, c, xzr, ne

// If z >= p_256k1 do z := z - p_256k1, i.e. add c in 4 digits

        adds    u0, u0, c
        adcs    u1, u1, xzr
        adcs    u2, u2, xzr
        adc     u3, u3, xzr

// Write back

        stp     u0, u1, [x0]
        stp     u2, u3, [x0, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

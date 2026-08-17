// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^256) mod p_256k1
// Input x[4]; output z[4]
//
//    extern void bignum_montsqr_p256k1_alt(uint64_t z[static 4],
//                                          const uint64_t x[static 4]);
//
// Does z := (x^2 / 2^256) mod p_256k1, assuming x^2 <= 2^256 * p_256k1, which
// is guaranteed in particular if x < p_256k1 initially (the "intended" case).
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p256k1_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_p256k1_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p256k1_alt)
        .text
        .balign 4

#define z x0
#define x x1

#define a0 x2
#define a1 x3
#define a2 x4
#define a3 x5

#define h x6
#define l x7

#define u0 x8
#define u1 x9
#define u2 x10
#define u3 x11
#define u4 x12
#define u5 x13
#define u6 x14
#define u7 x15

// Just aliases (we only use w after loading the inputs)

#define w x
#define t h
#define c a0
#define uu a1

S2N_BN_SYMBOL(bignum_montsqr_p256k1_alt):
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

// Start the Montgomery reductions now to interleave better, though
// conceptually they all happen after the multiplication, only modifying
// any u_i when the multiplication process no longer uses it. Set up
// constants c = 4294968273 so that p_256k1 = 2^256 - c, and w the negated
// multiplicative inverse so that p_256k1 * w == -1 (mod 2^64).
// Precompute a little ahead of the main Montgomery stage.

        movz    w, #0x3531
        movk    w, #0xd225, lsl #16
        movk    w, #0x091d, lsl #32
        movk    w, #0xd838, lsl #48
        mov     c, #977
        orr     c, c, #0x100000000
        mul     u0, w, u0

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

// Now we have the full 8-digit product 2^256 * h + l where
// h = [u7,u6,u5,u4] and l = [u3,u2,u1,u0']. We actually precomputed
// the Montgomery multiplier in u0, but otherwise continue with
// 4 iterations of Montgomery reduction, rotating [u3;u2;u1;u0]

        umulh   l, u0, c
        subs    u1, u1, l

        mul     u1, w, u1
        umulh   l, u1, c
        sbcs    u2, u2, l

        mul     u2, w, u2
        umulh   l, u2, c
        sbcs    u3, u3, l

        mul     u3, w, u3
        umulh   l, u3, c
        sbcs    u0, u0, l

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

        stp     u0, u1, [z]
        stp     u2, u3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_p256k1_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

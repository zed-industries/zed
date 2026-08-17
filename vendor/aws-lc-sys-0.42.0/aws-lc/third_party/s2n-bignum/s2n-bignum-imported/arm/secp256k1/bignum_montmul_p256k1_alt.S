// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^256) mod p_256k1
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_montmul_p256k1_alt(uint64_t z[static 4],
//                                          const uint64_t x[static 4],
//                                          const uint64_t y[static 4]);
//
// Does z := (2^{-256} * x * y) mod p_256k1, assuming that the inputs x and y
// satisfy x * y <= 2^256 * p_256k1 (in particular this is true if we are in
// the "usual" case x < p_256k1 and y < p_256k1).
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p256k1_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_p256k1_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p256k1_alt)
        .text
        .balign 4

#define z x0
#define x x1
#define y x2

#define a0 x3
#define a1 x4
#define a2 x5
#define a3 x6
#define b0 x7
#define b1 x8
#define b2 x9
#define b3 x10

#define l x11

#define u0 x12
#define u1 x13
#define u2 x14
#define u3 x15
#define u4 x16

#define c x17

// These alias to the input arguments when no longer needed

#define u5 a0
#define u6 a1
#define u7 a2

#define w x1
#define t x2
#define uu b3

S2N_BN_SYMBOL(bignum_montmul_p256k1_alt):
        CFI_START

// Load operands and set up row 0 = [u4;...;u0] = a0 * [b3;...;b0]

        ldp     a0, a1, [x]
        ldp     b0, b1, [y]

        mul     u0, a0, b0
        umulh   u1, a0, b0
        mul     l, a0, b1
        umulh   u2, a0, b1
        adds    u1, u1, l

        ldp     b2, b3, [y, #16]

        mul     l, a0, b2
        umulh   u3, a0, b2
        adcs    u2, u2, l

        mul     l, a0, b3
        umulh   u4, a0, b3
        adcs    u3, u3, l
        adc     u4, u4, xzr

        ldp     a2, a3, [x, #16]

// Start the Montgomery reductions now to interleave better, though
// conceptually they all happen after the multiplication, only modifying
// any u_i when the multiplication process no longer uses it. Set up
// constants c = 4294968273 so that p_256k1 = 2^256 - c, and w the negated
// multiplicative inverse so that p_256k1 * w == -1 (mod 2^64).

        movz    w, #0x3531
        movk    w, #0xd225, lsl #16
        movk    w, #0x091d, lsl #32
        movk    w, #0xd838, lsl #48
        mov     c, #977
        orr     c, c, #0x100000000

// Precompute this part ahead of the main Montgomery stage. This
// is a repeated pattern below, since it seems to slightly improve
// dependent latencies.

        mul     u0, w, u0

// Row 1 = [u5;...;u0] = [a1;a0] * [b3;...;b0]

        mul     l, a1, b0
        adds    u1, u1, l
        mul     l, a1, b1
        adcs    u2, u2, l
        mul     l, a1, b2
        adcs    u3, u3, l
        mul     l, a1, b3
        adcs    u4, u4, l
        umulh   u5, a1, b3
        adc     u5, u5, xzr

        umulh   l, a1, b0
        adds    u2, u2, l
        umulh   l, a1, b1
        adcs    u3, u3, l
        umulh   l, a1, b2
        adcs    u4, u4, l
        adc     u5, u5, xzr

// Montgomery stage 0; use t to record the suspended carry

        umulh   l, u0, c
        subs    u1, u1, l
        cset    t, cc

// Row 2 = [u6;...;u0] = [a2;a1;a0] * [b3;...;b0]

        mul     l, a2, b0
        adds    u2, u2, l
        mul     l, a2, b1
        adcs    u3, u3, l
        mul     l, a2, b2
        adcs    u4, u4, l
        mul     l, a2, b3
        adcs    u5, u5, l
        umulh   u6, a2, b3
        adc     u6, u6, xzr

        mul     u1, w, u1

        umulh   l, a2, b0
        adds    u3, u3, l
        umulh   l, a2, b1
        adcs    u4, u4, l
        umulh   l, a2, b2
        adcs    u5, u5, l
        adc     u6, u6, xzr

// Montgomery stage 1

        umulh   l, u1, c
        add     l, l, t
        subs    u2, u2, l
        cset    t, cc

// Row 3 = [u7;...;u0] = [a3;...a0] * [b3;...;b0]

        mul     l, a3, b0
        adds    u3, u3, l
        mul     l, a3, b1
        adcs    u4, u4, l
        mul     l, a3, b2
        adcs    u5, u5, l
        mul     l, a3, b3
        adcs    u6, u6, l
        umulh   u7, a3, b3
        adc     u7, u7, xzr

        mul     u2, w, u2

        umulh   l, a3, b0
        adds    u4, u4, l
        umulh   l, a3, b1
        adcs    u5, u5, l
        umulh   l, a3, b2
        adcs    u6, u6, l
        adc     u7, u7, xzr

// Montgomery stages 2 and 3 (no longer using t to link the carries).

        umulh   l, u2, c
        add     l, l, t
        subs    u3, u3, l
        mul     u3, w, u3
        umulh   l, u3, c
        sbcs    u0, u0, l
        sbcs    u1, u1, xzr
        sbcs    u2, u2, xzr
        sbc     u3, u3, xzr

// Now if a * b = 2^256 * h + l is the full product, we now have
// [u7;u6;u5;u4] = h and 2^256 * [u3;u2;u1;u0] == l (mod p_256k1) because
// of the Montgomery reductions on the low half. Now add the high part
// and the Montgomery-reduced low part.

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

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_p256k1_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

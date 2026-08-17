// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^384) mod p_384
// Input x[6]; output z[6]
//
//    extern void bignum_montsqr_p384_alt(uint64_t z[static 6],
//                                        const uint64_t x[static 6]);
//
// Does z := (x^2 / 2^384) mod p_384, assuming x^2 <= 2^384 * p_384, which is
// guaranteed in particular if x < p_384 initially (the "intended" case).
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montsqr_p384_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montsqr_p384_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montsqr_p384_alt)
        .text
        .balign 4

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
/* Store it in d6 to make the 2^384 * w contribution already            */  \
        lsl     t1, d0, #32 __LF                                       \
        add     d6, t1, d0 __LF                                        \
/* Now let [t3;t2;t1;-] = (2^384 - p_384) * w                    */         \
/* We know the lowest word will cancel d0 so we don't need it    */         \
        mov     t1, #0xffffffff00000001 __LF                           \
        umulh   t1, t1, d6 __LF                                        \
        mov     t2, #0x00000000ffffffff __LF                           \
        mul     t3, t2, d6 __LF                                        \
        umulh   t2, t2, d6 __LF                                        \
        adds    t1, t1, t3 __LF                                        \
        adcs    t2, t2, d6 __LF                                        \
        adc     t3, xzr, xzr __LF                                      \
/* Now add it, by subtracting from 2^384 * w + x */                         \
        subs    d1, d1, t1 __LF                                        \
        sbcs    d2, d2, t2 __LF                                        \
        sbcs    d3, d3, t3 __LF                                        \
        sbcs    d4, d4, xzr __LF                                       \
        sbcs    d5, d5, xzr __LF                                       \
        sbc     d6, d6, xzr

#define z x0
#define x x1

#define a0 x2
#define a1 x3
#define a2 x4
#define a3 x5
#define a4 x6
#define a5 x7

#define l x8

#define u0 x2 // The same as a0, which is safe
#define u1 x9
#define u2 x10
#define u3 x11
#define u4 x12
#define u5 x13
#define u6 x14
#define u7 x15
#define u8 x16
#define u9 x17
#define u10 x19
#define u11 x20
#define h x6 // same as a4

S2N_BN_SYMBOL(bignum_montsqr_p384_alt):
        CFI_START

// It's convenient to have two more registers to play with

        CFI_PUSH2(x19,x20)

// Load all the elements as [a5;a4;a3;a2;a1;a0], set up an initial
// window [u8;u7; u6;u5; u4;u3; u2;u1] = [34;05;03;01], and then
// chain in the addition of 02 + 12 + 13 + 14 + 15 to that window
// (no carry-out possible since we add it to the top of a product).

        ldp     a0, a1, [x]

        mul     u1, a0, a1
        umulh   u2, a0, a1

        ldp     a2, a3, [x, #16]

        mul     l, a0, a2
        adds    u2, u2, l

        mul     u3, a0, a3
        mul     l, a1, a2
        adcs    u3, u3, l

        umulh   u4, a0, a3
        mul     l, a1, a3
        adcs    u4, u4, l

        ldp     a4, a5, [x, #32]

        mul     u5, a0, a5
        mul     l, a1, a4
        adcs    u5, u5, l

        umulh   u6, a0, a5
        mul     l, a1, a5
        adcs    u6, u6, l

        mul     u7, a3, a4
        adcs    u7, u7, xzr

        umulh   u8, a3, a4
        adc     u8, u8, xzr

        umulh   l, a0, a2
        adds    u3, u3, l
        umulh   l, a1, a2
        adcs    u4, u4, l
        umulh   l, a1, a3
        adcs    u5, u5, l
        umulh   l, a1, a4
        adcs    u6, u6, l
        umulh   l, a1, a5
        adcs    u7, u7, l
        adc     u8, u8, xzr

// Now chain in the 04 + 23 + 24 + 25 + 35 + 45 terms

        mul     l, a0, a4
        adds    u4, u4, l
        mul     l, a2, a3
        adcs    u5, u5, l
        mul     l, a2, a4
        adcs    u6, u6, l
        mul     l, a2, a5
        adcs    u7, u7, l
        mul     l, a3, a5
        adcs    u8, u8, l
        mul     u9, a4, a5
        adcs    u9, u9, xzr
        umulh   u10, a4, a5
        adc     u10, u10, xzr

        umulh   l, a0, a4
        adds    u5, u5, l
        umulh   l, a2, a3
        adcs    u6, u6, l
        umulh   l, a2, a4
        adcs    u7, u7, l
        umulh   l, a2, a5
        adcs    u8, u8, l
        umulh   l, a3, a5
        adcs    u9, u9, l
        adc     u10, u10, xzr

// Double that, with u11 holding the top carry

        adds    u1, u1, u1
        adcs    u2, u2, u2
        adcs    u3, u3, u3
        adcs    u4, u4, u4
        adcs    u5, u5, u5
        adcs    u6, u6, u6
        adcs    u7, u7, u7
        adcs    u8, u8, u8
        adcs    u9, u9, u9
        adcs    u10, u10, u10
        cset    u11, cs

// Add the homogeneous terms 00 + 11 + 22 + 33 + 44 + 55

        umulh   l, a0, a0
        mul     u0, a0, a0
        adds    u1, u1, l

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
        adcs    u7, u7, l

        mul     l, a4, a4
        adcs    u8, u8, l
        umulh   l, a4, a4
        adcs    u9, u9, l

        mul     l, a5, a5
        adcs    u10, u10, l
        umulh   l, a5, a5
        adc     u11, u11, l

// Montgomery rotate the low half

        montreds(u0,u5,u4,u3,u2,u1,u0, a1,a2,a3)
        montreds(u1,u0,u5,u4,u3,u2,u1, a1,a2,a3)
        montreds(u2,u1,u0,u5,u4,u3,u2, a1,a2,a3)
        montreds(u3,u2,u1,u0,u5,u4,u3, a1,a2,a3)
        montreds(u4,u3,u2,u1,u0,u5,u4, a1,a2,a3)
        montreds(u5,u4,u3,u2,u1,u0,u5, a1,a2,a3)

// Add up the high and low parts as [h; u5;u4;u3;u2;u1;u0] = z

        adds    u0, u0, u6
        adcs    u1, u1, u7
        adcs    u2, u2, u8
        adcs    u3, u3, u9
        adcs    u4, u4, u10
        adcs    u5, u5, u11
        adc     h, xzr, xzr

// Now add [h; u11;u10;u9;u8;u7;u6] = z + (2^384 - p_384)

        mov     l, #0xffffffff00000001
        adds    u6, u0, l
        mov     l, #0x00000000ffffffff
        adcs    u7, u1, l
        mov     l, #0x0000000000000001
        adcs    u8, u2, l
        adcs    u9, u3, xzr
        adcs    u10, u4, xzr
        adcs    u11, u5, xzr
        adcs    h, h, xzr

// Now z >= p_384 iff h is nonzero, so select accordingly

        csel    u0, u0, u6, eq
        csel    u1, u1, u7, eq
        csel    u2, u2, u8, eq
        csel    u3, u3, u9, eq
        csel    u4, u4, u10, eq
        csel    u5, u5, u11, eq

// Store back final result

        stp     u0, u1, [z]
        stp     u2, u3, [z, #16]
        stp     u4, u5, [z, #32]

// Restore registers

        CFI_POP2(x19,x20)

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montsqr_p384_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

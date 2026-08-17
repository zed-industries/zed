// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square modulo p_25519, z := (x^2) mod p_25519
// Input x[4]; output z[4]
//
//    extern void bignum_sqr_p25519(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_p25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_p25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_p25519)
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

#define u0short w2
#define u1short w3
#define u2short w4
#define u3short w5
#define u4short w6
#define u5short w7
#define u6short w8
#define u7short w9

#define c x10
#define cshort w10
#define l x11
#define lshort w11
#define h x12
#define hshort w12
#define q x13
#define qshort w13

#define t1 x14
#define t1short w14
#define t2 x15
#define t2short w15
#define t3 x16
#define t3short w16

S2N_BN_SYMBOL(bignum_sqr_p25519):
        CFI_START

// First just a near-clone of bignum_sqr_4_8 to get the square, using
// different registers to collect full product without writeback.

        ldp     c, l, [x]
        ldp     h, q, [x, #16]
        umull   u0, cshort, cshort
        lsr     t1, c, #32
        umull   u1, t1short, t1short
        umull   t1, cshort, t1short
        adds    u0, u0, t1, lsl #33
        lsr     t1, t1, #31
        adc     u1, u1, t1
        umull   u2, lshort, lshort
        lsr     t1, l, #32
        umull   u3, t1short, t1short
        umull   t1, lshort, t1short
        mul     t2, c, l
        umulh   t3, c, l
        adds    u2, u2, t1, lsl #33
        lsr     t1, t1, #31
        adc     u3, u3, t1
        adds    t2, t2, t2
        adcs    t3, t3, t3
        adc     u3, u3, xzr
        adds    u1, u1, t2
        adcs    u2, u2, t3
        adc     u3, u3, xzr
        umull   u4, hshort, hshort
        lsr     t1, h, #32
        umull   u5, t1short, t1short
        umull   t1, hshort, t1short
        adds    u4, u4, t1, lsl #33
        lsr     t1, t1, #31
        adc     u5, u5, t1
        umull   u6, qshort, qshort
        lsr     t1, q, #32
        umull   u7, t1short, t1short
        umull   t1, qshort, t1short
        mul     t2, h, q
        umulh   t3, h, q
        adds    u6, u6, t1, lsl #33
        lsr     t1, t1, #31
        adc     u7, u7, t1
        adds    t2, t2, t2
        adcs    t3, t3, t3
        adc     u7, u7, xzr
        adds    u5, u5, t2
        adcs    u6, u6, t3
        adc     u7, u7, xzr
        subs    c, c, h
        sbcs    l, l, q
        csetm   t3, cc
        eor     c, c, t3
        subs    c, c, t3
        eor     l, l, t3
        sbc     l, l, t3
        adds    u4, u4, u2
        adcs    u5, u5, u3
        adcs    u6, u6, xzr
        adc     u7, u7, xzr
        umull   h, cshort, cshort
        lsr     u3, c, #32
        umull   q, u3short, u3short
        umull   u3, cshort, u3short
        adds    h, h, u3, lsl #33
        lsr     u3, u3, #31
        adc     q, q, u3
        umull   t2, lshort, lshort
        lsr     u3, l, #32
        umull   t1, u3short, u3short
        umull   u3, lshort, u3short
        mul     u2, c, l
        umulh   t3, c, l
        adds    t2, t2, u3, lsl #33
        lsr     u3, u3, #31
        adc     t1, t1, u3
        adds    u2, u2, u2
        adcs    t3, t3, t3
        adc     t1, t1, xzr
        adds    q, q, u2
        adcs    t2, t2, t3
        adc     t1, t1, xzr
        adds    u2, u0, u4
        adcs    u3, u1, u5
        adcs    u4, u4, u6
        adcs    u5, u5, u7
        csetm   t3, cc
        subs    u2, u2, h
        sbcs    u3, u3, q
        sbcs    u4, u4, t2
        sbcs    u5, u5, t1
        adcs    u6, u6, t3
        adc     u7, u7, t3

// Now we have the full 8-digit product 2^256 * h + l where
// h = [u7,u6,u5,u4] and l = [u3,u2,u1,u0]
// and this is == 38 * h + l (mod p_25519).
// We do the 38 * h + l using 32-bit multiplies avoiding umulh,
// and pre-estimate and feed in the next-level quotient
// q = h + 1 where h = an early version of the high 255 bits.
// We add 2^255 * h - 19 * (h + 1), so end up offset by 2^255.

        mov     c, #38

        umull   h, u4short, cshort
        add     h, h, u0short, uxtw
        lsr     u0, u0, #32
        lsr     u4, u4, #32
        umaddl  u4, u4short, cshort, u0
        mov     u0, h

        umull   h, u5short, cshort
        add     h, h, u1short, uxtw
        lsr     u1, u1, #32
        lsr     u5, u5, #32
        umaddl  u5, u5short, cshort, u1
        mov     u1, h

        umull   h, u6short, cshort
        add     h, h, u2short, uxtw
        lsr     u2, u2, #32
        lsr     u6, u6, #32
        umaddl  u6, u6short, cshort, u2
        mov     u2, h

        umull   h, u7short, cshort
        add     h, h, u3short, uxtw
        lsr     u3, u3, #32
        lsr     u7, u7, #32
        umaddl  u7, u7short, cshort, u3
        mov     u3, h

        lsr     q, u7, #31

        mov     l, #19
        umaddl  l, lshort, qshort, l
        add     u0, u0, l

        adds    u0, u0, u4, lsl #32
        extr    c, u5, u4, #32
        adcs    u1, u1, c
        extr    c, u6, u5, #32
        adcs    u2, u2, c
        extr    c, u7, u6, #32
        lsl     l, q, #63
        eor     u3, u3, l
        adc     u3, u3, c

// Now we correct by a final 2^255-19 if the top bit is clear
// meaning that the "real" pre-reduced result is negative.

        mov     c, #19
        tst     u3, #0x8000000000000000
        csel    c, c, xzr, pl
        subs    u0, u0, c
        sbcs    u1, u1, xzr
        sbcs    u2, u2, xzr
        sbc     u3, u3, xzr
        and     u3, u3, #~0x8000000000000000

// Write back result

        stp     u0, u1, [x0]
        stp     u2, u3, [x0, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

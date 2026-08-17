// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[4]; output z[8]
//
//    extern void bignum_sqr_4_8(uint64_t z[static 8], const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_4_8)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_4_8)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_4_8)
        .text
        .balign 4

// ---------------------------------------------------------------------------
// 2x2 squaring macro: [s3;s2;s1;s0] := [a1;a0]^2 with t0,t1,t2 temporaries
// This uses 32x32->64 multiplications to reduce the number of UMULHs
// ---------------------------------------------------------------------------

#define sqr2(s3,s2,s1,s0, a1,a1short,a0,a0short, t2,t1,t0,t0short) \
        umull   s0, a0short, a0short __LF          \
        lsr     t0, a0, #32 __LF                   \
        umull   s1, t0short, t0short __LF          \
        umull   t0, a0short, t0short __LF          \
        adds    s0, s0, t0, lsl #33 __LF           \
        lsr     t0, t0, #31 __LF                   \
        adc     s1, s1, t0 __LF                    \
        umull   s2, a1short, a1short __LF          \
        lsr     t0, a1, #32 __LF                   \
        umull   s3, t0short, t0short __LF          \
        umull   t0, a1short, t0short __LF          \
        mul     t1, a0, a1 __LF                    \
        umulh   t2, a0, a1 __LF                    \
        adds    s2, s2, t0, lsl #33 __LF           \
        lsr     t0, t0, #31 __LF                   \
        adc     s3, s3, t0 __LF                    \
        adds    t1, t1, t1 __LF                    \
        adcs    t2, t2, t2 __LF                    \
        adc     s3, s3, xzr __LF                   \
        adds    s1, s1, t1 __LF                    \
        adcs    s2, s2, t2 __LF                    \
        adc     s3, s3, xzr

// Main code

#define z x0
#define x x1

#define a0 x2
#define a1 x3
#define a2 x4
#define a3 x5

#define s0 x6
#define s1 x7
#define s2 x8
#define s3 x9
#define s4 x10
#define s5 x11
#define s6 x12
#define s7 x13

#define d0 x14
#define d1 x15
#define d2 x16

// Short versions

#define a0short w2
#define a1short w3
#define a2short w4
#define a3short w5
#define d2short w16
#define s3short w9

S2N_BN_SYMBOL(bignum_sqr_4_8):
        CFI_START

// Load all the elements

        ldp     a0, a1, [x]
        ldp     a2, a3, [x, #16]

// Compute L = [s3;s2;s1;s0] = square of lower half

        sqr2(s3,s2,s1,s0, a1,a1short,a0,a0short, d0,d1,d2,d2short)

// Compute H = [s7;s6;s5;s4] = square of upper half

        sqr2(s7,s6,s5,s4, a3,a3short,a2,a2short, d0,d1,d2,d2short)

// Let [a1;a0] = |[a3;a2] - [a1;a0]| be the absolute difference

        subs    a0, a0, a2
        sbcs    a1, a1, a3
        csetm   d0, cc
        eor     a0, a0, d0
        subs    a0, a0, d0
        eor     a1, a1, d0
        sbc     a1, a1, d0

// Form H' = H + L_hi (which fits in 4 words)

        adds    s4, s4, s2
        adcs    s5, s5, s3
        adcs    s6, s6, xzr
        adc     s7, s7, xzr

// Let M = [d2;d1;a3;a2] = ([a3;a2] - [a1;a0])^2

        sqr2(d2,d1,a3,a2, a1,a1short,a0,a0short, d0,s2,s3,s3short)

// Now form (2^64 + 1) * (H'::L), with a bit of carry-shortening

        adds    s2, s0, s4
        adcs    s3, s1, s5
        adcs    s4, s4, s6
        adcs    s5, s5, s7
        csetm   d0, cc

// Subtract the middle term M

        subs    s2, s2, a2
        sbcs    s3, s3, a3
        sbcs    s4, s4, d1
        sbcs    s5, s5, d2
        adcs    s6, s6, d0
        adc     s7, s7, d0

// Store back

        stp     s0, s1, [z]
        stp     s2, s3, [z, 16]
        stp     s4, s5, [z, 32]
        stp     s6, s7, [z, 48]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_4_8)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

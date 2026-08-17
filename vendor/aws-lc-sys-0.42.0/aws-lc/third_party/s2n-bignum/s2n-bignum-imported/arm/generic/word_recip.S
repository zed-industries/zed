// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Single-word reciprocal, underestimate of floor(2^128 / a) - 2^64
// Input a; output function return
//
//    extern uint64_t word_recip(uint64_t a);
//
// Given an input word "a" with its top bit set (i.e. 2^63 <= a < 2^64), the
// result "x" is implicitly augmented with a leading 1 giving x' = 2^64 + x.
// The result is x' = ceil(2^128 / a) - 1, which except for the single
// special case a = 2^63 is the same thing as x' = floor(2^128 / a).
//
// Standard ARM ABI: X0 = a, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(word_recip)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(word_recip)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(word_recip)
        .text
        .balign 4

#define a x0
#define x x1

// Some of these are aliased for clarity

#define b x2

#define t x3
#define l x3

#define d x4
#define h x4

S2N_BN_SYMBOL(word_recip):
        CFI_START

// Scale the input down: b overestimates a/2^16 with b <= 2^48 and
// x underestimates 2^64/b with b * x =~= 2^64, accurate to ~2 bits.

        lsr     b, a, #16
        eor     x, b, #0x1FFFFFFFFFFFF
        add     b, b, #1
        lsr     x, x, #32

// Suppose x = 2^64/b * (1 - e). and get scaled error d = 2^64 * e

        msub    d, b, x, xzr

// Rescale to give c = 2^15 * e (so c <= 2^13) and compute
// e + e^2 + e^3 + e^4 = (1 + e^2) (e + e^2)
// = (2^30 + c^2) * (2^15 * c + c^2) / 2^60
// and then x * (1 + e + e^2 + e^3 + e^4)
// = (2^30 * x + x * (2^30 + c^2) * (2^30 * c + c^2) / 2^30) / 2^30

        lsr     t, d, #49
        mul     t, t, t
        lsr     d, d, #34
        add     d, t, d
        orr     t, t, #0x40000000
        mul     t, d, t
        lsr     t, t, #30
        lsl     d, x, #30
        madd    x, x, t, d
        lsr     x, x, #30

// Now b * x =~= 2^64, accurate to ~10 bits.
// Do a 64-bit Newton step, scaling up x by 16 bits in the process.

        msub    d, b, x, xzr
        lsr     d, d, #24
        mul     d, d, x
        lsl     x, x, #16
        lsr     d, d, #24
        add     x, x, d

// Now b * x =~= 2^80, accurate to ~20 bits.
// Do a 64-bit Newton step, scaling up x by 31 bits in the process

        msub    d, b, x, xzr
        lsr     d, d, #32
        mul     d, d, x
        lsl     x, x, #31
        lsr     d, d, #17
        add     x, x, d

// Now a * x =~= 2^127, accurate to ~40 bits. Do a Newton step at full size.
// Instead of literally negating the product (h,l) we complement bits in
// the extracted bitfield, which is close enough and a bit faster.
// At the end we also shift x one more bit left, losing the known-1 top bit
// so that a * (2^64 + x) =~= 2^128.

        mul     l, a, x
        umulh   h, a, x
        extr    l, h, l, #60
        lsr     h, x, #33
        mvn     l, l
        mul     l, h, l
        lsl     x, x, #1
        lsr     l, l, #33
        add     x, x, l

// Test if (x' + 1) * a < 2^128 where x' = 2^64 + x, catching the special
// case where x + 1 would wrap, corresponding to input a = 2^63.

        adds    t, x, #1
        cinv    t, t, eq
        umulh   h, a, t
        adds    h, h, a

// Select either x or x + 1 accordingly as the final answer

        csel    x0, x, t, cs
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(word_recip)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_521, z := (c * x) mod p_521, assuming
// x reduced
// Inputs c, x[9]; output z[9]
//
//    extern void bignum_cmul_p521(uint64_t z[static 9], uint64_t c,
//                                 const uint64_t x[static 9]);
//
// Standard ARM ABI: X0 = z, X1 = c, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p521)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p521_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p521_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p521_alt)
        .text
        .balign 4

#define z x0
#define c x1
#define x x2

#define d0 x3
#define d1 x4
#define d2 x5
#define d3 x6
#define d4 x7
#define d5 x8
#define d6 x9
#define d7 x10
#define d8 x11
#define d9 x12

// Heavily aliased subject to ordering

#define a0 d3
#define a1 d4
#define a2 d5
#define a3 d6
#define a4 d7
#define a5 d8
#define a6 d9
#define h d9

// Other variables

#define a7 x13
#define a8 x14
#define dd x15

S2N_BN_SYMBOL(bignum_cmul_p521):

S2N_BN_SYMBOL(bignum_cmul_p521_alt):
        CFI_START

// First do the multiply, getting [d9; ...; d0], and as this is done
// accumulate an AND "dd" of digits d7,...,d1 for later use

        ldp     a0, a1, [x]
        mul     d0, c, a0
        mul     d1, c, a1
        umulh   a0, c, a0
        adds    d1, d1, a0
        umulh   a1, c, a1

        ldp     a2, a3, [x, #16]
        mul     d2, c, a2
        mul     d3, c, a3
        umulh   a2, c, a2
        adcs    d2, d2, a1
        and     dd, d1, d2
        umulh   a3, c, a3
        adcs    d3, d3, a2
        and     dd, dd, d3

        ldp     a4, a5, [x, #32]
        mul     d4, c, a4
        mul     d5, c, a5
        umulh   a4, c, a4
        adcs    d4, d4, a3
        and     dd, dd, d4
        umulh   a5, c, a5
        adcs    d5, d5, a4
        and     dd, dd, d5

        ldp     a6, a7, [x, #48]
        mul     d6, c, a6
        mul     d7, c, a7
        umulh   a6, c, a6
        adcs    d6, d6, a5
        and     dd, dd, d6
        umulh   a7, c, a7
        adcs    d7, d7, a6
        and     dd, dd, d7

        ldr     a8, [x, #64]
        mul     d8, c, a8
        adcs    d8, d8, a7
        umulh   a8, c, a8
        adc     d9, xzr, a8

// Extract the high part h and mask off the low part l = [d8;d7;...;d0]
// but stuff d8 with 1 bits at the left to ease a comparison below

        extr    h, d9, d8, #9
        orr     d8, d8, #~0x1FF

// Decide whether h + l >= p_521 <=> h + l + 1 >= 2^521. Since this can only
// happen if digits d7,...d1 are all 1s, we use the AND of them "dd" to
// condense the carry chain, and since we stuffed 1 bits into d8 we get
// the result in CF without an additional comparison.

        subs    xzr, xzr, xzr
        adcs    xzr, d0, h
        adcs    xzr, dd, xzr
        adcs    xzr, d8, xzr

// Now if CF is set we want (h + l) - p_521 = (h + l + 1) - 2^521
// while otherwise we want just h + l. So mask h + l + CF to 521 bits.
// This masking also gets rid of the stuffing with 1s we did above.

        adcs    d0, d0, h
        adcs    d1, d1, xzr
        adcs    d2, d2, xzr
        adcs    d3, d3, xzr
        adcs    d4, d4, xzr
        adcs    d5, d5, xzr
        adcs    d6, d6, xzr
        adcs    d7, d7, xzr
        adc     d8, d8, xzr
        and     d8, d8, #0x1FF

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]
        stp     d6, d7, [z, #48]
        str     d8, [z, #64]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

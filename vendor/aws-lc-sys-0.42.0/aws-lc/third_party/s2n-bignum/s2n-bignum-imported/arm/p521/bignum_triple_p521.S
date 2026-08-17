// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Triple modulo p_521, z := (3 * x) mod p_521, assuming x reduced
// Input x[9]; output z[9]
//
//    extern void bignum_triple_p521(uint64_t z[static 9],
//                                   const uint64_t x[static 9]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p521)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p521_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p521_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p521_alt)
        .text
        .balign 4

#define z x0
#define x x1

#define h x2
#define l x3

#define d0 x4
#define d1 x5
#define d2 x6
#define d3 x7
#define d4 x8
#define d5 x9
#define d6 x10
#define d7 x11
#define d8 x12


S2N_BN_SYMBOL(bignum_triple_p521):

S2N_BN_SYMBOL(bignum_triple_p521_alt):
        CFI_START

// Pick out top bit to wrap to the zero position in the doubling step

        ldr     d8, [x, #64]
        lsl     l, d8, #55

// Rotate left to get x' == 2 * x (mod p_521) and add to x + 1 (carryin) to get
// s = [d8;d7;d6;d5;d4;d3;d2;d1;d0] = x + x' + 1 == 3 * x + 1 (mod p_521)

        subs    xzr, xzr, xzr

        ldp     d0, d1, [x]
        extr    l, d0, l, #63
        extr    h, d1, d0, #63
        adcs    d0, d0, l

        ldp     d2, d3, [x, #16]
        extr    l, d2, d1, #63
        adcs    d1, d1, h
        extr    h, d3, d2, #63
        adcs    d2, d2, l

        ldp     d4, d5, [x, #32]
        extr    l, d4, d3, #63
        adcs    d3, d3, h
        extr    h, d5, d4, #63
        adcs    d4, d4, l

        ldp     d6, d7, [x, #48]
        extr    l, d6, d5, #63
        adcs    d5, d5, h
        extr    h, d7, d6, #63
        adcs    d6, d6, l

        extr    l, d8, d7, #63
        adcs    d7, d7, h
        and     l, l, #0x1FF
        adcs    d8, d8, l

// We know x, x' < p_521 (they are the same bits except for the positions)
// so x + x' + 1 <= 2 * (p_521 - 1) + 1 < 2 * p_521.
// Note that x + x' >= p_521 <=> s = x + x' + 1 >= 2^521
// Set CF <=> s = x + x' + 1 >= 2^521 and make it a mask in l as well

        subs    l, d8, #512
        csetm   l, cs

// Now if CF is set (and l is all 1s), we want (x + x') - p_521 = s - 2^521
// while otherwise we want x + x' = s - 1 (from existing CF, which is nice)

        sbcs    d0, d0, xzr
        and     l, l, #512
        sbcs    d1, d1, xzr
        sbcs    d2, d2, xzr
        sbcs    d3, d3, xzr
        sbcs    d4, d4, xzr
        sbcs    d5, d5, xzr
        sbcs    d6, d6, xzr
        sbcs    d7, d7, xzr
        sbc     d8, d8, l

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]
        stp     d6, d7, [z, #48]
        str     d8, [z, #64]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_triple_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

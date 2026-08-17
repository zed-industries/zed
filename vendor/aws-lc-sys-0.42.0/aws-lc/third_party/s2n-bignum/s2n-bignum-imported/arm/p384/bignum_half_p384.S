// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Halve modulo p_384, z := (x / 2) mod p_384, assuming x reduced
// Input x[6]; output z[6]
//
//    extern void bignum_half_p384(uint64_t z[static 6], const uint64_t x[static 6]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_half_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_half_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_half_p384)
        .text
        .balign 4

#define z x0
#define x x1

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define d4 x6
#define d5 x7
#define d6 x8
#define d7 x9
#define m x10
#define n x11


S2N_BN_SYMBOL(bignum_half_p384):
        CFI_START

// Load the 4 digits of x

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]
        ldp     d4, d5, [x, #32]

// Get a bitmask corresponding to the lowest bit of the input

        and     m, d0, #1
        neg     m, m

// Do a masked addition of p_384, catching carry in a 7th word

        and     n, m, #0x00000000ffffffff
        adds    d0, d0, n
        and     n, m, #0xffffffff00000000
        adcs    d1, d1, n
        and     n, m, #0xfffffffffffffffe
        adcs    d2, d2, n
        adcs    d3, d3, m
        adcs    d4, d4, m
        adcs    d5, d5, m
        adc     d6, xzr, xzr

// Now shift that sum right one place

        extr    d0, d1, d0, #1
        extr    d1, d2, d1, #1
        extr    d2, d3, d2, #1
        extr    d3, d4, d3, #1
        extr    d4, d5, d4, #1
        extr    d5, d6, d5, #1

// Store back

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]

// Return

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_half_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

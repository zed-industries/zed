// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Halve modulo p_256, z := (x / 2) mod p_256, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_half_p256(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_half_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_half_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_half_p256)
        .text
        .balign 4

#define z x0
#define x x1

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define d4 x6
#define m x7
#define n x8

S2N_BN_SYMBOL(bignum_half_p256):
        CFI_START

// Load the 4 digits of x

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]

// Get a bitmask corresponding to the lowest bit of the input

        and     m, d0, #1
        neg     m, m

// Do a masked addition of p_256, catching carry in a 5th word

        adds    d0, d0, m
        and     n, m, #0x00000000ffffffff
        adcs    d1, d1, n
        adcs    d2, d2, xzr
        and     n, m, #0xffffffff00000001
        adcs    d3, d3, n
        adc     d4, xzr, xzr

// Now shift that sum right one place

        extr    d0, d1, d0, #1
        extr    d1, d2, d1, #1
        extr    d2, d3, d2, #1
        extr    d3, d4, d3, #1

// Store back

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

// Return

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_half_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

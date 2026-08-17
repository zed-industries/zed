// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negate modulo p_521, z := (-x) mod p_521, assuming x reduced
// Input x[9]; output z[9]
//
//    extern void bignum_neg_p521(uint64_t z[static 9], const uint64_t x[static 9]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_neg_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_neg_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_neg_p521)
        .text
        .balign 4

#define z x0
#define x x1

#define p x2

#define d0 x3
#define d1 x4
#define d2 x5
#define d3 x6
#define d4 x7
#define d5 x8
#define d6 x9
#define d7 x10
#define d8 x11

S2N_BN_SYMBOL(bignum_neg_p521):
        CFI_START

// Load the 9 digits of x and generate p = the OR of them all

        ldp     d0, d1, [x]
        orr     d6, d0, d1
        ldp     d2, d3, [x, #16]
        orr     d7, d2, d3
        orr     p, d6, d7
        ldp     d4, d5, [x, #32]
        orr     d8, d4, d5
        orr     p, p, d8
        ldp     d6, d7, [x, #48]
        orr     d8, d6, d7
        orr     p, p, d8
        ldr     d8, [x, #64]
        orr     p, p, d8

// Turn p into a bitmask for "input is nonzero", so that we avoid doing
// -0 = p_521 and hence maintain strict modular reduction

        cmp     p, #0
        csetm   p, ne

// Since p_521 is all 1s, the subtraction is just an exclusive-or with p
// to give an optional inversion, with a slight fiddle for the top digit.

        eor     d0, d0, p
        eor     d1, d1, p
        eor     d2, d2, p
        eor     d3, d3, p
        eor     d4, d4, p
        eor     d5, d5, p
        eor     d6, d6, p
        eor     d7, d7, p
        and     p, p, #0x1FF
        eor     d8, d8, p

// Write back the result and return

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]
        stp     d6, d7, [z, #48]
        str     d8, [z, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_neg_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally negate modulo p_521, z := (-x) mod p_521 (if p nonzero) or
// z := x (if p zero), assuming x reduced
// Inputs p, x[9]; output z[9]
//
//    extern void bignum_optneg_p521(uint64_t z[static 9], uint64_t p,
//                                   const uint64_t x[static 9]);
//
// Standard ARM ABI: X0 = z, X1 = p, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optneg_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optneg_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optneg_p521)
        .text
        .balign 4

#define z x0
#define p x1
#define x x2
#define q x3

#define d0 x4
#define d1 x5
#define d2 x6
#define d3 x7
#define d4 x8
#define d5 x9
#define d6 x10
#define d7 x11
#define d8 x12

S2N_BN_SYMBOL(bignum_optneg_p521):
        CFI_START

// Load the 9 digits of x and generate q = the OR of them all

        ldp     d0, d1, [x]
        orr     d6, d0, d1
        ldp     d2, d3, [x, #16]
        orr     d7, d2, d3
        orr     q, d6, d7
        ldp     d4, d5, [x, #32]
        orr     d8, d4, d5
        orr     q, q, d8
        ldp     d6, d7, [x, #48]
        orr     d8, d6, d7
        orr     q, q, d8
        ldr     d8, [x, #64]
        orr     q, q, d8

// Turn q into a bitmask for "input is nonzero and p is nonzero", so that
// we avoid doing -0 = p_521 and hence maintain strict modular reduction

        cmp     q, #0
        csetm   q, ne
        cmp     p, #0
        csel    q, xzr, q, eq

// Since p_521 is all 1s, the subtraction is just an exclusive-or with q
// to give an optional inversion, with a slight fiddle for the top digit.

        eor     d0, d0, q
        eor     d1, d1, q
        eor     d2, d2, q
        eor     d3, d3, q
        eor     d4, d4, q
        eor     d5, d5, q
        eor     d6, d6, q
        eor     d7, d7, q
        and     q, q, #0x1FF
        eor     d8, d8, q

// Write back the result and return

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]
        stp     d6, d7, [z, #48]
        str     d8, [z, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optneg_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

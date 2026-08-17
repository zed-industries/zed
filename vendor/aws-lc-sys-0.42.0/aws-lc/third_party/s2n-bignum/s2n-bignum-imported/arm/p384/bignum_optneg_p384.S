// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally negate modulo p_384, z := (-x) mod p_384 (if p nonzero) or
// z := x (if p zero), assuming x reduced
// Inputs p, x[6]; output z[6]
//
//    extern void bignum_optneg_p384(uint64_t z[static 6], uint64_t p,
//                                   const uint64_t x[static 6]);
//
// Standard ARM ABI: X0 = z, X1 = p, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optneg_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optneg_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optneg_p384)
        .text
        .balign 4

#define z x0
#define p x1
#define x x2

#define d0 x3
#define d1 x4
#define d2 x5
#define d3 x6
#define d4 x7
#define d5 x8
#define n0 x9
#define n1 x10
#define n2 x11
#define n3 x12
#define n4 x13
#define n5 x14


S2N_BN_SYMBOL(bignum_optneg_p384):
        CFI_START

// Load the 6 digits of x

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]
        ldp     d4, d5, [x, #32]

// Adjust p by zeroing it if the input is zero (to avoid giving -0 = p, which
// is not strictly reduced even though it's correct modulo p)

        orr     n0, d0, d1
        orr     n1, d2, d3
        orr     n2, d4, d5
        orr     n3, n0, n1
        orr     n4, n2, n3
        cmp     n4, #0
        csel    p, xzr, p, eq

// Load the complicated lower three words of p_384 = [-1;-1;-1;n2;n1;n0] and -1

        mov     n0, #0x00000000ffffffff
        mov     n1, #0xffffffff00000000
        mov     n2, #0xfffffffffffffffe
        mov     n5, #0xffffffffffffffff

// Do the subtraction, which by hypothesis does not underflow

        subs    n0, n0, d0
        sbcs    n1, n1, d1
        sbcs    n2, n2, d2
        sbcs    n3, n5, d3
        sbcs    n4, n5, d4
        sbcs    n5, n5, d5

// Set condition code if original x is nonzero and p was nonzero

        cmp     p, #0

// Hence multiplex and write back

        csel    n0, n0, d0, ne
        csel    n1, n1, d1, ne
        csel    n2, n2, d2, ne
        csel    n3, n3, d3, ne
        csel    n4, n4, d4, ne
        csel    n5, n5, d5, ne

        stp     n0, n1, [z]
        stp     n2, n3, [z, #16]
        stp     n4, n5, [z, #32]

// Return

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optneg_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

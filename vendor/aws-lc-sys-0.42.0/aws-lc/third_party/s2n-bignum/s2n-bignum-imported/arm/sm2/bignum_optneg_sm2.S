// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally negate modulo p_sm2, z := (-x) mod p_sm2 (if p nonzero) or
// z := x (if p zero), assuming x reduced
// Inputs p, x[4]; output z[4]
//
//    extern void bignum_optneg_sm2(uint64_t z[static 4], uint64_t p,
//                                  const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = p, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optneg_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optneg_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optneg_sm2)
        .text
        .balign 4

#define z x0
#define p x1
#define x x2

#define d0 x3
#define d1 x4
#define d2 x5
#define d3 x6
#define n0 x7
#define n1 x8
#define n2 x9
#define n3 x10

S2N_BN_SYMBOL(bignum_optneg_sm2):
        CFI_START

// Load the 4 digits of x

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]

// Adjust p by zeroing it if the input is zero (to avoid giving -0 = p, which
// is not strictly reduced even though it's correct modulo p)

        orr     n0, d0, d1
        orr     n1, d2, d3
        orr     n2, n0, n1
        cmp     n2, #0
        csel    p, xzr, p, eq

// Load the nontrivial words of p_sm2 = [n3;-1;n1;-1]

        mov     n2, #0xffffffffffffffff
        mov     n1, #0xffffffff00000000
        mov     n3, #0xfffffffeffffffff

// Do the subtraction, which by hypothesis does not underflow

        subs    n0, n2, d0
        sbcs    n1, n1, d1
        sbcs    n2, n2, d2
        sbc     n3, n3, d3

// Set condition code if original x is nonzero and p was nonzero

        cmp     p, #0

// Hence multiplex and write back

        csel    n0, n0, d0, ne
        csel    n1, n1, d1, ne
        csel    n2, n2, d2, ne
        csel    n3, n3, d3, ne

        stp     n0, n1, [z]
        stp     n2, n3, [z, #16]

// Return

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optneg_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

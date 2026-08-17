// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negate modulo p_384, z := (-x) mod p_384, assuming x reduced
// Input x[6]; output z[6]
//
//    extern void bignum_neg_p384(uint64_t z[static 6], const uint64_t x[static 6]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_neg_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_neg_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_neg_p384)
        .text
        .balign 4

#define z x0
#define x x1

#define p x2
#define t x3

#define d0 x4
#define d1 x5
#define d2 x6
#define d3 x7
#define d4 x8
#define d5 x9

S2N_BN_SYMBOL(bignum_neg_p384):
        CFI_START

// Load the 6 digits of x

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]
        ldp     d4, d5, [x, #32]

// Set a bitmask p for the input being nonzero, so that we avoid doing
// -0 = p_384 and hence maintain strict modular reduction

        orr     p, d0, d1
        orr     t, d2, d3
        orr     p, p, t
        orr     t, d4, d5
        orr     p, p, t
        cmp     p, #0
        csetm   p, ne

// Mask the complicated lower three words of p_384 = [-1;-1;-1;n2;n1;n0]
// and subtract, using mask itself for upper digits

        and     t, p, #0x00000000ffffffff
        subs    d0, t, d0
        and     t, p, #0xffffffff00000000
        sbcs    d1, t, d1
        and     t, p, #0xfffffffffffffffe
        sbcs    d2, t, d2
        sbcs    d3, p, d3
        sbcs    d4, p, d4
        sbc     d5, p, d5

// Write back the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]

// Return

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_neg_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

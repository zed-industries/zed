// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negate modulo p_sm2, z := (-x) mod p_sm2, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_neg_sm2(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_neg_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_neg_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_neg_sm2)
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

S2N_BN_SYMBOL(bignum_neg_sm2):
        CFI_START

// Load the 4 digits of x

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]

// Set a bitmask p for the input being nonzero, so that we avoid doing
// -0 = p_sm2 and hence maintain strict modular reduction

        orr     t, d0, d1
        orr     p, d2, d3
        orr     p, p, t
        cmp     p, #0
        csetm   p, ne

// Mask nontrivial words of p_sm2 = [n3;-1;n1;-1] and subtract

        subs    d0, p, d0
        and     t, p, #0xffffffff00000000
        sbcs    d1, t, d1
        sbcs    d2, p, d2
        and     t, p, #0xfffffffeffffffff
        sbc     d3, t, d3

// Write back the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

// Return

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_neg_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

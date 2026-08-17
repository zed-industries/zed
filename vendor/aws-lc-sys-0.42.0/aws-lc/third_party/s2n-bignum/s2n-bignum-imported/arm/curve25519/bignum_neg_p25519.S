// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negate modulo p_25519, z := (-x) mod p_25519, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_neg_p25519(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_neg_p25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_neg_p25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_neg_p25519)
        .text
        .balign 4

#define z x0
#define x x1

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define c x6
#define d x7

S2N_BN_SYMBOL(bignum_neg_p25519):
        CFI_START

// Load the digits of x and compute [d3;d2;d1;d0] = (2^255 - 19) - x
// while also computing c = the OR of the digits of x

        ldp     d0, d1, [x]
        mov     d, #-19
        orr     c, d0, d1
        subs    d0, d, d0
        mov     d, #-1
        sbcs    d1, d, d1
        ldp     d2, d3, [x, #16]
        orr     c, c, d2
        sbcs    d2, d, d2
        mov     d, #0x7FFFFFFFFFFFFFFF
        orr     c, c, d3
        sbc     d3, d, d3

// If in fact c = 0 then the result is zero, otherwise the main result

        cmp     c, xzr
        csel    d0, d0, xzr, ne
        csel    d1, d1, xzr, ne
        csel    d2, d2, xzr, ne
        csel    d3, d3, xzr, ne

// Write back result and return

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_neg_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

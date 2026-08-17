// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negate modulo p_256k1, z := (-x) mod p_256k1, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_neg_p256k1(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_neg_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_neg_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_neg_p256k1)
        .text
        .balign 4

#define z x0
#define x x1

#define p x2
#define d0 x3
#define d1 x4
#define d2 x5
#define d3 x6
#define c x7

S2N_BN_SYMBOL(bignum_neg_p256k1):
        CFI_START

// Load the 4 digits of x and let c be an OR of all the digits

        ldp     d0, d1, [x]
        orr     c, d0, d1
        ldp     d2, d3, [x, #16]
        orr     c, c, d2
        orr     c, c, d3

// Turn q into a strict bitmask, and c a masked constant -4294968273,
// computing it in effect as ~4294968272 = ~(2^32 + 976)

        cmp     c, xzr
        csetm   p, ne
        mov     c, #976
        orr     c, c, #0x100000000
        bic     c, p, c

// Now just do [2^256 - 4294968273] - x where the constant is masked

        subs    d0, c, d0
        sbcs    d1, p, d1
        sbcs    d2, p, d2
        sbc     d3, p, d3

// Write back result and return

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_neg_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

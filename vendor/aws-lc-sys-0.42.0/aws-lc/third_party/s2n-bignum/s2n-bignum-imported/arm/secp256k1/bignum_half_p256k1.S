// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Halve modulo p_256k1, z := (x / 2) mod p_256k1, assuming x reduced
// Input x[4]; output z[4]
//
//    extern void bignum_half_p256k1(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_half_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_half_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_half_p256k1)
        .text
        .balign 4

#define z x0
#define x x1

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define c x6

S2N_BN_SYMBOL(bignum_half_p256k1):
        CFI_START

// Load the 4 digits of x

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]

// Let b be the LSB of the input (i.e. whether it is odd).
// Create c = 4294968273 * b

        mov     c, #977
        orr     c, c, #0x100000000
        tst     d0, #1
        csel    c, c, xzr, ne

// We want (x + b * p_256k1) / 2 where b is that LSB, in {0,1}.
// That amounts to (2^256 * b + x - 4294968273 * b) / 2, and
// modulo 4 words that's the same as ([2^256 * c + x] - c) / 2.
// So do that subtraction and shift a place right as we go.

        subs    d0, d0, c
        sbcs    d1, d1, xzr
        extr    d0, d1, d0, #1
        sbcs    d2, d2, xzr
        extr    d1, d2, d1, #1
        sbcs    d3, d3, xzr
        extr    d2, d3, d2, #1
        sbc     c, c, xzr
        extr    d3, c, d3, #1

// Store back and return

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_half_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

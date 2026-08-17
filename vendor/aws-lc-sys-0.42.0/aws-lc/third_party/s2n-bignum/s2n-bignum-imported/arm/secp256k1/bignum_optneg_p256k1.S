// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally negate modulo p_256k1, z := (-x) mod p_256k1 (if p nonzero) or
// z := x (if p zero), assuming x reduced
// Inputs p, x[4]; output z[4]
//
//    extern void bignum_optneg_p256k1(uint64_t z[static 4], uint64_t p,
//                                     const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = p, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optneg_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optneg_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optneg_p256k1)
        .text
        .balign 4

#define z x0
#define p x1
#define x x2

#define d0 x3
#define d1 x4
#define d2 x5
#define d3 x6
#define c x7

S2N_BN_SYMBOL(bignum_optneg_p256k1):
        CFI_START

// Load the 4 digits of x and let c be an OR of all the digits

        ldp     d0, d1, [x]
        orr     c, d0, d1
        ldp     d2, d3, [x, #16]
        orr     c, c, d2
        orr     c, c, d3

// Turn p into a strict bitmask. Force it to zero if the input is zero,
// to avoid giving -0 = p_256k1, which is not reduced though correct modulo.

        cmp     p, xzr
        csetm   p, ne
        cmp     c, xzr
        csel    p, xzr, p, eq

// We want z := if p then (2^256 - 4294968273) - x else x
// which is: [if p then ~x else x] - [if p then 4294968272 else 0]

        mov     c, #976
        orr     c, c, #0x100000000
        and     c, c, p

        eor     d0, d0, p
        subs    d0, d0, c
        eor     d1, d1, p
        sbcs    d1, d1, xzr
        eor     d2, d2, p
        sbcs    d2, d2, xzr
        eor     d3, d3, p
        sbc     d3, d3, xzr

// Write back result and return

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optneg_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

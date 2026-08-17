// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Triple modulo p_256k1, z := (3 * x) mod p_256k1
// Input x[4]; output z[4]
//
//    extern void bignum_triple_p256k1(uint64_t z[static 4],
//                                     const uint64_t x[static 4]);
//
// The input x can be any 4-digit bignum, not necessarily reduced modulo
// p_256k1, and the result is always fully reduced, z = (3 * x) mod p_256k1.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p256k1)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p256k1_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p256k1_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p256k1_alt)
        .text
        .balign 4

#define z x0
#define x x1

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define h x6

// Slightly offset aliases for the d_i for readability.

#define a0 x3
#define a1 x4
#define a2 x5
#define a3 x6

// More aliases for the same thing at different stages

#define m x6

// Other temporary variables

#define c x7

S2N_BN_SYMBOL(bignum_triple_p256k1):
S2N_BN_SYMBOL(bignum_triple_p256k1_alt):
        CFI_START

// Load the inputs

        ldp     a0, a1, [x]
        ldp     a2, a3, [x, #16]

// First do the multiplication by 3, getting z = [h; d3; ...; d0]

        adds    d0, a0, a0, lsl #1
        extr    d1, a1, a0, #63
        adcs    d1, d1, a1
        extr    d2, a2, a1, #63
        adcs    d2, d2, a2
        extr    d3, a3, a2, #63
        adcs    d3, d3, a3
        lsr     h, a3, #63
        adc     h, h, xzr

// For this limited range a simple quotient estimate of q = h + 1 works, where
// h = floor(z / 2^256). Then -p_256k1 <= z - q * p_256k1 < p_256k1.

        mov     c, #977
        orr     c, c, #0x100000000
        madd    m, h, c, c

// Initial subtraction of z - q * p_256k1, actually by adding q * 4294968273.

        adds    d0, d0, m
        adcs    d1, d1, xzr
        adcs    d2, d2, xzr
        adcs    d3, d3, xzr

// With z = 2^256 * h + l, the underlying result z' is actually
// (2^256 * h + l) - q * (2^256 - 4294968273) = (l + q * 4294968273) - 2^256
// so carry-clear <=> z' is negative. Correct by subtracting in that case.

        csel    c, c, xzr, cc
        subs    d0, d0, c
        sbcs    d1, d1, xzr
        sbcs    d2, d2, xzr
        sbc     d3, d3, xzr

// Finally store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_triple_p256k1)
S2N_BN_SIZE_DIRECTIVE(bignum_triple_p256k1_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

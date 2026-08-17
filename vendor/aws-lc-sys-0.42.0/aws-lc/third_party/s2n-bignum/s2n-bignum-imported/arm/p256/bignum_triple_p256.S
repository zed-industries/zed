// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Triple modulo p_256, z := (3 * x) mod p_256
// Input x[4]; output z[4]
//
//    extern void bignum_triple_p256(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// The input x can be any 4-digit bignum, not necessarily reduced modulo p_256,
// and the result is always fully reduced, i.e. z = (3 * x) mod p_256.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p256)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p256_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p256_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p256_alt)
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

#define q x7
#define c x7

// Other temporary variables

#define t0 x8
#define t1 x9


S2N_BN_SYMBOL(bignum_triple_p256):

S2N_BN_SYMBOL(bignum_triple_p256_alt):
        CFI_START

// Load the inputs

        ldp     a0, a1, [x]
        ldp     a2, a3, [x, #16]

// First do the multiplication by 3, getting z = [h; d3; ...; d0]

        lsl     d0, a0, #1
        adds    d0, d0, a0
        extr    d1, a1, a0, #63
        adcs    d1, d1, a1
        extr    d2, a2, a1, #63
        adcs    d2, d2, a2
        extr    d3, a3, a2, #63
        adcs    d3, d3, a3
        lsr     h, a3, #63
        adc     h, h, xzr

// For this limited range a simple quotient estimate of q = h + 1 works, where
// h = floor(z / 2^256). Then -p_256 <= z - q * p_256 < p_256, so we just need
// to subtract q * p_256 and then if that's negative, add back p_256.

        add     q, h, #1

// Initial subtraction of z - q * p_256, with bitmask c for the carry

        lsl     t1, q, #32
        adds    d3, d3, t1
        adc     h, h, xzr
        sub     t0, xzr, q
        sub     t1, t1, #1
        subs    d0, d0, t0
        sbcs    d1, d1, t1
        sbcs    d2, d2, xzr
        sbcs    d3, d3, q
        sbc     c, h, q

// Use the bitmask c for final masked addition of p_256.

        adds    d0, d0, c
        mov     t0, #0x00000000ffffffff
        and     t0, t0, c
        adcs    d1, d1, t0
        adcs    d2, d2, xzr
        neg     t1, t0
        adc     d3, d3, t1

// Finally store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_triple_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Triple modulo p_384, z := (3 * x) mod p_384
// Input x[6]; output z[6]
//
//    extern void bignum_triple_p384(uint64_t z[static 6],
//                                   const uint64_t x[static 6]);
//
// The input x can be any 6-digit bignum, not necessarily reduced modulo p_384,
// and the result is always fully reduced, i.e. z = (3 * x) mod p_384.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p384)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_triple_p384_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_triple_p384_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_triple_p384_alt)
        .text
        .balign 4

#define z x0
#define x x1

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5
#define d4 x6
#define d5 x7
#define h x8

// Slightly offset aliases for the d_i for readability.

#define a0 x3
#define a1 x4
#define a2 x5
#define a3 x6
#define a4 x7
#define a5 x8

// More aliases for the same thing at different stages

#define q x8
#define c x8

// Other temporary variables

#define t0 x9
#define t1 x10


S2N_BN_SYMBOL(bignum_triple_p384):

S2N_BN_SYMBOL(bignum_triple_p384_alt):
        CFI_START

// Load the inputs

        ldp     a0, a1, [x]
        ldp     a2, a3, [x, #16]
        ldp     a4, a5, [x, #32]

// First do the multiplication by 3, getting z = [h; d5; ...; d0]

        lsl     d0, a0, #1
        adds    d0, d0, a0
        extr    d1, a1, a0, #63
        adcs    d1, d1, a1
        extr    d2, a2, a1, #63
        adcs    d2, d2, a2
        extr    d3, a3, a2, #63
        adcs    d3, d3, a3
        extr    d4, a4, a3, #63
        adcs    d4, d4, a4
        extr    d5, a5, a4, #63
        adcs    d5, d5, a5
        lsr     h, a5, #63
        adc     h, h, xzr

// For this limited range a simple quotient estimate of q = h + 1 works, where
// h = floor(z / 2^384). Then -p_384 <= z - q * p_384 < p_384, so we just need
// to subtract q * p_384 and then if that's negative, add back p_384.

        add     q, h, #1

// Initial subtraction of z - q * p_384, with bitmask c for the carry
// Actually done as an addition of (z - 2^384 * h) + q * (2^384 - p_384)
// which, because q = h + 1, is exactly 2^384 + (z - q * p_384), and
// therefore CF <=> 2^384 + (z - q * p_384) >= 2^384 <=> z >= q * p_384.

        lsl     t1, q, #32
        subs    t0, q, t1
        sbc     t1, t1, xzr

        adds    d0, d0, t0
        adcs    d1, d1, t1
        adcs    d2, d2, q
        adcs    d3, d3, xzr
        adcs    d4, d4, xzr
        adcs    d5, d5, xzr
        csetm   c, cc

// Use the bitmask c for final masked addition of p_384.

        mov     t0, #0x00000000ffffffff
        and     t0, t0, c
        adds    d0, d0, t0
        eor     t0, t0, c
        adcs    d1, d1, t0
        mov     t0, #0xfffffffffffffffe
        and     t0, t0, c
        adcs    d2, d2, t0
        adcs    d3, d3, c
        adcs    d4, d4, c
        adc     d5, d5, c

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_triple_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

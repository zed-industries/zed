// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_25519, z := (c * x) mod p_25519, assuming
// x reduced
// Inputs c, x[4]; output z[4]
//
//    extern void bignum_cmul_p25519(uint64_t z[static 4], uint64_t c,
//                                   const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = c, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p25519)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p25519_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p25519_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p25519_alt)
        .text
        .balign 4

#define z x0
#define m x1
#define x x2

#define d0 x3
#define d1 x4
#define d2 x5
#define d3 x6

#define a0 x7
#define a1 x8
#define a2 x9
#define a3 x10

// Aliased to the a0,..,a3 when they are no longer needed

#define l x7
#define q x8
#define c x9
#define d4 x10
#define h x10

S2N_BN_SYMBOL(bignum_cmul_p25519):
S2N_BN_SYMBOL(bignum_cmul_p25519_alt):
        CFI_START

// First do the multiply, straightforwardly to get [d4;d3;d2;d1;d0]

        ldp     a0, a1, [x]
        ldp     a2, a3, [x, #16]
        mul     d0, m, a0
        mul     d1, m, a1
        mul     d2, m, a2
        mul     d3, m, a3
        umulh   a0, m, a0
        umulh   a1, m, a1
        umulh   a2, m, a2
        umulh   d4, m, a3
        adds    d1, d1, a0
        adcs    d2, d2, a1
        adcs    d3, d3, a2
        adcs    d4, d4, xzr

// Let 2^255 * h + l = [d4,d3,d2,d1,d0] = c * x, and use q = h + 1
// as the initial quotient estimate, either right or 1 too big.

        add     q, d4, 1
        adds    xzr, d3, d3
        orr     d3, d3, #0x8000000000000000
        adc     q, q, d4
        mov     c, #19
        mul     l, q, c
        umulh   h, q, c
        adds    d0, d0, l
        adcs    d1, d1, h
        adcs    d2, d2, xzr
        adcs    d3, d3, xzr

// Correct if CF = 0 by subtracting 19, either way masking to
// 255 bits, i.e. by effectively adding p_25519 to the "full" answer

        csel    c, c, xzr, cc
        subs    d0, d0, c
        sbcs    d1, d1, xzr
        sbcs    d2, d2, xzr
        sbc     d3, d3, xzr
        and     d3, d3, #~0x8000000000000000

// Finally store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p25519)
S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p25519_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word modulo p_256k1, z := (c * x) mod p_256k1, assuming
// x reduced
// Inputs c, x[4]; output z[4]
//
//    extern void bignum_cmul_p256k1(uint64_t z[static 4], uint64_t c,
//                                   const uint64_t x[static 4]);
//
// Standard ARM ABI: X0 = z, X1 = c, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p256k1)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul_p256k1_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul_p256k1_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul_p256k1_alt)
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
#define c x9

#define a3 x10
#define h x10
#define q x10


S2N_BN_SYMBOL(bignum_cmul_p256k1):

S2N_BN_SYMBOL(bignum_cmul_p256k1_alt):
        CFI_START

// First do the multiply, straightforwardly to get [h;d3;d2;d1;d0]

        ldp     a0, a1, [x]
        ldp     a2, a3, [x, #16]
        mul     d0, m, a0
        mul     d1, m, a1
        mul     d2, m, a2
        mul     d3, m, a3
        umulh   a0, m, a0
        umulh   a1, m, a1
        umulh   a2, m, a2
        umulh   h, m, a3
        adds    d1, d1, a0
        adcs    d2, d2, a1
        adcs    d3, d3, a2
        adcs    h, h, xzr

// Now the quotient estimate is q = h + 1, and then we do the reduction,
// writing z = [d3;d2;d1;d0], as z' = (2^256 * h + z) - q * p_256k1 =
// (2^256 * h + z) - q * (2^256 - 4294968273) = -2^256 + (z + 4294968273 * q)

        add     q, h, #1
        mov     c, #977
        orr     c, c, #0x100000000
        mul     a0, q, c
        umulh   a1, q, c
        adds    d0, d0, a0
        adcs    d1, d1, a1
        adcs    d2, d2, xzr
        adcs    d3, d3, xzr

// Because of the implicit -2^256, CF means >= 0 so z' is the answer; ~CF
// means z' < 0 so we add p_256k1, which in 4 digits means subtracting c.

        csel    c, c, xzr, cc
        subs    d0, d0, c
        sbcs    d1, d1, xzr
        sbcs    d2, d2, xzr
        sbc     d3, d3, xzr

// Finally store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from Montgomery form z := (x / 2^256) mod p_256k1,
// Input x[4]; output z[4]
//
//    extern void bignum_deamont_p256k1(uint64_t z[static 4],
//                                      const uint64_t x[static 4]);
//
// Convert a 4-digit bignum x out of its (optionally almost) Montgomery form,
// "almost" meaning any 4-digit input will work, with no range restriction.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_deamont_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_deamont_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_deamont_p256k1)
        .text
        .balign 4

// Input parameters

#define z x0
#define x x1

// Rotating registers for the intermediate windows

#define d0 x2
#define d1 x3
#define d2 x4
#define d3 x5

// Other temporaries

#define t x6
#define w x7
#define c x8
#define dd x9

S2N_BN_SYMBOL(bignum_deamont_p256k1):
        CFI_START

// Load input and set up constants c = 4294968273 so p_256k1 = 2^256 - c,
// and w the negated multiplicative inverse p_256k1 * w == -1 (mod 2^64).

        ldp     d0, d1, [x]
        movz    w, #0x3531
        movk    w, #0xd225, lsl #16
        ldp     d2, d3, [x, #16]
        movk    w, #0x091d, lsl #32
        movk    w, #0xd838, lsl #48
        mov     c, #977
        orr     c, c, #0x100000000

// Four stages of Montgomery reduction, rotating the register window
// Let dd be the AND of all 4 words of the cofactor q as it is computed

        mul     d0, w, d0
        umulh   t, d0, c
        subs    d1, d1, t

        mul     d1, w, d1
        umulh   t, d1, c
        and     dd, d0, d1
        sbcs    d2, d2, t

        mul     d2, w, d2
        umulh   t, d2, c
        and     dd, dd, d2
        sbcs    d3, d3, t

        mul     d3, w, d3
        umulh   t, d3, c
        and     dd, dd, d3
        sbcs    d0, d0, t

        sbcs    d1, d1, xzr
        sbcs    d2, d2, xzr
        sbc     d3, d3, xzr

// The result thus far is z = (x + q * p_256k1) / 2^256. Note that
// z < p_256k1 <=> x < (2^256 - q) * p_256k1, and since
// x < 2^256 < 2 * p_256k1, we have that *if* q < 2^256 - 1 then
// z < p_256k1. Conversely if q = 2^256 - 1 then since
// x + q * p_256k1 == 0 (mod 2^256) we have x == p_256k1 (mod 2^256)
// and thus x = p_256k1, and z >= p_256k1 (in fact z = p_256k1).
// So in summary z < p_256k1 <=> ~(q = 2^256 - 1) <=> ~(x = p_256k1).
// and hence iff q is all 1s, or equivalently dd is all 1s, we
// correct by subtracting p_256k1 to get 0. Since this is only one
// case we compute the result more explicitly rather than doing
// arithmetic with carry propagation.

        add     c, c, d0
        cmp     dd, #-1
        csel    d0, c, d0, eq
        csel    d1, xzr, d1, eq
        csel    d2, xzr, d2, eq
        csel    d3, xzr, d3, eq

// Write back result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_deamont_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

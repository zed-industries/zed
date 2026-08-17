// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo field characteristic, z := x mod p_521
// Input x[9]; output z[9]
//
//    extern void bignum_mod_p521_9(uint64_t z[static 9], const uint64_t x[static 9]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p521_9)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p521_9)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p521_9)
        .text
        .balign 4

#define z x0
#define x x1

#define h x2
#define t x3
#define d0 x4
#define d1 x5
#define d2 x6
#define d3 x7
#define d4 x8
#define d5 x9
#define d6 x10
#define d7 x11
#define d8 x12

S2N_BN_SYMBOL(bignum_mod_p521_9):
        CFI_START

// Load top digit first and get its upper bits in h so that we
// separate out x = 2^521 * H + L with h = H. Now x mod p_521 =
// (H + L) mod p_521 = if H + L >= p_521 then H + L - p_521 else H + L.

        ldr     d8, [x, #64]
        lsr     h, d8, #9

// Load in the other digits and decide whether H + L >= p_521. This is
// equivalent to H + L + 1 >= 2^521, and since this can only happen if
// digits d7,...,d1 consist entirely of 1 bits, we can condense the
// carry chain by ANDing digits together, perhaps reducing its latency.
// This condenses only three pairs; the payoff beyond that seems limited.
// By stuffing in 1 bits from 521 position upwards, get CF directly

        subs    xzr, xzr, xzr
        ldp     d0, d1, [x]
        adcs    xzr, d0, h
        adcs    xzr, d1, xzr
        ldp     d2, d3, [x, #16]
        and     t, d2, d3
        adcs    xzr, t, xzr
        ldp     d4, d5, [x, #32]
        and     t, d4, d5
        adcs    xzr, t, xzr
        ldp     d6, d7, [x, #48]
        and     t, d6, d7
        adcs    xzr, t, xzr
        orr     t, d8, #~0x1FF
        adcs    t, t, xzr

// Now H + L >= p_521 <=> H + L + 1 >= 2^521 <=> CF from this comparison.
// So if CF is set we want (H + L) - p_521 = (H + L + 1) - 2^521
// while otherwise we want just H + L. So mask H + L + CF to 521 bits.

        adcs    d0, d0, h
        adcs    d1, d1, xzr
        adcs    d2, d2, xzr
        adcs    d3, d3, xzr
        adcs    d4, d4, xzr
        adcs    d5, d5, xzr
        adcs    d6, d6, xzr
        adcs    d7, d7, xzr
        adc     d8, d8, xzr
        and     d8, d8, #0x1FF

// Store the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]
        stp     d6, d7, [z, #48]
        str     d8, [z, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_p521_9)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

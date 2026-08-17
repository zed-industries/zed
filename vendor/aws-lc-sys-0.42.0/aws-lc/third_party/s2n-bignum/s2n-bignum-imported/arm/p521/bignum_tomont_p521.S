// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert to Montgomery form z := (2^576 * x) mod p_521
// Input x[9]; output z[9]
//
//    extern void bignum_tomont_p521(uint64_t z[static 9],
//                                   const uint64_t x[static 9]);
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tomont_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tomont_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tomont_p521)
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

S2N_BN_SYMBOL(bignum_tomont_p521):
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

// So far, this is just a modular reduction as in bignum_mod_p521_9,
// except that the final masking of d8 is skipped since that comes out
// in the wash anyway from the next block, which is the Montgomery map,
// multiplying by 2^576 modulo p_521. Because 2^521 == 1 (mod p_521)
// this is just rotation left by 576 - 521 = 55 bits. To rotate in a
// right-to-left fashion, which might blend better with the carry
// chain above, the digit register indices themselves get shuffled up.

        lsl     t, d0, #55
        extr    d0, d1, d0, #9
        extr    d1, d2, d1, #9
        extr    d2, d3, d2, #9
        extr    d3, d4, d3, #9
        extr    d4, d5, d4, #9
        extr    d5, d6, d5, #9
        extr    d6, d7, d6, #9
        extr    d7, d8, d7, #9
        lsr     d8, d7, #9
        orr     t, t, d8
        and     d7, d7, #0x1FF

// Store the result from the shuffled registers [d7;d6;...;d1;d0;t]

        stp     t, d0, [z]
        stp     d1, d2, [z, #16]
        stp     d3, d4, [z, #32]
        stp     d5, d6, [z, #48]
        str     d7, [z, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_tomont_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

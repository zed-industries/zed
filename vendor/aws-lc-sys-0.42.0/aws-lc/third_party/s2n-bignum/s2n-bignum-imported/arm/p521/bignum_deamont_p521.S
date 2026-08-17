// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from Montgomery form z := (x / 2^576) mod p_521
// Input x[9]; output z[9]
//
//    extern void bignum_deamont_p521(uint64_t z[static 9],
//                                    const uint64_t x[static 9]);
//
// Convert a 9-digit bignum x out of its (optionally almost) Montgomery form,
// "almost" meaning any 9-digit input will work, with no range restriction.
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_deamont_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_deamont_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_deamont_p521)
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
#define d4 x6
#define d5 x7
#define d6 x8
#define d7 x9
#define d8 x10

// Some other variables, not all distinct

#define c x11
#define h x11
#define l x12
#define u x12

S2N_BN_SYMBOL(bignum_deamont_p521):
        CFI_START

// Load all the inputs

        ldp     d0, d1, [x]
        ldp     d2, d3, [x, #16]
        ldp     d4, d5, [x, #32]
        ldp     d6, d7, [x, #48]
        ldr     d8, [x, #64]

// Stash the lowest 55 bits at the top of c, then shift the whole 576-bit
// input right by 9*64 - 521 = 576 - 521 = 55 bits. As this is done,
// accumulate an AND of words d0..d6.

        lsl     c, d0, #9
        extr    d0, d1, d0, #55
        extr    d1, d2, d1, #55
        and     u, d0, d1
        extr    d2, d3, d2, #55
        and     u, u, d2
        extr    d3, d4, d3, #55
        and     u, u, d3
        extr    d4, d5, d4, #55
        and     u, u, d4
        extr    d5, d6, d5, #55
        and     u, u, d5
        extr    d6, d7, d6, #55
        and     u, u, d6
        extr    d7, d8, d7, #55
        lsr     d8, d8, #55

// Now writing x = 2^55 * h + l (so here [d8;..d0] = h and c = 2^9 * l)
// we want (h + 2^{521-55} * l) mod p_521 = s mod p_521. Since s < 2 * p_521
// this is just "if s >= p_521 then s - p_521 else s". First get
// CF <=> s >= p_521, creating the digits [h,l] to add for the l part.

        adds    xzr, u, #1
        lsl     l, c, #9
        adcs    xzr, d7, l
        orr     d8, d8, #~0x1FF
        lsr     h, c, #55
        adcs    xzr, d8, h

// Now the result = s mod p_521 = (if s >= p_521 then s - p_521 else s) =
// (s + CF) mod 2^521. So do the addition inheriting the carry-in.

        adcs    d0, d0, xzr
        adcs    d1, d1, xzr
        adcs    d2, d2, xzr
        adcs    d3, d3, xzr
        adcs    d4, d4, xzr
        adcs    d5, d5, xzr
        adcs    d6, d6, xzr
        adcs    d7, d7, l
        adc     d8, d8, h
        and     d8, d8, #0x1FF

// Store back the result

        stp     d0, d1, [z]
        stp     d2, d3, [z, #16]
        stp     d4, d5, [z, #32]
        stp     d6, d7, [z, #48]
        str     d8, [z, #64]
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_deamont_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Shift bignum right by c < 64 bits z := floor(x / 2^c)
// Inputs x[n], c; outputs function return (bits shifted out) and z[k]
//
//    extern uint64_t bignum_shr_small(uint64_t k, uint64_t *z, uint64_t n,
//                                     const uint64_t *x, uint64_t c);
//
// Does the "z := x >> c" operation where x is n digits, result z is p.
// The shift count c is masked to 6 bits so it actually uses c' = c mod 64.
// The return value is the inout mod 2^c'.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = n, X3 = x, X4 = c, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_shr_small)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_shr_small)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_shr_small)
        .text
        .balign 4

#define p x0
#define z x1
#define n x2
#define x x3
#define c x4

#define d x5
#define a x6
#define b x7
#define m x8
#define t x9


S2N_BN_SYMBOL(bignum_shr_small):
        CFI_START

// Set default carry-in word to 0

        mov     b, xzr

// First, if p > n then pad output on the left with p-n zeros

        cmp     n, p
        bcs     Lbignum_shr_small_nopad
Lbignum_shr_small_padloop:
        sub     p, p, #1
        str     xzr, [z, p, lsl #3]
        cmp     n, p
        bcc     Lbignum_shr_small_padloop

// We now know that p <= n. If in fact p < n let carry word = x[p] instead of 0

Lbignum_shr_small_nopad:
        beq     Lbignum_shr_small_shiftstart
        ldr     b, [x, p, lsl #3]
Lbignum_shr_small_shiftstart:

// Set up negated version of the shift and shift b in preparation.
// Use a mask for nonzero shift to fake 64-bit left shift in zero case

        neg     d, c
        lsl     b, b, d
        ands    xzr, c, #63
        csetm   m, ne
        and     b, b, m

// Now the main loop

        cbz     p, Lbignum_shr_small_end
Lbignum_shr_small_loop:
        sub     p, p, #1
        ldr     t, [x, p, lsl #3]
        lsr     a, t, c
        orr     a, a, b
        lsl     b, t, d
        and     b, b, m
        str     a, [z, p, lsl #3]
        cbnz    p, Lbignum_shr_small_loop

// Return top word, shifted back to be a modulus

Lbignum_shr_small_end:
        lsr     x0, b, d
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_shr_small)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

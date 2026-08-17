// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Shift bignum left by c < 64 bits z := x * 2^c
// Inputs x[n], c; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_shl_small(uint64_t k, uint64_t *z, uint64_t n,
//                                     const uint64_t *x, uint64_t c);
//
// Does the "z := x << c" operation where x is n digits, result z is p.
// The shift count c is masked to 6 bits so it actually uses c' = c mod 64.
// The return value is the "next word" of a p+1 bit result, if n <= p.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = n, X3 = x, X4 = c, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_shl_small)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_shl_small)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_shl_small)
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
#define i x10


S2N_BN_SYMBOL(bignum_shl_small):
        CFI_START

// First clamp the input size n := min(p,n) since we can never need to read
// past the p'th term of the input to generate p-digit output.

        cmp     n, p
        csel    n, p, n, cs

// Initialize counter i and "previous word" carry b to zero
// and skip main loop if n = 0

        mov     b, xzr
        mov     i, xzr
        cbz     n, Lbignum_shl_small_tail

// Set up a mask for nonzero shift and a negated version of the shift.
// Note that all basic word-level shifts are predictably masked to 6 bits.

        ands    xzr, c, #63
        csetm   m, ne
        neg     d, c

// Now the main loop
Lbignum_shl_small_loop:
        ldr     t, [x, i, lsl #3]
        lsl     a, t, c
        orr     a, a, b
        lsr     b, t, d
        and     b, b, m
        str     a, [z, i, lsl #3]
        add     i, i, #1
        cmp     i, n
        bcc     Lbignum_shl_small_loop

// If we are at the end, finish, otherwise write carry word then zeros

Lbignum_shl_small_tail:

        cmp     i, p
        bcs     Lbignum_shl_small_end
        str     b, [z, i, lsl #3]
        mov     b, xzr
        add     i, i, #1
        cmp     i, p
        bcs     Lbignum_shl_small_end

Lbignum_shl_small_tloop:
        str     xzr, [z, i, lsl #3]
        add     i, i, #1
        cmp     i, p
        bcc     Lbignum_shl_small_tloop

// Return top word

Lbignum_shl_small_end:

        mov     x0, b
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_shl_small)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

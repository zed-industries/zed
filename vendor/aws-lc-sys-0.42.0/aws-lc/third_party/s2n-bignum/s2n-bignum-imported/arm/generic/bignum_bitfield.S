// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Select bitfield starting at bit n with length l <= 64
// Inputs x[k], n, l; output function return
//
//    extern uint64_t bignum_bitfield(uint64_t k, const uint64_t *x, uint64_t n,
//                                    uint64_t l);
//
// One-word bitfield from a k-digit (digit=64 bits) bignum, in constant-time
// style. Bitfield starts at bit n and has length l, indexing from 0 (=LSB).
// Digits above the top are treated uniformly as zero, as usual. Since the
// result is returned in a single word, effectively we use l' = min(64,l)
// for the length.
//
// Standard ARM ABI: X0 = k, X1 = x, X2 = n, X3 = l, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_bitfield)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_bitfield)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_bitfield)
        .text
        .balign 4

#define k x0
#define x x1
#define n x2
#define l x3

#define d x4
#define e x5
#define i x6
#define a x7
#define m x8


S2N_BN_SYMBOL(bignum_bitfield):
        CFI_START

// For length zero finish immediately (the return value in x0 is 0)

        cbz     k, Lbignum_bitfield_end

// Decompose the index into n = 64 * n + m, then increment n for next part

        and     m, n, #63
        lsr     n, n, #6
        add     n, n, #1

// Run over the digits setting d = n'th and e = (n+1)'th

        mov     i, xzr
        mov     e, xzr
Lbignum_bitfield_loop:
        ldr     a, [x, i, lsl #3]
        cmp     i, n
        csel    d, a, d, cc
        csel    e, a, e, eq
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_bitfield_loop

// Override d with 0 if we ran off the end (e will retain original 0).

        cmp     i, n
        csel    d, xzr, d, cc

// Override e if we have m = 0 (i.e. original n was divisible by 64)
// This is because then we want to shift it right by 64 below.

        cmp     m, xzr
        csel    e, xzr, e, eq

// Combine shifted digits to get the bitfield(n,64)

        lsr     d, d, m
        neg     m, m
        lsl     e, e, m
        orr     a, d, e

// Now mask it down to get bitfield (n,l)

        cmp     l, #64
        cset    m, cc
        lsl     m, m, l
        sub     m, m, #1
        and     x0, a, m
Lbignum_bitfield_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_bitfield)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

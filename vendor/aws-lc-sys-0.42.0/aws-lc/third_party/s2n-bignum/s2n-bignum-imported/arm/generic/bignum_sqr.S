// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square z := x^2
// Input x[n]; output z[k]
//
//    extern void bignum_sqr(uint64_t k, uint64_t *z, uint64_t n, const uint64_t *x);
//
// Does the "z := x^2" operation where x is n digits and result z is k.
// Truncates the result in general unless k >= 2 * n
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = n, X3 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr)
        .text
        .balign 4

#define p x0
#define z x1
#define n x2
#define x x3

#define l x4
#define h x5
#define c x6
#define k x7
#define i x8
#define a x9
#define b x10
#define d x11
#define y x12
#define htop x13
#define hh x14
#define ll x15

S2N_BN_SYMBOL(bignum_sqr):
        CFI_START

// If p = 0 the result is trivial and nothing needs doing

        cbz     p, Lbignum_sqr_end

// initialize (hh,ll) = 0

        mov     ll, xzr
        mov     hh, xzr

// Iterate outer loop from k = 0 ... k = p - 1 producing result digits

        mov     k, xzr
Lbignum_sqr_outerloop:

// First let bot = MAX 0 (k + 1 - n) and top = MIN (k + 1) n
// We want to accumulate all x[i] * x[k - i] for bot <= i < top
// For the optimization of squaring we avoid duplication and do
// 2 * x[i] * x[k - i] for i < htop, where htop = MIN ((k+1)/2) n
// Initialize i = bot; in fact just compute bot as i directly.

        add     i, k, #1
        lsr     htop, i, #1
        cmp     htop, n
        csel    htop, htop, n, cc
        subs    i, i, n
        csel    i, i, xzr, cs

// Initialize the three-part local sum (c,h,l)

        mov     l, xzr
        mov     h, xzr
        mov     c, xzr

// If htop <= bot then main doubled part of the sum is empty

        cmp     htop, i
        bls     Lbignum_sqr_nosumming

// Use a moving pointer for [y] = x[k-i] for the cofactor

        sub     y, k, i
        lsl     y, y, #3
        add     y, x, y

// Do the main part of the sum x[i] * x[k - i] for 2 * i < k

Lbignum_sqr_innerloop:
        ldr     a, [x, i, lsl #3]
        ldr     b, [y], #-8
        mul     d, a, b
        umulh   a, a, b
        adds    l, l, d
        adcs    h, h, a
        adc     c, c, xzr
        add     i, i, #1
        cmp     i, htop
        bne     Lbignum_sqr_innerloop

// Now double it

        adds    l, l, l
        adcs    h, h, h
        adc     c, c, c

// If k is even (which means 2 * i = k) and i < n add the extra x[i]^2 term

Lbignum_sqr_nosumming:

        ands    xzr, k, #1
        bne     Lbignum_sqr_innerend
        cmp     i, n
        bcs     Lbignum_sqr_innerend

        ldr     a, [x, i, lsl #3]
        mul     d, a, a
        umulh   a, a, a
        adds    ll, ll, d
        adcs    hh, hh, a
        adc     c, c, xzr

// Now add the local sum into the global sum, store and shift

Lbignum_sqr_innerend:
        adds    l, l, ll
        str     l, [z, k, lsl #3]
        adcs    ll, h, hh
        adc     hh, c, xzr

        add     k, k, #1
        cmp     k, p
        bcc     Lbignum_sqr_outerloop

Lbignum_sqr_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

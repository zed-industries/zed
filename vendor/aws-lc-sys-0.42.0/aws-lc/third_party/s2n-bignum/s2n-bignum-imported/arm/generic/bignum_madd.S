// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply-add, z := z + x * y
// Inputs x[m], y[n]; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_madd(uint64_t k, uint64_t *z, uint64_t m,
//                                const uint64_t *x, uint64_t n, const uint64_t *y);
//
// Does the "z := x * y + z" operation, while also returning a "next" or
// "carry" word. In the case where m + n <= p (i.e. the pure product would
// fit in the destination) this is the remainder for the exact result.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = m, X3 = x, X4 = n, X5 = y, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_madd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_madd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_madd)
        .text
        .balign 4

#define p x0
#define z x1
#define m x2
#define x x3
#define n x4
#define y x5
#define l x6
#define h x7
#define c x8
#define k x9
#define i x10
#define a x11
#define b x12
#define d x13
#define xx x14
#define yy x15


S2N_BN_SYMBOL(bignum_madd):
        CFI_START

// If p = 0 the result is trivial and nothing needs doing
// Note that fortuitously our "carry/remainder" term is still right!
// As it's a multiply-add, could also do this if either argument is trivial

        cbz     p, Lbignum_madd_end

// initialize (h,l) = 0, saving c = 0 for inside the loop

        mov     l, xzr
        mov     h, xzr

// Iterate outer loop from k = 0 ... k = p - 1 producing result digits

        mov     k, xzr
Lbignum_madd_outerloop:

// Add the existing z[k] and (h,l) to get initial (c,h,l) combination

        ldr     c, [z, k, lsl #3]
        adds    l, l, c
        adcs    h, h, xzr
        adc     c, xzr, xzr

// First let a = MAX 0 (k + 1 - n) and b = MIN (k + 1) m
// We want to accumulate all x[i] * y[k - i] for a <= i < b

        add     a, k, #1
        cmp     a, m
        csel    b, a, m, cc
        subs    a, a, n
        csel    a, a, xzr, cs

// Set loop count i = b - a, and skip everything if it's <= 0

        subs    i, b, a
        bls     Lbignum_madd_innerend

// Use temporary pointers xx = x + 8 * a and yy = y + 8 * (k - b)
// Increment xx per iteration but just use loop counter with yy
// So we start with [xx] = x[a] and [yy] = y[(k - b) + (b - a)] = y[k - a]

        lsl     xx, a, #3
        add     xx, xx, x

        sub     yy, k, b
        lsl     yy, yy, #3
        add     yy, yy, y

// And index using the loop counter i = b - a, ..., i = 1

Lbignum_madd_innerloop:
        ldr     a, [xx], #8
        ldr     b, [yy, i, lsl #3]
        mul     d, a, b
        umulh   a, a, b
        adds    l, l, d
        adcs    h, h, a
        adc     c, c, xzr
        subs    i, i, #1
        bne     Lbignum_madd_innerloop

Lbignum_madd_innerend:
        str     l, [z, k, lsl #3]
        mov     l, h
        mov     h, c

        add     k, k, #1
        cmp     k, p
        bcc     Lbignum_madd_outerloop                       // Inverted carry flag!

// Return the "carry/remainder" term

        mov     x0, l

Lbignum_madd_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_madd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

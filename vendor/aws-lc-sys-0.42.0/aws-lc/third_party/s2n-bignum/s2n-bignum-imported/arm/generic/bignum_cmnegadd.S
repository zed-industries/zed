// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negated multiply-add with single-word multiplier, z := z - c * y
// Inputs c, y[n]; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_cmnegadd(uint64_t k, uint64_t *z, uint64_t c, uint64_t n,
//                                    const uint64_t *y);
//
// Does the "z := z - c * y" operation where y is n digits, result z is p.
// Truncates the result in general.
//
// The return value is a high/carry word that is meaningful when n <= p.
// It is interpreted negatively as z' - 2^{64k} * return = z - c * y.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = c, X3 = n, X4 = y, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmnegadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmnegadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmnegadd)
        .text
        .balign 4

#define p x0
#define z x1
#define c x2
#define n x3
#define x x4

#define i x5
#define h x6
#define l x7
#define a x8

#define b x9


S2N_BN_SYMBOL(bignum_cmnegadd):
        CFI_START

// First clamp the input size n := min(p,n) since we can never need to read
// past the p'th term of the input to generate p-digit output.
// Subtract p := p - min(n,p) so it holds the size of the extra tail needed

        cmp     n, p
        csel    n, p, n, cs
        sub     p, p, n

// Initialize high part h = 0; if n = 0 do nothing but return that zero

        mov     h, xzr
        cbz     n, Lbignum_cmnegadd_end

// Initialization of the loop: 2^64 * CF + [h,z_0'] = z_0 + c * ~x_0 + c

        ldr     a, [x]
        mvn     a, a
        mul     l, c, a
        umulh   h, c, a
        adds    l, l, c
        adc     h, h, xzr
        ldr     b, [z]
        adds    b, b, l
        str     b, [z]
        mov     i, #8
        sub     n, n, #1
        cbz     n, Lbignum_cmnegadd_tail

// Main loop, where we always have CF + previous high part h to add in

Lbignum_cmnegadd_loop:
        ldr     a, [x, i]
        ldr     b, [z, i]
        mvn     a, a
        mul     l, c, a
        adcs    b, b, h
        umulh   h, c, a
        adc     h, h, xzr
        adds    b, b, l
        str     b, [z, i]
        add     i, i, #8
        sub     n, n, #1
        cbnz    n, Lbignum_cmnegadd_loop

// At this point we have 2^{64n} * (h + CF) + z' = z + c * (2^{64n} - x)
// so z' - 2^{64n} * (c - (h + CF)) = z - c * x.
// Since z - c * x < 2^{64n} we must have c - (h + CF) >= 0.
// Accumulate the negative carry in h for consistency with trivial cases.

Lbignum_cmnegadd_tail:
        adc     h, h, xzr
        sub     h, c, h

// Propagate the carry all the way to the end with h as extra carry word

        cbz     p, Lbignum_cmnegadd_end
        ldr     b, [z, i]
        subs    b, b, h
        str     b, [z, i]
        mov     h, xzr
        sub     p, p, #1
        cbz     p, Lbignum_cmnegadd_highend

Lbignum_cmnegadd_tloop:
        add     i, i, #8
        ldr     b, [z, i]
        sbcs    b, b, xzr
        str     b, [z, i]
        sub     p, p, #1
        cbnz    p, Lbignum_cmnegadd_tloop

// Adjust the high word with the inverted carry h := h + (1 - CF)

Lbignum_cmnegadd_highend:
        cset    x0, cc
        add     h, h, x0

// Now copy h into the function return

Lbignum_cmnegadd_end:
        mov     x0, h
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmnegadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

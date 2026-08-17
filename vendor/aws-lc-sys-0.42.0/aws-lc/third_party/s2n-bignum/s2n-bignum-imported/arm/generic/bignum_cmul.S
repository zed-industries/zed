// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply by a single word, z := c * y
// Inputs c, y[n]; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_cmul(uint64_t k, uint64_t *z, uint64_t c, uint64_t n,
//                                const uint64_t *y);
//
// Does the "z := c * y" operation where y is n digits, result z is p.
// Truncates the result in general unless p >= n + 1.
//
// The return value is a high/carry word that is meaningful when p >= n as
// giving the high part of the result. Since this is always zero if p > n,
// it is mainly of interest in the special case p = n, i.e. where the source
// and destination have the same nominal size, when it gives the extra word
// of the full result.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = c, X3 = n, X4 = y, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmul)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmul)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmul)
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


S2N_BN_SYMBOL(bignum_cmul):
        CFI_START

// First clamp the input size n := min(p,n) since we can never need to read
// past the p'th term of the input to generate p-digit output.
// Subtract p := p - min(n,p) so it holds the size of the extra tail needed

        cmp     n, p
        csel    n, p, n, cs
        sub     p, p, n

// Initialize current input/output pointer offset i and high part h.
// But then if n = 0 skip the multiplication and go to the tail part

        mov     h, xzr
        mov     i, xzr
        cbz     n, Lbignum_cmul_tail

// Initialization of the loop: [h,l] = c * x_0

        ldr     a, [x]
        mul     l, c, a
        umulh   h, c, a
        str     l, [z]
        add     i, i, #8
        subs    n, n, #1
        beq     Lbignum_cmul_tail

// Main loop (force CF = 0 at the beginning)

        adds    xzr, xzr, xzr
Lbignum_cmul_loop:
        ldr     a, [x, i]
        mul     l, c, a
        adcs    l, l, h
        umulh   h, c, a
        str     l, [z, i]
        add     i, i, #8
        sub     n, n, #1
        cbnz    n, Lbignum_cmul_loop

        adc     h, h, xzr

Lbignum_cmul_tail:
        cbz     p, Lbignum_cmul_end
        str     h, [z, i]
        mov     h, xzr
        subs    p, p, #1
        beq     Lbignum_cmul_end

Lbignum_cmul_tloop:
        add     i, i, #8
        str     xzr, [z, i]
        sub     p, p, #1
        cbnz    p, Lbignum_cmul_tloop

// Return the high/carry word

Lbignum_cmul_end:
        mov     x0, h
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmul)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

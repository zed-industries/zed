// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply-add with single-word multiplier, z := z + c * y
// Inputs c, y[n]; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_cmadd(uint64_t k, uint64_t *z, uint64_t c, uint64_t n,
//                                 const uint64_t *y);
//
// Does the "z := z + c * y" operation where y is n digits, result z is p.
// Truncates the result in general.
//
// The return value is a high/carry word that is meaningful when p = n + 1, or
// more generally when n <= p and the result fits in p + 1 digits. In these
// cases it gives the top digit of the (p + 1)-digit result.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = c, X3 = n, X4 = y, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_cmadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_cmadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_cmadd)
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


S2N_BN_SYMBOL(bignum_cmadd):
        CFI_START

// First clamp the input size n := min(p,n) since we can never need to read
// past the p'th term of the input to generate p-digit output.
// Subtract p := p - min(n,p) so it holds the size of the extra tail needed

        cmp     n, p
        csel    n, p, n, cs
        sub     p, p, n

// Initialize high part h = 0; if n = 0 do nothing but return that zero

        adds    h, xzr, xzr
        cbz     n, Lbignum_cmadd_end

// Initialization of the loop: 2^64 * CF + [h,z_0'] = z_0 + c * x_0

        ldr     a, [x]
        mul     l, c, a
        umulh   h, c, a
        ldr     b, [z]
        adds    b, b, l
        str     b, [z]
        mov     i, #8
        sub     n, n, #1
        cbz     n, Lbignum_cmadd_tail

// Main loop, where we always have CF + previous high part h to add in

Lbignum_cmadd_loop:
        ldr     a, [x, i]
        ldr     b, [z, i]
        mul     l, c, a
        adcs    b, b, h
        umulh   h, c, a
        adc     h, h, xzr
        adds    b, b, l
        str     b, [z, i]
        add     i, i, #8
        sub     n, n, #1
        cbnz    n, Lbignum_cmadd_loop

// Propagate the carry all the way to the end with h as extra carry word

Lbignum_cmadd_tail:
        cbz     p, Lbignum_cmadd_end
        ldr     b, [z, i]
        adcs    b, b, h
        str     b, [z, i]
        mov     h, xzr
        sub     p, p, #1
        cbz     p, Lbignum_cmadd_end

Lbignum_cmadd_tloop:
        add     i, i, #8
        ldr     b, [z, i]
        adcs    b, b, xzr
        str     b, [z, i]
        sub     p, p, #1
        cbnz    p, Lbignum_cmadd_tloop

// Return the high/carry word. This gives the top word of the result provided
// n <= p and the result fits in p + 1 digits. More generally, indeed, the
// 2^64 * CF + return = the top part of the result whenever n <= p, though this
// is not very exploitable from a C call.

Lbignum_cmadd_end:
        adcs    x0, h, xzr
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_cmadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

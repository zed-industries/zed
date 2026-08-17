// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Extended Montgomery reduce, returning results in input-output buffer
// Inputs z[2*k], m[k], w; outputs function return (extra result bit) and z[2*k]
//
//    extern uint64_t bignum_emontredc(uint64_t k, uint64_t *z, const uint64_t *m,
//                                     uint64_t w);
//
// Assumes that z initially holds a 2k-digit bignum z_0, m is a k-digit odd
// bignum and m * w == -1 (mod 2^64). This function also uses z for the output
// as well as returning a carry c of 0 or 1. This encodes two numbers: in the
// lower half of the z buffer we have q = z[0..k-1], while the upper half
// together with the carry gives r = 2^{64k}*c + z[k..2k-1]. These values
// satisfy z_0 + q * m = 2^{64k} * r, i.e. r gives a raw (unreduced) Montgomery
// reduction while q gives the multiplier that was used. Another way of
// thinking of it is that if z' is the output z with the lower half replaced
// with zeros, then z_0 + q * m = 2^{128k} * c + z'.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = m, X3 = w, returns X0
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_emontredc)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_emontredc)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_emontredc)
        .text
        .balign 4

#define k x0
#define z x1
#define m x2
#define w x3

// Outer loop counter
#define i x4
// Inner loop counter
#define j x5
// Home for Montgomery multiplier
#define d x6

// Top carry for current window
#define c x7

#define h x8
#define e x9
#define l x10
#define a x11


S2N_BN_SYMBOL(bignum_emontredc):
        CFI_START

// If k = 0 the whole operation is trivial; note we also get a return of c = 0

        cbz     k, Lbignum_emontredc_end

// Initialize top carry to zero, and launch into the outer loop

        mov     c, xzr
        mov     i, xzr
Lbignum_emontredc_outerloop:

        ldr     e, [z]
        mul     d, e, w
        ldr     a, [m]
        mul     l, d, a
        umulh   h, d, a
        str     d, [z]
        adds    xzr, e, l       // Will be zero but want the carry
        mov     j, #1
        sub     a, k, #1
        cbz     a, Lbignum_emontredc_montend
Lbignum_emontredc_montloop:
        ldr     a, [m, j, lsl #3]
        ldr     e, [z, j, lsl #3]
        mul     l, d, a
        adcs    e, e, h
        umulh   h, d, a
        adc     h, h, xzr
        adds    e, e, l
        str     e, [z, j, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_emontredc_montloop
Lbignum_emontredc_montend:
        adcs    h, h, c
        adc     c, xzr, xzr
        ldr     a, [z, k, lsl #3]
        adds    h, h, a
        adc     c, c, xzr
        str     h, [z, k, lsl #3]

// End of outer loop

        add     z, z, #8         // For simple indexing, z pointer moves
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_emontredc_outerloop

// Return c in X0

        mov     x0, c

Lbignum_emontredc_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_emontredc)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

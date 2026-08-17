// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from (almost-)Montgomery form z := (x / 2^{64k}) mod m
// Inputs x[k], m[k]; output z[k]
//
//    extern void bignum_demont(uint64_t k, uint64_t *z, const uint64_t *x,
//                              const uint64_t *m);
//
// Does z := (x / 2^{64k}) mod m, hence mapping out of Montgomery domain.
// In other words, this is a k-fold Montgomery reduction with same-size input.
// This can handle almost-Montgomery inputs, i.e. any k-digit bignum.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = x, X3 = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_demont)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_demont)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_demont)
        .text
        .balign 4

#define k x0
#define z x1
#define x x2
#define m x3

// Negated modular inverse
#define w x4
// Outer loop counter
#define i x5
// Inner loop counter
#define j x6
// Home for Montgomery multiplier
#define d x7

#define h x8
#define e x9
#define l x10
#define a x11

// Some more intuitive names for temp regs in initial word-level negmodinv.
// These just use i and j again, which aren't used early on.

#define one x5
#define e1 x5
#define e2 x6
#define e4 x5
#define e8 x6


S2N_BN_SYMBOL(bignum_demont):
        CFI_START

// If k = 0 the whole operation is trivial

        cbz     k, Lbignum_demont_end

// Compute word-level negated modular inverse w for m == m[0].
// This is essentially the same as word_negmodinv.

        ldr     a, [m]
        lsl     w, a, #2
        sub     w, a, w
        eor     w, w, #2
        mov     one, #1
        madd    e1, a, w, one
        mul     e2, e1, e1
        madd    w, e1, w, w
        mul     e4, e2, e2
        madd    w, e2, w, w
        mul     e8, e4, e4
        madd    w, e4, w, w
        madd    w, e8, w, w

// Initially just copy the input to the output. It would be a little more
// efficient but somewhat fiddlier to tweak the zeroth iteration below instead.

        mov     i, xzr
Lbignum_demont_iloop:
        ldr     a, [x, i, lsl #3]
        str     a, [z, i, lsl #3]
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_demont_iloop

// Outer loop, just doing a standard Montgomery reduction on z

        mov     i, xzr
Lbignum_demont_outerloop:

        ldr     e, [z]
        mul     d, e, w
        ldr     a, [m]
        mul     l, d, a
        umulh   h, d, a
        adds    e, e, l         // Will be zero but want the carry
        mov     j, #1
        sub     a, k, #1
        cbz     a, Lbignum_demont_montend
Lbignum_demont_montloop:
        ldr     a, [m, j, lsl #3]
        ldr     e, [z, j, lsl #3]
        mul     l, d, a
        adcs    e, e, h
        umulh   h, d, a
        adc     h, h, xzr
        adds    e, e, l
        sub     l, j, #1
        str     e, [z, l, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_demont_montloop
Lbignum_demont_montend:
        adc     h, xzr, h
        sub     l, j, #1
        str     h, [z, l, lsl #3]

// End of outer loop

        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_demont_outerloop

// Now do a comparison of z with m to set a final correction mask
// indicating that z >= m and so we need to subtract m.

        subs    j, xzr, xzr
Lbignum_demont_cmploop:
        ldr     a, [z, j, lsl #3]
        ldr     e, [m, j, lsl #3]
        sbcs    xzr, a, e
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_demont_cmploop
        csetm   h, cs

// Now do a masked subtraction of m for the final reduced result.

        subs    j, xzr, xzr
Lbignum_demont_corrloop:
        ldr     a, [z, j, lsl #3]
        ldr     e, [m, j, lsl #3]
        and     e, e, h
        sbcs    a, a, e
        str     a, [z, j, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_demont_corrloop

Lbignum_demont_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_demont)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

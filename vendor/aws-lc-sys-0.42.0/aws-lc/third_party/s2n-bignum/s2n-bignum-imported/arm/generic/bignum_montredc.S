// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery reduce, z := (x' / 2^{64p}) MOD m
// Inputs x[n], m[k], p; output z[k]
//
//    extern void bignum_montredc(uint64_t k, uint64_t *z, uint64_t n,
//                                const uint64_t *x, const uint64_t *m, uint64_t p);
//
// Does a := (x' / 2^{64p}) mod m where x' = x if n <= p + k and in general
// is the lowest (p+k) digits of x, assuming x' <= 2^{64p} * m. That is,
// p-fold Montgomery reduction w.r.t. a k-digit modulus m giving a k-digit
// answer.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = n, X3 = x, X4 = m, X5 = p
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montredc)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montredc)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montredc)
        .text
        .balign 4

#define k x0
#define z x1
#define n x2
#define x x3
#define m x4
#define p x5

// Negated modular inverse
#define w x6
// Outer loop counter
#define i x7
// Inner loop counter
#define j x8
// Home for Montgomery multiplier
#define d x9
// Top carry for current window
#define c x14

#define h x10
#define e x11
#define l x12
#define a x13

// Some more intuitive names for temp regs in initial word-level negmodinv.
// These just use i and j again, which aren't used early on.

#define one x7
#define e1 x7
#define e2 x8
#define e4 x7
#define e8 x8


S2N_BN_SYMBOL(bignum_montredc):
        CFI_START

// If k = 0 the whole operation is trivial

        cbz     k, Lbignum_montredc_end

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

// Initialize z to the lowest k digits of the input, zero-padding if n < k.

        cmp     n, k
        csel    j, k, n, cs
        mov     i, xzr
        cbz     j, Lbignum_montredc_padloop
Lbignum_montredc_copyloop:
        ldr     a, [x, i, lsl #3]
        str     a, [z, i, lsl #3]
        add     i, i, #1
        cmp     i, j
        bcc     Lbignum_montredc_copyloop

        cmp     i, k
        bcs     Lbignum_montredc_initialized

Lbignum_montredc_padloop:
        str     xzr, [z, i, lsl #3]
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_montredc_padloop

Lbignum_montredc_initialized:
        mov     c, xzr

// Now if p = 0 we just need the corrective tail, and even that is
// only needed for the case when the input is exactly the modulus,
// to maintain the <= 2^64p * n precondition

        cbz     p, Lbignum_montredc_corrective

// Outer loop, just doing a standard Montgomery reduction on z

        mov     i, xzr
Lbignum_montredc_outerloop:

        ldr     e, [z]
        mul     d, e, w
        ldr     a, [m]
        mul     l, d, a
        umulh   h, d, a
        adds    e, e, l         // Will be zero but want the carry
        mov     j, #1
        sub     a, k, #1
        cbz     a, Lbignum_montredc_montend
Lbignum_montredc_montloop:
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
        cbnz    a, Lbignum_montredc_montloop
Lbignum_montredc_montend:
        adcs    h, h, c
        adc     c, xzr, xzr
        add     j, j, i
        cmp     j, n
        bcs     Lbignum_montredc_offtheend
        ldr     a, [x, j, lsl #3]
        adds    h, h, a
        adc     c, c, xzr
Lbignum_montredc_offtheend:
        sub     j, k, #1
        str     h, [z, j, lsl #3]

// End of outer loop

        add     i, i, #1
        cmp     i, p
        bcc     Lbignum_montredc_outerloop

// Now do a comparison of (c::z) with (0::m) to set a final correction mask
// indicating that (c::z) >= m and so we need to subtract m.

Lbignum_montredc_corrective:

        subs    j, xzr, xzr
Lbignum_montredc_cmploop:
        ldr     a, [z, j, lsl #3]
        ldr     e, [m, j, lsl #3]
        sbcs    xzr, a, e
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montredc_cmploop

        sbcs    xzr, c, xzr
        csetm   c, cs

// Now do a masked subtraction of m for the final reduced result.

        subs    j, xzr, xzr
Lbignum_montredc_corrloop:
        ldr     a, [z, j, lsl #3]
        ldr     e, [m, j, lsl #3]
        and     e, e, c
        sbcs    a, a, e
        str     a, [z, j, lsl #3]
        add     j, j, #1
        sub     a, j, k
        cbnz    a, Lbignum_montredc_corrloop

Lbignum_montredc_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_montredc)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif

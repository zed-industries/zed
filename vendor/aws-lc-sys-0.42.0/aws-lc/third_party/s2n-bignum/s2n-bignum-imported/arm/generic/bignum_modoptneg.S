// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally negate modulo m, z := (-x) mod m (if p nonzero) or z := x
// (if p zero), assuming x reduced
// Inputs p, x[k], m[k]; output z[k]
//
//    extern void bignum_modoptneg(uint64_t k, uint64_t *z, uint64_t p,
//                                 const uint64_t *x, const uint64_t *m);
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = p, X3 = x, X4 = m
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_modoptneg)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_modoptneg)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_modoptneg)
        .text
        .balign 4

#define k x0
#define z x1
#define p x2
#define x x3
#define m x4
#define i x5
#define a x6
#define b x7


S2N_BN_SYMBOL(bignum_modoptneg):
        CFI_START

// Do nothing if k = 0

        cbz     k, Lbignum_modoptneg_end

// Make an additional check for zero input, and force p to zero in this case.
// This can be skipped if the input is known not to be zero a priori.

        mov     i, xzr
        mov     a, xzr
Lbignum_modoptneg_cmploop:
        ldr     b, [x, i, lsl #3]
        orr     a, a, b
        add     i, i, #1
        cmp     i, k
        bcc     Lbignum_modoptneg_cmploop

        cmp     a, xzr
        csel    p, p, xzr, ne

// Turn the input p into a strict bitmask

        cmp     p, xzr
        csetm   p, ne

// Main loop

        mov     i, xzr
        adds    xzr, p, p
Lbignum_modoptneg_mainloop:

        ldr     a, [m, i, lsl #3]
        ldr     b, [x, i, lsl #3]
        and     a, a, p
        eor     b, b, p
        adcs    a, a, b
        str     a, [z, i, lsl #3]
        add     i, i, #1
        sub     a, i, k
        cbnz    a, Lbignum_modoptneg_mainloop

Lbignum_modoptneg_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_modoptneg)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
